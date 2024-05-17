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
use write_gpr::*;
use common::*;
pub fn set_R<T: Tracer>(state: &mut State, tracer: &T, n: i64, v: u64) -> () {
    #[derive(Default)]
    struct FunctionState {
        n: i64,
        v: u64,
    }
    let fn_state = FunctionState {
        n,
        v,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #20544u : u32
        let s_0_0: u32 = 20544;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: bool = {
            let value = state.read_register::<bool>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // N s_0_2: branch s_0_1 b2 b1
        if s_0_1 {
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
        // D s_1_0: read-var n:i64
        let s_1_0: i64 = fn_state.n;
        // D s_1_1: read-var v:u64
        let s_1_1: u64 = fn_state.v;
        // D s_1_2: call write_gpr(s_1_0, s_1_1)
        let s_1_2: () = write_gpr(state, tracer, s_1_0, s_1_1);
        // N s_1_3: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var n:i64
        let s_2_0: i64 = fn_state.n;
        // D s_2_1: read-var v:u64
        let s_2_1: u64 = fn_state.v;
        // D s_2_2: cast reint s_2_0 -> u64
        let s_2_2: u64 = (s_2_0 as u64);
        // C s_2_3: const #21216u : u64
        let s_2_3: u64 = 21216;
        // C s_2_4: const #8u : u64
        let s_2_4: u64 = 8;
        // D s_2_5: mul s_2_2 s_2_4
        let s_2_5: u64 = ((s_2_2) * (s_2_4));
        // D s_2_6: add s_2_3 s_2_5
        let s_2_6: u64 = (s_2_3 + s_2_5);
        // N s_2_7: write-reg s_2_6 <= s_2_1
        let s_2_7: () = {
            state.write_register::<u64>(s_2_6 as isize, s_2_1);
            tracer.write_register(s_2_6 as isize, s_2_1);
        };
        // N s_2_8: return
        return;
    }
}
