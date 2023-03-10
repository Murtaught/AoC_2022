use crate::pawn::Direction;
use std::collections::HashMap;

pub type JumpTable = HashMap<usize, [(usize, Direction); 4]>;

pub fn verify_jump_table(t: &JumpTable) {
    assert_eq!(t.len(), 6);

    // Cube face (zone) index:
    for (&i, nav_array) in t {
        // Face neighbor by direction.
        for (j, (k, kdir)) in nav_array.iter().enumerate() {
            let dir = Direction::from(j);
            assert_eq!(t[k][kdir.reverse() as usize], (i, dir.reverse()), "i = {i}, j = {j}");
        }
    }
}
