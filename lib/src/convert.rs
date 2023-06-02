pub trait ConvertableFrom<TFrom> {
    type Options;
    type Error;

    fn try_convert_from(value: TFrom, options: Self::Options) -> Result<Self, Self::Error> where Self: std::marker::Sized;

    fn convert_from(value: TFrom, options: Self::Options) -> Self where Self: std::marker::Sized, Self::Error: std::fmt::Debug {
       Self::try_convert_from(value, options).unwrap()
    }
}