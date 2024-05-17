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
pub fn decode_sqrdmlah_advsimd_elt_aarch64_instrs_vector_arithmetic_binary_element_mul_acc_high_sisd<
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
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_267736: i64,
        index: i64,
        idxdsize: i64,
        Rmhi: bool,
        Rd: u8,
        Rn: u8,
        H: bool,
        S: bool,
        Rm: u8,
        M: bool,
        L: bool,
        size: u8,
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
        // N s_0_3: branch s_0_2 b10 b1
        if s_0_2 {
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
        // N s_1_5: branch s_1_4 b9 b2
        if s_1_4 {
            return block_9(state, tracer, fn_state);
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
        // D s_2_1: write-var ga#267736 <= s_2_0
        fn_state.ga_267736 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var ga#267736:i64
        let s_3_0: i64 = fn_state.ga_267736;
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
        // N s_3_8: branch s_3_7 b6 b4
        if s_3_7 {
            return block_6(state, tracer, fn_state);
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
        // D s_5_2: read-var Rd:u8
        let s_5_2: u8 = fn_state.Rd;
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 5u16);
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (s_5_3.value() as i128);
        // D s_5_5: cast reint s_5_4 -> i64
        let s_5_5: i64 = (s_5_4 as i64);
        // D s_5_6: read-var Rn:u8
        let s_5_6: u8 = fn_state.Rn;
        // D s_5_7: cast zx s_5_6 -> bv
        let s_5_7: Bits = Bits::new(s_5_6 as u128, 5u16);
        // D s_5_8: cast zx s_5_7 -> i
        let s_5_8: i128 = (s_5_7.value() as i128);
        // D s_5_9: cast reint s_5_8 -> i64
        let s_5_9: i64 = (s_5_8 as i64);
        // D s_5_10: cast zx s_5_0 -> bv
        let s_5_10: Bits = Bits::new(s_5_0 as u128, 1u16);
        // D s_5_11: read-var Rm:u8
        let s_5_11: u8 = fn_state.Rm;
        // D s_5_12: cast zx s_5_11 -> bv
        let s_5_12: Bits = Bits::new(s_5_11 as u128, 4u16);
        // D s_5_13: cast reint s_5_10 -> u128
        let s_5_13: u128 = (s_5_10.value() as u128);
        // D s_5_14: size-of s_5_10
        let s_5_14: u16 = s_5_10.length();
        // D s_5_15: cast reint s_5_12 -> u128
        let s_5_15: u128 = (s_5_12.value() as u128);
        // D s_5_16: size-of s_5_12
        let s_5_16: u16 = s_5_12.length();
        // D s_5_17: lsl s_5_13 s_5_16
        let s_5_17: u128 = s_5_13 << s_5_16;
        // D s_5_18: or s_5_17 s_5_15
        let s_5_18: u128 = ((s_5_17) | (s_5_15));
        // D s_5_19: add s_5_14 s_5_16
        let s_5_19: u16 = (s_5_14 + s_5_16);
        // D s_5_20: create-bits s_5_18 s_5_19
        let s_5_20: Bits = Bits::new(s_5_18, s_5_19);
        // D s_5_21: cast reint s_5_20 -> u8
        let s_5_21: u8 = (s_5_20.value() as u8);
        // D s_5_22: cast zx s_5_21 -> bv
        let s_5_22: Bits = Bits::new(s_5_21 as u128, 5u16);
        // D s_5_23: cast zx s_5_22 -> i
        let s_5_23: i128 = (s_5_22.value() as i128);
        // D s_5_24: cast reint s_5_23 -> i64
        let s_5_24: i64 = (s_5_23 as i64);
        // D s_5_25: read-var size:u8
        let s_5_25: u8 = fn_state.size;
        // D s_5_26: cast zx s_5_25 -> bv
        let s_5_26: Bits = Bits::new(s_5_25 as u128, 2u16);
        // D s_5_27: cast zx s_5_26 -> i
        let s_5_27: i128 = (s_5_26.value() as i128);
        // D s_5_28: cast reint s_5_27 -> i64
        let s_5_28: i64 = (s_5_27 as i64);
        // C s_5_29: const #8s : i64
        let s_5_29: i64 = 8;
        // D s_5_30: lsl s_5_29 s_5_28
        let s_5_30: i64 = s_5_29 << s_5_28;
        // D s_5_31: read-var S:u8
        let s_5_31: bool = fn_state.S;
        // D s_5_32: cast zx s_5_31 -> bv
        let s_5_32: Bits = Bits::new(s_5_31 as u128, 1u16);
        // C s_5_33: const #1u : u8
        let s_5_33: bool = true;
        // C s_5_34: cast zx s_5_33 -> bv
        let s_5_34: Bits = Bits::new(s_5_33 as u128, 1u16);
        // D s_5_35: cmp-eq s_5_32 s_5_34
        let s_5_35: bool = ((s_5_32) == (s_5_34));
        // D s_5_36: cast zx s_5_30 -> i
        let s_5_36: i128 = (i128::try_from(s_5_30).unwrap());
        // D s_5_37: cast reint s_5_36 -> i64
        let s_5_37: i64 = (s_5_36 as i64);
        // D s_5_38: cast zx s_5_30 -> i
        let s_5_38: i128 = (i128::try_from(s_5_30).unwrap());
        // D s_5_39: cast reint s_5_38 -> i64
        let s_5_39: i64 = (s_5_38 as i64);
        // D s_5_40: read-var idxdsize:i64
        let s_5_40: i64 = fn_state.idxdsize;
        // D s_5_41: cast zx s_5_40 -> i
        let s_5_41: i128 = (i128::try_from(s_5_40).unwrap());
        // D s_5_42: cast reint s_5_41 -> i64
        let s_5_42: i64 = (s_5_41 as i64);
        // C s_5_43: const #1s : i
        let s_5_43: i128 = 1;
        // C s_5_44: const #1u : u8
        let s_5_44: bool = true;
        // D s_5_45: call execute_aarch64_instrs_vector_arithmetic_binary_element_mul_acc_high_sisd(s_5_5, s_5_37, s_5_43, s_5_39, s_5_42, s_5_1, s_5_24, s_5_9, s_5_44, s_5_35)
        let s_5_45: () = execute_aarch64_instrs_vector_arithmetic_binary_element_mul_acc_high_sisd(
            state,
            tracer,
            s_5_5,
            s_5_37,
            s_5_43,
            s_5_39,
            s_5_42,
            s_5_1,
            s_5_24,
            s_5_9,
            s_5_44,
            s_5_35,
        );
        // N s_5_46: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var size:u8
        let s_6_0: u8 = fn_state.size;
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
        // D s_7_0: read-var H:u8
        let s_7_0: bool = fn_state.H;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 1u16);
        // D s_7_2: read-var L:u8
        let s_7_2: bool = fn_state.L;
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 1u16);
        // D s_7_4: cast reint s_7_1 -> u128
        let s_7_4: u128 = (s_7_1.value() as u128);
        // D s_7_5: size-of s_7_1
        let s_7_5: u16 = s_7_1.length();
        // D s_7_6: cast reint s_7_3 -> u128
        let s_7_6: u128 = (s_7_3.value() as u128);
        // D s_7_7: size-of s_7_3
        let s_7_7: u16 = s_7_3.length();
        // D s_7_8: lsl s_7_4 s_7_7
        let s_7_8: u128 = s_7_4 << s_7_7;
        // D s_7_9: or s_7_8 s_7_6
        let s_7_9: u128 = ((s_7_8) | (s_7_6));
        // D s_7_10: add s_7_5 s_7_7
        let s_7_10: u16 = (s_7_5 + s_7_7);
        // D s_7_11: create-bits s_7_9 s_7_10
        let s_7_11: Bits = Bits::new(s_7_9, s_7_10);
        // D s_7_12: cast reint s_7_11 -> u8
        let s_7_12: u8 = (s_7_11.value() as u8);
        // D s_7_13: cast zx s_7_12 -> bv
        let s_7_13: Bits = Bits::new(s_7_12 as u128, 2u16);
        // D s_7_14: cast zx s_7_13 -> i
        let s_7_14: i128 = (s_7_13.value() as i128);
        // D s_7_15: cast reint s_7_14 -> i64
        let s_7_15: i64 = (s_7_14 as i64);
        // D s_7_16: write-var index <= s_7_15
        fn_state.index = s_7_15;
        // D s_7_17: read-var M:u8
        let s_7_17: bool = fn_state.M;
        // D s_7_18: write-var Rmhi <= s_7_17
        fn_state.Rmhi = s_7_17;
        // N s_7_19: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: panic
        panic!("{:?}", ());
        // N s_8_1: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #128s : i64
        let s_9_0: i64 = 128;
        // D s_9_1: write-var ga#267736 <= s_9_0
        fn_state.ga_267736 = s_9_0;
        // N s_9_2: jump b3
        return block_3(state, tracer, fn_state);
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
}
