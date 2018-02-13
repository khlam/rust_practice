fn main() {
	rock_paper_scissors();
}

fn win_condition(x: i32) -> bool{
	if x == 10 {
		return true;
	}
	false
}

fn rock_paper_scissors() {
	let mut count = 0;
	while !win_condition(count) {
		count = count + 1;
		println!("Count: {}", count);
	}
	println!("win condition met");
}
