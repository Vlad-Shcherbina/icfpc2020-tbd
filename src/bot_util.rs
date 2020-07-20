// Misc stuff that could be useful in various bots.

use crate::uforest::*;

pub const LASER_COST: i128 = 4;
pub const COOLING_COST: i128 = 12;
pub const HULL_COST: i128 = 2;

pub fn compute_fuel_from_params(params: &mut ShipParams, spec: &GameSpec) -> Result<(), ()> {
    params.fuel = spec.bounds.max_cost - (
        LASER_COST * params.laser +
        COOLING_COST * params.cooling +
        HULL_COST * params.hull);
    if params.fuel < 0 { Err(()) } else { Ok(())}
}

/// takes ship position relative to star
pub fn get_gravity(pos: Vec2) -> Vec2 {
    // TODO: what happens at abs(x) == abs(y)??
    Vec2 {
        x: if pos.x.abs() >= pos.y.abs() {
            -pos.x.signum()
        } else {
            0
        },
        y: if pos.y.abs() >= pos.x.abs() {
            -pos.y.signum()
        } else {
            0
        },
    }
}

pub fn old_ships_by_role(state: &GameState, role: Role) -> impl Iterator<Item = &Ship> {
    state
        .ships_list
        .iter()
        .filter(move |&ship| ship.ship_state.role == role)
}

pub fn ships_by_role(state: &GameState, role: Role) -> impl Iterator<Item = &ShipState> {
    old_ships_by_role(state, role)
        .map(|ship| &ship.ship_state)
}

pub fn get_expected_position(ship: &ShipState) -> Vec2 {
    ship.position + ship.velocity - get_gravity(ship.position)
}

// How much heat can we take without taking damage?
pub fn get_available_heat_sink(ship: &ShipState, thrust: Vec2) -> i128 {
    // TODO: 2x thruster support
    let thruster_heat = if thrust == Vec2::default() { 0 } else { 8 };
    ship.heat_capacity - ship.heat + ship.ship_params.cooling - thruster_heat
}

pub fn get_hp (ship: &ShipState) -> i128 {
    ship.ship_params.fuel + ship.ship_params.laser + ship.ship_params.cooling + ship.ship_params.hull
}

pub fn maybe_push_thrust_command(cmd_vector: &mut Vec<Command>, thrust: Vec2, ship_id: i128) {
    if thrust != Vec2::default() {
        cmd_vector.push(Command::Accelerate {
            ship_id,
            vector: thrust,
        })
    }
}

pub struct TimePrediction {
    pub collision: Option<i128>,
    pub fly_off: Option<i128>,
}

impl TimePrediction {
    // gives total nearest time to disaster of any king, -1 if none expected
    pub fn time(&self) -> i128 {
        let mut m = -1;
        if let Some(a) = self.collision {
            if m == -1 || m > a { m = a; }
        }
        if let Some(a) = self.fly_off {
            if m == -1 || m > a { m = a; }
        }
        m
    }
}

pub fn on_same_orbit(ship1: &Ship, ship2: &Ship) -> bool {
    ship1.ship_state.position == ship2.ship_state.position &&
    ship1.ship_state.velocity == ship2.ship_state.velocity
}

// is a ship on the exact same orbit as any other ships from the list?
// O(n^2) is not great
pub fn detect_exact_overlap(gamestate: &GameState) -> bool {
    let ships = &gamestate.ships_list;
    for fs in ships{
        for ship in ships{
            if ship.ship_state.ship_id > fs.ship_state.ship_id &&
                on_same_orbit(ship, fs) {
                    return true
            }
        }
    }
    false
}

// predicts time to collide to the star or to leave the frame without thrust
pub fn predict_collisions(mut pos: Vec2, mut vel: Vec2, field: &Field) -> TimePrediction {
    let turn_limit = 256;
    let mut count = 1;
    loop {
        let g = get_gravity(pos);
        vel = vel + g;
        pos = pos + vel;
        if pos.x.abs() <= field.planet_radius && pos.y.abs() <= field.planet_radius {
            break TimePrediction { collision: Some(count), fly_off: None };
        }
        if pos.x.abs() >= field.field_radius || pos.y.abs() >= field.field_radius {
            break TimePrediction { collision: None, fly_off: Some(count) };
        }
        count += 1;
        if count >= turn_limit {
            break TimePrediction { collision: None, fly_off: None };
        };
    }
}
