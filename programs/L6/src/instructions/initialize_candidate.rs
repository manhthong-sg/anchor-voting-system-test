use crate::schema::*;
use anchor_lang::prelude::*;
use anchor_spl::{associated_token, token};

#[derive(Accounts)]
pub struct InitializeCandidate<'info> {
  // #xxx la cac marco
  #[account(mut)]
  pub authority: Signer<'info>,

  // khi tao bien account thi can thue vung nho tren blockchain nen can phai co marco account init, payer,... 
  #[account(
    init,
    payer = authority,
    space = Candidate::SIZE,
  )]
  pub candidate: Account<'info, Candidate>,

  // account PDA (20-22)
  #[account(
    seeds = [b"treasurer".as_ref(), &candidate.key().to_bytes()],
    bump //255
  )]
  /// CHECK: Just a pure account
  pub treasurer: AccountInfo<'info>,

  pub mint: Box<Account<'info, token::Mint>>, //khai bao account trong heap thay vi stack
  #[account(
    init,
    payer = authority,
    associated_token::mint = mint,
    associated_token::authority = treasurer
  )]
  pub candidate_token_account: Account<'info, token::TokenAccount>,
  // System Program Address
  pub system_program: Program<'info, System>,
  pub token_program: Program<'info, token::Token>,
  pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
  pub rent: Sysvar<'info, Rent>,
}

pub fn exec(ctx: Context<InitializeCandidate>, start_date: i64, end_date: i64) -> Result<()> {
  let candidate = &mut ctx.accounts.candidate;
  candidate.start_date = start_date;
  candidate.end_date = end_date;
  candidate.amount = 0;
  candidate.mint = ctx.accounts.mint.key();
  Ok(())
}
