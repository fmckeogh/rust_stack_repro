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
use Zeros::*;
use common::*;
pub fn SPEEmptyRecord<T: Tracer>(state: &mut State, tracer: &T, gs_25917: ()) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_25923: i64,
        i: i64,
        gs_25917: (),
    }
    let fn_state = FunctionState {
        gs_25917,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #0s : i
        let s_0_0: i128 = 0;
        // C s_0_1: const #10384u : u32
        let s_0_1: u32 = 10384;
        // N s_0_2: write-reg s_0_1 <= s_0_0
        let s_0_2: () = {
            state.write_register::<i128>(s_0_1 as isize, s_0_0);
            tracer.write_register(s_0_1 as isize, s_0_0);
        };
        // C s_0_3: const #0s : i64
        let s_0_3: i64 = 0;
        // C s_0_4: const #1s : i
        let s_0_4: i128 = 1;
        // C s_0_5: const #1008u : u32
        let s_0_5: u32 = 1008;
        // D s_0_6: read-reg s_0_5:i64
        let s_0_6: i64 = {
            let value = state.read_register::<i64>(s_0_5 as isize);
            tracer.read_register(s_0_5 as isize, value);
            value
        };
        // D s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (i128::try_from(s_0_6).unwrap());
        // D s_0_8: sub s_0_7 s_0_4
        let s_0_8: i128 = ((s_0_7) - (s_0_4));
        // D s_0_9: cast reint s_0_8 -> i64
        let s_0_9: i64 = (s_0_8 as i64);
        // D s_0_10: write-var gs#25923 <= s_0_9
        fn_state.gs_25923 = s_0_9;
        // D s_0_11: write-var i <= s_0_3
        fn_state.i = s_0_3;
        // N s_0_12: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var i:i64
        let s_1_0: i64 = fn_state.i;
        // D s_1_1: read-var gs#25923:i64
        let s_1_1: i64 = fn_state.gs_25923;
        // D s_1_2: cmp-gt s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) > (s_1_1));
        // N s_1_3: branch s_1_2 b3 b2
        if s_1_2 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #8s : i
        let s_2_0: i128 = 8;
        // S s_2_1: call Zeros(s_2_0)
        let s_2_1: Bits = Zeros(state, tracer, s_2_0);
        // S s_2_2: cast reint s_2_1 -> u8
        let s_2_2: u8 = (s_2_1.value() as u8);
        // C s_2_3: const #10664u : u32
        let s_2_3: u32 = 10664;
        // D s_2_4: read-reg s_2_3:[u8; 64]
        let s_2_4: [u8; 64usize] = {
            let value = state.read_register::<[u8; 64usize]>(s_2_3 as isize);
            tracer.read_register(s_2_3 as isize, value);
            value
        };
        // D s_2_5: read-var i:i64
        let s_2_5: i64 = fn_state.i;
        // D s_2_6: cast zx s_2_5 -> i
        let s_2_6: i128 = (i128::try_from(s_2_5).unwrap());
        // D s_2_7: mutate-element s_2_4[s_2_6] <= s_2_2
        let s_2_7: [u8; 64usize] = {
            let mut local = s_2_4.clone();
            local[(s_2_6) as usize] = s_2_2;
            local
        };
        // D s_2_8: cast cvt s_2_7 -> [u8; 0]
        let s_2_8: alloc::vec::Vec<u8> = alloc::vec::Vec::from(s_2_7);
        // D s_2_9: cast cvt s_2_8 -> [u8; 64]
        let s_2_9: [u8; 64usize] = {
            let mut buf = [Default::default(); 64usize];
            buf.copy_from_slice(&s_2_8);
            buf
        };
        // C s_2_10: const #10664u : u32
        let s_2_10: u32 = 10664;
        // N s_2_11: write-reg s_2_10 <= s_2_9
        let s_2_11: () = {
            state.write_register::<[u8; 64usize]>(s_2_10 as isize, s_2_9);
            tracer.write_register(s_2_10 as isize, s_2_9);
        };
        // D s_2_12: read-var i:i64
        let s_2_12: i64 = fn_state.i;
        // C s_2_13: const #1s : i64
        let s_2_13: i64 = 1;
        // D s_2_14: add s_2_12 s_2_13
        let s_2_14: i64 = (s_2_12 + s_2_13);
        // D s_2_15: write-var i <= s_2_14
        fn_state.i = s_2_14;
        // N s_2_16: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: return
        return;
    }
}
