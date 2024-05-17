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
use execute_aarch64_instrs_vector_arithmetic_binary_element_mul_acc_long::*;
use common::*;
pub fn decode_umlsl_advsimd_elt_aarch64_instrs_vector_arithmetic_binary_element_mul_acc_long<
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
    size: u8,
    U: bool,
    Q: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        index: i64,
        ga_266959: i64,
        idxdsize: i64,
        Rmhi: bool,
        Rd: u8,
        Rn: u8,
        H: bool,
        o2: bool,
        Rm: u8,
        M: bool,
        L: bool,
        size: u8,
        U: bool,
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
        size,
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
        // D s_1_1: write-var ga#266959 <= s_1_0
        fn_state.ga_266959 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var ga#266959:i64
        let s_2_0: i64 = fn_state.ga_266959;
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
        // N s_2_8: branch s_2_7 b5 b3
        if s_2_7 {
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
        // D s_4_2: read-var Rd:u8
        let s_4_2: u8 = fn_state.Rd;
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 5u16);
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (s_4_3.value() as i128);
        // D s_4_5: cast reint s_4_4 -> i64
        let s_4_5: i64 = (s_4_4 as i64);
        // D s_4_6: read-var Rn:u8
        let s_4_6: u8 = fn_state.Rn;
        // D s_4_7: cast zx s_4_6 -> bv
        let s_4_7: Bits = Bits::new(s_4_6 as u128, 5u16);
        // D s_4_8: cast zx s_4_7 -> i
        let s_4_8: i128 = (s_4_7.value() as i128);
        // D s_4_9: cast reint s_4_8 -> i64
        let s_4_9: i64 = (s_4_8 as i64);
        // D s_4_10: cast zx s_4_0 -> bv
        let s_4_10: Bits = Bits::new(s_4_0 as u128, 1u16);
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
        // D s_4_25: read-var size:u8
        let s_4_25: u8 = fn_state.size;
        // D s_4_26: cast zx s_4_25 -> bv
        let s_4_26: Bits = Bits::new(s_4_25 as u128, 2u16);
        // D s_4_27: cast zx s_4_26 -> i
        let s_4_27: i128 = (s_4_26.value() as i128);
        // D s_4_28: cast reint s_4_27 -> i64
        let s_4_28: i64 = (s_4_27 as i64);
        // C s_4_29: const #8s : i64
        let s_4_29: i64 = 8;
        // D s_4_30: lsl s_4_29 s_4_28
        let s_4_30: i64 = s_4_29 << s_4_28;
        // D s_4_31: read-var Q:u8
        let s_4_31: bool = fn_state.Q;
        // D s_4_32: cast zx s_4_31 -> bv
        let s_4_32: Bits = Bits::new(s_4_31 as u128, 1u16);
        // D s_4_33: cast zx s_4_32 -> i
        let s_4_33: i128 = (s_4_32.value() as i128);
        // D s_4_34: cast reint s_4_33 -> i64
        let s_4_34: i64 = (s_4_33 as i64);
        // C s_4_35: const #64s : i
        let s_4_35: i128 = 64;
        // D s_4_36: cast zx s_4_30 -> i
        let s_4_36: i128 = (i128::try_from(s_4_30).unwrap());
        // D s_4_37: div s_4_35 s_4_36
        let s_4_37: i128 = ((s_4_35) / (s_4_36));
        // D s_4_38: cast reint s_4_37 -> i64
        let s_4_38: i64 = (s_4_37 as i64);
        // D s_4_39: read-var U:u8
        let s_4_39: bool = fn_state.U;
        // D s_4_40: cast zx s_4_39 -> bv
        let s_4_40: Bits = Bits::new(s_4_39 as u128, 1u16);
        // C s_4_41: const #1u : u8
        let s_4_41: bool = true;
        // C s_4_42: cast zx s_4_41 -> bv
        let s_4_42: Bits = Bits::new(s_4_41 as u128, 1u16);
        // D s_4_43: cmp-eq s_4_40 s_4_42
        let s_4_43: bool = ((s_4_40) == (s_4_42));
        // D s_4_44: read-var o2:u8
        let s_4_44: bool = fn_state.o2;
        // D s_4_45: cast zx s_4_44 -> bv
        let s_4_45: Bits = Bits::new(s_4_44 as u128, 1u16);
        // C s_4_46: const #1u : u8
        let s_4_46: bool = true;
        // C s_4_47: cast zx s_4_46 -> bv
        let s_4_47: Bits = Bits::new(s_4_46 as u128, 1u16);
        // D s_4_48: cmp-eq s_4_45 s_4_47
        let s_4_48: bool = ((s_4_45) == (s_4_47));
        // D s_4_49: cast zx s_4_30 -> i
        let s_4_49: i128 = (i128::try_from(s_4_30).unwrap());
        // D s_4_50: cast reint s_4_49 -> i64
        let s_4_50: i64 = (s_4_49 as i64);
        // D s_4_51: read-var idxdsize:i64
        let s_4_51: i64 = fn_state.idxdsize;
        // D s_4_52: cast zx s_4_51 -> i
        let s_4_52: i128 = (i128::try_from(s_4_51).unwrap());
        // D s_4_53: cast reint s_4_52 -> i64
        let s_4_53: i64 = (s_4_52 as i64);
        // C s_4_54: const #64s : i64
        let s_4_54: i64 = 64;
        // D s_4_55: cast zx s_4_38 -> i
        let s_4_55: i128 = (i128::try_from(s_4_38).unwrap());
        // D s_4_56: call execute_aarch64_instrs_vector_arithmetic_binary_element_mul_acc_long(s_4_5, s_4_54, s_4_55, s_4_50, s_4_53, s_4_1, s_4_24, s_4_9, s_4_34, s_4_48, s_4_43)
        let s_4_56: () = execute_aarch64_instrs_vector_arithmetic_binary_element_mul_acc_long(
            state,
            tracer,
            s_4_5,
            s_4_54,
            s_4_55,
            s_4_50,
            s_4_53,
            s_4_1,
            s_4_24,
            s_4_9,
            s_4_34,
            s_4_48,
            s_4_43,
        );
        // N s_4_57: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var size:u8
        let s_5_0: u8 = fn_state.size;
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
        // D s_6_2: read-var L:u8
        let s_6_2: bool = fn_state.L;
        // D s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // D s_6_4: cast reint s_6_1 -> u128
        let s_6_4: u128 = (s_6_1.value() as u128);
        // D s_6_5: size-of s_6_1
        let s_6_5: u16 = s_6_1.length();
        // D s_6_6: cast reint s_6_3 -> u128
        let s_6_6: u128 = (s_6_3.value() as u128);
        // D s_6_7: size-of s_6_3
        let s_6_7: u16 = s_6_3.length();
        // D s_6_8: lsl s_6_4 s_6_7
        let s_6_8: u128 = s_6_4 << s_6_7;
        // D s_6_9: or s_6_8 s_6_6
        let s_6_9: u128 = ((s_6_8) | (s_6_6));
        // D s_6_10: add s_6_5 s_6_7
        let s_6_10: u16 = (s_6_5 + s_6_7);
        // D s_6_11: create-bits s_6_9 s_6_10
        let s_6_11: Bits = Bits::new(s_6_9, s_6_10);
        // D s_6_12: cast reint s_6_11 -> u8
        let s_6_12: u8 = (s_6_11.value() as u8);
        // D s_6_13: cast zx s_6_12 -> bv
        let s_6_13: Bits = Bits::new(s_6_12 as u128, 2u16);
        // D s_6_14: cast zx s_6_13 -> i
        let s_6_14: i128 = (s_6_13.value() as i128);
        // D s_6_15: cast reint s_6_14 -> i64
        let s_6_15: i64 = (s_6_14 as i64);
        // D s_6_16: write-var index <= s_6_15
        fn_state.index = s_6_15;
        // D s_6_17: read-var M:u8
        let s_6_17: bool = fn_state.M;
        // D s_6_18: write-var Rmhi <= s_6_17
        fn_state.Rmhi = s_6_17;
        // N s_6_19: jump b4
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
        // D s_8_1: write-var ga#266959 <= s_8_0
        fn_state.ga_266959 = s_8_0;
        // N s_8_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
