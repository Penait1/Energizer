use domain::entity::generator::Generator;
use crate::use_case::UseCaseA0;
use domain::repository::generator_repository::GeneratorRepository;

pub struct ListGeneratorsUseCase {
    repository: Box<dyn GeneratorRepository>
}

impl UseCaseA0<Vec<Generator>> for ListGeneratorsUseCase {
    fn execute(&self) -> Vec<Generator> {
        self.repository.find_all()
    }
}
