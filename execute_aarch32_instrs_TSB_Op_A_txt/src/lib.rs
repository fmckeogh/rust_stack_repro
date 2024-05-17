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
use TraceSynchronizationBarrier::*;
use common::*;
pub fn execute_aarch32_instrs_TSB_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_325972: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_325972: (),
    }
    let fn_state = FunctionState {
        gs_325972,
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
        // S s_0_1: call TraceSynchronizationBarrier(s_0_0)
        let s_0_1: () = TraceSynchronizationBarrier(state, tracer, s_0_0);
        // N s_0_2: return
        return;
    }
}
