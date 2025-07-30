use crate::{Roles, Script};

pub(crate) fn trouble_brewing() -> Script {
    Script {
        roles: {
            vec![
                // Townsfolk
                Roles::Washerwoman,
                Roles::Librarian,
                Roles::Investigator,
                Roles::Chef,
                Roles::Empath,
                Roles::Fortuneteller,
                Roles::Undertaker,
                Roles::Monk,
                Roles::Ravenkeeper,
                Roles::Virgin,
                Roles::Slayer,
                Roles::Soldier,
                Roles::Mayor,
                // Outsiders
                Roles::Butler,
                Roles::Drunk,
                Roles::Recluse,
                Roles::Saint,
                // Minions
                Roles::Poisoner,
                Roles::Spy,
                Roles::Scarletwoman,
                Roles::Baron,
                // Demons
                Roles::Imp,
            ]
        },
    }
}
