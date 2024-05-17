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
use PC_read__1::*;
use common::*;
pub fn PCStoreValue<T: Tracer>(state: &mut State, tracer: &T, gs_31425: ()) -> u32 {
    #[derive(Default)]
    struct FunctionState {
        gs_31425: (),
    }
    let fn_state = FunctionState {
        gs_31425,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: tail-call PC_read__1(s_0_0)
        return PC_read__1(state, tracer, s_0_0);
    }
}
