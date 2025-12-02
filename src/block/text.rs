#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Text {
    content: String,
    options: TextOptions,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TextOptions {
    fill: Option<TextOptionValue>,
    lang: Option<TextOptionValue>,
    size: Option<TextOptionValue>,
    font: Option<TextOptionValue>,
    style: Option<TextOptionValue>,
    weight: Option<TextOptionValue>,
    tracking: Option<TextOptionValue>,
    stretch: Option<TextOptionValue>,
    variant: Option<TextOptionValue>,
    baseline: Option<TextOptionValue>,
    underline: Option<TextOptionValue>,
    overline: Option<TextOptionValue>,
    line_through: Option<TextOptionValue>,
    outline: Option<TextOptionValue>,
    shadow: Option<TextOptionValue>,
    offset: Option<TextOptionValue>,
    rotate: Option<TextOptionValue>,
    scale: Option<TextOptionValue>,
    dir: Option<TextOptionValue>,
    writing_mode: Option<TextOptionValue>,
    region: Option<TextOptionValue>,
    justification: Option<TextOptionValue>,
    align: Option<TextOptionValue>,
    first_line_indent: Option<TextOptionValue>,
    hanging_indent: Option<TextOptionValue>,
    leading: Option<TextOptionValue>,
    spacing: Option<TextOptionValue>,
    parbreak: Option<TextOptionValue>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TextOptionValue {
    Raw(String),
    Str(String),
}

impl Text {
    pub fn new<T: Into<String>>(content: T) -> Self {
        Self {
            content: content.into(),
            options: TextOptions::default(),
        }
    }

    pub fn with_options<T: Into<String>>(content: T, options: TextOptions) -> Self {
        Self {
            content: content.into(),
            options,
        }
    }

    pub fn as_str(&self) -> &str {
        &self.content
    }

    pub fn options(&self) -> &TextOptions {
        &self.options
    }

    pub fn fill<T: Into<String>>(mut self, color: T) -> Self {
        self.options.fill = Some(TextOptionValue::raw(color));
        self
    }

    pub fn lang<T: Into<String>>(mut self, lang: T) -> Self {
        self.options.lang = Some(TextOptionValue::str(lang));
        self
    }

    pub fn size<T: Into<String>>(mut self, size: T) -> Self {
        self.options.size = Some(TextOptionValue::raw(size));
        self
    }

    pub fn font<T: Into<String>>(mut self, font: T) -> Self {
        self.options.font = Some(TextOptionValue::str(font));
        self
    }

    pub fn style<T: Into<String>>(mut self, style: T) -> Self {
        self.options.style = Some(TextOptionValue::raw(style));
        self
    }

    pub fn weight<T: Into<String>>(mut self, weight: T) -> Self {
        self.options.weight = Some(TextOptionValue::raw(weight));
        self
    }

    pub fn tracking<T: Into<String>>(mut self, tracking: T) -> Self {
        self.options.tracking = Some(TextOptionValue::raw(tracking));
        self
    }

    pub fn stretch<T: Into<String>>(mut self, stretch: T) -> Self {
        self.options.stretch = Some(TextOptionValue::raw(stretch));
        self
    }

    pub fn variant<T: Into<String>>(mut self, variant: T) -> Self {
        self.options.variant = Some(TextOptionValue::raw(variant));
        self
    }

    pub fn baseline<T: Into<String>>(mut self, baseline: T) -> Self {
        self.options.baseline = Some(TextOptionValue::raw(baseline));
        self
    }

    pub fn underline<T: Into<String>>(mut self, underline: T) -> Self {
        self.options.underline = Some(TextOptionValue::raw(underline));
        self
    }

    pub fn overline<T: Into<String>>(mut self, overline: T) -> Self {
        self.options.overline = Some(TextOptionValue::raw(overline));
        self
    }

    pub fn line_through<T: Into<String>>(mut self, line_through: T) -> Self {
        self.options.line_through = Some(TextOptionValue::raw(line_through));
        self
    }

    pub fn outline<T: Into<String>>(mut self, outline: T) -> Self {
        self.options.outline = Some(TextOptionValue::raw(outline));
        self
    }

    pub fn shadow<T: Into<String>>(mut self, shadow: T) -> Self {
        self.options.shadow = Some(TextOptionValue::raw(shadow));
        self
    }

    pub fn offset<T: Into<String>>(mut self, offset: T) -> Self {
        self.options.offset = Some(TextOptionValue::raw(offset));
        self
    }

    pub fn rotate<T: Into<String>>(mut self, rotate: T) -> Self {
        self.options.rotate = Some(TextOptionValue::raw(rotate));
        self
    }

    pub fn scale<T: Into<String>>(mut self, scale: T) -> Self {
        self.options.scale = Some(TextOptionValue::raw(scale));
        self
    }

    pub fn dir<T: Into<String>>(mut self, dir: T) -> Self {
        self.options.dir = Some(TextOptionValue::raw(dir));
        self
    }

    pub fn writing_mode<T: Into<String>>(mut self, mode: T) -> Self {
        self.options.writing_mode = Some(TextOptionValue::raw(mode));
        self
    }

    pub fn region<T: Into<String>>(mut self, region: T) -> Self {
        self.options.region = Some(TextOptionValue::raw(region));
        self
    }

    pub fn justification<T: Into<String>>(mut self, justification: T) -> Self {
        self.options.justification = Some(TextOptionValue::raw(justification));
        self
    }

    pub fn align<T: Into<String>>(mut self, align: T) -> Self {
        self.options.align = Some(TextOptionValue::raw(align));
        self
    }

    pub fn first_line_indent<T: Into<String>>(mut self, indent: T) -> Self {
        self.options.first_line_indent = Some(TextOptionValue::raw(indent));
        self
    }

    pub fn hanging_indent<T: Into<String>>(mut self, indent: T) -> Self {
        self.options.hanging_indent = Some(TextOptionValue::raw(indent));
        self
    }

    pub fn leading<T: Into<String>>(mut self, leading: T) -> Self {
        self.options.leading = Some(TextOptionValue::raw(leading));
        self
    }

    pub fn spacing<T: Into<String>>(mut self, spacing: T) -> Self {
        self.options.spacing = Some(TextOptionValue::raw(spacing));
        self
    }

    pub fn parbreak<T: Into<String>>(mut self, parbreak: T) -> Self {
        self.options.parbreak = Some(TextOptionValue::raw(parbreak));
        self
    }

    pub fn render(&self) -> String {
        if self.options.is_empty() {
            return self.content.trim().to_string();
        }

        let mut rendered = String::from("#text(");
        rendered.push_str(&format!("\"{}\"", escape_str(self.content.trim())));

        for option in self.options.iter() {
            rendered.push_str(", ");
            rendered.push_str(&option);
        }

        rendered.push(')');
        rendered
    }
}

impl From<String> for Text {
    fn from(value: String) -> Self {
        Self {
            content: value,
            options: TextOptions::default(),
        }
    }
}

impl From<&str> for Text {
    fn from(value: &str) -> Self {
        Self {
            content: value.to_string(),
            options: TextOptions::default(),
        }
    }
}

impl std::ops::Deref for Text {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.content.as_str()
    }
}

