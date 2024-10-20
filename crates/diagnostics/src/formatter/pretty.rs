use nu_ansi_term::{Color, Style};

use crate::{Diagnostic, Print};

pub struct Pretty;

impl Print<Diagnostic> for Pretty {
    fn print(&self, target: &Diagnostic) {
        println!(
            "{:<8}: {}",
            &target.level().color().bold().paint(target.level().as_str()),
            Style::new().bold().paint(target.message())
        );

        let at_style: Style = Style::new().fg(Color::DarkGray);
        if let Some(source_file) = target.source_file() {
            println!(
                "    {}",
                at_style.paint(format!(
                    "in {}:{}",
                    source_file.display(),
                    target.position()
                )),
            );
        } else {
            // 標準入力の x 行 y 列 でエラーが発生しました
            println!(
                "    {}",
                at_style.paint(format!(
                    "at line {} column {}",
                    target.position().line(),
                    target.position().column()
                )),
            );
        }
    }
}
