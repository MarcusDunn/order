use std::str::CharIndices;

use crate::lexer::Token::{Colon, DoubleDash, Equals, LeftParen, NewLine, RightParen, SkinnyArrow, Space, Tab};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Error {
    pub location: usize,
    pub code: ErrorCode,
}

#[non_exhaustive]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ErrorCode {
    UnexpectedEndOfToken(&'static str)
}

#[derive(Debug, Copy, Clone)]
pub enum Token<'input> {
    // data
    Data,
    // where
    Where,

    // ->
    SkinnyArrow,
    // :
    Colon,
    // =
    Equals,
    // \n
    NewLine,
    //
    Space,
    // \t
    Tab,
    // --
    DoubleDash,
    // a series of identifiers (could be a pattern or a action)
    IdentifierSeries(&'input str),
    // before the `:` or after `data` (or in a pattern match of a function branch)
    Identifier(&'input str),
    // a literal number
    NumberLiteral(&'input str),
    // (
    LeftParen,
    // )
    RightParen,
}

pub struct Tokenizer<'input> {
    text: &'input str,
    chars: CharIndices<'input>,
    lookahead: Option<(usize, char)>,
    shift: usize,
}

pub struct Spanned<T> {
    start: usize,
    value: T,
    end: usize,
}

impl<T> From<(usize, T, usize)> for Spanned<T> {
    fn from(tup: (usize, T, usize)) -> Self {
        Spanned {
            start: tup.0,
            value: tup.1,
            end: tup.2,
        }
    }
}

impl<'input> Iterator for Tokenizer<'input> {
    type Item = Result<Spanned<Token<'input>>, Error>;

    fn next(&mut self) -> Option<Result<Spanned<Token<'input>>, Error>> {
        match self.next_unshifted() {
            None => None,
            Some(Ok(Spanned { start, value, end })) => Some(Ok(Spanned { start, value, end })),
            Some(Err(Error { location, code })) => Some(Err(Error {
                location: location + self.shift,
                code,
            })),
        }
    }
}

impl<'input> Tokenizer<'input> {
    pub fn new(text: &str, shift: usize) -> Tokenizer {
        Tokenizer {
            text,
            chars: text.char_indices(),
            lookahead: None,
            shift,
        }
    }

    fn next_unshifted(&mut self) -> Option<Result<Spanned<Token<'input>>, Error>> {
        loop {
            return match self.lookahead {
                Some((idx0, c)) => Some({
                    match c {
                        '\n' => Ok(Spanned {
                            start: idx0,
                            value: NewLine,
                            end: idx0 + 1,
                        }),
                        ' ' => Ok(
                            Spanned {
                                start: idx0,
                                value: Space,
                                end: idx0 + 1,
                            }
                        ),
                        ':' => Ok(Spanned {
                            start: idx0,
                            value: Colon,
                            end: idx0 + 1,
                        }),
                        '=' => Ok(
                            Spanned {
                                start: idx0,
                                value: Equals,
                                end: idx0 + 1,
                            }
                        ),
                        '\t' => Ok(
                            Spanned {
                                start: idx0,
                                value: Tab,
                                end: idx0 + 1,
                            }
                        ),
                        '(' => Ok(
                            Spanned {
                                start: idx0,
                                value: LeftParen,
                                end: idx0 + 1,
                            }
                        ),
                        ')' => Ok(
                            Spanned {
                                start: idx0,
                                value: RightParen,
                                end: idx0 + 1,
                            }
                        ),
                        '-' => match self.bump() {
                            Some((idx1, c)) => {
                                match c {
                                    '>' => Ok(Spanned {
                                        start: idx0,
                                        value: SkinnyArrow,
                                        end: idx1 + 1,
                                    }),
                                    '-' => Ok(Spanned {
                                        start: idx0,
                                        value: DoubleDash,
                                        end: idx1 + 1,
                                    }),
                                    _ => Err(Error { location: idx0, code: ErrorCode::UnexpectedEndOfToken("expected '-' to be followed by '>' or '-' but found something else") })
                                }
                            }
                            None => Err(Error { location: idx0, code: ErrorCode::UnexpectedEndOfToken("expected '-' to be followed by '>' or '-' but found nothing") })
                        }

                        c => self.identifierish(c)
                    }
                }),
                None => None,
            };
        }
    }

    fn bump(&mut self) -> Option<(usize, char)> {
        self.lookahead = self.chars.next();
        self.lookahead
    }

    fn identifierish(&self, current: char) -> Result<Spanned<Token<'input>>, Error> {
        panic!()
    }
}

const KEYWORDS: &[(&str, Token<'static>)] = &[
    ("data", Token::Data),
    ("where", Token::Where),
];