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
use Zeros::*;
use CheckSVEEnabled::*;
use Z_read::*;
use Elem_read::*;
use Z_set::*;
use common::*;
pub fn execute_ZIP2_Z_ZZ__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    m: i64,
    n: i64,
    part: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        p: i64,
        base: i64,
        gs_195530: i64,
        esizeshadow_2916: i64,
        result: Bits,
        operand1: Bits,
        VLshadow_2918: i64,
        VLshadow_2917: i64,
        operand2: Bits,
        VL: i64,
        d: i64,
        esize: i64,
        m: i64,
        n: i64,
        part: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        m,
        n,
        part,
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
        // D s_0_3: write-var esizeshadow#2916 <= s_0_2
        fn_state.esizeshadow_2916 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#2917 <= s_0_6
        fn_state.VLshadow_2917 = s_0_6;
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
        // D s_1_0: read-var VLshadow#2917:i64
        let s_1_0: i64 = fn_state.VLshadow_2917;
        // D s_1_1: write-var VLshadow#2918 <= s_1_0
        fn_state.VLshadow_2918 = s_1_0;
        // C s_1_2: const #2s : i
        let s_1_2: i128 = 2;
        // D s_1_3: read-var esizeshadow#2916:i64
        let s_1_3: i64 = fn_state.esizeshadow_2916;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: mul s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) * (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#2918:i64
        let s_1_7: i64 = fn_state.VLshadow_2918;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast zx s_1_6 -> i
        let s_1_9: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_10: div s_1_8 s_1_9
        let s_1_10: i128 = ((s_1_8) / (s_1_9));
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: read-var VLshadow#2918:i64
        let s_1_12: i64 = fn_state.VLshadow_2918;
        // D s_1_13: cast zx s_1_12 -> i
        let s_1_13: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_14: cast reint s_1_13 -> i64
        let s_1_14: i64 = (s_1_13 as i64);
        // D s_1_15: read-var n:i64
        let s_1_15: i64 = fn_state.n;
        // D s_1_16: cast zx s_1_15 -> i
        let s_1_16: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_17: cast zx s_1_14 -> i
        let s_1_17: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_18: call Z_read(s_1_16, s_1_17)
        let s_1_18: Bits = Z_read(state, tracer, s_1_16, s_1_17);
        // D s_1_19: write-var operand1 <= s_1_18
        fn_state.operand1 = s_1_18;
        // D s_1_20: read-var VLshadow#2918:i64
        let s_1_20: i64 = fn_state.VLshadow_2918;
        // D s_1_21: cast zx s_1_20 -> i
        let s_1_21: i128 = (i128::try_from(s_1_20).unwrap());
        // D s_1_22: cast reint s_1_21 -> i64
        let s_1_22: i64 = (s_1_21 as i64);
        // D s_1_23: read-var m:i64
        let s_1_23: i64 = fn_state.m;
        // D s_1_24: cast zx s_1_23 -> i
        let s_1_24: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_25: cast zx s_1_22 -> i
        let s_1_25: i128 = (i128::try_from(s_1_22).unwrap());
        // D s_1_26: call Z_read(s_1_24, s_1_25)
        let s_1_26: Bits = Z_read(state, tracer, s_1_24, s_1_25);
        // D s_1_27: write-var operand2 <= s_1_26
        fn_state.operand2 = s_1_26;
        // D s_1_28: read-var VLshadow#2918:i64
        let s_1_28: i64 = fn_state.VLshadow_2918;
        // D s_1_29: cast zx s_1_28 -> i
        let s_1_29: i128 = (i128::try_from(s_1_28).unwrap());
        // D s_1_30: call Zeros(s_1_29)
        let s_1_30: Bits = Zeros(state, tracer, s_1_29);
        // D s_1_31: write-var result <= s_1_30
        fn_state.result = s_1_30;
        // D s_1_32: read-var part:i64
        let s_1_32: i64 = fn_state.part;
        // D s_1_33: cast zx s_1_32 -> i
        let s_1_33: i128 = (i128::try_from(s_1_32).unwrap());
        // D s_1_34: cast zx s_1_11 -> i
        let s_1_34: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_35: mul s_1_33 s_1_34
        let s_1_35: i128 = ((s_1_33) * (s_1_34));
        // D s_1_36: cast reint s_1_35 -> i64
        let s_1_36: i64 = (s_1_35 as i64);
        // D s_1_37: write-var base <= s_1_36
        fn_state.base = s_1_36;
        // C s_1_38: const #0s : i64
        let s_1_38: i64 = 0;
        // C s_1_39: const #1s : i
        let s_1_39: i128 = 1;
        // D s_1_40: cast zx s_1_11 -> i
        let s_1_40: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_41: sub s_1_40 s_1_39
        let s_1_41: i128 = ((s_1_40) - (s_1_39));
        // D s_1_42: cast reint s_1_41 -> i64
        let s_1_42: i64 = (s_1_41 as i64);
        // D s_1_43: write-var gs#195530 <= s_1_42
        fn_state.gs_195530 = s_1_42;
        // D s_1_44: write-var p <= s_1_38
        fn_state.p = s_1_38;
        // N s_1_45: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var p:i64
        let s_2_0: i64 = fn_state.p;
        // D s_2_1: read-var gs#195530:i64
        let s_2_1: i64 = fn_state.gs_195530;
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
        // C s_3_0: const #2s : i
        let s_3_0: i128 = 2;
        // D s_3_1: read-var p:i64
        let s_3_1: i64 = fn_state.p;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: mul s_3_0 s_3_2
        let s_3_3: i128 = ((s_3_0) * (s_3_2));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // C s_3_5: const #0s : i
        let s_3_5: i128 = 0;
        // D s_3_6: cast zx s_3_4 -> i
        let s_3_6: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_7: add s_3_6 s_3_5
        let s_3_7: i128 = (s_3_6 + s_3_5);
        // D s_3_8: cast reint s_3_7 -> i64
        let s_3_8: i64 = (s_3_7 as i64);
        // D s_3_9: read-var esizeshadow#2916:i64
        let s_3_9: i64 = fn_state.esizeshadow_2916;
        // D s_3_10: cast zx s_3_9 -> i
        let s_3_10: i128 = (i128::try_from(s_3_9).unwrap());
        // D s_3_11: cast reint s_3_10 -> i64
        let s_3_11: i64 = (s_3_10 as i64);
        // D s_3_12: read-var base:i64
        let s_3_12: i64 = fn_state.base;
        // D s_3_13: cast zx s_3_12 -> i
        let s_3_13: i128 = (i128::try_from(s_3_12).unwrap());
        // D s_3_14: read-var p:i64
        let s_3_14: i64 = fn_state.p;
        // D s_3_15: cast zx s_3_14 -> i
        let s_3_15: i128 = (i128::try_from(s_3_14).unwrap());
        // D s_3_16: add s_3_13 s_3_15
        let s_3_16: i128 = (s_3_13 + s_3_15);
        // D s_3_17: cast reint s_3_16 -> i64
        let s_3_17: i64 = (s_3_16 as i64);
        // D s_3_18: read-var esizeshadow#2916:i64
        let s_3_18: i64 = fn_state.esizeshadow_2916;
        // D s_3_19: cast zx s_3_18 -> i
        let s_3_19: i128 = (i128::try_from(s_3_18).unwrap());
        // D s_3_20: cast reint s_3_19 -> i64
        let s_3_20: i64 = (s_3_19 as i64);
        // D s_3_21: cast zx s_3_17 -> i
        let s_3_21: i128 = (i128::try_from(s_3_17).unwrap());
        // D s_3_22: cast zx s_3_20 -> i
        let s_3_22: i128 = (i128::try_from(s_3_20).unwrap());
        // D s_3_23: read-var operand1:bv
        let s_3_23: Bits = fn_state.operand1;
        // D s_3_24: call Elem_read(s_3_23, s_3_21, s_3_22)
        let s_3_24: Bits = Elem_read(state, tracer, s_3_23, s_3_21, s_3_22);
        // D s_3_25: cast zx s_3_8 -> i
        let s_3_25: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_26: cast zx s_3_11 -> i
        let s_3_26: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_27: read-var result:bv
        let s_3_27: Bits = fn_state.result;
        // D s_3_28: call Elem_set(s_3_27, s_3_25, s_3_26, s_3_24)
        let s_3_28: Bits = Elem_set(state, tracer, s_3_27, s_3_25, s_3_26, s_3_24);
        // D s_3_29: write-var result <= s_3_28
        fn_state.result = s_3_28;
        // C s_3_30: const #2s : i
        let s_3_30: i128 = 2;
        // D s_3_31: read-var p:i64
        let s_3_31: i64 = fn_state.p;
        // D s_3_32: cast zx s_3_31 -> i
        let s_3_32: i128 = (i128::try_from(s_3_31).unwrap());
        // D s_3_33: mul s_3_30 s_3_32
        let s_3_33: i128 = ((s_3_30) * (s_3_32));
        // D s_3_34: cast reint s_3_33 -> i64
        let s_3_34: i64 = (s_3_33 as i64);
        // C s_3_35: const #1s : i
        let s_3_35: i128 = 1;
        // D s_3_36: cast zx s_3_34 -> i
        let s_3_36: i128 = (i128::try_from(s_3_34).unwrap());
        // D s_3_37: add s_3_36 s_3_35
        let s_3_37: i128 = (s_3_36 + s_3_35);
        // D s_3_38: cast reint s_3_37 -> i64
        let s_3_38: i64 = (s_3_37 as i64);
        // D s_3_39: read-var esizeshadow#2916:i64
        let s_3_39: i64 = fn_state.esizeshadow_2916;
        // D s_3_40: cast zx s_3_39 -> i
        let s_3_40: i128 = (i128::try_from(s_3_39).unwrap());
        // D s_3_41: cast reint s_3_40 -> i64
        let s_3_41: i64 = (s_3_40 as i64);
        // D s_3_42: read-var base:i64
        let s_3_42: i64 = fn_state.base;
        // D s_3_43: cast zx s_3_42 -> i
        let s_3_43: i128 = (i128::try_from(s_3_42).unwrap());
        // D s_3_44: read-var p:i64
        let s_3_44: i64 = fn_state.p;
        // D s_3_45: cast zx s_3_44 -> i
        let s_3_45: i128 = (i128::try_from(s_3_44).unwrap());
        // D s_3_46: add s_3_43 s_3_45
        let s_3_46: i128 = (s_3_43 + s_3_45);
        // D s_3_47: cast reint s_3_46 -> i64
        let s_3_47: i64 = (s_3_46 as i64);
        // D s_3_48: read-var esizeshadow#2916:i64
        let s_3_48: i64 = fn_state.esizeshadow_2916;
        // D s_3_49: cast zx s_3_48 -> i
        let s_3_49: i128 = (i128::try_from(s_3_48).unwrap());
        // D s_3_50: cast reint s_3_49 -> i64
        let s_3_50: i64 = (s_3_49 as i64);
        // D s_3_51: cast zx s_3_47 -> i
        let s_3_51: i128 = (i128::try_from(s_3_47).unwrap());
        // D s_3_52: cast zx s_3_50 -> i
        let s_3_52: i128 = (i128::try_from(s_3_50).unwrap());
        // D s_3_53: read-var operand2:bv
        let s_3_53: Bits = fn_state.operand2;
        // D s_3_54: call Elem_read(s_3_53, s_3_51, s_3_52)
        let s_3_54: Bits = Elem_read(state, tracer, s_3_53, s_3_51, s_3_52);
        // D s_3_55: cast zx s_3_38 -> i
        let s_3_55: i128 = (i128::try_from(s_3_38).unwrap());
        // D s_3_56: cast zx s_3_41 -> i
        let s_3_56: i128 = (i128::try_from(s_3_41).unwrap());
        // D s_3_57: read-var result:bv
        let s_3_57: Bits = fn_state.result;
        // D s_3_58: call Elem_set(s_3_57, s_3_55, s_3_56, s_3_54)
        let s_3_58: Bits = Elem_set(state, tracer, s_3_57, s_3_55, s_3_56, s_3_54);
        // D s_3_59: write-var result <= s_3_58
        fn_state.result = s_3_58;
        // D s_3_60: read-var p:i64
        let s_3_60: i64 = fn_state.p;
        // C s_3_61: const #1s : i64
        let s_3_61: i64 = 1;
        // D s_3_62: add s_3_60 s_3_61
        let s_3_62: i64 = (s_3_60 + s_3_61);
        // D s_3_63: write-var p <= s_3_62
        fn_state.p = s_3_62;
        // N s_3_64: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var VLshadow#2918:i64
        let s_4_0: i64 = fn_state.VLshadow_2918;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // D s_4_3: read-var d:i64
        let s_4_3: i64 = fn_state.d;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: cast zx s_4_2 -> i
        let s_4_5: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_6: read-var result:bv
        let s_4_6: Bits = fn_state.result;
        // D s_4_7: call Z_set(s_4_4, s_4_5, s_4_6)
        let s_4_7: () = Z_set(state, tracer, s_4_4, s_4_5, s_4_6);
        // N s_4_8: return
        return;
    }
}
