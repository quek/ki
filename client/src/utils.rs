use crate::routes::AppRoute;
use anyhow::Error;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement, ScrollBehavior, ScrollToOptions};
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

pub fn change_route_with_query(route: AppRoute, query: &str) {
    let mut x = Route::from(route);
    x.route = format!("{}?{}", x.route, query);
    RouteAgentDispatcher::<()>::new().send(RouteRequest::ChangeRoute(x));
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

pub fn query_string() -> String {
    web_sys::window()
        .unwrap()
        .location()
        .search()
        .unwrap_or("".to_string())
        .replace("?", "")
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

pub fn set_title(title: &str) {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .set_title(title);
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
