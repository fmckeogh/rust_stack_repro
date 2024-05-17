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
pub fn u_update_TTBCR_Type_EPD1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    v: ProductType700c18a878c5601b,
    x: bool,
) -> ProductType700c18a878c5601b {
    #[derive(Default)]
    struct FunctionState {
        v: ProductType700c18a878c5601b,
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
    ) -> ProductType700c18a878c5601b {
        // D s_0_0: read-var v:struct
        let s_0_0: ProductType700c18a878c5601b = fn_state.v;
        // N s_0_1: return s_0_0
        return s_0_0;
    }
}
