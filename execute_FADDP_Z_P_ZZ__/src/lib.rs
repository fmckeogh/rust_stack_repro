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
use Zeros::*;
use Elem_read::*;
use IsEven::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_FADDP_Z_P_ZZ__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    dn: i64,
    esize: i64,
    g: i64,
    m: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        esizeshadow_2327: i64,
        e: i64,
        gs_181779: bool,
        gs_181771: i64,
        mask: Bits,
        element2: Bits,
        gs_181777: bool,
        VLshadow_2328: i64,
        ga_275543: Bits,
        ga_275542: i64,
        element1: Bits,
        elements: i64,
        result: Bits,
        operand1: Bits,
        VLshadow_2329: i64,
        operand2: Bits,
        VL: i64,
        dn: i64,
        esize: i64,
        g: i64,
        m: i64,
    }
    let fn_state = FunctionState {
        VL,
        dn,
        esize,
        g,
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
        // D s_0_3: write-var esizeshadow#2327 <= s_0_2
        fn_state.esizeshadow_2327 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#2328 <= s_0_6
        fn_state.VLshadow_2328 = s_0_6;
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
        // D s_1_0: read-var VLshadow#2328:i64
        let s_1_0: i64 = fn_state.VLshadow_2328;
        // D s_1_1: write-var VLshadow#2329 <= s_1_0
        fn_state.VLshadow_2329 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#2329:i64
        let s_1_3: i64 = fn_state.VLshadow_2329;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#2329:i64
        let s_1_7: i64 = fn_state.VLshadow_2329;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: read-var esizeshadow#2327:i64
        let s_1_9: i64 = fn_state.esizeshadow_2327;
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
        // D s_1_21: read-var VLshadow#2329:i64
        let s_1_21: i64 = fn_state.VLshadow_2329;
        // D s_1_22: cast zx s_1_21 -> i
        let s_1_22: i128 = (i128::try_from(s_1_21).unwrap());
        // D s_1_23: cast reint s_1_22 -> i64
        let s_1_23: i64 = (s_1_22 as i64);
        // D s_1_24: read-var dn:i64
        let s_1_24: i64 = fn_state.dn;
        // D s_1_25: cast zx s_1_24 -> i
        let s_1_25: i128 = (i128::try_from(s_1_24).unwrap());
        // D s_1_26: cast zx s_1_23 -> i
        let s_1_26: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_27: call Z_read(s_1_25, s_1_26)
        let s_1_27: Bits = Z_read(state, tracer, s_1_25, s_1_26);
        // D s_1_28: write-var operand1 <= s_1_27
        fn_state.operand1 = s_1_27;
        // D s_1_29: read-var esizeshadow#2327:i64
        let s_1_29: i64 = fn_state.esizeshadow_2327;
        // D s_1_30: cast zx s_1_29 -> i
        let s_1_30: i128 = (i128::try_from(s_1_29).unwrap());
        // D s_1_31: read-var mask:bv
        let s_1_31: Bits = fn_state.mask;
        // D s_1_32: call AnyActiveElement(s_1_31, s_1_30)
        let s_1_32: bool = AnyActiveElement(state, tracer, s_1_31, s_1_30);
        // N s_1_33: branch s_1_32 b20 b2
        if s_1_32 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var VLshadow#2329:i64
        let s_2_0: i64 = fn_state.VLshadow_2329;
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
        // D s_3_0: read-var VLshadow#2329:i64
        let s_3_0: i64 = fn_state.VLshadow_2329;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: cast reint s_3_1 -> i64
        let s_3_2: i64 = (s_3_1 as i64);
        // D s_3_3: read-var dn:i64
        let s_3_3: i64 = fn_state.dn;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: cast zx s_3_2 -> i
        let s_3_5: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_6: call Z_read(s_3_4, s_3_5)
        let s_3_6: Bits = Z_read(state, tracer, s_3_4, s_3_5);
        // D s_3_7: write-var result <= s_3_6
        fn_state.result = s_3_6;
        // C s_3_8: const #0s : i64
        let s_3_8: i64 = 0;
        // C s_3_9: const #1s : i
        let s_3_9: i128 = 1;
        // D s_3_10: read-var elements:i64
        let s_3_10: i64 = fn_state.elements;
        // D s_3_11: cast zx s_3_10 -> i
        let s_3_11: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_12: sub s_3_11 s_3_9
        let s_3_12: i128 = ((s_3_11) - (s_3_9));
        // D s_3_13: cast reint s_3_12 -> i64
        let s_3_13: i64 = (s_3_12 as i64);
        // D s_3_14: write-var gs#181771 <= s_3_13
        fn_state.gs_181771 = s_3_13;
        // D s_3_15: write-var e <= s_3_8
        fn_state.e = s_3_8;
        // N s_3_16: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#181771:i64
        let s_4_1: i64 = fn_state.gs_181771;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b19 b5
        if s_4_2 {
            return block_19(state, tracer, fn_state);
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
        // D s_5_2: read-var esizeshadow#2327:i64
        let s_5_2: i64 = fn_state.esizeshadow_2327;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: read-var mask:bv
        let s_5_4: Bits = fn_state.mask;
        // D s_5_5: call ActivePredicateElement(s_5_4, s_5_1, s_5_3)
        let s_5_5: bool = ActivePredicateElement(state, tracer, s_5_4, s_5_1, s_5_3);
        // N s_5_6: branch s_5_5 b8 b6
        if s_5_5 {
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
        // N s_6_0: jump b7
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
        // D s_8_0: read-var e:i64
        let s_8_0: i64 = fn_state.e;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: call IsEven(s_8_1)
        let s_8_2: bool = IsEven(state, tracer, s_8_1);
        // N s_8_3: branch s_8_2 b18 b9
        if s_8_2 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #1s : i
        let s_9_0: i128 = 1;
        // D s_9_1: read-var e:i64
        let s_9_1: i64 = fn_state.e;
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (i128::try_from(s_9_1).unwrap());
        // D s_9_3: sub s_9_2 s_9_0
        let s_9_3: i128 = ((s_9_2) - (s_9_0));
        // D s_9_4: cast reint s_9_3 -> i64
        let s_9_4: i64 = (s_9_3 as i64);
        // D s_9_5: read-var esizeshadow#2327:i64
        let s_9_5: i64 = fn_state.esizeshadow_2327;
        // D s_9_6: cast zx s_9_5 -> i
        let s_9_6: i128 = (i128::try_from(s_9_5).unwrap());
        // D s_9_7: cast reint s_9_6 -> i64
        let s_9_7: i64 = (s_9_6 as i64);
        // D s_9_8: cast zx s_9_4 -> i
        let s_9_8: i128 = (i128::try_from(s_9_4).unwrap());
        // D s_9_9: cast zx s_9_7 -> i
        let s_9_9: i128 = (i128::try_from(s_9_7).unwrap());
        // D s_9_10: read-var operand2:bv
        let s_9_10: Bits = fn_state.operand2;
        // D s_9_11: call Elem_read(s_9_10, s_9_8, s_9_9)
        let s_9_11: Bits = Elem_read(state, tracer, s_9_10, s_9_8, s_9_9);
        // D s_9_12: write-var element1 <= s_9_11
        fn_state.element1 = s_9_11;
        // C s_9_13: const #0s : i
        let s_9_13: i128 = 0;
        // D s_9_14: read-var e:i64
        let s_9_14: i64 = fn_state.e;
        // D s_9_15: cast zx s_9_14 -> i
        let s_9_15: i128 = (i128::try_from(s_9_14).unwrap());
        // D s_9_16: add s_9_15 s_9_13
        let s_9_16: i128 = (s_9_15 + s_9_13);
        // D s_9_17: cast reint s_9_16 -> i64
        let s_9_17: i64 = (s_9_16 as i64);
        // D s_9_18: read-var esizeshadow#2327:i64
        let s_9_18: i64 = fn_state.esizeshadow_2327;
        // D s_9_19: cast zx s_9_18 -> i
        let s_9_19: i128 = (i128::try_from(s_9_18).unwrap());
        // D s_9_20: cast reint s_9_19 -> i64
        let s_9_20: i64 = (s_9_19 as i64);
        // D s_9_21: cast zx s_9_17 -> i
        let s_9_21: i128 = (i128::try_from(s_9_17).unwrap());
        // D s_9_22: cast zx s_9_20 -> i
        let s_9_22: i128 = (i128::try_from(s_9_20).unwrap());
        // D s_9_23: read-var operand2:bv
        let s_9_23: Bits = fn_state.operand2;
        // D s_9_24: call Elem_read(s_9_23, s_9_21, s_9_22)
        let s_9_24: Bits = Elem_read(state, tracer, s_9_23, s_9_21, s_9_22);
        // D s_9_25: write-var element2 <= s_9_24
        fn_state.element2 = s_9_24;
        // N s_9_26: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var esizeshadow#2327:i64
        let s_10_0: i64 = fn_state.esizeshadow_2327;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: call __id(s_10_1)
        let s_10_2: i128 = u__id(state, tracer, s_10_1);
        // D s_10_3: cast reint s_10_2 -> i64
        let s_10_3: i64 = (s_10_2 as i64);
        // C s_10_4: const #16s : i
        let s_10_4: i128 = 16;
        // D s_10_5: cast zx s_10_3 -> i
        let s_10_5: i128 = (i128::try_from(s_10_3).unwrap());
        // D s_10_6: cmp-eq s_10_5 s_10_4
        let s_10_6: bool = ((s_10_5) == (s_10_4));
        // N s_10_7: branch s_10_6 b17 b11
        if s_10_6 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var esizeshadow#2327:i64
        let s_11_0: i64 = fn_state.esizeshadow_2327;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: call __id(s_11_1)
        let s_11_2: i128 = u__id(state, tracer, s_11_1);
        // D s_11_3: cast reint s_11_2 -> i64
        let s_11_3: i64 = (s_11_2 as i64);
        // C s_11_4: const #32s : i
        let s_11_4: i128 = 32;
        // D s_11_5: cast zx s_11_3 -> i
        let s_11_5: i128 = (i128::try_from(s_11_3).unwrap());
        // D s_11_6: cmp-eq s_11_5 s_11_4
        let s_11_6: bool = ((s_11_5) == (s_11_4));
        // D s_11_7: write-var gs#181777 <= s_11_6
        fn_state.gs_181777 = s_11_6;
        // N s_11_8: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#181777:u8
        let s_12_0: bool = fn_state.gs_181777;
        // N s_12_1: branch s_12_0 b16 b13
        if s_12_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var esizeshadow#2327:i64
        let s_13_0: i64 = fn_state.esizeshadow_2327;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: call __id(s_13_1)
        let s_13_2: i128 = u__id(state, tracer, s_13_1);
        // D s_13_3: cast reint s_13_2 -> i64
        let s_13_3: i64 = (s_13_2 as i64);
        // C s_13_4: const #64s : i
        let s_13_4: i128 = 64;
        // D s_13_5: cast zx s_13_3 -> i
        let s_13_5: i128 = (i128::try_from(s_13_3).unwrap());
        // D s_13_6: cmp-eq s_13_5 s_13_4
        let s_13_6: bool = ((s_13_5) == (s_13_4));
        // D s_13_7: write-var gs#181779 <= s_13_6
        fn_state.gs_181779 = s_13_6;
        // N s_13_8: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#181779:u8
        let s_14_0: bool = fn_state.gs_181779;
        // N s_14_1: assert s_14_0
        let s_14_1: () = assert!(s_14_0);
        // D s_14_2: read-var esizeshadow#2327:i64
        let s_14_2: i64 = fn_state.esizeshadow_2327;
        // D s_14_3: cast zx s_14_2 -> i
        let s_14_3: i128 = (i128::try_from(s_14_2).unwrap());
        // D s_14_4: cast reint s_14_3 -> i64
        let s_14_4: i64 = (s_14_3 as i64);
        // D s_14_5: write-var ga#275542 <= s_14_4
        fn_state.ga_275542 = s_14_4;
        // C s_14_6: const #() : ()
        let s_14_6: () = ();
        // S s_14_7: call FPCR_read(s_14_6)
        let s_14_7: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_14_6);
        // D s_14_8: read-var element1:bv
        let s_14_8: Bits = fn_state.element1;
        // D s_14_9: read-var element2:bv
        let s_14_9: Bits = fn_state.element2;
        // D s_14_10: call FPAdd(s_14_8, s_14_9, s_14_7)
        let s_14_10: Bits = FPAdd(state, tracer, s_14_8, s_14_9, s_14_7);
        // D s_14_11: write-var ga#275543 <= s_14_10
        fn_state.ga_275543 = s_14_10;
        // N s_14_12: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var e:i64
        let s_15_0: i64 = fn_state.e;
        // D s_15_1: cast zx s_15_0 -> i
        let s_15_1: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_2: read-var ga#275542:i64
        let s_15_2: i64 = fn_state.ga_275542;
        // D s_15_3: cast zx s_15_2 -> i
        let s_15_3: i128 = (i128::try_from(s_15_2).unwrap());
        // D s_15_4: read-var result:bv
        let s_15_4: Bits = fn_state.result;
        // D s_15_5: read-var ga#275543:bv
        let s_15_5: Bits = fn_state.ga_275543;
        // D s_15_6: call Elem_set(s_15_4, s_15_1, s_15_3, s_15_5)
        let s_15_6: Bits = Elem_set(state, tracer, s_15_4, s_15_1, s_15_3, s_15_5);
        // D s_15_7: write-var result <= s_15_6
        fn_state.result = s_15_6;
        // N s_15_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#181779 <= s_16_0
        fn_state.gs_181779 = s_16_0;
        // N s_16_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var gs#181777 <= s_17_0
        fn_state.gs_181777 = s_17_0;
        // N s_17_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #0s : i
        let s_18_0: i128 = 0;
        // D s_18_1: read-var e:i64
        let s_18_1: i64 = fn_state.e;
        // D s_18_2: cast zx s_18_1 -> i
        let s_18_2: i128 = (i128::try_from(s_18_1).unwrap());
        // D s_18_3: add s_18_2 s_18_0
        let s_18_3: i128 = (s_18_2 + s_18_0);
        // D s_18_4: cast reint s_18_3 -> i64
        let s_18_4: i64 = (s_18_3 as i64);
        // D s_18_5: read-var esizeshadow#2327:i64
        let s_18_5: i64 = fn_state.esizeshadow_2327;
        // D s_18_6: cast zx s_18_5 -> i
        let s_18_6: i128 = (i128::try_from(s_18_5).unwrap());
        // D s_18_7: cast reint s_18_6 -> i64
        let s_18_7: i64 = (s_18_6 as i64);
        // D s_18_8: cast zx s_18_4 -> i
        let s_18_8: i128 = (i128::try_from(s_18_4).unwrap());
        // D s_18_9: cast zx s_18_7 -> i
        let s_18_9: i128 = (i128::try_from(s_18_7).unwrap());
        // D s_18_10: read-var operand1:bv
        let s_18_10: Bits = fn_state.operand1;
        // D s_18_11: call Elem_read(s_18_10, s_18_8, s_18_9)
        let s_18_11: Bits = Elem_read(state, tracer, s_18_10, s_18_8, s_18_9);
        // D s_18_12: write-var element1 <= s_18_11
        fn_state.element1 = s_18_11;
        // C s_18_13: const #1s : i
        let s_18_13: i128 = 1;
        // D s_18_14: read-var e:i64
        let s_18_14: i64 = fn_state.e;
        // D s_18_15: cast zx s_18_14 -> i
        let s_18_15: i128 = (i128::try_from(s_18_14).unwrap());
        // D s_18_16: add s_18_15 s_18_13
        let s_18_16: i128 = (s_18_15 + s_18_13);
        // D s_18_17: cast reint s_18_16 -> i64
        let s_18_17: i64 = (s_18_16 as i64);
        // D s_18_18: read-var esizeshadow#2327:i64
        let s_18_18: i64 = fn_state.esizeshadow_2327;
        // D s_18_19: cast zx s_18_18 -> i
        let s_18_19: i128 = (i128::try_from(s_18_18).unwrap());
        // D s_18_20: cast reint s_18_19 -> i64
        let s_18_20: i64 = (s_18_19 as i64);
        // D s_18_21: cast zx s_18_17 -> i
        let s_18_21: i128 = (i128::try_from(s_18_17).unwrap());
        // D s_18_22: cast zx s_18_20 -> i
        let s_18_22: i128 = (i128::try_from(s_18_20).unwrap());
        // D s_18_23: read-var operand1:bv
        let s_18_23: Bits = fn_state.operand1;
        // D s_18_24: call Elem_read(s_18_23, s_18_21, s_18_22)
        let s_18_24: Bits = Elem_read(state, tracer, s_18_23, s_18_21, s_18_22);
        // D s_18_25: write-var element2 <= s_18_24
        fn_state.element2 = s_18_24;
        // N s_18_26: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var VLshadow#2329:i64
        let s_19_0: i64 = fn_state.VLshadow_2329;
        // D s_19_1: cast zx s_19_0 -> i
        let s_19_1: i128 = (i128::try_from(s_19_0).unwrap());
        // D s_19_2: cast reint s_19_1 -> i64
        let s_19_2: i64 = (s_19_1 as i64);
        // D s_19_3: read-var dn:i64
        let s_19_3: i64 = fn_state.dn;
        // D s_19_4: cast zx s_19_3 -> i
        let s_19_4: i128 = (i128::try_from(s_19_3).unwrap());
        // D s_19_5: cast zx s_19_2 -> i
        let s_19_5: i128 = (i128::try_from(s_19_2).unwrap());
        // D s_19_6: read-var result:bv
        let s_19_6: Bits = fn_state.result;
        // D s_19_7: call Z_set(s_19_4, s_19_5, s_19_6)
        let s_19_7: () = Z_set(state, tracer, s_19_4, s_19_5, s_19_6);
        // N s_19_8: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var VLshadow#2329:i64
        let s_20_0: i64 = fn_state.VLshadow_2329;
        // D s_20_1: cast zx s_20_0 -> i
        let s_20_1: i128 = (i128::try_from(s_20_0).unwrap());
        // D s_20_2: cast reint s_20_1 -> i64
        let s_20_2: i64 = (s_20_1 as i64);
        // D s_20_3: read-var m:i64
        let s_20_3: i64 = fn_state.m;
        // D s_20_4: cast zx s_20_3 -> i
        let s_20_4: i128 = (i128::try_from(s_20_3).unwrap());
        // D s_20_5: cast zx s_20_2 -> i
        let s_20_5: i128 = (i128::try_from(s_20_2).unwrap());
        // D s_20_6: call Z_read(s_20_4, s_20_5)
        let s_20_6: Bits = Z_read(state, tracer, s_20_4, s_20_5);
        // D s_20_7: write-var operand2 <= s_20_6
        fn_state.operand2 = s_20_6;
        // N s_20_8: jump b3
        return block_3(state, tracer, fn_state);
    }
}
