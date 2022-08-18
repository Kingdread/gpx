//! Implementation of a simple DOM-like tree for XML objects.
//!
//! The DOM objects here are used to represent arbitrary extensions, as `gpx` does not have a way
//! to sensibly consume them. Therefore, they are stored verbatim.
//!
//! You only need to use the types in this module if you plan on working with extensions.
//! Standardized GPX data can be accessed through the main structs.
//!
//! **Note**: This module re-defines many objects that are already defined by `xml-rs`. This is due
//! to the need to be serialiazable (`use-serde` feature). All of the objects are easily
//! convertible to their `xml-rs` counterpart and vice versa.
use std::collections::BTreeMap;

#[cfg(feature = "use-serde")]
use serde::{Deserialize, Serialize};
use xml::{attribute::Attribute, name::Name};

/// Our version of [`xml::name::OwnedName`].
///
/// Contrary to its original counterpart, this struct can be made (de)serializable.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "use-serde", derive(Serialize, Deserialize))]
pub struct OwnedName {
    /// The local name ("tag name") of the element.
    pub local_name: String,
    /// The resolved namespace URI, if any is given.
    pub namespace: Option<String>,
    /// The namespace prefix.
    pub prefix: Option<String>,
}

impl From<xml::name::OwnedName> for OwnedName {
    fn from(other: xml::name::OwnedName) -> Self {
        OwnedName {
            local_name: other.local_name,
            namespace: other.namespace,
            prefix: other.prefix,
        }
    }
}

impl From<OwnedName> for xml::name::OwnedName {
    fn from(other: OwnedName) -> Self {
        xml::name::OwnedName {
            local_name: other.local_name,
            namespace: other.namespace,
            prefix: other.prefix,
        }
    }
}

impl PartialEq<xml::name::OwnedName> for OwnedName {
    fn eq(&self, other: &xml::name::OwnedName) -> bool {
        self.local_name == other.local_name
            && self.namespace == other.namespace
            && self.prefix == other.prefix
    }
}

impl PartialEq<OwnedName> for xml::name::OwnedName {
    fn eq(&self, other: &OwnedName) -> bool {
        self.local_name == other.local_name
            && self.namespace == other.namespace
            && self.prefix == other.prefix
    }
}

impl OwnedName {
    /// Create a new [`OwnedName`] with just the local name part set.
    pub fn from_local_name<I: Into<String>>(name: I) -> Self {
        OwnedName {
            local_name: name.into(),
            namespace: None,
            prefix: None,
        }
    }

    /// Create a borrowed version of this name.
    ///
    /// Useful for using it in a writer.
    pub fn borrow(&self) -> Name {
        Name {
            local_name: &self.local_name,
            namespace: self.namespace.as_deref(),
            prefix: self.prefix.as_deref(),
        }
    }
}

/// Our version of [`xml::attribute::OwnedAttribute`].
///
/// Contrary to its original counterpart, this struct can be made (de)serializable.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "use-serde", derive(Serialize, Deserialize))]
pub struct OwnedAttribute {
    /// Name of the attribute.
    pub name: OwnedName,
    /// Value of the attribute.
    pub value: String,
}

impl From<xml::attribute::OwnedAttribute> for OwnedAttribute {
    fn from(other: xml::attribute::OwnedAttribute) -> Self {
        OwnedAttribute {
            name: other.name.into(),
            value: other.value,
        }
    }
}

impl From<OwnedAttribute> for xml::attribute::OwnedAttribute {
    fn from(other: OwnedAttribute) -> Self {
        xml::attribute::OwnedAttribute {
            name: other.name.into(),
            value: other.value,
        }
    }
}

impl PartialEq<xml::attribute::OwnedAttribute> for OwnedAttribute {
    fn eq(&self, other: &xml::attribute::OwnedAttribute) -> bool {
        self.name == other.name && self.value == other.value
    }
}

