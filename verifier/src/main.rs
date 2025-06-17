use std::path::Path;
#[cfg(target_arch = "aarch64")]
use valida_vm_api_linux_arm::*;
#[cfg(target_arch = "x86_64")]
use valida_vm_api_linux_x86::*;

fn main() {
    let program = Path::new("test_data").join("program.bin");
    let proof = Path::new("test_data").join("proof.bin");
    let stdout = Path::new("test_data").join("stdout");

    let valida = create_valida().unwrap();
    let verify_status_correct_statement = valida.verify(
        &program,
        proof.as_ref(),
        &stdout.as_ref(),
        Default::default(),
        Default::default(),
        Default::default(),
    );
    assert_eq!(verify_status_correct_statement, VerifyStatus::Success);

    println!("All checks completed successfully.");
}
