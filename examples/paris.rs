use eth_rlp_verify::block_header::BlockHeader as VerifiableBlockHeader;
use eth_rlp_verify::verify_block;

fn main() {
    let block_header = VerifiableBlockHeader {
        block_hash: "0xa49fa930183f0fa8ca46244e946a635637b90c374d845e9c68dd7fd66cb6e051"
            .to_string(),
        number: 15537395,
        gas_limit: 30000000,
        gas_used: 29982083,
        nonce: "0x0000000000000000".to_string(),
        transaction_root: Some(
            "0x5c56184fbce74e9c98d2a51aa2110963396047d84e8ce1ae1785337255cd11e0".to_string(),
        ),
        receipts_root: Some(
            "0x1707e457973ce280debe93f5d478663d97ad192beea1f2fc65f391db80226e5e".to_string(),
        ),
        state_root: Some(
            "0x2ca38a39c5517f658d107c19550334a9820a7393d14857f2c0e0458292668d64".to_string(),
        ),
        base_fee_per_gas: Some("0xcc8ac8283".to_string()),
        parent_hash: Some(
            "0x56a9bb0302da44b8c0b3df540781424684c3af04d0b7a38d72842b762076a664".to_string(),
        ),
        miner: Some("0x0b3b161b8abeb6b04cb95c3e6047f80c120a0292".to_string()),
        logs_bloom: Some("0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_string()),
        difficulty: Some("0x0".to_string()),
        totaldifficulty: Some("0xc8fb2...".to_string()),
        sha3_uncles: Some(
            "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347".to_string(),
        ),
        timestamp: Some("0x6322c97f".to_string()),
        extra_data: Some("0x".to_string()),
        mix_hash: Some(
            "0xfd7be062e87b2193dff12ca89e90bbf61684e6676df4e86cca6730237863dd23".to_string(),
        ),
        ommers_hash: Some(
            "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347".to_string(),
        ),
        withdrawals_root: None,
        blob_gas_used: None,
        excess_blob_gas: None,
        parent_beacon_block_root: None,
    };

    let block_hash = "0xe37e1a183a3d1c7234d090bfb7196081635919c26f2e65c67c106513158a7db4";

    let is_valid = verify_block(15537401, &block_header, block_hash);
    println!("Paris era block verification result: {}", is_valid);
}
