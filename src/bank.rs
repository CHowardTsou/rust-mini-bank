struct Account {
    id: u32,
    balance: u64,
}

pub struct Bank {
    accounts: Vec<Account>,
    next_id: u32,
    total_supply: u64,
}

#[derive(Debug, PartialEq)]
pub enum BankError {
    AccountNotFound,
    BalanceOverflow,
    InsufficientBalance,
    SupplyUnderflow,
}

impl Bank {
    pub fn new() -> Self {
        // Initialize an empty bank
        Self {
            accounts: Vec::new(),
            next_id: 0,
            total_supply: 0,
        }
    }

    pub fn create_account(&mut self) -> u32 {
        // Create an account with:
        // - A unique ID
        // - Balance = 0
        //
        // Return its ID
        let id = self.next_id;
        let next_new_id = self.next_id.checked_add(1).expect("id overflow");
        let account = Account {
            id: self.next_id,
            balance: 0,
        };
        self.accounts.push(account);
        self.next_id = next_new_id;
        id
    }

    pub fn deposit(&mut self, id: u32, amount: u64) -> Result<(), BankError> {
        let account = self
            .accounts
            .iter_mut()
            .find(|account| account.id == id)
            .ok_or(BankError::AccountNotFound)?;
        let new_balance = account
            .balance
            .checked_add(amount)
            .ok_or(BankError::BalanceOverflow)?;

        let new_total_supply = self
            .total_supply
            .checked_add(amount)
            .ok_or(BankError::BalanceOverflow)?;

        account.balance = new_balance;
        self.total_supply = new_total_supply;

        Ok(())
    }

    pub fn withdraw(&mut self, id: u32, amount: u64) -> Result<(), BankError> {
        let account = self
            .accounts
            .iter_mut()
            .find(|account| account.id == id)
            .ok_or(BankError::AccountNotFound)?;
        let new_balance = account
            .balance
            .checked_sub(amount)
            .ok_or(BankError::InsufficientBalance)?;

        let new_total_supply = self
            .total_supply
            .checked_sub(amount)
            .ok_or(BankError::SupplyUnderflow)?;
        self.total_supply = new_total_supply;

        account.balance = new_balance;

        Ok(())
    }

    pub fn transfer(&mut self, from: u32, to: u32, amount: u64) -> Result<(), BankError> {
        // 1. Find sender and recipient indices.
        let sender_position = self
            .accounts
            .iter_mut()
            .position(|a| a.id == from)
            .ok_or(BankError::AccountNotFound)?;
        let receiver_position = self
            .accounts
            .iter_mut()
            .position(|a| a.id == to)
            .ok_or(BankError::AccountNotFound)?;

        // 2. Return an error if either is missing.

        // 3. Handle self-transfers.
        if from == to {
            return Ok(());
        }
        // 4. Borrow both accounts safely.
        let split_index = sender_position.max(receiver_position);

        let (left, right) = self.accounts.split_at_mut(split_index);

        let (sender_account, receiver_account) = if sender_position < receiver_position {
            (&mut left[sender_position], &mut right[0])
        } else {
            (&mut right[0], &mut left[receiver_position])
        };

        // 5. Check sender subtraction.
        let sender_new_balance = sender_account
            .balance
            .checked_sub(amount)
            .ok_or(BankError::InsufficientBalance)?;

        // 6. Check recipient addition.
        let receiver_new_balance = receiver_account
            .balance
            .checked_add(amount)
            .ok_or(BankError::BalanceOverflow)?;

        // 7. Update both balances.
        sender_account.balance = sender_new_balance;
        receiver_account.balance = receiver_new_balance;
        Ok(())
    }

    pub fn balance_of(&self, id: u32) -> Result<u64, BankError> {
        // 1. Find the account by ID.
        // 2. Return AccountNotFound if missing.
        // 3. Return the account's balance.
        let account = self
            .accounts
            .iter()
            .find(|a| a.id == id)
            .ok_or(BankError::AccountNotFound)?;
        Ok(account.balance)
    }
}

#[cfg(test)]
mod tests;
