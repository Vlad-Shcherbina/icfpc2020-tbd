use tbd::uforest::*;
use tbd::ai_interface::Ai;
use tbd::bot_util as utils;

pub struct PredictingAi {

}

impl Ai for PredictingAi {
    fn initial_ship_params(&mut self, _spec: &GameSpec) -> ShipParams {
        ShipParams {
            fuel: 30,
            laser: 20,
            cooling: 20,
            hull: 20,
        }
    }

    fn choose_commands(&mut self, spec: &GameSpec, state: &GameState) -> Commands {
        let me: Vec<&ShipState> = utils::ships_by_role(state, spec.role).collect();
        assert!(me.len() == 1);
        let me = me[0];

        match choose_acceleration(me, spec) {
            Some(a) => Commands(vec![Command::Accelerate { ship_id: me.ship_id, vector: a }]),
            None    => Commands(vec![])
        }
    }
}


// gives best acceleration, in the right direction and ready to be used in Command,
// None if no action is possible or profitable
pub fn choose_acceleration(me: &ShipState, spec: &GameSpec) -> Option<Vec2> {
    let pos = me.position;
    let vel = me.velocity;
    let mut acc: Vec2 = Vec2 {x: 0, y: 0 };
    let mut max = 0;
    // println!("\nTurn: {}", state.steps);
    // println!("Position: {:?}, velocity: {:?}", &pos, &vel);
    // println!("Fuel: {}", me.ship_params.fuel);
    // println!("Prediction: {}", utils::predict_collisions(pos, vel, r_planet, r_field).time());

    if me.ship_params.fuel == 0 { return None; }
    if me.heat + 8 - me.ship_params.cooling > me.heat_capacity {
        return None;
    }

    for x in -1..2 {
        for y in -1..2 {
            let delta = Vec2{x, y};
            let m = utils::predict_collisions(pos, vel + delta, spec.field.as_ref().unwrap()).time();
            if m == -1 || m > max {
                acc = -delta;
                max = m;
            }
            if max == -1 { break; }
        }
        if max == -1 { break; }
    }
    // println!("Decision: {:?}, estimated as {}", &acc, max);
    if acc.x == 0 && acc.y == 0 { return None; }
    Some(acc)
}
