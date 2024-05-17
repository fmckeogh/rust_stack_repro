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
use FPMinNum::*;
use u__id::*;
use CheckSVEEnabled::*;
use FPCR_read::*;
use Elem_set::*;
use FPMaxNum::*;
use Elem_read::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_FCLAMP_Z_ZZ__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_276847: Bits,
        element2: Bits,
        esizeshadow_2411: i64,
        e: i64,
        gs_183867: bool,
        element1: Bits,
        VLshadow_2413: i64,
        element3: Bits,
        operand3: Bits,
        ga_276849: i64,
        ga_276850: Bits,
        VLshadow_2412: i64,
        result: Bits,
        operand1: Bits,
        gs_183869: bool,
        gs_183862: i64,
        operand2: Bits,
        VL: i64,
        d: i64,
        esize: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
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
        // D s_0_3: write-var esizeshadow#2411 <= s_0_2
        fn_state.esizeshadow_2411 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#2412 <= s_0_6
        fn_state.VLshadow_2412 = s_0_6;
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call CheckSVEEnabled(s_0_8)
        let s_0_9: () = CheckSVEEnabled(state, tracer, s_0_8);
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#2412:i64
        let s_1_0: i64 = fn_state.VLshadow_2412;
        // D s_1_1: write-var VLshadow#2413 <= s_1_0
        fn_state.VLshadow_2413 = s_1_0;
        // D s_1_2: read-var VLshadow#2413:i64
        let s_1_2: i64 = fn_state.VLshadow_2413;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: read-var esizeshadow#2411:i64
        let s_1_4: i64 = fn_state.esizeshadow_2411;
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: div s_1_3 s_1_5
        let s_1_6: i128 = ((s_1_3) / (s_1_5));
        // D s_1_7: cast reint s_1_6 -> i64
        let s_1_7: i64 = (s_1_6 as i64);
        // D s_1_8: read-var VLshadow#2413:i64
        let s_1_8: i64 = fn_state.VLshadow_2413;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: cast reint s_1_9 -> i64
        let s_1_10: i64 = (s_1_9 as i64);
        // D s_1_11: read-var n:i64
        let s_1_11: i64 = fn_state.n;
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_13: cast zx s_1_10 -> i
        let s_1_13: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_14: call Z_read(s_1_12, s_1_13)
        let s_1_14: Bits = Z_read(state, tracer, s_1_12, s_1_13);
        // D s_1_15: write-var operand1 <= s_1_14
        fn_state.operand1 = s_1_14;
        // D s_1_16: read-var VLshadow#2413:i64
        let s_1_16: i64 = fn_state.VLshadow_2413;
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_18: cast reint s_1_17 -> i64
        let s_1_18: i64 = (s_1_17 as i64);
        // D s_1_19: read-var m:i64
        let s_1_19: i64 = fn_state.m;
        // D s_1_20: cast zx s_1_19 -> i
        let s_1_20: i128 = (i128::try_from(s_1_19).unwrap());
        // D s_1_21: cast zx s_1_18 -> i
        let s_1_21: i128 = (i128::try_from(s_1_18).unwrap());
        // D s_1_22: call Z_read(s_1_20, s_1_21)
        let s_1_22: Bits = Z_read(state, tracer, s_1_20, s_1_21);
        // D s_1_23: write-var operand2 <= s_1_22
        fn_state.operand2 = s_1_22;
        // D s_1_24: read-var VLshadow#2413:i64
        let s_1_24: i64 = fn_state.VLshadow_2413;
        // D s_1_25: cast zx s_1_24 -> i
        let s_1_25: i128 = (i128::try_from(s_1_24).unwrap());
        // D s_1_26: cast reint s_1_25 -> i64
        let s_1_26: i64 = (s_1_25 as i64);
        // D s_1_27: read-var d:i64
        let s_1_27: i64 = fn_state.d;
        // D s_1_28: cast zx s_1_27 -> i
        let s_1_28: i128 = (i128::try_from(s_1_27).unwrap());
        // D s_1_29: cast zx s_1_26 -> i
        let s_1_29: i128 = (i128::try_from(s_1_26).unwrap());
        // D s_1_30: call Z_read(s_1_28, s_1_29)
        let s_1_30: Bits = Z_read(state, tracer, s_1_28, s_1_29);
        // D s_1_31: write-var operand3 <= s_1_30
        fn_state.operand3 = s_1_30;
        // C s_1_32: const #0s : i64
        let s_1_32: i64 = 0;
        // C s_1_33: const #1s : i
        let s_1_33: i128 = 1;
        // D s_1_34: cast zx s_1_7 -> i
        let s_1_34: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_35: sub s_1_34 s_1_33
        let s_1_35: i128 = ((s_1_34) - (s_1_33));
        // D s_1_36: cast reint s_1_35 -> i64
        let s_1_36: i64 = (s_1_35 as i64);
        // D s_1_37: write-var gs#183862 <= s_1_36
        fn_state.gs_183862 = s_1_36;
        // D s_1_38: write-var e <= s_1_32
        fn_state.e = s_1_32;
        // N s_1_39: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#183862:i64
        let s_2_1: i64 = fn_state.gs_183862;
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
        // D s_3_0: read-var esizeshadow#2411:i64
        let s_3_0: i64 = fn_state.esizeshadow_2411;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: cast reint s_3_1 -> i64
        let s_3_2: i64 = (s_3_1 as i64);
        // D s_3_3: read-var e:i64
        let s_3_3: i64 = fn_state.e;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: cast zx s_3_2 -> i
        let s_3_5: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_6: read-var operand1:bv
        let s_3_6: Bits = fn_state.operand1;
        // D s_3_7: call Elem_read(s_3_6, s_3_4, s_3_5)
        let s_3_7: Bits = Elem_read(state, tracer, s_3_6, s_3_4, s_3_5);
        // D s_3_8: write-var element1 <= s_3_7
        fn_state.element1 = s_3_7;
        // D s_3_9: read-var esizeshadow#2411:i64
        let s_3_9: i64 = fn_state.esizeshadow_2411;
        // D s_3_10: cast zx s_3_9 -> i
        let s_3_10: i128 = (i128::try_from(s_3_9).unwrap());
        // D s_3_11: cast reint s_3_10 -> i64
        let s_3_11: i64 = (s_3_10 as i64);
        // D s_3_12: read-var e:i64
        let s_3_12: i64 = fn_state.e;
        // D s_3_13: cast zx s_3_12 -> i
        let s_3_13: i128 = (i128::try_from(s_3_12).unwrap());
        // D s_3_14: cast zx s_3_11 -> i
        let s_3_14: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_15: read-var operand2:bv
        let s_3_15: Bits = fn_state.operand2;
        // D s_3_16: call Elem_read(s_3_15, s_3_13, s_3_14)
        let s_3_16: Bits = Elem_read(state, tracer, s_3_15, s_3_13, s_3_14);
        // D s_3_17: write-var element2 <= s_3_16
        fn_state.element2 = s_3_16;
        // D s_3_18: read-var esizeshadow#2411:i64
        let s_3_18: i64 = fn_state.esizeshadow_2411;
        // D s_3_19: cast zx s_3_18 -> i
        let s_3_19: i128 = (i128::try_from(s_3_18).unwrap());
        // D s_3_20: cast reint s_3_19 -> i64
        let s_3_20: i64 = (s_3_19 as i64);
        // D s_3_21: read-var e:i64
        let s_3_21: i64 = fn_state.e;
        // D s_3_22: cast zx s_3_21 -> i
        let s_3_22: i128 = (i128::try_from(s_3_21).unwrap());
        // D s_3_23: cast zx s_3_20 -> i
        let s_3_23: i128 = (i128::try_from(s_3_20).unwrap());
        // D s_3_24: read-var operand3:bv
        let s_3_24: Bits = fn_state.operand3;
        // D s_3_25: call Elem_read(s_3_24, s_3_22, s_3_23)
        let s_3_25: Bits = Elem_read(state, tracer, s_3_24, s_3_22, s_3_23);
        // D s_3_26: write-var element3 <= s_3_25
        fn_state.element3 = s_3_25;
        // D s_3_27: read-var esizeshadow#2411:i64
        let s_3_27: i64 = fn_state.esizeshadow_2411;
        // D s_3_28: cast zx s_3_27 -> i
        let s_3_28: i128 = (i128::try_from(s_3_27).unwrap());
        // D s_3_29: call __id(s_3_28)
        let s_3_29: i128 = u__id(state, tracer, s_3_28);
        // D s_3_30: cast reint s_3_29 -> i64
        let s_3_30: i64 = (s_3_29 as i64);
        // C s_3_31: const #16s : i
        let s_3_31: i128 = 16;
        // D s_3_32: cast zx s_3_30 -> i
        let s_3_32: i128 = (i128::try_from(s_3_30).unwrap());
        // D s_3_33: cmp-eq s_3_32 s_3_31
        let s_3_33: bool = ((s_3_32) == (s_3_31));
        // N s_3_34: branch s_3_33 b11 b4
        if s_3_33 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var esizeshadow#2411:i64
        let s_4_0: i64 = fn_state.esizeshadow_2411;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: call __id(s_4_1)
        let s_4_2: i128 = u__id(state, tracer, s_4_1);
        // D s_4_3: cast reint s_4_2 -> i64
        let s_4_3: i64 = (s_4_2 as i64);
        // C s_4_4: const #32s : i
        let s_4_4: i128 = 32;
        // D s_4_5: cast zx s_4_3 -> i
        let s_4_5: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_6: cmp-eq s_4_5 s_4_4
        let s_4_6: bool = ((s_4_5) == (s_4_4));
        // D s_4_7: write-var gs#183867 <= s_4_6
        fn_state.gs_183867 = s_4_6;
        // N s_4_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#183867:u8
        let s_5_0: bool = fn_state.gs_183867;
        // N s_5_1: branch s_5_0 b10 b6
        if s_5_0 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var esizeshadow#2411:i64
        let s_6_0: i64 = fn_state.esizeshadow_2411;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: call __id(s_6_1)
        let s_6_2: i128 = u__id(state, tracer, s_6_1);
        // D s_6_3: cast reint s_6_2 -> i64
        let s_6_3: i64 = (s_6_2 as i64);
        // C s_6_4: const #64s : i
        let s_6_4: i128 = 64;
        // D s_6_5: cast zx s_6_3 -> i
        let s_6_5: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_6: cmp-eq s_6_5 s_6_4
        let s_6_6: bool = ((s_6_5) == (s_6_4));
        // D s_6_7: write-var gs#183869 <= s_6_6
        fn_state.gs_183869 = s_6_6;
        // N s_6_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#183869:u8
        let s_7_0: bool = fn_state.gs_183869;
        // N s_7_1: assert s_7_0
        let s_7_1: () = assert!(s_7_0);
        // D s_7_2: read-var esizeshadow#2411:i64
        let s_7_2: i64 = fn_state.esizeshadow_2411;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: cast reint s_7_3 -> i64
        let s_7_4: i64 = (s_7_3 as i64);
        // D s_7_5: write-var ga#276849 <= s_7_4
        fn_state.ga_276849 = s_7_4;
        // C s_7_6: const #() : ()
        let s_7_6: () = ();
        // S s_7_7: call FPCR_read(s_7_6)
        let s_7_7: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_7_6);
        // D s_7_8: read-var element1:bv
        let s_7_8: Bits = fn_state.element1;
        // D s_7_9: read-var element3:bv
        let s_7_9: Bits = fn_state.element3;
        // D s_7_10: call FPMaxNum(s_7_8, s_7_9, s_7_7)
        let s_7_10: Bits = FPMaxNum(state, tracer, s_7_8, s_7_9, s_7_7);
        // D s_7_11: write-var ga#276847 <= s_7_10
        fn_state.ga_276847 = s_7_10;
        // N s_7_12: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call FPCR_read(s_8_0)
        let s_8_1: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_8_0);
        // D s_8_2: read-var ga#276847:bv
        let s_8_2: Bits = fn_state.ga_276847;
        // D s_8_3: read-var element2:bv
        let s_8_3: Bits = fn_state.element2;
        // D s_8_4: call FPMinNum(s_8_2, s_8_3, s_8_1)
        let s_8_4: Bits = FPMinNum(state, tracer, s_8_2, s_8_3, s_8_1);
        // D s_8_5: write-var ga#276850 <= s_8_4
        fn_state.ga_276850 = s_8_4;
        // N s_8_6: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var e:i64
        let s_9_0: i64 = fn_state.e;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: read-var ga#276849:i64
        let s_9_2: i64 = fn_state.ga_276849;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: read-var result:bv
        let s_9_4: Bits = fn_state.result;
        // D s_9_5: read-var ga#276850:bv
        let s_9_5: Bits = fn_state.ga_276850;
        // D s_9_6: call Elem_set(s_9_4, s_9_1, s_9_3, s_9_5)
        let s_9_6: Bits = Elem_set(state, tracer, s_9_4, s_9_1, s_9_3, s_9_5);
        // D s_9_7: write-var result <= s_9_6
        fn_state.result = s_9_6;
        // D s_9_8: read-var e:i64
        let s_9_8: i64 = fn_state.e;
        // C s_9_9: const #1s : i64
        let s_9_9: i64 = 1;
        // D s_9_10: add s_9_8 s_9_9
        let s_9_10: i64 = (s_9_8 + s_9_9);
        // D s_9_11: write-var e <= s_9_10
        fn_state.e = s_9_10;
        // N s_9_12: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #1u : u8
        let s_10_0: bool = true;
        // D s_10_1: write-var gs#183869 <= s_10_0
        fn_state.gs_183869 = s_10_0;
        // N s_10_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #1u : u8
        let s_11_0: bool = true;
        // D s_11_1: write-var gs#183867 <= s_11_0
        fn_state.gs_183867 = s_11_0;
        // N s_11_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var VLshadow#2413:i64
        let s_12_0: i64 = fn_state.VLshadow_2413;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: cast reint s_12_1 -> i64
        let s_12_2: i64 = (s_12_1 as i64);
        // D s_12_3: read-var d:i64
        let s_12_3: i64 = fn_state.d;
        // D s_12_4: cast zx s_12_3 -> i
        let s_12_4: i128 = (i128::try_from(s_12_3).unwrap());
        // D s_12_5: cast zx s_12_2 -> i
        let s_12_5: i128 = (i128::try_from(s_12_2).unwrap());
        // D s_12_6: read-var result:bv
        let s_12_6: Bits = fn_state.result;
        // D s_12_7: call Z_set(s_12_4, s_12_5, s_12_6)
        let s_12_7: () = Z_set(state, tracer, s_12_4, s_12_5, s_12_6);
        // N s_12_8: return
        return;
    }
}
