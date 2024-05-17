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
pub fn Hint_Prefetch<T: Tracer>(
    state: &mut State,
    tracer: &T,
    address: u64,
    hint: u32,
    target: i128,
    stream: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        address: u64,
        hint: u32,
        target: i128,
        stream: bool,
    }
    let fn_state = FunctionState {
        address,
        hint,
        target,
        stream,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_0_0: return
        return;
    }
}
