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
use R_read::*;
use R_set::*;
use integer_subrange::*;
use common::*;
pub fn execute_aarch32_instrs_UMAAL_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    dHi: i64,
    dLo: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        dHi: i64,
        dLo: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        dHi,
        dLo,
        m,
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var n:i64
        let s_0_0: i64 = fn_state.n;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: call R_read(s_0_1)
        let s_0_2: u32 = R_read(state, tracer, s_0_1);
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 32u16);
        // D s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (s_0_3.value() as i128);
        // D s_0_5: cast reint s_0_4 -> i64
        let s_0_5: i64 = (s_0_4 as i64);
        // D s_0_6: read-var m:i64
        let s_0_6: i64 = fn_state.m;
        // D s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (i128::try_from(s_0_6).unwrap());
        // D s_0_8: call R_read(s_0_7)
        let s_0_8: u32 = R_read(state, tracer, s_0_7);
        // D s_0_9: cast zx s_0_8 -> bv
        let s_0_9: Bits = Bits::new(s_0_8 as u128, 32u16);
        // D s_0_10: cast zx s_0_9 -> i
        let s_0_10: i128 = (s_0_9.value() as i128);
        // D s_0_11: cast reint s_0_10 -> i64
        let s_0_11: i64 = (s_0_10 as i64);
        // D s_0_12: cast zx s_0_5 -> i
        let s_0_12: i128 = (i128::try_from(s_0_5).unwrap());
        // D s_0_13: cast zx s_0_11 -> i
        let s_0_13: i128 = (i128::try_from(s_0_11).unwrap());
        // D s_0_14: mul s_0_12 s_0_13
        let s_0_14: i128 = ((s_0_12) * (s_0_13));
        // D s_0_15: read-var dHi:i64
        let s_0_15: i64 = fn_state.dHi;
        // D s_0_16: cast zx s_0_15 -> i
        let s_0_16: i128 = (i128::try_from(s_0_15).unwrap());
        // D s_0_17: call R_read(s_0_16)
        let s_0_17: u32 = R_read(state, tracer, s_0_16);
        // D s_0_18: cast zx s_0_17 -> bv
        let s_0_18: Bits = Bits::new(s_0_17 as u128, 32u16);
        // D s_0_19: cast zx s_0_18 -> i
        let s_0_19: i128 = (s_0_18.value() as i128);
        // D s_0_20: cast reint s_0_19 -> i64
        let s_0_20: i64 = (s_0_19 as i64);
        // D s_0_21: cast zx s_0_20 -> i
        let s_0_21: i128 = (i128::try_from(s_0_20).unwrap());
        // D s_0_22: add s_0_14 s_0_21
        let s_0_22: i128 = (s_0_14 + s_0_21);
        // D s_0_23: read-var dLo:i64
        let s_0_23: i64 = fn_state.dLo;
        // D s_0_24: cast zx s_0_23 -> i
        let s_0_24: i128 = (i128::try_from(s_0_23).unwrap());
        // D s_0_25: call R_read(s_0_24)
        let s_0_25: u32 = R_read(state, tracer, s_0_24);
        // D s_0_26: cast zx s_0_25 -> bv
        let s_0_26: Bits = Bits::new(s_0_25 as u128, 32u16);
        // D s_0_27: cast zx s_0_26 -> i
        let s_0_27: i128 = (s_0_26.value() as i128);
        // D s_0_28: cast reint s_0_27 -> i64
        let s_0_28: i64 = (s_0_27 as i64);
        // D s_0_29: cast zx s_0_28 -> i
        let s_0_29: i128 = (i128::try_from(s_0_28).unwrap());
        // D s_0_30: add s_0_22 s_0_29
        let s_0_30: i128 = (s_0_22 + s_0_29);
        // C s_0_31: const #63s : i
        let s_0_31: i128 = 63;
        // C s_0_32: const #32s : i
        let s_0_32: i128 = 32;
        // D s_0_33: call integer_subrange(s_0_30, s_0_31, s_0_32)
        let s_0_33: Bits = integer_subrange(state, tracer, s_0_30, s_0_31, s_0_32);
        // D s_0_34: cast reint s_0_33 -> u32
        let s_0_34: u32 = (s_0_33.value() as u32);
        // D s_0_35: read-var dHi:i64
        let s_0_35: i64 = fn_state.dHi;
        // D s_0_36: cast zx s_0_35 -> i
        let s_0_36: i128 = (i128::try_from(s_0_35).unwrap());
        // D s_0_37: call R_set(s_0_36, s_0_34)
        let s_0_37: () = R_set(state, tracer, s_0_36, s_0_34);
        // C s_0_38: const #31s : i
        let s_0_38: i128 = 31;
        // C s_0_39: const #0s : i
        let s_0_39: i128 = 0;
        // D s_0_40: call integer_subrange(s_0_30, s_0_38, s_0_39)
        let s_0_40: Bits = integer_subrange(state, tracer, s_0_30, s_0_38, s_0_39);
        // D s_0_41: cast reint s_0_40 -> u32
        let s_0_41: u32 = (s_0_40.value() as u32);
        // D s_0_42: read-var dLo:i64
        let s_0_42: i64 = fn_state.dLo;
        // D s_0_43: cast zx s_0_42 -> i
        let s_0_43: i128 = (i128::try_from(s_0_42).unwrap());
        // D s_0_44: call R_set(s_0_43, s_0_41)
        let s_0_44: () = R_set(state, tracer, s_0_43, s_0_41);
        // N s_0_45: return
        return;
    }
}
