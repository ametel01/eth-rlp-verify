use crate::block_header::{BlockHeader as VerifiableBlockHeader, BlockHeaderTrait}; // Alias for clarity
use ethereum_types::{H160, H256, U256};
use rlp::RlpStream;
use std::str::FromStr;
use tracing::debug;
use tracing::info;

/// Represents the block header for the Dencun upgrade in Ethereum.
///
/// The Dencun upgrade introduces several new fields to the Ethereum block header, such as `parent_beacon_block_root`,
/// `blob_gas_used`, and `excess_blob_gas`. This struct holds the critical fields for verifying and validating
/// blocks from the Dencun era, which marks significant changes in the Ethereum consensus mechanism.
///
/// # Fields
///
/// - `parent_hash`: The hash of the parent block, linking this block to the blockchain.
/// - `ommers_hash`: The hash of the ommers (uncles) included in this block.
/// - `beneficiary`: The Ethereum address of the miner who produced the block.
/// - `state_root`: The root of the state trie after the block is processed.
/// - `transactions_root`: The Merkle root of the transactions included in the block.
/// - `receipts_root`: The root of the transaction receipts for transactions in the block.
/// - `logs_bloom`: A bloom filter used for efficient searching and filtering of logs, 256 bytes in size.
/// - `difficulty`: The difficulty level for mining this block, used in proof-of-work (PoW) algorithms.
/// - `number`: The block number, which indicates its position in the blockchain.
/// - `gas_limit`: The maximum gas allowed to be consumed by transactions in this block.
/// - `gas_used`: The actual gas used by the transactions in this block.
/// - `timestamp`: The time when the block was mined.
/// - `extra_data`: Arbitrary data added by the miner, up to 32 bytes in length.
/// - `mix_hash`: A hash used in the proof-of-work algorithm for verifying the mining process.
/// - `nonce`: A 64-bit nonce used to validate the mining process in the proof-of-work consensus mechanism.
/// - `base_fee_per_gas`: The base gas fee per unit for transactions in this block, part of EIP-1559.
/// - `withdrawals_root`: The Merkle root of the withdrawals processed in the block (introduced in Shapella).
/// - `parent_beacon_block_root`: The root of the parent beacon block, newly introduced in the Dencun upgrade.
/// - `blob_gas_used`: The gas used for blob-related transactions (new in Dencun).
/// - `excess_blob_gas`: The excess blob gas in the block, used to manage blob-related transaction fees (new in Dencun).
#[derive(Debug)]
pub struct BlockHeaderDencun {
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
    pub withdrawals_root: H256,
    pub parent_beacon_block_root: H256, // New in Dencun
    pub blob_gas_used: U256,            // New in Dencun
    pub excess_blob_gas: U256,          // New in Dencun
}

impl BlockHeaderDencun {
    /// Converts a `VerifiableBlockHeader` from the database into a `BlockHeaderDencun`.
    ///
    /// This method takes a block header from the database, parses its fields, and converts it
    /// into a `BlockHeaderDencun` struct. It handles conversion of fields such as `logs_bloom`,
    /// `nonce`, and various cryptographic root fields, ensuring that the header is ready for verification.
    ///
    /// # Arguments
    ///
    // / - `db_header`: A `VerifiableBlockHeader` fetched from the database, containing the raw string data
    ///   for the block header fields.
    ///
    /// # Returns
    ///
    /// A `BlockHeaderDencun` instance populated with the parsed and validated block header data.
    pub fn from_db_header(db_header: &VerifiableBlockHeader) -> Self {
        let logs_bloom = <Self as BlockHeaderTrait>::hex_to_fixed_array::<256>(
            &db_header.logs_bloom.clone().unwrap_or_default(),
        );
        let nonce = <Self as BlockHeaderTrait>::hex_to_fixed_array::<8>(&db_header.nonce);

        BlockHeaderDencun {
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
            withdrawals_root: H256::from_str(
                &db_header.withdrawals_root.clone().unwrap_or_default(),
            )
            .unwrap(),
            parent_beacon_block_root: H256::from_str(
                &db_header
                    .parent_beacon_block_root
                    .clone()
                    .unwrap_or_default(),
            )
            .unwrap(),
            blob_gas_used: U256::from_str(&db_header.blob_gas_used.clone().unwrap_or_default())
                .unwrap(),
            excess_blob_gas: U256::from_str(&db_header.excess_blob_gas.clone().unwrap_or_default())
                .unwrap(),
        }
    }
}

/// Implements the `BlockHeaderTrait` for `BlockHeaderDencun`.
///
/// This implementation provides RLP encoding for the Dencun block header, which is necessary for
/// compact serialization and for verifying blocks on the Ethereum network. This trait enables
/// serialization and hash verification, critical for validating blocks.
///
/// # Example
///
/// ```rust
/// let header = BlockHeaderDencun::from_db_header(db_header);
/// let encoded_header = header.rlp_encode();
/// ```
impl BlockHeaderTrait for BlockHeaderDencun {
    /// RLP encodes the Dencun block header, returning a vector of bytes.
    ///
    /// This method serializes all 20 fields of the Dencun block header using Ethereum's
    /// RLP (Recursive Length Prefix) encoding. This encoding is essential for compact
    /// storage and transmission, and is used for verifying block consistency and integrity.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the RLP-encoded block header data.
    fn rlp_encode(&self) -> Vec<u8> {
        let mut stream = RlpStream::new_list(20); // 20 fields in Dencun block header
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
        stream.append(&self.withdrawals_root);
        stream.append(&self.blob_gas_used);
        stream.append(&self.excess_blob_gas);
        stream.append(&self.parent_beacon_block_root);
        stream.out().to_vec()
    }
}

/// Verifies the hash of a Dencun block by comparing the computed hash with the provided hash.
///
/// This function verifies the integrity of a block by taking the block header from the database,
/// converting it to a `BlockHeaderDencun`, and encoding it using RLP. The resulting encoded data
/// is hashed using Keccak256, and the computed hash is compared to the provided `block_hash` to
/// ensure the block's authenticity.
///
/// # Arguments
///
/// - `block_hash`: A string containing the expected hash of the block.
/// - `db_header`: A `VerifiableBlockHeader` fetched from the database, containing raw block header data.
///
/// # Returns
///
/// A boolean indicating whether the computed block hash matches the provided block hash.
///
/// # Example
///
/// ```rust
/// let is_valid = verify_hash_dencun("0xabc...".to_string(), db_header);
/// if is_valid {
///     println!("Block hash is valid!");
/// } else {
///     println!("Invalid block hash.");
/// }
/// ```
pub fn verify_hash_dencun(block_hash: String, db_header: &VerifiableBlockHeader) -> bool {
    let header = BlockHeaderDencun::from_db_header(db_header);

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
