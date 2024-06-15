mod wrap;

use initiative_core::app::AutocompleteSuggestion;
use initiative_core::App;
use std::fmt;
use std::io;
use std::io::prelude::*;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use termion::color;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use wrap::wrap;

const CTRL_UP_ARROW: [u8; 6] = [27, 91, 49, 59, 53, 65];
const CTRL_DOWN_ARROW: [u8; 6] = [27, 91, 49, 59, 53, 66];
const CTRL_RIGHT_ARROW: [u8; 6] = [27, 91, 49, 59, 53, 67];
const CTRL_LEFT_ARROW: [u8; 6] = [27, 91, 49, 59, 53, 68];
const CTRL_DELETE: [u8; 6] = [27, 91, 51, 59, 53, 126];

#[derive(Debug)]
struct Input {
    history: Vec<String>,
    index: usize,
    cursor: usize,
    search_query: Option<String>,
}

#[derive(Debug)]
struct Autocomplete {
    suggestions: Vec<AutocompleteSuggestion>,
    selected: Option<usize>,
    query: String,
}

impl Autocomplete {
    fn up(self) -> Autocomplete {
        let suggestions = self.suggestions;
        let suggestions_length = suggestions.len();

        Autocomplete {
            suggestions,
            selected: match self.selected {
                Some(0) => Some(suggestions_length - 1),
                Some(x) => Some(x - 1),
                None => Some(suggestions_length - 1),
            },
            query: self.query,
        }
    }

    fn down(self) -> Autocomplete {
        let suggestions = self.suggestions;
        let suggestions_length = suggestions.len();

        Autocomplete {
            suggestions,
            selected: match self.selected {
                Some(selected) => Some((selected + 1) % suggestions_length),
                None => Some(0),
            },
            query: self.query,
        }
    }

    fn len(&self) -> usize {
        self.suggestions.len()
    }

    fn get_selected_suggestion(&self) -> Option<&AutocompleteSuggestion> {
        self.selected.map(|selected| &self.suggestions[selected])
    }

    fn get_only_suggestion(&self) -> Option<&AutocompleteSuggestion> {
        return match self.suggestions.len() {
            1 => self.suggestions.first(),
            _ => None,
        };
    }

    fn suggestions_share_prefix(&self, len: usize) -> bool {
        let prefixes = self
            .suggestions
            .iter()
            .map(|suggestion| {
                suggestion
                    .term
                    .chars()
                    .take(len)
                    .flat_map(char::to_lowercase)
                    .collect::<String>()
            })
            .collect::<std::collections::HashSet<_>>();

        prefixes.len() == 1
    }

    fn expand_match(self) -> Autocomplete {
        let Some(min_suggestion_len) = self.suggestions.iter().map(|s| s.term.len()).min() else {
            return self;
        };

        let mut expanded_end = self.query.len();
        while expanded_end <= min_suggestion_len && self.suggestions_share_prefix(expanded_end) {
            expanded_end += 1;
        }

        let additions = self
            .suggestions
            .first()
            .unwrap()
            .term
            .chars()
            .skip(self.query.len())
            .take(expanded_end - 1 - self.query.len());

        let mut next_query = self.query.clone();
        next_query.extend(additions);

        Autocomplete {
            suggestions: self.suggestions,
            selected: self.selected,
            query: next_query,
        }
    }

    async fn try_new(app: &App, input: &Input) -> Option<Autocomplete> {
        let query = input.get_text();

        if query.is_empty() {
            return None;
        }

        Some(Autocomplete {
            suggestions: app.autocomplete(query).await,
            selected: None,
            query: query.to_string(),
        })
    }
}

