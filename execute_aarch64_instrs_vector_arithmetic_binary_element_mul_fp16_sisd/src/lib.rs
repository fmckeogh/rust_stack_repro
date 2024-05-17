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
use FPMulX::*;
use Elem_set::*;
use V_read::*;
use FPCR_read::*;
use FPMul::*;
use V_set::*;
use IsMerging::*;
use Elem_read::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_arithmetic_binary_element_mul_fp16_sisd<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    elements: i128,
    esize: i64,
    idxdsize: i64,
    index: i64,
    m: i64,
    mulx_op: bool,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_255900: i64,
        element2: Bits,
        ga_255903: Bits,
        idxdsizeshadow_1413: i64,
        fpcr: ProductType5c790c8ef59cc8b2,
        e: i64,
        element1: Bits,
        gs_153626: bool,
        datasizeshadow_1415: i64,
        ga_255901: Bits,
        gs_153632: i64,
        esizeshadow_1414: i64,
        ga_255902: i64,
        result: u128,
        operand1: Bits,
        d: i64,
        datasize: i64,
        elements: i128,
        esize: i64,
        idxdsize: i64,
        index: i64,
        m: i64,
        mulx_op: bool,
        n: i64,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        elements,
        esize,
        idxdsize,
        index,
        m,
        mulx_op,
        n,
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
        // D s_0_3: write-var idxdsizeshadow#1413 <= s_0_2
        fn_state.idxdsizeshadow_1413 = s_0_2;
        // D s_0_4: read-var esize:i64
        let s_0_4: i64 = fn_state.esize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var esizeshadow#1414 <= s_0_6
        fn_state.esizeshadow_1414 = s_0_6;
        // D s_0_8: read-var datasize:i64
        let s_0_8: i64 = fn_state.datasize;
        // D s_0_9: cast zx s_0_8 -> i
        let s_0_9: i128 = (i128::try_from(s_0_8).unwrap());
        // D s_0_10: cast reint s_0_9 -> i64
        let s_0_10: i64 = (s_0_9 as i64);
        // D s_0_11: write-var datasizeshadow#1415 <= s_0_10
        fn_state.datasizeshadow_1415 = s_0_10;
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
        // D s_1_0: read-var datasizeshadow#1415:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1415;
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
        // D s_1_7: read-var idxdsizeshadow#1413:i64
        let s_1_7: i64 = fn_state.idxdsizeshadow_1413;
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
        // D s_1_13: read-var esizeshadow#1414:i64
        let s_1_13: i64 = fn_state.esizeshadow_1414;
        // D s_1_14: cast zx s_1_13 -> i
        let s_1_14: i128 = (i128::try_from(s_1_13).unwrap());
        // D s_1_15: cast reint s_1_14 -> i64
        let s_1_15: i64 = (s_1_14 as i64);
        // D s_1_16: read-var index:i64
        let s_1_16: i64 = fn_state.index;
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_18: cast zx s_1_15 -> i
        let s_1_18: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_19: call Elem_read(s_1_12, s_1_17, s_1_18)
        let s_1_19: Bits = Elem_read(state, tracer, s_1_12, s_1_17, s_1_18);
        // D s_1_20: write-var element2 <= s_1_19
        fn_state.element2 = s_1_19;
        // C s_1_21: const #() : ()
        let s_1_21: () = ();
        // S s_1_22: call FPCR_read(s_1_21)
        let s_1_22: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_1_21);
        // D s_1_23: write-var fpcr <= s_1_22
        fn_state.fpcr = s_1_22;
        // C s_1_24: const #1s : i
        let s_1_24: i128 = 1;
        // D s_1_25: read-var elements:i
        let s_1_25: i128 = fn_state.elements;
        // D s_1_26: cmp-eq s_1_25 s_1_24
        let s_1_26: bool = ((s_1_25) == (s_1_24));
        // N s_1_27: branch s_1_26 b15 b2
        if s_1_26 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#153626 <= s_2_0
        fn_state.gs_153626 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#153626:u8
        let s_3_0: bool = fn_state.gs_153626;
        // N s_3_1: branch s_3_0 b14 b4
        if s_3_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: u8 = 0;
        // C s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 0u16);
        // C s_4_2: const #0u : u64
        let s_4_2: u64 = 0;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 64u16);
        // C s_4_4: cast reint s_4_1 -> u128
        let s_4_4: u128 = (s_4_1.value() as u128);
        // D s_4_5: size-of s_4_1
        let s_4_5: u16 = s_4_1.length();
        // C s_4_6: cast reint s_4_3 -> u128
        let s_4_6: u128 = (s_4_3.value() as u128);
        // D s_4_7: size-of s_4_3
        let s_4_7: u16 = s_4_3.length();
        // D s_4_8: lsl s_4_4 s_4_7
        let s_4_8: u128 = s_4_4 << s_4_7;
        // D s_4_9: or s_4_8 s_4_6
        let s_4_9: u128 = ((s_4_8) | (s_4_6));
        // D s_4_10: add s_4_5 s_4_7
        let s_4_10: u16 = (s_4_5 + s_4_7);
        // D s_4_11: create-bits s_4_9 s_4_10
        let s_4_11: Bits = Bits::new(s_4_9, s_4_10);
        // C s_4_12: const #0u : u64
        let s_4_12: u64 = 0;
        // C s_4_13: cast zx s_4_12 -> bv
        let s_4_13: Bits = Bits::new(s_4_12 as u128, 64u16);
        // D s_4_14: cast reint s_4_11 -> u128
        let s_4_14: u128 = (s_4_11.value() as u128);
        // D s_4_15: size-of s_4_11
        let s_4_15: u16 = s_4_11.length();
        // C s_4_16: cast reint s_4_13 -> u128
        let s_4_16: u128 = (s_4_13.value() as u128);
        // D s_4_17: size-of s_4_13
        let s_4_17: u16 = s_4_13.length();
        // D s_4_18: lsl s_4_14 s_4_17
        let s_4_18: u128 = s_4_14 << s_4_17;
        // D s_4_19: or s_4_18 s_4_16
        let s_4_19: u128 = ((s_4_18) | (s_4_16));
        // D s_4_20: add s_4_15 s_4_17
        let s_4_20: u16 = (s_4_15 + s_4_17);
        // D s_4_21: create-bits s_4_19 s_4_20
        let s_4_21: Bits = Bits::new(s_4_19, s_4_20);
        // D s_4_22: cast reint s_4_21 -> u128
        let s_4_22: u128 = (s_4_21.value() as u128);
        // D s_4_23: write-var result <= s_4_22
        fn_state.result = s_4_22;
        // N s_4_24: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0s : i64
        let s_5_0: i64 = 0;
        // C s_5_1: const #1s : i
        let s_5_1: i128 = 1;
        // D s_5_2: read-var elements:i
        let s_5_2: i128 = fn_state.elements;
        // D s_5_3: sub s_5_2 s_5_1
        let s_5_3: i128 = ((s_5_2) - (s_5_1));
        // D s_5_4: cast reint s_5_3 -> i64
        let s_5_4: i64 = (s_5_3 as i64);
        // D s_5_5: write-var gs#153632 <= s_5_4
        fn_state.gs_153632 = s_5_4;
        // D s_5_6: write-var e <= s_5_0
        fn_state.e = s_5_0;
        // N s_5_7: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var e:i64
        let s_6_0: i64 = fn_state.e;
        // D s_6_1: read-var gs#153632:i64
        let s_6_1: i64 = fn_state.gs_153632;
        // D s_6_2: cmp-gt s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) > (s_6_1));
        // N s_6_3: branch s_6_2 b13 b7
        if s_6_2 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var esizeshadow#1414:i64
        let s_7_0: i64 = fn_state.esizeshadow_1414;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: cast reint s_7_1 -> i64
        let s_7_2: i64 = (s_7_1 as i64);
        // D s_7_3: read-var e:i64
        let s_7_3: i64 = fn_state.e;
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_5: cast zx s_7_2 -> i
        let s_7_5: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_6: read-var operand1:bv
        let s_7_6: Bits = fn_state.operand1;
        // D s_7_7: call Elem_read(s_7_6, s_7_4, s_7_5)
        let s_7_7: Bits = Elem_read(state, tracer, s_7_6, s_7_4, s_7_5);
        // D s_7_8: write-var element1 <= s_7_7
        fn_state.element1 = s_7_7;
        // D s_7_9: read-var mulx_op:u8
        let s_7_9: bool = fn_state.mulx_op;
        // N s_7_10: branch s_7_9 b11 b8
        if s_7_9 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var esizeshadow#1414:i64
        let s_8_0: i64 = fn_state.esizeshadow_1414;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: cast reint s_8_1 -> i64
        let s_8_2: i64 = (s_8_1 as i64);
        // D s_8_3: write-var ga#255902 <= s_8_2
        fn_state.ga_255902 = s_8_2;
        // D s_8_4: read-var element1:bv
        let s_8_4: Bits = fn_state.element1;
        // D s_8_5: read-var element2:bv
        let s_8_5: Bits = fn_state.element2;
        // D s_8_6: read-var fpcr:struct
        let s_8_6: ProductType5c790c8ef59cc8b2 = fn_state.fpcr;
        // D s_8_7: call FPMul(s_8_4, s_8_5, s_8_6)
        let s_8_7: Bits = FPMul(state, tracer, s_8_4, s_8_5, s_8_6);
        // D s_8_8: write-var ga#255903 <= s_8_7
        fn_state.ga_255903 = s_8_7;
        // N s_8_9: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var result:u128
        let s_9_0: u128 = fn_state.result;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 128u16);
        // D s_9_2: read-var e:i64
        let s_9_2: i64 = fn_state.e;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: read-var ga#255902:i64
        let s_9_4: i64 = fn_state.ga_255902;
        // D s_9_5: cast zx s_9_4 -> i
        let s_9_5: i128 = (i128::try_from(s_9_4).unwrap());
        // D s_9_6: read-var ga#255903:bv
        let s_9_6: Bits = fn_state.ga_255903;
        // D s_9_7: call Elem_set(s_9_1, s_9_3, s_9_5, s_9_6)
        let s_9_7: Bits = Elem_set(state, tracer, s_9_1, s_9_3, s_9_5, s_9_6);
        // D s_9_8: cast reint s_9_7 -> u128
        let s_9_8: u128 = (s_9_7.value() as u128);
        // D s_9_9: write-var result <= s_9_8
        fn_state.result = s_9_8;
        // N s_9_10: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var e:i64
        let s_10_0: i64 = fn_state.e;
        // C s_10_1: const #1s : i64
        let s_10_1: i64 = 1;
        // D s_10_2: add s_10_0 s_10_1
        let s_10_2: i64 = (s_10_0 + s_10_1);
        // D s_10_3: write-var e <= s_10_2
        fn_state.e = s_10_2;
        // N s_10_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var esizeshadow#1414:i64
        let s_11_0: i64 = fn_state.esizeshadow_1414;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: cast reint s_11_1 -> i64
        let s_11_2: i64 = (s_11_1 as i64);
        // D s_11_3: write-var ga#255900 <= s_11_2
        fn_state.ga_255900 = s_11_2;
        // D s_11_4: read-var element1:bv
        let s_11_4: Bits = fn_state.element1;
        // D s_11_5: read-var element2:bv
        let s_11_5: Bits = fn_state.element2;
        // D s_11_6: read-var fpcr:struct
        let s_11_6: ProductType5c790c8ef59cc8b2 = fn_state.fpcr;
        // D s_11_7: call FPMulX(s_11_4, s_11_5, s_11_6)
        let s_11_7: Bits = FPMulX(state, tracer, s_11_4, s_11_5, s_11_6);
        // D s_11_8: write-var ga#255901 <= s_11_7
        fn_state.ga_255901 = s_11_7;
        // N s_11_9: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var result:u128
        let s_12_0: u128 = fn_state.result;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 128u16);
        // D s_12_2: read-var e:i64
        let s_12_2: i64 = fn_state.e;
        // D s_12_3: cast zx s_12_2 -> i
        let s_12_3: i128 = (i128::try_from(s_12_2).unwrap());
        // D s_12_4: read-var ga#255900:i64
        let s_12_4: i64 = fn_state.ga_255900;
        // D s_12_5: cast zx s_12_4 -> i
        let s_12_5: i128 = (i128::try_from(s_12_4).unwrap());
        // D s_12_6: read-var ga#255901:bv
        let s_12_6: Bits = fn_state.ga_255901;
        // D s_12_7: call Elem_set(s_12_1, s_12_3, s_12_5, s_12_6)
        let s_12_7: Bits = Elem_set(state, tracer, s_12_1, s_12_3, s_12_5, s_12_6);
        // D s_12_8: cast reint s_12_7 -> u128
        let s_12_8: u128 = (s_12_7.value() as u128);
        // D s_12_9: write-var result <= s_12_8
        fn_state.result = s_12_8;
        // N s_12_10: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #128s : i64
        let s_13_0: i64 = 128;
        // D s_13_1: read-var d:i64
        let s_13_1: i64 = fn_state.d;
        // D s_13_2: cast zx s_13_1 -> i
        let s_13_2: i128 = (i128::try_from(s_13_1).unwrap());
        // D s_13_3: read-var result:u128
        let s_13_3: u128 = fn_state.result;
        // D s_13_4: cast zx s_13_3 -> bv
        let s_13_4: Bits = Bits::new(s_13_3 as u128, 128u16);
        // D s_13_5: call V_set(s_13_2, s_13_0, s_13_4)
        let s_13_5: () = V_set(state, tracer, s_13_2, s_13_0, s_13_4);
        // N s_13_6: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #128s : i64
        let s_14_0: i64 = 128;
        // D s_14_1: read-var n:i64
        let s_14_1: i64 = fn_state.n;
        // D s_14_2: cast zx s_14_1 -> i
        let s_14_2: i128 = (i128::try_from(s_14_1).unwrap());
        // D s_14_3: call V_read(s_14_2, s_14_0)
        let s_14_3: Bits = V_read(state, tracer, s_14_2, s_14_0);
        // D s_14_4: cast reint s_14_3 -> u128
        let s_14_4: u128 = (s_14_3.value() as u128);
        // D s_14_5: write-var result <= s_14_4
        fn_state.result = s_14_4;
        // N s_14_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var fpcr:struct
        let s_15_0: ProductType5c790c8ef59cc8b2 = fn_state.fpcr;
        // D s_15_1: call IsMerging(s_15_0)
        let s_15_1: bool = IsMerging(state, tracer, s_15_0);
        // D s_15_2: write-var gs#153626 <= s_15_1
        fn_state.gs_153626 = s_15_1;
        // N s_15_3: jump b3
        return block_3(state, tracer, fn_state);
    }
}
