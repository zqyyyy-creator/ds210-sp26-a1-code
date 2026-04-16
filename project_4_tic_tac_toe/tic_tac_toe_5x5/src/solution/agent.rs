use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

// Your solution solution.
pub struct SolutionAgent {}

// Put your solution here.
impl SolutionAgent {
    fn heuristic(board: &Board) -> i32 {
       let cells = board.get_cells();
       let n = cells.len();
       let mut score = board.score();
       //Horizontal
       for i in 0..n {
        for j in 0..n-2 {
            //evaluation
            }
        }
         //Vertical
         for i in 0..n-2 {
            for j in 0..n {
                //evaluation
            }
        }
        //Diagonal down-right
        for i in 0..n-2 {
            for j in 0..n-2 {
                //evaluation
            }
        }
        //Diagonal down-left
        for i in 0..n-2 {
            for j in 2..n {
                //evaluation
            }
        }
        return score;
    }

    fn minmax(board: &mut Board, player: Player,depth: usize, max_depth: usize) -> (i32, usize, usize) {
        if board.game_over() {
            return (board.score(), 0, 0);
        }

        if depth == max_depth {
            return (SolutionAgent::heuristic(board), 0, 0);
        }

        let moves = board.moves();
        let mut best_move = moves[0]; 
        let mut best_score = match player {
            Player::X => i32::MIN,
            Player::O => i32::MAX,
        };
        let next_player = player.flip();
        for m in moves {
            board.apply_move(m, player);
            let (score, _, _) = SolutionAgent::minmax(board, next_player, depth + 1, max_depth);
            board.undo_move(m, player);
            // Add recursive call here
            match player {
                Player::X => {
                    if score > best_score {
                        best_score = score;
                        best_move = m;
                    }

                }
                Player::O => {
                    if score < best_score {
                        best_score = score;
                        best_move = m;
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
        let depth_limit = if move_count < 10 {
            move_count
        } else {
            4
        };
        SolutionAgent::minmax(board, player, 0, depth_limit)
    }
}
