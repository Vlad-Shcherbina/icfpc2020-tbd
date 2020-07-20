use tbd::runners::{run_in_submission, run_bots_local};
use tbd::ai_interface::Ai;
use tbd::bot_util::{ships_by_role };
use tbd::uforest::{JoinRequest, ShipParams, GameSpec, Data, GameState, Commands, Command };

// fuel x1, laser x4, cooling x12, hull x2

const POINT_BUY : &[i128 ; 4] = &[ 1, 4, 12, 2 ];
const FUEL : usize = 0;
const LASER : usize = 1;
const COOLING : usize = 2;
const HULL : usize = 3;

enum ScarecrowType { 
    Sniper,
    Carrier,
    Kamikaze,
    KamikazeCrew,
}

pub struct ScarecrowBot {
    t: ScarecrowType,
}

fn scarecrow_params(max_cost : i128, t: &ScarecrowType) -> ShipParams {
    // now let's split point buy so that we have at least 10
    let cooling = 1;
    match t {
        // Laser >> everything
        ScarecrowType::Sniper => {
            let hull = 1;
            let fuel = 5;
            let min = hull * POINT_BUY[HULL] + cooling * POINT_BUY[COOLING] + fuel * POINT_BUY[FUEL];
            let laser = (max_cost - min) / POINT_BUY[LASER];
            ShipParams { fuel, laser, cooling, hull }
        },
        // Hull >> everything
        ScarecrowType::Carrier => {
            let laser = 5;
            let fuel = 5;
            let min = laser * POINT_BUY[LASER] + cooling * POINT_BUY[COOLING] + fuel * POINT_BUY[FUEL];
            let hull = (max_cost - min) / POINT_BUY[HULL];
            ShipParams { fuel, laser, cooling, hull }
        },
        // Fuel >> everything + Laser = 0
        ScarecrowType::Kamikaze => {
            let laser = 0;
            let hull = 1;
            let min = laser * POINT_BUY[LASER] + cooling * POINT_BUY[COOLING] + hull * POINT_BUY[HULL];
            let fuel = (max_cost - min) / POINT_BUY[FUEL];
            ShipParams { fuel, laser, cooling, hull }
        },
        // Fuel >> everything + Hull > 0
        ScarecrowType::KamikazeCrew => {
            let laser = 0;
            let hull = 10;
            let min = laser * POINT_BUY[LASER] + cooling * POINT_BUY[COOLING] + hull * POINT_BUY[HULL];
            let fuel = (max_cost - min) / POINT_BUY[FUEL];
            ShipParams { fuel, laser, cooling, hull }
        },
    }
}


impl Ai for ScarecrowBot {
    fn initial_ship_params(&mut self, spec: &GameSpec) -> ShipParams {
        let p = scarecrow_params(spec.bounds.max_cost, &self.t);
        println!("{:?}", p);
        p
    }

    fn choose_commands(&mut self, spec: &GameSpec, _state: &GameState) -> Commands {
        let me = ships_by_role(_state, spec.role).next().unwrap();
        Commands(vec![Command::Detonate{ ship_id: me.ship_id}])
    }
}


fn main() {
    if tbd::is_running_in_submission() {
        // run_in_submission(ScarecrowBot {t: ScarecrowType::Sniper});
        run_in_submission(ScarecrowBot {t: ScarecrowType::Carrier});
        // run_in_submission(ScarecrowBot {t: ScarecrowType::Kamikaze});
        // run_in_submission(ScarecrowBot {t: ScarecrowType::KamikazeCrew});
    } else {
        run_bots_local(
            ScarecrowBot {t: ScarecrowType::Sniper},
            ScarecrowBot {t: ScarecrowType::Carrier}
        );
        run_bots_local(
            ScarecrowBot {t: ScarecrowType::Kamikaze},
            ScarecrowBot {t: ScarecrowType::KamikazeCrew}
        );
    }
}
