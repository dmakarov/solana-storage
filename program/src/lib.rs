#![allow(unused_doc_comments)]
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// The type of state managed by this program. The type defined here
/// much match the `ObjectSchema` type defined by the client.
#[derive(BorshSerialize, BorshDeserialize)]
pub struct ControlWrapper<T> {
    pub exists: bool,
    pub value: T,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct ColorObject {
    /// The color components.
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

/// Declare the programs entrypoint. The entrypoint is the function
/// that will get run when the program is executed.
#[cfg(not(feature = "exclude_entrypoint"))]
entrypoint!(process_instruction);

/// The accounts passed in ought to contain enough data space to hold a
/// `ColorObject`, and a control byte to indicate existence of a valid
/// ColorObject in the account data.
///
/// This program will update the ColorObject value and control byte in
/// the account(s) data when executed.
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> entrypoint::ProgramResult {
    let mut data = instruction_data;
    let entry = match String::deserialize(&mut data) {
        Ok(s) => s,
        Err(_) => return Err(ProgramError::InvalidInstructionData),
    };

    let accounts_iter = &mut accounts.iter();

    if entry == "create" {
        let red = u8::deserialize(& mut data).unwrap_or_default();
        let green = u8::deserialize(& mut data).unwrap_or_default();
        let blue = u8::deserialize(& mut data).unwrap_or_default();
        let sender = next_account_info(accounts_iter)?;
        // The account must be owned by the program in order for the
        // program to write to it. If that is not the case then the
        // program has been invoked incorrectly and we report as much.
        if sender.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }
        create(sender, red, green, blue)?;
    } else if entry == "transfer" {
        let receiver = next_account_info(accounts_iter)?;
        if receiver.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }
        let sender = next_account_info(accounts_iter)?;
        if sender.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }
        transfer(receiver, sender)?;
    } else {
        return Err(ProgramError::InvalidInstructionData);
    }

    Ok(())
}

fn create(sender: &AccountInfo, red: u8, green: u8, blue: u8) -> entrypoint::ProgramResult {
    move_to(sender, ColorObject{red, green, blue})
}

fn transfer(receiver: &AccountInfo, sender: &AccountInfo) -> entrypoint::ProgramResult {
    match move_from::<ColorObject>(sender) {
        Ok(co) => move_to(receiver, co),
        Err(e) => Err(e),
    }
}

/// Deserialize the ControlWrapper information from the account,
/// modify it, and then write it back.
fn move_to<T>(receiver: &AccountInfo, value: T) -> entrypoint::ProgramResult
where T: BorshDeserialize, T: BorshSerialize {
    let mut control = ControlWrapper::<T>::try_from_slice(&receiver.data.borrow())?;
    if control.exists {
        Err(ProgramError::Custom(17))
    } else {
        control.exists = true;
        control.value = value;
        control.serialize(&mut &mut receiver.data.borrow_mut()[..])?;
        Ok(())
    }
}

fn move_from<T>(sender: &AccountInfo) -> std::result::Result<T, ProgramError>
where T: BorshDeserialize, T: BorshSerialize {
    let mut control = ControlWrapper::<T>::try_from_slice(&sender.data.borrow())?;
    if control.exists {
        control.exists = false;
        control.serialize(&mut &mut sender.data.borrow_mut()[..])?;
        Ok(control.value)
    } else {
        Err(ProgramError::Custom(19))
    }
}
