use crate::Element;


/// Generates the HTML representation of each element.
pub fn generate(elems: Vec<Element>) -> String {
   elems 
        .iter()
        .fold(String::new(), |mut out, elem| {
            out.push_str(&elem.gen());
            out
        })
}

use Element::*;
impl Element {

    /// Generates the HTML representation for self. May recurse into children.
    fn gen (&self) -> String {
        match &self {
            Text { elements } => {
                // Generate the representation for each "element" within the text element.
                // For example, hi *shine* **!** is reprsented as Text: { elems: [ Plain{ hi }, Italic{ shine }, Bold{ ! } ] }
                let mut out = elements
                    .iter()
                    .fold("<p>".to_owned(), |mut a, elem| {
                        a.push_str(&elem.gen());
                        a
                    });
                out.push_str("</p><br>");
                out
            },
            Divider => "<hr />".to_owned(),
            PlainText { text }  => format!("{}", text),
            Bold { text }       => format!("<b>{}</b>", text),
            Italics { text }    => format!("<i>{}</i>", text),
            InlineCode { text } => format!("<code>{}</code>", text),
            Blockquote { text } => format!("<blockquote>{}</blockquote>", text),
            Heading { level, text } => format!("<h{}>{}</h{}>", level, text, level),
            _ => "".to_owned()
        }
    }
}

/*
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
*/
