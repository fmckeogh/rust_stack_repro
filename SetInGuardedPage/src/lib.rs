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
pub fn SetInGuardedPage<T: Tracer>(
    state: &mut State,
    tracer: &T,
    guardedpage: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        guardedpage: bool,
    }
    let fn_state = FunctionState {
        guardedpage,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var guardedpage:u8
        let s_0_0: bool = fn_state.guardedpage;
        // C s_0_1: const #102520u : u32
        let s_0_1: u32 = 102520;
        // N s_0_2: write-reg s_0_1 <= s_0_0
        let s_0_2: () = {
            state.write_register::<bool>(s_0_1 as isize, s_0_0);
            tracer.write_register(s_0_1 as isize, s_0_0);
        };
        // N s_0_3: return
        return;
    }
}
