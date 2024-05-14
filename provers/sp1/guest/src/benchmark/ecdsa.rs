#![no_main]
harness::entrypoint!(main);
use revm_precompile::zk_op::ZkvmOperator;
use std::hint::black_box;
use zk_op::Sp1Operator;

fn main() {
    let sig = black_box([
        0xb5, 0x0b, 0xb6, 0x79, 0x5f, 0x31, 0x74, 0x8a, 0x4d, 0x37, 0xc3, 0xa9, 0x7e, 0xbd, 0x06,
        0xa2, 0x2e, 0xa3, 0x37, 0x71, 0x04, 0x0f, 0x5c, 0x05, 0xd6, 0xe2, 0xbb, 0x2d, 0x38, 0xc6,
        0x22, 0x7c, 0x34, 0x3b, 0x66, 0x59, 0xdb, 0x96, 0x99, 0x59, 0xd9, 0xfd, 0xdb, 0x44, 0xbd,
        0x0d, 0xd9, 0xb9, 0xdd, 0x47, 0x66, 0x6a, 0xb5, 0x28, 0x71, 0x90, 0x1d, 0x17, 0x61, 0xeb,
        0x82, 0xec, 0x87, 0x22,
    ]);
    let recid = black_box(0);
    let msg: [u8; 32] = black_box([
        0x6b, 0x6f, 0x6f, 0x74, 0x68, 0x65, 0x6e, 0x65, 0x76, 0x65, 0x72, 0x67, 0x6f, 0x6e, 0x6e,
        0x61, 0x67, 0x69, 0x76, 0x65, 0x79, 0x6f, 0x75, 0x72, 0x6d, 0x69, 0x6e, 0x64, 0x6f, 0x6e,
        0x6e, 0x61,
    ]);

    let op = Sp1Operator {};
    let res = op.secp256k1_ecrecover(&sig, recid, &msg).unwrap();

    sp1_zkvm::io::commit(&res);
}