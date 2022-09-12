use std::{fmt, io};

pub enum Value {
    String(String),
}

fn read(input: &str) -> &str {
    input
}

fn eval(input: &str) -> &str {
    input
}

fn print(input: &str) -> Value {
    Value::String(input.to_string())
}

impl fmt::Debug for Value{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(arg0) => <String as fmt::Debug>::fmt(arg0, f),
        }
    }
}

pub fn repl() {
    for line in io::stdin().lines() {
	let line = line.unwrap();
	let line = line.trim();
	match line {
	    "quit" | "exit" => break,
	    line => println!("{:?}", print(eval(read(line)))),
	}
    }
}

fn main() {
    repl();
}
