use borsh::{
    BorshDeserialize,
    BorshSerialize,
};
use solana_program::{
        account_info::{next_account_info, AccountInfo}, entrypoint, entrypoint::ProgramResult, msg, program_error::ProgramError, pubkey::Pubkey
};

pub mod instruction;
use instruction::HelloInstruction;


#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct  GreetinAccount{
    pub counter: u32
}

entrypoint!(instruction_process);
pub fn instruction_process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
)-> ProgramResult{

    msg!("Hello solana developer");
    let instruction =  HelloInstruction::unpack(instruction_data)?;

    let account_iter = &mut accounts.iter();

    let account = next_account_info(account_iter)?;

    if account.owner != program_id{
        msg!("Account does not have correct program id");
        return  Err(ProgramError::IncorrectProgramId);
    }

    let mut gretting_account =  GreetinAccount::try_from_slice(&account.data.borrow())?;
    match instruction{
        HelloInstruction::Increment => {
          gretting_account.counter += 1;  
        },
        HelloInstruction::Decrement => {
            gretting_account.counter -= 1;
        },
        HelloInstruction::Set(val) =>{
            gretting_account.counter = val;
        }
    }

    gretting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    Ok(())
}

