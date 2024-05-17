use arch::{decode_execute, State, Tracer};

fn main() {
    let instruction = 0x0000_0000;
    let mut state = State::init(0x0);

    let res = decode_execute(instruction, &mut state, &NoopTracer);
    dbg!(res);
}

struct NoopTracer;

impl Tracer for NoopTracer {
    fn begin(&self, _: u32, _: u64) {}

    fn end(&self) {}

    fn read_register<T: core::fmt::Debug>(&self, _: isize, _: T) {}

    fn write_register<T: core::fmt::Debug>(&self, _: isize, _: T) {}

    fn read_memory<T: core::fmt::Debug>(&self, _: usize, _: T) {}

    fn write_memory<T: core::fmt::Debug>(&self, _: usize, _: T) {}
}
