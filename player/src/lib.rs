#![no_std]
use gstd::{exec,debug, msg, prelude::*, ActorId};
use monopoly_io::*;
// static mut MONOPOLY: ActorId = ActorId::zero();
pub const COST_FOR_UPGRADE: u32 = 500;
pub const FINE: u32 = 1_000;

#[gstd::async_main]
async fn main() {
    //let monopoly_id = unsafe { MONOPOLY };
    let monopoly_id = msg::source();
    // assert_eq!(
    //     msg::source(),
    //     monopoly_id,
    //     "Only monopoly contract can call strategic contract"
    // );
    let mut message: YourTurn = msg::load().expect("Unable to decode struct`YourTurn`");
    let my_player = message
        .players
        .get_mut(&exec::program_id())
        .expect("Players: Cant be `None`");
    if my_player.in_jail {
        if my_player.balance <= FINE {
            let reply: GameEvent = msg::send_for_reply_as(
                monopoly_id,
                GameAction::ThrowRoll {
                    pay_fine: false,
                    properties_for_sale: None,
                },
                0,
            )
            .expect("Error in sending a message `GameAction::ThrowRoll`")
            .await
            .expect("Unable to decode `GameEvent");

            match reply {
                GameEvent::Jail { in_jail, position } => {
                    if !in_jail {
                        my_player.position = position;
                    } else {

                        msg::reply("", 0).expect("Error in sending a reply to monopoly contract");
                        return;
                    }
                }
                _ => {}
            }
        } else {
            msg::send_for_reply_as::<_, GameEvent>(
                monopoly_id,
                GameAction::ThrowRoll {
                    pay_fine: true,
                    properties_for_sale: None,
                },
                0,
            )
            .expect("Error in sending a message `GameAction::ThrowRoll`")
            .await
            .expect("Unable to decode `GameEvent");



            msg::reply("", 0).expect("Error in sending a reply to monopoly contract");
            return;
        }
    }

    let position = my_player.position;
    let my_cell = my_player.cells.contains(&position);
    if my_cell {
     //   debug!("ADD GEAR");
        let (gears, _, _) = &mut message.properties[position as usize];
        if gears.len() < 3 {
            msg::send_for_reply_as::<_, GameEvent>(
                monopoly_id,
                GameAction::AddGear {
                    properties_for_sale: None,
                },
                0,
            )
            .expect("Error in sending a message `GameAction::AddGear`")
            .await
            .expect("Unable to decode `GameEvent");
            msg::reply("", 0).expect("Error in sending a reply to monopoly contract");
            return;
        } else {
          //  debug!("UPGRADE");
            msg::send_for_reply_as::<_, GameEvent>(
                monopoly_id,
                GameAction::Upgrade {
                    properties_for_sale: None,
                },
                0,
            )
            .expect("Error in sending a message `GameAction::Upgrade`")
            .await
            .expect("Unable to decode `GameEvent");
            msg::reply("", 0).expect("Error in sending a reply to monopoly contract");
            return;
        }
    }
    let free_cell = message.ownership[position as usize] == ActorId::zero();
    if free_cell {
        debug!("BUY CELL");
        msg::send_for_reply_as::<_, GameEvent>(
            monopoly_id,
            GameAction::BuyCell {
                properties_for_sale: None,
            },
            0,
        )
        .expect("Error in sending a message `GameAction::BuyCell`")
        .await
        .expect("Unable to decode `GameEvent");
    } else {
        msg::send_for_reply_as::<_, GameEvent>(
            monopoly_id,
            GameAction::PayRent {
                properties_for_sale: None,
            },
            0,
        )
        .expect("Error in sending a message `GameAction::PayRent`")
        .await
        .expect("Unable to decode `GameEvent");
    }
    msg::reply("", 0).expect("Error in sending a reply to monopoly contract");
}

#[no_mangle]
unsafe extern "C" fn init() {
    //   MONOPOLY = msg::load::<ActorId>().expect("Unable to decode ActorId");
}

gstd::metadata! {
title: "Player",
 //   init:
   //     input: ActorId,
}
