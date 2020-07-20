use tbd::runners::run_bots_local;
use tbd::ai_interface::Ai;
use tbd::uforest::{JoinRequest, ShipParams, GameSpec, Data, GameState, Commands, Role, Command};
use tbd::bot_util::{ships_by_role, get_gravity, predict_collisions, compute_fuel_from_params};
use tbd::vec2::Vec2;

#[derive(Default)]
pub struct OrbitBot {
    expected_position: Vec2,
}

pub fn acc(pos: Vec2) -> Vec2 {
    // TODO: what happens at abs(x) == abs(y)??
    if pos.x.abs() >= pos.y.abs() {
        if pos.x > 0 {
            Vec2 { x: -1, y: 1 }
        } else {
            Vec2 { x: 1, y: -1 }
        }
    } else {
        if pos.y > 0 {
            Vec2 { x: -1, y: -1 }
        } else {
            Vec2 { x: 1, y: 1 }
        }
    }
}

impl Ai for OrbitBot {
    fn choose_join_request(&mut self) -> JoinRequest {
        JoinRequest { mystery: Data::Nil }
    }

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

    fn choose_commands(&mut self, spec: &GameSpec, _state: &GameState) -> Commands {
        let our_role = spec.role;
        let their_role = if our_role == Role::Attacker {
            Role::Defender
        } else {
            Role::Attacker
        };

        let our_ship = ships_by_role(_state, our_role).next().unwrap();
        let their_ship = ships_by_role(_state, their_role).next().unwrap();

        let position = our_ship.ship_state.position;
        let velocity = our_ship.ship_state.velocity;
        let field = spec.field.as_ref().unwrap();
        let gravity = get_gravity(our_ship.ship_state.position);

        if position != self.expected_position && self.expected_position != Vec2::default() {
            eprintln!("!!!! Wrong physics, expected {:?} got {:?}", self.expected_position, position);
        }

        self.expected_position = position + velocity + gravity;
        let colision = predict_collisions(position, velocity, field);

        if colision.collision.is_some() {
            let acceleration = acc(our_ship.ship_state.position);
            self.expected_position -= acceleration; // it's thrust!
            let thrust = Command::Accelerate {
                ship_id: our_ship.ship_state.ship_id,
                vector: acceleration,
            };
            Commands(vec![thrust])
        } else {
            Commands(vec![])
        }
    }
}

fn main() {
    run_bots_local(OrbitBot::default(), OrbitBot::default());
}
