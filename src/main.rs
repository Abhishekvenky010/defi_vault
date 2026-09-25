use crate::vault::{mul_div_floor, mul_div_ceil, deposit_with_slippage, redeem_with_slippage};


mod vault;
mod error;

fn main() {
    let mut vault = vault::Vault::new();
    vault.deposit("alice".to_string(), 100).unwrap();
    println!("After Alice deposit: {:?}", vault);

    let redeemed = vault.redeem("alice".to_string(), 50).unwrap();
    println!("Alice redeemed 50 shares, got {} assets", redeemed);
    println!("After redemption: {:?}", vault);
       // Safe ceiling division
    println!("Ceil(10*3/7) = {:?}", mul_div_ceil(10, 3, 7)); // 5

    // Deposit with slippage
    match deposit_with_slippage(49, 500, 20, 2) {
        Ok(shares) => println!("Minted shares: {}", shares),
        Err(e) => println!("Deposit failed: {}", e),
    }

    // Redeem with slippage
    match redeem_with_slippage(4, 23, 13, 7) {
        Ok(assets) => println!("Redeemed assets: {}", assets),
        Err(e) => println!("Redeem failed: {}", e),
    }
}