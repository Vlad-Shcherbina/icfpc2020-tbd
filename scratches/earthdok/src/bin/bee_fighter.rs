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

        let thrust = compute_actual_thrust(spec, state, our_ship);
        let expected_position = get_expected_position(our_ship) - thrust;


        // Pew pew
        //if our_role == Role::Attacker {
            
        //}

        let mut cmd_vec = vec![];
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
    false
}

fn main() {
    run_bots_local(OrbitBot::default(), OrbitBot::default());
}
