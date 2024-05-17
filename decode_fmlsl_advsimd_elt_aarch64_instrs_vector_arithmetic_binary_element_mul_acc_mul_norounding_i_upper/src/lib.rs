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
use execute_aarch64_instrs_vector_arithmetic_binary_element_mul_acc_mul_norounding_i_lower::*;
use HaveFP16MulNoRoundingToFP32Ext::*;
use common::*;
pub fn decode_fmlsl_advsimd_elt_aarch64_instrs_vector_arithmetic_binary_element_mul_acc_mul_norounding_i_upper<
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
    sz: bool,
    Q: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        ga_255714: i64,
        n: i64,
        index: i64,
        d: i64,
        Rd: u8,
        Rn: u8,
        H: bool,
        S: bool,
        Rm: u8,
        M: bool,
        L: bool,
        sz: bool,
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
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call HaveFP16MulNoRoundingToFP32Ext(s_0_0)
        let s_0_1: bool = HaveFP16MulNoRoundingToFP32Ext(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b7 b1
        if s_0_2 {
            return block_7(state, tracer, fn_state);
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
        // C s_1_10: const #0u : u8
        let s_1_10: bool = false;
        // C s_1_11: cast zx s_1_10 -> bv
        let s_1_11: Bits = Bits::new(s_1_10 as u128, 1u16);
        // D s_1_12: read-var Rm:u8
        let s_1_12: u8 = fn_state.Rm;
        // D s_1_13: cast zx s_1_12 -> bv
        let s_1_13: Bits = Bits::new(s_1_12 as u128, 4u16);
        // C s_1_14: cast reint s_1_11 -> u128
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
        // D s_1_23: cast zx s_1_22 -> bv
        let s_1_23: Bits = Bits::new(s_1_22 as u128, 5u16);
        // D s_1_24: cast zx s_1_23 -> i
        let s_1_24: i128 = (s_1_23.value() as i128);
        // D s_1_25: cast reint s_1_24 -> i64
        let s_1_25: i64 = (s_1_24 as i64);
        // D s_1_26: write-var m <= s_1_25
        fn_state.m = s_1_25;
        // D s_1_27: read-var sz:u8
        let s_1_27: bool = fn_state.sz;
        // D s_1_28: cast zx s_1_27 -> bv
        let s_1_28: Bits = Bits::new(s_1_27 as u128, 1u16);
        // C s_1_29: const #1u : u8
        let s_1_29: bool = true;
        // C s_1_30: cast zx s_1_29 -> bv
        let s_1_30: Bits = Bits::new(s_1_29 as u128, 1u16);
        // D s_1_31: cmp-eq s_1_28 s_1_30
        let s_1_31: bool = ((s_1_28) == (s_1_30));
        // N s_1_32: branch s_1_31 b6 b2
        if s_1_31 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var H:u8
        let s_2_0: bool = fn_state.H;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // D s_2_2: read-var L:u8
        let s_2_2: bool = fn_state.L;
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // D s_2_4: cast reint s_2_1 -> u128
        let s_2_4: u128 = (s_2_1.value() as u128);
        // D s_2_5: size-of s_2_1
        let s_2_5: u16 = s_2_1.length();
        // D s_2_6: cast reint s_2_3 -> u128
        let s_2_6: u128 = (s_2_3.value() as u128);
        // D s_2_7: size-of s_2_3
        let s_2_7: u16 = s_2_3.length();
        // D s_2_8: lsl s_2_4 s_2_7
        let s_2_8: u128 = s_2_4 << s_2_7;
        // D s_2_9: or s_2_8 s_2_6
        let s_2_9: u128 = ((s_2_8) | (s_2_6));
        // D s_2_10: add s_2_5 s_2_7
        let s_2_10: u16 = (s_2_5 + s_2_7);
        // D s_2_11: create-bits s_2_9 s_2_10
        let s_2_11: Bits = Bits::new(s_2_9, s_2_10);
        // D s_2_12: cast reint s_2_11 -> u8
        let s_2_12: u8 = (s_2_11.value() as u8);
        // D s_2_13: cast zx s_2_12 -> bv
        let s_2_13: Bits = Bits::new(s_2_12 as u128, 2u16);
        // D s_2_14: read-var M:u8
        let s_2_14: bool = fn_state.M;
        // D s_2_15: cast zx s_2_14 -> bv
        let s_2_15: Bits = Bits::new(s_2_14 as u128, 1u16);
        // D s_2_16: cast reint s_2_13 -> u128
        let s_2_16: u128 = (s_2_13.value() as u128);
        // D s_2_17: size-of s_2_13
        let s_2_17: u16 = s_2_13.length();
        // D s_2_18: cast reint s_2_15 -> u128
        let s_2_18: u128 = (s_2_15.value() as u128);
        // D s_2_19: size-of s_2_15
        let s_2_19: u16 = s_2_15.length();
        // D s_2_20: lsl s_2_16 s_2_19
        let s_2_20: u128 = s_2_16 << s_2_19;
        // D s_2_21: or s_2_20 s_2_18
        let s_2_21: u128 = ((s_2_20) | (s_2_18));
        // D s_2_22: add s_2_17 s_2_19
        let s_2_22: u16 = (s_2_17 + s_2_19);
        // D s_2_23: create-bits s_2_21 s_2_22
        let s_2_23: Bits = Bits::new(s_2_21, s_2_22);
        // D s_2_24: cast reint s_2_23 -> u8
        let s_2_24: u8 = (s_2_23.value() as u8);
        // D s_2_25: cast zx s_2_24 -> bv
        let s_2_25: Bits = Bits::new(s_2_24 as u128, 3u16);
        // D s_2_26: cast zx s_2_25 -> i
        let s_2_26: i128 = (s_2_25.value() as i128);
        // D s_2_27: cast reint s_2_26 -> i64
        let s_2_27: i64 = (s_2_26 as i64);
        // D s_2_28: write-var index <= s_2_27
        fn_state.index = s_2_27;
        // D s_2_29: read-var Q:u8
        let s_2_29: bool = fn_state.Q;
        // D s_2_30: cast zx s_2_29 -> bv
        let s_2_30: Bits = Bits::new(s_2_29 as u128, 1u16);
        // C s_2_31: const #1u : u8
        let s_2_31: bool = true;
        // C s_2_32: cast zx s_2_31 -> bv
        let s_2_32: Bits = Bits::new(s_2_31 as u128, 1u16);
        // D s_2_33: cmp-eq s_2_30 s_2_32
        let s_2_33: bool = ((s_2_30) == (s_2_32));
        // N s_2_34: branch s_2_33 b5 b3
        if s_2_33 {
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
        // D s_3_1: write-var ga#255714 <= s_3_0
        fn_state.ga_255714 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var ga#255714:i64
        let s_4_0: i64 = fn_state.ga_255714;
        // C s_4_1: const #32s : i
        let s_4_1: i128 = 32;
        // D s_4_2: cast zx s_4_0 -> i
        let s_4_2: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_3: div s_4_2 s_4_1
        let s_4_3: i128 = ((s_4_2) / (s_4_1));
        // D s_4_4: cast reint s_4_3 -> i64
        let s_4_4: i64 = (s_4_3 as i64);
        // D s_4_5: read-var S:u8
        let s_4_5: bool = fn_state.S;
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
        // D s_4_12: cast zx s_4_4 -> i
        let s_4_12: i128 = (i128::try_from(s_4_4).unwrap());
        // C s_4_13: const #32s : i64
        let s_4_13: i64 = 32;
        // C s_4_14: const #1s : i64
        let s_4_14: i64 = 1;
        // D s_4_15: read-var d:i64
        let s_4_15: i64 = fn_state.d;
        // D s_4_16: read-var index:i64
        let s_4_16: i64 = fn_state.index;
        // D s_4_17: read-var m:i64
        let s_4_17: i64 = fn_state.m;
        // D s_4_18: read-var n:i64
        let s_4_18: i64 = fn_state.n;
        // D s_4_19: call execute_aarch64_instrs_vector_arithmetic_binary_element_mul_acc_mul_norounding_i_lower(s_4_15, s_4_11, s_4_12, s_4_13, s_4_16, s_4_17, s_4_18, s_4_14, s_4_9)
        let s_4_19: () = execute_aarch64_instrs_vector_arithmetic_binary_element_mul_acc_mul_norounding_i_lower(
            state,
            tracer,
            s_4_15,
            s_4_11,
            s_4_12,
            s_4_13,
            s_4_16,
            s_4_17,
            s_4_18,
            s_4_14,
            s_4_9,
        );
        // N s_4_20: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #128s : i64
        let s_5_0: i64 = 128;
        // D s_5_1: write-var ga#255714 <= s_5_0
        fn_state.ga_255714 = s_5_0;
        // N s_5_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: panic
        panic!("{:?}", ());
        // N s_6_1: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: panic
        panic!("{:?}", ());
        // N s_7_1: return
        return;
    }
}
