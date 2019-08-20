
use std::fmt::Display;
use std::fmt;

#[derive(Debug)]
pub struct Property {
    pub name: PropertyName,
    pub value: String,
}

#[derive(Debug)]
pub enum PropertyName {
    color,
    background_color,
}

impl Display for PropertyName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            background_color => "background-color".to_string(),
            _ => self.to_string(),
        };

        write!(f, "{}", name)
    }
}

struct Block {
    selector: String,
    properties: Vec<Property>,
}

impl Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {};", self.name, self.value)
    }
}

#[macro_export]
macro_rules! css {
    ($name:ident: $value:ident;) => {
        format!("{}", css::Property {
            name: $crate::css::PropertyName::$name,
            value: stringify!($value).to_string(),
        })
    };
}
