use crate::vault::mul_div_floor;

mod vault;
mod error;

fn main() {
    let mut vault = vault::Vault::new();
    vault.deposit("alice".to_string(), 100).unwrap();
    println!("After Alice deposit: {:?}", vault);

    let redeemed = vault.redeem("alice".to_string(), 50).unwrap();
    println!("Alice redeemed 50 shares, got {} assets", redeemed);
    println!("After redemption: {:?}", vault);
}