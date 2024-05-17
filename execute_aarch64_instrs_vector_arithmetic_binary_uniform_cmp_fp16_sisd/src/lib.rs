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
use FPCompareEQ::*;
use Elem_set::*;
use V_read::*;
use FPCR_read::*;
use Ones::*;
use FPCompareGE::*;
use FPAbs::*;
use V_set::*;
use IsMerging::*;
use Elem_read::*;
use Zeros::*;
use FPCompareGT::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_arithmetic_binary_uniform_cmp_fp16_sisd<T: Tracer>(
    state: &mut State,
    tracer: &T,
    abs: bool,
    cmp: u32,
    d: i64,
    datasize: i64,
    elements: i128,
    esize: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        element2: Bits,
        fpcr: ProductType5c790c8ef59cc8b2,
        e: i64,
        element1: Bits,
        gs_148396: bool,
        test_passed: bool,
        esizeshadow_1202: i64,
        gs_148402: i64,
        datasizeshadow_1203: i64,
        ga_252750: i64,
        ga_252751: Bits,
        result: u128,
        operand1: Bits,
        operand2: Bits,
        abs: bool,
        cmp: u32,
        d: i64,
        datasize: i64,
        elements: i128,
        esize: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        abs,
        cmp,
        d,
        datasize,
        elements,
        esize,
        m,
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
        // D s_0_3: write-var esizeshadow#1202 <= s_0_2
        fn_state.esizeshadow_1202 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var datasizeshadow#1203 <= s_0_6
        fn_state.datasizeshadow_1203 = s_0_6;
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
        // D s_1_0: read-var datasizeshadow#1203:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1203;
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
        // D s_1_7: read-var datasizeshadow#1203:i64
        let s_1_7: i64 = fn_state.datasizeshadow_1203;
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
        // C s_1_14: const #() : ()
        let s_1_14: () = ();
        // S s_1_15: call FPCR_read(s_1_14)
        let s_1_15: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_1_14);
        // D s_1_16: write-var fpcr <= s_1_15
        fn_state.fpcr = s_1_15;
        // C s_1_17: const #1s : i
        let s_1_17: i128 = 1;
        // D s_1_18: read-var elements:i
        let s_1_18: i128 = fn_state.elements;
        // D s_1_19: cmp-eq s_1_18 s_1_17
        let s_1_19: bool = ((s_1_18) == (s_1_17));
        // N s_1_20: branch s_1_19 b28 b2
        if s_1_19 {
            return block_28(state, tracer, fn_state);
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
        // D s_2_1: write-var gs#148396 <= s_2_0
        fn_state.gs_148396 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#148396:u8
        let s_3_0: bool = fn_state.gs_148396;
        // N s_3_1: branch s_3_0 b27 b4
        if s_3_0 {
            return block_27(state, tracer, fn_state);
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
        // D s_5_5: write-var gs#148402 <= s_5_4
        fn_state.gs_148402 = s_5_4;
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
        // D s_6_1: read-var gs#148402:i64
        let s_6_1: i64 = fn_state.gs_148402;
        // D s_6_2: cmp-gt s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) > (s_6_1));
        // N s_6_3: branch s_6_2 b26 b7
        if s_6_2 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var esizeshadow#1202:i64
        let s_7_0: i64 = fn_state.esizeshadow_1202;
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
        // D s_7_9: read-var esizeshadow#1202:i64
        let s_7_9: i64 = fn_state.esizeshadow_1202;
        // D s_7_10: cast zx s_7_9 -> i
        let s_7_10: i128 = (i128::try_from(s_7_9).unwrap());
        // D s_7_11: cast reint s_7_10 -> i64
        let s_7_11: i64 = (s_7_10 as i64);
        // D s_7_12: read-var e:i64
        let s_7_12: i64 = fn_state.e;
        // D s_7_13: cast zx s_7_12 -> i
        let s_7_13: i128 = (i128::try_from(s_7_12).unwrap());
        // D s_7_14: cast zx s_7_11 -> i
        let s_7_14: i128 = (i128::try_from(s_7_11).unwrap());
        // D s_7_15: read-var operand2:bv
        let s_7_15: Bits = fn_state.operand2;
        // D s_7_16: call Elem_read(s_7_15, s_7_13, s_7_14)
        let s_7_16: Bits = Elem_read(state, tracer, s_7_15, s_7_13, s_7_14);
        // D s_7_17: write-var element2 <= s_7_16
        fn_state.element2 = s_7_16;
        // D s_7_18: read-var abs:u8
        let s_7_18: bool = fn_state.abs;
        // N s_7_19: branch s_7_18 b23 b8
        if s_7_18 {
            return block_23(state, tracer, fn_state);
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
        // C s_9_0: const #2u : u32
        let s_9_0: u32 = 2;
        // D s_9_1: read-var cmp:u32
        let s_9_1: u32 = fn_state.cmp;
        // D s_9_2: cmp-eq s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) == (s_9_1));
        // D s_9_3: not s_9_2
        let s_9_3: bool = !s_9_2;
        // N s_9_4: branch s_9_3 b16 b10
        if s_9_3 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var element1:bv
        let s_10_0: Bits = fn_state.element1;
        // D s_10_1: read-var element2:bv
        let s_10_1: Bits = fn_state.element2;
        // D s_10_2: read-var fpcr:struct
        let s_10_2: ProductType5c790c8ef59cc8b2 = fn_state.fpcr;
        // D s_10_3: call FPCompareEQ(s_10_0, s_10_1, s_10_2)
        let s_10_3: bool = FPCompareEQ(state, tracer, s_10_0, s_10_1, s_10_2);
        // D s_10_4: write-var test_passed <= s_10_3
        fn_state.test_passed = s_10_3;
        // N s_10_5: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var esizeshadow#1202:i64
        let s_12_0: i64 = fn_state.esizeshadow_1202;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: cast reint s_12_1 -> i64
        let s_12_2: i64 = (s_12_1 as i64);
        // D s_12_3: write-var ga#252750 <= s_12_2
        fn_state.ga_252750 = s_12_2;
        // D s_12_4: read-var test_passed:u8
        let s_12_4: bool = fn_state.test_passed;
        // N s_12_5: branch s_12_4 b15 b13
        if s_12_4 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var esizeshadow#1202:i64
        let s_13_0: i64 = fn_state.esizeshadow_1202;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: call Zeros(s_13_1)
        let s_13_2: Bits = Zeros(state, tracer, s_13_1);
        // D s_13_3: write-var ga#252751 <= s_13_2
        fn_state.ga_252751 = s_13_2;
        // N s_13_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var result:u128
        let s_14_0: u128 = fn_state.result;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 128u16);
        // D s_14_2: read-var e:i64
        let s_14_2: i64 = fn_state.e;
        // D s_14_3: cast zx s_14_2 -> i
        let s_14_3: i128 = (i128::try_from(s_14_2).unwrap());
        // D s_14_4: read-var ga#252750:i64
        let s_14_4: i64 = fn_state.ga_252750;
        // D s_14_5: cast zx s_14_4 -> i
        let s_14_5: i128 = (i128::try_from(s_14_4).unwrap());
        // D s_14_6: read-var ga#252751:bv
        let s_14_6: Bits = fn_state.ga_252751;
        // D s_14_7: call Elem_set(s_14_1, s_14_3, s_14_5, s_14_6)
        let s_14_7: Bits = Elem_set(state, tracer, s_14_1, s_14_3, s_14_5, s_14_6);
        // D s_14_8: cast reint s_14_7 -> u128
        let s_14_8: u128 = (s_14_7.value() as u128);
        // D s_14_9: write-var result <= s_14_8
        fn_state.result = s_14_8;
        // D s_14_10: read-var e:i64
        let s_14_10: i64 = fn_state.e;
        // C s_14_11: const #1s : i64
        let s_14_11: i64 = 1;
        // D s_14_12: add s_14_10 s_14_11
        let s_14_12: i64 = (s_14_10 + s_14_11);
        // D s_14_13: write-var e <= s_14_12
        fn_state.e = s_14_12;
        // N s_14_14: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var esizeshadow#1202:i64
        let s_15_0: i64 = fn_state.esizeshadow_1202;
        // D s_15_1: cast zx s_15_0 -> i
        let s_15_1: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_2: call Ones(s_15_1)
        let s_15_2: Bits = Ones(state, tracer, s_15_1);
        // D s_15_3: write-var ga#252751 <= s_15_2
        fn_state.ga_252751 = s_15_2;
        // N s_15_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u32
        let s_16_0: u32 = 1;
        // D s_16_1: read-var cmp:u32
        let s_16_1: u32 = fn_state.cmp;
        // D s_16_2: cmp-eq s_16_0 s_16_1
        let s_16_2: bool = ((s_16_0) == (s_16_1));
        // D s_16_3: not s_16_2
        let s_16_3: bool = !s_16_2;
        // N s_16_4: branch s_16_3 b19 b17
        if s_16_3 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var element1:bv
        let s_17_0: Bits = fn_state.element1;
        // D s_17_1: read-var element2:bv
        let s_17_1: Bits = fn_state.element2;
        // D s_17_2: read-var fpcr:struct
        let s_17_2: ProductType5c790c8ef59cc8b2 = fn_state.fpcr;
        // D s_17_3: call FPCompareGE(s_17_0, s_17_1, s_17_2)
        let s_17_3: bool = FPCompareGE(state, tracer, s_17_0, s_17_1, s_17_2);
        // D s_17_4: write-var test_passed <= s_17_3
        fn_state.test_passed = s_17_3;
        // N s_17_5: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #0u : u32
        let s_19_0: u32 = 0;
        // D s_19_1: read-var cmp:u32
        let s_19_1: u32 = fn_state.cmp;
        // D s_19_2: cmp-eq s_19_0 s_19_1
        let s_19_2: bool = ((s_19_0) == (s_19_1));
        // D s_19_3: not s_19_2
        let s_19_3: bool = !s_19_2;
        // N s_19_4: branch s_19_3 b22 b20
        if s_19_3 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var element1:bv
        let s_20_0: Bits = fn_state.element1;
        // D s_20_1: read-var element2:bv
        let s_20_1: Bits = fn_state.element2;
        // D s_20_2: read-var fpcr:struct
        let s_20_2: ProductType5c790c8ef59cc8b2 = fn_state.fpcr;
        // D s_20_3: call FPCompareGT(s_20_0, s_20_1, s_20_2)
        let s_20_3: bool = FPCompareGT(state, tracer, s_20_0, s_20_1, s_20_2);
        // D s_20_4: write-var test_passed <= s_20_3
        fn_state.test_passed = s_20_3;
        // N s_20_5: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_21_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var element1:bv
        let s_23_0: Bits = fn_state.element1;
        // D s_23_1: call FPAbs(s_23_0)
        let s_23_1: Bits = FPAbs(state, tracer, s_23_0);
        // D s_23_2: write-var element1 <= s_23_1
        fn_state.element1 = s_23_1;
        // N s_23_3: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var element2:bv
        let s_24_0: Bits = fn_state.element2;
        // D s_24_1: call FPAbs(s_24_0)
        let s_24_1: Bits = FPAbs(state, tracer, s_24_0);
        // D s_24_2: write-var element2 <= s_24_1
        fn_state.element2 = s_24_1;
        // N s_24_3: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_25_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #128s : i64
        let s_26_0: i64 = 128;
        // D s_26_1: read-var d:i64
        let s_26_1: i64 = fn_state.d;
        // D s_26_2: cast zx s_26_1 -> i
        let s_26_2: i128 = (i128::try_from(s_26_1).unwrap());
        // D s_26_3: read-var result:u128
        let s_26_3: u128 = fn_state.result;
        // D s_26_4: cast zx s_26_3 -> bv
        let s_26_4: Bits = Bits::new(s_26_3 as u128, 128u16);
        // D s_26_5: call V_set(s_26_2, s_26_0, s_26_4)
        let s_26_5: () = V_set(state, tracer, s_26_2, s_26_0, s_26_4);
        // N s_26_6: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #128s : i64
        let s_27_0: i64 = 128;
        // D s_27_1: read-var m:i64
        let s_27_1: i64 = fn_state.m;
        // D s_27_2: cast zx s_27_1 -> i
        let s_27_2: i128 = (i128::try_from(s_27_1).unwrap());
        // D s_27_3: call V_read(s_27_2, s_27_0)
        let s_27_3: Bits = V_read(state, tracer, s_27_2, s_27_0);
        // D s_27_4: cast reint s_27_3 -> u128
        let s_27_4: u128 = (s_27_3.value() as u128);
        // D s_27_5: write-var result <= s_27_4
        fn_state.result = s_27_4;
        // N s_27_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var fpcr:struct
        let s_28_0: ProductType5c790c8ef59cc8b2 = fn_state.fpcr;
        // D s_28_1: call IsMerging(s_28_0)
        let s_28_1: bool = IsMerging(state, tracer, s_28_0);
        // D s_28_2: write-var gs#148396 <= s_28_1
        fn_state.gs_148396 = s_28_1;
        // N s_28_3: jump b3
        return block_3(state, tracer, fn_state);
    }
}