impl TextOptions {
    pub fn fill<T: Into<String>>(mut self, color: T) -> Self {
        self.fill = Some(TextOptionValue::raw(color));
        self
    }

    pub fn lang<T: Into<String>>(mut self, lang: T) -> Self {
        self.lang = Some(TextOptionValue::str(lang));
        self
    }

    pub fn size<T: Into<String>>(mut self, size: T) -> Self {
        self.size = Some(TextOptionValue::raw(size));
        self
    }

    pub fn font<T: Into<String>>(mut self, font: T) -> Self {
        self.font = Some(TextOptionValue::str(font));
        self
    }

    pub fn style<T: Into<String>>(mut self, style: T) -> Self {
        self.style = Some(TextOptionValue::raw(style));
        self
    }

    pub fn weight<T: Into<String>>(mut self, weight: T) -> Self {
        self.weight = Some(TextOptionValue::raw(weight));
        self
    }

    pub fn tracking<T: Into<String>>(mut self, tracking: T) -> Self {
        self.tracking = Some(TextOptionValue::raw(tracking));
        self
    }

    pub fn stretch<T: Into<String>>(mut self, stretch: T) -> Self {
        self.stretch = Some(TextOptionValue::raw(stretch));
        self
    }

    pub fn variant<T: Into<String>>(mut self, variant: T) -> Self {
        self.variant = Some(TextOptionValue::raw(variant));
        self
    }

    pub fn baseline<T: Into<String>>(mut self, baseline: T) -> Self {
        self.baseline = Some(TextOptionValue::raw(baseline));
        self
    }

    pub fn underline<T: Into<String>>(mut self, underline: T) -> Self {
        self.underline = Some(TextOptionValue::raw(underline));
        self
    }

