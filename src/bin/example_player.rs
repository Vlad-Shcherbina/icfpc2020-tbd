// Minimal player example. It can be used as a submission.
// Feel free to copy this and create your own bot.

use tbd::uforest::*;
use tbd::ai_interface::Ai;

pub struct ExampleAi {}

impl Ai for ExampleAi {
    fn choose_join_request(&mut self) -> JoinRequest {
        JoinRequest { mystery: Data::Nil }
    }

    fn initial_ship_params(&mut self, _spec: &GameSpec) -> ShipParams {
        ShipParams {
            fuel: 1,
            laser: 1,
            bars: 1,
            hull: 1,
        }
    }

    fn choose_commands(&mut self, _spec: &GameSpec, _state: &GameState) -> Commands {
        Commands(vec![])
    }
}

fn main() {
    tbd::runners::run_in_submission(ExampleAi {} );
}
