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
use R_set::*;
use common::*;
pub fn LR_write<T: Tracer>(state: &mut State, tracer: &T, value_name: u32) -> () {
    #[derive(Default)]
    struct FunctionState {
        value_name: u32,
    }
    let fn_state = FunctionState {
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #14s : i
        let s_0_0: i128 = 14;
        // D s_0_1: read-var value_name:u32
        let s_0_1: u32 = fn_state.value_name;
        // D s_0_2: call R_set(s_0_0, s_0_1)
        let s_0_2: () = R_set(state, tracer, s_0_0, s_0_1);
        // N s_0_3: return
        return;
    }
}
