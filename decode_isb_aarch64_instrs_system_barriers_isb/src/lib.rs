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
pub fn decode_isb_aarch64_instrs_system_barriers_isb<T: Tracer>(
    state: &mut State,
    tracer: &T,
    opc: u8,
    CRm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        opc: u8,
        CRm: u8,
    }
    let fn_state = FunctionState {
        opc,
        CRm,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_0_0: return
        return;
    }
}
