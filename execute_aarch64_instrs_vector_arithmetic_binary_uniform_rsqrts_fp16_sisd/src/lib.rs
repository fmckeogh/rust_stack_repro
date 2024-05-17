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
use CheckFPEnabled64::*;
use V_set::*;
use Elem_set::*;
use V_read::*;
use FPCR_read::*;
use FPRSqrtStepFused::*;
use IsMerging::*;
use Elem_read::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_arithmetic_binary_uniform_rsqrts_fp16_sisd<
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
) -> () {
    #[derive(Default)]
    struct FunctionState {
        fpcr: ProductType5c790c8ef59cc8b2,
        esizeshadow_1467: i64,
        e: i64,
        ga_256677: Bits,
        datasizeshadow_1468: i64,
        result: u128,
        operand1: Bits,
        gs_154858: i64,
        gs_154852: bool,
        ga_256676: i64,
        operand2: Bits,
        d: i64,
        datasize: i64,
        elements: i128,
        esize: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
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
        // D s_0_3: write-var esizeshadow#1467 <= s_0_2
        fn_state.esizeshadow_1467 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var datasizeshadow#1468 <= s_0_6
        fn_state.datasizeshadow_1468 = s_0_6;
        // C s_0_8: const #1s : i
        let s_0_8: i128 = 1;
        // D s_0_9: read-var elements:i
        let s_0_9: i128 = fn_state.elements;
        // D s_0_10: cmp-eq s_0_9 s_0_8
        let s_0_10: bool = ((s_0_9) == (s_0_8));
        // N s_0_11: branch s_0_10 b13 b1
        if s_0_10 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call CheckFPAdvSIMDEnabled64(s_1_0)
        let s_1_1: () = CheckFPAdvSIMDEnabled64(state, tracer, s_1_0);
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var datasizeshadow#1468:i64
        let s_2_0: i64 = fn_state.datasizeshadow_1468;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: cast reint s_2_1 -> i64
        let s_2_2: i64 = (s_2_1 as i64);
        // D s_2_3: read-var n:i64
        let s_2_3: i64 = fn_state.n;
        // D s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_5: call V_read(s_2_4, s_2_2)
        let s_2_5: Bits = V_read(state, tracer, s_2_4, s_2_2);
        // D s_2_6: write-var operand1 <= s_2_5
        fn_state.operand1 = s_2_5;
        // D s_2_7: read-var datasizeshadow#1468:i64
        let s_2_7: i64 = fn_state.datasizeshadow_1468;
        // D s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (i128::try_from(s_2_7).unwrap());
        // D s_2_9: cast reint s_2_8 -> i64
        let s_2_9: i64 = (s_2_8 as i64);
        // D s_2_10: read-var m:i64
        let s_2_10: i64 = fn_state.m;
        // D s_2_11: cast zx s_2_10 -> i
        let s_2_11: i128 = (i128::try_from(s_2_10).unwrap());
        // D s_2_12: call V_read(s_2_11, s_2_9)
        let s_2_12: Bits = V_read(state, tracer, s_2_11, s_2_9);
        // D s_2_13: write-var operand2 <= s_2_12
        fn_state.operand2 = s_2_12;
        // C s_2_14: const #() : ()
        let s_2_14: () = ();
        // S s_2_15: call FPCR_read(s_2_14)
        let s_2_15: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_2_14);
        // D s_2_16: write-var fpcr <= s_2_15
        fn_state.fpcr = s_2_15;
        // C s_2_17: const #1s : i
        let s_2_17: i128 = 1;
        // D s_2_18: read-var elements:i
        let s_2_18: i128 = fn_state.elements;
        // D s_2_19: cmp-eq s_2_18 s_2_17
        let s_2_19: bool = ((s_2_18) == (s_2_17));
        // N s_2_20: branch s_2_19 b12 b3
        if s_2_19 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#154852 <= s_3_0
        fn_state.gs_154852 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#154852:u8
        let s_4_0: bool = fn_state.gs_154852;
        // N s_4_1: branch s_4_0 b11 b5
        if s_4_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: u8 = 0;
        // C s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 0u16);
        // C s_5_2: const #0u : u64
        let s_5_2: u64 = 0;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 64u16);
        // C s_5_4: cast reint s_5_1 -> u128
        let s_5_4: u128 = (s_5_1.value() as u128);
        // D s_5_5: size-of s_5_1
        let s_5_5: u16 = s_5_1.length();
        // C s_5_6: cast reint s_5_3 -> u128
        let s_5_6: u128 = (s_5_3.value() as u128);
        // D s_5_7: size-of s_5_3
        let s_5_7: u16 = s_5_3.length();
        // D s_5_8: lsl s_5_4 s_5_7
        let s_5_8: u128 = s_5_4 << s_5_7;
        // D s_5_9: or s_5_8 s_5_6
        let s_5_9: u128 = ((s_5_8) | (s_5_6));
        // D s_5_10: add s_5_5 s_5_7
        let s_5_10: u16 = (s_5_5 + s_5_7);
        // D s_5_11: create-bits s_5_9 s_5_10
        let s_5_11: Bits = Bits::new(s_5_9, s_5_10);
        // C s_5_12: const #0u : u64
        let s_5_12: u64 = 0;
        // C s_5_13: cast zx s_5_12 -> bv
        let s_5_13: Bits = Bits::new(s_5_12 as u128, 64u16);
        // D s_5_14: cast reint s_5_11 -> u128
        let s_5_14: u128 = (s_5_11.value() as u128);
        // D s_5_15: size-of s_5_11
        let s_5_15: u16 = s_5_11.length();
        // C s_5_16: cast reint s_5_13 -> u128
        let s_5_16: u128 = (s_5_13.value() as u128);
        // D s_5_17: size-of s_5_13
        let s_5_17: u16 = s_5_13.length();
        // D s_5_18: lsl s_5_14 s_5_17
        let s_5_18: u128 = s_5_14 << s_5_17;
        // D s_5_19: or s_5_18 s_5_16
        let s_5_19: u128 = ((s_5_18) | (s_5_16));
        // D s_5_20: add s_5_15 s_5_17
        let s_5_20: u16 = (s_5_15 + s_5_17);
        // D s_5_21: create-bits s_5_19 s_5_20
        let s_5_21: Bits = Bits::new(s_5_19, s_5_20);
        // D s_5_22: cast reint s_5_21 -> u128
        let s_5_22: u128 = (s_5_21.value() as u128);
        // D s_5_23: write-var result <= s_5_22
        fn_state.result = s_5_22;
        // N s_5_24: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0s : i64
        let s_6_0: i64 = 0;
        // C s_6_1: const #1s : i
        let s_6_1: i128 = 1;
        // D s_6_2: read-var elements:i
        let s_6_2: i128 = fn_state.elements;
        // D s_6_3: sub s_6_2 s_6_1
        let s_6_3: i128 = ((s_6_2) - (s_6_1));
        // D s_6_4: cast reint s_6_3 -> i64
        let s_6_4: i64 = (s_6_3 as i64);
        // D s_6_5: write-var gs#154858 <= s_6_4
        fn_state.gs_154858 = s_6_4;
        // D s_6_6: write-var e <= s_6_0
        fn_state.e = s_6_0;
        // N s_6_7: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var e:i64
        let s_7_0: i64 = fn_state.e;
        // D s_7_1: read-var gs#154858:i64
        let s_7_1: i64 = fn_state.gs_154858;
        // D s_7_2: cmp-gt s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) > (s_7_1));
        // N s_7_3: branch s_7_2 b10 b8
        if s_7_2 {
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
        // D s_8_0: read-var esizeshadow#1467:i64
        let s_8_0: i64 = fn_state.esizeshadow_1467;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: cast reint s_8_1 -> i64
        let s_8_2: i64 = (s_8_1 as i64);
        // D s_8_3: read-var e:i64
        let s_8_3: i64 = fn_state.e;
        // D s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // D s_8_5: cast zx s_8_2 -> i
        let s_8_5: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_6: read-var operand1:bv
        let s_8_6: Bits = fn_state.operand1;
        // D s_8_7: call Elem_read(s_8_6, s_8_4, s_8_5)
        let s_8_7: Bits = Elem_read(state, tracer, s_8_6, s_8_4, s_8_5);
        // D s_8_8: read-var esizeshadow#1467:i64
        let s_8_8: i64 = fn_state.esizeshadow_1467;
        // D s_8_9: cast zx s_8_8 -> i
        let s_8_9: i128 = (i128::try_from(s_8_8).unwrap());
        // D s_8_10: cast reint s_8_9 -> i64
        let s_8_10: i64 = (s_8_9 as i64);
        // D s_8_11: read-var e:i64
        let s_8_11: i64 = fn_state.e;
        // D s_8_12: cast zx s_8_11 -> i
        let s_8_12: i128 = (i128::try_from(s_8_11).unwrap());
        // D s_8_13: cast zx s_8_10 -> i
        let s_8_13: i128 = (i128::try_from(s_8_10).unwrap());
        // D s_8_14: read-var operand2:bv
        let s_8_14: Bits = fn_state.operand2;
        // D s_8_15: call Elem_read(s_8_14, s_8_12, s_8_13)
        let s_8_15: Bits = Elem_read(state, tracer, s_8_14, s_8_12, s_8_13);
        // D s_8_16: read-var esizeshadow#1467:i64
        let s_8_16: i64 = fn_state.esizeshadow_1467;
        // D s_8_17: cast zx s_8_16 -> i
        let s_8_17: i128 = (i128::try_from(s_8_16).unwrap());
        // D s_8_18: cast reint s_8_17 -> i64
        let s_8_18: i64 = (s_8_17 as i64);
        // D s_8_19: write-var ga#256676 <= s_8_18
        fn_state.ga_256676 = s_8_18;
        // D s_8_20: call FPRSqrtStepFused(s_8_7, s_8_15)
        let s_8_20: Bits = FPRSqrtStepFused(state, tracer, s_8_7, s_8_15);
        // D s_8_21: write-var ga#256677 <= s_8_20
        fn_state.ga_256677 = s_8_20;
        // N s_8_22: jump b9
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
        // D s_9_4: read-var ga#256676:i64
        let s_9_4: i64 = fn_state.ga_256676;
        // D s_9_5: cast zx s_9_4 -> i
        let s_9_5: i128 = (i128::try_from(s_9_4).unwrap());
        // D s_9_6: read-var ga#256677:bv
        let s_9_6: Bits = fn_state.ga_256677;
        // D s_9_7: call Elem_set(s_9_1, s_9_3, s_9_5, s_9_6)
        let s_9_7: Bits = Elem_set(state, tracer, s_9_1, s_9_3, s_9_5, s_9_6);
        // D s_9_8: cast reint s_9_7 -> u128
        let s_9_8: u128 = (s_9_7.value() as u128);
        // D s_9_9: write-var result <= s_9_8
        fn_state.result = s_9_8;
        // D s_9_10: read-var e:i64
        let s_9_10: i64 = fn_state.e;
        // C s_9_11: const #1s : i64
        let s_9_11: i64 = 1;
        // D s_9_12: add s_9_10 s_9_11
        let s_9_12: i64 = (s_9_10 + s_9_11);
        // D s_9_13: write-var e <= s_9_12
        fn_state.e = s_9_12;
        // N s_9_14: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #128s : i64
        let s_10_0: i64 = 128;
        // D s_10_1: read-var d:i64
        let s_10_1: i64 = fn_state.d;
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // D s_10_3: read-var result:u128
        let s_10_3: u128 = fn_state.result;
        // D s_10_4: cast zx s_10_3 -> bv
        let s_10_4: Bits = Bits::new(s_10_3 as u128, 128u16);
        // D s_10_5: call V_set(s_10_2, s_10_0, s_10_4)
        let s_10_5: () = V_set(state, tracer, s_10_2, s_10_0, s_10_4);
        // N s_10_6: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #128s : i64
        let s_11_0: i64 = 128;
        // D s_11_1: read-var n:i64
        let s_11_1: i64 = fn_state.n;
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // D s_11_3: call V_read(s_11_2, s_11_0)
        let s_11_3: Bits = V_read(state, tracer, s_11_2, s_11_0);
        // D s_11_4: cast reint s_11_3 -> u128
        let s_11_4: u128 = (s_11_3.value() as u128);
        // D s_11_5: write-var result <= s_11_4
        fn_state.result = s_11_4;
        // N s_11_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var fpcr:struct
        let s_12_0: ProductType5c790c8ef59cc8b2 = fn_state.fpcr;
        // D s_12_1: call IsMerging(s_12_0)
        let s_12_1: bool = IsMerging(state, tracer, s_12_0);
        // D s_12_2: write-var gs#154852 <= s_12_1
        fn_state.gs_154852 = s_12_1;
        // N s_12_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call CheckFPEnabled64(s_13_0)
        let s_13_1: () = CheckFPEnabled64(state, tracer, s_13_0);
        // N s_13_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
