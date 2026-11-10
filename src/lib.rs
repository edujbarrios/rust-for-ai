//! Minimal, dependency-free machine-learning examples.
//!
//! The first example fits a simple linear regression model by ordinary least
//! squares. It is educational, not a substitute for production ML libraries.

/// A fitted line: `y = slope * x + intercept`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LinearRegression {
    pub slope: f64,
    pub intercept: f64,
}

/// Errors returned while fitting a line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FitError {
    LengthMismatch,
    TooFewSamples,
    NonFiniteInput,
    ConstantFeature,
    NumericalOverflow,
}

impl std::fmt::Display for FitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::LengthMismatch => "x and y must have the same length",
            Self::TooFewSamples => "at least two samples are required",
            Self::NonFiniteInput => "all input values must be finite",
            Self::ConstantFeature => "x values must not all be identical",
            Self::NumericalOverflow => "calculation produced a non-finite value",
        };
        f.write_str(message)
    }
}

impl std::error::Error for FitError {}

impl LinearRegression {
    /// Fit a straight line with ordinary least squares.
    ///
    /// Returns an error for unequal input lengths, insufficient samples,
    /// non-finite inputs, a constant feature, or non-finite intermediate results.
    pub fn fit(x: &[f64], y: &[f64]) -> Result<Self, FitError> {
        if x.len() != y.len() {
            return Err(FitError::LengthMismatch);
        }
        if x.len() < 2 {
            return Err(FitError::TooFewSamples);
        }
        if x.iter().chain(y.iter()).any(|value| !value.is_finite()) {
            return Err(FitError::NonFiniteInput);
        }

        let count = x.len() as f64;
        let mean_x = x.iter().sum::<f64>() / count;
        let mean_y = y.iter().sum::<f64>() / count;
        if !mean_x.is_finite() || !mean_y.is_finite() {
            return Err(FitError::NumericalOverflow);
        }

        let (numerator, denominator) =
            x.iter()
                .zip(y)
                .fold((0.0_f64, 0.0_f64), |(xy, xx), (&xi, &yi)| {
                    let dx = xi - mean_x;
                    (xy + dx * (yi - mean_y), xx + dx * dx)
                });

        if denominator == 0.0 {
            return Err(FitError::ConstantFeature);
        }
        if !numerator.is_finite() || !denominator.is_finite() {
            return Err(FitError::NumericalOverflow);
        }

        let slope = numerator / denominator;
        let intercept = mean_y - slope * mean_x;
        if !slope.is_finite() || !intercept.is_finite() {
            return Err(FitError::NumericalOverflow);
        }

        Ok(Self { slope, intercept })
    }

    /// Predict the output for a new input value.
    pub fn predict(&self, x: f64) -> f64 {
        self.slope * x + self.intercept
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn learns_a_straight_line() {
        let model = LinearRegression::fit(&[0.0, 1.0, 2.0, 3.0], &[1.0, 3.0, 5.0, 7.0])
            .expect("valid training data");
        assert!((model.slope - 2.0).abs() < 1e-12);
        assert!((model.intercept - 1.0).abs() < 1e-12);
        assert!((model.predict(6.0) - 13.0).abs() < 1e-12);
    }

    #[test]
    fn rejects_mismatched_lengths() {
        assert_eq!(
            LinearRegression::fit(&[1.0, 2.0], &[3.0]),
            Err(FitError::LengthMismatch)
        );
    }

    #[test]
    fn rejects_too_few_samples() {
        assert_eq!(
            LinearRegression::fit(&[1.0], &[2.0]),
            Err(FitError::TooFewSamples)
        );
    }

    #[test]
    fn rejects_nan_or_infinity() {
        assert_eq!(
            LinearRegression::fit(&[1.0, f64::NAN], &[2.0, 3.0]),
            Err(FitError::NonFiniteInput)
        );
        assert_eq!(
            LinearRegression::fit(&[1.0, 2.0], &[2.0, f64::INFINITY]),
            Err(FitError::NonFiniteInput)
        );
    }

    #[test]
    fn rejects_constant_x() {
        assert_eq!(
            LinearRegression::fit(&[2.0, 2.0], &[1.0, 3.0]),
            Err(FitError::ConstantFeature)
        );
    }

    #[test]
    fn rejects_overflow() {
        assert_eq!(
            LinearRegression::fit(&[f64::MAX, f64::MAX / 2.0], &[1.0, 2.0]),
            Err(FitError::NumericalOverflow)
        );
    }
}
