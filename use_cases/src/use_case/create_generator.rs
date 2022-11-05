use crate::use_case::UseCaseA1;

pub struct CreateGeneratorUseCase;

struct CreateGenerator {
    name: String,
    output: u64,
    running: bool
}

impl UseCaseA1<CreateGenerator> for CreateGeneratorUseCase {
    type Output = CreateGenerator;
    fn execute(generator: CreateGenerator) -> Self::Output {
        return generator
    }
}