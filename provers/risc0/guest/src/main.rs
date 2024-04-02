#![no_main]
use risc0_zkvm::guest::env;
risc0_zkvm::guest::entry!(main);

use raiko_lib::{
    builder::{BlockBuilderStrategy, TaikoStrategy},
    input::{GuestInput, GuestOutput, WrappedHeader},
};
use raiko_lib::protocol_instance::assemble_protocol_instance;
use raiko_lib::protocol_instance::EvidenceType;

fn main() {

    let input: GuestInput = env::read();
    let build_result = TaikoStrategy::build_from(&input);

    // TODO: cherry-pick risc0 latest output
    let output = match &build_result {
        Ok((header, _mpt_node)) => {
            let pi = assemble_protocol_instance(&input, &header)
                .expect("Failed to assemble protocol instance")
                .instance_hash(EvidenceType::Risc0);
            GuestOutput::Success((WrappedHeader {header: header.clone() }, pi))
        }
        Err(_) => {
            GuestOutput::Failure
        }
    };

    env::commit(&output);
}