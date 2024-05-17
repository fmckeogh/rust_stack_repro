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
use AESInvMixColumns::*;
use CheckCryptoEnabled32::*;
use Q_set::*;
use common::*;
pub fn execute_aarch32_instrs_AESIMC_Op_A_txt<T: Tracer>(
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
        // D s_1_8: call AESInvMixColumns(s_1_7)
        let s_1_8: u128 = AESInvMixColumns(state, tracer, s_1_7);
        // D s_1_9: cast zx s_1_2 -> i
        let s_1_9: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_10: call Q_set(s_1_9, s_1_8)
        let s_1_10: () = Q_set(state, tracer, s_1_9, s_1_8);
        // N s_1_11: return
        return;
    }
}
