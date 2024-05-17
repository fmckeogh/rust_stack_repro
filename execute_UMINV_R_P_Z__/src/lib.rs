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
use V_set::*;
use CheckSVEEnabled::*;
use AnyActiveElement::*;
use P_read::*;
use ActivePredicateElement::*;
use integer_subrange::*;
use asl_Int::*;
use Elem_read::*;
use Zeros::*;
use Z_read::*;
use common::*;
pub fn execute_UMINV_R_P_Z__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    g: i64,
    n: i64,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        VLshadow_2843: i64,
        esizeshadow_2841: i64,
        minimum: i128,
        e: i64,
        elements: i64,
        gs_193865: i64,
        VLshadow_2842: i64,
        mask: Bits,
        VL: i64,
        d: i64,
        esize: i64,
        g: i64,
        n: i64,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        g,
        n,
        is_unsigned,
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
        // D s_0_3: write-var esizeshadow#2841 <= s_0_2
        fn_state.esizeshadow_2841 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#2842 <= s_0_6
        fn_state.VLshadow_2842 = s_0_6;
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
        // D s_1_0: read-var VLshadow#2842:i64
        let s_1_0: i64 = fn_state.VLshadow_2842;
        // D s_1_1: write-var VLshadow#2843 <= s_1_0
        fn_state.VLshadow_2843 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#2843:i64
        let s_1_3: i64 = fn_state.VLshadow_2843;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#2843:i64
        let s_1_7: i64 = fn_state.VLshadow_2843;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: read-var esizeshadow#2841:i64
        let s_1_9: i64 = fn_state.esizeshadow_2841;
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
        // D s_1_21: read-var esizeshadow#2841:i64
        let s_1_21: i64 = fn_state.esizeshadow_2841;
        // D s_1_22: cast zx s_1_21 -> i
        let s_1_22: i128 = (i128::try_from(s_1_21).unwrap());
        // D s_1_23: read-var mask:bv
        let s_1_23: Bits = fn_state.mask;
        // D s_1_24: call AnyActiveElement(s_1_23, s_1_22)
        let s_1_24: bool = AnyActiveElement(state, tracer, s_1_23, s_1_22);
        // N s_1_25: branch s_1_24 b13 b2
        if s_1_24 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var VLshadow#2843:i64
        let s_2_0: i64 = fn_state.VLshadow_2843;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: call Zeros(s_2_1)
        let s_2_2: Bits = Zeros(state, tracer, s_2_1);
        // D s_2_3: write-var operand <= s_2_2
        fn_state.operand = s_2_2;
        // N s_2_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var is_unsigned:u8
        let s_3_0: bool = fn_state.is_unsigned;
        // N s_3_1: branch s_3_0 b12 b4
        if s_3_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #1s : i
        let s_4_0: i128 = 1;
        // D s_4_1: read-var esizeshadow#2841:i64
        let s_4_1: i64 = fn_state.esizeshadow_2841;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: sub s_4_2 s_4_0
        let s_4_3: i128 = ((s_4_2) - (s_4_0));
        // D s_4_4: cast reint s_4_3 -> i64
        let s_4_4: i64 = (s_4_3 as i64);
        // D s_4_5: cast zx s_4_4 -> i
        let s_4_5: i128 = (i128::try_from(s_4_4).unwrap());
        // D s_4_6: pow2 s_4_5
        let s_4_6: i128 = (s_4_5).pow(2);
        // C s_4_7: const #1s : i
        let s_4_7: i128 = 1;
        // D s_4_8: sub s_4_6 s_4_7
        let s_4_8: i128 = ((s_4_6) - (s_4_7));
        // D s_4_9: write-var minimum <= s_4_8
        fn_state.minimum = s_4_8;
        // N s_4_10: jump b5
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
        // D s_5_6: write-var gs#193865 <= s_5_5
        fn_state.gs_193865 = s_5_5;
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
        // D s_6_1: read-var gs#193865:i64
        let s_6_1: i64 = fn_state.gs_193865;
        // D s_6_2: cmp-gt s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) > (s_6_1));
        // N s_6_3: branch s_6_2 b11 b7
        if s_6_2 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var e:i64
        let s_7_0: i64 = fn_state.e;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: read-var esizeshadow#2841:i64
        let s_7_2: i64 = fn_state.esizeshadow_2841;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: read-var mask:bv
        let s_7_4: Bits = fn_state.mask;
        // D s_7_5: call ActivePredicateElement(s_7_4, s_7_1, s_7_3)
        let s_7_5: bool = ActivePredicateElement(state, tracer, s_7_4, s_7_1, s_7_3);
        // N s_7_6: branch s_7_5 b10 b8
        if s_7_5 {
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
        // D s_9_0: read-var e:i64
        let s_9_0: i64 = fn_state.e;
        // C s_9_1: const #1s : i64
        let s_9_1: i64 = 1;
        // D s_9_2: add s_9_0 s_9_1
        let s_9_2: i64 = (s_9_0 + s_9_1);
        // D s_9_3: write-var e <= s_9_2
        fn_state.e = s_9_2;
        // N s_9_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var esizeshadow#2841:i64
        let s_10_0: i64 = fn_state.esizeshadow_2841;
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
        // D s_10_6: read-var operand:bv
        let s_10_6: Bits = fn_state.operand;
        // D s_10_7: call Elem_read(s_10_6, s_10_4, s_10_5)
        let s_10_7: Bits = Elem_read(state, tracer, s_10_6, s_10_4, s_10_5);
        // D s_10_8: read-var is_unsigned:u8
        let s_10_8: bool = fn_state.is_unsigned;
        // D s_10_9: call asl_Int(s_10_7, s_10_8)
        let s_10_9: i128 = asl_Int(state, tracer, s_10_7, s_10_8);
        // D s_10_10: read-var minimum:i
        let s_10_10: i128 = fn_state.minimum;
        // D s_10_11: cmp-lt s_10_10 s_10_9
        let s_10_11: bool = ((s_10_10) < (s_10_9));
        // D s_10_12: select s_10_11 s_10_10 s_10_9
        let s_10_12: i128 = if s_10_11 { s_10_10 } else { s_10_9 };
        // D s_10_13: write-var minimum <= s_10_12
        fn_state.minimum = s_10_12;
        // N s_10_14: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var minimum:i
        let s_11_0: i128 = fn_state.minimum;
        // D s_11_1: read-var esizeshadow#2841:i64
        let s_11_1: i64 = fn_state.esizeshadow_2841;
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // D s_11_3: cast reint s_11_2 -> i64
        let s_11_3: i64 = (s_11_2 as i64);
        // C s_11_4: const #1s : i
        let s_11_4: i128 = 1;
        // D s_11_5: read-var esizeshadow#2841:i64
        let s_11_5: i64 = fn_state.esizeshadow_2841;
        // D s_11_6: cast zx s_11_5 -> i
        let s_11_6: i128 = (i128::try_from(s_11_5).unwrap());
        // D s_11_7: sub s_11_6 s_11_4
        let s_11_7: i128 = ((s_11_6) - (s_11_4));
        // D s_11_8: cast reint s_11_7 -> i64
        let s_11_8: i64 = (s_11_7 as i64);
        // C s_11_9: const #0s : i
        let s_11_9: i128 = 0;
        // D s_11_10: cast zx s_11_8 -> i
        let s_11_10: i128 = (i128::try_from(s_11_8).unwrap());
        // D s_11_11: call integer_subrange(s_11_0, s_11_10, s_11_9)
        let s_11_11: Bits = integer_subrange(state, tracer, s_11_0, s_11_10, s_11_9);
        // D s_11_12: read-var d:i64
        let s_11_12: i64 = fn_state.d;
        // D s_11_13: cast zx s_11_12 -> i
        let s_11_13: i128 = (i128::try_from(s_11_12).unwrap());
        // D s_11_14: call V_set(s_11_13, s_11_3, s_11_11)
        let s_11_14: () = V_set(state, tracer, s_11_13, s_11_3, s_11_11);
        // N s_11_15: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var esizeshadow#2841:i64
        let s_12_0: i64 = fn_state.esizeshadow_2841;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: pow2 s_12_1
        let s_12_2: i128 = (s_12_1).pow(2);
        // C s_12_3: const #1s : i
        let s_12_3: i128 = 1;
        // D s_12_4: sub s_12_2 s_12_3
        let s_12_4: i128 = ((s_12_2) - (s_12_3));
        // D s_12_5: write-var minimum <= s_12_4
        fn_state.minimum = s_12_4;
        // N s_12_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var VLshadow#2843:i64
        let s_13_0: i64 = fn_state.VLshadow_2843;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: cast reint s_13_1 -> i64
        let s_13_2: i64 = (s_13_1 as i64);
        // D s_13_3: read-var n:i64
        let s_13_3: i64 = fn_state.n;
        // D s_13_4: cast zx s_13_3 -> i
        let s_13_4: i128 = (i128::try_from(s_13_3).unwrap());
        // D s_13_5: cast zx s_13_2 -> i
        let s_13_5: i128 = (i128::try_from(s_13_2).unwrap());
        // D s_13_6: call Z_read(s_13_4, s_13_5)
        let s_13_6: Bits = Z_read(state, tracer, s_13_4, s_13_5);
        // D s_13_7: write-var operand <= s_13_6
        fn_state.operand = s_13_6;
        // N s_13_8: jump b3
        return block_3(state, tracer, fn_state);
    }
}
