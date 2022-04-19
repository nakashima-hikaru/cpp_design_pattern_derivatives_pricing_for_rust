use crate::chapter4::parameters::Parameters;
use crate::chapter6::random2::RandomBase;
use crate::chapter7::exotic_engine::ExoticEngine;
use crate::chapter7::exotic_engine::ExoticEngineField;
use std::sync::Arc;
use std::sync::Mutex;

///
pub struct ExoticBSEngine {
    exotic_engine_field: ExoticEngineField,
    /// A random number generator
    the_generator: Arc<Mutex<dyn RandomBase>>,
    /// Drifts
    drifts: Vec<f64>,
    /// The standard deviations of logarithm of the stock price
    standard_deviations: Vec<f64>,
    /// A logarithm of a spot value
    log_spot: f64,
    /// times to record drifts, standard deviations and variates
    number_of_times: u64,
}

impl ExoticBSEngine {
    /// Constructor.
    ///
    /// # Arguments
    ///
    /// * `exotic_engine_field` - Common information on exotic engine
    /// * `d` - A dividend
    /// * `vol` - A volatility
    /// * `the_generator` - A random number generator
    /// * `spot` - A spot value of a stock
    pub fn new(
        exotic_engine_field: ExoticEngineField,
        d: Box<dyn Parameters>,
        vol: Box<dyn Parameters>,
        the_generator: Arc<Mutex<dyn RandomBase>>,
        spot: f64,
    ) -> ExoticBSEngine {
        let times = exotic_engine_field.get_the_product().get_look_at_times();
        let number_of_times = times.len() as u64;

        the_generator
            .lock()
            .as_mut()
            .unwrap()
            .reset_dimensionality(number_of_times);
        let mut drifts = vec![0.0; number_of_times as usize];
        let mut standard_deviations = vec![0.0; number_of_times as usize];

        let variance = vol.integral_square(0.0, times[0]);
        drifts[0] = exotic_engine_field.get_r().integral(0.0, times[0])
            - d.integral(0.0, times[0])
            - 0.5 * variance;
        standard_deviations[0] = variance.sqrt();
        for j in 1..number_of_times {
            let this_variance = vol.integral_square(times[(j - 1) as usize], times[j as usize]);
            drifts[j as usize] = exotic_engine_field
                .get_r()
                .integral(times[(j - 1) as usize], times[j as usize])
                - d.integral(times[(j - 1) as usize], times[j as usize])
                - 0.5 * this_variance;
            standard_deviations[j as usize] = this_variance.sqrt();
        }
        ExoticBSEngine {
            exotic_engine_field,
            the_generator,
            drifts,
            standard_deviations,
            log_spot: spot.ln(),
            number_of_times,
        }
    }
}

impl ExoticEngine for ExoticBSEngine {
    /// Returns the pointer of `self.exotic_engine_field`
    fn as_exotic_engine_field(&self) -> &ExoticEngineField {
        &self.exotic_engine_field
    }

    /// Stores spot values on a path.
    ///
    /// # Arguments
    ///
    /// * `spot_values` - A container to store spot values
    fn get_one_path(&mut self) -> Vec<f64> {
        let variates = self.the_generator.lock().as_mut().unwrap().get_gaussians();
        let mut current_log_spot = self.log_spot;
        let mut spot_values = vec![0.0; self.number_of_times as usize];
        for j in 0..self.number_of_times {
            current_log_spot += self.drifts[j as usize];
            current_log_spot += self.standard_deviations[j as usize] * variates[j as usize];
            spot_values[j as usize] = current_log_spot.exp();
        }
        spot_values
    }
}
