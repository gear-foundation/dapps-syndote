use crate::*;

impl Game {
    // to throw rolls to go out from the prison
    // `pay_fine`: to pay fine or not if there is not double
    pub fn throw_roll(&mut self, pay_fine: bool, properties_for_sale: Option<Vec<u8>>) {
        self.only_player();
        let player = self.players_queue[self.current_turn as usize];
        let player_info = match get_player_info(&player, &mut self.players, self.round) {
            Ok(player_info) => player_info,
            Err(_) => {
                reply_strategic_error();
                return;
            }
        };

        // If a player is not in the jail
        if !player_info.in_jail {
            player_info.penalty += 1;
            reply_strategic_error();
            return;
        }

        if let Some(properties) = properties_for_sale {
            if sell_property(
                &mut self.ownership,
                &properties,
                &mut self.properties_in_bank,
                &self.properties,
                player_info,
            )
            .is_err()
            {
                reply_strategic_error();
                return;
            };
        }

        let (r1, r2) = get_rolls();
        if r1 == r2 {
            player_info.in_jail = false;
        } else if pay_fine {
            if player_info.balance < FINE {
                player_info.penalty += 1;
                reply_strategic_error();
                return;
            }
            player_info.balance -= FINE;
            player_info.in_jail = false;
        }
        reply_strategic_success();
    }

    // if a player is on his position
    // then he can upgrade his cell (put a gear on it a upgrade a gear)
    pub fn upgrade(&mut self, properties_for_sale: Option<Vec<u8>>) {
        self.only_player();
        let player = self.players_queue[self.current_turn as usize];
        let player_info = match get_player_info(&player, &mut self.players, self.round) {
            Ok(player_info) => player_info,
            Err(_) => {
                reply_strategic_error();
                return;
            }
        };

        if let Some(properties) = properties_for_sale {
            if sell_property(
                &mut self.ownership,
                &properties,
                &mut self.properties_in_bank,
                &self.properties,
                player_info,
            )
            .is_err()
            {
                reply_strategic_error();
                return;
            };
        }

        // if player did not check his balance itself
        if player_info.balance < COST_FOR_UPGRADE {
            player_info.penalty += 1;
            reply_strategic_error();
            return;
        }

        let position = player_info.position;

        // if a player tries to upgrade not on his cell
        if !player_info.cells.contains(&position) {
            player_info.penalty += 1;
            reply_strategic_error();
            return;
        }

        let (gears, price, rent) = self
            .properties
            .get_mut(&position)
            .expect("Properties: Can't be None");

        // if nothing to upgrade
        if gears.is_empty() {
            player_info.penalty += 1;
            reply_strategic_error();
            return;
        }

        for gear in gears {
            if *gear != Gear::Gold {
                *gear = gear.upgrade();
                // add 10 percent to the price of cell
                *price += *price / 10;
                // add 10 percent to the price of rent
                *rent += *rent / 10;
                break;
            }
        }
        player_info.balance -= COST_FOR_UPGRADE;

        reply_strategic_success();
    }

    // if a cell is free, a player can buy it
    pub fn buy_cell(&mut self, properties_for_sale: Option<Vec<u8>>) {
        self.only_player();
        let player = self.players_queue[self.current_turn as usize];
        let player_info = match get_player_info(&player, &mut self.players, self.round) {
            Ok(player_info) => player_info,
            Err(_) => {
                reply_strategic_error();
                return;
            }
        };
        let position = player_info.position;

        // if a player tries to buy position that has already been bought
        if self.ownership.contains_key(&position) {
            player_info.penalty += 1;
            reply_strategic_error();
            return;
        }

        if let Some(properties) = properties_for_sale {
            if sell_property(
                &mut self.ownership,
                &properties,
                &mut self.properties_in_bank,
                &self.properties,
                player_info,
            )
            .is_err()
            {
                reply_strategic_error();
                return;
            };
        }

        // if a player on the field that can't be sold (for example, jail)
        let price = if let Some((_, price, _)) = self.properties.get(&position) {
            price
        } else {
            player_info.penalty += 1;
            reply_strategic_error();
            return;
        };

        // if a player has not enough balance
        if player_info.balance < *price {
            player_info.penalty += 1;
            reply_strategic_error();
            return;
        }

        player_info.balance -= price;
        player_info.cells.insert(position);
        self.ownership.insert(position, msg::source());

        reply_strategic_success();
    }

    pub fn pay_rent(&mut self, properties_for_sale: Option<Vec<u8>>) {
        self.only_player();
        let player = self.players_queue[self.current_turn as usize];
        let player_info = match get_player_info(&player, &mut self.players, self.round) {
            Ok(player_info) => player_info,
            Err(_) => {
                reply_strategic_error();
                return;
            }
        };
        if let Some(properties) = properties_for_sale {
            if sell_property(
                &mut self.ownership,
                &properties,
                &mut self.properties_in_bank,
                &self.properties,
                player_info,
            )
            .is_err()
            {
                reply_strategic_error();
                return;
            };
        }

        let position = player_info.position;
        if let Some(account) = self.ownership.get(&position) {
            if account == &msg::source() {
                player_info.penalty += 1;
                reply_strategic_error();
                return;
            }
            let rent = if let Some((_, _, rent)) = self.properties.get(&position) {
                rent
            } else {
                player_info.penalty += 1;
                reply_strategic_error();
                return;
            };
            assert!(player_info.balance >= *rent, "Not enough balance");
            player_info.balance -= rent;
            self.players.entry(*account).and_modify(|player_info| {
                player_info.balance = player_info.balance.saturating_add(*rent)
            });
            reply_strategic_success();
        } else {
            player_info.penalty += 1;
            reply_strategic_error();
        }
    }
}

fn reply_strategic_error() {
    msg::reply(GameEvent::StrategicError, 0).expect("Error in a reply `GameEvent::StrategicError`");
}

fn reply_strategic_success() {
    msg::reply(GameEvent::StrategicSuccess, 0)
        .expect("Error in a reply `GameEvent::StrategicSuccess`");
}
