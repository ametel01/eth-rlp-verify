pub mod block_header;
pub mod constants;
pub mod eras;

use crate::block_header::BlockHeader as VerifiableBlockHeader;

/// Verifies the validity of an Ethereum block header based on the block number and expected hash.
///
/// This function determines the appropriate Ethereum era based on the block number, retrieves the corresponding
/// verification function, and verifies the block header by comparing its computed hash with the expected block hash.
/// The verification process ensures that the block is authentic and belongs to the correct place in the blockchain.
///
/// # Arguments
///
/// - `block_number`: A `u64` representing the block number of the block being verified. The block number is used
///   to determine the correct era (e.g., London, Paris, Shapella) and apply the corresponding block verification logic.
/// - `block_header`: A `VerifiableBlockHeader` struct containing the block header data that needs to be verified.
/// - `block_hash`: A string slice (`&str`) representing the expected hash of the block. This hash will be compared
///   to the computed hash of the block header to determine if the block is valid.
///
/// # Returns
///
/// A `bool` indicating whether the block header is valid:
/// - `true`: The block hash matches the computed hash, and the block header is valid.
/// - `false`: The block number is not within a supported era, or the computed block hash does not match the expected hash.
///
/// # Example
///
/// ```rust
/// let block_number = 15_537_394; // A block from the Paris era
/// let block_header = fetch_block_header_from_db(block_number); // Fetch the block header from the database
/// let block_hash = "0xabc..."; // Expected block hash for verification
///
/// let is_valid = verify_block(block_number, block_header, block_hash);
/// if is_valid {
///     println!("Block header is valid!");
/// } else {
///     println!("Invalid block header.");
/// }
/// ```
///
/// # Era Determination
///
/// The function uses `eras::determine_era` to determine the correct era based on the `block_number`.
/// This ensures that the correct block structure and verification rules are applied for each Ethereum upgrade:
/// - **Genesis to London**: The era from the Genesis block to the London upgrade.
/// - **London to Paris**: The era starting with the London upgrade and ending with the Paris (The Merge) upgrade.
/// - **Paris to Shapella**: The era from the Paris upgrade to the Shapella (Shanghai + Capella) upgrade.
/// - **Shapella to Dencun**: The era starting with Shapella and continuing into the Dencun upgrade.
///
/// # Notes
///
/// - If the block number is not within the supported range, the function returns `false`.
/// - This function is designed to be future-proof, allowing for additional Ethereum eras and upgrades to be supported
///   by simply adding them to the `eras::determine_era` function.
pub fn verify_block(
    block_number: u64,
    block_header: &VerifiableBlockHeader,
    block_hash: &str,
) -> bool {
    match eras::determine_era(block_number) {
        Some(verify_fn) => verify_fn(block_hash.to_string(), block_header),
        None => false, // If the block number is out of the supported range
    }
}
