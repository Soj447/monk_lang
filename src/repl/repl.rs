use std::io::{self, Write};
use crate::lexer::{Lexer};
use crate::token::{TokenType};


pub fn repl_start() -> io::Result<()>{
    loop {
        let mut input = String::new();
        print!(">>");
        io::stdout().flush();
        io::stdin().read_line(&mut input)?;
        let mut lexer = Lexer::new(input.trim().to_string());
        let mut token =  lexer.next_token();
        while token.t_type != TokenType::EOF {
            println!("{:?}", token);
            token = lexer.next_token();
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repl() {
        repl_start();
        assert_eq!(1, 1);
    }
}

