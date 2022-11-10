#![no_std]
use gstd::{prelude::*, ActorId};

#[derive(Encode, Decode, TypeInfo)]
pub struct YourTurn {
    pub players: BTreeMap<ActorId, PlayerInfo>,
    pub properties: BTreeMap<u8, (Vec<Gear>, u32, u32)>,
    pub ownership: BTreeMap<u8, ActorId>,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum GameAction {
    StartRegistration,
    Register {
        player: ActorId,
    },
    Play,
    ThrowRoll {
        pay_fine: bool,
        properties_for_sale: Option<Vec<u8>>,
    },
    AddGear {
        properties_for_sale: Option<Vec<u8>>,
    },
    Upgrade {
        properties_for_sale: Option<Vec<u8>>,
    },
    BuyCell {
        properties_for_sale: Option<Vec<u8>>,
    },
    PayRent {
        properties_for_sale: Option<Vec<u8>>,
    },
}

#[derive(Encode, Decode, TypeInfo)]
pub enum GameEvent {
    Registered,
    StartRegistration,
    GameFinished {
        winner: ActorId,
    },
    StrategicError,
    StrategicSuccess,
    Step {
        players: BTreeMap<ActorId, PlayerInfo>,
        properties: BTreeMap<u8, (Vec<Gear>, u32, u32)>,
    },
    Jail {
        in_jail: bool,
        position: u8,
    },
}
#[derive(Default, Debug, Clone, Encode, Decode, TypeInfo)]
pub struct PlayerInfo {
    pub position: u8,
    pub balance: u32,
    pub debt: u32,
    pub in_jail: bool,
    pub round: u128,
    pub cells: BTreeSet<u8>,
    pub penalty: u8,
}

#[derive(PartialEq, Encode, Decode, Clone, TypeInfo)]
pub enum Gear {
    Platinum,
    Silver,
    Gold,
}

impl Gear {
    pub fn upgrade(&self) -> Self {
        match *self {
            Self::Platinum => Self::Silver,
            Self::Silver => Self::Gold,
            Self::Gold => Self::Gold,
        }
    }
}

#[derive(Debug, PartialEq, Clone, TypeInfo, Encode, Decode)]
pub enum GameStatus {
    Registration,
    Play,
    Finished,
}

impl Default for GameStatus {
    fn default() -> Self {
        Self::Registration
    }
}
