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
use Elem_read::*;
use CheckNonStreamingSVEEnabled::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_ZIP1_Z_ZZ_Q<T: Tracer>(
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
        VLshadow_2923: i64,
        gs_195700: i64,
        VLshadow_2924: i64,
        result: Bits,
        operand1: Bits,
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
        // D s_0_0: read-var VL:i64
        let s_0_0: i64 = fn_state.VL;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var VLshadow#2923 <= s_0_2
        fn_state.VLshadow_2923 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckNonStreamingSVEEnabled(s_0_4)
        let s_0_5: () = CheckNonStreamingSVEEnabled(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#2923:i64
        let s_1_0: i64 = fn_state.VLshadow_2923;
        // D s_1_1: write-var VLshadow#2924 <= s_1_0
        fn_state.VLshadow_2924 = s_1_0;
        // C s_1_2: const #2s : i
        let s_1_2: i128 = 2;
        // D s_1_3: read-var esize:i64
        let s_1_3: i64 = fn_state.esize;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: mul s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) * (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#2924:i64
        let s_1_7: i64 = fn_state.VLshadow_2924;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast zx s_1_6 -> i
        let s_1_9: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_10: cmp-lt s_1_8 s_1_9
        let s_1_10: bool = ((s_1_8) < (s_1_9));
        // N s_1_11: branch s_1_10 b6 b2
        if s_1_10 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #2s : i
        let s_2_0: i128 = 2;
        // D s_2_1: read-var esize:i64
        let s_2_1: i64 = fn_state.esize;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: mul s_2_2 s_2_0
        let s_2_3: i128 = ((s_2_2) * (s_2_0));
        // D s_2_4: cast reint s_2_3 -> i64
        let s_2_4: i64 = (s_2_3 as i64);
        // D s_2_5: read-var VLshadow#2924:i64
        let s_2_5: i64 = fn_state.VLshadow_2924;
        // D s_2_6: cast zx s_2_5 -> i
        let s_2_6: i128 = (i128::try_from(s_2_5).unwrap());
        // D s_2_7: cast zx s_2_4 -> i
        let s_2_7: i128 = (i128::try_from(s_2_4).unwrap());
        // D s_2_8: div s_2_6 s_2_7
        let s_2_8: i128 = ((s_2_6) / (s_2_7));
        // D s_2_9: cast reint s_2_8 -> i64
        let s_2_9: i64 = (s_2_8 as i64);
        // D s_2_10: read-var VLshadow#2924:i64
        let s_2_10: i64 = fn_state.VLshadow_2924;
        // D s_2_11: cast zx s_2_10 -> i
        let s_2_11: i128 = (i128::try_from(s_2_10).unwrap());
        // D s_2_12: cast reint s_2_11 -> i64
        let s_2_12: i64 = (s_2_11 as i64);
        // D s_2_13: read-var n:i64
        let s_2_13: i64 = fn_state.n;
        // D s_2_14: cast zx s_2_13 -> i
        let s_2_14: i128 = (i128::try_from(s_2_13).unwrap());
        // D s_2_15: cast zx s_2_12 -> i
        let s_2_15: i128 = (i128::try_from(s_2_12).unwrap());
        // D s_2_16: call Z_read(s_2_14, s_2_15)
        let s_2_16: Bits = Z_read(state, tracer, s_2_14, s_2_15);
        // D s_2_17: write-var operand1 <= s_2_16
        fn_state.operand1 = s_2_16;
        // D s_2_18: read-var VLshadow#2924:i64
        let s_2_18: i64 = fn_state.VLshadow_2924;
        // D s_2_19: cast zx s_2_18 -> i
        let s_2_19: i128 = (i128::try_from(s_2_18).unwrap());
        // D s_2_20: cast reint s_2_19 -> i64
        let s_2_20: i64 = (s_2_19 as i64);
        // D s_2_21: read-var m:i64
        let s_2_21: i64 = fn_state.m;
        // D s_2_22: cast zx s_2_21 -> i
        let s_2_22: i128 = (i128::try_from(s_2_21).unwrap());
        // D s_2_23: cast zx s_2_20 -> i
        let s_2_23: i128 = (i128::try_from(s_2_20).unwrap());
        // D s_2_24: call Z_read(s_2_22, s_2_23)
        let s_2_24: Bits = Z_read(state, tracer, s_2_22, s_2_23);
        // D s_2_25: write-var operand2 <= s_2_24
        fn_state.operand2 = s_2_24;
        // D s_2_26: read-var VLshadow#2924:i64
        let s_2_26: i64 = fn_state.VLshadow_2924;
        // D s_2_27: cast zx s_2_26 -> i
        let s_2_27: i128 = (i128::try_from(s_2_26).unwrap());
        // D s_2_28: call Zeros(s_2_27)
        let s_2_28: Bits = Zeros(state, tracer, s_2_27);
        // D s_2_29: write-var result <= s_2_28
        fn_state.result = s_2_28;
        // D s_2_30: read-var part:i64
        let s_2_30: i64 = fn_state.part;
        // D s_2_31: cast zx s_2_30 -> i
        let s_2_31: i128 = (i128::try_from(s_2_30).unwrap());
        // D s_2_32: cast zx s_2_9 -> i
        let s_2_32: i128 = (i128::try_from(s_2_9).unwrap());
        // D s_2_33: mul s_2_31 s_2_32
        let s_2_33: i128 = ((s_2_31) * (s_2_32));
        // D s_2_34: cast reint s_2_33 -> i64
        let s_2_34: i64 = (s_2_33 as i64);
        // D s_2_35: write-var base <= s_2_34
        fn_state.base = s_2_34;
        // C s_2_36: const #0s : i64
        let s_2_36: i64 = 0;
        // C s_2_37: const #1s : i
        let s_2_37: i128 = 1;
        // D s_2_38: cast zx s_2_9 -> i
        let s_2_38: i128 = (i128::try_from(s_2_9).unwrap());
        // D s_2_39: sub s_2_38 s_2_37
        let s_2_39: i128 = ((s_2_38) - (s_2_37));
        // D s_2_40: cast reint s_2_39 -> i64
        let s_2_40: i64 = (s_2_39 as i64);
        // D s_2_41: write-var gs#195700 <= s_2_40
        fn_state.gs_195700 = s_2_40;
        // D s_2_42: write-var p <= s_2_36
        fn_state.p = s_2_36;
        // N s_2_43: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var p:i64
        let s_3_0: i64 = fn_state.p;
        // D s_3_1: read-var gs#195700:i64
        let s_3_1: i64 = fn_state.gs_195700;
        // D s_3_2: cmp-gt s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) > (s_3_1));
        // N s_3_3: branch s_3_2 b5 b4
        if s_3_2 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #2s : i
        let s_4_0: i128 = 2;
        // D s_4_1: read-var p:i64
        let s_4_1: i64 = fn_state.p;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: mul s_4_0 s_4_2
        let s_4_3: i128 = ((s_4_0) * (s_4_2));
        // D s_4_4: cast reint s_4_3 -> i64
        let s_4_4: i64 = (s_4_3 as i64);
        // C s_4_5: const #0s : i
        let s_4_5: i128 = 0;
        // D s_4_6: cast zx s_4_4 -> i
        let s_4_6: i128 = (i128::try_from(s_4_4).unwrap());
        // D s_4_7: add s_4_6 s_4_5
        let s_4_7: i128 = (s_4_6 + s_4_5);
        // D s_4_8: cast reint s_4_7 -> i64
        let s_4_8: i64 = (s_4_7 as i64);
        // D s_4_9: read-var esize:i64
        let s_4_9: i64 = fn_state.esize;
        // D s_4_10: cast zx s_4_9 -> i
        let s_4_10: i128 = (i128::try_from(s_4_9).unwrap());
        // D s_4_11: cast reint s_4_10 -> i64
        let s_4_11: i64 = (s_4_10 as i64);
        // D s_4_12: read-var base:i64
        let s_4_12: i64 = fn_state.base;
        // D s_4_13: cast zx s_4_12 -> i
        let s_4_13: i128 = (i128::try_from(s_4_12).unwrap());
        // D s_4_14: read-var p:i64
        let s_4_14: i64 = fn_state.p;
        // D s_4_15: cast zx s_4_14 -> i
        let s_4_15: i128 = (i128::try_from(s_4_14).unwrap());
        // D s_4_16: add s_4_13 s_4_15
        let s_4_16: i128 = (s_4_13 + s_4_15);
        // D s_4_17: cast reint s_4_16 -> i64
        let s_4_17: i64 = (s_4_16 as i64);
        // D s_4_18: read-var esize:i64
        let s_4_18: i64 = fn_state.esize;
        // D s_4_19: cast zx s_4_18 -> i
        let s_4_19: i128 = (i128::try_from(s_4_18).unwrap());
        // D s_4_20: cast reint s_4_19 -> i64
        let s_4_20: i64 = (s_4_19 as i64);
        // D s_4_21: cast zx s_4_17 -> i
        let s_4_21: i128 = (i128::try_from(s_4_17).unwrap());
        // D s_4_22: cast zx s_4_20 -> i
        let s_4_22: i128 = (i128::try_from(s_4_20).unwrap());
        // D s_4_23: read-var operand1:bv
        let s_4_23: Bits = fn_state.operand1;
        // D s_4_24: call Elem_read(s_4_23, s_4_21, s_4_22)
        let s_4_24: Bits = Elem_read(state, tracer, s_4_23, s_4_21, s_4_22);
        // D s_4_25: cast reint s_4_24 -> u128
        let s_4_25: u128 = (s_4_24.value() as u128);
        // D s_4_26: cast zx s_4_8 -> i
        let s_4_26: i128 = (i128::try_from(s_4_8).unwrap());
        // D s_4_27: cast zx s_4_11 -> i
        let s_4_27: i128 = (i128::try_from(s_4_11).unwrap());
        // D s_4_28: cast zx s_4_25 -> bv
        let s_4_28: Bits = Bits::new(s_4_25 as u128, 128u16);
        // D s_4_29: read-var result:bv
        let s_4_29: Bits = fn_state.result;
        // D s_4_30: call Elem_set(s_4_29, s_4_26, s_4_27, s_4_28)
        let s_4_30: Bits = Elem_set(state, tracer, s_4_29, s_4_26, s_4_27, s_4_28);
        // D s_4_31: write-var result <= s_4_30
        fn_state.result = s_4_30;
        // C s_4_32: const #2s : i
        let s_4_32: i128 = 2;
        // D s_4_33: read-var p:i64
        let s_4_33: i64 = fn_state.p;
        // D s_4_34: cast zx s_4_33 -> i
        let s_4_34: i128 = (i128::try_from(s_4_33).unwrap());
        // D s_4_35: mul s_4_32 s_4_34
        let s_4_35: i128 = ((s_4_32) * (s_4_34));
        // D s_4_36: cast reint s_4_35 -> i64
        let s_4_36: i64 = (s_4_35 as i64);
        // C s_4_37: const #1s : i
        let s_4_37: i128 = 1;
        // D s_4_38: cast zx s_4_36 -> i
        let s_4_38: i128 = (i128::try_from(s_4_36).unwrap());
        // D s_4_39: add s_4_38 s_4_37
        let s_4_39: i128 = (s_4_38 + s_4_37);
        // D s_4_40: cast reint s_4_39 -> i64
        let s_4_40: i64 = (s_4_39 as i64);
        // D s_4_41: read-var esize:i64
        let s_4_41: i64 = fn_state.esize;
        // D s_4_42: cast zx s_4_41 -> i
        let s_4_42: i128 = (i128::try_from(s_4_41).unwrap());
        // D s_4_43: cast reint s_4_42 -> i64
        let s_4_43: i64 = (s_4_42 as i64);
        // D s_4_44: read-var base:i64
        let s_4_44: i64 = fn_state.base;
        // D s_4_45: cast zx s_4_44 -> i
        let s_4_45: i128 = (i128::try_from(s_4_44).unwrap());
        // D s_4_46: read-var p:i64
        let s_4_46: i64 = fn_state.p;
        // D s_4_47: cast zx s_4_46 -> i
        let s_4_47: i128 = (i128::try_from(s_4_46).unwrap());
        // D s_4_48: add s_4_45 s_4_47
        let s_4_48: i128 = (s_4_45 + s_4_47);
        // D s_4_49: cast reint s_4_48 -> i64
        let s_4_49: i64 = (s_4_48 as i64);
        // D s_4_50: read-var esize:i64
        let s_4_50: i64 = fn_state.esize;
        // D s_4_51: cast zx s_4_50 -> i
        let s_4_51: i128 = (i128::try_from(s_4_50).unwrap());
        // D s_4_52: cast reint s_4_51 -> i64
        let s_4_52: i64 = (s_4_51 as i64);
        // D s_4_53: cast zx s_4_49 -> i
        let s_4_53: i128 = (i128::try_from(s_4_49).unwrap());
        // D s_4_54: cast zx s_4_52 -> i
        let s_4_54: i128 = (i128::try_from(s_4_52).unwrap());
        // D s_4_55: read-var operand2:bv
        let s_4_55: Bits = fn_state.operand2;
        // D s_4_56: call Elem_read(s_4_55, s_4_53, s_4_54)
        let s_4_56: Bits = Elem_read(state, tracer, s_4_55, s_4_53, s_4_54);
        // D s_4_57: cast reint s_4_56 -> u128
        let s_4_57: u128 = (s_4_56.value() as u128);
        // D s_4_58: cast zx s_4_40 -> i
        let s_4_58: i128 = (i128::try_from(s_4_40).unwrap());
        // D s_4_59: cast zx s_4_43 -> i
        let s_4_59: i128 = (i128::try_from(s_4_43).unwrap());
        // D s_4_60: cast zx s_4_57 -> bv
        let s_4_60: Bits = Bits::new(s_4_57 as u128, 128u16);
        // D s_4_61: read-var result:bv
        let s_4_61: Bits = fn_state.result;
        // D s_4_62: call Elem_set(s_4_61, s_4_58, s_4_59, s_4_60)
        let s_4_62: Bits = Elem_set(state, tracer, s_4_61, s_4_58, s_4_59, s_4_60);
        // D s_4_63: write-var result <= s_4_62
        fn_state.result = s_4_62;
        // D s_4_64: read-var p:i64
        let s_4_64: i64 = fn_state.p;
        // C s_4_65: const #1s : i64
        let s_4_65: i64 = 1;
        // D s_4_66: add s_4_64 s_4_65
        let s_4_66: i64 = (s_4_64 + s_4_65);
        // D s_4_67: write-var p <= s_4_66
        fn_state.p = s_4_66;
        // N s_4_68: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var VLshadow#2924:i64
        let s_5_0: i64 = fn_state.VLshadow_2924;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: cast reint s_5_1 -> i64
        let s_5_2: i64 = (s_5_1 as i64);
        // D s_5_3: read-var d:i64
        let s_5_3: i64 = fn_state.d;
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_5: cast zx s_5_2 -> i
        let s_5_5: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_6: read-var result:bv
        let s_5_6: Bits = fn_state.result;
        // D s_5_7: call Z_set(s_5_4, s_5_5, s_5_6)
        let s_5_7: () = Z_set(state, tracer, s_5_4, s_5_5, s_5_6);
        // N s_5_8: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: panic
        panic!("{:?}", ());
        // N s_6_1: return
        return;
    }
}
