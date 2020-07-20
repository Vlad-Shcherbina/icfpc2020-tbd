// Misc stuff that could be useful in various bots.

use crate::uforest::*;
use std::cmp::min;

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

pub fn on_same_orbit(ship1: &ShipState, ship2: &ShipState) -> bool {
    ship1.position == ship2.position &&
    ship1.velocity == ship2.velocity
}

// is a given ship on the exact same orbit as any other ships from the list?
// O(n^2) is not great
pub fn detect_exact_overlap(ship: &ShipState, other_ships: &Vec<ShipState>) -> bool {
    for fs in other_ships{
        if ship.ship_id > fs.ship_id &&
            on_same_orbit(ship, fs) {
                return true
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

/* Some painstakingly collected data:

32
0-8   full
9-12  half
13-13 quarter
14-18 zero
19-19 quarter
20-23 half
24-32 full
64
0-10   full
11-18  half
19-20  quarter
21-43  zero
44-45  quarter
46-53  half
54-64  full
Full damage width:
100 0-10
120 0-7
128 0-5
160 Doesn't exist.
At 104 half-damage extends to 20.
At 144 half-damage extends to 12.
*/
pub fn shot_quality(dist: Vec2) -> f64 {
    fn lerp(left: f64, left_y : f64, right: f64, right_y: f64, x: f64, y: f64) -> f64 {
        let pos = (x - left) / (right - left);
        let cutoffy = left_y * (1.0 - pos) + right_y * pos;

        if cutoffy < 1.0 || y > cutoffy {
            0.0
        } else {
            1.0 - y / cutoffy
        }
    }

    let mut x = dist.x.abs() as f64;
    let mut y = dist.y.abs() as f64;
    if y > x {
        let tmp = x;
        x = y;
        y = tmp;
    }

    if y > x / 2.0 {
        y = x - y;
    }

    if x <= 3.0 {
        1.0
    }
    else if x <= 32.0 {
        lerp(0.0, 0.0, 32.0, 8.0, x, y)
    }
    else if x <= 64.0 {
        lerp(32.0, 8.0, 64.0, 16.0, x, y)
    }
    else {
        lerp(64.0, 16.0, 120.0, 0.0, x, y)
    }
}

pub fn best_shot(me: &ShipState, ships: &Vec<ShipState>) -> Option<Command> {
    let mut best_chance = -1.0;
    let mut target: Vec2 = Vec2::default();

    let my_expected_position = me.position +
        me.velocity +
        get_gravity(me.position);

    for ship in ships {
        let expected_position =
            ship.position +
            ship.velocity +
            get_gravity(ship.position);
        let chance = shot_quality(my_expected_position - expected_position);
        if chance > best_chance {
            best_chance = chance;
            target = expected_position;
        }
    }
    if best_chance > 0.01 {
        Some(Command::Shoot {ship_id: me.ship_id, target: target,
            power: min(me.heat_capacity - me.heat, me.ship_params.laser)})
    }
    else {
        None
    }
}
