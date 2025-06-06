use crate::{Role, Script};

pub(crate) fn trouble_brewing() -> Script {
    Script {
        roles: {
            vec![
                // Townsfolk
                Role::Washerwoman,
                Role::Librarian,
                Role::Investigator,
                Role::Chef,
                Role::Empath,
                Role::Fortuneteller,
                Role::Undertaker,
                Role::Monk,
                Role::Ravenkeeper,
                Role::Virgin,
                Role::Slayer,
                Role::Soldier,
                Role::Mayor,
                // Outsiders
                Role::Butler,
                Role::Drunk,
                Role::Recluse,
                Role::Saint,
                // Minions
                Role::Poisoner,
                Role::Spy,
                Role::Scarletwoman,
                Role::Baron,
                // Demons
                Role::Imp,
            ]
        },
    }
}

