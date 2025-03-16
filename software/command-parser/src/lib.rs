//! lib
//!
//! Showcase how you can test on host and later use on target

// no_std library except for test
#![cfg_attr(not(test), no_std)]

#[derive(Debug, PartialEq)]
pub enum Command {
    FrequencyHz(u32),
    Duty(u8),
    Start,
    Stop,
}

/// Parse a byte slice to an `Option<Command>`
/// None indicates an error
///
/// # Examples
/// ```
/// use command_parser::*;
/// assert_eq!(
///     parse(b"start some text that we don't care about"),
///     Some(Command::Start)
/// );
/// ```
/// ```
/// use command_parser::*;
/// assert_eq!(
///     parse(b"freq 1000 some text we don't care about"),
///     Some(Command::FrequencyHz(1000))
/// );
/// ```
pub fn parse(bytes: &[u8]) -> Option<Command> {
    let mut split = bytes.splitn(bytes.len(), |c| *c == b' ');
    let next = split.next()?;
    match next {
        b"start" => Some(Command::Start),
        b"stop" => Some(Command::Stop),
        b"freq" => {
            let next = split.next()?;
            let str = core::str::from_utf8(next).ok()?;
            let v: u32 = str.parse().ok()?;
            Some(Command::FrequencyHz(v))
        }
        b"duty" => {
            let next = split.next()?;
            let str = core::str::from_utf8(next).ok()?;
            let v: u8 = str.parse().ok()?;
            Some(Command::Duty(v))
        }
        _ => None,
    }
}

#[cfg(test)]
mod test_parse {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(b"start some text that we don't care about"),
            Some(Command::Start)
        );
    }

    #[test]
    fn test_parse2() {
        assert_eq!(
            parse(b"stop some text that we don't care about"),
            Some(Command::Stop)
        );
    }

    #[test]
    fn test_parse3() {
        assert_eq!(
            parse(b"freq 100 some text that we don't care about"),
            Some(Command::FrequencyHz(100))
        );
    }

    #[test]
    fn test_parse4() {
        assert_eq!(parse(b"freq some text that we don't care about"), None);
    }

    #[test]
    fn test_parse5() {
        assert_eq!(parse(b"duty some text that we don't care about"), None);
    }

    #[test]
    fn test_parse6() {
        assert_eq!(parse(b"duty 5000"), None);
    }

    #[test]
    fn test_parse7() {
        assert_eq!(parse(b"duty 50"), Some(Command::Duty(50)));
    }
}

/// Error type for parse_result
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Input not Utf8 conformant
    NonUtf8,
    /// Command Missing or Illegal
    CommandNotFound,
    /// Argument missing
    ArgMissing,
    /// Parsing error for the argument
    ArgError,
    /// Nr Arguments wrong
    ArgNumber,
}

/// Parse a byte slice to an `Result<Command, Error>`
/// The Error type is locally defined indicating the error
///
/// # Examples
/// ```
/// use command_parser::*;
/// assert_eq!(
///     parse_result(b"start some text that we don't care about"),
///     Err(Error::ArgNumber)
/// );
/// ```
/// ```
/// use command_parser::*;
/// assert_eq!(
///     parse_result(b"freq    1000"),
///     Ok(Command::FrequencyHz(1000))
/// );
/// ```
pub fn parse_result(bytes: &[u8]) -> Result<Command, Error> {
    // let's work on &str instead of raw byte arrays
    let str = core::str::from_utf8(bytes).map_err(|_| Error::NonUtf8)?;
    let mut split = str.split_whitespace();
    let next = split.next().ok_or(Error::CommandNotFound)?;
    let result = match next {
        "start" => Ok(Command::Start),
        "stop" => Ok(Command::Stop),
        "freq" => {
            let next = split.next().ok_or(Error::ArgMissing)?;
            let v: u32 = next.parse().map_err(|_| Error::ArgError)?;
            Ok(Command::FrequencyHz(v))
        }
        "duty" => {
            let next = split.next().ok_or(Error::ArgMissing)?;
            let v: u8 = next.parse().map_err(|_| Error::ArgError)?;
            Ok(Command::Duty(v))
        }
        _ => Err(Error::CommandNotFound)?,
    };
    match split.next() {
        None => result,
        _ => Err(Error::ArgNumber),
    }
}

#[cfg(test)]
mod test_parse_result {
    use super::*;

    #[test]
    fn test_parse_result() {
        assert_eq!(parse_result(b"start"), Ok(Command::Start));
    }

    #[test]
    fn test_parse_result_spaces() {
        assert_eq!(parse_result(b"   start   "), Ok(Command::Start));
    }

    #[test]
    fn test_parse_result2() {
        assert_eq!(parse_result(b"stop"), Ok(Command::Stop));
    }

    #[test]
    fn test_parse_result3() {
        assert_eq!(parse_result(b"freq 100"), Ok(Command::FrequencyHz(100)));
    }

    #[test]
    fn test_parse_result4_spaces() {
        assert_eq!(
            parse_result(b"freq        100"),
            Ok(Command::FrequencyHz(100))
        );
    }

    #[test]
    fn test_parse_result3_arg_err() {
        assert_eq!(parse_result(b"freq"), Err(Error::ArgMissing));
    }

    #[test]
    fn test_parse_result3_arg_not_found() {
        assert_eq!(
            parse_result(b"freq x some text that we don't care about"),
            Err(Error::ArgError)
        );
    }

    #[test]
    fn test_parse_result_not_found() {
        assert_eq!(
            parse_result(b"unrecognized some text that we don't care about"),
            Err(Error::CommandNotFound)
        );
    }

    #[test]
    fn test_duty_ok() {
        assert_eq!(
            parse_result(b"duty 100"),
            Ok(Command::Duty(100))
        );
    }

    #[test]
    fn test_duty_unhappy() {
        assert_eq!(
            parse_result(b"duty -100"),
            Err(Error::ArgError)
        );
    }
}
