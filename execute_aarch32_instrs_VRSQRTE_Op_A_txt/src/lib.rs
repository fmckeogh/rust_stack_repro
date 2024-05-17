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
use D_set::*;
use FPRSqrtEstimate::*;
use Elem_read::*;
use UnsignedRSqrtEstimate::*;
use D_read::*;
use common::*;
pub fn execute_aarch32_instrs_VRSQRTE_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d__arg: i64,
    elements: i64,
    esize: i64,
    floating_point: bool,
    m: i64,
    regs: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        e: i64,
        gs_318277: i64,
        ga_360351: u64,
        ga_360352: i64,
        d: i128,
        esizeshadow_7774: i64,
        ga_360353: Bits,
        ga_360354: i128,
        gs_318283: i64,
        d__arg: i64,
        elements: i64,
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
        // D s_0_3: write-var esizeshadow#7774 <= s_0_2
        fn_state.esizeshadow_7774 = s_0_2;
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
        // D s_1_6: write-var gs#318277 <= s_1_5
        fn_state.gs_318277 = s_1_5;
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
        // D s_2_1: read-var gs#318277:i64
        let s_2_1: i64 = fn_state.gs_318277;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b11 b3
        if s_2_2 {
            return block_11(state, tracer, fn_state);
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
        // D s_3_6: write-var gs#318283 <= s_3_5
        fn_state.gs_318283 = s_3_5;
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
        // D s_4_1: read-var gs#318283:i64
        let s_4_1: i64 = fn_state.gs_318283;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b10 b5
        if s_4_2 {
            return block_10(state, tracer, fn_state);
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
        // N s_5_1: branch s_5_0 b8 b6
        if s_5_0 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var esizeshadow#7774:i64
        let s_6_0: i64 = fn_state.esizeshadow_7774;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: call __id(s_6_1)
        let s_6_2: i128 = u__id(state, tracer, s_6_1);
        // D s_6_3: cast reint s_6_2 -> i64
        let s_6_3: i64 = (s_6_2 as i64);
        // C s_6_4: const #32s : i
        let s_6_4: i128 = 32;
        // D s_6_5: cast zx s_6_3 -> i
        let s_6_5: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_6: cmp-eq s_6_5 s_6_4
        let s_6_6: bool = ((s_6_5) == (s_6_4));
        // N s_6_7: assert s_6_6
        let s_6_7: () = assert!(s_6_6);
        // D s_6_8: read-var r:i64
        let s_6_8: i64 = fn_state.r;
        // D s_6_9: cast zx s_6_8 -> i
        let s_6_9: i128 = (i128::try_from(s_6_8).unwrap());
        // D s_6_10: read-var d:i
        let s_6_10: i128 = fn_state.d;
        // D s_6_11: add s_6_10 s_6_9
        let s_6_11: i128 = (s_6_10 + s_6_9);
        // D s_6_12: read-var r:i64
        let s_6_12: i64 = fn_state.r;
        // D s_6_13: cast zx s_6_12 -> i
        let s_6_13: i128 = (i128::try_from(s_6_12).unwrap());
        // D s_6_14: read-var d:i
        let s_6_14: i128 = fn_state.d;
        // D s_6_15: add s_6_14 s_6_13
        let s_6_15: i128 = (s_6_14 + s_6_13);
        // D s_6_16: call D_read(s_6_15)
        let s_6_16: u64 = D_read(state, tracer, s_6_15);
        // D s_6_17: read-var esizeshadow#7774:i64
        let s_6_17: i64 = fn_state.esizeshadow_7774;
        // D s_6_18: cast zx s_6_17 -> i
        let s_6_18: i128 = (i128::try_from(s_6_17).unwrap());
        // D s_6_19: cast reint s_6_18 -> i64
        let s_6_19: i64 = (s_6_18 as i64);
        // D s_6_20: read-var m:i64
        let s_6_20: i64 = fn_state.m;
        // D s_6_21: cast zx s_6_20 -> i
        let s_6_21: i128 = (i128::try_from(s_6_20).unwrap());
        // D s_6_22: read-var r:i64
        let s_6_22: i64 = fn_state.r;
        // D s_6_23: cast zx s_6_22 -> i
        let s_6_23: i128 = (i128::try_from(s_6_22).unwrap());
        // D s_6_24: add s_6_21 s_6_23
        let s_6_24: i128 = (s_6_21 + s_6_23);
        // D s_6_25: cast reint s_6_24 -> i64
        let s_6_25: i64 = (s_6_24 as i64);
        // D s_6_26: cast zx s_6_25 -> i
        let s_6_26: i128 = (i128::try_from(s_6_25).unwrap());
        // D s_6_27: call D_read(s_6_26)
        let s_6_27: u64 = D_read(state, tracer, s_6_26);
        // D s_6_28: read-var esizeshadow#7774:i64
        let s_6_28: i64 = fn_state.esizeshadow_7774;
        // D s_6_29: cast zx s_6_28 -> i
        let s_6_29: i128 = (i128::try_from(s_6_28).unwrap());
        // D s_6_30: cast reint s_6_29 -> i64
        let s_6_30: i64 = (s_6_29 as i64);
        // D s_6_31: cast zx s_6_27 -> bv
        let s_6_31: Bits = Bits::new(s_6_27 as u128, 64u16);
        // D s_6_32: read-var e:i64
        let s_6_32: i64 = fn_state.e;
        // D s_6_33: cast zx s_6_32 -> i
        let s_6_33: i128 = (i128::try_from(s_6_32).unwrap());
        // D s_6_34: cast zx s_6_30 -> i
        let s_6_34: i128 = (i128::try_from(s_6_30).unwrap());
        // D s_6_35: call Elem_read(s_6_31, s_6_33, s_6_34)
        let s_6_35: Bits = Elem_read(state, tracer, s_6_31, s_6_33, s_6_34);
        // D s_6_36: cast reint s_6_35 -> u32
        let s_6_36: u32 = (s_6_35.value() as u32);
        // D s_6_37: cast zx s_6_36 -> bv
        let s_6_37: Bits = Bits::new(s_6_36 as u128, 32u16);
        // D s_6_38: call UnsignedRSqrtEstimate(s_6_37)
        let s_6_38: Bits = UnsignedRSqrtEstimate(state, tracer, s_6_37);
        // D s_6_39: cast reint s_6_38 -> u32
        let s_6_39: u32 = (s_6_38.value() as u32);
        // D s_6_40: cast zx s_6_16 -> bv
        let s_6_40: Bits = Bits::new(s_6_16 as u128, 64u16);
        // D s_6_41: read-var e:i64
        let s_6_41: i64 = fn_state.e;
        // D s_6_42: cast zx s_6_41 -> i
        let s_6_42: i128 = (i128::try_from(s_6_41).unwrap());
        // D s_6_43: cast zx s_6_19 -> i
        let s_6_43: i128 = (i128::try_from(s_6_19).unwrap());
        // D s_6_44: cast zx s_6_39 -> bv
        let s_6_44: Bits = Bits::new(s_6_39 as u128, 32u16);
        // D s_6_45: call Elem_set(s_6_40, s_6_42, s_6_43, s_6_44)
        let s_6_45: Bits = Elem_set(state, tracer, s_6_40, s_6_42, s_6_43, s_6_44);
        // D s_6_46: cast reint s_6_45 -> u64
        let s_6_46: u64 = (s_6_45.value() as u64);
        // D s_6_47: call D_set(s_6_11, s_6_46)
        let s_6_47: () = D_set(state, tracer, s_6_11, s_6_46);
        // N s_6_48: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var e:i64
        let s_7_0: i64 = fn_state.e;
        // C s_7_1: const #1s : i64
        let s_7_1: i64 = 1;
        // D s_7_2: add s_7_0 s_7_1
        let s_7_2: i64 = (s_7_0 + s_7_1);
        // D s_7_3: write-var e <= s_7_2
        fn_state.e = s_7_2;
        // N s_7_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var r:i64
        let s_8_0: i64 = fn_state.r;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: read-var d:i
        let s_8_2: i128 = fn_state.d;
        // D s_8_3: add s_8_2 s_8_1
        let s_8_3: i128 = (s_8_2 + s_8_1);
        // D s_8_4: write-var ga#360354 <= s_8_3
        fn_state.ga_360354 = s_8_3;
        // D s_8_5: read-var r:i64
        let s_8_5: i64 = fn_state.r;
        // D s_8_6: cast zx s_8_5 -> i
        let s_8_6: i128 = (i128::try_from(s_8_5).unwrap());
        // D s_8_7: read-var d:i
        let s_8_7: i128 = fn_state.d;
        // D s_8_8: add s_8_7 s_8_6
        let s_8_8: i128 = (s_8_7 + s_8_6);
        // D s_8_9: call D_read(s_8_8)
        let s_8_9: u64 = D_read(state, tracer, s_8_8);
        // D s_8_10: write-var ga#360351 <= s_8_9
        fn_state.ga_360351 = s_8_9;
        // D s_8_11: read-var esizeshadow#7774:i64
        let s_8_11: i64 = fn_state.esizeshadow_7774;
        // D s_8_12: cast zx s_8_11 -> i
        let s_8_12: i128 = (i128::try_from(s_8_11).unwrap());
        // D s_8_13: cast reint s_8_12 -> i64
        let s_8_13: i64 = (s_8_12 as i64);
        // D s_8_14: write-var ga#360352 <= s_8_13
        fn_state.ga_360352 = s_8_13;
        // D s_8_15: read-var m:i64
        let s_8_15: i64 = fn_state.m;
        // D s_8_16: cast zx s_8_15 -> i
        let s_8_16: i128 = (i128::try_from(s_8_15).unwrap());
        // D s_8_17: read-var r:i64
        let s_8_17: i64 = fn_state.r;
        // D s_8_18: cast zx s_8_17 -> i
        let s_8_18: i128 = (i128::try_from(s_8_17).unwrap());
        // D s_8_19: add s_8_16 s_8_18
        let s_8_19: i128 = (s_8_16 + s_8_18);
        // D s_8_20: cast reint s_8_19 -> i64
        let s_8_20: i64 = (s_8_19 as i64);
        // D s_8_21: cast zx s_8_20 -> i
        let s_8_21: i128 = (i128::try_from(s_8_20).unwrap());
        // D s_8_22: call D_read(s_8_21)
        let s_8_22: u64 = D_read(state, tracer, s_8_21);
        // D s_8_23: read-var esizeshadow#7774:i64
        let s_8_23: i64 = fn_state.esizeshadow_7774;
        // D s_8_24: cast zx s_8_23 -> i
        let s_8_24: i128 = (i128::try_from(s_8_23).unwrap());
        // D s_8_25: cast reint s_8_24 -> i64
        let s_8_25: i64 = (s_8_24 as i64);
        // D s_8_26: cast zx s_8_22 -> bv
        let s_8_26: Bits = Bits::new(s_8_22 as u128, 64u16);
        // D s_8_27: read-var e:i64
        let s_8_27: i64 = fn_state.e;
        // D s_8_28: cast zx s_8_27 -> i
        let s_8_28: i128 = (i128::try_from(s_8_27).unwrap());
        // D s_8_29: cast zx s_8_25 -> i
        let s_8_29: i128 = (i128::try_from(s_8_25).unwrap());
        // D s_8_30: call Elem_read(s_8_26, s_8_28, s_8_29)
        let s_8_30: Bits = Elem_read(state, tracer, s_8_26, s_8_28, s_8_29);
        // C s_8_31: const #() : ()
        let s_8_31: () = ();
        // S s_8_32: call StandardFPSCRValue(s_8_31)
        let s_8_32: ProductType5c790c8ef59cc8b2 = StandardFPSCRValue(
            state,
            tracer,
            s_8_31,
        );
        // D s_8_33: call FPRSqrtEstimate(s_8_30, s_8_32)
        let s_8_33: Bits = FPRSqrtEstimate(state, tracer, s_8_30, s_8_32);
        // D s_8_34: write-var ga#360353 <= s_8_33
        fn_state.ga_360353 = s_8_33;
        // N s_8_35: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var ga#360351:u64
        let s_9_0: u64 = fn_state.ga_360351;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 64u16);
        // D s_9_2: read-var e:i64
        let s_9_2: i64 = fn_state.e;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: read-var ga#360352:i64
        let s_9_4: i64 = fn_state.ga_360352;
        // D s_9_5: cast zx s_9_4 -> i
        let s_9_5: i128 = (i128::try_from(s_9_4).unwrap());
        // D s_9_6: read-var ga#360353:bv
        let s_9_6: Bits = fn_state.ga_360353;
        // D s_9_7: call Elem_set(s_9_1, s_9_3, s_9_5, s_9_6)
        let s_9_7: Bits = Elem_set(state, tracer, s_9_1, s_9_3, s_9_5, s_9_6);
        // D s_9_8: cast reint s_9_7 -> u64
        let s_9_8: u64 = (s_9_7.value() as u64);
        // D s_9_9: read-var ga#360354:i
        let s_9_9: i128 = fn_state.ga_360354;
        // D s_9_10: call D_set(s_9_9, s_9_8)
        let s_9_10: () = D_set(state, tracer, s_9_9, s_9_8);
        // N s_9_11: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var r:i64
        let s_10_0: i64 = fn_state.r;
        // C s_10_1: const #1s : i64
        let s_10_1: i64 = 1;
        // D s_10_2: add s_10_0 s_10_1
        let s_10_2: i64 = (s_10_0 + s_10_1);
        // D s_10_3: write-var r <= s_10_2
        fn_state.r = s_10_2;
        // N s_10_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: return
        return;
    }
}
