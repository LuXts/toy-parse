#![windows_subsystem = "windows"]

use crate::{parse::parse, token::tokenization};
use ariadne::{CharSet, Color, Config, Label, Report, ReportKind, Source};
use std::{
    io::{Cursor, Read, Seek, SeekFrom},
    rc::Rc,
};
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
    // 配置错误信息
    let mut c = Cursor::new(Vec::new());
    let config = Config::default()
        .with_color(false)
        .with_multiline_arrows(false)
        .with_char_set(CharSet::Unicode)
        .with_compact(false)
        .with_cross_gap(false);

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

                            Report::build(
                                ReportKind::Custom("语法分析阶段", Color::Unset),
                                (),
                                e2.position,
                            )
                            .with_config(config)
                            .with_message(e.reason)
                            .with_label(
                                Label::new(e2.position..(e2.position + e2.original_str.len()))
                                    .with_message(format!("这是一个{}。", e2.info)),
                            )
                            .finish()
                            .write(Source::from(input), &mut c)
                            .unwrap();
                            c.seek(SeekFrom::Start(0)).unwrap();
                            let mut out = String::new();
                            c.read_to_string(&mut out).unwrap();
                            return Err(out);
                        }
                        parse::ParseErrType::Insufficient => {
                            // 预期某一个 token 但是却突然终止
                            Report::build(
                                ReportKind::Custom("语法分析阶段", Color::Unset),
                                (),
                                input.len() - 1,
                            )
                            .with_config(config)
                            .with_message(e.reason)
                            .with_label(
                                Label::new((input.len())..(input.len() + 1))
                                    .with_message(format!("未预期到的结束")),
                            )
                            .finish()
                            .write(Source::from(input), &mut c)
                            .unwrap();
                            c.seek(SeekFrom::Start(0)).unwrap();
                            let mut out = String::new();
                            c.read_to_string(&mut out).unwrap();
                            return Err(out);
                        }
                    }
                }
            }
        }
        Err(e) => {
            // 词法分析出错
            // 根据词法分析的错误输出
            Report::build(
                ReportKind::Custom("词法分析阶段", Color::Unset),
                (),
                e.position,
            )
            .with_config(config)
            .with_message(e.reason)
            .with_label(
                Label::new((e.position)..(e.position + e.original_str.len()))
                    .with_message(format!("这不是合法的数字或者符号")),
            )
            .finish()
            .write(Source::from(input), &mut c)
            .unwrap();
            c.seek(SeekFrom::Start(0)).unwrap();
            let mut out = String::new();
            c.read_to_string(&mut out).unwrap();
            return Err(out);
        }
    }
}

slint::include_modules!();
fn main() {
    let mut old_input = String::new();
    let main_window = Rc::new(MainWindow::new());
    let main_window2 = main_window.clone();
    main_window2.on_input(move |input| {
        if input.is_ascii() {
            old_input = input.to_owned().into();
            if !input.is_empty() {
                // 如果输入不为空
                match parse_and_run(input.as_str()) {
                    Ok((re_polish, output)) => {
                        main_window.set_out_viewport_x(0f32);
                        main_window.set_output_width(output.len() as i32 + 20);
                        main_window.set_re_polish_width(re_polish.len() as i32 + 20);
                        main_window.set_output_content(output.into());
                        main_window.set_re_polish_content(re_polish.into());
                    }
                    Err(e) => {
                        main_window.set_output_width(input.len() as i32 + 20);
                        main_window.set_re_polish_width(15);
                        main_window.set_output_content(e.into());
                        main_window.set_re_polish_content("解析表达式失败！".into());
                    }
                }
            } else {
                // 如果输入为空
                main_window.set_output_content("".into());
                main_window.set_re_polish_content("".into());
            }
        } else {
            main_window.set_input_content(old_input.clone().into());
        }
    });
    main_window2.run();
}
