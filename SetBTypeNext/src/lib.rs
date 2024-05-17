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
pub fn SetBTypeNext<T: Tracer>(state: &mut State, tracer: &T, x: u8) -> () {
    #[derive(Default)]
    struct FunctionState {
        x: u8,
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
    ) -> () {
        // D s_0_0: read-var x:u8
        let s_0_0: u8 = fn_state.x;
        // C s_0_1: const #90912u : u32
        let s_0_1: u32 = 90912;
        // N s_0_2: write-reg s_0_1 <= s_0_0
        let s_0_2: () = {
            state.write_register::<u8>(s_0_1 as isize, s_0_0);
            tracer.write_register(s_0_1 as isize, s_0_0);
        };
        // N s_0_3: return
        return;
    }
}
