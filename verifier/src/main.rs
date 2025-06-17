use std::{path::Path, sync::LazyLock, thread};
#[cfg(target_arch = "aarch64")]
use valida_vm_api_linux_arm::*;
#[cfg(target_arch = "x86_64")]
use valida_vm_api_linux_x86::*;

static VALIDA: LazyLock<Valida> = LazyLock::new(|| create_valida().unwrap());

fn main() {
    let program = Path::new("test_data").join("program.bin");
    let proof = Path::new("test_data").join("proof.bin");
    let stdout = Path::new("test_data").join("stdout");

    let num_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    let handles: Vec<_> = (0..num_threads)
        .map(|i| {
            let program = program.clone();
            let proof = proof.clone();
            let stdout = stdout.clone();

            thread::spawn(move || {
                let status = VALIDA.verify(
                    &program,
                    &proof,
                    &stdout,
                    Default::default(),
                    Default::default(),
                );
                assert_eq!(status, VerifyStatus::Success, "Thread {i} failed");
                println!("Thread {i} verification successful");
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    println!("All checks completed successfully.");
}
