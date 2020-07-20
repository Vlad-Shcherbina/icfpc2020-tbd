// Minimal player example. It can be used as a submission.
// Feel free to copy this and create your own bot.

use tbd::uforest::*;
use tbd::ai_interface::Ai;
use tbd::uforest::Command::Accelerate;
use tbd::runners::{run_in_submission, run_bots_local};

pub struct MyAi {}

impl Ai for MyAi {
    fn choose_join_request(&mut self) -> JoinRequest {
        JoinRequest { mystery: Data::Nil }
    }

    fn initial_ship_params(&mut self, _spec: &GameSpec) -> ShipParams {
        ShipParams {
            fuel: 1,
            laser: 1,
            cooling: 1,
            hull: 1,
        }
    }

    fn choose_commands(&mut self, _spec: &GameSpec, state: &GameState) -> Commands {
        let ship = state.ships_list.iter()
            .find(|ship| ship.ship_state.ship_id == 0)
            .expect("No 0 ship_id");
        let acc_x = if ship.ship_state.position.x > 0 {
            -1
        } else if ship.ship_state.position.x == 0 {
            0
        } else {
            1
        };
        let acc_y = if ship.ship_state.position.y > 0 {
            -1
        } else if ship.ship_state.position.y == 0 {
            0
        } else {
            1
        };
        Commands(vec![Accelerate { ship_id: 0, vector: Vec2 { x: acc_x, y: acc_y } }])
    }
}

fn main() {
    if tbd::is_running_in_submission() {
        run_in_submission(MyAi {});
    } else {
        run_bots_local(MyAi {}, MyAi {});
    }
}
