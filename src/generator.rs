use crate::{Element, Node};


pub fn generate(nodes: Vec<Node>) -> String {
    nodes
        .iter()
        .fold(String::new(), |mut out, node| {
            out.push_str(&node.gen());
            out
        })
}

use Element::*;
impl Node {
    fn gen (&self) -> String {
        match &self.element {
            PlainText { text } => format!("<p>{}</p>", text),
            Bold { text } => format!("<b>{}</b>", text),
            Italics { text } => format!("<i>{}</i>", text),
            Blockquote { text } => {
                format!("<blockquote>{}</blockquote>", text)
            }
            Heading { level, text } => {
                format!("<h{}>{}</h{}>", level, text, level)
            }
            Divider => "<hr />".to_owned(),
            InlineCode { text } => format!("<code>{}</code>", text),
        }
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
            Element::Strikethrough {
                text: "strikethru text".to_owned(),
            },
        ];

        let tree = crate::parser::parse(tokens);
        let html = generate(tree);

        let expected = r#"<h1>Big heading</h1><p>Hello!</p><h3>Small heading</h3><p>divider?</p><hr /><p>divided.</p><b>bold text</b><i>italic text</i><s>strikethru text</s>"#;

        assert_eq!(html, expected);
    }
}
