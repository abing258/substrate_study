# Substrate Scale编解码

## 数据序列化和反序列化
数据对象转换成二进制码，搞笑的进行存储和传输;反之，以相同的规则将二进制解码，可以获得原始数据  

    1. 比特币使用的是Bitcion specific serialization format
    2. 以太坊使用的是RLP（recursive length prefix）
    3. Substrate使用的是Scale
    
### SCALE
简单拼接聚合的小端数据格式（simple concatenated Aggregate Little=Endian）优点:  
1. **轻量高效**的二进制码格式
2. 适用于**Blockchain runtime**，低内存的资源有限环境
3. **链上数据**和**交易传输**的编码格式
4. 不包含类型信息，解码调用方必须有类型信息
5. 新类型，#[derive(Encode, Decode)]
6. 不同类型对应的编码规则不同
    
**SCALE Codec原理**:不同类型的编码规则：
#### 整型
1. 固定宽度整数，如: u8,i8 u32 i32...（低位在前，高位在后）
   1. i8:69
2. 整数压缩编码
   1. 整数类型前标记
      1. 整数作为参数时--> #[compact]
      2. 结构体中的属性时--> #[codec[compact]]
   2. 最低位的两个bit位表示
      1. 0b00，单字节模式，高6位是值的LE编码 ( 0 ~ 63 )
      2. 0b01，两字节模式，高6位和下一个字节是值的LE编码 ( 64 ~ (2^14-1 )
      3. 0b10，四字节模式，高6位和下3个字节是值的 LE 编码 (  (2^14-1) ~ (2^30-1)  )
      4. 0b11，大整数模式，高6位表示用来编码值的字节数减去4，之后的字节是值的编码 ( (2^30-1) ~ (2^536-1) )
         
       
      unsigned integer 0,  binary origin: 0000 0000, 0b00 mode, binary add mode: 0000 0000,            hex: 0x00
      unsigned integer 1,  binary origin: 0000 0001, 0b00 mode, binary add mode: 0000 0100,            hex: 0x04
      unsigned integer 42, binary origin: 0010 1010, 0b00 mode, binary add mode: 1010 1000,            hex: 0xa8
      unsigned integer 69, binary origin: 0100 0101, 0b01 mode, binary add mode: 0000 0001, 0001 0101, hex: 0x1501

#### 布尔值
**布尔值**，单字节的最小位表示  
   1. false，binary: 0000 0000, hex: 0x00
   2. true， binary: 0000 0001,  hex: 0x01

#### Option<T> 类型
1. 如果有值，将保存的值编码后拼接，如 Option<i8>
   1. None，binary: 0000 0000, hex: 0x00
      Some(69)，binary: binary: 0000 0001, 0100 0101, hex: 0x01 45
2. 特例，Option<bool>
   1. None, hex: 0x00
      Some(true), hex: 0x01
      Some(false), hex: 0x02

#### Result<T, E> 类型
0x00 表示 Ok(v)，后面紧跟值 v 的编码
0x01 表示 Err(e)，后面紧跟错误信息 e 的编码
```rust
   type MyResult = std::result::Result<u8, bool>;
   Ok(42), hex: 0x002a
   Err(false), hex: 0x0100
```

#### Vectors (lists, series, sets)
以集合内元素数量的 compact 编码开始，紧跟各个元素值的编码，按顺序拼接，例如：
```rust
origin: u16 整数的集合，[4, 8, 15, 16, 23, 42]，共6个元素
binary: 0001 1000 (6 in compact), 
        0000 0000, 0000 0100 (4), 
        0000 0000, 0000 1000 (8),
        0000 0000, 0000 1111 (15), 
        0000 0000, 0001 0000 (16),
        0000 0000, 0001 0111 (23), 
        0000 0000, 0010 1010 (42),
hex:
0x18 0400 0800 0f00 1000 1700 2a00
```

#### 字符串 String
以 Vec<u8> 的形式进行表示和编码, u8 数值来源于字符的 UTF8 编码

#### 元组 Tuple
各个元素的编码直接拼接
```rust
origin: (3, false)，
binary: 0000 1100, 0000 0000 , 
hex: 0x0c00
```

#### 结构体 Struct
**属性名不会被编码到结果中**。和元组类似，通常是各个属性值的编码直接拼接
```rust
struct MyStruct {
       #[codec(compact)]
       a: u32,
       b: bool,
}

let my_struct = MyStruct {
     a: 42,
     b: true,
}

binary: 0010 1010, 0000 0001, 
binary add mode: 1010 1000,  0000 0001, 
hex: 0xa8 01
```

#### 枚举 Enum
第一个字节用来标识变体的位置，即最多支持256个变体，其后的内容用来编码变体里可能包含的值，
#[codec(index = "1")]，指定某个变体的 index 编码
```rust
enum IntOrBool {
     Int(u8),
     Bool(bool),
}

Int(42)， hex: 0x002a
Bool(true)，hex: 0x0101
```

### 课后补充 SCALE Codec 实现
**Rust**:            https://paritytech/parity-scale-codec  
**Python**:          https://polkascan/py-scale-codec  
**Golang**:          https://itering/scale.go  
**C+**:              https://soramitsu/scale  
**JavaScript**:      https://polkadot-js/api  
**AssemblyScript**:  https://LimeChain/as-scale-codec  
**Haskell**:         https://airalab/hs-web3  
**Java**:            https://emeraldpay/polkaj  
**Ruby**:            https://itering/scale.rb  
**Substrate**:       https://docs.substrate.io/reference/scale-codec/