pub async fn run(mut app: App) -> io::Result<()> {
    let mut screen = termion::screen::AlternateScreen::from(io::stdout())
        .into_raw_mode()
        .unwrap();

    let (send, events) = mpsc::channel();
    let tty = termion::get_tty().unwrap();

    thread::spawn(move || {
        for event in tty.events() {
            if send.send(event).is_err() {
                return;
            }
        }
    });

    let mut input = Input::default();
    let mut output = String::new();
    let mut autocomplete: Option<Autocomplete> = None;

    draw_output(&mut screen, &output)?;
    draw_autocomplete(&mut screen, autocomplete.as_ref())?;
    draw_status(&mut screen, "")?;
    draw_input(&mut screen, &input, None)?;
    screen.flush()?;

    loop {
        let command = loop {
            // TODO: Handle multi-byte characters
            match events.recv_timeout(Duration::from_millis(100)) {
                Ok(event) => {
                    let status = format!("{:?}", event);

                    match event {
                        Ok(Event::Key(key)) => match key {
                            Key::Up => {
                                autocomplete = autocomplete.take().map(Autocomplete::up);

                                match autocomplete
                                    .as_ref()
                                    .and_then(Autocomplete::get_selected_suggestion)
                                {
                                    Some(suggestion) => input.set_text(&suggestion.term),
                                    None => input.key(key, false),
                                }
                            }
                            Key::Down => {
                                autocomplete = autocomplete.take().map(Autocomplete::down);

                                match autocomplete
                                    .as_ref()
                                    .and_then(Autocomplete::get_selected_suggestion)
                                {
                                    Some(suggestion) => input.set_text(&suggestion.term),
                                    None => input.key(key, false),
                                }
                            }
                            Key::Char('\n') => {
                                autocomplete = None;
                                let command = input.get_text().to_string();
                                input.key(key, false);
                                break command;
                            }
                            Key::Char('\t') => {
                                autocomplete = autocomplete.take().map(Autocomplete::expand_match);

                                if let Some(query) = autocomplete.as_ref().map(|it| &it.query) {
                                    input.set_text(query);
                                }
                            }
                            Key::Ctrl('c') => return Ok(()),
                            Key::Ctrl('h') => input.key(Key::Backspace, true),
                            Key::Ctrl(c) => input.key(Key::Char(c), true),
                            k => {
                                input.key(k, false);
                                autocomplete = Autocomplete::try_new(&app, &input).await;
                            }
                        },
                        Ok(Event::Unsupported(bytes)) => match bytes.as_slice() {
                            s if s == &CTRL_LEFT_ARROW[..] => input.key(Key::Left, true),
                            s if s == &CTRL_RIGHT_ARROW[..] => input.key(Key::Right, true),
                            s if s == &CTRL_UP_ARROW[..] => input.key(Key::Up, true),
                            s if s == &CTRL_DOWN_ARROW[..] => input.key(Key::Down, true),
                            s if s == &CTRL_DELETE[..] => input.key(Key::Delete, true),
                            _ => {}
                        },
                        Err(e) => return Err(e),
                        _ => {}
                    }

                    draw_output(&mut screen, &output)?;
                    draw_autocomplete(&mut screen, autocomplete.as_ref())?;
                    draw_status(&mut screen, &status)?;
                    draw_input(
                        &mut screen,
                        &input,
                        autocomplete
                            .as_ref()
                            .and_then(Autocomplete::get_only_suggestion),
                    )?;
                    screen.flush()?;
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {}
                Err(mpsc::RecvTimeoutError::Disconnected) => return Ok(()),
            }
        };

        output = app.command(&command).await.unwrap_or_else(|e| {
            format!(
                "{}{}{}{}{}",
                color::Fg(color::Black),
                color::Bg(color::Red),
                e,
                color::Fg(color::Reset),
                color::Bg(color::Reset),
            )
        });

        draw_output(&mut screen, &output)?;
        draw_autocomplete(&mut screen, autocomplete.as_ref())?;
        draw_status(&mut screen, "")?;
        draw_input(&mut screen, &input, None)?;
        screen.flush()?;
    }
}

impl Input {
    fn get_text(&self) -> &str {
        self.history.get(self.index).unwrap()
    }

    fn get_text_mut(&mut self) -> &mut String {
        self.history.get_mut(self.index).unwrap()
    }

    fn set_text(&mut self, text: &str) {
        self.get_text_mut().clear();
        self.get_text_mut().push_str(text);
        self.cursor = text.len();
    }

    fn key(&mut self, key: Key, ctrl: bool) {
        match (key, ctrl) {
            (Key::Left, false) if !self.is_at_start() => self.cursor -= 1,
            (Key::Left, true) => self.cursor = self.find_boundary_left(),
            (Key::Right, false) if !self.is_at_end() => self.cursor += 1,
            (Key::Right, true) => self.cursor = self.find_boundary_right(),
            (Key::Up, false) if self.index > 0 => {
                self.index -= 1;
                self.cursor = self.get_text().len();
            }
            (Key::Down, false) if self.index < self.history.len() - 1 => {
                self.index += 1;
                self.cursor = self.get_text().len();
            }

            (Key::Char('r'), true) => {
                if self.search_query.is_some() {
                    self.index.checked_sub(1).map(|prev_index| {
                        self.search_history(prev_index).map(|(index, cursor)| {
                            self.index = index;
                            self.cursor = cursor;
                        })
                    });
                } else {
                    self.search_query = Some(String::new());
                }
            }

            (Key::Esc, false) => self.search_query = None,

            (Key::Char('\n'), false) => {
                while self.history.last().map_or(false, |s| s.is_empty()) {
                    self.history.pop();
                }

                if self.index < self.history.len() - 1 {
                    if let Some(command) = self.history.get(self.index).cloned() {
                        self.history.push(command);
                    }
                }

                if self.history.len() > 1
                    && self.history.last() == self.history.get(self.history.len() - 2)
                {
                    self.history.pop();
                }

                self.history.push(String::new());
                self.index = self.history.len() - 1;
                self.cursor = self.get_text().len();
                self.search_query = None;
            }

            (Key::Backspace, false) if !self.is_at_start() => {
                if let Some(query) = self.search_query.as_mut() {
                    query.pop();
                } else {
                    self.cursor -= 1;
                    let cursor = self.cursor;
                    self.get_text_mut().remove(cursor);
                }
            }
            (Key::Backspace, true) if !self.is_at_start() => {
                let boundary = self.find_boundary_left();
                let cursor = self.cursor;
                self.get_text_mut().replace_range(boundary..cursor, "");
                self.cursor = boundary;
            }

            (Key::Home, _) => self.cursor = 0,
            (Key::End, _) => self.cursor = self.get_text().len(),

            (Key::Delete, false) if !self.is_at_end() => {
                let cursor = self.cursor;
                self.get_text_mut().remove(cursor);
            }
            (Key::Delete, true) if !self.is_at_end() => {
                let boundary = self.find_boundary_right();
                let cursor = self.cursor;
                self.get_text_mut().replace_range(cursor..boundary, "");
            }
            (Key::Char(c), false) => {
                if let Some(query) = self.search_query.as_mut() {
                    query.push(c);
                    if let Some((index, cursor)) = self.search_history(self.index) {
                        self.index = index;
                        self.cursor = cursor;
                    }
                } else {
                    if self.cursor == self.get_text().len() {
                        self.get_text_mut().push(c);
                    } else {
                        let cursor = self.cursor;
                        self.get_text_mut().insert(cursor, c);
                    }
                    self.cursor += 1;
                }
            }
            _ => {}
        }
    }

    fn search_history(&self, end_index: usize) -> Option<(usize, usize)> {
        if let Some(query) = &self.search_query {
            return self.history[0..=end_index]
                .iter()
                .enumerate()
                .rev()
                .find_map(|(index, command)| command.find(query).map(|cursor| (index, cursor)));
        }

        None
    }

    fn is_at_start(&self) -> bool {
        self.cursor == 0
    }

    fn is_at_end(&self) -> bool {
        self.cursor == self.get_text().len()
    }

    fn find_boundary_left(&self) -> usize {
        let mut boundary = self.cursor;

        if !self.get_text().is_empty() && boundary > 0 {
            boundary -= 1;

            while boundary > 0 {
                let mut chars = self.get_text().chars().skip(boundary - 1);
                let (prev_char, cur_char) = (chars.next().unwrap(), chars.next().unwrap());
                if !prev_char.is_alphanumeric() && cur_char.is_alphanumeric() {
                    break;
                }
                boundary -= 1;
            }
        }

        boundary
    }

    fn find_boundary_right(&self) -> usize {
        let mut boundary = self.cursor;

        if boundary < self.get_text().len() {
            boundary += 1;
            let mut alphanumeric_char_encountered = false;

            while boundary < self.get_text().len() {
                let mut chars = self.get_text().chars().skip(boundary - 1);
                let (prev_char, cur_char) = (chars.next().unwrap(), chars.next().unwrap());

                if prev_char.is_alphanumeric() {
                    alphanumeric_char_encountered = true;
                }

                match (prev_char.is_alphanumeric(), cur_char.is_alphanumeric()) {
                    (false, true) if alphanumeric_char_encountered => break,
                    _ if cur_char == ' ' && alphanumeric_char_encountered => break,
                    _ => {}
                }

                boundary += 1;
            }
        }

        boundary
    }
}

impl Default for Input {
    fn default() -> Input {
        Input {
            history: vec![String::new()],
            index: 0,
            cursor: 0,
            search_query: None,
        }
    }
}

impl fmt::Display for &Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_text())
    }
}

