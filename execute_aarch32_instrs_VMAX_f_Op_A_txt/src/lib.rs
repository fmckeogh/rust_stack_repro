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
use D_set::*;
use FPMin::*;
use Elem_read::*;
use D_read::*;
use FPMax::*;
use common::*;
pub fn execute_aarch32_instrs_VMAX_f_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d__arg: i64,
    elements: i64,
    esize: i64,
    m: i64,
    maximum: bool,
    n: i64,
    regs: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_355302: i128,
        ga_355292: u64,
        r: i64,
        gs_311967: i64,
        e: i64,
        op2: Bits,
        ga_355299: u64,
        d: i128,
        ga_355294: Bits,
        op1: Bits,
        esizeshadow_7495: i64,
        ga_355293: i64,
        gs_311973: i64,
        ga_355301: Bits,
        ga_355295: i128,
        ga_355300: i64,
        d__arg: i64,
        elements: i64,
        esize: i64,
        m: i64,
        maximum: bool,
        n: i64,
        regs: i64,
    }
    let fn_state = FunctionState {
        d__arg,
        elements,
        esize,
        m,
        maximum,
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
        // D s_0_3: write-var esizeshadow#7495 <= s_0_2
        fn_state.esizeshadow_7495 = s_0_2;
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
        // D s_1_6: write-var gs#311967 <= s_1_5
        fn_state.gs_311967 = s_1_5;
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
        // D s_2_1: read-var gs#311967:i64
        let s_2_1: i64 = fn_state.gs_311967;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b12 b3
        if s_2_2 {
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
        // D s_3_6: write-var gs#311973 <= s_3_5
        fn_state.gs_311973 = s_3_5;
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
        // D s_4_1: read-var gs#311973:i64
        let s_4_1: i64 = fn_state.gs_311973;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b11 b5
        if s_4_2 {
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
        // D s_5_8: read-var esizeshadow#7495:i64
        let s_5_8: i64 = fn_state.esizeshadow_7495;
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
        // D s_5_25: read-var esizeshadow#7495:i64
        let s_5_25: i64 = fn_state.esizeshadow_7495;
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
        // D s_5_34: read-var maximum:u8
        let s_5_34: bool = fn_state.maximum;
        // N s_5_35: branch s_5_34 b9 b6
        if s_5_34 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var r:i64
        let s_6_0: i64 = fn_state.r;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: read-var d:i
        let s_6_2: i128 = fn_state.d;
        // D s_6_3: add s_6_2 s_6_1
        let s_6_3: i128 = (s_6_2 + s_6_1);
        // D s_6_4: write-var ga#355302 <= s_6_3
        fn_state.ga_355302 = s_6_3;
        // D s_6_5: read-var r:i64
        let s_6_5: i64 = fn_state.r;
        // D s_6_6: cast zx s_6_5 -> i
        let s_6_6: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_7: read-var d:i
        let s_6_7: i128 = fn_state.d;
        // D s_6_8: add s_6_7 s_6_6
        let s_6_8: i128 = (s_6_7 + s_6_6);
        // D s_6_9: call D_read(s_6_8)
        let s_6_9: u64 = D_read(state, tracer, s_6_8);
        // D s_6_10: write-var ga#355299 <= s_6_9
        fn_state.ga_355299 = s_6_9;
        // D s_6_11: read-var esizeshadow#7495:i64
        let s_6_11: i64 = fn_state.esizeshadow_7495;
        // D s_6_12: cast zx s_6_11 -> i
        let s_6_12: i128 = (i128::try_from(s_6_11).unwrap());
        // D s_6_13: cast reint s_6_12 -> i64
        let s_6_13: i64 = (s_6_12 as i64);
        // D s_6_14: write-var ga#355300 <= s_6_13
        fn_state.ga_355300 = s_6_13;
        // C s_6_15: const #() : ()
        let s_6_15: () = ();
        // S s_6_16: call StandardFPSCRValue(s_6_15)
        let s_6_16: ProductType5c790c8ef59cc8b2 = StandardFPSCRValue(
            state,
            tracer,
            s_6_15,
        );
        // D s_6_17: read-var op1:bv
        let s_6_17: Bits = fn_state.op1;
        // D s_6_18: read-var op2:bv
        let s_6_18: Bits = fn_state.op2;
        // D s_6_19: call FPMin(s_6_17, s_6_18, s_6_16)
        let s_6_19: Bits = FPMin(state, tracer, s_6_17, s_6_18, s_6_16);
        // D s_6_20: write-var ga#355301 <= s_6_19
        fn_state.ga_355301 = s_6_19;
        // N s_6_21: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var ga#355299:u64
        let s_7_0: u64 = fn_state.ga_355299;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 64u16);
        // D s_7_2: read-var e:i64
        let s_7_2: i64 = fn_state.e;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: read-var ga#355300:i64
        let s_7_4: i64 = fn_state.ga_355300;
        // D s_7_5: cast zx s_7_4 -> i
        let s_7_5: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_6: read-var ga#355301:bv
        let s_7_6: Bits = fn_state.ga_355301;
        // D s_7_7: call Elem_set(s_7_1, s_7_3, s_7_5, s_7_6)
        let s_7_7: Bits = Elem_set(state, tracer, s_7_1, s_7_3, s_7_5, s_7_6);
        // D s_7_8: cast reint s_7_7 -> u64
        let s_7_8: u64 = (s_7_7.value() as u64);
        // D s_7_9: read-var ga#355302:i
        let s_7_9: i128 = fn_state.ga_355302;
        // D s_7_10: call D_set(s_7_9, s_7_8)
        let s_7_10: () = D_set(state, tracer, s_7_9, s_7_8);
        // N s_7_11: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var e:i64
        let s_8_0: i64 = fn_state.e;
        // C s_8_1: const #1s : i64
        let s_8_1: i64 = 1;
        // D s_8_2: add s_8_0 s_8_1
        let s_8_2: i64 = (s_8_0 + s_8_1);
        // D s_8_3: write-var e <= s_8_2
        fn_state.e = s_8_2;
        // N s_8_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var r:i64
        let s_9_0: i64 = fn_state.r;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: read-var d:i
        let s_9_2: i128 = fn_state.d;
        // D s_9_3: add s_9_2 s_9_1
        let s_9_3: i128 = (s_9_2 + s_9_1);
        // D s_9_4: write-var ga#355295 <= s_9_3
        fn_state.ga_355295 = s_9_3;
        // D s_9_5: read-var r:i64
        let s_9_5: i64 = fn_state.r;
        // D s_9_6: cast zx s_9_5 -> i
        let s_9_6: i128 = (i128::try_from(s_9_5).unwrap());
        // D s_9_7: read-var d:i
        let s_9_7: i128 = fn_state.d;
        // D s_9_8: add s_9_7 s_9_6
        let s_9_8: i128 = (s_9_7 + s_9_6);
        // D s_9_9: call D_read(s_9_8)
        let s_9_9: u64 = D_read(state, tracer, s_9_8);
        // D s_9_10: write-var ga#355292 <= s_9_9
        fn_state.ga_355292 = s_9_9;
        // D s_9_11: read-var esizeshadow#7495:i64
        let s_9_11: i64 = fn_state.esizeshadow_7495;
        // D s_9_12: cast zx s_9_11 -> i
        let s_9_12: i128 = (i128::try_from(s_9_11).unwrap());
        // D s_9_13: cast reint s_9_12 -> i64
        let s_9_13: i64 = (s_9_12 as i64);
        // D s_9_14: write-var ga#355293 <= s_9_13
        fn_state.ga_355293 = s_9_13;
        // C s_9_15: const #() : ()
        let s_9_15: () = ();
        // S s_9_16: call StandardFPSCRValue(s_9_15)
        let s_9_16: ProductType5c790c8ef59cc8b2 = StandardFPSCRValue(
            state,
            tracer,
            s_9_15,
        );
        // D s_9_17: read-var op1:bv
        let s_9_17: Bits = fn_state.op1;
        // D s_9_18: read-var op2:bv
        let s_9_18: Bits = fn_state.op2;
        // D s_9_19: call FPMax(s_9_17, s_9_18, s_9_16)
        let s_9_19: Bits = FPMax(state, tracer, s_9_17, s_9_18, s_9_16);
        // D s_9_20: write-var ga#355294 <= s_9_19
        fn_state.ga_355294 = s_9_19;
        // N s_9_21: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var ga#355292:u64
        let s_10_0: u64 = fn_state.ga_355292;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 64u16);
        // D s_10_2: read-var e:i64
        let s_10_2: i64 = fn_state.e;
        // D s_10_3: cast zx s_10_2 -> i
        let s_10_3: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_4: read-var ga#355293:i64
        let s_10_4: i64 = fn_state.ga_355293;
        // D s_10_5: cast zx s_10_4 -> i
        let s_10_5: i128 = (i128::try_from(s_10_4).unwrap());
        // D s_10_6: read-var ga#355294:bv
        let s_10_6: Bits = fn_state.ga_355294;
        // D s_10_7: call Elem_set(s_10_1, s_10_3, s_10_5, s_10_6)
        let s_10_7: Bits = Elem_set(state, tracer, s_10_1, s_10_3, s_10_5, s_10_6);
        // D s_10_8: cast reint s_10_7 -> u64
        let s_10_8: u64 = (s_10_7.value() as u64);
        // D s_10_9: read-var ga#355295:i
        let s_10_9: i128 = fn_state.ga_355295;
        // D s_10_10: call D_set(s_10_9, s_10_8)
        let s_10_10: () = D_set(state, tracer, s_10_9, s_10_8);
        // N s_10_11: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var r:i64
        let s_11_0: i64 = fn_state.r;
        // C s_11_1: const #1s : i64
        let s_11_1: i64 = 1;
        // D s_11_2: add s_11_0 s_11_1
        let s_11_2: i64 = (s_11_0 + s_11_1);
        // D s_11_3: write-var r <= s_11_2
        fn_state.r = s_11_2;
        // N s_11_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: return
        return;
    }
}
