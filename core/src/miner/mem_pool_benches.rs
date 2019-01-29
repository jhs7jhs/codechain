extern crate test;

use std::sync::Arc;

use ckey::{Public, KeyPair};

use super::mem_pool_types::{
    AccountDetails, MemPoolInput, PoolingInstant, TxOrigin, TxTimelock,
};
use crate::transaction::SignedTransaction;

use self::test::{black_box, Bencher};
use super::mem_pool::MemPool;

use ckey::{Generator, Random};
use ctypes::transaction::{Action, Transaction};

#[bench]
pub fn add_ascending(bencher: &mut Bencher) {
    let fetch_account = |_p: &Public| -> AccountDetails {
        AccountDetails {
            seq: 0,
            balance: u64::max_value(),
        }
    };

    let keypair = &Random.generate().unwrap();
    let current_time = 100;
    let current_timestamp = 100;
    let num_txs = 10000;

    let mut inputs: Vec<MemPoolInput> = Vec::with_capacity(num_txs);
    for i in 0..num_txs {
        inputs.push(create_input(keypair, i as u64, None, None));
    }

    let inputs = &inputs;
    bencher.iter(|| {
        let db = Arc::new(kvdb_memorydb::create(crate::db::NUM_COLUMNS.unwrap_or(0)));
        let mut mem_pool = MemPool::with_limits(10000, usize::max_value(), 3, db.clone());
        for input in inputs {
            mem_pool.add(vec![input.clone()], current_time, current_timestamp, &fetch_account);
        }
        black_box(mem_pool);
    });
}

fn create_input(keypair: &KeyPair, seq: u64, block: Option<PoolingInstant>, timestamp: Option<u64>) -> MemPoolInput {
    let tx = Transaction {
        seq,
        fee: 100,
        network_id: "tc".into(),
        action: Action::Pay {
            receiver: 0.into(),
            quantity: 100,
        },
    };
    let timelock = TxTimelock {
        block,
        timestamp,
    };
    let signed = SignedTransaction::new_with_sign(tx, keypair.private());

    MemPoolInput::new(signed, TxOrigin::Local, timelock)
}
// pub fn bench_sign(bh: &mut Bencher) {
//     let s = Secp256k1::new();
//     let mut msg = [0u8; 32];
//     thread_rng().fill_bytes(&mut msg);
//     let msg = Message::from_slice(&msg).unwrap();
//     let (sk, _) = s.generate_keypair(&mut thread_rng()).unwrap();

//     bh.iter(|| {
//         let sig = s.sign(&msg, &sk).unwrap();
//         black_box(sig);
//     });
// }
