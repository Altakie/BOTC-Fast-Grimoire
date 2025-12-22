use crate::{RoleNames, Script};

pub(crate) fn trouble_brewing() -> Script {
    Script {
        roles: {
            vec![
                // Townsfolk
                RoleNames::Washerwoman,
                RoleNames::Librarian,
                RoleNames::Investigator,
                RoleNames::Chef,
                RoleNames::Empath,
                RoleNames::Fortuneteller,
                RoleNames::Undertaker,
                RoleNames::Monk,
                RoleNames::Ravenkeeper,
                RoleNames::Virgin,
                RoleNames::Slayer,
                RoleNames::Soldier,
                RoleNames::Mayor,
                // Outsiders
                RoleNames::Butler,
                RoleNames::Drunk,
                RoleNames::Recluse,
                RoleNames::Saint,
                // Minions
                RoleNames::Poisoner,
                RoleNames::Spy,
                RoleNames::ScarletWoman,
                RoleNames::Baron,
                // Demons
                RoleNames::Imp,
            ]
        },
    }
}
