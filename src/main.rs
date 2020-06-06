//! A simple ByteByteJump VM.


/// The VM.
struct ByteByteJump {
    memory: [u8; 256],
    pc: u8
}

impl ByteByteJump {
    pub fn new(memory: [u8; 256]) -> Self {
        Self {
            memory,
            pc: 0u8
        }
    }

    pub fn run(&mut self) {
        loop {
            let destination: usize = self.memory[self.pc as usize] as usize;
            let source: usize = self.memory[(self.pc + 1) as usize] as usize;
            let jump: u8 = self.memory[(self.pc + 2) as usize];

            if destination == source && jump == self.pc {
                println!("Encountered exit bytes. Exiting!");
                return;
            }

            self.memory[source] = self.memory[destination];
            self.pc = jump;
        }
    }
}

fn main() {
    // let mut vm = ByteByteJump::new([0; 256]);
    // vm.run();
}
