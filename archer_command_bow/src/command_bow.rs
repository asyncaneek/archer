use archer_model::*;

struct CommandBow;

impl bow::Bow for CommandBow
{
    fn shoot( arrow: &arrow::Arrow ) -> Result<String, bow::Error> {
        todo!()
    }
}