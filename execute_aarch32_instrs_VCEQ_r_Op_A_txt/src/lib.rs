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
use FPCompareEQ::*;
use Elem_set::*;
use u__id::*;
use CheckAdvSIMDEnabled::*;
use Ones::*;
use D_set::*;
use Elem_read::*;
use Zeros::*;
use D_read::*;
use common::*;
pub fn execute_aarch32_instrs_VCEQ_r_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d__arg: i64,
    elements: i128,
    esize: i64,
    int_operation: bool,
    m: i64,
    n: i64,
    regs: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        ga_351350: u64,
        ga_351351: i64,
        gs_306775: i64,
        e: i64,
        op2: Bits,
        test_passed: bool,
        d: i128,
        op1: Bits,
        ga_351352: Bits,
        gs_306783: bool,
        gs_306785: bool,
        esizeshadow_7367: i64,
        ga_351353: i128,
        gs_306769: i64,
        d__arg: i64,
        elements: i128,
        esize: i64,
        int_operation: bool,
        m: i64,
        n: i64,
        regs: i64,
    }
    let fn_state = FunctionState {
        d__arg,
        elements,
        esize,
        int_operation,
        m,
        n,
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
        // D s_0_3: write-var esizeshadow#7367 <= s_0_2
        fn_state.esizeshadow_7367 = s_0_2;
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
        // D s_1_6: write-var gs#306769 <= s_1_5
        fn_state.gs_306769 = s_1_5;
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
        // D s_2_1: read-var gs#306769:i64
        let s_2_1: i64 = fn_state.gs_306769;
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
        // D s_3_5: write-var gs#306775 <= s_3_4
        fn_state.gs_306775 = s_3_4;
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
        // D s_4_1: read-var gs#306775:i64
        let s_4_1: i64 = fn_state.gs_306775;
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
        // D s_5_8: read-var esizeshadow#7367:i64
        let s_5_8: i64 = fn_state.esizeshadow_7367;
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
        // D s_5_25: read-var esizeshadow#7367:i64
        let s_5_25: i64 = fn_state.esizeshadow_7367;
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
        // D s_5_34: read-var int_operation:u8
        let s_5_34: bool = fn_state.int_operation;
        // N s_5_35: branch s_5_34 b18 b6
        if s_5_34 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var esizeshadow#7367:i64
        let s_6_0: i64 = fn_state.esizeshadow_7367;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: call __id(s_6_1)
        let s_6_2: i128 = u__id(state, tracer, s_6_1);
        // D s_6_3: cast reint s_6_2 -> i64
        let s_6_3: i64 = (s_6_2 as i64);
        // C s_6_4: const #16s : i
        let s_6_4: i128 = 16;
        // D s_6_5: cast zx s_6_3 -> i
        let s_6_5: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_6: cmp-eq s_6_5 s_6_4
        let s_6_6: bool = ((s_6_5) == (s_6_4));
        // N s_6_7: branch s_6_6 b17 b7
        if s_6_6 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var esizeshadow#7367:i64
        let s_7_0: i64 = fn_state.esizeshadow_7367;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: call __id(s_7_1)
        let s_7_2: i128 = u__id(state, tracer, s_7_1);
        // D s_7_3: cast reint s_7_2 -> i64
        let s_7_3: i64 = (s_7_2 as i64);
        // C s_7_4: const #32s : i
        let s_7_4: i128 = 32;
        // D s_7_5: cast zx s_7_3 -> i
        let s_7_5: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_6: cmp-eq s_7_5 s_7_4
        let s_7_6: bool = ((s_7_5) == (s_7_4));
        // D s_7_7: write-var gs#306783 <= s_7_6
        fn_state.gs_306783 = s_7_6;
        // N s_7_8: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#306783:u8
        let s_8_0: bool = fn_state.gs_306783;
        // N s_8_1: branch s_8_0 b16 b9
        if s_8_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var esizeshadow#7367:i64
        let s_9_0: i64 = fn_state.esizeshadow_7367;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: call __id(s_9_1)
        let s_9_2: i128 = u__id(state, tracer, s_9_1);
        // D s_9_3: cast reint s_9_2 -> i64
        let s_9_3: i64 = (s_9_2 as i64);
        // C s_9_4: const #64s : i
        let s_9_4: i128 = 64;
        // D s_9_5: cast zx s_9_3 -> i
        let s_9_5: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_6: cmp-eq s_9_5 s_9_4
        let s_9_6: bool = ((s_9_5) == (s_9_4));
        // D s_9_7: write-var gs#306785 <= s_9_6
        fn_state.gs_306785 = s_9_6;
        // N s_9_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#306785:u8
        let s_10_0: bool = fn_state.gs_306785;
        // N s_10_1: assert s_10_0
        let s_10_1: () = assert!(s_10_0);
        // C s_10_2: const #() : ()
        let s_10_2: () = ();
        // S s_10_3: call StandardFPSCRValue(s_10_2)
        let s_10_3: ProductType5c790c8ef59cc8b2 = StandardFPSCRValue(
            state,
            tracer,
            s_10_2,
        );
        // D s_10_4: read-var op1:bv
        let s_10_4: Bits = fn_state.op1;
        // D s_10_5: read-var op2:bv
        let s_10_5: Bits = fn_state.op2;
        // D s_10_6: call FPCompareEQ(s_10_4, s_10_5, s_10_3)
        let s_10_6: bool = FPCompareEQ(state, tracer, s_10_4, s_10_5, s_10_3);
        // D s_10_7: write-var test_passed <= s_10_6
        fn_state.test_passed = s_10_6;
        // N s_10_8: jump b11
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
        // D s_12_0: read-var r:i64
        let s_12_0: i64 = fn_state.r;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: read-var d:i
        let s_12_2: i128 = fn_state.d;
        // D s_12_3: add s_12_2 s_12_1
        let s_12_3: i128 = (s_12_2 + s_12_1);
        // D s_12_4: write-var ga#351353 <= s_12_3
        fn_state.ga_351353 = s_12_3;
        // D s_12_5: read-var r:i64
        let s_12_5: i64 = fn_state.r;
        // D s_12_6: cast zx s_12_5 -> i
        let s_12_6: i128 = (i128::try_from(s_12_5).unwrap());
        // D s_12_7: read-var d:i
        let s_12_7: i128 = fn_state.d;
        // D s_12_8: add s_12_7 s_12_6
        let s_12_8: i128 = (s_12_7 + s_12_6);
        // D s_12_9: call D_read(s_12_8)
        let s_12_9: u64 = D_read(state, tracer, s_12_8);
        // D s_12_10: write-var ga#351350 <= s_12_9
        fn_state.ga_351350 = s_12_9;
        // D s_12_11: read-var esizeshadow#7367:i64
        let s_12_11: i64 = fn_state.esizeshadow_7367;
        // D s_12_12: cast zx s_12_11 -> i
        let s_12_12: i128 = (i128::try_from(s_12_11).unwrap());
        // D s_12_13: cast reint s_12_12 -> i64
        let s_12_13: i64 = (s_12_12 as i64);
        // D s_12_14: write-var ga#351351 <= s_12_13
        fn_state.ga_351351 = s_12_13;
        // D s_12_15: read-var test_passed:u8
        let s_12_15: bool = fn_state.test_passed;
        // N s_12_16: branch s_12_15 b15 b13
        if s_12_15 {
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
        // D s_13_0: read-var esizeshadow#7367:i64
        let s_13_0: i64 = fn_state.esizeshadow_7367;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: call Zeros(s_13_1)
        let s_13_2: Bits = Zeros(state, tracer, s_13_1);
        // D s_13_3: write-var ga#351352 <= s_13_2
        fn_state.ga_351352 = s_13_2;
        // N s_13_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var ga#351350:u64
        let s_14_0: u64 = fn_state.ga_351350;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 64u16);
        // D s_14_2: read-var e:i64
        let s_14_2: i64 = fn_state.e;
        // D s_14_3: cast zx s_14_2 -> i
        let s_14_3: i128 = (i128::try_from(s_14_2).unwrap());
        // D s_14_4: read-var ga#351351:i64
        let s_14_4: i64 = fn_state.ga_351351;
        // D s_14_5: cast zx s_14_4 -> i
        let s_14_5: i128 = (i128::try_from(s_14_4).unwrap());
        // D s_14_6: read-var ga#351352:bv
        let s_14_6: Bits = fn_state.ga_351352;
        // D s_14_7: call Elem_set(s_14_1, s_14_3, s_14_5, s_14_6)
        let s_14_7: Bits = Elem_set(state, tracer, s_14_1, s_14_3, s_14_5, s_14_6);
        // D s_14_8: cast reint s_14_7 -> u64
        let s_14_8: u64 = (s_14_7.value() as u64);
        // D s_14_9: read-var ga#351353:i
        let s_14_9: i128 = fn_state.ga_351353;
        // D s_14_10: call D_set(s_14_9, s_14_8)
        let s_14_10: () = D_set(state, tracer, s_14_9, s_14_8);
        // D s_14_11: read-var e:i64
        let s_14_11: i64 = fn_state.e;
        // C s_14_12: const #1s : i64
        let s_14_12: i64 = 1;
        // D s_14_13: add s_14_11 s_14_12
        let s_14_13: i64 = (s_14_11 + s_14_12);
        // D s_14_14: write-var e <= s_14_13
        fn_state.e = s_14_13;
        // N s_14_15: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var esizeshadow#7367:i64
        let s_15_0: i64 = fn_state.esizeshadow_7367;
        // D s_15_1: cast zx s_15_0 -> i
        let s_15_1: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_2: call Ones(s_15_1)
        let s_15_2: Bits = Ones(state, tracer, s_15_1);
        // D s_15_3: write-var ga#351352 <= s_15_2
        fn_state.ga_351352 = s_15_2;
        // N s_15_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#306785 <= s_16_0
        fn_state.gs_306785 = s_16_0;
        // N s_16_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var gs#306783 <= s_17_0
        fn_state.gs_306783 = s_17_0;
        // N s_17_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var op1:bv
        let s_18_0: Bits = fn_state.op1;
        // D s_18_1: read-var op2:bv
        let s_18_1: Bits = fn_state.op2;
        // D s_18_2: cmp-eq s_18_0 s_18_1
        let s_18_2: bool = ((s_18_0) == (s_18_1));
        // D s_18_3: write-var test_passed <= s_18_2
        fn_state.test_passed = s_18_2;
        // N s_18_4: jump b12
        return block_12(state, tracer, fn_state);
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
