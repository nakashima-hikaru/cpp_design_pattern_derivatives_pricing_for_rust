use crate::chapter4::parameters::Parameters;
use crate::chapter6::random2::Random;
use crate::chapter7::exotic_engine::ExoticEngine;
use crate::chapter7::exotic_engine::ExoticEngineField;

///
pub struct ExoticBSEngine<'a> {
    exotic_engine_field: ExoticEngineField<'a>,
    /// A random number generator
    the_generator: &'a mut dyn Random,
    /// Drifts
    drifts: Vec<f64>,
    /// The standard deviations of logarithm of the stock price
    standard_deviations: Vec<f64>,
    /// A logarithm of a spot value
    log_spot: f64,
    /// times to record drifts, standard deviations and variates
    number_of_times: usize,
    /// Gaussian random variables generated by `self.the_generator`
    variates: Vec<f64>,
}

impl<'a> ExoticBSEngine<'a> {
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
        exotic_engine_field: ExoticEngineField<'a>,
        d: impl Parameters,
        vol: impl Parameters,
        the_generator: &'a mut (impl Random + 'a),
        spot: f64,
    ) -> ExoticBSEngine<'a> {
        let times = exotic_engine_field.get_the_product().get_look_at_times();
        let number_of_times = times.len();

        the_generator.reset_dimensionality(number_of_times);
        let mut drifts = vec![0.0; number_of_times];
        let mut standard_deviations = vec![0.0; number_of_times];

        let variance = vol.integral_square(0.0, times[0]);
        drifts[0] = exotic_engine_field.get_r().integral(0.0, times[0])
            - d.integral(0.0, times[0])
            - 0.5 * variance;
        standard_deviations[0] = variance.sqrt();
        for j in 1..number_of_times {
            let this_variance = vol.integral_square(times[j - 1], times[j]);
            drifts[j] = exotic_engine_field.get_r().integral(times[j - 1], times[j])
                - d.integral(times[j - 1], times[j])
                - 0.5 * this_variance;
            standard_deviations[j] = this_variance.sqrt();
        }
        let variates = vec![0.0; number_of_times];
        ExoticBSEngine {
            exotic_engine_field,
            the_generator,
            drifts,
            standard_deviations,
            log_spot: spot.ln(),
            number_of_times,
            variates,
        }
    }
}

impl<'a> ExoticEngine for ExoticBSEngine<'a> {
    /// Returns the pointer of `self.exotic_engine_field`
    fn as_exotic_engine_field(&self) -> &ExoticEngineField {
        &self.exotic_engine_field
    }

    /// Stores spot values on a path.
    ///
    /// # Arguments
    ///
    /// * `spot_values` - A container to store spot values
    fn get_one_path(&mut self, spot_values: &mut [f64]) {
        self.the_generator.get_gaussians(&mut self.variates);
        let mut current_log_spot = self.log_spot;
        for j in 0..self.number_of_times {
            current_log_spot += self.drifts[j];
            current_log_spot += self.standard_deviations[j] * self.variates[j];
            spot_values[j] = current_log_spot.exp();
        }
    }
}
