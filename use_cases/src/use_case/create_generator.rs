use crate::use_case::UseCaseA1;
use domain::repository::generator_repository::GeneratorRepository;

pub struct CreateGeneratorUseCase {
    repository: GeneratorRepository
}

struct CreateGenerator {
    name: String,
    output: u64,
    running: bool
}

impl UseCaseA1<CreateGenerator> for CreateGeneratorUseCase {
    type Output = CreateGenerator;
    fn execute(&self, generator: CreateGenerator) -> Self::Output {
        self.repository.create(generator);
        return GeneratorRepository::create(generator);
    }
}
