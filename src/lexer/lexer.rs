use crate::lexer::token;
use crate::utils::{is_digit, is_letter};

#[derive(Debug, Clone)]
pub struct Lexer {
    input: String,
    current_char: String,
    pos: i32,
    read_pos: i32,
    line: u8,
    column: u8,
}

pub static EMPTY_STR: &'static str = "";

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            input,
            current_char: EMPTY_STR.to_string(),
            pos: 0,
            read_pos: 1,
            line: 1,
            column: 0,
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        self.current_char = self.get_char(self.pos);
        self.pos = self.read_pos;
        self.column += 1;
        self.read_pos += 1;
    }

    fn get_char(&self, pos: i32) -> String {
        match self.input.chars().nth(pos as usize) {
            Some(x) => x.to_string(),
            None => EMPTY_STR.to_string(),
        }
    }

    fn peek_char(&self) -> String {
        self.get_char(self.read_pos - 1)
    }

    pub fn next_token(&mut self) -> token::Token {
        self.skip_whitespace();
        let current_char = &self.current_char.clone();
        let start_pos = self.column;
        let is_string = current_char == "\"";
        let s = match current_char {
            x if is_letter(x) => self.read_identifier(),
            x if is_digit(x) => self.read_digit(),
            x if x == "\"" => {
                self.read_string()
            },
            x if x == "=" && self.peek_char() == "=" => {
                self.read_char();
                self.read_char();
                format!("{}{}", x, "=")
            },
            x if x == "!" && self.peek_char() == "=" => {
                self.read_char();
                self.read_char();
                format!("{}{}", x, "=")
            },
            x => {
                self.read_char();
                x.clone()
            },
        };

        token::Token::new(s, is_string, self.line, start_pos)
    }

    fn read_string(&mut self) -> String {
        self.read_char();
        let start = (self.pos - 1) as usize;
        while self.current_char != "\"" {
            self.read_char();
        }

        let input_chars: Vec<char> = self.input.chars().collect();
        let end = (self.pos - 1) as usize;
        self.read_char();

        (&input_chars[start..end])
            .iter()
            .fold("".to_string(), |acc, &s| { format!("{}{}", acc, s.to_string()) })
    }

    fn read_identifier(&mut self) -> String {
        let start = (self.pos - 1) as usize;
        while is_letter(&self.current_char) {
            self.read_char();
        }

        let input_chars: Vec<char> = self.input.chars().collect();
        let end = (self.pos - 1) as usize;
        (&input_chars[start..end]).iter().fold("".to_string(), |acc, &s| {format!("{}{}", acc, s.to_string())})
    }

    fn read_digit(&mut self) -> String {
        let start = (self.pos - 1) as usize;

        while is_digit(&self.current_char) {
            self.read_char();
        }

        let input_chars: Vec<char> = self.input.chars().collect();
        let end = (self.pos - 1) as usize;
        (&input_chars[start..end]).iter().fold("".to_string(), |acc, &s| { format!("{}{}", acc, s.to_string()) })
    }

    fn skip_whitespace(&mut self) {
        let (is_whitespace, is_newline) = match self.current_char.chars().last() {
            Some(x) => {
                (x.is_whitespace(), x == '\n')
            },
            _ => (false, false),
        };
        if is_newline {
            self.column = 0;
            self.line += 1;
        }
        if is_whitespace {
            self.read_char();
            self.skip_whitespace();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use self::token::TokenType::*;

    #[test]
    fn it_should_analysis_control_syntax() {
        let mut l = Lexer::new("
        if (5 < 10) {
            return true;
        } else {
            return false;
        }
        ".to_string());
        let expects = vec![
            (IF, "if"),
            (LPAREN, "("),
            (INT("5".to_string()), "5"),
            (LT, "<"),
            (INT("10".to_string()), "10"),
            (RPAREN, ")"),
            (LBRACE, "{"),
            (RETURN, "return"),
            (TRUE, "true"),
            (SEMICOLON, ";"),
            (RBRACE, "}"),
            (ELSE, "else"),
            (LBRACE, "{"),
            (RETURN, "return"),
            (FALSE, "false"),
            (SEMICOLON, ";"),
            (RBRACE, "}"),
            (EOF, "")
        ];

        for (token_type, literal) in expects {
            let t = l.next_token();
            assert_eq!(t.token_type, token_type);
            assert_eq!(t.literal, literal);
        }
    }
}