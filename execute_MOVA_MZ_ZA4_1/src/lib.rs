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
use Z_set::*;
use X_read::*;
use CheckStreamingSVEAndZAEnabled::*;
use ZAvector_read::*;
use common::*;
pub fn execute_MOVA_MZ_ZA4_1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    nreg: i64,
    offset: i64,
    v: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        VLshadow_5592: i64,
        VLshadow_5591: i64,
        gs_259159: i64,
        vec: i128,
        vstride: i64,
        VL: i64,
        d: i64,
        nreg: i64,
        offset: i64,
        v: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
        nreg,
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
        // D s_0_3: write-var VLshadow#5591 <= s_0_2
        fn_state.VLshadow_5591 = s_0_2;
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
        // D s_1_0: read-var VLshadow#5591:i64
        let s_1_0: i64 = fn_state.VLshadow_5591;
        // D s_1_1: write-var VLshadow#5592 <= s_1_0
        fn_state.VLshadow_5592 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#5592:i64
        let s_1_3: i64 = fn_state.VLshadow_5592;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_8: read-var nreg:i64
        let s_1_8: i64 = fn_state.nreg;
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
        // C s_1_31: const #0s : i64
        let s_1_31: i64 = 0;
        // C s_1_32: const #1s : i
        let s_1_32: i128 = 1;
        // D s_1_33: read-var nreg:i64
        let s_1_33: i64 = fn_state.nreg;
        // D s_1_34: cast zx s_1_33 -> i
        let s_1_34: i128 = (i128::try_from(s_1_33).unwrap());
        // D s_1_35: sub s_1_34 s_1_32
        let s_1_35: i128 = ((s_1_34) - (s_1_32));
        // D s_1_36: cast reint s_1_35 -> i64
        let s_1_36: i64 = (s_1_35 as i64);
        // D s_1_37: write-var gs#259159 <= s_1_36
        fn_state.gs_259159 = s_1_36;
        // D s_1_38: write-var r <= s_1_31
        fn_state.r = s_1_31;
        // N s_1_39: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var r:i64
        let s_2_0: i64 = fn_state.r;
        // D s_2_1: read-var gs#259159:i64
        let s_2_1: i64 = fn_state.gs_259159;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b4 b3
        if s_2_2 {
            return block_4(state, tracer, fn_state);
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
        // D s_3_1: read-var VLshadow#5592:i64
        let s_3_1: i64 = fn_state.VLshadow_5592;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: call ZAvector_read(s_3_0, s_3_4)
        let s_3_5: Bits = ZAvector_read(state, tracer, s_3_0, s_3_4);
        // D s_3_6: read-var d:i64
        let s_3_6: i64 = fn_state.d;
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_8: read-var r:i64
        let s_3_8: i64 = fn_state.r;
        // D s_3_9: cast zx s_3_8 -> i
        let s_3_9: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_10: add s_3_7 s_3_9
        let s_3_10: i128 = (s_3_7 + s_3_9);
        // D s_3_11: cast reint s_3_10 -> i64
        let s_3_11: i64 = (s_3_10 as i64);
        // D s_3_12: read-var VLshadow#5592:i64
        let s_3_12: i64 = fn_state.VLshadow_5592;
        // D s_3_13: cast zx s_3_12 -> i
        let s_3_13: i128 = (i128::try_from(s_3_12).unwrap());
        // D s_3_14: cast reint s_3_13 -> i64
        let s_3_14: i64 = (s_3_13 as i64);
        // D s_3_15: cast zx s_3_11 -> i
        let s_3_15: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_16: cast zx s_3_14 -> i
        let s_3_16: i128 = (i128::try_from(s_3_14).unwrap());
        // D s_3_17: call Z_set(s_3_15, s_3_16, s_3_5)
        let s_3_17: () = Z_set(state, tracer, s_3_15, s_3_16, s_3_5);
        // D s_3_18: read-var vstride:i64
        let s_3_18: i64 = fn_state.vstride;
        // D s_3_19: cast zx s_3_18 -> i
        let s_3_19: i128 = (i128::try_from(s_3_18).unwrap());
        // D s_3_20: read-var vec:i
        let s_3_20: i128 = fn_state.vec;
        // D s_3_21: add s_3_20 s_3_19
        let s_3_21: i128 = (s_3_20 + s_3_19);
        // D s_3_22: write-var vec <= s_3_21
        fn_state.vec = s_3_21;
        // D s_3_23: read-var r:i64
        let s_3_23: i64 = fn_state.r;
        // C s_3_24: const #1s : i64
        let s_3_24: i64 = 1;
        // D s_3_25: add s_3_23 s_3_24
        let s_3_25: i64 = (s_3_23 + s_3_24);
        // D s_3_26: write-var r <= s_3_25
        fn_state.r = s_3_25;
        // N s_3_27: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: return
        return;
    }
}
