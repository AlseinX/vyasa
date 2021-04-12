use std::{
    fmt::{Debug, Display},
    ops::Range,
};

#[derive(Clone)]
pub struct Block(pub Vec<Expr>);

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(&self.0).finish()
    }
}

#[derive(Clone)]
pub enum Expr {
    LitNum(f64, Range<usize>),
    LitStr(String, Range<usize>),
    LastLine(Range<usize>),
    Var(VarDef),
    Block(Block),
    Operation(Box<Operation>),
    Call(Ident, Vec<Expr>),
    Func(Box<(Vec<VarDef>, Option<Ident>, Expr)>),
}

impl std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Expr::LitNum(n, _) => Debug::fmt(n, f),
            Expr::LitStr(s, _) => Debug::fmt(s, f),
            Expr::LastLine(_) => Display::fmt("@", f),
            Expr::Var(v) => Debug::fmt(v, f),
            Expr::Block(Block(exprs)) => f.debug_list().entries(exprs).finish(),
            Expr::Operation(o) => Debug::fmt(o.as_ref(), f),
            Expr::Call(Ident(name, _), args) => {
                Display::fmt(name, f)?;
                Display::fmt("(", f)?;
                for (i, arg) in args.iter().enumerate() {
                    Debug::fmt(arg, f)?;
                    if i < args.len() - 1 {
                        Display::fmt(", ", f)?;
                    }
                }
                Display::fmt(")", f)?;
                Ok(())
            }
            Expr::Func(func) => {
                let (args, ret, body) = func.as_ref();
                Display::fmt("(", f)?;
                for (i, arg) in args.iter().enumerate() {
                    Debug::fmt(arg, f)?;
                    if i < args.len() - 1 {
                        Display::fmt(", ", f)?;
                    }
                }
                Display::fmt(")", f)?;
                if let Some(Ident(ret, _)) = ret {
                    Display::fmt(": ", f)?;
                    Debug::fmt(ret, f)?;
                }
                Display::fmt(" => ", f)?;
                Debug::fmt(body, f)?;
                Ok(())
            }
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Clone)]
pub enum Operation {
    If(Expr, Expr),
    IfElse(Expr, Expr, Expr),
    While(Expr, Expr),
    Add(Expr, Expr),
    Sub(Expr, Expr),
    Mul(Expr, Expr),
    Div(Expr, Expr),
    BitAnd(Expr, Expr),
    BitOr(Expr, Expr),
    EQ(Expr, Expr),
    NE(Expr, Expr),
    GT(Expr, Expr),
    LT(Expr, Expr),
    GE(Expr, Expr),
    LE(Expr, Expr),
    And(Expr, Expr),
    Or(Expr, Expr),
    Assign(Expr, Expr),
}

