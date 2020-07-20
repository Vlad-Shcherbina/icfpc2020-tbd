use tbd::runners::{run_bots_local, run_in_submission};
use tbd::ai_interface::Ai;
use tbd::uforest::*;
use tbd::bot_util::*;

pub struct BeeFork {}

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

// a simple function of dividing everything by 2
// TODO: extend with logic for defender / attacker
fn fork_params(old_params: &ShipParams) -> ShipParams {

    ShipParams {
        fuel: old_params.fuel - old_params.fuel / 2,
        laser: old_params.laser - old_params.laser / 2,
        cooling: old_params.cooling - old_params.cooling / 2,
        hull: old_params.hull - old_params.hull / 2,
    }

}

impl Ai for BeeFork {
    fn initial_ship_params(&mut self, spec: &GameSpec) -> ShipParams {
        let max_cost = spec.bounds.max_cost;
        let laser = 0;
        let hull = 4;
        let cooling = match spec.role {
            Role::Defender => 30,
            Role::Attacker => 8,
        };
        ShipParams {
            fuel: max_cost - hull * HULL_COST - cooling * COOLING_COST - laser * LASER_COST,
            laser,
            cooling,
            hull,
        }
    }

    fn choose_commands(&mut self, spec: &GameSpec, state: &GameState) -> Commands {
        let field = spec.field.as_ref().unwrap();
        let mut commands = Vec::new();
        for ship in ships_by_role(state, spec.role) {

            let s = &ship.ship_state;
            let pos = s.position;
            let vel = s.velocity;

            // fork every turn until (arbitrary) minimum values are reached
            let params = &s.ship_params;
            if params.fuel > 8 && params.hull > 1 {
                commands.push(Command::Fork {
                    ship_id: s.ship_id,
                    new_ship_params: fork_params(params),
                });
            };

            let score = match spec.role {
                Role::Attacker => atk_score,
                Role::Defender => def_score,
            };
            let control = (-30..=30).max_by_key(|&control| {
                score(&predict(pos, vel, control), field)
                - control.abs()  // penalty for wasting fuel
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
        run_in_submission(BeeFork {});
    } else {
        run_bots_local(
            BeeFork {},
            BeeFork {},
        );
    }
}
