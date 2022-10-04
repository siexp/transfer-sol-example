use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

#[derive(BorshSerialize, BorshDeserialize)]
pub struct TransferInstruction {
    pub lamports: u64,
}

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Transfer $SOL example program_id {}", program_id);

    let accounts_iter = &mut accounts.iter();
    let from_account = next_account_info(accounts_iter)?;
    let to_account = next_account_info(accounts_iter)?;

    // we don't use given account as we rely on the system account received via system_program::id()
    // inside system_instruction::transfer however you should check it with solana_program::system_program::check_id
    let _system_program_account = next_account_info(accounts_iter)?;

    let lamports = TransferInstruction::try_from_slice(instruction_data)?.lamports;
    msg!(
        "transfering {} lamports from {} to {}",
        lamports,
        from_account.key,
        to_account.key
    );

    if from_account.lamports() < lamports {
        return Err(ProgramError::InsufficientFunds);
    }

    msg!(
        "from_account balance {} is enough to transfer {}",
        from_account.lamports(),
        lamports
    );

    invoke(
        &system_instruction::transfer(from_account.key, to_account.key, lamports),
        &[from_account.clone(), to_account.clone()],
    )
}
