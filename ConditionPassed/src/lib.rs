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
use AArch32_CurrentCond::*;
use ConditionHolds::*;
use common::*;
pub fn ConditionPassed<T: Tracer>(state: &mut State, tracer: &T, gs_31503: ()) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_31503: (),
    }
    let fn_state = FunctionState {
        gs_31503,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call AArch32_CurrentCond(s_0_0)
        let s_0_1: u8 = AArch32_CurrentCond(state, tracer, s_0_0);
        // S s_0_2: tail-call ConditionHolds(s_0_1)
        return ConditionHolds(state, tracer, s_0_1);
    }
}
