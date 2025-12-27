pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

#[allow(unused_imports)]
use solana_security_txt::security_txt;

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "Program Verify Test",
    project_url: "https://github.com/AnAllergyToAnalogy/verifytest0",
    contacts: "email:anallergytoanalogy@gmail.com",
    policy: "https://github.com/AnAllergyToAnalogy/verifytest0/blob/main/SECURITY.md",
    preferred_languages: "en",
    source_code: "https://github.com/AnAllergyToAnalogy/verifytest0"
}

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("7DF4vtVhP9ATVkE6K8M5Ji1tCxf4aHqTQkZh5bwbEEzC");

#[program]
pub mod verifytest0 {
    use super::*;

    pub fn dweeb(ctx: Context<Dweeb>) -> Result<()> {
        dweeb::handler(ctx)
    }
}
