use tbd::runners::{run_bots_local, run_in_submission};
use tbd::ai_interface::Ai;
use tbd::uforest::*;
use tbd::bot_util::*;

pub struct Bee {}

pub fn acc(Vec2 { x, y }: Vec2) -> Vec2 {
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

// pub fn predict(mut pos: Vec2, mut vel: Vec2, mut thrust: i128)

pub fn norm_range(mut pos: Vec2, mut vel: Vec2, time: i32) -> (i128, i128) {
    let mut min = pos.norm();
    let mut max = pos.norm();
    for _ in 0..time {
        vel = vel + get_gravity(pos);
        pos = pos + vel;
        min = min.min(pos.norm());
        max = max.max(pos.norm());
    }
    (min, max)
}

pub fn defender_norm_score((min, max): (i128, i128), field: &Field) -> i128 {
    if max >= field.field_radius {
        return -1000;
    }
    if min <= field.planet_radius {
        return -1000;
    }
    min
}

impl Ai for Bee {
    fn choose_join_request(&mut self) -> JoinRequest {
        JoinRequest { mystery: Data::Nil }
    }

    fn initial_ship_params(&mut self, spec: &GameSpec) -> ShipParams {
        let max_cost = spec.bounds.max_cost;
        ShipParams {
            fuel: max_cost - HULL_COST,
            laser: 0,
            cooling: 0,
            hull: 1,
        }
    }

    fn choose_commands(&mut self, spec: &GameSpec, state: &GameState) -> Commands {
        let field = spec.field.as_ref().unwrap();
        let mut commands = Vec::new();
        let time = 150;
        for ship in ships_by_role(state, spec.role) {
            let s = &ship.ship_state;
            let pos = s.position;
            let vel = s.velocity;

            let a = acc(pos);

            let fwd = defender_norm_score(norm_range(pos, vel + a, time), field);
            let hold = defender_norm_score(norm_range(pos, vel, time), field);
            let back = defender_norm_score(norm_range(pos, vel - a, time), field);
            if hold < fwd.max(back) {
                commands.push(Command::Accelerate {
                    ship_id: s.ship_id,
                    vector: if fwd > back { -a } else { a },
                });
            }
            /*let a = acc(s.position);
            if predict_collisions(s.position, s.velocity + a, field).fly_off.is_none() {
                commands.push(Command::Accelerate {
                    ship_id: s.ship_id,
                    vector: -a,
                });
            }*/
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
