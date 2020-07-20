use tbd::runners::run_bots_local;
use tbd::ai_interface::Ai;
use tbd::uforest::*;
use tbd::bot_util::*;
use tbd::bee_player_lib::*;
use tbd::vec2::Vec2;
use rand::Rng;

#[derive(Default)]
pub struct OrbitBot {
    expected_position: Vec2,
}

impl Ai for OrbitBot {
    fn initial_ship_params(&mut self, spec: &GameSpec) -> ShipParams {
        let mut params = ShipParams {
            fuel: 0,
            laser: 0,
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
        let _their_ship = ships_by_role(state, their_role).next().unwrap();

        let position = our_ship.ship_state.position;
        let velocity = our_ship.ship_state.velocity;
        let _field = spec.field.as_ref().unwrap();
        let gravity = get_gravity(our_ship.ship_state.position);

        if position != self.expected_position && self.expected_position != Vec2::default() {
            eprintln!("!!!! Wrong physics, expected {:?} got {:?}", self.expected_position, position);
        }

        self.expected_position = position + velocity + gravity;

        if our_ship.ship_state.ship_params.fuel == 0 {
            return Commands(vec![])
        }

        let mut acceleration = compute_acceleration(spec, our_ship);

        if acceleration == Vec2::default() && state.steps % 5 == 0  {
            acceleration = Vec2 {
                x: rand::thread_rng().gen_range(-1, 1),
                y: rand::thread_rng().gen_range(-1, 1)
            }
        }

        if acceleration != Vec2::default() {
            self.expected_position -= acceleration; // it's thrust!
            let thrust = Command::Accelerate {
                ship_id: our_ship.ship_state.ship_id,
                vector: acceleration,
            };
            Commands(vec![thrust])
        }
        else {
            Commands(vec![])
        }
    }
}


fn main() {
    run_bots_local(OrbitBot::default(), OrbitBot::default());
}
