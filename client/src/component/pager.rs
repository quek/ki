use web_sys::MouseEvent;
use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};

const NPAGES: i64 = 5;

pub struct Model {
    link: ComponentLink<Self>,
    props: Props,
}

pub enum Msg {
    Click(MouseEvent, i64),
}

#[derive(Clone, Properties)]
pub struct Props {
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub callback: Callback<i64>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click(event, page) => {
                event.prevent_default();
                self.props.callback.emit(page);
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let total_pages = self.total_pages();
        if total_pages <= 1 {
            html! { {""} }
        } else {
            html! {
              <div class="pager">
                {self.prev()}
                {for self.pages().into_iter().map(|page| self.render_page(page))}
                {self.next()}
              </div>
            }
        }
    }
}

impl Model {
    fn next(&self) -> yew::Html {
        let total_pages = self.total_pages();
        if self.props.page >= total_pages {
            yew::html! {
                <div>{">"}</div>
            }
        } else {
            let page = self.props.page + 1;
            let onclick = self.link.callback(move |event| Msg::Click(event, page));
            yew::html! { <a href="#" onclick=onclick>{">"}</a> }
        }
    }

    fn pages(&self) -> Vec<i64> {
        let from = std::cmp::max(self.props.page - NPAGES, 1);
        let to = std::cmp::min(self.props.page + NPAGES, self.total_pages());
        (from..=to).collect()
    }

    fn prev(&self) -> yew::Html {
        if self.props.page <= 1 {
            yew::html! {
                <div>{"<"}</div>
            }
        } else {
            let page = self.props.page - 1;
            let onclick = self.link.callback(move |event| Msg::Click(event, page));
            yew::html! { <a href="#" onclick=onclick>{"<"}</a> }
        }
    }

    fn render_page(&self, page: i64) -> yew::Html {
        if page == self.props.page {
            yew::html! {
                <div>{page}</div>
            }
        } else {
            let onclick = self.link.callback(move |event| Msg::Click(event, page));
            yew::html! {
                <a href="#" onclick=onclick>{page}</a>
            }
        }
    }

    fn total_pages(&self) -> i64 {
        (self.props.total as f64 / self.props.per_page as f64).ceil() as i64
    }
}
