use crate::common::dto::{Id, Post};
use crate::common::form::{PostErrors, PostForm};
use crate::component::admin::posts::form;
use crate::fetch;
use crate::utils;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct Model {
    link: ComponentLink<Self>,
    props: Props,
    post: Option<Post>,
    errors: PostErrors,
    #[allow(dead_code)]
    fetch_task: fetch::FetchTask,
}

pub enum Msg {
    Post(Post),
    Submit(PostForm),
    SubmitError(PostErrors),
    Updated,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub id: Id,
}

impl Component for Model {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|post: Post| Msg::Post(post));
        let fetch_task = fetch::FetchService::new()
            .get(&format!("/api/admin/posts/{}/edit", props.id), callback);
        Self {
            link,
            props,
            post: None,
            errors: PostErrors::default(),
            fetch_task,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Post(post) => {
                self.post = Some(post);
                true
            }
            Msg::Submit(form) => {
                self.fetch_task = fetch::FetchService::new().put(
                    &format!("/api/admin/posts/{}", self.props.id),
                    &form,
                    self.link
                        .callback(|response: Result<(), PostErrors>| match response {
                            Ok(()) => Msg::Updated,
                            Err(errors) => Msg::SubmitError(errors),
                        }),
                );
                false
            }
            Msg::SubmitError(errors) => {
                self.errors = errors;
                true
            }
            Msg::Updated => {
                utils::toast("更新しました。");
                true
            }
        }
    }

    fn view(&self) -> Html {
        let callback = self.link.callback(|form: PostForm| Msg::Submit(form));
        match &self.post {
            None => html! { "" },
            Some(post) => html! {
              <>
                <h1>{"編集"}</h1>
                <form::Model button_label="保存", onsubmit=callback post=post errors=&self.errors />
              </>
            },
        }
    }
}
