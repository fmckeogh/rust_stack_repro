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
use execute_aarch64_instrs_integer_arithmetic_mul_widening_32_64::*;
use common::*;
pub fn decode_umsubl_aarch64_instrs_integer_arithmetic_mul_widening_32_64<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    Ra: u8,
    o0: bool,
    Rm: u8,
    U: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        Rd: u8,
        Rn: u8,
        Ra: u8,
        o0: bool,
        Rm: u8,
        U: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        Ra,
        o0,
        Rm,
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
        // D s_0_8: read-var Rm:u8
        let s_0_8: u8 = fn_state.Rm;
        // D s_0_9: cast zx s_0_8 -> bv
        let s_0_9: Bits = Bits::new(s_0_8 as u128, 5u16);
        // D s_0_10: cast zx s_0_9 -> i
        let s_0_10: i128 = (s_0_9.value() as i128);
        // D s_0_11: cast reint s_0_10 -> i64
        let s_0_11: i64 = (s_0_10 as i64);
        // D s_0_12: read-var Ra:u8
        let s_0_12: u8 = fn_state.Ra;
        // D s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 5u16);
        // D s_0_14: cast zx s_0_13 -> i
        let s_0_14: i128 = (s_0_13.value() as i128);
        // D s_0_15: cast reint s_0_14 -> i64
        let s_0_15: i64 = (s_0_14 as i64);
        // C s_0_16: const #64s : i64
        let s_0_16: i64 = 64;
        // C s_0_17: const #32s : i64
        let s_0_17: i64 = 32;
        // D s_0_18: read-var o0:u8
        let s_0_18: bool = fn_state.o0;
        // D s_0_19: cast zx s_0_18 -> bv
        let s_0_19: Bits = Bits::new(s_0_18 as u128, 1u16);
        // C s_0_20: const #1u : u8
        let s_0_20: bool = true;
        // C s_0_21: cast zx s_0_20 -> bv
        let s_0_21: Bits = Bits::new(s_0_20 as u128, 1u16);
        // D s_0_22: cmp-eq s_0_19 s_0_21
        let s_0_22: bool = ((s_0_19) == (s_0_21));
        // D s_0_23: read-var U:u8
        let s_0_23: bool = fn_state.U;
        // D s_0_24: cast zx s_0_23 -> bv
        let s_0_24: Bits = Bits::new(s_0_23 as u128, 1u16);
        // C s_0_25: const #1u : u8
        let s_0_25: bool = true;
        // C s_0_26: cast zx s_0_25 -> bv
        let s_0_26: Bits = Bits::new(s_0_25 as u128, 1u16);
        // D s_0_27: cmp-eq s_0_24 s_0_26
        let s_0_27: bool = ((s_0_24) == (s_0_26));
        // D s_0_28: call execute_aarch64_instrs_integer_arithmetic_mul_widening_32_64(s_0_15, s_0_3, s_0_17, s_0_16, s_0_11, s_0_7, s_0_22, s_0_27)
        let s_0_28: () = execute_aarch64_instrs_integer_arithmetic_mul_widening_32_64(
            state,
            tracer,
            s_0_15,
            s_0_3,
            s_0_17,
            s_0_16,
            s_0_11,
            s_0_7,
            s_0_22,
            s_0_27,
        );
        // N s_0_29: return
        return;
    }
}
