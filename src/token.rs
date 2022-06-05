use bigdecimal::BigDecimal;
use once_cell::sync::Lazy;
use std::{collections::HashMap, fmt};

type Num = BigDecimal;

// u8(C++ 中的 char) 和 TokenInfo 的对应表，包含了默认符号
static DEFAULE_TOKEN_LIST: Lazy<HashMap<u8, TokenInfo>> = Lazy::new(|| {
    let mut data: HashMap<u8, TokenInfo> = HashMap::new();
    data.insert(b' ', TokenInfo::Symbol(SymbolType::Blank));
    data.insert(b'\n', TokenInfo::Symbol(SymbolType::Blank));
    data.insert(b'\t', TokenInfo::Symbol(SymbolType::Blank));

    data.insert(b'(', TokenInfo::Symbol(SymbolType::LeftBracket));
    data.insert(b')', TokenInfo::Symbol(SymbolType::RightBracket));
    data.insert(b'+', TokenInfo::Symbol(SymbolType::Add));
    data.insert(b'-', TokenInfo::Symbol(SymbolType::Sub));
    data.insert(b'*', TokenInfo::Symbol(SymbolType::Mul));
    data.insert(b'/', TokenInfo::Symbol(SymbolType::Div));
    data
});

/**
语言意义上的单个符号 token 。

* `position` 是当前符号在输入串中的起始位置。
* `info` 是当前符号的具体信息。
* `original_str` 是符号原文本。

 */
#[derive(Debug, Clone)]
pub struct Token {
    /// 当前符号在输入串中的起始位置。
    pub position: usize,
    /// 当前符号的具体信息。
    pub info: TokenInfo,
    /// 符号原文本。
    pub original_str: String,
}

/**
符号信息。

* `TokenInfo::Number` 是数字类型。
* `TokenInfo::Symbol` 是符号类型。

 */
#[derive(Debug, Clone)]
pub enum TokenInfo {
    /// 数字类型
    Number(Num),
    /// 符号类型
    Symbol(SymbolType),
}

/**
具体的符号类型。

* `SymbolType::LeftBracket` 是左括号。
* `SymbolType::RightBracket` 是右括号。
* `SymbolType::Blank` 是空白符号。
* `SymbolType::Add` 是加。
* `SymbolType::Sub` 是减。
* `SymbolType::Mul` 是乘。
* `SymbolType::Div` 是除。

*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolType {
    /// 左括号
    LeftBracket,
    /// 右括号
    RightBracket,
    /// 空白
    Blank,
    /// 加
    Add,
    /// 减
    Sub,
    /// 乘
    Mul,
    /// 除
    Div,
}

impl fmt::Display for TokenInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenInfo::Symbol(symbol) => match symbol {
                SymbolType::LeftBracket => write!(f, "左括号"),
                SymbolType::RightBracket => write!(f, "右括号"),
                SymbolType::Blank => write!(f, "空白符号"),
                SymbolType::Add => write!(f, "加号"),
                SymbolType::Sub => write!(f, "减号"),
                SymbolType::Mul => write!(f, "乘号"),
                SymbolType::Div => write!(f, "除号"),
            },
            TokenInfo::Number(n) => {
                write!(f, "数字：{}", n.with_scale(15).normalized().to_string())
            }
        }
    }
}

/**
词法分析器的错误类型。

* `reason` 错误原因
* `position` 错误发生的位置
 */
#[derive(Debug)]
pub struct LexerErr {
    /// 错误原因
    pub reason: String,
    /// 错误发生的位置
    pub position: usize,
    /// 错误原字符串
    pub original_str: String,
}

