use crate::global_state;
use crate::utils;
use anyhow::Error;
use std::ops::Deref;
use yew::agent::Dispatched;
use yew::callback::Callback;
use yew::format::Nothing;
use yew::format::{Json, Text};
use yew::services::fetch;
use yew::services::Task;

pub type Request<T> = fetch::Request<T>;
pub type Response<T> = fetch::Response<Json<Result<T, Error>>>;

#[derive(Default, Debug)]
pub struct FetchService(fetch::FetchService);

pub struct FetchTask(fetch::FetchTask);

impl FetchService {
    pub fn new() -> Self {
        Self(fetch::FetchService::new())
    }

    pub fn get<OUT>(&mut self, url: &str, callback: Callback<OUT>) -> FetchTask
    where
        Json<Result<OUT, Error>>: From<Text> + 'static,
    {
        let request = Request::get(url)
            .header("Content-Type", "application/json")
            .body(Nothing)
            .unwrap();
        self.fetch(request, callback)
    }

    pub fn post<IN, OUT>(&mut self, url: &str, post_data: IN, callback: Callback<OUT>) -> FetchTask
    where
        Json<IN>: Into<Text>,
        Json<Result<OUT, Error>>: From<Text> + 'static,
    {
        let request = Request::post(url)
            .header("Content-Type", "application/json")
            .body(Json(post_data))
            .unwrap();
        self.fetch(request, callback)
    }

    pub fn put<IN, OUT>(&mut self, url: &str, post_data: IN, callback: Callback<OUT>) -> FetchTask
    where
        Json<IN>: Into<Text>,
        Json<Result<OUT, Error>>: From<Text> + 'static,
    {
        let request = Request::put(url)
            .header("Content-Type", "application/json")
            .body(Json(post_data))
            .unwrap();
        self.fetch(request, callback)
    }

    fn fetch<IN, OUT>(&mut self, request: fetch::Request<IN>, callback: Callback<OUT>) -> FetchTask
    where
        IN: Into<Text>,
        Json<Result<OUT, Error>>: From<Text> + 'static,
    {
        let callback = Callback::from(move |input: Response<OUT>| {
            let (meta, Json(json_data)) = input.into_parts();
            if meta.status.is_success() {
                match json_data {
                    Ok(out) => {
                        callback.emit(out);
                    }
                    Err(error) => {
                        utils::handle_api_error(Some(error));
                    }
                }
            } else if meta.status.as_u16() == 401 {
                utils::handle_unauthorized();
            } else {
                utils::handle_api_error(None);
            }
        });
        self.fetch_impl(request, callback)
    }

    fn fetch_impl<IN, OUT: 'static>(
        &mut self,
        request: fetch::Request<IN>,
        callback: Callback<fetch::Response<OUT>>,
    ) -> FetchTask
    where
        IN: Into<Text>,
        OUT: From<Text>,
    {
        global_state::GlobalStateAgent::dispatcher().send(global_state::Request::StartLoading);
        utils::disable_buttons();
        let x = Callback::from(move |input| {
            global_state::GlobalStateAgent::dispatcher().send(global_state::Request::StopLoading);
            utils::enable_buttons();
            callback.emit(input)
        });
        FetchTask(self.0.fetch(request, x).unwrap())
    }
}

impl Task for FetchTask {
    fn is_active(&self) -> bool {
        self.0.is_active()
    }
}

impl Drop for FetchTask {
    fn drop(&mut self) {
        if self.is_active() {
            global_state::GlobalStateAgent::dispatcher().send(global_state::Request::StopLoading);
            utils::enable_buttons();
        }
    }
}

impl Deref for FetchService {
    type Target = fetch::FetchService;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for FetchTask {
    type Target = fetch::FetchTask;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
