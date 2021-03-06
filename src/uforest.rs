use crate::webapi::Endpoint;

pub use crate::squiggle::Data;
pub use crate::vec2::Vec2;
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
pub enum Role {
    Attacker,
    Defender,
}

#[derive(Debug, Clone)]
pub struct Ship {
    pub ship_state: ShipState,
    pub commands_list: AppliedCommands,
}

#[derive(Debug, Clone)]
pub struct ShipState {
    pub role: Role,
    pub ship_id: i128,
    pub position: Vec2,
    pub velocity: Vec2,
    pub ship_params: ShipParams,
    pub heat: i128,
    pub heat_capacity: i128,
    pub max_thrust: i128,

}

#[derive(Debug, Clone)]
pub struct GameSpec {
    pub timer: i128, // number of max possible steps until game over
    pub role: Role,
    pub bounds: Bounds,
    pub field: Option<Field>,  // None in tutorials without a star?
    pub defender_params: Option<ShipParams>, // a field which only the attacker gets
}

#[derive(Debug, Clone)]
pub struct Field {
    pub planet_radius: i128,
    pub field_radius: i128,
}

#[derive(Debug, Clone)]
pub struct Bounds {
    pub max_cost: i128,
    pub max_thrust: i128,
    pub heat_capacity: i128,
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub steps: i128, //number of steps from the start of a run
    pub field: Option<Field>,
    pub ships_list: Vec<Ship>,
}

// GameResponse should contain _all_ information from the server response.
// If some parts are not yet understood, leave them in mystery fields of type Data.
#[derive(Debug, Clone)]
pub struct GameResponse {
    pub success: i128,  // always 1 ??
    pub stage: Stage,
    pub spec: GameSpec,
    pub state: Option<GameState>,  // can only be None if stage == NotStarted
}

#[derive(Debug, Clone)]
pub struct JoinRequest {
    pub upgrades: Data,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShipParams {
    pub fuel: i128,
    pub laser: i128,
    pub cooling: i128,
    pub hull: i128,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Accelerate {
        ship_id: i128,
        vector: Vec2,
    },
    Detonate {
        ship_id: i128,
    },
    Shoot {
        ship_id: i128,
        target: Vec2,
        power: i128
    },
    Fork {
        ship_id: i128, // ship to fork
        new_ship_params: ShipParams,
    },
    // TODO: add more commands, but keep Unknown around just in case
    Unknown(Data),
}

// the difference between Commands and AppliedCommands is that
// AppliedCommands is used in Ship descriptions whereas Commands
// are the ones directly executed.

#[derive(Debug, Clone, PartialEq)]
pub struct Commands(pub Vec<Command>);

#[derive(Debug, Clone, PartialEq)]
pub enum AppliedCommand {
    Accelerate {
        vector: Vec2,
    },
    Detonate {
        number1: i128,
        blast_radius: i128,
    },
    Shoot {
        target: Vec2,
        power: i128,
        number2: i128,
        number3: i128
    },
    Fork {
        new_ship_params: ShipParams,
    },
    // TODO: add more commands, but keep Unknown around just in case
    Unknown(Data),
}

#[derive(Debug, Clone, PartialEq)]
pub struct AppliedCommands(pub Vec<AppliedCommand>);

impl IntoIterator for AppliedCommands {
    type Item = AppliedCommand;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub struct Client {
    pub endpoint: Endpoint,
    pub player_key: i128,
}

impl Client {
    pub fn join(&self, j: JoinRequest) -> GameResponse {
        let req = Data::make_list3(2, self.player_key, j.upgrades);
        self.endpoint.aliens_send(req).try_into().unwrap()
    }

    pub fn start(&self, i: ShipParams) -> GameResponse {
        let i = Data::make_list4(
            i.fuel,
            i.laser,
            i.cooling,
            i.hull,
        );
        let req = Data::make_list3(3, self.player_key, i);
        self.endpoint.aliens_send(req).try_into().unwrap()
    }

    pub fn commands(&self, c: Commands) -> GameResponse {
        let req = Data::make_list3(4, self.player_key, Data::from(c));
        self.endpoint.aliens_send(req).try_into().unwrap()
    }

