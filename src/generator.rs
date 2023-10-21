use crate::parser::Element;

pub struct Generator {
    elements: Vec<Element>,
}

impl Generator {
    pub fn new(elements: Vec<Element>) -> Self {
        Self { elements }
    }

    pub fn generate(&self) -> String {
        let mut output = String::new();

        for element in &self.elements {
            match element {
                Element::PlainText { text } => output.push_str(&format!("<p>{}</p>", text)),
                Element::Bold { text } => output.push_str(&format!("<b>{}</b>", text)),
                Element::Italics { text } => output.push_str(&format!("<i>{}</i>", text)),
                Element::Blockquote { text } => {
                    output.push_str(&format!("<blockquote>{}</blockquote>", text))
                }
                Element::Heading { level, text } => {
                    output.push_str(&format!("<h{}>{}</h{}>", level, text, level))
                }
                Element::Divider => output.push_str("<hr />"),
            }
        }

        return output;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generate_html() {
        let tokens: Vec<Element> = vec![
            Element::Heading {
                level: 1,
                text: "Big heading".to_owned(),
            },
            Element::PlainText {
                text: "Hello!".to_owned(),
            },
            Element::Heading {
                level: 3,
                text: "Small heading".to_owned(),
            },
            Element::PlainText {
                text: "divider?".to_owned(),
            },
            Element::Divider,
            Element::PlainText {
                text: "divided.".to_owned(),
            },
            Element::Bold {
                text: "bold text".to_owned(),
            },
            Element::Italics {
                text: "italic text".to_owned(),
            },
        ];

        let generator = Generator::new(tokens);
        let html = generator.generate();

        let expected = r#"<h1>Big heading</h1><p>Hello!</p><h3>Small heading</h3><p>divider?</p><hr /><p>divided.</p><b>bold text</b><i>italic text</i>"#;

        assert_eq!(html, expected);
    }
}
