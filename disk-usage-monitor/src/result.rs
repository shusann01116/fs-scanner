use crate::error::ScannerError;
use std::result::Result as StdResult;

pub(crate) type Result<T> = StdResult<T, ScannerError>;
