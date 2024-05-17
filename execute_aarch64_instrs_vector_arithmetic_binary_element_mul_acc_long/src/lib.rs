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
pub fn execute_aarch64_instrs_vector_arithmetic_binary_element_mul_acc_long<T: Tracer>(
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
    part: i64,
    sub_op: bool,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        product: Bits,
        e: i64,
        operand3: Bits,
        esizeshadow_1869: i64,
        idxdsizeshadow_1868: i64,
        element2: i128,
        result: Bits,
        operand1: Bits,
        gs_169992: i64,
        d: i64,
        datasize: i64,
        elements: i128,
        esize: i64,
        idxdsize: i64,
        index: i64,
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
        idxdsize,
        index,
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
        // D s_0_0: read-var idxdsize:i64
        let s_0_0: i64 = fn_state.idxdsize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var idxdsizeshadow#1868 <= s_0_2
        fn_state.idxdsizeshadow_1868 = s_0_2;
        // D s_0_4: read-var esize:i64
        let s_0_4: i64 = fn_state.esize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var esizeshadow#1869 <= s_0_6
        fn_state.esizeshadow_1869 = s_0_6;
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
        // D s_1_0: read-var datasize:i64
        let s_1_0: i64 = fn_state.datasize;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: cast reint s_1_1 -> i64
        let s_1_2: i64 = (s_1_1 as i64);
        // D s_1_3: read-var n:i64
        let s_1_3: i64 = fn_state.n;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: read-var part:i64
        let s_1_5: i64 = fn_state.part;
        // D s_1_6: cast zx s_1_5 -> i
        let s_1_6: i128 = (i128::try_from(s_1_5).unwrap());
        // D s_1_7: cast zx s_1_2 -> i
        let s_1_7: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_8: call Vpart_read(s_1_4, s_1_6, s_1_7)
        let s_1_8: Bits = Vpart_read(state, tracer, s_1_4, s_1_6, s_1_7);
        // D s_1_9: write-var operand1 <= s_1_8
        fn_state.operand1 = s_1_8;
        // D s_1_10: read-var idxdsizeshadow#1868:i64
        let s_1_10: i64 = fn_state.idxdsizeshadow_1868;
        // D s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_12: cast reint s_1_11 -> i64
        let s_1_12: i64 = (s_1_11 as i64);
        // D s_1_13: read-var m:i64
        let s_1_13: i64 = fn_state.m;
        // D s_1_14: cast zx s_1_13 -> i
        let s_1_14: i128 = (i128::try_from(s_1_13).unwrap());
        // D s_1_15: call V_read(s_1_14, s_1_12)
        let s_1_15: Bits = V_read(state, tracer, s_1_14, s_1_12);
        // C s_1_16: const #2s : i
        let s_1_16: i128 = 2;
        // D s_1_17: read-var datasize:i64
        let s_1_17: i64 = fn_state.datasize;
        // D s_1_18: cast zx s_1_17 -> i
        let s_1_18: i128 = (i128::try_from(s_1_17).unwrap());
        // D s_1_19: mul s_1_16 s_1_18
        let s_1_19: i128 = ((s_1_16) * (s_1_18));
        // D s_1_20: cast reint s_1_19 -> i64
        let s_1_20: i64 = (s_1_19 as i64);
        // D s_1_21: cast zx s_1_20 -> i
        let s_1_21: i128 = (i128::try_from(s_1_20).unwrap());
        // D s_1_22: cast reint s_1_21 -> i64
        let s_1_22: i64 = (s_1_21 as i64);
        // D s_1_23: read-var d:i64
        let s_1_23: i64 = fn_state.d;
        // D s_1_24: cast zx s_1_23 -> i
        let s_1_24: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_25: call V_read(s_1_24, s_1_22)
        let s_1_25: Bits = V_read(state, tracer, s_1_24, s_1_22);
        // D s_1_26: write-var operand3 <= s_1_25
        fn_state.operand3 = s_1_25;
        // D s_1_27: read-var esizeshadow#1869:i64
        let s_1_27: i64 = fn_state.esizeshadow_1869;
        // D s_1_28: cast zx s_1_27 -> i
        let s_1_28: i128 = (i128::try_from(s_1_27).unwrap());
        // D s_1_29: cast reint s_1_28 -> i64
        let s_1_29: i64 = (s_1_28 as i64);
        // D s_1_30: read-var index:i64
        let s_1_30: i64 = fn_state.index;
        // D s_1_31: cast zx s_1_30 -> i
        let s_1_31: i128 = (i128::try_from(s_1_30).unwrap());
        // D s_1_32: cast zx s_1_29 -> i
        let s_1_32: i128 = (i128::try_from(s_1_29).unwrap());
        // D s_1_33: call Elem_read(s_1_15, s_1_31, s_1_32)
        let s_1_33: Bits = Elem_read(state, tracer, s_1_15, s_1_31, s_1_32);
        // D s_1_34: read-var is_unsigned:u8
        let s_1_34: bool = fn_state.is_unsigned;
        // D s_1_35: call asl_Int(s_1_33, s_1_34)
        let s_1_35: i128 = asl_Int(state, tracer, s_1_33, s_1_34);
        // D s_1_36: write-var element2 <= s_1_35
        fn_state.element2 = s_1_35;
        // C s_1_37: const #0s : i64
        let s_1_37: i64 = 0;
        // C s_1_38: const #1s : i
        let s_1_38: i128 = 1;
        // D s_1_39: read-var elements:i
        let s_1_39: i128 = fn_state.elements;
        // D s_1_40: sub s_1_39 s_1_38
        let s_1_40: i128 = ((s_1_39) - (s_1_38));
        // D s_1_41: cast reint s_1_40 -> i64
        let s_1_41: i64 = (s_1_40 as i64);
        // D s_1_42: write-var gs#169992 <= s_1_41
        fn_state.gs_169992 = s_1_41;
        // D s_1_43: write-var e <= s_1_37
        fn_state.e = s_1_37;
        // N s_1_44: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#169992:i64
        let s_2_1: i64 = fn_state.gs_169992;
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
        // D s_3_0: read-var esizeshadow#1869:i64
        let s_3_0: i64 = fn_state.esizeshadow_1869;
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
        // D s_3_8: read-var is_unsigned:u8
        let s_3_8: bool = fn_state.is_unsigned;
        // D s_3_9: call asl_Int(s_3_7, s_3_8)
        let s_3_9: i128 = asl_Int(state, tracer, s_3_7, s_3_8);
        // D s_3_10: read-var element2:i
        let s_3_10: i128 = fn_state.element2;
        // D s_3_11: mul s_3_9 s_3_10
        let s_3_11: i128 = ((s_3_9) * (s_3_10));
        // C s_3_12: const #2s : i
        let s_3_12: i128 = 2;
        // D s_3_13: read-var esizeshadow#1869:i64
        let s_3_13: i64 = fn_state.esizeshadow_1869;
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (i128::try_from(s_3_13).unwrap());
        // D s_3_15: mul s_3_12 s_3_14
        let s_3_15: i128 = ((s_3_12) * (s_3_14));
        // D s_3_16: cast reint s_3_15 -> i64
        let s_3_16: i64 = (s_3_15 as i64);
        // C s_3_17: const #1s : i
        let s_3_17: i128 = 1;
        // D s_3_18: cast zx s_3_16 -> i
        let s_3_18: i128 = (i128::try_from(s_3_16).unwrap());
        // D s_3_19: sub s_3_18 s_3_17
        let s_3_19: i128 = ((s_3_18) - (s_3_17));
        // D s_3_20: cast reint s_3_19 -> i64
        let s_3_20: i64 = (s_3_19 as i64);
        // C s_3_21: const #0s : i
        let s_3_21: i128 = 0;
        // D s_3_22: cast zx s_3_20 -> i
        let s_3_22: i128 = (i128::try_from(s_3_20).unwrap());
        // D s_3_23: call integer_subrange(s_3_11, s_3_22, s_3_21)
        let s_3_23: Bits = integer_subrange(state, tracer, s_3_11, s_3_22, s_3_21);
        // D s_3_24: write-var product <= s_3_23
        fn_state.product = s_3_23;
        // D s_3_25: read-var sub_op:u8
        let s_3_25: bool = fn_state.sub_op;
        // N s_3_26: branch s_3_25 b6 b4
        if s_3_25 {
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
        // C s_4_0: const #2s : i
        let s_4_0: i128 = 2;
        // D s_4_1: read-var esizeshadow#1869:i64
        let s_4_1: i64 = fn_state.esizeshadow_1869;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: mul s_4_0 s_4_2
        let s_4_3: i128 = ((s_4_0) * (s_4_2));
        // D s_4_4: cast reint s_4_3 -> i64
        let s_4_4: i64 = (s_4_3 as i64);
        // D s_4_5: cast zx s_4_4 -> i
        let s_4_5: i128 = (i128::try_from(s_4_4).unwrap());
        // D s_4_6: cast reint s_4_5 -> i64
        let s_4_6: i64 = (s_4_5 as i64);
        // C s_4_7: const #2s : i
        let s_4_7: i128 = 2;
        // D s_4_8: read-var esizeshadow#1869:i64
        let s_4_8: i64 = fn_state.esizeshadow_1869;
        // D s_4_9: cast zx s_4_8 -> i
        let s_4_9: i128 = (i128::try_from(s_4_8).unwrap());
        // D s_4_10: mul s_4_7 s_4_9
        let s_4_10: i128 = ((s_4_7) * (s_4_9));
        // D s_4_11: cast reint s_4_10 -> i64
        let s_4_11: i64 = (s_4_10 as i64);
        // D s_4_12: cast zx s_4_11 -> i
        let s_4_12: i128 = (i128::try_from(s_4_11).unwrap());
        // D s_4_13: cast reint s_4_12 -> i64
        let s_4_13: i64 = (s_4_12 as i64);
        // D s_4_14: read-var e:i64
        let s_4_14: i64 = fn_state.e;
        // D s_4_15: cast zx s_4_14 -> i
        let s_4_15: i128 = (i128::try_from(s_4_14).unwrap());
        // D s_4_16: cast zx s_4_13 -> i
        let s_4_16: i128 = (i128::try_from(s_4_13).unwrap());
        // D s_4_17: read-var operand3:bv
        let s_4_17: Bits = fn_state.operand3;
        // D s_4_18: call Elem_read(s_4_17, s_4_15, s_4_16)
        let s_4_18: Bits = Elem_read(state, tracer, s_4_17, s_4_15, s_4_16);
        // D s_4_19: read-var product:bv
        let s_4_19: Bits = fn_state.product;
        // D s_4_20: add s_4_18 s_4_19
        let s_4_20: Bits = (s_4_18 + s_4_19);
        // D s_4_21: read-var e:i64
        let s_4_21: i64 = fn_state.e;
        // D s_4_22: cast zx s_4_21 -> i
        let s_4_22: i128 = (i128::try_from(s_4_21).unwrap());
        // D s_4_23: cast zx s_4_6 -> i
        let s_4_23: i128 = (i128::try_from(s_4_6).unwrap());
        // D s_4_24: read-var result:bv
        let s_4_24: Bits = fn_state.result;
        // D s_4_25: call Elem_set(s_4_24, s_4_22, s_4_23, s_4_20)
        let s_4_25: Bits = Elem_set(state, tracer, s_4_24, s_4_22, s_4_23, s_4_20);
        // D s_4_26: write-var result <= s_4_25
        fn_state.result = s_4_25;
        // N s_4_27: jump b5
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
        // C s_6_0: const #2s : i
        let s_6_0: i128 = 2;
        // D s_6_1: read-var esizeshadow#1869:i64
        let s_6_1: i64 = fn_state.esizeshadow_1869;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: mul s_6_0 s_6_2
        let s_6_3: i128 = ((s_6_0) * (s_6_2));
        // D s_6_4: cast reint s_6_3 -> i64
        let s_6_4: i64 = (s_6_3 as i64);
        // D s_6_5: cast zx s_6_4 -> i
        let s_6_5: i128 = (i128::try_from(s_6_4).unwrap());
        // D s_6_6: cast reint s_6_5 -> i64
        let s_6_6: i64 = (s_6_5 as i64);
        // C s_6_7: const #2s : i
        let s_6_7: i128 = 2;
        // D s_6_8: read-var esizeshadow#1869:i64
        let s_6_8: i64 = fn_state.esizeshadow_1869;
        // D s_6_9: cast zx s_6_8 -> i
        let s_6_9: i128 = (i128::try_from(s_6_8).unwrap());
        // D s_6_10: mul s_6_7 s_6_9
        let s_6_10: i128 = ((s_6_7) * (s_6_9));
        // D s_6_11: cast reint s_6_10 -> i64
        let s_6_11: i64 = (s_6_10 as i64);
        // D s_6_12: cast zx s_6_11 -> i
        let s_6_12: i128 = (i128::try_from(s_6_11).unwrap());
        // D s_6_13: cast reint s_6_12 -> i64
        let s_6_13: i64 = (s_6_12 as i64);
        // D s_6_14: read-var e:i64
        let s_6_14: i64 = fn_state.e;
        // D s_6_15: cast zx s_6_14 -> i
        let s_6_15: i128 = (i128::try_from(s_6_14).unwrap());
        // D s_6_16: cast zx s_6_13 -> i
        let s_6_16: i128 = (i128::try_from(s_6_13).unwrap());
        // D s_6_17: read-var operand3:bv
        let s_6_17: Bits = fn_state.operand3;
        // D s_6_18: call Elem_read(s_6_17, s_6_15, s_6_16)
        let s_6_18: Bits = Elem_read(state, tracer, s_6_17, s_6_15, s_6_16);
        // D s_6_19: read-var product:bv
        let s_6_19: Bits = fn_state.product;
        // D s_6_20: sub s_6_18 s_6_19
        let s_6_20: Bits = ((s_6_18) - (s_6_19));
        // D s_6_21: read-var e:i64
        let s_6_21: i64 = fn_state.e;
        // D s_6_22: cast zx s_6_21 -> i
        let s_6_22: i128 = (i128::try_from(s_6_21).unwrap());
        // D s_6_23: cast zx s_6_6 -> i
        let s_6_23: i128 = (i128::try_from(s_6_6).unwrap());
        // D s_6_24: read-var result:bv
        let s_6_24: Bits = fn_state.result;
        // D s_6_25: call Elem_set(s_6_24, s_6_22, s_6_23, s_6_20)
        let s_6_25: Bits = Elem_set(state, tracer, s_6_24, s_6_22, s_6_23, s_6_20);
        // D s_6_26: write-var result <= s_6_25
        fn_state.result = s_6_25;
        // N s_6_27: jump b5
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
