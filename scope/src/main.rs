fn main() {
	let s1 = String::from("hello");
	println!("s1: {s1}");
	// 힙영역에 쌓임
	// s1의 변수가 s2로 move
	let s2 = s1;
	// 이후로는 s1을 사용 못함
	// println!("s1 is {s1}");
	println!("s2: {s2}");
	println!();
	
	// 할당되지 않은 변수는 이동이 아닌 copy
	// 크기가 정해진 값이기 때문에 스택 영역에 두개가 쌓임
	let x = 5;
	let y = x;
	println!("x : {x}, y: {y}");
	println!();
	
	// 문자열 리터럴도 컴파일 시 알 수 있음
	// 스택영역에 "Hello"가 두개쌓임
	let s3 = "Hello";
	let s4 = s3;
	println!("s3: {s3}");
	println!("s4: {s4}");
	println!();
	
	// clone으로 힙영역의 객체를 복사해서 만듬
	let s1 = String::from("hello");
	let mut s2 = s1.clone();	// 가변으로 만들어야
	s2.push_str(", world");		// 변형가능
	println!("s1: {s1}");
	println!("s2: {s2}");
	println!();
	
	// 모든 정수형 타입
	// bool 타입
	// 부동 소수점 타입
	// 문자타입
	// 카피 가능한 타입들로 이루어진 튜플
	// 이 다섯가지만 카피가능
	let t1: (i32, i64, f32, f64, bool) = (-100, 100, 100.001, -100.001, true);
	let t2: (i32, i64, f32, f64, bool) = t1;
	println!("t1: ({}, {}, {}, {}, {})", t1.0, t1.1, t1.2, t1.3, t1.4);
	println!("t2: ({}, {}, {}, {}, {})", t2.0, t2.1, t2.2, t2.3, t2.4);

	let s = String::from("Hello");	// main에서 소요권이 있는 변수 s
	takes_ownership(s);				// 함수에 넘기는 순간 move가 일어나서 s의 소유를 잃음
	// takes_ownership(s);			// 이후로는 s변수를 사용 불가

	let x = 5;
	make_copys(x);

}

fn takes_ownership(parm: String)
{
	println!("{parm}");
}

fn make_copys(parm: i32)
{
	println!("{parm}");
}
