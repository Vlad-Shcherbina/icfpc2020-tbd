use crate::runners::{run_bots_local, run_in_submission};
use crate::ai_interface::Ai;
use crate::uforest::*;
use crate::bot_util::*;
use crate::bee_player_lib::compute_thrust;

pub struct Bee {}

impl Ai for Bee {
    fn initial_ship_params(&mut self, spec: &GameSpec) -> ShipParams {
        let mut params = match spec.role {
            Role::Defender => ShipParams {
                fuel: 0,
                laser: 0,
                cooling: 30,
                hull: 1,
            },
            Role::Attacker => ShipParams {
                fuel: 0,
                laser: 0,
                cooling: 8,
                hull: 1,
            }
        };
        compute_fuel_from_params(&mut params, spec).unwrap();
        params
    }

    fn choose_commands(&mut self, spec: &GameSpec, state: &GameState) -> Commands {
        let mut commands = Vec::new();
        for ship in ships_by_role(state, spec.role) {
            let acc = compute_thrust(spec, ship);
            if acc != Vec2::default() {
                commands.push(Command::Accelerate {
                    ship_id: ship.ship_id,
                    vector: acc,
                });
            }
        }
        Commands(commands)
    }
}

pub fn main() {
    if crate::is_running_in_submission() {
        run_in_submission(Bee {});
    } else {
        run_bots_local(
            Bee {},
            Bee {},
        );
    }
}
