use rand;

fn main() {}

struct SeatingChart {
    players: Vec<Player>,
}

impl SeatingChart {
    fn new() -> Self {
        Self { players: vec![] }
    }

    fn setup(&mut self, mut roles: Vec<Role>, player_names: Vec<String>) {
        if roles.len() != player_names.len() {
            eprintln!("Number of players does not match number of roles")
        }

        // Randomly assign numbers to roles
        while roles.len() > 0 {
            let rand_i = rand::random_range(0..roles.len());
            // Create players around each role
            let new_player = Player::new({
                let role = roles.get(rand_i);
                match role {
                    Some(r) => *r,
                    None => {
                        eprintln!("role not found");
                        return;
                    }
                }
            });
            // Randomly place these new players into the the players vector
            self.players.push(new_player);
            roles.remove(rand_i);
        }
        // Assign each player a name based on the player vector
        for i in 0..self.players.len() {
            let player = &mut self.players[i];
            let name = player_names[i].clone();
            player.set_name(name);
        }
    }

    // Should return true if the player was successfully killed, false if the player failed to die
    fn kill_player(&self, player: &Player) -> bool {
        // Find the target player in the array and set their status to dead
        todo!();
    }

    fn left_player(&self, player: &Player) -> &Player {
        todo!();
    }
    fn right_player(&self, player: &Player) -> &Player {
        todo!();
    }
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Role {
    Investigator,
    Empath,
    Gossip,
    Imp,
    Innkeeper,
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum StatusEffects {
    Drunk,
    Mad,
    Poisoned,
    Protected,
}

#[derive(Debug)]
struct Player {
    name: String,
    role: Role,
    // Order should be implemented through external array
    ghost_vote: bool,
    statuses: Vec<StatusEffects>,
    dead: bool,
}

impl Player {
    fn new(role: Role) -> Self {
        Self {
            name: String::from(""),
            role,
            ghost_vote: true,
            statuses: vec![],
            dead: false,
        }
    }

    fn set_name(&mut self, name: String) {
        self.name = name
    }
}

impl ToString for Player {
    fn to_string(&self) -> String {
        let player_string = format!(
            "Player {}\n\tRole: {:?}\n
                \tDead?: Not Yet Implemented\n
                \t Statuses: Not yet implemented \n
                \tHas Ghost Vote?: {}\n",
            self.name, self.dead, self.ghost_vote
        );

        return player_string;
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        if self.name != other.name {
            return false;
        }

        if self.role != other.role {
            return false;
        }

        return true;
    }
}
impl Eq for Player {}

#[cfg(test)]
mod tests {
    // Setup Tests

    use core::panic;

    use super::*;
    #[test]
    fn test_setup() {
        let roles = vec![Role::Investigator, Role::Innkeeper, Role::Imp];
        let player_names = vec![String::from("P1"), String::from("P2"), String::from("P3")];

        let mut seating_chart = SeatingChart::new();
        seating_chart.setup(roles.clone(), player_names);

        assert_eq!(seating_chart.players.len(), 3);
        assert_eq!(seating_chart.players[1].name, "P2");

        {
            let mut roles = roles.clone();
            for player in seating_chart.players {
                let role_i = match roles.iter().position(|&r| r == player.role) {
                    Some(x) => x,
                    None => {
                        eprintln!("Role not assigned to player");
                        panic!();
                    }
                };

                roles.remove(role_i);
            }

            assert_eq!(roles.len(), 0);
        }
    }

    #[test]
    fn kill_player() {
        let roles = vec![Role::Investigator, Role::Innkeeper, Role::Imp];
        let player_names = vec![String::from("P1"), String::from("P2"), String::from("P3")];

        let mut seating_chart = SeatingChart::new();
        seating_chart.setup(roles.clone(), player_names);

        todo!();
    }

    #[test]
    fn kill_protected_player() {
        let roles = vec![Role::Investigator, Role::Innkeeper, Role::Imp];
        let player_names = vec![String::from("P1"), String::from("P2"), String::from("P3")];

        let mut seating_chart = SeatingChart::new();
        seating_chart.setup(roles.clone(), player_names);

        todo!();
    }

    #[test]
    fn test_left() {
        let roles = vec![Role::Investigator, Role::Innkeeper, Role::Imp];
        let player_names = vec![String::from("P1"), String::from("P2"), String::from("P3")];

        let mut seating_chart = SeatingChart::new();
        seating_chart.setup(roles.clone(), player_names);

        assert_eq!(
            seating_chart.left_player(&seating_chart.players[2]).name,
            seating_chart.players[1].name
        );

        // Kill set the left player to dead and see that the left player is updated accordingly
        todo!();
    }

    #[test]
    fn test_right() {
        let roles = vec![Role::Investigator, Role::Innkeeper, Role::Imp];
        let player_names = vec![String::from("P1"), String::from("P2"), String::from("P3")];

        let mut seating_chart = SeatingChart::new();
        seating_chart.setup(roles.clone(), player_names);

        assert_eq!(
            seating_chart.left_player(&seating_chart.players[2]).name,
            seating_chart.players[1].name
        );

        // Kill the right player and make sure the right player is updated accordingly
        todo!();
    }
}