/**
解析数字
* `input` 数字字符串
* `start_position` 输入字符串在输入串中的位置
*/
fn parse_number_token(input: &str, start_position: usize) -> Result<Token, String> {
    fn parse_number(input: &str) -> Result<Num, String> {
        match input.parse() {
            Ok(n) => Ok(n),
            Err(_) => Err(format!("遇到未预期的输入: '{}' ！", input).to_owned()),
        }
    }
    let n = parse_number(input)?;
    Ok(Token {
        position: start_position,
        info: TokenInfo::Number(n),
        original_str: input.to_string(),
    })
}

/**
符号化字符串/对字符串分词。

* `input` 被分词的字符串
 */
pub fn tokenization(input: &str) -> Result<Vec<Token>, LexerErr> {
    let mut tokens = vec![]; // 存放解析结果
    let mut unparsed_position: usize = 0; // 目前第一个未解析符号

    for current_position in 0..input.len() + 1 {
        // 如果没有到达结尾
        if current_position != input.len() {
            // 当前解析的符号
            let item = input.as_bytes()[current_position];

            // 看看当前解析的符号在不在符号表里面
            if let Some(token) = DEFAULE_TOKEN_LIST.get(&item) {
                // 如果第一个未解析符号不等于当前符号，证明它们之间有一些符号需要尝试解析为数字
                if unparsed_position != current_position {
                    // 解析数字
                    match parse_number_token(
                        &input[unparsed_position..current_position],
                        unparsed_position,
                    ) {
                        Ok(item) => tokens.push(item),
                        Err(reason) => {
                            return Err(LexerErr {
                                reason,
                                position: unparsed_position,
                                original_str: input[unparsed_position..current_position].to_owned(),
                            });
                        }
                    };
                }

                // 根据符号表返回的结果处理
                match token {
                    TokenInfo::Symbol(SymbolType::Blank) => {
                        // 不解析空格
                    }
                    _ => {
                        // 其他符号统统输出
                        tokens.push(Token {
                            position: current_position,
                            info: token.clone(),
                            original_str: input[current_position..current_position + 1].to_owned(),
                        });
                    }
                }

                // 更新第一个未解析符号位置
                unparsed_position = current_position + 1;
            }
        } else {
            // 如果还有需要解析的符号，证明这西符号需要尝试解析为数字
            if unparsed_position != current_position {
                // 解析数字
                match parse_number_token(&input[unparsed_position..], unparsed_position) {
                    Ok(item) => tokens.push(item),
                    Err(reason) => {
                        return Err(LexerErr {
                            reason,
                            position: unparsed_position,
                            original_str: input[unparsed_position..current_position].to_owned(),
                        });
                    }
                };
            }
        }
    }

    Ok(tokens)
}

// 单元测试
#[cfg(test)]
mod test {
    use super::tokenization;

    #[test]
    fn parse_test_success() {
        // 测试符合词法的内容
        let input_vec = vec![
            "56+88-9999",
            "1e9",
            "1.0",
            ".0",
            ".23",
            "1.23e4",
            "()",
            "(",
            ")",
            "67",
            "3/   4",
            "3   4",
            "3   ",
            "34 45",
            "( )",
            "( )34",
            "13.",
            "   1",
        ];
        for i in 0..input_vec.len() {
            let re = tokenization(input_vec[i]);
            assert!(re.is_ok());
        }
    }

    #[test]
    fn parse_test_fail() {
        // 测试不符合词法的内容
        let input_vec = vec![
            "56+88-99a99",
            "1a9",
            "u",
            "1231+67a8",
            "1e",
            "13, + 4",
            "   ",
            "6+12a",
            "12a",
            "12.2e3.4",
        ];
        let index_vec = vec![6, 0, 0, 5, 0, 0, 0, 2, 0, 0];
        assert!(index_vec.len() == input_vec.len());
        for i in 0..index_vec.len() {
            let re = tokenization(input_vec[i]);
            assert!(re.is_err(), "input: {}", input_vec[i]);
            if let Err(e) = re {
                assert!(
                    e.position == index_vec[i],
                    "input: {}, e: {:?}",
                    input_vec[i],
                    e
                );
            }
        }
    }
}
