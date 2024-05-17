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
pub fn MaxImplementedVL<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_331515: (),
) -> i128 {
    #[derive(Default)]
    struct FunctionState {
        gs_331515: (),
    }
    let fn_state = FunctionState {
        gs_331515,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_0_0: const #808u : u32
        let s_0_0: u32 = 808;
        // D s_0_1: read-reg s_0_0:i64
        let s_0_1: i64 = {
            let value = state.read_register::<i64>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (i128::try_from(s_0_1).unwrap());
        // N s_0_3: return s_0_2
        return s_0_2;
    }
}
