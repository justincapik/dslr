mod relu;
mod sigmoid;

pub use relu::ReLU;
pub use sigmoid::Sigmoid;

use crate::Float;

pub trait Activation {
	fn forward(x: Float) -> Float;
	fn backward(x: Float) -> Float;
}
