pub mod bitmap {
    ///
    /// Bitmap signature should always be ASCII BM in little endian
    /// i.e. 0x4D42
    /// 
    pub const SIGNATURE: u16 = 0x4D42;

    ///
    /// The size of the DIB header, in bytes
    /// 
    pub const HEADER_SIZE: u32 = 14;

    ///
    /// The size of the info header, in bytes
    /// 
    pub const INFO_HEADER_SIZE: u32 = 40;

    ///
    /// The size of the color table is this times the number of records
    /// 
    pub const COLOR_TABLE_SIZE_FACTOR: u32 = 4;
}