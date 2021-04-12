use lazy_static::lazy_static;

use crate::lexer::TokenValue;
use crate::{
    ast::{Block, Expr, Ident, Operation, VarDef},
    lexer::{puncts, Token},
    utils::*,
};
use pom::parser::*;

type Parser<'a, O> = pom::parser::Parser<'a, Token, O>;

#[derive(Debug, Clone, Copy)]
struct Context {
    indent: usize,
}

pub const BIN_OPS: &[(&str, usize)] = &[
    ("?", 14),
    ("!?", 14),
    ("^", 14),
    ("=", 14),
    ("+", 4),
    ("-", 4),
    ("*", 3),
    ("/", 3),
    ("&", 8),
    ("|", 10),
    ("==", 7),
    ("!=", 7),
    (">", 6),
    ("<", 6),
    (">=", 6),
    ("<=", 6),
    ("&&", 11),
    ("||", 12),
];

fn reduce_binary(l: Expr, op: &str, r: Expr) -> Expr {
    Expr::Operation(Box::new(match op {
        "?" => Operation::If(l, r),
        "^" => Operation::While(l, r),
        "=" => Operation::Assign(l, r),
        "+" => Operation::Add(l, r),
        "-" => Operation::Sub(l, r),
        "*" => Operation::Mul(l, r),
        "/" => Operation::Div(l, r),
        "&" => Operation::BitAnd(l, r),
        "|" => Operation::BitOr(l, r),
        "==" => Operation::EQ(l, r),
        "!=" => Operation::NE(l, r),
        ">" => Operation::GT(l, r),
        "<" => Operation::LT(l, r),
        ">=" => Operation::GE(l, r),
        "<=" => Operation::LE(l, r),
        "&&" => Operation::And(l, r),
        "||" => Operation::Or(l, r),
        _ => panic!(),
    }))
}

lazy_static! {
    pub static ref BIN_OP_TOKENS: Vec<Token> = {
        BIN_OPS
            .iter()
            .map(|(s, _)| TokenValue::Punct(puncts(s)).token())
            .collect()
    };
    pub static ref BIN_OP_GROUPS: Vec<(usize, Vec<usize>)> = {
        if let Some(min) = BIN_OPS.iter().map(|x| x.1).min() {
            if let Some(max) = BIN_OPS.iter().map(|x| x.1).max() {
                return (min..=max)
                    .into_iter()
                    .filter_map(|i| {
                        let group = BIN_OPS
                            .iter()
                            .enumerate()
                            .filter_map(|(idx, &(_, p))| if p == i { Some(idx) } else { None })
                            .collect::<Vec<_>>();
                        if group.len() > 0 {
                            Some((i, group))
                        } else {
                            None
                        }
                    })
                    .collect();
            }
        }
        Vec::new()
    };
}

fn peek_before<'a, I, O: 'a, U: 'a>(
    b: pom::parser::Parser<'a, I, U>,
    t: pom::parser::Parser<'a, I, O>,
) -> pom::parser::Parser<'a, I, O> {
    pom::parser::Parser::new(move |target, start| {
        if start > 0 {
            if let Ok(_) = (b.method)(target, start - 1) {
                return (t.method)(target, start);
            }
        }

        Err(pom::Error::Mismatch {
            message: "".to_string(),
            position: start,
        })
    })
}

fn binary<'a>() -> Parser<'a, Expr> {
    let cluster = non_left_recursive()
        + (one_of(&**BIN_OP_TOKENS) + non_left_recursive()
            | peek_before(
                sym(TokenValue::EndBlock.token()),
                sym(TokenValue::Line(0).token()),
            ) * punct("!?")
                + non_left_recursive())
        .repeat(..);
    cluster.map(|(mut first, mut rest)| {
        let mut elses = Vec::new();
        for (pri, group) in &*BIN_OP_GROUPS {
            // Right to left when parsing = ? ^
            let (mut i, step, on_reduce) = if *pri == 14 {
                (rest.len() as isize - 1, -1, -1)
            } else {
                (0isize, 1, 0)
            };
            while i < (rest.len() as _) && i >= 0 {
                if *pri == 14 && rest[i as usize].0 .0 == TokenValue::Punct(puncts("!?")) {
                    elses.push(rest.remove(i as _).1);
                    i += on_reduce;
                } else if let Some(op) = group.iter().find_map(|&j| {
                    if BIN_OP_TOKENS[j] == rest[i as usize].0 {
                        Some(BIN_OPS[j].0)
                    } else {
                        None
                    }
                }) {
                    let last = rest.remove(i as _).1;
                    let first = if i == 0 {
                        &mut first
                    } else {
                        &mut rest[i as usize - 1].1
                    };
                    if op == "?" && elses.len() > 0 {
                        call_replace(first, |first| {
                            Expr::Operation(Box::new(Operation::IfElse(
                                first,
                                last,
                                elses.pop().unwrap(),
                            )))
                        });
                    } else {
                        call_replace(first, |first| reduce_binary(first, op, last));
                    }
                    i += on_reduce;
                } else {
                    i += step;
                }
            }
        }
        first
    })
}

