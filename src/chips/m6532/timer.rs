pub struct Timer {
    /// Timer value as it is read / written through register, measured in intervals.
    value: u8,

    /// Cycles remaining in the current interval.
    cycles_remaining: u16,

    /// Interval configured via register. Can be 1, 8, 64, or 1024.
    interval: u16,

    expired: bool,
}

impl Timer {
    pub fn new() -> Self {
        // According to https://atariage.com/forums/topic/256802-two-questions-about-the-pia/?do=findComment&comment=3590223,
        // the interval is set to 1024T at startup, while the actual timer value is random.
        Self {
            value: 0xAA, // "random" value
            cycles_remaining: 1024,
            interval: 1024,
            expired: false,
        }
    }

    pub fn expired(&self) -> bool {
        self.expired
    }

    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn reset(&mut self, value: u8, interval: u16) {
        self.value = value - 1;
        self.interval = interval;
        self.cycles_remaining = interval - 1;
    }

    pub fn tick(&mut self) {
        // Decrement cycles remaining in current interval.
        self.cycles_remaining = self.cycles_remaining.wrapping_sub(1);

        // Did the cycles remaining go below 0?
        if self.cycles_remaining == std::u16::MAX {
            // Decrement timer value.
            self.value = self.value.wrapping_sub(1);

            // Did the timer go below 0?
            if self.value == 0xFF {
                self.expired = true;
            }

            if self.expired {
                // Timer is "finished" - now we should start counting down once per clock cycle.
                self.cycles_remaining = 0;
            } else {
                // Start new interval.
                self.cycles_remaining = self.interval - 1;
            }
        }
    }
}