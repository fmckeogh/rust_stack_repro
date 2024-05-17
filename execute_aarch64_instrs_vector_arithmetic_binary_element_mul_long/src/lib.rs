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
pub fn execute_aarch64_instrs_vector_arithmetic_binary_element_mul_long<T: Tracer>(
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
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        element2: i128,
        e: i64,
        esizeshadow_1890: i64,
        idxdsizeshadow_1889: i64,
        result: Bits,
        operand1: Bits,
        gs_170307: i64,
        d: i64,
        datasize: i64,
        elements: i128,
        esize: i64,
        idxdsize: i64,
        index: i64,
        m: i64,
        n: i64,
        part: i64,
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
        // D s_0_3: write-var idxdsizeshadow#1889 <= s_0_2
        fn_state.idxdsizeshadow_1889 = s_0_2;
        // D s_0_4: read-var esize:i64
        let s_0_4: i64 = fn_state.esize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var esizeshadow#1890 <= s_0_6
        fn_state.esizeshadow_1890 = s_0_6;
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
        // D s_1_10: read-var idxdsizeshadow#1889:i64
        let s_1_10: i64 = fn_state.idxdsizeshadow_1889;
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
        // D s_1_16: read-var esizeshadow#1890:i64
        let s_1_16: i64 = fn_state.esizeshadow_1890;
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_18: cast reint s_1_17 -> i64
        let s_1_18: i64 = (s_1_17 as i64);
        // D s_1_19: read-var index:i64
        let s_1_19: i64 = fn_state.index;
        // D s_1_20: cast zx s_1_19 -> i
        let s_1_20: i128 = (i128::try_from(s_1_19).unwrap());
        // D s_1_21: cast zx s_1_18 -> i
        let s_1_21: i128 = (i128::try_from(s_1_18).unwrap());
        // D s_1_22: call Elem_read(s_1_15, s_1_20, s_1_21)
        let s_1_22: Bits = Elem_read(state, tracer, s_1_15, s_1_20, s_1_21);
        // D s_1_23: read-var is_unsigned:u8
        let s_1_23: bool = fn_state.is_unsigned;
        // D s_1_24: call asl_Int(s_1_22, s_1_23)
        let s_1_24: i128 = asl_Int(state, tracer, s_1_22, s_1_23);
        // D s_1_25: write-var element2 <= s_1_24
        fn_state.element2 = s_1_24;
        // C s_1_26: const #0s : i64
        let s_1_26: i64 = 0;
        // C s_1_27: const #1s : i
        let s_1_27: i128 = 1;
        // D s_1_28: read-var elements:i
        let s_1_28: i128 = fn_state.elements;
        // D s_1_29: sub s_1_28 s_1_27
        let s_1_29: i128 = ((s_1_28) - (s_1_27));
        // D s_1_30: cast reint s_1_29 -> i64
        let s_1_30: i64 = (s_1_29 as i64);
        // D s_1_31: write-var gs#170307 <= s_1_30
        fn_state.gs_170307 = s_1_30;
        // D s_1_32: write-var e <= s_1_26
        fn_state.e = s_1_26;
        // N s_1_33: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#170307:i64
        let s_2_1: i64 = fn_state.gs_170307;
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
        // D s_3_0: read-var esizeshadow#1890:i64
        let s_3_0: i64 = fn_state.esizeshadow_1890;
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
        // D s_3_13: read-var esizeshadow#1890:i64
        let s_3_13: i64 = fn_state.esizeshadow_1890;
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
        // C s_3_24: const #2s : i
        let s_3_24: i128 = 2;
        // D s_3_25: read-var esizeshadow#1890:i64
        let s_3_25: i64 = fn_state.esizeshadow_1890;
        // D s_3_26: cast zx s_3_25 -> i
        let s_3_26: i128 = (i128::try_from(s_3_25).unwrap());
        // D s_3_27: mul s_3_24 s_3_26
        let s_3_27: i128 = ((s_3_24) * (s_3_26));
        // D s_3_28: cast reint s_3_27 -> i64
        let s_3_28: i64 = (s_3_27 as i64);
        // D s_3_29: cast zx s_3_28 -> i
        let s_3_29: i128 = (i128::try_from(s_3_28).unwrap());
        // D s_3_30: cast reint s_3_29 -> i64
        let s_3_30: i64 = (s_3_29 as i64);
        // D s_3_31: read-var e:i64
        let s_3_31: i64 = fn_state.e;
        // D s_3_32: cast zx s_3_31 -> i
        let s_3_32: i128 = (i128::try_from(s_3_31).unwrap());
        // D s_3_33: cast zx s_3_30 -> i
        let s_3_33: i128 = (i128::try_from(s_3_30).unwrap());
        // D s_3_34: read-var result:bv
        let s_3_34: Bits = fn_state.result;
        // D s_3_35: call Elem_set(s_3_34, s_3_32, s_3_33, s_3_23)
        let s_3_35: Bits = Elem_set(state, tracer, s_3_34, s_3_32, s_3_33, s_3_23);
        // D s_3_36: write-var result <= s_3_35
        fn_state.result = s_3_35;
        // D s_3_37: read-var e:i64
        let s_3_37: i64 = fn_state.e;
        // C s_3_38: const #1s : i64
        let s_3_38: i64 = 1;
        // D s_3_39: add s_3_37 s_3_38
        let s_3_39: i64 = (s_3_37 + s_3_38);
        // D s_3_40: write-var e <= s_3_39
        fn_state.e = s_3_39;
        // N s_3_41: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #2s : i
        let s_4_0: i128 = 2;
        // D s_4_1: read-var datasize:i64
        let s_4_1: i64 = fn_state.datasize;
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
        // D s_4_7: read-var d:i64
        let s_4_7: i64 = fn_state.d;
        // D s_4_8: cast zx s_4_7 -> i
        let s_4_8: i128 = (i128::try_from(s_4_7).unwrap());
        // D s_4_9: read-var result:bv
        let s_4_9: Bits = fn_state.result;
        // D s_4_10: call V_set(s_4_8, s_4_6, s_4_9)
        let s_4_10: () = V_set(state, tracer, s_4_8, s_4_6, s_4_9);
        // N s_4_11: return
        return;
    }
}