fn is_ident<'a>() -> Parser<'a, Token> {
    is_a(|Token(v, _)| {
        if let TokenValue::Ident(_) = v {
            true
        } else {
            false
        }
    })
}

fn ident<'a>(token: Token) -> Result<Ident, &'static str> {
    if let TokenValue::Ident(result) = token.0 {
        Ok(Ident(result, token.1))
    } else {
        Err("")
    }
}

pub fn punct<'a>(v: &'static str) -> Parser<'a, Token> {
    sym(TokenValue::Punct(puncts(v)).token())
}

fn var_def<'a>() -> Parser<'a, VarDef> {
    let expr = any() + (punct(":") * any()).opt();
    expr.convert::<_, &'static str, _>(|(n, t)| {
        Ok(if let Some(t) = t {
            VarDef(ident(n)?, Some(ident(t)?))
        } else {
            VarDef(ident(n)?, None)
        })
    })
}

fn var<'a>() -> Parser<'a, Expr> {
    var_def().map(|d| Expr::Var(d))
}

fn non_left_recursive<'a>() -> Parser<'a, Expr> {
    func() | var() | last_line() | lit_number() | lit_string() | call(block)
}

pub fn func<'a>() -> Parser<'a, Expr> {
    let args = punct("(") * var_def().repeat(..) - punct(")");
    let ret_type = (punct(":") * is_ident()).opt();
    let body = punct("=>") * call(expr);
    (args + ret_type + body).convert::<_, &'static str, _>(|((args, ret_type), body)| {
        Ok(Expr::Func(Box::new((
            args,
            if let Some(x) = ret_type {
                Some(ident(x)?)
            } else {
                None
            },
            body,
        ))))
    })
}

fn last_line<'a>() -> Parser<'a, Expr> {
    punct("@").map(|Token(_, range)| Expr::LastLine(range))
}

fn lit_string<'a>() -> Parser<'a, Expr> {
    any().convert(|c: Token| {
        if let TokenValue::LitStr(s) = c.0 {
            Ok(Expr::LitStr(s, c.1))
        } else {
            Err("")
        }
    })
}

fn lit_number<'a>() -> Parser<'a, Expr> {
    any().convert(|c: Token| {
        if let TokenValue::LitNum(n) = c.0 {
            Ok(Expr::LitNum(n, c.1))
        } else {
            Err("")
        }
    })
}

fn expr<'a>() -> Parser<'a, Expr> {
    binary()
}

fn indent_block<'a>() -> Parser<'a, Expr> {
    (sym(TokenValue::BeginBlock.token()) * ml_block() - sym(TokenValue::EndBlock.token()))
        .map(|b| Expr::Block(b))
}

fn block<'a>() -> Parser<'a, Expr> {
    let bracketed_block = punct("{") * sym(TokenValue::BeginBlock.token()) * ml_block()
        - sym(TokenValue::EndBlock.token())
        - sym(TokenValue::Line(0).token())
        - punct("}");
    let inline_block = punct("{")
        * (((expr() - punct(";")).repeat(1..) + expr().opt()).map(|(mut v, o)| {
            if let Some(o) = o {
                v.push(o);
            }
            v
        }) | expr().map(|e| vec![e]))
        .map(|v| Block(v))
        - punct("}");
    indent_block() | (bracketed_block | inline_block).map(|b| Expr::Block(b))
}

fn ml_block<'a>() -> Parser<'a, Block> {
    let block = (sym(TokenValue::Line(0).token()) * expr()).repeat(..);
    block.map(|v| Block(v))
}

pub fn parser<'a>() -> Parser<'a, Block> {
    ml_block() - end()
}
