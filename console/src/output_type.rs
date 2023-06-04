#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub enum OutputType {
    #[default]
    OutputToConsole,
    DrawToConsole,
    WriteToFile,
    OutputHex
}