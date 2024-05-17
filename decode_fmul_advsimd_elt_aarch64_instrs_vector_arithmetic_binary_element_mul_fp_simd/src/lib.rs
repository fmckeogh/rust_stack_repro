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
use execute_aarch64_instrs_vector_arithmetic_binary_element_mul_fp16_sisd::*;
use common::*;
pub fn decode_fmul_advsimd_elt_aarch64_instrs_vector_arithmetic_binary_element_mul_fp_simd<
    T: Tracer,
>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    H: bool,
    Rm: u8,
    M: bool,
    L: bool,
    sz: bool,
    U: bool,
    Q: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        ga_255966: i64,
        ga_255952: u8,
        ga_255951: i64,
        esize: i64,
        n: i64,
        index: i64,
        d: i64,
        idxdsize: i64,
        Rmhi: bool,
        indexshadow_1417: i64,
        Rd: u8,
        Rn: u8,
        H: bool,
        Rm: u8,
        M: bool,
        L: bool,
        sz: bool,
        U: bool,
        Q: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        H,
        Rm,
        M,
        L,
        sz,
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
        // N s_0_5: branch s_0_4 b13 b1
        if s_0_4 {
            return block_13(state, tracer, fn_state);
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
        // D s_1_1: write-var ga#255951 <= s_1_0
        fn_state.ga_255951 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var ga#255951:i64
        let s_2_0: i64 = fn_state.ga_255951;
        // D s_2_1: write-var idxdsize <= s_2_0
        fn_state.idxdsize = s_2_0;
        // D s_2_2: read-var M:u8
        let s_2_2: bool = fn_state.M;
        // D s_2_3: write-var Rmhi <= s_2_2
        fn_state.Rmhi = s_2_2;
        // D s_2_4: read-var sz:u8
        let s_2_4: bool = fn_state.sz;
        // D s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 1u16);
        // D s_2_6: read-var L:u8
        let s_2_6: bool = fn_state.L;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 1u16);
        // D s_2_8: cast reint s_2_5 -> u128
        let s_2_8: u128 = (s_2_5.value() as u128);
        // D s_2_9: size-of s_2_5
        let s_2_9: u16 = s_2_5.length();
        // D s_2_10: cast reint s_2_7 -> u128
        let s_2_10: u128 = (s_2_7.value() as u128);
        // D s_2_11: size-of s_2_7
        let s_2_11: u16 = s_2_7.length();
        // D s_2_12: lsl s_2_8 s_2_11
        let s_2_12: u128 = s_2_8 << s_2_11;
        // D s_2_13: or s_2_12 s_2_10
        let s_2_13: u128 = ((s_2_12) | (s_2_10));
        // D s_2_14: add s_2_9 s_2_11
        let s_2_14: u16 = (s_2_9 + s_2_11);
        // D s_2_15: create-bits s_2_13 s_2_14
        let s_2_15: Bits = Bits::new(s_2_13, s_2_14);
        // D s_2_16: cast reint s_2_15 -> u8
        let s_2_16: u8 = (s_2_15.value() as u8);
        // D s_2_17: write-var ga#255952 <= s_2_16
        fn_state.ga_255952 = s_2_16;
        // D s_2_18: read-var ga#255952:u8
        let s_2_18: u8 = fn_state.ga_255952;
        // C s_2_19: const #1s : i
        let s_2_19: i128 = 1;
        // D s_2_20: cast zx s_2_18 -> bv
        let s_2_20: Bits = Bits::new(s_2_18 as u128, 2u16);
        // C s_2_21: const #1s : i64
        let s_2_21: i64 = 1;
        // C s_2_22: cast zx s_2_21 -> i
        let s_2_22: i128 = (i128::try_from(s_2_21).unwrap());
        // C s_2_23: const #0s : i
        let s_2_23: i128 = 0;
        // C s_2_24: add s_2_23 s_2_22
        let s_2_24: i128 = (s_2_23 + s_2_22);
        // D s_2_25: bit-extract s_2_20 s_2_19 s_2_24
        let s_2_25: Bits = (Bits::new(
            ((s_2_20) >> (s_2_19)).value(),
            u16::try_from(s_2_24).unwrap(),
        ));
        // D s_2_26: cast reint s_2_25 -> u8
        let s_2_26: bool = ((s_2_25.value()) != 0);
        // D s_2_27: cast zx s_2_26 -> bv
        let s_2_27: Bits = Bits::new(s_2_26 as u128, 1u16);
        // C s_2_28: const #0u : u8
        let s_2_28: bool = false;
        // C s_2_29: cast zx s_2_28 -> bv
        let s_2_29: Bits = Bits::new(s_2_28 as u128, 1u16);
        // D s_2_30: cmp-eq s_2_27 s_2_29
        let s_2_30: bool = ((s_2_27) == (s_2_29));
        // D s_2_31: not s_2_30
        let s_2_31: bool = !s_2_30;
        // N s_2_32: branch s_2_31 b10 b3
        if s_2_31 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var H:u8
        let s_3_0: bool = fn_state.H;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 1u16);
        // D s_3_2: read-var L:u8
        let s_3_2: bool = fn_state.L;
        // D s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 1u16);
        // D s_3_4: cast reint s_3_1 -> u128
        let s_3_4: u128 = (s_3_1.value() as u128);
        // D s_3_5: size-of s_3_1
        let s_3_5: u16 = s_3_1.length();
        // D s_3_6: cast reint s_3_3 -> u128
        let s_3_6: u128 = (s_3_3.value() as u128);
        // D s_3_7: size-of s_3_3
        let s_3_7: u16 = s_3_3.length();
        // D s_3_8: lsl s_3_4 s_3_7
        let s_3_8: u128 = s_3_4 << s_3_7;
        // D s_3_9: or s_3_8 s_3_6
        let s_3_9: u128 = ((s_3_8) | (s_3_6));
        // D s_3_10: add s_3_5 s_3_7
        let s_3_10: u16 = (s_3_5 + s_3_7);
        // D s_3_11: create-bits s_3_9 s_3_10
        let s_3_11: Bits = Bits::new(s_3_9, s_3_10);
        // D s_3_12: cast reint s_3_11 -> u8
        let s_3_12: u8 = (s_3_11.value() as u8);
        // D s_3_13: cast zx s_3_12 -> bv
        let s_3_13: Bits = Bits::new(s_3_12 as u128, 2u16);
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (s_3_13.value() as i128);
        // D s_3_15: cast reint s_3_14 -> i64
        let s_3_15: i64 = (s_3_14 as i64);
        // D s_3_16: write-var index <= s_3_15
        fn_state.index = s_3_15;
        // N s_3_17: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var index:i64
        let s_4_0: i64 = fn_state.index;
        // D s_4_1: write-var indexshadow#1417 <= s_4_0
        fn_state.indexshadow_1417 = s_4_0;
        // D s_4_2: read-var Rd:u8
        let s_4_2: u8 = fn_state.Rd;
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 5u16);
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (s_4_3.value() as i128);
        // D s_4_5: cast reint s_4_4 -> i64
        let s_4_5: i64 = (s_4_4 as i64);
        // D s_4_6: write-var d <= s_4_5
        fn_state.d = s_4_5;
        // D s_4_7: read-var Rn:u8
        let s_4_7: u8 = fn_state.Rn;
        // D s_4_8: cast zx s_4_7 -> bv
        let s_4_8: Bits = Bits::new(s_4_7 as u128, 5u16);
        // D s_4_9: cast zx s_4_8 -> i
        let s_4_9: i128 = (s_4_8.value() as i128);
        // D s_4_10: cast reint s_4_9 -> i64
        let s_4_10: i64 = (s_4_9 as i64);
        // D s_4_11: write-var n <= s_4_10
        fn_state.n = s_4_10;
        // D s_4_12: read-var Rmhi:u8
        let s_4_12: bool = fn_state.Rmhi;
        // D s_4_13: cast zx s_4_12 -> bv
        let s_4_13: Bits = Bits::new(s_4_12 as u128, 1u16);
        // D s_4_14: read-var Rm:u8
        let s_4_14: u8 = fn_state.Rm;
        // D s_4_15: cast zx s_4_14 -> bv
        let s_4_15: Bits = Bits::new(s_4_14 as u128, 4u16);
        // D s_4_16: cast reint s_4_13 -> u128
        let s_4_16: u128 = (s_4_13.value() as u128);
        // D s_4_17: size-of s_4_13
        let s_4_17: u16 = s_4_13.length();
        // D s_4_18: cast reint s_4_15 -> u128
        let s_4_18: u128 = (s_4_15.value() as u128);
        // D s_4_19: size-of s_4_15
        let s_4_19: u16 = s_4_15.length();
        // D s_4_20: lsl s_4_16 s_4_19
        let s_4_20: u128 = s_4_16 << s_4_19;
        // D s_4_21: or s_4_20 s_4_18
        let s_4_21: u128 = ((s_4_20) | (s_4_18));
        // D s_4_22: add s_4_17 s_4_19
        let s_4_22: u16 = (s_4_17 + s_4_19);
        // D s_4_23: create-bits s_4_21 s_4_22
        let s_4_23: Bits = Bits::new(s_4_21, s_4_22);
        // D s_4_24: cast reint s_4_23 -> u8
        let s_4_24: u8 = (s_4_23.value() as u8);
        // D s_4_25: cast zx s_4_24 -> bv
        let s_4_25: Bits = Bits::new(s_4_24 as u128, 5u16);
        // D s_4_26: cast zx s_4_25 -> i
        let s_4_26: i128 = (s_4_25.value() as i128);
        // D s_4_27: cast reint s_4_26 -> i64
        let s_4_27: i64 = (s_4_26 as i64);
        // D s_4_28: write-var m <= s_4_27
        fn_state.m = s_4_27;
        // D s_4_29: read-var sz:u8
        let s_4_29: bool = fn_state.sz;
        // D s_4_30: cast zx s_4_29 -> bv
        let s_4_30: Bits = Bits::new(s_4_29 as u128, 1u16);
        // D s_4_31: read-var Q:u8
        let s_4_31: bool = fn_state.Q;
        // D s_4_32: cast zx s_4_31 -> bv
        let s_4_32: Bits = Bits::new(s_4_31 as u128, 1u16);
        // D s_4_33: cast reint s_4_30 -> u128
        let s_4_33: u128 = (s_4_30.value() as u128);
        // D s_4_34: size-of s_4_30
        let s_4_34: u16 = s_4_30.length();
        // D s_4_35: cast reint s_4_32 -> u128
        let s_4_35: u128 = (s_4_32.value() as u128);
        // D s_4_36: size-of s_4_32
        let s_4_36: u16 = s_4_32.length();
        // D s_4_37: lsl s_4_33 s_4_36
        let s_4_37: u128 = s_4_33 << s_4_36;
        // D s_4_38: or s_4_37 s_4_35
        let s_4_38: u128 = ((s_4_37) | (s_4_35));
        // D s_4_39: add s_4_34 s_4_36
        let s_4_39: u16 = (s_4_34 + s_4_36);
        // D s_4_40: create-bits s_4_38 s_4_39
        let s_4_40: Bits = Bits::new(s_4_38, s_4_39);
        // D s_4_41: cast reint s_4_40 -> u8
        let s_4_41: u8 = (s_4_40.value() as u8);
        // D s_4_42: cast zx s_4_41 -> bv
        let s_4_42: Bits = Bits::new(s_4_41 as u128, 2u16);
        // C s_4_43: const #2u : u8
        let s_4_43: u8 = 2;
        // C s_4_44: cast zx s_4_43 -> bv
        let s_4_44: Bits = Bits::new(s_4_43 as u128, 2u16);
        // D s_4_45: cmp-eq s_4_42 s_4_44
        let s_4_45: bool = ((s_4_42) == (s_4_44));
        // N s_4_46: branch s_4_45 b9 b5
        if s_4_45 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var sz:u8
        let s_5_0: bool = fn_state.sz;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 1u16);
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (s_5_1.value() as i128);
        // D s_5_3: cast reint s_5_2 -> i64
        let s_5_3: i64 = (s_5_2 as i64);
        // C s_5_4: const #32s : i64
        let s_5_4: i64 = 32;
        // D s_5_5: lsl s_5_4 s_5_3
        let s_5_5: i64 = s_5_4 << s_5_3;
        // D s_5_6: write-var esize <= s_5_5
        fn_state.esize = s_5_5;
        // D s_5_7: read-var Q:u8
        let s_5_7: bool = fn_state.Q;
        // D s_5_8: cast zx s_5_7 -> bv
        let s_5_8: Bits = Bits::new(s_5_7 as u128, 1u16);
        // C s_5_9: const #1u : u8
        let s_5_9: bool = true;
        // C s_5_10: cast zx s_5_9 -> bv
        let s_5_10: Bits = Bits::new(s_5_9 as u128, 1u16);
        // D s_5_11: cmp-eq s_5_8 s_5_10
        let s_5_11: bool = ((s_5_8) == (s_5_10));
        // N s_5_12: branch s_5_11 b8 b6
        if s_5_11 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #64s : i64
        let s_6_0: i64 = 64;
        // D s_6_1: write-var ga#255966 <= s_6_0
        fn_state.ga_255966 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var ga#255966:i64
        let s_7_0: i64 = fn_state.ga_255966;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: read-var esize:i64
        let s_7_2: i64 = fn_state.esize;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: div s_7_1 s_7_3
        let s_7_4: i128 = ((s_7_1) / (s_7_3));
        // D s_7_5: cast reint s_7_4 -> i64
        let s_7_5: i64 = (s_7_4 as i64);
        // D s_7_6: read-var U:u8
        let s_7_6: bool = fn_state.U;
        // D s_7_7: cast zx s_7_6 -> bv
        let s_7_7: Bits = Bits::new(s_7_6 as u128, 1u16);
        // C s_7_8: const #1u : u8
        let s_7_8: bool = true;
        // C s_7_9: cast zx s_7_8 -> bv
        let s_7_9: Bits = Bits::new(s_7_8 as u128, 1u16);
        // D s_7_10: cmp-eq s_7_7 s_7_9
        let s_7_10: bool = ((s_7_7) == (s_7_9));
        // D s_7_11: cast zx s_7_0 -> i
        let s_7_11: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_12: cast reint s_7_11 -> i64
        let s_7_12: i64 = (s_7_11 as i64);
        // D s_7_13: read-var esize:i64
        let s_7_13: i64 = fn_state.esize;
        // D s_7_14: cast zx s_7_13 -> i
        let s_7_14: i128 = (i128::try_from(s_7_13).unwrap());
        // D s_7_15: cast reint s_7_14 -> i64
        let s_7_15: i64 = (s_7_14 as i64);
        // D s_7_16: read-var idxdsize:i64
        let s_7_16: i64 = fn_state.idxdsize;
        // D s_7_17: cast zx s_7_16 -> i
        let s_7_17: i128 = (i128::try_from(s_7_16).unwrap());
        // D s_7_18: cast reint s_7_17 -> i64
        let s_7_18: i64 = (s_7_17 as i64);
        // D s_7_19: cast zx s_7_5 -> i
        let s_7_19: i128 = (i128::try_from(s_7_5).unwrap());
        // D s_7_20: read-var d:i64
        let s_7_20: i64 = fn_state.d;
        // D s_7_21: read-var indexshadow#1417:i64
        let s_7_21: i64 = fn_state.indexshadow_1417;
        // D s_7_22: read-var m:i64
        let s_7_22: i64 = fn_state.m;
        // D s_7_23: read-var n:i64
        let s_7_23: i64 = fn_state.n;
        // D s_7_24: call execute_aarch64_instrs_vector_arithmetic_binary_element_mul_fp16_sisd(s_7_20, s_7_12, s_7_19, s_7_15, s_7_18, s_7_21, s_7_22, s_7_10, s_7_23)
        let s_7_24: () = execute_aarch64_instrs_vector_arithmetic_binary_element_mul_fp16_sisd(
            state,
            tracer,
            s_7_20,
            s_7_12,
            s_7_19,
            s_7_15,
            s_7_18,
            s_7_21,
            s_7_22,
            s_7_10,
            s_7_23,
        );
        // N s_7_25: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #128s : i64
        let s_8_0: i64 = 128;
        // D s_8_1: write-var ga#255966 <= s_8_0
        fn_state.ga_255966 = s_8_0;
        // N s_8_2: jump b7
        return block_7(state, tracer, fn_state);
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
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var ga#255952:u8
        let s_10_0: u8 = fn_state.ga_255952;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 2u16);
        // C s_10_2: const #2u : u8
        let s_10_2: u8 = 2;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 2u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // D s_10_5: not s_10_4
        let s_10_5: bool = !s_10_4;
        // N s_10_6: branch s_10_5 b12 b11
        if s_10_5 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var H:u8
        let s_11_0: bool = fn_state.H;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 1u16);
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (s_11_1.value() as i128);
        // D s_11_3: cast reint s_11_2 -> i64
        let s_11_3: i64 = (s_11_2 as i64);
        // D s_11_4: write-var index <= s_11_3
        fn_state.index = s_11_3;
        // N s_11_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: panic
        panic!("{:?}", ());
        // N s_12_1: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #128s : i64
        let s_13_0: i64 = 128;
        // D s_13_1: write-var ga#255951 <= s_13_0
        fn_state.ga_255951 = s_13_0;
        // N s_13_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
