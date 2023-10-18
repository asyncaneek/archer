use archer_model::*;

struct Bow;

impl bow::Bow for Bow {
    fn shoot(arrow: &arrow::Arrow) -> Result<String, bow::Error> {
        todo!()
    }
}
