#[derive(Clone, Copy, Debug)]
pub enum ContextPosition {
    Point(usize),
    Range((usize, usize)),
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum AlertKind {
    #[default]
    Error,
    Warn,
}

#[derive(Default, Debug)]
pub struct ScriptAlert {
    pub line_num: usize,
    pub line_ctx: String,
    pub file_path: Option<String>,
    pub line_pos: Option<ContextPosition>,
    pub message: String,
    pub kind: AlertKind,
    pub additional_info: Option<String>,
}

impl ScriptAlert {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn as_mut(&mut self) -> &mut Self {
        self
    }

    pub fn with_line(&mut self, line_num: usize) -> &mut Self {
        self.line_num = line_num;
        self
    }

    pub fn with_context(&mut self, line_ctx: impl Into<String>) -> &mut Self {
        self.line_ctx = line_ctx.into();
        self
    }

    pub fn with_file(&mut self, file_path: Option<impl Into<String>>) -> &mut Self {
        self.file_path = match file_path {
            Some(val) => Some(val.into()),
            None => None,
        };
        self
    }

    pub fn with_position(&mut self, line_pos: Option<ContextPosition>) -> &mut Self {
        self.line_pos = line_pos;
        self
    }

    pub fn with_message(&mut self, message: impl Into<String>) -> &mut Self {
        self.message = message.into();
        self
    }

    pub fn with_kind(&mut self, kind: AlertKind) -> &mut Self {
        self.kind = kind;
        self
    }

    pub fn with_additional_info(&mut self, additional_info: String) -> &mut Self {
        self.additional_info = Some(additional_info);
        self
    }
}
