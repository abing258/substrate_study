# Substrate 智能合约
**!ink** 编程语言，是一种嵌入式领域特定语言。该语言使您能够使用 Rust 编程语言编写基于 WebAssembly 的智能合约。  
该语言使用带有专用 #[ink(...)] 属性宏的标准 Rust 模式。  
这些属性宏描述了智能合约的不同部分所代表的内容，以便可以将它们转换为与 Substrate 兼容的 WebAssembly 字节码。  

## 前提
1. 更新rust环境
```bash
rustup component add rust-src --toolchain nightly

rustup target add wasm32-unknown-unknown --toolchain nightly

#已经是最新会显示一下内容
#info: component 'rust-std' for target 'wasm32-unknown-unknown' is up to date
```
2. 有一个Substrate 合约节点
```
cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node.git --tag <latest-tag> --force --locked
```
在 https://github.com/paritytech/substrate-contracts-node/tags 找最新的节点

3. 安装 WebAssembly 优化器
```bash
#macos
brew install binaryen

#Ubuntu 或 Debian
sudo apt install binaryen

```
4. 安装 cargo-contract
```bash
cargo install dylint-link

cargo install cargo-contract --force

cargo contract --help
```

## 创建智能合约
```bash
#新建合约
cargo contract new flipper

#测试合约
cargo +nightly test

#编译智能合约
cargo +nightly contract build

```
**build** 是为项目构建 WebAssembly 二进制文件，包含以下数据：
1. default_contract.contract (code + metadata)      .contract用于部署合约的文件 
2. default_contract.wasm (the contract's code)
3. metadata.json (the contract's metadata)          合约应用程序二进制接口 (ABI) 的元数据文件

**metadata.json** 详解：描述了您可以用来与此合约交互的所有接口  
1. **spec**部分包括有关可以调用的函数（如构造函数和消息）、发出的事件以及可以显示的任何文档的信息。
   1. 本节还包括一个selector包含函数名称的 4 字节散列的字段，用于将合约调用路由到正确的函数。
2. **storage**部分定义了合约管理的所有存储项目以及如何访问它们
3. **types**部分提供了整个 JSON 其余部分中使用的自定义数据类型

## 测试build的智能合约
可以使用 [官方的前端模版框架](https://github.com/substrate-developer-hub/substrate-front-end-template)测试  
也可以使用 [波卡](https://polkadot.js.org/apps)测试： 

## !ink智能合约的架构
mod incrementer
   1. pub struct Incrementer(一个mod中只能有一个 存储结构)
   2. impl Incrementer（可以有多个函数）
      1. #[ink(constructor)] 用来定义构造函数
      2. #[ink(message)] 公共函数都必须使用该属性

## 智能合约的编写
支持的**数据类型**  
1. rust中的布尔值、无符号和有符号整数、字符串、元组和数组
2. Substrate中的AccountId、Balance和Hash

**self.env().caller()** 获取合约调用者，如果用户访问一个合约然后调用后续合约，self.env().caller()则第二个合约中的地址是第一个合约的地址  

存储Map键值对：
1. 引入 use ink_storage::traits::SpreadAllocate;
2. 在存储结构上添加 #[derive(SpreadAllocate)]
3. ink_lang::utils::initalize_contract函数来初始化合约的映射()
4. Mapping API中的方法
   1. insert() 新增
   2. get() 获取
   3. remove() 移除

## token合约
**ERC-20** token合约，ERC-20 规范（不是唯一的，但是是最常见的）定义了可替代代币的通用标准。  
拥有定义令牌的属性的标准使遵循规范的开发人员能够构建可以与其他产品和服务互操作的应用程序。  

### ERC-20 标准
ERC-20 代币标准定义了在以太坊区块链上运行的大多数智能合约的接口。这些标准接口允许个人在现有的智能合约平台之上部署自己的加密货币。  
核心功能：
```
// ----------------------------------------------------------------------------
// ERC Token Standard #20 Interface
// https://github.com/ethereum/EIPs/blob/master/EIPS/eip-20.md
// ----------------------------------------------------------------------------

contract ERC20Interface {
    // Storage Getters
    function totalSupply() public view returns (uint);
    function balanceOf(address tokenOwner) public view returns (uint balance);
    function allowance(address tokenOwner, address spender) public view returns (uint remaining);

    // Public Functions
    function transfer(address to, uint tokens) public returns (bool success);
    function approve(address spender, uint tokens) public returns (bool success);
    function transferFrom(address from, address to, uint tokens) public returns (bool success);

    // Contract Events
    event Transfer(address indexed from, address indexed to, uint tokens);
    event Approval(address indexed tokenOwner, address indexed spender, uint tokens);
}
```
用户余额映射到账户地址，界面允许用户转移他们拥有的代币或允许第三方代表他们转移代币。  
最重要的是，必须实施智能合约逻辑以确保资金不会被无意创建或销毁，并且用户的资金不会受到恶意行为者的影响。  
所有公共函数都返回一个bool仅指示调用是否成功的 a。在 Rust 中，这些函数通常会返回一个Result.  

### 创建代币供应
处理 ERC-20 代币的智能合约类似于使用地图存储值的增量器合约，使用地图存储值。  
对于 ERC-20 代币合约，初始存储包括：
1. **total_supply** 代表合约中代币的总供应量。
2. **balances**代表每个账户的个人余额。

**第三方传输**  允许一个账户代表另一个账户使用代币，主要有**批准**和**转移**两个操作。
1. approve 批准函数，多次调用会覆盖原先的值，想撤销时，直接调用0即可。默认情况下，任意两个账户之间的批准值为0
2. transfer_from 使批准的用户能够转移代币，本质是调用私有transfer_from_to函数来完成大部分传输逻辑
   1. self.env().caller()必须为合约调用者分配from账户中可用的代币。
   2. 存储为 an 的分配allowance必须大于要传输的值。

### 碰到的问题
1. **contracts.StorageDepositLimitExhausted** 原因是代币的存储限额已经用完  
由于是基于polkadot-v0.9.26自己配置的合约模块，一切都是默认的。  
更改 runtime/src/lib.rs 中的 **deposit()** 函数
2. ClientImport("Unexpected epoch change") 是因为在未正确停止的情况下中断正在运行的节点（关闭终端或计算机切换到睡眠模式）
3. 遇到版本问题，可查找合约版本相对应时间点的rust版本进行编译
````bash
#以下解决了cargo-contract版本v1.5.0 不能编译3.3.0合约的问题

rustup toolchain install nightly-2022-08-15

rustup target add wasm32-unknown-unknown --toolchain nightly-2022-08-15 

rustup component add rust-src --toolchain nightly-2022-08-15 

cargo +nightly-2022-08-15 contract build 

````