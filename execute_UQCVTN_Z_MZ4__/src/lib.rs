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
use CheckStreamingSVEEnabled::*;
use Elem_read::*;
use UnsignedSat::*;
use Z_read::*;
use Elem_set::*;
use Z_set::*;
use common::*;
pub fn execute_UQCVTN_Z_MZ4__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        gs_287704: i64,
        VLshadow_6775: i64,
        esizeshadow_6774: i64,
        result: Bits,
        VLshadow_6776: i64,
        i: i64,
        VL: i64,
        d: i64,
        esize: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        n,
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
        // D s_0_3: write-var esizeshadow#6774 <= s_0_2
        fn_state.esizeshadow_6774 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#6775 <= s_0_6
        fn_state.VLshadow_6775 = s_0_6;
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call CheckStreamingSVEEnabled(s_0_8)
        let s_0_9: () = CheckStreamingSVEEnabled(state, tracer, s_0_8);
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#6775:i64
        let s_1_0: i64 = fn_state.VLshadow_6775;
        // D s_1_1: write-var VLshadow#6776 <= s_1_0
        fn_state.VLshadow_6776 = s_1_0;
        // C s_1_2: const #4s : i
        let s_1_2: i128 = 4;
        // D s_1_3: read-var esizeshadow#6774:i64
        let s_1_3: i64 = fn_state.esizeshadow_6774;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: mul s_1_2 s_1_4
        let s_1_5: i128 = ((s_1_2) * (s_1_4));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#6776:i64
        let s_1_7: i64 = fn_state.VLshadow_6776;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast zx s_1_6 -> i
        let s_1_9: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_10: div s_1_8 s_1_9
        let s_1_10: i128 = ((s_1_8) / (s_1_9));
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // C s_1_12: const #0s : i64
        let s_1_12: i64 = 0;
        // C s_1_13: const #1s : i
        let s_1_13: i128 = 1;
        // D s_1_14: cast zx s_1_11 -> i
        let s_1_14: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_15: sub s_1_14 s_1_13
        let s_1_15: i128 = ((s_1_14) - (s_1_13));
        // D s_1_16: cast reint s_1_15 -> i64
        let s_1_16: i64 = (s_1_15 as i64);
        // D s_1_17: write-var gs#287704 <= s_1_16
        fn_state.gs_287704 = s_1_16;
        // D s_1_18: write-var e <= s_1_12
        fn_state.e = s_1_12;
        // N s_1_19: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#287704:i64
        let s_2_1: i64 = fn_state.gs_287704;
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
        // C s_3_0: const #0s : i64
        let s_3_0: i64 = 0;
        // D s_3_1: write-var i <= s_3_0
        fn_state.i = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var i:i64
        let s_4_0: i64 = fn_state.i;
        // C s_4_1: const #3s : i64
        let s_4_1: i64 = 3;
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
        // D s_5_0: read-var n:i64
        let s_5_0: i64 = fn_state.n;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: read-var i:i64
        let s_5_2: i64 = fn_state.i;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: add s_5_1 s_5_3
        let s_5_4: i128 = (s_5_1 + s_5_3);
        // D s_5_5: cast reint s_5_4 -> i64
        let s_5_5: i64 = (s_5_4 as i64);
        // D s_5_6: read-var VLshadow#6776:i64
        let s_5_6: i64 = fn_state.VLshadow_6776;
        // D s_5_7: cast zx s_5_6 -> i
        let s_5_7: i128 = (i128::try_from(s_5_6).unwrap());
        // D s_5_8: cast reint s_5_7 -> i64
        let s_5_8: i64 = (s_5_7 as i64);
        // D s_5_9: cast zx s_5_5 -> i
        let s_5_9: i128 = (i128::try_from(s_5_5).unwrap());
        // D s_5_10: cast zx s_5_8 -> i
        let s_5_10: i128 = (i128::try_from(s_5_8).unwrap());
        // D s_5_11: call Z_read(s_5_9, s_5_10)
        let s_5_11: Bits = Z_read(state, tracer, s_5_9, s_5_10);
        // C s_5_12: const #4s : i
        let s_5_12: i128 = 4;
        // D s_5_13: read-var esizeshadow#6774:i64
        let s_5_13: i64 = fn_state.esizeshadow_6774;
        // D s_5_14: cast zx s_5_13 -> i
        let s_5_14: i128 = (i128::try_from(s_5_13).unwrap());
        // D s_5_15: mul s_5_12 s_5_14
        let s_5_15: i128 = ((s_5_12) * (s_5_14));
        // D s_5_16: cast reint s_5_15 -> i64
        let s_5_16: i64 = (s_5_15 as i64);
        // D s_5_17: cast zx s_5_16 -> i
        let s_5_17: i128 = (i128::try_from(s_5_16).unwrap());
        // D s_5_18: cast reint s_5_17 -> i64
        let s_5_18: i64 = (s_5_17 as i64);
        // D s_5_19: read-var e:i64
        let s_5_19: i64 = fn_state.e;
        // D s_5_20: cast zx s_5_19 -> i
        let s_5_20: i128 = (i128::try_from(s_5_19).unwrap());
        // D s_5_21: cast zx s_5_18 -> i
        let s_5_21: i128 = (i128::try_from(s_5_18).unwrap());
        // D s_5_22: call Elem_read(s_5_11, s_5_20, s_5_21)
        let s_5_22: Bits = Elem_read(state, tracer, s_5_11, s_5_20, s_5_21);
        // D s_5_23: cast zx s_5_22 -> i
        let s_5_23: i128 = (s_5_22.value() as i128);
        // C s_5_24: const #4s : i
        let s_5_24: i128 = 4;
        // D s_5_25: read-var e:i64
        let s_5_25: i64 = fn_state.e;
        // D s_5_26: cast zx s_5_25 -> i
        let s_5_26: i128 = (i128::try_from(s_5_25).unwrap());
        // D s_5_27: mul s_5_24 s_5_26
        let s_5_27: i128 = ((s_5_24) * (s_5_26));
        // D s_5_28: cast reint s_5_27 -> i64
        let s_5_28: i64 = (s_5_27 as i64);
        // D s_5_29: cast zx s_5_28 -> i
        let s_5_29: i128 = (i128::try_from(s_5_28).unwrap());
        // D s_5_30: read-var i:i64
        let s_5_30: i64 = fn_state.i;
        // D s_5_31: cast zx s_5_30 -> i
        let s_5_31: i128 = (i128::try_from(s_5_30).unwrap());
        // D s_5_32: add s_5_29 s_5_31
        let s_5_32: i128 = (s_5_29 + s_5_31);
        // D s_5_33: cast reint s_5_32 -> i64
        let s_5_33: i64 = (s_5_32 as i64);
        // D s_5_34: read-var esizeshadow#6774:i64
        let s_5_34: i64 = fn_state.esizeshadow_6774;
        // D s_5_35: cast zx s_5_34 -> i
        let s_5_35: i128 = (i128::try_from(s_5_34).unwrap());
        // D s_5_36: cast reint s_5_35 -> i64
        let s_5_36: i64 = (s_5_35 as i64);
        // D s_5_37: read-var esizeshadow#6774:i64
        let s_5_37: i64 = fn_state.esizeshadow_6774;
        // D s_5_38: cast zx s_5_37 -> i
        let s_5_38: i128 = (i128::try_from(s_5_37).unwrap());
        // D s_5_39: cast reint s_5_38 -> i64
        let s_5_39: i64 = (s_5_38 as i64);
        // D s_5_40: cast zx s_5_39 -> i
        let s_5_40: i128 = (i128::try_from(s_5_39).unwrap());
        // D s_5_41: call UnsignedSat(s_5_23, s_5_40)
        let s_5_41: Bits = UnsignedSat(state, tracer, s_5_23, s_5_40);
        // D s_5_42: cast zx s_5_33 -> i
        let s_5_42: i128 = (i128::try_from(s_5_33).unwrap());
        // D s_5_43: cast zx s_5_36 -> i
        let s_5_43: i128 = (i128::try_from(s_5_36).unwrap());
        // D s_5_44: read-var result:bv
        let s_5_44: Bits = fn_state.result;
        // D s_5_45: call Elem_set(s_5_44, s_5_42, s_5_43, s_5_41)
        let s_5_45: Bits = Elem_set(state, tracer, s_5_44, s_5_42, s_5_43, s_5_41);
        // D s_5_46: write-var result <= s_5_45
        fn_state.result = s_5_45;
        // D s_5_47: read-var i:i64
        let s_5_47: i64 = fn_state.i;
        // C s_5_48: const #1s : i64
        let s_5_48: i64 = 1;
        // D s_5_49: add s_5_47 s_5_48
        let s_5_49: i64 = (s_5_47 + s_5_48);
        // D s_5_50: write-var i <= s_5_49
        fn_state.i = s_5_49;
        // N s_5_51: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var e:i64
        let s_6_0: i64 = fn_state.e;
        // C s_6_1: const #1s : i64
        let s_6_1: i64 = 1;
        // D s_6_2: add s_6_0 s_6_1
        let s_6_2: i64 = (s_6_0 + s_6_1);
        // D s_6_3: write-var e <= s_6_2
        fn_state.e = s_6_2;
        // N s_6_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var VLshadow#6776:i64
        let s_7_0: i64 = fn_state.VLshadow_6776;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: cast reint s_7_1 -> i64
        let s_7_2: i64 = (s_7_1 as i64);
        // D s_7_3: read-var d:i64
        let s_7_3: i64 = fn_state.d;
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_5: cast zx s_7_2 -> i
        let s_7_5: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_6: read-var result:bv
        let s_7_6: Bits = fn_state.result;
        // D s_7_7: call Z_set(s_7_4, s_7_5, s_7_6)
        let s_7_7: () = Z_set(state, tracer, s_7_4, s_7_5, s_7_6);
        // N s_7_8: return
        return;
    }
}
