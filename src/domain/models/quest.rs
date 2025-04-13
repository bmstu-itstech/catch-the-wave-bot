#[derive(Clone)]
pub struct Quest {
    pub id: i64,
    pub text: String,
}

impl Quest {
    pub fn new(id: i64, text: impl Into<String>) -> Self {
        Quest{ id, text: text.into(), }
    }
}
