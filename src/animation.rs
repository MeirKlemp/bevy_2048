//! This module contains the impl of the struct `Animation`.

use bevy::core::Timer;

/// Component used to animate with "updates".
/// Update will occur in rate of 60 fps.
pub struct Animation {
    timer: Timer,
    ticks: usize,
    max_ticks: usize,
    finished: bool,
}

impl Animation {
    /// Creates new animation.
    /// `max_ticks` is the number of updates
    /// the animation will do.
    pub fn new(max_ticks: usize) -> Self {
        Self {
            timer: Timer::from_seconds(1.0 / 60.0, true),
            ticks: 0,
            max_ticks,
            finished: false,
        }
    }

    /// Returns a value in the range [0, 1] for the animation.
    pub fn value(&self) -> f32 {
        self.ticks as f32 / self.max_ticks as f32
    }

    /// Returns the reversed value, for "backwards" animation.
    pub fn rev_value(&self) -> f32 {
        1.0 - self.value()
    }

    /// Updates the animation, needs `delta_seconds` from the time resource.
    /// Returns `true` if the timer finished,
    /// which means the `value()` have been changed.
    pub fn update(&mut self, delta_seconds: f32) -> bool {
        if !self.finished {
            self.timer.tick(delta_seconds);

            if self.timer.finished {
                self.ticks += 1;
                if self.ticks >= self.max_ticks {
                    self.finished = true;
                }
            }

            return self.timer.finished;
        }

        false
    }

    /// Returns `true` if the animation finished.
    pub fn finished(&self) -> bool {
        self.finished
    }

    /// Resets the animation.
    pub fn reset(&mut self) {
        self.timer.reset();
        self.ticks = 0;
        self.finished = false;
    }
}
