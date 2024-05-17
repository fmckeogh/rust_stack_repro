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
use AArch64_SoftwareBreakpoint::*;
use common::*;
pub fn execute_aarch64_instrs_system_exceptions_debug_breakpoint<T: Tracer>(
    state: &mut State,
    tracer: &T,
    comment: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        comment: u16,
    }
    let fn_state = FunctionState {
        comment,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var comment:u16
        let s_0_0: u16 = fn_state.comment;
        // D s_0_1: call AArch64_SoftwareBreakpoint(s_0_0)
        let s_0_1: () = AArch64_SoftwareBreakpoint(state, tracer, s_0_0);
        // N s_0_2: return
        return;
    }
}
