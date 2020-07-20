use tbd::runners::run_bots_local;
use tbd::ai_interface::Ai;
use tbd::uforest::*;
use tbd::bot_util::*;
use tbd::bee_player_lib::*;
use tbd::vec2::Vec2;

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

        let position = our_ship.position;
        let velocity = our_ship.velocity;
        let _field = spec.field.as_ref().unwrap();
        let gravity = get_gravity(our_ship.position);

        if our_ship.ship_params.fuel == 0 {
            return Commands(vec![])
        }

        let thrust = compute_thrust(spec, our_ship);
        let thrust = if state.steps % 5 == 0 { perturb(thrust) } else { thrust };

        let expected_position = get_expected_position(our_ship) - thrust;


        // Pew pew
        //if our_role == Role::Attacker {
            //
        //}

        if thrust != Vec2::default() {
            let thrust_cmd = Command::Accelerate {
                ship_id: our_ship.ship_id,
                vector: thrust,
            };
            Commands(vec![thrust_cmd])
        }
        else {
            Commands(vec![])
        }
    }
}

fn good_to_shoot(our_ship: &ShipState, their_ship: &ShipState) -> bool {
    false
}

fn main() {
    run_bots_local(OrbitBot::default(), OrbitBot::default());
}
