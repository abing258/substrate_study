#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod incrementer {
    /// 存储结构上添加特征
    use ink_storage::traits::SpreadAllocate;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    /// 指定映射键和映射到它的值
    #[derive(SpreadAllocate)]
    pub struct Incrementer {
        // Storage Declaration
        value: i32,
        my_value: ink_storage::Mapping<AccountId, i32>,
    }

    // #[ink(storage)]
    // pub struct MyContract {
    //     // Store a bool
    //     my_bool: bool,
    //     // Store a number
    //     my_number: u32,
    //     // Store some AccountId
    //     my_account: AccountId,
    //     // Store some Balance
    //     my_balance: Balance,
    // }

    impl Incrementer {
        /// self.env().caller() 获取合约调用者
        /// 如果用户访问一个合约然后调用后续合约，self.env().caller()则第二个合约中的地址是第一个合约的地址
        //构造函数
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            /// ink_lang::utils::initalize_contract 函数来初始化合约的映射
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.value = init_value;
                let caller = Self::env().caller();
                contract.my_value.insert(&caller, &0);
            })
        }

        // 默认构造函数
        #[ink(constructor)]
        pub fn default() -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.value = Default::default();
            })
        }

        /// 获取存储的值
        //只从合约存储中读取&self
        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.value
        }

        /// 修改存储值
        //必须显式标记value为可变变量
        #[ink(message)]
        pub fn inc(&mut self, by: i32) {
            self.value += by;
        }

        /// 读取并返回给合约调用者
        #[ink(message)]
        pub fn get_mine(&self) -> i32 {
            self.my_value.get(&self.env().caller()).unwrap_or_default()
        }

        /// 允许合约调用者获取my_value存储项并将增量value插入到映射中
        #[ink(message)]
        pub fn inc_mine(&mut self, by: i32) {
            let caller = self.env().caller();
            let my_value = self.get_mine();
            self.my_value.insert(caller, &(my_value + by));
        }

        /// 允许合约调用者my_value从存储中清除存储项
        #[ink(message)]
        pub fn remove_mine(&mut self) {
            self.my_value.remove(&self.env().caller())
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn default_works() {
            // Test Your Contract
            let contract = Incrementer::default();
            assert_eq!(contract.get(), 0);
        }

        #[ink::test]
        fn is_inc_works() {
            let mut incrementer = Incrementer::new(42);
            assert_eq!(incrementer.get(), 42);
            incrementer.inc(5);
            assert_eq!(incrementer.get(), 47);
            incrementer.inc(-50);
            assert_eq!(incrementer.get(), -3);
        }

        #[ink::test]
        fn my_value_works() {
            let contract = Incrementer::new(11);
            assert_eq!(contract.get(), 11);
            assert_eq!(contract.get_mine(), 0);
        }

        #[ink::test]
        fn inc_mine_works() {
            let mut contract = Incrementer::new(11);
            assert_eq!(contract.get_mine(), 0);
            contract.inc_mine(5);
            assert_eq!(contract.get_mine(), 5);
            contract.inc_mine(5);
            assert_eq!(contract.get_mine(), 10);
        }

        #[ink::test]
        fn remove_mine_works() {
            let mut contract = Incrementer::new(11);
            assert_eq!(contract.get_mine(), 0);
            contract.inc_mine(5);
            assert_eq!(contract.get_mine(), 5);
            contract.remove_mine();
            assert_eq!(contract.get_mine(), 0);
        }
    }
}


