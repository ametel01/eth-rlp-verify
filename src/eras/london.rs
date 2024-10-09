use crate::block_header::{BlockHeader as VerifiableBlockHeader, BlockHeaderTrait}; // Alias for clarity
use ethereum_types::{H160, H256, U256};
use rlp::RlpStream;
use std::str::FromStr;
use tracing::debug;

/// Represents an Ethereum block header for the London upgrade.
///
/// The London upgrade introduced EIP-1559, a major change to the gas fee mechanism, and
/// this struct contains the relevant fields for verifying and managing Ethereum block headers
/// during this era. Each field represents critical data for ensuring the correctness and
/// consistency of blocks.
///
/// # Fields
///
/// - `parent_hash`: The hash of the previous block, linking this block to the blockchain.
/// - `ommers_hash`: The hash of the ommer (uncle) blocks included in this block.
/// - `beneficiary`: The Ethereum address of the miner/validator who produced the block.
/// - `state_root`: The root of the state trie after processing all transactions.
/// - `transactions_root`: The root hash of the Merkle tree containing all transactions in the block.
/// - `receipts_root`: The root hash of the Merkle tree containing transaction receipts in the block.
/// - `logs_bloom`: A 256-byte bloom filter used for efficiently filtering and searching logs.
/// - `difficulty`: The difficulty required to mine this block, used in proof-of-work (PoW).
/// - `number`: The block number, which identifies its position in the blockchain.
/// - `gas_limit`: The maximum amount of gas allowed to be used by transactions in the block.
/// - `gas_used`: The actual gas consumed by the transactions within this block.
/// - `timestamp`: The time when the block was mined.
/// - `extra_data`: Arbitrary data provided by the miner, with a maximum size of 32 bytes.
/// - `mix_hash`: A PoW-related hash used to verify the mining process.
/// - `nonce`: A 64-bit PoW nonce used in the block's PoW verification.
/// - `base_fee_per_gas`: The minimum gas price per unit introduced by EIP-1559 during the London upgrade.
#[derive(Debug)]
pub struct BlockHeaderLondon {
    pub parent_hash: H256,
    pub ommers_hash: H256,
    pub beneficiary: H160,
    pub state_root: H256,
    pub transactions_root: H256,
    pub receipts_root: H256,
    pub logs_bloom: [u8; 256],
    pub difficulty: U256,
    pub number: U256,
    pub gas_limit: U256,
    pub gas_used: U256,
    pub timestamp: U256,
    pub extra_data: Vec<u8>,
    pub mix_hash: H256,
    pub nonce: [u8; 8],
    pub base_fee_per_gas: U256,
}

impl BlockHeaderLondon {
    /// Converts a `VerifiableBlockHeader` (typically fetched from the database) into a `BlockHeaderLondon`.
    ///
    /// This function maps a database block header into a `BlockHeaderLondon` struct by parsing
    /// various string fields such as `logs_bloom`, `nonce`, and cryptographic fields like `parent_hash`.
    ///
    /// # Arguments
    ///
    /// - `db_header`: A `VerifiableBlockHeader` containing the raw data of the block header.
    ///
    /// # Returns
    ///
    /// A `BlockHeaderLondon` instance with the fields populated and parsed.
    pub fn from_db_header(db_header: &VerifiableBlockHeader) -> Self {
        let logs_bloom = <Self as BlockHeaderTrait>::hex_to_fixed_array::<256>(
            &db_header.logs_bloom.clone().unwrap_or_default(),
        );
        let nonce = <Self as BlockHeaderTrait>::hex_to_fixed_array::<8>(&db_header.nonce);

        BlockHeaderLondon {
            parent_hash: H256::from_str(&db_header.parent_hash.clone().unwrap_or_default())
                .unwrap(),
            ommers_hash: H256::from_str(&db_header.sha3_uncles.clone().unwrap_or_default())
                .unwrap(),
            beneficiary: H160::from_str(&db_header.miner.clone().unwrap_or_default()).unwrap(),
            state_root: H256::from_str(&db_header.state_root.clone().unwrap_or_default()).unwrap(),
            transactions_root: H256::from_str(
                &db_header.transaction_root.clone().unwrap_or_default(),
            )
            .unwrap(),
            receipts_root: H256::from_str(&db_header.receipts_root.clone().unwrap_or_default())
                .unwrap(),
            logs_bloom,
            difficulty: U256::from_str(&db_header.difficulty.clone().unwrap_or("0x0".to_string()))
                .unwrap(),
            number: U256::from(db_header.number as u64),
            gas_limit: U256::from(db_header.gas_limit as u64),
            gas_used: U256::from(db_header.gas_used as u64),
            timestamp: U256::from_str(&db_header.timestamp.clone().unwrap_or_default()).unwrap(),
            extra_data: hex::decode(&db_header.extra_data.clone().unwrap_or_default()[2..])
                .unwrap_or_default(),
            mix_hash: H256::from_str(&db_header.mix_hash.clone().unwrap_or_default()).unwrap(),
            nonce,
            base_fee_per_gas: U256::from_str(
                &db_header.base_fee_per_gas.clone().unwrap_or_default(),
            )
            .unwrap(),
        }
    }
}

