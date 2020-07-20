use crate::uforest::*;
use crate::bot_util::*;
use rand::Rng;


pub fn ccw(Vec2 { x, y }: Vec2) -> Vec2 {
    if x > 0 {
        if y > 0 {
            Vec2 { x: 1, y: -1 }
        } else {
            Vec2 { x: -1, y: -1 }
        }
    } else {
        if y > 0 {
            Vec2 { x: 1, y: 1 }
        } else {
            Vec2 { x: -1, y: 1 }
        }
    }
}

pub fn predict(mut pos: Vec2, mut vel: Vec2, mut control: i128) -> Vec<Vec2> {
    let mut result = Vec::new();
    for _ in 0..250 {
        let a = ccw(pos);
        if control > 0 {
            vel = vel + a;
            control -= 1;
        }
        if control < 0 {
            vel = vel - a;
            control += 1;
        }
        vel = vel + get_gravity(pos);
        pos = pos + vel;
        result.push(pos)
    }
    result
}

pub fn def_score(trajectory: &[Vec2], field: &Field) -> i128 {
    let mut min = 1000000;
    let mut max = -1000000;
    for (i, &p) in trajectory.iter().enumerate() {
        if i < 50 {
            let err = field.planet_radius + 3 - p.norm();
            if err > 0 {
                return -1000 * err;
            }
            let err = p.norm() - field.field_radius + 3;
            if err > 0 {
                return -1000 * err;
            }
        }
        if p.norm() >= field.field_radius - 3 {
            return -500;
        }
        min = min.min(p.norm());
        max = max.max(p.norm());
    }
    // defender wants to be on the largest possible orbit
    return min
}

pub fn atk_score(trajectory: &[Vec2], field: &Field) -> i128 {
    let mut min = 1000000;
    let mut max = -1000000;
    for (i, &p) in trajectory.iter().enumerate() {
        let err = field.planet_radius + 3 - p.norm();
        if err > 0 {
            return -10000;
        }
        let err = p.norm() - field.field_radius + 3;
        if err > 0 {
            return -10000;
        }

        min = min.min(p.norm());
        max = max.max(p.norm());
    }
    // attacker wants to be on the most eccentric orbit to have
    // more opportunities for approaching defender
    return (max - min) / 2
}

pub fn compute_thrust(spec: &GameSpec, ship: &ShipState) -> Vec2 {
    let field = spec.field.as_ref().unwrap();
    let pos = ship.position;
    let vel = ship.velocity;

    let score = match spec.role {
        Role::Attacker => atk_score,
        Role::Defender => def_score,
    };
    let control = (-30..=30).max_by_key(|&control| {
        score(&predict(pos, vel, control), field)
        - control.abs()  // penalty for wasting fuel
    }).unwrap();

    let a = ccw(pos);
    if control < 0 { a }
    else if control > 0 { -a }
    else { Vec2::default() }
}

pub fn perturb(thrust: Vec2) -> Vec2 {
    if thrust == Vec2::default()  {
        Vec2 {
            x: rand::thread_rng().gen_range(-1, 1),
            y: rand::thread_rng().gen_range(-1, 1)
        }
    } else {
        thrust
    }
}
