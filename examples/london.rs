use eth_rlp_verify::block_header::BlockHeader as VerifiableBlockHeader;
use eth_rlp_verify::verify_block;

fn main() {
    let block_header = VerifiableBlockHeader {
        block_hash: "0x9b83c12c69edb74f6c8dd5d052765c1adf940e320bd1291696e6fa07829eee71".to_string(),
        number: 12965001, // Correct decimal representation of "0xc5d489"
        gas_limit: 29999798, // Correct decimal representation of "0x1c9c2b6"
        gas_used: 29985144, // Correct decimal representation of "0x1c98978"
        nonce: "0x0956e895d988798e".to_string(),
        transaction_root: Some("0x03c97f958cc4db3cc60def5ce1e83aaf1490837f5f57c529a6ccffef0d201edb".to_string()),
        receipts_root: Some("0x2335850563dbf51f65a37508f2fdd9da1780f70cfa46734107a2e86a9fde46d7".to_string()),
        state_root: Some("0x0180d59eb0855ef6dbca806fbe81491bea252ab2e0d3a8bb8786326d598e3cd9".to_string()),
        base_fee_per_gas: Some("0x430da58e".to_string()),
        parent_hash: Some("0x9b83c12c69edb74f6c8dd5d052765c1adf940e320bd1291696e6fa07829eee71".to_string()),
        ommers_hash: Some("0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347".to_string()),
        miner: Some("0x829bd824b016326a401d083b33d092293333a830".to_string()),
        logs_bloom: Some("0x74adf8cfdd0a1ddf12f3d6d5bbd79cab73a19b6986fc007932d9acffafebb747debf512456c87e9afffa5f40fd21ad403b97f3b38e86e9e9db62433eb2b6f8547ad677fdab07f1adcb83686fb37db9ea7acb113f0d74b397324d9cfbf8f33cb3dbfb0d256bcbdaf608dd7b1ac168ee40e322b69bf675a6f4fbbbbe72dccbdd88fab28e7d94685c34bffc9bd1ff98ef777af7ff9793de951d336a1b75acbc7f11ce9dac7e9942ab6a363b4fbebbc3d738dbee5a993fa7c87adce26cbeddfdfcf4d59bba977fb7514a3da550c0b21f399e8bf56778c7dfdcfeeb2457abef1fe63eaf38ecbabdae6c237afd34378163feb6ccdb42f56782cd474bdf9ee9fadb94b4".to_string()),
        difficulty: Some("0x1b81c23e05b218".to_string()),
        totaldifficulty: Some("0x608af5dd578251af429".to_string()),
        sha3_uncles: Some("0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347".to_string()),
        timestamp: Some("0x610bdab3".to_string()), // Correct decimal representation of "0x610bdab3"
        extra_data: Some("0xe4b883e5bda9e7a59ee4bb99e9b1bc030521".to_string()),
        mix_hash: Some("0xcb3166ebb1888430069b769145b20ba5e3a55f32fd2fa39f0ebdc08d60b4557e".to_string()),
        withdrawals_root: None,
        blob_gas_used: None,
        excess_blob_gas: None,
        parent_beacon_block_root: None,
    };

    let block_hash = "0xa32d159805750cbe428b799a49b85dcb2300f61d806786f317260e721727d162";

    let is_valid = verify_block(12965001, &block_header, block_hash);
    println!("London era block verification result: {}", is_valid);
}
