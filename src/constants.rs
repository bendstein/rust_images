///
/// Prefix for command line arguments.
/// 
pub const ARGUMENT_PREFIX: &str = "/";

///
/// Delimiter to split command line arguments
/// as key to value.
/// 
pub const ARGUMENT_DELIMITER: &str = ":";

///
/// Command line argument keys
/// 
pub mod keys {
    ///
    /// Command line argument key to print help docs.
    /// 
    pub const HELP: &str = "help";

    ///
    /// Command line argument key for file path.
    /// 
    pub const FILE_PATH: &str = "path";
}