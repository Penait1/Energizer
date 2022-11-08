pub mod create_generator;

pub trait UseCaseA1<Rhs> {
    type Output;

    fn execute(&self, rhs: Rhs) -> Self::Output;
}
