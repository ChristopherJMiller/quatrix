/// Represents an instance of game scoring
///
/// Game scoring is performed every time a player drops a block and clears
/// a row or column. The score increases based on a series of multipliers.
pub struct GameScore {
    /// Raw accumulated score.
    ///
    /// Is increased based on the following formula of multipliers:
    ///
    /// `score_delta = 1 * drop_timer_mult * mult * rank_mult (if Some)`
    score: u64,
    /// The rank the player has achieved.
    /// Starts at 1 and increments on a logarithmic scale.
    ///
    /// Ranks can be consumed to increase the rank boost timer,
    /// based on the number of the rank.
    rank: u32,
    /// The current rank multiplier. Set by a rank boost being activated.
    rank_mult: Option<f32>,
    /// The active rank boost timer. Decays with time and floors to 0.
    rank_boost_timer: f32,
    /// The rank buffer. As scores are increased, the buffer is also increased.
    ///
    /// Once the buffer reached a certain value related to the rank, it is consumed and a rank up occurs
    rank_buffer: u64,
    /// The score requires before the next rank is reached
    next_rank: u64,
    /// An active multiplier on score. Decays with time.
    mult: f32,
    /// A rate modifier for the multiplier to declay.
    mult_decay_rate: f32,
    /// The drop timer
    drop_timer: DropTimer,
}

impl GameScore {
    /// Creates a new game score
    pub fn new() -> Self {
        Self {
            score: 0,
            rank: 1,
            rank_mult: None,
            rank_boost_timer: 0.0,
            rank_buffer: 0,
            next_rank: Self::next_rank_score(1),
            mult: 1.0,
            mult_decay_rate: 3.0,
            drop_timer: DropTimer::new(5.0, 10.0),
        }
    }

    /// Update with elapsed delta time secs
    pub fn update(&mut self, dt_secs: f32) {
        // Decrease timer if any value on it, and turn off rank mult if timer runs out
        if self.rank_boost_timer >= f32::EPSILON {
            self.rank_boost_timer = (self.rank_boost_timer - dt_secs).max(0.0);
            if self.rank_boost_timer <= f32::EPSILON {
                self.rank_mult = None;
            }
        }

        // Decay standard multiplier
        self.mult = (self.mult - dt_secs * (self.mult_decay_rate)).max(1.0);

        // Pass time on drop timer
        self.drop_timer.pass_time(dt_secs);
    }

    /// Gets the current score
    pub fn score(&self) -> u64 {
        self.score
    }

    /// Gets the current rank
    pub fn rank(&self) -> u32 {
        self.rank
    }

    /// Calculates the next rank up score
    fn next_rank_score(current_rank: u32) -> u64 {
        10 * current_rank.pow(2)
    }

    /// Adds score with all the extra multipliers.
    ///
    /// Increases the current standard multiplier before scoring, which decays over time.
    pub fn add_score(&mut self, number_of_row_cols_cleared: u32) {
        self.mult += (number_of_row_cols_cleared as f32).powf(2.0);

        let score_delta =
            points as f32 * self.drop_timer.mult() * self.mult * self.rank_mult.unwrap_or(1.0);
        let score_delta = score_delta.round() as u64;

        self.rank_buffer += score_delta;

        if self.rank_buffer >= self.next_rank {
            self.rank_buffer = self.rank_buffer.saturating_sub(self.next_rank as u64);
            self.rank += 1;
            self.next_rank = Self::next_rank_score(self.rank);
        }

        self.score += score_delta;
    }

    /// Consume a rank and activate a boost based on the rank
    pub fn rank_boost(&mut self) {
        if self.rank > 1 {
            self.rank -= 1;
            self.rank_boost_timer = self.rank as f32 * 5.0;
            self.rank_mult = Some(1.0 + (self.rank as f32 * 0.1));
        }
    }
}

/// A drop timer and it's configuration
///
/// Given a maximum grantable multiplier and timer,
/// can calculate a scoring multiplier based on how long it took
/// for the player to drop a block.
pub struct DropTimer {
    /// The maximum multiplier value
    max_mult: f32,
    /// The starting drop timer in seconds
    start_timer_secs: f32,
    /// The current time remaining. Floors to 0
    current_remaining: f32,
}

impl DropTimer {
    pub fn new(max_mult: f32, timer_secs: f32) -> Self {
        Self {
            max_mult,
            start_timer_secs: timer_secs,
            current_remaining: timer_secs,
        }
    }

    /// Passes time on the drop timer
    pub fn pass_time(&mut self, elapsed_secs: f32) -> f32 {
        self.current_remaining = (self.current_remaining - elapsed_secs).max(0.0);

        self.current_remaining
    }

    /// Resets the drop timer
    pub fn reset(&mut self) {
        self.current_remaining = self.start_timer_secs;
    }

    /// Gets the current
    pub fn mult(&self) -> f32 {
        self.current_remaining * self.max_mult
    }
}
