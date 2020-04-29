use crate::common::form::{PostErrors, PostForm};
use crate::component::admin::posts::form;
use crate::fetch;
use crate::routes::{AdminRoute, AppRoute};
use crate::utils;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Model {
    link: ComponentLink<Self>,
    errors: PostErrors,
    #[allow(dead_code)]
    fetch_task: Option<fetch::FetchTask>,
}

pub enum Msg {
    Submit(PostForm),
    SubmitError(PostErrors),
    Created,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            errors: PostErrors::default(),
            fetch_task: None,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Submit(form) => {
                let callback =
                    self.link
                        .callback(|response: Result<(), PostErrors>| match response {
                            Ok(()) => Msg::Created,
                            Err(errors) => Msg::SubmitError(errors),
                        });
                let fetch_task =
                    fetch::FetchService::new().post("/api/admin/posts", &form, callback);
                self.fetch_task = Some(fetch_task);
                false
            }
            Msg::SubmitError(errors) => {
                self.errors = errors;
                true
            }
            Msg::Created => {
                utils::toast("登録しました。");
                utils::change_route(AppRoute::Admin(AdminRoute::Index));
                true
            }
        }
    }

    fn view(&self) -> Html {
        let callback = self.link.callback(|form: PostForm| Msg::Submit(form));
        html! {
          <>
            <h1>{"新規"}</h1>
            <form::Model button_label="登録", onsubmit=callback errors=&self.errors />
          </>
        }
    }
}
