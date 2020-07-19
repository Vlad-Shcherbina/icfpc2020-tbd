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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    x: i128,
    y: i128,
}

#[derive(Debug)]
pub struct Ship {
    pub ship_state: ShipState,
    pub mystery: Data
}

#[derive(Debug)]
pub struct ShipState {
    pub role: i128,
    pub ship_id: i128,
    pub position: Vec2,
    pub mystery3: Data,
    pub ship_params: ShipParams,
    pub number3: i128,
    pub number4: i128,
    pub number5: i128,

}

#[derive(Debug)]
pub struct GameSpec {
    pub timer: i128, // number of max possible steps until game over
    pub role: i128,
    pub mystery2: Data,
    pub mystery3: Data,
    pub mystery4: Data,
}

#[derive(Debug)]
pub struct GameState {
    pub steps: i128, //number of steps from the start of a run
    pub mystery1: Data,
    pub ships_list: Vec<Ship>,
}

// GameResponse should contain _all_ information from the server response.
// If some parts are not yet understood, leave them in mystery fields of type Data.
#[derive(Debug)]
pub struct GameResponse {
    pub success: i128,  // always 1 ??
    pub stage: Stage,
    pub spec: GameSpec,
    pub state: Option<GameState>,
}

#[derive(Debug)]
pub struct JoinRequest {
    pub mystery: Data,
}

#[derive(Debug)]
pub struct ShipParams {
    pub fuel: i128,
    pub number2: i128,
    pub number3: i128,
    pub number4: i128,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Accelerate {
        ship_id: i128,
        vector: Vec2,
    },
    // TODO: add more commands, but keep Unknown around just in case
    Unknown(Data),
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

    pub fn start(&self, i: ShipParams) -> GameResponse {
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
        // c.mystery
        match c {
            Command::Accelerate { ship_id, vector } => Data::make_list3(0, ship_id, vector),
            Command::Unknown(data) => data,
        }
    }
}

impl TryFrom<Data> for Command {
    type Error = String;

    // Never panic, handle all errors!
    fn try_from(data: Data) -> Result<Self, Self::Error> {
        let parts = data.clone().try_into_vec().ok_or("command is not a list")?;
        let kind = parts.first().ok_or("command is empty list")?
            .try_as_number().ok_or("command kind is not number")?;
        Ok(match kind {
            0 => {
                if parts.len() != 3 {
                    Err(format!("accelerate cmd {:?}", parts))?
                }
                let ship_id = parts[1].try_as_number().ok_or("cmd ship id not number")?;
                let vector = Vec2::try_from(parts[2].clone())?;
                Command::Accelerate {
                    ship_id,
                    vector,
                }
            }
            _ => Command::Unknown(data),
        })
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
        let spec = parts[2].clone().try_into()?;
        let state = if parts[3] == Data::Nil { None } else { Some(parts[3].clone().try_into()?) };
        Ok(GameResponse {
            success,
            stage,
            spec,
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

impl TryFrom<Data> for GameSpec {
    type Error = String;

    fn try_from(data: Data) -> Result<Self, Self::Error> {
        if !data.is_list() {
            Err("not a list")?
        }
        let parts = data.try_into_vec().ok_or("not a list")?;
        if parts.len() != 5 {
            Err(format!("{} elements instead of 5", parts.len()))?;
        }
        let timer = parts[0].try_as_number().ok_or("timer is not a number")?;
        let role = parts[1].try_as_number().ok_or("role is not a number")?;
        let mystery2 = parts[2].clone();
        let mystery3 = parts[3].clone();
        let mystery4 = parts[4].clone();
        Ok(GameSpec {
            timer,
            role,
            mystery2,
            mystery3,
            mystery4,
        })
    }
}

impl TryFrom<Data> for GameState {
    type Error = String;

    fn try_from(data: Data) -> Result<Self, Self::Error> {
        if !data.is_list() {
            Err("not a game state")?
        }
        let parts = data.try_into_vec().ok_or("not a game state")?;
        if parts.len() != 3 {
            Err(format!("{} elements instead of 3", parts.len()))?;
        }
        let steps = parts[0].try_as_number().ok_or("# of steps is not a number")?;
        let mystery1 = parts[1].clone();
        let ships_list_data = parts[2].clone().try_into_vec().ok_or("not a list")?;

        let mut ships_list = Vec::new();

        for ls in ships_list_data {
            ships_list.push(ls.try_into()?);
        }

        Ok(GameState {
            steps,
            mystery1,
            ships_list
        })
    }
}

impl From<Vec2> for Data {
    fn from(v: Vec2) -> Data {
        Data::make_cons(v.x, v.y)
    }
}

impl TryFrom<Data> for Vec2 {
    type Error = String;

    fn try_from(data: Data) -> Result<Self, Self::Error> {
        let parts = data.try_to_coords().ok_or("not a pair of numbers")?;
        Ok(Vec2 {
            x : parts.0,
            y : parts.1,
        })
    }
}


impl TryFrom<Data> for ShipParams {
    type Error = String;

    fn try_from(data: Data) -> Result<Self, Self::Error> {
        if !data.is_list() {
            Err("not a list")?
        }
        let parts = data.try_into_vec().ok_or("not a list")?;
        if parts.len() != 4 {
            Err(format!("{} elements instead of 4", parts.len()))?;
        }
        let fuel = parts[0].try_as_number().ok_or("fuel is not a number")?;
        let number2 = parts[1].try_as_number().ok_or("ship param is not a number")?;
        let number3 = parts[2].try_as_number().ok_or("ship param is not a number")?;
        let number4 = parts[3].try_as_number().ok_or("ship param is not a number")?;
        Ok(ShipParams {
            fuel,
            number2,
            number3,
            number4,
        })
    }
}


impl TryFrom<Data> for Ship {
    type Error = String;

    fn try_from(data: Data) -> Result<Self, Self::Error> {
        if !data.is_list() {
            Err("not a list")?
        }
        let parts = data.try_into_vec().ok_or("not a list")?;
        if parts.len() != 2 {
            Err(format!("{} elements instead of 2", parts.len()))?;
        }
        let ship_state = parts[0].clone().try_into()?;
        let mystery = parts[1].clone();
        Ok(Ship {
            ship_state,
            mystery,
        })
    }
}

impl TryFrom<Data> for ShipState {
    type Error = String;

    fn try_from(data: Data) -> Result<Self, Self::Error> {
        if !data.is_list() {
            Err("not a list")?
        }
        let parts = data.try_into_vec().ok_or("not a list")?;
        if parts.len() != 8 {
            Err(format!("{} elements instead of 8", parts.len()))?;
        }
        let role = parts[0].try_as_number().ok_or("shipstate.role not a number")?;
        let ship_id = parts[1].try_as_number().ok_or("shipstate.ship_id not a number")?;
        let position = parts[2].clone().try_into()?;
        let mystery3 = parts[3].clone();
        let ship_params = parts[4].clone().try_into()?;
        let number3 = parts[5].try_as_number().ok_or("shipstate.number3 not a number")?;
        let number4 = parts[6].try_as_number().ok_or("shipstate.number4 not a number")?;
        let number5 = parts[7].try_as_number().ok_or("shipstate.number5 not a number")?;
        Ok(ShipState {
            role,
            ship_id,
            position,
            mystery3,
            ship_params,
            number3,
            number4,
            number5,
        })
    }
}
