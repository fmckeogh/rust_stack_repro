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
use X_read::*;
use u__id::*;
use CheckSVEEnabled::*;
use Z_read::*;
use subrange_subrange_concat::*;
use Z_set::*;
use common::*;
pub fn execute_INSR_Z_R__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    dn: i64,
    esize: i64,
    m: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        VLshadow_3325: i64,
        esizeshadow_3324: i64,
        VL: i64,
        dn: i64,
        esize: i64,
        m: i64,
    }
    let fn_state = FunctionState {
        VL,
        dn,
        esize,
        m,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var esize:i64
        let s_0_0: i64 = fn_state.esize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var esizeshadow#3324 <= s_0_2
        fn_state.esizeshadow_3324 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#3325 <= s_0_6
        fn_state.VLshadow_3325 = s_0_6;
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call CheckSVEEnabled(s_0_8)
        let s_0_9: () = CheckSVEEnabled(state, tracer, s_0_8);
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#3325:i64
        let s_1_0: i64 = fn_state.VLshadow_3325;
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
        // D s_1_7: read-var esizeshadow#3324:i64
        let s_1_7: i64 = fn_state.esizeshadow_3324;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast reint s_1_8 -> i64
        let s_1_9: i64 = (s_1_8 as i64);
        // D s_1_10: read-var m:i64
        let s_1_10: i64 = fn_state.m;
        // D s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_12: call X_read(s_1_11, s_1_9)
        let s_1_12: Bits = X_read(state, tracer, s_1_11, s_1_9);
        // D s_1_13: cast zx s_1_0 -> i
        let s_1_13: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_14: cast reint s_1_13 -> i64
        let s_1_14: i64 = (s_1_13 as i64);
        // D s_1_15: cast zx s_1_0 -> i
        let s_1_15: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_16: call __id(s_1_15)
        let s_1_16: i128 = u__id(state, tracer, s_1_15);
        // D s_1_17: cast reint s_1_16 -> i64
        let s_1_17: i64 = (s_1_16 as i64);
        // D s_1_18: cast zx s_1_0 -> i
        let s_1_18: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_19: read-var esizeshadow#3324:i64
        let s_1_19: i64 = fn_state.esizeshadow_3324;
        // D s_1_20: cast zx s_1_19 -> i
        let s_1_20: i128 = (i128::try_from(s_1_19).unwrap());
        // D s_1_21: sub s_1_18 s_1_20
        let s_1_21: i128 = ((s_1_18) - (s_1_20));
        // D s_1_22: cast reint s_1_21 -> i64
        let s_1_22: i64 = (s_1_21 as i64);
        // C s_1_23: const #1s : i
        let s_1_23: i128 = 1;
        // D s_1_24: cast zx s_1_22 -> i
        let s_1_24: i128 = (i128::try_from(s_1_22).unwrap());
        // D s_1_25: sub s_1_24 s_1_23
        let s_1_25: i128 = ((s_1_24) - (s_1_23));
        // D s_1_26: cast reint s_1_25 -> i64
        let s_1_26: i64 = (s_1_25 as i64);
        // D s_1_27: size-of s_1_12
        let s_1_27: u16 = s_1_12.length();
        // D s_1_28: cast zx s_1_27 -> i
        let s_1_28: i128 = (i128::try_from(s_1_27).unwrap());
        // D s_1_29: cast reint s_1_28 -> i64
        let s_1_29: i64 = (s_1_28 as i64);
        // C s_1_30: const #1s : i
        let s_1_30: i128 = 1;
        // D s_1_31: cast zx s_1_29 -> i
        let s_1_31: i128 = (i128::try_from(s_1_29).unwrap());
        // D s_1_32: sub s_1_31 s_1_30
        let s_1_32: i128 = ((s_1_31) - (s_1_30));
        // D s_1_33: cast reint s_1_32 -> i64
        let s_1_33: i64 = (s_1_32 as i64);
        // C s_1_34: const #0s : i
        let s_1_34: i128 = 0;
        // C s_1_35: const #0s : i
        let s_1_35: i128 = 0;
        // D s_1_36: cast zx s_1_17 -> i
        let s_1_36: i128 = (i128::try_from(s_1_17).unwrap());
        // D s_1_37: cast zx s_1_26 -> i
        let s_1_37: i128 = (i128::try_from(s_1_26).unwrap());
        // D s_1_38: cast zx s_1_33 -> i
        let s_1_38: i128 = (i128::try_from(s_1_33).unwrap());
        // D s_1_39: call subrange_subrange_concat(s_1_36, s_1_6, s_1_37, s_1_34, s_1_12, s_1_38, s_1_35)
        let s_1_39: Bits = subrange_subrange_concat(
            state,
            tracer,
            s_1_36,
            s_1_6,
            s_1_37,
            s_1_34,
            s_1_12,
            s_1_38,
            s_1_35,
        );
        // D s_1_40: read-var dn:i64
        let s_1_40: i64 = fn_state.dn;
        // D s_1_41: cast zx s_1_40 -> i
        let s_1_41: i128 = (i128::try_from(s_1_40).unwrap());
        // D s_1_42: cast zx s_1_14 -> i
        let s_1_42: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_43: call Z_set(s_1_41, s_1_42, s_1_39)
        let s_1_43: () = Z_set(state, tracer, s_1_41, s_1_42, s_1_39);
        // N s_1_44: return
        return;
    }
}
