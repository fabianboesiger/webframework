use super::Content;
use database::Database;

pub struct Tag<'a> {
    name: &'static str,
    inner: Vec<Box<dyn Content + 'a>>,
    attributes: Vec<(&'static str, String)>
}

impl<'a> Tag<'a> {
    pub fn new(name: &'static str) -> Tag {
        Tag {
            name,
            inner: Vec::new(),
            attributes: Vec::new()
        }
    }

    pub fn push<C: Content + 'a>(&mut self, content: C) 
        where C: Content + Send + Sync
    {
        self.inner.push(Box::new(content));
    }

    pub fn append(&mut self, mut content: Vec<Box<dyn Content + 'a>>) {
        self.inner.append(&mut content);
    }

    pub fn attribute(&mut self, key: &'static str, value: String) {
        self.attributes.push((key, value));
    }
}

impl Content for Tag<'_> {
    fn get(&self, database: &Database) -> Vec<u8> {
        let mut attributes = String::new();
        for attribute in &self.attributes {
            attributes.push_str(&format!(" {}={}", attribute.0, attribute.1));
        }
        let mut output = format!("<{}{}>", self.name, attributes).into_bytes();
        for tag in &self.inner {
            output.append(&mut tag.get(database));
        }
        output.append(&mut format!("</{}>", self.name).into_bytes());
        output
    }

    fn post(&self, database: &Database) -> Vec<u8> {
        let mut output = Vec::new();
        for tag in &self.inner {
            output.append(&mut tag.post(database));
        }
        output
    }
}


#[macro_export]
macro_rules! html {
    () => {Vec::new()}; 
    ($tag:ident $( ( $key:ident = $value:expr ) ),* { $($inner:tt)* } $($rest:tt)*) => {
        {
            let mut content: Vec<Box<dyn Content>> = Vec::new();
            let mut tag = Tag::new(stringify!($tag));
            $(
                tag.attribute(stringify!($key), format!("{}", $value));
            )*
            tag.append(html!($($inner)*));
            content.push(Box::new(tag));
            content.append(&mut html!($($rest)*));
            content
        }
    };
    ($content:expr ; $($rest:tt)*) => {
        {
            let mut content: Vec<Box<dyn Content>> = Vec::new();
            content.push(Box::new($content));
            content.append(&mut html!($($rest)*));
            content
        }
    };
}


/*
#[macro_export]
macro_rules! html {
    () => {""}; 
    ($tag:ident $( ( $keys:ident = $values:expr ) ),* [ $($inner:tt)* ] $($rest:tt)*) => {
        /*
        format!("<{tag}{pairs}>{inner}</{tag}>{rest}", 
            tag = stringify!($tag),
            pairs = html!($( $keys = $values )*),
            inner = html!($($inner)*),
            rest = html!($($rest)*)
        )
        */
        Tag::new(stringify!($tag))
    };
    ( ( $key:ident = $value:expr ) $( , ( $keys:ident = $values:expr ) )*) => {
        format!(" {}=\"{}\"{}", stringify!($key), $value, html!($( , ( $keys = $values ) )*))
    };
    ({ $($eval:tt)* } $($rest:tt)*) => {
        format!("{}{}", {$($eval)*}, html!($($rest)*))
    };
    ($content:tt $($rest:tt)*) => {
        format!("{}{}", $content, html!($($rest)*))
    };
}
*/