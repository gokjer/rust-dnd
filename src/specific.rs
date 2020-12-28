use crate::common::{Int, Id};
use std::collections::{HashSet, hash_set};
use crate::characteristics::{Incoming, Entity, State, Outgoing};
use crate::values::{Modifyable, ValueHolder};


struct Characteristic<'a, ValueType, ModType> where ValueType: Modifyable<ModType> {
    id: Id,
    value: ValueHolder<ValueType, ModType>,
    ins: & 'a mut HashSet<Id>,
    outs: &'a mut HashSet<Id>
}

struct MyState {}

struct CharIterator {

}

impl State for MyState {
    fn get_modified<InType, InEntity: Incoming<InType>>(&self, entity_id: Id) -> Option<Box<InEntity>> {
        unimplemented!()
    }

    fn get_modifying<OutType, OutEntity: Outgoing<OutType>>(&self, entity_id: Id) -> Option<Box<OutEntity>> {
        unimplemented!()
    }
}

impl<ValueType: Modifyable<ModType>, ModType> Entity for Characteristic<'_, ValueType, ModType> {
    type State = MyState;

    fn get_id(&self) -> Id {
        return self.id.clone()
    }
}

impl<'a, ValueType: Modifyable<ModType>, ModType : Copy> Incoming<ModType> for Characteristic<'a, ValueType, ModType> {
    // type IdIterator = hash_set::Iter<'a, Id>;

    fn add_in(&mut self, in_id: Id) {
        self.ins.insert(in_id);
    }

    // fn all_in_ids(&self) -> Self::IdIterator {
    //     return self.ins.iter()
    //     // unimplemented!()
    // }

    fn update_modifier(&mut self, in_id: Id, modifier: ModType) {
        if self.ins.contains(&in_id) {
            self.value.set_component(in_id, modifier);
        }
    }

    fn recount_all_ins(&mut self, state: &mut Self::State) {
        let in_ids = self.ins.clone();
        for in_id in in_ids {
            let modifying : Option<Box<Characteristic<'a, ValueType, ModType>>> = state.get_modifying(in_id.clone());
            if let Some(input) = modifying {
                self.update_modifier(in_id.clone(), input.get_modifier(self.get_id()));
            }
        }
    }
}

impl<'a, ValueType: Modifyable<ModType>, ModType : Copy> Outgoing<ModType> for Characteristic<'a, ValueType, ModType> {
    fn add_out(&mut self, out_id: Id) {
        self.outs.insert(out_id);
    }

    fn get_modifier(&self, out_id: Id) -> ModType {
        unimplemented!()
    }

    fn update_all_outs(&self, state: &mut Self::State) {
        unimplemented!()
    }
}
