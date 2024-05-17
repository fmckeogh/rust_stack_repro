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
use AESSubBytes::*;
use Q_read::*;
use Q_set::*;
use CheckCryptoEnabled32::*;
use AESShiftRows::*;
use common::*;
pub fn execute_aarch32_instrs_AESE_Op_A_txt<T: Tracer>(
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
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: call Q_read(s_1_3)
        let s_1_4: u128 = Q_read(state, tracer, s_1_3);
        // C s_1_5: const #1s : i64
        let s_1_5: i64 = 1;
        // D s_1_6: read-var m:i64
        let s_1_6: i64 = fn_state.m;
        // D s_1_7: lsr s_1_6 s_1_5
        let s_1_7: i64 = s_1_6 >> s_1_5;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: call Q_read(s_1_8)
        let s_1_9: u128 = Q_read(state, tracer, s_1_8);
        // C s_1_10: const #1s : i64
        let s_1_10: i64 = 1;
        // D s_1_11: read-var d:i64
        let s_1_11: i64 = fn_state.d;
        // D s_1_12: lsr s_1_11 s_1_10
        let s_1_12: i64 = s_1_11 >> s_1_10;
        // D s_1_13: cast zx s_1_4 -> bv
        let s_1_13: Bits = Bits::new(s_1_4 as u128, 128u16);
        // D s_1_14: cast zx s_1_9 -> bv
        let s_1_14: Bits = Bits::new(s_1_9 as u128, 128u16);
        // D s_1_15: xor s_1_13 s_1_14
        let s_1_15: Bits = ((s_1_13) ^ (s_1_14));
        // D s_1_16: cast reint s_1_15 -> u128
        let s_1_16: u128 = (s_1_15.value() as u128);
        // D s_1_17: call AESShiftRows(s_1_16)
        let s_1_17: u128 = AESShiftRows(state, tracer, s_1_16);
        // D s_1_18: call AESSubBytes(s_1_17)
        let s_1_18: u128 = AESSubBytes(state, tracer, s_1_17);
        // D s_1_19: cast zx s_1_12 -> i
        let s_1_19: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_20: call Q_set(s_1_19, s_1_18)
        let s_1_20: () = Q_set(state, tracer, s_1_19, s_1_18);
        // N s_1_21: return
        return;
    }
}
