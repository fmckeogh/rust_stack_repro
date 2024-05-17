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
use HaveSVE2p1::*;
use CheckStreamingSVEEnabled::*;
use P_set::*;
use EncodePredCount::*;
use CheckSVEEnabled::*;
use common::*;
pub fn execute_PTRUE_PN_I__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        VL: i64,
        d: i64,
        esize: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
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
        // S s_0_1: call HaveSVE2p1(s_0_0)
        let s_0_1: bool = HaveSVE2p1(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b3 b1
        if s_0_1 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call CheckStreamingSVEEnabled(s_1_0)
        let s_1_1: () = CheckStreamingSVEEnabled(state, tracer, s_1_0);
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var VL:i64
        let s_2_0: i64 = fn_state.VL;
        // C s_2_1: const #8s : i
        let s_2_1: i128 = 8;
        // D s_2_2: cast zx s_2_0 -> i
        let s_2_2: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_3: div s_2_2 s_2_1
        let s_2_3: i128 = ((s_2_2) / (s_2_1));
        // D s_2_4: cast reint s_2_3 -> i64
        let s_2_4: i64 = (s_2_3 as i64);
        // D s_2_5: cast zx s_2_0 -> i
        let s_2_5: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_6: read-var esize:i64
        let s_2_6: i64 = fn_state.esize;
        // D s_2_7: cast zx s_2_6 -> i
        let s_2_7: i128 = (i128::try_from(s_2_6).unwrap());
        // D s_2_8: div s_2_5 s_2_7
        let s_2_8: i128 = ((s_2_5) / (s_2_7));
        // D s_2_9: cast reint s_2_8 -> i64
        let s_2_9: i64 = (s_2_8 as i64);
        // D s_2_10: cast zx s_2_4 -> i
        let s_2_10: i128 = (i128::try_from(s_2_4).unwrap());
        // D s_2_11: cast reint s_2_10 -> i64
        let s_2_11: i64 = (s_2_10 as i64);
        // D s_2_12: read-var esize:i64
        let s_2_12: i64 = fn_state.esize;
        // D s_2_13: cast zx s_2_12 -> i
        let s_2_13: i128 = (i128::try_from(s_2_12).unwrap());
        // D s_2_14: cast zx s_2_9 -> i
        let s_2_14: i128 = (i128::try_from(s_2_9).unwrap());
        // D s_2_15: cast zx s_2_9 -> i
        let s_2_15: i128 = (i128::try_from(s_2_9).unwrap());
        // D s_2_16: cast zx s_2_11 -> i
        let s_2_16: i128 = (i128::try_from(s_2_11).unwrap());
        // C s_2_17: const #0u : u8
        let s_2_17: bool = false;
        // D s_2_18: call EncodePredCount(s_2_13, s_2_14, s_2_15, s_2_17, s_2_16)
        let s_2_18: Bits = EncodePredCount(
            state,
            tracer,
            s_2_13,
            s_2_14,
            s_2_15,
            s_2_17,
            s_2_16,
        );
        // D s_2_19: cast zx s_2_4 -> i
        let s_2_19: i128 = (i128::try_from(s_2_4).unwrap());
        // D s_2_20: cast reint s_2_19 -> i64
        let s_2_20: i64 = (s_2_19 as i64);
        // D s_2_21: read-var d:i64
        let s_2_21: i64 = fn_state.d;
        // D s_2_22: cast zx s_2_21 -> i
        let s_2_22: i128 = (i128::try_from(s_2_21).unwrap());
        // D s_2_23: cast zx s_2_20 -> i
        let s_2_23: i128 = (i128::try_from(s_2_20).unwrap());
        // D s_2_24: call P_set(s_2_22, s_2_23, s_2_18)
        let s_2_24: () = P_set(state, tracer, s_2_22, s_2_23, s_2_18);
        // N s_2_25: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call CheckSVEEnabled(s_3_0)
        let s_3_1: () = CheckSVEEnabled(state, tracer, s_3_0);
        // N s_3_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
