use super::play::PlayerScores;

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
        // return Err(PointError::Overshoot);
        return None;
    }

    Some(new_score)
}

pub fn has_won_set(leg_score: i32, set_length: i32) -> bool {
    leg_score == set_length
}

pub fn has_won_game(set_score: i32, game_length: i32) -> bool {
    set_score == game_length
}

// updates score and checks if set or game score should be updated
// and if new legs or sets should be created
pub fn leg_win_score_update(
    scores: &mut PlayerScores,
    set_length: i32,
    game_length: i32,
) -> (bool, bool, bool) {
    let mut should_create_new_leg = false;
    let mut should_create_new_set = false;
    let mut game_won = false;
    scores.leg_score += 1;
    // check set
    if has_won_set(scores.leg_score, set_length) {
        scores.set_score += 1;
        // check game
        if has_won_game(scores.set_score, game_length) {
            game_won = true;
        } else {
            should_create_new_set = true;
        }
    } else {
        should_create_new_leg = true;
    }
    (should_create_new_set, should_create_new_leg, game_won)
}
