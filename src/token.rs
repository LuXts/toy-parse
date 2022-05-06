use bigdecimal::BigDecimal;
use once_cell::sync::Lazy;
use std::collections::HashMap;
type Num = BigDecimal;

// u8(C++ 中的 char) 和 TokenInfo 的对应表，包含了默认符号
static DEFAULE_TOKEN_LIST: Lazy<HashMap<u8, TokenInfo>> = Lazy::new(|| {
    let mut data: HashMap<u8, TokenInfo> = HashMap::new();
    data.insert(b' ', TokenInfo::Symbol(SymbolType::Blank));
    data.insert(b'(', TokenInfo::Symbol(SymbolType::LeftBracket));
    data.insert(b')', TokenInfo::Symbol(SymbolType::RightBracket));
    data.insert(b'+', TokenInfo::Func(FuncType::Add));
    data.insert(b'-', TokenInfo::Func(FuncType::Sub));
    data.insert(b'*', TokenInfo::Func(FuncType::Mul));
    data.insert(b'/', TokenInfo::Func(FuncType::Div));
    data
});

#[derive(Debug)]
pub struct Token {
    pub position: usize,
    pub info: TokenInfo,
    pub original: String,
}

#[derive(Debug, Clone)]
pub enum TokenInfo {
    Number(Num),        // 数字
    Func(FuncType),     // 函数
    Symbol(SymbolType), // 符号
}

// 函数类型
#[derive(Debug, Clone)]
pub enum FuncType {
    Add, // 加
    Sub, // 减
    Mul, // 乘
    Div, // 除
}

// 符号类型
#[derive(Debug, Clone)]
pub enum SymbolType {
    LeftBracket,  // 左括号
    RightBracket, // 右括号
    Blank,        // 空格
}

#[derive(Debug)]
pub struct TokenParseErr {
    pub reason: String,  // 错误原因
    pub position: usize, // 错误发生的位置
}

pub fn parse_number_token(
    input: &str,
    start_position: usize,
    end_position: usize,
) -> Result<Token, TokenParseErr> {
    fn parse_number(input: &str) -> Result<Num, String> {
        match input.parse() {
            Ok(n) => Ok(n),
            Err(_) => Err(format!("遇到未预期的输入: '{}' ！", input).to_owned()),
        }
    }
    match parse_number(&input[start_position..end_position]) {
        Ok(n) => Ok(Token {
            position: (start_position + 1) as usize,
            info: TokenInfo::Number(n),
            original: input[start_position..end_position].to_string(),
        }),
        Err(e) => Err(TokenParseErr {
            reason: e,
            position: start_position,
        }),
    }
}

pub fn parse_token(input: &str) -> Result<Vec<Token>, TokenParseErr> {
    let mut tokens = vec![]; // 解析结果
    let mut parsed_position: usize = 0;
    let input_vec = input.as_bytes();
    let mut parsing_number = false; // 是否正在解析数字
    for current_position in 0..input_vec.len() {
        let item = input_vec[current_position];
        if let Some(token) = DEFAULE_TOKEN_LIST.get(&item) {
            if parsing_number {
                match parse_number_token(input, parsed_position, current_position) {
                    Ok(item) => tokens.push(item),
                    Err(e) => {
                        return Err(e);
                    }
                };
                parsing_number = false;
            }
            match token {
                TokenInfo::Symbol(SymbolType::Blank) => {
                    // 不解析空格
                }
                _ => {
                    tokens.push(Token {
                        position: current_position,
                        info: token.clone(),
                        original: input[current_position..current_position + 1].to_owned(),
                    });
                }
            }
            parsed_position = current_position + 1;
        } else {
            parsing_number = true;
        }
    }
    // 如果结束的时候还在解析数字状态，就把剩下的所有字符都丢进去解析。
    if parsing_number {
        match parse_number_token(input, parsed_position, input.len()) {
            Ok(item) => tokens.push(item),
            Err(e) => {
                return Err(e);
            }
        };
    } else if tokens.is_empty() {
        // 如果最后解析的 tokens 里面什么都没有，那就代表输入的表达式完全无效
        return Err(TokenParseErr {
            reason: "没有输入任何有效表达式！".to_owned(),
            position: 0,
        });
    }
    Ok(tokens)
}

// 单元测试
#[cfg(test)]
mod test {
    use super::parse_token;

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
            let re = parse_token(input_vec[i]);
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
            let re = parse_token(input_vec[i]);
            assert!(re.is_err());
            if let Err(e) = re {
                assert!(e.position == index_vec[i]);
            }
        }
    }
}
