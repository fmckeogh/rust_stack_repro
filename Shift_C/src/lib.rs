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
pub fn Shift_C<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: Bits,
    srtype: u32,
    amount: i128,
    carry_in: bool,
) -> ProductTypef506aa96a892fe52 {
    #[derive(Default)]
    struct FunctionState {
        value_name: Bits,
        srtype: u32,
        amount: i128,
        carry_in: bool,
    }
    let fn_state = FunctionState {
        value_name,
        srtype,
        amount,
        carry_in,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef506aa96a892fe52 {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // N s_0_1: panic s_0_0
        panic!("{:?}", (s_0_0));
    }
}
