use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
        if player.ask_to_compare(min) == 0 {
            return min;
        }
        let mut low = min + 1;
        let mut high = max;
        while low <= high {
            let mid = low + (high - low) / 2;
            let answer = player.ask_to_compare(mid);
            if answer == 0{
                return mid;
            } else if answer == -1{
                high = mid -1;
            } else if answer == 1{
                low = mid + 1;
            }
        }
        panic!("Not in range");
    }
}

