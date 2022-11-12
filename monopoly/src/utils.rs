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
//        debug!("PENALTY: WRONG MSG::SOURCE()");
        players.entry(msg::source()).and_modify(|player_info| {
            player_info.penalty += 1;
        });
        return Err(());
    }
    let player_info = players.get_mut(player).expect("Cant be None: Get Player");
    if player_info.round >= current_round {



     //   debug!("PENALTY: MOVE ALREADY MADE");
        player_info.penalty += 1;
        return Err(());
    }
    Ok(player_info)
}

pub fn sell_property(
    admin: &ActorId,
    ownership: &mut Vec<ActorId>,
    properties_for_sale: &Vec<u8>,
    properties_in_bank: &mut BTreeSet<u8>,
    properties: &Vec<(Vec<Gear>, u32, u32)>,
    player_info: &mut PlayerInfo,
) -> Result<(), ()> {
    for property in properties_for_sale {
        if ownership[*property as usize] != msg::source() {
     //       debug!("PENALTY: TRY TO SELL NOT OWN PROPERTY");
            player_info.penalty += 1;
            return Err(());
        } else {
    //        debug!("PENALTY: TRY TO SELL FREE PROPERTY");
            player_info.penalty += 1;
            return Err(());
        };
    }

    for property in properties_for_sale {
        let (_, price, _) = properties[*property as usize];
        player_info.cells.remove(property);
        player_info.balance += price / 2;
        ownership[*property as usize] = *admin;
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
    properties: &mut Vec<(Vec<Gear>, u32, u32)>,
    properties_in_bank: &mut BTreeSet<u8>,
    ownership: &mut Vec<ActorId>,
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
                let (_, price, _) = &properties[*cell as usize];
                player_info.balance += price / 2;
                player_info.cells.remove(cell);
                ownership[*cell as usize] = *admin;
                properties_in_bank.insert(*cell);
            }
        }
    }

    for (player, mut player_info) in players.clone() {
        if player_info.penalty >= PENALTY || player_info.debt > 0 {
            player_info.lost = true;
            players_queue.retain(|&p| p != player);
            *number_of_players -= 1;
            for cell in &player_info.cells.clone() {
                ownership[*cell as usize] = *admin;
                properties_in_bank.insert(*cell);
            }
            players.insert(player, player_info);
        }
    }
}

pub fn init_properties(properties: &mut Vec<(Vec<Gear>, u32, u32)>, ownership: &mut Vec<ActorId>) {
    // 0
    properties.push((Vec::new(), 0, 0));
    // 1
    properties.push((Vec::new(), 1_050, 105));
    // 2
    properties.push((Vec::new(), 1_050, 105));
    // 3
    properties.push((Vec::new(), 1_050, 105));
    // 4
    properties.push((Vec::new(), 1_050, 105));
    // 5
    properties.push((Vec::new(), 1_050, 105));
    // 6
    properties.push((Vec::new(), 1_050, 105));
    // 7
    properties.push((Vec::new(), 1_050, 105));
    // 8
    properties.push((Vec::new(), 1_050, 105));
    // 9
    properties.push((Vec::new(), 1_050, 105));

    // 10
    properties.push((Vec::new(), 0, 0));
    // 11
    properties.push((Vec::new(), 1_050, 105));
    // 12
    properties.push((Vec::new(), 1_050, 105));
    // 13
    properties.push((Vec::new(), 1_050, 105));
    // 14
    properties.push((Vec::new(), 1_050, 105));
    // 15
    properties.push((Vec::new(), 1_050, 105));
    // 16
    properties.push((Vec::new(), 1_050, 105));
    // 17
    properties.push((Vec::new(), 1_050, 105));
    // 18
    properties.push((Vec::new(), 1_050, 105));
    // 19
    properties.push((Vec::new(), 1_050, 105));

    // 20
    properties.push((Vec::new(), 0, 0));
    // 21
    properties.push((Vec::new(), 1_050, 105));
    // 22
    properties.push((Vec::new(), 1_050, 105));
    // 23
    properties.push((Vec::new(), 1_050, 105));
    // 24
    properties.push((Vec::new(), 1_050, 105));
    // 25
    properties.push((Vec::new(), 1_050, 105));
    // 26
    properties.push((Vec::new(), 1_050, 105));
    // 27
    properties.push((Vec::new(), 1_050, 105));
    // 28
    properties.push((Vec::new(), 1_050, 105));
    // 29
    properties.push((Vec::new(), 1_050, 105));

    // 30
    properties.push((Vec::new(), 0, 0));
    // 31
    properties.push((Vec::new(), 1_050, 105));
    // 32
    properties.push((Vec::new(), 1_050, 105));
    // 33
    properties.push((Vec::new(), 1_050, 105));
    // 34
    properties.push((Vec::new(), 1_050, 105));
    // 35
    properties.push((Vec::new(), 1_050, 105));
    // 36
    properties.push((Vec::new(), 1_050, 105));
    // 37
    properties.push((Vec::new(), 1_050, 105));
    // 38
    properties.push((Vec::new(), 1_050, 105));
    // 39
    properties.push((Vec::new(), 1_050, 105));

    for _i in 0..40 {
        ownership.push(ActorId::zero());
    }
    // properties.insert(1, (Vec::new(), 1_000, 100));
    // properties.insert(2, (Vec::new(), 1_050, 105));
    // properties.insert(3, (Vec::new(), 1_050, 105));
    // properties.insert(4, (Vec::new(), 1_050, 105));
    // properties.insert(5, (Vec::new(), 1_050, 105));
    // properties.insert(6, (Vec::new(), 1_050, 105));
    // properties.insert(7, (Vec::new(), 1_050, 105));
    // properties.insert(8, (Vec::new(), 1_050, 105));
    // properties.insert(9, (Vec::new(), 1_050, 105));

    // properties.insert(11, (Vec::new(), 1_000, 100));
    // properties.insert(12, (Vec::new(), 1_050, 105));
    // properties.insert(13, (Vec::new(), 1_050, 105));
    // properties.insert(14, (Vec::new(), 1_050, 105));
    // properties.insert(15, (Vec::new(), 1_050, 105));
    // properties.insert(16, (Vec::new(), 1_050, 105));
    // properties.insert(17, (Vec::new(), 1_050, 105));
    // properties.insert(18, (Vec::new(), 1_050, 105));
    // properties.insert(19, (Vec::new(), 1_050, 105));

    // properties.insert(21, (Vec::new(), 1_000, 100));
    // properties.insert(22, (Vec::new(), 1_050, 105));
    // properties.insert(23, (Vec::new(), 1_050, 105));
    // properties.insert(24, (Vec::new(), 1_050, 105));
    // properties.insert(25, (Vec::new(), 1_050, 105));
    // properties.insert(26, (Vec::new(), 1_050, 105));
    // properties.insert(27, (Vec::new(), 1_050, 105));
    // properties.insert(28, (Vec::new(), 1_050, 105));
    // properties.insert(29, (Vec::new(), 1_050, 105));

    // properties.insert(31, (Vec::new(), 1_000, 100));
    // properties.insert(32, (Vec::new(), 1_050, 105));
    // properties.insert(33, (Vec::new(), 1_050, 105));
    // properties.insert(34, (Vec::new(), 1_050, 105));
    // properties.insert(35, (Vec::new(), 1_050, 105));
    // properties.insert(36, (Vec::new(), 1_050, 105));
    // properties.insert(37, (Vec::new(), 1_050, 105));
    // properties.insert(38, (Vec::new(), 1_050, 105));
    // properties.insert(39, (Vec::new(), 1_050, 105));
}
