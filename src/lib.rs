#![no_std]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    KeywordWebPage,
    KeywordUIEngine,
    KeywordText,
    KeywordButton,
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(f32),
    OpenBrace,
    CloseBrace,
    Colon,
    Comma,
}

#[derive(Debug, Clone)]
pub enum VoltNode {
    WebPage {
        name: String,
        domain: String,
        children: Vec<VoltNode>,
    },
    UIComponent {
        kind: String,
        name: String,
        properties: Vec<(String, String)>,
    },
}

pub struct VoltLexer<'a> {
    input: &'a str,
}

impl<'a> VoltLexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }

    pub fn tokenize(&self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = self.input.chars().peekable();

        while let Some(&c) = chars.peek() {
            match c {
                ' ' | '\t' | '\r' | '\n' => {
                    chars.next();
                }
                '{' => {
                    tokens.push(Token::OpenBrace);
                    chars.next();
                }
                '}' => {
                    tokens.push(Token::CloseBrace);
                    chars.next();
                }
                ':' => {
                    tokens.push(Token::Colon);
                    chars.next();
                }
                ',' => {
                    tokens.push(Token::Comma);
                    chars.next();
                }
                '"' => {
                    chars.next();
                    let mut str_val = String::new();
                    while let Some(&sc) = chars.peek() {
                        if sc == '"' {
                            chars.next();
                            break;
                        }
                        str_val.push(sc);
                        chars.next();
                    }
                    tokens.push(Token::StringLiteral(str_val));
                }
                a if a.is_alphabetic() || a == '_' => {
                    let mut ident = String::new();
                    while let Some(&ic) = chars.peek() {
                        if ic.is_alphanumeric() || ic == '_' {
                            ident.push(ic);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    match ident.as_str() {
                        "WebPage" => tokens.push(Token::KeywordWebPage),
                        "UIEngine" => tokens.push(Token::KeywordUIEngine),
                        "Text" => tokens.push(Token::KeywordText),
                        "Button" => tokens.push(Token::KeywordButton),
                        _ => tokens.push(Token::Identifier(ident)),
                    }
                }
                _ => {
                    chars.next();
                }
            }
        }
        tokens
    }
}
