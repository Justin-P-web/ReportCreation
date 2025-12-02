use super::Block;

#[derive(Debug, Clone, Default)]
pub struct ImageOptions {
    alt: Option<ImageOptionValue>,
    width: Option<ImageOptionValue>,
    height: Option<ImageOptionValue>,
    fit: Option<ImageOptionValue>,
    format: Option<ImageOptionValue>,
    dpi: Option<ImageOptionValue>,
    gamma: Option<ImageOptionValue>,
    frame: Option<ImageOptionValue>,
    invert: Option<ImageOptionValue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImageOptionValue {
    Raw(String),
    Str(String),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub struct Image {
    path: String,
    options: ImageOptions,
}

impl Image {
    pub fn new<P: Into<String>>(path: P) -> Self {
        Self {
            path: path.into(),
            options: ImageOptions::default(),
        }
    }

    pub fn with_options(mut self, options: ImageOptions) -> Self {
        self.options = options;
        self
    }

    pub fn alt<T: Into<String>>(mut self, alt: T) -> Self {
        self.options.alt = Some(ImageOptionValue::str(alt));
        self
    }

    pub fn width<T: Into<String>>(mut self, width: T) -> Self {
        self.options.width = Some(ImageOptionValue::raw(width));
        self
    }

    pub fn height<T: Into<String>>(mut self, height: T) -> Self {
        self.options.height = Some(ImageOptionValue::raw(height));
        self
    }

    pub fn fit<T: Into<String>>(mut self, fit: T) -> Self {
        self.options.fit = Some(ImageOptionValue::raw(fit));
        self
    }

    pub fn format<T: Into<String>>(mut self, format: T) -> Self {
        self.options.format = Some(ImageOptionValue::str(format));
        self
    }

    pub fn dpi<T: Into<String>>(mut self, dpi: T) -> Self {
        self.options.dpi = Some(ImageOptionValue::raw(dpi));
        self
    }

    pub fn gamma<T: Into<String>>(mut self, gamma: T) -> Self {
        self.options.gamma = Some(ImageOptionValue::raw(gamma));
        self
    }

    pub fn frame<T: Into<String>>(mut self, frame: T) -> Self {
        self.options.frame = Some(ImageOptionValue::raw(frame));
        self
    }

    pub fn invert(mut self, invert: bool) -> Self {
        self.options.invert = Some(ImageOptionValue::Bool(invert));
        self
    }
}

impl Block for Image {
    fn render(&self, output: &mut String) {
        use std::fmt::Write;

        write!(output, "#image(\"{}\"", escape_str(self.path.trim()))
            .expect("writing to string never fails");

        for option in self.options.iter() {
            write!(output, ", {}", option).expect("writing to string never fails");
        }

        writeln!(output, ")").expect("writing to string never fails");
        output.push('\n');
    }
}

impl From<Image> for super::BlockNode {
    fn from(value: Image) -> Self {
        Box::new(value)
    }
}

impl From<&str> for Image {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for Image {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl ImageOptions {
    pub fn alt<T: Into<String>>(mut self, alt: T) -> Self {
        self.alt = Some(ImageOptionValue::str(alt));
        self
    }

    pub fn width<T: Into<String>>(mut self, width: T) -> Self {
        self.width = Some(ImageOptionValue::raw(width));
        self
    }

    pub fn height<T: Into<String>>(mut self, height: T) -> Self {
        self.height = Some(ImageOptionValue::raw(height));
        self
    }

    pub fn fit<T: Into<String>>(mut self, fit: T) -> Self {
        self.fit = Some(ImageOptionValue::raw(fit));
        self
    }

    pub fn format<T: Into<String>>(mut self, format: T) -> Self {
        self.format = Some(ImageOptionValue::str(format));
        self
    }

    pub fn dpi<T: Into<String>>(mut self, dpi: T) -> Self {
        self.dpi = Some(ImageOptionValue::raw(dpi));
        self
    }

    pub fn gamma<T: Into<String>>(mut self, gamma: T) -> Self {
        self.gamma = Some(ImageOptionValue::raw(gamma));
        self
    }

    pub fn frame<T: Into<String>>(mut self, frame: T) -> Self {
        self.frame = Some(ImageOptionValue::raw(frame));
        self
    }

    pub fn invert(mut self, invert: bool) -> Self {
        self.invert = Some(ImageOptionValue::Bool(invert));
        self
    }

    fn iter(&self) -> impl Iterator<Item = String> + '_ {
        [
            ("alt", &self.alt),
            ("width", &self.width),
            ("height", &self.height),
            ("fit", &self.fit),
            ("format", &self.format),
            ("dpi", &self.dpi),
            ("gamma", &self.gamma),
            ("frame", &self.frame),
            ("invert", &self.invert),
        ]
        .into_iter()
        .filter_map(|(name, value)| value.as_ref().map(|v| format!("{}: {}", name, v)))
    }
}

impl ImageOptionValue {
    pub fn raw<T: Into<String>>(value: T) -> Self {
        Self::Raw(value.into())
    }

    pub fn str<T: Into<String>>(value: T) -> Self {
        Self::Str(value.into())
    }
}

impl std::fmt::Display for ImageOptionValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageOptionValue::Raw(value) => write!(f, "{}", value),
            ImageOptionValue::Str(value) => write!(f, "\"{}\"", escape_str(value)),
            ImageOptionValue::Bool(value) => write!(f, "{}", value),
        }
    }
}

fn escape_str(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_basic_image() {
        let mut rendered = String::new();
        Image::new("./plot.png").render(&mut rendered);

        assert_eq!(rendered, "#image(\"./plot.png\")\n\n");
    }

    #[test]
    fn renders_image_with_options() {
        let mut rendered = String::new();
        let options = ImageOptions::default()
            .alt("Line chart of growth over time")
            .width("80%")
            .height("120pt")
            .fit("cover")
            .format("png")
            .dpi("144")
            .gamma("2.2")
            .frame("inset")
            .invert(true);

        Image::new("./plot.png")
            .with_options(options)
            .render(&mut rendered);

        assert_eq!(
            rendered,
            "#image(\"./plot.png\", alt: \"Line chart of growth over time\", width: 80%, height: 120pt, fit: cover, format: \"png\", dpi: 144, gamma: 2.2, frame: inset, invert: true)\n\n"
        );
    }

    #[test]
    fn escapes_quotes_and_backslashes() {
        let mut rendered = String::new();
        Image::new(".\\\"plot\".png").render(&mut rendered);

        assert_eq!(rendered, "#image(\".\\\\\\\"plot\\\".png\")\n\n");
    }
}
