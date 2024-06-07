pub fn estimate(theta0: f32, theta1: f32, x: f32) -> f32 {
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
