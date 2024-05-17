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
use HaveFP16Ext::*;
use execute_aarch64_instrs_vector_arithmetic_unary_cmp_fp16_bulk_sisd::*;
use common::*;
pub fn decode_fcmle_advsimd_aarch64_instrs_vector_arithmetic_unary_cmp_fp16_bulk_sisd<
    T: Tracer,
>(state: &mut State, tracer: &T, Rd: u8, Rn: u8, op: bool, U: bool) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_253298: u8,
        n: i64,
        comparison: u32,
        d: i64,
        Rd: u8,
        Rn: u8,
        op: bool,
        U: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        op,
        U,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call HaveFP16Ext(s_0_0)
        let s_0_1: bool = HaveFP16Ext(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b9 b1
        if s_0_2 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var Rd:u8
        let s_1_0: u8 = fn_state.Rd;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 5u16);
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (s_1_1.value() as i128);
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // D s_1_4: write-var d <= s_1_3
        fn_state.d = s_1_3;
        // D s_1_5: read-var Rn:u8
        let s_1_5: u8 = fn_state.Rn;
        // D s_1_6: cast zx s_1_5 -> bv
        let s_1_6: Bits = Bits::new(s_1_5 as u128, 5u16);
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (s_1_6.value() as i128);
        // D s_1_8: cast reint s_1_7 -> i64
        let s_1_8: i64 = (s_1_7 as i64);
        // D s_1_9: write-var n <= s_1_8
        fn_state.n = s_1_8;
        // D s_1_10: read-var op:u8
        let s_1_10: bool = fn_state.op;
        // D s_1_11: cast zx s_1_10 -> bv
        let s_1_11: Bits = Bits::new(s_1_10 as u128, 1u16);
        // D s_1_12: read-var U:u8
        let s_1_12: bool = fn_state.U;
        // D s_1_13: cast zx s_1_12 -> bv
        let s_1_13: Bits = Bits::new(s_1_12 as u128, 1u16);
        // D s_1_14: cast reint s_1_11 -> u128
        let s_1_14: u128 = (s_1_11.value() as u128);
        // D s_1_15: size-of s_1_11
        let s_1_15: u16 = s_1_11.length();
        // D s_1_16: cast reint s_1_13 -> u128
        let s_1_16: u128 = (s_1_13.value() as u128);
        // D s_1_17: size-of s_1_13
        let s_1_17: u16 = s_1_13.length();
        // D s_1_18: lsl s_1_14 s_1_17
        let s_1_18: u128 = s_1_14 << s_1_17;
        // D s_1_19: or s_1_18 s_1_16
        let s_1_19: u128 = ((s_1_18) | (s_1_16));
        // D s_1_20: add s_1_15 s_1_17
        let s_1_20: u16 = (s_1_15 + s_1_17);
        // D s_1_21: create-bits s_1_19 s_1_20
        let s_1_21: Bits = Bits::new(s_1_19, s_1_20);
        // D s_1_22: cast reint s_1_21 -> u8
        let s_1_22: u8 = (s_1_21.value() as u8);
        // D s_1_23: write-var ga#253298 <= s_1_22
        fn_state.ga_253298 = s_1_22;
        // D s_1_24: read-var ga#253298:u8
        let s_1_24: u8 = fn_state.ga_253298;
        // D s_1_25: cast zx s_1_24 -> bv
        let s_1_25: Bits = Bits::new(s_1_24 as u128, 2u16);
        // C s_1_26: const #0u : u8
        let s_1_26: u8 = 0;
        // C s_1_27: cast zx s_1_26 -> bv
        let s_1_27: Bits = Bits::new(s_1_26 as u128, 2u16);
        // D s_1_28: cmp-eq s_1_25 s_1_27
        let s_1_28: bool = ((s_1_25) == (s_1_27));
        // D s_1_29: not s_1_28
        let s_1_29: bool = !s_1_28;
        // N s_1_30: branch s_1_29 b4 b2
        if s_1_29 {
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
        // C s_2_0: const #0u : u32
        let s_2_0: u32 = 0;
        // D s_2_1: write-var comparison <= s_2_0
        fn_state.comparison = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var comparison:u32
        let s_3_0: u32 = fn_state.comparison;
        // C s_3_1: const #16s : i64
        let s_3_1: i64 = 16;
        // C s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // C s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // C s_3_4: const #16s : i64
        let s_3_4: i64 = 16;
        // C s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // C s_3_6: cast reint s_3_5 -> i64
        let s_3_6: i64 = (s_3_5 as i64);
        // C s_3_7: const #1s : i64
        let s_3_7: i64 = 1;
        // C s_3_8: cast zx s_3_7 -> i
        let s_3_8: i128 = (i128::try_from(s_3_7).unwrap());
        // D s_3_9: read-var d:i64
        let s_3_9: i64 = fn_state.d;
        // D s_3_10: read-var n:i64
        let s_3_10: i64 = fn_state.n;
        // D s_3_11: call execute_aarch64_instrs_vector_arithmetic_unary_cmp_fp16_bulk_sisd(s_3_0, s_3_9, s_3_3, s_3_8, s_3_6, s_3_10)
        let s_3_11: () = execute_aarch64_instrs_vector_arithmetic_unary_cmp_fp16_bulk_sisd(
            state,
            tracer,
            s_3_0,
            s_3_9,
            s_3_3,
            s_3_8,
            s_3_6,
            s_3_10,
        );
        // N s_3_12: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var ga#253298:u8
        let s_4_0: u8 = fn_state.ga_253298;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 2u16);
        // C s_4_2: const #1u : u8
        let s_4_2: u8 = 1;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 2u16);
        // D s_4_4: cmp-eq s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) == (s_4_3));
        // D s_4_5: not s_4_4
        let s_4_5: bool = !s_4_4;
        // N s_4_6: branch s_4_5 b6 b5
        if s_4_5 {
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
        // C s_5_0: const #1u : u32
        let s_5_0: u32 = 1;
        // D s_5_1: write-var comparison <= s_5_0
        fn_state.comparison = s_5_0;
        // N s_5_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ga#253298:u8
        let s_6_0: u8 = fn_state.ga_253298;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 2u16);
        // C s_6_2: const #2u : u8
        let s_6_2: u8 = 2;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 2u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // D s_6_5: not s_6_4
        let s_6_5: bool = !s_6_4;
        // N s_6_6: branch s_6_5 b8 b7
        if s_6_5 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #2u : u32
        let s_7_0: u32 = 2;
        // D s_7_1: write-var comparison <= s_7_0
        fn_state.comparison = s_7_0;
        // N s_7_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #3u : u32
        let s_8_0: u32 = 3;
        // D s_8_1: write-var comparison <= s_8_0
        fn_state.comparison = s_8_0;
        // N s_8_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: panic
        panic!("{:?}", ());
        // N s_9_1: return
        return;
    }
}
