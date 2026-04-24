use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
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
        _ => None, // This covers 'Empty' and 'Wall'
    }
}

    fn evaluate_three_cells(cells:[Option<Player>; 3]) -> i32 {
        let mut x_count = 0;
    let mut o_count = 0;

    for cell in cells {
        if cell == Some(Player::X) { x_count += 1; }
        else if cell == Some(Player::O) { o_count += 1; }
    }

    // SCORING LOGIC
    if x_count == 3 { return 100; }  // X gets a point!
    if o_count == 3 { return -100; } // O gets a point!

    // POTENTIAL (Important for beating the basic agent)
    // If it's "X X _" it's worth points because X can finish it.
    if x_count == 2 && o_count == 0 { return 15; }
    if o_count == 2 && x_count == 0 { return -20; }

    if x_count == 1 && o_count == 0 { return 2;}
    if o_count == 1 && x_count == 0 { return -2;}

    0
    }

    fn heuristic(board: &Board) -> i32 {
       let cells = board.get_cells();
       let n = cells.len();
       let mut score = board.score();
       let center = n as i32 / 2;
       for i in 0..n {
        for j in 0..n {
            let distance = (i as i32 - center).abs() + (j as i32 - center).abs();
            let positive_value = 4 - distance;

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

    fn minmax(board: &mut Board, player: Player,depth: usize, max_depth: usize, mut alpha: i32, mut beta: i32) -> (i32, usize, usize) {
        if board.game_over() {
            return (board.score(), 0, 0);
        }

        if depth == max_depth {
            return (SolutionAgent::heuristic(board), 0, 0);
        }

        let mut moves = board.moves();
        moves.sort_by_key(|&m| {
            board.apply_move(m, player);
            let score = Self::heuristic(board);
            board.undo_move(m, player);

            match player {
                Player::X => -score,
                Player::O => score,
            }
        });
        let mut best_move = moves[0]; 
        let mut best_score = match player {
            Player::X => i32::MIN,
            Player::O => i32::MAX,
        };
        let next_player = player.flip();
        for m in moves {
            board.apply_move(m, player);
            let (score, _, _) = SolutionAgent::minmax(board, next_player, depth + 1, max_depth, alpha, beta);
            board.undo_move(m, player);
            // Add recursive call here
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

    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        let move_count = board.moves().len();
        let depth_limit = if move_count < 8 {
            move_count
        } else if move_count <= 12{
            6
        } else if move_count <= 18{
            5
        } else {
            4
        };
        SolutionAgent::minmax(board, player, 0, depth_limit, i32::MIN, i32::MAX)
    }
}
