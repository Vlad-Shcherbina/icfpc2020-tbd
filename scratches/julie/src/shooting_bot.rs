// use tbd::shooting_bot_lib::*;
use tbd::bot_util::*;
use tbd::uforest::*;
use tbd::ai_interface::Ai;
use tbd::bee_player_lib::compute_thrust;

pub struct ShootingAI {}

impl Ai for ShootingAI {
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
                cooling: 20,
                hull: 1,
            }
        };
        compute_fuel_from_params(&mut params, spec).unwrap();
        params
    }

    fn choose_commands(&mut self, spec: &GameSpec, state: &GameState) -> Commands {
        let mut commands = Vec::new();
        
        let other_role = match &spec.role {
            Role::Attacker => Role::Defender,
            Role::Defender => Role::Attacker,
        };

        for ship in ships_by_role(state, spec.role) {
            let mut expected_heat = ship.heat - ship.ship_params.cooling;
            let acc = compute_thrust(spec, ship);
            if acc != Vec2::default() {
                commands.push(Command::Accelerate {
                    ship_id: ship.ship_id,
                    vector: acc,
                });
                expected_heat += 8;
            }
            if ship.heat_capacity - expected_heat < 10 { continue; }
            for opponent in ships_by_role(state, other_role) {
                if get_hp(opponent) <= 0 { continue; }
                // println!("Shoot at {:?}", get_expected_position(opponent));
                commands.push(Command::Shoot {
                    ship_id: ship.ship_id,
                    target: get_expected_position(opponent),
                    power: ship.ship_params.laser,
                });
                break;
            }
        }        

        Commands(commands)
    }
}
