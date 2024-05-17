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
use u__UNKNOWN_TLBLine::*;
use common::*;
pub fn S1TLBLookup<T: Tracer>(
    state: &mut State,
    tracer: &T,
    tlbcontext: ProductTypec0d0fb0603850c4c,
) -> ProductTypeeb828c17bbe5e68 {
    #[derive(Default)]
    struct FunctionState {
        tlbcontext: ProductTypec0d0fb0603850c4c,
    }
    let fn_state = FunctionState {
        tlbcontext,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeeb828c17bbe5e68 {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: tail-call __UNKNOWN_TLBLine(s_0_0)
        return u__UNKNOWN_TLBLine(state, tracer, s_0_0);
    }
}