impl Debug for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::If(c, b) => {
                Display::fmt("(", f)?;
                Debug::fmt(c, f)?;
                Display::fmt("? ", f)?;
                Debug::fmt(b, f)?;
                Display::fmt(")", f)?;
                Ok(())
            }
            Operation::IfElse(c, b, e) => {
                Display::fmt("(", f)?;
                Debug::fmt(c, f)?;
                Display::fmt("? ", f)?;
                Debug::fmt(b, f)?;
                Display::fmt("!? ", f)?;
                Debug::fmt(e, f)?;
                Display::fmt(")", f)?;
                Ok(())
            }
            Operation::While(c, b) => {
                Display::fmt("(", f)?;
                Debug::fmt(c, f)?;
                Display::fmt("^ ", f)?;
                Debug::fmt(b, f)?;
                Display::fmt(")", f)?;
                Ok(())
            }
            Operation::Add(l, r) => {
                Display::fmt("(", f)?;
                Debug::fmt(l, f)?;
                Display::fmt(" + ", f)?;
                Debug::fmt(r, f)?;
                Display::fmt(")", f)?;
                Ok(())
            }
            Operation::Sub(l, r) => {
                Display::fmt("(", f)?;
                Debug::fmt(l, f)?;
                Display::fmt(" - ", f)?;
                Debug::fmt(r, f)?;
                Display::fmt(")", f)?;
                Ok(())
            }
            Operation::Mul(l, r) => {
                Display::fmt("(", f)?;
                Debug::fmt(l, f)?;
                Display::fmt(" * ", f)?;
                Debug::fmt(r, f)?;
                Display::fmt(")", f)?;
                Ok(())
            }
            Operation::Div(l, r) => {
                Display::fmt("(", f)?;
                Debug::fmt(l, f)?;
                Display::fmt(" / ", f)?;
                Debug::fmt(r, f)?;
                Display::fmt(")", f)?;
                Ok(())
            }
            Operation::BitAnd(l, r) => {
                Display::fmt("(", f)?;
                Debug::fmt(l, f)?;
                Display::fmt(" & ", f)?;
                Debug::fmt(r, f)?;
                Display::fmt(")", f)?;
                Ok(())
            }
            Operation::BitOr(l, r) => {
                Display::fmt("(", f)?;
                Debug::fmt(l, f)?;
                Display::fmt(" & ", f)?;
                Debug::fmt(r, f)?;
                Display::fmt(")", f)?;
                Ok(())
            }
            Operation::EQ(l, r) => {
                Display::fmt("(", f)?;
                Debug::fmt(l, f)?;
                Display::fmt(" == ", f)?;
                Debug::fmt(r, f)?;
                Display::fmt(")", f)?;
                Ok(())
            }
            Operation::NE(l, r) => {
                Display::fmt("(", f)?;
                Debug::fmt(l, f)?;
                Display::fmt(" != ", f)?;
                Debug::fmt(r, f)?;
                Display::fmt(")", f)?;
                Ok(())
            }
            Operation::GT(l, r) => {
                Display::fmt("(", f)?;
                Debug::fmt(l, f)?;
                Display::fmt(" > ", f)?;
                Debug::fmt(r, f)?;
                Display::fmt(")", f)?;
                Ok(())
            }
            Operation::LT(l, r) => {
                Display::fmt("(", f)?;
                Debug::fmt(l, f)?;
                Display::fmt(" < ", f)?;
                Debug::fmt(r, f)?;
                Display::fmt(")", f)?;
                Ok(())
            }
            Operation::GE(l, r) => {
                Display::fmt("(", f)?;
                Debug::fmt(l, f)?;
                Display::fmt(" <= ", f)?;
                Debug::fmt(r, f)?;
                Display::fmt(")", f)?;
                Ok(())
            }
            Operation::LE(l, r) => {
                Display::fmt("(", f)?;
                Debug::fmt(l, f)?;
                Display::fmt(" >= ", f)?;
                Debug::fmt(r, f)?;
                Display::fmt(")", f)?;
                Ok(())
            }
            Operation::And(l, r) => {
                Display::fmt("(", f)?;
                Debug::fmt(l, f)?;
                Display::fmt(" && ", f)?;
                Debug::fmt(r, f)?;
                Display::fmt(")", f)?;
                Ok(())
            }
            Operation::Or(l, r) => {
                Display::fmt("(", f)?;
                Debug::fmt(l, f)?;
                Display::fmt(" || ", f)?;
                Debug::fmt(r, f)?;
                Display::fmt(")", f)?;
                Ok(())
            }
            Operation::Assign(l, r) => {
                Display::fmt("(", f)?;
                Debug::fmt(l, f)?;
                Display::fmt(" = ", f)?;
                Debug::fmt(r, f)?;
                Display::fmt(")", f)?;
                Ok(())
            }
        }
    }
}

#[derive(Clone)]
pub struct Ident(pub String, pub Range<usize>);

impl Debug for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

#[derive(Clone)]
pub struct VarDef(pub Ident, pub Option<Ident>);

impl Debug for VarDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)?;
        if let Some(Ident(t, _)) = &self.1 {
            Display::fmt(": ", f)?;
            Display::fmt(t, f)?;
        }
        Ok(())
    }
}
