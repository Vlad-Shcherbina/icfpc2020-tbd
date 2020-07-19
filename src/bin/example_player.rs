// Minimal player example. It can be used as a submission.
// Feel free to copy this and create your own bot.

use tbd::uforest::*;
use tbd::ai_interface::Ai;

pub struct ExampleAi {}

impl Ai for ExampleAi {
    fn choose_join_request(&mut self) -> JoinRequest {
        JoinRequest { mystery: Data::Nil }
    }

    fn initial_ship_params(&mut self, spec: &GameSpec) -> ShipParams {
        ShipParams {
            fuel: 1,
            number2: 1,
            number3: 1,
            number4: 1,
        }
    }

    fn choose_commands(&mut self, spec: &GameSpec, state: &GameState) -> Commands {
        Commands(vec![])
    }
}

fn main() {
    tbd::runners::run_in_submission(ExampleAi {} );
}
