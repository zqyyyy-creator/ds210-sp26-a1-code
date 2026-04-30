use std::time::{Duration, Instant};
use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::board::Cell;
use tic_tac_toe_stencil::player::Player;

// Your solution solution.
pub struct SolutionAgent {}

// Put your solution here.
impl SolutionAgent {
    fn cell_to_player(cell: &tic_tac_toe_stencil::board::Cell) -> Option<Player> {
        use tic_tac_toe_stencil::board::Cell;
        match cell {
            Cell::X => Some(Player::X),
            Cell::O => Some(Player::O),
            _ => None,
        }
    }
    fn position_potential(board: &Board, row: usize, col: usize, player: Player) -> i32 {
        let cells = board.get_cells();
        let n = cells.len();
        let mut value = 0;
        let opponent = player.flip();

        let directions = [
            (0, 1),  // Horizontal right
            (1, 0),  // Vertical down
            (1, 1),  // Diagonal down-right
            (1, -1), // Diagonal down-left
        ];

        for (dr, dc) in directions {
            for offset in 0..3 {
                let start_row = row as i32 - offset as i32 * dr;
                let start_col = col as i32 - offset as i32 * dc;

                let mut valid = true;
                let mut empty_count = 0;

                for i in 0..3 {
                    let r = start_row + i as i32 * dr;
                    let c = start_col + i as i32 * dc;

                    if r < 0 || r >= n as i32 || c < 0 || c >= n as i32 {
                        valid = false;
                        break;
                    }

                    match &cells[r as usize][c as usize] {
                        Cell::Wall => {
                            valid = false;
                            break;
                        }
                        Cell::Empty => {
                            empty_count += 1;
                        }
                        Cell::X if opponent == Player::X => {
                            valid = false;
                            break;
                        }
                        Cell::O if opponent == Player::O => {
                            valid = false;
                            break;
                        }
                        _ => {}
                    }
                }
                if valid {
                    value += 1 + empty_count;
                }
            }
        }
        value
    }

    fn score_for_player(score: i32, player: Player) -> i32 {
        match player {
            Player::X => score,
            Player::O => -score,
        }
    }

    fn immediate_move_value(board: &mut Board, m: (usize, usize), player: Player) -> i32 {
        let before = Self::heuristic(board);

        board.apply_move(m, player);
        let after = Self::heuristic(board);
        board.undo_move(m, player);

        match player {
            Player::X => after - before,
            Player::O => before - after,
        }
    }

    fn move_priority(board: &mut Board, m: (usize, usize), player: Player) -> i32 {
        let own_value = Self::immediate_move_value(board, m, player);
        let opponent_value = Self::immediate_move_value(board, m, player.flip());
        let potential = Self::position_potential(board, m.0, m.1, player);

        match player {
            Player::X => own_value * 2 + opponent_value + potential * 3,
            Player::O => own_value * 3 + opponent_value * 3 + potential * 2,
        }
    }

    fn evaluate_three_cells(cells: [Option<Player>; 3]) -> i32 {
        let mut x_count = 0;
        let mut o_count = 0;
        let mut empty_count = 0;

        for cell in cells {
            if cell == Some(Player::X) {
                x_count += 1;
            } else if cell == Some(Player::O) {
                o_count += 1;
            } else {
                empty_count += 1;
            }
        }

        if x_count > 0 && o_count > 0 {
            return 0;
        }
        // SCORING LOGIC
        if x_count == 3 {
            return 1000;
        } // X gets a point!
        if o_count == 3 {
            return -1000;
        } // O gets a point!

        if x_count == 2 && empty_count == 1 {
            return 150;
        }
        if o_count == 2 && empty_count == 1 {
            return -150;
        }
        if x_count == 1 && empty_count == 2 {
            return 10;
        }
        if o_count == 1 && empty_count == 2 {
            return -10;
        }

        0
    }

