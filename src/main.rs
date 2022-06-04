#![windows_subsystem = "windows"]

use crate::{parse::parse, token::tokenization};
use std::rc::Rc;
use token_render::TokenRender;

/// 计算逆波兰式的模块
mod calculate;

/// 语法分析的模块
mod parse;

/// 词法分析的模块
mod token;

/// 储存词法分析的结果的结构
mod token_render;

fn parse_and_run(input: &str) -> Result<(String, String), String> {
    // 对输入进行分词
    match tokenization(input) {
        Ok(t) => {
            // 分词成功
            // 语法分析
            match parse(&mut TokenRender::new_with_tokens(t)) {
                Ok(v) => {
                    // 语法分析成功

                    // 拼装字符串输出逆波兰式
                    let mut rpn_str = String::from("[ ");
                    for item in &v {
                        rpn_str.push_str(&format!("{} ", item));
                    }
                    rpn_str.push_str("]");

                    // 计算逆波兰式
                    let result = calculate::calculate(&v);
                    match result {
                        Ok(n) => {
                            // 计算成功，格式化数字后输出

                            return Ok((
                                rpn_str,
                                format!(
                                    "计算结果为: {} 。",
                                    n.with_scale(15).normalized().to_string()
                                ),
                            ));
                        }
                        Err(e) => {
                            // 计算失败，输出结果

                            return Ok((rpn_str, format!("计算结果为: {} ！", e)));
                        }
                    }
                }
                Err(e) => {
                    // 根据错误类型输出不同的结果
                    match e.err_type {
                        parse::ParseErrType::Unexpected(e2) => {
                            // 未预期的 token

                            return Err(
                                format!(
                                    "语法分析阶段->在输入的第 {} 个字符处遇到错误: {}，输入处原字符串为: '{}' 。",
                                    e2.position + 1,
                                    e.reason,
                                    e2.original_str
                                ).to_owned()
                            );
                        }
                        parse::ParseErrType::Insufficient => {
                            // 预期某一个 token 但是却突然终止

                            return Err(format!("语法分析阶段->输入末尾遇到错误: {} !", e.reason)
                                .to_owned());
                        }
                    }
                }
            }
        }
        Err(e) => {
            // 词法分析出错
            // 根据词法分析的错误输出

            return Err(format!(
                "词法分析阶段->在输入的第 {} 个字符处：{}",
                e.position + 1,
                e.reason
            )
            .to_owned());
        }
    }
}

slint::include_modules!();
fn main() {
    let main_window = Rc::new(MainWindow::new());
    let main_window2 = main_window.clone();
    main_window2.on_input(move |input| {
        if !input.is_empty() {
            // 如果输入不为空
            match parse_and_run(input.as_str()) {
                Ok((re_polish, output)) => {
                    main_window.set_output_content(output.into());
                    main_window.set_re_polish_content(re_polish.into());
                }
                Err(e) => {
                    main_window.set_output_content(format!("错误: {}", e).into());
                    main_window.set_re_polish_content("解析表达式失败！".into());
                }
            }
        } else {
            // 如果输入为空
            main_window.set_output_content("".into());
            main_window.set_re_polish_content("".into());
        }
    });
    main_window2.run();
}
