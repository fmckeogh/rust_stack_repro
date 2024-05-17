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
use u__UNKNOWN_MemoryAttributes::*;
use common::*;
pub fn u__IMPDEF_MemoryAttributes<T: Tracer>(
    state: &mut State,
    tracer: &T,
    x: &'static str,
) -> ProductTypef170cab34335b70c {
    #[derive(Default)]
    struct FunctionState {
        x: &'static str,
    }
    let fn_state = FunctionState {
        x,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: tail-call __UNKNOWN_MemoryAttributes(s_0_0)
        return u__UNKNOWN_MemoryAttributes(state, tracer, s_0_0);
    }
}
