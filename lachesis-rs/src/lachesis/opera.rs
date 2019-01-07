use super::parents_list::ParentsList;
use crate::event::{event_hash::EventHash, Event};
use std::collections::HashMap;

pub struct Opera {
    graph: HashMap<EventHash, Event<ParentsList>>,
}

impl Opera {
    pub fn new() -> Opera {
        let graph = HashMap::new();
        Opera { graph }
    }

    pub fn sync(&mut self, other: Opera) {
        for (eh, ev) in other.graph {
            self.graph.insert(eh, ev);
        }
    }

    pub fn wire(&self) -> OperaWire {
        OperaWire {}
    }
}

pub struct OperaWire {}