    fn heuristic(board: &Board) -> i32 {
        let cells = board.get_cells();
        let n = cells.len();
        let mut score = board.score() * 1000;

        for i in 0..n {
            for j in 0..n {
                match Self::cell_to_player(&cells[i][j]) {
                    Some(Player::X) => {
                        let potential = Self::position_potential(board, i, j, Player::X);
                        score += potential * 3;
                    }
                    Some(Player::O) => {
                        let potential = Self::position_potential(board, i, j, Player::O);
                        score -= potential * 3;
                    }
                    _ => {}
                }
            }
        }
        //Horizontal
        for i in 0..n {
            for j in 0..n - 2 {
                score += Self::evaluate_three_cells([
                    Self::cell_to_player(&cells[i][j]),
                    Self::cell_to_player(&cells[i][j + 1]),
                    Self::cell_to_player(&cells[i][j + 2]),
                ]);
            }
        }
        //Vertical
        for i in 0..n - 2 {
            for j in 0..n {
                score += Self::evaluate_three_cells([
                    Self::cell_to_player(&cells[i][j]),
                    Self::cell_to_player(&cells[i + 1][j]),
                    Self::cell_to_player(&cells[i + 2][j]),
                ]);
            }
        }
        //Diagonal down-right
        for i in 0..n - 2 {
            for j in 0..n - 2 {
                score += Self::evaluate_three_cells([
                    Self::cell_to_player(&cells[i][j]),
                    Self::cell_to_player(&cells[i + 1][j + 1]),
                    Self::cell_to_player(&cells[i + 2][j + 2]),
                ]);
            }
        }
        //Diagonal down-left
        for i in 0..n - 2 {
            for j in 2..n {
                score += Self::evaluate_three_cells([
                    Self::cell_to_player(&cells[i][j]),
                    Self::cell_to_player(&cells[i + 1][j - 1]),
                    Self::cell_to_player(&cells[i + 2][j - 2]),
                ]);
            }
        }

        return score;
    }

    fn minmax(
        board: &mut Board,
        player: Player,
        perspective: Player,
        depth: usize,
        max_depth: usize,
        mut alpha: i32,
        mut beta: i32,
        deadline: Instant,
        timed_out: &mut bool,
    ) -> (i32, usize, usize) {
        if Instant::now() >= deadline {
            *timed_out = true;
            return (
                Self::score_for_player(Self::heuristic(board), perspective),
                0,
                0,
            );
        }
        if board.game_over() {
            return (
                Self::score_for_player(board.score() * 1000, perspective),
                0,
                0,
            );
        }

        if depth == max_depth {
            return (
                Self::score_for_player(Self::heuristic(board), perspective),
                0,
                0,
            );
        }

        let mut moves = board.moves();
        if depth <= 1 {
            moves.sort_by_key(|&m| -Self::move_priority(board, m, player));
        }
        let mut best_move = moves[0];
        let maximizing = player == perspective;
        let mut best_score = if maximizing { i32::MIN } else { i32::MAX };
        let next_player = player.flip();
        for m in moves {
            board.apply_move(m, player);
            let (score, _, _) = SolutionAgent::minmax(
                board,
                next_player,
                perspective,
                depth + 1,
                max_depth,
                alpha,
                beta,
                deadline,
                timed_out,
            );
            board.undo_move(m, player);
            if *timed_out {
                break;
            }
            if maximizing {
                if score > best_score {
                    best_score = score;
                    best_move = m;
                }
                alpha = alpha.max(best_score);
                if beta <= alpha {
                    break;
                }
            } else {
                if score < best_score {
                    best_score = score;
                    best_move = m;
                }
                beta = beta.min(best_score);
                if beta <= alpha {
                    break;
                }
            }
        }
        // If you want to make a recursive call to this solution, use
        // `SolutionAgent::solve(...)`
        return (best_score, best_move.0, best_move.1);
    }
}

impl Agent for SolutionAgent {
    fn solve(board: &mut Board, player: Player, time_limit: u64) -> (i32, usize, usize) {
        let moves = board.moves();
        let fallback = moves[0];

        let deadline = Instant::now() + Duration::from_millis(time_limit.saturating_sub(500));

        let mut best_result = (0, fallback.0, fallback.1);
        let move_count = moves.len();
        let max_possible_depth = if move_count < 8 {
            move_count
        } else if move_count <= 12 {
            5
        } else if move_count <= 18 {
            4
        } else {
            3
        };

        for depth in 1..=max_possible_depth {
            let mut timed_out = false;
            let result = SolutionAgent::minmax(
                board,
                player,
                player,
                0,
                depth,
                i32::MIN,
                i32::MAX,
                deadline,
                &mut timed_out,
            );
            if timed_out {
                break;
            }
            best_result = result;
        }
        best_result
    }
}
