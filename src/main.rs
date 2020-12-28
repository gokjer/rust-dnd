mod common;
mod dice;
mod characteristics;
mod state;
mod specific;
mod values;

fn main() {
    println!("Hello, world!");
    let check = dice::Check{dice: [dice::Dice::D(6)].to_vec(), modifier: common::Modifier::Add(5)};
    println!("{}", check.check());
}
