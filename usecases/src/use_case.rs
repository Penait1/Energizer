pub trait UseCase {
    fn execute(&self) -> bool;
}
