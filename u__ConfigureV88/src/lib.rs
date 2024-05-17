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
use u__ConfigureV87::*;
use common::*;
pub fn u__ConfigureV88<T: Tracer>(state: &mut State, tracer: &T, enabled: bool) -> () {
    #[derive(Default)]
    struct FunctionState {
        enabled: bool,
    }
    let fn_state = FunctionState {
        enabled,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var enabled:u8
        let s_0_0: bool = fn_state.enabled;
        // C s_0_1: const #17704u : u32
        let s_0_1: u32 = 17704;
        // N s_0_2: write-reg s_0_1 <= s_0_0
        let s_0_2: () = {
            state.write_register::<bool>(s_0_1 as isize, s_0_0);
            tracer.write_register(s_0_1 as isize, s_0_0);
        };
        // D s_0_3: read-var enabled:u8
        let s_0_3: bool = fn_state.enabled;
        // N s_0_4: branch s_0_3 b2 b1
        if s_0_3 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var enabled:u8
        let s_2_0: bool = fn_state.enabled;
        // D s_2_1: call __ConfigureV87(s_2_0)
        let s_2_1: () = u__ConfigureV87(state, tracer, s_2_0);
        // N s_2_2: return
        return;
    }
}
