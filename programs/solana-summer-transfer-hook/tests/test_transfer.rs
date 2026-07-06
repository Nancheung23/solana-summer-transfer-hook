#[allow(dead_code)]
mod helpers;

use {solana_keypair::Keypair, solana_signer::Signer};

use helpers::{
    build_transfer_with_hook_ix, create_ata, mint_tokens, send_ix, setup,
    setup_mint_and_extra_metas,
};

#[test]
fn test_transfer_with_hook() {
    let (mut svm, payer, program_id) = setup();
    let mint = Keypair::new();

    setup_mint_and_extra_metas(&mut svm, &payer, &mint, &program_id);

    let receiver = Keypair::new();
    svm.airdrop(&receiver.pubkey(), 1_000_000_000).unwrap();

    let sender_ata = create_ata(&mut svm, &payer, &payer.pubkey(), &mint.pubkey());
    mint_tokens(&mut svm, &payer, &mint.pubkey(), &sender_ata, 1_000_000);

    let receiver_ata = create_ata(&mut svm, &payer, &receiver.pubkey(), &mint.pubkey());

    let amount = 100u64;
    let decimals = 9u8;

    let ix = build_transfer_with_hook_ix(
        &sender_ata,
        &receiver_ata,
        &mint.pubkey(),
        &payer.pubkey(),
        &program_id,
        amount,
        decimals,
    );

    send_ix(&mut svm, ix, &payer, &[&payer]);

    let receiver_account = svm.get_account(&receiver_ata).unwrap();
    assert!(receiver_account.lamports > 0);

    println!("Transfer successful!");
}
