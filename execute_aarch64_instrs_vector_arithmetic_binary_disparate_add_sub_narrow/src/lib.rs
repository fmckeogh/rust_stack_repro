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
use Vpart_set::*;
use Elem_set::*;
use V_read::*;
use Elem_read::*;
use RShr::*;
use integer_subrange::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_arithmetic_binary_disparate_add_sub_narrow<
    T: Tracer,
>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    elements: i128,
    esize: i64,
    m: i64,
    n: i64,
    part: i64,
    round: bool,
    sub_op: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        element2: i128,
        esizeshadow_1031: i64,
        e: i64,
        element1: i128,
        sum: i128,
        gs_143148: i64,
        result: Bits,
        operand1: Bits,
        operand2: Bits,
        d: i64,
        datasize: i64,
        elements: i128,
        esize: i64,
        m: i64,
        n: i64,
        part: i64,
        round: bool,
        sub_op: bool,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        elements,
        esize,
        m,
        n,
        part,
        round,
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
        // D s_0_3: write-var esizeshadow#1031 <= s_0_2
        fn_state.esizeshadow_1031 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckFPAdvSIMDEnabled64(s_0_4)
        let s_0_5: () = CheckFPAdvSIMDEnabled64(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #2s : i
        let s_1_0: i128 = 2;
        // D s_1_1: read-var datasize:i64
        let s_1_1: i64 = fn_state.datasize;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: mul s_1_0 s_1_2
        let s_1_3: i128 = ((s_1_0) * (s_1_2));
        // D s_1_4: cast reint s_1_3 -> i64
        let s_1_4: i64 = (s_1_3 as i64);
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var n:i64
        let s_1_7: i64 = fn_state.n;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: call V_read(s_1_8, s_1_6)
        let s_1_9: Bits = V_read(state, tracer, s_1_8, s_1_6);
        // D s_1_10: write-var operand1 <= s_1_9
        fn_state.operand1 = s_1_9;
        // C s_1_11: const #2s : i
        let s_1_11: i128 = 2;
        // D s_1_12: read-var datasize:i64
        let s_1_12: i64 = fn_state.datasize;
        // D s_1_13: cast zx s_1_12 -> i
        let s_1_13: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_14: mul s_1_11 s_1_13
        let s_1_14: i128 = ((s_1_11) * (s_1_13));
        // D s_1_15: cast reint s_1_14 -> i64
        let s_1_15: i64 = (s_1_14 as i64);
        // D s_1_16: cast zx s_1_15 -> i
        let s_1_16: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_17: cast reint s_1_16 -> i64
        let s_1_17: i64 = (s_1_16 as i64);
        // D s_1_18: read-var m:i64
        let s_1_18: i64 = fn_state.m;
        // D s_1_19: cast zx s_1_18 -> i
        let s_1_19: i128 = (i128::try_from(s_1_18).unwrap());
        // D s_1_20: call V_read(s_1_19, s_1_17)
        let s_1_20: Bits = V_read(state, tracer, s_1_19, s_1_17);
        // D s_1_21: write-var operand2 <= s_1_20
        fn_state.operand2 = s_1_20;
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
        // D s_1_27: write-var gs#143148 <= s_1_26
        fn_state.gs_143148 = s_1_26;
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
        // D s_2_1: read-var gs#143148:i64
        let s_2_1: i64 = fn_state.gs_143148;
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
        // C s_3_0: const #2s : i
        let s_3_0: i128 = 2;
        // D s_3_1: read-var esizeshadow#1031:i64
        let s_3_1: i64 = fn_state.esizeshadow_1031;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: mul s_3_0 s_3_2
        let s_3_3: i128 = ((s_3_0) * (s_3_2));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // D s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_6: cast reint s_3_5 -> i64
        let s_3_6: i64 = (s_3_5 as i64);
        // D s_3_7: read-var e:i64
        let s_3_7: i64 = fn_state.e;
        // D s_3_8: cast zx s_3_7 -> i
        let s_3_8: i128 = (i128::try_from(s_3_7).unwrap());
        // D s_3_9: cast zx s_3_6 -> i
        let s_3_9: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_10: read-var operand1:bv
        let s_3_10: Bits = fn_state.operand1;
        // D s_3_11: call Elem_read(s_3_10, s_3_8, s_3_9)
        let s_3_11: Bits = Elem_read(state, tracer, s_3_10, s_3_8, s_3_9);
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (s_3_11.value() as i128);
        // D s_3_13: write-var element1 <= s_3_12
        fn_state.element1 = s_3_12;
        // C s_3_14: const #2s : i
        let s_3_14: i128 = 2;
        // D s_3_15: read-var esizeshadow#1031:i64
        let s_3_15: i64 = fn_state.esizeshadow_1031;
        // D s_3_16: cast zx s_3_15 -> i
        let s_3_16: i128 = (i128::try_from(s_3_15).unwrap());
        // D s_3_17: mul s_3_14 s_3_16
        let s_3_17: i128 = ((s_3_14) * (s_3_16));
        // D s_3_18: cast reint s_3_17 -> i64
        let s_3_18: i64 = (s_3_17 as i64);
        // D s_3_19: cast zx s_3_18 -> i
        let s_3_19: i128 = (i128::try_from(s_3_18).unwrap());
        // D s_3_20: cast reint s_3_19 -> i64
        let s_3_20: i64 = (s_3_19 as i64);
        // D s_3_21: read-var e:i64
        let s_3_21: i64 = fn_state.e;
        // D s_3_22: cast zx s_3_21 -> i
        let s_3_22: i128 = (i128::try_from(s_3_21).unwrap());
        // D s_3_23: cast zx s_3_20 -> i
        let s_3_23: i128 = (i128::try_from(s_3_20).unwrap());
        // D s_3_24: read-var operand2:bv
        let s_3_24: Bits = fn_state.operand2;
        // D s_3_25: call Elem_read(s_3_24, s_3_22, s_3_23)
        let s_3_25: Bits = Elem_read(state, tracer, s_3_24, s_3_22, s_3_23);
        // D s_3_26: cast zx s_3_25 -> i
        let s_3_26: i128 = (s_3_25.value() as i128);
        // D s_3_27: write-var element2 <= s_3_26
        fn_state.element2 = s_3_26;
        // D s_3_28: read-var sub_op:u8
        let s_3_28: bool = fn_state.sub_op;
        // N s_3_29: branch s_3_28 b6 b4
        if s_3_28 {
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
        // D s_4_0: read-var element1:i
        let s_4_0: i128 = fn_state.element1;
        // D s_4_1: read-var element2:i
        let s_4_1: i128 = fn_state.element2;
        // D s_4_2: add s_4_0 s_4_1
        let s_4_2: i128 = (s_4_0 + s_4_1);
        // D s_4_3: write-var sum <= s_4_2
        fn_state.sum = s_4_2;
        // N s_4_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var esizeshadow#1031:i64
        let s_5_0: i64 = fn_state.esizeshadow_1031;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: read-var sum:i
        let s_5_2: i128 = fn_state.sum;
        // D s_5_3: read-var round:u8
        let s_5_3: bool = fn_state.round;
        // D s_5_4: call RShr(s_5_2, s_5_1, s_5_3)
        let s_5_4: i128 = RShr(state, tracer, s_5_2, s_5_1, s_5_3);
        // D s_5_5: write-var sum <= s_5_4
        fn_state.sum = s_5_4;
        // D s_5_6: read-var esizeshadow#1031:i64
        let s_5_6: i64 = fn_state.esizeshadow_1031;
        // D s_5_7: cast zx s_5_6 -> i
        let s_5_7: i128 = (i128::try_from(s_5_6).unwrap());
        // D s_5_8: cast reint s_5_7 -> i64
        let s_5_8: i64 = (s_5_7 as i64);
        // C s_5_9: const #1s : i
        let s_5_9: i128 = 1;
        // D s_5_10: read-var esizeshadow#1031:i64
        let s_5_10: i64 = fn_state.esizeshadow_1031;
        // D s_5_11: cast zx s_5_10 -> i
        let s_5_11: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_12: sub s_5_11 s_5_9
        let s_5_12: i128 = ((s_5_11) - (s_5_9));
        // D s_5_13: cast reint s_5_12 -> i64
        let s_5_13: i64 = (s_5_12 as i64);
        // C s_5_14: const #0s : i
        let s_5_14: i128 = 0;
        // D s_5_15: cast zx s_5_13 -> i
        let s_5_15: i128 = (i128::try_from(s_5_13).unwrap());
        // D s_5_16: read-var sum:i
        let s_5_16: i128 = fn_state.sum;
        // D s_5_17: call integer_subrange(s_5_16, s_5_15, s_5_14)
        let s_5_17: Bits = integer_subrange(state, tracer, s_5_16, s_5_15, s_5_14);
        // D s_5_18: read-var e:i64
        let s_5_18: i64 = fn_state.e;
        // D s_5_19: cast zx s_5_18 -> i
        let s_5_19: i128 = (i128::try_from(s_5_18).unwrap());
        // D s_5_20: cast zx s_5_8 -> i
        let s_5_20: i128 = (i128::try_from(s_5_8).unwrap());
        // D s_5_21: read-var result:bv
        let s_5_21: Bits = fn_state.result;
        // D s_5_22: call Elem_set(s_5_21, s_5_19, s_5_20, s_5_17)
        let s_5_22: Bits = Elem_set(state, tracer, s_5_21, s_5_19, s_5_20, s_5_17);
        // D s_5_23: write-var result <= s_5_22
        fn_state.result = s_5_22;
        // D s_5_24: read-var e:i64
        let s_5_24: i64 = fn_state.e;
        // C s_5_25: const #1s : i64
        let s_5_25: i64 = 1;
        // D s_5_26: add s_5_24 s_5_25
        let s_5_26: i64 = (s_5_24 + s_5_25);
        // D s_5_27: write-var e <= s_5_26
        fn_state.e = s_5_26;
        // N s_5_28: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var element1:i
        let s_6_0: i128 = fn_state.element1;
        // D s_6_1: read-var element2:i
        let s_6_1: i128 = fn_state.element2;
        // D s_6_2: sub s_6_0 s_6_1
        let s_6_2: i128 = ((s_6_0) - (s_6_1));
        // D s_6_3: write-var sum <= s_6_2
        fn_state.sum = s_6_2;
        // N s_6_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var datasize:i64
        let s_7_0: i64 = fn_state.datasize;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: cast reint s_7_1 -> i64
        let s_7_2: i64 = (s_7_1 as i64);
        // D s_7_3: read-var d:i64
        let s_7_3: i64 = fn_state.d;
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_5: read-var part:i64
        let s_7_5: i64 = fn_state.part;
        // D s_7_6: cast zx s_7_5 -> i
        let s_7_6: i128 = (i128::try_from(s_7_5).unwrap());
        // D s_7_7: cast zx s_7_2 -> i
        let s_7_7: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_8: read-var result:bv
        let s_7_8: Bits = fn_state.result;
        // D s_7_9: call Vpart_set(s_7_4, s_7_6, s_7_7, s_7_8)
        let s_7_9: () = Vpart_set(state, tracer, s_7_4, s_7_6, s_7_7, s_7_8);
        // N s_7_10: return
        return;
    }
}