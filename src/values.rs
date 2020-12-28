use core::marker::Copy;
use std::collections::HashMap;
use crate::common::{Id, Int};

pub trait Nullable {
    fn zero() -> Self;
}

pub trait Modifyable<ModType>: Copy + Nullable {
    fn add(self, modifier: ModType) -> Self;
    fn subtract(self, modifier: ModType) -> Self;
}

pub trait Modifying<ModType> {
    fn get_modifier(&self) -> ModType;
}


pub struct ValueHolder<ValueType, ModType> {
    value: ValueType,
    components: HashMap<Id, ModType>
}


impl<ModType : Copy, ValueType: Modifyable<ModType>> ValueHolder<ValueType, ModType> {
    pub fn set_component(&mut self, comp_id: Id, new_val: ModType) {
        if let Some(old_val) = self.components.insert(comp_id, new_val) {
            self.value.subtract(old_val);
        }
        self.value.add(new_val);
    }
    pub fn recount(&mut self) {
        self.value = Nullable::zero();
        for val in self.components.values() {
            self.value.add(*val);
        }
    }
}

impl<ValueType : Modifying<OutType>, OutType, ModType> Modifying<OutType> for ValueHolder<ValueType, ModType> {
    fn get_modifier(&self) -> OutType {
        return self.value.get_modifier()
    }
}

impl Nullable for Int {
    fn zero() -> Int {
        return 0;
    }
}

impl Modifyable<Int> for Int {
    fn add(self, modifier: Int) -> Self {
        return self + modifier;
    }

    fn subtract(self, modifier: Int) -> Self {
        return self - modifier;
    }
}
