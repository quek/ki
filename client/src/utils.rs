use crate::routes::AppRoute;
use anyhow::Error;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement, Node, ScrollBehavior, ScrollToOptions};
use yew::virtual_dom::VNode;
use yew_router::agent::{RouteAgentDispatcher, RouteRequest};
use yew_router::route::Route;

pub fn alert(message: &str) {
    web_sys::window()
        .unwrap()
        .alert_with_message(message)
        .unwrap();
}

pub fn change_route(route: AppRoute) {
    RouteAgentDispatcher::<()>::new().send(RouteRequest::ChangeRoute(Route::from(route)));
}

pub fn disable_buttons() {
    let document = web_sys::window().unwrap().document().unwrap();
    let node_list = document
        .query_selector_all("form button, form input[type='submit']")
        .unwrap();
    for i in 0..node_list.length() {
        let node = node_list.get(i).unwrap();
        let element = node.dyn_ref::<HtmlElement>().unwrap();
        element.set_attribute("disabled", "true").unwrap();
        element.dataset().set("disabled", "true").unwrap();
    }
}

pub fn enable_buttons() {
    let document = web_sys::window().unwrap().document().unwrap();
    let node_list = document.query_selector_all("[data-disabled=true]").unwrap();
    for i in 0..node_list.length() {
        let node = node_list.get(i).unwrap();
        let element = node.dyn_ref::<HtmlElement>().unwrap();
        element.remove_attribute("disabled").unwrap();
    }
}

pub fn handle_api_error(error: Option<Error>) -> bool {
    if let Some(error) = error {
        web_sys::console::error_1(&format!("{:?}", error).into());
    }
    alert("リロード後にもう一度実行してください。");
    true
}

pub fn handle_unauthorized() -> () {
    alert("ログインしてください。");
    set_location("/login");
}

pub fn markdown(text: &str) -> String {
    use crate::prism::{highlight, languages};
    use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag};

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    let mut lang: Option<String> = None;
    let mut in_code = false;
    let mut codes = String::new();
    let parser = Parser::new_ext(text, options).map(|event| match event {
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(x))) => {
            lang = Some(x.to_string());
            in_code = true;
            Event::Text("".into())
        }
        Event::Start(Tag::CodeBlock(_)) => {
            lang = None;
            in_code = true;
            Event::Text("".into())
        }
        Event::End(Tag::CodeBlock(_)) => {
            in_code = false;
            let html = highlight(
                codes.clone(),
                languages.get(lang.as_ref().unwrap_or(&"text".to_string()).to_string()),
            );
            codes = String::new();
            lang = None;
            Event::Html(format!("<pre><code>{}</code></pre>", html).into())
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

pub fn markdown_node(text: &str) -> VNode {
    let div = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();
    div.set_inner_html(&markdown(text));
    VNode::VRef(Node::from(div))
}

pub fn scroll_to_error() {
    let callback = Closure::wrap(Box::new(|| {
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.query_selector(".error").unwrap();
        if let Some(element) = element {
            let body_rect = document.body().unwrap().get_bounding_client_rect();
            let element_rect = element.get_bounding_client_rect();
            let offset = element_rect.top() - body_rect.top() - 80.0;
            web_sys::window().unwrap().scroll_to_with_scroll_to_options(
                ScrollToOptions::new()
                    .top(offset)
                    .behavior(ScrollBehavior::Smooth),
            );
        }
    }) as Box<dyn Fn()>);
    web_sys::window()
        .unwrap()
        .set_timeout_with_callback_and_timeout_and_arguments_0(callback.as_ref().unchecked_ref(), 0)
        .unwrap();
    callback.forget();
}

pub fn set_location(url: &str) -> () {
    let window = window().unwrap();
    window.location().set_href(url).unwrap();
}

pub fn toast(message: &str) {
    let document = web_sys::window().unwrap().document().unwrap();
    if let Some(toast) = document.get_element_by_id("toast") {
        let toast = toast.dyn_ref::<HtmlElement>().unwrap();
        toast.set_inner_text(message);
        toast.set_class_name("show");
        let callback = Closure::wrap(Box::new(|| {
            let document = web_sys::window().unwrap().document().unwrap();
            if let Some(toast) = document.get_element_by_id("toast") {
                let toast = toast.dyn_ref::<HtmlElement>().unwrap();
                toast.set_class_name("");
            }
        }) as Box<dyn Fn()>);
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                3000,
            )
            .unwrap();
        callback.forget();
    }
}