impl PartialEq<OwnedAttribute> for xml::attribute::OwnedAttribute {
    fn eq(&self, other: &OwnedAttribute) -> bool {
        self.name == other.name && self.value == other.value
    }
}

impl OwnedAttribute {
    /// Create a borrowed version of this name.
    ///
    /// Useful for using it in a writer.
    pub fn borrow(&self) -> Attribute {
        Attribute {
            name: self.name.borrow(),
            value: &self.value,
        }
    }
}

/// Our version of [`xml::namespace::Namespace`].
///
/// Contrary to its original counterpart, this struct can be made (de)serializable.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "use-serde", derive(Serialize, Deserialize))]
pub struct Namespace(pub BTreeMap<String, String>);

impl From<xml::namespace::Namespace> for Namespace {
    fn from(other: xml::namespace::Namespace) -> Namespace {
        Namespace(other.0)
    }
}

impl From<Namespace> for xml::namespace::Namespace {
    fn from(other: Namespace) -> Self {
        xml::namespace::Namespace(other.0)
    }
}

impl PartialEq<xml::namespace::Namespace> for Namespace {
    fn eq(&self, other: &xml::namespace::Namespace) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<Namespace> for xml::namespace::Namespace {
    fn eq(&self, other: &Namespace) -> bool {
        self.0 == other.0
    }
}

impl Namespace {
    /// Creates a new, empty namespace mapping.
    pub fn new() -> Self {
        Namespace(BTreeMap::new())
    }
}

/// Represents an XML element.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "use-serde", derive(Serialize, Deserialize))]
pub struct Element {
    /// The name of the element.
    pub name: OwnedName,
    /// The attributes of the element.
    pub attributes: Vec<OwnedAttribute>,
    /// The namespace of the element.
    pub namespace: Namespace,
    /// Children nodes.
    pub children: Vec<Node>,
}

impl Element {
    /// Create a new element with the given name, attributes and namespace.
    pub fn new<O: Into<OwnedName>, A: Into<OwnedAttribute>, N: Into<Namespace>>(
        name: O,
        attributes: Vec<A>,
        namespace: N,
    ) -> Self {
        Element {
            name: name.into(),
            attributes: attributes.into_iter().map(Into::into).collect(),
            namespace: namespace.into(),
            children: Vec::new(),
        }
    }

    /// Creates a new element with the given name.
    pub fn with_name(name: OwnedName) -> Self {
        Element::new(name, Vec::<OwnedAttribute>::new(), Namespace::new())
    }

    /// Creates a new element with the given local name, and no prefix and namespace set.
    pub fn with_local_name<I: Into<String>>(name: I) -> Self {
        Element::with_name(OwnedName::from_local_name(name))
    }
}

/// Represents a processing instruction.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "use-serde", derive(Serialize, Deserialize))]
pub struct ProcessingInstruction {
    /// Target of the processing instruction.
    pub name: String,
    /// (Opaque) data of the processing instruction.
    pub data: Option<String>,
}

/// Represents raw characters in the input string, outside of tags.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "use-serde", derive(Serialize, Deserialize))]
pub struct Text(pub String);

/// Represents a comment.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "use-serde", derive(Serialize, Deserialize))]
pub struct Comment(pub String);

/// Represents any XML node.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "use-serde", derive(Serialize, Deserialize))]
pub enum Node {
    Element(Element),
    ProcessingInstruction(ProcessingInstruction),
    Text(Text),
    Comment(Comment),
}

impl From<Element> for Node {
    fn from(element: Element) -> Self {
        Node::Element(element)
    }
}

impl From<ProcessingInstruction> for Node {
    fn from(pi: ProcessingInstruction) -> Self {
        Node::ProcessingInstruction(pi)
    }
}

impl From<Text> for Node {
    fn from(text: Text) -> Self {
        Node::Text(text)
    }
}

impl From<Comment> for Node {
    fn from(comment: Comment) -> Self {
        Node::Comment(comment)
    }
}
