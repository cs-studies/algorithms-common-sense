#[derive(Debug, PartialEq, Copy, Clone)]
enum Player {
    You,
    Them,
}

impl Player {
    fn toggle(self) -> Self {
        match self {
            Player::You => Player::Them,
            Player::Them => Player::You,
        }
    }
}

pub fn is_winning_1(n: u8) -> bool {
    game_winner_1(n.into(), Player::You) == Player::You
}

fn game_winner_1(n: i16, current_player: Player) -> Player {
    if n <= 0 {
        return current_player;
    }

    let next_player = current_player.toggle();

    if game_winner_1(n - 1, next_player) == current_player
        || game_winner_1(n - 2, next_player) == current_player
    {
        current_player
    } else {
        next_player
    }
}

pub fn is_winning_2(n: u8) -> bool {
    game_winner_2(n) == Player::You
}

fn game_winner_2(n: u8) -> Player {
    if n % 3 == 1 {
        Player::Them
    } else {
        Player::You
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do_i_win_1() {
        assert!(is_winning_1(0));
        assert!(!is_winning_1(1));
        assert!(is_winning_1(2));
        assert!(is_winning_1(3));
        assert!(!is_winning_1(4));
        assert!(is_winning_1(5));
        assert!(is_winning_1(6));
    }

    #[test]
    fn test_do_i_win_2() {
        assert!(is_winning_2(0));
        assert!(!is_winning_2(1));
        assert!(is_winning_2(2));
        assert!(is_winning_2(3));
        assert!(!is_winning_2(4));
        assert!(is_winning_2(5));
        assert!(is_winning_2(6));
    }
}
