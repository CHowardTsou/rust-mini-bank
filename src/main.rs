mod bank;
use bank::Bank;
fn main() {
    println!("Mini Bank project!");
    let mut bank = Bank::new();

    let alice = bank.create_account();
    let bob = bank.create_account();

    bank.deposit(alice, 100).unwrap();
    bank.transfer(alice, bob, 30).unwrap();
    bank.withdraw(bob, 10).unwrap();

    println!("Alice: {:?}", bank.balance_of(alice));
    println!("Bob: {:?}", bank.balance_of(bob));
}
