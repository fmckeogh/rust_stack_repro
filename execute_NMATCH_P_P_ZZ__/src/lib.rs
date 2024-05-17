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
use AnyActiveElement::*;
use ActivePredicateElement::*;
use P_read::*;
use PredTest::*;
use Zeros::*;
use Elem_read::*;
use CheckNonStreamingSVEEnabled::*;
use P_set::*;
use Z_read::*;
use common::*;
pub fn execute_NMATCH_P_P_ZZ__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    g: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        esizeshadow_4106: i64,
        VLshadow_4108: i64,
        VLshadow_4107: i64,
        PL: i64,
        mask: Bits,
        gs_216016: i64,
        element1: Bits,
        gs_216008: i64,
        psize: i64,
        elements: i64,
        result: Bits,
        i: i64,
        operand1: Bits,
        eltspersegment: i64,
        operand2: Bits,
        VL: i64,
        d: i64,
        esize: i64,
        g: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        g,
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
        // D s_0_3: write-var esizeshadow#4106 <= s_0_2
        fn_state.esizeshadow_4106 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#4107 <= s_0_6
        fn_state.VLshadow_4107 = s_0_6;
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call CheckNonStreamingSVEEnabled(s_0_8)
        let s_0_9: () = CheckNonStreamingSVEEnabled(state, tracer, s_0_8);
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#4107:i64
        let s_1_0: i64 = fn_state.VLshadow_4107;
        // D s_1_1: write-var VLshadow#4108 <= s_1_0
        fn_state.VLshadow_4108 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#4108:i64
        let s_1_3: i64 = fn_state.VLshadow_4108;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: write-var PL <= s_1_6
        fn_state.PL = s_1_6;
        // D s_1_8: read-var VLshadow#4108:i64
        let s_1_8: i64 = fn_state.VLshadow_4108;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: read-var esizeshadow#4106:i64
        let s_1_10: i64 = fn_state.esizeshadow_4106;
        // D s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_12: div s_1_9 s_1_11
        let s_1_12: i128 = ((s_1_9) / (s_1_11));
        // D s_1_13: cast reint s_1_12 -> i64
        let s_1_13: i64 = (s_1_12 as i64);
        // D s_1_14: write-var elements <= s_1_13
        fn_state.elements = s_1_13;
        // C s_1_15: const #128s : i
        let s_1_15: i128 = 128;
        // D s_1_16: read-var esizeshadow#4106:i64
        let s_1_16: i64 = fn_state.esizeshadow_4106;
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_18: div s_1_15 s_1_17
        let s_1_18: i128 = ((s_1_15) / (s_1_17));
        // D s_1_19: cast reint s_1_18 -> i64
        let s_1_19: i64 = (s_1_18 as i64);
        // D s_1_20: write-var eltspersegment <= s_1_19
        fn_state.eltspersegment = s_1_19;
        // D s_1_21: read-var PL:i64
        let s_1_21: i64 = fn_state.PL;
        // D s_1_22: cast zx s_1_21 -> i
        let s_1_22: i128 = (i128::try_from(s_1_21).unwrap());
        // D s_1_23: cast reint s_1_22 -> i64
        let s_1_23: i64 = (s_1_22 as i64);
        // D s_1_24: read-var g:i64
        let s_1_24: i64 = fn_state.g;
        // D s_1_25: cast zx s_1_24 -> i
        let s_1_25: i128 = (i128::try_from(s_1_24).unwrap());
        // D s_1_26: cast zx s_1_23 -> i
        let s_1_26: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_27: call P_read(s_1_25, s_1_26)
        let s_1_27: Bits = P_read(state, tracer, s_1_25, s_1_26);
        // D s_1_28: write-var mask <= s_1_27
        fn_state.mask = s_1_27;
        // D s_1_29: read-var esizeshadow#4106:i64
        let s_1_29: i64 = fn_state.esizeshadow_4106;
        // D s_1_30: cast zx s_1_29 -> i
        let s_1_30: i128 = (i128::try_from(s_1_29).unwrap());
        // D s_1_31: read-var mask:bv
        let s_1_31: Bits = fn_state.mask;
        // D s_1_32: call AnyActiveElement(s_1_31, s_1_30)
        let s_1_32: bool = AnyActiveElement(state, tracer, s_1_31, s_1_30);
        // N s_1_33: branch s_1_32 b19 b2
        if s_1_32 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var VLshadow#4108:i64
        let s_2_0: i64 = fn_state.VLshadow_4108;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: call Zeros(s_2_1)
        let s_2_2: Bits = Zeros(state, tracer, s_2_1);
        // D s_2_3: write-var operand1 <= s_2_2
        fn_state.operand1 = s_2_2;
        // N s_2_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var esizeshadow#4106:i64
        let s_3_0: i64 = fn_state.esizeshadow_4106;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var mask:bv
        let s_3_2: Bits = fn_state.mask;
        // D s_3_3: call AnyActiveElement(s_3_2, s_3_1)
        let s_3_3: bool = AnyActiveElement(state, tracer, s_3_2, s_3_1);
        // N s_3_4: branch s_3_3 b18 b4
        if s_3_3 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var VLshadow#4108:i64
        let s_4_0: i64 = fn_state.VLshadow_4108;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: call Zeros(s_4_1)
        let s_4_2: Bits = Zeros(state, tracer, s_4_1);
        // D s_4_3: write-var operand2 <= s_4_2
        fn_state.operand2 = s_4_2;
        // N s_4_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #8s : i
        let s_5_0: i128 = 8;
        // D s_5_1: read-var esizeshadow#4106:i64
        let s_5_1: i64 = fn_state.esizeshadow_4106;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: div s_5_2 s_5_0
        let s_5_3: i128 = ((s_5_2) / (s_5_0));
        // D s_5_4: cast reint s_5_3 -> i64
        let s_5_4: i64 = (s_5_3 as i64);
        // D s_5_5: write-var psize <= s_5_4
        fn_state.psize = s_5_4;
        // C s_5_6: const #0s : i64
        let s_5_6: i64 = 0;
        // C s_5_7: const #1s : i
        let s_5_7: i128 = 1;
        // D s_5_8: read-var elements:i64
        let s_5_8: i64 = fn_state.elements;
        // D s_5_9: cast zx s_5_8 -> i
        let s_5_9: i128 = (i128::try_from(s_5_8).unwrap());
        // D s_5_10: sub s_5_9 s_5_7
        let s_5_10: i128 = ((s_5_9) - (s_5_7));
        // D s_5_11: cast reint s_5_10 -> i64
        let s_5_11: i64 = (s_5_10 as i64);
        // D s_5_12: write-var gs#216008 <= s_5_11
        fn_state.gs_216008 = s_5_11;
        // D s_5_13: write-var e <= s_5_6
        fn_state.e = s_5_6;
        // N s_5_14: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var e:i64
        let s_6_0: i64 = fn_state.e;
        // D s_6_1: read-var gs#216008:i64
        let s_6_1: i64 = fn_state.gs_216008;
        // D s_6_2: cmp-gt s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) > (s_6_1));
        // N s_6_3: branch s_6_2 b17 b7
        if s_6_2 {
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
        // D s_7_0: read-var e:i64
        let s_7_0: i64 = fn_state.e;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: read-var esizeshadow#4106:i64
        let s_7_2: i64 = fn_state.esizeshadow_4106;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: read-var mask:bv
        let s_7_4: Bits = fn_state.mask;
        // D s_7_5: call ActivePredicateElement(s_7_4, s_7_1, s_7_3)
        let s_7_5: bool = ActivePredicateElement(state, tracer, s_7_4, s_7_1, s_7_3);
        // N s_7_6: branch s_7_5 b10 b8
        if s_7_5 {
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
        // D s_8_0: read-var psize:i64
        let s_8_0: i64 = fn_state.psize;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: cast reint s_8_1 -> i64
        let s_8_2: i64 = (s_8_1 as i64);
        // C s_8_3: const #0u : u8
        let s_8_3: bool = false;
        // C s_8_4: cast zx s_8_3 -> bv
        let s_8_4: Bits = Bits::new(s_8_3 as u128, 1u16);
        // D s_8_5: read-var psize:i64
        let s_8_5: i64 = fn_state.psize;
        // D s_8_6: cast zx s_8_5 -> i
        let s_8_6: i128 = (i128::try_from(s_8_5).unwrap());
        // D s_8_7: bits-cast zx s_8_4 -> bv length s_8_6
        let s_8_7: Bits = s_8_4.zero_extend(s_8_6);
        // D s_8_8: read-var e:i64
        let s_8_8: i64 = fn_state.e;
        // D s_8_9: cast zx s_8_8 -> i
        let s_8_9: i128 = (i128::try_from(s_8_8).unwrap());
        // D s_8_10: cast zx s_8_2 -> i
        let s_8_10: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_11: read-var result:bv
        let s_8_11: Bits = fn_state.result;
        // D s_8_12: call Elem_set(s_8_11, s_8_9, s_8_10, s_8_7)
        let s_8_12: Bits = Elem_set(state, tracer, s_8_11, s_8_9, s_8_10, s_8_7);
        // D s_8_13: write-var result <= s_8_12
        fn_state.result = s_8_12;
        // N s_8_14: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var e:i64
        let s_9_0: i64 = fn_state.e;
        // C s_9_1: const #1s : i64
        let s_9_1: i64 = 1;
        // D s_9_2: add s_9_0 s_9_1
        let s_9_2: i64 = (s_9_0 + s_9_1);
        // D s_9_3: write-var e <= s_9_2
        fn_state.e = s_9_2;
        // N s_9_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var e:i64
        let s_10_0: i64 = fn_state.e;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: read-var eltspersegment:i64
        let s_10_2: i64 = fn_state.eltspersegment;
        // D s_10_3: cast zx s_10_2 -> i
        let s_10_3: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_4: mod s_10_1 s_10_3
        let s_10_4: i128 = ((s_10_1) % (s_10_3));
        // D s_10_5: cast reint s_10_4 -> i64
        let s_10_5: i64 = (s_10_4 as i64);
        // D s_10_6: read-var e:i64
        let s_10_6: i64 = fn_state.e;
        // D s_10_7: cast zx s_10_6 -> i
        let s_10_7: i128 = (i128::try_from(s_10_6).unwrap());
        // D s_10_8: cast zx s_10_5 -> i
        let s_10_8: i128 = (i128::try_from(s_10_5).unwrap());
        // D s_10_9: sub s_10_7 s_10_8
        let s_10_9: i128 = ((s_10_7) - (s_10_8));
        // D s_10_10: cast reint s_10_9 -> i64
        let s_10_10: i64 = (s_10_9 as i64);
        // D s_10_11: read-var psize:i64
        let s_10_11: i64 = fn_state.psize;
        // D s_10_12: cast zx s_10_11 -> i
        let s_10_12: i128 = (i128::try_from(s_10_11).unwrap());
        // D s_10_13: cast reint s_10_12 -> i64
        let s_10_13: i64 = (s_10_12 as i64);
        // C s_10_14: const #1u : u8
        let s_10_14: bool = true;
        // C s_10_15: cast zx s_10_14 -> bv
        let s_10_15: Bits = Bits::new(s_10_14 as u128, 1u16);
        // D s_10_16: read-var psize:i64
        let s_10_16: i64 = fn_state.psize;
        // D s_10_17: cast zx s_10_16 -> i
        let s_10_17: i128 = (i128::try_from(s_10_16).unwrap());
        // D s_10_18: bits-cast zx s_10_15 -> bv length s_10_17
        let s_10_18: Bits = s_10_15.zero_extend(s_10_17);
        // D s_10_19: read-var e:i64
        let s_10_19: i64 = fn_state.e;
        // D s_10_20: cast zx s_10_19 -> i
        let s_10_20: i128 = (i128::try_from(s_10_19).unwrap());
        // D s_10_21: cast zx s_10_13 -> i
        let s_10_21: i128 = (i128::try_from(s_10_13).unwrap());
        // D s_10_22: read-var result:bv
        let s_10_22: Bits = fn_state.result;
        // D s_10_23: call Elem_set(s_10_22, s_10_20, s_10_21, s_10_18)
        let s_10_23: Bits = Elem_set(state, tracer, s_10_22, s_10_20, s_10_21, s_10_18);
        // D s_10_24: write-var result <= s_10_23
        fn_state.result = s_10_23;
        // D s_10_25: read-var esizeshadow#4106:i64
        let s_10_25: i64 = fn_state.esizeshadow_4106;
        // D s_10_26: cast zx s_10_25 -> i
        let s_10_26: i128 = (i128::try_from(s_10_25).unwrap());
        // D s_10_27: cast reint s_10_26 -> i64
        let s_10_27: i64 = (s_10_26 as i64);
        // D s_10_28: read-var e:i64
        let s_10_28: i64 = fn_state.e;
        // D s_10_29: cast zx s_10_28 -> i
        let s_10_29: i128 = (i128::try_from(s_10_28).unwrap());
        // D s_10_30: cast zx s_10_27 -> i
        let s_10_30: i128 = (i128::try_from(s_10_27).unwrap());
        // D s_10_31: read-var operand1:bv
        let s_10_31: Bits = fn_state.operand1;
        // D s_10_32: call Elem_read(s_10_31, s_10_29, s_10_30)
        let s_10_32: Bits = Elem_read(state, tracer, s_10_31, s_10_29, s_10_30);
        // D s_10_33: write-var element1 <= s_10_32
        fn_state.element1 = s_10_32;
        // D s_10_34: cast zx s_10_10 -> i
        let s_10_34: i128 = (i128::try_from(s_10_10).unwrap());
        // D s_10_35: read-var eltspersegment:i64
        let s_10_35: i64 = fn_state.eltspersegment;
        // D s_10_36: cast zx s_10_35 -> i
        let s_10_36: i128 = (i128::try_from(s_10_35).unwrap());
        // D s_10_37: add s_10_34 s_10_36
        let s_10_37: i128 = (s_10_34 + s_10_36);
        // D s_10_38: cast reint s_10_37 -> i64
        let s_10_38: i64 = (s_10_37 as i64);
        // C s_10_39: const #1s : i
        let s_10_39: i128 = 1;
        // D s_10_40: cast zx s_10_38 -> i
        let s_10_40: i128 = (i128::try_from(s_10_38).unwrap());
        // D s_10_41: sub s_10_40 s_10_39
        let s_10_41: i128 = ((s_10_40) - (s_10_39));
        // D s_10_42: cast reint s_10_41 -> i64
        let s_10_42: i64 = (s_10_41 as i64);
        // D s_10_43: write-var gs#216016 <= s_10_42
        fn_state.gs_216016 = s_10_42;
        // D s_10_44: write-var i <= s_10_10
        fn_state.i = s_10_10;
        // N s_10_45: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var i:i64
        let s_11_0: i64 = fn_state.i;
        // D s_11_1: read-var gs#216016:i64
        let s_11_1: i64 = fn_state.gs_216016;
        // D s_11_2: cmp-gt s_11_0 s_11_1
        let s_11_2: bool = ((s_11_0) > (s_11_1));
        // N s_11_3: branch s_11_2 b16 b12
        if s_11_2 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var esizeshadow#4106:i64
        let s_12_0: i64 = fn_state.esizeshadow_4106;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: cast reint s_12_1 -> i64
        let s_12_2: i64 = (s_12_1 as i64);
        // D s_12_3: read-var i:i64
        let s_12_3: i64 = fn_state.i;
        // D s_12_4: cast zx s_12_3 -> i
        let s_12_4: i128 = (i128::try_from(s_12_3).unwrap());
        // D s_12_5: cast zx s_12_2 -> i
        let s_12_5: i128 = (i128::try_from(s_12_2).unwrap());
        // D s_12_6: read-var operand2:bv
        let s_12_6: Bits = fn_state.operand2;
        // D s_12_7: call Elem_read(s_12_6, s_12_4, s_12_5)
        let s_12_7: Bits = Elem_read(state, tracer, s_12_6, s_12_4, s_12_5);
        // D s_12_8: read-var element1:bv
        let s_12_8: Bits = fn_state.element1;
        // D s_12_9: cmp-eq s_12_8 s_12_7
        let s_12_9: bool = ((s_12_8) == (s_12_7));
        // N s_12_10: branch s_12_9 b15 b13
        if s_12_9 {
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
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var i:i64
        let s_14_0: i64 = fn_state.i;
        // C s_14_1: const #1s : i64
        let s_14_1: i64 = 1;
        // D s_14_2: add s_14_0 s_14_1
        let s_14_2: i64 = (s_14_0 + s_14_1);
        // D s_14_3: write-var i <= s_14_2
        fn_state.i = s_14_2;
        // N s_14_4: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var psize:i64
        let s_15_0: i64 = fn_state.psize;
        // D s_15_1: cast zx s_15_0 -> i
        let s_15_1: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_2: cast reint s_15_1 -> i64
        let s_15_2: i64 = (s_15_1 as i64);
        // C s_15_3: const #0u : u8
        let s_15_3: bool = false;
        // C s_15_4: cast zx s_15_3 -> bv
        let s_15_4: Bits = Bits::new(s_15_3 as u128, 1u16);
        // D s_15_5: read-var psize:i64
        let s_15_5: i64 = fn_state.psize;
        // D s_15_6: cast zx s_15_5 -> i
        let s_15_6: i128 = (i128::try_from(s_15_5).unwrap());
        // D s_15_7: bits-cast zx s_15_4 -> bv length s_15_6
        let s_15_7: Bits = s_15_4.zero_extend(s_15_6);
        // D s_15_8: read-var e:i64
        let s_15_8: i64 = fn_state.e;
        // D s_15_9: cast zx s_15_8 -> i
        let s_15_9: i128 = (i128::try_from(s_15_8).unwrap());
        // D s_15_10: cast zx s_15_2 -> i
        let s_15_10: i128 = (i128::try_from(s_15_2).unwrap());
        // D s_15_11: read-var result:bv
        let s_15_11: Bits = fn_state.result;
        // D s_15_12: call Elem_set(s_15_11, s_15_9, s_15_10, s_15_7)
        let s_15_12: Bits = Elem_set(state, tracer, s_15_11, s_15_9, s_15_10, s_15_7);
        // D s_15_13: write-var result <= s_15_12
        fn_state.result = s_15_12;
        // N s_15_14: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var esizeshadow#4106:i64
        let s_17_0: i64 = fn_state.esizeshadow_4106;
        // D s_17_1: cast zx s_17_0 -> i
        let s_17_1: i128 = (i128::try_from(s_17_0).unwrap());
        // D s_17_2: read-var mask:bv
        let s_17_2: Bits = fn_state.mask;
        // D s_17_3: read-var result:bv
        let s_17_3: Bits = fn_state.result;
        // D s_17_4: call PredTest(s_17_2, s_17_3, s_17_1)
        let s_17_4: u8 = PredTest(state, tracer, s_17_2, s_17_3, s_17_1);
        // C s_17_5: const #3s : i
        let s_17_5: i128 = 3;
        // D s_17_6: cast zx s_17_4 -> bv
        let s_17_6: Bits = Bits::new(s_17_4 as u128, 4u16);
        // C s_17_7: const #1s : i64
        let s_17_7: i64 = 1;
        // C s_17_8: cast zx s_17_7 -> i
        let s_17_8: i128 = (i128::try_from(s_17_7).unwrap());
        // C s_17_9: const #0s : i
        let s_17_9: i128 = 0;
        // C s_17_10: add s_17_9 s_17_8
        let s_17_10: i128 = (s_17_9 + s_17_8);
        // D s_17_11: bit-extract s_17_6 s_17_5 s_17_10
        let s_17_11: Bits = (Bits::new(
            ((s_17_6) >> (s_17_5)).value(),
            u16::try_from(s_17_10).unwrap(),
        ));
        // D s_17_12: cast reint s_17_11 -> u8
        let s_17_12: bool = ((s_17_11.value()) != 0);
        // C s_17_13: const #16984u : u32
        let s_17_13: u32 = 16984;
        // N s_17_14: write-reg s_17_13 <= s_17_12
        let s_17_14: () = {
            state.write_register::<bool>(s_17_13 as isize, s_17_12);
            tracer.write_register(s_17_13 as isize, s_17_12);
        };
        // C s_17_15: const #2s : i
        let s_17_15: i128 = 2;
        // D s_17_16: cast zx s_17_4 -> bv
        let s_17_16: Bits = Bits::new(s_17_4 as u128, 4u16);
        // C s_17_17: const #1s : i64
        let s_17_17: i64 = 1;
        // C s_17_18: cast zx s_17_17 -> i
        let s_17_18: i128 = (i128::try_from(s_17_17).unwrap());
        // C s_17_19: const #0s : i
        let s_17_19: i128 = 0;
        // C s_17_20: add s_17_19 s_17_18
        let s_17_20: i128 = (s_17_19 + s_17_18);
        // D s_17_21: bit-extract s_17_16 s_17_15 s_17_20
        let s_17_21: Bits = (Bits::new(
            ((s_17_16) >> (s_17_15)).value(),
            u16::try_from(s_17_20).unwrap(),
        ));
        // D s_17_22: cast reint s_17_21 -> u8
        let s_17_22: bool = ((s_17_21.value()) != 0);
        // C s_17_23: const #16997u : u32
        let s_17_23: u32 = 16997;
        // N s_17_24: write-reg s_17_23 <= s_17_22
        let s_17_24: () = {
            state.write_register::<bool>(s_17_23 as isize, s_17_22);
            tracer.write_register(s_17_23 as isize, s_17_22);
        };
        // C s_17_25: const #1s : i
        let s_17_25: i128 = 1;
        // D s_17_26: cast zx s_17_4 -> bv
        let s_17_26: Bits = Bits::new(s_17_4 as u128, 4u16);
        // C s_17_27: const #1s : i64
        let s_17_27: i64 = 1;
        // C s_17_28: cast zx s_17_27 -> i
        let s_17_28: i128 = (i128::try_from(s_17_27).unwrap());
        // C s_17_29: const #0s : i
        let s_17_29: i128 = 0;
        // C s_17_30: add s_17_29 s_17_28
        let s_17_30: i128 = (s_17_29 + s_17_28);
        // D s_17_31: bit-extract s_17_26 s_17_25 s_17_30
        let s_17_31: Bits = (Bits::new(
            ((s_17_26) >> (s_17_25)).value(),
            u16::try_from(s_17_30).unwrap(),
        ));
        // D s_17_32: cast reint s_17_31 -> u8
        let s_17_32: bool = ((s_17_31.value()) != 0);
        // C s_17_33: const #16971u : u32
        let s_17_33: u32 = 16971;
        // N s_17_34: write-reg s_17_33 <= s_17_32
        let s_17_34: () = {
            state.write_register::<bool>(s_17_33 as isize, s_17_32);
            tracer.write_register(s_17_33 as isize, s_17_32);
        };
        // C s_17_35: const #0s : i
        let s_17_35: i128 = 0;
        // D s_17_36: cast zx s_17_4 -> bv
        let s_17_36: Bits = Bits::new(s_17_4 as u128, 4u16);
        // C s_17_37: const #1s : i64
        let s_17_37: i64 = 1;
        // C s_17_38: cast zx s_17_37 -> i
        let s_17_38: i128 = (i128::try_from(s_17_37).unwrap());
        // C s_17_39: const #0s : i
        let s_17_39: i128 = 0;
        // C s_17_40: add s_17_39 s_17_38
        let s_17_40: i128 = (s_17_39 + s_17_38);
        // D s_17_41: bit-extract s_17_36 s_17_35 s_17_40
        let s_17_41: Bits = (Bits::new(
            ((s_17_36) >> (s_17_35)).value(),
            u16::try_from(s_17_40).unwrap(),
        ));
        // D s_17_42: cast reint s_17_41 -> u8
        let s_17_42: bool = ((s_17_41.value()) != 0);
        // C s_17_43: const #16996u : u32
        let s_17_43: u32 = 16996;
        // N s_17_44: write-reg s_17_43 <= s_17_42
        let s_17_44: () = {
            state.write_register::<bool>(s_17_43 as isize, s_17_42);
            tracer.write_register(s_17_43 as isize, s_17_42);
        };
        // D s_17_45: read-var PL:i64
        let s_17_45: i64 = fn_state.PL;
        // D s_17_46: cast zx s_17_45 -> i
        let s_17_46: i128 = (i128::try_from(s_17_45).unwrap());
        // D s_17_47: cast reint s_17_46 -> i64
        let s_17_47: i64 = (s_17_46 as i64);
        // D s_17_48: read-var d:i64
        let s_17_48: i64 = fn_state.d;
        // D s_17_49: cast zx s_17_48 -> i
        let s_17_49: i128 = (i128::try_from(s_17_48).unwrap());
        // D s_17_50: cast zx s_17_47 -> i
        let s_17_50: i128 = (i128::try_from(s_17_47).unwrap());
        // D s_17_51: read-var result:bv
        let s_17_51: Bits = fn_state.result;
        // D s_17_52: call P_set(s_17_49, s_17_50, s_17_51)
        let s_17_52: () = P_set(state, tracer, s_17_49, s_17_50, s_17_51);
        // N s_17_53: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var VLshadow#4108:i64
        let s_18_0: i64 = fn_state.VLshadow_4108;
        // D s_18_1: cast zx s_18_0 -> i
        let s_18_1: i128 = (i128::try_from(s_18_0).unwrap());
        // D s_18_2: cast reint s_18_1 -> i64
        let s_18_2: i64 = (s_18_1 as i64);
        // D s_18_3: read-var m:i64
        let s_18_3: i64 = fn_state.m;
        // D s_18_4: cast zx s_18_3 -> i
        let s_18_4: i128 = (i128::try_from(s_18_3).unwrap());
        // D s_18_5: cast zx s_18_2 -> i
        let s_18_5: i128 = (i128::try_from(s_18_2).unwrap());
        // D s_18_6: call Z_read(s_18_4, s_18_5)
        let s_18_6: Bits = Z_read(state, tracer, s_18_4, s_18_5);
        // D s_18_7: write-var operand2 <= s_18_6
        fn_state.operand2 = s_18_6;
        // N s_18_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var VLshadow#4108:i64
        let s_19_0: i64 = fn_state.VLshadow_4108;
        // D s_19_1: cast zx s_19_0 -> i
        let s_19_1: i128 = (i128::try_from(s_19_0).unwrap());
        // D s_19_2: cast reint s_19_1 -> i64
        let s_19_2: i64 = (s_19_1 as i64);
        // D s_19_3: read-var n:i64
        let s_19_3: i64 = fn_state.n;
        // D s_19_4: cast zx s_19_3 -> i
        let s_19_4: i128 = (i128::try_from(s_19_3).unwrap());
        // D s_19_5: cast zx s_19_2 -> i
        let s_19_5: i128 = (i128::try_from(s_19_2).unwrap());
        // D s_19_6: call Z_read(s_19_4, s_19_5)
        let s_19_6: Bits = Z_read(state, tracer, s_19_4, s_19_5);
        // D s_19_7: write-var operand1 <= s_19_6
        fn_state.operand1 = s_19_6;
        // N s_19_8: jump b3
        return block_3(state, tracer, fn_state);
    }
}
