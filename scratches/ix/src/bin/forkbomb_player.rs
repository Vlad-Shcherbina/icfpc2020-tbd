use tbd::runners::{run_bots_local, run_in_submission};
use tbd::ai_interface::Ai;
use tbd::uforest::*;
use tbd::bot_util::*;
use tbd::bee_player_lib::compute_thrust;

pub struct BeeFork {}

// a simple function of dividing everything by 2
// TODO: extend with logic for defender / attacker
fn fork_params(old_params: &ShipParams) -> ShipParams {

    ShipParams {
        fuel: old_params.fuel - old_params.fuel / 2,
        laser: old_params.laser - old_params.laser / 2,
        cooling: old_params.cooling - old_params.cooling / 2,
        hull: old_params.hull - old_params.hull / 2,
    }

}

impl Ai for BeeFork {
    fn initial_ship_params(&mut self, spec: &GameSpec) -> ShipParams {
        let mut params = match spec.role {
            Role::Defender => ShipParams {
                fuel: 0,
                laser: 0,
                cooling: 20,
                hull: 65,
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

            // fork every turn until (arbitrary) minimum values are reached
            let params = &ship.ship_params;
            if params.fuel > 8 && params.hull > 1 {
                commands.push(Command::Fork {
                    ship_id: ship.ship_id,
                    new_ship_params: fork_params(params),
                });
            };
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


fn main() {
    if tbd::is_running_in_submission() {
        run_in_submission(BeeFork {});
    } else {
        run_bots_local(
            BeeFork {},
            BeeFork {},
        );
    }
}
