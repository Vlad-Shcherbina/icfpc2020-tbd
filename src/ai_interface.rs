use crate::uforest::*;

pub trait Ai {
    fn choose_join_request(&mut self) -> JoinRequest;
    fn initial_ship_params(&mut self, spec: &GameSpec) -> ShipParams;
    fn choose_commands(&mut self, spec: &GameSpec, state: &GameState) -> Commands;
}

// See example_player.rs
