use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Max tasks reached")]
    MaxTasksReached,
}
