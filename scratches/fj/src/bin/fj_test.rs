use tbd::runners::run_bots_local;
use tbd::ai_interface::Ai;
use tbd::uforest::*;
use tbd::bot_util::*;
use tbd::vec2::Vec2;
use rand::Rng;

#[derive(Default)]
pub struct OrbitBot {
    expected_position: Vec2,
}

fn ccw(Vec2 { x, y }: Vec2) -> Vec2 {
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

fn predict(mut pos: Vec2, mut vel: Vec2, mut control: i128) -> Vec<Vec2> {
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

fn def_score(trajectory: &[Vec2], field: &Field) -> i128 {
    let mut min = 1000000;
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
    }
    return min
}

fn atk_score(trajectory: &[Vec2], field: &Field) -> i128 {
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
    return (max - min) / 2
}

fn compute_acceleration(spec: &GameSpec, ship: &Ship) -> Vec2 {
    let field = spec.field.as_ref().unwrap();
    let s = &ship.ship_state;
    let pos = s.position;
    let vel = s.velocity;

    let score = match spec.role {
        Role::Attacker => atk_score,
        Role::Defender => def_score,
    };
    let control = (-30..=30).max_by_key(|&control| {
        score(&predict(pos, vel, control), field)
        - control.abs()  // penalty for wasting fuel
    }).unwrap();

    let a = ccw(pos);
    if control < 0 {
        a
    }
    else if control > 0 {
        -a
    }
    else {
        Vec2::default()
    }
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
