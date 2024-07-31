use rand::prelude::*;
use {
    solana_program_test::*,
    solana_sdk::{
        instruction::Instruction, pubkey::Pubkey, signature::Signer, transaction::Transaction,
    },
};

#[tokio::test]
async fn test_precise_sqrt_u64_max() {
    let pid = Pubkey::new_unique();
    let pc = ProgramTest::new(
        "rust_decimal_program",
        pid,
        None, // processor!(process_instruction),
    );

    let (mut banks_client, payer, recent_blockhash) = pc.start().await;

    let mut rng = thread_rng();
    let mut max_units_consumed = 0;
    for _ in 0..1000 {
        let data = rng.gen::<u64>().to_le_bytes().to_vec();
        let mut transaction = Transaction::new_with_payer(
            &[Instruction {
                program_id: pid,
                accounts: vec![],
                data,
            }],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);
        let btrws = banks_client
            .simulate_transaction(transaction)
            .await
            .unwrap();
        max_units_consumed =
            max_units_consumed.max(btrws.simulation_details.unwrap().units_consumed);
    }

    println!("max_units_consumed: {max_units_consumed}");
}
