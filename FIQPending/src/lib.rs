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
pub fn FIQPending<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_331358: (),
) -> ProductType8b847afc727d5818 {
    #[derive(Default)]
    struct FunctionState {
        gs_331358: (),
    }
    let fn_state = FunctionState {
        gs_331358,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_0_0: const #0u : u8
        let s_0_0: bool = false;
        // C s_0_1: const #0u : u8
        let s_0_1: bool = false;
        // D s_0_2: create-product struct = ["s_0_0", "s_0_1"]
        let s_0_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_0_0,
            _1: s_0_1,
        };
        // N s_0_3: return s_0_2
        return s_0_2;
    }
}
