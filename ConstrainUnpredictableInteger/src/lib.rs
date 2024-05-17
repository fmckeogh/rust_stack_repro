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
pub fn ConstrainUnpredictableInteger<T: Tracer>(
    state: &mut State,
    tracer: &T,
    low: i128,
    high: i128,
    which: u32,
) -> ProductType396b95aa74979079 {
    #[derive(Default)]
    struct FunctionState {
        ga_398840: ProductType396b95aa74979079,
        low: i128,
        high: i128,
        which: u32,
    }
    let fn_state = FunctionState {
        low,
        high,
        which,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType396b95aa74979079 {
        // D s_0_0: read-var ga#398840:struct
        let s_0_0: ProductType396b95aa74979079 = fn_state.ga_398840;
        // N s_0_1: return s_0_0
        return s_0_0;
    }
}
