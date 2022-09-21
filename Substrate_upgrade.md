# Substrate upgrade 链上升级和数据迁移

1. 为什么substrate能升级  
   Substrate 把runtime都编译成WASM，并保存在链上  
   Client读取WASM的代码，通过WASM Executor来进行状态转变  
   当新的WASM代码设置到链上之后，新的runtime逻辑就生效了  
2. 升级的过程  
   1. 升级spec版本号
   2. 编译新的WASM,WASM_TARGET_DIRECTORY=$(pwd)
   3. 通过Sudo或者链上治理来更新WASM
3. 链上数据存储  
   所有的数据都substrate是放在一个Key,Value的数据库中  
   原始数据的Key： Twox128(module_prefix) ++ Twox128(storage_prefix)  
   Map类型数据的Key： Twox128(module_prefix) ++ Twox128(storage_prefix) ++ hasher(encode(key)  
4. 什么情况数据迁移  
   模块名字改变，变量名字改变，Key改变，hash算法改变，值类型改变

## 链上升级

## 数据迁移