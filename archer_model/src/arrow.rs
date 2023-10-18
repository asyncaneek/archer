/// An arrow is any shell command that can fired on a target
/// It is the lowest level component of Archer
pub struct Arrow {
    // command to run, like "ping"
    command: String,
    // predicates to the command, like "127.0.0.1"
    predicate: String,
}
