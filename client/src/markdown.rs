use crate::prism::{highlight, languages};
use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag};
use web_sys::Node;
use yew::virtual_dom::VNode;

pub fn node(s: &str) -> VNode {
    let div = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();
    div.set_inner_html(&text(s));
    VNode::VRef(Node::from(div))
}

fn text(text: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    let mut in_code = false;
    let mut lang: Option<String> = None;
    let mut codes = String::new();
    let parser = Parser::new_ext(text, options).map(|event| match event {
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(x))) => {
            in_code = true;
            lang = Some(x.to_string());
            Event::Text("".into())
        }
        Event::Start(Tag::CodeBlock(_)) => {
            in_code = true;
            lang = None;
            Event::Text("".into())
        }
        Event::End(Tag::CodeBlock(_)) => {
            in_code = false;
            let mut code_class = None;
            let html = match &lang {
                Some(lang) => match languages.get(lang.to_string()) {
                    Some(syntax) => {
                        code_class = Some(format!("{}-language", lang).replace("\"", "&quot;"));
                        highlight(codes.clone(), syntax, lang.to_string())
                    }
                    None => codes.clone(),
                },
                _ => codes.clone(),
            };
            lang = None;
            codes = String::new();
            match code_class {
                Some(class) => Event::Html(
                    format!("<pre><code class=\"{}\">{}</code></pre>", class, html).into(),
                ),
                _ => Event::Html(format!("<pre><code>{}</code></pre>", html).into()),
            }
        }
        Event::Text(text) => {
            if in_code {
                codes += text.as_ref();
                Event::Text("".into())
            } else {
                Event::Text(text)
            }
        }
        _ => event,
    });
    let mut html_output: String = String::with_capacity(text.len() * 3 / 2);
    html::push_html(&mut html_output, parser);
    html_output
}
