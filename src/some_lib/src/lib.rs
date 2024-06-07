use std::ops::{Add, Mul};

pub fn estimate<F>(theta0: F, theta1: F, x: F) -> F
where
	F: Add<F, Output = F> + Mul<F, Output = F> + Sized,
{
	theta0 + (theta1 * x)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_estimate() {
		assert_eq!(estimate(0.0, 0.0, 0.0), 0.0);
		assert_eq!(estimate(0.0, 0.0, 1.0), 0.0);
		assert_eq!(estimate(0.0, 0.0, 2.0), 0.0);
		assert_eq!(estimate(0.0, 1.0, 0.0), 0.0);
		assert_eq!(estimate(0.0, 1.0, 1.0), 1.0);
		assert_eq!(estimate(0.0, 1.0, 2.0), 2.0);
		assert_eq!(estimate(1.0, 0.0, 0.0), 1.0);
		assert_eq!(estimate(1.0, 0.0, 1.0), 1.0);
		assert_eq!(estimate(1.0, 0.0, 2.0), 1.0);
		assert_eq!(estimate(1.0, 1.0, 0.0), 1.0);
		assert_eq!(estimate(1.0, 1.0, 1.0), 2.0);
		assert_eq!(estimate(1.0, 1.0, 2.0), 3.0);
		assert_eq!(estimate(10.0, 2.0, 42.0), 94.0);
	}
}
