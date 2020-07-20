use tbd::runners::{run_bots_local, run_in_submission};
use tbd::ai_interface::Ai;
use tbd::uforest::*;
use tbd::bot_util::*;

pub struct Bee {}

impl Ai for Bee {
    fn choose_join_request(&mut self) -> JoinRequest {
        JoinRequest { mystery: Data::Nil }
    }

    fn initial_ship_params(&mut self, spec: &GameSpec) -> ShipParams {
        let max_cost = spec.bounds.max_cost;
        ShipParams {
            fuel: max_cost - HULL_COST,
            laser: 0,
            cooling: 0,
            hull: 1,
        }
    }

    fn choose_commands(&mut self, spec: &GameSpec, state: &GameState) -> Commands {
        let field = spec.field.as_ref().unwrap();
        let mut commands = Vec::new();
        for ship in ships_by_role(state, spec.role) {
        }
        Commands(commands)
    }
}

fn main() {
    if tbd::is_running_in_submission() {
        run_in_submission(Bee {});
    } else {
        run_bots_local(
            Bee {},
            Bee {},
        );
    }
}
