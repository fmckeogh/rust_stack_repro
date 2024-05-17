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
use Elem_set::*;
use u__id::*;
use CheckSVEEnabled::*;
use AnyActiveElement::*;
use ActivePredicateElement::*;
use P_read::*;
use FPCR_read::*;
use Elem_read::*;
use Zeros::*;
use Z_read::*;
use FPRoundInt::*;
use Z_set::*;
use common::*;
pub fn execute_FRINTN_Z_P_Z__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    exact: bool,
    g: i64,
    n: i64,
    rounding: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        e: i64,
        VLshadow_2138: i64,
        gs_177640: bool,
        gs_177642: bool,
        esizeshadow_2136: i64,
        elements: i64,
        ga_272451: i64,
        element: Bits,
        result: Bits,
        VLshadow_2137: i64,
        gs_177634: i64,
        mask: Bits,
        ga_272452: Bits,
        VL: i64,
        d: i64,
        esize: i64,
        exact: bool,
        g: i64,
        n: i64,
        rounding: u32,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        exact,
        g,
        n,
        rounding,
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
        // D s_0_3: write-var esizeshadow#2136 <= s_0_2
        fn_state.esizeshadow_2136 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#2137 <= s_0_6
        fn_state.VLshadow_2137 = s_0_6;
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
        // D s_1_0: read-var VLshadow#2137:i64
        let s_1_0: i64 = fn_state.VLshadow_2137;
        // D s_1_1: write-var VLshadow#2138 <= s_1_0
        fn_state.VLshadow_2138 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#2138:i64
        let s_1_3: i64 = fn_state.VLshadow_2138;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#2138:i64
        let s_1_7: i64 = fn_state.VLshadow_2138;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: read-var esizeshadow#2136:i64
        let s_1_9: i64 = fn_state.esizeshadow_2136;
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_11: div s_1_8 s_1_10
        let s_1_11: i128 = ((s_1_8) / (s_1_10));
        // D s_1_12: cast reint s_1_11 -> i64
        let s_1_12: i64 = (s_1_11 as i64);
        // D s_1_13: write-var elements <= s_1_12
        fn_state.elements = s_1_12;
        // D s_1_14: cast zx s_1_6 -> i
        let s_1_14: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_15: cast reint s_1_14 -> i64
        let s_1_15: i64 = (s_1_14 as i64);
        // D s_1_16: read-var g:i64
        let s_1_16: i64 = fn_state.g;
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_18: cast zx s_1_15 -> i
        let s_1_18: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_19: call P_read(s_1_17, s_1_18)
        let s_1_19: Bits = P_read(state, tracer, s_1_17, s_1_18);
        // D s_1_20: write-var mask <= s_1_19
        fn_state.mask = s_1_19;
        // D s_1_21: read-var esizeshadow#2136:i64
        let s_1_21: i64 = fn_state.esizeshadow_2136;
        // D s_1_22: cast zx s_1_21 -> i
        let s_1_22: i128 = (i128::try_from(s_1_21).unwrap());
        // D s_1_23: read-var mask:bv
        let s_1_23: Bits = fn_state.mask;
        // D s_1_24: call AnyActiveElement(s_1_23, s_1_22)
        let s_1_24: bool = AnyActiveElement(state, tracer, s_1_23, s_1_22);
        // N s_1_25: branch s_1_24 b17 b2
        if s_1_24 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var VLshadow#2138:i64
        let s_2_0: i64 = fn_state.VLshadow_2138;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: call Zeros(s_2_1)
        let s_2_2: Bits = Zeros(state, tracer, s_2_1);
        // D s_2_3: write-var operand <= s_2_2
        fn_state.operand = s_2_2;
        // N s_2_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var VLshadow#2138:i64
        let s_3_0: i64 = fn_state.VLshadow_2138;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: cast reint s_3_1 -> i64
        let s_3_2: i64 = (s_3_1 as i64);
        // D s_3_3: read-var d:i64
        let s_3_3: i64 = fn_state.d;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: cast zx s_3_2 -> i
        let s_3_5: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_6: call Z_read(s_3_4, s_3_5)
        let s_3_6: Bits = Z_read(state, tracer, s_3_4, s_3_5);
        // D s_3_7: write-var result <= s_3_6
        fn_state.result = s_3_6;
        // C s_3_8: const #0s : i64
        let s_3_8: i64 = 0;
        // C s_3_9: const #1s : i
        let s_3_9: i128 = 1;
        // D s_3_10: read-var elements:i64
        let s_3_10: i64 = fn_state.elements;
        // D s_3_11: cast zx s_3_10 -> i
        let s_3_11: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_12: sub s_3_11 s_3_9
        let s_3_12: i128 = ((s_3_11) - (s_3_9));
        // D s_3_13: cast reint s_3_12 -> i64
        let s_3_13: i64 = (s_3_12 as i64);
        // D s_3_14: write-var gs#177634 <= s_3_13
        fn_state.gs_177634 = s_3_13;
        // D s_3_15: write-var e <= s_3_8
        fn_state.e = s_3_8;
        // N s_3_16: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#177634:i64
        let s_4_1: i64 = fn_state.gs_177634;
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
        // D s_5_0: read-var e:i64
        let s_5_0: i64 = fn_state.e;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: read-var esizeshadow#2136:i64
        let s_5_2: i64 = fn_state.esizeshadow_2136;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: read-var mask:bv
        let s_5_4: Bits = fn_state.mask;
        // D s_5_5: call ActivePredicateElement(s_5_4, s_5_1, s_5_3)
        let s_5_5: bool = ActivePredicateElement(state, tracer, s_5_4, s_5_1, s_5_3);
        // N s_5_6: branch s_5_5 b8 b6
        if s_5_5 {
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
        // N s_6_0: jump b7
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
        // D s_8_0: read-var esizeshadow#2136:i64
        let s_8_0: i64 = fn_state.esizeshadow_2136;
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
        // D s_8_6: read-var operand:bv
        let s_8_6: Bits = fn_state.operand;
        // D s_8_7: call Elem_read(s_8_6, s_8_4, s_8_5)
        let s_8_7: Bits = Elem_read(state, tracer, s_8_6, s_8_4, s_8_5);
        // D s_8_8: write-var element <= s_8_7
        fn_state.element = s_8_7;
        // D s_8_9: read-var esizeshadow#2136:i64
        let s_8_9: i64 = fn_state.esizeshadow_2136;
        // D s_8_10: cast zx s_8_9 -> i
        let s_8_10: i128 = (i128::try_from(s_8_9).unwrap());
        // D s_8_11: call __id(s_8_10)
        let s_8_11: i128 = u__id(state, tracer, s_8_10);
        // D s_8_12: cast reint s_8_11 -> i64
        let s_8_12: i64 = (s_8_11 as i64);
        // C s_8_13: const #16s : i
        let s_8_13: i128 = 16;
        // D s_8_14: cast zx s_8_12 -> i
        let s_8_14: i128 = (i128::try_from(s_8_12).unwrap());
        // D s_8_15: cmp-eq s_8_14 s_8_13
        let s_8_15: bool = ((s_8_14) == (s_8_13));
        // N s_8_16: branch s_8_15 b15 b9
        if s_8_15 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var esizeshadow#2136:i64
        let s_9_0: i64 = fn_state.esizeshadow_2136;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: call __id(s_9_1)
        let s_9_2: i128 = u__id(state, tracer, s_9_1);
        // D s_9_3: cast reint s_9_2 -> i64
        let s_9_3: i64 = (s_9_2 as i64);
        // C s_9_4: const #32s : i
        let s_9_4: i128 = 32;
        // D s_9_5: cast zx s_9_3 -> i
        let s_9_5: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_6: cmp-eq s_9_5 s_9_4
        let s_9_6: bool = ((s_9_5) == (s_9_4));
        // D s_9_7: write-var gs#177640 <= s_9_6
        fn_state.gs_177640 = s_9_6;
        // N s_9_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#177640:u8
        let s_10_0: bool = fn_state.gs_177640;
        // N s_10_1: branch s_10_0 b14 b11
        if s_10_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var esizeshadow#2136:i64
        let s_11_0: i64 = fn_state.esizeshadow_2136;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: call __id(s_11_1)
        let s_11_2: i128 = u__id(state, tracer, s_11_1);
        // D s_11_3: cast reint s_11_2 -> i64
        let s_11_3: i64 = (s_11_2 as i64);
        // C s_11_4: const #64s : i
        let s_11_4: i128 = 64;
        // D s_11_5: cast zx s_11_3 -> i
        let s_11_5: i128 = (i128::try_from(s_11_3).unwrap());
        // D s_11_6: cmp-eq s_11_5 s_11_4
        let s_11_6: bool = ((s_11_5) == (s_11_4));
        // D s_11_7: write-var gs#177642 <= s_11_6
        fn_state.gs_177642 = s_11_6;
        // N s_11_8: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#177642:u8
        let s_12_0: bool = fn_state.gs_177642;
        // N s_12_1: assert s_12_0
        let s_12_1: () = assert!(s_12_0);
        // D s_12_2: read-var esizeshadow#2136:i64
        let s_12_2: i64 = fn_state.esizeshadow_2136;
        // D s_12_3: cast zx s_12_2 -> i
        let s_12_3: i128 = (i128::try_from(s_12_2).unwrap());
        // D s_12_4: cast reint s_12_3 -> i64
        let s_12_4: i64 = (s_12_3 as i64);
        // D s_12_5: write-var ga#272451 <= s_12_4
        fn_state.ga_272451 = s_12_4;
        // C s_12_6: const #() : ()
        let s_12_6: () = ();
        // S s_12_7: call FPCR_read(s_12_6)
        let s_12_7: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_12_6);
        // D s_12_8: read-var element:bv
        let s_12_8: Bits = fn_state.element;
        // D s_12_9: read-var rounding:u32
        let s_12_9: u32 = fn_state.rounding;
        // D s_12_10: read-var exact:u8
        let s_12_10: bool = fn_state.exact;
        // D s_12_11: call FPRoundInt(s_12_8, s_12_7, s_12_9, s_12_10)
        let s_12_11: Bits = FPRoundInt(state, tracer, s_12_8, s_12_7, s_12_9, s_12_10);
        // D s_12_12: write-var ga#272452 <= s_12_11
        fn_state.ga_272452 = s_12_11;
        // N s_12_13: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var e:i64
        let s_13_0: i64 = fn_state.e;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: read-var ga#272451:i64
        let s_13_2: i64 = fn_state.ga_272451;
        // D s_13_3: cast zx s_13_2 -> i
        let s_13_3: i128 = (i128::try_from(s_13_2).unwrap());
        // D s_13_4: read-var result:bv
        let s_13_4: Bits = fn_state.result;
        // D s_13_5: read-var ga#272452:bv
        let s_13_5: Bits = fn_state.ga_272452;
        // D s_13_6: call Elem_set(s_13_4, s_13_1, s_13_3, s_13_5)
        let s_13_6: Bits = Elem_set(state, tracer, s_13_4, s_13_1, s_13_3, s_13_5);
        // D s_13_7: write-var result <= s_13_6
        fn_state.result = s_13_6;
        // N s_13_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var gs#177642 <= s_14_0
        fn_state.gs_177642 = s_14_0;
        // N s_14_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var gs#177640 <= s_15_0
        fn_state.gs_177640 = s_15_0;
        // N s_15_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var VLshadow#2138:i64
        let s_16_0: i64 = fn_state.VLshadow_2138;
        // D s_16_1: cast zx s_16_0 -> i
        let s_16_1: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_2: cast reint s_16_1 -> i64
        let s_16_2: i64 = (s_16_1 as i64);
        // D s_16_3: read-var d:i64
        let s_16_3: i64 = fn_state.d;
        // D s_16_4: cast zx s_16_3 -> i
        let s_16_4: i128 = (i128::try_from(s_16_3).unwrap());
        // D s_16_5: cast zx s_16_2 -> i
        let s_16_5: i128 = (i128::try_from(s_16_2).unwrap());
        // D s_16_6: read-var result:bv
        let s_16_6: Bits = fn_state.result;
        // D s_16_7: call Z_set(s_16_4, s_16_5, s_16_6)
        let s_16_7: () = Z_set(state, tracer, s_16_4, s_16_5, s_16_6);
        // N s_16_8: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var VLshadow#2138:i64
        let s_17_0: i64 = fn_state.VLshadow_2138;
        // D s_17_1: cast zx s_17_0 -> i
        let s_17_1: i128 = (i128::try_from(s_17_0).unwrap());
        // D s_17_2: cast reint s_17_1 -> i64
        let s_17_2: i64 = (s_17_1 as i64);
        // D s_17_3: read-var n:i64
        let s_17_3: i64 = fn_state.n;
        // D s_17_4: cast zx s_17_3 -> i
        let s_17_4: i128 = (i128::try_from(s_17_3).unwrap());
        // D s_17_5: cast zx s_17_2 -> i
        let s_17_5: i128 = (i128::try_from(s_17_2).unwrap());
        // D s_17_6: call Z_read(s_17_4, s_17_5)
        let s_17_6: Bits = Z_read(state, tracer, s_17_4, s_17_5);
        // D s_17_7: write-var operand <= s_17_6
        fn_state.operand = s_17_6;
        // N s_17_8: jump b3
        return block_3(state, tracer, fn_state);
    }
}
