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
use execute_aarch64_instrs_vector_arithmetic_unary_cmp_fp16_bulk_sisd::*;
use common::*;
pub fn decode_fcmge_advsimd_zero_aarch64_instrs_vector_arithmetic_unary_cmp_fp16_bulk_simd<
    T: Tracer,
>(state: &mut State, tracer: &T, Rd: u8, Rn: u8, op: bool, U: bool, Q: bool) -> () {
    #[derive(Default)]
    struct FunctionState {
        elements: i64,
        ga_253241: u8,
        datasize: i64,
        ga_253239: i64,
        n: i64,
        comparison: u32,
        d: i64,
        Rd: u8,
        Rn: u8,
        op: bool,
        U: bool,
        Q: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        op,
        U,
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
        // D s_0_10: read-var Q:u8
        let s_0_10: bool = fn_state.Q;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 1u16);
        // C s_0_12: const #1u : u8
        let s_0_12: bool = true;
        // C s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 1u16);
        // D s_0_14: cmp-eq s_0_11 s_0_13
        let s_0_14: bool = ((s_0_11) == (s_0_13));
        // N s_0_15: branch s_0_14 b10 b1
        if s_0_14 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #64s : i64
        let s_1_0: i64 = 64;
        // D s_1_1: write-var ga#253239 <= s_1_0
        fn_state.ga_253239 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var ga#253239:i64
        let s_2_0: i64 = fn_state.ga_253239;
        // D s_2_1: write-var datasize <= s_2_0
        fn_state.datasize = s_2_0;
        // C s_2_2: const #16s : i
        let s_2_2: i128 = 16;
        // D s_2_3: read-var datasize:i64
        let s_2_3: i64 = fn_state.datasize;
        // D s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_5: div s_2_4 s_2_2
        let s_2_5: i128 = ((s_2_4) / (s_2_2));
        // D s_2_6: cast reint s_2_5 -> i64
        let s_2_6: i64 = (s_2_5 as i64);
        // D s_2_7: write-var elements <= s_2_6
        fn_state.elements = s_2_6;
        // D s_2_8: read-var op:u8
        let s_2_8: bool = fn_state.op;
        // D s_2_9: cast zx s_2_8 -> bv
        let s_2_9: Bits = Bits::new(s_2_8 as u128, 1u16);
        // D s_2_10: read-var U:u8
        let s_2_10: bool = fn_state.U;
        // D s_2_11: cast zx s_2_10 -> bv
        let s_2_11: Bits = Bits::new(s_2_10 as u128, 1u16);
        // D s_2_12: cast reint s_2_9 -> u128
        let s_2_12: u128 = (s_2_9.value() as u128);
        // D s_2_13: size-of s_2_9
        let s_2_13: u16 = s_2_9.length();
        // D s_2_14: cast reint s_2_11 -> u128
        let s_2_14: u128 = (s_2_11.value() as u128);
        // D s_2_15: size-of s_2_11
        let s_2_15: u16 = s_2_11.length();
        // D s_2_16: lsl s_2_12 s_2_15
        let s_2_16: u128 = s_2_12 << s_2_15;
        // D s_2_17: or s_2_16 s_2_14
        let s_2_17: u128 = ((s_2_16) | (s_2_14));
        // D s_2_18: add s_2_13 s_2_15
        let s_2_18: u16 = (s_2_13 + s_2_15);
        // D s_2_19: create-bits s_2_17 s_2_18
        let s_2_19: Bits = Bits::new(s_2_17, s_2_18);
        // D s_2_20: cast reint s_2_19 -> u8
        let s_2_20: u8 = (s_2_19.value() as u8);
        // D s_2_21: write-var ga#253241 <= s_2_20
        fn_state.ga_253241 = s_2_20;
        // D s_2_22: read-var ga#253241:u8
        let s_2_22: u8 = fn_state.ga_253241;
        // D s_2_23: cast zx s_2_22 -> bv
        let s_2_23: Bits = Bits::new(s_2_22 as u128, 2u16);
        // C s_2_24: const #0u : u8
        let s_2_24: u8 = 0;
        // C s_2_25: cast zx s_2_24 -> bv
        let s_2_25: Bits = Bits::new(s_2_24 as u128, 2u16);
        // D s_2_26: cmp-eq s_2_23 s_2_25
        let s_2_26: bool = ((s_2_23) == (s_2_25));
        // D s_2_27: not s_2_26
        let s_2_27: bool = !s_2_26;
        // N s_2_28: branch s_2_27 b5 b3
        if s_2_27 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u32
        let s_3_0: u32 = 0;
        // D s_3_1: write-var comparison <= s_3_0
        fn_state.comparison = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var comparison:u32
        let s_4_0: u32 = fn_state.comparison;
        // D s_4_1: read-var datasize:i64
        let s_4_1: i64 = fn_state.datasize;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: cast reint s_4_2 -> i64
        let s_4_3: i64 = (s_4_2 as i64);
        // C s_4_4: const #16s : i64
        let s_4_4: i64 = 16;
        // D s_4_5: read-var elements:i64
        let s_4_5: i64 = fn_state.elements;
        // D s_4_6: cast zx s_4_5 -> i
        let s_4_6: i128 = (i128::try_from(s_4_5).unwrap());
        // D s_4_7: read-var d:i64
        let s_4_7: i64 = fn_state.d;
        // D s_4_8: read-var n:i64
        let s_4_8: i64 = fn_state.n;
        // D s_4_9: call execute_aarch64_instrs_vector_arithmetic_unary_cmp_fp16_bulk_sisd(s_4_0, s_4_7, s_4_3, s_4_6, s_4_4, s_4_8)
        let s_4_9: () = execute_aarch64_instrs_vector_arithmetic_unary_cmp_fp16_bulk_sisd(
            state,
            tracer,
            s_4_0,
            s_4_7,
            s_4_3,
            s_4_6,
            s_4_4,
            s_4_8,
        );
        // N s_4_10: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var ga#253241:u8
        let s_5_0: u8 = fn_state.ga_253241;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // C s_5_2: const #1u : u8
        let s_5_2: u8 = 1;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 2u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // D s_5_5: not s_5_4
        let s_5_5: bool = !s_5_4;
        // N s_5_6: branch s_5_5 b7 b6
        if s_5_5 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #1u : u32
        let s_6_0: u32 = 1;
        // D s_6_1: write-var comparison <= s_6_0
        fn_state.comparison = s_6_0;
        // N s_6_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var ga#253241:u8
        let s_7_0: u8 = fn_state.ga_253241;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 2u16);
        // C s_7_2: const #2u : u8
        let s_7_2: u8 = 2;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 2u16);
        // D s_7_4: cmp-eq s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) == (s_7_3));
        // D s_7_5: not s_7_4
        let s_7_5: bool = !s_7_4;
        // N s_7_6: branch s_7_5 b9 b8
        if s_7_5 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #2u : u32
        let s_8_0: u32 = 2;
        // D s_8_1: write-var comparison <= s_8_0
        fn_state.comparison = s_8_0;
        // N s_8_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #3u : u32
        let s_9_0: u32 = 3;
        // D s_9_1: write-var comparison <= s_9_0
        fn_state.comparison = s_9_0;
        // N s_9_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #128s : i64
        let s_10_0: i64 = 128;
        // D s_10_1: write-var ga#253239 <= s_10_0
        fn_state.ga_253239 = s_10_0;
        // N s_10_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
