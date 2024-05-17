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
pub fn sail_mem_read<T: Tracer>(
    state: &mut State,
    tracer: &T,
    request: ProductTypeee12e330a5f80ce,
) -> SumTypebfdf2f926abd32c5 {
    #[derive(Default)]
    struct FunctionState {
        request: ProductTypeee12e330a5f80ce,
    }
    let fn_state = FunctionState {
        request,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> SumTypebfdf2f926abd32c5 {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // N s_0_1: panic s_0_0
        panic!("{:?}", (s_0_0));
    }
}
