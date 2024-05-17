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
use CurrentSecurityState::*;
use DebugTargetFrom::*;
use common::*;
pub fn DebugTarget<T: Tracer>(state: &mut State, tracer: &T, gs_24615: ()) -> u8 {
    #[derive(Default)]
    struct FunctionState {
        gs_24615: (),
    }
    let fn_state = FunctionState {
        gs_24615,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call CurrentSecurityState(s_0_0)
        let s_0_1: u32 = CurrentSecurityState(state, tracer, s_0_0);
        // S s_0_2: tail-call DebugTargetFrom(s_0_1)
        return DebugTargetFrom(state, tracer, s_0_1);
    }
}
