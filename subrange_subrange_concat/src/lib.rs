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
pub fn subrange_subrange_concat<T: Tracer>(
    state: &mut State,
    tracer: &T,
    s: i128,
    xs: Bits,
    i: i128,
    j: i128,
    ys: Bits,
    i_: i128,
    j_: i128,
) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        s: i128,
        xs: Bits,
        i: i128,
        j: i128,
        ys: Bits,
        i_: i128,
        j_: i128,
    }
    let fn_state = FunctionState {
        s,
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
    ) -> Bits {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // N s_0_1: panic s_0_0
        panic!("{:?}", (s_0_0));
    }
}