    pub fn from_submission_argv() -> Self {
        assert!(crate::is_running_in_submission());
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

    pub fn from_player_key(player_key: i128) -> Self {
        let endpoint = Endpoint::Proxy;
        Client { endpoint, player_key }
    }
}

impl From<Command> for Data {
    fn from(c: Command) -> Self {
        match c {
            Command::Accelerate { ship_id, vector } => Data::make_list3(0, ship_id, vector),
            Command::Detonate { ship_id } => Data::make_list2(1, ship_id),
            Command::Shoot { ship_id, target , power} => Data::make_list4(2, ship_id, target, power),
            Command::Fork { ship_id, new_ship_params} => Data::make_list3(3, ship_id, new_ship_params),
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
            1 => {
                if parts.len() != 2 {
                    Err(format!("detonate cmd {:?}", parts))?
                }
                let ship_id = parts[1].try_as_number().ok_or("cmd ship id not number")?;
                Command::Detonate {
                    ship_id,
                }
            }
            2 => {
                if parts.len() != 4 {
                    Err(format!("shoot cmd {:?}", parts))?
                }
                let ship_id = parts[1].try_as_number().ok_or("cmd ship id not number")?;
                let target = Vec2::try_from(parts[2].clone())?;
                let power = parts[3].try_as_number().ok_or("cmd power not number")?;

                Command::Shoot {
                    ship_id,
                    target,
                    power
                }
            }
            3 => {
                if parts.len() != 3 {
                    Err(format!("fork cmd {:?}", parts))?
                }
                let ship_id = parts[1].try_as_number().ok_or("cmd ship id not number")?;
                let new_ship_params = parts[2].clone().try_into()?;

                Command::Fork {
                    ship_id,
                    new_ship_params
                }
            }
            _ => Command::Unknown(data),
        })
    }
}

impl From<AppliedCommand> for Data {
    fn from(c: AppliedCommand) -> Self {
        match c {
            AppliedCommand::Accelerate { vector } => Data::make_list2(0, vector),
            AppliedCommand::Detonate { number1, blast_radius } => Data::make_list3(1, number1, blast_radius),
            AppliedCommand::Shoot { target, power, number2, number3 } => Data::make_list5(2, target, power, number2, number3 ),
            AppliedCommand::Fork { new_ship_params } => Data::make_list2(3, new_ship_params),
            AppliedCommand::Unknown(data) => data,
        }
    }
}

impl TryFrom<Data> for AppliedCommand {
    type Error = String;

