use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Deserialize, Serialize)]
pub struct SwapInstruction {
    pub amount: u32,
}

/// The type of state managed by this swap_program. The type defined here
/// much match the `GreetingAccount` type defined by the client.
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// The number of greetings that have been sent to this account.
    pub counter: u32,
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

    let instruction = SwapInstruction::try_from_slice(instruction_data)?;

    // Deserialize the greeting information from the account, modify"
    // it, and then write it back.
    let mut greeting = GreetingAccount::try_from_slice(&account.data.borrow())?;
    greeting.counter = instruction.amount;
    greeting.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}