fn draw_output(screen: &mut dyn Write, output: &str) -> io::Result<()> {
    let (term_width, _) = termion::terminal_size().unwrap();

    print!("{}", termion::clear::All);

    for (num, line) in wrap(output, term_width as usize - 4).lines().enumerate() {
        write!(
            screen,
            "{}{}",
            termion::cursor::Goto(3, num as u16 + 1),
            line
        )?;
    }

    Ok(())
}

fn draw_status(screen: &mut dyn Write, status: &str) -> io::Result<()> {
    let (term_width, term_height) = termion::terminal_size().unwrap();

    write!(
        screen,
        "{}{}{}{} {}",
        termion::cursor::Save,
        termion::cursor::Goto(1, term_height),
        termion::color::Fg(termion::color::White),
        termion::color::Bg(termion::color::Blue),
        status,
    )?;

    for _ in status.len() + 1..term_width as usize {
        write!(screen, " ")?;
    }

    write!(
        screen,
        "{}{}{}",
        termion::color::Reset.fg_str(),
        termion::color::Reset.bg_str(),
        termion::cursor::Restore,
    )?;

    Ok(())
}

fn draw_input(
    screen: &mut dyn Write,
    input: &Input,
    suggestion: Option<&AutocompleteSuggestion>,
) -> io::Result<()> {
    let (_, term_height) = termion::terminal_size().unwrap();
    let input_row = term_height - 2;

    write!(
        screen,
        "{}{} > {}{}",
        termion::cursor::Goto(1, input_row),
        termion::clear::CurrentLine,
        input,
        termion::cursor::Goto(4 + input.cursor as u16, input_row)
    )?;

    if let Some(suggestion) = suggestion.map(|it| it.term.as_ref()) {
        write!(
            screen,
            "{}{}{}{}{}{}{}",
            termion::cursor::Save,
            termion::color::Fg(termion::color::LightWhite),
            termion::color::Bg(termion::color::Blue),
            suggestion
                .chars()
                .skip(input.get_text().len())
                .collect::<String>(),
            termion::color::Reset.fg_str(),
            termion::color::Reset.bg_str(),
            termion::cursor::Restore,
        )?;
    }

    Ok(())
}

