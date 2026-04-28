use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;
use std::time::{Duration, Instant};

// Your solution solution.
pub struct SolutionAgent {}

// Put your solution here.
impl SolutionAgent {

    fn cell_to_player(cell: &tic_tac_toe_stencil::board::Cell) -> Option<Player> {
    use tic_tac_toe_stencil::board::Cell;
    match cell {
        Cell::X => Some(Player::X),
        Cell::O => Some(Player::O),
        _ => None, // This covers 'Empty' and 'Wall'
    }
}
    fn immediate_move_value(board:&mut Board, m: (usize, usize), player: Player) -> i32 {
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

        return own_value + opponent_value
    }

    fn evaluate_three_cells(cells:[Option<Player>; 3]) -> i32 {
        let mut x_count = 0;
        let mut o_count = 0;
        let mut empty_count = 0;

    for cell in cells {
        if cell == Some(Player::X) { x_count += 1; }
        else if cell == Some(Player::O) { o_count += 1; }
        else { empty_count += 1; }
    }

    // SCORING LOGIC
    if x_count == 3 { return 0; }  // X gets a point!
    if o_count == 3 { return 0; } // O gets a point!

    if x_count > 0 && o_count > 0 { return 0; } 

    if x_count == 2 && empty_count == 1 { return 25; }
    if o_count == 2 && empty_count == 1 { return -30; }
    if x_count == 1 && empty_count == 2 { return 5; }
    if o_count == 1 && empty_count == 2 { return -5; }

    0
    }

    fn heuristic(board: &Board) -> i32 {
       let cells = board.get_cells();
       let n = cells.len();
       let mut score = board.score()*100;
       let center = n as i32 / 2;
       for i in 0..n {
        for j in 0..n {
            let distance = (i as i32 - center).abs() + (j as i32 - center).abs();
            let positive_value = (4 - distance).max(0);

            match Self::cell_to_player(&cells[i][j]) {
                Some(Player::X) => score += positive_value,
                Some(Player::O) => score -= positive_value,
                _ => {}
            }
        }
       }
       //Horizontal
       for i in 0..n {
        for j in 0..n-2 {
            score += Self::evaluate_three_cells([
            Self::cell_to_player(&cells[i][j]),
            Self::cell_to_player(&cells[i][j+1]), 
            Self::cell_to_player(&cells[i][j+2])
        ]);
            }
        }
         //Vertical
         for i in 0..n-2 {
            for j in 0..n {
                score += Self::evaluate_three_cells([
            Self::cell_to_player(&cells[i][j]), 
            Self::cell_to_player(&cells[i+1][j]), 
            Self::cell_to_player(&cells[i+2][j])
          ]);
            }
        }
        //Diagonal down-right
        for i in 0..n-2 {
            for j in 0..n-2 {
                score += Self::evaluate_three_cells([
            Self::cell_to_player(&cells[i][j]), 
            Self::cell_to_player(&cells[i+1][j+1]), 
            Self::cell_to_player(&cells[i+2][j+2])
                ]);
            }
        }
        //Diagonal down-left
        for i in 0..n-2 {
            for j in 2..n {
                score += Self::evaluate_three_cells([
            Self::cell_to_player(&cells[i][j]), 
            Self::cell_to_player(&cells[i+1][j-1]), 
            Self::cell_to_player(&cells[i+2][j-2])
              ]);
            }
        }
        
        return score;
    }

    fn minmax(board: &mut Board, player: Player,depth: usize, max_depth: usize, mut alpha: i32, mut beta: i32, deadline: Instant, timed_out:&mut bool) -> (i32, usize, usize) {
        if Instant::now() >= deadline {
            *timed_out = true;
            return (Self::heuristic(board), 0, 0); 
        }
        if board.game_over() {
            return (board.score()*100, 0, 0);
        }

        if depth == max_depth {
            return (SolutionAgent::heuristic(board), 0, 0);
        }

        let mut moves = board.moves();
        if depth <= 1{
            moves.sort_by_key(|&m| {
                -Self::move_priority(board, m, player)
            });
        }
        let mut best_move = moves[0]; 
        let mut best_score = match player {
            Player::X => i32::MIN,
            Player::O => i32::MAX,
        };
        let next_player = player.flip();
        for m in moves {
            board.apply_move(m, player);
            let (score, _, _) = SolutionAgent::minmax(board, next_player, depth + 1, max_depth, alpha, beta, deadline, timed_out);
            board.undo_move(m, player);
            if *timed_out {
                break;
            }
            match player {
                Player::X => {
                    if score > best_score {
                        best_score = score;
                        best_move = m;
                    }
                    alpha = alpha.max(best_score);
                    if beta <= alpha {
                        break;
                    }

                }
                Player::O => {
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
            let result = SolutionAgent::minmax(board, player, 0, depth, i32::MIN, i32::MAX, deadline, &mut timed_out);
            if timed_out {
                break;
            }
            best_result = result;
        }
        best_result
    }
}
