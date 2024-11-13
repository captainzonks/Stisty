use crate::error_types::CSVErrorKind::DataExtraction;
use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
// use crate::error_types::CSVErrorKind::Generic;

#[non_exhaustive]
pub struct CSVError<T>
where
    T: FromStr,
{
    pub row: usize,
    pub column: usize,
    pub value: String,
    pub kind: CSVErrorKind<T>,
}

impl<T> Display for CSVError<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug + Error + 'static,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: could not parse \"{}\" at row {}, column {}",
            self.kind, self.value, self.row, self.column
        )
    }
}

impl<T> Debug for CSVError<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug + Error + 'static,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("CSVError")
            .field("row", &self.row)
            .field("column", &self.column)
            .field("value", &self.value)
            .field("kind", &self.kind)
            .finish()
    }
}

impl<T> Error for CSVError<T>
where
    T: FromStr + 'static,
    <T as FromStr>::Err: Error,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.kind)
    }
}

#[non_exhaustive]
pub enum CSVErrorKind<T>
where
    T: FromStr,
{
    // ParseError { source: ParseError },
    // ParseIntError { source: ParseIntError },
    // ParseFloatError { source: ParseFloatError },
    // ParseCharError { source: ParseCharError },
    // ParseBoolError { source: ParseBoolError },
    DataExtraction { source: T::Err },
}

impl<T> Display for CSVErrorKind<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // error!("Stisty encountered an error");
        match &self {
            // CSVErrorKind::ParseError { .. } => write!(f, "parse error"),
            // CSVErrorKind::ParseIntError { source } => write!(f, "the string could not be parsed as int"),
            // CSVErrorKind::ParseFloatError { .. } => write!(f, "the string could not be parsed as float"),
            // CSVErrorKind::ParseCharError { .. } => write!(f, "the string could not be parsed as char"),
            // CSVErrorKind::ParseBoolError { .. } => write!(f, "the string could not be parsed as bool"),
            DataExtraction { source } => {
                // error!("{:?}", source);
                write!(f, "{:?}", source)
            }
            _ => {
                write!(f, "error while parsing")
            }
        }
    }
}

impl<T> Debug for CSVErrorKind<T>
where
    T: FromStr,
    <T as FromStr>::Err: Error + 'static,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("CSVErrorKind")
            .field("", &self.to_string())
            .finish()
    }
}

impl<T> Error for CSVErrorKind<T>
where
    T: FromStr,
    <T as FromStr>::Err: Error + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            // Self::ParseError { source } => Some(source),
            // Self::ParseIntError { source } => Some(source),
            // Self::ParseFloatError { source } => Some(source),
            // Self::ParseCharError { source } => Some(source),
            // Self::ParseBoolError { source } => Some(source),
            Self::DataExtraction { source } => Some(source),
            _ => None,
        }
    }
}
