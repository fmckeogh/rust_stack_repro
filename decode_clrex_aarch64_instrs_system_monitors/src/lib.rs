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
use execute_aarch64_instrs_system_monitors::*;
use common::*;
pub fn decode_clrex_aarch64_instrs_system_monitors<T: Tracer>(
    state: &mut State,
    tracer: &T,
    CRm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        CRm: u8,
    }
    let fn_state = FunctionState {
        CRm,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call execute_aarch64_instrs_system_monitors(s_0_0)
        let s_0_1: () = execute_aarch64_instrs_system_monitors(state, tracer, s_0_0);
        // N s_0_2: return
        return;
    }
}
