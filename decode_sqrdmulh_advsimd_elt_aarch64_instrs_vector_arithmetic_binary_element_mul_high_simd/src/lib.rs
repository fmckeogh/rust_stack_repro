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
use execute_aarch64_instrs_vector_arithmetic_binary_element_mul_high_sisd::*;
use common::*;
pub fn decode_sqrdmulh_advsimd_elt_aarch64_instrs_vector_arithmetic_binary_element_mul_high_simd<
    T: Tracer,
>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    H: bool,
    op: bool,
    Rm: u8,
    M: bool,
    L: bool,
    size: u8,
    Q: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        esize: i64,
        index: i64,
        n: i64,
        d: i64,
        indexshadow_1924: i64,
        ga_267537: i64,
        idxdsize: i64,
        Rmhi: bool,
        ga_267525: i64,
        Rd: u8,
        Rn: u8,
        H: bool,
        op: bool,
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
        op,
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
        // N s_0_5: branch s_0_4 b11 b1
        if s_0_4 {
            return block_11(state, tracer, fn_state);
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
        // D s_1_1: write-var ga#267525 <= s_1_0
        fn_state.ga_267525 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var ga#267525:i64
        let s_2_0: i64 = fn_state.ga_267525;
        // D s_2_1: write-var idxdsize <= s_2_0
        fn_state.idxdsize = s_2_0;
        // D s_2_2: read-var size:u8
        let s_2_2: u8 = fn_state.size;
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 2u16);
        // C s_2_4: const #1u : u8
        let s_2_4: u8 = 1;
        // C s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 2u16);
        // D s_2_6: cmp-eq s_2_3 s_2_5
        let s_2_6: bool = ((s_2_3) == (s_2_5));
        // D s_2_7: not s_2_6
        let s_2_7: bool = !s_2_6;
        // N s_2_8: branch s_2_7 b8 b3
        if s_2_7 {
            return block_8(state, tracer, fn_state);
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
        // D s_3_14: read-var M:u8
        let s_3_14: bool = fn_state.M;
        // D s_3_15: cast zx s_3_14 -> bv
        let s_3_15: Bits = Bits::new(s_3_14 as u128, 1u16);
        // D s_3_16: cast reint s_3_13 -> u128
        let s_3_16: u128 = (s_3_13.value() as u128);
        // D s_3_17: size-of s_3_13
        let s_3_17: u16 = s_3_13.length();
        // D s_3_18: cast reint s_3_15 -> u128
        let s_3_18: u128 = (s_3_15.value() as u128);
        // D s_3_19: size-of s_3_15
        let s_3_19: u16 = s_3_15.length();
        // D s_3_20: lsl s_3_16 s_3_19
        let s_3_20: u128 = s_3_16 << s_3_19;
        // D s_3_21: or s_3_20 s_3_18
        let s_3_21: u128 = ((s_3_20) | (s_3_18));
        // D s_3_22: add s_3_17 s_3_19
        let s_3_22: u16 = (s_3_17 + s_3_19);
        // D s_3_23: create-bits s_3_21 s_3_22
        let s_3_23: Bits = Bits::new(s_3_21, s_3_22);
        // D s_3_24: cast reint s_3_23 -> u8
        let s_3_24: u8 = (s_3_23.value() as u8);
        // D s_3_25: cast zx s_3_24 -> bv
        let s_3_25: Bits = Bits::new(s_3_24 as u128, 3u16);
        // D s_3_26: cast zx s_3_25 -> i
        let s_3_26: i128 = (s_3_25.value() as i128);
        // D s_3_27: cast reint s_3_26 -> i64
        let s_3_27: i64 = (s_3_26 as i64);
        // D s_3_28: write-var index <= s_3_27
        fn_state.index = s_3_27;
        // C s_3_29: const #0u : u8
        let s_3_29: bool = false;
        // D s_3_30: write-var Rmhi <= s_3_29
        fn_state.Rmhi = s_3_29;
        // N s_3_31: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var Rmhi:u8
        let s_4_0: bool = fn_state.Rmhi;
        // D s_4_1: read-var index:i64
        let s_4_1: i64 = fn_state.index;
        // D s_4_2: write-var indexshadow#1924 <= s_4_1
        fn_state.indexshadow_1924 = s_4_1;
        // D s_4_3: read-var Rd:u8
        let s_4_3: u8 = fn_state.Rd;
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 5u16);
        // D s_4_5: cast zx s_4_4 -> i
        let s_4_5: i128 = (s_4_4.value() as i128);
        // D s_4_6: cast reint s_4_5 -> i64
        let s_4_6: i64 = (s_4_5 as i64);
        // D s_4_7: write-var d <= s_4_6
        fn_state.d = s_4_6;
        // D s_4_8: read-var Rn:u8
        let s_4_8: u8 = fn_state.Rn;
        // D s_4_9: cast zx s_4_8 -> bv
        let s_4_9: Bits = Bits::new(s_4_8 as u128, 5u16);
        // D s_4_10: cast zx s_4_9 -> i
        let s_4_10: i128 = (s_4_9.value() as i128);
        // D s_4_11: cast reint s_4_10 -> i64
        let s_4_11: i64 = (s_4_10 as i64);
        // D s_4_12: write-var n <= s_4_11
        fn_state.n = s_4_11;
        // D s_4_13: cast zx s_4_0 -> bv
        let s_4_13: Bits = Bits::new(s_4_0 as u128, 1u16);
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
        // D s_4_29: read-var size:u8
        let s_4_29: u8 = fn_state.size;
        // D s_4_30: cast zx s_4_29 -> bv
        let s_4_30: Bits = Bits::new(s_4_29 as u128, 2u16);
        // D s_4_31: cast zx s_4_30 -> i
        let s_4_31: i128 = (s_4_30.value() as i128);
        // D s_4_32: cast reint s_4_31 -> i64
        let s_4_32: i64 = (s_4_31 as i64);
        // C s_4_33: const #8s : i64
        let s_4_33: i64 = 8;
        // D s_4_34: lsl s_4_33 s_4_32
        let s_4_34: i64 = s_4_33 << s_4_32;
        // D s_4_35: write-var esize <= s_4_34
        fn_state.esize = s_4_34;
        // D s_4_36: read-var Q:u8
        let s_4_36: bool = fn_state.Q;
        // D s_4_37: cast zx s_4_36 -> bv
        let s_4_37: Bits = Bits::new(s_4_36 as u128, 1u16);
        // C s_4_38: const #1u : u8
        let s_4_38: bool = true;
        // C s_4_39: cast zx s_4_38 -> bv
        let s_4_39: Bits = Bits::new(s_4_38 as u128, 1u16);
        // D s_4_40: cmp-eq s_4_37 s_4_39
        let s_4_40: bool = ((s_4_37) == (s_4_39));
        // N s_4_41: branch s_4_40 b7 b5
        if s_4_40 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #64s : i64
        let s_5_0: i64 = 64;
        // D s_5_1: write-var ga#267537 <= s_5_0
        fn_state.ga_267537 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ga#267537:i64
        let s_6_0: i64 = fn_state.ga_267537;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: read-var esize:i64
        let s_6_2: i64 = fn_state.esize;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: div s_6_1 s_6_3
        let s_6_4: i128 = ((s_6_1) / (s_6_3));
        // D s_6_5: cast reint s_6_4 -> i64
        let s_6_5: i64 = (s_6_4 as i64);
        // D s_6_6: read-var op:u8
        let s_6_6: bool = fn_state.op;
        // D s_6_7: cast zx s_6_6 -> bv
        let s_6_7: Bits = Bits::new(s_6_6 as u128, 1u16);
        // C s_6_8: const #1u : u8
        let s_6_8: bool = true;
        // C s_6_9: cast zx s_6_8 -> bv
        let s_6_9: Bits = Bits::new(s_6_8 as u128, 1u16);
        // D s_6_10: cmp-eq s_6_7 s_6_9
        let s_6_10: bool = ((s_6_7) == (s_6_9));
        // D s_6_11: cast zx s_6_0 -> i
        let s_6_11: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_12: cast reint s_6_11 -> i64
        let s_6_12: i64 = (s_6_11 as i64);
        // D s_6_13: read-var esize:i64
        let s_6_13: i64 = fn_state.esize;
        // D s_6_14: cast zx s_6_13 -> i
        let s_6_14: i128 = (i128::try_from(s_6_13).unwrap());
        // D s_6_15: cast reint s_6_14 -> i64
        let s_6_15: i64 = (s_6_14 as i64);
        // D s_6_16: read-var idxdsize:i64
        let s_6_16: i64 = fn_state.idxdsize;
        // D s_6_17: cast zx s_6_16 -> i
        let s_6_17: i128 = (i128::try_from(s_6_16).unwrap());
        // D s_6_18: cast reint s_6_17 -> i64
        let s_6_18: i64 = (s_6_17 as i64);
        // D s_6_19: cast zx s_6_5 -> i
        let s_6_19: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_20: read-var d:i64
        let s_6_20: i64 = fn_state.d;
        // D s_6_21: read-var indexshadow#1924:i64
        let s_6_21: i64 = fn_state.indexshadow_1924;
        // D s_6_22: read-var m:i64
        let s_6_22: i64 = fn_state.m;
        // D s_6_23: read-var n:i64
        let s_6_23: i64 = fn_state.n;
        // D s_6_24: call execute_aarch64_instrs_vector_arithmetic_binary_element_mul_high_sisd(s_6_20, s_6_12, s_6_19, s_6_15, s_6_18, s_6_21, s_6_22, s_6_23, s_6_10)
        let s_6_24: () = execute_aarch64_instrs_vector_arithmetic_binary_element_mul_high_sisd(
            state,
            tracer,
            s_6_20,
            s_6_12,
            s_6_19,
            s_6_15,
            s_6_18,
            s_6_21,
            s_6_22,
            s_6_23,
            s_6_10,
        );
        // N s_6_25: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #128s : i64
        let s_7_0: i64 = 128;
        // D s_7_1: write-var ga#267537 <= s_7_0
        fn_state.ga_267537 = s_7_0;
        // N s_7_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var size:u8
        let s_8_0: u8 = fn_state.size;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 2u16);
        // C s_8_2: const #2u : u8
        let s_8_2: u8 = 2;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 2u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // D s_8_5: not s_8_4
        let s_8_5: bool = !s_8_4;
        // N s_8_6: branch s_8_5 b10 b9
        if s_8_5 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var H:u8
        let s_9_0: bool = fn_state.H;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 1u16);
        // D s_9_2: read-var L:u8
        let s_9_2: bool = fn_state.L;
        // D s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 1u16);
        // D s_9_4: cast reint s_9_1 -> u128
        let s_9_4: u128 = (s_9_1.value() as u128);
        // D s_9_5: size-of s_9_1
        let s_9_5: u16 = s_9_1.length();
        // D s_9_6: cast reint s_9_3 -> u128
        let s_9_6: u128 = (s_9_3.value() as u128);
        // D s_9_7: size-of s_9_3
        let s_9_7: u16 = s_9_3.length();
        // D s_9_8: lsl s_9_4 s_9_7
        let s_9_8: u128 = s_9_4 << s_9_7;
        // D s_9_9: or s_9_8 s_9_6
        let s_9_9: u128 = ((s_9_8) | (s_9_6));
        // D s_9_10: add s_9_5 s_9_7
        let s_9_10: u16 = (s_9_5 + s_9_7);
        // D s_9_11: create-bits s_9_9 s_9_10
        let s_9_11: Bits = Bits::new(s_9_9, s_9_10);
        // D s_9_12: cast reint s_9_11 -> u8
        let s_9_12: u8 = (s_9_11.value() as u8);
        // D s_9_13: cast zx s_9_12 -> bv
        let s_9_13: Bits = Bits::new(s_9_12 as u128, 2u16);
        // D s_9_14: cast zx s_9_13 -> i
        let s_9_14: i128 = (s_9_13.value() as i128);
        // D s_9_15: cast reint s_9_14 -> i64
        let s_9_15: i64 = (s_9_14 as i64);
        // D s_9_16: write-var index <= s_9_15
        fn_state.index = s_9_15;
        // D s_9_17: read-var M:u8
        let s_9_17: bool = fn_state.M;
        // D s_9_18: write-var Rmhi <= s_9_17
        fn_state.Rmhi = s_9_17;
        // N s_9_19: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: panic
        panic!("{:?}", ());
        // N s_10_1: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #128s : i64
        let s_11_0: i64 = 128;
        // D s_11_1: write-var ga#267525 <= s_11_0
        fn_state.ga_267525 = s_11_0;
        // N s_11_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
