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
use S_read::*;
use CheckVFPEnabled::*;
use S_set::*;
use common::*;
pub fn execute_aarch32_instrs_VINS_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        d: i64,
        m: i64,
    }
    let fn_state = FunctionState {
        d,
        m,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #1u : u8
        let s_0_0: bool = true;
        // S s_0_1: call CheckVFPEnabled(s_0_0)
        let s_0_1: () = CheckVFPEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var m:i64
        let s_1_0: i64 = fn_state.m;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: call S_read(s_1_1)
        let s_1_2: u32 = S_read(state, tracer, s_1_1);
        // C s_1_3: const #0s : i
        let s_1_3: i128 = 0;
        // D s_1_4: cast zx s_1_2 -> bv
        let s_1_4: Bits = Bits::new(s_1_2 as u128, 32u16);
        // C s_1_5: const #1s : i64
        let s_1_5: i64 = 1;
        // C s_1_6: cast zx s_1_5 -> i
        let s_1_6: i128 = (i128::try_from(s_1_5).unwrap());
        // C s_1_7: const #15s : i
        let s_1_7: i128 = 15;
        // C s_1_8: add s_1_7 s_1_6
        let s_1_8: i128 = (s_1_7 + s_1_6);
        // D s_1_9: bit-extract s_1_4 s_1_3 s_1_8
        let s_1_9: Bits = (Bits::new(
            ((s_1_4) >> (s_1_3)).value(),
            u16::try_from(s_1_8).unwrap(),
        ));
        // D s_1_10: cast reint s_1_9 -> u16
        let s_1_10: u16 = (s_1_9.value() as u16);
        // D s_1_11: read-var d:i64
        let s_1_11: i64 = fn_state.d;
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_13: call S_read(s_1_12)
        let s_1_13: u32 = S_read(state, tracer, s_1_12);
        // C s_1_14: const #0s : i
        let s_1_14: i128 = 0;
        // D s_1_15: cast zx s_1_13 -> bv
        let s_1_15: Bits = Bits::new(s_1_13 as u128, 32u16);
        // C s_1_16: const #1s : i64
        let s_1_16: i64 = 1;
        // C s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // C s_1_18: const #15s : i
        let s_1_18: i128 = 15;
        // C s_1_19: add s_1_18 s_1_17
        let s_1_19: i128 = (s_1_18 + s_1_17);
        // D s_1_20: bit-extract s_1_15 s_1_14 s_1_19
        let s_1_20: Bits = (Bits::new(
            ((s_1_15) >> (s_1_14)).value(),
            u16::try_from(s_1_19).unwrap(),
        ));
        // D s_1_21: cast reint s_1_20 -> u16
        let s_1_21: u16 = (s_1_20.value() as u16);
        // D s_1_22: cast zx s_1_10 -> bv
        let s_1_22: Bits = Bits::new(s_1_10 as u128, 16u16);
        // D s_1_23: cast zx s_1_21 -> bv
        let s_1_23: Bits = Bits::new(s_1_21 as u128, 16u16);
        // D s_1_24: cast reint s_1_22 -> u128
        let s_1_24: u128 = (s_1_22.value() as u128);
        // D s_1_25: size-of s_1_22
        let s_1_25: u16 = s_1_22.length();
        // D s_1_26: cast reint s_1_23 -> u128
        let s_1_26: u128 = (s_1_23.value() as u128);
        // D s_1_27: size-of s_1_23
        let s_1_27: u16 = s_1_23.length();
        // D s_1_28: lsl s_1_24 s_1_27
        let s_1_28: u128 = s_1_24 << s_1_27;
        // D s_1_29: or s_1_28 s_1_26
        let s_1_29: u128 = ((s_1_28) | (s_1_26));
        // D s_1_30: add s_1_25 s_1_27
        let s_1_30: u16 = (s_1_25 + s_1_27);
        // D s_1_31: create-bits s_1_29 s_1_30
        let s_1_31: Bits = Bits::new(s_1_29, s_1_30);
        // D s_1_32: cast reint s_1_31 -> u32
        let s_1_32: u32 = (s_1_31.value() as u32);
        // D s_1_33: read-var d:i64
        let s_1_33: i64 = fn_state.d;
        // D s_1_34: cast zx s_1_33 -> i
        let s_1_34: i128 = (i128::try_from(s_1_33).unwrap());
        // D s_1_35: call S_set(s_1_34, s_1_32)
        let s_1_35: () = S_set(state, tracer, s_1_34, s_1_32);
        // N s_1_36: return
        return;
    }
}
