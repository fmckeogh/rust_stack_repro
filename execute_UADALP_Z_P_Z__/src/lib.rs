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
use CheckSVEEnabled::*;
use AnyActiveElement::*;
use ActivePredicateElement::*;
use P_read::*;
use integer_subrange::*;
use Elem_read::*;
use Zeros::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_UADALP_Z_P_Z__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    da: i64,
    esize: i64,
    g: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        operand_acc: Bits,
        VLshadow_3394: i64,
        mask: Bits,
        gs_203369: i64,
        elements: i64,
        result: Bits,
        VLshadow_3395: i64,
        esizeshadow_3393: i64,
        operand_src: Bits,
        VL: i64,
        da: i64,
        esize: i64,
        g: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        VL,
        da,
        esize,
        g,
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
        // D s_0_3: write-var esizeshadow#3393 <= s_0_2
        fn_state.esizeshadow_3393 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#3394 <= s_0_6
        fn_state.VLshadow_3394 = s_0_6;
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
        // D s_1_0: read-var VLshadow#3394:i64
        let s_1_0: i64 = fn_state.VLshadow_3394;
        // D s_1_1: write-var VLshadow#3395 <= s_1_0
        fn_state.VLshadow_3395 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#3395:i64
        let s_1_3: i64 = fn_state.VLshadow_3395;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#3395:i64
        let s_1_7: i64 = fn_state.VLshadow_3395;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: read-var esizeshadow#3393:i64
        let s_1_9: i64 = fn_state.esizeshadow_3393;
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_11: div s_1_8 s_1_10
        let s_1_11: i128 = ((s_1_8) / (s_1_10));
        // D s_1_12: cast reint s_1_11 -> i64
        let s_1_12: i64 = (s_1_11 as i64);
        // D s_1_13: write-var elements <= s_1_12
        fn_state.elements = s_1_12;
        // D s_1_14: cast zx s_1_6 -> i
        let s_1_14: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_15: cast reint s_1_14 -> i64
        let s_1_15: i64 = (s_1_14 as i64);
        // D s_1_16: read-var g:i64
        let s_1_16: i64 = fn_state.g;
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_18: cast zx s_1_15 -> i
        let s_1_18: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_19: call P_read(s_1_17, s_1_18)
        let s_1_19: Bits = P_read(state, tracer, s_1_17, s_1_18);
        // D s_1_20: write-var mask <= s_1_19
        fn_state.mask = s_1_19;
        // D s_1_21: read-var VLshadow#3395:i64
        let s_1_21: i64 = fn_state.VLshadow_3395;
        // D s_1_22: cast zx s_1_21 -> i
        let s_1_22: i128 = (i128::try_from(s_1_21).unwrap());
        // D s_1_23: cast reint s_1_22 -> i64
        let s_1_23: i64 = (s_1_22 as i64);
        // D s_1_24: read-var da:i64
        let s_1_24: i64 = fn_state.da;
        // D s_1_25: cast zx s_1_24 -> i
        let s_1_25: i128 = (i128::try_from(s_1_24).unwrap());
        // D s_1_26: cast zx s_1_23 -> i
        let s_1_26: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_27: call Z_read(s_1_25, s_1_26)
        let s_1_27: Bits = Z_read(state, tracer, s_1_25, s_1_26);
        // D s_1_28: write-var operand_acc <= s_1_27
        fn_state.operand_acc = s_1_27;
        // D s_1_29: read-var esizeshadow#3393:i64
        let s_1_29: i64 = fn_state.esizeshadow_3393;
        // D s_1_30: cast zx s_1_29 -> i
        let s_1_30: i128 = (i128::try_from(s_1_29).unwrap());
        // D s_1_31: read-var mask:bv
        let s_1_31: Bits = fn_state.mask;
        // D s_1_32: call AnyActiveElement(s_1_31, s_1_30)
        let s_1_32: bool = AnyActiveElement(state, tracer, s_1_31, s_1_30);
        // N s_1_33: branch s_1_32 b10 b2
        if s_1_32 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var VLshadow#3395:i64
        let s_2_0: i64 = fn_state.VLshadow_3395;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: call Zeros(s_2_1)
        let s_2_2: Bits = Zeros(state, tracer, s_2_1);
        // D s_2_3: write-var operand_src <= s_2_2
        fn_state.operand_src = s_2_2;
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
        // D s_3_2: read-var elements:i64
        let s_3_2: i64 = fn_state.elements;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: sub s_3_3 s_3_1
        let s_3_4: i128 = ((s_3_3) - (s_3_1));
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: write-var gs#203369 <= s_3_5
        fn_state.gs_203369 = s_3_5;
        // D s_3_7: write-var e <= s_3_0
        fn_state.e = s_3_0;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#203369:i64
        let s_4_1: i64 = fn_state.gs_203369;
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
        // D s_5_0: read-var e:i64
        let s_5_0: i64 = fn_state.e;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: read-var esizeshadow#3393:i64
        let s_5_2: i64 = fn_state.esizeshadow_3393;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: read-var mask:bv
        let s_5_4: Bits = fn_state.mask;
        // D s_5_5: call ActivePredicateElement(s_5_4, s_5_1, s_5_3)
        let s_5_5: bool = ActivePredicateElement(state, tracer, s_5_4, s_5_1, s_5_3);
        // D s_5_6: not s_5_5
        let s_5_6: bool = !s_5_5;
        // N s_5_7: branch s_5_6 b8 b6
        if s_5_6 {
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
        // C s_6_0: const #2s : i
        let s_6_0: i128 = 2;
        // D s_6_1: read-var e:i64
        let s_6_1: i64 = fn_state.e;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: mul s_6_0 s_6_2
        let s_6_3: i128 = ((s_6_0) * (s_6_2));
        // D s_6_4: cast reint s_6_3 -> i64
        let s_6_4: i64 = (s_6_3 as i64);
        // C s_6_5: const #0s : i
        let s_6_5: i128 = 0;
        // D s_6_6: cast zx s_6_4 -> i
        let s_6_6: i128 = (i128::try_from(s_6_4).unwrap());
        // D s_6_7: add s_6_6 s_6_5
        let s_6_7: i128 = (s_6_6 + s_6_5);
        // D s_6_8: cast reint s_6_7 -> i64
        let s_6_8: i64 = (s_6_7 as i64);
        // C s_6_9: const #2s : i
        let s_6_9: i128 = 2;
        // D s_6_10: read-var esizeshadow#3393:i64
        let s_6_10: i64 = fn_state.esizeshadow_3393;
        // D s_6_11: cast zx s_6_10 -> i
        let s_6_11: i128 = (i128::try_from(s_6_10).unwrap());
        // D s_6_12: div s_6_11 s_6_9
        let s_6_12: i128 = ((s_6_11) / (s_6_9));
        // D s_6_13: cast reint s_6_12 -> i64
        let s_6_13: i64 = (s_6_12 as i64);
        // D s_6_14: cast zx s_6_13 -> i
        let s_6_14: i128 = (i128::try_from(s_6_13).unwrap());
        // D s_6_15: cast reint s_6_14 -> i64
        let s_6_15: i64 = (s_6_14 as i64);
        // D s_6_16: cast zx s_6_8 -> i
        let s_6_16: i128 = (i128::try_from(s_6_8).unwrap());
        // D s_6_17: cast zx s_6_15 -> i
        let s_6_17: i128 = (i128::try_from(s_6_15).unwrap());
        // D s_6_18: read-var operand_src:bv
        let s_6_18: Bits = fn_state.operand_src;
        // D s_6_19: call Elem_read(s_6_18, s_6_16, s_6_17)
        let s_6_19: Bits = Elem_read(state, tracer, s_6_18, s_6_16, s_6_17);
        // D s_6_20: cast zx s_6_19 -> i
        let s_6_20: i128 = (s_6_19.value() as i128);
        // D s_6_21: cast reint s_6_20 -> i64
        let s_6_21: i64 = (s_6_20 as i64);
        // C s_6_22: const #2s : i
        let s_6_22: i128 = 2;
        // D s_6_23: read-var e:i64
        let s_6_23: i64 = fn_state.e;
        // D s_6_24: cast zx s_6_23 -> i
        let s_6_24: i128 = (i128::try_from(s_6_23).unwrap());
        // D s_6_25: mul s_6_22 s_6_24
        let s_6_25: i128 = ((s_6_22) * (s_6_24));
        // D s_6_26: cast reint s_6_25 -> i64
        let s_6_26: i64 = (s_6_25 as i64);
        // C s_6_27: const #1s : i
        let s_6_27: i128 = 1;
        // D s_6_28: cast zx s_6_26 -> i
        let s_6_28: i128 = (i128::try_from(s_6_26).unwrap());
        // D s_6_29: add s_6_28 s_6_27
        let s_6_29: i128 = (s_6_28 + s_6_27);
        // D s_6_30: cast reint s_6_29 -> i64
        let s_6_30: i64 = (s_6_29 as i64);
        // C s_6_31: const #2s : i
        let s_6_31: i128 = 2;
        // D s_6_32: read-var esizeshadow#3393:i64
        let s_6_32: i64 = fn_state.esizeshadow_3393;
        // D s_6_33: cast zx s_6_32 -> i
        let s_6_33: i128 = (i128::try_from(s_6_32).unwrap());
        // D s_6_34: div s_6_33 s_6_31
        let s_6_34: i128 = ((s_6_33) / (s_6_31));
        // D s_6_35: cast reint s_6_34 -> i64
        let s_6_35: i64 = (s_6_34 as i64);
        // D s_6_36: cast zx s_6_35 -> i
        let s_6_36: i128 = (i128::try_from(s_6_35).unwrap());
        // D s_6_37: cast reint s_6_36 -> i64
        let s_6_37: i64 = (s_6_36 as i64);
        // D s_6_38: cast zx s_6_30 -> i
        let s_6_38: i128 = (i128::try_from(s_6_30).unwrap());
        // D s_6_39: cast zx s_6_37 -> i
        let s_6_39: i128 = (i128::try_from(s_6_37).unwrap());
        // D s_6_40: read-var operand_src:bv
        let s_6_40: Bits = fn_state.operand_src;
        // D s_6_41: call Elem_read(s_6_40, s_6_38, s_6_39)
        let s_6_41: Bits = Elem_read(state, tracer, s_6_40, s_6_38, s_6_39);
        // D s_6_42: cast zx s_6_41 -> i
        let s_6_42: i128 = (s_6_41.value() as i128);
        // D s_6_43: cast reint s_6_42 -> i64
        let s_6_43: i64 = (s_6_42 as i64);
        // D s_6_44: read-var esizeshadow#3393:i64
        let s_6_44: i64 = fn_state.esizeshadow_3393;
        // D s_6_45: cast zx s_6_44 -> i
        let s_6_45: i128 = (i128::try_from(s_6_44).unwrap());
        // D s_6_46: cast reint s_6_45 -> i64
        let s_6_46: i64 = (s_6_45 as i64);
        // D s_6_47: read-var esizeshadow#3393:i64
        let s_6_47: i64 = fn_state.esizeshadow_3393;
        // D s_6_48: cast zx s_6_47 -> i
        let s_6_48: i128 = (i128::try_from(s_6_47).unwrap());
        // D s_6_49: cast reint s_6_48 -> i64
        let s_6_49: i64 = (s_6_48 as i64);
        // D s_6_50: read-var e:i64
        let s_6_50: i64 = fn_state.e;
        // D s_6_51: cast zx s_6_50 -> i
        let s_6_51: i128 = (i128::try_from(s_6_50).unwrap());
        // D s_6_52: cast zx s_6_49 -> i
        let s_6_52: i128 = (i128::try_from(s_6_49).unwrap());
        // D s_6_53: read-var operand_acc:bv
        let s_6_53: Bits = fn_state.operand_acc;
        // D s_6_54: call Elem_read(s_6_53, s_6_51, s_6_52)
        let s_6_54: Bits = Elem_read(state, tracer, s_6_53, s_6_51, s_6_52);
        // D s_6_55: cast zx s_6_21 -> i
        let s_6_55: i128 = (i128::try_from(s_6_21).unwrap());
        // D s_6_56: cast zx s_6_43 -> i
        let s_6_56: i128 = (i128::try_from(s_6_43).unwrap());
        // D s_6_57: add s_6_55 s_6_56
        let s_6_57: i128 = (s_6_55 + s_6_56);
        // D s_6_58: cast reint s_6_57 -> i64
        let s_6_58: i64 = (s_6_57 as i64);
        // C s_6_59: const #1s : i
        let s_6_59: i128 = 1;
        // D s_6_60: read-var esizeshadow#3393:i64
        let s_6_60: i64 = fn_state.esizeshadow_3393;
        // D s_6_61: cast zx s_6_60 -> i
        let s_6_61: i128 = (i128::try_from(s_6_60).unwrap());
        // D s_6_62: sub s_6_61 s_6_59
        let s_6_62: i128 = ((s_6_61) - (s_6_59));
        // D s_6_63: cast reint s_6_62 -> i64
        let s_6_63: i64 = (s_6_62 as i64);
        // C s_6_64: const #0s : i
        let s_6_64: i128 = 0;
        // D s_6_65: cast zx s_6_58 -> i
        let s_6_65: i128 = (i128::try_from(s_6_58).unwrap());
        // D s_6_66: cast zx s_6_63 -> i
        let s_6_66: i128 = (i128::try_from(s_6_63).unwrap());
        // D s_6_67: call integer_subrange(s_6_65, s_6_66, s_6_64)
        let s_6_67: Bits = integer_subrange(state, tracer, s_6_65, s_6_66, s_6_64);
        // D s_6_68: add s_6_54 s_6_67
        let s_6_68: Bits = (s_6_54 + s_6_67);
        // D s_6_69: read-var e:i64
        let s_6_69: i64 = fn_state.e;
        // D s_6_70: cast zx s_6_69 -> i
        let s_6_70: i128 = (i128::try_from(s_6_69).unwrap());
        // D s_6_71: cast zx s_6_46 -> i
        let s_6_71: i128 = (i128::try_from(s_6_46).unwrap());
        // D s_6_72: read-var result:bv
        let s_6_72: Bits = fn_state.result;
        // D s_6_73: call Elem_set(s_6_72, s_6_70, s_6_71, s_6_68)
        let s_6_73: Bits = Elem_set(state, tracer, s_6_72, s_6_70, s_6_71, s_6_68);
        // D s_6_74: write-var result <= s_6_73
        fn_state.result = s_6_73;
        // N s_6_75: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var e:i64
        let s_7_0: i64 = fn_state.e;
        // C s_7_1: const #1s : i64
        let s_7_1: i64 = 1;
        // D s_7_2: add s_7_0 s_7_1
        let s_7_2: i64 = (s_7_0 + s_7_1);
        // D s_7_3: write-var e <= s_7_2
        fn_state.e = s_7_2;
        // N s_7_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var esizeshadow#3393:i64
        let s_8_0: i64 = fn_state.esizeshadow_3393;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: cast reint s_8_1 -> i64
        let s_8_2: i64 = (s_8_1 as i64);
        // D s_8_3: read-var esizeshadow#3393:i64
        let s_8_3: i64 = fn_state.esizeshadow_3393;
        // D s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // D s_8_5: cast reint s_8_4 -> i64
        let s_8_5: i64 = (s_8_4 as i64);
        // D s_8_6: read-var e:i64
        let s_8_6: i64 = fn_state.e;
        // D s_8_7: cast zx s_8_6 -> i
        let s_8_7: i128 = (i128::try_from(s_8_6).unwrap());
        // D s_8_8: cast zx s_8_5 -> i
        let s_8_8: i128 = (i128::try_from(s_8_5).unwrap());
        // D s_8_9: read-var operand_acc:bv
        let s_8_9: Bits = fn_state.operand_acc;
        // D s_8_10: call Elem_read(s_8_9, s_8_7, s_8_8)
        let s_8_10: Bits = Elem_read(state, tracer, s_8_9, s_8_7, s_8_8);
        // D s_8_11: read-var e:i64
        let s_8_11: i64 = fn_state.e;
        // D s_8_12: cast zx s_8_11 -> i
        let s_8_12: i128 = (i128::try_from(s_8_11).unwrap());
        // D s_8_13: cast zx s_8_2 -> i
        let s_8_13: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_14: read-var result:bv
        let s_8_14: Bits = fn_state.result;
        // D s_8_15: call Elem_set(s_8_14, s_8_12, s_8_13, s_8_10)
        let s_8_15: Bits = Elem_set(state, tracer, s_8_14, s_8_12, s_8_13, s_8_10);
        // D s_8_16: write-var result <= s_8_15
        fn_state.result = s_8_15;
        // N s_8_17: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var VLshadow#3395:i64
        let s_9_0: i64 = fn_state.VLshadow_3395;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: cast reint s_9_1 -> i64
        let s_9_2: i64 = (s_9_1 as i64);
        // D s_9_3: read-var da:i64
        let s_9_3: i64 = fn_state.da;
        // D s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_5: cast zx s_9_2 -> i
        let s_9_5: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_6: read-var result:bv
        let s_9_6: Bits = fn_state.result;
        // D s_9_7: call Z_set(s_9_4, s_9_5, s_9_6)
        let s_9_7: () = Z_set(state, tracer, s_9_4, s_9_5, s_9_6);
        // N s_9_8: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var VLshadow#3395:i64
        let s_10_0: i64 = fn_state.VLshadow_3395;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: cast reint s_10_1 -> i64
        let s_10_2: i64 = (s_10_1 as i64);
        // D s_10_3: read-var n:i64
        let s_10_3: i64 = fn_state.n;
        // D s_10_4: cast zx s_10_3 -> i
        let s_10_4: i128 = (i128::try_from(s_10_3).unwrap());
        // D s_10_5: cast zx s_10_2 -> i
        let s_10_5: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_6: call Z_read(s_10_4, s_10_5)
        let s_10_6: Bits = Z_read(state, tracer, s_10_4, s_10_5);
        // D s_10_7: write-var operand_src <= s_10_6
        fn_state.operand_src = s_10_6;
        // N s_10_8: jump b3
        return block_3(state, tracer, fn_state);
    }
}
