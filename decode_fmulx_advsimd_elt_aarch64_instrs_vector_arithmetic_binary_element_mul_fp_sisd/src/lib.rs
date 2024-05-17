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
pub fn decode_fmulx_advsimd_elt_aarch64_instrs_vector_arithmetic_binary_element_mul_fp_sisd<
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
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_255984: u8,
        index: i64,
        ga_255983: i64,
        idxdsize: i64,
        Rmhi: bool,
        Rd: u8,
        Rn: u8,
        H: bool,
        Rm: u8,
        M: bool,
        L: bool,
        sz: bool,
        U: bool,
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
        // N s_0_5: branch s_0_4 b8 b1
        if s_0_4 {
            return block_8(state, tracer, fn_state);
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
        // D s_1_1: write-var ga#255983 <= s_1_0
        fn_state.ga_255983 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var ga#255983:i64
        let s_2_0: i64 = fn_state.ga_255983;
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
        // D s_2_17: write-var ga#255984 <= s_2_16
        fn_state.ga_255984 = s_2_16;
        // D s_2_18: read-var ga#255984:u8
        let s_2_18: u8 = fn_state.ga_255984;
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
        // N s_2_32: branch s_2_31 b5 b3
        if s_2_31 {
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
        // D s_4_1: read-var Rd:u8
        let s_4_1: u8 = fn_state.Rd;
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 5u16);
        // D s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (s_4_2.value() as i128);
        // D s_4_4: cast reint s_4_3 -> i64
        let s_4_4: i64 = (s_4_3 as i64);
        // D s_4_5: read-var Rn:u8
        let s_4_5: u8 = fn_state.Rn;
        // D s_4_6: cast zx s_4_5 -> bv
        let s_4_6: Bits = Bits::new(s_4_5 as u128, 5u16);
        // D s_4_7: cast zx s_4_6 -> i
        let s_4_7: i128 = (s_4_6.value() as i128);
        // D s_4_8: cast reint s_4_7 -> i64
        let s_4_8: i64 = (s_4_7 as i64);
        // D s_4_9: read-var Rmhi:u8
        let s_4_9: bool = fn_state.Rmhi;
        // D s_4_10: cast zx s_4_9 -> bv
        let s_4_10: Bits = Bits::new(s_4_9 as u128, 1u16);
        // D s_4_11: read-var Rm:u8
        let s_4_11: u8 = fn_state.Rm;
        // D s_4_12: cast zx s_4_11 -> bv
        let s_4_12: Bits = Bits::new(s_4_11 as u128, 4u16);
        // D s_4_13: cast reint s_4_10 -> u128
        let s_4_13: u128 = (s_4_10.value() as u128);
        // D s_4_14: size-of s_4_10
        let s_4_14: u16 = s_4_10.length();
        // D s_4_15: cast reint s_4_12 -> u128
        let s_4_15: u128 = (s_4_12.value() as u128);
        // D s_4_16: size-of s_4_12
        let s_4_16: u16 = s_4_12.length();
        // D s_4_17: lsl s_4_13 s_4_16
        let s_4_17: u128 = s_4_13 << s_4_16;
        // D s_4_18: or s_4_17 s_4_15
        let s_4_18: u128 = ((s_4_17) | (s_4_15));
        // D s_4_19: add s_4_14 s_4_16
        let s_4_19: u16 = (s_4_14 + s_4_16);
        // D s_4_20: create-bits s_4_18 s_4_19
        let s_4_20: Bits = Bits::new(s_4_18, s_4_19);
        // D s_4_21: cast reint s_4_20 -> u8
        let s_4_21: u8 = (s_4_20.value() as u8);
        // D s_4_22: cast zx s_4_21 -> bv
        let s_4_22: Bits = Bits::new(s_4_21 as u128, 5u16);
        // D s_4_23: cast zx s_4_22 -> i
        let s_4_23: i128 = (s_4_22.value() as i128);
        // D s_4_24: cast reint s_4_23 -> i64
        let s_4_24: i64 = (s_4_23 as i64);
        // D s_4_25: read-var sz:u8
        let s_4_25: bool = fn_state.sz;
        // D s_4_26: cast zx s_4_25 -> bv
        let s_4_26: Bits = Bits::new(s_4_25 as u128, 1u16);
        // D s_4_27: cast zx s_4_26 -> i
        let s_4_27: i128 = (s_4_26.value() as i128);
        // D s_4_28: cast reint s_4_27 -> i64
        let s_4_28: i64 = (s_4_27 as i64);
        // C s_4_29: const #32s : i64
        let s_4_29: i64 = 32;
        // D s_4_30: lsl s_4_29 s_4_28
        let s_4_30: i64 = s_4_29 << s_4_28;
        // D s_4_31: read-var U:u8
        let s_4_31: bool = fn_state.U;
        // D s_4_32: cast zx s_4_31 -> bv
        let s_4_32: Bits = Bits::new(s_4_31 as u128, 1u16);
        // C s_4_33: const #1u : u8
        let s_4_33: bool = true;
        // C s_4_34: cast zx s_4_33 -> bv
        let s_4_34: Bits = Bits::new(s_4_33 as u128, 1u16);
        // D s_4_35: cmp-eq s_4_32 s_4_34
        let s_4_35: bool = ((s_4_32) == (s_4_34));
        // D s_4_36: cast zx s_4_30 -> i
        let s_4_36: i128 = (i128::try_from(s_4_30).unwrap());
        // D s_4_37: cast reint s_4_36 -> i64
        let s_4_37: i64 = (s_4_36 as i64);
        // D s_4_38: cast zx s_4_30 -> i
        let s_4_38: i128 = (i128::try_from(s_4_30).unwrap());
        // D s_4_39: cast reint s_4_38 -> i64
        let s_4_39: i64 = (s_4_38 as i64);
        // D s_4_40: read-var idxdsize:i64
        let s_4_40: i64 = fn_state.idxdsize;
        // D s_4_41: cast zx s_4_40 -> i
        let s_4_41: i128 = (i128::try_from(s_4_40).unwrap());
        // D s_4_42: cast reint s_4_41 -> i64
        let s_4_42: i64 = (s_4_41 as i64);
        // C s_4_43: const #1s : i
        let s_4_43: i128 = 1;
        // D s_4_44: call execute_aarch64_instrs_vector_arithmetic_binary_element_mul_fp16_sisd(s_4_4, s_4_37, s_4_43, s_4_39, s_4_42, s_4_0, s_4_24, s_4_35, s_4_8)
        let s_4_44: () = execute_aarch64_instrs_vector_arithmetic_binary_element_mul_fp16_sisd(
            state,
            tracer,
            s_4_4,
            s_4_37,
            s_4_43,
            s_4_39,
            s_4_42,
            s_4_0,
            s_4_24,
            s_4_35,
            s_4_8,
        );
        // N s_4_45: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var ga#255984:u8
        let s_5_0: u8 = fn_state.ga_255984;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // C s_5_2: const #2u : u8
        let s_5_2: u8 = 2;
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
        // D s_6_0: read-var H:u8
        let s_6_0: bool = fn_state.H;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 1u16);
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (s_6_1.value() as i128);
        // D s_6_3: cast reint s_6_2 -> i64
        let s_6_3: i64 = (s_6_2 as i64);
        // D s_6_4: write-var index <= s_6_3
        fn_state.index = s_6_3;
        // N s_6_5: jump b4
        return block_4(state, tracer, fn_state);
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
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #128s : i64
        let s_8_0: i64 = 128;
        // D s_8_1: write-var ga#255983 <= s_8_0
        fn_state.ga_255983 = s_8_0;
        // N s_8_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
