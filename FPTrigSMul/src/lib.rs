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
pub fn FPTrigSMul<T: Tracer>(
    state: &mut State,
    tracer: &T,
    op1: Bits,
    op2: Bits,
    fpcr: ProductType5c790c8ef59cc8b2,
) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        op1: Bits,
        op2: Bits,
        fpcr: ProductType5c790c8ef59cc8b2,
    }
    let fn_state = FunctionState {
        op1,
        op2,
        fpcr,
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
