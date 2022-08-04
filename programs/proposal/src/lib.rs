use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token,
    token::{Mint, MintTo, Token, TokenAccount, Transfer},
};
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
mod proposals {
    use super::*;
    pub fn create(ctx: Context<Create>, _ended_at: i64) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        proposal.fee = 50;

        let proposal_info = &mut ctx.accounts.proposal_info;

        if _ended_at > ctx.accounts.clock.unix_timestamp {
            return Err(ErrorCode);
        }

        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.sender_token_x.to_account_info(),
                to: ctx.accounts.vault_x.to_account_info(),
                authority: ctx.accounts.sender.to_account_info(),
            },
        );
        token::transfer(transfer_ctx, proposal.fee)?;

        proposal_info.created_at = ctx.accounts.clock.unix_timestamp;
        proposal_info.ended_at = _ended_at;

        Ok(())
    }

    pub fn close(ctx: Context<Create>) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;

        let proposal_info = &mut ctx.accounts.proposal_info;

        proposal_info.ended_at = ctx.accounts.clock.unix_timestamp;

        Ok(())
    }

    pub fn set_fee(ctx: Context<Create>, _fee: u64) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        proposal.fee = _fee;

        Ok(())
    }


}

#[derive(Accounts)]
pub struct Create<'info> {
    pub token_x: Account<'info, Mint>,
    #[account(init, payer = user, space = 8 + 8)]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub sender: Signer<'info>,
    pub proposal_info: Account<'info, ProposalInfo>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub sender_token_x: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    #[account(mut, seeds=[b"vault", token_x.key().as_ref()], bump)]
    pub vault_x: Account<'info, TokenAccount>,
    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Proposal {
    pub fee: u64,
}

#[account]
#[derive(Default)]
pub struct ProposalInfo {
    pub created_at: i64,
    pub ended_at: i64,
    pub positive_votes: u64,
    pub negative_votes: u64,
}

#[error]
pub enum ErrorCode {
    #[msg("Incorrect param.")]
    IncorrectParam,
}
