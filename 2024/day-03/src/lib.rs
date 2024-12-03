pub mod parser;

#[derive(Debug)]
pub struct Program {
    code: Vec<Instruction>,
    pub conditionals: bool,
}

impl From<Vec<Instruction>> for Program {
    fn from(value: Vec<Instruction>) -> Self {
        Self {
            code: value,
            conditionals: true,
        }
    }
}

#[derive(Debug)]
pub struct State {
    enabled: bool,
}

impl Default for State {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[derive(Debug)]
pub enum Instruction {
    Mul(u64, u64),
    Do,
    Dont,
}

impl Program {
    #[must_use]
    pub fn run(self) -> u64 {
        let mut state = State::default();

        self.code
            .into_iter()
            .filter(|i| self.conditionals || matches!(i, Instruction::Mul(..)))
            .fold(0, |sum, i| sum + i.run(&mut state).unwrap_or(0))
    }
}

impl Instruction {
    pub fn run(self, state: &mut State) -> Option<u64> {
        match self {
            Self::Mul(n1, n2) => {
                if state.enabled {
                    Some(n1 * n2)
                } else {
                    Some(0)
                }
            }
            Self::Do => {
                state.enabled = true;
                None
            }
            Self::Dont => {
                state.enabled = false;
                None
            }
        }
    }
}
