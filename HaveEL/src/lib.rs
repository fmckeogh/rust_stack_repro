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
pub fn HaveEL<T: Tracer>(state: &mut State, tracer: &T, el: u8) -> bool {
    #[derive(Default)]
    struct FunctionState {
        el: u8,
    }
    let fn_state = FunctionState {
        el,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #1u : u8
        let s_0_0: bool = true;
        // N s_0_1: return s_0_0
        return s_0_0;
    }
}
