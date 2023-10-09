use super::app::{AppMeta, CommandMatches};
use crate::utils::CaseInsensitiveStr;
use caith::Roller;
use tokio::task::JoinSet;

pub enum Token {
    AnyNonEmpty,
    DiceFormula,
    Literal(&'static str),
}

pub enum ParsedToken<'a> {
    Text(&'a str),
    DiceFormula(&'a str),
}

enum TokenMatch<'a> {
    Full {
        token: ParsedToken<'a>,
        remainder: &'a str,
    },
    Partial,
    None,
}

pub struct Parser<'a, T>
where
    T: std::fmt::Debug + Send,
{
    app_meta: &'a AppMeta,
    rules: Vec<ParseRule<'a, T>>,
}

impl<'a, T> Parser<'a, T>
where
    T: std::fmt::Debug + Send,
{
    pub fn new(app_meta: &'a AppMeta) -> Self {
        Self {
            app_meta,
            rules: Vec::new(),
        }
    }

    pub fn rule<P: Fn(&[ParsedToken<'a>]) -> CommandMatches<T>>(
        mut self,
        tokens: &'a [Token],
        parse_callback: P,
    ) -> Self {
        self.rules.push(ParseRule {
            tokens,
            parse_callback: Box::new(parse_callback),
        });

        self
    }

    pub async fn parse(&self, input: &'a str) -> CommandMatches<T> {
        let mut join_set = JoinSet::new();
        let mut command_matches = CommandMatches::default();

        for rule in self.rules {
            join_set.spawn(rule.parse(self.app_meta, input));
        }

        while let Some(Ok(result)) = join_set.join_next().await {
            command_matches = command_matches.union(result);
        }

        command_matches
    }
}

struct ParseRule<'a, T> {
    tokens: &'a [Token],
    parse_callback: Box<dyn Fn(&[ParsedToken<'a>]) -> CommandMatches<T>>,
}

impl<'a, T> ParseRule<'a, T> {
    pub async fn parse(&self, app_meta: &AppMeta, input: &'a str) -> CommandMatches<T> {
        let mut input_remainder = input;
        let mut parsed_tokens = Vec::with_capacity(self.tokens.len());

        for token in self.tokens {
            if let TokenMatch::Full { token, remainder } =
                token.parse(app_meta, input_remainder).await
            {
                input_remainder = remainder.trim_start();
                parsed_tokens.push(token);
            } else {
                return CommandMatches::default();
            }
        }

        // Verify that we have consumed the entire input.
        if input_remainder.trim().is_empty() {
            (self.parse_callback)(&parsed_tokens)
        } else {
            CommandMatches::default()
        }
    }
}

impl Token {
    async fn parse<'a>(&'a self, _app_meta: &AppMeta, input: &'a str) -> TokenMatch<'a> {
        match self {
            Token::AnyNonEmpty => {
                if !input.trim().is_empty() {
                    TokenMatch::Full {
                        token: ParsedToken::Text(input),
                        remainder: "",
                    }
                } else {
                    TokenMatch::None
                }
            }
            Token::Literal(s) => {
                if input.starts_with_ci(s) {
                    if input.is_char_boundary(s.len()) {
                        let (a, b) = input.split_at(s.len());
                        TokenMatch::Full {
                            token: ParsedToken::Text(a),
                            remainder: b,
                        }
                    } else {
                        TokenMatch::None
                    }
                } else if s.starts_with_ci(input) {
                    TokenMatch::Partial
                } else {
                    TokenMatch::None
                }
            }
            Token::DiceFormula => {
                if !input.chars().all(|c| c.is_ascii_digit())
                    && Roller::new(input).map_or(false, |r| r.roll().is_ok())
                {
                    TokenMatch::Full {
                        token: ParsedToken::DiceFormula(input),
                        remainder: "",
                    }
                } else {
                    TokenMatch::None
                }
            }
        }
    }
}
