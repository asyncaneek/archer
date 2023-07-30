use crate::arrow::Arrow;

pub enum Error
{
    FileNotFound,
    ParsingError,
}

// Archer's quiver contains a collection of Arrows
// It is a list of shell commands that can be executed on a target 
pub struct Quiver
{
    target: String,
    arrows: Vec<Arrow>,
    file_on_disk: std::path::Path,
}

impl Quiver 
{
    pub fn new( quiver_json: &std::path::Path ) -> Result<(), Error>
    {
        todo!()
    }

    pub fn load()
    {
        todo!()
    }

    pub fn save( )
    {
        todo!()
    }
}