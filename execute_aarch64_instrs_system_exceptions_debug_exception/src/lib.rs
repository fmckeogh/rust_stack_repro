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
use DCPSInstruction::*;
use common::*;
pub fn execute_aarch64_instrs_system_exceptions_debug_exception<T: Tracer>(
    state: &mut State,
    tracer: &T,
    target_level: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        target_level: u8,
    }
    let fn_state = FunctionState {
        target_level,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var target_level:u8
        let s_0_0: u8 = fn_state.target_level;
        // D s_0_1: call DCPSInstruction(s_0_0)
        let s_0_1: () = DCPSInstruction(state, tracer, s_0_0);
        // N s_0_2: return
        return;
    }
}
