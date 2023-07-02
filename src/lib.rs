use axum::response::{Html, IntoResponse, Response};

pub trait Render {
    fn render(&self) -> String;
}

#[derive(Clone)]
enum TagType {
    Normal,
    Orphan,
    Slash,
    DocType,
}

// El is short for element.
#[derive(Clone)]
pub struct Element {
    name: String,
    class: Vec<String>,
    attrs: Vec<String>,
    content: String,
    tag_type: TagType,
}

pub type El = Element;

pub struct Elements {
    elements: Vec<El>,
}

impl Render for Element {
    fn render(&self) -> String {
        let mut tag = String::new();
        tag += &self.name;
        if !self.attrs.is_empty() {
            let attrs_str = self.attrs.join(" ");
            tag += " ";
            tag += &attrs_str;
        }

        if !self.class.is_empty() {
            let class_str = self.class.join(" ");
            let class_str = format!(" class=\"{}\"", class_str);
            tag += &class_str;
        }

        match self.tag_type {
            TagType::Normal => {
                format!("<{}>{}</{}>", tag, self.content, self.name)
            }
            TagType::Orphan => format!("<{}>", tag),
            TagType::Slash => format!("<{} />", tag),
            TagType::DocType => {
                format!(
                    "<!DOCTYPE html><{}>{}</{}>",
                    tag, self.content, self.name
                )
            }
        }
    }
}

impl Render for Vec<Element> {
    fn render(&self) -> String {
        let ss: Vec<String> = self.into_iter().map(|el| el.render()).collect();
        ss.join("")
    }
}

impl Render for Elements {
    fn render(&self) -> String {
        self.elements.render()
    }
}

impl IntoResponse for Element {
    fn into_response(self) -> Response {
        Html(self.render()).into_response()
    }
}

impl IntoResponse for Elements {
    fn into_response(self) -> Response {
        Html(self.render()).into_response()
    }
}

impl Render for &str {
    fn render(&self) -> String {
        self.to_string()
    }
}

impl Render for String {
    fn render(&self) -> String {
        self.to_string()
    }
}

impl Render for i64 {
    fn render(&self) -> String {
        self.to_string()
    }
}

impl Elements {
    pub fn new() -> Self {
        Elements {
            elements: Vec::new(),
        }
    }

    pub fn push(mut self, el: Element) -> Self {
        self.elements.push(el);
        self
    }

    pub fn append(mut self, els: &mut Elements) -> Self {
        self.elements.append(&mut els.elements);
        self
    }
}

impl Element {
    pub fn new(name: impl Into<String>) -> Self {
        Element {
            name: name.into(),
            class: Vec::new(),
            attrs: Vec::new(),
            content: String::new(),
            tag_type: TagType::Normal,
        }
    }

    pub fn push(self, el: Element) -> Elements {
        Elements::new().push(self).push(el)
    }

    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class.push(class.into());
        self
    }

    pub fn attr(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.attrs
            .push(format!("{}=\"{}\"", key.into(), value.into()));
        self
    }

    pub fn orphan_attr(mut self, key: impl Into<String>) -> Self {
        self.attrs.push(key.into());
        self
    }

    pub fn child(mut self, child: impl Render) -> Self {
        self.content += &child.render();
        self
    }

    pub fn orphan(mut self) -> Self {
        self.tag_type = TagType::Orphan;
        self
    }

    pub fn slash(mut self) -> Self {
        self.tag_type = TagType::Slash;
        self
    }

    pub fn doctype(mut self) -> Self {
        self.tag_type = TagType::DocType;
        self
    }

    // Below is some helper functions.
    pub fn id(self, value: impl Into<String>) -> Self {
        self.attr("id", value)
    }

    pub fn charset(self, value: impl Into<String>) -> Self {
        self.attr("charset", value)
    }

    pub fn src(self, value: impl Into<String>) -> Self {
        self.attr("src", value)
    }

    pub fn rel(self, value: impl Into<String>) -> Self {
        self.attr("rel", value)
    }

    pub fn href(self, value: impl Into<String>) -> Self {
        self.attr("href", value)
    }

    pub fn r#type(self, value: impl Into<String>) -> Self {
        self.attr("type", value)
    }

    pub fn name(self, value: impl Into<String>) -> Self {
        self.attr("name", value)
    }

    pub fn value(self, value: impl Into<String>) -> Self {
        self.attr("value", value)
    }

    pub fn r#for(self, value: impl Into<String>) -> Self {
        self.attr("for", value)
    }
}

pub fn html() -> Element {
    Element::new("html").doctype()
}

pub fn head() -> Element {
    Element::new("head")
}

pub fn body() -> Element {
    Element::new("body")
}

pub fn meta() -> Element {
    Element::new("meta").slash()
}

pub fn script() -> Element {
    Element::new("script")
}

pub fn div() -> Element {
    Element::new("div")
}

pub fn table() -> Element {
    Element::new("table")
}

pub fn thead() -> Element {
    Element::new("thead")
}

pub fn tbody() -> Element {
    Element::new("tbody")
}

pub fn th() -> Element {
    Element::new("th")
}

pub fn tr() -> Element {
    Element::new("tr")
}

pub fn td() -> Element {
    Element::new("td")
}

pub fn input() -> Element {
    Element::new("input")
}

pub fn button() -> Element {
    Element::new("button")
}

pub fn form() -> Element {
    Element::new("form")
}

pub fn link() -> Element {
    Element::new("link").slash()
}

pub fn title() -> Element {
    Element::new("title")
}

pub fn p() -> El {
    El::new("p")
}

pub fn label() -> El {
    El::new("label")
}

pub fn br() -> El {
    El::new("br").slash()
}

pub fn h1() -> El {
    El::new("h1")
}

pub fn h2() -> El {
    El::new("h2")
}

pub fn h3() -> El {
    El::new("h3")
}

pub fn h4() -> El {
    El::new("h4")
}

pub fn h5() -> El {
    El::new("h5")
}

pub fn h6() -> El {
    El::new("h6")
}

pub fn a() -> El {
    El::new("a")
}

pub fn ul() -> El {
    El::new("ul")
}

pub fn ol() -> El {
    El::new("ol")
}

pub fn li() -> El {
    El::new("li")
}

impl Element {
    // htmx
    pub fn hx_post(self, value: impl Into<String>) -> Self {
        self.attr("hx-post", value)
    }

    pub fn hx_target(self, value: impl Into<String>) -> Self {
        self.attr("hx-target", value)
    }

    pub fn hx_ext(self, value: impl Into<String>) -> Self {
        self.attr("hx-ext", value)
    }

    pub fn hx_swap(self, value: impl Into<String>) -> Self {
        self.attr("hx-swap", value)
    }
}
