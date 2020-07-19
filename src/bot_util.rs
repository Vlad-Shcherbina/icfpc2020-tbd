// Misc stuff that could be useful in various bots.

use crate::uforest::*;

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

// predicts time to collide to the star or to leave the frame without thrust
pub fn predict_collisions(state: &GameState, role: &Role) -> TimePrediction {
    let ship: Vec<&Ship> = ships_by_role(state, role.clone()).collect();
    assert!(ship.len() == 1);
    let ship = ship[0];
    let r = state.field.as_ref().unwrap().planet_radius;
    let mut position = ship.ship_state.position.clone();
    let mut velocity = ship.ship_state.velocity.clone();

    let mut count = 1;
    loop {
        let g = get_gravity(position);
        velocity = velocity + g;
        position = position + velocity;
        if position.x.abs() <= r && position.y.abs() <= r {
            break TimePrediction { collision: Some(count), fly_off: None };
        }
        if position.x.abs() <= r && position.y.abs() <= r {
            break TimePrediction { collision: None, fly_off: Some(count) };
        }
        count += 1;
        if count >= 256 { 
            break TimePrediction { collision: None, fly_off: None };
        };
    }
}
