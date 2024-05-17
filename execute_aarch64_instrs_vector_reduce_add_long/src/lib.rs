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
pub fn execute_aarch64_instrs_vector_reduce_add_long<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    elements: i128,
    esize: i64,
    n: i64,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        e: i64,
        sum: i128,
        esizeshadow_1807: i64,
        datasizeshadow_1808: i64,
        gs_167238: i64,
        d: i64,
        datasize: i64,
        elements: i128,
        esize: i64,
        n: i64,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        elements,
        esize,
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
        // D s_0_3: write-var esizeshadow#1807 <= s_0_2
        fn_state.esizeshadow_1807 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var datasizeshadow#1808 <= s_0_6
        fn_state.datasizeshadow_1808 = s_0_6;
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
        // D s_1_0: read-var datasizeshadow#1808:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1808;
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
        // D s_1_7: read-var esizeshadow#1807:i64
        let s_1_7: i64 = fn_state.esizeshadow_1807;
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
        // D s_1_16: write-var sum <= s_1_15
        fn_state.sum = s_1_15;
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
        // D s_1_22: write-var gs#167238 <= s_1_21
        fn_state.gs_167238 = s_1_21;
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
        // D s_2_1: read-var gs#167238:i64
        let s_2_1: i64 = fn_state.gs_167238;
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
        // D s_3_0: read-var esizeshadow#1807:i64
        let s_3_0: i64 = fn_state.esizeshadow_1807;
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
        // D s_3_10: read-var sum:i
        let s_3_10: i128 = fn_state.sum;
        // D s_3_11: add s_3_10 s_3_9
        let s_3_11: i128 = (s_3_10 + s_3_9);
        // D s_3_12: write-var sum <= s_3_11
        fn_state.sum = s_3_11;
        // D s_3_13: read-var e:i64
        let s_3_13: i64 = fn_state.e;
        // C s_3_14: const #1s : i64
        let s_3_14: i64 = 1;
        // D s_3_15: add s_3_13 s_3_14
        let s_3_15: i64 = (s_3_13 + s_3_14);
        // D s_3_16: write-var e <= s_3_15
        fn_state.e = s_3_15;
        // N s_3_17: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var sum:i
        let s_4_0: i128 = fn_state.sum;
        // C s_4_1: const #2s : i
        let s_4_1: i128 = 2;
        // D s_4_2: read-var esizeshadow#1807:i64
        let s_4_2: i64 = fn_state.esizeshadow_1807;
        // D s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_4: mul s_4_1 s_4_3
        let s_4_4: i128 = ((s_4_1) * (s_4_3));
        // D s_4_5: cast reint s_4_4 -> i64
        let s_4_5: i64 = (s_4_4 as i64);
        // D s_4_6: cast zx s_4_5 -> i
        let s_4_6: i128 = (i128::try_from(s_4_5).unwrap());
        // D s_4_7: cast reint s_4_6 -> i64
        let s_4_7: i64 = (s_4_6 as i64);
        // C s_4_8: const #2s : i
        let s_4_8: i128 = 2;
        // D s_4_9: read-var esizeshadow#1807:i64
        let s_4_9: i64 = fn_state.esizeshadow_1807;
        // D s_4_10: cast zx s_4_9 -> i
        let s_4_10: i128 = (i128::try_from(s_4_9).unwrap());
        // D s_4_11: mul s_4_8 s_4_10
        let s_4_11: i128 = ((s_4_8) * (s_4_10));
        // D s_4_12: cast reint s_4_11 -> i64
        let s_4_12: i64 = (s_4_11 as i64);
        // C s_4_13: const #1s : i
        let s_4_13: i128 = 1;
        // D s_4_14: cast zx s_4_12 -> i
        let s_4_14: i128 = (i128::try_from(s_4_12).unwrap());
        // D s_4_15: sub s_4_14 s_4_13
        let s_4_15: i128 = ((s_4_14) - (s_4_13));
        // D s_4_16: cast reint s_4_15 -> i64
        let s_4_16: i64 = (s_4_15 as i64);
        // C s_4_17: const #0s : i
        let s_4_17: i128 = 0;
        // D s_4_18: cast zx s_4_16 -> i
        let s_4_18: i128 = (i128::try_from(s_4_16).unwrap());
        // D s_4_19: call integer_subrange(s_4_0, s_4_18, s_4_17)
        let s_4_19: Bits = integer_subrange(state, tracer, s_4_0, s_4_18, s_4_17);
        // D s_4_20: read-var d:i64
        let s_4_20: i64 = fn_state.d;
        // D s_4_21: cast zx s_4_20 -> i
        let s_4_21: i128 = (i128::try_from(s_4_20).unwrap());
        // D s_4_22: call V_set(s_4_21, s_4_7, s_4_19)
        let s_4_22: () = V_set(state, tracer, s_4_21, s_4_7, s_4_19);
        // N s_4_23: return
        return;
    }
}
