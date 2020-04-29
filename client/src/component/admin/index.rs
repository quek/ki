use crate::common::models::{Post, PostQuery, PER_PAGE};
use crate::common::types::PostStatus;
use crate::component::pager;
use crate::component::Link;
use crate::fetch;
use crate::routes::{AdminRoute, AppRoute};
use crate::utils;
use yew::{html, Callback, Component, ComponentLink, Html, ShouldRender};

pub struct Model {
    posts: Vec<Post>,
    total_count: i64,
    query: PostQuery,
    pager_callback: Callback<i64>,
    #[allow(dead_code)]
    fetch_task: fetch::FetchTask,
}

pub enum Msg {
    Posts((Vec<Post>, i64)),
    Page(i64),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let query = serde_qs::from_str(&utils::query_string()).unwrap_or(PostQuery::default());
        let pager_callback = link.callback(move |page: i64| Msg::Page(page));
        let callback = link.callback(|posts: (Vec<Post>, i64)| Msg::Posts(posts));
        let fetch_task = fetch::FetchService::new().get(
            &format!(
                "/api/admin/posts?{}",
                serde_qs::to_string(&query).unwrap_or("".to_string())
            ),
            callback,
        );
        Self {
            posts: vec![],
            total_count: 0,
            query,
            pager_callback,
            fetch_task,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Posts((posts, total_count)) => {
                self.posts = posts;
                self.total_count = total_count;
                true
            }
            Msg::Page(page) => {
                self.query.page = page;
                let query_string = serde_qs::to_string(&self.query).unwrap();
                web_sys::console::log_1(&format!("{:?}", &query_string).into());
                utils::change_route_with_query(AppRoute::Admin(AdminRoute::Posts), &query_string);
                // TODO reload
                false
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
            <pager::Model
              page={self.query.page}
              total={self.total_count}
              per_page={PER_PAGE}
              callback={self.pager_callback.clone()}
              />
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
