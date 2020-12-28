use crate::common::{Id, Int};
use std::collections::HashMap;
use std::process::Output;

pub trait Entity {
    type State: State;
    fn get_id(&self) -> Id;
}

pub trait State {
    fn get_modified<InType, InEntity: Incoming<InType>>(&self, entity_id: Id) -> Option<Box<InEntity>>;
    fn get_modifying<OutType, OutEntity: Outgoing<OutType>>(&self, entity_id: Id) -> Option<Box<OutEntity>>;
}

pub trait Outgoing<OutType>: Entity {
    // type IdIterator: Iterator<Item = Id>;
    fn add_out(&mut self, out_id: Id);
    // fn all_out_ids(&self) -> Self::IdIterator;
    fn get_modifier(&self, out_id: Id) -> OutType;
    fn update_all_outs(&self, state: &mut Self::State);
    // fn update_all_outs(&self, state: &mut Self::State) {
    //     for out_id in self.all_out_ids() {
    //         let mut modified : Option<Box<dyn Incoming<InType = Self::OutType, State = Self::State, IdIterator = Self::IdIterator>>> = state.get_modified(out_id);
    //         if let Some(mut out) = modified {
    //             out.update_modifier(self.get_id(), self.get_modifier(out_id));
    //         }
    //     }
    // }
}

pub trait Incoming<InType>: Entity {
    // type IdIterator;
    fn add_in(&mut self, in_id: Id);
    // fn all_in_ids(&self) -> Self::IdIterator;
    fn update_modifier(&mut self, in_id: Id, modifier: InType);
    fn recount_all_ins(&mut self, state: &mut Self::State);
    // fn recount_all_ins(&mut self, state: &mut Self::State) {
    //     for in_id in self.all_in_ids() {
    //         let in_entity = state.get_modifying(in_id);
    //         self.update_modifier(in_id, in_entity.get_modifier(self.get_id()));
    //     }
    // }
}

