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
pub fn FPProcessNaNs4<T: Tracer>(
    state: &mut State,
    tracer: &T,
    type1: u32,
    type2: u32,
    type3: u32,
    type4: u32,
    op1: Bits,
    op2: Bits,
    op3: Bits,
    op4: Bits,
    fpcr: ProductType5c790c8ef59cc8b2,
    fpexc: bool,
) -> ProductType9d47af446174e9f7 {
    #[derive(Default)]
    struct FunctionState {
        type1: u32,
        type2: u32,
        type3: u32,
        type4: u32,
        op1: Bits,
        op2: Bits,
        op3: Bits,
        op4: Bits,
        fpcr: ProductType5c790c8ef59cc8b2,
        fpexc: bool,
    }
    let fn_state = FunctionState {
        type1,
        type2,
        type3,
        type4,
        op1,
        op2,
        op3,
        op4,
        fpcr,
        fpexc,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType9d47af446174e9f7 {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // N s_0_1: panic s_0_0
        panic!("{:?}", (s_0_0));
    }
}
