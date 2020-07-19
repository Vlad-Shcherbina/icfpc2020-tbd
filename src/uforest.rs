use crate::webapi::Endpoint;

pub use crate::squiggle::Data;
use std::convert::{TryInto, TryFrom};

// as our understanding of the game API improves this stuff
// well become more and more statically typed

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Stage {
    NotStarted,
    Started,
    Finished,
}

// GameResponse should contain _all_ information from the server response.
// If some parts are not yet understood, leave them in mystery fields of type Data.
#[derive(Debug)]
pub struct GameResponse {
    pub success: i128,  // always 1 ??
    pub stage: Stage,
    pub unknown_list_a: Data,
    pub state: Data,
}

#[derive(Debug)]
pub struct JoinRequest {
    pub mystery: Data,
}

#[derive(Debug)]
pub struct InitialShipParams {
    pub fuel: i128,
    pub number2: i128,
    pub number3: i128,
    pub number4: i128,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    pub mystery: Data,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Commands(pub Vec<Command>);

pub struct Client {
    pub endpoint: Endpoint,
    pub player_key: i128,
}

impl Client {
    pub fn join(&self, j: JoinRequest) -> GameResponse {
        let req = Data::make_list3(2, self.player_key, j.mystery);
        self.endpoint.aliens_send(req).try_into().unwrap()
    }

    pub fn start(&self, i: InitialShipParams) -> GameResponse {
        let i = Data::make_list4(
            i.fuel,
            i.number2,
            i.number3,
            i.number4,
        );
        let req = Data::make_list3(3, self.player_key, i);
        self.endpoint.aliens_send(req).try_into().unwrap()
    }

    pub fn commands(&self, c: Commands) -> GameResponse {
        let req = Data::make_list3(4, self.player_key, Data::from(c));
        self.endpoint.aliens_send(req).try_into().unwrap()
    }

    pub fn from_submission_argv() -> Self {
        let args: Vec<String> = std::env::args().collect();
        assert_eq!(args.len(), 3);
        let server_url = args[1].clone();
        let player_key = &args[2];
        eprintln!("Server URL: {}", server_url);
        eprintln!("Player key: {}", player_key);
        let endpoint = Endpoint::SubmissionServer { server_url };
        let player_key: i128 = player_key.parse().unwrap();
        Client { endpoint, player_key }
    }
}

impl From<Command> for Data {
    fn from(c: Command) -> Self {
        c.mystery
    }
}

impl TryFrom<Data> for Command {
    type Error = String;

    // Never panic, handle all errors!
    fn try_from(data: Data) -> Result<Self, Self::Error> {
        Ok(Command { mystery: data })
    }
}

impl From<Commands> for Data {
    fn from(c: Commands) -> Self {
        c.0.into_iter().map(Data::from).collect()
    }
}

impl TryFrom<Data> for Commands {
    type Error = String;
    // This function shouldn't panic because the web UI calls it
    // on every request (not only on command requests).
    // Handle all errors properly.
    fn try_from(data: Data) -> Result<Self, Self::Error> {
        let commands: Result<Vec<_>, _> = data
            .try_into_vec().ok_or("not a vec")?
            .into_iter()
            .map(Command::try_from).collect();
        commands.map(Commands)
    }
}

impl TryFrom<Data> for GameResponse {
    type Error = String;

    // This function shouldn't panic because the web UI call is
    // on every response (not only game responses).
    // Handle all errors properly.
    fn try_from(data: Data) -> Result<Self, Self::Error> {
        let parts = data.try_into_vec().ok_or("not a list")?;
        if parts.len() != 4 {
            Err(format!("{} elements instead of 4", parts.len()))?;
        }
        let success = parts[0].try_as_number().ok_or("success is not a number")?;
        let stage = parts[1].clone().into();
        let unknown_list_a = parts[2].clone();
        let state = parts[3].clone();
        Ok(GameResponse {
            success,
            stage,
            unknown_list_a,
            state,
        })
    }
}

impl From<Data> for Stage {
    fn from(data: Data) -> Self {
        match data.try_as_number().unwrap() {
            0 => Stage::NotStarted,
            1 => Stage::Started,
            2 => Stage::Finished,
            _ => panic!(),
        }
    }
}
