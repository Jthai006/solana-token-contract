use anchor_lang::prelude::*;
use anchor_spl::token::Token;

declare_id!("4VuB55bdu6fjN2jFW3cAAd4EbapnF2q7Z51v9d85pKXp");

#[program]
pub mod token_contract {
    use anchor_spl::token::{self, MintTo, Transfer};

    use super::*;

    pub fn mint_token(ctx: Context<MintToken>) -> Result<()> {
        let cpi_accounts: MintTo<'_> = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };

        let cpi_program: AccountInfo<'_> = ctx.accounts.token_program.to_account_info(); 
        // Create the CpiContext we need for the request
        let cpi_ctx: CpiContext<'_, '_, '_, '_, MintTo<'_>> = CpiContext::new(cpi_program, cpi_accounts);
        
        // Execute Anchors mint helper function to mint tokens
        token::mint_to(cpi_ctx, 10)?;
        Ok(())
    }

    pub fn transfer_token(ctx: Context<TransferToken>) -> Result<()> {
        // Create the Transfer struct for our context
        let transfer_instruction: token::Transfer<'_> = Transfer{
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.from_authority.to_account_info(),
        };
         
        let cpi_program: AccountInfo<'_> = ctx.accounts.token_program.to_account_info();
        // Create the Context for our Transfer request
        let cpi_ctx: CpiContext<'_, '_, '_, '_, token::Transfer<'_>> = CpiContext::new(cpi_program, transfer_instruction);

        // Execute anchor's helper function to transfer tokens
        anchor_spl::token::transfer(cpi_ctx, 5)?;
 
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintToken<'info> {
    /// Token we want to make more copies of
    /// CHECK: This is the token that we want to mint
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    /// Program for our CPI context
    pub token_program: Program<'info, Token>,
    /// CHECK: This is the token account that we want to mint tokens to
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    /// CHECK: the authority of the mint account
    #[account(mut)]
    pub authority: AccountInfo<'info>, 
}

#[derive(Accounts)]
pub struct TransferToken<'info> {
    /// Program for our CPI context
    pub token_program: Program<'info, Token>,
    /// CHECK: This is the token account that we want to transfer tokens from
    #[account(mut)]
    pub from: UncheckedAccount<'info>,
    /// CHECK: This is the token account that we want to transfer tokens to
    #[account(mut)]
    pub to: AccountInfo<'info>,
    /// CHECK: the authority of the from account
    #[account(mut)]
    pub from_authority: Signer<'info>, 
}

#[derive(Accounts)]
pub struct Initialize {}
