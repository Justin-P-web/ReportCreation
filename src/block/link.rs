use super::{Block, Text, text::escape_str};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkDestination {
    Url(String),
    Location(String),
}

#[derive(Debug, Clone)]
pub struct Link {
    destination: LinkDestination,
    content: Text,
}

impl Link {
    pub fn to_url<U: Into<String>, C: Into<Text>>(url: U, content: C) -> Self {
        Self {
            destination: LinkDestination::Url(url.into()),
            content: content.into(),
        }
    }

    pub fn to_location<L: Into<String>, C: Into<Text>>(location: L, content: C) -> Self {
        Self {
            destination: LinkDestination::Location(location.into()),
            content: content.into(),
        }
    }
}

impl Block for Link {
    fn render(&self, output: &mut String) {
        use std::fmt::Write;

        let destination = match &self.destination {
            LinkDestination::Url(url) => format!("target: \"{}\"", escape_str(url)),
            LinkDestination::Location(location) => format!("location: {}", location),
        };

        writeln!(output, "#link({})[{}]", destination, self.content.render())
            .expect("writing to string never fails");
        output.push('\n');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_url_links() {
        let mut output = String::new();

        Link::to_url("https://example.com", Text::new("Example")).render(&mut output);

        assert_eq!(
            output,
            "#link(target: \"https://example.com\")[Example]\n\n"
        );
    }

    #[test]
    fn renders_location_links() {
        let mut output = String::new();

        Link::to_location("@introduction", Text::new("Jump to Intro")).render(&mut output);

        assert_eq!(output, "#link(location: @introduction)[Jump to Intro]\n\n");
    }
}
