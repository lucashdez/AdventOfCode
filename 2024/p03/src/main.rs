use std::ptr::NonNull;

struct StateMachine {
	a: State,
}


struct State {
	next: Option<NonNull<State>>,
}


impl StateMachine {
}


fn main() {
	let read = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    println!("Hello, world!");
}
