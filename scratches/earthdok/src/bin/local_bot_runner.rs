use tbd::webapi::Endpoint;
use tbd::squiggle::Data;
use std::process::Command;
use tbd::project_path;
use tbd::ai_interface::ExampleAi;
use std::time::Duration;

use earthdok::simple_ai;

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

fn main() {
    let keys = get_player_keys().expect("Can't obtain player keys");
    let firs_bot = std::thread::spawn(move || {
        tbd::runners::run_local(keys.0, ExampleAi {});
    });
    let second_bot = std::thread::spawn(move || {
        tbd::runners::run_local(keys.1, simple_ai::SimpleAi {});
    });
    firs_bot.join().unwrap();
    second_bot.join().unwrap();
    println!("Use this state if you want to see replay");
    println!("(5, ((4, ({}, (nil, (nil, (nil, (nil, ((36, 0), (24324, nil)))))))), (9, (nil, nil))))", keys.0);
}