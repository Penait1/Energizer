pub mod generator_repository;

pub trait Repository<Rhs> {
    fn create(rhs: Rhs) -> Rhs;
    fn update(rhs: Rhs) -> Rhs;
    fn delete(rhs: Rhs) -> Rhs;
    fn find_by_id(id: i64) -> Rhs;
}


