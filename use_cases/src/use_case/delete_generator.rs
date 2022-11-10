use crate::use_case::UseCaseA1;
use domain::repository::generator_repository::GeneratorRepository;

pub struct DeleteGeneratorUsecase {
    repository: Box<dyn GeneratorRepository>
}

impl UseCaseA1<i64> for DeleteGeneratorUsecase {
    type Output = bool;

    fn execute(&self, id: i64) -> Self::Output {
        self.repository.delete(id)
    }
}
