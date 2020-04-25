use crate::component::Link;
use crate::fetch;
use crate::routes::AppRoute;
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
        let fetch_task = fetch::FetchService::new().get("/api/posts", callback);
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
            <div>{"記事一覧"}</div>
            {for self.posts.iter().map(|post| self.view_post(post))}
          </>
        }
    }
}

impl Model {
    fn view_post(&self, post: &Post) -> Html {
        html! {
          <div>
            <h3>
              <Link route=AppRoute::PostsShow(post.id)>
                {&post.title}
              </Link>
            </h3>
            <div>
              <i class="fas fa-upload"></i>
              {post.published_at.unwrap().format("%Y-%m-%d %H:%M:%S")}
            </div>
            <div>
              <i class="fas fa-history"></i>
              {post.updated_at.format("%Y-%m-%d %H:%M:%S")}
            </div>
          </div>
        }
    }
}
