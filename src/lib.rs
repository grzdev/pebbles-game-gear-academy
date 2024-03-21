use gstd::prelude::*;
use pebbles_game_io::{PebblesInit, PebblesAction, GameState, Player, DifficultyLevel};

fn get_random_u32() -> u32 {
    let salt = msg::id();
    let (hash, _num) = exec::random(salt.into()).unwrap_or_else(|_| panic!("get_random_u32(): random call failed"));
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}

#[no_mangle]
extern "C" fn init() {
    let init_data: PebblesInit = msg::load().unwrap_or_else(|_| panic!("Failed to load PebblesInit"));
    assert!(init_data.pebbles_count > 0, "Invalid pebbles count");

    let first_player: Player = if get_random_u32() % 2 == 0 {
        Player::Program
    } else {
        Player::User
    };

    let game_state = GameState {
        pebbles_count: init_data.pebbles_count,
        max_pebbles_per_turn: init_data.max_pebbles_per_turn,
        pebbles_remaining: init_data.pebbles_count,
        difficulty: init_data.difficulty,
        first_player: first_player,
        winner: None,
    };

    msg::reply(game_state, 0).unwrap_or_else(|_| panic!("Failed to reply with game state"));
}

#[no_mangle]
extern "C" fn handle() {
    let action: PebblesAction = msg::load().unwrap_or_else(|_| panic!("Failed to decode PebblesAction"));
    let mut game_state: GameState = get_game_state();

    match action {
        PebblesAction::Turn(count) => {
            let remaining_pebbles = game_state.pebbles_remaining;
            if count > game_state.max_pebbles_per_turn || count <= 0 || count > remaining_pebbles {
                panic!("Invalid turn count!");
            }
            game_state.pebbles_remaining -= count;

            if game_state.pebbles_remaining == 0 {
                game_state.winner = Some(Player::User);
            }
        }
        PebblesAction::GiveUp => {
            game_state.winner = Some(Player::Program);
        }
        PebblesAction::Restart { difficulty, pebbles_count, max_pebbles_per_turn } => {
            game_state = GameState {
                pebbles_count: pebbles_count,
                max_pebbles_per_turn: max_pebbles_per_turn,
                pebbles_remaining: pebbles_count,
                difficulty: difficulty,
                first_player: choose_first_player(),
                winner: None,
            };
        }
    }

    save_game_state(game_state);
    msg::reply((), 0).unwrap_or_else(|_| panic!("Failed to reply"));
}

#[no_mangle]
extern "C" fn state() {
    let game_state: GameState = get_game_state();
    msg::reply(game_state, 0).unwrap_or_else(|_| panic!("Failed to reply with game state"));
}

// You'll need to implement these functions:
fn get_game_state() -> GameState {
    unimplemented!("Function to retrieve game state")
}

fn save_game_state(game_state: GameState) {
    unimplemented!("Function to save game state")
}

fn choose_first_player() -> Player {
    unimplemented!("Function to choose first player")
}
