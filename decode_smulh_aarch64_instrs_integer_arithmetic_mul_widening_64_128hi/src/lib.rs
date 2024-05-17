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
use execute_aarch64_instrs_integer_arithmetic_mul_widening_64_128hi::*;
use common::*;
pub fn decode_smulh_aarch64_instrs_integer_arithmetic_mul_widening_64_128hi<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    Ra: u8,
    Rm: u8,
    U: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        Rd: u8,
        Rn: u8,
        Ra: u8,
        Rm: u8,
        U: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        Ra,
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
        // C s_0_12: const #64s : i64
        let s_0_12: i64 = 64;
        // D s_0_13: read-var U:u8
        let s_0_13: bool = fn_state.U;
        // D s_0_14: cast zx s_0_13 -> bv
        let s_0_14: Bits = Bits::new(s_0_13 as u128, 1u16);
        // C s_0_15: const #1u : u8
        let s_0_15: bool = true;
        // C s_0_16: cast zx s_0_15 -> bv
        let s_0_16: Bits = Bits::new(s_0_15 as u128, 1u16);
        // D s_0_17: cmp-eq s_0_14 s_0_16
        let s_0_17: bool = ((s_0_14) == (s_0_16));
        // D s_0_18: call execute_aarch64_instrs_integer_arithmetic_mul_widening_64_128hi(s_0_3, s_0_12, s_0_11, s_0_7, s_0_17)
        let s_0_18: () = execute_aarch64_instrs_integer_arithmetic_mul_widening_64_128hi(
            state,
            tracer,
            s_0_3,
            s_0_12,
            s_0_11,
            s_0_7,
            s_0_17,
        );
        // N s_0_19: return
        return;
    }
}
