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
pub fn Mk_ID_AA64ZFR0_EL1_Type<T: Tracer>(
    state: &mut State,
    tracer: &T,
    v: u64,
) -> ProductType5c790c8ef59cc8b2 {
    #[derive(Default)]
    struct FunctionState {
        v: u64,
    }
    let fn_state = FunctionState {
        v,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_0_0: read-var v:u64
        let s_0_0: u64 = fn_state.v;
        // D s_0_1: create-product struct = ["s_0_0"]
        let s_0_1: ProductType5c790c8ef59cc8b2 = ProductType5c790c8ef59cc8b2 {
            _0: s_0_0,
        };
        // N s_0_2: return s_0_1
        return s_0_1;
    }
}
