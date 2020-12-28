pub type Int = i32;

pub type Id = String;

pub enum Modifier {
    Add (Int),
}

trait A {
    type T: Clone;
}

trait B: A {
    fn clone_t(t: Self::T) -> Self::T {
        t.clone()
    }
}

pub struct DummyIter {}

impl Iterator for DummyIter {
    type Item = Id;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}
