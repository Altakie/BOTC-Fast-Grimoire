use super::{Game, Role};

pub(crate) enum RoleSelectionType {
    InPlay,
    NotInPlay,
    Script,
}

#[derive(Clone)]
pub(crate) enum InterfacePhase {
    Default,
    Done,
    ChoosingRoles,
    ChoosingPlayers,
    InputtingNumber,
    DisplayingNumber,
    DisplayingPlayers,
    DisplayingRoles,
}

pub(crate) struct InterfaceInfo {}

impl Game {
    pub(crate) fn choose_players(&self, num: usize) -> Vec<usize> {
        todo!()
    }

    pub(crate) fn choose_roles(&self, num: usize, selector: RoleSelectionType) -> Vec<Role> {
        let roles_in_scope: Vec<Role> = match selector {
            RoleSelectionType::InPlay => self.players.iter().clone().map(|p| p.role).collect(),
            RoleSelectionType::NotInPlay => todo!(),
            RoleSelectionType::Script => self.script.roles.clone(),
        };

        // self.storyteller_interface.choose_roles(num, roles_in_scope)
        todo!()
    }
}
