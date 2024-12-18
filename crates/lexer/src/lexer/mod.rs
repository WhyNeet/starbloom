pub mod extractors;
pub mod keywords;
pub mod token;

use std::mem;

use token::{Literal, Token};

pub struct Lexer {}

impl Lexer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self, input: &str) -> Vec<Token> {
        let bytes = input.as_bytes();
        let mut tokens = Vec::new();
        let mut pos = 0;

        while pos < bytes.len() {
            let char = bytes[pos] as char;

            if char.is_whitespace() {
                pos += 1;
            } else if char == '\n' {
                pos += 1;
            } else if char.is_ascii_digit() {
                let number = extractors::extract_number(&input[pos..]);
                pos += number.len();
                tokens.push(Token::Literal(Literal::Number(number)));
            } else if char == '"' {
                let string = extractors::extract_string(&input[pos..]);
                pos += string.len() + 2;
                tokens.push(Token::Literal(Literal::String(string)));
            } else if char.is_ascii() {
                let keyword = extractors::extract_keyword(&input[pos..]);
                if let Some(keyword) = keyword {
                    pos += keyword.len();
                    tokens.push(Token::Keyword(keyword));
                    continue;
                }

                let comment = extractors::extract_comment(&input[pos..]);
                if let Some(comment) = comment {
                    pos += comment.len();
                    tokens.push(Token::Comment(comment));
                    continue;
                }

                let operator = extractors::extract_operator(&input[pos..]);
                if let Some(operator) = operator {
                    if operator == "-"
                        && tokens.last().is_some()
                        && mem::discriminant(tokens.last().unwrap())
                            != mem::discriminant(&Token::Literal(Literal::Number("".to_string())))
                        && mem::discriminant(tokens.last().unwrap())
                            != mem::discriminant(&Token::Identifier("".to_string()))
                    {
                        let number = extractors::extract_number(&input[pos..]);
                        if number.len() != 0 {
                            pos += number.len();
                            tokens.push(Token::Literal(Literal::Number(number)));
                            continue;
                        }
                    }

                    pos += operator.len();
                    tokens.push(Token::Operator(operator));
                    continue;
                }

                let punctuation = extractors::extract_punctuation(&input[pos..]);
                if let Some(punctuation) = punctuation {
                    pos += 1;
                    tokens.push(Token::Punctuation(punctuation));
                    continue;
                }

                let extracted_type = extractors::extract_type(&input[pos..]);
                if let Some((extracted_type, len)) = extracted_type {
                    pos += len;
                    tokens.push(Token::Type(extracted_type));
                    continue;
                }

                let boolean = extractors::extract_boolean(&input[pos..]);
                if let Some(boolean) = boolean {
                    pos += boolean.len();
                    tokens.push(Token::Literal(Literal::Boolean(boolean)));
                    continue;
                }

                let identifier = extractors::extract_identifier(&input[pos..]);
                pos += identifier.len();
                tokens.push(Token::Identifier(identifier));
            }
        }

        tokens.push(Token::EOF);

        tokens
    }
}
