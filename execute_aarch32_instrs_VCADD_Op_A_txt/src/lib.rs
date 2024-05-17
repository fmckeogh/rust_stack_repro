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
use CheckAdvSIMDEnabled::*;
use FPAdd::*;
use FPNeg::*;
use D_set::*;
use Elem_read::*;
use D_read::*;
use fdiv_int::*;
use common::*;
pub fn execute_aarch32_instrs_VCADD_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d__arg: i64,
    elements: i128,
    esize: i64,
    m: i64,
    n: i64,
    regs: i64,
    rot: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        result2: Bits,
        e: i64,
        element3: Bits,
        gs_326494: i64,
        result1: Bits,
        esizeshadow_7988: i64,
        element1: Bits,
        d: i128,
        gs_326485: i64,
        element3shadow_7989: Bits,
        operand1: u64,
        operand2: u64,
        d__arg: i64,
        elements: i128,
        esize: i64,
        m: i64,
        n: i64,
        regs: i64,
        rot: bool,
    }
    let fn_state = FunctionState {
        d__arg,
        elements,
        esize,
        m,
        n,
        regs,
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
        // D s_0_3: write-var esizeshadow#7988 <= s_0_2
        fn_state.esizeshadow_7988 = s_0_2;
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
        // D s_1_6: write-var gs#326485 <= s_1_5
        fn_state.gs_326485 = s_1_5;
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
        // D s_2_1: read-var gs#326485:i64
        let s_2_1: i64 = fn_state.gs_326485;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b14 b3
        if s_2_2 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var n:i64
        let s_3_0: i64 = fn_state.n;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var r:i64
        let s_3_2: i64 = fn_state.r;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: add s_3_1 s_3_3
        let s_3_4: i128 = (s_3_1 + s_3_3);
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: cast zx s_3_5 -> i
        let s_3_6: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_7: call D_read(s_3_6)
        let s_3_7: u64 = D_read(state, tracer, s_3_6);
        // D s_3_8: write-var operand1 <= s_3_7
        fn_state.operand1 = s_3_7;
        // D s_3_9: read-var m:i64
        let s_3_9: i64 = fn_state.m;
        // D s_3_10: cast zx s_3_9 -> i
        let s_3_10: i128 = (i128::try_from(s_3_9).unwrap());
        // D s_3_11: read-var r:i64
        let s_3_11: i64 = fn_state.r;
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_13: add s_3_10 s_3_12
        let s_3_13: i128 = (s_3_10 + s_3_12);
        // D s_3_14: cast reint s_3_13 -> i64
        let s_3_14: i64 = (s_3_13 as i64);
        // D s_3_15: cast zx s_3_14 -> i
        let s_3_15: i128 = (i128::try_from(s_3_14).unwrap());
        // D s_3_16: call D_read(s_3_15)
        let s_3_16: u64 = D_read(state, tracer, s_3_15);
        // D s_3_17: write-var operand2 <= s_3_16
        fn_state.operand2 = s_3_16;
        // C s_3_18: const #0s : i64
        let s_3_18: i64 = 0;
        // C s_3_19: const #2s : i
        let s_3_19: i128 = 2;
        // D s_3_20: read-var elements:i
        let s_3_20: i128 = fn_state.elements;
        // D s_3_21: call fdiv_int(s_3_20, s_3_19)
        let s_3_21: i128 = fdiv_int(state, tracer, s_3_20, s_3_19);
        // C s_3_22: const #1s : i
        let s_3_22: i128 = 1;
        // D s_3_23: sub s_3_21 s_3_22
        let s_3_23: i128 = ((s_3_21) - (s_3_22));
        // D s_3_24: cast reint s_3_23 -> i64
        let s_3_24: i64 = (s_3_23 as i64);
        // D s_3_25: write-var gs#326494 <= s_3_24
        fn_state.gs_326494 = s_3_24;
        // D s_3_26: write-var e <= s_3_18
        fn_state.e = s_3_18;
        // N s_3_27: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#326494:i64
        let s_4_1: i64 = fn_state.gs_326494;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b13 b5
        if s_4_2 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var rot:u8
        let s_5_0: bool = fn_state.rot;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 1u16);
        // C s_5_2: const #0u : u8
        let s_5_2: bool = false;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // D s_5_5: not s_5_4
        let s_5_5: bool = !s_5_4;
        // N s_5_6: branch s_5_5 b11 b6
        if s_5_5 {
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
        // C s_6_0: const #2s : i
        let s_6_0: i128 = 2;
        // D s_6_1: read-var e:i64
        let s_6_1: i64 = fn_state.e;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: mul s_6_2 s_6_0
        let s_6_3: i128 = ((s_6_2) * (s_6_0));
        // C s_6_4: const #1s : i
        let s_6_4: i128 = 1;
        // D s_6_5: add s_6_3 s_6_4
        let s_6_5: i128 = (s_6_3 + s_6_4);
        // D s_6_6: read-var esizeshadow#7988:i64
        let s_6_6: i64 = fn_state.esizeshadow_7988;
        // D s_6_7: cast zx s_6_6 -> i
        let s_6_7: i128 = (i128::try_from(s_6_6).unwrap());
        // D s_6_8: cast reint s_6_7 -> i64
        let s_6_8: i64 = (s_6_7 as i64);
        // D s_6_9: read-var operand2:u64
        let s_6_9: u64 = fn_state.operand2;
        // D s_6_10: cast zx s_6_9 -> bv
        let s_6_10: Bits = Bits::new(s_6_9 as u128, 64u16);
        // D s_6_11: cast zx s_6_8 -> i
        let s_6_11: i128 = (i128::try_from(s_6_8).unwrap());
        // D s_6_12: call Elem_read(s_6_10, s_6_5, s_6_11)
        let s_6_12: Bits = Elem_read(state, tracer, s_6_10, s_6_5, s_6_11);
        // D s_6_13: call FPNeg(s_6_12)
        let s_6_13: Bits = FPNeg(state, tracer, s_6_12);
        // D s_6_14: write-var element1 <= s_6_13
        fn_state.element1 = s_6_13;
        // N s_6_15: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #2s : i
        let s_7_0: i128 = 2;
        // D s_7_1: read-var e:i64
        let s_7_1: i64 = fn_state.e;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: mul s_7_2 s_7_0
        let s_7_3: i128 = ((s_7_2) * (s_7_0));
        // D s_7_4: read-var esizeshadow#7988:i64
        let s_7_4: i64 = fn_state.esizeshadow_7988;
        // D s_7_5: cast zx s_7_4 -> i
        let s_7_5: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_6: cast reint s_7_5 -> i64
        let s_7_6: i64 = (s_7_5 as i64);
        // D s_7_7: read-var operand2:u64
        let s_7_7: u64 = fn_state.operand2;
        // D s_7_8: cast zx s_7_7 -> bv
        let s_7_8: Bits = Bits::new(s_7_7 as u128, 64u16);
        // D s_7_9: cast zx s_7_6 -> i
        let s_7_9: i128 = (i128::try_from(s_7_6).unwrap());
        // D s_7_10: call Elem_read(s_7_8, s_7_3, s_7_9)
        let s_7_10: Bits = Elem_read(state, tracer, s_7_8, s_7_3, s_7_9);
        // D s_7_11: write-var element3 <= s_7_10
        fn_state.element3 = s_7_10;
        // N s_7_12: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var element3:bv
        let s_8_0: Bits = fn_state.element3;
        // D s_8_1: write-var element3shadow#7989 <= s_8_0
        fn_state.element3shadow_7989 = s_8_0;
        // D s_8_2: read-var element1:bv
        let s_8_2: Bits = fn_state.element1;
        // C s_8_3: const #2s : i
        let s_8_3: i128 = 2;
        // D s_8_4: read-var e:i64
        let s_8_4: i64 = fn_state.e;
        // D s_8_5: cast zx s_8_4 -> i
        let s_8_5: i128 = (i128::try_from(s_8_4).unwrap());
        // D s_8_6: mul s_8_5 s_8_3
        let s_8_6: i128 = ((s_8_5) * (s_8_3));
        // D s_8_7: read-var esizeshadow#7988:i64
        let s_8_7: i64 = fn_state.esizeshadow_7988;
        // D s_8_8: cast zx s_8_7 -> i
        let s_8_8: i128 = (i128::try_from(s_8_7).unwrap());
        // D s_8_9: cast reint s_8_8 -> i64
        let s_8_9: i64 = (s_8_8 as i64);
        // D s_8_10: read-var operand1:u64
        let s_8_10: u64 = fn_state.operand1;
        // D s_8_11: cast zx s_8_10 -> bv
        let s_8_11: Bits = Bits::new(s_8_10 as u128, 64u16);
        // D s_8_12: cast zx s_8_9 -> i
        let s_8_12: i128 = (i128::try_from(s_8_9).unwrap());
        // D s_8_13: call Elem_read(s_8_11, s_8_6, s_8_12)
        let s_8_13: Bits = Elem_read(state, tracer, s_8_11, s_8_6, s_8_12);
        // C s_8_14: const #() : ()
        let s_8_14: () = ();
        // S s_8_15: call StandardFPSCRValue(s_8_14)
        let s_8_15: ProductType5c790c8ef59cc8b2 = StandardFPSCRValue(
            state,
            tracer,
            s_8_14,
        );
        // D s_8_16: call FPAdd(s_8_13, s_8_2, s_8_15)
        let s_8_16: Bits = FPAdd(state, tracer, s_8_13, s_8_2, s_8_15);
        // D s_8_17: write-var result1 <= s_8_16
        fn_state.result1 = s_8_16;
        // N s_8_18: jump b9
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
        // C s_9_4: const #1s : i
        let s_9_4: i128 = 1;
        // D s_9_5: add s_9_3 s_9_4
        let s_9_5: i128 = (s_9_3 + s_9_4);
        // D s_9_6: read-var esizeshadow#7988:i64
        let s_9_6: i64 = fn_state.esizeshadow_7988;
        // D s_9_7: cast zx s_9_6 -> i
        let s_9_7: i128 = (i128::try_from(s_9_6).unwrap());
        // D s_9_8: cast reint s_9_7 -> i64
        let s_9_8: i64 = (s_9_7 as i64);
        // D s_9_9: read-var operand1:u64
        let s_9_9: u64 = fn_state.operand1;
        // D s_9_10: cast zx s_9_9 -> bv
        let s_9_10: Bits = Bits::new(s_9_9 as u128, 64u16);
        // D s_9_11: cast zx s_9_8 -> i
        let s_9_11: i128 = (i128::try_from(s_9_8).unwrap());
        // D s_9_12: call Elem_read(s_9_10, s_9_5, s_9_11)
        let s_9_12: Bits = Elem_read(state, tracer, s_9_10, s_9_5, s_9_11);
        // C s_9_13: const #() : ()
        let s_9_13: () = ();
        // S s_9_14: call StandardFPSCRValue(s_9_13)
        let s_9_14: ProductType5c790c8ef59cc8b2 = StandardFPSCRValue(
            state,
            tracer,
            s_9_13,
        );
        // D s_9_15: read-var element3shadow#7989:bv
        let s_9_15: Bits = fn_state.element3shadow_7989;
        // D s_9_16: call FPAdd(s_9_12, s_9_15, s_9_14)
        let s_9_16: Bits = FPAdd(state, tracer, s_9_12, s_9_15, s_9_14);
        // D s_9_17: write-var result2 <= s_9_16
        fn_state.result2 = s_9_16;
        // N s_9_18: jump b10
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
        // D s_10_4: read-var r:i64
        let s_10_4: i64 = fn_state.r;
        // D s_10_5: cast zx s_10_4 -> i
        let s_10_5: i128 = (i128::try_from(s_10_4).unwrap());
        // D s_10_6: read-var d:i
        let s_10_6: i128 = fn_state.d;
        // D s_10_7: add s_10_6 s_10_5
        let s_10_7: i128 = (s_10_6 + s_10_5);
        // D s_10_8: call D_read(s_10_7)
        let s_10_8: u64 = D_read(state, tracer, s_10_7);
        // C s_10_9: const #2s : i
        let s_10_9: i128 = 2;
        // D s_10_10: read-var e:i64
        let s_10_10: i64 = fn_state.e;
        // D s_10_11: cast zx s_10_10 -> i
        let s_10_11: i128 = (i128::try_from(s_10_10).unwrap());
        // D s_10_12: mul s_10_11 s_10_9
        let s_10_12: i128 = ((s_10_11) * (s_10_9));
        // D s_10_13: read-var esizeshadow#7988:i64
        let s_10_13: i64 = fn_state.esizeshadow_7988;
        // D s_10_14: cast zx s_10_13 -> i
        let s_10_14: i128 = (i128::try_from(s_10_13).unwrap());
        // D s_10_15: cast reint s_10_14 -> i64
        let s_10_15: i64 = (s_10_14 as i64);
        // D s_10_16: cast zx s_10_8 -> bv
        let s_10_16: Bits = Bits::new(s_10_8 as u128, 64u16);
        // D s_10_17: cast zx s_10_15 -> i
        let s_10_17: i128 = (i128::try_from(s_10_15).unwrap());
        // D s_10_18: read-var result1:bv
        let s_10_18: Bits = fn_state.result1;
        // D s_10_19: call Elem_set(s_10_16, s_10_12, s_10_17, s_10_18)
        let s_10_19: Bits = Elem_set(state, tracer, s_10_16, s_10_12, s_10_17, s_10_18);
        // D s_10_20: cast reint s_10_19 -> u64
        let s_10_20: u64 = (s_10_19.value() as u64);
        // D s_10_21: call D_set(s_10_3, s_10_20)
        let s_10_21: () = D_set(state, tracer, s_10_3, s_10_20);
        // D s_10_22: read-var r:i64
        let s_10_22: i64 = fn_state.r;
        // D s_10_23: cast zx s_10_22 -> i
        let s_10_23: i128 = (i128::try_from(s_10_22).unwrap());
        // D s_10_24: read-var d:i
        let s_10_24: i128 = fn_state.d;
        // D s_10_25: add s_10_24 s_10_23
        let s_10_25: i128 = (s_10_24 + s_10_23);
        // D s_10_26: read-var r:i64
        let s_10_26: i64 = fn_state.r;
        // D s_10_27: cast zx s_10_26 -> i
        let s_10_27: i128 = (i128::try_from(s_10_26).unwrap());
        // D s_10_28: read-var d:i
        let s_10_28: i128 = fn_state.d;
        // D s_10_29: add s_10_28 s_10_27
        let s_10_29: i128 = (s_10_28 + s_10_27);
        // D s_10_30: call D_read(s_10_29)
        let s_10_30: u64 = D_read(state, tracer, s_10_29);
        // C s_10_31: const #2s : i
        let s_10_31: i128 = 2;
        // D s_10_32: read-var e:i64
        let s_10_32: i64 = fn_state.e;
        // D s_10_33: cast zx s_10_32 -> i
        let s_10_33: i128 = (i128::try_from(s_10_32).unwrap());
        // D s_10_34: mul s_10_33 s_10_31
        let s_10_34: i128 = ((s_10_33) * (s_10_31));
        // C s_10_35: const #1s : i
        let s_10_35: i128 = 1;
        // D s_10_36: add s_10_34 s_10_35
        let s_10_36: i128 = (s_10_34 + s_10_35);
        // D s_10_37: read-var esizeshadow#7988:i64
        let s_10_37: i64 = fn_state.esizeshadow_7988;
        // D s_10_38: cast zx s_10_37 -> i
        let s_10_38: i128 = (i128::try_from(s_10_37).unwrap());
        // D s_10_39: cast reint s_10_38 -> i64
        let s_10_39: i64 = (s_10_38 as i64);
        // D s_10_40: cast zx s_10_30 -> bv
        let s_10_40: Bits = Bits::new(s_10_30 as u128, 64u16);
        // D s_10_41: cast zx s_10_39 -> i
        let s_10_41: i128 = (i128::try_from(s_10_39).unwrap());
        // D s_10_42: read-var result2:bv
        let s_10_42: Bits = fn_state.result2;
        // D s_10_43: call Elem_set(s_10_40, s_10_36, s_10_41, s_10_42)
        let s_10_43: Bits = Elem_set(state, tracer, s_10_40, s_10_36, s_10_41, s_10_42);
        // D s_10_44: cast reint s_10_43 -> u64
        let s_10_44: u64 = (s_10_43.value() as u64);
        // D s_10_45: call D_set(s_10_25, s_10_44)
        let s_10_45: () = D_set(state, tracer, s_10_25, s_10_44);
        // D s_10_46: read-var e:i64
        let s_10_46: i64 = fn_state.e;
        // C s_10_47: const #1s : i64
        let s_10_47: i64 = 1;
        // D s_10_48: add s_10_46 s_10_47
        let s_10_48: i64 = (s_10_46 + s_10_47);
        // D s_10_49: write-var e <= s_10_48
        fn_state.e = s_10_48;
        // N s_10_50: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #2s : i
        let s_11_0: i128 = 2;
        // D s_11_1: read-var e:i64
        let s_11_1: i64 = fn_state.e;
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // D s_11_3: mul s_11_2 s_11_0
        let s_11_3: i128 = ((s_11_2) * (s_11_0));
        // C s_11_4: const #1s : i
        let s_11_4: i128 = 1;
        // D s_11_5: add s_11_3 s_11_4
        let s_11_5: i128 = (s_11_3 + s_11_4);
        // D s_11_6: read-var esizeshadow#7988:i64
        let s_11_6: i64 = fn_state.esizeshadow_7988;
        // D s_11_7: cast zx s_11_6 -> i
        let s_11_7: i128 = (i128::try_from(s_11_6).unwrap());
        // D s_11_8: cast reint s_11_7 -> i64
        let s_11_8: i64 = (s_11_7 as i64);
        // D s_11_9: read-var operand2:u64
        let s_11_9: u64 = fn_state.operand2;
        // D s_11_10: cast zx s_11_9 -> bv
        let s_11_10: Bits = Bits::new(s_11_9 as u128, 64u16);
        // D s_11_11: cast zx s_11_8 -> i
        let s_11_11: i128 = (i128::try_from(s_11_8).unwrap());
        // D s_11_12: call Elem_read(s_11_10, s_11_5, s_11_11)
        let s_11_12: Bits = Elem_read(state, tracer, s_11_10, s_11_5, s_11_11);
        // D s_11_13: write-var element1 <= s_11_12
        fn_state.element1 = s_11_12;
        // C s_11_14: const #2s : i
        let s_11_14: i128 = 2;
        // D s_11_15: read-var e:i64
        let s_11_15: i64 = fn_state.e;
        // D s_11_16: cast zx s_11_15 -> i
        let s_11_16: i128 = (i128::try_from(s_11_15).unwrap());
        // D s_11_17: mul s_11_16 s_11_14
        let s_11_17: i128 = ((s_11_16) * (s_11_14));
        // D s_11_18: read-var esizeshadow#7988:i64
        let s_11_18: i64 = fn_state.esizeshadow_7988;
        // D s_11_19: cast zx s_11_18 -> i
        let s_11_19: i128 = (i128::try_from(s_11_18).unwrap());
        // D s_11_20: cast reint s_11_19 -> i64
        let s_11_20: i64 = (s_11_19 as i64);
        // D s_11_21: read-var operand2:u64
        let s_11_21: u64 = fn_state.operand2;
        // D s_11_22: cast zx s_11_21 -> bv
        let s_11_22: Bits = Bits::new(s_11_21 as u128, 64u16);
        // D s_11_23: cast zx s_11_20 -> i
        let s_11_23: i128 = (i128::try_from(s_11_20).unwrap());
        // D s_11_24: call Elem_read(s_11_22, s_11_17, s_11_23)
        let s_11_24: Bits = Elem_read(state, tracer, s_11_22, s_11_17, s_11_23);
        // D s_11_25: call FPNeg(s_11_24)
        let s_11_25: Bits = FPNeg(state, tracer, s_11_24);
        // D s_11_26: write-var element3 <= s_11_25
        fn_state.element3 = s_11_25;
        // N s_11_27: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var r:i64
        let s_13_0: i64 = fn_state.r;
        // C s_13_1: const #1s : i64
        let s_13_1: i64 = 1;
        // D s_13_2: add s_13_0 s_13_1
        let s_13_2: i64 = (s_13_0 + s_13_1);
        // D s_13_3: write-var r <= s_13_2
        fn_state.r = s_13_2;
        // N s_13_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: return
        return;
    }
}
