/// 区块结构 (Block):
///
/// timestamp: 时间戳，表示区块创建时间，数据类型为 i64。
///
/// pre_block_hash: 前一个区块的哈希值，数据类型为 String。
///
/// hash: 当前区块的哈希值，数据类型为 String。
///
/// transactions: 交易列表，数据类型为 Vec<Transaction>。
///
/// nonce: 随机数，用于 Proof of Work 算法，数据类型为 i64。
///
/// height: 区块高度，表示区块在区块链中的位置，数据类型为 usize
///
pub struct Block {
    timestamp: i64,
    pre_block_hash: String,
    hash: String,
    transactions: Vec<Transaction>,
    nonce: u64,
    height: usize,
}

/// 交易结构 (Transaction):
///
/// id: 交易 ID，数据类型为 Vec<u8>。
///
/// vin: 交易输入列表，数据类型为 Vec<TXInput>。
///
/// vout: 交易输出列表，数据类型为 Vec<TXOutput>。
pub struct Transaction {
    id: Vec<u8>,
    vin: Vec<TXInput>,
    vout: Vec<TXOutput>,
}
/// 交易输入结构 (TXInput):
///
/// txid: 交易 ID，数据类型为 Vec<u8>。
///
/// vout: 交易输出索引，数据类型为 usize。
///
/// signature: 签名，数据类型为 Vec<u8>。
///
/// pub_key: 公钥，数据类型为 Vec<u8>。
pub struct TXInput {
    txid: Vec<u8>,
    vout: usize,
    signature: Vec<u8>,
    pub_key: Vec<u8>,
}

/// 交易输出结构 (TXOutput):
///
/// value: 交易金额，数据类型为 i32。
///
/// pub_key_hash: 公钥哈希值，数据类型为 Vec<u8>。
pub struct TXOutput {
    value: i32,
    pub_key_hash: Vec<u8>,
}

pub struct PoroofOfWork {
    block: Block,
    target: BigInt,
}

/// Once we have the struct to be able to implement a structure for the block,
/// we can take a look at some of the functions that will help us work with blockchain

/// The first step in creating the new block is to initialize a new Block object

pub fn new_block(pre_block_hash: String, transactions: &[Transaction], height: usize) -> Block {
    let mut block = Block {
        timestamp: crate::current_timestamp(),
        pre_block_hash,
        hash: String::new(),
        transactions: transactions.to_vec(),
        nonce: 0,
        height,
    };
    let pow = PoroofOfWork::new_proof_of_work(block.clone());
    let (nonce, hash) = pow.run();

    block.nonce = nonce;
    block.hash = hash;
    return block;
}
