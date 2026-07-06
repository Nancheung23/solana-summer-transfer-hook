#[allow(dead_code)]
mod helpers;

use {
    anchor_lang::{
        solana_program::instruction::Instruction, system_program::ID as SYSTEM_PROGRAM_ID,
        InstructionData, ToAccountMetas,
    },
    solana_keypair::Keypair,
    solana_pubkey::Pubkey,
    solana_signer::Signer,
};

use anchor_spl::token_2022::spl_token_2022;
use helpers::{initialize_mint, setup};

#[test]
fn test_initialize() {
    let (mut svm, payer, program_id) = setup();
    let mint = Keypair::new();
    // let payer == owner in this case
    let owner = payer.pubkey();

    // First create the mint via the dedicated instruction
    initialize_mint(&mut svm, &payer, &mint, &program_id);

    // Then initialize the rate limit account
    // add updated seeds
    let rate_limit = Pubkey::find_program_address(
        &[b"rate_limit", mint.pubkey().as_ref(), owner.as_ref()],
        &program_id,
    )
    .0;

    let instruction = Instruction::new_with_bytes(
        program_id,
        &solana_summer_transfer_hook::instruction::Initialize {}.data(),
        solana_summer_transfer_hook::accounts::Initialize {
            payer: payer.pubkey(),
            rate_limit,
            system_program: SYSTEM_PROGRAM_ID,
            mint: mint.pubkey(),
            owner: owner,
            token_program: spl_token_2022::ID,
        }
        .to_account_metas(None),
    );

    let blockhash = svm.latest_blockhash();
    let msg = solana_message::Message::new_with_blockhash(
        &[instruction],
        Some(&payer.pubkey()),
        &blockhash,
    );
    let tx = solana_transaction::versioned::VersionedTransaction::try_new(
        solana_message::VersionedMessage::Legacy(msg),
        &[&payer],
    )
    .unwrap();

    let res = svm.send_transaction(tx);
    assert!(res.is_ok(), "Initialization failed: {:?}", res.err());
}
