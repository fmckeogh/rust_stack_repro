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
pub fn execute_aarch32_instrs_UXTAB_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
    n: i64,
    rotation: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        d: i64,
        m: i64,
        n: i64,
        rotation: i64,
    }
    let fn_state = FunctionState {
        d,
        m,
        n,
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
        // D s_0_8: read-var n:i64
        let s_0_8: i64 = fn_state.n;
        // D s_0_9: cast zx s_0_8 -> i
        let s_0_9: i128 = (i128::try_from(s_0_8).unwrap());
        // D s_0_10: call R_read(s_0_9)
        let s_0_10: u32 = R_read(state, tracer, s_0_9);
        // C s_0_11: const #0s : i
        let s_0_11: i128 = 0;
        // D s_0_12: cast zx s_0_7 -> bv
        let s_0_12: Bits = Bits::new(s_0_7 as u128, 32u16);
        // C s_0_13: const #1s : i64
        let s_0_13: i64 = 1;
        // C s_0_14: cast zx s_0_13 -> i
        let s_0_14: i128 = (i128::try_from(s_0_13).unwrap());
        // C s_0_15: const #7s : i
        let s_0_15: i128 = 7;
        // C s_0_16: add s_0_15 s_0_14
        let s_0_16: i128 = (s_0_15 + s_0_14);
        // D s_0_17: bit-extract s_0_12 s_0_11 s_0_16
        let s_0_17: Bits = (Bits::new(
            ((s_0_12) >> (s_0_11)).value(),
            u16::try_from(s_0_16).unwrap(),
        ));
        // D s_0_18: cast reint s_0_17 -> u8
        let s_0_18: u8 = (s_0_17.value() as u8);
        // C s_0_19: const #32s : i
        let s_0_19: i128 = 32;
        // D s_0_20: cast zx s_0_18 -> bv
        let s_0_20: Bits = Bits::new(s_0_18 as u128, 8u16);
        // D s_0_21: bits-cast zx s_0_20 -> bv length s_0_19
        let s_0_21: Bits = s_0_20.zero_extend(s_0_19);
        // D s_0_22: cast reint s_0_21 -> u32
        let s_0_22: u32 = (s_0_21.value() as u32);
        // D s_0_23: cast zx s_0_10 -> bv
        let s_0_23: Bits = Bits::new(s_0_10 as u128, 32u16);
        // D s_0_24: cast zx s_0_22 -> bv
        let s_0_24: Bits = Bits::new(s_0_22 as u128, 32u16);
        // D s_0_25: add s_0_23 s_0_24
        let s_0_25: Bits = (s_0_23 + s_0_24);
        // D s_0_26: cast reint s_0_25 -> u32
        let s_0_26: u32 = (s_0_25.value() as u32);
        // D s_0_27: read-var d:i64
        let s_0_27: i64 = fn_state.d;
        // D s_0_28: cast zx s_0_27 -> i
        let s_0_28: i128 = (i128::try_from(s_0_27).unwrap());
        // D s_0_29: call R_set(s_0_28, s_0_26)
        let s_0_29: () = R_set(state, tracer, s_0_28, s_0_26);
        // N s_0_30: return
        return;
    }
}
