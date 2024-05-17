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
pub fn decode_fmul_advsimd_elt_aarch64_instrs_vector_arithmetic_binary_element_mul_fp16_sisd<
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
    U: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_255911: i64,
        Rd: u8,
        Rn: u8,
        H: bool,
        Rm: u8,
        M: bool,
        L: bool,
        U: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        H,
        Rm,
        M,
        L,
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
        // N s_0_5: branch s_0_4 b3 b1
        if s_0_4 {
            return block_3(state, tracer, fn_state);
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
        // D s_1_1: write-var ga#255911 <= s_1_0
        fn_state.ga_255911 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var ga#255911:i64
        let s_2_0: i64 = fn_state.ga_255911;
        // D s_2_1: read-var Rn:u8
        let s_2_1: u8 = fn_state.Rn;
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 5u16);
        // D s_2_3: cast zx s_2_2 -> i
        let s_2_3: i128 = (s_2_2.value() as i128);
        // D s_2_4: cast reint s_2_3 -> i64
        let s_2_4: i64 = (s_2_3 as i64);
        // D s_2_5: read-var Rm:u8
        let s_2_5: u8 = fn_state.Rm;
        // D s_2_6: cast zx s_2_5 -> bv
        let s_2_6: Bits = Bits::new(s_2_5 as u128, 4u16);
        // D s_2_7: cast zx s_2_6 -> i
        let s_2_7: i128 = (s_2_6.value() as i128);
        // D s_2_8: cast reint s_2_7 -> i64
        let s_2_8: i64 = (s_2_7 as i64);
        // D s_2_9: read-var Rd:u8
        let s_2_9: u8 = fn_state.Rd;
        // D s_2_10: cast zx s_2_9 -> bv
        let s_2_10: Bits = Bits::new(s_2_9 as u128, 5u16);
        // D s_2_11: cast zx s_2_10 -> i
        let s_2_11: i128 = (s_2_10.value() as i128);
        // D s_2_12: cast reint s_2_11 -> i64
        let s_2_12: i64 = (s_2_11 as i64);
        // D s_2_13: read-var H:u8
        let s_2_13: bool = fn_state.H;
        // D s_2_14: cast zx s_2_13 -> bv
        let s_2_14: Bits = Bits::new(s_2_13 as u128, 1u16);
        // D s_2_15: read-var L:u8
        let s_2_15: bool = fn_state.L;
        // D s_2_16: cast zx s_2_15 -> bv
        let s_2_16: Bits = Bits::new(s_2_15 as u128, 1u16);
        // D s_2_17: cast reint s_2_14 -> u128
        let s_2_17: u128 = (s_2_14.value() as u128);
        // D s_2_18: size-of s_2_14
        let s_2_18: u16 = s_2_14.length();
        // D s_2_19: cast reint s_2_16 -> u128
        let s_2_19: u128 = (s_2_16.value() as u128);
        // D s_2_20: size-of s_2_16
        let s_2_20: u16 = s_2_16.length();
        // D s_2_21: lsl s_2_17 s_2_20
        let s_2_21: u128 = s_2_17 << s_2_20;
        // D s_2_22: or s_2_21 s_2_19
        let s_2_22: u128 = ((s_2_21) | (s_2_19));
        // D s_2_23: add s_2_18 s_2_20
        let s_2_23: u16 = (s_2_18 + s_2_20);
        // D s_2_24: create-bits s_2_22 s_2_23
        let s_2_24: Bits = Bits::new(s_2_22, s_2_23);
        // D s_2_25: cast reint s_2_24 -> u8
        let s_2_25: u8 = (s_2_24.value() as u8);
        // D s_2_26: cast zx s_2_25 -> bv
        let s_2_26: Bits = Bits::new(s_2_25 as u128, 2u16);
        // D s_2_27: read-var M:u8
        let s_2_27: bool = fn_state.M;
        // D s_2_28: cast zx s_2_27 -> bv
        let s_2_28: Bits = Bits::new(s_2_27 as u128, 1u16);
        // D s_2_29: cast reint s_2_26 -> u128
        let s_2_29: u128 = (s_2_26.value() as u128);
        // D s_2_30: size-of s_2_26
        let s_2_30: u16 = s_2_26.length();
        // D s_2_31: cast reint s_2_28 -> u128
        let s_2_31: u128 = (s_2_28.value() as u128);
        // D s_2_32: size-of s_2_28
        let s_2_32: u16 = s_2_28.length();
        // D s_2_33: lsl s_2_29 s_2_32
        let s_2_33: u128 = s_2_29 << s_2_32;
        // D s_2_34: or s_2_33 s_2_31
        let s_2_34: u128 = ((s_2_33) | (s_2_31));
        // D s_2_35: add s_2_30 s_2_32
        let s_2_35: u16 = (s_2_30 + s_2_32);
        // D s_2_36: create-bits s_2_34 s_2_35
        let s_2_36: Bits = Bits::new(s_2_34, s_2_35);
        // D s_2_37: cast reint s_2_36 -> u8
        let s_2_37: u8 = (s_2_36.value() as u8);
        // D s_2_38: cast zx s_2_37 -> bv
        let s_2_38: Bits = Bits::new(s_2_37 as u128, 3u16);
        // D s_2_39: cast zx s_2_38 -> i
        let s_2_39: i128 = (s_2_38.value() as i128);
        // D s_2_40: cast reint s_2_39 -> i64
        let s_2_40: i64 = (s_2_39 as i64);
        // D s_2_41: read-var U:u8
        let s_2_41: bool = fn_state.U;
        // D s_2_42: cast zx s_2_41 -> bv
        let s_2_42: Bits = Bits::new(s_2_41 as u128, 1u16);
        // C s_2_43: const #1u : u8
        let s_2_43: bool = true;
        // C s_2_44: cast zx s_2_43 -> bv
        let s_2_44: Bits = Bits::new(s_2_43 as u128, 1u16);
        // D s_2_45: cmp-eq s_2_42 s_2_44
        let s_2_45: bool = ((s_2_42) == (s_2_44));
        // C s_2_46: const #16s : i64
        let s_2_46: i64 = 16;
        // C s_2_47: const #16s : i64
        let s_2_47: i64 = 16;
        // D s_2_48: cast zx s_2_0 -> i
        let s_2_48: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_49: cast reint s_2_48 -> i64
        let s_2_49: i64 = (s_2_48 as i64);
        // C s_2_50: const #1s : i
        let s_2_50: i128 = 1;
        // D s_2_51: call execute_aarch64_instrs_vector_arithmetic_binary_element_mul_fp16_sisd(s_2_12, s_2_46, s_2_50, s_2_47, s_2_49, s_2_40, s_2_8, s_2_45, s_2_4)
        let s_2_51: () = execute_aarch64_instrs_vector_arithmetic_binary_element_mul_fp16_sisd(
            state,
            tracer,
            s_2_12,
            s_2_46,
            s_2_50,
            s_2_47,
            s_2_49,
            s_2_40,
            s_2_8,
            s_2_45,
            s_2_4,
        );
        // N s_2_52: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #128s : i64
        let s_3_0: i64 = 128;
        // D s_3_1: write-var ga#255911 <= s_3_0
        fn_state.ga_255911 = s_3_0;
        // N s_3_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
