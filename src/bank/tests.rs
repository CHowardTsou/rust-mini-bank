use super::*;
use proptest::prelude::*;
use std::collections::HashSet;

#[cfg(test)]
#[derive(Debug, Clone)]
enum Operation {
    Deposit { account: u32, amount: u64 },
    Withdraw { account: u32, amount: u64 },
    Transfer { from: u32, to: u32, amount: u64 },
}
#[test]
fn test_create_accounts() {
    let mut bank = Bank::new();

    let alice_id = bank.create_account();
    let bob_id = bank.create_account();
    let charlie_id = bank.create_account();

    // Verify returned IDs are 0, 1, 2
    assert_eq!(alice_id, 0);
    assert_eq!(bob_id, 1);
    assert_eq!(charlie_id, 2);
    assert_eq!(bank.accounts[0].id, alice_id);
    assert_eq!(bank.accounts[1].id, bob_id);
    assert_eq!(bank.accounts[2].id, charlie_id);
    // Verify bank has 3 accounts
    assert_eq!(bank.accounts.len(), 3);
    // Verify all initial balances are 0
    assert_eq!(bank.accounts[0].balance, 0);
    assert_eq!(bank.accounts[1].balance, 0);
    assert_eq!(bank.accounts[2].balance, 0);

    // Verify next_id is 3
    assert_eq!(bank.next_id, 3);
}

#[test]
fn test_deposit() {
    let mut bank = Bank::new();
    let alice = bank.create_account();

    // 1. Deposit 100 tokens
    assert_eq!(bank.deposit(alice, 100), Ok(()));

    // Verify Alice's balance is 100
    assert_eq!(bank.accounts[0].balance, 100);
    // 2. Deposit another 50
    // Verify balance is 150
    assert_eq!(bank.deposit(alice, 50), Ok(()));
    assert_eq!(bank.accounts[0].balance, 150);

    // 3. Deposit zero
    // Verify balance is unchanged
    assert_eq!(bank.deposit(alice, 0), Ok(()));
    assert_eq!(bank.accounts[0].balance, 150);
    // 4. Deposit into nonexistent account
    // Verify AccountNotFound
    assert_eq!(bank.deposit(999, 999), Err(BankError::AccountNotFound));

    // 5. Test balance overflow
    // Hint: How can you bring Alice's balance
    // close to u64::MAX?

    // Verify BalanceOverflow
    // Verify balance remains unchanged
    assert_eq!(
        bank.deposit(alice, u64::MAX),
        Err(BankError::BalanceOverflow)
    );
    assert_eq!(bank.accounts[0].balance, 150);
}

#[test]
fn test_withdraw() {
    let mut bank = Bank::new();
    let alice = bank.create_account();

    bank.deposit(alice, 150).unwrap();

    // 1. Withdraw 50
    // Verify balance becomes 100
    assert_eq!(bank.withdraw(alice, 50), Ok(()));
    assert_eq!(bank.accounts[0].balance, 100);
    // 2. Withdraw 0
    // Verify balance remains 100
    assert_eq!(bank.withdraw(alice, 0), Ok(()));
    assert_eq!(bank.accounts[0].balance, 100);
    // 3. Try withdrawing 101
    // Verify InsufficientBalance
    // Verify balance remains 100
    assert_eq!(
        bank.withdraw(alice, 101),
        Err(BankError::InsufficientBalance)
    );
    assert_eq!(bank.accounts[0].balance, 100);
    // 4. Try withdrawing from ID 999
    // Verify AccountNotFound
    // Verify Alice's balance remains 100
    assert_eq!(bank.withdraw(999, 50), Err(BankError::AccountNotFound));
    assert_eq!(bank.accounts[0].balance, 100);
    // 5. Withdraw remaining 100
    // Verify balance becomes 0
    assert_eq!(bank.withdraw(alice, 100), Ok(()));
    assert_eq!(bank.accounts[0].balance, 0);
}

