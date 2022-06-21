use anchor_lang::prelude::*;

#[account]
pub struct Candidate {
  pub mint: Pubkey,
  pub amount: u64,
  pub start_date: i64,
  pub end_date: i64,
}

impl Candidate {
  //declare size of candidate
  pub const SIZE: usize = 8 + 32 + 8 + 8 + 8;

  //pubkey: 32 bytes, u64, i64: 8 bytes
  //8 bytes dau la cua anchor dung de dinh nghia, dinh danh cai schema (luon luon co)
}
