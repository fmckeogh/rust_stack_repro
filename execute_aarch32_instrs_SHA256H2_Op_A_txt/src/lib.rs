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
use SHA256hash::*;
use CheckCryptoEnabled32::*;
use Q_set::*;
use common::*;
pub fn execute_aarch32_instrs_SHA256H2_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        d: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        d,
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
        // D s_1_1: read-var n:i64
        let s_1_1: i64 = fn_state.n;
        // D s_1_2: lsr s_1_1 s_1_0
        let s_1_2: i64 = s_1_1 >> s_1_0;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: call Q_read(s_1_3)
        let s_1_4: u128 = Q_read(state, tracer, s_1_3);
        // C s_1_5: const #1s : i64
        let s_1_5: i64 = 1;
        // D s_1_6: read-var d:i64
        let s_1_6: i64 = fn_state.d;
        // D s_1_7: lsr s_1_6 s_1_5
        let s_1_7: i64 = s_1_6 >> s_1_5;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: call Q_read(s_1_8)
        let s_1_9: u128 = Q_read(state, tracer, s_1_8);
        // C s_1_10: const #1s : i64
        let s_1_10: i64 = 1;
        // D s_1_11: read-var m:i64
        let s_1_11: i64 = fn_state.m;
        // D s_1_12: lsr s_1_11 s_1_10
        let s_1_12: i64 = s_1_11 >> s_1_10;
        // D s_1_13: cast zx s_1_12 -> i
        let s_1_13: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_14: call Q_read(s_1_13)
        let s_1_14: u128 = Q_read(state, tracer, s_1_13);
        // C s_1_15: const #0u : u8
        let s_1_15: bool = false;
        // C s_1_16: const #1s : i64
        let s_1_16: i64 = 1;
        // D s_1_17: read-var d:i64
        let s_1_17: i64 = fn_state.d;
        // D s_1_18: lsr s_1_17 s_1_16
        let s_1_18: i64 = s_1_17 >> s_1_16;
        // D s_1_19: call SHA256hash(s_1_4, s_1_9, s_1_14, s_1_15)
        let s_1_19: u128 = SHA256hash(state, tracer, s_1_4, s_1_9, s_1_14, s_1_15);
        // D s_1_20: cast zx s_1_18 -> i
        let s_1_20: i128 = (i128::try_from(s_1_18).unwrap());
        // D s_1_21: call Q_set(s_1_20, s_1_19)
        let s_1_21: () = Q_set(state, tracer, s_1_20, s_1_19);
        // N s_1_22: return
        return;
    }
}