#[test]
fn test_transfer() {
    let mut bank = Bank::new();

    let alice = bank.create_account();
    let bob = bank.create_account();

    bank.deposit(alice, 100).unwrap();
    bank.deposit(bob, 50).unwrap();

    // Alice → Bob
    assert_eq!(bank.transfer(alice, bob, 30), Ok(()));
    assert_eq!(bank.accounts[0].balance, 70);
    assert_eq!(bank.accounts[1].balance, 80);

    // Bob → Alice
    assert_eq!(bank.transfer(bob, alice, 20), Ok(()));
    assert_eq!(bank.accounts[0].balance, 90);
    assert_eq!(bank.accounts[1].balance, 60);

    // Total must remain unchanged.
    assert_eq!(bank.accounts[0].balance + bank.accounts[1].balance, 150);

    // insufficient funds
    assert_eq!(
        bank.transfer(alice, bob, 91),
        Err(BankError::InsufficientBalance)
    );
    assert_eq!(bank.accounts[0].balance, 90);
    assert_eq!(bank.accounts[1].balance, 60);

    // missing accounts
    assert_eq!(
        bank.transfer(alice, 999, 10),
        Err(BankError::AccountNotFound)
    );
    assert_eq!(bank.transfer(999, bob, 10), Err(BankError::AccountNotFound));
    assert_eq!(bank.accounts[0].balance, 90);
    assert_eq!(bank.accounts[1].balance, 60);

    // zero-value transfers
    assert_eq!(bank.transfer(alice, bob, 0), Ok(()));
    assert_eq!(bank.accounts[0].balance, 90);
    assert_eq!(bank.accounts[1].balance, 60);

    // self-transfers
    assert_eq!(bank.transfer(alice, alice, 10), Ok(()));
    assert_eq!(bank.accounts[0].balance, 90);
    assert_eq!(bank.accounts[1].balance, 60);
}
#[test]
fn test_transfer_near_max_supply() {
    let mut bank = Bank::new();

    let alice = bank.create_account();
    let bob = bank.create_account();

    bank.deposit(alice, u64::MAX - 50).unwrap();
    bank.deposit(bob, 50).unwrap();

    assert_eq!(bank.total_supply, u64::MAX);

    assert_eq!(bank.transfer(alice, bob, 10), Ok(()));

    assert_eq!(bank.balance_of(alice), Ok(u64::MAX - 60));
    assert_eq!(bank.balance_of(bob), Ok(60));
    assert_eq!(bank.total_supply, u64::MAX);

    assert_total_supply(&bank);
}
#[test]
fn test_balance_of_account() {
    let mut bank = Bank::new();

    let alice = bank.create_account();
    let bob = bank.create_account();

    assert_eq!(bank.deposit(alice, 100), Ok(()));

    assert_eq!(bank.balance_of(alice), Ok(100));
    assert_eq!(bank.balance_of(999), Err(BankError::AccountNotFound));
    assert_eq!(bank.balance_of(bob), Ok(0));
}

#[test]
fn test_total_supply() {
    let mut bank = Bank::new();

    let alice = bank.create_account();
    let bob = bank.create_account();
    assert_total_supply(&bank);

    assert_eq!(bank.deposit(alice, 100), Ok(()));
    assert_eq!(bank.total_supply, 100);
    assert_total_supply(&bank);

    assert_eq!(bank.deposit(bob, 50), Ok(()));
    assert_eq!(bank.total_supply, 150);
    assert_total_supply(&bank);

    assert_eq!(bank.transfer(alice, bob, 30), Ok(()));
    assert_eq!(bank.total_supply, 150);
    assert_eq!(bank.balance_of(alice), Ok(70));
    assert_eq!(bank.balance_of(bob), Ok(80));
    assert_total_supply(&bank);

    assert_eq!(bank.withdraw(bob, 20), Ok(()));
    assert_eq!(bank.total_supply, 130);
    assert_total_supply(&bank);

    assert_eq!(
        bank.withdraw(alice, 100),
        Err(BankError::InsufficientBalance)
    );

    assert_eq!(bank.balance_of(alice), Ok(70));
    assert_eq!(bank.total_supply, 130);
    assert_total_supply(&bank);
}

