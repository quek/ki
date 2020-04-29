use crate::common::models::Post;
use crate::common::types::PostStatus;
use crate::component::Link;
use crate::fetch;
use crate::routes::{AdminRoute, AppRoute};
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

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
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
            <h1>{"記 管理画面"}</h1>
            <Link route=AppRoute::Admin(AdminRoute::PostsNew)>
                {"新規作成"}
            </Link>
            {for self.posts.iter().map(|post| self.view_post(post))}
          </>
        }
    }
}

impl Model {
    fn view_post(&self, post: &Post) -> Html {
        html! {
          <div class="posts">
            <h3>
              {if post.status == PostStatus::Published {
                  html! { <i class="fas fa-upload"></i> }
              } else {
                  html! {""}
              }}
              <Link route=AppRoute::Admin(AdminRoute::PostsEdit(post.id))>
                {&post.title}
              </Link>
            </h3>
            <div>
              <i class="fas fa-upload"></i>
              {post.published_at.map_or("-".to_string(), |x| x.format("%Y-%m-%d %H:%M:%S").to_string())}
              <i class="fas fa-history"></i>
              {post.updated_at.format("%Y-%m-%d %H:%M:%S")}
            </div>
          </div>
        }
    }
}
