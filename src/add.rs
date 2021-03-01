use std::ops::{Add, Sub};

pub fn add(f: u32, s: u32) -> u32 {
	let span = tracing::info_span!("Add function", ?f, ?s);
	let _guard = span.enter();
	tracing::info!("Info event");
	f.add(s)
}

pub fn fibonacci(till: u32) -> u32 {
	fib(till - 1) + fib(till - 2)
}

pub fn fib(till: u32) -> u32 {
	if till == 1 {
		tracing::info!("base case 1");
		1
	} else if till == 0 {
		tracing::info!("base case 0");
		0
	}
	else {
		let span = tracing::info_span!("Calculating for ", ?till);
		let _guard = span.enter();

		fib(till - 1) + fib(till - 2)
	}
}