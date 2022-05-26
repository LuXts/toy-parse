use crate::sentence::ParseErr;
use crate::sentence::ParseErrType;
use crate::token::*;

pub struct TokenRender {
    tokens: Vec<Token>,
    current_position: usize,
}

impl<'a> TokenRender {
    pub fn new_with_tokens(tokens: Vec<Token>) -> TokenRender {
        return TokenRender {
            tokens,
            current_position: 0,
        };
    }

    pub fn peek(&'a self) -> &'a Token {
        return &self.tokens[self.current_position];
    }

    pub fn next(&mut self) {
        self.current_position += 1;
    }

    pub fn expect(
        &self,
        token_info: TokenInfo,
        reason: fn(&Token) -> String,
    ) -> Result<Token, ParseErr> {
        let token = self.peek();
        match token_info {
            TokenInfo::Number(_) => {
                if let TokenInfo::Number(_) = token.info {
                    return Ok(token.to_owned());
                }
            }
            TokenInfo::Symbol(rhs) => {
                if let TokenInfo::Symbol(lhs) = &token.info {
                    if rhs == *lhs {
                        return Ok(token.to_owned());
                    }
                }
            }
        }
        return Err(ParseErr {
            reason: reason(&token),
            err_type: ParseErrType::Unexpected(token.to_owned()),
        });
    }

    pub fn try_token(&mut self, token_info: TokenInfo) -> bool {
        let token = self.peek().to_owned();
        match token_info {
            TokenInfo::Number(_) => {
                if let TokenInfo::Number(_) = token.info {
                    self.next();
                    return true;
                }
            }
            TokenInfo::Symbol(rhs) => {
                if let TokenInfo::Symbol(lhs) = token.info {
                    if rhs == lhs {
                        self.next();
                        return true;
                    }
                }
            }
        }
        return false;
    }

    pub fn is_empty(&self) -> bool {
        if self.current_position >= self.tokens.len() {
            return true;
        } else {
            return false;
        }
    }
}
