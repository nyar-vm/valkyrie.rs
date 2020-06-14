use jupyter::{Executed, JupyterTheme};
use serde_json::Value;

pub struct DisplayKeywords {
    text: String,
}
impl Executed for DisplayKeywords {
    fn mime_type(&self) -> String {
        "text/html".to_string()
    }

    fn as_json(&self, _: JupyterTheme) -> Value {
        Value::String(format!(r#"<span style="color: pink">{}</span>"#, self.text))
    }
}

impl DisplayKeywords {
    pub fn new<S: ToString>(text: S) -> Self {
        Self { text: text.to_string() }
    }
}

pub struct DisplayText {
    text: String,
}

impl Executed for DisplayText {
    fn mime_type(&self) -> String {
        "text/plaintext".to_string()
    }

    fn as_json(&self, _: JupyterTheme) -> Value {
        Value::String(self.text.clone())
    }
}

pub struct DisplayError {
    text: String,
}

impl Executed for DisplayError {
    fn mime_type(&self) -> String {
        "text/html".to_string()
    }

    fn as_json(&self, _: JupyterTheme) -> Value {
        Value::String(format!(r#"<span style="color: red">{}</span>"#, self.text))
    }
}

impl DisplayError {
    pub fn new<S: ToString>(text: S) -> Self {
        Self { text: text.to_string() }
    }
}

pub struct DisplayNumber {
    r#type: String,
    text: String,
}

impl Executed for DisplayNumber {
    fn mime_type(&self) -> String {
        "text/html".to_string()
    }

    fn as_json(&self, _: JupyterTheme) -> Value {
        Value::String(format!(r#"<span class="color: oriange">{}</span>"#, self.text))
    }
}

impl DisplayNumber {
    pub fn new<S: ToString>(text: S) -> Self {
        Self { r#type: String::new(), text: text.to_string() }
    }
    pub fn typed<T, S>(text: T, r#type: S) -> Self
    where
        T: ToString,
        S: ToString,
    {
        Self { r#type: r#type.to_string(), text: text.to_string() }
    }
}
