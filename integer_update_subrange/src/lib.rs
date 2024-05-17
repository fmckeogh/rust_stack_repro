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
use common::*;
pub fn integer_update_subrange<T: Tracer>(
    state: &mut State,
    tracer: &T,
    i: i128,
    hi: i128,
    lo: i128,
    x: Bits,
) -> i128 {
    #[derive(Default)]
    struct FunctionState {
        i: i128,
        hi: i128,
        lo: i128,
        x: Bits,
    }
    let fn_state = FunctionState {
        i,
        hi,
        lo,
        x,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // N s_0_1: panic s_0_0
        panic!("{:?}", (s_0_0));
    }
}
