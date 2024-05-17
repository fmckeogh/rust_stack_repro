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
pub fn execute_aarch64_instrs_vector_arithmetic_binary_uniform_mul_int_dotp<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    elements: i128,
    esize: i64,
    m: i64,
    n: i64,
    is_signed: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        gs_167667: i64,
        res: i128,
        element2: i128,
        element1: i128,
        esizeshadow_1817: i64,
        result: Bits,
        i: i64,
        operand1: Bits,
        datasizeshadow_1818: i64,
        operand2: Bits,
        d: i64,
        datasize: i64,
        elements: i128,
        esize: i64,
        m: i64,
        n: i64,
        is_signed: bool,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        elements,
        esize,
        m,
        n,
        is_signed,
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
        // D s_0_3: write-var esizeshadow#1817 <= s_0_2
        fn_state.esizeshadow_1817 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var datasizeshadow#1818 <= s_0_6
        fn_state.datasizeshadow_1818 = s_0_6;
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
        // D s_1_0: read-var datasizeshadow#1818:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1818;
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
        // D s_1_7: read-var datasizeshadow#1818:i64
        let s_1_7: i64 = fn_state.datasizeshadow_1818;
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
        // D s_1_13: write-var operand2 <= s_1_12
        fn_state.operand2 = s_1_12;
        // D s_1_14: read-var datasizeshadow#1818:i64
        let s_1_14: i64 = fn_state.datasizeshadow_1818;
        // D s_1_15: cast zx s_1_14 -> i
        let s_1_15: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_16: cast reint s_1_15 -> i64
        let s_1_16: i64 = (s_1_15 as i64);
        // D s_1_17: read-var d:i64
        let s_1_17: i64 = fn_state.d;
        // D s_1_18: cast zx s_1_17 -> i
        let s_1_18: i128 = (i128::try_from(s_1_17).unwrap());
        // D s_1_19: call V_read(s_1_18, s_1_16)
        let s_1_19: Bits = V_read(state, tracer, s_1_18, s_1_16);
        // D s_1_20: write-var result <= s_1_19
        fn_state.result = s_1_19;
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
        // D s_1_26: write-var gs#167667 <= s_1_25
        fn_state.gs_167667 = s_1_25;
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
        // D s_2_1: read-var gs#167667:i64
        let s_2_1: i64 = fn_state.gs_167667;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b10 b3
        if s_2_2 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0s : i
        let s_3_0: i128 = 0;
        // D s_3_1: write-var res <= s_3_0
        fn_state.res = s_3_0;
        // C s_3_2: const #0s : i64
        let s_3_2: i64 = 0;
        // D s_3_3: write-var i <= s_3_2
        fn_state.i = s_3_2;
        // N s_3_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var i:i64
        let s_4_0: i64 = fn_state.i;
        // C s_4_1: const #3s : i64
        let s_4_1: i64 = 3;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b9 b5
        if s_4_2 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var is_signed:u8
        let s_5_0: bool = fn_state.is_signed;
        // N s_5_1: branch s_5_0 b8 b6
        if s_5_0 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #4s : i
        let s_6_0: i128 = 4;
        // D s_6_1: read-var e:i64
        let s_6_1: i64 = fn_state.e;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: mul s_6_0 s_6_2
        let s_6_3: i128 = ((s_6_0) * (s_6_2));
        // D s_6_4: read-var i:i64
        let s_6_4: i64 = fn_state.i;
        // D s_6_5: cast zx s_6_4 -> i
        let s_6_5: i128 = (i128::try_from(s_6_4).unwrap());
        // D s_6_6: add s_6_3 s_6_5
        let s_6_6: i128 = (s_6_3 + s_6_5);
        // C s_6_7: const #4s : i
        let s_6_7: i128 = 4;
        // D s_6_8: read-var esizeshadow#1817:i64
        let s_6_8: i64 = fn_state.esizeshadow_1817;
        // D s_6_9: cast zx s_6_8 -> i
        let s_6_9: i128 = (i128::try_from(s_6_8).unwrap());
        // D s_6_10: div s_6_9 s_6_7
        let s_6_10: i128 = ((s_6_9) / (s_6_7));
        // D s_6_11: cast reint s_6_10 -> i64
        let s_6_11: i64 = (s_6_10 as i64);
        // D s_6_12: cast zx s_6_11 -> i
        let s_6_12: i128 = (i128::try_from(s_6_11).unwrap());
        // D s_6_13: cast reint s_6_12 -> i64
        let s_6_13: i64 = (s_6_12 as i64);
        // D s_6_14: cast zx s_6_13 -> i
        let s_6_14: i128 = (i128::try_from(s_6_13).unwrap());
        // D s_6_15: read-var operand1:bv
        let s_6_15: Bits = fn_state.operand1;
        // D s_6_16: call Elem_read(s_6_15, s_6_6, s_6_14)
        let s_6_16: Bits = Elem_read(state, tracer, s_6_15, s_6_6, s_6_14);
        // D s_6_17: cast zx s_6_16 -> i
        let s_6_17: i128 = (s_6_16.value() as i128);
        // D s_6_18: write-var element1 <= s_6_17
        fn_state.element1 = s_6_17;
        // C s_6_19: const #4s : i
        let s_6_19: i128 = 4;
        // D s_6_20: read-var e:i64
        let s_6_20: i64 = fn_state.e;
        // D s_6_21: cast zx s_6_20 -> i
        let s_6_21: i128 = (i128::try_from(s_6_20).unwrap());
        // D s_6_22: mul s_6_19 s_6_21
        let s_6_22: i128 = ((s_6_19) * (s_6_21));
        // D s_6_23: read-var i:i64
        let s_6_23: i64 = fn_state.i;
        // D s_6_24: cast zx s_6_23 -> i
        let s_6_24: i128 = (i128::try_from(s_6_23).unwrap());
        // D s_6_25: add s_6_22 s_6_24
        let s_6_25: i128 = (s_6_22 + s_6_24);
        // C s_6_26: const #4s : i
        let s_6_26: i128 = 4;
        // D s_6_27: read-var esizeshadow#1817:i64
        let s_6_27: i64 = fn_state.esizeshadow_1817;
        // D s_6_28: cast zx s_6_27 -> i
        let s_6_28: i128 = (i128::try_from(s_6_27).unwrap());
        // D s_6_29: div s_6_28 s_6_26
        let s_6_29: i128 = ((s_6_28) / (s_6_26));
        // D s_6_30: cast reint s_6_29 -> i64
        let s_6_30: i64 = (s_6_29 as i64);
        // D s_6_31: cast zx s_6_30 -> i
        let s_6_31: i128 = (i128::try_from(s_6_30).unwrap());
        // D s_6_32: cast reint s_6_31 -> i64
        let s_6_32: i64 = (s_6_31 as i64);
        // D s_6_33: cast zx s_6_32 -> i
        let s_6_33: i128 = (i128::try_from(s_6_32).unwrap());
        // D s_6_34: read-var operand2:bv
        let s_6_34: Bits = fn_state.operand2;
        // D s_6_35: call Elem_read(s_6_34, s_6_25, s_6_33)
        let s_6_35: Bits = Elem_read(state, tracer, s_6_34, s_6_25, s_6_33);
        // D s_6_36: cast zx s_6_35 -> i
        let s_6_36: i128 = (s_6_35.value() as i128);
        // D s_6_37: write-var element2 <= s_6_36
        fn_state.element2 = s_6_36;
        // N s_6_38: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var element1:i
        let s_7_0: i128 = fn_state.element1;
        // D s_7_1: read-var element2:i
        let s_7_1: i128 = fn_state.element2;
        // D s_7_2: mul s_7_0 s_7_1
        let s_7_2: i128 = ((s_7_0) * (s_7_1));
        // D s_7_3: read-var res:i
        let s_7_3: i128 = fn_state.res;
        // D s_7_4: add s_7_3 s_7_2
        let s_7_4: i128 = (s_7_3 + s_7_2);
        // D s_7_5: write-var res <= s_7_4
        fn_state.res = s_7_4;
        // D s_7_6: read-var i:i64
        let s_7_6: i64 = fn_state.i;
        // C s_7_7: const #1s : i64
        let s_7_7: i64 = 1;
        // D s_7_8: add s_7_6 s_7_7
        let s_7_8: i64 = (s_7_6 + s_7_7);
        // D s_7_9: write-var i <= s_7_8
        fn_state.i = s_7_8;
        // N s_7_10: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #4s : i
        let s_8_0: i128 = 4;
        // D s_8_1: read-var e:i64
        let s_8_1: i64 = fn_state.e;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // D s_8_3: mul s_8_0 s_8_2
        let s_8_3: i128 = ((s_8_0) * (s_8_2));
        // D s_8_4: read-var i:i64
        let s_8_4: i64 = fn_state.i;
        // D s_8_5: cast zx s_8_4 -> i
        let s_8_5: i128 = (i128::try_from(s_8_4).unwrap());
        // D s_8_6: add s_8_3 s_8_5
        let s_8_6: i128 = (s_8_3 + s_8_5);
        // C s_8_7: const #4s : i
        let s_8_7: i128 = 4;
        // D s_8_8: read-var esizeshadow#1817:i64
        let s_8_8: i64 = fn_state.esizeshadow_1817;
        // D s_8_9: cast zx s_8_8 -> i
        let s_8_9: i128 = (i128::try_from(s_8_8).unwrap());
        // D s_8_10: div s_8_9 s_8_7
        let s_8_10: i128 = ((s_8_9) / (s_8_7));
        // D s_8_11: cast reint s_8_10 -> i64
        let s_8_11: i64 = (s_8_10 as i64);
        // D s_8_12: cast zx s_8_11 -> i
        let s_8_12: i128 = (i128::try_from(s_8_11).unwrap());
        // D s_8_13: cast reint s_8_12 -> i64
        let s_8_13: i64 = (s_8_12 as i64);
        // D s_8_14: cast zx s_8_13 -> i
        let s_8_14: i128 = (i128::try_from(s_8_13).unwrap());
        // D s_8_15: read-var operand1:bv
        let s_8_15: Bits = fn_state.operand1;
        // D s_8_16: call Elem_read(s_8_15, s_8_6, s_8_14)
        let s_8_16: Bits = Elem_read(state, tracer, s_8_15, s_8_6, s_8_14);
        // D s_8_17: cast sx s_8_16 -> i
        let s_8_17: i128 = {
            let sign_bit = s_8_16.length() - 1;
            let mut result = s_8_16.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_8_18: write-var element1 <= s_8_17
        fn_state.element1 = s_8_17;
        // C s_8_19: const #4s : i
        let s_8_19: i128 = 4;
        // D s_8_20: read-var e:i64
        let s_8_20: i64 = fn_state.e;
        // D s_8_21: cast zx s_8_20 -> i
        let s_8_21: i128 = (i128::try_from(s_8_20).unwrap());
        // D s_8_22: mul s_8_19 s_8_21
        let s_8_22: i128 = ((s_8_19) * (s_8_21));
        // D s_8_23: read-var i:i64
        let s_8_23: i64 = fn_state.i;
        // D s_8_24: cast zx s_8_23 -> i
        let s_8_24: i128 = (i128::try_from(s_8_23).unwrap());
        // D s_8_25: add s_8_22 s_8_24
        let s_8_25: i128 = (s_8_22 + s_8_24);
        // C s_8_26: const #4s : i
        let s_8_26: i128 = 4;
        // D s_8_27: read-var esizeshadow#1817:i64
        let s_8_27: i64 = fn_state.esizeshadow_1817;
        // D s_8_28: cast zx s_8_27 -> i
        let s_8_28: i128 = (i128::try_from(s_8_27).unwrap());
        // D s_8_29: div s_8_28 s_8_26
        let s_8_29: i128 = ((s_8_28) / (s_8_26));
        // D s_8_30: cast reint s_8_29 -> i64
        let s_8_30: i64 = (s_8_29 as i64);
        // D s_8_31: cast zx s_8_30 -> i
        let s_8_31: i128 = (i128::try_from(s_8_30).unwrap());
        // D s_8_32: cast reint s_8_31 -> i64
        let s_8_32: i64 = (s_8_31 as i64);
        // D s_8_33: cast zx s_8_32 -> i
        let s_8_33: i128 = (i128::try_from(s_8_32).unwrap());
        // D s_8_34: read-var operand2:bv
        let s_8_34: Bits = fn_state.operand2;
        // D s_8_35: call Elem_read(s_8_34, s_8_25, s_8_33)
        let s_8_35: Bits = Elem_read(state, tracer, s_8_34, s_8_25, s_8_33);
        // D s_8_36: cast sx s_8_35 -> i
        let s_8_36: i128 = {
            let sign_bit = s_8_35.length() - 1;
            let mut result = s_8_35.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_8_37: write-var element2 <= s_8_36
        fn_state.element2 = s_8_36;
        // N s_8_38: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var res:i
        let s_9_0: i128 = fn_state.res;
        // D s_9_1: read-var esizeshadow#1817:i64
        let s_9_1: i64 = fn_state.esizeshadow_1817;
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (i128::try_from(s_9_1).unwrap());
        // D s_9_3: cast reint s_9_2 -> i64
        let s_9_3: i64 = (s_9_2 as i64);
        // D s_9_4: read-var esizeshadow#1817:i64
        let s_9_4: i64 = fn_state.esizeshadow_1817;
        // D s_9_5: cast zx s_9_4 -> i
        let s_9_5: i128 = (i128::try_from(s_9_4).unwrap());
        // D s_9_6: cast reint s_9_5 -> i64
        let s_9_6: i64 = (s_9_5 as i64);
        // D s_9_7: read-var e:i64
        let s_9_7: i64 = fn_state.e;
        // D s_9_8: cast zx s_9_7 -> i
        let s_9_8: i128 = (i128::try_from(s_9_7).unwrap());
        // D s_9_9: cast zx s_9_6 -> i
        let s_9_9: i128 = (i128::try_from(s_9_6).unwrap());
        // D s_9_10: read-var result:bv
        let s_9_10: Bits = fn_state.result;
        // D s_9_11: call Elem_read(s_9_10, s_9_8, s_9_9)
        let s_9_11: Bits = Elem_read(state, tracer, s_9_10, s_9_8, s_9_9);
        // D s_9_12: cast cvt s_9_0 -> bv
        let s_9_12: Bits = Bits::new(s_9_0 as u128, 128);
        // D s_9_13: add s_9_11 s_9_12
        let s_9_13: Bits = (s_9_11 + s_9_12);
        // D s_9_14: read-var e:i64
        let s_9_14: i64 = fn_state.e;
        // D s_9_15: cast zx s_9_14 -> i
        let s_9_15: i128 = (i128::try_from(s_9_14).unwrap());
        // D s_9_16: cast zx s_9_3 -> i
        let s_9_16: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_17: read-var result:bv
        let s_9_17: Bits = fn_state.result;
        // D s_9_18: call Elem_set(s_9_17, s_9_15, s_9_16, s_9_13)
        let s_9_18: Bits = Elem_set(state, tracer, s_9_17, s_9_15, s_9_16, s_9_13);
        // D s_9_19: write-var result <= s_9_18
        fn_state.result = s_9_18;
        // D s_9_20: read-var e:i64
        let s_9_20: i64 = fn_state.e;
        // C s_9_21: const #1s : i64
        let s_9_21: i64 = 1;
        // D s_9_22: add s_9_20 s_9_21
        let s_9_22: i64 = (s_9_20 + s_9_21);
        // D s_9_23: write-var e <= s_9_22
        fn_state.e = s_9_22;
        // N s_9_24: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var datasizeshadow#1818:i64
        let s_10_0: i64 = fn_state.datasizeshadow_1818;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: cast reint s_10_1 -> i64
        let s_10_2: i64 = (s_10_1 as i64);
        // D s_10_3: read-var d:i64
        let s_10_3: i64 = fn_state.d;
        // D s_10_4: cast zx s_10_3 -> i
        let s_10_4: i128 = (i128::try_from(s_10_3).unwrap());
        // D s_10_5: read-var result:bv
        let s_10_5: Bits = fn_state.result;
        // D s_10_6: call V_set(s_10_4, s_10_2, s_10_5)
        let s_10_6: () = V_set(state, tracer, s_10_4, s_10_2, s_10_5);
        // N s_10_7: return
        return;
    }
}
