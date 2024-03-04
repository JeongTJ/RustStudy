fn main() {
	test_func();
	parameter_func(42);


	// let y = 6은 구문으로써 값을 반환하지 않기 때문에 x에 값이 바인딩되지 않음.
	// let x = (let y = 6);

	// 중괄호로 묶인 줄은 표현식으로써 값을 반환하기 때문에 y에 값이 바인딩 된다.
	let y = {
		let x = 3;
		x + 1
	};
	println!("y is {y}");
	println!("five plus one is {}", plus_one(5));
}

// 호출 이후에 선언해도 사용 가능하다.
fn test_func() {
	println!("test_func");
}


// 변수명: 타입 으로 파라미터 사용 가능
fn parameter_func(var: i32) {
	println!("var is {var}");
}

// var을 파라미터로 받아서 i32형 값을 반환한다.
fn plus_one(var: i32) -> i32 {
	var + 1
}