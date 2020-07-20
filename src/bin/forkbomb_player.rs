use tbd::runners::{run_bots_local, run_in_submission};
use tbd::ai_interface::Ai;
use tbd::uforest::*;
use tbd::bot_util::*;
use tbd::bee_player_lib::compute_thrust;

pub struct BeeFork {}

/*
// a simple function of dividing everything by 2
// TODO: extend with logic for defender / attacker
fn fork_params(old_params: &ShipParams) -> ShipParams {
    let max_new_fuel = 63;

    ShipParams {
        fuel: std::cmp::min(old_params.fuel - old_params.fuel / 2, max_new_fuel),
        laser: old_params.laser - old_params.laser / 2,
        cooling: old_params.cooling - old_params.cooling / 2,
        hull: old_params.hull - old_params.hull / 2,
    }

}
*/

fn fork_params(p: &ShipParams) -> ShipParams {
    assert!(p.hull >= 2);
    let new_fuel = (p.fuel / 2).min(50);
    let laser = p.laser * new_fuel / (p.fuel + 1);
    let cooling = p.cooling * new_fuel / (p.fuel + 1);
    let hull = p.hull * new_fuel / (p.fuel + 1);
    let hull = hull.max(1).min(hull / 2);
    ShipParams {
        fuel: new_fuel,
        laser,
        cooling,
        hull,
    }
}

fn switch_gear(mut acc: Vec2) {
    if acc.x.abs() == 2 { acc.x = acc.x / 2 } else { acc.x = acc.x * 2}
    if acc.y.abs() == 2 { acc.y = acc.y / 2 } else { acc.y = acc.y * 2}

}

impl Ai for BeeFork {
    fn initial_ship_params(&mut self, spec: &GameSpec) -> ShipParams {
        let mut params = match spec.role {
            Role::Defender => ShipParams {
                fuel: 0,
                laser: 0,
                cooling: 5,
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
        let role_ships: Vec<ShipState> = ships_by_role(state, spec.role).cloned().collect();
        for ship in &role_ships {

            // fork every turn until (arbitrary) minimum values are reached
            let params = &ship.ship_params;
            if params.fuel > 8 && params.hull > 1 {
                commands.push(Command::Fork {
                    ship_id: ship.ship_id,
                    new_ship_params: fork_params(params),
                });
            };


            let mut acc = compute_thrust(spec, &ship);
            if acc != Vec2::default() {
                if detect_exact_overlap(&ship, &role_ships) {
                    if ship.ship_id % 2 == 1 {
                        switch_gear(acc);
                    }
                }
                commands.push(Command::Accelerate {
                    ship_id: ship.ship_id,
                    vector: acc.clone(),
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
