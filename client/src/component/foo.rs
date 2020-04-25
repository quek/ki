use anyhow::Error;
use yew::format::{Json, Nothing};
use yew::services::fetch;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Model {
    message: String,
    #[allow(dead_code)]
    fetch_task: fetch::FetchTask,
}

pub enum Msg {
    Foo(String),
    Unauthorized,
    ApiError(Option<Error>),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(
            move |response: fetch::Response<Json<Result<String, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                if meta.status.is_success() {
                    match data {
                        Ok(message) => Msg::Foo(message),
                        Err(e) => Msg::ApiError(Some(e)),
                    }
                } else if meta.status == 401 {
                    Msg::Unauthorized
                } else {
                    Msg::ApiError(None)
                }
            },
        );
        let request = fetch::Request::get("/api/hello")
            .header("Content-Type", "application/json")
            .body(Nothing)
            .unwrap();
        let fetch_task = fetch::FetchService::new().fetch(request, callback).unwrap();

        Self {
            message: "デフォルト".to_string(),
            fetch_task,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Foo(message) => {
                self.message = message;
                true
            }
            _ => false,
        }
    }

    fn view(&self) -> Html {
        html! {
            <p>{&self.message}</p>
        }
    }
}
