use crate::uforest::*;
use crate::ai_interface::Ai;

fn run_with_client(client: Client, mut ai: impl Ai) {
    let join_request = ai.choose_join_request();
    dbg!(&join_request);
    println!("Before join {:?}", client.player_key);
    let mut gr = client.join(join_request);
    println!("After join {:?}", client.player_key);
    loop {
        dbg!(&gr);
        match gr.stage {
            Stage::Finished => return,
            Stage::NotStarted => {
                assert!(gr.state.is_none());
                let initial_ship = ai.initial_ship_params(&gr.spec);
                dbg!(&initial_ship);
                println!("Before start {:?}", client.player_key);
                gr = client.start(initial_ship);
                println!("After start {:?}", client.player_key);
            }
            Stage::Started => {
                let commands = ai.choose_commands(&gr.spec, &gr.state.unwrap());
                dbg!(&commands);
                gr = client.commands(commands);
            }
        }
    }
}

pub fn  run_in_submission(mut ai: impl Ai) {
    let client = Client::from_submission_argv();
    run_with_client(client, ai);
}

pub fn run_local(player_key: i128, mut ai: impl Ai) {
    let client = Client::from_player_key(player_key);
    run_with_client(client, ai);
}
