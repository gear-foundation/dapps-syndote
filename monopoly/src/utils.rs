use crate::*;

impl Game {
    pub fn only_player(&self) {
        assert!(
            self.players.contains_key(&msg::source()),
            "You are not in the game"
        );
    }
    // pub fn check_bankrupt(&self, player: &ActorId) {

    // }
}

pub fn get_player_info<'a>(
    player: &'a ActorId,
    players: &'a mut BTreeMap<ActorId, PlayerInfo>,
    current_round: u128,
) -> Result<&'a mut PlayerInfo, ()> {
    if &msg::source() != player {
        players.entry(msg::source()).and_modify(|player_info| {
            player_info.penalty += 1;
        });
        return Err(());
    }
    let player_info = players.get_mut(player).expect("Cant be None: Get Player");
    if player_info.round >= current_round {
        player_info.penalty += 1;
        return Err(());
    }
    Err(())
}

pub fn sell_property<'a>(
    ownership: &'a mut BTreeMap<u8, ActorId>,
    properties_for_sale: &'a Vec<u8>,
    properties_in_bank: &'a mut BTreeSet<u8>,
    properties: &'a BTreeMap<u8, (Vec<Gear>, u32, u32)>,
    player_info: &'a mut PlayerInfo,
) -> Result<(), ()> {
    for property in properties_for_sale {
        if let Some(owner) = ownership.get(property) {
            if owner != &msg::source() {
                player_info.penalty += 1;
                return Err(());
            }
        } else {
            player_info.penalty += 1;
            return Err(());
        };
    }

    for property in properties_for_sale {
        let (_, price, _) = properties.get(property).expect("Properies: Cant be None");
        player_info.cells.remove(property);
        player_info.balance += price / 2;
        ownership.remove(property);
        properties_in_bank.insert(*property);
    }
    Ok(())
}
pub fn lottery() -> u32 {
    0
}

pub fn get_rolls() -> (u8, u8) {
    let random = exec::random(&exec::block_timestamp().to_be_bytes()).expect("");
    let r1: u8 = random.0[0] % 6 + 1;
    let r2: u8 = random.0[1] % 6 + 1;
    (r1, r2)
}
