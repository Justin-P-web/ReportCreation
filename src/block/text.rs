#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Text {
    content: String,
}

impl Text {
    pub fn new<T: Into<String>>(content: T) -> Self {
        Self {
            content: content.into(),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.content
    }
}

impl From<String> for Text {
    fn from(value: String) -> Self {
        Self { content: value }
    }
}

impl From<&str> for Text {
    fn from(value: &str) -> Self {
        Self {
            content: value.to_string(),
        }
    }
}

impl std::ops::Deref for Text {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.content.as_str()
    }
}
