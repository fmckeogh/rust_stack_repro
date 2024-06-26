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
use FPDecodeRounding::*;
use execute_aarch64_instrs_vector_arithmetic_unary_fp16_conv_float_bulk_sisd::*;
use common::*;
pub fn decode_fcvtzs_advsimd_int_aarch64_instrs_vector_arithmetic_unary_float_conv_float_bulk_simd<
    T: Tracer,
>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    o1: bool,
    sz: bool,
    o2: bool,
    U: bool,
    Q: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        esize: i64,
        n: i64,
        d: i64,
        ga_254671: i64,
        Rd: u8,
        Rn: u8,
        o1: bool,
        sz: bool,
        o2: bool,
        U: bool,
        Q: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        o1,
        sz,
        o2,
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
        // D s_0_10: read-var sz:u8
        let s_0_10: bool = fn_state.sz;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 1u16);
        // D s_0_12: read-var Q:u8
        let s_0_12: bool = fn_state.Q;
        // D s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 1u16);
        // D s_0_14: cast reint s_0_11 -> u128
        let s_0_14: u128 = (s_0_11.value() as u128);
        // D s_0_15: size-of s_0_11
        let s_0_15: u16 = s_0_11.length();
        // D s_0_16: cast reint s_0_13 -> u128
        let s_0_16: u128 = (s_0_13.value() as u128);
        // D s_0_17: size-of s_0_13
        let s_0_17: u16 = s_0_13.length();
        // D s_0_18: lsl s_0_14 s_0_17
        let s_0_18: u128 = s_0_14 << s_0_17;
        // D s_0_19: or s_0_18 s_0_16
        let s_0_19: u128 = ((s_0_18) | (s_0_16));
        // D s_0_20: add s_0_15 s_0_17
        let s_0_20: u16 = (s_0_15 + s_0_17);
        // D s_0_21: create-bits s_0_19 s_0_20
        let s_0_21: Bits = Bits::new(s_0_19, s_0_20);
        // D s_0_22: cast reint s_0_21 -> u8
        let s_0_22: u8 = (s_0_21.value() as u8);
        // D s_0_23: cast zx s_0_22 -> bv
        let s_0_23: Bits = Bits::new(s_0_22 as u128, 2u16);
        // C s_0_24: const #2u : u8
        let s_0_24: u8 = 2;
        // C s_0_25: cast zx s_0_24 -> bv
        let s_0_25: Bits = Bits::new(s_0_24 as u128, 2u16);
        // D s_0_26: cmp-eq s_0_23 s_0_25
        let s_0_26: bool = ((s_0_23) == (s_0_25));
        // N s_0_27: branch s_0_26 b5 b1
        if s_0_26 {
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
        // D s_2_1: write-var ga#254671 <= s_2_0
        fn_state.ga_254671 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var ga#254671:i64
        let s_3_0: i64 = fn_state.ga_254671;
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
        // D s_3_6: read-var o1:u8
        let s_3_6: bool = fn_state.o1;
        // D s_3_7: cast zx s_3_6 -> bv
        let s_3_7: Bits = Bits::new(s_3_6 as u128, 1u16);
        // D s_3_8: read-var o2:u8
        let s_3_8: bool = fn_state.o2;
        // D s_3_9: cast zx s_3_8 -> bv
        let s_3_9: Bits = Bits::new(s_3_8 as u128, 1u16);
        // D s_3_10: cast reint s_3_7 -> u128
        let s_3_10: u128 = (s_3_7.value() as u128);
        // D s_3_11: size-of s_3_7
        let s_3_11: u16 = s_3_7.length();
        // D s_3_12: cast reint s_3_9 -> u128
        let s_3_12: u128 = (s_3_9.value() as u128);
        // D s_3_13: size-of s_3_9
        let s_3_13: u16 = s_3_9.length();
        // D s_3_14: lsl s_3_10 s_3_13
        let s_3_14: u128 = s_3_10 << s_3_13;
        // D s_3_15: or s_3_14 s_3_12
        let s_3_15: u128 = ((s_3_14) | (s_3_12));
        // D s_3_16: add s_3_11 s_3_13
        let s_3_16: u16 = (s_3_11 + s_3_13);
        // D s_3_17: create-bits s_3_15 s_3_16
        let s_3_17: Bits = Bits::new(s_3_15, s_3_16);
        // D s_3_18: cast reint s_3_17 -> u8
        let s_3_18: u8 = (s_3_17.value() as u8);
        // D s_3_19: call FPDecodeRounding(s_3_18)
        let s_3_19: u32 = FPDecodeRounding(state, tracer, s_3_18);
        // D s_3_20: read-var U:u8
        let s_3_20: bool = fn_state.U;
        // D s_3_21: cast zx s_3_20 -> bv
        let s_3_21: Bits = Bits::new(s_3_20 as u128, 1u16);
        // C s_3_22: const #1u : u8
        let s_3_22: bool = true;
        // C s_3_23: cast zx s_3_22 -> bv
        let s_3_23: Bits = Bits::new(s_3_22 as u128, 1u16);
        // D s_3_24: cmp-eq s_3_21 s_3_23
        let s_3_24: bool = ((s_3_21) == (s_3_23));
        // D s_3_25: cast zx s_3_0 -> i
        let s_3_25: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_26: cast reint s_3_25 -> i64
        let s_3_26: i64 = (s_3_25 as i64);
        // D s_3_27: read-var esize:i64
        let s_3_27: i64 = fn_state.esize;
        // D s_3_28: cast zx s_3_27 -> i
        let s_3_28: i128 = (i128::try_from(s_3_27).unwrap());
        // D s_3_29: cast reint s_3_28 -> i64
        let s_3_29: i64 = (s_3_28 as i64);
        // D s_3_30: cast zx s_3_5 -> i
        let s_3_30: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_31: read-var d:i64
        let s_3_31: i64 = fn_state.d;
        // D s_3_32: read-var n:i64
        let s_3_32: i64 = fn_state.n;
        // D s_3_33: call execute_aarch64_instrs_vector_arithmetic_unary_fp16_conv_float_bulk_sisd(s_3_31, s_3_26, s_3_30, s_3_29, s_3_32, s_3_19, s_3_24)
        let s_3_33: () = execute_aarch64_instrs_vector_arithmetic_unary_fp16_conv_float_bulk_sisd(
            state,
            tracer,
            s_3_31,
            s_3_26,
            s_3_30,
            s_3_29,
            s_3_32,
            s_3_19,
            s_3_24,
        );
        // N s_3_34: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #128s : i64
        let s_4_0: i64 = 128;
        // D s_4_1: write-var ga#254671 <= s_4_0
        fn_state.ga_254671 = s_4_0;
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
