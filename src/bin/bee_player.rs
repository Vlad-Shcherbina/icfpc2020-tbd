use tbd::runners::{run_bots_local, run_in_submission};
use tbd::ai_interface::Ai;
use tbd::uforest::*;
use tbd::bot_util::*;

pub struct Bee {}

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

fn atk_score(trajectory: &[Vec2], field: &Field) -> i128 {
    let mut min = 1000000;
    let mut max = -1000000;
    for (i, &p) in trajectory.iter().enumerate() {
        if i < 50 {
            let err = field.planet_radius + 3 - p.norm();
            if err > 0 {
                return -10000;
            }
            let err = p.norm() - field.field_radius + 3;
            if err > 0 {
                return -10000;
            }
        }
        if p.norm() >= field.field_radius - 3 {
            return -5000;
        }

        min = min.min(p.norm());
        max = max.max(p.norm());
    }
    // attacker wants to be on the most eccentric orbit to have
    // more opportunities for approaching defender
    return (max - min) / 2
}

impl Ai for Bee {
    fn initial_ship_params(&mut self, spec: &GameSpec) -> ShipParams {
        let mut params = match spec.role {
            Role::Defender => ShipParams {
                fuel: 0,
                laser: 0,
                cooling: 30,
                hull: 1,
            },
            Role::Attacker => ShipParams {
                fuel: 0,
                laser: 0,
                cooling: 8,
                hull: 1,
            }
        };
        compute_fuel_from_params(&mut params, spec).unwrap();
        params
    }

    fn choose_commands(&mut self, spec: &GameSpec, state: &GameState) -> Commands {
        let field = spec.field.as_ref().unwrap();
        let mut commands = Vec::new();
        for ship in ships_by_role(state, spec.role) {
            let s = &ship.ship_state;
            let pos = s.position;
            let vel = s.velocity;

            let score = match spec.role {
                Role::Attacker => atk_score,
                Role::Defender => def_score,
            };

            let fuel_value = (100 / (ship.ship_state.ship_params.fuel + 1)) + 1;
            let control = (-30..=30).max_by_key(|&control| {
                score(&predict(pos, vel, control), field)
                - fuel_value * control.abs()  // penalty for wasting fuel
            }).unwrap();

            let a = ccw(pos);
            if control != 0 {
                commands.push(Command::Accelerate {
                    ship_id: s.ship_id,
                    vector: if control > 0 { -a } else { a },
                });
            }
        }
        Commands(commands)
    }
}

fn main() {
    if tbd::is_running_in_submission() {
        run_in_submission(Bee {});
    } else {
        run_bots_local(
            Bee {},
            Bee {},
        );
    }
}
