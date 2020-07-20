// Misc stuff that could be useful in various bots.

use crate::uforest::*;

pub const LASER_COST: i128 = 4;
pub const COOLING_COST: i128 = 12;
pub const HULL_COST: i128 = 2;

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

pub fn ships_by_role(state: &GameState, role: Role) -> impl Iterator<Item = &Ship> {
    state
        .ships_list
        .iter()
        .filter(move |&ship| ship.ship_state.role == role)
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
