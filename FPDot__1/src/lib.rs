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
pub fn FPDot__1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    op1_a: Bits,
    op1_b: Bits,
    op2_a: Bits,
    op2_b: Bits,
    fpcr_in: ProductType5c790c8ef59cc8b2,
    isbfloat16: bool,
    fpexc: bool,
) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        op1_a: Bits,
        op1_b: Bits,
        op2_a: Bits,
        op2_b: Bits,
        fpcr_in: ProductType5c790c8ef59cc8b2,
        isbfloat16: bool,
        fpexc: bool,
    }
    let fn_state = FunctionState {
        op1_a,
        op1_b,
        op2_a,
        op2_b,
        fpcr_in,
        isbfloat16,
        fpexc,
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
