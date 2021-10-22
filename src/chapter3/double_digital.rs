use crate::chapter3::payoff2;

pub struct PayoffDoubleDigital {
    lower_level: f64,
    upper_level: f64,
}

impl PayoffDoubleDigital {
    #[allow(dead_code)]
    pub fn new(lower_level: f64, upper_level: f64) -> Self {
        PayoffDoubleDigital {
            lower_level,
            upper_level,
        }
    }
}

impl payoff2::Payoff for PayoffDoubleDigital {
    fn forward_value(&self, spot: f64) -> f64 {
        if spot <= self.lower_level {
            return 0.0;
        }
        if spot >= self.upper_level {
            return 0.0;
        }
        1.0
    }
}