#[cfg(feature = "htmx")]
pub mod htmx;

#[cfg(feature = "axum")]
pub mod shapeless_axum;

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

#[derive(Clone)]
pub struct Element {
    name: String,
    class: Vec<String>,
    attrs: Vec<String>,
    content: String,
    tag_type: TagType,
}

pub struct Elements {
    elements: Vec<Element>,
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
                format!("<!DOCTYPE html><{}>{}</{}>", tag, self.content, self.name)
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

    pub fn from(el: Element) -> Self {
        let mut v: Vec<Element> = Vec::new();
        v.push(el);
        Elements { elements: v }
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

    pub fn to_elements(self) -> Elements {
        Elements::from(self)
    }

    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class.push(class.into());
        self
    }

    pub fn tw(self, tw: impl Into<String>) -> Self {
        // This is a tailwind helper class, it has no difference than
        // using plain class()
        self.class(tw)
    }

    pub fn attr(mut self, key: impl Render, value: impl Render) -> Self {
        self.attrs
            .push(format!("{}=\"{}\"", key.render(), value.render()));
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
}

impl Element {
    // Below is some helper functions.
    pub fn accept(self, value: impl Render) -> Self {
        self.attr("accept", value)
    }

    pub fn accesskey(self, value: impl Render) -> Self {
        self.attr("accesskey", value)
    }

    pub fn action(self, value: impl Render) -> Self {
        self.attr("action", value)
    }

    pub fn alt(self, value: impl Render) -> Self {
        self.attr("alt", value)
    }

    pub fn r#async(self, value: impl Render) -> Self {
        self.attr("async", value)
    }

    pub fn autocomplete(self, value: impl Render) -> Self {
        self.attr("autocomplete", value)
    }

    pub fn autofocus(self, value: impl Render) -> Self {
        self.attr("autofocus", value)
    }

    pub fn autoplay(self, value: impl Render) -> Self {
        self.attr("autoplay", value)
    }

    pub fn checked(self, value: impl Render) -> Self {
        self.attr("checked", value)
    }

    pub fn contenteditable(self, value: impl Render) -> Self {
        self.attr("contenteditable", value)
    }

    pub fn controls(self, value: impl Render) -> Self {
        self.attr("controls", value)
    }

    pub fn data(self, value: impl Render) -> Self {
        self.attr("data", value)
    }

    pub fn datetime(self, value: impl Render) -> Self {
        self.attr("datetime", value)
    }

    pub fn defer(self, value: impl Render) -> Self {
        self.attr("defer", value)
    }

    pub fn dir(self, value: impl Render) -> Self {
        self.attr("dir", value)
    }

    pub fn disabled(self, value: impl Render) -> Self {
        self.attr("disabled", value)
    }

    pub fn download(self, value: impl Render) -> Self {
        self.attr("download", value)
    }

    pub fn draggable(self, value: impl Render) -> Self {
        self.attr("draggable", value)
    }

    pub fn enctype(self, value: impl Render) -> Self {
        self.attr("enctype", value)
    }

