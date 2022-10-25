/// Audio callbacks are not always called at a perfectly regular interval, introducing jitter.
/// The HostTimeFilter utility struct performs a linear regression between
/// system time and sample time in order to improve the accuracy of system
/// time values used in the audio callback. On Windows, Ableton recommends using
/// ASIO (better timing accuracy and lower latency).
pub struct HostTimeFilter {
    points_buffer: Vec<TimeDataPoint>,
    max_buffer_size: usize,
    index: usize,
}

impl HostTimeFilter {
    pub fn new() -> Self {
        let max_buffer_size: usize = 512; // Default
        Self {
            points_buffer: Vec::with_capacity(max_buffer_size),
            max_buffer_size,
            index: 0,
        }
    }

    /// Reset internal buffer of [HostTimeFilter]
    pub fn reset(&mut self) {
        self.points_buffer = Vec::with_capacity(self.max_buffer_size);
        self.index = 0;
    }

    /// Performs a linear regression between system time and sample time in order
    /// to improve the accuracy of system time values. Usually used in the audio callback.
    pub fn sample_time_to_host_time(&mut self, clock_micros: i64, sample_clock: u64) -> i64 {
        assert!(clock_micros >= 0, "Negative clock values unsupported.");

        // Make a pair struct of the current sample time and corresponding clock_micros host time to add to the buffer
        let point = TimeDataPoint::new(sample_clock as u128, clock_micros as u128);

        // Fill buffer, then keep recycling it by adding new values at index
        if self.points_buffer.len() < self.max_buffer_size {
            self.points_buffer.push(point);
        } else {
            self.points_buffer[self.index] = point;
        }
        self.index = (self.index + 1) % self.max_buffer_size;

        // Calculate a line based on time data points currently in buffer
        let line = Self::linear_regression(&self.points_buffer);

        // Apply line to current sample time to get a filtered clock time in micros
        let filtered_clock_micros = line.slope * sample_clock as f64 + line.intercept;

        // Return result in i64 clock_micros() format
        filtered_clock_micros.round() as i64
    }

    /// Simple liner regression from a buffer of points in time on 2 different clocks. Math in microseconds
    /// can easily overflow a i64 or even a u64 when summing up lots of multiplications. Hence I am
    /// trying u128 here (Same as what Duration uses internally) to avoid that and to maintain accuracy.
    fn linear_regression(buffer: &Vec<TimeDataPoint>) -> Line {
        let num_points = buffer.len() as u128;
        assert!(num_points > 0, "Provide at least one TimeDataPoint.");

        let mut sum_x: u128 = 0;
        let mut sum_xx: u128 = 0;
        let mut sum_xy: u128 = 0;
        let mut sum_y: u128 = 0;

        for p in buffer {
            sum_x += p.sample_clock;
            sum_xx += p.sample_clock * p.sample_clock;
            sum_xy += p.sample_clock * p.host_clock;
            sum_y += p.host_clock;
        }

        let denominator = num_points * sum_xx - sum_x * sum_x;

        let slope = match denominator {
            0 => 0.0,
            _ => (num_points * sum_xy - sum_x * sum_y) as f64 / denominator as f64,
        };

        let intercept = (sum_y as f64 - slope * sum_x as f64) / num_points as f64;

        Line::new(slope, intercept)
    }
}

struct TimeDataPoint {
    sample_clock: u128,
    host_clock: u128,
}

impl TimeDataPoint {
    fn new(sample_clock: u128, host_clock: u128) -> Self {
        Self {
            sample_clock,
            host_clock,
        }
    }
}

struct Line {
    slope: f64,
    intercept: f64,
}

impl Line {
    fn new(slope: f64, intercept: f64) -> Self {
        Self { slope, intercept }
    }
}
