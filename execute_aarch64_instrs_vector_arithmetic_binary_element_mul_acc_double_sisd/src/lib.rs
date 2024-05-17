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
use Elem_read::*;
use V_set::*;
use SignedSatQ::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_arithmetic_binary_element_mul_acc_double_sisd<
    T: Tracer,
>(
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
) -> () {
    #[derive(Default)]
    struct FunctionState {
        product: Bits,
        e: i64,
        operand3: Bits,
        sat1: bool,
        idxdsizeshadow_1899: i64,
        accum: i128,
        sat2: bool,
        gs_170525: i64,
        element2: i64,
        datasizeshadow_1901: i64,
        gs_170538: bool,
        esizeshadow_1900: i64,
        ga_267237: ProductTypef506aa96a892fe52,
        ga_267250: ProductTypef506aa96a892fe52,
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
        part: i64,
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
        part,
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
        // D s_0_3: write-var idxdsizeshadow#1899 <= s_0_2
        fn_state.idxdsizeshadow_1899 = s_0_2;
        // D s_0_4: read-var esize:i64
        let s_0_4: i64 = fn_state.esize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var esizeshadow#1900 <= s_0_6
        fn_state.esizeshadow_1900 = s_0_6;
        // D s_0_8: read-var datasize:i64
        let s_0_8: i64 = fn_state.datasize;
        // D s_0_9: cast zx s_0_8 -> i
        let s_0_9: i128 = (i128::try_from(s_0_8).unwrap());
        // D s_0_10: cast reint s_0_9 -> i64
        let s_0_10: i64 = (s_0_9 as i64);
        // D s_0_11: write-var datasizeshadow#1901 <= s_0_10
        fn_state.datasizeshadow_1901 = s_0_10;
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
        // D s_1_0: read-var datasizeshadow#1901:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1901;
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
        // D s_1_10: read-var idxdsizeshadow#1899:i64
        let s_1_10: i64 = fn_state.idxdsizeshadow_1899;
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
        // D s_1_17: read-var datasizeshadow#1901:i64
        let s_1_17: i64 = fn_state.datasizeshadow_1901;
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
        // D s_1_27: read-var esizeshadow#1900:i64
        let s_1_27: i64 = fn_state.esizeshadow_1900;
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
        // D s_1_34: cast sx s_1_33 -> i
        let s_1_34: i128 = {
            let sign_bit = s_1_33.length() - 1;
            let mut result = s_1_33.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_1_35: cast reint s_1_34 -> i64
        let s_1_35: i64 = (s_1_34 as i64);
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
        // D s_1_42: write-var gs#170525 <= s_1_41
        fn_state.gs_170525 = s_1_41;
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
        // D s_2_1: read-var gs#170525:i64
        let s_2_1: i64 = fn_state.gs_170525;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b13 b3
        if s_2_2 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var esizeshadow#1900:i64
        let s_3_0: i64 = fn_state.esizeshadow_1900;
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
        // D s_3_8: cast sx s_3_7 -> i
        let s_3_8: i128 = {
            let sign_bit = s_3_7.length() - 1;
            let mut result = s_3_7.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // C s_3_9: const #2s : i
        let s_3_9: i128 = 2;
        // D s_3_10: mul s_3_9 s_3_8
        let s_3_10: i128 = ((s_3_9) * (s_3_8));
        // D s_3_11: read-var element2:i64
        let s_3_11: i64 = fn_state.element2;
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_13: mul s_3_10 s_3_12
        let s_3_13: i128 = ((s_3_10) * (s_3_12));
        // C s_3_14: const #2s : i
        let s_3_14: i128 = 2;
        // D s_3_15: read-var esizeshadow#1900:i64
        let s_3_15: i64 = fn_state.esizeshadow_1900;
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
        // D s_3_21: cast zx s_3_20 -> i
        let s_3_21: i128 = (i128::try_from(s_3_20).unwrap());
        // D s_3_22: call SignedSatQ(s_3_13, s_3_21)
        let s_3_22: ProductTypef506aa96a892fe52 = SignedSatQ(
            state,
            tracer,
            s_3_13,
            s_3_21,
        );
        // D s_3_23: write-var ga#267237 <= s_3_22
        fn_state.ga_267237 = s_3_22;
        // D s_3_24: read-var ga#267237.0:struct
        let s_3_24: Bits = fn_state.ga_267237._0;
        // D s_3_25: read-var ga#267237.1:struct
        let s_3_25: bool = fn_state.ga_267237._1;
        // D s_3_26: write-var product <= s_3_24
        fn_state.product = s_3_24;
        // D s_3_27: write-var sat1 <= s_3_25
        fn_state.sat1 = s_3_25;
        // D s_3_28: read-var sub_op:u8
        let s_3_28: bool = fn_state.sub_op;
        // N s_3_29: branch s_3_28 b12 b4
        if s_3_28 {
            return block_12(state, tracer, fn_state);
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
        // D s_4_1: read-var esizeshadow#1900:i64
        let s_4_1: i64 = fn_state.esizeshadow_1900;
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
        // D s_4_7: read-var e:i64
        let s_4_7: i64 = fn_state.e;
        // D s_4_8: cast zx s_4_7 -> i
        let s_4_8: i128 = (i128::try_from(s_4_7).unwrap());
        // D s_4_9: cast zx s_4_6 -> i
        let s_4_9: i128 = (i128::try_from(s_4_6).unwrap());
        // D s_4_10: read-var operand3:bv
        let s_4_10: Bits = fn_state.operand3;
        // D s_4_11: call Elem_read(s_4_10, s_4_8, s_4_9)
        let s_4_11: Bits = Elem_read(state, tracer, s_4_10, s_4_8, s_4_9);
        // D s_4_12: cast sx s_4_11 -> i
        let s_4_12: i128 = {
            let sign_bit = s_4_11.length() - 1;
            let mut result = s_4_11.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_4_13: read-var product:bv
        let s_4_13: Bits = fn_state.product;
        // D s_4_14: cast sx s_4_13 -> i
        let s_4_14: i128 = {
            let sign_bit = s_4_13.length() - 1;
            let mut result = s_4_13.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_4_15: add s_4_12 s_4_14
        let s_4_15: i128 = (s_4_12 + s_4_14);
        // D s_4_16: write-var accum <= s_4_15
        fn_state.accum = s_4_15;
        // N s_4_17: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #2s : i
        let s_5_0: i128 = 2;
        // D s_5_1: read-var esizeshadow#1900:i64
        let s_5_1: i64 = fn_state.esizeshadow_1900;
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
        // D s_5_7: cast zx s_5_6 -> i
        let s_5_7: i128 = (i128::try_from(s_5_6).unwrap());
        // D s_5_8: read-var accum:i
        let s_5_8: i128 = fn_state.accum;
        // D s_5_9: call SignedSatQ(s_5_8, s_5_7)
        let s_5_9: ProductTypef506aa96a892fe52 = SignedSatQ(state, tracer, s_5_8, s_5_7);
        // D s_5_10: write-var ga#267250 <= s_5_9
        fn_state.ga_267250 = s_5_9;
        // D s_5_11: read-var ga#267250.0:struct
        let s_5_11: Bits = fn_state.ga_267250._0;
        // D s_5_12: read-var ga#267250.1:struct
        let s_5_12: bool = fn_state.ga_267250._1;
        // C s_5_13: const #2s : i
        let s_5_13: i128 = 2;
        // D s_5_14: read-var esizeshadow#1900:i64
        let s_5_14: i64 = fn_state.esizeshadow_1900;
        // D s_5_15: cast zx s_5_14 -> i
        let s_5_15: i128 = (i128::try_from(s_5_14).unwrap());
        // D s_5_16: mul s_5_13 s_5_15
        let s_5_16: i128 = ((s_5_13) * (s_5_15));
        // D s_5_17: cast reint s_5_16 -> i64
        let s_5_17: i64 = (s_5_16 as i64);
        // D s_5_18: cast zx s_5_17 -> i
        let s_5_18: i128 = (i128::try_from(s_5_17).unwrap());
        // D s_5_19: cast reint s_5_18 -> i64
        let s_5_19: i64 = (s_5_18 as i64);
        // D s_5_20: read-var e:i64
        let s_5_20: i64 = fn_state.e;
        // D s_5_21: cast zx s_5_20 -> i
        let s_5_21: i128 = (i128::try_from(s_5_20).unwrap());
        // D s_5_22: cast zx s_5_19 -> i
        let s_5_22: i128 = (i128::try_from(s_5_19).unwrap());
        // D s_5_23: read-var result:bv
        let s_5_23: Bits = fn_state.result;
        // D s_5_24: call Elem_set(s_5_23, s_5_21, s_5_22, s_5_11)
        let s_5_24: Bits = Elem_set(state, tracer, s_5_23, s_5_21, s_5_22, s_5_11);
        // D s_5_25: write-var result <= s_5_24
        fn_state.result = s_5_24;
        // D s_5_26: write-var sat2 <= s_5_12
        fn_state.sat2 = s_5_12;
        // D s_5_27: read-var sat1:u8
        let s_5_27: bool = fn_state.sat1;
        // N s_5_28: branch s_5_27 b11 b6
        if s_5_27 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var sat2:u8
        let s_6_0: bool = fn_state.sat2;
        // D s_6_1: write-var gs#170538 <= s_6_0
        fn_state.gs_170538 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#170538:u8
        let s_7_0: bool = fn_state.gs_170538;
        // N s_7_1: branch s_7_0 b10 b8
        if s_7_0 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var e:i64
        let s_9_0: i64 = fn_state.e;
        // C s_9_1: const #1s : i64
        let s_9_1: i64 = 1;
        // D s_9_2: add s_9_0 s_9_1
        let s_9_2: i64 = (s_9_0 + s_9_1);
        // D s_9_3: write-var e <= s_9_2
        fn_state.e = s_9_2;
        // N s_9_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #20696u : u32
        let s_10_0: u32 = 20696;
        // D s_10_1: read-reg s_10_0:struct
        let s_10_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // C s_10_2: const #20696u : u32
        let s_10_2: u32 = 20696;
        // N s_10_3: write-reg s_10_2 <= s_10_1
        let s_10_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_10_2 as isize, s_10_1);
            tracer.write_register(s_10_2 as isize, s_10_1);
        };
        // N s_10_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #1u : u8
        let s_11_0: bool = true;
        // D s_11_1: write-var gs#170538 <= s_11_0
        fn_state.gs_170538 = s_11_0;
        // N s_11_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #2s : i
        let s_12_0: i128 = 2;
        // D s_12_1: read-var esizeshadow#1900:i64
        let s_12_1: i64 = fn_state.esizeshadow_1900;
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // D s_12_3: mul s_12_0 s_12_2
        let s_12_3: i128 = ((s_12_0) * (s_12_2));
        // D s_12_4: cast reint s_12_3 -> i64
        let s_12_4: i64 = (s_12_3 as i64);
        // D s_12_5: cast zx s_12_4 -> i
        let s_12_5: i128 = (i128::try_from(s_12_4).unwrap());
        // D s_12_6: cast reint s_12_5 -> i64
        let s_12_6: i64 = (s_12_5 as i64);
        // D s_12_7: read-var e:i64
        let s_12_7: i64 = fn_state.e;
        // D s_12_8: cast zx s_12_7 -> i
        let s_12_8: i128 = (i128::try_from(s_12_7).unwrap());
        // D s_12_9: cast zx s_12_6 -> i
        let s_12_9: i128 = (i128::try_from(s_12_6).unwrap());
        // D s_12_10: read-var operand3:bv
        let s_12_10: Bits = fn_state.operand3;
        // D s_12_11: call Elem_read(s_12_10, s_12_8, s_12_9)
        let s_12_11: Bits = Elem_read(state, tracer, s_12_10, s_12_8, s_12_9);
        // D s_12_12: cast sx s_12_11 -> i
        let s_12_12: i128 = {
            let sign_bit = s_12_11.length() - 1;
            let mut result = s_12_11.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_12_13: read-var product:bv
        let s_12_13: Bits = fn_state.product;
        // D s_12_14: cast sx s_12_13 -> i
        let s_12_14: i128 = {
            let sign_bit = s_12_13.length() - 1;
            let mut result = s_12_13.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_12_15: sub s_12_12 s_12_14
        let s_12_15: i128 = ((s_12_12) - (s_12_14));
        // D s_12_16: write-var accum <= s_12_15
        fn_state.accum = s_12_15;
        // N s_12_17: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #2s : i
        let s_13_0: i128 = 2;
        // D s_13_1: read-var datasizeshadow#1901:i64
        let s_13_1: i64 = fn_state.datasizeshadow_1901;
        // D s_13_2: cast zx s_13_1 -> i
        let s_13_2: i128 = (i128::try_from(s_13_1).unwrap());
        // D s_13_3: mul s_13_0 s_13_2
        let s_13_3: i128 = ((s_13_0) * (s_13_2));
        // D s_13_4: cast reint s_13_3 -> i64
        let s_13_4: i64 = (s_13_3 as i64);
        // D s_13_5: cast zx s_13_4 -> i
        let s_13_5: i128 = (i128::try_from(s_13_4).unwrap());
        // D s_13_6: cast reint s_13_5 -> i64
        let s_13_6: i64 = (s_13_5 as i64);
        // D s_13_7: read-var d:i64
        let s_13_7: i64 = fn_state.d;
        // D s_13_8: cast zx s_13_7 -> i
        let s_13_8: i128 = (i128::try_from(s_13_7).unwrap());
        // D s_13_9: read-var result:bv
        let s_13_9: Bits = fn_state.result;
        // D s_13_10: call V_set(s_13_8, s_13_6, s_13_9)
        let s_13_10: () = V_set(state, tracer, s_13_8, s_13_6, s_13_9);
        // N s_13_11: return
        return;
    }
}
