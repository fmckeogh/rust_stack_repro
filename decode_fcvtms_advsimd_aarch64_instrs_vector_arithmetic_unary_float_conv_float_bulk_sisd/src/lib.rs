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
pub fn decode_fcvtms_advsimd_aarch64_instrs_vector_arithmetic_unary_float_conv_float_bulk_sisd<
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
) -> () {
    #[derive(Default)]
    struct FunctionState {
        Rd: u8,
        Rn: u8,
        o1: bool,
        sz: bool,
        o2: bool,
        U: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        o1,
        sz,
        o2,
        U,
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
        // D s_0_4: read-var Rn:u8
        let s_0_4: u8 = fn_state.Rn;
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 5u16);
        // D s_0_6: cast zx s_0_5 -> i
        let s_0_6: i128 = (s_0_5.value() as i128);
        // D s_0_7: cast reint s_0_6 -> i64
        let s_0_7: i64 = (s_0_6 as i64);
        // D s_0_8: read-var sz:u8
        let s_0_8: bool = fn_state.sz;
        // D s_0_9: cast zx s_0_8 -> bv
        let s_0_9: Bits = Bits::new(s_0_8 as u128, 1u16);
        // D s_0_10: cast zx s_0_9 -> i
        let s_0_10: i128 = (s_0_9.value() as i128);
        // D s_0_11: cast reint s_0_10 -> i64
        let s_0_11: i64 = (s_0_10 as i64);
        // C s_0_12: const #32s : i64
        let s_0_12: i64 = 32;
        // D s_0_13: lsl s_0_12 s_0_11
        let s_0_13: i64 = s_0_12 << s_0_11;
        // D s_0_14: read-var o1:u8
        let s_0_14: bool = fn_state.o1;
        // D s_0_15: cast zx s_0_14 -> bv
        let s_0_15: Bits = Bits::new(s_0_14 as u128, 1u16);
        // D s_0_16: read-var o2:u8
        let s_0_16: bool = fn_state.o2;
        // D s_0_17: cast zx s_0_16 -> bv
        let s_0_17: Bits = Bits::new(s_0_16 as u128, 1u16);
        // D s_0_18: cast reint s_0_15 -> u128
        let s_0_18: u128 = (s_0_15.value() as u128);
        // D s_0_19: size-of s_0_15
        let s_0_19: u16 = s_0_15.length();
        // D s_0_20: cast reint s_0_17 -> u128
        let s_0_20: u128 = (s_0_17.value() as u128);
        // D s_0_21: size-of s_0_17
        let s_0_21: u16 = s_0_17.length();
        // D s_0_22: lsl s_0_18 s_0_21
        let s_0_22: u128 = s_0_18 << s_0_21;
        // D s_0_23: or s_0_22 s_0_20
        let s_0_23: u128 = ((s_0_22) | (s_0_20));
        // D s_0_24: add s_0_19 s_0_21
        let s_0_24: u16 = (s_0_19 + s_0_21);
        // D s_0_25: create-bits s_0_23 s_0_24
        let s_0_25: Bits = Bits::new(s_0_23, s_0_24);
        // D s_0_26: cast reint s_0_25 -> u8
        let s_0_26: u8 = (s_0_25.value() as u8);
        // D s_0_27: call FPDecodeRounding(s_0_26)
        let s_0_27: u32 = FPDecodeRounding(state, tracer, s_0_26);
        // D s_0_28: read-var U:u8
        let s_0_28: bool = fn_state.U;
        // D s_0_29: cast zx s_0_28 -> bv
        let s_0_29: Bits = Bits::new(s_0_28 as u128, 1u16);
        // C s_0_30: const #1u : u8
        let s_0_30: bool = true;
        // C s_0_31: cast zx s_0_30 -> bv
        let s_0_31: Bits = Bits::new(s_0_30 as u128, 1u16);
        // D s_0_32: cmp-eq s_0_29 s_0_31
        let s_0_32: bool = ((s_0_29) == (s_0_31));
        // D s_0_33: cast zx s_0_13 -> i
        let s_0_33: i128 = (i128::try_from(s_0_13).unwrap());
        // D s_0_34: cast reint s_0_33 -> i64
        let s_0_34: i64 = (s_0_33 as i64);
        // D s_0_35: cast zx s_0_13 -> i
        let s_0_35: i128 = (i128::try_from(s_0_13).unwrap());
        // D s_0_36: cast reint s_0_35 -> i64
        let s_0_36: i64 = (s_0_35 as i64);
        // C s_0_37: const #1s : i
        let s_0_37: i128 = 1;
        // D s_0_38: call execute_aarch64_instrs_vector_arithmetic_unary_fp16_conv_float_bulk_sisd(s_0_3, s_0_34, s_0_37, s_0_36, s_0_7, s_0_27, s_0_32)
        let s_0_38: () = execute_aarch64_instrs_vector_arithmetic_unary_fp16_conv_float_bulk_sisd(
            state,
            tracer,
            s_0_3,
            s_0_34,
            s_0_37,
            s_0_36,
            s_0_7,
            s_0_27,
            s_0_32,
        );
        // N s_0_39: return
        return;
    }
}
