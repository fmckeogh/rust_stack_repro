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
pub fn WalkMemAttrs<T: Tracer>(
    state: &mut State,
    tracer: &T,
    sh: u8,
    irgn: u8,
    orgn: u8,
) -> ProductTypef170cab34335b70c {
    #[derive(Default)]
    struct FunctionState {
        sh: u8,
        irgn: u8,
        orgn: u8,
    }
    let fn_state = FunctionState {
        sh,
        irgn,
        orgn,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // N s_0_1: panic s_0_0
        panic!("{:?}", (s_0_0));
    }
}
