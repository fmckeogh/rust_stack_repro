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
pub fn GranuleProtectionCheck<T: Tracer>(
    state: &mut State,
    tracer: &T,
    addrdesc: ProductTypece7c66ccb2cab13e,
    accdesc: ProductType9878976b5bcce9c9,
) -> ProductType396b95aa74979079 {
    #[derive(Default)]
    struct FunctionState {
        addrdesc: ProductTypece7c66ccb2cab13e,
        accdesc: ProductType9878976b5bcce9c9,
    }
    let fn_state = FunctionState {
        addrdesc,
        accdesc,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType396b95aa74979079 {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // N s_0_1: panic s_0_0
        panic!("{:?}", (s_0_0));
    }
}
