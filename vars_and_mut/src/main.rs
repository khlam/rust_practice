fn main() {
    let mut x = 5; // mut allows x to be changed from 5 to 6
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Shadowing
    let y = 5;
    let y = y + 1;
    let y = y * 2;

    println!("The value of y is: {}", y);

	let spaces = "   ";
	println!("Spaces Len: {}", spaces.len());

	print!("Result: 5+1={}\n", plus_one(5));

	let list = [10, 20, 30, 40, 50];
	for element in list.iter(){
		println!("Value: {}", element);
	} 
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
