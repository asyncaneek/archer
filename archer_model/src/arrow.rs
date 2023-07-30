/// An arrow is any shell command that can fired on a target
/// It is the lowest level component of Archer
pub struct Arrow
{
    // for now a single string will represent the entire command
    // can be split up into command and args in the future if needed
    command: String
}
