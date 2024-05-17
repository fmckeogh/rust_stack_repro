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
pub fn MaxImplementedSVL<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_331516: (),
) -> i128 {
    #[derive(Default)]
    struct FunctionState {
        gs_331516: (),
    }
    let fn_state = FunctionState {
        gs_331516,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_0_0: const #1576u : u32
        let s_0_0: u32 = 1576;
        // D s_0_1: read-reg s_0_0:i
        let s_0_1: i128 = {
            let value = state.read_register::<i128>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // N s_0_2: return s_0_1
        return s_0_1;
    }
}
