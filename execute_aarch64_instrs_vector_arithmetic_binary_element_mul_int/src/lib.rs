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
pub fn execute_aarch64_instrs_vector_arithmetic_binary_element_mul_int<T: Tracer>(
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
) -> () {
    #[derive(Default)]
    struct FunctionState {
        element2: i128,
        e: i64,
        idxdsizeshadow_1769: i64,
        result: Bits,
        operand1: Bits,
        gs_165481: i64,
        esizeshadow_1770: i64,
        datasizeshadow_1771: i64,
        d: i64,
        datasize: i64,
        elements: i128,
        esize: i64,
        idxdsize: i64,
        index: i64,
        m: i64,
        n: i64,
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
        // D s_0_3: write-var idxdsizeshadow#1769 <= s_0_2
        fn_state.idxdsizeshadow_1769 = s_0_2;
        // D s_0_4: read-var esize:i64
        let s_0_4: i64 = fn_state.esize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var esizeshadow#1770 <= s_0_6
        fn_state.esizeshadow_1770 = s_0_6;
        // D s_0_8: read-var datasize:i64
        let s_0_8: i64 = fn_state.datasize;
        // D s_0_9: cast zx s_0_8 -> i
        let s_0_9: i128 = (i128::try_from(s_0_8).unwrap());
        // D s_0_10: cast reint s_0_9 -> i64
        let s_0_10: i64 = (s_0_9 as i64);
        // D s_0_11: write-var datasizeshadow#1771 <= s_0_10
        fn_state.datasizeshadow_1771 = s_0_10;
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
        // D s_1_0: read-var datasizeshadow#1771:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1771;
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
        // D s_1_7: read-var idxdsizeshadow#1769:i64
        let s_1_7: i64 = fn_state.idxdsizeshadow_1769;
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
        // D s_1_13: read-var esizeshadow#1770:i64
        let s_1_13: i64 = fn_state.esizeshadow_1770;
        // D s_1_14: cast zx s_1_13 -> i
        let s_1_14: i128 = (i128::try_from(s_1_13).unwrap());
        // D s_1_15: cast reint s_1_14 -> i64
        let s_1_15: i64 = (s_1_14 as i64);
        // D s_1_16: read-var index:i64
        let s_1_16: i64 = fn_state.index;
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_18: cast zx s_1_15 -> i
        let s_1_18: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_19: call Elem_read(s_1_12, s_1_17, s_1_18)
        let s_1_19: Bits = Elem_read(state, tracer, s_1_12, s_1_17, s_1_18);
        // D s_1_20: cast zx s_1_19 -> i
        let s_1_20: i128 = (s_1_19.value() as i128);
        // D s_1_21: write-var element2 <= s_1_20
        fn_state.element2 = s_1_20;
        // C s_1_22: const #0s : i64
        let s_1_22: i64 = 0;
        // C s_1_23: const #1s : i
        let s_1_23: i128 = 1;
        // D s_1_24: read-var elements:i
        let s_1_24: i128 = fn_state.elements;
        // D s_1_25: sub s_1_24 s_1_23
        let s_1_25: i128 = ((s_1_24) - (s_1_23));
        // D s_1_26: cast reint s_1_25 -> i64
        let s_1_26: i64 = (s_1_25 as i64);
        // D s_1_27: write-var gs#165481 <= s_1_26
        fn_state.gs_165481 = s_1_26;
        // D s_1_28: write-var e <= s_1_22
        fn_state.e = s_1_22;
        // N s_1_29: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#165481:i64
        let s_2_1: i64 = fn_state.gs_165481;
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
        // D s_3_0: read-var esizeshadow#1770:i64
        let s_3_0: i64 = fn_state.esizeshadow_1770;
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
        // D s_3_12: read-var esizeshadow#1770:i64
        let s_3_12: i64 = fn_state.esizeshadow_1770;
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
        // D s_3_19: read-var esizeshadow#1770:i64
        let s_3_19: i64 = fn_state.esizeshadow_1770;
        // D s_3_20: cast zx s_3_19 -> i
        let s_3_20: i128 = (i128::try_from(s_3_19).unwrap());
        // D s_3_21: cast reint s_3_20 -> i64
        let s_3_21: i64 = (s_3_20 as i64);
        // D s_3_22: read-var e:i64
        let s_3_22: i64 = fn_state.e;
        // D s_3_23: cast zx s_3_22 -> i
        let s_3_23: i128 = (i128::try_from(s_3_22).unwrap());
        // D s_3_24: cast zx s_3_21 -> i
        let s_3_24: i128 = (i128::try_from(s_3_21).unwrap());
        // D s_3_25: read-var result:bv
        let s_3_25: Bits = fn_state.result;
        // D s_3_26: call Elem_set(s_3_25, s_3_23, s_3_24, s_3_18)
        let s_3_26: Bits = Elem_set(state, tracer, s_3_25, s_3_23, s_3_24, s_3_18);
        // D s_3_27: write-var result <= s_3_26
        fn_state.result = s_3_26;
        // D s_3_28: read-var e:i64
        let s_3_28: i64 = fn_state.e;
        // C s_3_29: const #1s : i64
        let s_3_29: i64 = 1;
        // D s_3_30: add s_3_28 s_3_29
        let s_3_30: i64 = (s_3_28 + s_3_29);
        // D s_3_31: write-var e <= s_3_30
        fn_state.e = s_3_30;
        // N s_3_32: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var datasizeshadow#1771:i64
        let s_4_0: i64 = fn_state.datasizeshadow_1771;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // D s_4_3: read-var d:i64
        let s_4_3: i64 = fn_state.d;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: read-var result:bv
        let s_4_5: Bits = fn_state.result;
        // D s_4_6: call V_set(s_4_4, s_4_2, s_4_5)
        let s_4_6: () = V_set(state, tracer, s_4_4, s_4_2, s_4_5);
        // N s_4_7: return
        return;
    }
}