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
pub fn u__UNKNOWN_GPTTable<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_1467: (),
) -> ProductTypeecb3a6c821d7caab {
    #[derive(Default)]
    struct FunctionState {
        return_value: ProductTypeecb3a6c821d7caab,
        gs_1467: (),
    }
    let fn_state = FunctionState {
        gs_1467,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeecb3a6c821d7caab {
        // D s_0_0: read-var return_value:struct
        let s_0_0: ProductTypeecb3a6c821d7caab = fn_state.return_value;
        // N s_0_1: return s_0_0
        return s_0_0;
    }
}