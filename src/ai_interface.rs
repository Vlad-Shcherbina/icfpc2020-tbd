use crate::uforest::*;

pub trait Ai: Send {
    fn choose_join_request(&mut self) -> JoinRequest {
        let join = Data::make_list2(103652820, 192496425430);
        JoinRequest { upgrades: join }
    }

    fn initial_ship_params(&mut self, spec: &GameSpec) -> ShipParams;
    fn choose_commands(&mut self, spec: &GameSpec, state: &GameState) -> Commands;
}

pub struct ExampleAi {}

impl Ai for ExampleAi {
    fn initial_ship_params(&mut self, _spec: &GameSpec) -> ShipParams {
        ShipParams {
            fuel: 1,
            laser: 1,
            cooling: 1,
            hull: 1,
        }
    }

    fn choose_commands(&mut self, _spec: &GameSpec, _state: &GameState) -> Commands {
        Commands(vec![])
    }
}

// See example_player.rs
