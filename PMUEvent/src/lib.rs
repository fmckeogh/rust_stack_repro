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
use PMUEvent__1::*;
use common::*;
pub fn PMUEvent<T: Tracer>(state: &mut State, tracer: &T, pmuevent: u16) -> () {
    #[derive(Default)]
    struct FunctionState {
        pmuevent: u16,
    }
    let fn_state = FunctionState {
        pmuevent,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #1s : i
        let s_0_0: i128 = 1;
        // D s_0_1: read-var pmuevent:u16
        let s_0_1: u16 = fn_state.pmuevent;
        // D s_0_2: call PMUEvent__1(s_0_1, s_0_0)
        let s_0_2: () = PMUEvent__1(state, tracer, s_0_1, s_0_0);
        // N s_0_3: return
        return;
    }
}
