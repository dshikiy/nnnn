#[derive(Debug, PartialEq, Eq)]
pub enum Error {
NotEnoughPinsLeft,
GameComplete,
}

pub struct BowlingGame {
rolls: Vec<u32>,
}

impl BowlingGame {
pub fn new() -> Self {
    BowlingGame { rolls: Vec::new() }
}

pub fn roll(&mut self, pins: u32) -> Result<(), Error> {
    if pins > 10 {
        return Err(Error::NotEnoughPinsLeft);
    }

    if self.is_game_complete() {
        return Err(Error::GameComplete);
    }

    // Validate frame constraints during standard 10 frames
    let mut frame = 1;
    let mut i = 0;
    let mut is_first_roll_in_frame = true;

    while i < self.rolls.len() && frame < 10 {
        if self.rolls[i] == 10 {
            // Strike
            frame += 1;
            i += 1;
            is_first_roll_in_frame = true;
        } else {
            if is_first_roll_in_frame {
                is_first_roll_in_frame = false;
                i += 1;
            } else {
                // Second roll of the frame
                if self.rolls[i - 1] + self.rolls[i] > 10 {
                    return Err(Error::NotEnoughPinsLeft);
                }
                frame += 1;
                i += 1;
                is_first_roll_in_frame = true;
            }
        }
    }

    // If we are currently in frame 10 (or bonus rolls)
    if frame == 10 {
        // Check if it's the second roll of frame 10 and not a strike previously
        // Let's accurately count how many rolls belong to frame 10 and bonus
        let mut f10_rolls = 0;
        let mut idx = i;
        while idx < self.rolls.len() {
            f10_rolls += 1;
            idx += 1;
        }

        if f10_rolls == 1 {
            // We already had 1 roll in frame 10. If it was NOT a strike, 
            // the current roll is the second roll of frame 10.
            let first_f10 = self.rolls[self.rolls.len() - 1];
            if first_f10 != 10 && first_f10 + pins > 10 {
                return Err(Error::NotEnoughPinsLeft);
            }
        } else if f10_rolls >= 2 {
            // We are in bonus/fill balls. 
            // Let's check what happened in frame 10: did they get a strike or a spare?
            // Let's find the rolls of frame 10 precisely.
            let mut roll_history = &self.rolls[i..];
            // Actually, let's trace standard frames properly to know exactly where frame 10 starts.
        }
    }

    // A safer and simpler validation approach: check total frame counts and pin bounds dynamically
    self.rolls.push(pins);

    // Validate after pushing to ensure no rule was broken
    if let Err(e) = self.validate_state() {
        self.rolls.pop();
        return Err(e);
    }

    Ok(())
}

fn validate_state(&self) -> Result<(), Error> {
    let mut frame = 1;
    let mut i = 0;

    while i < self.rolls.len() {
        if frame > 10 {
            return Err(Error::GameComplete);
        }

        if frame < 10 {
            if self.rolls[i] == 10 {
                frame += 1;
                i += 1;
            } else {
                if i + 1 >= self.rolls.len() {
                    // Incomplete frame, valid so far
                    break;
                }
                if self.rolls[i] + self.rolls[i + 1] > 10 {
                    return Err(Error::NotEnoughPinsLeft);
                }
                frame += 1;
                i += 2;
            }
        } else {
            // Frame 10 validation
            // Frame 10 can have 2 or 3 rolls depending on strike/spare
            let f10_start = i;
            let mut rolls_in_f10 = 0;
            while i < self.rolls.len() {
                rolls_in_f10 += 1;
                i += 1;
            }

            if rolls_in_f10 == 2 {
                let first = self.rolls[f10_start];
                let second = self.rolls[f10_start + 1];
                if first != 10 && first + second > 10 {
                    return Err(Error::NotEnoughPinsLeft);
                }
            } else if rolls_in_f10 == 3 {
                let first = self.rolls[f10_start];
                let second = self.rolls[f10_start + 1];
                // If first was not a strike and they didn't make a spare, 3 rolls aren't allowed
                if first != 10 && first + second != 10 {
                    return Err(Error::NotEnoughPinsLeft);
                }
            } else if rolls_in_f10 > 3 {
                return Err(Error::GameComplete);
            }
            break;
        }
    }

    Ok(())
}

pub fn is_game_complete(&self) -> bool {
    let mut frame = 1;
    let mut i = 0;

    while i < self.rolls.len() && frame <= 10 {
        if frame < 10 {
            if self.rolls[i] == 10 {
                frame += 1;
                i += 1;
            } else {
                if i + 1 < self.rolls.len() {
                    frame += 1;
                    i += 2;
                } else {
                    return false;
                }
            }
        } else {
            // Frame 10 completion check
            let f10_rolls = self.rolls.len() - i;
            if f10_rolls < 2 {
                return false;
            }
            let first = self.rolls[i];
            let second = self.rolls[i + 1];
            if first == 10 || first + second == 10 {
                return f10_rolls >= 3;
            } else {
                return f10_rolls >= 2;
            }
        }
    }

    false
}

pub fn score(&self) -> Option<u32> {
    if !self.is_game_complete() {
        return None;
    }

    let mut total_score = 0;
    let mut i = 0;

    for _frame in 0..10 {
        if self.rolls[i] == 10 {
            // Strike
            total_score += 10 + self.rolls[i + 1] + self.rolls[i + 2];
            i += 1;
        } else if self.rolls[i] + self.rolls[i + 1] == 10 {
            // Spare
            total_score += 10 + self.rolls[i + 2];
            i += 2;
        } else {
            // Open frame
            total_score += self.rolls[i] + self.rolls[i + 1];
            i += 2;
        }
    }

    Some(total_score)
}
}
