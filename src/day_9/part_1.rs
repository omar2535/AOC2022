struct Pos {
    x: i32,
    y: i32
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

pub fn solution() {
    let tail_pos: Pos = Pos {x: 0, y: 0};
    let head_pos: Pos = Pos {x: 0, y: 0};
}



fn get_next_tail_pos(tail_pos: Pos, head_pos: Pos) -> Pos {
    // if on top of each other, don't move tail
    if tail_pos == head_pos {
        return tail_pos;
    }
    // if diagonal, move 

    // stub
    return Pos {x: 0, y: 0};
}
