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
pub fn u_update_ICC_IGRPEN1_EL1_Type_Enable<T: Tracer>(
    state: &mut State,
    tracer: &T,
    v: ProductType5c790c8ef59cc8b2,
    x: bool,
) -> ProductType5c790c8ef59cc8b2 {
    #[derive(Default)]
    struct FunctionState {
        v: ProductType5c790c8ef59cc8b2,
        x: bool,
    }
    let fn_state = FunctionState {
        v,
        x,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_0_0: read-var v:struct
        let s_0_0: ProductType5c790c8ef59cc8b2 = fn_state.v;
        // N s_0_1: return s_0_0
        return s_0_0;
    }
}
