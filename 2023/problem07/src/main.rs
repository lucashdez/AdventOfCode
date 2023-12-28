mod lib;

fn p1() -> u64 {
	todo!()
}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn p1_working() {
		assert_eq!(p1(), 6440);
	}
}
