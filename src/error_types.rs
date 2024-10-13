use std::char::ParseCharError;
use std::{error, fmt};
use std::error::Error;
use std::fmt::{write, Debug, Display, Formatter};
use std::num::{ParseFloatError, ParseIntError};
use std::str::{FromStr, ParseBoolError};
use std::string::ParseError;
use thiserror::Error;

type Result<T> = std::result::Result<T, CSVErrorKind<T>>;

#[non_exhaustive]
pub struct CSVError<T: std::str::FromStr> {
    pub line: usize,
    pub kind: CSVErrorKind<T>,
}

impl<T> Display for CSVError<T>
where
    T: FromStr,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "invalid data on line {}", self.line + 1)
    }
}

impl<T> Debug for CSVError<T>
where
    T: FromStr,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl<T> Error for CSVError<T>
where
    T: FromStr,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.kind)
    }
}

#[derive(Error, Debug)]
pub enum CSVErrorKind<T: std::str::FromStr> {
    // #[error("parseerror")]
    // Parse(#[from] ParseError),
    // #[error("parseinterror")]
    // ParseInt(#[from] ParseIntError),
    // #[error("parsefloaterror")]
    // ParseFloat(#[from] ParseFloatError),
    // #[error("parsecharerror")]
    // ParseChar(#[from] ParseCharError),
    // #[error("parseboolerror")]
    // ParseBool(#[from] ParseBoolError),
    // #[error("anyhowerror")]
    // Anyhow(#[from] anyhow::Error),

    Parse(ParseError),
    ParseInt(ParseIntError),
    ParseFloat(ParseFloatError),
    ParseChar(ParseCharError),
    ParseBool(ParseBoolError),
    Anyhow(anyhow::Error),
    FromStr(<T as FromStr>::Err),


    // Parse { source: ParseError },
    // ParseInt { source: ParseIntError },
    // ParseFloat { source: ParseFloatError },
    // ParseChar { source: ParseCharError },
    // ParseBool { source: ParseBoolError },
    // Anyhow { source: Box<dyn Error> },
}


impl<T> fmt::Display for CSVErrorKind<T>
where
    T: FromStr,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            CSVErrorKind::Parse(err) => write!(f, "Parse error: {}", err),
            CSVErrorKind::ParseInt(err) => write!(f, "the provided string could not be parsed as int"),
            CSVErrorKind::ParseFloat(err) => write!(f, "the provided string could not be parsed as float"),
            CSVErrorKind::ParseChar(err) => write!(f, "the provided string could not be parsed as char"),
            CSVErrorKind::ParseBool(err) => write!(f, "the provided string could not be parsed as bool"),
            CSVErrorKind::Anyhow(err) => write!(f, "{}", err),
            CSVErrorKind::FromStr(err) => write!(f, "the provided string could not be parsed as string"),
            _ => { write!(f, "error while parsing") }
        }
    }
}

// impl Error for CSVErrorKind {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         match self {
//             Self::Parse { source } => Some(source),
//             Self::ParseInt { source } => Some(source),
//             Self::ParseFloat { source } => Some(source),
//             Self::ParseChar { source } => Some(source),
//             Self::ParseBool { source } => Some(source),
//             _ => None,
//         }
//     }
// }

// impl<T: error::Error + Send + Sync + 'static> From<T> for CSVError<T> {
//     fn from(e: T) -> Self {
//         Self::from(e)
//     }
// }

// impl From<T> for CSVErrorKind {
//     fn from(error: T) -> Self {
//         CSVErrorKind::Generic(error)
//     }
// }

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.


// impl<T> Debug for CSVError<T> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         match *self {
//             CSVError::Parse(ref err) => write!(f, "Parse error: {}", err),
//             CSVError::ParseInt(..) => write!(f, "the provided string could not be parsed as int"),
//             CSVError::ParseFloat(..) => write!(f, "the provided string could not be parsed as float"),
//             CSVError::ParseChar(..) => write!(f, "the provided string could not be parsed as char"),
//             CSVError::ParseBool(..) => write!(f, "the provided string could not be parsed as bool"),
//         }
//     }
// }

// impl<T> error::Error for CSVError<T> {
//     fn source(&self) -> Option<&(dyn error::Error + 'static)> {
//         match *self {
//             CSVError::Parse(ref err) => Some(err),
//             CSVError::ParseInt(ref err) => Some(err),
//             CSVError::ParseFloat(ref err) => Some(err),
//             CSVError::ParseChar(ref err) => Some(err),
//             CSVError::ParseBool(ref err) => Some(err),
//             _ => { None }
//         }
//     }
// }

impl<T> From<ParseError> for CSVErrorKind<T>
where
    T: FromStr,
{
    fn from(err: ParseError) -> CSVErrorKind<T> {
        CSVErrorKind::Parse(err)
    }
}

impl<T> From<ParseIntError> for CSVErrorKind<T>
where
    T: FromStr,
{
    fn from(err: ParseIntError) -> CSVErrorKind<T> {
        CSVErrorKind::ParseInt(err)
    }
}

impl<T> From<ParseFloatError> for CSVErrorKind<T>
where
    T: FromStr,
{
    fn from(err: ParseFloatError) -> CSVErrorKind<T> {
        CSVErrorKind::ParseFloat(err)
    }
}

impl<T> From<ParseCharError> for CSVErrorKind<T>
where
    T: FromStr,
{
    fn from(err: ParseCharError) -> CSVErrorKind<T> {
        CSVErrorKind::ParseChar(err)
    }
}

impl<T> From<ParseBoolError> for CSVErrorKind<T>
where
    T: FromStr,
{
    fn from(err: ParseBoolError) -> CSVErrorKind<T> {
        CSVErrorKind::ParseBool(err)
    }
}

impl<T> From<anyhow::Error> for CSVErrorKind<T>
where
    T: FromStr,
{
    fn from(err: anyhow::Error) -> CSVErrorKind<T> {
        CSVErrorKind::Anyhow(err)
    }
}

// impl<T> From<<T as FromStr>::Err> for CSVErrorKind<T>
// where
//     T: FromStr,
// {
//     fn from(err: <T as FromStr>::Err) -> Self {
//         CSVErrorKind::FromStr(err)
//     }
// }

// impl<T: FromStr + std::error::Error + std::marker::Send + std::marker::Sync> FromStr for CSVError<T>
// where
//     T::Err: error::Error + Send + Sync + 'static,
// {
//     type Err = anyhow::Error;
//     fn from_str(s: &str) -> std::result::Result<CSVError<T>, anyhow::Error>
//     where
//         <T as FromStr>::Err: std::error::Error,
//     {
//         let i: T = s.parse()?; //.with_context(|| format!("parsing {:?}", s))?;
//         Ok(CSVError::from(i))
//     }
// }

// 141381703