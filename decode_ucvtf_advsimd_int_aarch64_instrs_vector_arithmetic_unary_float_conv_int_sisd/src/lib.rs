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
use execute_aarch64_instrs_vector_arithmetic_unary_fp16_conv_int_sisd::*;
use common::*;
pub fn decode_ucvtf_advsimd_int_aarch64_instrs_vector_arithmetic_unary_float_conv_int_sisd<
    T: Tracer,
>(state: &mut State, tracer: &T, Rd: u8, Rn: u8, sz: bool, U: bool) -> () {
    #[derive(Default)]
    struct FunctionState {
        Rd: u8,
        Rn: u8,
        sz: bool,
        U: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
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
        // D s_0_14: read-var U:u8
        let s_0_14: bool = fn_state.U;
        // D s_0_15: cast zx s_0_14 -> bv
        let s_0_15: Bits = Bits::new(s_0_14 as u128, 1u16);
        // C s_0_16: const #1u : u8
        let s_0_16: bool = true;
        // C s_0_17: cast zx s_0_16 -> bv
        let s_0_17: Bits = Bits::new(s_0_16 as u128, 1u16);
        // D s_0_18: cmp-eq s_0_15 s_0_17
        let s_0_18: bool = ((s_0_15) == (s_0_17));
        // D s_0_19: cast zx s_0_13 -> i
        let s_0_19: i128 = (i128::try_from(s_0_13).unwrap());
        // D s_0_20: cast reint s_0_19 -> i64
        let s_0_20: i64 = (s_0_19 as i64);
        // D s_0_21: cast zx s_0_13 -> i
        let s_0_21: i128 = (i128::try_from(s_0_13).unwrap());
        // D s_0_22: cast reint s_0_21 -> i64
        let s_0_22: i64 = (s_0_21 as i64);
        // C s_0_23: const #1s : i
        let s_0_23: i128 = 1;
        // D s_0_24: call execute_aarch64_instrs_vector_arithmetic_unary_fp16_conv_int_sisd(s_0_3, s_0_20, s_0_23, s_0_22, s_0_7, s_0_18)
        let s_0_24: () = execute_aarch64_instrs_vector_arithmetic_unary_fp16_conv_int_sisd(
            state,
            tracer,
            s_0_3,
            s_0_20,
            s_0_23,
            s_0_22,
            s_0_7,
            s_0_18,
        );
        // N s_0_25: return
        return;
    }
}
