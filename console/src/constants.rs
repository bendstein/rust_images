///
/// Command line arguments
/// 
pub mod args {
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
        //pub const HELP: &str = "help";

        pub const OUTPUT_TYPE: &str = "output";

        ///
        /// Command line argument key for file path.
        /// 
        pub const FILE_PATH: &str = "path";

        ///
        /// Command line argument key indicating that even
        /// if truecolor is enabled, it should not be used
        /// when drawing to the console
        /// 
        pub const FORCE_DISABLE_TRUECOLOR: &str = "no_truecolor";
    }

    ///
    /// Command line argument values
    /// 
    pub mod values {
        pub mod output_type {
            pub const FILE: &str = "file";
            pub const DRAW: &str = "draw";
        }
    }
}

///
/// Environment variables
/// 
pub mod env {
    ///
    /// Environment variable keys
    /// 
    pub mod keys {
        ///
        /// Environment variable for whether console supports
        /// truecolor output
        /// 
        pub const TRUECOLOR_ENABLED: &str = "COLORTERM";
    }

    ///
    /// Environment variable values
    /// 
    pub mod values {
        ///
        /// Value for COLORTERM env variable indicating truecolor is enabled
        /// 
        pub const TRUECOLOR_ENABLED_TRUECOLOR: &str = "truecolor";

        ///
        /// Value for COLORTERM env variable indicating truecolor is enabled
        /// 
        pub const TRUECOLOR_ENABLED_24BIT: &str = "24bit";
    }
}

///
/// Constants for drawing to console
/// 
pub mod draw_to_console {
    ///
    /// Strings to use to represent a pixel in the console
    /// 
    pub const PIXEL_STRINGS: &str = "██,█▓,▓▓,▓▒,▒▒,▒░,░░,░ ";

    ///
    /// Delimiter between strings in PIXEL_STRINGS
    /// 
    pub const PIXEL_STRINGS_DELIMITER: &str = ",";
}