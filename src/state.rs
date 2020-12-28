use crate::common::Id;
use crate::characteristics::Entity;

pub trait State {
    fn get_entity(&self, entity_id: Id) -> &dyn Entity<State = Self>;
    fn get_entity_mut(&mut self, entity_id: Id) -> &mut dyn Entity<State = Self>;
    fn register_entity(&mut self, entity: dyn Entity<State = Self>);
}