    pub fn overline<T: Into<String>>(mut self, overline: T) -> Self {
        self.overline = Some(TextOptionValue::raw(overline));
        self
    }

    pub fn line_through<T: Into<String>>(mut self, line_through: T) -> Self {
        self.line_through = Some(TextOptionValue::raw(line_through));
        self
    }

    pub fn outline<T: Into<String>>(mut self, outline: T) -> Self {
        self.outline = Some(TextOptionValue::raw(outline));
        self
    }

    pub fn shadow<T: Into<String>>(mut self, shadow: T) -> Self {
        self.shadow = Some(TextOptionValue::raw(shadow));
        self
    }

    pub fn offset<T: Into<String>>(mut self, offset: T) -> Self {
        self.offset = Some(TextOptionValue::raw(offset));
        self
    }

    pub fn rotate<T: Into<String>>(mut self, rotate: T) -> Self {
        self.rotate = Some(TextOptionValue::raw(rotate));
        self
    }

    pub fn scale<T: Into<String>>(mut self, scale: T) -> Self {
        self.scale = Some(TextOptionValue::raw(scale));
        self
    }

    pub fn dir<T: Into<String>>(mut self, dir: T) -> Self {
        self.dir = Some(TextOptionValue::raw(dir));
        self
    }

    pub fn writing_mode<T: Into<String>>(mut self, mode: T) -> Self {
        self.writing_mode = Some(TextOptionValue::raw(mode));
        self
    }

    pub fn region<T: Into<String>>(mut self, region: T) -> Self {
        self.region = Some(TextOptionValue::raw(region));
        self
    }

    pub fn justification<T: Into<String>>(mut self, justification: T) -> Self {
        self.justification = Some(TextOptionValue::raw(justification));
        self
    }

    pub fn align<T: Into<String>>(mut self, align: T) -> Self {
        self.align = Some(TextOptionValue::raw(align));
        self
    }

    pub fn first_line_indent<T: Into<String>>(mut self, indent: T) -> Self {
        self.first_line_indent = Some(TextOptionValue::raw(indent));
        self
    }

    pub fn hanging_indent<T: Into<String>>(mut self, indent: T) -> Self {
        self.hanging_indent = Some(TextOptionValue::raw(indent));
        self
    }

    pub fn leading<T: Into<String>>(mut self, leading: T) -> Self {
        self.leading = Some(TextOptionValue::raw(leading));
        self
    }

    pub fn spacing<T: Into<String>>(mut self, spacing: T) -> Self {
        self.spacing = Some(TextOptionValue::raw(spacing));
        self
    }

    pub fn parbreak<T: Into<String>>(mut self, parbreak: T) -> Self {
        self.parbreak = Some(TextOptionValue::raw(parbreak));
        self
    }

    fn is_empty(&self) -> bool {
        self.iter().next().is_none()
    }

    fn iter(&self) -> impl Iterator<Item = String> + '_ {
        [
            ("fill", &self.fill),
            ("lang", &self.lang),
            ("size", &self.size),
            ("font", &self.font),
            ("style", &self.style),
            ("weight", &self.weight),
            ("tracking", &self.tracking),
            ("stretch", &self.stretch),
            ("variant", &self.variant),
            ("baseline", &self.baseline),
            ("underline", &self.underline),
            ("overline", &self.overline),
            ("line_through", &self.line_through),
            ("outline", &self.outline),
            ("shadow", &self.shadow),
            ("offset", &self.offset),
            ("rotate", &self.rotate),
            ("scale", &self.scale),
            ("dir", &self.dir),
            ("writing_mode", &self.writing_mode),
            ("region", &self.region),
            ("justification", &self.justification),
            ("align", &self.align),
            ("first_line_indent", &self.first_line_indent),
            ("hanging_indent", &self.hanging_indent),
            ("leading", &self.leading),
            ("spacing", &self.spacing),
            ("parbreak", &self.parbreak),
        ]
        .into_iter()
        .filter_map(|(name, value)| value.as_ref().map(|v| format!("{}: {}", name, v)))
    }
}

impl TextOptionValue {
    pub fn raw<T: Into<String>>(value: T) -> Self {
        Self::Raw(value.into())
    }

    pub fn str<T: Into<String>>(value: T) -> Self {
        Self::Str(value.into())
    }
}

impl std::fmt::Display for TextOptionValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextOptionValue::Raw(value) => write!(f, "{}", value),
            TextOptionValue::Str(value) => write!(f, "\"{}\"", escape_str(value)),
        }
    }
}

fn escape_str(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}
