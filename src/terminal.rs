use std::fmt;
use std::io;
use std::io::prelude::*;
use std::thread::sleep;
use std::time::Duration;

use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::Context;

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
}

pub fn run(mut context: Context) -> io::Result<()> {
    let mut screen = termion::screen::AlternateScreen::from(io::stdout())
        .into_raw_mode()
        .unwrap();

    let mut events = termion::async_stdin().events();
    let mut input = Input::default();

    draw_input(&mut screen, &input)?;
    draw_status(&mut screen, "")?;
    screen.flush()?;

    loop {
        let command = loop {
            // TODO: Handle multi-byte characters
            if let Some(event) = events.next() {
                let status = format!("{:?}", event);

                match event {
                    Ok(Event::Key(key)) => match key {
                        Key::Char('\n') => {
                            let command = input.text().to_string();
                            input.key(key, false);
                            break command;
                        }
                        Key::Ctrl('h') => input.key(Key::Backspace, true),
                        Key::Ctrl(c) => input.key(Key::Char(c), true),
                        Key::Esc => return Ok(()),
                        k => input.key(k, false),
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

                draw_status(&mut screen, status.as_str())?;
                draw_input(&mut screen, &input)?;
                screen.flush()?;
            } else {
                sleep(Duration::from_millis(100));
            }
        };

        write!(
            &mut screen,
            "{}{}",
            termion::cursor::Goto(1, 1),
            context.run(&command)
        )?;

        draw_status(&mut screen, "")?;
        draw_input(&mut screen, &input)?;
        screen.flush()?;
    }
}

impl Input {
    fn text(&self) -> &str {
        self.history.get(self.index).unwrap().as_str()
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

            (Key::Char('\n'), false) => {
                if !self.history.last().map_or(false, |s| s.is_empty()) {
                    self.history.push(String::new());
                }
                self.index = self.history.len() - 1;
                self.cursor = self.text().len();
            }

            (Key::Backspace, false) if !self.is_at_start() => {
                self.cursor -= 1;
                let cursor = self.cursor;
                self.text_mut().remove(cursor);
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
                if self.cursor == self.text().len() {
                    self.text_mut().push(c);
                } else {
                    let cursor = self.cursor;
                    self.text_mut().insert(cursor, c);
                }
                self.cursor += 1;
            }
            _ => {}
        }
    }

    fn is_at_start(&self) -> bool {
        self.cursor == 0
    }

    fn is_at_end(&self) -> bool {
        self.cursor == self.text().len()
    }

    fn find_boundary_left(&self) -> usize {
        let mut boundary = self.cursor;

        if self.text().len() > 0 && boundary > 0 {
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

#[cfg(test)]
mod test_input {
    use super::*;

    #[test]
    fn key_left_test() {
        let mut input = Input {
            history: vec!["foo bar".to_string()],
            index: 0,
            cursor: 7,
        };

        input.key(Key::Left, false);
        assert_eq!(6, input.cursor);

        input.key(Key::Left, true);
        assert_eq!(4, input.cursor);

        input.key(Key::Left, true);
        assert_eq!(0, input.cursor);

        input.key(Key::Left, false);
        assert_eq!(0, input.cursor);

        input.key(Key::Left, true);
        assert_eq!(0, input.cursor);
    }

    #[test]
    fn key_right_test() {
        let mut input = Input {
            history: vec!["foo bar".to_string()],
            index: 0,
            cursor: 0,
        };

        input.key(Key::Right, false);
        assert_eq!(1, input.cursor);

        input.key(Key::Right, true);
        assert_eq!(3, input.cursor);

        input.key(Key::Right, true);
        assert_eq!(7, input.cursor);

        input.key(Key::Right, false);
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
            index: 0,
            cursor: 3,
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
    fn key_backspace_test() {
        let mut input = Input {
            history: vec!["foo bar".to_string()],
            index: 0,
            cursor: 4,
        };

        input.key(Key::Backspace, false);
        assert_eq!("foobar", input.text());
        assert_eq!(3, input.cursor);

        input.key(Key::Backspace, true);
        assert_eq!("bar", input.text());
        assert_eq!(0, input.cursor);

        input.key(Key::Backspace, false);
        assert_eq!("bar", input.text());
        assert_eq!(0, input.cursor);

        input.key(Key::Backspace, true);
        assert_eq!("bar", input.text());
        assert_eq!(0, input.cursor);
    }

    #[test]
    fn key_home_end_test() {
        let mut input = Input {
            history: vec!["foo bar".to_string()],
            index: 0,
            cursor: 4,
        };

        input.key(Key::Home, false);
        assert_eq!(0, input.cursor);

        input.key(Key::End, false);
        assert_eq!(7, input.cursor);

        input.key(Key::Home, true);
        assert_eq!(0, input.cursor);

        input.key(Key::End, true);
        assert_eq!(7, input.cursor);
    }

    #[test]
    fn key_delete_test() {
        let mut input = Input {
            history: vec!["foo bar".to_string()],
            index: 0,
            cursor: 3,
        };

        input.key(Key::Delete, false);
        assert_eq!("foobar", input.text());
        assert_eq!(3, input.cursor);

        input.key(Key::Delete, true);
        assert_eq!("foo", input.text());
        assert_eq!(3, input.cursor);

        input.key(Key::Delete, false);
        assert_eq!("foo", input.text());
        assert_eq!(3, input.cursor);

        input.key(Key::Delete, true);
        assert_eq!("foo", input.text());
        assert_eq!(3, input.cursor);
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

impl Default for Input {
    fn default() -> Input {
        Input {
            history: vec![String::new()],
            index: 0,
            cursor: 0,
        }
    }
}

impl From<Input> for String {
    fn from(mut input: Input) -> String {
        input.history.drain(..).skip(input.index).next().unwrap()
    }
}

impl From<String> for Input {
    fn from(text: String) -> Input {
        let cursor = text.len();
        Input {
            history: vec![text],
            index: 0,
            cursor,
        }
    }
}

impl fmt::Display for &Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text())
    }
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
    let (term_width, term_height) = termion::terminal_size().unwrap();
    let input_row = term_height - 2;

    write!(
        screen,
        "{} > {}",
        termion::cursor::Goto(1, input_row),
        input
    )?;

    for _ in (input.text().len() as u16 + 3)..=term_width {
        write!(screen, " ")?;
    }

    write!(
        screen,
        "{}",
        termion::cursor::Goto(4 + input.cursor as u16, input_row)
    )?;

    Ok(())
}