fn draw_autocomplete(
    screen: &mut dyn Write,
    autocomplete: Option<&Autocomplete>,
) -> io::Result<()> {
    let Some(autocomplete) = autocomplete else {
        return Ok(());
    };

    let Some(term_width) = autocomplete.suggestions.iter().map(|i| i.term.len()).max() else {
        return Ok(());
    };
    let Some(summary_width) = autocomplete
        .suggestions
        .iter()
        .map(|i| i.summary.len())
        .max()
    else {
        return Ok(());
    };
    let width = term_width + summary_width + 2;

    let (_, term_height) = termion::terminal_size().unwrap();
    let start_row = term_height - 2 - autocomplete.len() as u16;

    for (pos, suggestion) in autocomplete.suggestions.iter().enumerate() {
        let padding =
            String::from_utf8(vec![
                b' ';
                width - suggestion.term.len() - suggestion.summary.len()
            ])
            .unwrap();
        let query_len = autocomplete.query.len();
        let line = pos as u16 + start_row;

        // This is a little indimidating but goes like so:
        // - Go to line position and set default color. Draw first part of border.
        // - Switch to the 'match color' and draw the matching text
        // - Switch to the text color, draw unmatched text
        // - Switch to summary color, draw padding and the summary.
        // - Return to default color and draw the last part of the border
        if Some(pos) == autocomplete.selected {
            write!(
                screen,
                "{}{}{} {}{}{}{}{}{}{}{}{}{}{}{} ",
                termion::cursor::Goto(3, line),
                termion::color::Fg(termion::color::White),
                termion::color::Bg(termion::color::LightBlack),
                termion::color::Fg(termion::color::White),
                termion::color::Bg(termion::color::Black),
                &suggestion.term[..query_len],
                termion::color::Fg(termion::color::White),
                termion::color::Bg(termion::color::Black),
                &suggestion.term[query_len..],
                termion::color::Fg(termion::color::LightBlack),
                termion::color::Bg(termion::color::Black),
                padding,
                suggestion.summary,
                termion::color::Fg(termion::color::White),
                termion::color::Bg(termion::color::LightBlack),
            )?;
        } else {
            write!(
                screen,
                "{}{}{} {}{}{}{}{}{}{}{}{}{}{}{} ",
                termion::cursor::Goto(3, line),
                termion::color::Fg(termion::color::White),
                termion::color::Bg(termion::color::LightBlack),
                termion::color::Fg(termion::color::Black),
                termion::color::Bg(termion::color::LightBlack),
                &suggestion.term[..query_len],
                termion::color::Fg(termion::color::White),
                termion::color::Bg(termion::color::LightBlack),
                &suggestion.term[query_len..],
                termion::color::Fg(termion::color::White),
                termion::color::Bg(termion::color::LightBlack),
                padding,
                suggestion.summary,
                termion::color::Fg(termion::color::White),
                termion::color::Bg(termion::color::LightBlack),
            )?;
        }
    }

    write!(
        screen,
        "{}{}",
        termion::color::Reset.fg_str(),
        termion::color::Reset.bg_str(),
    )?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn expand_single_match() {
        let autocomplete = Autocomplete {
            suggestions: vec![("shield", "a shield from the SRD").into()],
            selected: None,
            query: "s".into(),
        };

        let expanded = autocomplete.expand_match();
        assert_eq!(expanded.query, "shield");
    }

    #[test]
    fn expand_autocomplete() {
        let autocomplete = Autocomplete {
            suggestions: vec![
                ("shield", "a shield from the SRD").into(),
                ("shiny bauble", "so shiny!").into(),
            ],
            selected: None,
            query: "s".into(),
        };

        let expanded = autocomplete.expand_match();
        assert_eq!(expanded.query, "shi");
    }

    #[test]
    fn autocomplete_returns_single_suggestion() {
        let single = Autocomplete {
            suggestions: vec![("shield", "a shield from the SRD").into()],
            selected: None,
            query: "sh".into(),
        };
        assert_eq!(single.get_only_suggestion(), single.suggestions.get(0));

        let multiple = Autocomplete {
            suggestions: vec![
                ("shield", "a shield from the SRD").into(),
                ("shrine", "create a new shrine").into(),
            ],
            selected: None,
            query: "sh".into(),
        };
        assert_eq!(multiple.get_only_suggestion(), None);
    }

    #[test]
    fn autocomplete_up_test() {
        let mut autocomplete = Autocomplete {
            suggestions: vec![("shield", "a shield").into(), ("shrine", "a shrine").into()],
            selected: None,
            query: "sh".into(),
        };

        assert_eq!(autocomplete.len(), 2);
        assert_eq!(autocomplete.get_selected_suggestion(), None);

        autocomplete = autocomplete.up();
        assert_eq!(
            autocomplete.get_selected_suggestion(),
            autocomplete.suggestions.get(1)
        );

        autocomplete = autocomplete.up();
        assert_eq!(
            autocomplete.get_selected_suggestion(),
            autocomplete.suggestions.get(0)
        );

        autocomplete = autocomplete.up();
        assert_eq!(
            autocomplete.get_selected_suggestion(),
            autocomplete.suggestions.get(1)
        );
    }

    #[test]
    fn autocomplete_down_test() {
        let mut autocomplete = Autocomplete {
            suggestions: vec![
                ("shield", "a shield from the SRD").into(),
                ("shrine", "create a new shrine").into(),
            ],
            selected: None,
            query: "sh".into(),
        };

        assert_eq!(autocomplete.len(), 2);
        assert_eq!(autocomplete.get_selected_suggestion(), None);

        autocomplete = autocomplete.down();
        assert_eq!(
            autocomplete.get_selected_suggestion(),
            autocomplete.suggestions.get(0)
        );

        autocomplete = autocomplete.down();
        assert_eq!(
            autocomplete.get_selected_suggestion(),
            autocomplete.suggestions.get(1)
        );

        autocomplete = autocomplete.down();
        assert_eq!(
            autocomplete.get_selected_suggestion(),
            autocomplete.suggestions.get(0)
        );
    }

    #[test]
    fn key_left_test() {
        let mut input = Input {
            history: vec!["foo bar".to_string()],
            index: 0,
            cursor: 2,
            search_query: None,
        };

        input.key(Key::Left, false);
        assert_eq!(1, input.cursor);

        input.key(Key::Left, false);
        assert_eq!(0, input.cursor);

        input.key(Key::Left, false);
        assert_eq!(0, input.cursor);
    }

    #[test]
    fn key_ctrl_left_test() {
        let mut input = Input {
            history: vec!["foo bar".to_string()],
            index: 0,
            cursor: 7,
            search_query: None,
        };

        input.key(Key::Left, true);
        assert_eq!(4, input.cursor);

        input.key(Key::Left, true);
        assert_eq!(0, input.cursor);

        input.key(Key::Left, true);
        assert_eq!(0, input.cursor);
    }

    #[test]
    fn key_right_test() {
        let mut input = Input {
            history: vec!["foo bar".to_string()],
            index: 0,
            cursor: 5,
            search_query: None,
        };

        input.key(Key::Right, false);
        assert_eq!(6, input.cursor);

        input.key(Key::Right, false);
        assert_eq!(7, input.cursor);

        input.key(Key::Right, false);
        assert_eq!(7, input.cursor);
    }

    #[test]
    fn key_ctrl_right_test() {
        let mut input = Input {
            history: vec!["foo bar".to_string()],
            index: 0,
            cursor: 0,
            search_query: None,
        };

        input.key(Key::Right, true);
        assert_eq!(3, input.cursor);

        input.key(Key::Right, true);
        assert_eq!(7, input.cursor);

        input.key(Key::Right, true);
        assert_eq!(7, input.cursor);
    }

    #[test]
    fn key_up_test() {
        let mut input = Input {
            history: vec!["foo bar".to_string(), "baz".to_string()],
            index: 1,
            cursor: 0,
            search_query: None,
        };

        assert_eq!("baz", input.get_text());

        input.key(Key::Up, false);
        assert_eq!("foo bar", input.get_text());
        assert_eq!(0, input.index);
        assert_eq!(7, input.cursor);

        input.key(Key::Up, false);
        assert_eq!("foo bar", input.get_text());
    }

    #[test]
    fn key_down_test() {
        let mut input = Input {
            history: vec!["foo".to_string(), "bar baz".to_string()],
            index: 0,
            cursor: 0,
            search_query: None,
        };

        assert_eq!("foo", input.get_text());

        input.key(Key::Down, false);
        assert_eq!("bar baz", input.get_text());
        assert_eq!(1, input.index);
        assert_eq!(7, input.cursor);

        input.key(Key::Down, false);
        assert_eq!("bar baz", input.get_text());
    }

    #[test]
    fn key_enter_test() {
        let mut input = Input {
            history: vec!["foo".to_string(), "bar".to_string()],
            index: 1,
            cursor: 3,
            search_query: None,
        };

        input.key(Key::Char('\n'), false);
        assert_eq!(vec!["foo", "bar", ""], input.history);
        assert_eq!(2, input.index);
        assert_eq!(0, input.cursor);

        input.key(Key::Char('\n'), false);
        assert_eq!(vec!["foo", "bar", ""], input.history);
        assert_eq!(2, input.index);
        assert_eq!(0, input.cursor);
    }

    #[test]
    fn key_enter_with_history_test() {
        let mut input = Input {
            history: vec!["foo".to_string(), "bar".to_string()],
            index: 0,
            cursor: 3,
            search_query: None,
        };

        input.key(Key::Char('\n'), false);
        assert_eq!(vec!["foo", "bar", "foo", ""], input.history);
        assert_eq!(3, input.index);
        assert_eq!(0, input.cursor);

        input.history.last_mut().map(|s| s.push_str("foo"));
        input.cursor = 3;
        assert_eq!(vec!["foo", "bar", "foo", "foo"], input.history);

        input.key(Key::Char('\n'), false);
        assert_eq!(vec!["foo", "bar", "foo", ""], input.history);
        assert_eq!(3, input.index);
        assert_eq!(0, input.cursor);
    }

    #[test]
    fn key_ctrl_r_test() {
        let mut input = Input {
            history: vec![
                "foobar".to_string(),
                "barbaz".to_string(),
                "foobaz".to_string(),
                String::new(),
            ],
            index: 3,
            cursor: 0,
            search_query: None,
        };

        input.key(Key::Char('r'), true);
        assert_eq!(Some(String::new()), input.search_query);
        assert_eq!(3, input.index);

        input.key(Key::Char('b'), false);
        input.key(Key::Char('a'), false);
        assert_eq!(Some("ba".to_string()), input.search_query);
        assert!(input.history.last().map_or(false, |s| s.is_empty()));
        assert_eq!(2, input.index);

        input.key(Key::Char('r'), false);
        assert_eq!(Some("bar".to_string()), input.search_query);
        assert_eq!(1, input.index);

        input.key(Key::Char('r'), true);
        assert_eq!(0, input.index);
    }

    #[test]
    fn key_esc_test() {
        let mut input = Input::default();
        input.search_query = Some(String::new());

        input.key(Key::Esc, false);
        assert_eq!(None, input.search_query);

        input.key(Key::Esc, false);
        assert_eq!(None, input.search_query);
    }

    #[test]
    fn key_backspace_test() {
        let mut input = Input {
            history: vec!["bar baz".to_string()],
            index: 0,
            cursor: 2,
            search_query: None,
        };

        input.key(Key::Backspace, false);
        assert_eq!("br baz", input.get_text());
        assert_eq!(1, input.cursor);

        input.key(Key::Backspace, false);
        assert_eq!("r baz", input.get_text());
        assert_eq!(0, input.cursor);

        input.key(Key::Backspace, false);
        assert_eq!("r baz", input.get_text());
        assert_eq!(0, input.cursor);
    }

    #[test]
    fn key_ctrl_backspace_test() {
        let mut input = Input {
            history: vec!["foo bar".to_string()],
            index: 0,
            cursor: 5,
            search_query: None,
        };

        input.key(Key::Backspace, true);
        assert_eq!("foo ar", input.get_text());
        assert_eq!(4, input.cursor);

        input.key(Key::Backspace, true);
        assert_eq!("ar", input.get_text());
        assert_eq!(0, input.cursor);

        input.key(Key::Backspace, true);
        assert_eq!("ar", input.get_text());
        assert_eq!(0, input.cursor);
    }

    #[test]
    fn key_home_end_test() {
        let mut input = Input {
            history: vec!["foo bar".to_string()],
            index: 0,
            cursor: 4,
            search_query: None,
        };

        input.key(Key::Home, false);
        assert_eq!(0, input.cursor);

        input.key(Key::End, false);
        assert_eq!(7, input.cursor);
    }

    #[test]
    fn key_delete_test() {
        let mut input = Input {
            history: vec!["foo bar".to_string()],
            index: 0,
            cursor: 5,
            search_query: None,
        };

        input.key(Key::Delete, false);
        assert_eq!("foo br", input.get_text());
        assert_eq!(5, input.cursor);

        input.key(Key::Delete, false);
        assert_eq!("foo b", input.get_text());
        assert_eq!(5, input.cursor);

        input.key(Key::Delete, false);
        assert_eq!("foo b", input.get_text());
        assert_eq!(5, input.cursor);
    }

    #[test]
    fn key_ctrl_delete_test() {
        let mut input = Input {
            history: vec!["bar baz".to_string()],
            index: 0,
            cursor: 2,
            search_query: None,
        };

        input.key(Key::Delete, true);
        assert_eq!("ba baz", input.get_text());
        assert_eq!(2, input.cursor);

        input.key(Key::Delete, true);
        assert_eq!("ba", input.get_text());
        assert_eq!(2, input.cursor);

        input.key(Key::Delete, true);
        assert_eq!("ba", input.get_text());
        assert_eq!(2, input.cursor);
    }

    #[test]
    fn key_char_test() {
        let mut input = Input::default();

        input.key(Key::Char('A'), false);
        input.key(Key::Char('B'), false);
        input.key(Key::Char('X'), true);
        assert_eq!("AB", input.get_text());
        assert_eq!(2, input.cursor);

        input.cursor = 0;
        input.key(Key::Char('C'), false);
        assert_eq!("CAB", input.get_text());
        assert_eq!(1, input.cursor);
    }

    #[test]
    fn is_at_start_end_test() {
        let mut input = Input::default();
        assert!(input.is_at_start());
        assert!(input.is_at_end());

        input.get_text_mut().push_str("ab");
        assert!(input.is_at_start());
        assert!(!input.is_at_end());

        input.cursor += 1;
        assert!(!input.is_at_start());
        assert!(!input.is_at_end());

        input.cursor += 1;
        assert!(!input.is_at_start());
        assert!(input.is_at_end());
    }

    #[test]
    fn find_boundary_left_test() {
        let mut input = Input {
            history: vec!["A test-string with words, punctuation - and a dash!  ".to_string()],
            index: 0,
            cursor: 0,
            search_query: None,
        };
        input.cursor = input.get_text().len();

        let mut stops = Vec::new();

        // A test-string with words, punctuation - and a dash!
        // ^ ^    ^      ^    ^      ^             ^   ^ ^
        // 0 2    7      1    1      2             4   4 4
        //               4    9      6             0   4 6
        while input.cursor > 0 && stops.len() < 100 {
            input.cursor = input.find_boundary_left();
            stops.push(input.cursor);
        }

        assert_eq!(vec![46, 44, 40, 26, 19, 14, 7, 2, 0], stops);
        assert_eq!(0, input.find_boundary_left());
    }

    #[test]
    fn find_boundary_right_test() {
        let mut input = Input {
            history: vec!["A test-string with words, punctuation - and a dash!  ".to_string()],
            index: 0,
            cursor: 0,
            search_query: None,
        };

        let mut stops = Vec::new();

        // A test-string with words, punctuation - and a dash!
        //  ^     ^     ^    ^      ^           ^     ^ ^     ^ ^
        //  1     7     1    1      2           3     4 4     5 5
        //              3    8      5           7     3 5     1 3
        while input.cursor < input.get_text().len() && stops.len() < 100 {
            input.cursor = input.find_boundary_right();
            stops.push(input.cursor);
        }

        assert_eq!(vec![1, 7, 13, 18, 25, 37, 43, 45, 51, 53], stops);
        assert_eq!(input.get_text().len(), input.find_boundary_right());
    }
}
