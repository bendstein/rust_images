pub mod bitmap;

pub trait ConvertableFrom<TFormat> {
    type Options;
    type Error;

    fn try_convert_from(value: TFormat, options: Self::Options) -> Result<Self, Self::Error> where Self: std::marker::Sized;
}