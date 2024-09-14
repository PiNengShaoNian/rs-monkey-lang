use std::io::{Stdin, Stdout, Write};

use crate::{evaluator::Evaluator, lexer::Lexer, parser::Parser};

pub fn start(stdin: Stdin, stdout: Stdout) {
    let mut evaluator = Evaluator::new();

    loop {
        let mut out = stdout.lock();
        out.write(b">> ").unwrap();
        out.flush().unwrap();

        let mut line = String::new();

        stdin.read_line(&mut line).expect("Failed to read line");

        let mut parser = Parser::new(Lexer::new(&line));
        let program = parser.parse();
        let errors = parser.get_errors();

        if errors.len() > 0 {
            for err in errors {
                out.write(format!("{}", err).as_bytes()).unwrap();
            }
            out.flush().unwrap();
            continue;
        }

        if let Some(evaluated) = evaluator.eval(program) {
            out.write(format!("{}", evaluated).as_bytes()).unwrap();
            out.write(b"\n").unwrap();
        }

        out.flush().unwrap();
    }
}
