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
use Elem_set::*;
use Elem_read::*;
use CheckSVEEnabled::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_SDOT_Z_ZZZ__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    da: i64,
    esize: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        esizeshadow_3327: i64,
        operand3: Bits,
        VLshadow_3328: i64,
        gs_201824: i64,
        res: Bits,
        result: Bits,
        i: i64,
        operand1: Bits,
        VLshadow_3329: i64,
        operand2: Bits,
        VL: i64,
        da: i64,
        esize: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        VL,
        da,
        esize,
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
        // D s_0_0: read-var esize:i64
        let s_0_0: i64 = fn_state.esize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var esizeshadow#3327 <= s_0_2
        fn_state.esizeshadow_3327 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#3328 <= s_0_6
        fn_state.VLshadow_3328 = s_0_6;
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
        // D s_1_0: read-var VLshadow#3328:i64
        let s_1_0: i64 = fn_state.VLshadow_3328;
        // D s_1_1: write-var VLshadow#3329 <= s_1_0
        fn_state.VLshadow_3329 = s_1_0;
        // D s_1_2: read-var VLshadow#3329:i64
        let s_1_2: i64 = fn_state.VLshadow_3329;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: read-var esizeshadow#3327:i64
        let s_1_4: i64 = fn_state.esizeshadow_3327;
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: div s_1_3 s_1_5
        let s_1_6: i128 = ((s_1_3) / (s_1_5));
        // D s_1_7: cast reint s_1_6 -> i64
        let s_1_7: i64 = (s_1_6 as i64);
        // D s_1_8: read-var VLshadow#3329:i64
        let s_1_8: i64 = fn_state.VLshadow_3329;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: cast reint s_1_9 -> i64
        let s_1_10: i64 = (s_1_9 as i64);
        // D s_1_11: read-var n:i64
        let s_1_11: i64 = fn_state.n;
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_13: cast zx s_1_10 -> i
        let s_1_13: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_14: call Z_read(s_1_12, s_1_13)
        let s_1_14: Bits = Z_read(state, tracer, s_1_12, s_1_13);
        // D s_1_15: write-var operand1 <= s_1_14
        fn_state.operand1 = s_1_14;
        // D s_1_16: read-var VLshadow#3329:i64
        let s_1_16: i64 = fn_state.VLshadow_3329;
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_18: cast reint s_1_17 -> i64
        let s_1_18: i64 = (s_1_17 as i64);
        // D s_1_19: read-var m:i64
        let s_1_19: i64 = fn_state.m;
        // D s_1_20: cast zx s_1_19 -> i
        let s_1_20: i128 = (i128::try_from(s_1_19).unwrap());
        // D s_1_21: cast zx s_1_18 -> i
        let s_1_21: i128 = (i128::try_from(s_1_18).unwrap());
        // D s_1_22: call Z_read(s_1_20, s_1_21)
        let s_1_22: Bits = Z_read(state, tracer, s_1_20, s_1_21);
        // D s_1_23: write-var operand2 <= s_1_22
        fn_state.operand2 = s_1_22;
        // D s_1_24: read-var VLshadow#3329:i64
        let s_1_24: i64 = fn_state.VLshadow_3329;
        // D s_1_25: cast zx s_1_24 -> i
        let s_1_25: i128 = (i128::try_from(s_1_24).unwrap());
        // D s_1_26: cast reint s_1_25 -> i64
        let s_1_26: i64 = (s_1_25 as i64);
        // D s_1_27: read-var da:i64
        let s_1_27: i64 = fn_state.da;
        // D s_1_28: cast zx s_1_27 -> i
        let s_1_28: i128 = (i128::try_from(s_1_27).unwrap());
        // D s_1_29: cast zx s_1_26 -> i
        let s_1_29: i128 = (i128::try_from(s_1_26).unwrap());
        // D s_1_30: call Z_read(s_1_28, s_1_29)
        let s_1_30: Bits = Z_read(state, tracer, s_1_28, s_1_29);
        // D s_1_31: write-var operand3 <= s_1_30
        fn_state.operand3 = s_1_30;
        // C s_1_32: const #0s : i64
        let s_1_32: i64 = 0;
        // C s_1_33: const #1s : i
        let s_1_33: i128 = 1;
        // D s_1_34: cast zx s_1_7 -> i
        let s_1_34: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_35: sub s_1_34 s_1_33
        let s_1_35: i128 = ((s_1_34) - (s_1_33));
        // D s_1_36: cast reint s_1_35 -> i64
        let s_1_36: i64 = (s_1_35 as i64);
        // D s_1_37: write-var gs#201824 <= s_1_36
        fn_state.gs_201824 = s_1_36;
        // D s_1_38: write-var e <= s_1_32
        fn_state.e = s_1_32;
        // N s_1_39: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#201824:i64
        let s_2_1: i64 = fn_state.gs_201824;
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
        // D s_3_0: read-var esizeshadow#3327:i64
        let s_3_0: i64 = fn_state.esizeshadow_3327;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: cast reint s_3_1 -> i64
        let s_3_2: i64 = (s_3_1 as i64);
        // D s_3_3: read-var e:i64
        let s_3_3: i64 = fn_state.e;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: cast zx s_3_2 -> i
        let s_3_5: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_6: read-var operand3:bv
        let s_3_6: Bits = fn_state.operand3;
        // D s_3_7: call Elem_read(s_3_6, s_3_4, s_3_5)
        let s_3_7: Bits = Elem_read(state, tracer, s_3_6, s_3_4, s_3_5);
        // D s_3_8: write-var res <= s_3_7
        fn_state.res = s_3_7;
        // C s_3_9: const #0s : i64
        let s_3_9: i64 = 0;
        // D s_3_10: write-var i <= s_3_9
        fn_state.i = s_3_9;
        // N s_3_11: jump b4
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
        // C s_5_0: const #4s : i
        let s_5_0: i128 = 4;
        // D s_5_1: read-var e:i64
        let s_5_1: i64 = fn_state.e;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: mul s_5_0 s_5_2
        let s_5_3: i128 = ((s_5_0) * (s_5_2));
        // D s_5_4: cast reint s_5_3 -> i64
        let s_5_4: i64 = (s_5_3 as i64);
        // D s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_6: read-var i:i64
        let s_5_6: i64 = fn_state.i;
        // D s_5_7: cast zx s_5_6 -> i
        let s_5_7: i128 = (i128::try_from(s_5_6).unwrap());
        // D s_5_8: add s_5_5 s_5_7
        let s_5_8: i128 = (s_5_5 + s_5_7);
        // D s_5_9: cast reint s_5_8 -> i64
        let s_5_9: i64 = (s_5_8 as i64);
        // C s_5_10: const #4s : i
        let s_5_10: i128 = 4;
        // D s_5_11: read-var esizeshadow#3327:i64
        let s_5_11: i64 = fn_state.esizeshadow_3327;
        // D s_5_12: cast zx s_5_11 -> i
        let s_5_12: i128 = (i128::try_from(s_5_11).unwrap());
        // D s_5_13: div s_5_12 s_5_10
        let s_5_13: i128 = ((s_5_12) / (s_5_10));
        // D s_5_14: cast reint s_5_13 -> i64
        let s_5_14: i64 = (s_5_13 as i64);
        // D s_5_15: cast zx s_5_14 -> i
        let s_5_15: i128 = (i128::try_from(s_5_14).unwrap());
        // D s_5_16: cast reint s_5_15 -> i64
        let s_5_16: i64 = (s_5_15 as i64);
        // D s_5_17: cast zx s_5_9 -> i
        let s_5_17: i128 = (i128::try_from(s_5_9).unwrap());
        // D s_5_18: cast zx s_5_16 -> i
        let s_5_18: i128 = (i128::try_from(s_5_16).unwrap());
        // D s_5_19: read-var operand1:bv
        let s_5_19: Bits = fn_state.operand1;
        // D s_5_20: call Elem_read(s_5_19, s_5_17, s_5_18)
        let s_5_20: Bits = Elem_read(state, tracer, s_5_19, s_5_17, s_5_18);
        // D s_5_21: cast sx s_5_20 -> i
        let s_5_21: i128 = {
            let sign_bit = s_5_20.length() - 1;
            let mut result = s_5_20.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_5_22: cast reint s_5_21 -> i64
        let s_5_22: i64 = (s_5_21 as i64);
        // C s_5_23: const #4s : i
        let s_5_23: i128 = 4;
        // D s_5_24: read-var e:i64
        let s_5_24: i64 = fn_state.e;
        // D s_5_25: cast zx s_5_24 -> i
        let s_5_25: i128 = (i128::try_from(s_5_24).unwrap());
        // D s_5_26: mul s_5_23 s_5_25
        let s_5_26: i128 = ((s_5_23) * (s_5_25));
        // D s_5_27: cast reint s_5_26 -> i64
        let s_5_27: i64 = (s_5_26 as i64);
        // D s_5_28: cast zx s_5_27 -> i
        let s_5_28: i128 = (i128::try_from(s_5_27).unwrap());
        // D s_5_29: read-var i:i64
        let s_5_29: i64 = fn_state.i;
        // D s_5_30: cast zx s_5_29 -> i
        let s_5_30: i128 = (i128::try_from(s_5_29).unwrap());
        // D s_5_31: add s_5_28 s_5_30
        let s_5_31: i128 = (s_5_28 + s_5_30);
        // D s_5_32: cast reint s_5_31 -> i64
        let s_5_32: i64 = (s_5_31 as i64);
        // C s_5_33: const #4s : i
        let s_5_33: i128 = 4;
        // D s_5_34: read-var esizeshadow#3327:i64
        let s_5_34: i64 = fn_state.esizeshadow_3327;
        // D s_5_35: cast zx s_5_34 -> i
        let s_5_35: i128 = (i128::try_from(s_5_34).unwrap());
        // D s_5_36: div s_5_35 s_5_33
        let s_5_36: i128 = ((s_5_35) / (s_5_33));
        // D s_5_37: cast reint s_5_36 -> i64
        let s_5_37: i64 = (s_5_36 as i64);
        // D s_5_38: cast zx s_5_37 -> i
        let s_5_38: i128 = (i128::try_from(s_5_37).unwrap());
        // D s_5_39: cast reint s_5_38 -> i64
        let s_5_39: i64 = (s_5_38 as i64);
        // D s_5_40: cast zx s_5_32 -> i
        let s_5_40: i128 = (i128::try_from(s_5_32).unwrap());
        // D s_5_41: cast zx s_5_39 -> i
        let s_5_41: i128 = (i128::try_from(s_5_39).unwrap());
        // D s_5_42: read-var operand2:bv
        let s_5_42: Bits = fn_state.operand2;
        // D s_5_43: call Elem_read(s_5_42, s_5_40, s_5_41)
        let s_5_43: Bits = Elem_read(state, tracer, s_5_42, s_5_40, s_5_41);
        // D s_5_44: cast sx s_5_43 -> i
        let s_5_44: i128 = {
            let sign_bit = s_5_43.length() - 1;
            let mut result = s_5_43.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_5_45: cast reint s_5_44 -> i64
        let s_5_45: i64 = (s_5_44 as i64);
        // D s_5_46: cast zx s_5_22 -> i
        let s_5_46: i128 = (i128::try_from(s_5_22).unwrap());
        // D s_5_47: cast zx s_5_45 -> i
        let s_5_47: i128 = (i128::try_from(s_5_45).unwrap());
        // D s_5_48: mul s_5_46 s_5_47
        let s_5_48: i128 = ((s_5_46) * (s_5_47));
        // D s_5_49: cast reint s_5_48 -> i64
        let s_5_49: i64 = (s_5_48 as i64);
        // D s_5_50: cast zx s_5_49 -> i
        let s_5_50: i128 = (i128::try_from(s_5_49).unwrap());
        // D s_5_51: read-var res:bv
        let s_5_51: Bits = fn_state.res;
        // D s_5_52: cast cvt s_5_50 -> bv
        let s_5_52: Bits = Bits::new(s_5_50 as u128, 128);
        // D s_5_53: add s_5_51 s_5_52
        let s_5_53: Bits = (s_5_51 + s_5_52);
        // D s_5_54: write-var res <= s_5_53
        fn_state.res = s_5_53;
        // D s_5_55: read-var i:i64
        let s_5_55: i64 = fn_state.i;
        // C s_5_56: const #1s : i64
        let s_5_56: i64 = 1;
        // D s_5_57: add s_5_55 s_5_56
        let s_5_57: i64 = (s_5_55 + s_5_56);
        // D s_5_58: write-var i <= s_5_57
        fn_state.i = s_5_57;
        // N s_5_59: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var esizeshadow#3327:i64
        let s_6_0: i64 = fn_state.esizeshadow_3327;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: cast reint s_6_1 -> i64
        let s_6_2: i64 = (s_6_1 as i64);
        // D s_6_3: read-var e:i64
        let s_6_3: i64 = fn_state.e;
        // D s_6_4: cast zx s_6_3 -> i
        let s_6_4: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_5: cast zx s_6_2 -> i
        let s_6_5: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_6: read-var result:bv
        let s_6_6: Bits = fn_state.result;
        // D s_6_7: read-var res:bv
        let s_6_7: Bits = fn_state.res;
        // D s_6_8: call Elem_set(s_6_6, s_6_4, s_6_5, s_6_7)
        let s_6_8: Bits = Elem_set(state, tracer, s_6_6, s_6_4, s_6_5, s_6_7);
        // D s_6_9: write-var result <= s_6_8
        fn_state.result = s_6_8;
        // D s_6_10: read-var e:i64
        let s_6_10: i64 = fn_state.e;
        // C s_6_11: const #1s : i64
        let s_6_11: i64 = 1;
        // D s_6_12: add s_6_10 s_6_11
        let s_6_12: i64 = (s_6_10 + s_6_11);
        // D s_6_13: write-var e <= s_6_12
        fn_state.e = s_6_12;
        // N s_6_14: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var VLshadow#3329:i64
        let s_7_0: i64 = fn_state.VLshadow_3329;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: cast reint s_7_1 -> i64
        let s_7_2: i64 = (s_7_1 as i64);
        // D s_7_3: read-var da:i64
        let s_7_3: i64 = fn_state.da;
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
