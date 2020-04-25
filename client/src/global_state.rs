use serde::{Deserialize, Serialize};
use std::cell::{RefCell, RefMut};
use std::collections::HashSet;
use std::rc::Rc;
use yew::worker::{Agent, AgentLink, Context, HandlerId};

#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalState {
    pub loading_count: i32,
}

pub struct GlobalStateAgent {
    link: AgentLink<GlobalStateAgent>,
    global_state: Rc<RefCell<GlobalState>>,
    subscribers: HashSet<HandlerId>,
}

pub enum Msg {}

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    StartLoading,
    StopLoading,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    StartLoading,
    StopLoading,
    Answer(Rc<RefCell<GlobalState>>), // delete me
}

impl Agent for GlobalStateAgent {
    // Available:
    // - `Job` (one per bridge on the main thread)
    // - `Context` (shared in the main thread)
    // - `Private` (one per bridge in a separate thread)
    // - `Public` (shared in a separate thread)
    type Reach = Context; // Spawn only one instance on the main thread (all components can share this agent)
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    // Create an instance with a link to agent's environment.
    fn create(link: AgentLink<Self>) -> Self {
        let global_state = Rc::new(RefCell::new(GlobalState { loading_count: 0 }));
        GlobalStateAgent {
            link,
            global_state,
            subscribers: HashSet::new(),
        }
    }

    // Handle inner messages (of services of `callback` callbacks)
    fn update(&mut self, _msg: Self::Message) {}

    // Handle incoming messages from components of other agents.
    fn handle_input(&mut self, msg: Self::Input, _who: HandlerId) {
        match msg {
            Request::StartLoading => {
                let mut state: RefMut<_> = self.global_state.borrow_mut();
                state.loading_count += 1;
                if state.loading_count == 1 {
                    for subscriber in self.subscribers.iter() {
                        self.link.respond(*subscriber, Response::StartLoading);
                    }
                }
            }
            Request::StopLoading => {
                let mut state: RefMut<_> = self.global_state.borrow_mut();
                state.loading_count -= 1;
                if state.loading_count == 0 {
                    for subscriber in self.subscribers.iter() {
                        self.link.respond(*subscriber, Response::StopLoading);
                    }
                }
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
