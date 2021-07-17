use super::App;
use std::fmt;
use std::io;
use std::io::prelude::*;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

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

pub fn run(mut app: App) -> io::Result<()> {
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

    draw_input(&mut screen, &input)?;
    draw_status(&mut screen, "")?;
    screen.flush()?;

    loop {
        let command = loop {
            // TODO: Handle multi-byte characters
            match events.recv_timeout(Duration::from_millis(100)) {
                Ok(event) => {
                    let status = format!("{:?}", event);

                    match event {
                        Ok(Event::Key(key)) => match key {
                            Key::Char('\n') => {
                                let command = input.text().to_string();
                                input.key(key, false);
                                break command;
                            }
                            Key::Ctrl('c') => return Ok(()),
                            Key::Ctrl('h') => input.key(Key::Backspace, true),
                            Key::Ctrl(c) => input.key(Key::Char(c), true),
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
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {}
                Err(mpsc::RecvTimeoutError::Disconnected) => return Ok(()),
            }
        };

        print!("{}", termion::clear::All);

        let output = format!("{}", app.command(&command));
        wrap(&output, termion::terminal_size().unwrap().0 as usize - 4)
            .lines()
            .enumerate()
            .for_each(|(num, line)| {
                write!(
                    &mut screen,
                    "{}{}",
                    termion::cursor::Goto(3, num as u16 + 1),
                    line
                )
                .unwrap();
            });

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

#[cfg(test)]
mod test_input {
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

fn wrap(input: &str, line_len: usize) -> String {
    let mut result = String::with_capacity(input.len());
    let mut cur_line_len = 0;

    input.split_inclusive(char::is_whitespace).for_each(|word| {
        let word_len = word.trim_end().len();

        if word_len + cur_line_len <= line_len {
            result.push_str(word);
            if word.ends_with('\n') {
                cur_line_len = 0;
            } else {
                cur_line_len += word.len();
            }
        } else {
            // Trim trailing whitespace from the previous line.
            while let Some(c) = result.pop() {
                if !c.is_whitespace() {
                    result.push(c);
                    break;
                }
            }

            result.push('\n');

            cur_line_len = if word_len > line_len {
                word.chars().enumerate().for_each(|(i, c)| {
                    result.push(c);
                    if i % line_len == line_len - 1 && !c.is_whitespace() {
                        result.push('\n');
                    }
                });

                word.len() % line_len
            } else {
                result.push_str(word);
                word.len()
            };
        }
    });

    result
}

#[cfg(test)]
mod test_wrap {
    use super::*;

    #[test]
    fn wrap_short_test() {
        assert_eq!(
            "A word\n\
             wrappe\n\
             d\n\
             senten\n\
             ce\n\
             with\n\
             a\n\
             line\n\
             break.",
            wrap("A word wrapped sentence with\na\nline break.", 6).as_str()
        );
    }

    #[test]
    fn wrap_long_test() {
        let input = "\
CHAPTER 1
Loomings

Call me Ishmael. Some years ago- never mind how long precisely- having little or no money in my purse, and nothing particular to interest me on shore, I thought I would sail about a little and see the watery part of the world. It is a way I have of driving off the spleen and regulating the circulation. Whenever I find myself growing grim about the mouth; whenever it is a damp, drizzly November in my soul; whenever I find myself involuntarily pausing before coffin warehouses, and bringing up the rear of every funeral I meet; and especially whenever my hypos get such an upper hand of me, that it requires a strong moral principle to prevent me from deliberately stepping into the street, and methodically knocking people's hats off- then, I account it high time to get to sea as soon as I can. This is my substitute for pistol and ball. With a philosophical flourish Cato throws himself upon his sword; I quietly take to the ship. There is nothing surprising in this. If they but knew it, almost all men in their degree, some time or other, cherish very nearly the same feelings towards the ocean with me.

There now is your insular city of the Manhattoes, belted round by wharves as Indian isles by coral reefs- commerce surrounds it with her surf. Right and left, the streets take you waterward. Its extreme downtown is the battery, where that noble mole is washed by waves, and cooled by breezes, which a few hours previous were out of sight of land. Look at the crowds of water-gazers there.

Circumambulate the city of a dreamy Sabbath afternoon. Go from Corlears Hook to Coenties Slip, and from thence, by Whitehall, northward. What do you see?- Posted like silent sentinels all around the town, stand thousands upon thousands of mortal men fixed in ocean reveries. Some leaning against the spiles; some seated upon the pier-heads; some looking over the bulwarks of ships from China; some high aloft in the rigging, as if striving to get a still better seaward peep. But these are all landsmen; of week days pent up in lath and plaster- tied to counters, nailed to benches, clinched to desks. How then is this? Are the green fields gone? What do they here?";

        let output_expected = "\
CHAPTER 1
Loomings

Call me Ishmael. Some years ago- never mind how long precisely- having little or
no money in my purse, and nothing particular to interest me on shore, I thought
I would sail about a little and see the watery part of the world. It is a way I
have of driving off the spleen and regulating the circulation. Whenever I find
myself growing grim about the mouth; whenever it is a damp, drizzly November in
my soul; whenever I find myself involuntarily pausing before coffin warehouses,
and bringing up the rear of every funeral I meet; and especially whenever my
hypos get such an upper hand of me, that it requires a strong moral principle to
prevent me from deliberately stepping into the street, and methodically knocking
people's hats off- then, I account it high time to get to sea as soon as I can.
This is my substitute for pistol and ball. With a philosophical flourish Cato
throws himself upon his sword; I quietly take to the ship. There is nothing
surprising in this. If they but knew it, almost all men in their degree, some
time or other, cherish very nearly the same feelings towards the ocean with me.

There now is your insular city of the Manhattoes, belted round by wharves as
Indian isles by coral reefs- commerce surrounds it with her surf. Right and
left, the streets take you waterward. Its extreme downtown is the battery, where
that noble mole is washed by waves, and cooled by breezes, which a few hours
previous were out of sight of land. Look at the crowds of water-gazers there.

Circumambulate the city of a dreamy Sabbath afternoon. Go from Corlears Hook to
Coenties Slip, and from thence, by Whitehall, northward. What do you see?-
Posted like silent sentinels all around the town, stand thousands upon thousands
of mortal men fixed in ocean reveries. Some leaning against the spiles; some
seated upon the pier-heads; some looking over the bulwarks of ships from China;
some high aloft in the rigging, as if striving to get a still better seaward
peep. But these are all landsmen; of week days pent up in lath and plaster- tied
to counters, nailed to benches, clinched to desks. How then is this? Are the
green fields gone? What do they here?";

        let output_actual = wrap(input, 80);

        assert_eq!(output_expected, output_actual.as_str());
        assert!(output_actual.lines().all(|line| line.len() <= 80));
        assert!(output_actual.lines().any(|line| line.len() == 80));
    }
}
