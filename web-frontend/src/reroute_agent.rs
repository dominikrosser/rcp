use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use yew::worker::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum RerouteRequestMsg {

    // Reroute to suburl (starting with '/')
    Reroute(String),

}

pub struct RerouteAgent {
    link: AgentLink<Self>,
    subscribers: HashSet<HandlerId>,
}

impl Agent for RerouteAgent {
    type Reach = Context<Self>;
    type Message = ();
    type Input = RerouteRequestMsg;
    type Output = String;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {


        match msg {

            RerouteRequestMsg::Reroute(route) => {
                for sub in self.subscribers.iter() {
                    self.link.respond(*sub, route.clone());
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