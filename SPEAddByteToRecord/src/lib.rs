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
pub fn SPEAddByteToRecord<T: Tracer>(state: &mut State, tracer: &T, b: u8) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_25888: bool,
        size: i128,
        b: u8,
    }
    let fn_state = FunctionState {
        b,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #10384u : u32
        let s_0_0: u32 = 10384;
        // D s_0_1: read-reg s_0_0:i
        let s_0_1: i128 = {
            let value = state.read_register::<i128>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: write-var size <= s_0_1
        fn_state.size = s_0_1;
        // C s_0_3: const #0s : i
        let s_0_3: i128 = 0;
        // D s_0_4: read-var size:i
        let s_0_4: i128 = fn_state.size;
        // D s_0_5: cmp-le s_0_3 s_0_4
        let s_0_5: bool = ((s_0_3) <= (s_0_4));
        // N s_0_6: branch s_0_5 b3 b1
        if s_0_5 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#25888 <= s_1_0
        fn_state.gs_25888 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#25888:u8
        let s_2_0: bool = fn_state.gs_25888;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // C s_2_2: const #10664u : u32
        let s_2_2: u32 = 10664;
        // D s_2_3: read-reg s_2_2:[u8; 64]
        let s_2_3: [u8; 64usize] = {
            let value = state.read_register::<[u8; 64usize]>(s_2_2 as isize);
            tracer.read_register(s_2_2 as isize, value);
            value
        };
        // D s_2_4: read-var size:i
        let s_2_4: i128 = fn_state.size;
        // D s_2_5: read-var b:u8
        let s_2_5: u8 = fn_state.b;
        // D s_2_6: mutate-element s_2_3[s_2_4] <= s_2_5
        let s_2_6: [u8; 64usize] = {
            let mut local = s_2_3.clone();
            local[(s_2_4) as usize] = s_2_5;
            local
        };
        // D s_2_7: cast cvt s_2_6 -> [u8; 0]
        let s_2_7: alloc::vec::Vec<u8> = alloc::vec::Vec::from(s_2_6);
        // D s_2_8: cast cvt s_2_7 -> [u8; 64]
        let s_2_8: [u8; 64usize] = {
            let mut buf = [Default::default(); 64usize];
            buf.copy_from_slice(&s_2_7);
            buf
        };
        // C s_2_9: const #10664u : u32
        let s_2_9: u32 = 10664;
        // N s_2_10: write-reg s_2_9 <= s_2_8
        let s_2_10: () = {
            state.write_register::<[u8; 64usize]>(s_2_9 as isize, s_2_8);
            tracer.write_register(s_2_9 as isize, s_2_8);
        };
        // C s_2_11: const #1s : i
        let s_2_11: i128 = 1;
        // D s_2_12: read-var size:i
        let s_2_12: i128 = fn_state.size;
        // D s_2_13: add s_2_12 s_2_11
        let s_2_13: i128 = (s_2_12 + s_2_11);
        // C s_2_14: const #10384u : u32
        let s_2_14: u32 = 10384;
        // N s_2_15: write-reg s_2_14 <= s_2_13
        let s_2_15: () = {
            state.write_register::<i128>(s_2_14 as isize, s_2_13);
            tracer.write_register(s_2_14 as isize, s_2_13);
        };
        // N s_2_16: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #1008u : u32
        let s_3_0: u32 = 1008;
        // D s_3_1: read-reg s_3_0:i64
        let s_3_1: i64 = {
            let value = state.read_register::<i64>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: read-var size:i
        let s_3_3: i128 = fn_state.size;
        // D s_3_4: cmp-lt s_3_3 s_3_2
        let s_3_4: bool = ((s_3_3) < (s_3_2));
        // D s_3_5: write-var gs#25888 <= s_3_4
        fn_state.gs_25888 = s_3_4;
        // N s_3_6: jump b2
        return block_2(state, tracer, fn_state);
    }
}
