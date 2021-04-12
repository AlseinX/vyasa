use pom::parser::{Parser, *};
use std::{fmt::Debug, ops::Range, str::FromStr, usize};

use TokenValue::*;

type TokenParser<'a> = Parser<'a, char, TokenValue>;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenValue {
    Line(usize),
    Ident(String),
    Punct(&'static [char]),
    LitStr(String),
    LitNum(f64),
    BeginBlock,
    EndBlock,
}

impl TokenValue {
    pub fn token(self) -> Token {
        Token(self, 0..0)
    }
}

#[derive(Debug, Clone)]
pub struct Token(pub TokenValue, pub Range<usize>);

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

fn digit<'a>() -> Parser<'a, char, char> {
    one_of("0123456789")
}

fn space<'a>() -> Parser<'a, char, ()> {
    one_of(" \t").repeat(0..).discard()
}

fn line<'a>() -> TokenParser<'a> {
    let new_line = sym('\r') | sym('\n') | sym('\r') - sym('\n');
    let indent = (sym(' ') | sym('\t'))
        .repeat(..)
        .map(|i| Line(i.iter().map(|c| if *c == ' ' { 1 } else { 8 }).sum()));
    (new_line * indent)
        .repeat(1..)
        .map(|c| c.into_iter().last().unwrap())
}

fn ident<'a>() -> TokenParser<'a> {
    let alphabet_ = || {
        is_a(|c| match c {
            'a'..='z' => true,
            'A'..='Z' => true,
            '_' => true,
            _ => false,
        })
    };
    (alphabet_() - (alphabet_() | digit()).repeat(..))
        .collect()
        .map(|s| Ident(s.into_iter().collect()))
}

pub const fn puncts(v: &str) -> &'static [char] {
    let b = v.as_bytes();
    let mut i = 0;
    while i < PUNCTS.len() {
        if b.len() == PUNCTS[i].len() {
            let mut j = 0;
            while j < b.len() {
                if b[j] as char != PUNCTS[i][j] {
                    break;
                }
                j += 1;
            }
            if j == b.len() {
                return PUNCTS[i];
            }
        }
        i += 1;
    }
    return PUNCTS[1 / zero()];
}

const fn zero() -> usize {
    1 - 1
}

const PUNCTS: &[&[char]] = &[
    &['=', '='],
    &['!', '='],
    &['<', '='],
    &['>', '='],
    &['=', '>'],
    &['!', '?'],
    &['&', '&'],
    &['|', '|'],
    &[';'],
    &[':'],
    &['?'],
    &['^'],
    &[','],
    &['('],
    &[')'],
    &['+'],
    &['-'],
    &['*'],
    &['/'],
    &['&'],
    &['|'],
    &['>'],
    &['<'],
    &['='],
    &['.'],
    &['@'],
    &['['],
    &[']'],
    &['{'],
    &['}'],
];

fn punct<'a>() -> TokenParser<'a> {
    let mut result = seq(&PUNCTS[0]).map(|_| Punct(PUNCTS[0]));
    for i in 1..PUNCTS.len() {
        result = result | seq(PUNCTS[i]).map(move |_| Punct(PUNCTS[i]));
    }
    result
}

fn lit_string<'a>() -> TokenParser<'a> {
    let special_char = sym('\\')
        | sym('/')
        | sym('"')
        | sym('b').map(|_| '\x08')
        | sym('f').map(|_| '\x0C')
        | sym('n').map(|_| '\n')
        | sym('r').map(|_| '\r')
        | sym('t').map(|_| '\t');
    let escape_sequence = sym('\\') * special_char;
    let string = sym('"') * (none_of("\\\"") | escape_sequence).repeat(0..) - sym('"');
    string.map(|s| LitStr(s.into_iter().collect()))
}

fn lit_number<'a>() -> TokenParser<'a> {
    let integer = one_of("123456789") - digit().repeat(0..) | sym('0');
    let frac = sym('.') + digit().repeat(1..);
    let exp = one_of("eE") + one_of("+-").opt() + digit().repeat(1..);
    let number = sym('-').opt() + integer + frac.opt() + exp.opt();
    number
        .collect()
        .convert(|s| f64::from_str(s.into_iter().collect::<String>().as_str()).map(LitNum))
}

fn with_pos<'a>(origin: Parser<'a, char, TokenValue>) -> Parser<'a, char, Token> {
    Parser::new(move |input, start| {
        (origin.method)(input, start).map(|(result, end)| (Token(result, start..end), end))
    })
}

pub fn lexer<'a>() -> Parser<'a, char, Vec<Token>> {
    (space().opt() * with_pos(line() | ident() | punct() | lit_string() | lit_number())).repeat(..)
        - space()
        - end()
}

pub fn arrange<'a>() -> Parser<'a, Token, Vec<Token>> {
    Parser::new(|origin: &[Token], pos| {
        let mut result = Vec::new();
        result.reserve((origin.len() as f64 * 1.5) as _);
        let mut levels = Vec::new();
        for Token(token, range) in origin {
            if let &Line(ind) = token {
                let mut last = *levels.last().unwrap_or(&0);
                let pos = range.start;
                if ind > last {
                    result.push(Token(BeginBlock, pos..pos));
                    levels.push(ind);
                } else {
                    while ind < last {
                        levels.pop();
                        last = *levels.last().unwrap_or(&0);
                        result.push(Token(EndBlock, pos..pos));
                    }
                    if ind != last {
                        return Err(pom::Error::Custom {
                            message: "Invalid indention.".to_string(),
                            position: pos,
                            inner: None,
                        });
                    }
                }
                result.push(Token(Line(0), range.clone()));
            } else {
                result.push(Token(token.clone(), range.clone()));
            }
        }

        let last_pos = result.last().unwrap().1.end;
        for _ in levels.into_iter() {
            result.push(Token(EndBlock, last_pos..last_pos));
        }

        Ok((result, pos))
    })
}
