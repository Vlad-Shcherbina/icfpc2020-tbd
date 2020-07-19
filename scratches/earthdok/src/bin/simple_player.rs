// Minimal player example. It can be used as a submission.
// Feel free to copy this and create your own bot.

use tbd::ai_interface::Ai;
use tbd::{bot_util::*, uforest::*};

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

fn main() {
    tbd::runners::run_in_submission(ExampleAi {});
}
