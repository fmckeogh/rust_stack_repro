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
pub fn GPCNoFault<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_10007: (),
) -> ProductType396b95aa74979079 {
    #[derive(Default)]
    struct FunctionState {
        result: ProductType396b95aa74979079,
        gs_10007: (),
    }
    let fn_state = FunctionState {
        gs_10007,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType396b95aa74979079 {
        // C s_0_0: const #0u : u32
        let s_0_0: u32 = 0;
        // D s_0_1: write-var result.0 <= s_0_0
        fn_state.result._0 = s_0_0;
        // D s_0_2: read-var result:struct
        let s_0_2: ProductType396b95aa74979079 = fn_state.result;
        // N s_0_3: return s_0_2
        return s_0_2;
    }
}
