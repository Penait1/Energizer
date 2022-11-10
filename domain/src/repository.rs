pub mod generator_repository;

pub trait Repository<Rhs> {
    fn create(&self, rhs: Rhs) -> Rhs;
    fn update(&self, rhs: Rhs) -> Rhs;
    fn delete(&self, id: i64) -> bool;
    fn find_by_id(&self, id: i64) -> Rhs;
    fn find_all(&self) -> Vec<Rhs>;
}


