#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub enum OutputType {
    #[default]
    DrawToConsole,
    WriteToFile
}