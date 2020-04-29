use crate::common::form::{PostErrors, PostForm};
use crate::common::models::Post;
use crate::common::types::PostStatus;
use crate::component::error;
use crate::utils;
use web_sys::Event;
use web_sys::HtmlInputElement;
use yew::{
    html, Callback, Component, ComponentLink, Html, InputData, NodeRef, Properties, ShouldRender,
};

pub struct Model {
    link: ComponentLink<Self>,
    props: Props,
    body: String,
    title_ref: NodeRef,
}

pub enum Msg {
    Submit(Event),
    Body(InputData),
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
        let body = props
            .post
            .as_ref()
            .map_or("".to_string(), |x| x.body.to_string());
        Self {
            link,
            props,
            body,
            title_ref: NodeRef::default(),
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(x) = self.title_ref.cast::<HtmlInputElement>() {
                x.focus().unwrap();
            }
        }
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
            Msg::Body(input_data) => {
                self.body = input_data.value;
                true
            }
        }
    }

    fn view(&self) -> Html {
        let post = &self.props.post;
        let oninput = self.link.callback(|event| Msg::Body(event));
        html! {
          <form class="post" onsubmit=self.link.callback(|input_data| Msg::Submit(input_data))>
            <div>
              <div>
                <div>
                  <input type="text" name="title" value=post.as_ref().map_or("", |x| &x.title)
                         ref=self.title_ref.clone() />
                  <error::Model message={&self.props.errors.title} />
                </div>
                <div>
                  <textarea name="body" oninput=oninput>{post.as_ref().map_or("", |x| &x.body)}</textarea>
                  <error::Model message={&self.props.errors.body} />
                </div>
                <div>
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
              </div>
              {crate::markdown::node(&self.body)}
            </div>
          </form>
        }
    }
}
