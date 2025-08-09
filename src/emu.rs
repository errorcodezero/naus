#[derive(Default)]
struct Emulator {
    memory: Vec<u8>,
    halted: bool,
}

impl Emulator {
    fn new() -> Self {
        Self::default()
    }

    fn step(&self) {
        if (!self.halted) {}
    }
}
