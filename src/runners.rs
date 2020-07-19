use crate::uforest::*;
use crate::ai_interface::Ai;

// TODO: also run_practice_game(), or something

pub fn run_in_submission(mut ai: impl Ai) {
    let client = Client::from_submission_argv();

    let join_request = ai.choose_join_request();
    dbg!(&join_request);
    let mut gr = client.join(join_request);
    loop {
        dbg!(&gr);
        match gr.stage {
            Stage::Finished => return,
            Stage::NotStarted => {
                assert!(gr.state.is_none());
                let initial_ship = ai.initial_ship_params(&gr.spec);
                dbg!(&initial_ship);
                gr = client.start(initial_ship);
            }
            Stage::Started => {
                let commands = ai.choose_commands(&gr.spec, &gr.state.unwrap());
                dbg!(&commands);
                gr = client.commands(commands);
            }
        }
    }
}
