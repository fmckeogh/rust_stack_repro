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
use Rmode_set::*;
use common::*;
pub fn R_set<T: Tracer>(state: &mut State, tracer: &T, n: i128, value_name: u32) -> () {
    #[derive(Default)]
    struct FunctionState {
        n: i128,
        value_name: u32,
    }
    let fn_state = FunctionState {
        n,
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #16983u : u32
        let s_0_0: u32 = 16983;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: read-var n:i
        let s_0_2: i128 = fn_state.n;
        // D s_0_3: read-var value_name:u32
        let s_0_3: u32 = fn_state.value_name;
        // D s_0_4: call Rmode_set(s_0_2, s_0_1, s_0_3)
        let s_0_4: () = Rmode_set(state, tracer, s_0_2, s_0_1, s_0_3);
        // N s_0_5: return
        return;
    }
}
