fn main() {
	let x = 5;

	// 일반적으로 사용하는 if - else 문
	if x == 5 {
		println!("True");
	} else {
		println!("False");
	}
	
	// 마찬가지로 다중 if elseif else 문
	if x % 4 == 0 {
		println!("{x} is divisible 4");
	} else if x % 3 == 0 {
		println!("{x} is divisible 3");
	} else if x % 2 == 0 {
		println!("{x} is divisible 2");
	} else {
		println!("{x} is not divisible 4, 3, 2");
	}
	
	// 변수 선언도 if 문으로 가능! 삼항연산자랑 비슷한듯
	let condition = true;
	let var = if condition { 5 } else { 6 };
	println!("var is {var}");
}
