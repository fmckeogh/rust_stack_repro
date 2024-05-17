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
pub fn neq_any_r_<T: Tracer>(state: &mut State, tracer: &T, x: f32, y: f32) -> bool {
    #[derive(Default)]
    struct FunctionState {
        x: f32,
        y: f32,
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
    ) -> bool {
        // D s_0_0: read-var x:f32
        let s_0_0: f32 = fn_state.x;
        // D s_0_1: read-var y:f32
        let s_0_1: f32 = fn_state.y;
        // D s_0_2: cmp-eq s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) == (s_0_1));
        // D s_0_3: not s_0_2
        let s_0_3: bool = !s_0_2;
        // N s_0_4: return s_0_3
        return s_0_3;
    }
}
