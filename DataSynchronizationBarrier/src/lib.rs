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
pub fn DataSynchronizationBarrier<T: Tracer>(
    state: &mut State,
    tracer: &T,
    domain: u32,
    types: u32,
    nXS: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        domain: u32,
        types: u32,
        nXS: bool,
    }
    let fn_state = FunctionState {
        domain,
        types,
        nXS,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // N s_0_1: panic s_0_0
        panic!("{:?}", (s_0_0));
    }
}
