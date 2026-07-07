use errgonomic::{exit_result, handle, handle_bool, handle_opt};
use fdc::Database;
use rkyv::rancor::Error as RkyvError;
use rkyv::to_bytes;
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::ExitCode;
use thiserror::Error;

pub fn main() -> ExitCode {
    exit_result(run())
}

fn run() -> Result<ExitCode, RunError> {
    use RunError::*;
    let mut args = env::args_os();
    let _program = args.next();
    let dir = handle_opt!(args.next().map(PathBuf::from), DirArgNotFound);
    handle_bool!(args.next().is_some(), TooManyArgsInvalid);
    let database = handle!(Database::try_from(dir.as_path()), DatabaseTryFromFailed, dir);
    let bytes = handle!(to_bytes::<RkyvError>(&database), SerializeDatabaseFailed);
    let mut stdout = io::stdout().lock();
    handle!(stdout.write_all(&bytes), WriteStdoutFailed);
    Ok(ExitCode::SUCCESS)
}

#[derive(Error, Debug)]
pub enum RunError {
    #[error("missing required data directory argument")]
    DirArgNotFound,
    #[error("too many arguments")]
    TooManyArgsInvalid,
    #[error("failed to read FDC database from '{dir}'")]
    DatabaseTryFromFailed { source: fdc::TryFromPathForDatabaseError, dir: PathBuf },
    #[error("failed to serialize FDC database")]
    SerializeDatabaseFailed { source: RkyvError },
    #[error("failed to write rkyv database to stdout")]
    WriteStdoutFailed { source: io::Error },
}
