use anchor_lang::prelude::*;

#[error_code]
pub enum StakeError {
    #[msg("Freeze period not passed")]
    FreezePeriodNotPassed,
    #[msg("Max stake reached")]
    MaxStakeReached,
}
