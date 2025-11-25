use crate::errors::builder::{AlertKind, ContextPosition, ScriptAlert};
use cli_colors::Colorizer;

impl ScriptAlert {
    pub fn format(&self) -> String {
        let mut response: Vec<String> = vec![];

        let col = Colorizer::new();
        let response_type = match self.kind {
            AlertKind::Error => col.bold(col.red("Error")),
            AlertKind::Warn => col.bold(col.yellow("Warning")),
        };

        response.push(format!("{}: {}", response_type, self.format_position(),));

        let whitespace = "    ";
        let line_buffer = format!("{}{} |", whitespace, self.line_num);
        response.push(format!("{}{}", line_buffer, self.line_ctx));

        if let Some(pos) = self.line_pos {
            let marker = match pos {
                ContextPosition::Point(point) => {
                    " ".repeat(point + line_buffer.len()) + "^-- " + self.message.as_str()
                }
                ContextPosition::Range((start, end)) => {
                    " ".repeat(start + line_buffer.len())
                        + "^".repeat(end - start).as_str()
                        + " "
                        + self.message.as_str()
                }
            };

            let format = match self.kind {
                AlertKind::Error => col.bold(col.red(marker)),
                AlertKind::Warn => col.bold(col.yellow(marker)),
            };

            response.push(format!("{}", format));
        }

        response.join("\n")
    }

    pub fn format_position(&self) -> String {
        let mut response: Vec<String> = vec![];
        response.push(format!(
            "{}",
            self.file_path.clone().unwrap_or("unknown".into())
        ));
        response.push(format!("{}", self.line_num));

        if let Some(pos) = self.line_pos {
            let marker = match pos {
                ContextPosition::Point(point) => {
                    format!("{}", point)
                }
                ContextPosition::Range((start, _)) => {
                    format!("{}", start)
                }
            };
            response.push(marker);
        }

        response.join(":")
    }

    pub fn report_and_crash(&self) {
        self.report();
        if self.kind == AlertKind::Error {
            std::process::exit(1);
        }
    }

    pub fn report(&self) {
        let format = self.format();
        println!("{}\n", format);
    }
}
