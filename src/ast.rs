#[derive(Default, Clone, Debug)]
pub struct Node {
	pub token: char,
	pub a: Option<Box<Node>>,
	pub b: Option<Box<Node>>,
}
