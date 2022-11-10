pub mod create_generator;
pub mod delete_generator;
pub mod list_generators;

pub trait UseCaseA0<Rhs> {
    fn execute(&self) -> Rhs;
}

pub trait UseCaseA1<Rhs> {
    type Output;

    fn execute(&self, rhs: Rhs) -> Self::Output;
}
