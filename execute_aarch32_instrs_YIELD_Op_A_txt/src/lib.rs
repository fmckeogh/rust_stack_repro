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
use Hint_Yield::*;
use common::*;
pub fn execute_aarch32_instrs_YIELD_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_323155: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_323155: (),
    }
    let fn_state = FunctionState {
        gs_323155,
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
        // S s_0_1: call Hint_Yield(s_0_0)
        let s_0_1: () = Hint_Yield(state, tracer, s_0_0);
        // N s_0_2: return
        return;
    }
}
