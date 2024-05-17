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
extern crate alloc;
use common::*;
pub fn decode_udf_perm_undef_aarch64_instrs_udf<T: Tracer>(
    state: &mut State,
    tracer: &T,
    imm16: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        imm16: u16,
    }
    let fn_state = FunctionState {
        imm16,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_0_0: panic
        panic!("{:?}", ());
        // N s_0_1: return
        return;
    }
}
