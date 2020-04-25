use crate::component::Link;
use crate::fetch;
use crate::routes::AppRoute;
use crate::utils;
use common::dto::{Id, Post};
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct Model {
    post: Option<Post>,
    #[allow(dead_code)]
    fetch_task: fetch::FetchTask,
}

pub enum Msg {
    Post(Post),
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
        let fetch_task =
            fetch::FetchService::new().get(&format!("/api/posts/{}", props.id), callback);
        Self {
            post: None,
            fetch_task,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Post(post) => {
                self.post = Some(post);
                true
            }
        }
    }

    fn view(&self) -> Html {
        match self.post {
            None => html! { "" },
            Some(ref post) => {
                html! {
                  <>
                    <Link route=AppRoute::Index>{"top"}</Link>
                    <div>{&post.title}</div>
                    {utils::markdown_node(&post.body)}
                  </>
                }
            }
        }
    }
}
