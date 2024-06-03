mod wrap;

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
use initiative_core::app::AutocompleteSuggestion;
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
    index: Option<usize>,
}

impl Autocomplete {
    fn up(self) -> Autocomplete { 
        let suggestions = self.suggestions;
        let suggestions_length = suggestions.len();
        
        Autocomplete {
            suggestions,
            index: match self.index {
                Some(i) => Some(if i == 0 { suggestions_length - 1 } else { i - 1 }),
                None => Some(suggestions_length - 1),
            },
        }
    }
    
    fn down(self) -> Autocomplete {
        let suggestions = self.suggestions;
        let suggestions_length = suggestions.len();

        Autocomplete {
            suggestions,
            index: match self.index {
                Some(i) => Some(if i == suggestions_length - 1 { 0 } else { i + 1 }),
                None => Some(0),
            },
        }
    }

    fn len(&self) -> u16 {
        self.suggestions.len().try_into().unwrap()
    }

    async fn maybe_create(app: &App, input: &Input) -> Option<Autocomplete> {
        let query = input.text();

        if query.is_empty() {
            return None
        }
        
        Some(Autocomplete {
            suggestions: app.autocomplete(query).await,
            index: None,
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
    draw_input(&mut screen, &input)?;
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
                                input.key(key, false)
                            }
                            Key::Down => {
                                autocomplete = autocomplete.take().map(Autocomplete::down);
                                input.key(key, false)
                            }
                            Key::Char('\n') => {
                                autocomplete = None;
                                let command = input.text().to_string();
                                input.key(key, false);
                                break command;
                            }
                            Key::Ctrl('c') => return Ok(()),
                            Key::Ctrl('h') => input.key(Key::Backspace, true),
                            Key::Ctrl(c) => input.key(Key::Char(c), true),
                            k => {
                                input.key(k, false);
                                autocomplete = Autocomplete::maybe_create(&app, &input).await;
                            },
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
                    draw_input(&mut screen, &input)?;
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
        draw_input(&mut screen, &input)?;
        screen.flush()?;
    }
}

impl Input {
    fn text(&self) -> &str {
        self.history.get(self.index).unwrap()
    }

    fn text_mut(&mut self) -> &mut String {
        self.history.get_mut(self.index).unwrap()
    }

