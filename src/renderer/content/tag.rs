use super::Content;

pub struct Tag<'a> {
    name: &'static str,
    inner: Vec<Box<dyn Content + 'a>>,
}

impl<'a> Tag<'a> {
    pub fn new(name: &'static str) -> Tag {
        Tag {
            name,
            inner: Vec::new(),
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
    }
}

impl Content for Tag<'_> {
    fn get(&self) -> Vec<u8> {
        let mut output = format!("<{}>", self.name).into_bytes();
        for tag in &self.inner {
            output.append(&mut tag.get());
        }
        output.append(&mut format!("</{}>", self.name).into_bytes());
        output
    }

    fn post(&self) -> Vec<u8> {
        let mut output = Vec::new();
        for tag in &self.inner {
            output.append(&mut tag.post());
        }
        output
    }
}


#[macro_export]
macro_rules! html {
    () => {Vec::new()}; 
    ($tag:ident $( ( $key:ident = $value:expr ) ),* [ $($inner:tt)* ] $($rest:tt)*) => {
        {
            let mut tags: Vec<Box<dyn Content>> = Vec::new();
            let mut tag = Tag::new(stringify!($tag));
            $(
                tag.attribute(stringify!($key), format!("{}", $value));
            )*
            tag.append(html!($($inner)*));
            tags.push(Box::new(tag));
            tags.append(&mut html!($($rest)*));
            tags
        }
    };
    ({ $($eval:tt)* } $($rest:tt)*) => {
        format!("{}{}", {$($eval)*}, html!($($rest)*))
    };
    ($content:tt $($rest:tt)*) => {
        format!("{}{}", $content, html!($($rest)*))
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