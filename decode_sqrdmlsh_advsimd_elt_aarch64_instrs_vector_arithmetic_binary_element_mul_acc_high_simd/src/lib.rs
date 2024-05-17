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
use HaveQRDMLAHExt::*;
use execute_aarch64_instrs_vector_arithmetic_binary_element_mul_acc_high_sisd::*;
use common::*;
pub fn decode_sqrdmlsh_advsimd_elt_aarch64_instrs_vector_arithmetic_binary_element_mul_acc_high_simd<
    T: Tracer,
>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    H: bool,
    S: bool,
    Rm: u8,
    M: bool,
    L: bool,
    size: u8,
    Q: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_267805: i64,
        m: i64,
        esize: i64,
        n: i64,
        index: i64,
        d: i64,
        ga_267793: i64,
        idxdsize: i64,
        Rmhi: bool,
        indexshadow_1946: i64,
        Rd: u8,
        Rn: u8,
        H: bool,
        S: bool,
        Rm: u8,
        M: bool,
        L: bool,
        size: u8,
        Q: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        H,
        S,
        Rm,
        M,
        L,
        size,
        Q,
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
        // S s_0_1: call HaveQRDMLAHExt(s_0_0)
        let s_0_1: bool = HaveQRDMLAHExt(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b13 b1
        if s_0_2 {
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
        // D s_1_0: read-var H:u8
        let s_1_0: bool = fn_state.H;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 1u16);
        // C s_1_2: const #1u : u8
        let s_1_2: bool = true;
        // C s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 1u16);
        // D s_1_4: cmp-eq s_1_1 s_1_3
        let s_1_4: bool = ((s_1_1) == (s_1_3));
        // N s_1_5: branch s_1_4 b12 b2
        if s_1_4 {
            return block_12(state, tracer, fn_state);
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
        // D s_2_1: write-var ga#267793 <= s_2_0
        fn_state.ga_267793 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var ga#267793:i64
        let s_3_0: i64 = fn_state.ga_267793;
        // D s_3_1: write-var idxdsize <= s_3_0
        fn_state.idxdsize = s_3_0;
        // D s_3_2: read-var size:u8
        let s_3_2: u8 = fn_state.size;
        // D s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 2u16);
        // C s_3_4: const #1u : u8
        let s_3_4: u8 = 1;
        // C s_3_5: cast zx s_3_4 -> bv
        let s_3_5: Bits = Bits::new(s_3_4 as u128, 2u16);
        // D s_3_6: cmp-eq s_3_3 s_3_5
        let s_3_6: bool = ((s_3_3) == (s_3_5));
        // D s_3_7: not s_3_6
        let s_3_7: bool = !s_3_6;
        // N s_3_8: branch s_3_7 b9 b4
        if s_3_7 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var H:u8
        let s_4_0: bool = fn_state.H;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 1u16);
        // D s_4_2: read-var L:u8
        let s_4_2: bool = fn_state.L;
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 1u16);
        // D s_4_4: cast reint s_4_1 -> u128
        let s_4_4: u128 = (s_4_1.value() as u128);
        // D s_4_5: size-of s_4_1
        let s_4_5: u16 = s_4_1.length();
        // D s_4_6: cast reint s_4_3 -> u128
        let s_4_6: u128 = (s_4_3.value() as u128);
        // D s_4_7: size-of s_4_3
        let s_4_7: u16 = s_4_3.length();
        // D s_4_8: lsl s_4_4 s_4_7
        let s_4_8: u128 = s_4_4 << s_4_7;
        // D s_4_9: or s_4_8 s_4_6
        let s_4_9: u128 = ((s_4_8) | (s_4_6));
        // D s_4_10: add s_4_5 s_4_7
        let s_4_10: u16 = (s_4_5 + s_4_7);
        // D s_4_11: create-bits s_4_9 s_4_10
        let s_4_11: Bits = Bits::new(s_4_9, s_4_10);
        // D s_4_12: cast reint s_4_11 -> u8
        let s_4_12: u8 = (s_4_11.value() as u8);
        // D s_4_13: cast zx s_4_12 -> bv
        let s_4_13: Bits = Bits::new(s_4_12 as u128, 2u16);
        // D s_4_14: read-var M:u8
        let s_4_14: bool = fn_state.M;
        // D s_4_15: cast zx s_4_14 -> bv
        let s_4_15: Bits = Bits::new(s_4_14 as u128, 1u16);
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
        let s_4_25: Bits = Bits::new(s_4_24 as u128, 3u16);
        // D s_4_26: cast zx s_4_25 -> i
        let s_4_26: i128 = (s_4_25.value() as i128);
        // D s_4_27: cast reint s_4_26 -> i64
        let s_4_27: i64 = (s_4_26 as i64);
        // D s_4_28: write-var index <= s_4_27
        fn_state.index = s_4_27;
        // C s_4_29: const #0u : u8
        let s_4_29: bool = false;
        // D s_4_30: write-var Rmhi <= s_4_29
        fn_state.Rmhi = s_4_29;
        // N s_4_31: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var Rmhi:u8
        let s_5_0: bool = fn_state.Rmhi;
        // D s_5_1: read-var index:i64
        let s_5_1: i64 = fn_state.index;
        // D s_5_2: write-var indexshadow#1946 <= s_5_1
        fn_state.indexshadow_1946 = s_5_1;
        // D s_5_3: read-var Rd:u8
        let s_5_3: u8 = fn_state.Rd;
        // D s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 5u16);
        // D s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (s_5_4.value() as i128);
        // D s_5_6: cast reint s_5_5 -> i64
        let s_5_6: i64 = (s_5_5 as i64);
        // D s_5_7: write-var d <= s_5_6
        fn_state.d = s_5_6;
        // D s_5_8: read-var Rn:u8
        let s_5_8: u8 = fn_state.Rn;
        // D s_5_9: cast zx s_5_8 -> bv
        let s_5_9: Bits = Bits::new(s_5_8 as u128, 5u16);
        // D s_5_10: cast zx s_5_9 -> i
        let s_5_10: i128 = (s_5_9.value() as i128);
        // D s_5_11: cast reint s_5_10 -> i64
        let s_5_11: i64 = (s_5_10 as i64);
        // D s_5_12: write-var n <= s_5_11
        fn_state.n = s_5_11;
        // D s_5_13: cast zx s_5_0 -> bv
        let s_5_13: Bits = Bits::new(s_5_0 as u128, 1u16);
        // D s_5_14: read-var Rm:u8
        let s_5_14: u8 = fn_state.Rm;
        // D s_5_15: cast zx s_5_14 -> bv
        let s_5_15: Bits = Bits::new(s_5_14 as u128, 4u16);
        // D s_5_16: cast reint s_5_13 -> u128
        let s_5_16: u128 = (s_5_13.value() as u128);
        // D s_5_17: size-of s_5_13
        let s_5_17: u16 = s_5_13.length();
        // D s_5_18: cast reint s_5_15 -> u128
        let s_5_18: u128 = (s_5_15.value() as u128);
        // D s_5_19: size-of s_5_15
        let s_5_19: u16 = s_5_15.length();
        // D s_5_20: lsl s_5_16 s_5_19
        let s_5_20: u128 = s_5_16 << s_5_19;
        // D s_5_21: or s_5_20 s_5_18
        let s_5_21: u128 = ((s_5_20) | (s_5_18));
        // D s_5_22: add s_5_17 s_5_19
        let s_5_22: u16 = (s_5_17 + s_5_19);
        // D s_5_23: create-bits s_5_21 s_5_22
        let s_5_23: Bits = Bits::new(s_5_21, s_5_22);
        // D s_5_24: cast reint s_5_23 -> u8
        let s_5_24: u8 = (s_5_23.value() as u8);
        // D s_5_25: cast zx s_5_24 -> bv
        let s_5_25: Bits = Bits::new(s_5_24 as u128, 5u16);
        // D s_5_26: cast zx s_5_25 -> i
        let s_5_26: i128 = (s_5_25.value() as i128);
        // D s_5_27: cast reint s_5_26 -> i64
        let s_5_27: i64 = (s_5_26 as i64);
        // D s_5_28: write-var m <= s_5_27
        fn_state.m = s_5_27;
        // D s_5_29: read-var size:u8
        let s_5_29: u8 = fn_state.size;
        // D s_5_30: cast zx s_5_29 -> bv
        let s_5_30: Bits = Bits::new(s_5_29 as u128, 2u16);
        // D s_5_31: cast zx s_5_30 -> i
        let s_5_31: i128 = (s_5_30.value() as i128);
        // D s_5_32: cast reint s_5_31 -> i64
        let s_5_32: i64 = (s_5_31 as i64);
        // C s_5_33: const #8s : i64
        let s_5_33: i64 = 8;
        // D s_5_34: lsl s_5_33 s_5_32
        let s_5_34: i64 = s_5_33 << s_5_32;
        // D s_5_35: write-var esize <= s_5_34
        fn_state.esize = s_5_34;
        // D s_5_36: read-var Q:u8
        let s_5_36: bool = fn_state.Q;
        // D s_5_37: cast zx s_5_36 -> bv
        let s_5_37: Bits = Bits::new(s_5_36 as u128, 1u16);
        // C s_5_38: const #1u : u8
        let s_5_38: bool = true;
        // C s_5_39: cast zx s_5_38 -> bv
        let s_5_39: Bits = Bits::new(s_5_38 as u128, 1u16);
        // D s_5_40: cmp-eq s_5_37 s_5_39
        let s_5_40: bool = ((s_5_37) == (s_5_39));
        // N s_5_41: branch s_5_40 b8 b6
        if s_5_40 {
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
        // D s_6_1: write-var ga#267805 <= s_6_0
        fn_state.ga_267805 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var ga#267805:i64
        let s_7_0: i64 = fn_state.ga_267805;
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
        // D s_7_6: read-var S:u8
        let s_7_6: bool = fn_state.S;
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
        // D s_7_21: read-var indexshadow#1946:i64
        let s_7_21: i64 = fn_state.indexshadow_1946;
        // D s_7_22: read-var m:i64
        let s_7_22: i64 = fn_state.m;
        // D s_7_23: read-var n:i64
        let s_7_23: i64 = fn_state.n;
        // C s_7_24: const #1u : u8
        let s_7_24: bool = true;
        // D s_7_25: call execute_aarch64_instrs_vector_arithmetic_binary_element_mul_acc_high_sisd(s_7_20, s_7_12, s_7_19, s_7_15, s_7_18, s_7_21, s_7_22, s_7_23, s_7_24, s_7_10)
        let s_7_25: () = execute_aarch64_instrs_vector_arithmetic_binary_element_mul_acc_high_sisd(
            state,
            tracer,
            s_7_20,
            s_7_12,
            s_7_19,
            s_7_15,
            s_7_18,
            s_7_21,
            s_7_22,
            s_7_23,
            s_7_24,
            s_7_10,
        );
        // N s_7_26: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #128s : i64
        let s_8_0: i64 = 128;
        // D s_8_1: write-var ga#267805 <= s_8_0
        fn_state.ga_267805 = s_8_0;
        // N s_8_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var size:u8
        let s_9_0: u8 = fn_state.size;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 2u16);
        // C s_9_2: const #2u : u8
        let s_9_2: u8 = 2;
        // C s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 2u16);
        // D s_9_4: cmp-eq s_9_1 s_9_3
        let s_9_4: bool = ((s_9_1) == (s_9_3));
        // D s_9_5: not s_9_4
        let s_9_5: bool = !s_9_4;
        // N s_9_6: branch s_9_5 b11 b10
        if s_9_5 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var H:u8
        let s_10_0: bool = fn_state.H;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 1u16);
        // D s_10_2: read-var L:u8
        let s_10_2: bool = fn_state.L;
        // D s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // D s_10_4: cast reint s_10_1 -> u128
        let s_10_4: u128 = (s_10_1.value() as u128);
        // D s_10_5: size-of s_10_1
        let s_10_5: u16 = s_10_1.length();
        // D s_10_6: cast reint s_10_3 -> u128
        let s_10_6: u128 = (s_10_3.value() as u128);
        // D s_10_7: size-of s_10_3
        let s_10_7: u16 = s_10_3.length();
        // D s_10_8: lsl s_10_4 s_10_7
        let s_10_8: u128 = s_10_4 << s_10_7;
        // D s_10_9: or s_10_8 s_10_6
        let s_10_9: u128 = ((s_10_8) | (s_10_6));
        // D s_10_10: add s_10_5 s_10_7
        let s_10_10: u16 = (s_10_5 + s_10_7);
        // D s_10_11: create-bits s_10_9 s_10_10
        let s_10_11: Bits = Bits::new(s_10_9, s_10_10);
        // D s_10_12: cast reint s_10_11 -> u8
        let s_10_12: u8 = (s_10_11.value() as u8);
        // D s_10_13: cast zx s_10_12 -> bv
        let s_10_13: Bits = Bits::new(s_10_12 as u128, 2u16);
        // D s_10_14: cast zx s_10_13 -> i
        let s_10_14: i128 = (s_10_13.value() as i128);
        // D s_10_15: cast reint s_10_14 -> i64
        let s_10_15: i64 = (s_10_14 as i64);
        // D s_10_16: write-var index <= s_10_15
        fn_state.index = s_10_15;
        // D s_10_17: read-var M:u8
        let s_10_17: bool = fn_state.M;
        // D s_10_18: write-var Rmhi <= s_10_17
        fn_state.Rmhi = s_10_17;
        // N s_10_19: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: panic
        panic!("{:?}", ());
        // N s_11_1: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #128s : i64
        let s_12_0: i64 = 128;
        // D s_12_1: write-var ga#267793 <= s_12_0
        fn_state.ga_267793 = s_12_0;
        // N s_12_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: panic
        panic!("{:?}", ());
        // N s_13_1: return
        return;
    }
}
