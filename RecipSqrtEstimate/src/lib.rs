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
pub fn RecipSqrtEstimate<T: Tracer>(
    state: &mut State,
    tracer: &T,
    a_in: i128,
    increasedprecision_name: bool,
) -> i128 {
    #[derive(Default)]
    struct FunctionState {
        a_in: i128,
        increasedprecision_name: bool,
    }
    let fn_state = FunctionState {
        a_in,
        increasedprecision_name,
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
