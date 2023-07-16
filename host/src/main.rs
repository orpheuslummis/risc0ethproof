use methods::{ETHPROOF_ELF, ETHPROOF_ID};
use risc0_zkvm::{
    default_executor_from_elf,
    serde::{from_slice, to_vec},
    ExecutorEnv,
};

fn main() {
    let env = ExecutorEnv::builder().build().unwrap();

    // TODO: add guest input to the executor environment using
    // ExecutorEnvBuilder::add_input().
    // To access this method, you'll need to use the alternate construction
    // ExecutorEnv::builder(), which creates an ExecutorEnvBuilder. When you're
    // done adding input, call ExecutorEnvBuilder::build().

    // For example:
    // let env = ExecutorEnv::builder().add_input(&vec).build().unwrap();

    let mut exec = default_executor_from_elf(env, ETHPROOF_ELF).unwrap();

    let session = exec.run().unwrap();

    let receipt = session.prove().unwrap();

    receipt.verify(ETHPROOF_ID).unwrap();
}
