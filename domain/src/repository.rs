pub mod generator_repository;

pub trait Repository<Rhs> {
    fn create(&self, rhs: Rhs) -> Rhs;
    fn update(&self, rhs: Rhs) -> Rhs;
    fn delete(&self, rhs: Rhs) -> Rhs;
    fn find_by_id(&self, id: i64) -> Rhs;
}


