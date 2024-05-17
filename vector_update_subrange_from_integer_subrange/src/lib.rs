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
pub fn vector_update_subrange_from_integer_subrange<T: Tracer>(
    state: &mut State,
    tracer: &T,
    n1: i128,
    v1: Bits,
    s1: i128,
    e1: i128,
    i: i128,
    s2: i128,
    e2: i128,
) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        n1: i128,
        v1: Bits,
        s1: i128,
        e1: i128,
        i: i128,
        s2: i128,
        e2: i128,
    }
    let fn_state = FunctionState {
        n1,
        v1,
        s1,
        e1,
        i,
        s2,
        e2,
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
