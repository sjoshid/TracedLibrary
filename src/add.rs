use std::ops::Add;

pub fn add(f: u32, s: u32) -> u32 {
	let span = tracing::info_span!("Add function", ?f, ?s);
	let _guard = span.enter();
	tracing::info!("Info event");
	f.add(s)
}