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
    sysvar::{rent::Rent, Sysvar},
};
use spl_token::instruction::transfer;

// Define the state of a staking account
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct StakingAccount {
    pub is_initialized: bool,
    pub owner: Pubkey,
    pub amount: u64,
}

// Define the instructions that the program can process
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum StakingInstruction {
    InitializeStakeAccount,
    Stake { amount: u64 },
    Unstake { amount: u64 },
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = StakingInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        StakingInstruction::InitializeStakeAccount => {
            msg!("Instruction: InitializeStakeAccount");
            initialize_stake_account(program_id, accounts)
        }
        StakingInstruction::Stake { amount } => {
            msg!("Instruction: Stake");
            stake(program_id, accounts, amount)
        }
        StakingInstruction::Unstake { amount } => {
            msg!("Instruction: Unstake");
            unstake(program_id, accounts, amount)
        }
    }
}

fn initialize_stake_account(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let staking_account = next_account_info(accounts_iter)?;
    let owner = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // Create the staking account
    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(std::mem::size_of::<StakingAccount>());

    invoke(
        &system_instruction::create_account(
            owner.key,
            staking_account.key,
            rent_lamports,
            std::mem::size_of::<StakingAccount>() as u64,
            program_id,
        ),
        &[owner.clone(), staking_account.clone(), system_program.clone()],
    )?;

    // Initialize the staking account data
    let mut account_data = staking_account.try_borrow_mut_data()?;
    let mut staking_account_data = StakingAccount::try_from_slice(&account_data)?;
    staking_account_data.is_initialized = true;
    staking_account_data.owner = *owner.key;
    staking_account_data.amount = 0;
    staking_account_data.serialize(&mut &mut account_data[..])?;

    Ok(())
}

fn stake(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let staking_account = next_account_info(accounts_iter)?;
    let owner = next_account_info(accounts_iter)?;
    let source_token_account = next_account_info(accounts_iter)?;
    let destination_token_account = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;

    // Transfer tokens from the user's account to the program's account
    invoke(
        &transfer(
            token_program.key,
            source_token_account.key,
            destination_token_account.key,
            owner.key,
            &[],
            amount,
        )?,
        &[
            source_token_account.clone(),
            destination_token_account.clone(),
            owner.clone(),
            token_program.clone(),
        ],
    )?;

    // Update the staking account data
    let mut account_data = staking_account.try_borrow_mut_data()?;
    let mut staking_account_data = StakingAccount::try_from_slice(&account_data)?;
    staking_account_data.amount += amount;
    staking_account_data.serialize(&mut &mut account_data[..])?;

    Ok(())
}

fn unstake(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let staking_account = next_account_info(accounts_iter)?;
    let owner = next_account_info(accounts_iter)?;
    let source_token_account = next_account_info(accounts_iter)?;
    let destination_token_account = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;

    // Update the staking account data
    let mut account_data = staking_account.try_borrow_mut_data()?;
    let mut staking_account_data = StakingAccount::try_from_slice(&account_data)?;
    if staking_account_data.amount < amount {
        return Err(ProgramError::InsufficientFunds);
    }
    staking_account_data.amount -= amount;
    staking_account_data.serialize(&mut &mut account_data[..])?;

    // Transfer tokens from the program's account back to the user's account
    invoke(
        &transfer(
            token_program.key,
            source_token_account.key,
            destination_token_account.key,
            owner.key,
            &[],
            amount,
        )?,
        &[
            source_token_account.clone(),
            destination_token_account.clone(),
            owner.clone(),
            token_program.clone(),
        ],
    )?;

    Ok(())
}
