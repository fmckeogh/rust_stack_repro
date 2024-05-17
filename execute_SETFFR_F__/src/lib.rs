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
use Ones::*;
use CheckNonStreamingSVEEnabled::*;
use FFR_set::*;
use common::*;
pub fn execute_SETFFR_F__<T: Tracer>(state: &mut State, tracer: &T, VL: i64) -> () {
    #[derive(Default)]
    struct FunctionState {
        VL: i64,
    }
    let fn_state = FunctionState {
        VL,
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
        // S s_0_1: call CheckNonStreamingSVEEnabled(s_0_0)
        let s_0_1: () = CheckNonStreamingSVEEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VL:i64
        let s_1_0: i64 = fn_state.VL;
        // C s_1_1: const #8s : i
        let s_1_1: i128 = 8;
        // D s_1_2: cast zx s_1_0 -> i
        let s_1_2: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_3: div s_1_2 s_1_1
        let s_1_3: i128 = ((s_1_2) / (s_1_1));
        // D s_1_4: cast reint s_1_3 -> i64
        let s_1_4: i64 = (s_1_3 as i64);
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: cast zx s_1_4 -> i
        let s_1_7: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_8: call Ones(s_1_7)
        let s_1_8: Bits = Ones(state, tracer, s_1_7);
        // D s_1_9: cast zx s_1_6 -> i
        let s_1_9: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_10: call FFR_set(s_1_9, s_1_8)
        let s_1_10: () = FFR_set(state, tracer, s_1_9, s_1_8);
        // N s_1_11: return
        return;
    }
}
