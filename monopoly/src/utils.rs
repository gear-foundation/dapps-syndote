use crate::*;

impl Game {
    pub fn check_status(&self, game_status: GameStatus) {
        assert_eq!(self.game_status, game_status, "Wrong game status");
    }

    pub fn only_admin(&self) {
        assert_eq!(msg::source(), self.admin, "Only admin can start the game");
    }
    pub fn only_player(&self) {
        assert!(
            self.players.contains_key(&msg::source()),
            "You are not in the game"
        );
    }
}

pub fn get_player_info<'a>(
    player: &'a ActorId,
    players: &'a mut BTreeMap<ActorId, PlayerInfo>,
    current_round: u128,
) -> Result<&'a mut PlayerInfo, ()> {
    if &msg::source() != player {
        debug!("PENALTY: WRONG MSG::SOURCE()");
        players.entry(msg::source()).and_modify(|player_info| {
            player_info.penalty += 1;
        });
        return Err(());
    }
    let player_info = players.get_mut(player).expect("Cant be None: Get Player");
    if player_info.round >= current_round {
        debug!("PENALTY: MOVE ALREADY MADE");
        player_info.penalty += 1;
        return Err(());
    }
    Ok(player_info)
}

pub fn sell_property(
    ownership: &mut BTreeMap<u8, ActorId>,
    properties_for_sale: &Vec<u8>,
    properties_in_bank: &mut BTreeSet<u8>,
    properties: &BTreeMap<u8, (Vec<Gear>, u32, u32)>,
    player_info: &mut PlayerInfo,
) -> Result<(), ()> {
    for property in properties_for_sale {
        if let Some(owner) = ownership.get(property) {
            if owner != &msg::source() {
                debug!("PENALTY: TRY TO SELL NOT OWN PROPERTY");
                player_info.penalty += 1;
                return Err(());
            }
        } else {
            debug!("PENALTY: TRY TO SELL FREE PROPERTY");
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

static mut SEED: u64 = 0;
pub fn get_rolls() -> (u8, u8) {
    let seed = unsafe {
        SEED = SEED + 1;
        SEED
    };
    let random = exec::random(&(exec::block_timestamp() + seed).to_be_bytes()).expect("");
    let r1: u8 = random.0[0] % 6 + 1;
    let r2: u8 = random.0[1] % 6 + 1;
    (r1, r2)
}

pub fn bankrupt_and_penalty(
    admin: &ActorId,
    players: &mut BTreeMap<ActorId, PlayerInfo>,
    players_queue: &mut Vec<ActorId>,
    number_of_players: &mut u8,
    properties: &mut BTreeMap<u8, (Vec<Gear>, u32, u32)>,
    properties_in_bank: &mut BTreeSet<u8>,
    ownership: &mut BTreeMap<u8, ActorId>,
) {
    for (player, mut player_info) in players.clone() {
        if player_info.debt > 0 {
            for cell in &player_info.cells.clone() {
                if player_info.balance >= player_info.debt {
                    player_info.balance -= player_info.debt;
                    player_info.debt = 0;
                    player_info.penalty += 1;
                    players.insert(player, player_info);
                    break;
                }
                let (_, price, _) = properties.get(&cell).expect("Properties: Cant be `None`");
                player_info.balance += price / 2;
                player_info.cells.remove(cell);
                ownership.insert(*cell, *admin);
                properties_in_bank.insert(*cell);
            }
        }
    }

    for (player, player_info) in players.clone() {
        if player_info.penalty >= PENALTY || player_info.debt > 0 {
            players.remove(&player);
            players_queue.retain(|&p| p != player);
            *number_of_players -= 1;
            for cell in &player_info.cells.clone() {
                ownership.insert(*cell, *admin);
                properties_in_bank.insert(*cell);
            }
        }
    }
}

pub fn init_properties(properties: &mut BTreeMap<u8, (Vec<Gear>, u32, u32)>) {
    properties.insert(1, (Vec::new(), 1_000, 100));
    properties.insert(2, (Vec::new(), 1_050, 105));
    properties.insert(3, (Vec::new(), 1_050, 105));
    properties.insert(4, (Vec::new(), 1_050, 105));
    properties.insert(5, (Vec::new(), 1_050, 105));
    properties.insert(6, (Vec::new(), 1_050, 105));
    properties.insert(7, (Vec::new(), 1_050, 105));
    properties.insert(8, (Vec::new(), 1_050, 105));
    properties.insert(9, (Vec::new(), 1_050, 105));

    properties.insert(11, (Vec::new(), 1_000, 100));
    properties.insert(12, (Vec::new(), 1_050, 105));
    properties.insert(13, (Vec::new(), 1_050, 105));
    properties.insert(14, (Vec::new(), 1_050, 105));
    properties.insert(15, (Vec::new(), 1_050, 105));
    properties.insert(16, (Vec::new(), 1_050, 105));
    properties.insert(17, (Vec::new(), 1_050, 105));
    properties.insert(18, (Vec::new(), 1_050, 105));
    properties.insert(19, (Vec::new(), 1_050, 105));

    properties.insert(21, (Vec::new(), 1_000, 100));
    properties.insert(22, (Vec::new(), 1_050, 105));
    properties.insert(23, (Vec::new(), 1_050, 105));
    properties.insert(24, (Vec::new(), 1_050, 105));
    properties.insert(25, (Vec::new(), 1_050, 105));
    properties.insert(26, (Vec::new(), 1_050, 105));
    properties.insert(27, (Vec::new(), 1_050, 105));
    properties.insert(28, (Vec::new(), 1_050, 105));
    properties.insert(29, (Vec::new(), 1_050, 105));

    properties.insert(31, (Vec::new(), 1_000, 100));
    properties.insert(32, (Vec::new(), 1_050, 105));
    properties.insert(33, (Vec::new(), 1_050, 105));
    properties.insert(34, (Vec::new(), 1_050, 105));
    properties.insert(35, (Vec::new(), 1_050, 105));
    properties.insert(36, (Vec::new(), 1_050, 105));
    properties.insert(37, (Vec::new(), 1_050, 105));
    properties.insert(38, (Vec::new(), 1_050, 105));
    properties.insert(39, (Vec::new(), 1_050, 105));
}
