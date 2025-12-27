use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Dweeb {}

pub fn handler(ctx: Context<Dweeb>) -> Result<()> {
    msg!("You are a dweeb from: {:?}", ctx.program_id);
    Ok(())
}
