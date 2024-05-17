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
use FPCompareGE::*;
use D_set::*;
use Elem_read::*;
use Zeros::*;
use D_read::*;
use common::*;
pub fn execute_aarch32_instrs_VCGE_r_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d__arg: i64,
    elements: i128,
    esize: i64,
    m: i64,
    n: i64,
    regs: i64,
    vtype: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        esizeshadow_7373: i64,
        r: i64,
        e: i64,
        op2: Bits,
        gs_307023: bool,
        test_passed: bool,
        ga_351547: Bits,
        ga_351545: u64,
        gs_307011: i64,
        d: i128,
        op1: Bits,
        gs_307021: bool,
        ga_351548: i128,
        gs_307005: i64,
        ga_351546: i64,
        d__arg: i64,
        elements: i128,
        esize: i64,
        m: i64,
        n: i64,
        regs: i64,
        vtype: u32,
    }
    let fn_state = FunctionState {
        d__arg,
        elements,
        esize,
        m,
        n,
        regs,
        vtype,
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
        // D s_0_3: write-var esizeshadow#7373 <= s_0_2
        fn_state.esizeshadow_7373 = s_0_2;
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
        // D s_1_6: write-var gs#307005 <= s_1_5
        fn_state.gs_307005 = s_1_5;
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
        // D s_2_1: read-var gs#307005:i64
        let s_2_1: i64 = fn_state.gs_307005;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b24 b3
        if s_2_2 {
            return block_24(state, tracer, fn_state);
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
        // D s_3_5: write-var gs#307011 <= s_3_4
        fn_state.gs_307011 = s_3_4;
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
        // D s_4_1: read-var gs#307011:i64
        let s_4_1: i64 = fn_state.gs_307011;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b23 b5
        if s_4_2 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var n:i64
        let s_5_0: i64 = fn_state.n;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: read-var r:i64
        let s_5_2: i64 = fn_state.r;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: add s_5_1 s_5_3
        let s_5_4: i128 = (s_5_1 + s_5_3);
        // D s_5_5: cast reint s_5_4 -> i64
        let s_5_5: i64 = (s_5_4 as i64);
        // D s_5_6: cast zx s_5_5 -> i
        let s_5_6: i128 = (i128::try_from(s_5_5).unwrap());
        // D s_5_7: call D_read(s_5_6)
        let s_5_7: u64 = D_read(state, tracer, s_5_6);
        // D s_5_8: read-var esizeshadow#7373:i64
        let s_5_8: i64 = fn_state.esizeshadow_7373;
        // D s_5_9: cast zx s_5_8 -> i
        let s_5_9: i128 = (i128::try_from(s_5_8).unwrap());
        // D s_5_10: cast reint s_5_9 -> i64
        let s_5_10: i64 = (s_5_9 as i64);
        // D s_5_11: cast zx s_5_7 -> bv
        let s_5_11: Bits = Bits::new(s_5_7 as u128, 64u16);
        // D s_5_12: read-var e:i64
        let s_5_12: i64 = fn_state.e;
        // D s_5_13: cast zx s_5_12 -> i
        let s_5_13: i128 = (i128::try_from(s_5_12).unwrap());
        // D s_5_14: cast zx s_5_10 -> i
        let s_5_14: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_15: call Elem_read(s_5_11, s_5_13, s_5_14)
        let s_5_15: Bits = Elem_read(state, tracer, s_5_11, s_5_13, s_5_14);
        // D s_5_16: write-var op1 <= s_5_15
        fn_state.op1 = s_5_15;
        // D s_5_17: read-var m:i64
        let s_5_17: i64 = fn_state.m;
        // D s_5_18: cast zx s_5_17 -> i
        let s_5_18: i128 = (i128::try_from(s_5_17).unwrap());
        // D s_5_19: read-var r:i64
        let s_5_19: i64 = fn_state.r;
        // D s_5_20: cast zx s_5_19 -> i
        let s_5_20: i128 = (i128::try_from(s_5_19).unwrap());
        // D s_5_21: add s_5_18 s_5_20
        let s_5_21: i128 = (s_5_18 + s_5_20);
        // D s_5_22: cast reint s_5_21 -> i64
        let s_5_22: i64 = (s_5_21 as i64);
        // D s_5_23: cast zx s_5_22 -> i
        let s_5_23: i128 = (i128::try_from(s_5_22).unwrap());
        // D s_5_24: call D_read(s_5_23)
        let s_5_24: u64 = D_read(state, tracer, s_5_23);
        // D s_5_25: read-var esizeshadow#7373:i64
        let s_5_25: i64 = fn_state.esizeshadow_7373;
        // D s_5_26: cast zx s_5_25 -> i
        let s_5_26: i128 = (i128::try_from(s_5_25).unwrap());
        // D s_5_27: cast reint s_5_26 -> i64
        let s_5_27: i64 = (s_5_26 as i64);
        // D s_5_28: cast zx s_5_24 -> bv
        let s_5_28: Bits = Bits::new(s_5_24 as u128, 64u16);
        // D s_5_29: read-var e:i64
        let s_5_29: i64 = fn_state.e;
        // D s_5_30: cast zx s_5_29 -> i
        let s_5_30: i128 = (i128::try_from(s_5_29).unwrap());
        // D s_5_31: cast zx s_5_27 -> i
        let s_5_31: i128 = (i128::try_from(s_5_27).unwrap());
        // D s_5_32: call Elem_read(s_5_28, s_5_30, s_5_31)
        let s_5_32: Bits = Elem_read(state, tracer, s_5_28, s_5_30, s_5_31);
        // D s_5_33: write-var op2 <= s_5_32
        fn_state.op2 = s_5_32;
        // C s_5_34: const #0u : u32
        let s_5_34: u32 = 0;
        // D s_5_35: read-var vtype:u32
        let s_5_35: u32 = fn_state.vtype;
        // D s_5_36: cmp-eq s_5_34 s_5_35
        let s_5_36: bool = ((s_5_34) == (s_5_35));
        // D s_5_37: not s_5_36
        let s_5_37: bool = !s_5_36;
        // N s_5_38: branch s_5_37 b11 b6
        if s_5_37 {
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
        // D s_6_0: read-var op1:bv
        let s_6_0: Bits = fn_state.op1;
        // D s_6_1: cast sx s_6_0 -> i
        let s_6_1: i128 = {
            let sign_bit = s_6_0.length() - 1;
            let mut result = s_6_0.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_6_2: cast reint s_6_1 -> i64
        let s_6_2: i64 = (s_6_1 as i64);
        // D s_6_3: read-var op2:bv
        let s_6_3: Bits = fn_state.op2;
        // D s_6_4: cast sx s_6_3 -> i
        let s_6_4: i128 = {
            let sign_bit = s_6_3.length() - 1;
            let mut result = s_6_3.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_6_5: cast reint s_6_4 -> i64
        let s_6_5: i64 = (s_6_4 as i64);
        // D s_6_6: cast zx s_6_2 -> i
        let s_6_6: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_7: cast zx s_6_5 -> i
        let s_6_7: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_8: cmp-ge s_6_6 s_6_7
        let s_6_8: bool = ((s_6_6) >= (s_6_7));
        // D s_6_9: write-var test_passed <= s_6_8
        fn_state.test_passed = s_6_8;
        // N s_6_10: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var test_passed:u8
        let s_7_0: bool = fn_state.test_passed;
        // D s_7_1: read-var r:i64
        let s_7_1: i64 = fn_state.r;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: read-var d:i
        let s_7_3: i128 = fn_state.d;
        // D s_7_4: add s_7_3 s_7_2
        let s_7_4: i128 = (s_7_3 + s_7_2);
        // D s_7_5: write-var ga#351548 <= s_7_4
        fn_state.ga_351548 = s_7_4;
        // D s_7_6: read-var r:i64
        let s_7_6: i64 = fn_state.r;
        // D s_7_7: cast zx s_7_6 -> i
        let s_7_7: i128 = (i128::try_from(s_7_6).unwrap());
        // D s_7_8: read-var d:i
        let s_7_8: i128 = fn_state.d;
        // D s_7_9: add s_7_8 s_7_7
        let s_7_9: i128 = (s_7_8 + s_7_7);
        // D s_7_10: call D_read(s_7_9)
        let s_7_10: u64 = D_read(state, tracer, s_7_9);
        // D s_7_11: write-var ga#351545 <= s_7_10
        fn_state.ga_351545 = s_7_10;
        // D s_7_12: read-var esizeshadow#7373:i64
        let s_7_12: i64 = fn_state.esizeshadow_7373;
        // D s_7_13: cast zx s_7_12 -> i
        let s_7_13: i128 = (i128::try_from(s_7_12).unwrap());
        // D s_7_14: cast reint s_7_13 -> i64
        let s_7_14: i64 = (s_7_13 as i64);
        // D s_7_15: write-var ga#351546 <= s_7_14
        fn_state.ga_351546 = s_7_14;
        // N s_7_16: branch s_7_0 b10 b8
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
        // D s_8_0: read-var esizeshadow#7373:i64
        let s_8_0: i64 = fn_state.esizeshadow_7373;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: call Zeros(s_8_1)
        let s_8_2: Bits = Zeros(state, tracer, s_8_1);
        // D s_8_3: write-var ga#351547 <= s_8_2
        fn_state.ga_351547 = s_8_2;
        // N s_8_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var ga#351545:u64
        let s_9_0: u64 = fn_state.ga_351545;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 64u16);
        // D s_9_2: read-var e:i64
        let s_9_2: i64 = fn_state.e;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: read-var ga#351546:i64
        let s_9_4: i64 = fn_state.ga_351546;
        // D s_9_5: cast zx s_9_4 -> i
        let s_9_5: i128 = (i128::try_from(s_9_4).unwrap());
        // D s_9_6: read-var ga#351547:bv
        let s_9_6: Bits = fn_state.ga_351547;
        // D s_9_7: call Elem_set(s_9_1, s_9_3, s_9_5, s_9_6)
        let s_9_7: Bits = Elem_set(state, tracer, s_9_1, s_9_3, s_9_5, s_9_6);
        // D s_9_8: cast reint s_9_7 -> u64
        let s_9_8: u64 = (s_9_7.value() as u64);
        // D s_9_9: read-var ga#351548:i
        let s_9_9: i128 = fn_state.ga_351548;
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
        // D s_10_0: read-var esizeshadow#7373:i64
        let s_10_0: i64 = fn_state.esizeshadow_7373;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: call Ones(s_10_1)
        let s_10_2: Bits = Ones(state, tracer, s_10_1);
        // D s_10_3: write-var ga#351547 <= s_10_2
        fn_state.ga_351547 = s_10_2;
        // N s_10_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #1u : u32
        let s_11_0: u32 = 1;
        // D s_11_1: read-var vtype:u32
        let s_11_1: u32 = fn_state.vtype;
        // D s_11_2: cmp-eq s_11_0 s_11_1
        let s_11_2: bool = ((s_11_0) == (s_11_1));
        // D s_11_3: not s_11_2
        let s_11_3: bool = !s_11_2;
        // N s_11_4: branch s_11_3 b13 b12
        if s_11_3 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var op1:bv
        let s_12_0: Bits = fn_state.op1;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (s_12_0.value() as i128);
        // D s_12_2: read-var op2:bv
        let s_12_2: Bits = fn_state.op2;
        // D s_12_3: cast zx s_12_2 -> i
        let s_12_3: i128 = (s_12_2.value() as i128);
        // D s_12_4: cmp-ge s_12_1 s_12_3
        let s_12_4: bool = ((s_12_1) >= (s_12_3));
        // D s_12_5: write-var test_passed <= s_12_4
        fn_state.test_passed = s_12_4;
        // N s_12_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #2u : u32
        let s_13_0: u32 = 2;
        // D s_13_1: read-var vtype:u32
        let s_13_1: u32 = fn_state.vtype;
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // D s_13_3: not s_13_2
        let s_13_3: bool = !s_13_2;
        // N s_13_4: branch s_13_3 b22 b14
        if s_13_3 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var esizeshadow#7373:i64
        let s_14_0: i64 = fn_state.esizeshadow_7373;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: call __id(s_14_1)
        let s_14_2: i128 = u__id(state, tracer, s_14_1);
        // D s_14_3: cast reint s_14_2 -> i64
        let s_14_3: i64 = (s_14_2 as i64);
        // C s_14_4: const #16s : i
        let s_14_4: i128 = 16;
        // D s_14_5: cast zx s_14_3 -> i
        let s_14_5: i128 = (i128::try_from(s_14_3).unwrap());
        // D s_14_6: cmp-eq s_14_5 s_14_4
        let s_14_6: bool = ((s_14_5) == (s_14_4));
        // N s_14_7: branch s_14_6 b21 b15
        if s_14_6 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var esizeshadow#7373:i64
        let s_15_0: i64 = fn_state.esizeshadow_7373;
        // D s_15_1: cast zx s_15_0 -> i
        let s_15_1: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_2: call __id(s_15_1)
        let s_15_2: i128 = u__id(state, tracer, s_15_1);
        // D s_15_3: cast reint s_15_2 -> i64
        let s_15_3: i64 = (s_15_2 as i64);
        // C s_15_4: const #32s : i
        let s_15_4: i128 = 32;
        // D s_15_5: cast zx s_15_3 -> i
        let s_15_5: i128 = (i128::try_from(s_15_3).unwrap());
        // D s_15_6: cmp-eq s_15_5 s_15_4
        let s_15_6: bool = ((s_15_5) == (s_15_4));
        // D s_15_7: write-var gs#307021 <= s_15_6
        fn_state.gs_307021 = s_15_6;
        // N s_15_8: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#307021:u8
        let s_16_0: bool = fn_state.gs_307021;
        // N s_16_1: branch s_16_0 b20 b17
        if s_16_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var esizeshadow#7373:i64
        let s_17_0: i64 = fn_state.esizeshadow_7373;
        // D s_17_1: cast zx s_17_0 -> i
        let s_17_1: i128 = (i128::try_from(s_17_0).unwrap());
        // D s_17_2: call __id(s_17_1)
        let s_17_2: i128 = u__id(state, tracer, s_17_1);
        // D s_17_3: cast reint s_17_2 -> i64
        let s_17_3: i64 = (s_17_2 as i64);
        // C s_17_4: const #64s : i
        let s_17_4: i128 = 64;
        // D s_17_5: cast zx s_17_3 -> i
        let s_17_5: i128 = (i128::try_from(s_17_3).unwrap());
        // D s_17_6: cmp-eq s_17_5 s_17_4
        let s_17_6: bool = ((s_17_5) == (s_17_4));
        // D s_17_7: write-var gs#307023 <= s_17_6
        fn_state.gs_307023 = s_17_6;
        // N s_17_8: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#307023:u8
        let s_18_0: bool = fn_state.gs_307023;
        // N s_18_1: assert s_18_0
        let s_18_1: () = assert!(s_18_0);
        // C s_18_2: const #() : ()
        let s_18_2: () = ();
        // S s_18_3: call StandardFPSCRValue(s_18_2)
        let s_18_3: ProductType5c790c8ef59cc8b2 = StandardFPSCRValue(
            state,
            tracer,
            s_18_2,
        );
        // D s_18_4: read-var op1:bv
        let s_18_4: Bits = fn_state.op1;
        // D s_18_5: read-var op2:bv
        let s_18_5: Bits = fn_state.op2;
        // D s_18_6: call FPCompareGE(s_18_4, s_18_5, s_18_3)
        let s_18_6: bool = FPCompareGE(state, tracer, s_18_4, s_18_5, s_18_3);
        // D s_18_7: write-var test_passed <= s_18_6
        fn_state.test_passed = s_18_6;
        // N s_18_8: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#307023 <= s_20_0
        fn_state.gs_307023 = s_20_0;
        // N s_20_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#307021 <= s_21_0
        fn_state.gs_307021 = s_21_0;
        // N s_21_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var r:i64
        let s_23_0: i64 = fn_state.r;
        // C s_23_1: const #1s : i64
        let s_23_1: i64 = 1;
        // D s_23_2: add s_23_0 s_23_1
        let s_23_2: i64 = (s_23_0 + s_23_1);
        // D s_23_3: write-var r <= s_23_2
        fn_state.r = s_23_2;
        // N s_23_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_24_0: return
        return;
    }
}
