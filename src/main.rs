mod ast;
mod evaluator;
mod lexer;
mod parser;
mod repl;
mod token;

fn main() {
    repl::start();
}
