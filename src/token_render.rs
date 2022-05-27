use crate::sentence::ParseErr;
use crate::sentence::ParseErrType;
use crate::token::*;

/**
TokenRender

储存词法分析的结果的结构。

提供相应接口方便语法分析。
 */
pub struct TokenRender {
    tokens: Vec<Token>,
    current_position: usize,
}

impl TokenRender {
    /**
    用 `Vec<Token>` 初始化一个 TokenRender。
     */
    pub fn new_with_tokens(tokens: Vec<Token>) -> TokenRender {
        return TokenRender {
            tokens,
            current_position: 0,
        };
    }

    /**
    返回下一个 token 以供测试。

    如果 `self.is_empty() == true` 的话会 `panic`

    */
    pub fn peek<'a>(&'a self) -> &'a Token {
        assert!(!self.is_empty());
        return &self.tokens[self.current_position];
    }

    /**
    消费一个 token 。
     */
    pub fn next(&mut self) {
        self.current_position += 1;
    }

    /**
    检查 peek() 的 token 类型是否和输入的 token_info 一致。

    如果类型不相符会返回 `reason(self.peek())` 。

    如果 `self.is_empty() == true` 返回 `empty_reason` 。

    # Examples

    ```
    render.expect(
        TokenInfo::Symbol(SymbolType::RightBracket),
        |token| {
            format!("期望获得 )，却得到了 '{}' ", token.info).to_owned()
        },
        "期望获得 ) ，却意外终止".to_owned(),
    );
    ```
    */
    pub fn expect(
        &self,
        token_info: TokenInfo,
        reason: fn(&Token) -> String,
        empty_reason: String,
    ) -> Result<Token, ParseErr> {
        if !self.is_empty() {
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
                reason: reason(token),
                err_type: ParseErrType::Unexpected(token.to_owned()),
            });
        } else {
            return Err(ParseErr {
                reason: empty_reason,
                err_type: ParseErrType::Insufficient,
            });
        }
    }

    /**
    代替以下代码：

    ```
    if token_info == self.peek().info {
        self.next();
        return true;
    }else{
        return false;
    }
    ```
    */
    pub fn try_token(&mut self, token_info: TokenInfo) -> bool {
        if !self.is_empty() {
            let token = self.peek();
            match token_info {
                TokenInfo::Number(_) => {
                    if let TokenInfo::Number(_) = token.info {
                        self.next();
                        return true;
                    }
                }
                TokenInfo::Symbol(rhs) => {
                    if let TokenInfo::Symbol(lhs) = &token.info {
                        if rhs == *lhs {
                            self.next();
                            return true;
                        }
                    }
                }
            }
        }
        return false;
    }

    /** 检查 TokenRender 是否已经为空。 */
    pub fn is_empty(&self) -> bool {
        if self.current_position >= self.tokens.len() {
            return true;
        } else {
            return false;
        }
    }
}
