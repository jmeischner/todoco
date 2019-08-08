pub mod controlterm;
pub mod searchterm;

#[derive(Clone)]
enum MatchType {
    All,
    Tags,
    Files,
    Text,
    None,
}