use crate::common::validate;
use crate::generated::post::PostStatus;
use serde::{Deserialize, Serialize};

#[cfg(target_arch = "wasm32")]
use js_sys::Object;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::{Event, FormData, HtmlFormElement};

#[cfg(target_arch = "wasm32")]
pub fn event_to_object(event: &Event) -> Object {
    let target = event.target().unwrap();
    let form = target.dyn_ref::<HtmlFormElement>().unwrap();
    let form_data = FormData::new_with_form(&form).unwrap();
    Object::from_entries(&form_data).unwrap()
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PostForm {
    pub title: String,
    pub body: String,
    pub status: String,
}

impl PostForm {
    pub fn validate(&self) -> Result<(), PostErrors> {
        let mut errors = PostErrors::default();

        errors.title = validate::require(&self.title, "タイトル");
        errors.body = validate::require(&self.body, "本文");
        errors.status = validate::enumrate::<PostStatus>(&self.status, "ステータス");

        if errors == PostErrors::default() {
            Ok(())
        } else {
            return Err(errors);
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct PostErrors {
    pub title: Option<String>,
    pub body: Option<String>,
    pub status: Option<String>,
}

#[cfg(target_arch = "wasm32")]
impl From<&Event> for PostForm {
    fn from(event: &Event) -> Self {
        let object = event_to_object(event);
        object.into_serde().unwrap()
    }
}
