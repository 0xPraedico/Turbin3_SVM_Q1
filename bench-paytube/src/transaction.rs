//! PayTube's custom transaction format, tailored specifically for SOL or SPL
//! token transfers.
//!
//! Mostly for demonstration purposes, to show how projects may use completely
//! different transactions in their protocol, then convert the resulting state
//! transitions into the necessary transactions for the base chain - in this
//! case Solana.

use {
    solana_sdk::{
        instruction::Instruction as SolanaInstruction,
        pubkey::Pubkey,
        system_instruction,
        transaction::{
            SanitizedTransaction as SolanaSanitizedTransaction, Transaction as SolanaTransaction,
        },
    },
    spl_associated_token_account::get_associated_token_address,
    std::collections::HashSet,
    std::sync::OnceLock,
};

static EMPTY_HASHSET: OnceLock<HashSet<Pubkey>> = OnceLock::new();

/// A simple PayTube transaction. Transfers SPL tokens or SOL from one account
/// to another.
///
/// A `None` value for `mint` represents native SOL.
#[derive(Copy, Clone)]
pub struct PayTubeTransaction {
    pub mint: Option<Pubkey>,
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
}

impl From<&PayTubeTransaction> for SolanaInstruction {
    fn from(value: &PayTubeTransaction) -> Self {
        let PayTubeTransaction {
            mint,
            from,
            to,
            amount,
        } = value;
        if let Some(mint) = mint {
            let source_pubkey = get_associated_token_address(from, mint);
            let destination_pubkey = get_associated_token_address(to, mint);
            return spl_token::instruction::transfer(
                &spl_token::id(),
                &source_pubkey,
                &destination_pubkey,
                from,
                &[],
                *amount,
            )
            .unwrap();
        }
        system_instruction::transfer(from, to, *amount)
    }
}

impl From<&PayTubeTransaction> for SolanaTransaction {
    fn from(value: &PayTubeTransaction) -> Self {
        SolanaTransaction::new_with_payer(&[SolanaInstruction::from(value)], Some(&value.from))
    }
}

impl From<&PayTubeTransaction> for SolanaSanitizedTransaction {
    fn from(value: &PayTubeTransaction) -> Self {
        let hashset = EMPTY_HASHSET.get_or_init(HashSet::new);
        SolanaSanitizedTransaction::try_from_legacy_transaction(
            SolanaTransaction::from(value),
            hashset,
        )
        .unwrap()
    }
}

/// Original version - simple and direct
pub fn create_svm_transactions_original(
    paytube_transactions: &[PayTubeTransaction],
) -> Vec<SolanaSanitizedTransaction> {
    paytube_transactions
        .iter()
        .map(SolanaSanitizedTransaction::from)
        .collect()
}

/// Version with memory pre-allocation for optimization
pub fn create_svm_transactions_preallocated(
    paytube_transactions: &[PayTubeTransaction],
) -> Vec<SolanaSanitizedTransaction> {
    let mut result = Vec::with_capacity(paytube_transactions.len());
    result.extend(paytube_transactions.iter().map(SolanaSanitizedTransaction::from));
    result
}