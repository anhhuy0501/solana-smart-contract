use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    program_error::ProgramError,
    pubkey::Pubkey,
};
const TOKEN_PRICE: u64 = 10;

/// ConstantPriceCurve struct implementing CurveCalculator
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SwapToken {
    /// 1 token SOL trade for Amount of token MOVE
    pub token_price: u64,
}

/// Encodes all results of swapping from a source token to a destination token
#[derive(Debug, PartialEq)]
pub struct SwapWithoutFeesResult {
    /// Amount of source token swapped
    pub source_amount_swapped: u128,
    /// Amount of destination token swapped
    pub destination_amount_swapped: u128,
}

impl SwapToken {
    fn swap_without_fees(&self, source_amount: u128) -> Option<SwapWithoutFeesResult> {
        let token_b_price = self.token_price as u128;

        let (source_amount_swapped, destination_amount_swapped) =
            (source_amount, source_amount.checked_mul(token_b_price)?);
        let source_amount_swapped = map_zero_to_none(source_amount_swapped)?;
        let destination_amount_swapped = map_zero_to_none(destination_amount_swapped)?;
        Some(SwapWithoutFeesResult {
            source_amount_swapped,
            destination_amount_swapped,
        })
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Deserialize, Serialize)]
pub struct SwapInstruction {
    pub amount: u128,
}

/// The type of state managed by this swap_program. The type defined here
/// much match the `GreetingAccount` type defined by the client.
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// The number of greetings that have been sent to this account.
    pub counter: u128,
}

/// Declare the programs entrypoint. The entrypoint is the function
/// that will get run when the swap_program is executed.
#[cfg(not(feature = "exclude_entrypoint"))]
entrypoint!(process_instruction);

/// Logic that runs when the swap_program is executed. This swap_program expects
/// a single account that is owned by the swap_program as an argument and
/// no instructions.
///
/// The account passed in ought to contain a `GreetingAccount`. This
/// swap_program will increment the `counter` value in the
/// `GreetingAccount` when executed.
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> entrypoint::ProgramResult {
    // Get the account that stores greeting count information.
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the swap_program in order for the
    // swap_program to write to it. If that is not the case then the
    // swap_program has been invoked incorrectly and we report as much.
    if account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let swap = SwapInstruction::try_from_slice(instruction_data)?;

    let swap_token = SwapToken {
        token_price: TOKEN_PRICE,
    };
    let result = swap_token.swap_without_fees(swap.amount);

    // Deserialize the greeting information from the account, modify"
    // it, and then write it back.
    let mut greeting = GreetingAccount::try_from_slice(&account.data.borrow())?;
    greeting.counter = result
        .ok_or(ProgramError::InvalidArgument)
        .destination_amount_swapped;
    greeting.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}

/// Helper function for mapping to SwapError::CalculationFailure
pub fn map_zero_to_none(x: u128) -> Option<u128> {
    if x == 0 {
        None
    } else {
        Some(x)
    }
}
