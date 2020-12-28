use crate::common::{Int, Modifier};

#[derive(Clone)]
pub enum Dice {
    Sum (Box<Dice>, Box<Dice>),
    Mul (Int, Box<Dice>),
    D (Int)
}

pub struct Check {
    pub dice: Vec<Dice>,
    pub modifier: Modifier,
}

impl Dice {
    fn throw(&self) -> Int {
       unimplemented!()
    }
}

impl Check {
    fn throw_dices(&self) -> Int {
        let mut res = 0;
        for dice in &self.dice {
            res += dice.throw();
        }
        res
    }
    pub fn check(&self) -> Int {
        match &self.modifier {
            Modifier::Add (i) => i + self.throw_dices()
        }
    }
}
