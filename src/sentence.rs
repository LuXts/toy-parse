use crate::token::*;
use crate::token_render::*;
use bigdecimal::BigDecimal;
use bigdecimal::Zero;
use std::fmt;

type Num = BigDecimal;
type Position = usize;

/**
语法分析器的错误输出。

* `reason` 错误原因
* `err_type` 错误类型
 */
#[derive(Debug, Clone)]
pub struct ParseErr {
    /// 错误原因
    pub reason: String,
    /// 错误类型
    pub err_type: ParseErrType,
}

/**
语法分析器的错误输出类型。

* `ParseErrType::Unexpected(Token)` 未预期的 `token`
* `ParseErrType::Insufficient` 预期某一个 `token` 但是却突然终止
* `ParseErrType::Redundant(Position)` 被放弃，未能解析的输入

 */
#[derive(Debug, Clone)]
pub enum ParseErrType {
    /// 未预期的 `token`
    Unexpected(Token),
    /// 预期某一个 `token` 但是却突然终止
    Insufficient,
    /// 被放弃，未能解析的输入
    Redundant(Position),
}

/**
Reverse Polish notation RPN 逆波兰式元素。

* `Operator` 运算符
* `Number` 数字
 */
pub enum RPNItem {
    /// 运算符
    Operator(Operator),
    /// 数字
    Number(Num),
}

/**
逆波兰式元素中具体的运算符

* `Operator::Add` 加运算符
* `Operator::Sub` 减运算符
* `Operator::Mul` 乘运算符
* `Operator::Div` 除运算符
* `Operator::Minus` 取负运算符
*/
pub enum Operator {
    /// 加运算符
    Add,
    /// 减运算符
    Sub,
    /// 乘运算符
    Mul,
    /// 除运算符
    Div,
    /// 取负运算符
    Minus,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Sub => write!(f, "-"),
            Operator::Mul => write!(f, "*"),
            Operator::Div => write!(f, "/"),
            Operator::Minus => write!(f, "@"),
        }
    }
}

impl fmt::Display for RPNItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RPNItem::Operator(op) => write!(f, "{}", op),
            RPNItem::Number(n) => {
                write!(f, "{}", n.normalized().to_string())
            }
        }
    }
}

// parse_sentence -> a1#
// a1 -> m1 o1 a | m1
// m1 -> at1 o2 m | at1
// at1 -> -(a1) | (a1) | -num | num
// o1 -> + | -
// o2 -> * | /
// a -> m o1 a | m
// m -> at o2 m | at
// at -> (a) | num

/// 语法分析部分
pub fn parse_sentence(render: &mut TokenRender) -> Result<Vec<RPNItem>, ParseErr> {
    let mut output = vec![];

    a(render, &mut output)?;

    if render.is_empty() {
        // 如果全部 token 都解析完了

        return Ok(output);
    } else {
        // 如果还有剩余的 token

        return Err(ParseErr {
            reason: "有未能解析的输入，感觉这个错误不应该出现，请告诉我这个表达式是什么".to_owned(),
            err_type: ParseErrType::Redundant(render.peek().position),
        });
    }
}

fn a(render: &mut TokenRender, output: &mut Vec<RPNItem>) -> Result<(), ParseErr> {
    // 交给 m 解析，is_first 为 true
    m(render, output, true)?;

    // 如果还有未解析完的 token ，就试试接着解析
    while !render.is_empty() {
        // 检查 next 是不是 '+' / '-'
        let op = o1(render);

        if op.is_err() {
            // 如果不是 '+' / '-'
            // 检查 a 的 follow 集，也就是检查下一个是不是 ')'
            // 如果不是，那就报错

            render.expect(
                TokenInfo::Symbol(SymbolType::RightBracket),
                |token| {
                    return format!("期望获得运算符，却得到了 '{}' ", token.info).to_owned();
                },
                "预料之外的终止错误，感觉好像输入被中途篡改一样离谱".to_owned(),
            )?;
            break;
        }

        // 交给 m 解析，is_first 为 false
        m(render, output, false)?;

        // 把之前得到的操作符 push 到输出。
        output.push(RPNItem::Operator(op.unwrap()));
    }

    return Ok(());
}

/// 解析 render 中 next 是不是 '+' / '-' ，如果是就返回相应的运算符
fn o1(render: &mut TokenRender) -> Result<Operator, ()> {
    if render.try_token(TokenInfo::Symbol(SymbolType::Add)) {
        return Ok(Operator::Add);
    }
    if render.try_token(TokenInfo::Symbol(SymbolType::Sub)) {
        return Ok(Operator::Sub);
    }
    return Err(());
}

