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
use CheckSVEEnabled::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_MOVPRFX_Z_Z__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        VLshadow_2879: i64,
        VL: i64,
        d: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var VL:i64
        let s_0_0: i64 = fn_state.VL;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var VLshadow#2879 <= s_0_2
        fn_state.VLshadow_2879 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckSVEEnabled(s_0_4)
        let s_0_5: () = CheckSVEEnabled(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#2879:i64
        let s_1_0: i64 = fn_state.VLshadow_2879;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: cast reint s_1_1 -> i64
        let s_1_2: i64 = (s_1_1 as i64);
        // D s_1_3: read-var n:i64
        let s_1_3: i64 = fn_state.n;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: cast zx s_1_2 -> i
        let s_1_5: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_6: call Z_read(s_1_4, s_1_5)
        let s_1_6: Bits = Z_read(state, tracer, s_1_4, s_1_5);
        // D s_1_7: cast zx s_1_0 -> i
        let s_1_7: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_8: cast reint s_1_7 -> i64
        let s_1_8: i64 = (s_1_7 as i64);
        // D s_1_9: read-var d:i64
        let s_1_9: i64 = fn_state.d;
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_11: cast zx s_1_8 -> i
        let s_1_11: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_12: call Z_set(s_1_10, s_1_11, s_1_6)
        let s_1_12: () = Z_set(state, tracer, s_1_10, s_1_11, s_1_6);
        // N s_1_13: return
        return;
    }
}
