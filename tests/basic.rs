mod tests {
    use super::*;
    use gstd::prelude::*;
    use gstd::testing::*;
    use gstd::println;
    use pebbles_game_io::*;

    #[test]
    fn test_init_valid() {
        // Test initialization with valid parameters
        let init_data = PebblesInit {
            difficulty: DifficultyLevel::Easy,
            pebbles_count: 15,
            max_pebbles_per_turn: 2,
        };
        msg::set_msg(init_data.clone());
        init();
        let game_state: GameState = msg::read();
        assert_eq!(game_state.pebbles_count, init_data.pebbles_count);
        assert_eq!(game_state.max_pebbles_per_turn, init_data.max_pebbles_per_turn);
        assert_eq!(game_state.pebbles_remaining, init_data.pebbles_count);
        assert!(game_state.first_player == Player::User || game_state.first_player == Player::Program);
        assert!(game_state.winner.is_none());
    }

    #[test]
    fn test_handle_user_turn() {
        // Test User's turn processing
        let mut game_state = GameState {
            pebbles_count: 10,
            max_pebbles_per_turn: 3,
            pebbles_remaining: 10,
            difficulty: DifficultyLevel::Easy,
            first_player: Player::User,
            winner: None,
        };
        let user_turn_count = 2;
        msg::set_msg(PebblesAction::Turn(user_turn_count));
        handle();
        game_state.pebbles_remaining -= user_turn_count;
        assert_eq!(msg::read::<PebblesEvent>(), PebblesEvent::CounterTurn(user_turn_count));
        assert_eq!(get_game_state(), game_state);
    }

    #[test]
    fn test_handle_program_turn() {
        // Test Program's turn processing
        let mut game_state = GameState {
            pebbles_count: 10,
            max_pebbles_per_turn: 3,
            pebbles_remaining: 10,
            difficulty: DifficultyLevel::Easy,
            first_player: Player::Program,
            winner: None,
        };
        // Simulate Program's turn with a random count of pebbles removed
        let program_turn_count = get_random_u32() % game_state.max_pebbles_per_turn + 1;
        msg::set_msg(PebblesAction::Turn(program_turn_count));
        handle();
        game_state.pebbles_remaining -= program_turn_count;
        assert_eq!(msg::read::<PebblesEvent>(), PebblesEvent::CounterTurn(program_turn_count));
        assert_eq!(get_game_state(), game_state);
    }

    #[test]
    fn test_handle_restart() {
        // Test game restart
        let init_data = PebblesInit {
            difficulty: DifficultyLevel::Easy,
            pebbles_count: 10,
            max_pebbles_per_turn: 3,
        };
        let mut game_state = GameState {
            pebbles_count: 15,
            max_pebbles_per_turn: 2,
            pebbles_remaining: 15,
            difficulty: DifficultyLevel::Easy,
            first_player: Player::Program,
            winner: Some(Player::User),
        };
        msg::set_msg(PebblesAction::Restart {
            difficulty: init_data.difficulty,
            pebbles_count: init_data.pebbles_count,
            max_pebbles_per_turn: init_data.max_pebbles_per_turn,
        });
        handle();
        game_state.pebbles_count = init_data.pebbles_count;
        game_state.max_pebbles_per_turn = init_data.max_pebbles_per_turn;
        game_state.pebbles_remaining = init_data.pebbles_count;
        game_state.difficulty = init_data.difficulty;
        game_state.first_player = if get_random_u32() % 2 == 0 {
            Player::Program
        } else {
            Player::User
        };
        game_state.winner = None;
        assert_eq!(msg::read::<PebblesEvent>(), PebblesEvent::Restart);
        assert_eq!(get_game_state(), game_state);
    }

    #[test]
    fn test_handle_give_up() {
        // Test User's give up action
        let mut game_state = GameState {
            pebbles_count: 10,
            max_pebbles_per_turn: 3,
            pebbles_remaining: 10,
            difficulty: DifficultyLevel::Easy,
            first_player: Player::User,
            winner: None,
        };
        msg::set_msg(PebblesAction::GiveUp);
        handle();
        game_state.winner = Some(Player::Program);
        assert_eq!(msg::read::<PebblesEvent>(), PebblesEvent::Won(Player::Program));
        assert_eq!(get_game_state(), game_state);
    }
}
