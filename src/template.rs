#![allow(unlinked-file)]
trait Character {
    fn night_action(&mut self) -> ();
}

#[derive(Ord, PartialOrd, Copy, Clone, Eq, PartialEq)]
enum Role {
    Empath,
    Investigator,
    Lickanthrope,
}

struct Empath {}
struct Investigator {}

impl Character for Empath {
    fn night_action(&mut self) -> () {
        todo!()
    }
}

impl Character for Role {
    fn night_action(&mut self) -> () {
        match self {
            Role::Empath => {
                todo!();
            }
            Role::Investigator => todo!(),
            Role::Lickanthrope => todo!(),
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {

    #[test]
    fn hello() {
        assert_eq!(1 + 1, 3);
    }
}