#[test]
fn test_deposit_supply_overflow_preserves_state() {
    let mut bank = Bank::new();

    let alice = bank.create_account();
    let bob = bank.create_account();

    // Create a valid bank with maximum total supply.
    assert_eq!(bank.deposit(alice, u64::MAX - 10), Ok(()));
    assert_eq!(bank.deposit(bob, 10), Ok(()));

    assert_eq!(bank.total_supply, u64::MAX);
    assert_total_supply(&bank);

    // Alice's balance has room for another token,
    // but total_supply does not.
    assert_eq!(bank.deposit(alice, 1), Err(BankError::BalanceOverflow));

    // No state changes are permitted.
    assert_eq!(bank.balance_of(alice), Ok(u64::MAX - 10));
    assert_eq!(bank.balance_of(bob), Ok(10));
    assert_eq!(bank.total_supply, u64::MAX);
    assert_total_supply(&bank);
}

#[test]
fn test_bank_invariants() {
    let mut bank = Bank::new();

    let alice = bank.create_account();
    let bob = bank.create_account();
    let charlie = bank.create_account();

    assert_bank_invariants(&bank);

    // 1. Deposit 100 to Alice.
    // 2. Deposit 200 to Bob.
    // 3. Transfer 40 from Alice to Charlie.
    // 4. Transfer 75 from Bob to Alice.
    // 5. Withdraw 20 from Charlie.
    // 6. Attempt to withdraw 500 from Alice.
    assert_eq!(bank.deposit(alice, 100), Ok(()));
    assert_bank_invariants(&bank);

    assert_eq!(bank.deposit(bob, 200), Ok(()));
    assert_bank_invariants(&bank);

    assert_eq!(bank.transfer(alice, charlie, 40), Ok(()));
    assert_eq!(bank.balance_of(alice), Ok(60));
    assert_eq!(bank.balance_of(charlie), Ok(40));
    assert_bank_invariants(&bank);

    assert_eq!(bank.transfer(bob, alice, 75), Ok(()));
    assert_eq!(bank.balance_of(bob), Ok(125));
    assert_eq!(bank.balance_of(alice), Ok(135));
    assert_bank_invariants(&bank);

    assert_eq!(bank.withdraw(charlie, 20), Ok(()));
    assert_bank_invariants(&bank);

    assert_eq!(
        bank.withdraw(alice, 500),
        Err(BankError::InsufficientBalance)
    );
    assert_eq!(bank.balance_of(alice), Ok(135));
    assert_eq!(bank.balance_of(bob), Ok(125));
    assert_eq!(bank.balance_of(charlie), Ok(20));

    assert_bank_invariants(&bank);
    assert_eq!(bank.total_supply, 280);
    // After EVERY operation:
    // - Check the expected Ok/Err result.
    // - Call assert_bank_invariants(&bank).
    //
    // For the failed withdrawal:
    // - Verify all balances remain unchanged.

    // Finally, verify total_supply == 280.
}

#[test]
fn test_many_bank_operations() {
    let mut bank = Bank::new();

    let alice = bank.create_account();
    let bob = bank.create_account();

    bank.deposit(alice, 1000).unwrap();
    bank.deposit(bob, 1000).unwrap();

    assert_bank_invariants(&bank);

    for i in 0..100 {
        let amount = (i % 20 + 1) as u64;
        // If i is even, transfer from Alice to Bob.
        // If i is odd, transfer from Bob to Alice.
        if i % 2 == 0 {
            bank.transfer(alice, bob, amount).unwrap();
            println!(
                "Step {} | Alice: {} | Bob: {} | Supply: {}",
                i,
                bank.balance_of(alice).unwrap(),
                bank.balance_of(bob).unwrap(),
                bank.total_supply
            );
            assert_bank_invariants(&bank);
            assert_eq!(
                bank.balance_of(alice).unwrap() as u128 + bank.balance_of(bob).unwrap() as u128,
                2000
            );
        } else {
            bank.transfer(bob, alice, amount).unwrap();
            println!(
                "Step {} | Alice: {} | Bob: {} | Supply: {}",
                i,
                bank.balance_of(alice).unwrap(),
                bank.balance_of(bob).unwrap(),
                bank.total_supply
            );
            assert_bank_invariants(&bank);
            assert_eq!(
                bank.balance_of(alice).unwrap() as u128 + bank.balance_of(bob).unwrap() as u128,
                2000
            );
        }
        // Check the result of the transfer.
        // Check the bank invariants after each operation.
    }
    assert_eq!(bank.total_supply, 2000);
    // Verify total_supply is still 2000.
}

