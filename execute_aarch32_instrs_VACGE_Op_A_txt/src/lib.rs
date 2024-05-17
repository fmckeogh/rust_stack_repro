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
use FPAbs::*;
use Elem_set::*;
use CheckAdvSIMDEnabled::*;
use Ones::*;
use FPCompareGE::*;
use D_set::*;
use Elem_read::*;
use Zeros::*;
use D_read::*;
use FPCompareGT::*;
use common::*;
pub fn execute_aarch32_instrs_VACGE_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d__arg: i64,
    elements: i64,
    esize: i64,
    m: i64,
    n: i64,
    or_equal: bool,
    regs: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        e: i64,
        ga_350646: i128,
        op2: Bits,
        test_passed: bool,
        gs_306004: i64,
        d: i128,
        ga_350644: i64,
        esizeshadow_7344: i64,
        op1: Bits,
        gs_305998: i64,
        ga_350643: u64,
        ga_350645: Bits,
        d__arg: i64,
        elements: i64,
        esize: i64,
        m: i64,
        n: i64,
        or_equal: bool,
        regs: i64,
    }
    let fn_state = FunctionState {
        d__arg,
        elements,
        esize,
        m,
        n,
        or_equal,
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
        // D s_0_3: write-var esizeshadow#7344 <= s_0_2
        fn_state.esizeshadow_7344 = s_0_2;
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
        // D s_1_6: write-var gs#305998 <= s_1_5
        fn_state.gs_305998 = s_1_5;
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
        // D s_2_1: read-var gs#305998:i64
        let s_2_1: i64 = fn_state.gs_305998;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b17 b3
        if s_2_2 {
            return block_17(state, tracer, fn_state);
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
        // D s_3_2: read-var elements:i64
        let s_3_2: i64 = fn_state.elements;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: sub s_3_3 s_3_1
        let s_3_4: i128 = ((s_3_3) - (s_3_1));
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: write-var gs#306004 <= s_3_5
        fn_state.gs_306004 = s_3_5;
        // D s_3_7: write-var e <= s_3_0
        fn_state.e = s_3_0;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#306004:i64
        let s_4_1: i64 = fn_state.gs_306004;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b16 b5
        if s_4_2 {
            return block_16(state, tracer, fn_state);
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
        // D s_5_8: read-var esizeshadow#7344:i64
        let s_5_8: i64 = fn_state.esizeshadow_7344;
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
        // D s_5_16: call FPAbs(s_5_15)
        let s_5_16: Bits = FPAbs(state, tracer, s_5_15);
        // D s_5_17: write-var op1 <= s_5_16
        fn_state.op1 = s_5_16;
        // N s_5_18: jump b6
        return block_6(state, tracer, fn_state);
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
        // D s_6_8: read-var esizeshadow#7344:i64
        let s_6_8: i64 = fn_state.esizeshadow_7344;
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
        // D s_6_16: call FPAbs(s_6_15)
        let s_6_16: Bits = FPAbs(state, tracer, s_6_15);
        // D s_6_17: write-var op2 <= s_6_16
        fn_state.op2 = s_6_16;
        // N s_6_18: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var or_equal:u8
        let s_7_0: bool = fn_state.or_equal;
        // N s_7_1: branch s_7_0 b14 b8
        if s_7_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call StandardFPSCRValue(s_8_0)
        let s_8_1: ProductType5c790c8ef59cc8b2 = StandardFPSCRValue(
            state,
            tracer,
            s_8_0,
        );
        // D s_8_2: read-var op1:bv
        let s_8_2: Bits = fn_state.op1;
        // D s_8_3: read-var op2:bv
        let s_8_3: Bits = fn_state.op2;
        // D s_8_4: call FPCompareGT(s_8_2, s_8_3, s_8_1)
        let s_8_4: bool = FPCompareGT(state, tracer, s_8_2, s_8_3, s_8_1);
        // D s_8_5: write-var test_passed <= s_8_4
        fn_state.test_passed = s_8_4;
        // N s_8_6: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var r:i64
        let s_10_0: i64 = fn_state.r;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: read-var d:i
        let s_10_2: i128 = fn_state.d;
        // D s_10_3: add s_10_2 s_10_1
        let s_10_3: i128 = (s_10_2 + s_10_1);
        // D s_10_4: write-var ga#350646 <= s_10_3
        fn_state.ga_350646 = s_10_3;
        // D s_10_5: read-var r:i64
        let s_10_5: i64 = fn_state.r;
        // D s_10_6: cast zx s_10_5 -> i
        let s_10_6: i128 = (i128::try_from(s_10_5).unwrap());
        // D s_10_7: read-var d:i
        let s_10_7: i128 = fn_state.d;
        // D s_10_8: add s_10_7 s_10_6
        let s_10_8: i128 = (s_10_7 + s_10_6);
        // D s_10_9: call D_read(s_10_8)
        let s_10_9: u64 = D_read(state, tracer, s_10_8);
        // D s_10_10: write-var ga#350643 <= s_10_9
        fn_state.ga_350643 = s_10_9;
        // D s_10_11: read-var esizeshadow#7344:i64
        let s_10_11: i64 = fn_state.esizeshadow_7344;
        // D s_10_12: cast zx s_10_11 -> i
        let s_10_12: i128 = (i128::try_from(s_10_11).unwrap());
        // D s_10_13: cast reint s_10_12 -> i64
        let s_10_13: i64 = (s_10_12 as i64);
        // D s_10_14: write-var ga#350644 <= s_10_13
        fn_state.ga_350644 = s_10_13;
        // D s_10_15: read-var test_passed:u8
        let s_10_15: bool = fn_state.test_passed;
        // N s_10_16: branch s_10_15 b13 b11
        if s_10_15 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var esizeshadow#7344:i64
        let s_11_0: i64 = fn_state.esizeshadow_7344;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: call Zeros(s_11_1)
        let s_11_2: Bits = Zeros(state, tracer, s_11_1);
        // D s_11_3: write-var ga#350645 <= s_11_2
        fn_state.ga_350645 = s_11_2;
        // N s_11_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var ga#350643:u64
        let s_12_0: u64 = fn_state.ga_350643;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 64u16);
        // D s_12_2: read-var e:i64
        let s_12_2: i64 = fn_state.e;
        // D s_12_3: cast zx s_12_2 -> i
        let s_12_3: i128 = (i128::try_from(s_12_2).unwrap());
        // D s_12_4: read-var ga#350644:i64
        let s_12_4: i64 = fn_state.ga_350644;
        // D s_12_5: cast zx s_12_4 -> i
        let s_12_5: i128 = (i128::try_from(s_12_4).unwrap());
        // D s_12_6: read-var ga#350645:bv
        let s_12_6: Bits = fn_state.ga_350645;
        // D s_12_7: call Elem_set(s_12_1, s_12_3, s_12_5, s_12_6)
        let s_12_7: Bits = Elem_set(state, tracer, s_12_1, s_12_3, s_12_5, s_12_6);
        // D s_12_8: cast reint s_12_7 -> u64
        let s_12_8: u64 = (s_12_7.value() as u64);
        // D s_12_9: read-var ga#350646:i
        let s_12_9: i128 = fn_state.ga_350646;
        // D s_12_10: call D_set(s_12_9, s_12_8)
        let s_12_10: () = D_set(state, tracer, s_12_9, s_12_8);
        // D s_12_11: read-var e:i64
        let s_12_11: i64 = fn_state.e;
        // C s_12_12: const #1s : i64
        let s_12_12: i64 = 1;
        // D s_12_13: add s_12_11 s_12_12
        let s_12_13: i64 = (s_12_11 + s_12_12);
        // D s_12_14: write-var e <= s_12_13
        fn_state.e = s_12_13;
        // N s_12_15: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var esizeshadow#7344:i64
        let s_13_0: i64 = fn_state.esizeshadow_7344;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: call Ones(s_13_1)
        let s_13_2: Bits = Ones(state, tracer, s_13_1);
        // D s_13_3: write-var ga#350645 <= s_13_2
        fn_state.ga_350645 = s_13_2;
        // N s_13_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call StandardFPSCRValue(s_14_0)
        let s_14_1: ProductType5c790c8ef59cc8b2 = StandardFPSCRValue(
            state,
            tracer,
            s_14_0,
        );
        // D s_14_2: read-var op1:bv
        let s_14_2: Bits = fn_state.op1;
        // D s_14_3: read-var op2:bv
        let s_14_3: Bits = fn_state.op2;
        // D s_14_4: call FPCompareGE(s_14_2, s_14_3, s_14_1)
        let s_14_4: bool = FPCompareGE(state, tracer, s_14_2, s_14_3, s_14_1);
        // D s_14_5: write-var test_passed <= s_14_4
        fn_state.test_passed = s_14_4;
        // N s_14_6: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var r:i64
        let s_16_0: i64 = fn_state.r;
        // C s_16_1: const #1s : i64
        let s_16_1: i64 = 1;
        // D s_16_2: add s_16_0 s_16_1
        let s_16_2: i64 = (s_16_0 + s_16_1);
        // D s_16_3: write-var r <= s_16_2
        fn_state.r = s_16_2;
        // N s_16_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: return
        return;
    }
}
