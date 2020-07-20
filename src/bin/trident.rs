use tbd::runners::{run_bots_local, run_in_submission};
use tbd::ai_interface::Ai;
use tbd::uforest::*;
use tbd::bot_util::*;
use tbd::bee_player_lib::{atk_score, def_score, predict, ccw};


fn fork_params(p: &ShipParams) -> ShipParams {
    assert!(p.hull >= 2);
    let new_fuel = (p.fuel / 2).min(50);
    let laser = p.laser * new_fuel / (p.fuel + 1);
    let cooling = p.cooling * new_fuel / (p.fuel + 1);
    let hull = p.hull * new_fuel / (p.fuel + 1);
    let hull = hull.max(1).min(hull / 2);

    ShipParams {
        fuel: new_fuel,
        laser,
        cooling,
        hull,
    }
}

pub struct Trident {}

impl Ai for Trident {
    fn initial_ship_params(&mut self, spec: &GameSpec) -> ShipParams {
        let mut params = ShipParams {
            fuel: 0,
            laser: 60,
            cooling: 5,
            hull: 10,
        };
        compute_fuel_from_params(&mut params, spec).unwrap();
        params
    }

    fn choose_commands(&mut self, spec: &GameSpec, state: &GameState) -> Commands {
        let mut commands = Vec::new();
        let their_role = if spec.role == Role::Attacker {
            Role::Defender
        } else {
            Role::Attacker
        };
        let their_ships: Vec<ShipState> = ships_by_role(state, their_role).cloned().collect();
        let num_ships = ships_by_role(state, spec.role).count();
        for ship in ships_by_role(state, spec.role) {
            // let s = ship.ship_state;
            // if state.steps > 5 && ship.ship_params.hull >= 2 {
            //     todo!("fork");
            // }

            let acc = compute_thrust(spec, ship, spec.role);
            if acc != Vec2::default() {
                commands.push(Command::Accelerate {
                    ship_id: ship.ship_id,
                    vector: acc,
                });
                continue;
            }

            // let role = if state.steps <= 5 || state.steps > 20 {
            if num_ships > 1 && [7, 10, 15, 20, 25].contains(&state.steps) {
                if rand::random() {
                    Role::Attacker
                } else {
                    Role::Defender
                }
            } else {
                Role::Defender
            };

            let params = &ship.ship_params;
            if params.fuel > 8 && params.hull > 1 {
                commands.push(Command::Fork {
                    ship_id: ship.ship_id,
                    new_ship_params: fork_params(params),
                });
                continue;
            };

            if let Some(shot) = best_shot(ship, &their_ships) {
                commands.push(shot);
                continue;
            }

        }
        Commands(commands)
    }
}

pub fn compute_thrust(spec: &GameSpec, ship: &ShipState, role: Role) -> Vec2 {
    let field = spec.field.as_ref().unwrap();
    let pos = ship.position;
    let vel = ship.velocity;

    let score = match role {
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

fn main() {
    if tbd::is_running_in_submission() {
        run_in_submission(Trident {});
    } else {
        run_bots_local(
            Trident {},
            Trident {},
        );
    }
}
