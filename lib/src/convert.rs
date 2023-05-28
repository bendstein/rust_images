pub trait ConvertableFrom<TFrom> {
    type Options;
    type Error;

    fn try_convert_from(value: TFrom, options: Self::Options) -> Result<Self, Self::Error> where Self: std::marker::Sized;
}