    pub fn r#for(self, value: impl Render) -> Self {
        self.attr("for", value)
    }

    pub fn form(self, value: impl Render) -> Self {
        self.attr("form", value)
    }

    pub fn headers(self, value: impl Render) -> Self {
        self.attr("headers", value)
    }

    pub fn height(self, value: impl Render) -> Self {
        self.attr("height", value)
    }

    pub fn hidden(self, value: impl Render) -> Self {
        self.attr("hidden", value)
    }

    pub fn href(self, value: impl Render) -> Self {
        self.attr("href", value)
    }

    pub fn id(self, value: impl Render) -> Self {
        self.attr("id", value)
    }

    pub fn ismap(self, value: impl Render) -> Self {
        self.attr("ismap", value)
    }

    pub fn itemprop(self, value: impl Render) -> Self {
        self.attr("itemprop", value)
    }

    pub fn lang(self, value: impl Render) -> Self {
        self.attr("lang", value)
    }

    pub fn list(self, value: impl Render) -> Self {
        self.attr("list", value)
    }

    pub fn max(self, value: impl Render) -> Self {
        self.attr("max", value)
    }

    pub fn maxlength(self, value: impl Render) -> Self {
        self.attr("maxlength", value)
    }

    pub fn media(self, value: impl Render) -> Self {
        self.attr("media", value)
    }

    pub fn method(self, value: impl Render) -> Self {
        self.attr("method", value)
    }

    pub fn min(self, value: impl Render) -> Self {
        self.attr("min", value)
    }

    pub fn multiple(self, value: impl Render) -> Self {
        self.attr("multiple", value)
    }

    pub fn name(self, value: impl Render) -> Self {
        self.attr("name", value)
    }

    pub fn novalidate(self, value: impl Render) -> Self {
        self.attr("novalidate", value)
    }

    pub fn pattern(self, value: impl Render) -> Self {
        self.attr("pattern", value)
    }

    pub fn placeholder(self, value: impl Render) -> Self {
        self.attr("placeholder", value)
    }

    pub fn readonly(self, value: impl Render) -> Self {
        self.attr("readonly", value)
    }

    pub fn rel(self, value: impl Render) -> Self {
        self.attr("rel", value)
    }

    pub fn required(self, value: impl Render) -> Self {
        self.attr("required", value)
    }

    pub fn rows(self, value: impl Render) -> Self {
        self.attr("rows", value)
    }

    pub fn selected(self, value: impl Render) -> Self {
        self.attr("selected", value)
    }

    pub fn src(self, value: impl Render) -> Self {
        self.attr("src", value)
    }

    pub fn step(self, value: impl Render) -> Self {
        self.attr("step", value)
    }

    pub fn style(self, value: impl Render) -> Self {
        self.attr("style", value)
    }

    pub fn tabindex(self, value: impl Render) -> Self {
        self.attr("tabindex", value)
    }

    pub fn target(self, value: impl Render) -> Self {
        self.attr("target", value)
    }

    pub fn title(self, value: impl Render) -> Self {
        self.attr("title", value)
    }

    pub fn r#type(self, value: impl Render) -> Self {
        self.attr("type", value)
    }

    pub fn value(self, value: impl Render) -> Self {
        self.attr("value", value)
    }

    pub fn width(self, value: impl Render) -> Self {
        self.attr("width", value)
    }
}

// HTML tags.
pub fn html() -> Element {
    Element::new("html").doctype()
}

pub fn abbr() -> Element {
    Element::new("abbr")
}

pub fn address() -> Element {
    Element::new("address")
}

pub fn article() -> Element {
    Element::new("article")
}

pub fn aside() -> Element {
    Element::new("aside")
}

pub fn audio() -> Element {
    Element::new("audio")
}

pub fn b() -> Element {
    Element::new("b")
}

pub fn bdi() -> Element {
    Element::new("bdi")
}

pub fn bdo() -> Element {
    Element::new("bdo")
}

pub fn blockquote() -> Element {
    Element::new("blockquote")
}

pub fn body() -> Element {
    Element::new("body")
}

pub fn button() -> Element {
    Element::new("button")
}

pub fn canvas() -> Element {
    Element::new("canvas")
}

pub fn caption() -> Element {
    Element::new("caption")
}

pub fn cite() -> Element {
    Element::new("cite")
}

pub fn code() -> Element {
    Element::new("code")
}

pub fn colgroup() -> Element {
    Element::new("colgroup")
}

pub fn data() -> Element {
    Element::new("data")
}

pub fn datalist() -> Element {
    Element::new("datalist")
}

pub fn dd() -> Element {
    Element::new("dd")
}

pub fn del() -> Element {
    Element::new("del")
}

pub fn details() -> Element {
    Element::new("details")
}

pub fn dfn() -> Element {
    Element::new("dfn")
}

pub fn dialog() -> Element {
    Element::new("dialog")
}

pub fn div() -> Element {
    Element::new("div")
}

pub fn dl() -> Element {
    Element::new("dl")
}

pub fn dt() -> Element {
    Element::new("dt")
}

pub fn em() -> Element {
    Element::new("em")
}

pub fn fieldset() -> Element {
    Element::new("fieldset")
}

pub fn figcaption() -> Element {
    Element::new("figcaption")
}

pub fn figure() -> Element {
    Element::new("figure")
}

pub fn footer() -> Element {
    Element::new("footer")
}

pub fn form() -> Element {
    Element::new("form")
}

pub fn h1() -> Element {
    Element::new("h1")
}

pub fn h2() -> Element {
    Element::new("h2")
}