fn m(render: &mut TokenRender, output: &mut Vec<RPNItem>, is_first: bool) -> Result<(), ParseErr> {
    // 交给 at 解析，is_first 为 true
    at(render, output, is_first)?;

    // 如果还有未解析完的 token ，就试试接着解析
    while !render.is_empty() {
        // 检查 next 是不是 '*' / '/'
        let op = o2(render);

        if op.is_err() {
            // 如果不是 '+' / '-' 就跑路，不用解析 follow 集
            break;
        }

        // 交给 at 解析，is_first 为 false
        at(render, output, false)?;

        // 把之前得到的操作符 push 到输出。
        output.push(RPNItem::Operator(op.unwrap()));
    }

    return Ok(());
}

/// 解析 render 中 next 是不是 '*' / '/' ，如果是就返回相应的运算符
fn o2(render: &mut TokenRender) -> Result<Operator, ()> {
    if render.try_token(TokenInfo::Symbol(SymbolType::Mul)) {
        return Ok(Operator::Mul);
    }
    if render.try_token(TokenInfo::Symbol(SymbolType::Div)) {
        return Ok(Operator::Div);
    }
    return Err(());
}

fn at(render: &mut TokenRender, output: &mut Vec<RPNItem>, is_first: bool) -> Result<(), ParseErr> {
    let mut is_neg = false;

    if is_first && render.try_token(TokenInfo::Symbol(SymbolType::Sub)) {
        // 当前为第一个子表达式且以减号开头时解析减号为单目运算符负号
        is_neg = true;
    }

    if !num(render, output) {
        // 解析左括号
        render.expect(
            TokenInfo::Symbol(SymbolType::LeftBracket),
            |token| {
                return format!("期望获得 ( 或数字，却得到了 '{}' ", token.info).to_owned();
            },
            "期望获得 ( 或数字，却意外终止".to_owned(),
        )?;
        render.next(); // 消费左括号

        // 递归调用 a
        a(render, output)?;

        // 解析右括号
        render.expect(
            TokenInfo::Symbol(SymbolType::RightBracket),
            |token| {
                return format!("期望获得 )，却得到了 '{}' ", token.info).to_owned();
            },
            "期望获得 ) ，却意外终止".to_owned(),
        )?;
        render.next(); // 消费右括号
    }

    if is_neg {
        // 如果解析出符号就往输出中 push 一个取负运算符
        output.push(RPNItem::Operator(Operator::Minus));
    }

    return Ok(());
}

// 解析 next 是不是数字
fn num(render: &mut TokenRender, output: &mut Vec<RPNItem>) -> bool {
    // 检查是不是数字类型
    // 错了就错了，没有人关心这里的错误信息，只要有错误就可以
    if let Ok(temp) = render.expect(
        TokenInfo::Number(Num::zero()),
        |_| {
            return "".to_owned();
        },
        "".to_owned(),
    ) {
        if let TokenInfo::Number(n) = temp.info {
            // 取出具体的数字信息
            // 消费掉这个 token
            render.next();
            // 往输出中 push 这个数字
            output.push(RPNItem::Number(n));

            return true;
        }
    }

    return false;
}

#[cfg(test)]
mod test {
    use super::tokenization;
    use crate::sentence::parse_sentence;
    use crate::token_render::*;

    #[test]
    fn parse_test_success() {
        // 测试符合语法的内容
        let input_vec = vec![
            "56+88-9999",
            "1e9",
            "1.0",
            ".0",
            ".0+1",
            "3-2",
            "3-(-2)",
            "(-2)+3",
            "(12)+1",
            "2*3",
            "2*3-1",
            "3-2*1",
            "3*4*5/2",
            "3*4*5/(-2)",
            "-2",
            "-3+.5",
            "(-2)",
            "-(2)",
        ];
        for i in 0..input_vec.len() {
            let re = tokenization(input_vec[i]);
            assert!(re.is_ok());
            let re = parse_sentence(&mut TokenRender::new_with_tokens(re.unwrap()));
            assert!(re.is_ok(), "input: {}", input_vec[i]);
        }
    }

    #[test]
    fn parse_test_fail() {
        // 测试不符合语法的内容
        let input_vec = vec![
            "56+", "1e9-", "*1.0", "(", ")", "()", "(((2)", "3**3", "4-*2", "45(+6)", "4 5", "++",
            "--15", "-(+5)", "++++++1", "+1-", "+3", "3++2", "3--2",
        ];
        for i in 0..input_vec.len() {
            let re = tokenization(input_vec[i]);
            assert!(re.is_ok());
            let re = parse_sentence(&mut TokenRender::new_with_tokens(re.unwrap()));
            assert!(re.is_err(), "input: {}", input_vec[i]);
        }
    }
}
