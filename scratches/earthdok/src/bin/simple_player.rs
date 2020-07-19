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
            fuel: 254,
            laser: 0,
            bars: 16,
            hull: 1,
        }
    }

    fn choose_commands(&mut self, _spec: &GameSpec, _state: &GameState) -> Commands {
        let role = _spec.role;
        let ship_state = &_state
            .ships_list
            .iter()
            .find(|&ship| ship.ship_state.role == role)
            .unwrap()
            .ship_state;
        
        let gravity = get_gravity(ship_state.position);
        // TODO: NEGATE
        let command = Command::Accelerate{ship_id: ship_state.ship_id,
                                                   vector: gravity.negate()};
        Commands(vec![command])
    }
}   

fn get_gravity(pos: Vec2) -> Vec2 {
    // TODO: what happens at abs(x) == abs(y)??
    Vec2{x: if pos.x.abs() >= pos.y.abs() { - pos.x.signum() } else { 0 },
         y: if pos.y.abs() >= pos.x.abs() { - pos.y.signum() } else { 0 }}
}

fn main() {
    tbd::runners::run_in_submission(ExampleAi {} );
}
