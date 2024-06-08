use anchor_lang::prelude::*;

declare_id!("FsjN2a9xkTjGSETa8mN7NBBYrUPSDGubnopjAjUX6Ypu");

#[program]
pub mod anchor_generics_mre {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub my_account: Account<'info, MyAccount>,
}

#[account]
struct MyAccount {
    data: GStruct<MyStruct>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
struct GStruct<T> {
    data: T,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
struct MyStruct {
    data: u32,
}
