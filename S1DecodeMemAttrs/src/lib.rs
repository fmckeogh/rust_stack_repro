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
pub fn S1DecodeMemAttrs<T: Tracer>(
    state: &mut State,
    tracer: &T,
    attr_in: u8,
    sh: u8,
    s1aarch64: bool,
    walkparams: ProductTypeef284266e139aee2,
) -> ProductTypef170cab34335b70c {
    #[derive(Default)]
    struct FunctionState {
        attr_in: u8,
        sh: u8,
        s1aarch64: bool,
        walkparams: ProductTypeef284266e139aee2,
    }
    let fn_state = FunctionState {
        attr_in,
        sh,
        s1aarch64,
        walkparams,
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
