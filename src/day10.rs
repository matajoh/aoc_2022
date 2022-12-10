use clock_circuit::Program;

mod clock_circuit {
    use crate::utils::read_some_to_vec;

    #[derive(Copy, Clone)]
    enum Instruction {
        AddX(usize, i32),
        Noop(usize),
        Start,
        End,
    }

    pub struct Program {
        counter: usize,
        cycle: usize,
        register: i32,
        current: Instruction,
        instructions: Vec<Instruction>,
    }

    fn to_instruction(line: &str) -> Option<Instruction> {
        let parts: Vec<&str> = line.trim().split(" ").collect();
        match parts[..] {
            ["noop"] => Some(Instruction::Noop(1)),
            ["addx", value] => Some(Instruction::AddX(2, value.parse().unwrap())),
            _ => None,
        }
    }

    impl Program {
        pub fn new(filename: &str) -> Program {
            Program {
                counter: 0,
                cycle: 1,
                register: 1,
                current: Instruction::Start,
                instructions: read_some_to_vec(filename, to_instruction),
            }
        }

        pub fn len(&self) -> usize {
            self.instructions.len()
        }

        fn inc(&mut self) -> Instruction {
            self.counter += 1;
            if self.counter < self.len() {
                self.instructions[self.counter]
            } else {
                Instruction::End
            }
        }

        pub fn next(&mut self) {
            (self.current, self.cycle) = match (self.current, self.cycle) {
                (Instruction::Start, 1) => (self.instructions[0], 1),
                (Instruction::End, cycle) => (Instruction::End, cycle),
                (Instruction::AddX(2, val), cycle) => (Instruction::AddX(1, val), cycle + 1),
                (Instruction::AddX(1, val), cycle) => {
                    self.register += val;
                    (self.inc(), cycle + 1)
                }
                (Instruction::Noop(1), cycle) => (self.inc(), cycle + 1),
                _ => panic!("Invalid program state"),
            }
        }

        pub fn reset(&mut self) {
            self.current = Instruction::Start;
            self.cycle = 1;
            self.register = 1;
            self.counter = 0;
        }

        pub fn signal_strength(&self) -> i32 {
            (self.cycle as i32) * self.register
        }

        pub fn is_running(&self) -> bool {
            match self.current {
                Instruction::End => false,
                _ => true,
            }
        }

        pub fn cycle(&self) -> usize {
            self.cycle
        }

        pub fn value(&self) -> i32 {
            self.register
        }
    }
}

fn part1(program: &mut Program) -> i32 {
    let mut total = 0;
    while program.cycle() < 221 {
        if program.cycle() >= 20 && (program.cycle() - 20) % 40 == 0 {
            total += program.signal_strength()
        }
        program.next()
    }

    total
}

fn part2(program: &mut Program) -> String {
    let mut line = [' '; 40];
    let mut lines: Vec<String> = vec![];
    while program.is_running() {
        let index = (program.cycle() - 1) % 40;
        line[index] = match (program.value() - (index as i32)).abs() {
            diff if diff < 2 => '#',
            _ => '.',
        };
        if program.cycle() % 40 == 0 {
            lines.push(line.iter().collect());
            line.fill(' ')
        };
        program.next()
    }

    lines.join("\n")
}

pub fn run() {
    let mut program = Program::new("data/day10.txt");
    println!("== Day 10 ==");
    println!("Part 1: {}", part1(&mut program));
    program.reset();
    println!("Part 2");
    print!("{}", part2(&mut program))
}
