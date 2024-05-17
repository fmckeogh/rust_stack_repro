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
use execute_aarch64_instrs_vector_arithmetic_binary_uniform_mul_fp16_extended_sisd::*;
use common::*;
pub fn decode_fmulx_advsimd_vec_aarch64_instrs_vector_arithmetic_binary_uniform_mul_fp_extended_simd<
    T: Tracer,
>(state: &mut State, tracer: &T, Rd: u8, Rn: u8, Rm: u8, sz: bool, Q: bool) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        esize: i64,
        ga_256074: i64,
        n: i64,
        d: i64,
        Rd: u8,
        Rn: u8,
        Rm: u8,
        sz: bool,
        Q: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        Rm,
        sz,
        Q,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var Rd:u8
        let s_0_0: u8 = fn_state.Rd;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 5u16);
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (s_0_1.value() as i128);
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // D s_0_4: write-var d <= s_0_3
        fn_state.d = s_0_3;
        // D s_0_5: read-var Rn:u8
        let s_0_5: u8 = fn_state.Rn;
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 5u16);
        // D s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (s_0_6.value() as i128);
        // D s_0_8: cast reint s_0_7 -> i64
        let s_0_8: i64 = (s_0_7 as i64);
        // D s_0_9: write-var n <= s_0_8
        fn_state.n = s_0_8;
        // D s_0_10: read-var Rm:u8
        let s_0_10: u8 = fn_state.Rm;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 5u16);
        // D s_0_12: cast zx s_0_11 -> i
        let s_0_12: i128 = (s_0_11.value() as i128);
        // D s_0_13: cast reint s_0_12 -> i64
        let s_0_13: i64 = (s_0_12 as i64);
        // D s_0_14: write-var m <= s_0_13
        fn_state.m = s_0_13;
        // D s_0_15: read-var sz:u8
        let s_0_15: bool = fn_state.sz;
        // D s_0_16: cast zx s_0_15 -> bv
        let s_0_16: Bits = Bits::new(s_0_15 as u128, 1u16);
        // D s_0_17: read-var Q:u8
        let s_0_17: bool = fn_state.Q;
        // D s_0_18: cast zx s_0_17 -> bv
        let s_0_18: Bits = Bits::new(s_0_17 as u128, 1u16);
        // D s_0_19: cast reint s_0_16 -> u128
        let s_0_19: u128 = (s_0_16.value() as u128);
        // D s_0_20: size-of s_0_16
        let s_0_20: u16 = s_0_16.length();
        // D s_0_21: cast reint s_0_18 -> u128
        let s_0_21: u128 = (s_0_18.value() as u128);
        // D s_0_22: size-of s_0_18
        let s_0_22: u16 = s_0_18.length();
        // D s_0_23: lsl s_0_19 s_0_22
        let s_0_23: u128 = s_0_19 << s_0_22;
        // D s_0_24: or s_0_23 s_0_21
        let s_0_24: u128 = ((s_0_23) | (s_0_21));
        // D s_0_25: add s_0_20 s_0_22
        let s_0_25: u16 = (s_0_20 + s_0_22);
        // D s_0_26: create-bits s_0_24 s_0_25
        let s_0_26: Bits = Bits::new(s_0_24, s_0_25);
        // D s_0_27: cast reint s_0_26 -> u8
        let s_0_27: u8 = (s_0_26.value() as u8);
        // D s_0_28: cast zx s_0_27 -> bv
        let s_0_28: Bits = Bits::new(s_0_27 as u128, 2u16);
        // C s_0_29: const #2u : u8
        let s_0_29: u8 = 2;
        // C s_0_30: cast zx s_0_29 -> bv
        let s_0_30: Bits = Bits::new(s_0_29 as u128, 2u16);
        // D s_0_31: cmp-eq s_0_28 s_0_30
        let s_0_31: bool = ((s_0_28) == (s_0_30));
        // N s_0_32: branch s_0_31 b5 b1
        if s_0_31 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var sz:u8
        let s_1_0: bool = fn_state.sz;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 1u16);
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (s_1_1.value() as i128);
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // C s_1_4: const #32s : i64
        let s_1_4: i64 = 32;
        // D s_1_5: lsl s_1_4 s_1_3
        let s_1_5: i64 = s_1_4 << s_1_3;
        // D s_1_6: write-var esize <= s_1_5
        fn_state.esize = s_1_5;
        // D s_1_7: read-var Q:u8
        let s_1_7: bool = fn_state.Q;
        // D s_1_8: cast zx s_1_7 -> bv
        let s_1_8: Bits = Bits::new(s_1_7 as u128, 1u16);
        // C s_1_9: const #1u : u8
        let s_1_9: bool = true;
        // C s_1_10: cast zx s_1_9 -> bv
        let s_1_10: Bits = Bits::new(s_1_9 as u128, 1u16);
        // D s_1_11: cmp-eq s_1_8 s_1_10
        let s_1_11: bool = ((s_1_8) == (s_1_10));
        // N s_1_12: branch s_1_11 b4 b2
        if s_1_11 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #64s : i64
        let s_2_0: i64 = 64;
        // D s_2_1: write-var ga#256074 <= s_2_0
        fn_state.ga_256074 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var ga#256074:i64
        let s_3_0: i64 = fn_state.ga_256074;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var esize:i64
        let s_3_2: i64 = fn_state.esize;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: div s_3_1 s_3_3
        let s_3_4: i128 = ((s_3_1) / (s_3_3));
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: cast zx s_3_0 -> i
        let s_3_6: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_7: cast reint s_3_6 -> i64
        let s_3_7: i64 = (s_3_6 as i64);
        // D s_3_8: read-var esize:i64
        let s_3_8: i64 = fn_state.esize;
        // D s_3_9: cast zx s_3_8 -> i
        let s_3_9: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_10: cast reint s_3_9 -> i64
        let s_3_10: i64 = (s_3_9 as i64);
        // D s_3_11: cast zx s_3_5 -> i
        let s_3_11: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_12: read-var d:i64
        let s_3_12: i64 = fn_state.d;
        // D s_3_13: read-var m:i64
        let s_3_13: i64 = fn_state.m;
        // D s_3_14: read-var n:i64
        let s_3_14: i64 = fn_state.n;
        // D s_3_15: call execute_aarch64_instrs_vector_arithmetic_binary_uniform_mul_fp16_extended_sisd(s_3_12, s_3_7, s_3_11, s_3_10, s_3_13, s_3_14)
        let s_3_15: () = execute_aarch64_instrs_vector_arithmetic_binary_uniform_mul_fp16_extended_sisd(
            state,
            tracer,
            s_3_12,
            s_3_7,
            s_3_11,
            s_3_10,
            s_3_13,
            s_3_14,
        );
        // N s_3_16: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #128s : i64
        let s_4_0: i64 = 128;
        // D s_4_1: write-var ga#256074 <= s_4_0
        fn_state.ga_256074 = s_4_0;
        // N s_4_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: panic
        panic!("{:?}", ());
        // N s_5_1: return
        return;
    }
}
