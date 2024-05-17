#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
pub use common::*;
use u__DecodeA64::u__DecodeA64;
pub fn decode_execute<T: Tracer>(
    value: u32,
    state: &mut State,
    tracer: &T,
) -> ExecuteResult {
    state.write_register(REG_SEE, 0u64);
    tracer.begin(value, state.read_register::<u64>(REG_U_PC));
    u__DecodeA64(state, tracer, i128::from(state.read_register::<u64>(REG_U_PC)), value);
    if !state.read_register::<bool>(REG_U__BRANCHTAKEN) {
        let pc = state.read_register::<u64>(REG_U_PC);
        state.write_register(REG_U_PC, pc + 4);
    }
    state.write_register(REG_U__BRANCHTAKEN, false);
    tracer.end();
    ExecuteResult::Ok
}
