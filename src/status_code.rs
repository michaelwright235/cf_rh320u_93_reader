use std::{error::Error, fmt::Display};

/// ReaderError enum contains two types of possible errors:
/// UsbError and CommandError.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ReaderError {
    /// UsbError is returned if there was some kind of connection problem.
    UsbError(rusb::Error),
    /// CommandError is returned if a reader couldn't proccess given command in some way.
    CommandError(StatusCode),
    /// This command is not implemented yet.
    NotImplemented
}

impl Display for ReaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReaderError::UsbError(e) => f.write_str(e.to_string().as_str()),
            ReaderError::CommandError(e) => f.write_str(e.to_string().as_str()),
            ReaderError::NotImplemented => f.write_str("Not implemented yet")
        }
    }
}

impl From<rusb::Error> for ReaderError {
    fn from(e: rusb::Error) -> Self {
        Self::UsbError(e)
    }
}

impl From<StatusCode> for ReaderError {
    fn from(e: StatusCode) -> Self {
        Self::CommandError(e)
    }
}

impl Error for ReaderError {}

/// Status codes that a reader can throw.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum StatusCode {
    Ok,
    Failure,
    TimeoutError,
    NoCard,
    InvalidData,
    AuthenticationFailure,
    InternalError,
    UnknownError,
    OperationError,
    UnknownCommand,
    UnsupportedCommandForCard,
    InvalidCommandFormat,
    UnsupportedOptionForm, // ?
    BlockDoesntExist,
    BlockIsLocked,
    LockBlockFailure,
    WriteFailure,
    UnknownCode(u8),
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let StatusCode::UnknownCode(c) = self {
            let un = String::from("Received unknown status code: ") + format!("{:#X}", c).as_str();
            return f.write_str(un.as_str());
        }

        f.write_str(match self {
            StatusCode::Ok => "Success",
            StatusCode::Failure => "Failure",
            StatusCode::TimeoutError => "Timeout error",
            StatusCode::NoCard => "Card doesn't exist",
            StatusCode::InvalidData => "Receiving card data error",
            StatusCode::AuthenticationFailure => "The input parameter or input command format is incorrect",
            StatusCode::InternalError => "Internal error",
            StatusCode::UnknownError => "Unknown error",
            StatusCode::OperationError => "Operation error",
            StatusCode::UnknownCommand => "Unknown command",
            StatusCode::UnsupportedCommandForCard => "Unsupported command for this type of card",
            StatusCode::InvalidCommandFormat => "Invalid command format",
            StatusCode::UnsupportedOptionForm => "In the FLAG parameter of the command, OPTION mode is not supported (only for 15693 commands)", //?
            StatusCode::BlockDoesntExist => "Specified block doesn't exit",
            StatusCode::BlockIsLocked => "Specified block is locked",
            StatusCode::LockBlockFailure => "Couldn't lock specified block",
            StatusCode::WriteFailure => "Failed to write data",
            _ => "",
        })
    }
}

impl Error for StatusCode {}

impl From<u8> for StatusCode {
    fn from(byte: u8) -> Self {
        match byte {
            0x80 => Self::Ok,
            0x81 => Self::Failure,
            0x82 => Self::TimeoutError,
            0x83 => Self::NoCard,
            0x84 => Self::InvalidData,
            0x85 => Self::AuthenticationFailure,
            0x86 => Self::InternalError,
            0x87 => Self::UnknownError,
            0x89 => Self::OperationError,
            0x8f => Self::UnknownCommand,
            0x90 => Self::UnsupportedCommandForCard,
            0x91 => Self::InvalidCommandFormat,
            0x92 => Self::UnsupportedOptionForm,
            0x93 => Self::BlockDoesntExist,
            0x94 => Self::BlockIsLocked,
            0x95 => Self::LockBlockFailure,
            0x96 => Self::WriteFailure,
            _ => Self::UnknownCode(byte)
        }
    }
}

impl From<StatusCode> for u8 {
    fn from(code: StatusCode) -> Self {
        match code {
            StatusCode::Ok => 0x80,
            StatusCode::Failure => 0x81,
            StatusCode::TimeoutError => 0x82,
            StatusCode::NoCard => 0x83,
            StatusCode::InvalidData => 0x84,
            StatusCode::AuthenticationFailure => 0x85,
            StatusCode::InternalError => 0x86,
            StatusCode::UnknownError => 0x87,
            StatusCode::OperationError => 0x89,
            StatusCode::UnknownCommand => 0x8f,
            StatusCode::UnsupportedCommandForCard => 0x90,
            StatusCode::InvalidCommandFormat => 0x91,
            StatusCode::UnsupportedOptionForm => 0x92,
            StatusCode::BlockDoesntExist => 0x93,
            StatusCode::BlockIsLocked => 0x94,
            StatusCode::LockBlockFailure => 0x95,
            StatusCode::WriteFailure => 0x96,
            StatusCode::UnknownCode(byte) => byte,
        }
    }
}