pub fn h3() -> Element {
    Element::new("h3")
}

pub fn h4() -> Element {
    Element::new("h4")
}

pub fn h5() -> Element {
    Element::new("h5")
}

pub fn h6() -> Element {
    Element::new("h6")
}

pub fn head() -> Element {
    Element::new("head")
}

pub fn header() -> Element {
    Element::new("header")
}

pub fn i() -> Element {
    Element::new("i")
}

pub fn iframe() -> Element {
    Element::new("iframe")
}

pub fn ins() -> Element {
    Element::new("ins")
}

pub fn kbd() -> Element {
    Element::new("kbd")
}

pub fn label() -> Element {
    Element::new("label")
}

pub fn legend() -> Element {
    Element::new("legend")
}

pub fn li() -> Element {
    Element::new("li")
}

pub fn main() -> Element {
    Element::new("main")
}

pub fn map() -> Element {
    Element::new("map")
}

pub fn mark() -> Element {
    Element::new("mark")
}

pub fn meter() -> Element {
    Element::new("meter")
}

pub fn nav() -> Element {
    Element::new("nav")
}

pub fn noscript() -> Element {
    Element::new("noscript")
}

pub fn object() -> Element {
    Element::new("object")
}

pub fn ol() -> Element {
    Element::new("ol")
}

pub fn optgroup() -> Element {
    Element::new("optgroup")
}

pub fn option() -> Element {
    Element::new("option")
}

pub fn output() -> Element {
    Element::new("output")
}

pub fn p() -> Element {
    Element::new("p")
}

pub fn picture() -> Element {
    Element::new("picture")
}

pub fn pre() -> Element {
    Element::new("pre")
}

pub fn progress() -> Element {
    Element::new("progress")
}

pub fn q() -> Element {
    Element::new("q")
}

pub fn rp() -> Element {
    Element::new("rp")
}

pub fn rt() -> Element {
    Element::new("rt")
}

pub fn ruby() -> Element {
    Element::new("ruby")
}

pub fn s() -> Element {
    Element::new("s")
}

pub fn samp() -> Element {
    Element::new("samp")
}

pub fn section() -> Element {
    Element::new("section")
}

pub fn select() -> Element {
    Element::new("select")
}

pub fn small() -> Element {
    Element::new("small")
}

pub fn span() -> Element {
    Element::new("span")
}

pub fn strong() -> Element {
    Element::new("strong")
}

pub fn sub() -> Element {
    Element::new("sub")
}

pub fn sup() -> Element {
    Element::new("sup")
}

pub fn table() -> Element {
    Element::new("table")
}

pub fn tbody() -> Element {
    Element::new("tbody")
}

pub fn td() -> Element {
    Element::new("td")
}

pub fn textarea() -> Element {
    Element::new("textarea")
}

pub fn tfoot() -> Element {
    Element::new("tfoot")
}

pub fn th() -> Element {
    Element::new("th")
}

pub fn thead() -> Element {
    Element::new("thead")
}

pub fn time() -> Element {
    Element::new("time")
}

pub fn title() -> Element {
    Element::new("title")
}

pub fn tr() -> Element {
    Element::new("tr")
}

pub fn u() -> Element {
    Element::new("u")
}

pub fn ul() -> Element {
    Element::new("ul")
}

pub fn var() -> Element {
    Element::new("var")
}

pub fn video() -> Element {
    Element::new("video")
}

pub fn area() -> Element {
    Element::new("area").slash()
}

pub fn base() -> Element {
    Element::new("base").slash()
}

pub fn br() -> Element {
    Element::new("br").slash()
}

pub fn col() -> Element {
    Element::new("col").slash()
}

pub fn embed() -> Element {
    Element::new("embed").slash()
}

pub fn hr() -> Element {
    Element::new("hr").slash()
}

pub fn img() -> Element {
    Element::new("img").slash()
}

pub fn input() -> Element {
    Element::new("input").slash()
}

pub fn link() -> Element {
    Element::new("link").slash()
}

pub fn meta() -> Element {
    Element::new("meta").slash()
}

pub fn param() -> Element {
    Element::new("param").slash()
}

pub fn source() -> Element {
    Element::new("source").slash()
}

pub fn track() -> Element {
    Element::new("track").slash()
}

pub fn wbr() -> Element {
    Element::new("wbr").slash()
}
