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
use u__id::*;
use CheckSVEEnabled::*;
use AnyActiveElement::*;
use ActivePredicateElement::*;
use P_read::*;
use FPCR_read::*;
use FPAdd::*;
use FPNeg::*;
use Elem_read::*;
use Zeros::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_FCADD_Z_P_ZZ__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    dn: i64,
    esize: i64,
    g: i64,
    m: i64,
    sub_i: bool,
    sub_r: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        esizeshadow_2295: i64,
        gs_180897: bool,
        gs_180888: bool,
        gs_180912: bool,
        p: i64,
        gs_180905: bool,
        acc_r: Bits,
        gs_180907: bool,
        elt2_r: Bits,
        mask: Bits,
        VLshadow_2297: i64,
        gs_180890: bool,
        elt2_i: Bits,
        gs_180869: i64,
        operand1: Bits,
        gs_180895: bool,
        operand2: Bits,
        VLshadow_2296: i64,
        gs_180914: bool,
        acc_i: Bits,
        pairs: i64,
        result: Bits,
        VL: i64,
        dn: i64,
        esize: i64,
        g: i64,
        m: i64,
        sub_i: bool,
        sub_r: bool,
    }
    let fn_state = FunctionState {
        VL,
        dn,
        esize,
        g,
        m,
        sub_i,
        sub_r,
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
        // D s_0_3: write-var esizeshadow#2295 <= s_0_2
        fn_state.esizeshadow_2295 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#2296 <= s_0_6
        fn_state.VLshadow_2296 = s_0_6;
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
        // D s_1_0: read-var VLshadow#2296:i64
        let s_1_0: i64 = fn_state.VLshadow_2296;
        // D s_1_1: write-var VLshadow#2297 <= s_1_0
        fn_state.VLshadow_2297 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#2297:i64
        let s_1_3: i64 = fn_state.VLshadow_2297;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // C s_1_7: const #2s : i
        let s_1_7: i128 = 2;
        // D s_1_8: read-var esizeshadow#2295:i64
        let s_1_8: i64 = fn_state.esizeshadow_2295;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: mul s_1_7 s_1_9
        let s_1_10: i128 = ((s_1_7) * (s_1_9));
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: read-var VLshadow#2297:i64
        let s_1_12: i64 = fn_state.VLshadow_2297;
        // D s_1_13: cast zx s_1_12 -> i
        let s_1_13: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_14: cast zx s_1_11 -> i
        let s_1_14: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_15: div s_1_13 s_1_14
        let s_1_15: i128 = ((s_1_13) / (s_1_14));
        // D s_1_16: cast reint s_1_15 -> i64
        let s_1_16: i64 = (s_1_15 as i64);
        // D s_1_17: write-var pairs <= s_1_16
        fn_state.pairs = s_1_16;
        // D s_1_18: cast zx s_1_6 -> i
        let s_1_18: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_19: cast reint s_1_18 -> i64
        let s_1_19: i64 = (s_1_18 as i64);
        // D s_1_20: read-var g:i64
        let s_1_20: i64 = fn_state.g;
        // D s_1_21: cast zx s_1_20 -> i
        let s_1_21: i128 = (i128::try_from(s_1_20).unwrap());
        // D s_1_22: cast zx s_1_19 -> i
        let s_1_22: i128 = (i128::try_from(s_1_19).unwrap());
        // D s_1_23: call P_read(s_1_21, s_1_22)
        let s_1_23: Bits = P_read(state, tracer, s_1_21, s_1_22);
        // D s_1_24: write-var mask <= s_1_23
        fn_state.mask = s_1_23;
        // D s_1_25: read-var VLshadow#2297:i64
        let s_1_25: i64 = fn_state.VLshadow_2297;
        // D s_1_26: cast zx s_1_25 -> i
        let s_1_26: i128 = (i128::try_from(s_1_25).unwrap());
        // D s_1_27: cast reint s_1_26 -> i64
        let s_1_27: i64 = (s_1_26 as i64);
        // D s_1_28: read-var dn:i64
        let s_1_28: i64 = fn_state.dn;
        // D s_1_29: cast zx s_1_28 -> i
        let s_1_29: i128 = (i128::try_from(s_1_28).unwrap());
        // D s_1_30: cast zx s_1_27 -> i
        let s_1_30: i128 = (i128::try_from(s_1_27).unwrap());
        // D s_1_31: call Z_read(s_1_29, s_1_30)
        let s_1_31: Bits = Z_read(state, tracer, s_1_29, s_1_30);
        // D s_1_32: write-var operand1 <= s_1_31
        fn_state.operand1 = s_1_31;
        // D s_1_33: read-var esizeshadow#2295:i64
        let s_1_33: i64 = fn_state.esizeshadow_2295;
        // D s_1_34: cast zx s_1_33 -> i
        let s_1_34: i128 = (i128::try_from(s_1_33).unwrap());
        // D s_1_35: read-var mask:bv
        let s_1_35: Bits = fn_state.mask;
        // D s_1_36: call AnyActiveElement(s_1_35, s_1_34)
        let s_1_36: bool = AnyActiveElement(state, tracer, s_1_35, s_1_34);
        // N s_1_37: branch s_1_36 b47 b2
        if s_1_36 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var VLshadow#2297:i64
        let s_2_0: i64 = fn_state.VLshadow_2297;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: call Zeros(s_2_1)
        let s_2_2: Bits = Zeros(state, tracer, s_2_1);
        // D s_2_3: write-var operand2 <= s_2_2
        fn_state.operand2 = s_2_2;
        // N s_2_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0s : i64
        let s_3_0: i64 = 0;
        // C s_3_1: const #1s : i
        let s_3_1: i128 = 1;
        // D s_3_2: read-var pairs:i64
        let s_3_2: i64 = fn_state.pairs;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: sub s_3_3 s_3_1
        let s_3_4: i128 = ((s_3_3) - (s_3_1));
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: write-var gs#180869 <= s_3_5
        fn_state.gs_180869 = s_3_5;
        // D s_3_7: write-var p <= s_3_0
        fn_state.p = s_3_0;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var p:i64
        let s_4_0: i64 = fn_state.p;
        // D s_4_1: read-var gs#180869:i64
        let s_4_1: i64 = fn_state.gs_180869;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b46 b5
        if s_4_2 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #2s : i
        let s_5_0: i128 = 2;
        // D s_5_1: read-var p:i64
        let s_5_1: i64 = fn_state.p;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: mul s_5_0 s_5_2
        let s_5_3: i128 = ((s_5_0) * (s_5_2));
        // D s_5_4: cast reint s_5_3 -> i64
        let s_5_4: i64 = (s_5_3 as i64);
        // C s_5_5: const #0s : i
        let s_5_5: i128 = 0;
        // D s_5_6: cast zx s_5_4 -> i
        let s_5_6: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_7: add s_5_6 s_5_5
        let s_5_7: i128 = (s_5_6 + s_5_5);
        // D s_5_8: cast reint s_5_7 -> i64
        let s_5_8: i64 = (s_5_7 as i64);
        // D s_5_9: read-var esizeshadow#2295:i64
        let s_5_9: i64 = fn_state.esizeshadow_2295;
        // D s_5_10: cast zx s_5_9 -> i
        let s_5_10: i128 = (i128::try_from(s_5_9).unwrap());
        // D s_5_11: cast reint s_5_10 -> i64
        let s_5_11: i64 = (s_5_10 as i64);
        // D s_5_12: cast zx s_5_8 -> i
        let s_5_12: i128 = (i128::try_from(s_5_8).unwrap());
        // D s_5_13: cast zx s_5_11 -> i
        let s_5_13: i128 = (i128::try_from(s_5_11).unwrap());
        // D s_5_14: read-var operand1:bv
        let s_5_14: Bits = fn_state.operand1;
        // D s_5_15: call Elem_read(s_5_14, s_5_12, s_5_13)
        let s_5_15: Bits = Elem_read(state, tracer, s_5_14, s_5_12, s_5_13);
        // D s_5_16: write-var acc_r <= s_5_15
        fn_state.acc_r = s_5_15;
        // C s_5_17: const #2s : i
        let s_5_17: i128 = 2;
        // D s_5_18: read-var p:i64
        let s_5_18: i64 = fn_state.p;
        // D s_5_19: cast zx s_5_18 -> i
        let s_5_19: i128 = (i128::try_from(s_5_18).unwrap());
        // D s_5_20: mul s_5_17 s_5_19
        let s_5_20: i128 = ((s_5_17) * (s_5_19));
        // D s_5_21: cast reint s_5_20 -> i64
        let s_5_21: i64 = (s_5_20 as i64);
        // C s_5_22: const #1s : i
        let s_5_22: i128 = 1;
        // D s_5_23: cast zx s_5_21 -> i
        let s_5_23: i128 = (i128::try_from(s_5_21).unwrap());
        // D s_5_24: add s_5_23 s_5_22
        let s_5_24: i128 = (s_5_23 + s_5_22);
        // D s_5_25: cast reint s_5_24 -> i64
        let s_5_25: i64 = (s_5_24 as i64);
        // D s_5_26: read-var esizeshadow#2295:i64
        let s_5_26: i64 = fn_state.esizeshadow_2295;
        // D s_5_27: cast zx s_5_26 -> i
        let s_5_27: i128 = (i128::try_from(s_5_26).unwrap());
        // D s_5_28: cast reint s_5_27 -> i64
        let s_5_28: i64 = (s_5_27 as i64);
        // D s_5_29: cast zx s_5_25 -> i
        let s_5_29: i128 = (i128::try_from(s_5_25).unwrap());
        // D s_5_30: cast zx s_5_28 -> i
        let s_5_30: i128 = (i128::try_from(s_5_28).unwrap());
        // D s_5_31: read-var operand1:bv
        let s_5_31: Bits = fn_state.operand1;
        // D s_5_32: call Elem_read(s_5_31, s_5_29, s_5_30)
        let s_5_32: Bits = Elem_read(state, tracer, s_5_31, s_5_29, s_5_30);
        // D s_5_33: write-var acc_i <= s_5_32
        fn_state.acc_i = s_5_32;
        // C s_5_34: const #2s : i
        let s_5_34: i128 = 2;
        // D s_5_35: read-var p:i64
        let s_5_35: i64 = fn_state.p;
        // D s_5_36: cast zx s_5_35 -> i
        let s_5_36: i128 = (i128::try_from(s_5_35).unwrap());
        // D s_5_37: mul s_5_34 s_5_36
        let s_5_37: i128 = ((s_5_34) * (s_5_36));
        // D s_5_38: cast reint s_5_37 -> i64
        let s_5_38: i64 = (s_5_37 as i64);
        // C s_5_39: const #0s : i
        let s_5_39: i128 = 0;
        // D s_5_40: cast zx s_5_38 -> i
        let s_5_40: i128 = (i128::try_from(s_5_38).unwrap());
        // D s_5_41: add s_5_40 s_5_39
        let s_5_41: i128 = (s_5_40 + s_5_39);
        // D s_5_42: cast reint s_5_41 -> i64
        let s_5_42: i64 = (s_5_41 as i64);
        // D s_5_43: cast zx s_5_42 -> i
        let s_5_43: i128 = (i128::try_from(s_5_42).unwrap());
        // D s_5_44: read-var esizeshadow#2295:i64
        let s_5_44: i64 = fn_state.esizeshadow_2295;
        // D s_5_45: cast zx s_5_44 -> i
        let s_5_45: i128 = (i128::try_from(s_5_44).unwrap());
        // D s_5_46: read-var mask:bv
        let s_5_46: Bits = fn_state.mask;
        // D s_5_47: call ActivePredicateElement(s_5_46, s_5_43, s_5_45)
        let s_5_47: bool = ActivePredicateElement(state, tracer, s_5_46, s_5_43, s_5_45);
        // N s_5_48: branch s_5_47 b28 b6
        if s_5_47 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #2s : i
        let s_7_0: i128 = 2;
        // D s_7_1: read-var p:i64
        let s_7_1: i64 = fn_state.p;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: mul s_7_0 s_7_2
        let s_7_3: i128 = ((s_7_0) * (s_7_2));
        // D s_7_4: cast reint s_7_3 -> i64
        let s_7_4: i64 = (s_7_3 as i64);
        // C s_7_5: const #1s : i
        let s_7_5: i128 = 1;
        // D s_7_6: cast zx s_7_4 -> i
        let s_7_6: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_7: add s_7_6 s_7_5
        let s_7_7: i128 = (s_7_6 + s_7_5);
        // D s_7_8: cast reint s_7_7 -> i64
        let s_7_8: i64 = (s_7_7 as i64);
        // D s_7_9: cast zx s_7_8 -> i
        let s_7_9: i128 = (i128::try_from(s_7_8).unwrap());
        // D s_7_10: read-var esizeshadow#2295:i64
        let s_7_10: i64 = fn_state.esizeshadow_2295;
        // D s_7_11: cast zx s_7_10 -> i
        let s_7_11: i128 = (i128::try_from(s_7_10).unwrap());
        // D s_7_12: read-var mask:bv
        let s_7_12: Bits = fn_state.mask;
        // D s_7_13: call ActivePredicateElement(s_7_12, s_7_9, s_7_11)
        let s_7_13: bool = ActivePredicateElement(state, tracer, s_7_12, s_7_9, s_7_11);
        // N s_7_14: branch s_7_13 b10 b8
        if s_7_13 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #2s : i
        let s_9_0: i128 = 2;
        // D s_9_1: read-var p:i64
        let s_9_1: i64 = fn_state.p;
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (i128::try_from(s_9_1).unwrap());
        // D s_9_3: mul s_9_0 s_9_2
        let s_9_3: i128 = ((s_9_0) * (s_9_2));
        // D s_9_4: cast reint s_9_3 -> i64
        let s_9_4: i64 = (s_9_3 as i64);
        // C s_9_5: const #0s : i
        let s_9_5: i128 = 0;
        // D s_9_6: cast zx s_9_4 -> i
        let s_9_6: i128 = (i128::try_from(s_9_4).unwrap());
        // D s_9_7: add s_9_6 s_9_5
        let s_9_7: i128 = (s_9_6 + s_9_5);
        // D s_9_8: cast reint s_9_7 -> i64
        let s_9_8: i64 = (s_9_7 as i64);
        // D s_9_9: read-var esizeshadow#2295:i64
        let s_9_9: i64 = fn_state.esizeshadow_2295;
        // D s_9_10: cast zx s_9_9 -> i
        let s_9_10: i128 = (i128::try_from(s_9_9).unwrap());
        // D s_9_11: cast reint s_9_10 -> i64
        let s_9_11: i64 = (s_9_10 as i64);
        // D s_9_12: cast zx s_9_8 -> i
        let s_9_12: i128 = (i128::try_from(s_9_8).unwrap());
        // D s_9_13: cast zx s_9_11 -> i
        let s_9_13: i128 = (i128::try_from(s_9_11).unwrap());
        // D s_9_14: read-var result:bv
        let s_9_14: Bits = fn_state.result;
        // D s_9_15: read-var acc_r:bv
        let s_9_15: Bits = fn_state.acc_r;
        // D s_9_16: call Elem_set(s_9_14, s_9_12, s_9_13, s_9_15)
        let s_9_16: Bits = Elem_set(state, tracer, s_9_14, s_9_12, s_9_13, s_9_15);
        // D s_9_17: write-var result <= s_9_16
        fn_state.result = s_9_16;
        // C s_9_18: const #2s : i
        let s_9_18: i128 = 2;
        // D s_9_19: read-var p:i64
        let s_9_19: i64 = fn_state.p;
        // D s_9_20: cast zx s_9_19 -> i
        let s_9_20: i128 = (i128::try_from(s_9_19).unwrap());
        // D s_9_21: mul s_9_18 s_9_20
        let s_9_21: i128 = ((s_9_18) * (s_9_20));
        // D s_9_22: cast reint s_9_21 -> i64
        let s_9_22: i64 = (s_9_21 as i64);
        // C s_9_23: const #1s : i
        let s_9_23: i128 = 1;
        // D s_9_24: cast zx s_9_22 -> i
        let s_9_24: i128 = (i128::try_from(s_9_22).unwrap());
        // D s_9_25: add s_9_24 s_9_23
        let s_9_25: i128 = (s_9_24 + s_9_23);
        // D s_9_26: cast reint s_9_25 -> i64
        let s_9_26: i64 = (s_9_25 as i64);
        // D s_9_27: read-var esizeshadow#2295:i64
        let s_9_27: i64 = fn_state.esizeshadow_2295;
        // D s_9_28: cast zx s_9_27 -> i
        let s_9_28: i128 = (i128::try_from(s_9_27).unwrap());
        // D s_9_29: cast reint s_9_28 -> i64
        let s_9_29: i64 = (s_9_28 as i64);
        // D s_9_30: cast zx s_9_26 -> i
        let s_9_30: i128 = (i128::try_from(s_9_26).unwrap());
        // D s_9_31: cast zx s_9_29 -> i
        let s_9_31: i128 = (i128::try_from(s_9_29).unwrap());
        // D s_9_32: read-var result:bv
        let s_9_32: Bits = fn_state.result;
        // D s_9_33: read-var acc_i:bv
        let s_9_33: Bits = fn_state.acc_i;
        // D s_9_34: call Elem_set(s_9_32, s_9_30, s_9_31, s_9_33)
        let s_9_34: Bits = Elem_set(state, tracer, s_9_32, s_9_30, s_9_31, s_9_33);
        // D s_9_35: write-var result <= s_9_34
        fn_state.result = s_9_34;
        // D s_9_36: read-var p:i64
        let s_9_36: i64 = fn_state.p;
        // C s_9_37: const #1s : i64
        let s_9_37: i64 = 1;
        // D s_9_38: add s_9_36 s_9_37
        let s_9_38: i64 = (s_9_36 + s_9_37);
        // D s_9_39: write-var p <= s_9_38
        fn_state.p = s_9_38;
        // N s_9_40: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #2s : i
        let s_10_0: i128 = 2;
        // D s_10_1: read-var p:i64
        let s_10_1: i64 = fn_state.p;
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // D s_10_3: mul s_10_0 s_10_2
        let s_10_3: i128 = ((s_10_0) * (s_10_2));
        // D s_10_4: cast reint s_10_3 -> i64
        let s_10_4: i64 = (s_10_3 as i64);
        // C s_10_5: const #0s : i
        let s_10_5: i128 = 0;
        // D s_10_6: cast zx s_10_4 -> i
        let s_10_6: i128 = (i128::try_from(s_10_4).unwrap());
        // D s_10_7: add s_10_6 s_10_5
        let s_10_7: i128 = (s_10_6 + s_10_5);
        // D s_10_8: cast reint s_10_7 -> i64
        let s_10_8: i64 = (s_10_7 as i64);
        // D s_10_9: read-var esizeshadow#2295:i64
        let s_10_9: i64 = fn_state.esizeshadow_2295;
        // D s_10_10: cast zx s_10_9 -> i
        let s_10_10: i128 = (i128::try_from(s_10_9).unwrap());
        // D s_10_11: cast reint s_10_10 -> i64
        let s_10_11: i64 = (s_10_10 as i64);
        // D s_10_12: cast zx s_10_8 -> i
        let s_10_12: i128 = (i128::try_from(s_10_8).unwrap());
        // D s_10_13: cast zx s_10_11 -> i
        let s_10_13: i128 = (i128::try_from(s_10_11).unwrap());
        // D s_10_14: read-var operand2:bv
        let s_10_14: Bits = fn_state.operand2;
        // D s_10_15: call Elem_read(s_10_14, s_10_12, s_10_13)
        let s_10_15: Bits = Elem_read(state, tracer, s_10_14, s_10_12, s_10_13);
        // D s_10_16: write-var elt2_r <= s_10_15
        fn_state.elt2_r = s_10_15;
        // D s_10_17: read-var sub_r:u8
        let s_10_17: bool = fn_state.sub_r;
        // N s_10_18: branch s_10_17 b20 b11
        if s_10_17 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var esizeshadow#2295:i64
        let s_12_0: i64 = fn_state.esizeshadow_2295;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: call __id(s_12_1)
        let s_12_2: i128 = u__id(state, tracer, s_12_1);
        // D s_12_3: cast reint s_12_2 -> i64
        let s_12_3: i64 = (s_12_2 as i64);
        // C s_12_4: const #16s : i
        let s_12_4: i128 = 16;
        // D s_12_5: cast zx s_12_3 -> i
        let s_12_5: i128 = (i128::try_from(s_12_3).unwrap());
        // D s_12_6: cmp-eq s_12_5 s_12_4
        let s_12_6: bool = ((s_12_5) == (s_12_4));
        // N s_12_7: branch s_12_6 b19 b13
        if s_12_6 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var esizeshadow#2295:i64
        let s_13_0: i64 = fn_state.esizeshadow_2295;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: call __id(s_13_1)
        let s_13_2: i128 = u__id(state, tracer, s_13_1);
        // D s_13_3: cast reint s_13_2 -> i64
        let s_13_3: i64 = (s_13_2 as i64);
        // C s_13_4: const #32s : i
        let s_13_4: i128 = 32;
        // D s_13_5: cast zx s_13_3 -> i
        let s_13_5: i128 = (i128::try_from(s_13_3).unwrap());
        // D s_13_6: cmp-eq s_13_5 s_13_4
        let s_13_6: bool = ((s_13_5) == (s_13_4));
        // D s_13_7: write-var gs#180888 <= s_13_6
        fn_state.gs_180888 = s_13_6;
        // N s_13_8: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#180888:u8
        let s_14_0: bool = fn_state.gs_180888;
        // N s_14_1: branch s_14_0 b18 b15
        if s_14_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var esizeshadow#2295:i64
        let s_15_0: i64 = fn_state.esizeshadow_2295;
        // D s_15_1: cast zx s_15_0 -> i
        let s_15_1: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_2: call __id(s_15_1)
        let s_15_2: i128 = u__id(state, tracer, s_15_1);
        // D s_15_3: cast reint s_15_2 -> i64
        let s_15_3: i64 = (s_15_2 as i64);
        // C s_15_4: const #64s : i
        let s_15_4: i128 = 64;
        // D s_15_5: cast zx s_15_3 -> i
        let s_15_5: i128 = (i128::try_from(s_15_3).unwrap());
        // D s_15_6: cmp-eq s_15_5 s_15_4
        let s_15_6: bool = ((s_15_5) == (s_15_4));
        // D s_15_7: write-var gs#180890 <= s_15_6
        fn_state.gs_180890 = s_15_6;
        // N s_15_8: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#180890:u8
        let s_16_0: bool = fn_state.gs_180890;
        // N s_16_1: assert s_16_0
        let s_16_1: () = assert!(s_16_0);
        // C s_16_2: const #() : ()
        let s_16_2: () = ();
        // S s_16_3: call FPCR_read(s_16_2)
        let s_16_3: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_16_2);
        // D s_16_4: read-var acc_i:bv
        let s_16_4: Bits = fn_state.acc_i;
        // D s_16_5: read-var elt2_r:bv
        let s_16_5: Bits = fn_state.elt2_r;
        // D s_16_6: call FPAdd(s_16_4, s_16_5, s_16_3)
        let s_16_6: Bits = FPAdd(state, tracer, s_16_4, s_16_5, s_16_3);
        // D s_16_7: write-var acc_i <= s_16_6
        fn_state.acc_i = s_16_6;
        // N s_16_8: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#180890 <= s_18_0
        fn_state.gs_180890 = s_18_0;
        // N s_18_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #1u : u8
        let s_19_0: bool = true;
        // D s_19_1: write-var gs#180888 <= s_19_0
        fn_state.gs_180888 = s_19_0;
        // N s_19_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var esizeshadow#2295:i64
        let s_20_0: i64 = fn_state.esizeshadow_2295;
        // D s_20_1: cast zx s_20_0 -> i
        let s_20_1: i128 = (i128::try_from(s_20_0).unwrap());
        // D s_20_2: call __id(s_20_1)
        let s_20_2: i128 = u__id(state, tracer, s_20_1);
        // D s_20_3: cast reint s_20_2 -> i64
        let s_20_3: i64 = (s_20_2 as i64);
        // C s_20_4: const #16s : i
        let s_20_4: i128 = 16;
        // D s_20_5: cast zx s_20_3 -> i
        let s_20_5: i128 = (i128::try_from(s_20_3).unwrap());
        // D s_20_6: cmp-eq s_20_5 s_20_4
        let s_20_6: bool = ((s_20_5) == (s_20_4));
        // N s_20_7: branch s_20_6 b27 b21
        if s_20_6 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var esizeshadow#2295:i64
        let s_21_0: i64 = fn_state.esizeshadow_2295;
        // D s_21_1: cast zx s_21_0 -> i
        let s_21_1: i128 = (i128::try_from(s_21_0).unwrap());
        // D s_21_2: call __id(s_21_1)
        let s_21_2: i128 = u__id(state, tracer, s_21_1);
        // D s_21_3: cast reint s_21_2 -> i64
        let s_21_3: i64 = (s_21_2 as i64);
        // C s_21_4: const #32s : i
        let s_21_4: i128 = 32;
        // D s_21_5: cast zx s_21_3 -> i
        let s_21_5: i128 = (i128::try_from(s_21_3).unwrap());
        // D s_21_6: cmp-eq s_21_5 s_21_4
        let s_21_6: bool = ((s_21_5) == (s_21_4));
        // D s_21_7: write-var gs#180895 <= s_21_6
        fn_state.gs_180895 = s_21_6;
        // N s_21_8: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#180895:u8
        let s_22_0: bool = fn_state.gs_180895;
        // N s_22_1: branch s_22_0 b26 b23
        if s_22_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var esizeshadow#2295:i64
        let s_23_0: i64 = fn_state.esizeshadow_2295;
        // D s_23_1: cast zx s_23_0 -> i
        let s_23_1: i128 = (i128::try_from(s_23_0).unwrap());
        // D s_23_2: call __id(s_23_1)
        let s_23_2: i128 = u__id(state, tracer, s_23_1);
        // D s_23_3: cast reint s_23_2 -> i64
        let s_23_3: i64 = (s_23_2 as i64);
        // C s_23_4: const #64s : i
        let s_23_4: i128 = 64;
        // D s_23_5: cast zx s_23_3 -> i
        let s_23_5: i128 = (i128::try_from(s_23_3).unwrap());
        // D s_23_6: cmp-eq s_23_5 s_23_4
        let s_23_6: bool = ((s_23_5) == (s_23_4));
        // D s_23_7: write-var gs#180897 <= s_23_6
        fn_state.gs_180897 = s_23_6;
        // N s_23_8: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#180897:u8
        let s_24_0: bool = fn_state.gs_180897;
        // N s_24_1: assert s_24_0
        let s_24_1: () = assert!(s_24_0);
        // D s_24_2: read-var elt2_r:bv
        let s_24_2: Bits = fn_state.elt2_r;
        // D s_24_3: call FPNeg(s_24_2)
        let s_24_3: Bits = FPNeg(state, tracer, s_24_2);
        // D s_24_4: write-var elt2_r <= s_24_3
        fn_state.elt2_r = s_24_3;
        // N s_24_5: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_25_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // D s_26_1: write-var gs#180897 <= s_26_0
        fn_state.gs_180897 = s_26_0;
        // N s_26_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // D s_27_1: write-var gs#180895 <= s_27_0
        fn_state.gs_180895 = s_27_0;
        // N s_27_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #2s : i
        let s_28_0: i128 = 2;
        // D s_28_1: read-var p:i64
        let s_28_1: i64 = fn_state.p;
        // D s_28_2: cast zx s_28_1 -> i
        let s_28_2: i128 = (i128::try_from(s_28_1).unwrap());
        // D s_28_3: mul s_28_0 s_28_2
        let s_28_3: i128 = ((s_28_0) * (s_28_2));
        // D s_28_4: cast reint s_28_3 -> i64
        let s_28_4: i64 = (s_28_3 as i64);
        // C s_28_5: const #1s : i
        let s_28_5: i128 = 1;
        // D s_28_6: cast zx s_28_4 -> i
        let s_28_6: i128 = (i128::try_from(s_28_4).unwrap());
        // D s_28_7: add s_28_6 s_28_5
        let s_28_7: i128 = (s_28_6 + s_28_5);
        // D s_28_8: cast reint s_28_7 -> i64
        let s_28_8: i64 = (s_28_7 as i64);
        // D s_28_9: read-var esizeshadow#2295:i64
        let s_28_9: i64 = fn_state.esizeshadow_2295;
        // D s_28_10: cast zx s_28_9 -> i
        let s_28_10: i128 = (i128::try_from(s_28_9).unwrap());
        // D s_28_11: cast reint s_28_10 -> i64
        let s_28_11: i64 = (s_28_10 as i64);
        // D s_28_12: cast zx s_28_8 -> i
        let s_28_12: i128 = (i128::try_from(s_28_8).unwrap());
        // D s_28_13: cast zx s_28_11 -> i
        let s_28_13: i128 = (i128::try_from(s_28_11).unwrap());
        // D s_28_14: read-var operand2:bv
        let s_28_14: Bits = fn_state.operand2;
        // D s_28_15: call Elem_read(s_28_14, s_28_12, s_28_13)
        let s_28_15: Bits = Elem_read(state, tracer, s_28_14, s_28_12, s_28_13);
        // D s_28_16: write-var elt2_i <= s_28_15
        fn_state.elt2_i = s_28_15;
        // D s_28_17: read-var sub_i:u8
        let s_28_17: bool = fn_state.sub_i;
        // N s_28_18: branch s_28_17 b38 b29
        if s_28_17 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_29_0: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var esizeshadow#2295:i64
        let s_30_0: i64 = fn_state.esizeshadow_2295;
        // D s_30_1: cast zx s_30_0 -> i
        let s_30_1: i128 = (i128::try_from(s_30_0).unwrap());
        // D s_30_2: call __id(s_30_1)
        let s_30_2: i128 = u__id(state, tracer, s_30_1);
        // D s_30_3: cast reint s_30_2 -> i64
        let s_30_3: i64 = (s_30_2 as i64);
        // C s_30_4: const #16s : i
        let s_30_4: i128 = 16;
        // D s_30_5: cast zx s_30_3 -> i
        let s_30_5: i128 = (i128::try_from(s_30_3).unwrap());
        // D s_30_6: cmp-eq s_30_5 s_30_4
        let s_30_6: bool = ((s_30_5) == (s_30_4));
        // N s_30_7: branch s_30_6 b37 b31
        if s_30_6 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var esizeshadow#2295:i64
        let s_31_0: i64 = fn_state.esizeshadow_2295;
        // D s_31_1: cast zx s_31_0 -> i
        let s_31_1: i128 = (i128::try_from(s_31_0).unwrap());
        // D s_31_2: call __id(s_31_1)
        let s_31_2: i128 = u__id(state, tracer, s_31_1);
        // D s_31_3: cast reint s_31_2 -> i64
        let s_31_3: i64 = (s_31_2 as i64);
        // C s_31_4: const #32s : i
        let s_31_4: i128 = 32;
        // D s_31_5: cast zx s_31_3 -> i
        let s_31_5: i128 = (i128::try_from(s_31_3).unwrap());
        // D s_31_6: cmp-eq s_31_5 s_31_4
        let s_31_6: bool = ((s_31_5) == (s_31_4));
        // D s_31_7: write-var gs#180905 <= s_31_6
        fn_state.gs_180905 = s_31_6;
        // N s_31_8: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#180905:u8
        let s_32_0: bool = fn_state.gs_180905;
        // N s_32_1: branch s_32_0 b36 b33
        if s_32_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var esizeshadow#2295:i64
        let s_33_0: i64 = fn_state.esizeshadow_2295;
        // D s_33_1: cast zx s_33_0 -> i
        let s_33_1: i128 = (i128::try_from(s_33_0).unwrap());
        // D s_33_2: call __id(s_33_1)
        let s_33_2: i128 = u__id(state, tracer, s_33_1);
        // D s_33_3: cast reint s_33_2 -> i64
        let s_33_3: i64 = (s_33_2 as i64);
        // C s_33_4: const #64s : i
        let s_33_4: i128 = 64;
        // D s_33_5: cast zx s_33_3 -> i
        let s_33_5: i128 = (i128::try_from(s_33_3).unwrap());
        // D s_33_6: cmp-eq s_33_5 s_33_4
        let s_33_6: bool = ((s_33_5) == (s_33_4));
        // D s_33_7: write-var gs#180907 <= s_33_6
        fn_state.gs_180907 = s_33_6;
        // N s_33_8: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#180907:u8
        let s_34_0: bool = fn_state.gs_180907;
        // N s_34_1: assert s_34_0
        let s_34_1: () = assert!(s_34_0);
        // C s_34_2: const #() : ()
        let s_34_2: () = ();
        // S s_34_3: call FPCR_read(s_34_2)
        let s_34_3: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_34_2);
        // D s_34_4: read-var acc_r:bv
        let s_34_4: Bits = fn_state.acc_r;
        // D s_34_5: read-var elt2_i:bv
        let s_34_5: Bits = fn_state.elt2_i;
        // D s_34_6: call FPAdd(s_34_4, s_34_5, s_34_3)
        let s_34_6: Bits = FPAdd(state, tracer, s_34_4, s_34_5, s_34_3);
        // D s_34_7: write-var acc_r <= s_34_6
        fn_state.acc_r = s_34_6;
        // N s_34_8: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_35_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #1u : u8
        let s_36_0: bool = true;
        // D s_36_1: write-var gs#180907 <= s_36_0
        fn_state.gs_180907 = s_36_0;
        // N s_36_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #1u : u8
        let s_37_0: bool = true;
        // D s_37_1: write-var gs#180905 <= s_37_0
        fn_state.gs_180905 = s_37_0;
        // N s_37_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var esizeshadow#2295:i64
        let s_38_0: i64 = fn_state.esizeshadow_2295;
        // D s_38_1: cast zx s_38_0 -> i
        let s_38_1: i128 = (i128::try_from(s_38_0).unwrap());
        // D s_38_2: call __id(s_38_1)
        let s_38_2: i128 = u__id(state, tracer, s_38_1);
        // D s_38_3: cast reint s_38_2 -> i64
        let s_38_3: i64 = (s_38_2 as i64);
        // C s_38_4: const #16s : i
        let s_38_4: i128 = 16;
        // D s_38_5: cast zx s_38_3 -> i
        let s_38_5: i128 = (i128::try_from(s_38_3).unwrap());
        // D s_38_6: cmp-eq s_38_5 s_38_4
        let s_38_6: bool = ((s_38_5) == (s_38_4));
        // N s_38_7: branch s_38_6 b45 b39
        if s_38_6 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var esizeshadow#2295:i64
        let s_39_0: i64 = fn_state.esizeshadow_2295;
        // D s_39_1: cast zx s_39_0 -> i
        let s_39_1: i128 = (i128::try_from(s_39_0).unwrap());
        // D s_39_2: call __id(s_39_1)
        let s_39_2: i128 = u__id(state, tracer, s_39_1);
        // D s_39_3: cast reint s_39_2 -> i64
        let s_39_3: i64 = (s_39_2 as i64);
        // C s_39_4: const #32s : i
        let s_39_4: i128 = 32;
        // D s_39_5: cast zx s_39_3 -> i
        let s_39_5: i128 = (i128::try_from(s_39_3).unwrap());
        // D s_39_6: cmp-eq s_39_5 s_39_4
        let s_39_6: bool = ((s_39_5) == (s_39_4));
        // D s_39_7: write-var gs#180912 <= s_39_6
        fn_state.gs_180912 = s_39_6;
        // N s_39_8: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#180912:u8
        let s_40_0: bool = fn_state.gs_180912;
        // N s_40_1: branch s_40_0 b44 b41
        if s_40_0 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var esizeshadow#2295:i64
        let s_41_0: i64 = fn_state.esizeshadow_2295;
        // D s_41_1: cast zx s_41_0 -> i
        let s_41_1: i128 = (i128::try_from(s_41_0).unwrap());
        // D s_41_2: call __id(s_41_1)
        let s_41_2: i128 = u__id(state, tracer, s_41_1);
        // D s_41_3: cast reint s_41_2 -> i64
        let s_41_3: i64 = (s_41_2 as i64);
        // C s_41_4: const #64s : i
        let s_41_4: i128 = 64;
        // D s_41_5: cast zx s_41_3 -> i
        let s_41_5: i128 = (i128::try_from(s_41_3).unwrap());
        // D s_41_6: cmp-eq s_41_5 s_41_4
        let s_41_6: bool = ((s_41_5) == (s_41_4));
        // D s_41_7: write-var gs#180914 <= s_41_6
        fn_state.gs_180914 = s_41_6;
        // N s_41_8: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#180914:u8
        let s_42_0: bool = fn_state.gs_180914;
        // N s_42_1: assert s_42_0
        let s_42_1: () = assert!(s_42_0);
        // D s_42_2: read-var elt2_i:bv
        let s_42_2: Bits = fn_state.elt2_i;
        // D s_42_3: call FPNeg(s_42_2)
        let s_42_3: Bits = FPNeg(state, tracer, s_42_2);
        // D s_42_4: write-var elt2_i <= s_42_3
        fn_state.elt2_i = s_42_3;
        // N s_42_5: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_43_0: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #1u : u8
        let s_44_0: bool = true;
        // D s_44_1: write-var gs#180914 <= s_44_0
        fn_state.gs_180914 = s_44_0;
        // N s_44_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #1u : u8
        let s_45_0: bool = true;
        // D s_45_1: write-var gs#180912 <= s_45_0
        fn_state.gs_180912 = s_45_0;
        // N s_45_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var VLshadow#2297:i64
        let s_46_0: i64 = fn_state.VLshadow_2297;
        // D s_46_1: cast zx s_46_0 -> i
        let s_46_1: i128 = (i128::try_from(s_46_0).unwrap());
        // D s_46_2: cast reint s_46_1 -> i64
        let s_46_2: i64 = (s_46_1 as i64);
        // D s_46_3: read-var dn:i64
        let s_46_3: i64 = fn_state.dn;
        // D s_46_4: cast zx s_46_3 -> i
        let s_46_4: i128 = (i128::try_from(s_46_3).unwrap());
        // D s_46_5: cast zx s_46_2 -> i
        let s_46_5: i128 = (i128::try_from(s_46_2).unwrap());
        // D s_46_6: read-var result:bv
        let s_46_6: Bits = fn_state.result;
        // D s_46_7: call Z_set(s_46_4, s_46_5, s_46_6)
        let s_46_7: () = Z_set(state, tracer, s_46_4, s_46_5, s_46_6);
        // N s_46_8: return
        return;
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var VLshadow#2297:i64
        let s_47_0: i64 = fn_state.VLshadow_2297;
        // D s_47_1: cast zx s_47_0 -> i
        let s_47_1: i128 = (i128::try_from(s_47_0).unwrap());
        // D s_47_2: cast reint s_47_1 -> i64
        let s_47_2: i64 = (s_47_1 as i64);
        // D s_47_3: read-var m:i64
        let s_47_3: i64 = fn_state.m;
        // D s_47_4: cast zx s_47_3 -> i
        let s_47_4: i128 = (i128::try_from(s_47_3).unwrap());
        // D s_47_5: cast zx s_47_2 -> i
        let s_47_5: i128 = (i128::try_from(s_47_2).unwrap());
        // D s_47_6: call Z_read(s_47_4, s_47_5)
        let s_47_6: Bits = Z_read(state, tracer, s_47_4, s_47_5);
        // D s_47_7: write-var operand2 <= s_47_6
        fn_state.operand2 = s_47_6;
        // N s_47_8: jump b3
        return block_3(state, tracer, fn_state);
    }
}
