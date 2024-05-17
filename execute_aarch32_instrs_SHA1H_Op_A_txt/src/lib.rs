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
use Q_read::*;
use ROL::*;
use CheckCryptoEnabled32::*;
use Q_set::*;
use common::*;
pub fn execute_aarch32_instrs_SHA1H_Op_A_txt<T: Tracer>(
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
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call CheckCryptoEnabled32(s_0_0)
        let s_0_1: () = CheckCryptoEnabled32(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #1s : i64
        let s_1_0: i64 = 1;
        // D s_1_1: read-var d:i64
        let s_1_1: i64 = fn_state.d;
        // D s_1_2: lsr s_1_1 s_1_0
        let s_1_2: i64 = s_1_1 >> s_1_0;
        // C s_1_3: const #1s : i64
        let s_1_3: i64 = 1;
        // D s_1_4: read-var m:i64
        let s_1_4: i64 = fn_state.m;
        // D s_1_5: lsr s_1_4 s_1_3
        let s_1_5: i64 = s_1_4 >> s_1_3;
        // D s_1_6: cast zx s_1_5 -> i
        let s_1_6: i128 = (i128::try_from(s_1_5).unwrap());
        // D s_1_7: call Q_read(s_1_6)
        let s_1_7: u128 = Q_read(state, tracer, s_1_6);
        // C s_1_8: const #0s : i
        let s_1_8: i128 = 0;
        // D s_1_9: cast zx s_1_7 -> bv
        let s_1_9: Bits = Bits::new(s_1_7 as u128, 128u16);
        // C s_1_10: const #1s : i64
        let s_1_10: i64 = 1;
        // C s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // C s_1_12: const #31s : i
        let s_1_12: i128 = 31;
        // C s_1_13: add s_1_12 s_1_11
        let s_1_13: i128 = (s_1_12 + s_1_11);
        // D s_1_14: bit-extract s_1_9 s_1_8 s_1_13
        let s_1_14: Bits = (Bits::new(
            ((s_1_9) >> (s_1_8)).value(),
            u16::try_from(s_1_13).unwrap(),
        ));
        // D s_1_15: cast reint s_1_14 -> u32
        let s_1_15: u32 = (s_1_14.value() as u32);
        // C s_1_16: const #30s : i
        let s_1_16: i128 = 30;
        // D s_1_17: cast zx s_1_15 -> bv
        let s_1_17: Bits = Bits::new(s_1_15 as u128, 32u16);
        // D s_1_18: call ROL(s_1_17, s_1_16)
        let s_1_18: Bits = ROL(state, tracer, s_1_17, s_1_16);
        // D s_1_19: cast reint s_1_18 -> u32
        let s_1_19: u32 = (s_1_18.value() as u32);
        // C s_1_20: const #128s : i
        let s_1_20: i128 = 128;
        // D s_1_21: cast zx s_1_19 -> bv
        let s_1_21: Bits = Bits::new(s_1_19 as u128, 32u16);
        // D s_1_22: bits-cast zx s_1_21 -> bv length s_1_20
        let s_1_22: Bits = s_1_21.zero_extend(s_1_20);
        // D s_1_23: cast reint s_1_22 -> u128
        let s_1_23: u128 = (s_1_22.value() as u128);
        // D s_1_24: cast zx s_1_2 -> i
        let s_1_24: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_25: call Q_set(s_1_24, s_1_23)
        let s_1_25: () = Q_set(state, tracer, s_1_24, s_1_23);
        // N s_1_26: return
        return;
    }
}
