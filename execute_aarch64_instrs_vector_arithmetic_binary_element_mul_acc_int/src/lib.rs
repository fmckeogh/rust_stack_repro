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
use V_read::*;
use Elem_read::*;
use V_set::*;
use integer_subrange::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_arithmetic_binary_element_mul_acc_int<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    elements: i128,
    esize: i64,
    idxdsize: i64,
    index: i64,
    m: i64,
    n: i64,
    sub_op: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        esizeshadow_1755: i64,
        element2: i128,
        product: Bits,
        e: i64,
        operand3: Bits,
        gs_165087: i64,
        idxdsizeshadow_1754: i64,
        datasizeshadow_1756: i64,
        result: Bits,
        operand1: Bits,
        d: i64,
        datasize: i64,
        elements: i128,
        esize: i64,
        idxdsize: i64,
        index: i64,
        m: i64,
        n: i64,
        sub_op: bool,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        elements,
        esize,
        idxdsize,
        index,
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
        // D s_0_0: read-var idxdsize:i64
        let s_0_0: i64 = fn_state.idxdsize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var idxdsizeshadow#1754 <= s_0_2
        fn_state.idxdsizeshadow_1754 = s_0_2;
        // D s_0_4: read-var esize:i64
        let s_0_4: i64 = fn_state.esize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var esizeshadow#1755 <= s_0_6
        fn_state.esizeshadow_1755 = s_0_6;
        // D s_0_8: read-var datasize:i64
        let s_0_8: i64 = fn_state.datasize;
        // D s_0_9: cast zx s_0_8 -> i
        let s_0_9: i128 = (i128::try_from(s_0_8).unwrap());
        // D s_0_10: cast reint s_0_9 -> i64
        let s_0_10: i64 = (s_0_9 as i64);
        // D s_0_11: write-var datasizeshadow#1756 <= s_0_10
        fn_state.datasizeshadow_1756 = s_0_10;
        // C s_0_12: const #() : ()
        let s_0_12: () = ();
        // S s_0_13: call CheckFPAdvSIMDEnabled64(s_0_12)
        let s_0_13: () = CheckFPAdvSIMDEnabled64(state, tracer, s_0_12);
        // N s_0_14: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var datasizeshadow#1756:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1756;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: cast reint s_1_1 -> i64
        let s_1_2: i64 = (s_1_1 as i64);
        // D s_1_3: read-var n:i64
        let s_1_3: i64 = fn_state.n;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: call V_read(s_1_4, s_1_2)
        let s_1_5: Bits = V_read(state, tracer, s_1_4, s_1_2);
        // D s_1_6: write-var operand1 <= s_1_5
        fn_state.operand1 = s_1_5;
        // D s_1_7: read-var idxdsizeshadow#1754:i64
        let s_1_7: i64 = fn_state.idxdsizeshadow_1754;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast reint s_1_8 -> i64
        let s_1_9: i64 = (s_1_8 as i64);
        // D s_1_10: read-var m:i64
        let s_1_10: i64 = fn_state.m;
        // D s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_12: call V_read(s_1_11, s_1_9)
        let s_1_12: Bits = V_read(state, tracer, s_1_11, s_1_9);
        // D s_1_13: read-var datasizeshadow#1756:i64
        let s_1_13: i64 = fn_state.datasizeshadow_1756;
        // D s_1_14: cast zx s_1_13 -> i
        let s_1_14: i128 = (i128::try_from(s_1_13).unwrap());
        // D s_1_15: cast reint s_1_14 -> i64
        let s_1_15: i64 = (s_1_14 as i64);
        // D s_1_16: read-var d:i64
        let s_1_16: i64 = fn_state.d;
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_18: call V_read(s_1_17, s_1_15)
        let s_1_18: Bits = V_read(state, tracer, s_1_17, s_1_15);
        // D s_1_19: write-var operand3 <= s_1_18
        fn_state.operand3 = s_1_18;
        // D s_1_20: read-var esizeshadow#1755:i64
        let s_1_20: i64 = fn_state.esizeshadow_1755;
        // D s_1_21: cast zx s_1_20 -> i
        let s_1_21: i128 = (i128::try_from(s_1_20).unwrap());
        // D s_1_22: cast reint s_1_21 -> i64
        let s_1_22: i64 = (s_1_21 as i64);
        // D s_1_23: read-var index:i64
        let s_1_23: i64 = fn_state.index;
        // D s_1_24: cast zx s_1_23 -> i
        let s_1_24: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_25: cast zx s_1_22 -> i
        let s_1_25: i128 = (i128::try_from(s_1_22).unwrap());
        // D s_1_26: call Elem_read(s_1_12, s_1_24, s_1_25)
        let s_1_26: Bits = Elem_read(state, tracer, s_1_12, s_1_24, s_1_25);
        // D s_1_27: cast zx s_1_26 -> i
        let s_1_27: i128 = (s_1_26.value() as i128);
        // D s_1_28: write-var element2 <= s_1_27
        fn_state.element2 = s_1_27;
        // C s_1_29: const #0s : i64
        let s_1_29: i64 = 0;
        // C s_1_30: const #1s : i
        let s_1_30: i128 = 1;
        // D s_1_31: read-var elements:i
        let s_1_31: i128 = fn_state.elements;
        // D s_1_32: sub s_1_31 s_1_30
        let s_1_32: i128 = ((s_1_31) - (s_1_30));
        // D s_1_33: cast reint s_1_32 -> i64
        let s_1_33: i64 = (s_1_32 as i64);
        // D s_1_34: write-var gs#165087 <= s_1_33
        fn_state.gs_165087 = s_1_33;
        // D s_1_35: write-var e <= s_1_29
        fn_state.e = s_1_29;
        // N s_1_36: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#165087:i64
        let s_2_1: i64 = fn_state.gs_165087;
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
        // D s_3_0: read-var esizeshadow#1755:i64
        let s_3_0: i64 = fn_state.esizeshadow_1755;
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
        // D s_3_6: read-var operand1:bv
        let s_3_6: Bits = fn_state.operand1;
        // D s_3_7: call Elem_read(s_3_6, s_3_4, s_3_5)
        let s_3_7: Bits = Elem_read(state, tracer, s_3_6, s_3_4, s_3_5);
        // D s_3_8: cast zx s_3_7 -> i
        let s_3_8: i128 = (s_3_7.value() as i128);
        // D s_3_9: read-var element2:i
        let s_3_9: i128 = fn_state.element2;
        // D s_3_10: mul s_3_8 s_3_9
        let s_3_10: i128 = ((s_3_8) * (s_3_9));
        // C s_3_11: const #1s : i
        let s_3_11: i128 = 1;
        // D s_3_12: read-var esizeshadow#1755:i64
        let s_3_12: i64 = fn_state.esizeshadow_1755;
        // D s_3_13: cast zx s_3_12 -> i
        let s_3_13: i128 = (i128::try_from(s_3_12).unwrap());
        // D s_3_14: sub s_3_13 s_3_11
        let s_3_14: i128 = ((s_3_13) - (s_3_11));
        // D s_3_15: cast reint s_3_14 -> i64
        let s_3_15: i64 = (s_3_14 as i64);
        // C s_3_16: const #0s : i
        let s_3_16: i128 = 0;
        // D s_3_17: cast zx s_3_15 -> i
        let s_3_17: i128 = (i128::try_from(s_3_15).unwrap());
        // D s_3_18: call integer_subrange(s_3_10, s_3_17, s_3_16)
        let s_3_18: Bits = integer_subrange(state, tracer, s_3_10, s_3_17, s_3_16);
        // D s_3_19: write-var product <= s_3_18
        fn_state.product = s_3_18;
        // D s_3_20: read-var sub_op:u8
        let s_3_20: bool = fn_state.sub_op;
        // N s_3_21: branch s_3_20 b6 b4
        if s_3_20 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var esizeshadow#1755:i64
        let s_4_0: i64 = fn_state.esizeshadow_1755;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // D s_4_3: read-var esizeshadow#1755:i64
        let s_4_3: i64 = fn_state.esizeshadow_1755;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: cast reint s_4_4 -> i64
        let s_4_5: i64 = (s_4_4 as i64);
        // D s_4_6: read-var e:i64
        let s_4_6: i64 = fn_state.e;
        // D s_4_7: cast zx s_4_6 -> i
        let s_4_7: i128 = (i128::try_from(s_4_6).unwrap());
        // D s_4_8: cast zx s_4_5 -> i
        let s_4_8: i128 = (i128::try_from(s_4_5).unwrap());
        // D s_4_9: read-var operand3:bv
        let s_4_9: Bits = fn_state.operand3;
        // D s_4_10: call Elem_read(s_4_9, s_4_7, s_4_8)
        let s_4_10: Bits = Elem_read(state, tracer, s_4_9, s_4_7, s_4_8);
        // D s_4_11: read-var product:bv
        let s_4_11: Bits = fn_state.product;
        // D s_4_12: add s_4_10 s_4_11
        let s_4_12: Bits = (s_4_10 + s_4_11);
        // D s_4_13: read-var e:i64
        let s_4_13: i64 = fn_state.e;
        // D s_4_14: cast zx s_4_13 -> i
        let s_4_14: i128 = (i128::try_from(s_4_13).unwrap());
        // D s_4_15: cast zx s_4_2 -> i
        let s_4_15: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_16: read-var result:bv
        let s_4_16: Bits = fn_state.result;
        // D s_4_17: call Elem_set(s_4_16, s_4_14, s_4_15, s_4_12)
        let s_4_17: Bits = Elem_set(state, tracer, s_4_16, s_4_14, s_4_15, s_4_12);
        // D s_4_18: write-var result <= s_4_17
        fn_state.result = s_4_17;
        // N s_4_19: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var e:i64
        let s_5_0: i64 = fn_state.e;
        // C s_5_1: const #1s : i64
        let s_5_1: i64 = 1;
        // D s_5_2: add s_5_0 s_5_1
        let s_5_2: i64 = (s_5_0 + s_5_1);
        // D s_5_3: write-var e <= s_5_2
        fn_state.e = s_5_2;
        // N s_5_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var esizeshadow#1755:i64
        let s_6_0: i64 = fn_state.esizeshadow_1755;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: cast reint s_6_1 -> i64
        let s_6_2: i64 = (s_6_1 as i64);
        // D s_6_3: read-var esizeshadow#1755:i64
        let s_6_3: i64 = fn_state.esizeshadow_1755;
        // D s_6_4: cast zx s_6_3 -> i
        let s_6_4: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_5: cast reint s_6_4 -> i64
        let s_6_5: i64 = (s_6_4 as i64);
        // D s_6_6: read-var e:i64
        let s_6_6: i64 = fn_state.e;
        // D s_6_7: cast zx s_6_6 -> i
        let s_6_7: i128 = (i128::try_from(s_6_6).unwrap());
        // D s_6_8: cast zx s_6_5 -> i
        let s_6_8: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_9: read-var operand3:bv
        let s_6_9: Bits = fn_state.operand3;
        // D s_6_10: call Elem_read(s_6_9, s_6_7, s_6_8)
        let s_6_10: Bits = Elem_read(state, tracer, s_6_9, s_6_7, s_6_8);
        // D s_6_11: read-var product:bv
        let s_6_11: Bits = fn_state.product;
        // D s_6_12: sub s_6_10 s_6_11
        let s_6_12: Bits = ((s_6_10) - (s_6_11));
        // D s_6_13: read-var e:i64
        let s_6_13: i64 = fn_state.e;
        // D s_6_14: cast zx s_6_13 -> i
        let s_6_14: i128 = (i128::try_from(s_6_13).unwrap());
        // D s_6_15: cast zx s_6_2 -> i
        let s_6_15: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_16: read-var result:bv
        let s_6_16: Bits = fn_state.result;
        // D s_6_17: call Elem_set(s_6_16, s_6_14, s_6_15, s_6_12)
        let s_6_17: Bits = Elem_set(state, tracer, s_6_16, s_6_14, s_6_15, s_6_12);
        // D s_6_18: write-var result <= s_6_17
        fn_state.result = s_6_17;
        // N s_6_19: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var datasizeshadow#1756:i64
        let s_7_0: i64 = fn_state.datasizeshadow_1756;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: cast reint s_7_1 -> i64
        let s_7_2: i64 = (s_7_1 as i64);
        // D s_7_3: read-var d:i64
        let s_7_3: i64 = fn_state.d;
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_5: read-var result:bv
        let s_7_5: Bits = fn_state.result;
        // D s_7_6: call V_set(s_7_4, s_7_2, s_7_5)
        let s_7_6: () = V_set(state, tracer, s_7_4, s_7_2, s_7_5);
        // N s_7_7: return
        return;
    }
}