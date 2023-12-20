#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    String(String), // 文字列
    LeftBracket,    // [
    RightBracket,   // ]
    NewLine,        // \n
    Tail,           // ファイル末尾
}

pub struct Lexer<'a> {
    /// 読み込み中の先頭文字列を指す
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Lexer<'a> {
    /// 文字列を受け取りTokenを返す
    pub fn new(input: &str) -> Lexer {
        Lexer {
            chars: input.chars().peekable(),
        }
    }

    /// 文字列を`Token`単位に分割をする
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = vec![];
        while let Some(token) = self.next_token()? {
            match token {
                // 改行は今回は捨てる
                Token::NewLine => {}
                _ => {
                    tokens.push(token);
                }
            }
        }

        tokens.push(Token::Tail);
        Ok(tokens)
    }

    /// 一文字分だけ読み進め、tokenを返す
    fn next_return_token(&mut self, token: Token) -> Option<Token> {
        self.chars.next();
        Some(token)
    }

    /// 文字列を読み込み、マッチしたTokenを返す
    fn next_token(&mut self) -> Result<Option<Token>, LexerError> {
        // 先頭の文字列を読み込む
        match self.chars.peek() {
            Some(c) => match c {
                // 一文字分だけ読み進め、Tokenを返す
                // NewLineは'\n'
                c if c.is_whitespace() || *c == '\n' => {
                    Ok(self.next_return_token(Token::NewLine))
                }
                '[' => Ok(self.next_return_token(Token::LeftBracket)),
                ']' => Ok(self.next_return_token(Token::RightBracket)),
                '"' => {
                    // parse string
                    self.chars.next();
                    self.parse_string_token()
                },
                _ => Err(LexerError::new(&format!("error: an unexpected char {}", c))),
            },
            None => Ok(None),
        }
    }

    /// 終端文字'\"'まで文字列を読み込む。UTF-16(\u0000~\uFFFF)や特殊なエスケープ文字(e.g. '\t','\n')も考慮する
    fn parse_string_token(&mut self) -> Result<Option<Token>, LexerError> {
        let mut utf16 = vec![];
        let mut result = String::new();

        while let Some(c1) = self.chars.next() {
            match c1 {
                // Escapeの開始文字'\\'
                '\\' => {
                    // 次の文字を読み込む
                    let c2 = self
                        .chars
                        .next()
                        .ok_or_else(|| LexerError::new("error: a next char is expected"))?;
                    if matches!(c2, '"' | '\\' | '/' | 'b' | 'f' | 'n' | 'r' | 't') {
                        // 特殊なエスケープ文字列の処理
                        // https://www.rfc-editor.org/rfc/rfc8259#section-7
                        // utf16のバッファを文字列にpushしておく
                        Self::push_utf16(&mut result, &mut utf16)?;
                        // 今回はエスケープ処理はせずに入力のまま保存する
                        result.push('\\');
                        result.push(c2);
                    } else if c2 == 'u' {
                        // UTF-16
                        // \u0000 ~ \uFFFF
                        // \uまで読み込んだので残りの0000~XXXXの4文字を読み込む
                        // UTF-16に関してはエスケープ処理を行う
                        let hexs = (0..4)
                            .filter_map(|_| {
                                let c = self.chars.next()?;
                                if c.is_ascii_hexdigit() {
                                    Some(c)
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<_>>();

                        // 読み込んだ文字列を16進数として評価しutf16のバッファにpushしておく
                        match u16::from_str_radix(&hexs.iter().collect::<String>(), 16) {
                            Ok(code_point) => utf16.push(code_point),
                            Err(e) => {
                                return Err(LexerError::new(&format!(
                                    "error: a unicode character is expected {}",
                                    e.to_string()
                                )))
                            }
                        };
                    } else {
                        return Err(LexerError::new(&format!(
                            "error: an unexpected escaped char {}",
                            c2
                        )));
                    }
                }
                // 文字列の終端
                '\"' => {
                    // utf16のバッファを文字列にpushしておく
                    Self::push_utf16(&mut result, &mut utf16)?;
                    return Ok(Some(Token::String(result)));
                }
                // それ以外の文字列
                _ => {
                    // utf16のバッファを文字列にpushしておく
                    Self::push_utf16(&mut result, &mut utf16)?;
                    result.push(c1);
                }
            }
        }
        Ok(None)
    }

    /// utf16のバッファが存在するならば連結しておく
    fn push_utf16(result: &mut String, utf16: &mut Vec<u16>) -> Result<(), LexerError> {
        if utf16.is_empty() {
            return Ok(());
        }
        match String::from_utf16(utf16) {
            Ok(utf16_str) => {
                result.push_str(&utf16_str);
                utf16.clear();
            }
            Err(e) => {
                return Err(LexerError::new(&format!("error: {}", e.to_string())));
            }
        };
        Ok(())
    }

}

#[derive(Debug)]
pub struct LexerError {
    pub msg: String,
}

impl LexerError {
    fn new(msg: &str) -> LexerError {
        LexerError {
            msg: msg.to_string(),
        }
    }
}