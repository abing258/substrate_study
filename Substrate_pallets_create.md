# 
```rust lib.rs

#![cfg_attr(not(feature = "std"), no_std)]

/// 方便让别的模块调用
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	/// 模块配置接口
	#[pallet::config]
	pub trait Config: frame_system::Config {
	}

	#[pallet::pallet]
	//定义自己所需的存储项所需的宏
	#[pallet::generate_store(pub(super) trait Store)]
	//定义模块所需的结构体
	pub struct Pallet<T>(_);

	#[pallet::event]
	//generate_deposit 生成了一个帮助方法 deposit_event
	//deposit_event 方便调用生成事件的宏
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
	}

	#[pallet::hooks]
	//定义保留函数
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
	}

	impl<T: Config> Pallet<T> {
	}
}



```

## pallet 功能复用
应用基本的软件设计的最佳实践  
1. 模块之间做到尽量的解除耦合  
2. 面向接口的编程  
3. 模块可以很好的被复用  
4. pallet 应该是可以任意组合的  