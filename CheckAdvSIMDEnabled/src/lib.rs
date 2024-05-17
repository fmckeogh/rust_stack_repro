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
use AArch32_CheckAdvSIMDOrFPEnabled::*;
use D_read::*;
use common::*;
pub fn CheckAdvSIMDEnabled<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_30981: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        i: i64,
        gs_30981: (),
    }
    let fn_state = FunctionState {
        gs_30981,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #1u : u8
        let s_0_0: bool = true;
        // C s_0_1: const #1u : u8
        let s_0_1: bool = true;
        // S s_0_2: call AArch32_CheckAdvSIMDOrFPEnabled(s_0_0, s_0_1)
        let s_0_2: () = AArch32_CheckAdvSIMDOrFPEnabled(state, tracer, s_0_0, s_0_1);
        // N s_0_3: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0s : i64
        let s_1_0: i64 = 0;
        // D s_1_1: write-var i <= s_1_0
        fn_state.i = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var i:i64
        let s_2_0: i64 = fn_state.i;
        // C s_2_1: const #31s : i64
        let s_2_1: i64 = 31;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b4 b3
        if s_2_2 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var i:i64
        let s_3_0: i64 = fn_state.i;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: call D_read(s_3_1)
        let s_3_2: u64 = D_read(state, tracer, s_3_1);
        // C s_3_3: const #12976u : u32
        let s_3_3: u32 = 12976;
        // D s_3_4: read-reg s_3_3:[u64; 32]
        let s_3_4: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_3_3 as isize);
            tracer.read_register(s_3_3 as isize, value);
            value
        };
        // D s_3_5: read-var i:i64
        let s_3_5: i64 = fn_state.i;
        // D s_3_6: cast zx s_3_5 -> i
        let s_3_6: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_7: mutate-element s_3_4[s_3_6] <= s_3_2
        let s_3_7: [u64; 32usize] = {
            let mut local = s_3_4.clone();
            local[(s_3_6) as usize] = s_3_2;
            local
        };
        // D s_3_8: cast cvt s_3_7 -> [u64; 0]
        let s_3_8: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_3_7);
        // D s_3_9: cast cvt s_3_8 -> [u64; 32]
        let s_3_9: [u64; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_3_8);
            buf
        };
        // C s_3_10: const #12976u : u32
        let s_3_10: u32 = 12976;
        // N s_3_11: write-reg s_3_10 <= s_3_9
        let s_3_11: () = {
            state.write_register::<[u64; 32usize]>(s_3_10 as isize, s_3_9);
            tracer.write_register(s_3_10 as isize, s_3_9);
        };
        // D s_3_12: read-var i:i64
        let s_3_12: i64 = fn_state.i;
        // C s_3_13: const #1s : i64
        let s_3_13: i64 = 1;
        // D s_3_14: add s_3_12 s_3_13
        let s_3_14: i64 = (s_3_12 + s_3_13);
        // D s_3_15: write-var i <= s_3_14
        fn_state.i = s_3_14;
        // N s_3_16: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: return
        return;
    }
}
