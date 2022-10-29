use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Faucet address is None")]
    NoFaucetAddy {},

    #[error("Submessage Reply Failure ||| ParseReplyError: {p_r_e} ||| Err Value: {v}")]
    SubMsgReplyFailure {p_r_e: String, v: String},

    #[error("Submessage Reply uncaught Error ||| {st}")]
    SubMsgUncaughtErr {st: String},

    #[error("Invalid submsg id: {x}")]
    InvalidId {x: u64},

}