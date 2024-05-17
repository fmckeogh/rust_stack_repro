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
use fdiv_int::*;
use common::*;
pub fn Align_int<T: Tracer>(state: &mut State, tracer: &T, x: i128, y: i128) -> i128 {
    #[derive(Default)]
    struct FunctionState {
        x: i128,
        y: i128,
    }
    let fn_state = FunctionState {
        x,
        y,
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
        // D s_0_1: read-var y:i
        let s_0_1: i128 = fn_state.y;
        // D s_0_2: call fdiv_int(s_0_0, s_0_1)
        let s_0_2: i128 = fdiv_int(state, tracer, s_0_0, s_0_1);
        // D s_0_3: read-var y:i
        let s_0_3: i128 = fn_state.y;
        // D s_0_4: mul s_0_2 s_0_3
        let s_0_4: i128 = ((s_0_2) * (s_0_3));
        // N s_0_5: return s_0_4
        return s_0_4;
    }
}
