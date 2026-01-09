#[cfg(test)]
mod tests;

mod ast;
mod lexer;
mod parser;
mod repl;
mod token;

fn main() {
    println!("WELCOME TO THE MONKEY LANGUAGE REPL!");
    println!("TYPE ANY COMMAND TO GET STARTED");

    repl::start();
}
