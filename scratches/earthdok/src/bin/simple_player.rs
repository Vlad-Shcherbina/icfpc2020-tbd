// Minimal player example. It can be used as a submission.
// Feel free to copy this and create your own bot.

use tbd::ai_interface::Ai;
use tbd::uforest::*;

pub struct ExampleAi {}

impl Ai for ExampleAi {
    fn choose_join_request(&mut self) -> JoinRequest {
        JoinRequest { mystery: Data::Nil }
    }

    fn initial_ship_params(&mut self, _spec: &GameSpec) -> ShipParams {
        ShipParams {
            fuel: 254,
            laser: 0,
            cooling: 16,
            hull: 1,
        }
    }

    fn choose_commands(&mut self, _spec: &GameSpec, _state: &GameState) -> Commands {
        let our_role = _spec.role;
        let their_role = if our_role == Role::Attacker {
            Role::Defender
        } else {
            Role::Attacker
        };

        let our_ship = ships_by_role(_state, our_role).next().unwrap();
        let their_ship = ships_by_role(_state, their_role).next().unwrap();

        // Fight gravity.
        let thrust = Command::Accelerate {
            ship_id: our_ship.ship_state.ship_id,
            vector: get_gravity(our_ship.ship_state.position),
        };

        // Fight enemies pew pew.
        let heat = our_ship.ship_state.heat;
        // WIP...

        Commands(vec![thrust])
    }
}

fn get_gravity(pos: Vec2) -> Vec2 {
    // TODO: what happens at abs(x) == abs(y)??
    Vec2 {
        x: if pos.x.abs() >= pos.y.abs() {
            -pos.x.signum()
        } else {
            0
        },
        y: if pos.y.abs() >= pos.x.abs() {
            -pos.y.signum()
        } else {
            0
        },
    }
}

fn ships_by_role(state: &GameState, role: Role) -> impl Iterator<Item = &Ship> {
    state
        .ships_list
        .iter()
        .filter(move |&ship| ship.ship_state.role == role)
}

fn main() {
    tbd::runners::run_in_submission(ExampleAi {});
}
