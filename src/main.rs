#![windows_subsystem = "windows"]

use token_render::TokenRender;

use crate::{sentence::parse_sentence, token::parse_token};
use std::rc::Rc;

mod calculate;
mod sentence;
mod token;
mod token_render;

fn parse_and_run(input: &str) -> Result<(String, String), String> {
    //println!("输入字符串为    ||    '{}'", &input);
    let mut out = String::from("[ ");
    match parse_token(input) {
        Ok(t) => match parse_sentence(&mut TokenRender::new_with_tokens(t)) {
            Ok(v) => {
                //print!("逆波兰式为      ||    [ ");
                for item in &v {
                    //print!("{} ", item);
                    out.push_str(&format!("{} ", item));
                }
                //println!("]");
                out.push_str("]");
                let re = calculate::calculate(&v);
                match re {
                    Ok(n) => {
                        return Ok((
                            out,
                            format!(
                                "计算结果为: {} 。",
                                n.with_scale(15).normalized().to_string()
                            ),
                        ));
                    }
                    Err(e) => return Ok((out, format!("计算结果为: {} ！", e))),
                }
            }
            Err(e) => match e.err_type {
                sentence::ParseErrType::Unexpected(e2) => {
                    return Err(format!(
                        "语法分析阶段->在输入的第 {} 个字符处遇到错误: {}，输入处原字符串为: '{}' 。",
                        e2.position + 1,
                        e.reason,
                        e2.original
                    )
                    .to_owned());
                }
                sentence::ParseErrType::Insufficient => {
                    return Err(
                        format!("语法分析阶段->输入末尾遇到错误: {} !", e.reason).to_owned()
                    );
                }
                sentence::ParseErrType::Redundant(e2) => {
                    return Err(format!(
                        "语法分析阶段->在输入的第 {} 个字符处遇到错误: 未能解析的输入：'{}' 。",
                        e2 + 1,
                        &input[e2..]
                    )
                    .to_owned());
                }
            },
        },
        Err(e) => {
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
            main_window.set_output_content("".into());
            main_window.set_re_polish_content("".into());
        }
    });
    main_window2.run();
}
