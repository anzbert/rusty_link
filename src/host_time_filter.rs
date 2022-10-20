use crate::AblLink;

pub struct HostTimeFilter {
    points: Vec<TimeDataPoint>,
    index: usize,
    k_num_points: usize,
}

impl HostTimeFilter {
    pub fn new() -> Self {
        const K_NUM_POINTS: usize = 512; // Default Value
        Self {
            points: Vec::with_capacity(K_NUM_POINTS),
            k_num_points: K_NUM_POINTS,
            index: 0,
        }
    }

    pub fn sample_time_to_host_time(
        &mut self,
        unfiltered_clock_micros: i64,
        sample_time: u64,
    ) -> i64 {
        let point = TimeDataPoint::new(sample_time, unfiltered_clock_micros as u64);

        if self.points.len() < self.k_num_points {
            self.points.push(point);
        } else {
            self.points[self.index] = point;
        }
        self.index = (self.index + 1) % self.k_num_points;

        let result = Self::linear_regression(&self.points);

        let host_time = result.slope * sample_time as f64 + result.intercept;

        host_time.round() as i64
    }

    fn linear_regression(buffer: &Vec<TimeDataPoint>) -> Line {
        let num_points = buffer.len() as u64;
        assert!(num_points > 0);

        let mut sum_x: u64 = 0;
        let mut sum_xx: u64 = 0;
        let mut sum_xy: u64 = 0;
        let mut sum_y: u64 = 0;

        for p in buffer {
            sum_x += p.x;
            sum_xx += p.x * p.x;
            sum_xy += p.x * p.y;
            sum_y += p.y;
        }

        let denominator = num_points * sum_xx - sum_x * sum_x;
        let slope = match denominator {
            0 => 0.,
            _ => (num_points * sum_xy - sum_x * sum_y) as f64 / denominator as f64,
        };

        let intercept = (sum_y as f64 - slope * sum_x as f64) / num_points as f64;

        Line::new(slope, intercept)
    }
}

struct TimeDataPoint {
    x: u64,
    y: u64,
}

impl TimeDataPoint {
    fn new(x: u64, y: u64) -> Self {
        Self { x, y }
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
