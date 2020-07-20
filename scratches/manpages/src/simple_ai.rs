use tbd::ai_interface::Ai;
use tbd::uforest::{JoinRequest, ShipParams, GameSpec, Data, GameState, Commands, Role, Command};
use tbd::bot_util::{ships_by_role, get_gravity, predict_collisions};
use tbd::vec2::Vec2;

pub struct OrbitBot {}

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

const FUEL : usize = 0;
const LASER : usize = 1;
const COOLING : usize = 2;
const HULL : usize = 3;

//                    fuel x1, laser x4, cooling x12, hull x2
const POINT_BUY : &[i128 ; 4] = &[ 1, 4, 12, 2 ];

pub fn scare_them_ship(max_cost : i128) -> ShipParams {
    // now let's split point buy so that we have at least 10
    let fuel = 20;
    let hull = 1;
    let cooling = 8;
    let rem_cost = max_cost - fuel - hull * POINT_BUY[HULL] - 0 - cooling * POINT_BUY[COOLING];
    let laser = rem_cost / POINT_BUY[LASER];
    let fuel_surplus = rem_cost % POINT_BUY[LASER];
    ShipParams { fuel, laser, cooling, hull }
}

pub fn orbit_tank(max_cost : i128) -> ShipParams {
    // now let's split point buy so that we have at least 10
    let fuel0 = 10;
    let hull = 1;
    let laser = 0;
    let rem_cost = max_cost - fuel0 - hull * POINT_BUY[3] - 0;
    let cooling = rem_cost / POINT_BUY[2];
    let fuel_surplus = rem_cost % POINT_BUY[2];
    let fuel = fuel0 + fuel_surplus;
    ShipParams { fuel, laser, cooling, hull }
}

impl Ai for OrbitBot {
    fn choose_join_request(&mut self) -> JoinRequest {
        JoinRequest { mystery: Data::Nil }
    }

    // Plan is to fork as much as we can, giving a couple afterburners to each fork
    // after we stand on orbit
    fn initial_ship_params(&mut self, spec: &GameSpec) -> ShipParams {
        orbit_tank(spec.bounds.max_cost)
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

        let colision = predict_collisions(position, velocity, field);
        if colision.collision.is_some() {
            let thrust = Command::Accelerate {
                ship_id: our_ship.ship_state.ship_id,
                vector: acc(our_ship.ship_state.position),
            };
            Commands(vec![thrust])
        } else {
            Commands(vec![])
        }
    }
}
