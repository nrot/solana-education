use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey
};

fn to_lamports(sol: u64) -> u64 {
    sol * 1000000000
}

entrypoint!(process);
fn process(program_id: &Pubkey, accounts: &[AccountInfo], instructions: &[u8])->ProgramResult {
    msg!("Hello start contract");
    let account_iter = &mut accounts.iter();

    let src_acc = next_account_info(account_iter)?;
    let dst = next_account_info(account_iter)?;

    Ok(())
}
