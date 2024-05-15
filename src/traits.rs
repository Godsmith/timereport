pub trait Parsable
where
    Self: Sized,
{
    fn from_str(text: &str) -> Result<Self, String>;
}
