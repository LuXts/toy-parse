use std::rc::Rc;

use crate::{sentence::parse_sentence, token::parse_token, translation::translate_ast};

mod calculate;
mod sentence;
mod token;
mod translation;

use bigdecimal::ToPrimitive;

fn parse_and_run(input: &str) -> Result<(String, String), String> {
    //println!("输入字符串为    ||    '{}'", &input);
    let mut out = String::from("[ ");
    match parse_token(input) {
        Ok(mut t) => match parse_sentence(&mut t) {
            Ok(root) => {
                let v = translate_ast(root);
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
                            format!("计算结果为: {} 。", n.to_f64().unwrap()).to_owned(),
                        ))
                    }
                    Err(e) => return Err(format!("计算结果为: {} !", e)),
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
                        "语法分析阶段->在输入的第 {} 个字符处遇到错误: {}，未能解析的输入：'{}' 。",
                        e2 + 1,
                        e.reason,
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

slint::slint! {
import {
    ComboBox, VerticalBox, HorizontalBox, GridBox, Button,
    LineEdit, ListView, GroupBox, CheckBox,Slider,TextEdit
} from "std-widgets.slint";


MainWindow := Window {
    min-width: 700px;
    min-height: 400px;
    preferred-width: 700px;
    preferred-height: 400px;
    default-font-size:16px;
    title:"四则运算解释器";
    property <string> re-polish-content <=> re-polish.text;
    property <string> output-content <=> output.text;
    callback input(string);
    GridBox{
        Row{
            Text {
                max-height: 30px;
                preferred-width: 180px;
                text: "四则运算输入:";
                font-weight: 600;
                vertical-alignment: center;
                horizontal-alignment: right;
            }
            HorizontalLayout{
                max-height: 50px;
                InputEdit:= LineEdit {
                    horizontal_stretch: 1;
                    placeholder-text:"请输入表达式！";
                }
            }
            Button {
                text: "解析";
                clicked => {root.input(InputEdit.text) }
            }

        }
        Row{
            Text {
                font-weight: 600;
                text: "逆波兰式:";
                vertical-alignment: top;
                horizontal-alignment: right;
            }
            HorizontalLayout{
                Rectangle {
                    preferred-height: 50px;
                    background: #EFEFEF20;
                        re-polish:= Text {
                            font-size: 14px;
                            text: "";
                            wrap: word-wrap;
                            width: parent.width;
                        }
                }
            }

        }
        Row{
            Text {
                font-weight: 600;
                text: "输出:";
                vertical-alignment: top;
                horizontal-alignment: right;
            }
            HorizontalLayout{
                output:= Text {
                    font-size: 14px;
                    text: "";
                    wrap: word-wrap;
                    horizontal-stretch: 1;
                }
            }
        }
    }
}
}

fn main() {
    let main_window = Rc::new(MainWindow::new());
    let main_window2 = main_window.clone();
    main_window2.on_input(move |input| match parse_and_run(input.as_str()) {
        Ok((re_polish, output)) => {
            main_window.set_output_content(output.into());
            main_window.set_re_polish_content(re_polish.into());
        }
        Err(e) => {
            main_window.set_output_content(format!("错误: {}", e).into());
            main_window.set_re_polish_content("".into());
        }
    });
    main_window2.run();
}
