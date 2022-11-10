use crate::entity::generator::Generator;
use crate::repository::Repository;


pub trait GeneratorRepository : Repository<Generator> {}
