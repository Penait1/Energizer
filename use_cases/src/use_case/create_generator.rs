use domain::entity::generator::Generator;
use crate::use_case::UseCaseA1;
use domain::repository::generator_repository::GeneratorRepository;

pub struct CreateGeneratorUseCase {
    repository: Box<dyn GeneratorRepository>
}

struct CreateGenerator {
    name: String,
    output: u64,
    running: bool
}

impl UseCaseA1<CreateGenerator> for CreateGeneratorUseCase {
    type Output = Generator;

    fn execute(&self, generator: CreateGenerator) -> Self::Output {
        self.repository.create(Generator {
            name: generator.name,
            output: generator.output,
            running: generator.running
        })
    }
}
