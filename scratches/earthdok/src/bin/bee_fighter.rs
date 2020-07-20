use tbd::runners::run_bots_local;
use tbd::runners::run_in_submission;
use tbd::ai_interface::Ai;
use tbd::uforest::*;
use tbd::bot_util::*;
use tbd::bee_player_lib::*;
use tbd::vec2::Vec2;
use std::cmp;

#[derive(Default)]
pub struct OrbitBot {
    expected_position: Vec2,
}

impl Ai for OrbitBot {
    fn initial_ship_params(&mut self, spec: &GameSpec) -> ShipParams {
        let mut params = ShipParams {
            fuel: 0,
            laser: 64,
            cooling: 8,
            hull: 1,
        };
        compute_fuel_from_params(&mut params, &spec).unwrap();
        params
    }


    fn choose_commands(&mut self, spec: &GameSpec, state: &GameState) -> Commands {
        let our_role = spec.role;
        let their_role = if our_role == Role::Attacker {
            Role::Defender
        } else {
            Role::Attacker
        };

        let our_ship = ships_by_role(state, our_role).next().unwrap();
        let their_ship = ships_by_role(state, their_role).next().unwrap();

        let thrust = compute_actual_thrust(spec, state, our_ship);
        let expected_position = get_expected_position(our_ship) - thrust;

        let mut cmd_vec = vec![];

        // Pew pew
        if our_role == Role::Attacker {
            let shot_power = cmp::min(our_ship.ship_params.laser, get_available_heat_sink(our_ship));
            if shot_power > 0 {
                for target_ship in ships_by_role(state, their_role) {
                    if good_to_shoot(our_ship, their_ship) {
                        cmd_vec.push(Command::Shoot {
                            ship_id: our_ship.ship_id,
                            target: get_expected_position(their_ship),
                            power: shot_power
                        });
                        break;
                    }
                }
            }   
        }

        maybe_push_thrust_command(&mut cmd_vec, thrust, our_ship.ship_id);

        Commands(cmd_vec)
    }
}

fn compute_actual_thrust(spec: &GameSpec, state: &GameState, ship: &ShipState) -> Vec2 {
    if ship.ship_params.fuel == 0 {
        Vec2::default()
    } else {
        let thrust = compute_thrust(spec, ship);
        if state.steps % 5 == 0 {
            perturb(thrust)
        } else {
            thrust
        }
    }
}

fn good_to_shoot(our_ship: &ShipState, their_ship: &ShipState) -> bool {
    true
}

fn main() {
    if tbd::is_running_in_submission() {
        run_in_submission(OrbitBot::default());
    } else {
        run_bots_local(OrbitBot::default(), OrbitBot::default());
    }
}
