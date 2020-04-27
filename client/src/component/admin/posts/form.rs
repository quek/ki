use crate::common::dto::Post;
use crate::common::form::{PostErrors, PostForm};
use crate::common::types::PostStatus;
use crate::component::error;
use crate::utils;
use web_sys::Event;
use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct Model {
    link: ComponentLink<Self>,
    props: Props,
}

pub enum Msg {
    Submit(Event),
}

#[derive(Clone, Properties)]
pub struct Props {
    #[prop_or_default]
    pub post: Option<Post>,
    pub button_label: String,
    pub onsubmit: Callback<PostForm>,
    pub errors: PostErrors,
}

impl Component for Model {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Submit(event) => {
                event.prevent_default();
                let form: PostForm = (&event).into();
                web_sys::console::log_1(&format!("{:?}", &form).into());
                match form.validate() {
                    Err(errors) => {
                        self.props.errors = errors;
                        utils::scroll_to_error();
                        true
                    }
                    Ok(()) => {
                        self.props.errors = PostErrors::default();
                        self.props.onsubmit.emit(form);
                        true
                    }
                }
            }
        }
    }

    fn view(&self) -> Html {
        let post = &self.props.post;
        html! {
          <form class="std" onsubmit=self.link.callback(|event| Msg::Submit(event))>
            <div>
              <label>{"タイトル"}</label>
              <input type="text" name="title" value=post.as_ref().map_or("", |x| &x.title) />
              <error::Model message={&self.props.errors.title} />
            </div>
            <div>
              <label>{"本文"}</label>
              <textarea name="body">{post.as_ref().map_or("", |x| &x.body)}</textarea>
              <error::Model message={&self.props.errors.body} />
            </div>
            <div>
              <label>{"ステータス"}</label>
              <div>
                <label>
                  <input type="radio" name="status" value=PostStatus::Draft
                         checked={post.as_ref().map_or(true, |x| x.status == PostStatus::Draft)} />
                  {"下書き"}
                </label>
                <label>
                  <input type="radio" name="status" value=PostStatus::Published
                         checked={post.as_ref().map_or(false, |x| x.status == PostStatus::Published)} />
                  {"公開"}
                </label>
              </div>
              <error::Model message={&self.props.errors.status} />
            </div>
            <button>{&self.props.button_label}</button>
          </form>
        }
    }
}
