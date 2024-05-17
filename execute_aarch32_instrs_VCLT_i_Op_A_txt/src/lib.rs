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
use StandardFPSCRValue::*;
use Elem_set::*;
use u__id::*;
use CheckAdvSIMDEnabled::*;
use Ones::*;
use D_set::*;
use FPZero::*;
use Elem_read::*;
use Zeros::*;
use D_read::*;
use FPCompareGT::*;
use common::*;
pub fn execute_aarch32_instrs_VCLT_i_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d__arg: i64,
    elements: i128,
    esize: i64,
    floating_point: bool,
    m: i64,
    regs: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        e: i64,
        test_passed: bool,
        gs_307561: bool,
        ga_351996: i128,
        ga_351993: u64,
        esizeshadow_7388: i64,
        d: i128,
        gs_307548: i64,
        gs_307554: i64,
        ga_351995: Bits,
        ga_351994: i64,
        gs_307563: bool,
        d__arg: i64,
        elements: i128,
        esize: i64,
        floating_point: bool,
        m: i64,
        regs: i64,
    }
    let fn_state = FunctionState {
        d__arg,
        elements,
        esize,
        floating_point,
        m,
        regs,
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
        // D s_0_3: write-var esizeshadow#7388 <= s_0_2
        fn_state.esizeshadow_7388 = s_0_2;
        // D s_0_4: read-var d__arg:i64
        let s_0_4: i64 = fn_state.d__arg;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: write-var d <= s_0_5
        fn_state.d = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call CheckAdvSIMDEnabled(s_0_7)
        let s_0_8: () = CheckAdvSIMDEnabled(state, tracer, s_0_7);
        // N s_0_9: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0s : i64
        let s_1_0: i64 = 0;
        // C s_1_1: const #1s : i
        let s_1_1: i128 = 1;
        // D s_1_2: read-var regs:i64
        let s_1_2: i64 = fn_state.regs;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: sub s_1_3 s_1_1
        let s_1_4: i128 = ((s_1_3) - (s_1_1));
        // D s_1_5: cast reint s_1_4 -> i64
        let s_1_5: i64 = (s_1_4 as i64);
        // D s_1_6: write-var gs#307548 <= s_1_5
        fn_state.gs_307548 = s_1_5;
        // D s_1_7: write-var r <= s_1_0
        fn_state.r = s_1_0;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var r:i64
        let s_2_0: i64 = fn_state.r;
        // D s_2_1: read-var gs#307548:i64
        let s_2_1: i64 = fn_state.gs_307548;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b20 b3
        if s_2_2 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0s : i64
        let s_3_0: i64 = 0;
        // C s_3_1: const #1s : i
        let s_3_1: i128 = 1;
        // D s_3_2: read-var elements:i
        let s_3_2: i128 = fn_state.elements;
        // D s_3_3: sub s_3_2 s_3_1
        let s_3_3: i128 = ((s_3_2) - (s_3_1));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // D s_3_5: write-var gs#307554 <= s_3_4
        fn_state.gs_307554 = s_3_4;
        // D s_3_6: write-var e <= s_3_0
        fn_state.e = s_3_0;
        // N s_3_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#307554:i64
        let s_4_1: i64 = fn_state.gs_307554;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b19 b5
        if s_4_2 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var floating_point:u8
        let s_5_0: bool = fn_state.floating_point;
        // N s_5_1: branch s_5_0 b11 b6
        if s_5_0 {
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
        // D s_6_0: read-var m:i64
        let s_6_0: i64 = fn_state.m;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: read-var r:i64
        let s_6_2: i64 = fn_state.r;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: add s_6_1 s_6_3
        let s_6_4: i128 = (s_6_1 + s_6_3);
        // D s_6_5: cast reint s_6_4 -> i64
        let s_6_5: i64 = (s_6_4 as i64);
        // D s_6_6: cast zx s_6_5 -> i
        let s_6_6: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_7: call D_read(s_6_6)
        let s_6_7: u64 = D_read(state, tracer, s_6_6);
        // D s_6_8: read-var esizeshadow#7388:i64
        let s_6_8: i64 = fn_state.esizeshadow_7388;
        // D s_6_9: cast zx s_6_8 -> i
        let s_6_9: i128 = (i128::try_from(s_6_8).unwrap());
        // D s_6_10: cast reint s_6_9 -> i64
        let s_6_10: i64 = (s_6_9 as i64);
        // D s_6_11: cast zx s_6_7 -> bv
        let s_6_11: Bits = Bits::new(s_6_7 as u128, 64u16);
        // D s_6_12: read-var e:i64
        let s_6_12: i64 = fn_state.e;
        // D s_6_13: cast zx s_6_12 -> i
        let s_6_13: i128 = (i128::try_from(s_6_12).unwrap());
        // D s_6_14: cast zx s_6_10 -> i
        let s_6_14: i128 = (i128::try_from(s_6_10).unwrap());
        // D s_6_15: call Elem_read(s_6_11, s_6_13, s_6_14)
        let s_6_15: Bits = Elem_read(state, tracer, s_6_11, s_6_13, s_6_14);
        // D s_6_16: cast sx s_6_15 -> i
        let s_6_16: i128 = {
            let sign_bit = s_6_15.length() - 1;
            let mut result = s_6_15.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_6_17: cast reint s_6_16 -> i64
        let s_6_17: i64 = (s_6_16 as i64);
        // C s_6_18: const #0s : i
        let s_6_18: i128 = 0;
        // D s_6_19: cast zx s_6_17 -> i
        let s_6_19: i128 = (i128::try_from(s_6_17).unwrap());
        // D s_6_20: cmp-lt s_6_19 s_6_18
        let s_6_20: bool = ((s_6_19) < (s_6_18));
        // D s_6_21: write-var test_passed <= s_6_20
        fn_state.test_passed = s_6_20;
        // N s_6_22: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var r:i64
        let s_7_0: i64 = fn_state.r;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: read-var d:i
        let s_7_2: i128 = fn_state.d;
        // D s_7_3: add s_7_2 s_7_1
        let s_7_3: i128 = (s_7_2 + s_7_1);
        // D s_7_4: write-var ga#351996 <= s_7_3
        fn_state.ga_351996 = s_7_3;
        // D s_7_5: read-var r:i64
        let s_7_5: i64 = fn_state.r;
        // D s_7_6: cast zx s_7_5 -> i
        let s_7_6: i128 = (i128::try_from(s_7_5).unwrap());
        // D s_7_7: read-var d:i
        let s_7_7: i128 = fn_state.d;
        // D s_7_8: add s_7_7 s_7_6
        let s_7_8: i128 = (s_7_7 + s_7_6);
        // D s_7_9: call D_read(s_7_8)
        let s_7_9: u64 = D_read(state, tracer, s_7_8);
        // D s_7_10: write-var ga#351993 <= s_7_9
        fn_state.ga_351993 = s_7_9;
        // D s_7_11: read-var esizeshadow#7388:i64
        let s_7_11: i64 = fn_state.esizeshadow_7388;
        // D s_7_12: cast zx s_7_11 -> i
        let s_7_12: i128 = (i128::try_from(s_7_11).unwrap());
        // D s_7_13: cast reint s_7_12 -> i64
        let s_7_13: i64 = (s_7_12 as i64);
        // D s_7_14: write-var ga#351994 <= s_7_13
        fn_state.ga_351994 = s_7_13;
        // D s_7_15: read-var test_passed:u8
        let s_7_15: bool = fn_state.test_passed;
        // N s_7_16: branch s_7_15 b10 b8
        if s_7_15 {
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
        // D s_8_0: read-var esizeshadow#7388:i64
        let s_8_0: i64 = fn_state.esizeshadow_7388;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: call Zeros(s_8_1)
        let s_8_2: Bits = Zeros(state, tracer, s_8_1);
        // D s_8_3: write-var ga#351995 <= s_8_2
        fn_state.ga_351995 = s_8_2;
        // N s_8_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var ga#351993:u64
        let s_9_0: u64 = fn_state.ga_351993;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 64u16);
        // D s_9_2: read-var e:i64
        let s_9_2: i64 = fn_state.e;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: read-var ga#351994:i64
        let s_9_4: i64 = fn_state.ga_351994;
        // D s_9_5: cast zx s_9_4 -> i
        let s_9_5: i128 = (i128::try_from(s_9_4).unwrap());
        // D s_9_6: read-var ga#351995:bv
        let s_9_6: Bits = fn_state.ga_351995;
        // D s_9_7: call Elem_set(s_9_1, s_9_3, s_9_5, s_9_6)
        let s_9_7: Bits = Elem_set(state, tracer, s_9_1, s_9_3, s_9_5, s_9_6);
        // D s_9_8: cast reint s_9_7 -> u64
        let s_9_8: u64 = (s_9_7.value() as u64);
        // D s_9_9: read-var ga#351996:i
        let s_9_9: i128 = fn_state.ga_351996;
        // D s_9_10: call D_set(s_9_9, s_9_8)
        let s_9_10: () = D_set(state, tracer, s_9_9, s_9_8);
        // D s_9_11: read-var e:i64
        let s_9_11: i64 = fn_state.e;
        // C s_9_12: const #1s : i64
        let s_9_12: i64 = 1;
        // D s_9_13: add s_9_11 s_9_12
        let s_9_13: i64 = (s_9_11 + s_9_12);
        // D s_9_14: write-var e <= s_9_13
        fn_state.e = s_9_13;
        // N s_9_15: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var esizeshadow#7388:i64
        let s_10_0: i64 = fn_state.esizeshadow_7388;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: call Ones(s_10_1)
        let s_10_2: Bits = Ones(state, tracer, s_10_1);
        // D s_10_3: write-var ga#351995 <= s_10_2
        fn_state.ga_351995 = s_10_2;
        // N s_10_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var esizeshadow#7388:i64
        let s_11_0: i64 = fn_state.esizeshadow_7388;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: call __id(s_11_1)
        let s_11_2: i128 = u__id(state, tracer, s_11_1);
        // D s_11_3: cast reint s_11_2 -> i64
        let s_11_3: i64 = (s_11_2 as i64);
        // C s_11_4: const #16s : i
        let s_11_4: i128 = 16;
        // D s_11_5: cast zx s_11_3 -> i
        let s_11_5: i128 = (i128::try_from(s_11_3).unwrap());
        // D s_11_6: cmp-eq s_11_5 s_11_4
        let s_11_6: bool = ((s_11_5) == (s_11_4));
        // N s_11_7: branch s_11_6 b18 b12
        if s_11_6 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var esizeshadow#7388:i64
        let s_12_0: i64 = fn_state.esizeshadow_7388;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: call __id(s_12_1)
        let s_12_2: i128 = u__id(state, tracer, s_12_1);
        // D s_12_3: cast reint s_12_2 -> i64
        let s_12_3: i64 = (s_12_2 as i64);
        // C s_12_4: const #32s : i
        let s_12_4: i128 = 32;
        // D s_12_5: cast zx s_12_3 -> i
        let s_12_5: i128 = (i128::try_from(s_12_3).unwrap());
        // D s_12_6: cmp-eq s_12_5 s_12_4
        let s_12_6: bool = ((s_12_5) == (s_12_4));
        // D s_12_7: write-var gs#307561 <= s_12_6
        fn_state.gs_307561 = s_12_6;
        // N s_12_8: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#307561:u8
        let s_13_0: bool = fn_state.gs_307561;
        // N s_13_1: branch s_13_0 b17 b14
        if s_13_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var esizeshadow#7388:i64
        let s_14_0: i64 = fn_state.esizeshadow_7388;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: call __id(s_14_1)
        let s_14_2: i128 = u__id(state, tracer, s_14_1);
        // D s_14_3: cast reint s_14_2 -> i64
        let s_14_3: i64 = (s_14_2 as i64);
        // C s_14_4: const #64s : i
        let s_14_4: i128 = 64;
        // D s_14_5: cast zx s_14_3 -> i
        let s_14_5: i128 = (i128::try_from(s_14_3).unwrap());
        // D s_14_6: cmp-eq s_14_5 s_14_4
        let s_14_6: bool = ((s_14_5) == (s_14_4));
        // D s_14_7: write-var gs#307563 <= s_14_6
        fn_state.gs_307563 = s_14_6;
        // N s_14_8: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#307563:u8
        let s_15_0: bool = fn_state.gs_307563;
        // N s_15_1: assert s_15_0
        let s_15_1: () = assert!(s_15_0);
        // D s_15_2: read-var esizeshadow#7388:i64
        let s_15_2: i64 = fn_state.esizeshadow_7388;
        // D s_15_3: cast zx s_15_2 -> i
        let s_15_3: i128 = (i128::try_from(s_15_2).unwrap());
        // D s_15_4: cast reint s_15_3 -> i64
        let s_15_4: i64 = (s_15_3 as i64);
        // C s_15_5: const #0u : u8
        let s_15_5: bool = false;
        // D s_15_6: call FPZero(s_15_5, s_15_4)
        let s_15_6: Bits = FPZero(state, tracer, s_15_5, s_15_4);
        // D s_15_7: read-var m:i64
        let s_15_7: i64 = fn_state.m;
        // D s_15_8: cast zx s_15_7 -> i
        let s_15_8: i128 = (i128::try_from(s_15_7).unwrap());
        // D s_15_9: read-var r:i64
        let s_15_9: i64 = fn_state.r;
        // D s_15_10: cast zx s_15_9 -> i
        let s_15_10: i128 = (i128::try_from(s_15_9).unwrap());
        // D s_15_11: add s_15_8 s_15_10
        let s_15_11: i128 = (s_15_8 + s_15_10);
        // D s_15_12: cast reint s_15_11 -> i64
        let s_15_12: i64 = (s_15_11 as i64);
        // D s_15_13: cast zx s_15_12 -> i
        let s_15_13: i128 = (i128::try_from(s_15_12).unwrap());
        // D s_15_14: call D_read(s_15_13)
        let s_15_14: u64 = D_read(state, tracer, s_15_13);
        // D s_15_15: read-var esizeshadow#7388:i64
        let s_15_15: i64 = fn_state.esizeshadow_7388;
        // D s_15_16: cast zx s_15_15 -> i
        let s_15_16: i128 = (i128::try_from(s_15_15).unwrap());
        // D s_15_17: cast reint s_15_16 -> i64
        let s_15_17: i64 = (s_15_16 as i64);
        // D s_15_18: cast zx s_15_14 -> bv
        let s_15_18: Bits = Bits::new(s_15_14 as u128, 64u16);
        // D s_15_19: read-var e:i64
        let s_15_19: i64 = fn_state.e;
        // D s_15_20: cast zx s_15_19 -> i
        let s_15_20: i128 = (i128::try_from(s_15_19).unwrap());
        // D s_15_21: cast zx s_15_17 -> i
        let s_15_21: i128 = (i128::try_from(s_15_17).unwrap());
        // D s_15_22: call Elem_read(s_15_18, s_15_20, s_15_21)
        let s_15_22: Bits = Elem_read(state, tracer, s_15_18, s_15_20, s_15_21);
        // C s_15_23: const #() : ()
        let s_15_23: () = ();
        // S s_15_24: call StandardFPSCRValue(s_15_23)
        let s_15_24: ProductType5c790c8ef59cc8b2 = StandardFPSCRValue(
            state,
            tracer,
            s_15_23,
        );
        // D s_15_25: call FPCompareGT(s_15_6, s_15_22, s_15_24)
        let s_15_25: bool = FPCompareGT(state, tracer, s_15_6, s_15_22, s_15_24);
        // D s_15_26: write-var test_passed <= s_15_25
        fn_state.test_passed = s_15_25;
        // N s_15_27: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var gs#307563 <= s_17_0
        fn_state.gs_307563 = s_17_0;
        // N s_17_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#307561 <= s_18_0
        fn_state.gs_307561 = s_18_0;
        // N s_18_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var r:i64
        let s_19_0: i64 = fn_state.r;
        // C s_19_1: const #1s : i64
        let s_19_1: i64 = 1;
        // D s_19_2: add s_19_0 s_19_1
        let s_19_2: i64 = (s_19_0 + s_19_1);
        // D s_19_3: write-var r <= s_19_2
        fn_state.r = s_19_2;
        // N s_19_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: return
        return;
    }
}
