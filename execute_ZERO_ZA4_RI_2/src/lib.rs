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
use fmod_int::*;
use X_read::*;
use Zeros::*;
use CheckStreamingSVEAndZAEnabled::*;
use ZAvector_set::*;
use common::*;
pub fn execute_ZERO_ZA4_RI_2<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    ngrp: i64,
    nvec: i64,
    offset: i64,
    v: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        gs_290431: i64,
        VLshadow_6874: i64,
        vecshadow_6876: i128,
        gs_290424: i64,
        i: i64,
        vec: i128,
        VLshadow_6875: i64,
        vstride: i64,
        VL: i64,
        ngrp: i64,
        nvec: i64,
        offset: i64,
        v: i64,
    }
    let fn_state = FunctionState {
        VL,
        ngrp,
        nvec,
        offset,
        v,
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
        // D s_0_3: write-var VLshadow#6874 <= s_0_2
        fn_state.VLshadow_6874 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckStreamingSVEAndZAEnabled(s_0_4)
        let s_0_5: () = CheckStreamingSVEAndZAEnabled(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#6874:i64
        let s_1_0: i64 = fn_state.VLshadow_6874;
        // D s_1_1: write-var VLshadow#6875 <= s_1_0
        fn_state.VLshadow_6875 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#6875:i64
        let s_1_3: i64 = fn_state.VLshadow_6875;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_8: read-var ngrp:i64
        let s_1_8: i64 = fn_state.ngrp;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: div s_1_7 s_1_9
        let s_1_10: i128 = ((s_1_7) / (s_1_9));
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: write-var vstride <= s_1_11
        fn_state.vstride = s_1_11;
        // C s_1_13: const #32s : i64
        let s_1_13: i64 = 32;
        // D s_1_14: read-var v:i64
        let s_1_14: i64 = fn_state.v;
        // D s_1_15: cast zx s_1_14 -> i
        let s_1_15: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_16: call X_read(s_1_15, s_1_13)
        let s_1_16: Bits = X_read(state, tracer, s_1_15, s_1_13);
        // D s_1_17: cast reint s_1_16 -> u32
        let s_1_17: u32 = (s_1_16.value() as u32);
        // D s_1_18: cast zx s_1_17 -> bv
        let s_1_18: Bits = Bits::new(s_1_17 as u128, 32u16);
        // D s_1_19: cast zx s_1_18 -> i
        let s_1_19: i128 = (s_1_18.value() as i128);
        // D s_1_20: cast reint s_1_19 -> i64
        let s_1_20: i64 = (s_1_19 as i64);
        // D s_1_21: cast zx s_1_20 -> i
        let s_1_21: i128 = (i128::try_from(s_1_20).unwrap());
        // D s_1_22: read-var offset:i64
        let s_1_22: i64 = fn_state.offset;
        // D s_1_23: cast zx s_1_22 -> i
        let s_1_23: i128 = (i128::try_from(s_1_22).unwrap());
        // D s_1_24: add s_1_21 s_1_23
        let s_1_24: i128 = (s_1_21 + s_1_23);
        // D s_1_25: cast reint s_1_24 -> i64
        let s_1_25: i64 = (s_1_24 as i64);
        // D s_1_26: cast zx s_1_25 -> i
        let s_1_26: i128 = (i128::try_from(s_1_25).unwrap());
        // D s_1_27: read-var vstride:i64
        let s_1_27: i64 = fn_state.vstride;
        // D s_1_28: cast zx s_1_27 -> i
        let s_1_28: i128 = (i128::try_from(s_1_27).unwrap());
        // D s_1_29: mod s_1_26 s_1_28
        let s_1_29: i128 = ((s_1_26) % (s_1_28));
        // D s_1_30: write-var vec <= s_1_29
        fn_state.vec = s_1_29;
        // D s_1_31: read-var nvec:i64
        let s_1_31: i64 = fn_state.nvec;
        // D s_1_32: cast zx s_1_31 -> i
        let s_1_32: i128 = (i128::try_from(s_1_31).unwrap());
        // D s_1_33: read-var vec:i
        let s_1_33: i128 = fn_state.vec;
        // D s_1_34: call fmod_int(s_1_33, s_1_32)
        let s_1_34: i128 = fmod_int(state, tracer, s_1_33, s_1_32);
        // D s_1_35: read-var vec:i
        let s_1_35: i128 = fn_state.vec;
        // D s_1_36: sub s_1_35 s_1_34
        let s_1_36: i128 = ((s_1_35) - (s_1_34));
        // D s_1_37: write-var vec <= s_1_36
        fn_state.vec = s_1_36;
        // C s_1_38: const #0s : i64
        let s_1_38: i64 = 0;
        // C s_1_39: const #1s : i
        let s_1_39: i128 = 1;
        // D s_1_40: read-var ngrp:i64
        let s_1_40: i64 = fn_state.ngrp;
        // D s_1_41: cast zx s_1_40 -> i
        let s_1_41: i128 = (i128::try_from(s_1_40).unwrap());
        // D s_1_42: sub s_1_41 s_1_39
        let s_1_42: i128 = ((s_1_41) - (s_1_39));
        // D s_1_43: cast reint s_1_42 -> i64
        let s_1_43: i64 = (s_1_42 as i64);
        // D s_1_44: write-var gs#290424 <= s_1_43
        fn_state.gs_290424 = s_1_43;
        // D s_1_45: write-var r <= s_1_38
        fn_state.r = s_1_38;
        // N s_1_46: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var r:i64
        let s_2_0: i64 = fn_state.r;
        // D s_2_1: read-var gs#290424:i64
        let s_2_1: i64 = fn_state.gs_290424;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b7 b3
        if s_2_2 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var vec:i
        let s_3_0: i128 = fn_state.vec;
        // D s_3_1: write-var vecshadow#6876 <= s_3_0
        fn_state.vecshadow_6876 = s_3_0;
        // C s_3_2: const #0s : i64
        let s_3_2: i64 = 0;
        // C s_3_3: const #1s : i
        let s_3_3: i128 = 1;
        // D s_3_4: read-var nvec:i64
        let s_3_4: i64 = fn_state.nvec;
        // D s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_6: sub s_3_5 s_3_3
        let s_3_6: i128 = ((s_3_5) - (s_3_3));
        // D s_3_7: cast reint s_3_6 -> i64
        let s_3_7: i64 = (s_3_6 as i64);
        // D s_3_8: write-var gs#290431 <= s_3_7
        fn_state.gs_290431 = s_3_7;
        // D s_3_9: write-var i <= s_3_2
        fn_state.i = s_3_2;
        // N s_3_10: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var i:i64
        let s_4_0: i64 = fn_state.i;
        // D s_4_1: read-var gs#290431:i64
        let s_4_1: i64 = fn_state.gs_290431;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b6 b5
        if s_4_2 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var i:i64
        let s_5_0: i64 = fn_state.i;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: read-var vecshadow#6876:i
        let s_5_2: i128 = fn_state.vecshadow_6876;
        // D s_5_3: add s_5_2 s_5_1
        let s_5_3: i128 = (s_5_2 + s_5_1);
        // D s_5_4: read-var VLshadow#6875:i64
        let s_5_4: i64 = fn_state.VLshadow_6875;
        // D s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_6: cast reint s_5_5 -> i64
        let s_5_6: i64 = (s_5_5 as i64);
        // D s_5_7: read-var VLshadow#6875:i64
        let s_5_7: i64 = fn_state.VLshadow_6875;
        // D s_5_8: cast zx s_5_7 -> i
        let s_5_8: i128 = (i128::try_from(s_5_7).unwrap());
        // D s_5_9: call Zeros(s_5_8)
        let s_5_9: Bits = Zeros(state, tracer, s_5_8);
        // D s_5_10: cast zx s_5_6 -> i
        let s_5_10: i128 = (i128::try_from(s_5_6).unwrap());
        // D s_5_11: call ZAvector_set(s_5_3, s_5_10, s_5_9)
        let s_5_11: () = ZAvector_set(state, tracer, s_5_3, s_5_10, s_5_9);
        // D s_5_12: read-var i:i64
        let s_5_12: i64 = fn_state.i;
        // C s_5_13: const #1s : i64
        let s_5_13: i64 = 1;
        // D s_5_14: add s_5_12 s_5_13
        let s_5_14: i64 = (s_5_12 + s_5_13);
        // D s_5_15: write-var i <= s_5_14
        fn_state.i = s_5_14;
        // N s_5_16: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var vstride:i64
        let s_6_0: i64 = fn_state.vstride;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: read-var vec:i
        let s_6_2: i128 = fn_state.vec;
        // D s_6_3: add s_6_2 s_6_1
        let s_6_3: i128 = (s_6_2 + s_6_1);
        // D s_6_4: write-var vec <= s_6_3
        fn_state.vec = s_6_3;
        // D s_6_5: read-var r:i64
        let s_6_5: i64 = fn_state.r;
        // C s_6_6: const #1s : i64
        let s_6_6: i64 = 1;
        // D s_6_7: add s_6_5 s_6_6
        let s_6_7: i64 = (s_6_5 + s_6_6);
        // D s_6_8: write-var r <= s_6_7
        fn_state.r = s_6_7;
        // N s_6_9: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: return
        return;
    }
}
