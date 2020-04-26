use crate::common::dto::{Id, Post};
use crate::common::form::{PostErrors, PostForm};
use crate::common::types::PostStatus;
use crate::component::error;
use crate::fetch;
use crate::routes::{AdminRoute, AppRoute};
use crate::utils;
use web_sys::Event;
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
    Submit(Event),
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
            errors: Default::default(),
            fetch_task,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Post(post) => {
                self.post = Some(post);
                true
            }
            Msg::Submit(event) => {
                event.prevent_default();
                let form: PostForm = (&event).into();
                web_sys::console::log_1(&format!("{:?}", &form).into());
                match form.validate() {
                    Err(errors) => {
                        self.errors = errors;
                        utils::scroll_to_error();
                        true
                    }
                    Ok(()) => {
                        self.errors = PostErrors::default();
                        self.fetch_task = fetch::FetchService::new().put(
                            &format!("/api/admin/posts/{}", self.props.id),
                            &form,
                            self.link.callback(|_: ()| Msg::Updated),
                        );
                        true
                    }
                }
            }
            Msg::Updated => {
                utils::toast("更新しました。");
                utils::change_route(AppRoute::Admin(AdminRoute::Index));
                true
            }
        }
    }

    fn view(&self) -> Html {
        match self.post {
            None => html! { "" },
            Some(ref post) => html! {
              <>
                <h1>{"編集"}</h1>
                <form class="std" onsubmit=self.link.callback(|event| Msg::Submit(event))>
                  <div>
                    <label>{"タイトル"}</label>
                    <input type="text" name="title" value=&post.title />
                    <error::Model message={&self.errors.title} />
                  </div>
                  <div>
                    <label>{"本文"}</label>
                    <textarea name="body">{&post.body}</textarea>
                    <error::Model message={&self.errors.body} />
                  </div>
                  <div>
                    <label>{"ステータス"}</label>
                    <div>
                      <label>
                        <input type="radio" name="status" value=PostStatus::Draft
                               checked={post.status == PostStatus::Draft} />
                        {"下書き"}
                      </label>
                      <label>
                        <input type="radio" name="status" value=PostStatus::Published
                               checked={post.status == PostStatus::Published} />
                        {"公開"}
                      </label>
                    </div>
                    <error::Model message={&self.errors.status} />
                  </div>
                  <button>{"更新"}</button>
                </form>
              </>
            },
        }
    }
}
