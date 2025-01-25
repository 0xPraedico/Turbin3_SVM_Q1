use criterion::{black_box, criterion_group, criterion_main, Criterion};
use paytube_svm::transaction::{
    PayTubeTransaction, 
    create_svm_transactions_original,
    create_svm_transactions_preallocated,
};
use solana_sdk::pubkey::Pubkey;

fn compare_single_transaction(c: &mut Criterion) {
    let mut group = c.benchmark_group("Single Transaction Comparison");
    
    // Test data preparation
    let from = Pubkey::new_unique();
    let to = Pubkey::new_unique();
    let mint = Pubkey::new_unique();
    
    // Creating an SPL transaction
    let spl_transaction = PayTubeTransaction {
        mint: Some(mint),
        from,
        to,
        amount: 1000,
    };

    // Creating a native SOL transaction
    let sol_transaction = PayTubeTransaction {
        mint: None,
        from,
        to,
        amount: 1000,
    };

    // Testing SPL transaction
    group.bench_function("original_spl", |b| {
        b.iter(|| black_box(create_svm_transactions_original(&[spl_transaction])))
    });

    group.bench_function("preallocated_spl", |b| {
        b.iter(|| black_box(create_svm_transactions_preallocated(&[spl_transaction])))
    });

    // Testing native SOL transaction
    group.bench_function("original_sol", |b| {
        b.iter(|| black_box(create_svm_transactions_original(&[sol_transaction])))
    });

    group.bench_function("preallocated_sol", |b| {
        b.iter(|| black_box(create_svm_transactions_preallocated(&[sol_transaction])))
    });

    group.finish();
}

fn compare_bulk_transactions(c: &mut Criterion) {
    let mut group = c.benchmark_group("Bulk Transactions (100) Comparison");
    
    // Configure the benchmark parameters
    group.measurement_time(std::time::Duration::from_secs(15));  // Increase measurement time to 15 seconds
    
    // Test data preparation
    let from = Pubkey::new_unique();
    let to = Pubkey::new_unique();
    let mint = Pubkey::new_unique();
    
    // Creating an SPL transaction template
    let spl_transaction = PayTubeTransaction {
        mint: Some(mint),
        from,
        to,
        amount: 1000,
    };

    // Creating a native SOL transaction template
    let sol_transaction = PayTubeTransaction {
        mint: None,
        from,
        to,
        amount: 1000,
    };

    // Create vectors with 100 transactions
    let spl_transactions = vec![spl_transaction; 100];
    let sol_transactions = vec![sol_transaction; 100];

    // Testing SPL transactions
    group.bench_function("original_spl_bulk", |b| {
        b.iter(|| black_box(create_svm_transactions_original(&spl_transactions)))
    });

    group.bench_function("preallocated_spl_bulk", |b| {
        b.iter(|| black_box(create_svm_transactions_preallocated(&spl_transactions)))
    });

    // Testing native SOL transactions
    group.bench_function("original_sol_bulk", |b| {
        b.iter(|| black_box(create_svm_transactions_original(&sol_transactions)))
    });

    group.bench_function("preallocated_sol_bulk", |b| {
        b.iter(|| black_box(create_svm_transactions_preallocated(&sol_transactions)))
    });

    group.finish();
}

criterion_group!(benches, compare_single_transaction, compare_bulk_transactions);
criterion_main!(benches);
