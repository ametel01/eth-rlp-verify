use eth_rlp_verify::block_header::BlockHeader as VerifiableBlockHeader;
use eth_rlp_verify::verify_block;

fn main() {
    let block_header = VerifiableBlockHeader {
        block_hash: "0xb49fa930183f0fa8ca46244e946a635637b90c374d845e9c68dd7fd66cb6e0a1"
            .to_string(),
        number: 17034871,
        gas_limit: 30000000,
        gas_used: 29970493,
        nonce: "0x0000000000000000".to_string(),
        transaction_root: Some(
            "0xd3deb43136a95fac91dc5a451caf62b18f0d3ffa5d8744b769ac5df6d8ef40bc".to_string(),
        ),
        receipts_root: Some(
            "0xe4001611ca993adcb8948b4f114bc0985948bee0fef32c4e2292ad87356afd58".to_string(),
        ),
        state_root: Some(
            "0x1d49150d23cc59f06491e138b7a9bf0d307bcd7bb2bab6e38754740931d6c3ef".to_string(),
        ),
        base_fee_per_gas: Some("0x4b5b025b0".to_string()),
        parent_hash: Some(
            "0xe22c56f211f03baadcc91e4eb9a24344e6848c5df4473988f893b58223f5216c".to_string(),
        ),
        miner: Some("0x0b70b578abd96aab5e80d24d1f3c28dbde14356a".to_string()),
        logs_bloom: Some("0x9479b5dce9a45ebc5af9eee4f1a9cb73dfb3437b2b7edff2a5f36addcfa4319e74212ff4d469c756dfddbeb6db79d5ab5effee57ac57fdfdbbe30a1fd8ffa9fdb9ddfb19771ffdbd7eff4bdbd3d5f97fdf571edeeafefe5a6eb55eceffd739b27ffcff7f16a38dbff45ddd25fb9d7b9bef9bae37e0bb6f7dfbf5fbf3cdfd5dbdbe57ffdfe7ef77ddb7fef267af53587bfb7ffcb3f9f673fc7eababe5fbfe77eddff945e347a6fffdfbfbe8fd7ffee79efcfcce5d77a66db3f9fabe9f3b8b1d79f9a99bb7af11f93f6f9b736f2cbf5697d57f4b6de478bdfef6759dfbbe5bebee7cfff7e9df8acdd46f9777fb47edfaf0b5f5ba74393acff5d24bfcf389fb9eff".to_string()),
        difficulty: Some("0x0".to_string()),
        totaldifficulty: Some("0xd8fb2...".to_string()),
        sha3_uncles: Some(
            "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347".to_string(),
        ),
        timestamp: Some("0x64373087".to_string()),
        extra_data: Some("0x4e65746865726d696e64".to_string()),
        mix_hash: Some(
            "0xd7a4a06e28abcc8ac4e0bab5f0a7e60ea7a0c3de93b2f7e7a4cc3c9a79e60186".to_string(),
        ),
        ommers_hash: Some(
            "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347".to_string(),
        ),
        withdrawals_root: Some(
            "0xc32381c919dad80afe8fe0df79460418e350725a63f67c55b27ee168ef464e5d".to_string(),
        ),
        blob_gas_used: Some("0xa0000".to_string()),
        excess_blob_gas: Some("0x20000".to_string()),
        parent_beacon_block_root: Some(
            "0x2b5b8c2d329148bc5d724cac0a7abc557f93471c24ea027b119ecedb937c0045".to_string(),
        ),
    };

    let block_hash = "0x17cf53189035bbae5bce5c844355badd701aa9d2dd4b4f5ab1f9f0e8dd9fea5b";

    let is_valid = verify_block(17034871, &block_header, block_hash);
    println!("Shapella era block verification result: {}", is_valid);
}
