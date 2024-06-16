use some_lib::estimate;

fn main() {
	println!("Hello, world!");

	println!("test=> {}", estimate(2f32, 3f32, 1f32));

	let x = estimate(4f32, 2f32, 4f32);

	println!("hello to {name}", name = x);
}
