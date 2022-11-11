#![no_std]
use gstd::{exec, msg, prelude::*, ActorId};
pub const NUMBER_OF_CELLS: u8 = 40;
pub const NUMBER_OF_PLAYERS: u8 = 4;
pub const JAIL_POSITION: u8 = 10;
pub const LOTTERY_POSITION: u8 = 20;
pub const COST_FOR_UPGRADE: u32 = 10;
pub const FINE: u32 = 10;
pub const PENALTY: u8 = 5;
pub const INITIAL_BALANCE: u32 = 15_000;
pub const NEW_CIRCLE: u32 = 2_000;
pub const WAIT_DURATION: u32 = 5;
pub mod strategic_actions;
pub mod utils;
use monopoly_io::*;
use utils::*;

#[derive(Clone, Default, Encode, Decode, TypeInfo)]
pub struct Game {
    admin: ActorId,
    properties_in_bank: BTreeSet<u8>,
    round: u128,
    players: BTreeMap<ActorId, PlayerInfo>,
    players_queue: Vec<ActorId>,
    current_turn: u8,
    // mapping from cells to built properties,
    properties: BTreeMap<u8, (Vec<Gear>, u32, u32)>,
    // mapping from cells to accounts who have properties on it
    ownership: BTreeMap<u8, ActorId>,
    game_status: GameStatus,
    number_of_players: u8,
    winner: ActorId,
}

static mut GAME: Option<Game> = None;

impl Game {
    fn register(&mut self) {
        assert_eq!(
            self.game_status,
            GameStatus::Registration,
            "Game must be in registration status"
        );
        assert!(
            !self.players.contains_key(&msg::source()),
            "You have already registered"
        );
        self.players.insert(
            msg::source(),
            PlayerInfo {
                balance: INITIAL_BALANCE,
                ..Default::default()
            },
        );
        self.players_queue.push(msg::source());
        self.number_of_players += 1;
        if self.number_of_players == NUMBER_OF_PLAYERS {
            self.game_status = GameStatus::Play;
        }
        msg::reply(GameEvent::Registered, 0)
            .expect("Error in sending a reply `GameEvent::Registered`");
    }

    async fn play(&mut self) {
        assert_eq!(
            self.game_status,
            GameStatus::Play,
            "GameStatus must be `Play`"
        );
        assert_eq!(msg::source(), self.admin, "Only admin can start the game");

        let mut number_of_players = NUMBER_OF_PLAYERS;
        while self.game_status == GameStatus::Play {
            self.round = self.round.wrapping_add(1);
            if number_of_players == 1 {
                self.winner = self.players_queue[0];
                self.game_status = GameStatus::Finished;
                msg::reply(
                    GameEvent::GameFinished {
                        winner: self.winner,
                    },
                    0,
                )
                .expect("Error in sending a reply `GameEvent::GameFinished`");
                break;
            }
            for _i in 0..number_of_players {
                let state = self.clone();
                let player = self.players_queue[self.current_turn as usize];
                let player_info = self
                    .players
                    .get_mut(&player)
                    .expect("Cant be None: Get Player");
                if player_info.in_jail {
                    let reply = msg::send_for_reply(
                        player,
                        StrategicAction::YourTurn {
                            players: state.players.clone(),
                            properties: state.properties.clone(),
                        },
                        0,
                    )
                    .expect("Error on sending `StrategicAction::YourTurn` message")
                    .up_to(Some(WAIT_DURATION))
                    .expect("Invalid wait duration.")
                    .await;

                    if reply.is_err() {
                        // if the message to a player was invalid we have to restore the state
                        *self = state;
                        self.players.remove(&player);
                        self.players_queue.retain(|&p| p != player);
                        number_of_players -= 1;
                        if self.number_of_players == 1 {
                            self.winner = self.players_queue[0];
                            self.game_status = GameStatus::Finished;
                            msg::reply(
                                GameEvent::GameFinished {
                                    winner: self.winner,
                                },
                                0,
                            )
                            .expect("Error in sending a reply `GameEvent::GameFinished`");
                        }
                    } else {
                        player_info.round = self.round;
                        self.current_turn = (self.current_turn + 1) % self.number_of_players;
                    }
                } else {
                    let (r1, r2) = get_rolls();
                    let roll_sum = r1 + r2;
                    let position = (player_info.position + roll_sum) % NUMBER_OF_CELLS;
                    player_info.position = position;
                    player_info.in_jail = position == JAIL_POSITION;
                    match position {
                        0 => {
                            player_info.balance += NEW_CIRCLE;
                            player_info.round = self.round;
                            self.current_turn = (self.current_turn + 1) % self.number_of_players;
                        }
                        LOTTERY_POSITION => {
                            let reward = lottery();
                            player_info.balance += reward;
                            player_info.round = self.round;
                            self.current_turn = (self.current_turn + 1) % self.number_of_players;
                        }
                        _ => {
                            let reply = msg::send_for_reply(
                                player,
                                StrategicAction::YourTurn {
                                    players: state.players.clone(),
                                    properties: state.properties.clone(),
                                },
                                0,
                            )
                            .expect("Error on sending `StrategicAction::YourTurn` message")
                            .up_to(Some(WAIT_DURATION))
                            .expect("Invalid wait duration.")
                            .await;

                            if reply.is_err() {
                                // if the message to a player was invalid we have to restore the state
                                *self = state;
                                self.players.remove(&player);
                                self.number_of_players -= 1;
                                self.players_queue.retain(|&p| p != player);
                                if self.number_of_players == 1 {
                                    self.winner = self.players_queue[0];
                                    self.game_status = GameStatus::Finished;
                                    msg::reply(
                                        GameEvent::GameFinished {
                                            winner: self.winner,
                                        },
                                        0,
                                    )
                                    .expect("Error in sending a reply `GameEvent::GameFinished`");
                                    break;
                                }
                            } else {
                                player_info.round = self.round;
                                self.current_turn =
                                    (self.current_turn + 1) % self.number_of_players;
                            }
                        }
                    }
                }
                msg::send(
                    self.admin,
                    GameEvent::Step {
                        players: self.players.clone(),
                        properties: self.properties.clone(),
                    },
                    0,
                )
                .expect("Error in sending a message `GameEvent::Step`");
            }
        }
    }
}

#[gstd::async_main]
async fn main() {
    let action: GameAction = msg::load().expect("Could not load `GameAction`");
    let game: &mut Game = unsafe { GAME.get_or_insert(Default::default()) };
    match action {
        GameAction::Register => game.register(),
        GameAction::Play => game.play().await,
        GameAction::ThrowRoll {
            pay_fine,
            properties_for_sale,
        } => game.throw_roll(pay_fine, properties_for_sale),
        GameAction::Upgrade {
            properties_for_sale,
        } => game.upgrade(properties_for_sale),
        GameAction::BuyCell {
            properties_for_sale,
        } => game.buy_cell(properties_for_sale),
        GameAction::PayRent {
            properties_for_sale,
        } => game.pay_rent(properties_for_sale),
    }
}

#[no_mangle]
extern "C" fn meta_state() -> *mut [i32; 2] {
    let game: &mut Game = unsafe { GAME.get_or_insert(Default::default()) };
    let encoded = game.encode();
    gstd::util::to_leak_ptr(encoded)
}

#[no_mangle]
unsafe extern "C" fn init() {
    let game = Game {
        admin: msg::source(),
        ..Default::default()
    };
    GAME = Some(game);
}

gstd::metadata! {
title: "MonopolyGame",
    handle:
        input: GameAction,
        output: GameEvent,
   state:
       output: Game,
}