    fn key(&mut self, key: Key, ctrl: bool) {
        match (key, ctrl) {
            (Key::Left, false) if !self.is_at_start() => self.cursor -= 1,
            (Key::Left, true) => self.cursor = self.find_boundary_left(),
            (Key::Right, false) if !self.is_at_end() => self.cursor += 1,
            (Key::Right, true) => self.cursor = self.find_boundary_right(),
            (Key::Up, false) if self.index > 0 => {
                self.index -= 1;
                self.cursor = self.text().len();
            }
            (Key::Down, false) if self.index < self.history.len() - 1 => {
                self.index += 1;
                self.cursor = self.text().len();
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
                self.cursor = self.text().len();
                self.search_query = None;
            }

            (Key::Backspace, false) if !self.is_at_start() => {
                if let Some(query) = self.search_query.as_mut() {
                    query.pop();
                } else {
                    self.cursor -= 1;
                    let cursor = self.cursor;
                    self.text_mut().remove(cursor);
                }
            }
            (Key::Backspace, true) if !self.is_at_start() => {
                let boundary = self.find_boundary_left();
                let cursor = self.cursor;
                self.text_mut().replace_range(boundary..cursor, "");
                self.cursor = boundary;
            }

            (Key::Home, _) => self.cursor = 0,
            (Key::End, _) => self.cursor = self.text().len(),

            (Key::Delete, false) if !self.is_at_end() => {
                let cursor = self.cursor;
                self.text_mut().remove(cursor);
            }
            (Key::Delete, true) if !self.is_at_end() => {
                let boundary = self.find_boundary_right();
                let cursor = self.cursor;
                self.text_mut().replace_range(cursor..boundary, "");
            }
            (Key::Char(c), false) => {
                if let Some(query) = self.search_query.as_mut() {
                    query.push(c);
                    if let Some((index, cursor)) = self.search_history(self.index) {
                        self.index = index;
                        self.cursor = cursor;
                    }
                } else {
                    if self.cursor == self.text().len() {
                        self.text_mut().push(c);
                    } else {
                        let cursor = self.cursor;
                        self.text_mut().insert(cursor, c);
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
        self.cursor == self.text().len()
    }

    fn find_boundary_left(&self) -> usize {
        let mut boundary = self.cursor;

        if !self.text().is_empty() && boundary > 0 {
            boundary -= 1;

            while boundary > 0 {
                let mut chars = self.text().chars().skip(boundary - 1);
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

        if boundary < self.text().len() {
            boundary += 1;
            let mut alphanumeric_char_encountered = false;

            while boundary < self.text().len() {
                let mut chars = self.text().chars().skip(boundary - 1);
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
        write!(f, "{}", self.text())
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

fn draw_input(screen: &mut dyn Write, input: &Input) -> io::Result<()> {
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

    Ok(())
}

fn draw_autocomplete(screen: &mut dyn Write, autocomplete: Option<&Autocomplete>) -> io::Result<()> {
    let ac = match autocomplete {
        Some(ac) => ac,
        None => return Ok(()),
    };

    let max_term_width = match ac.suggestions.iter().map(|i| i.term.len()).max() {
        Some(size) => size,
        None => return Ok(()),
    };
    let max_summary_width = match ac.suggestions.iter().map(|i| i.summary.len()).max() {
        Some(size) => size,
        None => return Ok(()),
    };
    let width = max_term_width + max_summary_width + 2;

    let (_, term_height) = termion::terminal_size().unwrap();
    let start_row = term_height - 2 - ac.len();

    write!(
        screen,
        "{}{}",
        termion::color::Fg(termion::color::Black),
        termion::color::Bg(termion::color::LightBlack),
    )?;

    for (pos, suggestion) in ac.suggestions.iter().enumerate() {
        let offset: u16 = pos.try_into().unwrap();

        write!(
            screen,
            "{}",
            termion::cursor::Goto(3, start_row + offset),
        )?;
        
        if Some(pos) == ac.index {
            write!(
                screen,
                "{}{}",
                termion::style::Italic,
                termion::color::Fg(termion::color::White),
            )?;
        }

        write!(screen, "{}", suggestion.term)?;
        for _ in 0..(width - suggestion.term.len() - suggestion.summary.len()) {
            write!(screen, "{}", " ")?;
        }
        write!(screen, "{}", suggestion.summary)?;
        
        if Some(pos) == ac.index {
            write!(
                screen,
                "{}{}",
                termion::style::NoItalic,
                termion::color::Fg(termion::color::Black),
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

        assert_eq!("baz", input.text());

        input.key(Key::Up, false);
        assert_eq!("foo bar", input.text());
        assert_eq!(0, input.index);
        assert_eq!(7, input.cursor);

        input.key(Key::Up, false);
        assert_eq!("foo bar", input.text());
    }

    #[test]
    fn key_down_test() {
        let mut input = Input {
            history: vec!["foo".to_string(), "bar baz".to_string()],
            index: 0,
            cursor: 0,
            search_query: None,
        };

        assert_eq!("foo", input.text());

        input.key(Key::Down, false);
        assert_eq!("bar baz", input.text());
        assert_eq!(1, input.index);
        assert_eq!(7, input.cursor);

        input.key(Key::Down, false);
        assert_eq!("bar baz", input.text());
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
        assert_eq!("br baz", input.text());
        assert_eq!(1, input.cursor);

        input.key(Key::Backspace, false);
        assert_eq!("r baz", input.text());
        assert_eq!(0, input.cursor);

        input.key(Key::Backspace, false);
        assert_eq!("r baz", input.text());
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
        assert_eq!("foo ar", input.text());
        assert_eq!(4, input.cursor);

        input.key(Key::Backspace, true);
        assert_eq!("ar", input.text());
        assert_eq!(0, input.cursor);

        input.key(Key::Backspace, true);
        assert_eq!("ar", input.text());
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
        assert_eq!("foo br", input.text());
        assert_eq!(5, input.cursor);

        input.key(Key::Delete, false);
        assert_eq!("foo b", input.text());
        assert_eq!(5, input.cursor);

        input.key(Key::Delete, false);
        assert_eq!("foo b", input.text());
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
        assert_eq!("ba baz", input.text());
        assert_eq!(2, input.cursor);

        input.key(Key::Delete, true);
        assert_eq!("ba", input.text());
        assert_eq!(2, input.cursor);

        input.key(Key::Delete, true);
        assert_eq!("ba", input.text());
        assert_eq!(2, input.cursor);
    }

    #[test]
    fn key_char_test() {
        let mut input = Input::default();

        input.key(Key::Char('A'), false);
        input.key(Key::Char('B'), false);
        input.key(Key::Char('X'), true);
        assert_eq!("AB", input.text());
        assert_eq!(2, input.cursor);

        input.cursor = 0;
        input.key(Key::Char('C'), false);
        assert_eq!("CAB", input.text());
        assert_eq!(1, input.cursor);
    }

    #[test]
    fn is_at_start_end_test() {
        let mut input = Input::default();
        assert!(input.is_at_start());
        assert!(input.is_at_end());

        input.text_mut().push_str("ab");
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
        input.cursor = input.text().len();

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
        while input.cursor < input.text().len() && stops.len() < 100 {
            input.cursor = input.find_boundary_right();
            stops.push(input.cursor);
        }

        assert_eq!(vec![1, 7, 13, 18, 25, 37, 43, 45, 51, 53], stops);
        assert_eq!(input.text().len(), input.find_boundary_right());
    }
}
