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
use execute_aarch64_instrs_vector_arithmetic_binary_element_mul_acc_fp16_sisd::*;
use common::*;
pub fn decode_fmls_advsimd_elt_aarch64_instrs_vector_arithmetic_binary_element_mul_acc_fp16_simd<
    T: Tracer,
>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    H: bool,
    o2: bool,
    Rm: u8,
    M: bool,
    L: bool,
    Q: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        n: i64,
        index: i64,
        ga_255560: i64,
        d: i64,
        idxdsize: i64,
        ga_255552: i64,
        Rd: u8,
        Rn: u8,
        H: bool,
        o2: bool,
        Rm: u8,
        M: bool,
        L: bool,
        Q: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        H,
        o2,
        Rm,
        M,
        L,
        Q,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var H:u8
        let s_0_0: bool = fn_state.H;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 1u16);
        // C s_0_2: const #1u : u8
        let s_0_2: bool = true;
        // C s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 1u16);
        // D s_0_4: cmp-eq s_0_1 s_0_3
        let s_0_4: bool = ((s_0_1) == (s_0_3));
        // N s_0_5: branch s_0_4 b6 b1
        if s_0_4 {
            return block_6(state, tracer, fn_state);
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
        // D s_1_1: write-var ga#255552 <= s_1_0
        fn_state.ga_255552 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var ga#255552:i64
        let s_2_0: i64 = fn_state.ga_255552;
        // D s_2_1: write-var idxdsize <= s_2_0
        fn_state.idxdsize = s_2_0;
        // D s_2_2: read-var Rn:u8
        let s_2_2: u8 = fn_state.Rn;
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 5u16);
        // D s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (s_2_3.value() as i128);
        // D s_2_5: cast reint s_2_4 -> i64
        let s_2_5: i64 = (s_2_4 as i64);
        // D s_2_6: write-var n <= s_2_5
        fn_state.n = s_2_5;
        // D s_2_7: read-var Rm:u8
        let s_2_7: u8 = fn_state.Rm;
        // D s_2_8: cast zx s_2_7 -> bv
        let s_2_8: Bits = Bits::new(s_2_7 as u128, 4u16);
        // D s_2_9: cast zx s_2_8 -> i
        let s_2_9: i128 = (s_2_8.value() as i128);
        // D s_2_10: cast reint s_2_9 -> i64
        let s_2_10: i64 = (s_2_9 as i64);
        // D s_2_11: write-var m <= s_2_10
        fn_state.m = s_2_10;
        // D s_2_12: read-var Rd:u8
        let s_2_12: u8 = fn_state.Rd;
        // D s_2_13: cast zx s_2_12 -> bv
        let s_2_13: Bits = Bits::new(s_2_12 as u128, 5u16);
        // D s_2_14: cast zx s_2_13 -> i
        let s_2_14: i128 = (s_2_13.value() as i128);
        // D s_2_15: cast reint s_2_14 -> i64
        let s_2_15: i64 = (s_2_14 as i64);
        // D s_2_16: write-var d <= s_2_15
        fn_state.d = s_2_15;
        // D s_2_17: read-var H:u8
        let s_2_17: bool = fn_state.H;
        // D s_2_18: cast zx s_2_17 -> bv
        let s_2_18: Bits = Bits::new(s_2_17 as u128, 1u16);
        // D s_2_19: read-var L:u8
        let s_2_19: bool = fn_state.L;
        // D s_2_20: cast zx s_2_19 -> bv
        let s_2_20: Bits = Bits::new(s_2_19 as u128, 1u16);
        // D s_2_21: cast reint s_2_18 -> u128
        let s_2_21: u128 = (s_2_18.value() as u128);
        // D s_2_22: size-of s_2_18
        let s_2_22: u16 = s_2_18.length();
        // D s_2_23: cast reint s_2_20 -> u128
        let s_2_23: u128 = (s_2_20.value() as u128);
        // D s_2_24: size-of s_2_20
        let s_2_24: u16 = s_2_20.length();
        // D s_2_25: lsl s_2_21 s_2_24
        let s_2_25: u128 = s_2_21 << s_2_24;
        // D s_2_26: or s_2_25 s_2_23
        let s_2_26: u128 = ((s_2_25) | (s_2_23));
        // D s_2_27: add s_2_22 s_2_24
        let s_2_27: u16 = (s_2_22 + s_2_24);
        // D s_2_28: create-bits s_2_26 s_2_27
        let s_2_28: Bits = Bits::new(s_2_26, s_2_27);
        // D s_2_29: cast reint s_2_28 -> u8
        let s_2_29: u8 = (s_2_28.value() as u8);
        // D s_2_30: cast zx s_2_29 -> bv
        let s_2_30: Bits = Bits::new(s_2_29 as u128, 2u16);
        // D s_2_31: read-var M:u8
        let s_2_31: bool = fn_state.M;
        // D s_2_32: cast zx s_2_31 -> bv
        let s_2_32: Bits = Bits::new(s_2_31 as u128, 1u16);
        // D s_2_33: cast reint s_2_30 -> u128
        let s_2_33: u128 = (s_2_30.value() as u128);
        // D s_2_34: size-of s_2_30
        let s_2_34: u16 = s_2_30.length();
        // D s_2_35: cast reint s_2_32 -> u128
        let s_2_35: u128 = (s_2_32.value() as u128);
        // D s_2_36: size-of s_2_32
        let s_2_36: u16 = s_2_32.length();
        // D s_2_37: lsl s_2_33 s_2_36
        let s_2_37: u128 = s_2_33 << s_2_36;
        // D s_2_38: or s_2_37 s_2_35
        let s_2_38: u128 = ((s_2_37) | (s_2_35));
        // D s_2_39: add s_2_34 s_2_36
        let s_2_39: u16 = (s_2_34 + s_2_36);
        // D s_2_40: create-bits s_2_38 s_2_39
        let s_2_40: Bits = Bits::new(s_2_38, s_2_39);
        // D s_2_41: cast reint s_2_40 -> u8
        let s_2_41: u8 = (s_2_40.value() as u8);
        // D s_2_42: cast zx s_2_41 -> bv
        let s_2_42: Bits = Bits::new(s_2_41 as u128, 3u16);
        // D s_2_43: cast zx s_2_42 -> i
        let s_2_43: i128 = (s_2_42.value() as i128);
        // D s_2_44: cast reint s_2_43 -> i64
        let s_2_44: i64 = (s_2_43 as i64);
        // D s_2_45: write-var index <= s_2_44
        fn_state.index = s_2_44;
        // D s_2_46: read-var Q:u8
        let s_2_46: bool = fn_state.Q;
        // D s_2_47: cast zx s_2_46 -> bv
        let s_2_47: Bits = Bits::new(s_2_46 as u128, 1u16);
        // C s_2_48: const #1u : u8
        let s_2_48: bool = true;
        // C s_2_49: cast zx s_2_48 -> bv
        let s_2_49: Bits = Bits::new(s_2_48 as u128, 1u16);
        // D s_2_50: cmp-eq s_2_47 s_2_49
        let s_2_50: bool = ((s_2_47) == (s_2_49));
        // N s_2_51: branch s_2_50 b5 b3
        if s_2_50 {
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
        // C s_3_0: const #64s : i64
        let s_3_0: i64 = 64;
        // D s_3_1: write-var ga#255560 <= s_3_0
        fn_state.ga_255560 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var ga#255560:i64
        let s_4_0: i64 = fn_state.ga_255560;
        // C s_4_1: const #16s : i
        let s_4_1: i128 = 16;
        // D s_4_2: cast zx s_4_0 -> i
        let s_4_2: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_3: div s_4_2 s_4_1
        let s_4_3: i128 = ((s_4_2) / (s_4_1));
        // D s_4_4: cast reint s_4_3 -> i64
        let s_4_4: i64 = (s_4_3 as i64);
        // D s_4_5: read-var o2:u8
        let s_4_5: bool = fn_state.o2;
        // D s_4_6: cast zx s_4_5 -> bv
        let s_4_6: Bits = Bits::new(s_4_5 as u128, 1u16);
        // C s_4_7: const #1u : u8
        let s_4_7: bool = true;
        // C s_4_8: cast zx s_4_7 -> bv
        let s_4_8: Bits = Bits::new(s_4_7 as u128, 1u16);
        // D s_4_9: cmp-eq s_4_6 s_4_8
        let s_4_9: bool = ((s_4_6) == (s_4_8));
        // D s_4_10: cast zx s_4_0 -> i
        let s_4_10: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_11: cast reint s_4_10 -> i64
        let s_4_11: i64 = (s_4_10 as i64);
        // C s_4_12: const #16s : i64
        let s_4_12: i64 = 16;
        // D s_4_13: read-var idxdsize:i64
        let s_4_13: i64 = fn_state.idxdsize;
        // D s_4_14: cast zx s_4_13 -> i
        let s_4_14: i128 = (i128::try_from(s_4_13).unwrap());
        // D s_4_15: cast reint s_4_14 -> i64
        let s_4_15: i64 = (s_4_14 as i64);
        // D s_4_16: cast zx s_4_4 -> i
        let s_4_16: i128 = (i128::try_from(s_4_4).unwrap());
        // D s_4_17: read-var d:i64
        let s_4_17: i64 = fn_state.d;
        // D s_4_18: read-var index:i64
        let s_4_18: i64 = fn_state.index;
        // D s_4_19: read-var m:i64
        let s_4_19: i64 = fn_state.m;
        // D s_4_20: read-var n:i64
        let s_4_20: i64 = fn_state.n;
        // D s_4_21: call execute_aarch64_instrs_vector_arithmetic_binary_element_mul_acc_fp16_sisd(s_4_17, s_4_11, s_4_16, s_4_12, s_4_15, s_4_18, s_4_19, s_4_20, s_4_9)
        let s_4_21: () = execute_aarch64_instrs_vector_arithmetic_binary_element_mul_acc_fp16_sisd(
            state,
            tracer,
            s_4_17,
            s_4_11,
            s_4_16,
            s_4_12,
            s_4_15,
            s_4_18,
            s_4_19,
            s_4_20,
            s_4_9,
        );
        // N s_4_22: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #128s : i64
        let s_5_0: i64 = 128;
        // D s_5_1: write-var ga#255560 <= s_5_0
        fn_state.ga_255560 = s_5_0;
        // N s_5_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #128s : i64
        let s_6_0: i64 = 128;
        // D s_6_1: write-var ga#255552 <= s_6_0
        fn_state.ga_255552 = s_6_0;
        // N s_6_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
