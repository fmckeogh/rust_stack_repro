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
use HaveFP16Ext::*;
use execute_aarch64_instrs_vector_arithmetic_unary_fp16_conv_float_bulk_sisd::*;
use common::*;
pub fn decode_fcvtps_advsimd_aarch64_instrs_vector_arithmetic_unary_fp16_conv_float_bulk_sisd<
    T: Tracer,
>(state: &mut State, tracer: &T, Rd: u8, Rn: u8, o1: bool, o2: bool, U: bool) -> () {
    #[derive(Default)]
    struct FunctionState {
        Rd: u8,
        Rn: u8,
        o1: bool,
        o2: bool,
        U: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        o1,
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
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call HaveFP16Ext(s_0_0)
        let s_0_1: bool = HaveFP16Ext(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b2 b1
        if s_0_2 {
            return block_2(state, tracer, fn_state);
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
        // D s_1_4: read-var Rn:u8
        let s_1_4: u8 = fn_state.Rn;
        // D s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 5u16);
        // D s_1_6: cast zx s_1_5 -> i
        let s_1_6: i128 = (s_1_5.value() as i128);
        // D s_1_7: cast reint s_1_6 -> i64
        let s_1_7: i64 = (s_1_6 as i64);
        // C s_1_8: const #16s : i64
        let s_1_8: i64 = 16;
        // C s_1_9: const #1s : i64
        let s_1_9: i64 = 1;
        // D s_1_10: read-var o1:u8
        let s_1_10: bool = fn_state.o1;
        // D s_1_11: cast zx s_1_10 -> bv
        let s_1_11: Bits = Bits::new(s_1_10 as u128, 1u16);
        // D s_1_12: read-var o2:u8
        let s_1_12: bool = fn_state.o2;
        // D s_1_13: cast zx s_1_12 -> bv
        let s_1_13: Bits = Bits::new(s_1_12 as u128, 1u16);
        // D s_1_14: cast reint s_1_11 -> u128
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
        // D s_1_23: call FPDecodeRounding(s_1_22)
        let s_1_23: u32 = FPDecodeRounding(state, tracer, s_1_22);
        // D s_1_24: read-var U:u8
        let s_1_24: bool = fn_state.U;
        // D s_1_25: cast zx s_1_24 -> bv
        let s_1_25: Bits = Bits::new(s_1_24 as u128, 1u16);
        // C s_1_26: const #1u : u8
        let s_1_26: bool = true;
        // C s_1_27: cast zx s_1_26 -> bv
        let s_1_27: Bits = Bits::new(s_1_26 as u128, 1u16);
        // D s_1_28: cmp-eq s_1_25 s_1_27
        let s_1_28: bool = ((s_1_25) == (s_1_27));
        // C s_1_29: cast zx s_1_8 -> i
        let s_1_29: i128 = (i128::try_from(s_1_8).unwrap());
        // C s_1_30: cast reint s_1_29 -> i64
        let s_1_30: i64 = (s_1_29 as i64);
        // C s_1_31: cast zx s_1_8 -> i
        let s_1_31: i128 = (i128::try_from(s_1_8).unwrap());
        // C s_1_32: cast reint s_1_31 -> i64
        let s_1_32: i64 = (s_1_31 as i64);
        // C s_1_33: cast zx s_1_9 -> i
        let s_1_33: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_34: call execute_aarch64_instrs_vector_arithmetic_unary_fp16_conv_float_bulk_sisd(s_1_3, s_1_30, s_1_33, s_1_32, s_1_7, s_1_23, s_1_28)
        let s_1_34: () = execute_aarch64_instrs_vector_arithmetic_unary_fp16_conv_float_bulk_sisd(
            state,
            tracer,
            s_1_3,
            s_1_30,
            s_1_33,
            s_1_32,
            s_1_7,
            s_1_23,
            s_1_28,
        );
        // N s_1_35: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_2_0: panic
        panic!("{:?}", ());
        // N s_2_1: return
        return;
    }
}
