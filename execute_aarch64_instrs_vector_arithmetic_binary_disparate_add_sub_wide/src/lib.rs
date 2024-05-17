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
use Vpart_read::*;
use Elem_set::*;
use V_read::*;
use V_set::*;
use integer_subrange::*;
use asl_Int::*;
use Elem_read::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_arithmetic_binary_disparate_add_sub_wide<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    elements: i128,
    esize: i64,
    m: i64,
    n: i64,
    part: i64,
    sub_op: bool,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        element2: i128,
        e: i64,
        element1: i128,
        sum: i128,
        esizeshadow_1810: i64,
        gs_167284: i64,
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
        sub_op: bool,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        elements,
        esize,
        m,
        n,
        part,
        sub_op,
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
        // D s_0_3: write-var esizeshadow#1810 <= s_0_2
        fn_state.esizeshadow_1810 = s_0_2;
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
        // D s_1_11: read-var datasize:i64
        let s_1_11: i64 = fn_state.datasize;
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_13: cast reint s_1_12 -> i64
        let s_1_13: i64 = (s_1_12 as i64);
        // D s_1_14: read-var m:i64
        let s_1_14: i64 = fn_state.m;
        // D s_1_15: cast zx s_1_14 -> i
        let s_1_15: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_16: read-var part:i64
        let s_1_16: i64 = fn_state.part;
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_18: cast zx s_1_13 -> i
        let s_1_18: i128 = (i128::try_from(s_1_13).unwrap());
        // D s_1_19: call Vpart_read(s_1_15, s_1_17, s_1_18)
        let s_1_19: Bits = Vpart_read(state, tracer, s_1_15, s_1_17, s_1_18);
        // D s_1_20: write-var operand2 <= s_1_19
        fn_state.operand2 = s_1_19;
        // C s_1_21: const #0s : i64
        let s_1_21: i64 = 0;
        // C s_1_22: const #1s : i
        let s_1_22: i128 = 1;
        // D s_1_23: read-var elements:i
        let s_1_23: i128 = fn_state.elements;
        // D s_1_24: sub s_1_23 s_1_22
        let s_1_24: i128 = ((s_1_23) - (s_1_22));
        // D s_1_25: cast reint s_1_24 -> i64
        let s_1_25: i64 = (s_1_24 as i64);
        // D s_1_26: write-var gs#167284 <= s_1_25
        fn_state.gs_167284 = s_1_25;
        // D s_1_27: write-var e <= s_1_21
        fn_state.e = s_1_21;
        // N s_1_28: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#167284:i64
        let s_2_1: i64 = fn_state.gs_167284;
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
        // D s_3_1: read-var esizeshadow#1810:i64
        let s_3_1: i64 = fn_state.esizeshadow_1810;
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
        // D s_3_12: read-var is_unsigned:u8
        let s_3_12: bool = fn_state.is_unsigned;
        // D s_3_13: call asl_Int(s_3_11, s_3_12)
        let s_3_13: i128 = asl_Int(state, tracer, s_3_11, s_3_12);
        // D s_3_14: write-var element1 <= s_3_13
        fn_state.element1 = s_3_13;
        // D s_3_15: read-var esizeshadow#1810:i64
        let s_3_15: i64 = fn_state.esizeshadow_1810;
        // D s_3_16: cast zx s_3_15 -> i
        let s_3_16: i128 = (i128::try_from(s_3_15).unwrap());
        // D s_3_17: cast reint s_3_16 -> i64
        let s_3_17: i64 = (s_3_16 as i64);
        // D s_3_18: read-var e:i64
        let s_3_18: i64 = fn_state.e;
        // D s_3_19: cast zx s_3_18 -> i
        let s_3_19: i128 = (i128::try_from(s_3_18).unwrap());
        // D s_3_20: cast zx s_3_17 -> i
        let s_3_20: i128 = (i128::try_from(s_3_17).unwrap());
        // D s_3_21: read-var operand2:bv
        let s_3_21: Bits = fn_state.operand2;
        // D s_3_22: call Elem_read(s_3_21, s_3_19, s_3_20)
        let s_3_22: Bits = Elem_read(state, tracer, s_3_21, s_3_19, s_3_20);
        // D s_3_23: read-var is_unsigned:u8
        let s_3_23: bool = fn_state.is_unsigned;
        // D s_3_24: call asl_Int(s_3_22, s_3_23)
        let s_3_24: i128 = asl_Int(state, tracer, s_3_22, s_3_23);
        // D s_3_25: write-var element2 <= s_3_24
        fn_state.element2 = s_3_24;
        // D s_3_26: read-var sub_op:u8
        let s_3_26: bool = fn_state.sub_op;
        // N s_3_27: branch s_3_26 b6 b4
        if s_3_26 {
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
        // C s_5_0: const #2s : i
        let s_5_0: i128 = 2;
        // D s_5_1: read-var esizeshadow#1810:i64
        let s_5_1: i64 = fn_state.esizeshadow_1810;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: mul s_5_0 s_5_2
        let s_5_3: i128 = ((s_5_0) * (s_5_2));
        // D s_5_4: cast reint s_5_3 -> i64
        let s_5_4: i64 = (s_5_3 as i64);
        // D s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_6: cast reint s_5_5 -> i64
        let s_5_6: i64 = (s_5_5 as i64);
        // C s_5_7: const #2s : i
        let s_5_7: i128 = 2;
        // D s_5_8: read-var esizeshadow#1810:i64
        let s_5_8: i64 = fn_state.esizeshadow_1810;
        // D s_5_9: cast zx s_5_8 -> i
        let s_5_9: i128 = (i128::try_from(s_5_8).unwrap());
        // D s_5_10: mul s_5_7 s_5_9
        let s_5_10: i128 = ((s_5_7) * (s_5_9));
        // D s_5_11: cast reint s_5_10 -> i64
        let s_5_11: i64 = (s_5_10 as i64);
        // C s_5_12: const #1s : i
        let s_5_12: i128 = 1;
        // D s_5_13: cast zx s_5_11 -> i
        let s_5_13: i128 = (i128::try_from(s_5_11).unwrap());
        // D s_5_14: sub s_5_13 s_5_12
        let s_5_14: i128 = ((s_5_13) - (s_5_12));
        // D s_5_15: cast reint s_5_14 -> i64
        let s_5_15: i64 = (s_5_14 as i64);
        // C s_5_16: const #0s : i
        let s_5_16: i128 = 0;
        // D s_5_17: cast zx s_5_15 -> i
        let s_5_17: i128 = (i128::try_from(s_5_15).unwrap());
        // D s_5_18: read-var sum:i
        let s_5_18: i128 = fn_state.sum;
        // D s_5_19: call integer_subrange(s_5_18, s_5_17, s_5_16)
        let s_5_19: Bits = integer_subrange(state, tracer, s_5_18, s_5_17, s_5_16);
        // D s_5_20: read-var e:i64
        let s_5_20: i64 = fn_state.e;
        // D s_5_21: cast zx s_5_20 -> i
        let s_5_21: i128 = (i128::try_from(s_5_20).unwrap());
        // D s_5_22: cast zx s_5_6 -> i
        let s_5_22: i128 = (i128::try_from(s_5_6).unwrap());
        // D s_5_23: read-var result:bv
        let s_5_23: Bits = fn_state.result;
        // D s_5_24: call Elem_set(s_5_23, s_5_21, s_5_22, s_5_19)
        let s_5_24: Bits = Elem_set(state, tracer, s_5_23, s_5_21, s_5_22, s_5_19);
        // D s_5_25: write-var result <= s_5_24
        fn_state.result = s_5_24;
        // D s_5_26: read-var e:i64
        let s_5_26: i64 = fn_state.e;
        // C s_5_27: const #1s : i64
        let s_5_27: i64 = 1;
        // D s_5_28: add s_5_26 s_5_27
        let s_5_28: i64 = (s_5_26 + s_5_27);
        // D s_5_29: write-var e <= s_5_28
        fn_state.e = s_5_28;
        // N s_5_30: jump b2
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
        // C s_7_0: const #2s : i
        let s_7_0: i128 = 2;
        // D s_7_1: read-var datasize:i64
        let s_7_1: i64 = fn_state.datasize;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: mul s_7_0 s_7_2
        let s_7_3: i128 = ((s_7_0) * (s_7_2));
        // D s_7_4: cast reint s_7_3 -> i64
        let s_7_4: i64 = (s_7_3 as i64);
        // D s_7_5: cast zx s_7_4 -> i
        let s_7_5: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_6: cast reint s_7_5 -> i64
        let s_7_6: i64 = (s_7_5 as i64);
        // D s_7_7: read-var d:i64
        let s_7_7: i64 = fn_state.d;
        // D s_7_8: cast zx s_7_7 -> i
        let s_7_8: i128 = (i128::try_from(s_7_7).unwrap());
        // D s_7_9: read-var result:bv
        let s_7_9: Bits = fn_state.result;
        // D s_7_10: call V_set(s_7_8, s_7_6, s_7_9)
        let s_7_10: () = V_set(state, tracer, s_7_8, s_7_6, s_7_9);
        // N s_7_11: return
        return;
    }
}
