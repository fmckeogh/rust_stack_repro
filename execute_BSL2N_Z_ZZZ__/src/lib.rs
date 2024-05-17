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
pub fn execute_BSL2N_Z_ZZZ__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    dn: i64,
    k: i64,
    m: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        VLshadow_4294: i64,
        VL: i64,
        dn: i64,
        k: i64,
        m: i64,
    }
    let fn_state = FunctionState {
        VL,
        dn,
        k,
        m,
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
        // D s_0_3: write-var VLshadow#4294 <= s_0_2
        fn_state.VLshadow_4294 = s_0_2;
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
        // D s_1_0: read-var VLshadow#4294:i64
        let s_1_0: i64 = fn_state.VLshadow_4294;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: cast reint s_1_1 -> i64
        let s_1_2: i64 = (s_1_1 as i64);
        // D s_1_3: read-var dn:i64
        let s_1_3: i64 = fn_state.dn;
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
        // D s_1_9: read-var m:i64
        let s_1_9: i64 = fn_state.m;
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_11: cast zx s_1_8 -> i
        let s_1_11: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_12: call Z_read(s_1_10, s_1_11)
        let s_1_12: Bits = Z_read(state, tracer, s_1_10, s_1_11);
        // D s_1_13: cast zx s_1_0 -> i
        let s_1_13: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_14: cast reint s_1_13 -> i64
        let s_1_14: i64 = (s_1_13 as i64);
        // D s_1_15: read-var k:i64
        let s_1_15: i64 = fn_state.k;
        // D s_1_16: cast zx s_1_15 -> i
        let s_1_16: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_17: cast zx s_1_14 -> i
        let s_1_17: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_18: call Z_read(s_1_16, s_1_17)
        let s_1_18: Bits = Z_read(state, tracer, s_1_16, s_1_17);
        // D s_1_19: cast zx s_1_0 -> i
        let s_1_19: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_20: cast reint s_1_19 -> i64
        let s_1_20: i64 = (s_1_19 as i64);
        // D s_1_21: and s_1_6 s_1_18
        let s_1_21: Bits = ((s_1_6) & (s_1_18));
        // D s_1_22: not s_1_12
        let s_1_22: Bits = !s_1_12;
        // D s_1_23: not s_1_18
        let s_1_23: Bits = !s_1_18;
        // D s_1_24: and s_1_22 s_1_23
        let s_1_24: Bits = ((s_1_22) & (s_1_23));
        // D s_1_25: or s_1_21 s_1_24
        let s_1_25: Bits = ((s_1_21) | (s_1_24));
        // D s_1_26: read-var dn:i64
        let s_1_26: i64 = fn_state.dn;
        // D s_1_27: cast zx s_1_26 -> i
        let s_1_27: i128 = (i128::try_from(s_1_26).unwrap());
        // D s_1_28: cast zx s_1_20 -> i
        let s_1_28: i128 = (i128::try_from(s_1_20).unwrap());
        // D s_1_29: call Z_set(s_1_27, s_1_28, s_1_25)
        let s_1_29: () = Z_set(state, tracer, s_1_27, s_1_28, s_1_25);
        // N s_1_30: return
        return;
    }
}
