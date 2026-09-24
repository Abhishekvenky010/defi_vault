use std::collections::HashMap;
use crate::error::AccountingError;
type UserId = String;

#[derive(Debug)]
pub struct Vault{
    pub total_assets: u64,
    pub total_shares: u64,
    pub balances: HashMap<UserId, u64>,
}
impl Vault{
    pub fn new()->Self{
       Self{
            total_assets: 0,
            total_shares: 0,
            balances: HashMap::new(),
       }
    }
    pub fn deposit(&mut self, user: UserId,amount: u64)->Result<(),AccountingError>{
        let shares_to_mint = if self.total_shares == 0 || self.total_assets == 0 {
            amount
        } else {
            mul_div_floor(amount, self.total_shares, self.total_assets)?
        };
        self.total_assets += amount;
        self.total_shares += shares_to_mint;
        *self.balances.entry(user).or_insert(0) += shares_to_mint;

        Ok(())
    }
    
    pub fn redeem(&mut self, user: UserId, shares: u64) -> Result<u64, AccountingError> {
        let user_balance = self.balances.get(&user).copied().unwrap_or(0);
        if shares > user_balance {
            return Err(AccountingError::InsufficientShares);
        }

        let assets_out = mul_div_floor(shares, self.total_assets, self.total_shares)?;
        self.total_assets -= assets_out;
        self.total_shares -= shares;
        *self.balances.entry(user).or_insert(0) -= shares;

        Ok(assets_out)
    
}

}
pub fn mul_div_floor(a: u64, b: u64, c: u64) -> Result<u64, AccountingError> {
    if c == 0 {
        return Err(AccountingError::DivisionByZero);
    }
    let product = (a as u128) * (b as u128);
    let result = product / (c as u128);
    u64::try_from(result).map_err(|_| AccountingError::Overflow)
}