proptest! {
    #[test]
    fn prop_transfer_preserves_supply(
        amount in 0u64..=100
    ) {
        let mut bank = Bank::new();

        let alice = bank.create_account();
        let bob = bank.create_account();

        bank.deposit(alice, 100).unwrap();
        bank.deposit(bob, 50).unwrap();

        let supply_before = bank.total_supply;

        // Every generated amount is valid.
        prop_assert_eq!(
            bank.transfer(alice, bob, amount),
            Ok(())
        );

        // Check exact balances.
        prop_assert_eq!(
            bank.balance_of(alice),
            Ok(100 - amount)
        );

        prop_assert_eq!(
            bank.balance_of(bob),
            Ok(50 + amount)
        );

        // Supply must never change during transfers.
        prop_assert_eq!(
            bank.total_supply,
            supply_before
        );

        assert_bank_invariants(&bank);
    }
}

proptest! {
    #[test]
    fn prop_failed_transfer_preserves_state(
        amount in 101u64..=1000
    ) {
        let mut bank = Bank::new();

        let alice = bank.create_account();
        let bob = bank.create_account();

        bank.deposit(alice, 100).unwrap();
        bank.deposit(bob, 50).unwrap();

        // Save Alice's balance, Bob's balance,
        // and total_supply before the transfer.
        let before_alice_balance = bank.balance_of(alice).unwrap();
        let before_bob_balance = bank.balance_of(bob).unwrap();
        let before_total_supply_balance = bank.total_supply;

        // Transfer `amount` from Alice to Bob.
        // Expect Err(InsufficientBalance).
        prop_assert_eq!(bank.transfer(alice, bob, amount), Err(BankError::InsufficientBalance));

        // Verify both balances remain unchanged.
        prop_assert_eq!(bank.balance_of(alice).unwrap(), before_alice_balance);
        prop_assert_eq!(bank.balance_of(bob).unwrap(), before_bob_balance);

        // Verify total_supply remains unchanged.
        prop_assert_eq!(bank.total_supply, before_total_supply_balance);

        // Check assert_bank_invariants(&bank).
        assert_bank_invariants(&bank);
    }
}

proptest! {
    #[test]
    fn prop_multiple_transfers(
        amounts in prop::collection::vec(0u64..=200, 1..50)
    ) {
        let mut bank = Bank::new();

        let alice = bank.create_account();
        let bob = bank.create_account();

        bank.deposit(alice, 500).unwrap();
        bank.deposit(bob, 500).unwrap();


        for (i, &amount) in amounts.iter().enumerate() {
            let (from, to) = if i % 2 == 0 {
                (alice, bob)
            } else {
                (bob, alice)
            };

            let sender_before = bank.balance_of(from).unwrap();
            let receiver_before = bank.balance_of(to).unwrap();

            let result = bank.transfer(from, to, amount);

            if amount <= sender_before {
                prop_assert_eq!(result, Ok(()));

                prop_assert_eq!(
                    bank.balance_of(from),
                    Ok(sender_before - amount)
                );
                prop_assert_eq!(
                    bank.balance_of(to),
                    Ok(receiver_before + amount)
                );
            } else {
                prop_assert_eq!(
                    result,
                    Err(BankError::InsufficientBalance)
                );

                prop_assert_eq!(
                    bank.balance_of(from),
                    Ok(sender_before)
                );
                prop_assert_eq!(
                    bank.balance_of(to),
                    Ok(receiver_before)
                );
            }

            prop_assert_eq!(bank.total_supply, 1000);
            assert_bank_invariants(&bank);
        }
    }
}

fn assert_total_supply(bank: &Bank) {
    let sum: u128 = bank
        .accounts
        .iter()
        .map(|account| account.balance as u128)
        .sum();

    assert_eq!(
        bank.total_supply as u128, sum,
        "Total supply invariant violated!"
    );
}

fn assert_bank_invariants(bank: &Bank) {
    let mut seen = HashSet::new();
    let mut sum: u128 = 0;

    for account in &bank.accounts {
        // 1. Verify IDs are unique.
        assert!(
            seen.insert(account.id),
            "Duplicate account ID: {}",
            account.id
        );

        // 2. Verify IDs are within the issued range.
        assert!(account.id < bank.next_id);

        // 3. Sum all account balances.
        sum += account.balance as u128;
    }

    // 4. Verify total supply matches.
    assert_eq!(
        sum, bank.total_supply as u128,
        "Total supply invariant violated"
    );
}

