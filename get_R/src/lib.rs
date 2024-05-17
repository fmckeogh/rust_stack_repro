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
use read_gpr::*;
use common::*;
pub fn get_R<T: Tracer>(state: &mut State, tracer: &T, n: i64) -> u64 {
    #[derive(Default)]
    struct FunctionState {
        return_value: u64,
        n: i64,
    }
    let fn_state = FunctionState {
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_0_0: const #20544u : u32
        let s_0_0: u32 = 20544;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: bool = {
            let value = state.read_register::<bool>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // N s_0_2: branch s_0_1 b3 b1
        if s_0_1 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_1_0: read-var n:i64
        let s_1_0: i64 = fn_state.n;
        // D s_1_1: call read_gpr(s_1_0)
        let s_1_1: u64 = read_gpr(state, tracer, s_1_0);
        // D s_1_2: write-var return_value <= s_1_1
        fn_state.return_value = s_1_1;
        // N s_1_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_2_0: read-var return_value:u64
        let s_2_0: u64 = fn_state.return_value;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_3_0: read-var n:i64
        let s_3_0: i64 = fn_state.n;
        // D s_3_1: cast reint s_3_0 -> u64
        let s_3_1: u64 = (s_3_0 as u64);
        // C s_3_2: const #21216u : u64
        let s_3_2: u64 = 21216;
        // C s_3_3: const #8u : u64
        let s_3_3: u64 = 8;
        // D s_3_4: mul s_3_1 s_3_3
        let s_3_4: u64 = ((s_3_1) * (s_3_3));
        // D s_3_5: add s_3_2 s_3_4
        let s_3_5: u64 = (s_3_2 + s_3_4);
        // D s_3_6: read-reg s_3_5:u64
        let s_3_6: u64 = {
            let value = state.read_register::<u64>(s_3_5 as isize);
            tracer.read_register(s_3_5 as isize, value);
            value
        };
        // D s_3_7: write-var return_value <= s_3_6
        fn_state.return_value = s_3_6;
        // N s_3_8: jump b2
        return block_2(state, tracer, fn_state);
    }
}
