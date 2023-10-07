use super::app::{AppMeta, CommandMatches};
use crate::utils::CaseInsensitiveStr;
use caith::Roller;

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

pub async fn parse<'a, P, T>(
    app_meta: &AppMeta,
    input: &'a str,
    tokens: &'a [Token],
    parse_callback: P,
) -> CommandMatches<T>
where
    P: Fn(&[ParsedToken<'a>]) -> CommandMatches<T>,
{
    let mut input_remainder = input;
    let mut parsed_tokens = Vec::with_capacity(tokens.len());

    for token in tokens {
        if let TokenMatch::Full { token, remainder } = token.parse(app_meta, input_remainder).await
        {
            input_remainder = remainder.trim_start();
            parsed_tokens.push(token);
        } else {
            return CommandMatches::default();
        }
    }

    // Verify that we have consumed the entire input.
    if input_remainder.trim().is_empty() {
        parse_callback(&parsed_tokens)
    } else {
        CommandMatches::default()
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
