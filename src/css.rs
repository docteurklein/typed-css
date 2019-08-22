use std::fmt;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum PropertyNames {
    background_color,
    color,
    border,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum PropertyValues {
    border,
    blue,
    green,
}

impl fmt::Display for PropertyNames {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PropertyNames::background_color => write!(f, "{}", "background-color"),
            _ => write!(f, "{:?}", self)
        }
    }
}

impl fmt::Display for PropertyValues {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[macro_export]
macro_rules! css {
    ($($selector:ident { $props:block }),*) => {
        $( format!("{} \\{
            {}
        \\}",
            $selector,
            $props
        ) )*
    };
    ($($property:ident: $value:ident;),*) => {
        $( format!("{}: {};",
            $crate::css::PropertyNames::$property,
            $crate::css::PropertyValues::$value,
        ) )*
    };
}
