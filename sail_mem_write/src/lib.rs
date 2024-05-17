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
pub fn sail_mem_write<T: Tracer>(
    state: &mut State,
    tracer: &T,
    request: ProductType5c38c56b0a400358,
) -> SumType7151e9c01acfacea {
    #[derive(Default)]
    struct FunctionState {
        request: ProductType5c38c56b0a400358,
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
    ) -> SumType7151e9c01acfacea {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // N s_0_1: panic s_0_0
        panic!("{:?}", (s_0_0));
    }
}
