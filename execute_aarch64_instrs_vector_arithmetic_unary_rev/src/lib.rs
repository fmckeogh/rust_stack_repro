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
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_arithmetic_unary_rev<T: Tracer>(
    state: &mut State,
    tracer: &T,
    containers: i128,
    d: i64,
    datasize: i64,
    elements_per_container: i128,
    esize: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        rev_element: i128,
        e: i64,
        esizeshadow_1792: i64,
        gs_166660: i64,
        gs_166668: i64,
        datasizeshadow_1793: i64,
        element: i128,
        result: Bits,
        c: i64,
        containers: i128,
        d: i64,
        datasize: i64,
        elements_per_container: i128,
        esize: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        containers,
        d,
        datasize,
        elements_per_container,
        esize,
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
        // D s_0_3: write-var esizeshadow#1792 <= s_0_2
        fn_state.esizeshadow_1792 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var datasizeshadow#1793 <= s_0_6
        fn_state.datasizeshadow_1793 = s_0_6;
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call CheckFPAdvSIMDEnabled64(s_0_8)
        let s_0_9: () = CheckFPAdvSIMDEnabled64(state, tracer, s_0_8);
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var datasizeshadow#1793:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1793;
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
        // D s_1_6: write-var operand <= s_1_5
        fn_state.operand = s_1_5;
        // C s_1_7: const #0s : i
        let s_1_7: i128 = 0;
        // D s_1_8: write-var element <= s_1_7
        fn_state.element = s_1_7;
        // C s_1_9: const #0s : i64
        let s_1_9: i64 = 0;
        // C s_1_10: const #1s : i
        let s_1_10: i128 = 1;
        // D s_1_11: read-var containers:i
        let s_1_11: i128 = fn_state.containers;
        // D s_1_12: sub s_1_11 s_1_10
        let s_1_12: i128 = ((s_1_11) - (s_1_10));
        // D s_1_13: cast reint s_1_12 -> i64
        let s_1_13: i64 = (s_1_12 as i64);
        // D s_1_14: write-var gs#166660 <= s_1_13
        fn_state.gs_166660 = s_1_13;
        // D s_1_15: write-var c <= s_1_9
        fn_state.c = s_1_9;
        // N s_1_16: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var c:i64
        let s_2_0: i64 = fn_state.c;
        // D s_2_1: read-var gs#166660:i64
        let s_2_1: i64 = fn_state.gs_166660;
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
        // D s_3_0: read-var element:i
        let s_3_0: i128 = fn_state.element;
        // D s_3_1: read-var elements_per_container:i
        let s_3_1: i128 = fn_state.elements_per_container;
        // D s_3_2: add s_3_0 s_3_1
        let s_3_2: i128 = (s_3_0 + s_3_1);
        // C s_3_3: const #1s : i
        let s_3_3: i128 = 1;
        // D s_3_4: sub s_3_2 s_3_3
        let s_3_4: i128 = ((s_3_2) - (s_3_3));
        // D s_3_5: write-var rev_element <= s_3_4
        fn_state.rev_element = s_3_4;
        // C s_3_6: const #0s : i64
        let s_3_6: i64 = 0;
        // C s_3_7: const #1s : i
        let s_3_7: i128 = 1;
        // D s_3_8: read-var elements_per_container:i
        let s_3_8: i128 = fn_state.elements_per_container;
        // D s_3_9: sub s_3_8 s_3_7
        let s_3_9: i128 = ((s_3_8) - (s_3_7));
        // D s_3_10: cast reint s_3_9 -> i64
        let s_3_10: i64 = (s_3_9 as i64);
        // D s_3_11: write-var gs#166668 <= s_3_10
        fn_state.gs_166668 = s_3_10;
        // D s_3_12: write-var e <= s_3_6
        fn_state.e = s_3_6;
        // N s_3_13: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#166668:i64
        let s_4_1: i64 = fn_state.gs_166668;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b6 b5
        if s_4_2 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var rev_element:i
        let s_5_0: i128 = fn_state.rev_element;
        // D s_5_1: read-var element:i
        let s_5_1: i128 = fn_state.element;
        // D s_5_2: read-var esizeshadow#1792:i64
        let s_5_2: i64 = fn_state.esizeshadow_1792;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: cast reint s_5_3 -> i64
        let s_5_4: i64 = (s_5_3 as i64);
        // D s_5_5: read-var esizeshadow#1792:i64
        let s_5_5: i64 = fn_state.esizeshadow_1792;
        // D s_5_6: cast zx s_5_5 -> i
        let s_5_6: i128 = (i128::try_from(s_5_5).unwrap());
        // D s_5_7: cast reint s_5_6 -> i64
        let s_5_7: i64 = (s_5_6 as i64);
        // D s_5_8: cast zx s_5_7 -> i
        let s_5_8: i128 = (i128::try_from(s_5_7).unwrap());
        // D s_5_9: read-var operand:bv
        let s_5_9: Bits = fn_state.operand;
        // D s_5_10: call Elem_read(s_5_9, s_5_1, s_5_8)
        let s_5_10: Bits = Elem_read(state, tracer, s_5_9, s_5_1, s_5_8);
        // D s_5_11: cast zx s_5_4 -> i
        let s_5_11: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_12: read-var result:bv
        let s_5_12: Bits = fn_state.result;
        // D s_5_13: call Elem_set(s_5_12, s_5_0, s_5_11, s_5_10)
        let s_5_13: Bits = Elem_set(state, tracer, s_5_12, s_5_0, s_5_11, s_5_10);
        // D s_5_14: write-var result <= s_5_13
        fn_state.result = s_5_13;
        // C s_5_15: const #1s : i
        let s_5_15: i128 = 1;
        // D s_5_16: read-var element:i
        let s_5_16: i128 = fn_state.element;
        // D s_5_17: add s_5_16 s_5_15
        let s_5_17: i128 = (s_5_16 + s_5_15);
        // D s_5_18: write-var element <= s_5_17
        fn_state.element = s_5_17;
        // C s_5_19: const #1s : i
        let s_5_19: i128 = 1;
        // D s_5_20: read-var rev_element:i
        let s_5_20: i128 = fn_state.rev_element;
        // D s_5_21: sub s_5_20 s_5_19
        let s_5_21: i128 = ((s_5_20) - (s_5_19));
        // D s_5_22: write-var rev_element <= s_5_21
        fn_state.rev_element = s_5_21;
        // D s_5_23: read-var e:i64
        let s_5_23: i64 = fn_state.e;
        // C s_5_24: const #1s : i64
        let s_5_24: i64 = 1;
        // D s_5_25: add s_5_23 s_5_24
        let s_5_25: i64 = (s_5_23 + s_5_24);
        // D s_5_26: write-var e <= s_5_25
        fn_state.e = s_5_25;
        // N s_5_27: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var c:i64
        let s_6_0: i64 = fn_state.c;
        // C s_6_1: const #1s : i64
        let s_6_1: i64 = 1;
        // D s_6_2: add s_6_0 s_6_1
        let s_6_2: i64 = (s_6_0 + s_6_1);
        // D s_6_3: write-var c <= s_6_2
        fn_state.c = s_6_2;
        // N s_6_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var datasizeshadow#1793:i64
        let s_7_0: i64 = fn_state.datasizeshadow_1793;
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
