//! extensions handles parsing of GPX-spec extensions.

// TODO: extensions are not implemented

use std::io::Read;

use xml::reader::XmlEvent;

use crate::dom::{Comment, Element, Text};
use crate::errors::{GpxError, GpxResult};
use crate::parser::Context;

/// consume consumes a single string as tag content.
pub fn consume<R: Read>(context: &mut Context<R>) -> GpxResult<Element> {
    let mut started = false;
    let mut stack = vec![Element::with_local_name("extensions")];

    for event in context.reader() {
        match event? {
            XmlEvent::StartElement {
                name,
                attributes,
                namespace,
            } => {
                // flip started depending on conditions
                if &name.local_name == "extensions" {
                    if started {
                        return Err(GpxError::TagOpenedTwice("extensions"));
                    }

                    started = true;
                } else {
                    let new_element = Element::new(name, attributes, namespace);
                    stack.push(new_element);
                }
            }

            XmlEvent::EndElement { name, .. } => {
                if &name.local_name == "extensions" {
                    assert_eq!(stack.len(), 1);
                    return Ok(stack.remove(0));
                }
                // There is always at least one more element on the stack, due to the starting
                // <extensions>-element
                let element = stack.pop().unwrap();
                if element.name == name {
                    stack.last_mut().unwrap().children.push(element.into());
                } else {
                    return Err(GpxError::MissingClosingTag("EXTENSION MALFORMED"));
                }
            }

            XmlEvent::Characters(data) => {
                let text = Text(data);
                stack.last_mut().unwrap().children.push(text.into());
            }

            XmlEvent::Comment(data) => {
                let comment = Comment(data);
                stack.last_mut().unwrap().children.push(comment.into());
            }

            _ => {}
        }
    }

    Err(GpxError::MissingClosingTag("extensions"))
}

#[cfg(test)]
mod tests {
    use super::consume;
    use crate::GpxVersion;

    #[test]
    fn consume_arbitrary_extensions() {
        let result = consume!(
            "<extensions>
                hello world
                <a><b cond=\"no\"><c>derp</c></b></a>
                <tag>yadda yadda we dont care</tag>
            </extensions>",
            GpxVersion::Gpx11
        );

        assert!(result.is_ok());
    }
}
