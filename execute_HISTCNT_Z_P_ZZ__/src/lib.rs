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
use AnyActiveElement::*;
use ActivePredicateElement::*;
use integer_subrange::*;
use P_read::*;
use Zeros::*;
use Elem_read::*;
use CheckNonStreamingSVEEnabled::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_HISTCNT_Z_P_ZZ__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    g: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        VLshadow_4111: i64,
        gs_216092: i64,
        VLshadow_4110: i64,
        e: i64,
        element1: Bits,
        esizeshadow_4109: i64,
        count: i128,
        elements: i64,
        result: Bits,
        i: i64,
        operand1: Bits,
        gs_216085: i64,
        mask: Bits,
        operand2: Bits,
        VL: i64,
        d: i64,
        esize: i64,
        g: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        g,
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
        // D s_0_3: write-var esizeshadow#4109 <= s_0_2
        fn_state.esizeshadow_4109 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#4110 <= s_0_6
        fn_state.VLshadow_4110 = s_0_6;
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call CheckNonStreamingSVEEnabled(s_0_8)
        let s_0_9: () = CheckNonStreamingSVEEnabled(state, tracer, s_0_8);
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#4110:i64
        let s_1_0: i64 = fn_state.VLshadow_4110;
        // D s_1_1: write-var VLshadow#4111 <= s_1_0
        fn_state.VLshadow_4111 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#4111:i64
        let s_1_3: i64 = fn_state.VLshadow_4111;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#4111:i64
        let s_1_7: i64 = fn_state.VLshadow_4111;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: read-var esizeshadow#4109:i64
        let s_1_9: i64 = fn_state.esizeshadow_4109;
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
        // D s_1_21: read-var esizeshadow#4109:i64
        let s_1_21: i64 = fn_state.esizeshadow_4109;
        // D s_1_22: cast zx s_1_21 -> i
        let s_1_22: i128 = (i128::try_from(s_1_21).unwrap());
        // D s_1_23: read-var mask:bv
        let s_1_23: Bits = fn_state.mask;
        // D s_1_24: call AnyActiveElement(s_1_23, s_1_22)
        let s_1_24: bool = AnyActiveElement(state, tracer, s_1_23, s_1_22);
        // N s_1_25: branch s_1_24 b22 b2
        if s_1_24 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var VLshadow#4111:i64
        let s_2_0: i64 = fn_state.VLshadow_4111;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: call Zeros(s_2_1)
        let s_2_2: Bits = Zeros(state, tracer, s_2_1);
        // D s_2_3: write-var operand1 <= s_2_2
        fn_state.operand1 = s_2_2;
        // N s_2_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var esizeshadow#4109:i64
        let s_3_0: i64 = fn_state.esizeshadow_4109;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var mask:bv
        let s_3_2: Bits = fn_state.mask;
        // D s_3_3: call AnyActiveElement(s_3_2, s_3_1)
        let s_3_3: bool = AnyActiveElement(state, tracer, s_3_2, s_3_1);
        // N s_3_4: branch s_3_3 b21 b4
        if s_3_3 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var VLshadow#4111:i64
        let s_4_0: i64 = fn_state.VLshadow_4111;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: call Zeros(s_4_1)
        let s_4_2: Bits = Zeros(state, tracer, s_4_1);
        // D s_4_3: write-var operand2 <= s_4_2
        fn_state.operand2 = s_4_2;
        // N s_4_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0s : i64
        let s_5_0: i64 = 0;
        // C s_5_1: const #1s : i
        let s_5_1: i128 = 1;
        // D s_5_2: read-var elements:i64
        let s_5_2: i64 = fn_state.elements;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: sub s_5_3 s_5_1
        let s_5_4: i128 = ((s_5_3) - (s_5_1));
        // D s_5_5: cast reint s_5_4 -> i64
        let s_5_5: i64 = (s_5_4 as i64);
        // D s_5_6: write-var gs#216085 <= s_5_5
        fn_state.gs_216085 = s_5_5;
        // D s_5_7: write-var e <= s_5_0
        fn_state.e = s_5_0;
        // N s_5_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var e:i64
        let s_6_0: i64 = fn_state.e;
        // D s_6_1: read-var gs#216085:i64
        let s_6_1: i64 = fn_state.gs_216085;
        // D s_6_2: cmp-gt s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) > (s_6_1));
        // N s_6_3: branch s_6_2 b20 b7
        if s_6_2 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0s : i
        let s_7_0: i128 = 0;
        // D s_7_1: write-var count <= s_7_0
        fn_state.count = s_7_0;
        // D s_7_2: read-var e:i64
        let s_7_2: i64 = fn_state.e;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: read-var esizeshadow#4109:i64
        let s_7_4: i64 = fn_state.esizeshadow_4109;
        // D s_7_5: cast zx s_7_4 -> i
        let s_7_5: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_6: read-var mask:bv
        let s_7_6: Bits = fn_state.mask;
        // D s_7_7: call ActivePredicateElement(s_7_6, s_7_3, s_7_5)
        let s_7_7: bool = ActivePredicateElement(state, tracer, s_7_6, s_7_3, s_7_5);
        // N s_7_8: branch s_7_7 b10 b8
        if s_7_7 {
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
        // D s_9_0: read-var count:i
        let s_9_0: i128 = fn_state.count;
        // D s_9_1: read-var esizeshadow#4109:i64
        let s_9_1: i64 = fn_state.esizeshadow_4109;
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (i128::try_from(s_9_1).unwrap());
        // D s_9_3: cast reint s_9_2 -> i64
        let s_9_3: i64 = (s_9_2 as i64);
        // C s_9_4: const #1s : i
        let s_9_4: i128 = 1;
        // D s_9_5: read-var esizeshadow#4109:i64
        let s_9_5: i64 = fn_state.esizeshadow_4109;
        // D s_9_6: cast zx s_9_5 -> i
        let s_9_6: i128 = (i128::try_from(s_9_5).unwrap());
        // D s_9_7: sub s_9_6 s_9_4
        let s_9_7: i128 = ((s_9_6) - (s_9_4));
        // D s_9_8: cast reint s_9_7 -> i64
        let s_9_8: i64 = (s_9_7 as i64);
        // C s_9_9: const #0s : i
        let s_9_9: i128 = 0;
        // D s_9_10: cast zx s_9_8 -> i
        let s_9_10: i128 = (i128::try_from(s_9_8).unwrap());
        // D s_9_11: call integer_subrange(s_9_0, s_9_10, s_9_9)
        let s_9_11: Bits = integer_subrange(state, tracer, s_9_0, s_9_10, s_9_9);
        // D s_9_12: read-var e:i64
        let s_9_12: i64 = fn_state.e;
        // D s_9_13: cast zx s_9_12 -> i
        let s_9_13: i128 = (i128::try_from(s_9_12).unwrap());
        // D s_9_14: cast zx s_9_3 -> i
        let s_9_14: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_15: read-var result:bv
        let s_9_15: Bits = fn_state.result;
        // D s_9_16: call Elem_set(s_9_15, s_9_13, s_9_14, s_9_11)
        let s_9_16: Bits = Elem_set(state, tracer, s_9_15, s_9_13, s_9_14, s_9_11);
        // D s_9_17: write-var result <= s_9_16
        fn_state.result = s_9_16;
        // D s_9_18: read-var e:i64
        let s_9_18: i64 = fn_state.e;
        // C s_9_19: const #1s : i64
        let s_9_19: i64 = 1;
        // D s_9_20: add s_9_18 s_9_19
        let s_9_20: i64 = (s_9_18 + s_9_19);
        // D s_9_21: write-var e <= s_9_20
        fn_state.e = s_9_20;
        // N s_9_22: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var esizeshadow#4109:i64
        let s_10_0: i64 = fn_state.esizeshadow_4109;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: cast reint s_10_1 -> i64
        let s_10_2: i64 = (s_10_1 as i64);
        // D s_10_3: read-var e:i64
        let s_10_3: i64 = fn_state.e;
        // D s_10_4: cast zx s_10_3 -> i
        let s_10_4: i128 = (i128::try_from(s_10_3).unwrap());
        // D s_10_5: cast zx s_10_2 -> i
        let s_10_5: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_6: read-var operand1:bv
        let s_10_6: Bits = fn_state.operand1;
        // D s_10_7: call Elem_read(s_10_6, s_10_4, s_10_5)
        let s_10_7: Bits = Elem_read(state, tracer, s_10_6, s_10_4, s_10_5);
        // D s_10_8: write-var element1 <= s_10_7
        fn_state.element1 = s_10_7;
        // C s_10_9: const #0s : i64
        let s_10_9: i64 = 0;
        // D s_10_10: read-var e:i64
        let s_10_10: i64 = fn_state.e;
        // D s_10_11: write-var gs#216092 <= s_10_10
        fn_state.gs_216092 = s_10_10;
        // D s_10_12: write-var i <= s_10_9
        fn_state.i = s_10_9;
        // N s_10_13: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var i:i64
        let s_11_0: i64 = fn_state.i;
        // D s_11_1: read-var gs#216092:i64
        let s_11_1: i64 = fn_state.gs_216092;
        // D s_11_2: cmp-gt s_11_0 s_11_1
        let s_11_2: bool = ((s_11_0) > (s_11_1));
        // N s_11_3: branch s_11_2 b19 b12
        if s_11_2 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var i:i64
        let s_12_0: i64 = fn_state.i;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: read-var esizeshadow#4109:i64
        let s_12_2: i64 = fn_state.esizeshadow_4109;
        // D s_12_3: cast zx s_12_2 -> i
        let s_12_3: i128 = (i128::try_from(s_12_2).unwrap());
        // D s_12_4: read-var mask:bv
        let s_12_4: Bits = fn_state.mask;
        // D s_12_5: call ActivePredicateElement(s_12_4, s_12_1, s_12_3)
        let s_12_5: bool = ActivePredicateElement(state, tracer, s_12_4, s_12_1, s_12_3);
        // N s_12_6: branch s_12_5 b15 b13
        if s_12_5 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var i:i64
        let s_14_0: i64 = fn_state.i;
        // C s_14_1: const #1s : i64
        let s_14_1: i64 = 1;
        // D s_14_2: add s_14_0 s_14_1
        let s_14_2: i64 = (s_14_0 + s_14_1);
        // D s_14_3: write-var i <= s_14_2
        fn_state.i = s_14_2;
        // N s_14_4: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var esizeshadow#4109:i64
        let s_15_0: i64 = fn_state.esizeshadow_4109;
        // D s_15_1: cast zx s_15_0 -> i
        let s_15_1: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_2: cast reint s_15_1 -> i64
        let s_15_2: i64 = (s_15_1 as i64);
        // D s_15_3: read-var i:i64
        let s_15_3: i64 = fn_state.i;
        // D s_15_4: cast zx s_15_3 -> i
        let s_15_4: i128 = (i128::try_from(s_15_3).unwrap());
        // D s_15_5: cast zx s_15_2 -> i
        let s_15_5: i128 = (i128::try_from(s_15_2).unwrap());
        // D s_15_6: read-var operand2:bv
        let s_15_6: Bits = fn_state.operand2;
        // D s_15_7: call Elem_read(s_15_6, s_15_4, s_15_5)
        let s_15_7: Bits = Elem_read(state, tracer, s_15_6, s_15_4, s_15_5);
        // D s_15_8: read-var element1:bv
        let s_15_8: Bits = fn_state.element1;
        // D s_15_9: cmp-eq s_15_8 s_15_7
        let s_15_9: bool = ((s_15_8) == (s_15_7));
        // N s_15_10: branch s_15_9 b18 b16
        if s_15_9 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1s : i
        let s_18_0: i128 = 1;
        // D s_18_1: read-var count:i
        let s_18_1: i128 = fn_state.count;
        // D s_18_2: add s_18_1 s_18_0
        let s_18_2: i128 = (s_18_1 + s_18_0);
        // D s_18_3: write-var count <= s_18_2
        fn_state.count = s_18_2;
        // N s_18_4: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var VLshadow#4111:i64
        let s_20_0: i64 = fn_state.VLshadow_4111;
        // D s_20_1: cast zx s_20_0 -> i
        let s_20_1: i128 = (i128::try_from(s_20_0).unwrap());
        // D s_20_2: cast reint s_20_1 -> i64
        let s_20_2: i64 = (s_20_1 as i64);
        // D s_20_3: read-var d:i64
        let s_20_3: i64 = fn_state.d;
        // D s_20_4: cast zx s_20_3 -> i
        let s_20_4: i128 = (i128::try_from(s_20_3).unwrap());
        // D s_20_5: cast zx s_20_2 -> i
        let s_20_5: i128 = (i128::try_from(s_20_2).unwrap());
        // D s_20_6: read-var result:bv
        let s_20_6: Bits = fn_state.result;
        // D s_20_7: call Z_set(s_20_4, s_20_5, s_20_6)
        let s_20_7: () = Z_set(state, tracer, s_20_4, s_20_5, s_20_6);
        // N s_20_8: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var VLshadow#4111:i64
        let s_21_0: i64 = fn_state.VLshadow_4111;
        // D s_21_1: cast zx s_21_0 -> i
        let s_21_1: i128 = (i128::try_from(s_21_0).unwrap());
        // D s_21_2: cast reint s_21_1 -> i64
        let s_21_2: i64 = (s_21_1 as i64);
        // D s_21_3: read-var m:i64
        let s_21_3: i64 = fn_state.m;
        // D s_21_4: cast zx s_21_3 -> i
        let s_21_4: i128 = (i128::try_from(s_21_3).unwrap());
        // D s_21_5: cast zx s_21_2 -> i
        let s_21_5: i128 = (i128::try_from(s_21_2).unwrap());
        // D s_21_6: call Z_read(s_21_4, s_21_5)
        let s_21_6: Bits = Z_read(state, tracer, s_21_4, s_21_5);
        // D s_21_7: write-var operand2 <= s_21_6
        fn_state.operand2 = s_21_6;
        // N s_21_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var VLshadow#4111:i64
        let s_22_0: i64 = fn_state.VLshadow_4111;
        // D s_22_1: cast zx s_22_0 -> i
        let s_22_1: i128 = (i128::try_from(s_22_0).unwrap());
        // D s_22_2: cast reint s_22_1 -> i64
        let s_22_2: i64 = (s_22_1 as i64);
        // D s_22_3: read-var n:i64
        let s_22_3: i64 = fn_state.n;
        // D s_22_4: cast zx s_22_3 -> i
        let s_22_4: i128 = (i128::try_from(s_22_3).unwrap());
        // D s_22_5: cast zx s_22_2 -> i
        let s_22_5: i128 = (i128::try_from(s_22_2).unwrap());
        // D s_22_6: call Z_read(s_22_4, s_22_5)
        let s_22_6: Bits = Z_read(state, tracer, s_22_4, s_22_5);
        // D s_22_7: write-var operand1 <= s_22_6
        fn_state.operand1 = s_22_6;
        // N s_22_8: jump b3
        return block_3(state, tracer, fn_state);
    }
}
