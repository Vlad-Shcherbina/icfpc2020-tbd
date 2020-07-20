use crate::uforest::*;
use crate::{webapi::Endpoint, ai_interface::Ai};

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

pub fn run_in_submission(ai: impl Ai) {
    let client = Client::from_submission_argv();
    run_with_client(client, ai);
}

fn run_local(player_key: i128, ai: impl Ai) {
    let client = Client::from_player_key(player_key);
    run_with_client(client, ai);
}

fn get_player_keys() -> Result<(i128, i128), String> {
    let responce = Endpoint::Proxy.aliens_send(Data::make_list2(Data::Number(1),
                                                                Data::Number(0)));
    let responce = responce.try_into_vec().ok_or("Vec expected")?;
    let keys = responce[1].clone().try_into_vec().ok_or("Vec expected")?;
    let first_key = keys[0].clone().try_into_vec().ok_or("Vec expected")?;
    let second_key = keys[1].clone().try_into_vec().ok_or("Vec expected")?;
    let first_key = first_key[1].try_as_number().ok_or("Number expected")?;
    let second_key = second_key[1].try_as_number().ok_or("Number expected")?;
    Ok((first_key, second_key))
}

pub fn run_bots_local(ai1: impl Ai + 'static, ai2: impl Ai + 'static) {
    let keys = get_player_keys().expect("Can't obtain player keys");
    let firs_bot = std::thread::spawn(move || {
        run_local(keys.0, ai1);
    });
    let second_bot = std::thread::spawn(move || {
        run_local(keys.1, ai2);
    });
    firs_bot.join().unwrap();
    second_bot.join().unwrap();
    println!("Use this state if you want to see replay");
    println!("(5, ((4, ({}, (nil, (nil, (nil, (nil, ((36, 0), (24324, nil)))))))), (9, (nil, nil))))", keys.0);
    println!("Paste it in galaxy pad");
    println!("Close player_key input");
    println!("Click galaxy");
    println!("Step replay by galaxy clicking");

}