/// Implements the `BlockHeaderTrait` for `BlockHeaderLondon`.
///
/// This implementation enables encoding of the London block header using RLP (Recursive Length Prefix),
/// a compact serialization format used in Ethereum. This method allows for serialization, verification,
/// and the comparison of block headers.
///
/// # Example
///
/// ```rust
/// let header = BlockHeaderLondon::from_db_header(db_header);
/// let encoded_header = header.rlp_encode();
/// ```
impl BlockHeaderTrait for BlockHeaderLondon {
    /// RLP encodes the London block header, returning a vector of bytes.
    ///
    /// This function serializes all 16 fields of the London block header into RLP format, which is used
    /// for compact storage and transmission. The RLP encoding is critical for Ethereum block verification
    /// and consensus.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the RLP-encoded block header data.
    fn rlp_encode(&self) -> Vec<u8> {
        let mut stream = RlpStream::new_list(16); // 16 fields in London block header
        stream.append(&self.parent_hash);
        stream.append(&self.ommers_hash);
        stream.append(&self.beneficiary);
        stream.append(&self.state_root);
        stream.append(&self.transactions_root);
        stream.append(&self.receipts_root);
        stream.append(&self.logs_bloom.to_vec());
        stream.append(&self.difficulty);
        stream.append(&self.number);
        stream.append(&self.gas_limit);
        stream.append(&self.gas_used);
        stream.append(&self.timestamp);
        stream.append(&self.extra_data);
        stream.append(&self.mix_hash);
        stream.append(&self.nonce.as_slice());
        stream.append(&self.base_fee_per_gas);
        stream.out().to_vec()
    }
}

/// Verifies the hash of a London block by comparing the computed hash with the provided hash.
///
/// This function verifies the integrity of a block by first converting a `VerifiableBlockHeader`
/// to a `BlockHeaderLondon`, then RLP encoding the block header and computing its hash using the
/// Keccak256 hashing algorithm. It compares the computed hash with the given `block_hash`.
///
/// # Arguments
///
/// - `block_hash`: A string containing the expected hash of the block.
/// - `db_header`: A `VerifiableBlockHeader` fetched from the database, containing the raw block header data.
///
/// # Returns
///
/// A boolean value indicating whether the computed hash matches the provided hash.
///
/// # Example
///
/// ```rust
/// let is_valid = verify_hash_london("0xabc...".to_string(), db_header);
/// if is_valid {
///     println!("Block hash is valid!");
/// } else {
///     println!("Invalid block hash.");
/// }
/// ```
pub fn verify_hash_london(block_hash: String, db_header: &VerifiableBlockHeader) -> bool {
    let header = BlockHeaderLondon::from_db_header(db_header);

    // Log the RLP encoded data for debugging purposes
    let rlp_encoded = header.rlp_encode();
    debug!("RLP Encoded: {:?}", rlp_encoded);

    // Compute the block hash
    let computed_block_hash = header.compute_hash();
    debug!("Computed Block Hash: {:?}", computed_block_hash);

    // Check if the computed hash matches the given block hash
    let is_valid = computed_block_hash == H256::from_str(&block_hash).unwrap();

    is_valid
}
