pub enum PointError {
    Overshoot,
}

pub enum LegUpdateError {
    PointError,
    InvalidPlayer,
}

pub fn check_score(player_score: i32, throw: u8) -> Option<i32> {
    let new_score = player_score - i32::from(throw);
    if new_score < 0 {
        return None;
    }

    Some(new_score)
}