fn amount_strategy() -> impl Strategy<Value = u64> {
    prop_oneof![
        0u64..=200,
        Just(u64::MAX),
        Just(u64::MAX - 1),
        Just(u64::MAX / 2),
    ]
}
fn operation_strategy() -> impl Strategy<Value = Operation> {
    prop_oneof![
        (0u32..5, amount_strategy())
            .prop_map(|(account, amount)| { Operation::Deposit { account, amount } }),
        (0u32..5, amount_strategy())
            .prop_map(|(account, amount)| { Operation::Withdraw { account, amount } }),
        (0u32..5, 0u32..2, amount_strategy())
            .prop_map(|(from, to, amount)| { Operation::Transfer { from, to, amount } }),
    ]
}

proptest! {
    #[test]
    fn prop_mixed_bank_operations(
        operations in prop::collection::vec(
            operation_strategy(),
            1..50
        )
    ) {
        let mut bank = Bank::new();

        let alice = bank.create_account();
        let bob = bank.create_account();




        assert_bank_invariants(&bank);

        for operation in operations {
            match operation {
                Operation::Deposit { account, amount } => {
                    let before_balance = bank.balance_of(account);
                    let before_supply = bank.total_supply;

                    let before_alice_balance = bank.balance_of(alice).unwrap();
                    let before_bob_balance = bank.balance_of(bob).unwrap();

                    match before_balance {
                        Err(BankError::AccountNotFound) => {
                            // TODO:
                            // 1. Expect deposit() to return AccountNotFound.
                            // 2. Verify total_supply remains unchanged.
                            // 3. Verify Alice and Bob's balances are unchanged.
                            prop_assert_eq!(bank.deposit(account, amount), Err(BankError::AccountNotFound));
                            prop_assert_eq!(bank.total_supply, before_supply);
                            prop_assert_eq!(bank.balance_of(alice), Ok(before_alice_balance));
                            prop_assert_eq!(bank.balance_of(bob), Ok(before_bob_balance));
                        }

                        Ok(balance) => {
                            let expected_balance = balance.checked_add(amount);
                            let expected_supply = before_supply.checked_add(amount);

                            // If both expected calculations are Some(...):
                            //   - Expect Ok(())
                            //   - Verify new account balance
                            //   - Verify new total supply
                            //
                            // Otherwise:
                            //   - Expect Err(BalanceOverflow)
                            //   - Verify balance and supply are unchanged
                            match (expected_balance, expected_supply) {
                                (Some(balance),Some(supply)) => {
                                    prop_assert_eq!(bank.deposit(account, amount), Ok(()));
                                    prop_assert_eq!(bank.balance_of(account), Ok(balance));
                                    prop_assert_eq!(bank.total_supply, supply);
                                } ,
                                _ => {
                                    prop_assert_eq!(bank.deposit(account,amount), Err(BankError::BalanceOverflow));
                                    prop_assert_eq!(bank.balance_of(account), Ok(balance));
                                    prop_assert_eq!(bank.total_supply, before_supply);
                                },
                            };
                        }

                        Err(error) => {
                            // Unexpected error.
                            prop_assert!(false, "Unexpected error: {:?}", error);
                        }
                    }
                }

                Operation::Withdraw { account, amount } => {
                    let before_balance = bank.balance_of(account);
                    let before_supply = bank.total_supply;

                    let before_alice_balance = bank.balance_of(alice).unwrap();
                    let before_bob_balance = bank.balance_of(bob).unwrap();

                    match before_balance {
                        Err(BankError::AccountNotFound) => {
                            // Expect AccountNotFound.
                            // Verify Alice, Bob, and supply unchanged.
                            prop_assert_eq!(bank.withdraw(account, amount), Err(BankError::AccountNotFound));
                            prop_assert_eq!(bank.total_supply, before_supply);
                            prop_assert_eq!(bank.balance_of(alice), Ok(before_alice_balance));
                            prop_assert_eq!(bank.balance_of(bob), Ok(before_bob_balance));
                        }

                        Ok(balance) => {
                            // If amount <= balance:
                            //   Expect Ok(()).
                            //   Verify account balance decreases.
                            //   Verify total supply decreases.
                            //
                            // Otherwise:
                            //   Expect InsufficientBalance.
                            //   Verify account balance and supply unchanged.

                            let expected_balance = balance.checked_sub(amount);
                            let expected_supply = before_supply.checked_sub(amount);

                            match (expected_balance, expected_supply) {
                                (Some(balance), Some(supply)) => {
                                    prop_assert_eq!(bank.withdraw(account, amount), Ok(()));
                                    prop_assert_eq!(bank.balance_of(account), Ok(balance));
                                    prop_assert_eq!(bank.total_supply, supply);
                                    if account == alice {
                                        prop_assert_eq!(
                                            bank.balance_of(bob),
                                            Ok(before_bob_balance)
                                        );
                                    } else {
                                        prop_assert_eq!(
                                            bank.balance_of(alice),
                                            Ok(before_alice_balance)
                                        );
                                    }
                                },
                                _ => {
                                    prop_assert_eq!(bank.withdraw(account, amount), Err(BankError::InsufficientBalance));
                                    prop_assert_eq!(bank.balance_of(account), Ok(balance));
                                    prop_assert_eq!(bank.total_supply, before_supply);
                                }

                            }
                        }

                        Err(error) => {
                            prop_assert!(
                                false,
                                "Unexpected error: {:?}",
                                error
                            );
                        }
                    }

                    // 1. Calculate expected_balance using checked_sub().
                    // 2. Calculate expected_supply using checked_sub().
                    // 3. Determine the expected success or error.
                    // 4. Call bank.withdraw().
                    // 5. Verify actual balance and supply.
                    // 6. Verify unchanged state if it fails.
                }

                Operation::Transfer { from, to, amount } => {
                let sender_before = bank.balance_of(from);
                let receiver_before = bank.balance_of(to);

                let alice_before = bank.balance_of(alice).unwrap();
                let bob_before = bank.balance_of(bob).unwrap();
                let supply_before = bank.total_supply;

                match (sender_before, receiver_before) {
                    // Both accounts exist.
                    (Ok(sender_balance), Ok(receiver_balance)) => {
                        // 1. Handle self-transfer.
                        // 2. Handle insufficient funds.
                        // 3. Handle successful transfer.
                        if from == to {
                            prop_assert_eq!(bank.transfer(from, to, amount), Ok(()));
                            prop_assert_eq!(bank.balance_of(from), Ok(sender_balance));

                        } else if from != to && amount <= sender_balance{
                            // If from != to and amount <= sender_before:
                            // Expect Ok(()).
                            // Verify sender loses amount.
                            // Verify receiver gains amount.
                            prop_assert_eq!(bank.transfer(from, to, amount), Ok(()));
                            prop_assert_eq!(bank.balance_of(from), Ok(sender_balance - amount));
                            prop_assert_eq!(bank.balance_of(to), Ok(receiver_balance + amount));

                        } else {
                            // Otherwise:
                            // Expect Err(InsufficientBalance).
                            // Verify both balances remain unchanged.
                            prop_assert_eq!(bank.transfer(from, to, amount), Err(BankError::InsufficientBalance));
                            prop_assert_eq!(bank.balance_of(from), Ok(sender_balance));
                            prop_assert_eq!(bank.balance_of(to), Ok(receiver_balance));
                        }
                        // Verify total_supply == supply_before.
                        prop_assert_eq!(bank.total_supply, supply_before);
                    }

                    // At least one account does not exist.
                    _ => {
                        // Expect AccountNotFound.
                        // Verify Alice and Bob unchanged.
                        // Verify total supply unchanged.
                        prop_assert_eq!(bank.transfer(from, to, amount), Err(BankError::AccountNotFound));
                        prop_assert_eq!(bank.balance_of(alice), Ok(alice_before));
                        prop_assert_eq!(bank.balance_of(bob), Ok(bob_before));
                        prop_assert_eq!(bank.total_supply, supply_before);
                    }
                }

            }
            }

            assert_bank_invariants(&bank);
        }
    }
}
