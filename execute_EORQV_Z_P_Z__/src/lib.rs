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
use V_set::*;
use CheckSVEEnabled::*;
use AnyActiveElement::*;
use P_read::*;
use ActivePredicateElement::*;
use Elem_read::*;
use Zeros::*;
use Z_read::*;
use common::*;
pub fn execute_EORQV_Z_P_Z__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    g: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        e: i64,
        VLshadow_4377: i64,
        gs_220720: i64,
        gs_220726: i64,
        mask: Bits,
        VLshadow_4376: i64,
        s: i64,
        elempersegment: i64,
        esizeshadow_4375: i64,
        stmp: u128,
        dtmp: Bits,
        segments: i64,
        result: u128,
        VL: i64,
        d: i64,
        esize: i64,
        g: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        g,
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
        // D s_0_3: write-var esizeshadow#4375 <= s_0_2
        fn_state.esizeshadow_4375 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#4376 <= s_0_6
        fn_state.VLshadow_4376 = s_0_6;
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
        // D s_1_0: read-var VLshadow#4376:i64
        let s_1_0: i64 = fn_state.VLshadow_4376;
        // D s_1_1: write-var VLshadow#4377 <= s_1_0
        fn_state.VLshadow_4377 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#4377:i64
        let s_1_3: i64 = fn_state.VLshadow_4377;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // C s_1_7: const #128s : i
        let s_1_7: i128 = 128;
        // D s_1_8: read-var VLshadow#4377:i64
        let s_1_8: i64 = fn_state.VLshadow_4377;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: div s_1_9 s_1_7
        let s_1_10: i128 = ((s_1_9) / (s_1_7));
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: write-var segments <= s_1_11
        fn_state.segments = s_1_11;
        // C s_1_13: const #128s : i
        let s_1_13: i128 = 128;
        // D s_1_14: read-var esizeshadow#4375:i64
        let s_1_14: i64 = fn_state.esizeshadow_4375;
        // D s_1_15: cast zx s_1_14 -> i
        let s_1_15: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_16: div s_1_13 s_1_15
        let s_1_16: i128 = ((s_1_13) / (s_1_15));
        // D s_1_17: cast reint s_1_16 -> i64
        let s_1_17: i64 = (s_1_16 as i64);
        // D s_1_18: write-var elempersegment <= s_1_17
        fn_state.elempersegment = s_1_17;
        // D s_1_19: cast zx s_1_6 -> i
        let s_1_19: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_20: cast reint s_1_19 -> i64
        let s_1_20: i64 = (s_1_19 as i64);
        // D s_1_21: read-var g:i64
        let s_1_21: i64 = fn_state.g;
        // D s_1_22: cast zx s_1_21 -> i
        let s_1_22: i128 = (i128::try_from(s_1_21).unwrap());
        // D s_1_23: cast zx s_1_20 -> i
        let s_1_23: i128 = (i128::try_from(s_1_20).unwrap());
        // D s_1_24: call P_read(s_1_22, s_1_23)
        let s_1_24: Bits = P_read(state, tracer, s_1_22, s_1_23);
        // D s_1_25: write-var mask <= s_1_24
        fn_state.mask = s_1_24;
        // D s_1_26: read-var esizeshadow#4375:i64
        let s_1_26: i64 = fn_state.esizeshadow_4375;
        // D s_1_27: cast zx s_1_26 -> i
        let s_1_27: i128 = (i128::try_from(s_1_26).unwrap());
        // D s_1_28: read-var mask:bv
        let s_1_28: Bits = fn_state.mask;
        // D s_1_29: call AnyActiveElement(s_1_28, s_1_27)
        let s_1_29: bool = AnyActiveElement(state, tracer, s_1_28, s_1_27);
        // N s_1_30: branch s_1_29 b13 b2
        if s_1_29 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var VLshadow#4377:i64
        let s_2_0: i64 = fn_state.VLshadow_4377;
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
        // C s_3_0: const #0u : u8
        let s_3_0: u8 = 0;
        // C s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 0u16);
        // C s_3_2: const #0u : u64
        let s_3_2: u64 = 0;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 64u16);
        // C s_3_4: cast reint s_3_1 -> u128
        let s_3_4: u128 = (s_3_1.value() as u128);
        // D s_3_5: size-of s_3_1
        let s_3_5: u16 = s_3_1.length();
        // C s_3_6: cast reint s_3_3 -> u128
        let s_3_6: u128 = (s_3_3.value() as u128);
        // D s_3_7: size-of s_3_3
        let s_3_7: u16 = s_3_3.length();
        // D s_3_8: lsl s_3_4 s_3_7
        let s_3_8: u128 = s_3_4 << s_3_7;
        // D s_3_9: or s_3_8 s_3_6
        let s_3_9: u128 = ((s_3_8) | (s_3_6));
        // D s_3_10: add s_3_5 s_3_7
        let s_3_10: u16 = (s_3_5 + s_3_7);
        // D s_3_11: create-bits s_3_9 s_3_10
        let s_3_11: Bits = Bits::new(s_3_9, s_3_10);
        // C s_3_12: const #0u : u64
        let s_3_12: u64 = 0;
        // C s_3_13: cast zx s_3_12 -> bv
        let s_3_13: Bits = Bits::new(s_3_12 as u128, 64u16);
        // D s_3_14: cast reint s_3_11 -> u128
        let s_3_14: u128 = (s_3_11.value() as u128);
        // D s_3_15: size-of s_3_11
        let s_3_15: u16 = s_3_11.length();
        // C s_3_16: cast reint s_3_13 -> u128
        let s_3_16: u128 = (s_3_13.value() as u128);
        // D s_3_17: size-of s_3_13
        let s_3_17: u16 = s_3_13.length();
        // D s_3_18: lsl s_3_14 s_3_17
        let s_3_18: u128 = s_3_14 << s_3_17;
        // D s_3_19: or s_3_18 s_3_16
        let s_3_19: u128 = ((s_3_18) | (s_3_16));
        // D s_3_20: add s_3_15 s_3_17
        let s_3_20: u16 = (s_3_15 + s_3_17);
        // D s_3_21: create-bits s_3_19 s_3_20
        let s_3_21: Bits = Bits::new(s_3_19, s_3_20);
        // D s_3_22: cast reint s_3_21 -> u128
        let s_3_22: u128 = (s_3_21.value() as u128);
        // D s_3_23: write-var result <= s_3_22
        fn_state.result = s_3_22;
        // C s_3_24: const #0u : u8
        let s_3_24: u8 = 0;
        // C s_3_25: cast zx s_3_24 -> bv
        let s_3_25: Bits = Bits::new(s_3_24 as u128, 0u16);
        // C s_3_26: const #0u : u64
        let s_3_26: u64 = 0;
        // C s_3_27: cast zx s_3_26 -> bv
        let s_3_27: Bits = Bits::new(s_3_26 as u128, 64u16);
        // C s_3_28: cast reint s_3_25 -> u128
        let s_3_28: u128 = (s_3_25.value() as u128);
        // D s_3_29: size-of s_3_25
        let s_3_29: u16 = s_3_25.length();
        // C s_3_30: cast reint s_3_27 -> u128
        let s_3_30: u128 = (s_3_27.value() as u128);
        // D s_3_31: size-of s_3_27
        let s_3_31: u16 = s_3_27.length();
        // D s_3_32: lsl s_3_28 s_3_31
        let s_3_32: u128 = s_3_28 << s_3_31;
        // D s_3_33: or s_3_32 s_3_30
        let s_3_33: u128 = ((s_3_32) | (s_3_30));
        // D s_3_34: add s_3_29 s_3_31
        let s_3_34: u16 = (s_3_29 + s_3_31);
        // D s_3_35: create-bits s_3_33 s_3_34
        let s_3_35: Bits = Bits::new(s_3_33, s_3_34);
        // C s_3_36: const #0u : u64
        let s_3_36: u64 = 0;
        // C s_3_37: cast zx s_3_36 -> bv
        let s_3_37: Bits = Bits::new(s_3_36 as u128, 64u16);
        // D s_3_38: cast reint s_3_35 -> u128
        let s_3_38: u128 = (s_3_35.value() as u128);
        // D s_3_39: size-of s_3_35
        let s_3_39: u16 = s_3_35.length();
        // C s_3_40: cast reint s_3_37 -> u128
        let s_3_40: u128 = (s_3_37.value() as u128);
        // D s_3_41: size-of s_3_37
        let s_3_41: u16 = s_3_37.length();
        // D s_3_42: lsl s_3_38 s_3_41
        let s_3_42: u128 = s_3_38 << s_3_41;
        // D s_3_43: or s_3_42 s_3_40
        let s_3_43: u128 = ((s_3_42) | (s_3_40));
        // D s_3_44: add s_3_39 s_3_41
        let s_3_44: u16 = (s_3_39 + s_3_41);
        // D s_3_45: create-bits s_3_43 s_3_44
        let s_3_45: Bits = Bits::new(s_3_43, s_3_44);
        // D s_3_46: cast reint s_3_45 -> u128
        let s_3_46: u128 = (s_3_45.value() as u128);
        // D s_3_47: write-var stmp <= s_3_46
        fn_state.stmp = s_3_46;
        // C s_3_48: const #0s : i64
        let s_3_48: i64 = 0;
        // C s_3_49: const #1s : i
        let s_3_49: i128 = 1;
        // D s_3_50: read-var elempersegment:i64
        let s_3_50: i64 = fn_state.elempersegment;
        // D s_3_51: cast zx s_3_50 -> i
        let s_3_51: i128 = (i128::try_from(s_3_50).unwrap());
        // D s_3_52: sub s_3_51 s_3_49
        let s_3_52: i128 = ((s_3_51) - (s_3_49));
        // D s_3_53: cast reint s_3_52 -> i64
        let s_3_53: i64 = (s_3_52 as i64);
        // D s_3_54: write-var gs#220720 <= s_3_53
        fn_state.gs_220720 = s_3_53;
        // D s_3_55: write-var e <= s_3_48
        fn_state.e = s_3_48;
        // N s_3_56: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#220720:i64
        let s_4_1: i64 = fn_state.gs_220720;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b12 b5
        if s_4_2 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var esizeshadow#4375:i64
        let s_5_0: i64 = fn_state.esizeshadow_4375;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: call Zeros(s_5_1)
        let s_5_2: Bits = Zeros(state, tracer, s_5_1);
        // D s_5_3: write-var dtmp <= s_5_2
        fn_state.dtmp = s_5_2;
        // C s_5_4: const #0s : i64
        let s_5_4: i64 = 0;
        // C s_5_5: const #1s : i
        let s_5_5: i128 = 1;
        // D s_5_6: read-var segments:i64
        let s_5_6: i64 = fn_state.segments;
        // D s_5_7: cast zx s_5_6 -> i
        let s_5_7: i128 = (i128::try_from(s_5_6).unwrap());
        // D s_5_8: sub s_5_7 s_5_5
        let s_5_8: i128 = ((s_5_7) - (s_5_5));
        // D s_5_9: cast reint s_5_8 -> i64
        let s_5_9: i64 = (s_5_8 as i64);
        // D s_5_10: write-var gs#220726 <= s_5_9
        fn_state.gs_220726 = s_5_9;
        // D s_5_11: write-var s <= s_5_4
        fn_state.s = s_5_4;
        // N s_5_12: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var s:i64
        let s_6_0: i64 = fn_state.s;
        // D s_6_1: read-var gs#220726:i64
        let s_6_1: i64 = fn_state.gs_220726;
        // D s_6_2: cmp-gt s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) > (s_6_1));
        // N s_6_3: branch s_6_2 b11 b7
        if s_6_2 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var s:i64
        let s_7_0: i64 = fn_state.s;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: read-var elempersegment:i64
        let s_7_2: i64 = fn_state.elempersegment;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: mul s_7_1 s_7_3
        let s_7_4: i128 = ((s_7_1) * (s_7_3));
        // D s_7_5: cast reint s_7_4 -> i64
        let s_7_5: i64 = (s_7_4 as i64);
        // D s_7_6: cast zx s_7_5 -> i
        let s_7_6: i128 = (i128::try_from(s_7_5).unwrap());
        // D s_7_7: read-var e:i64
        let s_7_7: i64 = fn_state.e;
        // D s_7_8: cast zx s_7_7 -> i
        let s_7_8: i128 = (i128::try_from(s_7_7).unwrap());
        // D s_7_9: add s_7_6 s_7_8
        let s_7_9: i128 = (s_7_6 + s_7_8);
        // D s_7_10: cast reint s_7_9 -> i64
        let s_7_10: i64 = (s_7_9 as i64);
        // D s_7_11: cast zx s_7_10 -> i
        let s_7_11: i128 = (i128::try_from(s_7_10).unwrap());
        // D s_7_12: read-var esizeshadow#4375:i64
        let s_7_12: i64 = fn_state.esizeshadow_4375;
        // D s_7_13: cast zx s_7_12 -> i
        let s_7_13: i128 = (i128::try_from(s_7_12).unwrap());
        // D s_7_14: read-var mask:bv
        let s_7_14: Bits = fn_state.mask;
        // D s_7_15: call ActivePredicateElement(s_7_14, s_7_11, s_7_13)
        let s_7_15: bool = ActivePredicateElement(state, tracer, s_7_14, s_7_11, s_7_13);
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
        // N s_8_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var s:i64
        let s_9_0: i64 = fn_state.s;
        // C s_9_1: const #1s : i64
        let s_9_1: i64 = 1;
        // D s_9_2: add s_9_0 s_9_1
        let s_9_2: i64 = (s_9_0 + s_9_1);
        // D s_9_3: write-var s <= s_9_2
        fn_state.s = s_9_2;
        // N s_9_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #128s : i64
        let s_10_0: i64 = 128;
        // D s_10_1: read-var s:i64
        let s_10_1: i64 = fn_state.s;
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // C s_10_3: cast zx s_10_0 -> i
        let s_10_3: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_4: read-var operand:bv
        let s_10_4: Bits = fn_state.operand;
        // D s_10_5: call Elem_read(s_10_4, s_10_2, s_10_3)
        let s_10_5: Bits = Elem_read(state, tracer, s_10_4, s_10_2, s_10_3);
        // D s_10_6: cast reint s_10_5 -> u128
        let s_10_6: u128 = (s_10_5.value() as u128);
        // D s_10_7: write-var stmp <= s_10_6
        fn_state.stmp = s_10_6;
        // D s_10_8: read-var esizeshadow#4375:i64
        let s_10_8: i64 = fn_state.esizeshadow_4375;
        // D s_10_9: cast zx s_10_8 -> i
        let s_10_9: i128 = (i128::try_from(s_10_8).unwrap());
        // D s_10_10: cast reint s_10_9 -> i64
        let s_10_10: i64 = (s_10_9 as i64);
        // D s_10_11: read-var stmp:u128
        let s_10_11: u128 = fn_state.stmp;
        // D s_10_12: cast zx s_10_11 -> bv
        let s_10_12: Bits = Bits::new(s_10_11 as u128, 128u16);
        // D s_10_13: read-var e:i64
        let s_10_13: i64 = fn_state.e;
        // D s_10_14: cast zx s_10_13 -> i
        let s_10_14: i128 = (i128::try_from(s_10_13).unwrap());
        // D s_10_15: cast zx s_10_10 -> i
        let s_10_15: i128 = (i128::try_from(s_10_10).unwrap());
        // D s_10_16: call Elem_read(s_10_12, s_10_14, s_10_15)
        let s_10_16: Bits = Elem_read(state, tracer, s_10_12, s_10_14, s_10_15);
        // D s_10_17: read-var dtmp:bv
        let s_10_17: Bits = fn_state.dtmp;
        // D s_10_18: xor s_10_17 s_10_16
        let s_10_18: Bits = ((s_10_17) ^ (s_10_16));
        // D s_10_19: write-var dtmp <= s_10_18
        fn_state.dtmp = s_10_18;
        // N s_10_20: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var esizeshadow#4375:i64
        let s_11_0: i64 = fn_state.esizeshadow_4375;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: cast reint s_11_1 -> i64
        let s_11_2: i64 = (s_11_1 as i64);
        // C s_11_3: const #1s : i
        let s_11_3: i128 = 1;
        // D s_11_4: read-var esizeshadow#4375:i64
        let s_11_4: i64 = fn_state.esizeshadow_4375;
        // D s_11_5: cast zx s_11_4 -> i
        let s_11_5: i128 = (i128::try_from(s_11_4).unwrap());
        // D s_11_6: sub s_11_5 s_11_3
        let s_11_6: i128 = ((s_11_5) - (s_11_3));
        // D s_11_7: cast reint s_11_6 -> i64
        let s_11_7: i64 = (s_11_6 as i64);
        // C s_11_8: const #0s : i
        let s_11_8: i128 = 0;
        // D s_11_9: cast zx s_11_7 -> i
        let s_11_9: i128 = (i128::try_from(s_11_7).unwrap());
        // D s_11_10: read-var dtmp:bv
        let s_11_10: Bits = fn_state.dtmp;
        // C s_11_11: const #1s : i64
        let s_11_11: i64 = 1;
        // C s_11_12: cast zx s_11_11 -> i
        let s_11_12: i128 = (i128::try_from(s_11_11).unwrap());
        // D s_11_13: sub s_11_9 s_11_8
        let s_11_13: i128 = ((s_11_9) - (s_11_8));
        // D s_11_14: add s_11_13 s_11_12
        let s_11_14: i128 = (s_11_13 + s_11_12);
        // D s_11_15: bit-extract s_11_10 s_11_8 s_11_14
        let s_11_15: Bits = (Bits::new(
            ((s_11_10) >> (s_11_8)).value(),
            u16::try_from(s_11_14).unwrap(),
        ));
        // D s_11_16: read-var result:u128
        let s_11_16: u128 = fn_state.result;
        // D s_11_17: cast zx s_11_16 -> bv
        let s_11_17: Bits = Bits::new(s_11_16 as u128, 128u16);
        // D s_11_18: read-var e:i64
        let s_11_18: i64 = fn_state.e;
        // D s_11_19: cast zx s_11_18 -> i
        let s_11_19: i128 = (i128::try_from(s_11_18).unwrap());
        // D s_11_20: cast zx s_11_2 -> i
        let s_11_20: i128 = (i128::try_from(s_11_2).unwrap());
        // D s_11_21: call Elem_set(s_11_17, s_11_19, s_11_20, s_11_15)
        let s_11_21: Bits = Elem_set(state, tracer, s_11_17, s_11_19, s_11_20, s_11_15);
        // D s_11_22: cast reint s_11_21 -> u128
        let s_11_22: u128 = (s_11_21.value() as u128);
        // D s_11_23: write-var result <= s_11_22
        fn_state.result = s_11_22;
        // D s_11_24: read-var e:i64
        let s_11_24: i64 = fn_state.e;
        // C s_11_25: const #1s : i64
        let s_11_25: i64 = 1;
        // D s_11_26: add s_11_24 s_11_25
        let s_11_26: i64 = (s_11_24 + s_11_25);
        // D s_11_27: write-var e <= s_11_26
        fn_state.e = s_11_26;
        // N s_11_28: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #128s : i64
        let s_12_0: i64 = 128;
        // D s_12_1: read-var d:i64
        let s_12_1: i64 = fn_state.d;
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // D s_12_3: read-var result:u128
        let s_12_3: u128 = fn_state.result;
        // D s_12_4: cast zx s_12_3 -> bv
        let s_12_4: Bits = Bits::new(s_12_3 as u128, 128u16);
        // D s_12_5: call V_set(s_12_2, s_12_0, s_12_4)
        let s_12_5: () = V_set(state, tracer, s_12_2, s_12_0, s_12_4);
        // N s_12_6: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var VLshadow#4377:i64
        let s_13_0: i64 = fn_state.VLshadow_4377;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: cast reint s_13_1 -> i64
        let s_13_2: i64 = (s_13_1 as i64);
        // D s_13_3: read-var n:i64
        let s_13_3: i64 = fn_state.n;
        // D s_13_4: cast zx s_13_3 -> i
        let s_13_4: i128 = (i128::try_from(s_13_3).unwrap());
        // D s_13_5: cast zx s_13_2 -> i
        let s_13_5: i128 = (i128::try_from(s_13_2).unwrap());
        // D s_13_6: call Z_read(s_13_4, s_13_5)
        let s_13_6: Bits = Z_read(state, tracer, s_13_4, s_13_5);
        // D s_13_7: write-var operand <= s_13_6
        fn_state.operand = s_13_6;
        // N s_13_8: jump b3
        return block_3(state, tracer, fn_state);
    }
}
