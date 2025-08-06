use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was deployed to
    accounts: &[AccountInfo], // Ignored, all helloworld instructions are helloworlds
    instruction_data: &[u8], // Ignored, all helloworld instructions are helloworlds
) -> ProgramResult {
    msg!("Hello, world!");

    Ok(())
}
