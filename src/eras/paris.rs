use crate::block_header::{BlockHeader as VerifiableBlockHeader, BlockHeaderTrait}; // Alias for clarity
use ethereum_types::{H160, H256, U256};
use rlp::RlpStream;
use std::str::FromStr;
use tracing::debug;
use tracing::info;

/// Represents an Ethereum block header for the Paris upgrade.
///
/// The Paris upgrade marks Ethereum's transition from proof-of-work (PoW) to proof-of-stake (PoS),
/// and this struct stores key properties of a block header during that period. It contains fields
/// that describe various cryptographic roots, gas limits, and mining information required to verify
/// and authenticate the block.
///
/// # Fields
///
/// - `parent_hash`: The hash of the parent block, which links this block to the previous one.
/// - `ommers_hash`: The hash of ommer (uncle) blocks included in this block.
/// - `beneficiary`: The Ethereum address of the miner or validator who produced this block.
/// - `state_root`: The root hash of the state trie after this block is processed.
/// - `transactions_root`: The root hash of the Merkle tree of transactions in this block.
/// - `receipts_root`: The root hash of the Merkle tree of transaction receipts for this block.
/// - `logs_bloom`: A bloom filter used for fast log searching, filtering relevant logs efficiently.
/// - `difficulty`: The difficulty value that was required to mine the block in proof-of-work.
/// - `number`: The block number, indicating its position in the blockchain.
/// - `gas_limit`: The maximum gas allowed to be consumed by transactions in this block.
/// - `gas_used`: The total amount of gas used by the transactions in the block.
/// - `timestamp`: The timestamp indicating when the block was mined.
/// - `extra_data`: Extra data associated with the block, typically set by the miner, up to 32 bytes.
/// - `mix_hash`: A hash used to verify the proof-of-work (PoW) mining result.
/// - `nonce`: The 64-bit nonce used to verify the PoW and mine the block.
/// - `base_fee_per_gas`: The minimum gas fee for transactions in this block, as defined in EIP-1559.
#[derive(Debug)]
pub struct BlockHeaderParis {
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

impl BlockHeaderParis {
    /// Converts a `VerifiableBlockHeader` fetched from the database into a `BlockHeaderParis`.
    ///
    /// This function transforms a `VerifiableBlockHeader` into a `BlockHeaderParis` structure,
    /// parsing necessary fields such as the state root, transactions root, and miner's address.
    ///
    /// # Arguments
    ///
    /// - `db_header`: A `VerifiableBlockHeader` fetched from the database, containing the raw data.
    ///
    /// # Returns
    ///
    /// A `BlockHeaderParis` instance with parsed and populated fields.
    pub fn from_db_header(db_header: &VerifiableBlockHeader) -> Self {
        let logs_bloom = <Self as BlockHeaderTrait>::hex_to_fixed_array::<256>(
            &db_header.logs_bloom.clone().unwrap_or_default(),
        );
        let nonce = <Self as BlockHeaderTrait>::hex_to_fixed_array::<8>(&db_header.nonce);

        BlockHeaderParis {
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

/// Implements the `BlockHeaderTrait` for `BlockHeaderParis`.
///
/// This trait implementation enables RLP encoding for the Paris block header, which is essential
/// for compact and efficient serialization. It ensures that the Paris block header can be serialized
/// and verified using Ethereum's standard methods.
impl BlockHeaderTrait for BlockHeaderParis {
    /// RLP encodes the Paris block header, producing a vector of bytes.
    ///
    /// This function encodes all 16 fields of the Paris block header in compliance with
    /// Ethereum's RLP encoding scheme, which is used for serialization and block verification.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the RLP-encoded block header data.
    fn rlp_encode(&self) -> Vec<u8> {
        let mut stream = RlpStream::new_list(16); // 16 fields in Paris block header
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

/// Verifies the integrity of a Paris block hash by comparing it with the computed hash.
///
/// This function uses the provided block hash and the database block header to create a `BlockHeaderParis`,
/// then computes the block hash by RLP encoding the header and applying the Keccak256 hashing algorithm.
/// The computed hash is compared to the provided `block_hash` to check if the block is valid.
///
/// # Arguments
///
/// - `block_hash`: The expected hash of the block (as a hexadecimal string).
/// - `db_header`: A `VerifiableBlockHeader` containing block header information from the database.
///
/// # Returns
///
/// A boolean indicating whether the computed block hash matches the provided hash.
///
/// # Example
///
/// ```rust
/// let is_valid = verify_hash_paris("0xabc...".to_string(), db_header);
/// if is_valid {
///     println!("The block hash is valid!");
/// } else {
///     println!("Invalid block hash.");
/// }
/// ```
pub fn verify_hash_paris(block_hash: String, db_header: &VerifiableBlockHeader) -> bool {
    let header = BlockHeaderParis::from_db_header(db_header);

    // Log the RLP encoded data for debugging purposes
    let rlp_encoded = header.rlp_encode();
    debug!("RLP Encoded: {:?}", rlp_encoded);

    // Compute the block hash
    let computed_block_hash = header.compute_hash();
    info!("Computed Block Hash: {:?}", computed_block_hash);

    // Check if the computed hash matches the given block hash
    let is_valid = computed_block_hash == H256::from_str(&block_hash).unwrap();
    info!("Is the block hash valid? {}", is_valid);
    is_valid
}
