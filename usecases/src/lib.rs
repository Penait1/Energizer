use crate::use_case::UseCaseA1;

mod use_case;

struct CreateGeneratorUseCase;

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