//cd -> cargo new rsmain -> cargo build -> cargo run
/* 
MAT1830 - Discete Math for Comp Sci Tutorial Sheet 1 | Q4 a

Imagine you are Commissioner Gordon, chief of the Gotham City police. The Joker
has set up a bomb which will explode unless a jug containing exactly 4L of water is
placed on a scale. Only jugs with capacities of 7L, 21L, and 28L can be used to obtain
this amount. Do you send Barbara Noether (Gotham’s foremost expert on maths) or 
Batman (Gotham’s foremost expert on punching the Joker and stopping things from blowing up)?

Below is a BFS solution:
*/

use std::collections::{VecDeque, HashSet};

pub fn jugs(n: i32) -> String {
    let target_vol = n;
    let jug_cap = [7, 21, 28];
    let solution = water_jugs(target_vol, jug_cap);
    match solution {
        None => String::from("No solution found."),
        Some(steps) => {
            for (i, state) in steps.iter().enumerate() {
                println!("Step {}: ({}, {}, {})", i, state.0, state.1, state.2);
            }
            String::from("Solution found.")
        }
    }
}
    
fn water_jugs(target_vol: i32, jug_cap: [i32; 3]) -> Option<Vec<(i32, i32, i32)>> {
    let jg1 = jug_cap[0];
    let jg2 = jug_cap[1];
    let jg3 = jug_cap[2];
    let init_state = (0,0,0);
    let mut queue: VecDeque<((i32, i32, i32), Vec<(i32, i32, i32)>)> = VecDeque::new();
    /*
    The front is the first element. The back is the last element.
    In an ordinary vector, it is hard to add stuff to the front.
        [2, 3, 4, 5, _, _, _, _, _, _, _]
        to add a "1" to the front we
        need to copy all the entries over
        [_, 2, 3, 4, 5, _, _, _, _, _, _]
        and then add our "1"
        [1, 2, 3, 4, 5, _, _, _, _, _, _]
    The copy is expensive - O(n).
    With a VecDeque, the front is allowed to "wrap around"
        [2, 3, 4, 5, _, _, _, _, _, _, _]
        to add a "1" to the front
        we just "wrap" the front around
        [2, 3, 4, 5, _, _, _, _, _, _, 1]
             This is now the front-----^
    So it's super easy to add stuff to the front - O(1)! It's actually as easy as adding stuff to the back.
    */
    queue.push_back((init_state, Vec::new()));
    let mut visited = HashSet::new();
    /*
    Used HashSet as we don't need to keep nodes in order or know where they are, just need to check if they've been visited before.
    */
    visited.insert(init_state);

    /*
    cur_state : Where you are
    Path : How you got there
    */
    while let Some((cur_state, path)) = queue.pop_front() {
        let (j1,j2,j3) = cur_state;
        if j1 == target_vol || j2 == target_vol || j3 == target_vol {
            return Some(path);
        }

        let mut actions = Vec::new();

        //Fill Jugs
        actions.push((jg1, j2, j3));
        actions.push((j1, jg2, jg3));
        actions.push((j1, j2, jg3));

        //Empty Jugs
        actions.push((0, j2, j3));
        actions.push((j1, 0, j3));
        actions.push((j1, j2, 0));

        //Pour Jugs
        /* 
        std::cmp::max : Prevent negative overflow
        std::cmp::min : Prevent overflow
        */
        actions.push((std::cmp::max(j1-jg2, 0), std::cmp::min(j2+jg1, j1), j3)); //Pour j1 into j2
        actions.push((std::cmp::max(j1-jg3, 0), j2, std::cmp::min(j3+jg1, j1))); //Pour j1 into j3
        actions.push((std::cmp::min(j1+jg2, j2), std::cmp::max(j2-jg1, 0), j3)); //Pour j2 into j1
        actions.push((std::cmp::max(0, j2 - (jg3 - j3)), std::cmp::max(j3-jg1, 0), std::cmp::min(j1+jg2, j2))); //Pour j2 into j3
        actions.push((std::cmp::max(j1-jg3, 0), std::cmp::min(j3+jg2, j2), j3)); //Pour j3 into j1
        actions.push((j1, std::cmp::max(j2-jg3, 0), std::cmp::min(j3+jg2, j2))); //Pour j3 into j2

        for next_state in actions{
            if !visited.contains(&next_state) {
                visited.insert(next_state);
                let mut new_path = path.clone();
                new_path.push(next_state);  
                queue.push_back((next_state, new_path));
            }
        }
    }

    None
}