    // Never panic, handle all errors!
    fn try_from(data: Data) -> Result<Self, Self::Error> {
        let parts = data.clone().try_into_vec().ok_or("applied command is not a list")?;
        let kind = parts.first().ok_or("applied command is empty list")?
            .try_as_number().ok_or("applied command kind is not number")?;
        Ok(match kind {
            0 => {
                if parts.len() != 2 {
                    Err(format!("accelerate cmd {:?}", parts))?
                }
                let vector = Vec2::try_from(parts[1].clone())?;
                AppliedCommand::Accelerate {
                    vector,
                }
            }
            1 => {
                if parts.len() != 3 {
                    Err(format!("detonate cmd {:?}", parts))?
                }
                let number1 = parts[1].try_as_number().ok_or("detonate number1 not number")?;
                let blast_radius = parts[2].try_as_number().ok_or("detonate number2 not number")?;
                AppliedCommand::Detonate {
                    number1,
                    blast_radius
                }
            }
            2 => {
                if parts.len() != 5 {
                    Err(format!("shoot cmd {:?}", parts))?
                }
                let target = Vec2::try_from(parts[1].clone())?;
                let power = parts[2].try_as_number().ok_or("shoot number1 not number")?;
                let number2 = parts[3].try_as_number().ok_or("shoot number2 not number")?;
                let number3 = parts[4].try_as_number().ok_or("shoot number3 not number")?;

                AppliedCommand::Shoot {
                    target,
                    power,
                    number2,
                    number3,
                }
            }
            3 => {
                if parts.len() != 2 {
                    Err(format!("fork cmd {:?}", parts))?
                }
                let new_ship_params = parts[1].clone().try_into()?;

                AppliedCommand::Fork {
                    new_ship_params
                }
            }
            _ => AppliedCommand::Unknown(data),
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

impl From<AppliedCommands> for Data {
    fn from(c: AppliedCommands) -> Self {
        c.0.into_iter().map(Data::from).collect()
    }
}

impl TryFrom<Data> for AppliedCommands {
    type Error = String;
    // This function shouldn't panic because the web UI calls it
    // on every request (not only on command requests).
    // Handle all errors properly.
    fn try_from(data: Data) -> Result<Self, Self::Error> {
        let applied_commands: Result<Vec<_>, _> = data
            .try_into_vec().ok_or("not a vec")?
            .into_iter()
            .map(AppliedCommand::try_from).collect();
        applied_commands.map(AppliedCommands)
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

impl TryFrom<Data> for Role {
    type Error = String;

    fn try_from(data: Data) -> Result<Self, Self::Error> {
        let role = data.try_as_number().ok_or("role not well-defined")?;
        match role {
            0 => Ok(Role::Attacker),
            1 => Ok(Role::Defender),
            _ => Err("role not attacker or defender")?,
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
        let role = parts[1].clone().try_into()?;
        let bounds = parts[2].clone().try_into()?;
        let field = parts[3].clone().try_into()?;
        let defender_params = if parts[4] == Data::Nil { None } else { Some(parts[4].clone().try_into()?) };
        Ok(GameSpec {
            timer,
            role,
            bounds,
            field,
            defender_params,
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
        let field = parts[1].clone().try_into()?;
        let ships_list_data = parts[2].clone().try_into_vec().ok_or("not a list")?;

        let mut ships_list = Vec::new();

        for ls in ships_list_data {
            ships_list.push(ls.try_into()?);
        }

        Ok(GameState {
            steps,
            field: field,
            ships_list
        })
    }
}

impl From<ShipParams> for Data {
    fn from(p: ShipParams) -> Self {
        Data::make_list4(p.fuel, p.laser, p.cooling, p.hull)
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
        let laser = parts[1].try_as_number().ok_or("laser is not a number")?;
        let cooling = parts[2].try_as_number().ok_or("cooling is not a number")?;
        let hull = parts[3].try_as_number().ok_or("hull is not a number")?;
        Ok(ShipParams {
            fuel,
            laser,
            cooling,
            hull,
        })
    }
}

impl TryFrom<Data> for Bounds {
    type Error = String;

    fn try_from(data: Data) -> Result<Self, Self::Error> {
        if !data.is_list() {
            Err("not a list")?
        }
        let parts = data.try_into_vec().ok_or("not a list")?;
        if parts.len() != 3 {
            Err(format!("{} elements instead of 3", parts.len()))?;
        }
        let max_cost = parts[0].try_as_number().ok_or("bounds.max_cost not a number")?;
        let max_thrust = parts[1].try_as_number().ok_or("bounds.max_thrust not a number")?;
        let heat_capacity = parts[2].try_as_number().ok_or("bounds.heat_capacity not a number")?;
        Ok(Bounds {
            max_cost,
            max_thrust,
            heat_capacity,
        })
    }
}

impl TryFrom<Data> for Option<Field> {
    type Error = String;

    fn try_from(data: Data) -> Result<Self, Self::Error> {
        if !data.is_list() {
            Err("not a list")?
        }
        let parts = data.try_into_vec().ok_or("not a list")?;
        if parts.len() == 0 {
            return Ok(None)
        }
        if parts.len() != 2 {
            Err(format!("{} elements instead of 2", parts.len()))?;
        }
        let planet_radius = parts[0].try_as_number().ok_or("Field.planet_radius not a number")?;
        let field_radius = parts[1].try_as_number().ok_or("Field.field_radius not a number")?;
        return Ok(Some(Field { planet_radius, field_radius }))
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
        let commands_list = parts[1].clone().try_into()?;
        Ok(Ship {
            ship_state,
            commands_list,
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
        let role = parts[0].clone().try_into()?;
        let ship_id = parts[1].try_as_number().ok_or("shipstate.ship_id not a number")?;
        let position = parts[2].clone().try_into()?;
        let velocity = parts[3].clone().try_into()?;
        let ship_params = parts[4].clone().try_into()?;
        let heat = parts[5].try_as_number().ok_or("shipstate.heat not a number")?;
        let heat_capacity = parts[6].try_as_number().ok_or("shipstate.heat_capacity not a number")?;
        let max_thrust = parts[7].try_as_number().ok_or("shipstate.max_thrust not a number")?;
        Ok(ShipState {
            role,
            ship_id,
            position,
            velocity,
            ship_params,
            heat,
            heat_capacity,
            max_thrust,
        })
    }
}
