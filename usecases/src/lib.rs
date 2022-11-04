use crate::use_case::UseCase;

mod use_case;

struct CreateGenerator {
    name: bool
}

impl UseCase for CreateGenerator {
    fn execute(&self) -> bool {
        todo!()
    }
}
