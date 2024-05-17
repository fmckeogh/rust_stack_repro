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
use CountLeadingSignBits::*;
use Elem_set::*;
use V_read::*;
use CountLeadingZeroBits::*;
use V_set::*;
use integer_subrange::*;
use Elem_read::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_arithmetic_unary_clsz<T: Tracer>(
    state: &mut State,
    tracer: &T,
    countop: u32,
    d: i64,
    datasize: i64,
    elements: i128,
    esize: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        datasizeshadow_1117: i64,
        operand: Bits,
        esizeshadow_1116: i64,
        e: i64,
        gs_145928: i64,
        count: i128,
        result: Bits,
        countop: u32,
        d: i64,
        datasize: i64,
        elements: i128,
        esize: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        countop,
        d,
        datasize,
        elements,
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
        // D s_0_3: write-var esizeshadow#1116 <= s_0_2
        fn_state.esizeshadow_1116 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var datasizeshadow#1117 <= s_0_6
        fn_state.datasizeshadow_1117 = s_0_6;
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
        // D s_1_0: read-var datasizeshadow#1117:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1117;
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
        // C s_1_7: const #0s : i64
        let s_1_7: i64 = 0;
        // C s_1_8: const #1s : i
        let s_1_8: i128 = 1;
        // D s_1_9: read-var elements:i
        let s_1_9: i128 = fn_state.elements;
        // D s_1_10: sub s_1_9 s_1_8
        let s_1_10: i128 = ((s_1_9) - (s_1_8));
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: write-var gs#145928 <= s_1_11
        fn_state.gs_145928 = s_1_11;
        // D s_1_13: write-var e <= s_1_7
        fn_state.e = s_1_7;
        // N s_1_14: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#145928:i64
        let s_2_1: i64 = fn_state.gs_145928;
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
        // D s_3_0: read-var countop:u32
        let s_3_0: u32 = fn_state.countop;
        // C s_3_1: const #1u : u32
        let s_3_1: u32 = 1;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // N s_3_3: branch s_3_2 b6 b4
        if s_3_2 {
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
        // D s_4_0: read-var esizeshadow#1116:i64
        let s_4_0: i64 = fn_state.esizeshadow_1116;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // D s_4_3: read-var e:i64
        let s_4_3: i64 = fn_state.e;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: cast zx s_4_2 -> i
        let s_4_5: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_6: read-var operand:bv
        let s_4_6: Bits = fn_state.operand;
        // D s_4_7: call Elem_read(s_4_6, s_4_4, s_4_5)
        let s_4_7: Bits = Elem_read(state, tracer, s_4_6, s_4_4, s_4_5);
        // D s_4_8: call CountLeadingZeroBits(s_4_7)
        let s_4_8: i128 = CountLeadingZeroBits(state, tracer, s_4_7);
        // D s_4_9: write-var count <= s_4_8
        fn_state.count = s_4_8;
        // N s_4_10: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var esizeshadow#1116:i64
        let s_5_0: i64 = fn_state.esizeshadow_1116;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: cast reint s_5_1 -> i64
        let s_5_2: i64 = (s_5_1 as i64);
        // C s_5_3: const #1s : i
        let s_5_3: i128 = 1;
        // D s_5_4: read-var esizeshadow#1116:i64
        let s_5_4: i64 = fn_state.esizeshadow_1116;
        // D s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_6: sub s_5_5 s_5_3
        let s_5_6: i128 = ((s_5_5) - (s_5_3));
        // D s_5_7: cast reint s_5_6 -> i64
        let s_5_7: i64 = (s_5_6 as i64);
        // C s_5_8: const #0s : i
        let s_5_8: i128 = 0;
        // D s_5_9: cast zx s_5_7 -> i
        let s_5_9: i128 = (i128::try_from(s_5_7).unwrap());
        // D s_5_10: read-var count:i
        let s_5_10: i128 = fn_state.count;
        // D s_5_11: call integer_subrange(s_5_10, s_5_9, s_5_8)
        let s_5_11: Bits = integer_subrange(state, tracer, s_5_10, s_5_9, s_5_8);
        // D s_5_12: read-var e:i64
        let s_5_12: i64 = fn_state.e;
        // D s_5_13: cast zx s_5_12 -> i
        let s_5_13: i128 = (i128::try_from(s_5_12).unwrap());
        // D s_5_14: cast zx s_5_2 -> i
        let s_5_14: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_15: read-var result:bv
        let s_5_15: Bits = fn_state.result;
        // D s_5_16: call Elem_set(s_5_15, s_5_13, s_5_14, s_5_11)
        let s_5_16: Bits = Elem_set(state, tracer, s_5_15, s_5_13, s_5_14, s_5_11);
        // D s_5_17: write-var result <= s_5_16
        fn_state.result = s_5_16;
        // D s_5_18: read-var e:i64
        let s_5_18: i64 = fn_state.e;
        // C s_5_19: const #1s : i64
        let s_5_19: i64 = 1;
        // D s_5_20: add s_5_18 s_5_19
        let s_5_20: i64 = (s_5_18 + s_5_19);
        // D s_5_21: write-var e <= s_5_20
        fn_state.e = s_5_20;
        // N s_5_22: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var esizeshadow#1116:i64
        let s_6_0: i64 = fn_state.esizeshadow_1116;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: cast reint s_6_1 -> i64
        let s_6_2: i64 = (s_6_1 as i64);
        // D s_6_3: read-var e:i64
        let s_6_3: i64 = fn_state.e;
        // D s_6_4: cast zx s_6_3 -> i
        let s_6_4: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_5: cast zx s_6_2 -> i
        let s_6_5: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_6: read-var operand:bv
        let s_6_6: Bits = fn_state.operand;
        // D s_6_7: call Elem_read(s_6_6, s_6_4, s_6_5)
        let s_6_7: Bits = Elem_read(state, tracer, s_6_6, s_6_4, s_6_5);
        // D s_6_8: call CountLeadingSignBits(s_6_7)
        let s_6_8: i128 = CountLeadingSignBits(state, tracer, s_6_7);
        // D s_6_9: write-var count <= s_6_8
        fn_state.count = s_6_8;
        // N s_6_10: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var datasizeshadow#1117:i64
        let s_7_0: i64 = fn_state.datasizeshadow_1117;
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
