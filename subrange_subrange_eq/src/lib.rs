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
pub fn subrange_subrange_eq<T: Tracer>(
    state: &mut State,
    tracer: &T,
    xs: Bits,
    i: i128,
    j: i128,
    ys: Bits,
    i_: i128,
    j_: i128,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        xs: Bits,
        i: i128,
        j: i128,
        ys: Bits,
        i_: i128,
        j_: i128,
    }
    let fn_state = FunctionState {
        xs,
        i,
        j,
        ys,
        i_,
        j_,
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
        // N s_0_1: panic s_0_0
        panic!("{:?}", (s_0_0));
    }
}
