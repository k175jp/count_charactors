use crate::{lexer::Token, Value};

#[derive(Debug, Clone)]
pub struct ParserError {
    pub msg: String,
}

impl ParserError {
    pub fn new(msg: &str) -> ParserError {
        ParserError {
            msg: msg.to_string(),
        }
    }
}
pub struct Parser {
    /// `Lexer`で`tokenize`した`Token`一覧
    tokens: Vec<Token>,
    /// `tokens`の先頭
    index: usize,
}

impl Parser {
    /// `Token`の一覧を受け取り`Parser`を返す。
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, index: 0 }
    }

    pub fn parse(&mut self) -> Result<Value, ParserError> {
        let token = self.peek_expect()?.clone();
        let value = match token {
            Token::LeftBracket => self.parse_obj(),
            _ => {
                return Err(ParserError::new(&format!(
                    "error: a token must start [ {:?}",
                    token
                )))
            }
        };
        value
    }

    fn parse_obj(&mut self) -> Result<Value, ParserError> {
        let mut object = std::collections::BTreeMap::new();
        loop {
            let token1 = self.next_expect()?.clone();
            let token2 = self.next_expect()?.clone();
            let token3 = self.next_expect()?.clone();
            match (token1.clone(), token2.clone(), token3.clone()) {
                (Token::LeftBracket, Token::String(value), Token::RightBracket) => {
                    let mut content = String::new();
                    loop {
                        let token4 = self.next_expect()?.clone();
                        let token5 = self.peek_expect()?.clone();
                        match (token4.clone(), token5.clone()) {
                            (Token::String(s), Token::String(_)) => {
                                content += &s;
                            },
                            (Token::String(s), Token::LeftBracket) => {
                                content += &s;
                                break
                            },
                            (Token::String(s), Token::Tail) => {
                                content += &s;
                                object.insert(value,content);
                                return Ok(Value::Object(object));
                            },
                            (_, _) => {
                                return Err(ParserError::new(&format!(
                                    "error: String is required next to [\"title\"] {:?}",
                                    token4
                                )));
                            }
                        }
                    }
                    object.insert(value, content);
                },
                (Token::LeftBracket, Token::String(_), _) => {
                    return Err(ParserError::new(&format!(
                        "error: ] is required next to [\"title\" {:?}",
                        token3
                    )));
                },
                (Token::LeftBracket, _, _) => {
                    return Err(ParserError::new(&format!(
                        "error: String is required next to [ {:?}",
                        token2
                    )));
                },
                (_, _, _) => {
                    return Err(ParserError::new(&format!(
                        "error: a token must start [ {:?}",
                        token1
                    )));
                },
            }
        }
    }

    /// 先頭の`Token`を返す。
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    /// 先頭の`Token`を返す。(先頭に`Token`があることを想定してる)
    fn peek_expect(&self) -> Result<&Token, ParserError> {
        self.peek()
            .ok_or_else(|| ParserError::new("error: a token isn't peekable"))
    }

    /// 先頭の`Token`を返して、1トークン進める。
    fn next(&mut self) -> Option<&Token> {
        self.index += 1;
        self.tokens.get(self.index - 1)
    }

    /// 先頭の`Token`を返して、1トークン進める。(先頭に`Token`があることを想定してる)
    fn next_expect(&mut self) -> Result<&Token, ParserError> {
        self.next()
            .ok_or_else(|| ParserError::new("error: a token isn't peekable"))
    }
}