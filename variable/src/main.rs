fn main() {
	let x = 5;

	let x = x + 1; // 섀도잉, 이전 변수의 값을 가려버린다.
	{
		let x = x * 2;
		println!("x is {x}");
	}
	println!("x is {x}");
	
	let string = "abcde";
	println!("string is {string}");
	let string = string.len(); // 다른 타입이어도 섀도잉이 된다! 아예 새로운 변수가 되는것이기 때문
	println!("string is {string}");

	let tuple: (i32, f64, char) = (2147483647, 1234.5678, 'a'); // 바인딩
	let (x, y, z) = tuple; // 튜플의 구조해체

	println!("{x}, {y}, {z}");
	println!("{}, {}, {}", tuple.0, tuple.1, tuple.2); // 인덱싱접근 가능!

	let a = [1, 2, 3, 4, 5]; // 초기화와 동시에 배열 선언
	let b: [i32; 5] = [1, 2, 3, 4, 5]; // [배열 타입; 크기] 선언 후 대입
	let c = [3; 5]; // 특정값으로 배열 초기화
	let d: [[i32; 5]; 5] = [[1, 2, 3, 4, 5], 
							[1, 2, 3, 4, 5], 
							[1, 2, 3, 4, 5], 
							[1, 2, 3, 4, 5], 
							[1, 2, 3, 4, 5]]; // 이차원 배열
	println!("test {}", d[0][0]);
}
