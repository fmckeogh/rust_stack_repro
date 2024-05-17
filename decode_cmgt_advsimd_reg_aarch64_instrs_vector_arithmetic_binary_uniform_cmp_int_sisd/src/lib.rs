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
use execute_aarch64_instrs_vector_arithmetic_binary_uniform_cmp_int_sisd::*;
use common::*;
pub fn decode_cmgt_advsimd_reg_aarch64_instrs_vector_arithmetic_binary_uniform_cmp_int_sisd<
    T: Tracer,
>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    eq: bool,
    Rm: u8,
    size: u8,
    U: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        n: i64,
        d: i64,
        Rd: u8,
        Rn: u8,
        eq: bool,
        Rm: u8,
        size: u8,
        U: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        eq,
        Rm,
        size,
        U,
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
        // D s_0_15: read-var size:u8
        let s_0_15: u8 = fn_state.size;
        // D s_0_16: cast zx s_0_15 -> bv
        let s_0_16: Bits = Bits::new(s_0_15 as u128, 2u16);
        // C s_0_17: const #3u : u8
        let s_0_17: u8 = 3;
        // C s_0_18: cast zx s_0_17 -> bv
        let s_0_18: Bits = Bits::new(s_0_17 as u128, 2u16);
        // D s_0_19: cmp-ne s_0_16 s_0_18
        let s_0_19: bool = ((s_0_16) != (s_0_18));
        // N s_0_20: branch s_0_19 b2 b1
        if s_0_19 {
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
        // D s_1_0: read-var size:u8
        let s_1_0: u8 = fn_state.size;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 2u16);
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (s_1_1.value() as i128);
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // C s_1_4: const #8s : i64
        let s_1_4: i64 = 8;
        // D s_1_5: lsl s_1_4 s_1_3
        let s_1_5: i64 = s_1_4 << s_1_3;
        // D s_1_6: read-var U:u8
        let s_1_6: bool = fn_state.U;
        // D s_1_7: cast zx s_1_6 -> bv
        let s_1_7: Bits = Bits::new(s_1_6 as u128, 1u16);
        // C s_1_8: const #1u : u8
        let s_1_8: bool = true;
        // C s_1_9: cast zx s_1_8 -> bv
        let s_1_9: Bits = Bits::new(s_1_8 as u128, 1u16);
        // D s_1_10: cmp-eq s_1_7 s_1_9
        let s_1_10: bool = ((s_1_7) == (s_1_9));
        // D s_1_11: read-var eq:u8
        let s_1_11: bool = fn_state.eq;
        // D s_1_12: cast zx s_1_11 -> bv
        let s_1_12: Bits = Bits::new(s_1_11 as u128, 1u16);
        // C s_1_13: const #1u : u8
        let s_1_13: bool = true;
        // C s_1_14: cast zx s_1_13 -> bv
        let s_1_14: Bits = Bits::new(s_1_13 as u128, 1u16);
        // D s_1_15: cmp-eq s_1_12 s_1_14
        let s_1_15: bool = ((s_1_12) == (s_1_14));
        // D s_1_16: cast zx s_1_5 -> i
        let s_1_16: i128 = (i128::try_from(s_1_5).unwrap());
        // D s_1_17: cast reint s_1_16 -> i64
        let s_1_17: i64 = (s_1_16 as i64);
        // D s_1_18: cast zx s_1_5 -> i
        let s_1_18: i128 = (i128::try_from(s_1_5).unwrap());
        // D s_1_19: cast reint s_1_18 -> i64
        let s_1_19: i64 = (s_1_18 as i64);
        // C s_1_20: const #1s : i
        let s_1_20: i128 = 1;
        // D s_1_21: read-var d:i64
        let s_1_21: i64 = fn_state.d;
        // D s_1_22: read-var m:i64
        let s_1_22: i64 = fn_state.m;
        // D s_1_23: read-var n:i64
        let s_1_23: i64 = fn_state.n;
        // D s_1_24: call execute_aarch64_instrs_vector_arithmetic_binary_uniform_cmp_int_sisd(s_1_15, s_1_21, s_1_17, s_1_20, s_1_19, s_1_22, s_1_23, s_1_10)
        let s_1_24: () = execute_aarch64_instrs_vector_arithmetic_binary_uniform_cmp_int_sisd(
            state,
            tracer,
            s_1_15,
            s_1_21,
            s_1_17,
            s_1_20,
            s_1_19,
            s_1_22,
            s_1_23,
            s_1_10,
        );
        // N s_1_25: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_2_0: panic
        panic!("{:?}", ());
        // N s_2_1: return
        return;
    }
}
