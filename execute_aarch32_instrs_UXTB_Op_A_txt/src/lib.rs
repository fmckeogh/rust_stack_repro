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
use R_set::*;
use ROR::*;
use R_read::*;
use common::*;
pub fn execute_aarch32_instrs_UXTB_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
    rotation: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        d: i64,
        m: i64,
        rotation: i64,
    }
    let fn_state = FunctionState {
        d,
        m,
        rotation,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var m:i64
        let s_0_0: i64 = fn_state.m;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: call R_read(s_0_1)
        let s_0_2: u32 = R_read(state, tracer, s_0_1);
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 32u16);
        // D s_0_4: read-var rotation:i64
        let s_0_4: i64 = fn_state.rotation;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: call ROR(s_0_3, s_0_5)
        let s_0_6: Bits = ROR(state, tracer, s_0_3, s_0_5);
        // D s_0_7: cast reint s_0_6 -> u32
        let s_0_7: u32 = (s_0_6.value() as u32);
        // C s_0_8: const #0s : i
        let s_0_8: i128 = 0;
        // D s_0_9: cast zx s_0_7 -> bv
        let s_0_9: Bits = Bits::new(s_0_7 as u128, 32u16);
        // C s_0_10: const #1s : i64
        let s_0_10: i64 = 1;
        // C s_0_11: cast zx s_0_10 -> i
        let s_0_11: i128 = (i128::try_from(s_0_10).unwrap());
        // C s_0_12: const #7s : i
        let s_0_12: i128 = 7;
        // C s_0_13: add s_0_12 s_0_11
        let s_0_13: i128 = (s_0_12 + s_0_11);
        // D s_0_14: bit-extract s_0_9 s_0_8 s_0_13
        let s_0_14: Bits = (Bits::new(
            ((s_0_9) >> (s_0_8)).value(),
            u16::try_from(s_0_13).unwrap(),
        ));
        // D s_0_15: cast reint s_0_14 -> u8
        let s_0_15: u8 = (s_0_14.value() as u8);
        // C s_0_16: const #32s : i
        let s_0_16: i128 = 32;
        // D s_0_17: cast zx s_0_15 -> bv
        let s_0_17: Bits = Bits::new(s_0_15 as u128, 8u16);
        // D s_0_18: bits-cast zx s_0_17 -> bv length s_0_16
        let s_0_18: Bits = s_0_17.zero_extend(s_0_16);
        // D s_0_19: cast reint s_0_18 -> u32
        let s_0_19: u32 = (s_0_18.value() as u32);
        // D s_0_20: read-var d:i64
        let s_0_20: i64 = fn_state.d;
        // D s_0_21: cast zx s_0_20 -> i
        let s_0_21: i128 = (i128::try_from(s_0_20).unwrap());
        // D s_0_22: call R_set(s_0_21, s_0_19)
        let s_0_22: () = R_set(state, tracer, s_0_21, s_0_19);
        // N s_0_23: return
        return;
    }
}
