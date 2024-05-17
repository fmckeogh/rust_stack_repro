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
use Halt__1::*;
use common::*;
pub fn execute_aarch64_instrs_system_exceptions_debug_halt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_155034: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_155034: (),
    }
    let fn_state = FunctionState {
        gs_155034,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #0u : u8
        let s_0_0: bool = false;
        // C s_0_1: const #1152u : u32
        let s_0_1: u32 = 1152;
        // D s_0_2: read-reg s_0_1:u8
        let s_0_2: u8 = {
            let value = state.read_register::<u8>(s_0_1 as isize);
            tracer.read_register(s_0_1 as isize, value);
            value
        };
        // D s_0_3: call Halt__1(s_0_2, s_0_0)
        let s_0_3: () = Halt__1(state, tracer, s_0_2, s_0_0);
        // N s_0_4: return
        return;
    }
}
