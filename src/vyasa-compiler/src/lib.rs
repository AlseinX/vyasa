use ast::Block;
use lexer::{arrange, lexer};
use parser::parser;

mod ast;
mod lexer;
mod parser;
mod utils;

pub fn compile<'a>(src: &'a str) -> pom::Result<Block> {
    let chars = Some('\n')
        .into_iter()
        .chain(src.chars())
        .collect::<Vec<_>>();
    let tokens = lexer().parse(chars.as_slice())?;
    let tokens = arrange().parse(tokens.as_ref())?;
    let ast = parser().parse(tokens.as_ref())?;
    Ok(ast)
}

#[cfg(test)]
mod tests;
