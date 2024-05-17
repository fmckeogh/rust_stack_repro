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
use HaveFP16Ext::*;
use execute_aarch64_instrs_vector_arithmetic_unary_fp16_conv_float_tieaway_sisd::*;
use common::*;
pub fn decode_fcvtas_advsimd_aarch64_instrs_vector_arithmetic_unary_fp16_conv_float_tieaway_sisd<
    T: Tracer,
>(state: &mut State, tracer: &T, Rd: u8, Rn: u8, U: bool) -> () {
    #[derive(Default)]
    struct FunctionState {
        Rd: u8,
        Rn: u8,
        U: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
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
        // C s_1_10: const #4u : u32
        let s_1_10: u32 = 4;
        // D s_1_11: read-var U:u8
        let s_1_11: bool = fn_state.U;
        // D s_1_12: cast zx s_1_11 -> bv
        let s_1_12: Bits = Bits::new(s_1_11 as u128, 1u16);
        // C s_1_13: const #1u : u8
        let s_1_13: bool = true;
        // C s_1_14: cast zx s_1_13 -> bv
        let s_1_14: Bits = Bits::new(s_1_13 as u128, 1u16);
        // D s_1_15: cmp-eq s_1_12 s_1_14
        let s_1_15: bool = ((s_1_12) == (s_1_14));
        // C s_1_16: cast zx s_1_8 -> i
        let s_1_16: i128 = (i128::try_from(s_1_8).unwrap());
        // C s_1_17: cast reint s_1_16 -> i64
        let s_1_17: i64 = (s_1_16 as i64);
        // C s_1_18: cast zx s_1_8 -> i
        let s_1_18: i128 = (i128::try_from(s_1_8).unwrap());
        // C s_1_19: cast reint s_1_18 -> i64
        let s_1_19: i64 = (s_1_18 as i64);
        // C s_1_20: cast zx s_1_9 -> i
        let s_1_20: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_21: call execute_aarch64_instrs_vector_arithmetic_unary_fp16_conv_float_tieaway_sisd(s_1_3, s_1_17, s_1_20, s_1_19, s_1_7, s_1_10, s_1_15)
        let s_1_21: () = execute_aarch64_instrs_vector_arithmetic_unary_fp16_conv_float_tieaway_sisd(
            state,
            tracer,
            s_1_3,
            s_1_17,
            s_1_20,
            s_1_19,
            s_1_7,
            s_1_10,
            s_1_15,
        );
        // N s_1_22: return
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