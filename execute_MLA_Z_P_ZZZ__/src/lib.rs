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
use Elem_read::*;
use Zeros::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_MLA_Z_P_ZZZ__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    da: i64,
    esize: i64,
    g: i64,
    m: i64,
    n: i64,
    sub_op: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        VLshadow_2874: i64,
        product: i128,
        e: i64,
        operand3: Bits,
        gs_194446: i64,
        mask: Bits,
        VLshadow_2875: i64,
        elements: i64,
        result: Bits,
        operand1: Bits,
        esizeshadow_2873: i64,
        operand2: Bits,
        VL: i64,
        da: i64,
        esize: i64,
        g: i64,
        m: i64,
        n: i64,
        sub_op: bool,
    }
    let fn_state = FunctionState {
        VL,
        da,
        esize,
        g,
        m,
        n,
        sub_op,
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
        // D s_0_3: write-var esizeshadow#2873 <= s_0_2
        fn_state.esizeshadow_2873 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#2874 <= s_0_6
        fn_state.VLshadow_2874 = s_0_6;
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
        // D s_1_0: read-var VLshadow#2874:i64
        let s_1_0: i64 = fn_state.VLshadow_2874;
        // D s_1_1: write-var VLshadow#2875 <= s_1_0
        fn_state.VLshadow_2875 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#2875:i64
        let s_1_3: i64 = fn_state.VLshadow_2875;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#2875:i64
        let s_1_7: i64 = fn_state.VLshadow_2875;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: read-var esizeshadow#2873:i64
        let s_1_9: i64 = fn_state.esizeshadow_2873;
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
        // D s_1_21: read-var esizeshadow#2873:i64
        let s_1_21: i64 = fn_state.esizeshadow_2873;
        // D s_1_22: cast zx s_1_21 -> i
        let s_1_22: i128 = (i128::try_from(s_1_21).unwrap());
        // D s_1_23: read-var mask:bv
        let s_1_23: Bits = fn_state.mask;
        // D s_1_24: call AnyActiveElement(s_1_23, s_1_22)
        let s_1_24: bool = AnyActiveElement(state, tracer, s_1_23, s_1_22);
        // N s_1_25: branch s_1_24 b16 b2
        if s_1_24 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var VLshadow#2875:i64
        let s_2_0: i64 = fn_state.VLshadow_2875;
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
        // D s_3_0: read-var esizeshadow#2873:i64
        let s_3_0: i64 = fn_state.esizeshadow_2873;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var mask:bv
        let s_3_2: Bits = fn_state.mask;
        // D s_3_3: call AnyActiveElement(s_3_2, s_3_1)
        let s_3_3: bool = AnyActiveElement(state, tracer, s_3_2, s_3_1);
        // N s_3_4: branch s_3_3 b15 b4
        if s_3_3 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var VLshadow#2875:i64
        let s_4_0: i64 = fn_state.VLshadow_2875;
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
        // D s_5_0: read-var VLshadow#2875:i64
        let s_5_0: i64 = fn_state.VLshadow_2875;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: cast reint s_5_1 -> i64
        let s_5_2: i64 = (s_5_1 as i64);
        // D s_5_3: read-var da:i64
        let s_5_3: i64 = fn_state.da;
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_5: cast zx s_5_2 -> i
        let s_5_5: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_6: call Z_read(s_5_4, s_5_5)
        let s_5_6: Bits = Z_read(state, tracer, s_5_4, s_5_5);
        // D s_5_7: write-var operand3 <= s_5_6
        fn_state.operand3 = s_5_6;
        // C s_5_8: const #0s : i64
        let s_5_8: i64 = 0;
        // C s_5_9: const #1s : i
        let s_5_9: i128 = 1;
        // D s_5_10: read-var elements:i64
        let s_5_10: i64 = fn_state.elements;
        // D s_5_11: cast zx s_5_10 -> i
        let s_5_11: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_12: sub s_5_11 s_5_9
        let s_5_12: i128 = ((s_5_11) - (s_5_9));
        // D s_5_13: cast reint s_5_12 -> i64
        let s_5_13: i64 = (s_5_12 as i64);
        // D s_5_14: write-var gs#194446 <= s_5_13
        fn_state.gs_194446 = s_5_13;
        // D s_5_15: write-var e <= s_5_8
        fn_state.e = s_5_8;
        // N s_5_16: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var e:i64
        let s_6_0: i64 = fn_state.e;
        // D s_6_1: read-var gs#194446:i64
        let s_6_1: i64 = fn_state.gs_194446;
        // D s_6_2: cmp-gt s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) > (s_6_1));
        // N s_6_3: branch s_6_2 b14 b7
        if s_6_2 {
            return block_14(state, tracer, fn_state);
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
        // D s_7_2: read-var esizeshadow#2873:i64
        let s_7_2: i64 = fn_state.esizeshadow_2873;
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
        // D s_8_0: read-var esizeshadow#2873:i64
        let s_8_0: i64 = fn_state.esizeshadow_2873;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: cast reint s_8_1 -> i64
        let s_8_2: i64 = (s_8_1 as i64);
        // D s_8_3: read-var esizeshadow#2873:i64
        let s_8_3: i64 = fn_state.esizeshadow_2873;
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
        // D s_8_9: read-var operand3:bv
        let s_8_9: Bits = fn_state.operand3;
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
        // N s_8_17: jump b9
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
        // D s_10_0: read-var esizeshadow#2873:i64
        let s_10_0: i64 = fn_state.esizeshadow_2873;
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
        // D s_10_8: cast zx s_10_7 -> i
        let s_10_8: i128 = (s_10_7.value() as i128);
        // D s_10_9: read-var esizeshadow#2873:i64
        let s_10_9: i64 = fn_state.esizeshadow_2873;
        // D s_10_10: cast zx s_10_9 -> i
        let s_10_10: i128 = (i128::try_from(s_10_9).unwrap());
        // D s_10_11: cast reint s_10_10 -> i64
        let s_10_11: i64 = (s_10_10 as i64);
        // D s_10_12: read-var e:i64
        let s_10_12: i64 = fn_state.e;
        // D s_10_13: cast zx s_10_12 -> i
        let s_10_13: i128 = (i128::try_from(s_10_12).unwrap());
        // D s_10_14: cast zx s_10_11 -> i
        let s_10_14: i128 = (i128::try_from(s_10_11).unwrap());
        // D s_10_15: read-var operand2:bv
        let s_10_15: Bits = fn_state.operand2;
        // D s_10_16: call Elem_read(s_10_15, s_10_13, s_10_14)
        let s_10_16: Bits = Elem_read(state, tracer, s_10_15, s_10_13, s_10_14);
        // D s_10_17: cast zx s_10_16 -> i
        let s_10_17: i128 = (s_10_16.value() as i128);
        // D s_10_18: mul s_10_8 s_10_17
        let s_10_18: i128 = ((s_10_8) * (s_10_17));
        // D s_10_19: write-var product <= s_10_18
        fn_state.product = s_10_18;
        // D s_10_20: read-var sub_op:u8
        let s_10_20: bool = fn_state.sub_op;
        // N s_10_21: branch s_10_20 b13 b11
        if s_10_20 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var esizeshadow#2873:i64
        let s_11_0: i64 = fn_state.esizeshadow_2873;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: cast reint s_11_1 -> i64
        let s_11_2: i64 = (s_11_1 as i64);
        // D s_11_3: read-var esizeshadow#2873:i64
        let s_11_3: i64 = fn_state.esizeshadow_2873;
        // D s_11_4: cast zx s_11_3 -> i
        let s_11_4: i128 = (i128::try_from(s_11_3).unwrap());
        // D s_11_5: cast reint s_11_4 -> i64
        let s_11_5: i64 = (s_11_4 as i64);
        // D s_11_6: read-var e:i64
        let s_11_6: i64 = fn_state.e;
        // D s_11_7: cast zx s_11_6 -> i
        let s_11_7: i128 = (i128::try_from(s_11_6).unwrap());
        // D s_11_8: cast zx s_11_5 -> i
        let s_11_8: i128 = (i128::try_from(s_11_5).unwrap());
        // D s_11_9: read-var operand3:bv
        let s_11_9: Bits = fn_state.operand3;
        // D s_11_10: call Elem_read(s_11_9, s_11_7, s_11_8)
        let s_11_10: Bits = Elem_read(state, tracer, s_11_9, s_11_7, s_11_8);
        // D s_11_11: read-var product:i
        let s_11_11: i128 = fn_state.product;
        // D s_11_12: cast cvt s_11_11 -> bv
        let s_11_12: Bits = Bits::new(s_11_11 as u128, 128);
        // D s_11_13: add s_11_10 s_11_12
        let s_11_13: Bits = (s_11_10 + s_11_12);
        // D s_11_14: read-var e:i64
        let s_11_14: i64 = fn_state.e;
        // D s_11_15: cast zx s_11_14 -> i
        let s_11_15: i128 = (i128::try_from(s_11_14).unwrap());
        // D s_11_16: cast zx s_11_2 -> i
        let s_11_16: i128 = (i128::try_from(s_11_2).unwrap());
        // D s_11_17: read-var result:bv
        let s_11_17: Bits = fn_state.result;
        // D s_11_18: call Elem_set(s_11_17, s_11_15, s_11_16, s_11_13)
        let s_11_18: Bits = Elem_set(state, tracer, s_11_17, s_11_15, s_11_16, s_11_13);
        // D s_11_19: write-var result <= s_11_18
        fn_state.result = s_11_18;
        // N s_11_20: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var esizeshadow#2873:i64
        let s_13_0: i64 = fn_state.esizeshadow_2873;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: cast reint s_13_1 -> i64
        let s_13_2: i64 = (s_13_1 as i64);
        // D s_13_3: read-var esizeshadow#2873:i64
        let s_13_3: i64 = fn_state.esizeshadow_2873;
        // D s_13_4: cast zx s_13_3 -> i
        let s_13_4: i128 = (i128::try_from(s_13_3).unwrap());
        // D s_13_5: cast reint s_13_4 -> i64
        let s_13_5: i64 = (s_13_4 as i64);
        // D s_13_6: read-var e:i64
        let s_13_6: i64 = fn_state.e;
        // D s_13_7: cast zx s_13_6 -> i
        let s_13_7: i128 = (i128::try_from(s_13_6).unwrap());
        // D s_13_8: cast zx s_13_5 -> i
        let s_13_8: i128 = (i128::try_from(s_13_5).unwrap());
        // D s_13_9: read-var operand3:bv
        let s_13_9: Bits = fn_state.operand3;
        // D s_13_10: call Elem_read(s_13_9, s_13_7, s_13_8)
        let s_13_10: Bits = Elem_read(state, tracer, s_13_9, s_13_7, s_13_8);
        // D s_13_11: read-var product:i
        let s_13_11: i128 = fn_state.product;
        // D s_13_12: cast cvt s_13_11 -> bv
        let s_13_12: Bits = Bits::new(s_13_11 as u128, 128);
        // D s_13_13: sub s_13_10 s_13_12
        let s_13_13: Bits = ((s_13_10) - (s_13_12));
        // D s_13_14: read-var e:i64
        let s_13_14: i64 = fn_state.e;
        // D s_13_15: cast zx s_13_14 -> i
        let s_13_15: i128 = (i128::try_from(s_13_14).unwrap());
        // D s_13_16: cast zx s_13_2 -> i
        let s_13_16: i128 = (i128::try_from(s_13_2).unwrap());
        // D s_13_17: read-var result:bv
        let s_13_17: Bits = fn_state.result;
        // D s_13_18: call Elem_set(s_13_17, s_13_15, s_13_16, s_13_13)
        let s_13_18: Bits = Elem_set(state, tracer, s_13_17, s_13_15, s_13_16, s_13_13);
        // D s_13_19: write-var result <= s_13_18
        fn_state.result = s_13_18;
        // N s_13_20: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var VLshadow#2875:i64
        let s_14_0: i64 = fn_state.VLshadow_2875;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: cast reint s_14_1 -> i64
        let s_14_2: i64 = (s_14_1 as i64);
        // D s_14_3: read-var da:i64
        let s_14_3: i64 = fn_state.da;
        // D s_14_4: cast zx s_14_3 -> i
        let s_14_4: i128 = (i128::try_from(s_14_3).unwrap());
        // D s_14_5: cast zx s_14_2 -> i
        let s_14_5: i128 = (i128::try_from(s_14_2).unwrap());
        // D s_14_6: read-var result:bv
        let s_14_6: Bits = fn_state.result;
        // D s_14_7: call Z_set(s_14_4, s_14_5, s_14_6)
        let s_14_7: () = Z_set(state, tracer, s_14_4, s_14_5, s_14_6);
        // N s_14_8: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var VLshadow#2875:i64
        let s_15_0: i64 = fn_state.VLshadow_2875;
        // D s_15_1: cast zx s_15_0 -> i
        let s_15_1: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_2: cast reint s_15_1 -> i64
        let s_15_2: i64 = (s_15_1 as i64);
        // D s_15_3: read-var m:i64
        let s_15_3: i64 = fn_state.m;
        // D s_15_4: cast zx s_15_3 -> i
        let s_15_4: i128 = (i128::try_from(s_15_3).unwrap());
        // D s_15_5: cast zx s_15_2 -> i
        let s_15_5: i128 = (i128::try_from(s_15_2).unwrap());
        // D s_15_6: call Z_read(s_15_4, s_15_5)
        let s_15_6: Bits = Z_read(state, tracer, s_15_4, s_15_5);
        // D s_15_7: write-var operand2 <= s_15_6
        fn_state.operand2 = s_15_6;
        // N s_15_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var VLshadow#2875:i64
        let s_16_0: i64 = fn_state.VLshadow_2875;
        // D s_16_1: cast zx s_16_0 -> i
        let s_16_1: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_2: cast reint s_16_1 -> i64
        let s_16_2: i64 = (s_16_1 as i64);
        // D s_16_3: read-var n:i64
        let s_16_3: i64 = fn_state.n;
        // D s_16_4: cast zx s_16_3 -> i
        let s_16_4: i128 = (i128::try_from(s_16_3).unwrap());
        // D s_16_5: cast zx s_16_2 -> i
        let s_16_5: i128 = (i128::try_from(s_16_2).unwrap());
        // D s_16_6: call Z_read(s_16_4, s_16_5)
        let s_16_6: Bits = Z_read(state, tracer, s_16_4, s_16_5);
        // D s_16_7: write-var operand1 <= s_16_6
        fn_state.operand1 = s_16_6;
        // N s_16_8: jump b3
        return block_3(state, tracer, fn_state);
    }
}
