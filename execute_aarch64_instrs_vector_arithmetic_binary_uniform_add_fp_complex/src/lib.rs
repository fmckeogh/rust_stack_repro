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
use fdiv_int::*;
use Elem_set::*;
use V_read::*;
use u__id::*;
use FPCR_read::*;
use FPAdd::*;
use V_set::*;
use FPNeg::*;
use Elem_read::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_arithmetic_binary_uniform_add_fp_complex<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    elements: i128,
    esize: i64,
    m: i64,
    n: i64,
    rot: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_149148: bool,
        e: i64,
        element3: Bits,
        gs_149141: bool,
        gs_149126: bool,
        datasizeshadow_1252: i64,
        gs_149120: i64,
        gs_149139: bool,
        ga_253130: Bits,
        ga_253138: i64,
        element1: Bits,
        gs_149150: bool,
        ga_253139: Bits,
        result: Bits,
        ga_253129: i64,
        operand1: Bits,
        gs_149128: bool,
        esizeshadow_1251: i64,
        ga_253137: i128,
        ga_253128: i128,
        operand2: Bits,
        d: i64,
        datasize: i64,
        elements: i128,
        esize: i64,
        m: i64,
        n: i64,
        rot: bool,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        elements,
        esize,
        m,
        n,
        rot,
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
        // D s_0_3: write-var esizeshadow#1251 <= s_0_2
        fn_state.esizeshadow_1251 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var datasizeshadow#1252 <= s_0_6
        fn_state.datasizeshadow_1252 = s_0_6;
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
        // D s_1_0: read-var datasizeshadow#1252:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1252;
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
        // D s_1_7: read-var datasizeshadow#1252:i64
        let s_1_7: i64 = fn_state.datasizeshadow_1252;
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
        // C s_1_14: const #0s : i64
        let s_1_14: i64 = 0;
        // C s_1_15: const #2s : i
        let s_1_15: i128 = 2;
        // D s_1_16: read-var elements:i
        let s_1_16: i128 = fn_state.elements;
        // D s_1_17: call fdiv_int(s_1_16, s_1_15)
        let s_1_17: i128 = fdiv_int(state, tracer, s_1_16, s_1_15);
        // C s_1_18: const #1s : i
        let s_1_18: i128 = 1;
        // D s_1_19: sub s_1_17 s_1_18
        let s_1_19: i128 = ((s_1_17) - (s_1_18));
        // D s_1_20: cast reint s_1_19 -> i64
        let s_1_20: i64 = (s_1_19 as i64);
        // D s_1_21: write-var gs#149120 <= s_1_20
        fn_state.gs_149120 = s_1_20;
        // D s_1_22: write-var e <= s_1_14
        fn_state.e = s_1_14;
        // N s_1_23: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#149120:i64
        let s_2_1: i64 = fn_state.gs_149120;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b29 b3
        if s_2_2 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var rot:u8
        let s_3_0: bool = fn_state.rot;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 1u16);
        // C s_3_2: const #0u : u8
        let s_3_2: bool = false;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 1u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // D s_3_5: not s_3_4
        let s_3_5: bool = !s_3_4;
        // N s_3_6: branch s_3_5 b21 b4
        if s_3_5 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var esizeshadow#1251:i64
        let s_4_0: i64 = fn_state.esizeshadow_1251;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: call __id(s_4_1)
        let s_4_2: i128 = u__id(state, tracer, s_4_1);
        // D s_4_3: cast reint s_4_2 -> i64
        let s_4_3: i64 = (s_4_2 as i64);
        // C s_4_4: const #16s : i
        let s_4_4: i128 = 16;
        // D s_4_5: cast zx s_4_3 -> i
        let s_4_5: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_6: cmp-eq s_4_5 s_4_4
        let s_4_6: bool = ((s_4_5) == (s_4_4));
        // N s_4_7: branch s_4_6 b20 b5
        if s_4_6 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var esizeshadow#1251:i64
        let s_5_0: i64 = fn_state.esizeshadow_1251;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: call __id(s_5_1)
        let s_5_2: i128 = u__id(state, tracer, s_5_1);
        // D s_5_3: cast reint s_5_2 -> i64
        let s_5_3: i64 = (s_5_2 as i64);
        // C s_5_4: const #32s : i
        let s_5_4: i128 = 32;
        // D s_5_5: cast zx s_5_3 -> i
        let s_5_5: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_6: cmp-eq s_5_5 s_5_4
        let s_5_6: bool = ((s_5_5) == (s_5_4));
        // D s_5_7: write-var gs#149126 <= s_5_6
        fn_state.gs_149126 = s_5_6;
        // N s_5_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#149126:u8
        let s_6_0: bool = fn_state.gs_149126;
        // N s_6_1: branch s_6_0 b19 b7
        if s_6_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var esizeshadow#1251:i64
        let s_7_0: i64 = fn_state.esizeshadow_1251;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: call __id(s_7_1)
        let s_7_2: i128 = u__id(state, tracer, s_7_1);
        // D s_7_3: cast reint s_7_2 -> i64
        let s_7_3: i64 = (s_7_2 as i64);
        // C s_7_4: const #64s : i
        let s_7_4: i128 = 64;
        // D s_7_5: cast zx s_7_3 -> i
        let s_7_5: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_6: cmp-eq s_7_5 s_7_4
        let s_7_6: bool = ((s_7_5) == (s_7_4));
        // D s_7_7: write-var gs#149128 <= s_7_6
        fn_state.gs_149128 = s_7_6;
        // N s_7_8: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#149128:u8
        let s_8_0: bool = fn_state.gs_149128;
        // N s_8_1: assert s_8_0
        let s_8_1: () = assert!(s_8_0);
        // C s_8_2: const #2s : i
        let s_8_2: i128 = 2;
        // D s_8_3: read-var e:i64
        let s_8_3: i64 = fn_state.e;
        // D s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // D s_8_5: mul s_8_4 s_8_2
        let s_8_5: i128 = ((s_8_4) * (s_8_2));
        // C s_8_6: const #1s : i
        let s_8_6: i128 = 1;
        // D s_8_7: add s_8_5 s_8_6
        let s_8_7: i128 = (s_8_5 + s_8_6);
        // D s_8_8: read-var esizeshadow#1251:i64
        let s_8_8: i64 = fn_state.esizeshadow_1251;
        // D s_8_9: cast zx s_8_8 -> i
        let s_8_9: i128 = (i128::try_from(s_8_8).unwrap());
        // D s_8_10: cast reint s_8_9 -> i64
        let s_8_10: i64 = (s_8_9 as i64);
        // D s_8_11: cast zx s_8_10 -> i
        let s_8_11: i128 = (i128::try_from(s_8_10).unwrap());
        // D s_8_12: read-var operand2:bv
        let s_8_12: Bits = fn_state.operand2;
        // D s_8_13: call Elem_read(s_8_12, s_8_7, s_8_11)
        let s_8_13: Bits = Elem_read(state, tracer, s_8_12, s_8_7, s_8_11);
        // D s_8_14: call FPNeg(s_8_13)
        let s_8_14: Bits = FPNeg(state, tracer, s_8_13);
        // D s_8_15: write-var element1 <= s_8_14
        fn_state.element1 = s_8_14;
        // N s_8_16: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #2s : i
        let s_9_0: i128 = 2;
        // D s_9_1: read-var e:i64
        let s_9_1: i64 = fn_state.e;
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (i128::try_from(s_9_1).unwrap());
        // D s_9_3: mul s_9_2 s_9_0
        let s_9_3: i128 = ((s_9_2) * (s_9_0));
        // D s_9_4: read-var esizeshadow#1251:i64
        let s_9_4: i64 = fn_state.esizeshadow_1251;
        // D s_9_5: cast zx s_9_4 -> i
        let s_9_5: i128 = (i128::try_from(s_9_4).unwrap());
        // D s_9_6: cast reint s_9_5 -> i64
        let s_9_6: i64 = (s_9_5 as i64);
        // D s_9_7: cast zx s_9_6 -> i
        let s_9_7: i128 = (i128::try_from(s_9_6).unwrap());
        // D s_9_8: read-var operand2:bv
        let s_9_8: Bits = fn_state.operand2;
        // D s_9_9: call Elem_read(s_9_8, s_9_3, s_9_7)
        let s_9_9: Bits = Elem_read(state, tracer, s_9_8, s_9_3, s_9_7);
        // D s_9_10: write-var element3 <= s_9_9
        fn_state.element3 = s_9_9;
        // N s_9_11: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var esizeshadow#1251:i64
        let s_10_0: i64 = fn_state.esizeshadow_1251;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: call __id(s_10_1)
        let s_10_2: i128 = u__id(state, tracer, s_10_1);
        // D s_10_3: cast reint s_10_2 -> i64
        let s_10_3: i64 = (s_10_2 as i64);
        // C s_10_4: const #16s : i
        let s_10_4: i128 = 16;
        // D s_10_5: cast zx s_10_3 -> i
        let s_10_5: i128 = (i128::try_from(s_10_3).unwrap());
        // D s_10_6: cmp-eq s_10_5 s_10_4
        let s_10_6: bool = ((s_10_5) == (s_10_4));
        // N s_10_7: branch s_10_6 b18 b11
        if s_10_6 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var esizeshadow#1251:i64
        let s_11_0: i64 = fn_state.esizeshadow_1251;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: call __id(s_11_1)
        let s_11_2: i128 = u__id(state, tracer, s_11_1);
        // D s_11_3: cast reint s_11_2 -> i64
        let s_11_3: i64 = (s_11_2 as i64);
        // C s_11_4: const #32s : i
        let s_11_4: i128 = 32;
        // D s_11_5: cast zx s_11_3 -> i
        let s_11_5: i128 = (i128::try_from(s_11_3).unwrap());
        // D s_11_6: cmp-eq s_11_5 s_11_4
        let s_11_6: bool = ((s_11_5) == (s_11_4));
        // D s_11_7: write-var gs#149148 <= s_11_6
        fn_state.gs_149148 = s_11_6;
        // N s_11_8: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#149148:u8
        let s_12_0: bool = fn_state.gs_149148;
        // N s_12_1: branch s_12_0 b17 b13
        if s_12_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var esizeshadow#1251:i64
        let s_13_0: i64 = fn_state.esizeshadow_1251;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: call __id(s_13_1)
        let s_13_2: i128 = u__id(state, tracer, s_13_1);
        // D s_13_3: cast reint s_13_2 -> i64
        let s_13_3: i64 = (s_13_2 as i64);
        // C s_13_4: const #64s : i
        let s_13_4: i128 = 64;
        // D s_13_5: cast zx s_13_3 -> i
        let s_13_5: i128 = (i128::try_from(s_13_3).unwrap());
        // D s_13_6: cmp-eq s_13_5 s_13_4
        let s_13_6: bool = ((s_13_5) == (s_13_4));
        // D s_13_7: write-var gs#149150 <= s_13_6
        fn_state.gs_149150 = s_13_6;
        // N s_13_8: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#149150:u8
        let s_14_0: bool = fn_state.gs_149150;
        // N s_14_1: assert s_14_0
        let s_14_1: () = assert!(s_14_0);
        // C s_14_2: const #2s : i
        let s_14_2: i128 = 2;
        // D s_14_3: read-var e:i64
        let s_14_3: i64 = fn_state.e;
        // D s_14_4: cast zx s_14_3 -> i
        let s_14_4: i128 = (i128::try_from(s_14_3).unwrap());
        // D s_14_5: mul s_14_4 s_14_2
        let s_14_5: i128 = ((s_14_4) * (s_14_2));
        // D s_14_6: write-var ga#253128 <= s_14_5
        fn_state.ga_253128 = s_14_5;
        // D s_14_7: read-var esizeshadow#1251:i64
        let s_14_7: i64 = fn_state.esizeshadow_1251;
        // D s_14_8: cast zx s_14_7 -> i
        let s_14_8: i128 = (i128::try_from(s_14_7).unwrap());
        // D s_14_9: cast reint s_14_8 -> i64
        let s_14_9: i64 = (s_14_8 as i64);
        // D s_14_10: write-var ga#253129 <= s_14_9
        fn_state.ga_253129 = s_14_9;
        // C s_14_11: const #2s : i
        let s_14_11: i128 = 2;
        // D s_14_12: read-var e:i64
        let s_14_12: i64 = fn_state.e;
        // D s_14_13: cast zx s_14_12 -> i
        let s_14_13: i128 = (i128::try_from(s_14_12).unwrap());
        // D s_14_14: mul s_14_13 s_14_11
        let s_14_14: i128 = ((s_14_13) * (s_14_11));
        // D s_14_15: read-var esizeshadow#1251:i64
        let s_14_15: i64 = fn_state.esizeshadow_1251;
        // D s_14_16: cast zx s_14_15 -> i
        let s_14_16: i128 = (i128::try_from(s_14_15).unwrap());
        // D s_14_17: cast reint s_14_16 -> i64
        let s_14_17: i64 = (s_14_16 as i64);
        // D s_14_18: cast zx s_14_17 -> i
        let s_14_18: i128 = (i128::try_from(s_14_17).unwrap());
        // D s_14_19: read-var operand1:bv
        let s_14_19: Bits = fn_state.operand1;
        // D s_14_20: call Elem_read(s_14_19, s_14_14, s_14_18)
        let s_14_20: Bits = Elem_read(state, tracer, s_14_19, s_14_14, s_14_18);
        // C s_14_21: const #() : ()
        let s_14_21: () = ();
        // S s_14_22: call FPCR_read(s_14_21)
        let s_14_22: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_14_21);
        // D s_14_23: read-var element1:bv
        let s_14_23: Bits = fn_state.element1;
        // D s_14_24: call FPAdd(s_14_20, s_14_23, s_14_22)
        let s_14_24: Bits = FPAdd(state, tracer, s_14_20, s_14_23, s_14_22);
        // D s_14_25: write-var ga#253130 <= s_14_24
        fn_state.ga_253130 = s_14_24;
        // N s_14_26: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var ga#253129:i64
        let s_15_0: i64 = fn_state.ga_253129;
        // D s_15_1: cast zx s_15_0 -> i
        let s_15_1: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_2: read-var result:bv
        let s_15_2: Bits = fn_state.result;
        // D s_15_3: read-var ga#253128:i
        let s_15_3: i128 = fn_state.ga_253128;
        // D s_15_4: read-var ga#253130:bv
        let s_15_4: Bits = fn_state.ga_253130;
        // D s_15_5: call Elem_set(s_15_2, s_15_3, s_15_1, s_15_4)
        let s_15_5: Bits = Elem_set(state, tracer, s_15_2, s_15_3, s_15_1, s_15_4);
        // D s_15_6: write-var result <= s_15_5
        fn_state.result = s_15_5;
        // C s_15_7: const #2s : i
        let s_15_7: i128 = 2;
        // D s_15_8: read-var e:i64
        let s_15_8: i64 = fn_state.e;
        // D s_15_9: cast zx s_15_8 -> i
        let s_15_9: i128 = (i128::try_from(s_15_8).unwrap());
        // D s_15_10: mul s_15_9 s_15_7
        let s_15_10: i128 = ((s_15_9) * (s_15_7));
        // C s_15_11: const #1s : i
        let s_15_11: i128 = 1;
        // D s_15_12: add s_15_10 s_15_11
        let s_15_12: i128 = (s_15_10 + s_15_11);
        // D s_15_13: write-var ga#253137 <= s_15_12
        fn_state.ga_253137 = s_15_12;
        // D s_15_14: read-var esizeshadow#1251:i64
        let s_15_14: i64 = fn_state.esizeshadow_1251;
        // D s_15_15: cast zx s_15_14 -> i
        let s_15_15: i128 = (i128::try_from(s_15_14).unwrap());
        // D s_15_16: cast reint s_15_15 -> i64
        let s_15_16: i64 = (s_15_15 as i64);
        // D s_15_17: write-var ga#253138 <= s_15_16
        fn_state.ga_253138 = s_15_16;
        // C s_15_18: const #2s : i
        let s_15_18: i128 = 2;
        // D s_15_19: read-var e:i64
        let s_15_19: i64 = fn_state.e;
        // D s_15_20: cast zx s_15_19 -> i
        let s_15_20: i128 = (i128::try_from(s_15_19).unwrap());
        // D s_15_21: mul s_15_20 s_15_18
        let s_15_21: i128 = ((s_15_20) * (s_15_18));
        // C s_15_22: const #1s : i
        let s_15_22: i128 = 1;
        // D s_15_23: add s_15_21 s_15_22
        let s_15_23: i128 = (s_15_21 + s_15_22);
        // D s_15_24: read-var esizeshadow#1251:i64
        let s_15_24: i64 = fn_state.esizeshadow_1251;
        // D s_15_25: cast zx s_15_24 -> i
        let s_15_25: i128 = (i128::try_from(s_15_24).unwrap());
        // D s_15_26: cast reint s_15_25 -> i64
        let s_15_26: i64 = (s_15_25 as i64);
        // D s_15_27: cast zx s_15_26 -> i
        let s_15_27: i128 = (i128::try_from(s_15_26).unwrap());
        // D s_15_28: read-var operand1:bv
        let s_15_28: Bits = fn_state.operand1;
        // D s_15_29: call Elem_read(s_15_28, s_15_23, s_15_27)
        let s_15_29: Bits = Elem_read(state, tracer, s_15_28, s_15_23, s_15_27);
        // C s_15_30: const #() : ()
        let s_15_30: () = ();
        // S s_15_31: call FPCR_read(s_15_30)
        let s_15_31: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_15_30);
        // D s_15_32: read-var element3:bv
        let s_15_32: Bits = fn_state.element3;
        // D s_15_33: call FPAdd(s_15_29, s_15_32, s_15_31)
        let s_15_33: Bits = FPAdd(state, tracer, s_15_29, s_15_32, s_15_31);
        // D s_15_34: write-var ga#253139 <= s_15_33
        fn_state.ga_253139 = s_15_33;
        // N s_15_35: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var ga#253138:i64
        let s_16_0: i64 = fn_state.ga_253138;
        // D s_16_1: cast zx s_16_0 -> i
        let s_16_1: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_2: read-var result:bv
        let s_16_2: Bits = fn_state.result;
        // D s_16_3: read-var ga#253137:i
        let s_16_3: i128 = fn_state.ga_253137;
        // D s_16_4: read-var ga#253139:bv
        let s_16_4: Bits = fn_state.ga_253139;
        // D s_16_5: call Elem_set(s_16_2, s_16_3, s_16_1, s_16_4)
        let s_16_5: Bits = Elem_set(state, tracer, s_16_2, s_16_3, s_16_1, s_16_4);
        // D s_16_6: write-var result <= s_16_5
        fn_state.result = s_16_5;
        // D s_16_7: read-var e:i64
        let s_16_7: i64 = fn_state.e;
        // C s_16_8: const #1s : i64
        let s_16_8: i64 = 1;
        // D s_16_9: add s_16_7 s_16_8
        let s_16_9: i64 = (s_16_7 + s_16_8);
        // D s_16_10: write-var e <= s_16_9
        fn_state.e = s_16_9;
        // N s_16_11: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var gs#149150 <= s_17_0
        fn_state.gs_149150 = s_17_0;
        // N s_17_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#149148 <= s_18_0
        fn_state.gs_149148 = s_18_0;
        // N s_18_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #1u : u8
        let s_19_0: bool = true;
        // D s_19_1: write-var gs#149128 <= s_19_0
        fn_state.gs_149128 = s_19_0;
        // N s_19_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#149126 <= s_20_0
        fn_state.gs_149126 = s_20_0;
        // N s_20_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #2s : i
        let s_21_0: i128 = 2;
        // D s_21_1: read-var e:i64
        let s_21_1: i64 = fn_state.e;
        // D s_21_2: cast zx s_21_1 -> i
        let s_21_2: i128 = (i128::try_from(s_21_1).unwrap());
        // D s_21_3: mul s_21_2 s_21_0
        let s_21_3: i128 = ((s_21_2) * (s_21_0));
        // C s_21_4: const #1s : i
        let s_21_4: i128 = 1;
        // D s_21_5: add s_21_3 s_21_4
        let s_21_5: i128 = (s_21_3 + s_21_4);
        // D s_21_6: read-var esizeshadow#1251:i64
        let s_21_6: i64 = fn_state.esizeshadow_1251;
        // D s_21_7: cast zx s_21_6 -> i
        let s_21_7: i128 = (i128::try_from(s_21_6).unwrap());
        // D s_21_8: cast reint s_21_7 -> i64
        let s_21_8: i64 = (s_21_7 as i64);
        // D s_21_9: cast zx s_21_8 -> i
        let s_21_9: i128 = (i128::try_from(s_21_8).unwrap());
        // D s_21_10: read-var operand2:bv
        let s_21_10: Bits = fn_state.operand2;
        // D s_21_11: call Elem_read(s_21_10, s_21_5, s_21_9)
        let s_21_11: Bits = Elem_read(state, tracer, s_21_10, s_21_5, s_21_9);
        // D s_21_12: write-var element1 <= s_21_11
        fn_state.element1 = s_21_11;
        // D s_21_13: read-var esizeshadow#1251:i64
        let s_21_13: i64 = fn_state.esizeshadow_1251;
        // D s_21_14: cast zx s_21_13 -> i
        let s_21_14: i128 = (i128::try_from(s_21_13).unwrap());
        // D s_21_15: call __id(s_21_14)
        let s_21_15: i128 = u__id(state, tracer, s_21_14);
        // D s_21_16: cast reint s_21_15 -> i64
        let s_21_16: i64 = (s_21_15 as i64);
        // C s_21_17: const #16s : i
        let s_21_17: i128 = 16;
        // D s_21_18: cast zx s_21_16 -> i
        let s_21_18: i128 = (i128::try_from(s_21_16).unwrap());
        // D s_21_19: cmp-eq s_21_18 s_21_17
        let s_21_19: bool = ((s_21_18) == (s_21_17));
        // N s_21_20: branch s_21_19 b28 b22
        if s_21_19 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var esizeshadow#1251:i64
        let s_22_0: i64 = fn_state.esizeshadow_1251;
        // D s_22_1: cast zx s_22_0 -> i
        let s_22_1: i128 = (i128::try_from(s_22_0).unwrap());
        // D s_22_2: call __id(s_22_1)
        let s_22_2: i128 = u__id(state, tracer, s_22_1);
        // D s_22_3: cast reint s_22_2 -> i64
        let s_22_3: i64 = (s_22_2 as i64);
        // C s_22_4: const #32s : i
        let s_22_4: i128 = 32;
        // D s_22_5: cast zx s_22_3 -> i
        let s_22_5: i128 = (i128::try_from(s_22_3).unwrap());
        // D s_22_6: cmp-eq s_22_5 s_22_4
        let s_22_6: bool = ((s_22_5) == (s_22_4));
        // D s_22_7: write-var gs#149139 <= s_22_6
        fn_state.gs_149139 = s_22_6;
        // N s_22_8: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#149139:u8
        let s_23_0: bool = fn_state.gs_149139;
        // N s_23_1: branch s_23_0 b27 b24
        if s_23_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var esizeshadow#1251:i64
        let s_24_0: i64 = fn_state.esizeshadow_1251;
        // D s_24_1: cast zx s_24_0 -> i
        let s_24_1: i128 = (i128::try_from(s_24_0).unwrap());
        // D s_24_2: call __id(s_24_1)
        let s_24_2: i128 = u__id(state, tracer, s_24_1);
        // D s_24_3: cast reint s_24_2 -> i64
        let s_24_3: i64 = (s_24_2 as i64);
        // C s_24_4: const #64s : i
        let s_24_4: i128 = 64;
        // D s_24_5: cast zx s_24_3 -> i
        let s_24_5: i128 = (i128::try_from(s_24_3).unwrap());
        // D s_24_6: cmp-eq s_24_5 s_24_4
        let s_24_6: bool = ((s_24_5) == (s_24_4));
        // D s_24_7: write-var gs#149141 <= s_24_6
        fn_state.gs_149141 = s_24_6;
        // N s_24_8: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#149141:u8
        let s_25_0: bool = fn_state.gs_149141;
        // N s_25_1: assert s_25_0
        let s_25_1: () = assert!(s_25_0);
        // C s_25_2: const #2s : i
        let s_25_2: i128 = 2;
        // D s_25_3: read-var e:i64
        let s_25_3: i64 = fn_state.e;
        // D s_25_4: cast zx s_25_3 -> i
        let s_25_4: i128 = (i128::try_from(s_25_3).unwrap());
        // D s_25_5: mul s_25_4 s_25_2
        let s_25_5: i128 = ((s_25_4) * (s_25_2));
        // D s_25_6: read-var esizeshadow#1251:i64
        let s_25_6: i64 = fn_state.esizeshadow_1251;
        // D s_25_7: cast zx s_25_6 -> i
        let s_25_7: i128 = (i128::try_from(s_25_6).unwrap());
        // D s_25_8: cast reint s_25_7 -> i64
        let s_25_8: i64 = (s_25_7 as i64);
        // D s_25_9: cast zx s_25_8 -> i
        let s_25_9: i128 = (i128::try_from(s_25_8).unwrap());
        // D s_25_10: read-var operand2:bv
        let s_25_10: Bits = fn_state.operand2;
        // D s_25_11: call Elem_read(s_25_10, s_25_5, s_25_9)
        let s_25_11: Bits = Elem_read(state, tracer, s_25_10, s_25_5, s_25_9);
        // D s_25_12: call FPNeg(s_25_11)
        let s_25_12: Bits = FPNeg(state, tracer, s_25_11);
        // D s_25_13: write-var element3 <= s_25_12
        fn_state.element3 = s_25_12;
        // N s_25_14: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_26_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // D s_27_1: write-var gs#149141 <= s_27_0
        fn_state.gs_149141 = s_27_0;
        // N s_27_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var gs#149139 <= s_28_0
        fn_state.gs_149139 = s_28_0;
        // N s_28_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var datasizeshadow#1252:i64
        let s_29_0: i64 = fn_state.datasizeshadow_1252;
        // D s_29_1: cast zx s_29_0 -> i
        let s_29_1: i128 = (i128::try_from(s_29_0).unwrap());
        // D s_29_2: cast reint s_29_1 -> i64
        let s_29_2: i64 = (s_29_1 as i64);
        // D s_29_3: read-var d:i64
        let s_29_3: i64 = fn_state.d;
        // D s_29_4: cast zx s_29_3 -> i
        let s_29_4: i128 = (i128::try_from(s_29_3).unwrap());
        // D s_29_5: read-var result:bv
        let s_29_5: Bits = fn_state.result;
        // D s_29_6: call V_set(s_29_4, s_29_2, s_29_5)
        let s_29_6: () = V_set(state, tracer, s_29_4, s_29_2, s_29_5);
        // N s_29_7: return
        return;
    }
}
