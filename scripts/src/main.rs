use json::JsonValue;
use quick_xml::events::Event;
use quick_xml::Reader;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = Vec::new();
    let mut json_objects: HashMap<InfoboxType, JsonValue> = HashMap::new();
    let stdin = io::stdin();
    let mut reader = Reader::from_reader(stdin.lock());
    let mut read_mode = None;
    let mut skip_page = false;
    let mut title = String::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(e)) => {
                read_mode = match e.name() {
                    b"page" => {
                        skip_page = false;
                        title = String::new();
                        None
                    }
                    b"title" => Some(ReadMode::Title),
                    b"text" => Some(ReadMode::Text),
                    _ => None,
                }
            }
            Ok(Event::Empty(e)) if e.name() == b"redirect" => {
                skip_page = true;
            }
            Ok(Event::Text(e)) => {
                match read_mode {
                    Some(ReadMode::Title) => {
                        title = e.unescape_and_decode(&reader)?;
                        if title.contains(':') {
                            skip_page = true;
                        }
                    }
                    Some(ReadMode::Text) if !skip_page => {
                        if let Some((infobox_type, fields)) =
                            parse(e.unescape_and_decode(&reader)?.as_str())
                        {
                            json_objects
                                .entry(infobox_type)
                                .or_insert_with(|| JsonValue::new_object())
                                .insert(title.as_str(), fields)
                                .unwrap();
                        }
                    }
                    _ => {}
                }
                read_mode = None;
            }
            Ok(Event::Eof) => break,
            _ => (),
        }
        buf.clear();
    }

    for (infobox_type, json_object) in json_objects.iter() {
        json_object.write_pretty(&mut File::create(format!("{}s.json", infobox_type))?, 2)?;
    }

    Ok(())
}

fn parse(text: &str) -> Option<(InfoboxType, HashMap<&str, String>)> {
    let infobox_start =
        Regex::new(r"\{\{(?P<name>[Cc]reature|[Pp]erson|[Ss]tate|[Ll]ocation|[Bb]uilding)\b")
            .unwrap();
    let template_start_end = Regex::new(r"\{\{|\}\}").unwrap();
    let field_start =
        Regex::new(r"^\|\s*(?P<name>[A-Za-z0-9][^=]+[A-Za-z0-9])\s*=\s*(?P<value>\S.+)$").unwrap();

    let mut depth: Option<u8> = None;
    let mut key = "";
    let mut value = String::new();
    let mut infobox_type: Option<InfoboxType> = None;
    let mut fields = HashMap::new();

    for line in text.lines() {
        if let Some(caps) = infobox_start.captures(line) {
            infobox_type = caps.name("name").unwrap().as_str().parse().ok();
            depth = Some(0);
        } else if let Some(caps) = field_start.captures(line) {
            if !key.is_empty() {
                value = strip_tags(value.as_str());
                if !value.is_empty() {
                    fields.insert(key, value);
                }
            }

            key = caps.name("name").unwrap().as_str();
            value = String::from(caps.name("value").unwrap().as_str().trim());
        } else if !line.starts_with('|') {
            value.push('\n');
            value.push_str(line.trim());
        }

        if let Some(d) = depth {
            for caps in template_start_end.captures_iter(line) {
                match caps.get(0).map(|m| m.as_str()) {
                    Some("{{") => depth = d.checked_add(1),
                    Some("}}") => depth = d.checked_sub(1),
                    _ => unreachable!(),
                }

                if depth.is_none() {
                    return infobox_type.map(|t| (t, fields));
                }
            }
        }
    }

    None
}

fn strip_tags(text: &str) -> String {
    let mut buf = Vec::new();
    let mut output = String::with_capacity(text.len());
    let mut reader = Reader::from_str(text);
    let mut tag_depth: u8 = 0;
    let wikitext_pattern = Regex::new(r"\{\{|\}\}|\[\[|\]\]|\|").unwrap();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(_)) => tag_depth += 1,
            Ok(Event::End(_)) => tag_depth -= 1,
            Ok(Event::Empty(e)) if e.name() == b"br" => output.push('\n'),
            Ok(Event::Text(e)) if tag_depth == 0 => {
                let mut i = 0;
                let mut template_depth: u8 = 0;
                let mut link_depth: u8 = 0;

                if let Ok(text) = e.unescape_and_decode(&reader) {
                    for caps in wikitext_pattern.captures_iter(&text) {
                        let cap_match = caps.get(0).unwrap();

                        if template_depth == 0 && link_depth == 0 {
                            output.push_str(&text[i..cap_match.start()]);
                        }

                        match cap_match.as_str() {
                            "{{" => template_depth += 1,
                            "}}" => template_depth = template_depth.saturating_sub(1),
                            "[[" => link_depth += 1,
                            "]]" => {
                                link_depth = link_depth.saturating_sub(1);
                                if link_depth == 0 {
                                    output.push_str(&text[i..cap_match.start()]);
                                }
                            }
                            _ => {}
                        }

                        i = cap_match.end();
                    }
                    output.push_str(&text[i..text.len()]);
                }
            }
            Ok(Event::Eof) => break,
            _ => {}
        }
    }

    output
}

#[derive(Debug)]
enum ReadMode {
    Title,
    Text,
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum InfoboxType {
    Creature,
    Person,
    State,
    Location,
    Building,
}

impl FromStr for InfoboxType {
    type Err = ();

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        Ok(match raw {
            "creature" | "Creature" => InfoboxType::Creature,
            "person" | "Person" => InfoboxType::Person,
            "state" | "State" => InfoboxType::State,
            "location" | "Location" => InfoboxType::Location,
            "building" | "Building" => InfoboxType::Building,
            _ => Err(())?,
        })
    }
}

impl fmt::Display for InfoboxType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InfoboxType::Creature => write!(f, "creature"),
            InfoboxType::Person => write!(f, "person"),
            InfoboxType::State => write!(f, "state"),
            InfoboxType::Location => write!(f, "location"),
            InfoboxType::Building => write!(f, "building"),
        }
    }
}
