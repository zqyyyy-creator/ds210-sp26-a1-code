use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

// Your solution solution.
pub struct SolutionAgent {}

// Put your solution here.
impl Agent for SolutionAgent {
    // Should returns (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your solution will make.
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        if board.game_over() {
            return (board.score(), 0, 0);
        };
        let moves = board.moves();
        let mut best_move = moves[0]; 
        let mut best_score = match player {
            Player::X => i32::MIN,
            Player::O => i32::MAX,
        };
        let next_player = player.flip();
        for m in moves {
            board.apply_move(m, player);
            let (score, _, _) = SolutionAgent::solve(board, next_player, _time_limit);
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
