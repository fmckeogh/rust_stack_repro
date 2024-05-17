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
pub fn u__id<T: Tracer>(state: &mut State, tracer: &T, x: i128) -> i128 {
    #[derive(Default)]
    struct FunctionState {
        x: i128,
    }
    let fn_state = FunctionState {
        x,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_0_0: read-var x:i
        let s_0_0: i128 = fn_state.x;
        // N s_0_1: return s_0_0
        return s_0_0;
    }
}
