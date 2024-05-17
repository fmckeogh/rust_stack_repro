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
use Mk_FPSR_Type::*;
use Zeros::*;
use common::*;
pub fn ResetSVEState<T: Tracer>(state: &mut State, tracer: &T, gs_5980: ()) -> () {
    #[derive(Default)]
    struct FunctionState {
        u_209: i64,
        n: i64,
        gs_5980: (),
    }
    let fn_state = FunctionState {
        gs_5980,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #0s : i64
        let s_0_0: i64 = 0;
        // D s_0_1: write-var n <= s_0_0
        fn_state.n = s_0_0;
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var n:i64
        let s_1_0: i64 = fn_state.n;
        // C s_1_1: const #31s : i64
        let s_1_1: i64 = 31;
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
        // C s_2_0: const #808u : u32
        let s_2_0: u32 = 808;
        // D s_2_1: read-reg s_2_0:i64
        let s_2_1: i64 = {
            let value = state.read_register::<i64>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: call Zeros(s_2_2)
        let s_2_3: Bits = Zeros(state, tracer, s_2_2);
        // D s_2_4: cast reint s_2_3 -> u2048
        let s_2_4: u64 = (s_2_3.value() as u64);
        // C s_2_5: const #1800u : u32
        let s_2_5: u32 = 1800;
        // D s_2_6: read-reg s_2_5:[u2048; 32]
        let s_2_6: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_2_5 as isize);
            tracer.read_register(s_2_5 as isize, value);
            value
        };
        // D s_2_7: read-var n:i64
        let s_2_7: i64 = fn_state.n;
        // D s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (i128::try_from(s_2_7).unwrap());
        // D s_2_9: mutate-element s_2_6[s_2_8] <= s_2_4
        let s_2_9: [u64; 32usize] = {
            let mut local = s_2_6.clone();
            local[(s_2_8) as usize] = s_2_4;
            local
        };
        // D s_2_10: cast cvt s_2_9 -> [u2048; 0]
        let s_2_10: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_2_9);
        // D s_2_11: cast cvt s_2_10 -> [u2048; 32]
        let s_2_11: [u64; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_2_10);
            buf
        };
        // C s_2_12: const #1800u : u32
        let s_2_12: u32 = 1800;
        // N s_2_13: write-reg s_2_12 <= s_2_11
        let s_2_13: () = {
            state.write_register::<[u64; 32usize]>(s_2_12 as isize, s_2_11);
            tracer.write_register(s_2_12 as isize, s_2_11);
        };
        // D s_2_14: read-var n:i64
        let s_2_14: i64 = fn_state.n;
        // C s_2_15: const #1s : i64
        let s_2_15: i64 = 1;
        // D s_2_16: add s_2_14 s_2_15
        let s_2_16: i64 = (s_2_14 + s_2_15);
        // D s_2_17: write-var n <= s_2_16
        fn_state.n = s_2_16;
        // N s_2_18: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0s : i64
        let s_3_0: i64 = 0;
        // D s_3_1: write-var u#209 <= s_3_0
        fn_state.u_209 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var u#209:i64
        let s_4_0: i64 = fn_state.u_209;
        // C s_4_1: const #15s : i64
        let s_4_1: i64 = 15;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b6 b5
        if s_4_2 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #816u : u32
        let s_5_0: u32 = 816;
        // D s_5_1: read-reg s_5_0:i64
        let s_5_1: i64 = {
            let value = state.read_register::<i64>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: call Zeros(s_5_2)
        let s_5_3: Bits = Zeros(state, tracer, s_5_2);
        // D s_5_4: cast reint s_5_3 -> u256
        let s_5_4: u64 = (s_5_3.value() as u64);
        // C s_5_5: const #17736u : u32
        let s_5_5: u32 = 17736;
        // D s_5_6: read-reg s_5_5:[u256; 16]
        let s_5_6: [u64; 16usize] = {
            let value = state.read_register::<[u64; 16usize]>(s_5_5 as isize);
            tracer.read_register(s_5_5 as isize, value);
            value
        };
        // D s_5_7: read-var u#209:i64
        let s_5_7: i64 = fn_state.u_209;
        // D s_5_8: cast zx s_5_7 -> i
        let s_5_8: i128 = (i128::try_from(s_5_7).unwrap());
        // D s_5_9: mutate-element s_5_6[s_5_8] <= s_5_4
        let s_5_9: [u64; 16usize] = {
            let mut local = s_5_6.clone();
            local[(s_5_8) as usize] = s_5_4;
            local
        };
        // D s_5_10: cast cvt s_5_9 -> [u256; 0]
        let s_5_10: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_5_9);
        // D s_5_11: cast cvt s_5_10 -> [u256; 16]
        let s_5_11: [u64; 16usize] = {
            let mut buf = [Default::default(); 16usize];
            buf.copy_from_slice(&s_5_10);
            buf
        };
        // C s_5_12: const #17736u : u32
        let s_5_12: u32 = 17736;
        // N s_5_13: write-reg s_5_12 <= s_5_11
        let s_5_13: () = {
            state.write_register::<[u64; 16usize]>(s_5_12 as isize, s_5_11);
            tracer.write_register(s_5_12 as isize, s_5_11);
        };
        // D s_5_14: read-var u#209:i64
        let s_5_14: i64 = fn_state.u_209;
        // C s_5_15: const #1s : i64
        let s_5_15: i64 = 1;
        // D s_5_16: add s_5_14 s_5_15
        let s_5_16: i64 = (s_5_14 + s_5_15);
        // D s_5_17: write-var u#209 <= s_5_16
        fn_state.u_209 = s_5_16;
        // N s_5_18: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #816u : u32
        let s_6_0: u32 = 816;
        // D s_6_1: read-reg s_6_0:i64
        let s_6_1: i64 = {
            let value = state.read_register::<i64>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: call Zeros(s_6_2)
        let s_6_3: Bits = Zeros(state, tracer, s_6_2);
        // D s_6_4: cast reint s_6_3 -> u256
        let s_6_4: u64 = (s_6_3.value() as u64);
        // C s_6_5: const #14552u : u32
        let s_6_5: u32 = 14552;
        // N s_6_6: write-reg s_6_5 <= s_6_4
        let s_6_6: () = {
            state.write_register::<u64>(s_6_5 as isize, s_6_4);
            tracer.write_register(s_6_5 as isize, s_6_4);
        };
        // C s_6_7: const #64s : i
        let s_6_7: i128 = 64;
        // C s_6_8: const #134217887u : u32
        let s_6_8: u32 = 134217887;
        // C s_6_9: cast zx s_6_8 -> bv
        let s_6_9: Bits = Bits::new(s_6_8 as u128, 32u16);
        // D s_6_10: bits-cast zx s_6_9 -> bv length s_6_7
        let s_6_10: Bits = s_6_9.zero_extend(s_6_7);
        // D s_6_11: cast reint s_6_10 -> u64
        let s_6_11: u64 = (s_6_10.value() as u64);
        // D s_6_12: call Mk_FPSR_Type(s_6_11)
        let s_6_12: ProductType5c790c8ef59cc8b2 = Mk_FPSR_Type(state, tracer, s_6_11);
        // C s_6_13: const #20696u : u32
        let s_6_13: u32 = 20696;
        // N s_6_14: write-reg s_6_13 <= s_6_12
        let s_6_14: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_6_13 as isize, s_6_12);
            tracer.write_register(s_6_13 as isize, s_6_12);
        };
        // N s_6_15: return
        return;
    }
}
