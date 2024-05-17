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
pub fn execute_CDOT_Z_ZZZi_D<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    da: i64,
    esize: i64,
    index: i64,
    m: i64,
    n: i64,
    sel_a: i64,
    sel_b: i64,
    sub_i: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        elt1_i: i64,
        elt2_a: i64,
        i: i64,
        operand1: Bits,
        eltspersegment: i64,
        operand2: Bits,
        e: i64,
        VLshadow_4139: i64,
        operand3: Bits,
        elt2_b: i64,
        res: Bits,
        elt1_r: i64,
        VLshadow_4138: i64,
        s: i64,
        gs_216798: i64,
        result: Bits,
        VL: i64,
        da: i64,
        esize: i64,
        index: i64,
        m: i64,
        n: i64,
        sel_a: i64,
        sel_b: i64,
        sub_i: bool,
    }
    let fn_state = FunctionState {
        VL,
        da,
        esize,
        index,
        m,
        n,
        sel_a,
        sel_b,
        sub_i,
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
        // D s_0_3: write-var VLshadow#4138 <= s_0_2
        fn_state.VLshadow_4138 = s_0_2;
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
        // D s_1_0: read-var VLshadow#4138:i64
        let s_1_0: i64 = fn_state.VLshadow_4138;
        // D s_1_1: write-var VLshadow#4139 <= s_1_0
        fn_state.VLshadow_4139 = s_1_0;
        // D s_1_2: read-var VLshadow#4139:i64
        let s_1_2: i64 = fn_state.VLshadow_4139;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: read-var esize:i64
        let s_1_4: i64 = fn_state.esize;
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: div s_1_3 s_1_5
        let s_1_6: i128 = ((s_1_3) / (s_1_5));
        // D s_1_7: cast reint s_1_6 -> i64
        let s_1_7: i64 = (s_1_6 as i64);
        // C s_1_8: const #128s : i
        let s_1_8: i128 = 128;
        // D s_1_9: read-var esize:i64
        let s_1_9: i64 = fn_state.esize;
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_11: div s_1_8 s_1_10
        let s_1_11: i128 = ((s_1_8) / (s_1_10));
        // D s_1_12: cast reint s_1_11 -> i64
        let s_1_12: i64 = (s_1_11 as i64);
        // D s_1_13: write-var eltspersegment <= s_1_12
        fn_state.eltspersegment = s_1_12;
        // D s_1_14: read-var VLshadow#4139:i64
        let s_1_14: i64 = fn_state.VLshadow_4139;
        // D s_1_15: cast zx s_1_14 -> i
        let s_1_15: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_16: cast reint s_1_15 -> i64
        let s_1_16: i64 = (s_1_15 as i64);
        // D s_1_17: read-var n:i64
        let s_1_17: i64 = fn_state.n;
        // D s_1_18: cast zx s_1_17 -> i
        let s_1_18: i128 = (i128::try_from(s_1_17).unwrap());
        // D s_1_19: cast zx s_1_16 -> i
        let s_1_19: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_20: call Z_read(s_1_18, s_1_19)
        let s_1_20: Bits = Z_read(state, tracer, s_1_18, s_1_19);
        // D s_1_21: write-var operand1 <= s_1_20
        fn_state.operand1 = s_1_20;
        // D s_1_22: read-var VLshadow#4139:i64
        let s_1_22: i64 = fn_state.VLshadow_4139;
        // D s_1_23: cast zx s_1_22 -> i
        let s_1_23: i128 = (i128::try_from(s_1_22).unwrap());
        // D s_1_24: cast reint s_1_23 -> i64
        let s_1_24: i64 = (s_1_23 as i64);
        // D s_1_25: read-var m:i64
        let s_1_25: i64 = fn_state.m;
        // D s_1_26: cast zx s_1_25 -> i
        let s_1_26: i128 = (i128::try_from(s_1_25).unwrap());
        // D s_1_27: cast zx s_1_24 -> i
        let s_1_27: i128 = (i128::try_from(s_1_24).unwrap());
        // D s_1_28: call Z_read(s_1_26, s_1_27)
        let s_1_28: Bits = Z_read(state, tracer, s_1_26, s_1_27);
        // D s_1_29: write-var operand2 <= s_1_28
        fn_state.operand2 = s_1_28;
        // D s_1_30: read-var VLshadow#4139:i64
        let s_1_30: i64 = fn_state.VLshadow_4139;
        // D s_1_31: cast zx s_1_30 -> i
        let s_1_31: i128 = (i128::try_from(s_1_30).unwrap());
        // D s_1_32: cast reint s_1_31 -> i64
        let s_1_32: i64 = (s_1_31 as i64);
        // D s_1_33: read-var da:i64
        let s_1_33: i64 = fn_state.da;
        // D s_1_34: cast zx s_1_33 -> i
        let s_1_34: i128 = (i128::try_from(s_1_33).unwrap());
        // D s_1_35: cast zx s_1_32 -> i
        let s_1_35: i128 = (i128::try_from(s_1_32).unwrap());
        // D s_1_36: call Z_read(s_1_34, s_1_35)
        let s_1_36: Bits = Z_read(state, tracer, s_1_34, s_1_35);
        // D s_1_37: write-var operand3 <= s_1_36
        fn_state.operand3 = s_1_36;
        // C s_1_38: const #0s : i64
        let s_1_38: i64 = 0;
        // C s_1_39: const #1s : i
        let s_1_39: i128 = 1;
        // D s_1_40: cast zx s_1_7 -> i
        let s_1_40: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_41: sub s_1_40 s_1_39
        let s_1_41: i128 = ((s_1_40) - (s_1_39));
        // D s_1_42: cast reint s_1_41 -> i64
        let s_1_42: i64 = (s_1_41 as i64);
        // D s_1_43: write-var gs#216798 <= s_1_42
        fn_state.gs_216798 = s_1_42;
        // D s_1_44: write-var e <= s_1_38
        fn_state.e = s_1_38;
        // N s_1_45: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#216798:i64
        let s_2_1: i64 = fn_state.gs_216798;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b10 b3
        if s_2_2 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var e:i64
        let s_3_0: i64 = fn_state.e;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var eltspersegment:i64
        let s_3_2: i64 = fn_state.eltspersegment;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: mod s_3_1 s_3_3
        let s_3_4: i128 = ((s_3_1) % (s_3_3));
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: read-var e:i64
        let s_3_6: i64 = fn_state.e;
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_8: cast zx s_3_5 -> i
        let s_3_8: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_9: sub s_3_7 s_3_8
        let s_3_9: i128 = ((s_3_7) - (s_3_8));
        // D s_3_10: cast reint s_3_9 -> i64
        let s_3_10: i64 = (s_3_9 as i64);
        // D s_3_11: cast zx s_3_10 -> i
        let s_3_11: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_12: read-var index:i64
        let s_3_12: i64 = fn_state.index;
        // D s_3_13: cast zx s_3_12 -> i
        let s_3_13: i128 = (i128::try_from(s_3_12).unwrap());
        // D s_3_14: add s_3_11 s_3_13
        let s_3_14: i128 = (s_3_11 + s_3_13);
        // D s_3_15: cast reint s_3_14 -> i64
        let s_3_15: i64 = (s_3_14 as i64);
        // D s_3_16: write-var s <= s_3_15
        fn_state.s = s_3_15;
        // D s_3_17: read-var esize:i64
        let s_3_17: i64 = fn_state.esize;
        // D s_3_18: cast zx s_3_17 -> i
        let s_3_18: i128 = (i128::try_from(s_3_17).unwrap());
        // D s_3_19: cast reint s_3_18 -> i64
        let s_3_19: i64 = (s_3_18 as i64);
        // D s_3_20: read-var e:i64
        let s_3_20: i64 = fn_state.e;
        // D s_3_21: cast zx s_3_20 -> i
        let s_3_21: i128 = (i128::try_from(s_3_20).unwrap());
        // D s_3_22: cast zx s_3_19 -> i
        let s_3_22: i128 = (i128::try_from(s_3_19).unwrap());
        // D s_3_23: read-var operand3:bv
        let s_3_23: Bits = fn_state.operand3;
        // D s_3_24: call Elem_read(s_3_23, s_3_21, s_3_22)
        let s_3_24: Bits = Elem_read(state, tracer, s_3_23, s_3_21, s_3_22);
        // D s_3_25: write-var res <= s_3_24
        fn_state.res = s_3_24;
        // C s_3_26: const #0s : i64
        let s_3_26: i64 = 0;
        // D s_3_27: write-var i <= s_3_26
        fn_state.i = s_3_26;
        // N s_3_28: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var i:i64
        let s_4_0: i64 = fn_state.i;
        // C s_4_1: const #1s : i64
        let s_4_1: i64 = 1;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b9 b5
        if s_4_2 {
            return block_9(state, tracer, fn_state);
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
        // C s_5_5: const #2s : i
        let s_5_5: i128 = 2;
        // D s_5_6: read-var i:i64
        let s_5_6: i64 = fn_state.i;
        // D s_5_7: cast zx s_5_6 -> i
        let s_5_7: i128 = (i128::try_from(s_5_6).unwrap());
        // D s_5_8: mul s_5_5 s_5_7
        let s_5_8: i128 = ((s_5_5) * (s_5_7));
        // D s_5_9: cast reint s_5_8 -> i64
        let s_5_9: i64 = (s_5_8 as i64);
        // D s_5_10: cast zx s_5_4 -> i
        let s_5_10: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_11: cast zx s_5_9 -> i
        let s_5_11: i128 = (i128::try_from(s_5_9).unwrap());
        // D s_5_12: add s_5_10 s_5_11
        let s_5_12: i128 = (s_5_10 + s_5_11);
        // D s_5_13: cast reint s_5_12 -> i64
        let s_5_13: i64 = (s_5_12 as i64);
        // C s_5_14: const #0s : i
        let s_5_14: i128 = 0;
        // D s_5_15: cast zx s_5_13 -> i
        let s_5_15: i128 = (i128::try_from(s_5_13).unwrap());
        // D s_5_16: add s_5_15 s_5_14
        let s_5_16: i128 = (s_5_15 + s_5_14);
        // D s_5_17: cast reint s_5_16 -> i64
        let s_5_17: i64 = (s_5_16 as i64);
        // C s_5_18: const #4s : i
        let s_5_18: i128 = 4;
        // D s_5_19: read-var esize:i64
        let s_5_19: i64 = fn_state.esize;
        // D s_5_20: cast zx s_5_19 -> i
        let s_5_20: i128 = (i128::try_from(s_5_19).unwrap());
        // D s_5_21: div s_5_20 s_5_18
        let s_5_21: i128 = ((s_5_20) / (s_5_18));
        // D s_5_22: cast reint s_5_21 -> i64
        let s_5_22: i64 = (s_5_21 as i64);
        // D s_5_23: cast zx s_5_22 -> i
        let s_5_23: i128 = (i128::try_from(s_5_22).unwrap());
        // D s_5_24: cast reint s_5_23 -> i64
        let s_5_24: i64 = (s_5_23 as i64);
        // D s_5_25: cast zx s_5_17 -> i
        let s_5_25: i128 = (i128::try_from(s_5_17).unwrap());
        // D s_5_26: cast zx s_5_24 -> i
        let s_5_26: i128 = (i128::try_from(s_5_24).unwrap());
        // D s_5_27: read-var operand1:bv
        let s_5_27: Bits = fn_state.operand1;
        // D s_5_28: call Elem_read(s_5_27, s_5_25, s_5_26)
        let s_5_28: Bits = Elem_read(state, tracer, s_5_27, s_5_25, s_5_26);
        // D s_5_29: cast reint s_5_28 -> u16
        let s_5_29: u16 = (s_5_28.value() as u16);
        // D s_5_30: cast zx s_5_29 -> bv
        let s_5_30: Bits = Bits::new(s_5_29 as u128, 16u16);
        // D s_5_31: cast sx s_5_30 -> i
        let s_5_31: i128 = {
            let sign_bit = s_5_30.length() - 1;
            let mut result = s_5_30.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_5_32: cast reint s_5_31 -> i64
        let s_5_32: i64 = (s_5_31 as i64);
        // D s_5_33: write-var elt1_r <= s_5_32
        fn_state.elt1_r = s_5_32;
        // C s_5_34: const #4s : i
        let s_5_34: i128 = 4;
        // D s_5_35: read-var e:i64
        let s_5_35: i64 = fn_state.e;
        // D s_5_36: cast zx s_5_35 -> i
        let s_5_36: i128 = (i128::try_from(s_5_35).unwrap());
        // D s_5_37: mul s_5_34 s_5_36
        let s_5_37: i128 = ((s_5_34) * (s_5_36));
        // D s_5_38: cast reint s_5_37 -> i64
        let s_5_38: i64 = (s_5_37 as i64);
        // C s_5_39: const #2s : i
        let s_5_39: i128 = 2;
        // D s_5_40: read-var i:i64
        let s_5_40: i64 = fn_state.i;
        // D s_5_41: cast zx s_5_40 -> i
        let s_5_41: i128 = (i128::try_from(s_5_40).unwrap());
        // D s_5_42: mul s_5_39 s_5_41
        let s_5_42: i128 = ((s_5_39) * (s_5_41));
        // D s_5_43: cast reint s_5_42 -> i64
        let s_5_43: i64 = (s_5_42 as i64);
        // D s_5_44: cast zx s_5_38 -> i
        let s_5_44: i128 = (i128::try_from(s_5_38).unwrap());
        // D s_5_45: cast zx s_5_43 -> i
        let s_5_45: i128 = (i128::try_from(s_5_43).unwrap());
        // D s_5_46: add s_5_44 s_5_45
        let s_5_46: i128 = (s_5_44 + s_5_45);
        // D s_5_47: cast reint s_5_46 -> i64
        let s_5_47: i64 = (s_5_46 as i64);
        // C s_5_48: const #1s : i
        let s_5_48: i128 = 1;
        // D s_5_49: cast zx s_5_47 -> i
        let s_5_49: i128 = (i128::try_from(s_5_47).unwrap());
        // D s_5_50: add s_5_49 s_5_48
        let s_5_50: i128 = (s_5_49 + s_5_48);
        // D s_5_51: cast reint s_5_50 -> i64
        let s_5_51: i64 = (s_5_50 as i64);
        // C s_5_52: const #4s : i
        let s_5_52: i128 = 4;
        // D s_5_53: read-var esize:i64
        let s_5_53: i64 = fn_state.esize;
        // D s_5_54: cast zx s_5_53 -> i
        let s_5_54: i128 = (i128::try_from(s_5_53).unwrap());
        // D s_5_55: div s_5_54 s_5_52
        let s_5_55: i128 = ((s_5_54) / (s_5_52));
        // D s_5_56: cast reint s_5_55 -> i64
        let s_5_56: i64 = (s_5_55 as i64);
        // D s_5_57: cast zx s_5_56 -> i
        let s_5_57: i128 = (i128::try_from(s_5_56).unwrap());
        // D s_5_58: cast reint s_5_57 -> i64
        let s_5_58: i64 = (s_5_57 as i64);
        // D s_5_59: cast zx s_5_51 -> i
        let s_5_59: i128 = (i128::try_from(s_5_51).unwrap());
        // D s_5_60: cast zx s_5_58 -> i
        let s_5_60: i128 = (i128::try_from(s_5_58).unwrap());
        // D s_5_61: read-var operand1:bv
        let s_5_61: Bits = fn_state.operand1;
        // D s_5_62: call Elem_read(s_5_61, s_5_59, s_5_60)
        let s_5_62: Bits = Elem_read(state, tracer, s_5_61, s_5_59, s_5_60);
        // D s_5_63: cast reint s_5_62 -> u16
        let s_5_63: u16 = (s_5_62.value() as u16);
        // D s_5_64: cast zx s_5_63 -> bv
        let s_5_64: Bits = Bits::new(s_5_63 as u128, 16u16);
        // D s_5_65: cast sx s_5_64 -> i
        let s_5_65: i128 = {
            let sign_bit = s_5_64.length() - 1;
            let mut result = s_5_64.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_5_66: cast reint s_5_65 -> i64
        let s_5_66: i64 = (s_5_65 as i64);
        // D s_5_67: write-var elt1_i <= s_5_66
        fn_state.elt1_i = s_5_66;
        // C s_5_68: const #4s : i
        let s_5_68: i128 = 4;
        // D s_5_69: read-var s:i64
        let s_5_69: i64 = fn_state.s;
        // D s_5_70: cast zx s_5_69 -> i
        let s_5_70: i128 = (i128::try_from(s_5_69).unwrap());
        // D s_5_71: mul s_5_68 s_5_70
        let s_5_71: i128 = ((s_5_68) * (s_5_70));
        // D s_5_72: cast reint s_5_71 -> i64
        let s_5_72: i64 = (s_5_71 as i64);
        // C s_5_73: const #2s : i
        let s_5_73: i128 = 2;
        // D s_5_74: read-var i:i64
        let s_5_74: i64 = fn_state.i;
        // D s_5_75: cast zx s_5_74 -> i
        let s_5_75: i128 = (i128::try_from(s_5_74).unwrap());
        // D s_5_76: mul s_5_73 s_5_75
        let s_5_76: i128 = ((s_5_73) * (s_5_75));
        // D s_5_77: cast reint s_5_76 -> i64
        let s_5_77: i64 = (s_5_76 as i64);
        // D s_5_78: cast zx s_5_72 -> i
        let s_5_78: i128 = (i128::try_from(s_5_72).unwrap());
        // D s_5_79: cast zx s_5_77 -> i
        let s_5_79: i128 = (i128::try_from(s_5_77).unwrap());
        // D s_5_80: add s_5_78 s_5_79
        let s_5_80: i128 = (s_5_78 + s_5_79);
        // D s_5_81: cast reint s_5_80 -> i64
        let s_5_81: i64 = (s_5_80 as i64);
        // D s_5_82: cast zx s_5_81 -> i
        let s_5_82: i128 = (i128::try_from(s_5_81).unwrap());
        // D s_5_83: read-var sel_a:i64
        let s_5_83: i64 = fn_state.sel_a;
        // D s_5_84: cast zx s_5_83 -> i
        let s_5_84: i128 = (i128::try_from(s_5_83).unwrap());
        // D s_5_85: add s_5_82 s_5_84
        let s_5_85: i128 = (s_5_82 + s_5_84);
        // D s_5_86: cast reint s_5_85 -> i64
        let s_5_86: i64 = (s_5_85 as i64);
        // C s_5_87: const #4s : i
        let s_5_87: i128 = 4;
        // D s_5_88: read-var esize:i64
        let s_5_88: i64 = fn_state.esize;
        // D s_5_89: cast zx s_5_88 -> i
        let s_5_89: i128 = (i128::try_from(s_5_88).unwrap());
        // D s_5_90: div s_5_89 s_5_87
        let s_5_90: i128 = ((s_5_89) / (s_5_87));
        // D s_5_91: cast reint s_5_90 -> i64
        let s_5_91: i64 = (s_5_90 as i64);
        // D s_5_92: cast zx s_5_91 -> i
        let s_5_92: i128 = (i128::try_from(s_5_91).unwrap());
        // D s_5_93: cast reint s_5_92 -> i64
        let s_5_93: i64 = (s_5_92 as i64);
        // D s_5_94: cast zx s_5_86 -> i
        let s_5_94: i128 = (i128::try_from(s_5_86).unwrap());
        // D s_5_95: cast zx s_5_93 -> i
        let s_5_95: i128 = (i128::try_from(s_5_93).unwrap());
        // D s_5_96: read-var operand2:bv
        let s_5_96: Bits = fn_state.operand2;
        // D s_5_97: call Elem_read(s_5_96, s_5_94, s_5_95)
        let s_5_97: Bits = Elem_read(state, tracer, s_5_96, s_5_94, s_5_95);
        // D s_5_98: cast reint s_5_97 -> u16
        let s_5_98: u16 = (s_5_97.value() as u16);
        // D s_5_99: cast zx s_5_98 -> bv
        let s_5_99: Bits = Bits::new(s_5_98 as u128, 16u16);
        // D s_5_100: cast sx s_5_99 -> i
        let s_5_100: i128 = {
            let sign_bit = s_5_99.length() - 1;
            let mut result = s_5_99.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_5_101: cast reint s_5_100 -> i64
        let s_5_101: i64 = (s_5_100 as i64);
        // D s_5_102: write-var elt2_a <= s_5_101
        fn_state.elt2_a = s_5_101;
        // C s_5_103: const #4s : i
        let s_5_103: i128 = 4;
        // D s_5_104: read-var s:i64
        let s_5_104: i64 = fn_state.s;
        // D s_5_105: cast zx s_5_104 -> i
        let s_5_105: i128 = (i128::try_from(s_5_104).unwrap());
        // D s_5_106: mul s_5_103 s_5_105
        let s_5_106: i128 = ((s_5_103) * (s_5_105));
        // D s_5_107: cast reint s_5_106 -> i64
        let s_5_107: i64 = (s_5_106 as i64);
        // C s_5_108: const #2s : i
        let s_5_108: i128 = 2;
        // D s_5_109: read-var i:i64
        let s_5_109: i64 = fn_state.i;
        // D s_5_110: cast zx s_5_109 -> i
        let s_5_110: i128 = (i128::try_from(s_5_109).unwrap());
        // D s_5_111: mul s_5_108 s_5_110
        let s_5_111: i128 = ((s_5_108) * (s_5_110));
        // D s_5_112: cast reint s_5_111 -> i64
        let s_5_112: i64 = (s_5_111 as i64);
        // D s_5_113: cast zx s_5_107 -> i
        let s_5_113: i128 = (i128::try_from(s_5_107).unwrap());
        // D s_5_114: cast zx s_5_112 -> i
        let s_5_114: i128 = (i128::try_from(s_5_112).unwrap());
        // D s_5_115: add s_5_113 s_5_114
        let s_5_115: i128 = (s_5_113 + s_5_114);
        // D s_5_116: cast reint s_5_115 -> i64
        let s_5_116: i64 = (s_5_115 as i64);
        // D s_5_117: cast zx s_5_116 -> i
        let s_5_117: i128 = (i128::try_from(s_5_116).unwrap());
        // D s_5_118: read-var sel_b:i64
        let s_5_118: i64 = fn_state.sel_b;
        // D s_5_119: cast zx s_5_118 -> i
        let s_5_119: i128 = (i128::try_from(s_5_118).unwrap());
        // D s_5_120: add s_5_117 s_5_119
        let s_5_120: i128 = (s_5_117 + s_5_119);
        // D s_5_121: cast reint s_5_120 -> i64
        let s_5_121: i64 = (s_5_120 as i64);
        // C s_5_122: const #4s : i
        let s_5_122: i128 = 4;
        // D s_5_123: read-var esize:i64
        let s_5_123: i64 = fn_state.esize;
        // D s_5_124: cast zx s_5_123 -> i
        let s_5_124: i128 = (i128::try_from(s_5_123).unwrap());
        // D s_5_125: div s_5_124 s_5_122
        let s_5_125: i128 = ((s_5_124) / (s_5_122));
        // D s_5_126: cast reint s_5_125 -> i64
        let s_5_126: i64 = (s_5_125 as i64);
        // D s_5_127: cast zx s_5_126 -> i
        let s_5_127: i128 = (i128::try_from(s_5_126).unwrap());
        // D s_5_128: cast reint s_5_127 -> i64
        let s_5_128: i64 = (s_5_127 as i64);
        // D s_5_129: cast zx s_5_121 -> i
        let s_5_129: i128 = (i128::try_from(s_5_121).unwrap());
        // D s_5_130: cast zx s_5_128 -> i
        let s_5_130: i128 = (i128::try_from(s_5_128).unwrap());
        // D s_5_131: read-var operand2:bv
        let s_5_131: Bits = fn_state.operand2;
        // D s_5_132: call Elem_read(s_5_131, s_5_129, s_5_130)
        let s_5_132: Bits = Elem_read(state, tracer, s_5_131, s_5_129, s_5_130);
        // D s_5_133: cast reint s_5_132 -> u16
        let s_5_133: u16 = (s_5_132.value() as u16);
        // D s_5_134: cast zx s_5_133 -> bv
        let s_5_134: Bits = Bits::new(s_5_133 as u128, 16u16);
        // D s_5_135: cast sx s_5_134 -> i
        let s_5_135: i128 = {
            let sign_bit = s_5_134.length() - 1;
            let mut result = s_5_134.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_5_136: cast reint s_5_135 -> i64
        let s_5_136: i64 = (s_5_135 as i64);
        // D s_5_137: write-var elt2_b <= s_5_136
        fn_state.elt2_b = s_5_136;
        // D s_5_138: read-var sub_i:u8
        let s_5_138: bool = fn_state.sub_i;
        // N s_5_139: branch s_5_138 b8 b6
        if s_5_138 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var elt1_r:i64
        let s_6_0: i64 = fn_state.elt1_r;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: read-var elt2_a:i64
        let s_6_2: i64 = fn_state.elt2_a;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: mul s_6_1 s_6_3
        let s_6_4: i128 = ((s_6_1) * (s_6_3));
        // D s_6_5: cast reint s_6_4 -> i64
        let s_6_5: i64 = (s_6_4 as i64);
        // D s_6_6: cast zx s_6_5 -> i
        let s_6_6: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_7: read-var res:bv
        let s_6_7: Bits = fn_state.res;
        // D s_6_8: cast cvt s_6_6 -> bv
        let s_6_8: Bits = Bits::new(s_6_6 as u128, 128);
        // D s_6_9: add s_6_7 s_6_8
        let s_6_9: Bits = (s_6_7 + s_6_8);
        // D s_6_10: read-var elt1_i:i64
        let s_6_10: i64 = fn_state.elt1_i;
        // D s_6_11: cast zx s_6_10 -> i
        let s_6_11: i128 = (i128::try_from(s_6_10).unwrap());
        // D s_6_12: read-var elt2_b:i64
        let s_6_12: i64 = fn_state.elt2_b;
        // D s_6_13: cast zx s_6_12 -> i
        let s_6_13: i128 = (i128::try_from(s_6_12).unwrap());
        // D s_6_14: mul s_6_11 s_6_13
        let s_6_14: i128 = ((s_6_11) * (s_6_13));
        // D s_6_15: cast reint s_6_14 -> i64
        let s_6_15: i64 = (s_6_14 as i64);
        // D s_6_16: cast zx s_6_15 -> i
        let s_6_16: i128 = (i128::try_from(s_6_15).unwrap());
        // D s_6_17: cast cvt s_6_16 -> bv
        let s_6_17: Bits = Bits::new(s_6_16 as u128, 128);
        // D s_6_18: add s_6_9 s_6_17
        let s_6_18: Bits = (s_6_9 + s_6_17);
        // D s_6_19: write-var res <= s_6_18
        fn_state.res = s_6_18;
        // N s_6_20: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var i:i64
        let s_7_0: i64 = fn_state.i;
        // C s_7_1: const #1s : i64
        let s_7_1: i64 = 1;
        // D s_7_2: add s_7_0 s_7_1
        let s_7_2: i64 = (s_7_0 + s_7_1);
        // D s_7_3: write-var i <= s_7_2
        fn_state.i = s_7_2;
        // N s_7_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var elt1_r:i64
        let s_8_0: i64 = fn_state.elt1_r;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: read-var elt2_a:i64
        let s_8_2: i64 = fn_state.elt2_a;
        // D s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_4: mul s_8_1 s_8_3
        let s_8_4: i128 = ((s_8_1) * (s_8_3));
        // D s_8_5: cast reint s_8_4 -> i64
        let s_8_5: i64 = (s_8_4 as i64);
        // D s_8_6: cast zx s_8_5 -> i
        let s_8_6: i128 = (i128::try_from(s_8_5).unwrap());
        // D s_8_7: read-var res:bv
        let s_8_7: Bits = fn_state.res;
        // D s_8_8: cast cvt s_8_6 -> bv
        let s_8_8: Bits = Bits::new(s_8_6 as u128, 128);
        // D s_8_9: add s_8_7 s_8_8
        let s_8_9: Bits = (s_8_7 + s_8_8);
        // D s_8_10: read-var elt1_i:i64
        let s_8_10: i64 = fn_state.elt1_i;
        // D s_8_11: cast zx s_8_10 -> i
        let s_8_11: i128 = (i128::try_from(s_8_10).unwrap());
        // D s_8_12: read-var elt2_b:i64
        let s_8_12: i64 = fn_state.elt2_b;
        // D s_8_13: cast zx s_8_12 -> i
        let s_8_13: i128 = (i128::try_from(s_8_12).unwrap());
        // D s_8_14: mul s_8_11 s_8_13
        let s_8_14: i128 = ((s_8_11) * (s_8_13));
        // D s_8_15: cast reint s_8_14 -> i64
        let s_8_15: i64 = (s_8_14 as i64);
        // D s_8_16: cast zx s_8_15 -> i
        let s_8_16: i128 = (i128::try_from(s_8_15).unwrap());
        // D s_8_17: cast cvt s_8_16 -> bv
        let s_8_17: Bits = Bits::new(s_8_16 as u128, 128);
        // D s_8_18: sub s_8_9 s_8_17
        let s_8_18: Bits = ((s_8_9) - (s_8_17));
        // D s_8_19: write-var res <= s_8_18
        fn_state.res = s_8_18;
        // N s_8_20: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var esize:i64
        let s_9_0: i64 = fn_state.esize;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: cast reint s_9_1 -> i64
        let s_9_2: i64 = (s_9_1 as i64);
        // D s_9_3: read-var e:i64
        let s_9_3: i64 = fn_state.e;
        // D s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_5: cast zx s_9_2 -> i
        let s_9_5: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_6: read-var result:bv
        let s_9_6: Bits = fn_state.result;
        // D s_9_7: read-var res:bv
        let s_9_7: Bits = fn_state.res;
        // D s_9_8: call Elem_set(s_9_6, s_9_4, s_9_5, s_9_7)
        let s_9_8: Bits = Elem_set(state, tracer, s_9_6, s_9_4, s_9_5, s_9_7);
        // D s_9_9: write-var result <= s_9_8
        fn_state.result = s_9_8;
        // D s_9_10: read-var e:i64
        let s_9_10: i64 = fn_state.e;
        // C s_9_11: const #1s : i64
        let s_9_11: i64 = 1;
        // D s_9_12: add s_9_10 s_9_11
        let s_9_12: i64 = (s_9_10 + s_9_11);
        // D s_9_13: write-var e <= s_9_12
        fn_state.e = s_9_12;
        // N s_9_14: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var VLshadow#4139:i64
        let s_10_0: i64 = fn_state.VLshadow_4139;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: cast reint s_10_1 -> i64
        let s_10_2: i64 = (s_10_1 as i64);
        // D s_10_3: read-var da:i64
        let s_10_3: i64 = fn_state.da;
        // D s_10_4: cast zx s_10_3 -> i
        let s_10_4: i128 = (i128::try_from(s_10_3).unwrap());
        // D s_10_5: cast zx s_10_2 -> i
        let s_10_5: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_6: read-var result:bv
        let s_10_6: Bits = fn_state.result;
        // D s_10_7: call Z_set(s_10_4, s_10_5, s_10_6)
        let s_10_7: () = Z_set(state, tracer, s_10_4, s_10_5, s_10_6);
        // N s_10_8: return
        return;
    }
}
