use crate::component::Link;
use crate::fetch;
use crate::routes::{AdminRoute, AppRoute};
use common::dto::Post;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Model {
    posts: Vec<Post>,
    #[allow(dead_code)]
    fetch_task: fetch::FetchTask,
}

pub enum Msg {
    Posts(Vec<Post>),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|posts: Vec<Post>| Msg::Posts(posts));
        let fetch_task = fetch::FetchService::new().get("/api/admin/posts", callback);
        Self {
            posts: vec![],
            fetch_task,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Posts(posts) => {
                self.posts = posts;
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
          <>
            <p>{"管理画面"}</p>
            {for self.posts.iter().map(|post| self.view_post(post))}
            <Link route=AppRoute::Admin(AdminRoute::Bar)>
              <i class="fas fa-camera"></i>{"Bar へ"}
            </Link>
          </>
        }
    }
}

impl Model {
    fn view_post(&self, post: &Post) -> Html {
        html! {
          <div>
            <h3>
              <Link route=AppRoute::Admin(AdminRoute::PostsEdit(post.id))>
                {&post.title}
              </Link>
            </h3>
          </div>
        }
    }
}
