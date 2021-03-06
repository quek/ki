use crate::component::Link;
use crate::generated::post::Post;
use crate::routes::AppRoute;
use crate::{fetch, utils};
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
    pub id: i32,
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

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
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
                utils::set_title(&post.title);
                html! {
                  <>
                    <Link route=AppRoute::Index>{"top"}</Link>
                    <div  class="post">
                      <h1>{&post.title}</h1>
                      <div>
                        <i class="fas fa-upload"></i>
                        {post.published_at.unwrap().format("%Y-%m-%d %H:%M:%S")}
                        <i class="fas fa-history"></i>
                        {post.updated_at.format("%Y-%m-%d %H:%M:%S")}
                      </div>
                      {crate::markdown::node(&post.body)}
                    </div>
                  </>
                }
            }
        }
    }
}
