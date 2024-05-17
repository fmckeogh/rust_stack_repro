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
use asl_Int::*;
use V_set::*;
use V_read::*;
use Elem_read::*;
use integer_subrange::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_reduce_int_max<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    elements: i128,
    esize: i64,
    min: bool,
    n: i64,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        e: i64,
        esizeshadow_1862: i64,
        element: i128,
        datasizeshadow_1863: i64,
        maxmin: i128,
        gs_169802: i64,
        d: i64,
        datasize: i64,
        elements: i128,
        esize: i64,
        min: bool,
        n: i64,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        elements,
        esize,
        min,
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
        // D s_0_3: write-var esizeshadow#1862 <= s_0_2
        fn_state.esizeshadow_1862 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var datasizeshadow#1863 <= s_0_6
        fn_state.datasizeshadow_1863 = s_0_6;
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
        // D s_1_0: read-var datasizeshadow#1863:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1863;
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
        // D s_1_7: read-var esizeshadow#1862:i64
        let s_1_7: i64 = fn_state.esizeshadow_1862;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast reint s_1_8 -> i64
        let s_1_9: i64 = (s_1_8 as i64);
        // C s_1_10: const #0s : i
        let s_1_10: i128 = 0;
        // D s_1_11: cast zx s_1_9 -> i
        let s_1_11: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_12: read-var operand:bv
        let s_1_12: Bits = fn_state.operand;
        // D s_1_13: call Elem_read(s_1_12, s_1_10, s_1_11)
        let s_1_13: Bits = Elem_read(state, tracer, s_1_12, s_1_10, s_1_11);
        // D s_1_14: read-var is_unsigned:u8
        let s_1_14: bool = fn_state.is_unsigned;
        // D s_1_15: call asl_Int(s_1_13, s_1_14)
        let s_1_15: i128 = asl_Int(state, tracer, s_1_13, s_1_14);
        // D s_1_16: write-var maxmin <= s_1_15
        fn_state.maxmin = s_1_15;
        // C s_1_17: const #1s : i64
        let s_1_17: i64 = 1;
        // C s_1_18: const #1s : i
        let s_1_18: i128 = 1;
        // D s_1_19: read-var elements:i
        let s_1_19: i128 = fn_state.elements;
        // D s_1_20: sub s_1_19 s_1_18
        let s_1_20: i128 = ((s_1_19) - (s_1_18));
        // D s_1_21: cast reint s_1_20 -> i64
        let s_1_21: i64 = (s_1_20 as i64);
        // D s_1_22: write-var gs#169802 <= s_1_21
        fn_state.gs_169802 = s_1_21;
        // D s_1_23: write-var e <= s_1_17
        fn_state.e = s_1_17;
        // N s_1_24: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#169802:i64
        let s_2_1: i64 = fn_state.gs_169802;
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
        // D s_3_0: read-var esizeshadow#1862:i64
        let s_3_0: i64 = fn_state.esizeshadow_1862;
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
        // D s_3_6: read-var operand:bv
        let s_3_6: Bits = fn_state.operand;
        // D s_3_7: call Elem_read(s_3_6, s_3_4, s_3_5)
        let s_3_7: Bits = Elem_read(state, tracer, s_3_6, s_3_4, s_3_5);
        // D s_3_8: read-var is_unsigned:u8
        let s_3_8: bool = fn_state.is_unsigned;
        // D s_3_9: call asl_Int(s_3_7, s_3_8)
        let s_3_9: i128 = asl_Int(state, tracer, s_3_7, s_3_8);
        // D s_3_10: write-var element <= s_3_9
        fn_state.element = s_3_9;
        // D s_3_11: read-var min:u8
        let s_3_11: bool = fn_state.min;
        // N s_3_12: branch s_3_11 b6 b4
        if s_3_11 {
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
        // D s_4_0: read-var maxmin:i
        let s_4_0: i128 = fn_state.maxmin;
        // D s_4_1: read-var element:i
        let s_4_1: i128 = fn_state.element;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // D s_4_3: select s_4_2 s_4_0 s_4_1
        let s_4_3: i128 = if s_4_2 { s_4_0 } else { s_4_1 };
        // D s_4_4: write-var maxmin <= s_4_3
        fn_state.maxmin = s_4_3;
        // N s_4_5: jump b5
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
        // D s_6_0: read-var maxmin:i
        let s_6_0: i128 = fn_state.maxmin;
        // D s_6_1: read-var element:i
        let s_6_1: i128 = fn_state.element;
        // D s_6_2: cmp-lt s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) < (s_6_1));
        // D s_6_3: select s_6_2 s_6_0 s_6_1
        let s_6_3: i128 = if s_6_2 { s_6_0 } else { s_6_1 };
        // D s_6_4: write-var maxmin <= s_6_3
        fn_state.maxmin = s_6_3;
        // N s_6_5: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var maxmin:i
        let s_7_0: i128 = fn_state.maxmin;
        // D s_7_1: read-var esizeshadow#1862:i64
        let s_7_1: i64 = fn_state.esizeshadow_1862;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: cast reint s_7_2 -> i64
        let s_7_3: i64 = (s_7_2 as i64);
        // C s_7_4: const #1s : i
        let s_7_4: i128 = 1;
        // D s_7_5: read-var esizeshadow#1862:i64
        let s_7_5: i64 = fn_state.esizeshadow_1862;
        // D s_7_6: cast zx s_7_5 -> i
        let s_7_6: i128 = (i128::try_from(s_7_5).unwrap());
        // D s_7_7: sub s_7_6 s_7_4
        let s_7_7: i128 = ((s_7_6) - (s_7_4));
        // D s_7_8: cast reint s_7_7 -> i64
        let s_7_8: i64 = (s_7_7 as i64);
        // C s_7_9: const #0s : i
        let s_7_9: i128 = 0;
        // D s_7_10: cast zx s_7_8 -> i
        let s_7_10: i128 = (i128::try_from(s_7_8).unwrap());
        // D s_7_11: call integer_subrange(s_7_0, s_7_10, s_7_9)
        let s_7_11: Bits = integer_subrange(state, tracer, s_7_0, s_7_10, s_7_9);
        // D s_7_12: read-var d:i64
        let s_7_12: i64 = fn_state.d;
        // D s_7_13: cast zx s_7_12 -> i
        let s_7_13: i128 = (i128::try_from(s_7_12).unwrap());
        // D s_7_14: call V_set(s_7_13, s_7_3, s_7_11)
        let s_7_14: () = V_set(state, tracer, s_7_13, s_7_3, s_7_11);
        // N s_7_15: return
        return;
    }
}
