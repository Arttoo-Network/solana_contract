use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, TokenAccount};


declare_id!("HiiYfCVs6Xwt2nw8PEnwaBXQK2y1z87NShikyPXTD5QS");

#[program]
pub mod airdrop {
    use super::*;

    #[event]
    pub struct PDAAddressGenerated(Pubkey);

    pub fn initialize(ctx: Context<Initialize>, owner: Pubkey) -> Result<()> {
        let seeds = &[&b"landing_page"[..], &[ctx.accounts.airdrop_state.key().as_ref()[0]]];
        let (pda, _bump_seed) = Pubkey::find_program_address(seeds, &ctx.program_id);

        emit!(PDAAddressGenerated(pda));

        let airdrop_state = &mut ctx.accounts.airdrop_state;
        airdrop_state.owner = owner;
        airdrop_state.whitelist = vec![];
        Ok(())
    }

    pub fn add_to_whitelist(ctx: Context<WhitelistManagement>, address: Pubkey, amount: u64) -> Result<()> {
        let airdrop_state = &mut ctx.accounts.airdrop_state;
        require!(airdrop_state.owner == ctx.accounts.owner.key(), ErrorCode::Unauthorized); 
        if airdrop_state.whitelist.iter().any(|entry| entry.address == address) {
            return Err(ErrorCode::AlreadyExists.into());
        }

        // 创建新的 White 结构并将其添加到 addresses 中
        let new_entry = White {
            address,
            amount,
            is_airdrop: false, // 默认设置为 false
        };
        airdrop_state.whitelist.push(new_entry);
        Ok(())



    }

    pub fn remove_from_whitelist(ctx: Context<WhitelistManagement>, address: Pubkey) -> Result<()> {
        let airdrop_state = &mut ctx.accounts.airdrop_state;
        require!(airdrop_state.owner == ctx.accounts.owner.key(), ErrorCode::Unauthorized);
        if let Some(index) = airdrop_state.whitelist
            .iter()
            .position(|entry| entry.address == address)
        {
            // 如果找到了匹配的索引，则删除该元素
            airdrop_state.whitelist.remove(index);
        } else {
            return Err(ErrorCode::NotFound.into());
        }
        Ok(())
    }

    pub fn airdrop(ctx: Context<Airdrop>) -> Result<()> {
        let airdrop_state = &mut ctx.accounts.airdrop_state;
        let mut found = false;
        let mut amount = 0;

        for entry in airdrop_state.whitelist.iter_mut() {
            if entry.address == ctx.accounts.from.key() {
                if entry.is_airdrop {
                    return Err(ErrorCode::AlreadyAirdropped.into());
                }
                entry.is_airdrop = true;
                amount = entry.amount; // 获取白名单中指定的数量进行空投

                found = true;
                break;
            }
        }

        if !found {
            return Err(ErrorCode::NotInWhitelist.into());
        }

        token::transfer(ctx.accounts.into_transfer_context(), amount)?;
        Ok(())

        
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 10240)]
    pub airdrop_state: Account<'info, AirdropState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WhitelistManagement<'info> {
    #[account(mut, has_one = owner)]
    pub airdrop_state: Account<'info, AirdropState>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct Airdrop<'info> {
    #[account(mut)]
    pub airdrop_state: Account<'info, AirdropState>,
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, token::Token>,
}

#[account]
pub struct AirdropState {
    pub owner: Pubkey,
    pub whitelist: Vec<White>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct White {
    pub address: Pubkey,
    pub amount: u64,
    pub is_airdrop: bool,
}



impl<'info> Airdrop<'info> {
    fn into_transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.from.to_account_info().clone(),
            to: self.recipient.to_account_info().clone(),
            authority: self.authority.to_account_info().clone(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Account already exists in whitelist")]
    AlreadyExists,
    #[msg("Account not found in whitelist")]
    NotFound,
    #[msg("Account not in whitelist")]
    NotInWhitelist,
    #[msg("Already airdropped")]
    AlreadyAirdropped,
}
