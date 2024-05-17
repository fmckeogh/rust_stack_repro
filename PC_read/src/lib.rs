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
pub fn PC_read<T: Tracer>(state: &mut State, tracer: &T, gs_3399: ()) -> u64 {
    #[derive(Default)]
    struct FunctionState {
        gs_3399: (),
    }
    let fn_state = FunctionState {
        gs_3399,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_0_0: const #12744u : u32
        let s_0_0: u32 = 12744;
        // D s_0_1: read-reg s_0_0:u64
        let s_0_1: u64 = {
            let value = state.read_register::<u64>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // N s_0_2: return s_0_1
        return s_0_1;
    }
}
