use clar2wasm::tools::crosscheck_compare_only_advancing_tip;
use proptest::proptest;

const BLOCK_INFO: [&str; 8] = [
    "burnchain-header-hash",
    "id-header-hash",
    "header-hash",
    "miner-address",
    "block-reward",
    "miner-spend-total",
    "miner-spend-winner",
    "time",
];
const BURN_BLOCK_INFO: [&str; 2] = ["header-hash", "pox-addrs"];
const STACKS_BLOCK_HEIGHT_LIMIT: u32 = 100;
const BURN_BLOCK_HEIGHT_LIMIT: u32 = 100;

#[cfg(any(feature = "test-clarity-v1", feature = "test-clarity-v2"))]
mod clarity_v1_v2 {
    use super::*;
    use crate::runtime_config;

    proptest! {
        #![proptest_config(runtime_config())]

        #[test]
        fn crossprop_blockinfo_within_controlled_range(block_height in 1..=STACKS_BLOCK_HEIGHT_LIMIT, tip in 1..=80u32) {
            for info in &BLOCK_INFO {
                crosscheck_compare_only_advancing_tip(&format!("(get-block-info? {info} u{block_height})"), tip)
            }
        }
    }
}

#[cfg(not(any(feature = "test-clarity-v1", feature = "test-clarity-v2")))]
mod clarity_v3 {
    use super::*;
    use crate::runtime_config;

    proptest! {
        #![proptest_config(runtime_config())]

        #[ignore = "see issue #428"]
        #[test]
        fn crossprop_blockinfo_within_controlled_range(block_height in 1..=STACKS_BLOCK_HEIGHT_LIMIT, tip in 1..=80u32) {
            for info in &BLOCK_INFO {
                crosscheck_compare_only_advancing_tip(&format!("(get-stacks-block-info? {info} u{block_height})"), tip)
            }
        }
    }
}

proptest! {
    #![proptest_config(super::runtime_config())]

    # [test]
    fn crossprop_blockinfo_burnchain_within_controlled_range(block_height in 1..=BURN_BLOCK_HEIGHT_LIMIT, tip in 1..=80u32) {
        for info in &BURN_BLOCK_INFO {
            crosscheck_compare_only_advancing_tip(
                &format!("(get-burn-block-info? {info} u{block_height})"), tip
            )
        }
    }
}
