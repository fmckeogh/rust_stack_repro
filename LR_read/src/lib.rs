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
use R_read::*;
use common::*;
pub fn LR_read<T: Tracer>(state: &mut State, tracer: &T, gs_37878: ()) -> u32 {
    #[derive(Default)]
    struct FunctionState {
        gs_37878: (),
    }
    let fn_state = FunctionState {
        gs_37878,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_0_0: const #14s : i
        let s_0_0: i128 = 14;
        // S s_0_1: tail-call R_read(s_0_0)
        return R_read(state, tracer, s_0_0);
    }
}
