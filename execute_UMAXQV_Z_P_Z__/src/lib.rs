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
use integer_subrange::*;
use P_read::*;
use ActivePredicateElement::*;
use Elem_read::*;
use Zeros::*;
use Z_read::*;
use common::*;
pub fn execute_UMAXQV_Z_P_Z__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    g: i64,
    n: i64,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        e: i64,
        VLshadow_4388: i64,
        mask: Bits,
        s: i64,
        elempersegment: i64,
        VLshadow_4389: i64,
        stmp: u128,
        dtmp: i128,
        esizeshadow_4387: i64,
        gs_220991: i64,
        segments: i64,
        gs_220999: i64,
        result: u128,
        VL: i64,
        d: i64,
        esize: i64,
        g: i64,
        n: i64,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        g,
        n,
        is_unsigned,
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
        // D s_0_3: write-var esizeshadow#4387 <= s_0_2
        fn_state.esizeshadow_4387 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#4388 <= s_0_6
        fn_state.VLshadow_4388 = s_0_6;
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
        // D s_1_0: read-var VLshadow#4388:i64
        let s_1_0: i64 = fn_state.VLshadow_4388;
        // D s_1_1: write-var VLshadow#4389 <= s_1_0
        fn_state.VLshadow_4389 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#4389:i64
        let s_1_3: i64 = fn_state.VLshadow_4389;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // C s_1_7: const #128s : i
        let s_1_7: i128 = 128;
        // D s_1_8: read-var VLshadow#4389:i64
        let s_1_8: i64 = fn_state.VLshadow_4389;
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
        // D s_1_14: read-var esizeshadow#4387:i64
        let s_1_14: i64 = fn_state.esizeshadow_4387;
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
        // D s_1_26: read-var esizeshadow#4387:i64
        let s_1_26: i64 = fn_state.esizeshadow_4387;
        // D s_1_27: cast zx s_1_26 -> i
        let s_1_27: i128 = (i128::try_from(s_1_26).unwrap());
        // D s_1_28: read-var mask:bv
        let s_1_28: Bits = fn_state.mask;
        // D s_1_29: call AnyActiveElement(s_1_28, s_1_27)
        let s_1_29: bool = AnyActiveElement(state, tracer, s_1_28, s_1_27);
        // N s_1_30: branch s_1_29 b16 b2
        if s_1_29 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var VLshadow#4389:i64
        let s_2_0: i64 = fn_state.VLshadow_4389;
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
        // D s_3_54: write-var gs#220991 <= s_3_53
        fn_state.gs_220991 = s_3_53;
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
        // D s_4_1: read-var gs#220991:i64
        let s_4_1: i64 = fn_state.gs_220991;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b15 b5
        if s_4_2 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var is_unsigned:u8
        let s_5_0: bool = fn_state.is_unsigned;
        // N s_5_1: branch s_5_0 b14 b6
        if s_5_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #1s : i
        let s_6_0: i128 = 1;
        // D s_6_1: read-var esizeshadow#4387:i64
        let s_6_1: i64 = fn_state.esizeshadow_4387;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: sub s_6_2 s_6_0
        let s_6_3: i128 = ((s_6_2) - (s_6_0));
        // D s_6_4: cast reint s_6_3 -> i64
        let s_6_4: i64 = (s_6_3 as i64);
        // D s_6_5: cast zx s_6_4 -> i
        let s_6_5: i128 = (i128::try_from(s_6_4).unwrap());
        // D s_6_6: pow2 s_6_5
        let s_6_6: i128 = (s_6_5).pow(2);
        // D s_6_7: neg s_6_6
        let s_6_7: i128 = -s_6_6;
        // D s_6_8: write-var dtmp <= s_6_7
        fn_state.dtmp = s_6_7;
        // N s_6_9: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0s : i64
        let s_7_0: i64 = 0;
        // C s_7_1: const #1s : i
        let s_7_1: i128 = 1;
        // D s_7_2: read-var segments:i64
        let s_7_2: i64 = fn_state.segments;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: sub s_7_3 s_7_1
        let s_7_4: i128 = ((s_7_3) - (s_7_1));
        // D s_7_5: cast reint s_7_4 -> i64
        let s_7_5: i64 = (s_7_4 as i64);
        // D s_7_6: write-var gs#220999 <= s_7_5
        fn_state.gs_220999 = s_7_5;
        // D s_7_7: write-var s <= s_7_0
        fn_state.s = s_7_0;
        // N s_7_8: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var s:i64
        let s_8_0: i64 = fn_state.s;
        // D s_8_1: read-var gs#220999:i64
        let s_8_1: i64 = fn_state.gs_220999;
        // D s_8_2: cmp-gt s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) > (s_8_1));
        // N s_8_3: branch s_8_2 b13 b9
        if s_8_2 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var s:i64
        let s_9_0: i64 = fn_state.s;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: read-var elempersegment:i64
        let s_9_2: i64 = fn_state.elempersegment;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: mul s_9_1 s_9_3
        let s_9_4: i128 = ((s_9_1) * (s_9_3));
        // D s_9_5: cast reint s_9_4 -> i64
        let s_9_5: i64 = (s_9_4 as i64);
        // D s_9_6: cast zx s_9_5 -> i
        let s_9_6: i128 = (i128::try_from(s_9_5).unwrap());
        // D s_9_7: read-var e:i64
        let s_9_7: i64 = fn_state.e;
        // D s_9_8: cast zx s_9_7 -> i
        let s_9_8: i128 = (i128::try_from(s_9_7).unwrap());
        // D s_9_9: add s_9_6 s_9_8
        let s_9_9: i128 = (s_9_6 + s_9_8);
        // D s_9_10: cast reint s_9_9 -> i64
        let s_9_10: i64 = (s_9_9 as i64);
        // D s_9_11: cast zx s_9_10 -> i
        let s_9_11: i128 = (i128::try_from(s_9_10).unwrap());
        // D s_9_12: read-var esizeshadow#4387:i64
        let s_9_12: i64 = fn_state.esizeshadow_4387;
        // D s_9_13: cast zx s_9_12 -> i
        let s_9_13: i128 = (i128::try_from(s_9_12).unwrap());
        // D s_9_14: read-var mask:bv
        let s_9_14: Bits = fn_state.mask;
        // D s_9_15: call ActivePredicateElement(s_9_14, s_9_11, s_9_13)
        let s_9_15: bool = ActivePredicateElement(state, tracer, s_9_14, s_9_11, s_9_13);
        // N s_9_16: branch s_9_15 b12 b10
        if s_9_15 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var s:i64
        let s_11_0: i64 = fn_state.s;
        // C s_11_1: const #1s : i64
        let s_11_1: i64 = 1;
        // D s_11_2: add s_11_0 s_11_1
        let s_11_2: i64 = (s_11_0 + s_11_1);
        // D s_11_3: write-var s <= s_11_2
        fn_state.s = s_11_2;
        // N s_11_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #128s : i64
        let s_12_0: i64 = 128;
        // D s_12_1: read-var s:i64
        let s_12_1: i64 = fn_state.s;
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // C s_12_3: cast zx s_12_0 -> i
        let s_12_3: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_4: read-var operand:bv
        let s_12_4: Bits = fn_state.operand;
        // D s_12_5: call Elem_read(s_12_4, s_12_2, s_12_3)
        let s_12_5: Bits = Elem_read(state, tracer, s_12_4, s_12_2, s_12_3);
        // D s_12_6: cast reint s_12_5 -> u128
        let s_12_6: u128 = (s_12_5.value() as u128);
        // D s_12_7: write-var stmp <= s_12_6
        fn_state.stmp = s_12_6;
        // D s_12_8: read-var esizeshadow#4387:i64
        let s_12_8: i64 = fn_state.esizeshadow_4387;
        // D s_12_9: cast zx s_12_8 -> i
        let s_12_9: i128 = (i128::try_from(s_12_8).unwrap());
        // D s_12_10: cast reint s_12_9 -> i64
        let s_12_10: i64 = (s_12_9 as i64);
        // D s_12_11: read-var stmp:u128
        let s_12_11: u128 = fn_state.stmp;
        // D s_12_12: cast zx s_12_11 -> bv
        let s_12_12: Bits = Bits::new(s_12_11 as u128, 128u16);
        // D s_12_13: read-var e:i64
        let s_12_13: i64 = fn_state.e;
        // D s_12_14: cast zx s_12_13 -> i
        let s_12_14: i128 = (i128::try_from(s_12_13).unwrap());
        // D s_12_15: cast zx s_12_10 -> i
        let s_12_15: i128 = (i128::try_from(s_12_10).unwrap());
        // D s_12_16: call Elem_read(s_12_12, s_12_14, s_12_15)
        let s_12_16: Bits = Elem_read(state, tracer, s_12_12, s_12_14, s_12_15);
        // D s_12_17: cast zx s_12_16 -> i
        let s_12_17: i128 = (s_12_16.value() as i128);
        // D s_12_18: read-var dtmp:i
        let s_12_18: i128 = fn_state.dtmp;
        // D s_12_19: cmp-gt s_12_18 s_12_17
        let s_12_19: bool = ((s_12_18) > (s_12_17));
        // D s_12_20: select s_12_19 s_12_18 s_12_17
        let s_12_20: i128 = if s_12_19 { s_12_18 } else { s_12_17 };
        // D s_12_21: write-var dtmp <= s_12_20
        fn_state.dtmp = s_12_20;
        // N s_12_22: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var esizeshadow#4387:i64
        let s_13_0: i64 = fn_state.esizeshadow_4387;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: cast reint s_13_1 -> i64
        let s_13_2: i64 = (s_13_1 as i64);
        // C s_13_3: const #1s : i
        let s_13_3: i128 = 1;
        // D s_13_4: read-var esizeshadow#4387:i64
        let s_13_4: i64 = fn_state.esizeshadow_4387;
        // D s_13_5: cast zx s_13_4 -> i
        let s_13_5: i128 = (i128::try_from(s_13_4).unwrap());
        // D s_13_6: sub s_13_5 s_13_3
        let s_13_6: i128 = ((s_13_5) - (s_13_3));
        // D s_13_7: cast reint s_13_6 -> i64
        let s_13_7: i64 = (s_13_6 as i64);
        // C s_13_8: const #0s : i
        let s_13_8: i128 = 0;
        // D s_13_9: cast zx s_13_7 -> i
        let s_13_9: i128 = (i128::try_from(s_13_7).unwrap());
        // D s_13_10: read-var dtmp:i
        let s_13_10: i128 = fn_state.dtmp;
        // D s_13_11: call integer_subrange(s_13_10, s_13_9, s_13_8)
        let s_13_11: Bits = integer_subrange(state, tracer, s_13_10, s_13_9, s_13_8);
        // D s_13_12: read-var result:u128
        let s_13_12: u128 = fn_state.result;
        // D s_13_13: cast zx s_13_12 -> bv
        let s_13_13: Bits = Bits::new(s_13_12 as u128, 128u16);
        // D s_13_14: read-var e:i64
        let s_13_14: i64 = fn_state.e;
        // D s_13_15: cast zx s_13_14 -> i
        let s_13_15: i128 = (i128::try_from(s_13_14).unwrap());
        // D s_13_16: cast zx s_13_2 -> i
        let s_13_16: i128 = (i128::try_from(s_13_2).unwrap());
        // D s_13_17: call Elem_set(s_13_13, s_13_15, s_13_16, s_13_11)
        let s_13_17: Bits = Elem_set(state, tracer, s_13_13, s_13_15, s_13_16, s_13_11);
        // D s_13_18: cast reint s_13_17 -> u128
        let s_13_18: u128 = (s_13_17.value() as u128);
        // D s_13_19: write-var result <= s_13_18
        fn_state.result = s_13_18;
        // D s_13_20: read-var e:i64
        let s_13_20: i64 = fn_state.e;
        // C s_13_21: const #1s : i64
        let s_13_21: i64 = 1;
        // D s_13_22: add s_13_20 s_13_21
        let s_13_22: i64 = (s_13_20 + s_13_21);
        // D s_13_23: write-var e <= s_13_22
        fn_state.e = s_13_22;
        // N s_13_24: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0s : i
        let s_14_0: i128 = 0;
        // D s_14_1: write-var dtmp <= s_14_0
        fn_state.dtmp = s_14_0;
        // N s_14_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #128s : i64
        let s_15_0: i64 = 128;
        // D s_15_1: read-var d:i64
        let s_15_1: i64 = fn_state.d;
        // D s_15_2: cast zx s_15_1 -> i
        let s_15_2: i128 = (i128::try_from(s_15_1).unwrap());
        // D s_15_3: read-var result:u128
        let s_15_3: u128 = fn_state.result;
        // D s_15_4: cast zx s_15_3 -> bv
        let s_15_4: Bits = Bits::new(s_15_3 as u128, 128u16);
        // D s_15_5: call V_set(s_15_2, s_15_0, s_15_4)
        let s_15_5: () = V_set(state, tracer, s_15_2, s_15_0, s_15_4);
        // N s_15_6: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var VLshadow#4389:i64
        let s_16_0: i64 = fn_state.VLshadow_4389;
        // D s_16_1: cast zx s_16_0 -> i
        let s_16_1: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_2: cast reint s_16_1 -> i64
        let s_16_2: i64 = (s_16_1 as i64);
        // D s_16_3: read-var n:i64
        let s_16_3: i64 = fn_state.n;
        // D s_16_4: cast zx s_16_3 -> i
        let s_16_4: i128 = (i128::try_from(s_16_3).unwrap());
        // D s_16_5: cast zx s_16_2 -> i
        let s_16_5: i128 = (i128::try_from(s_16_2).unwrap());
        // D s_16_6: call Z_read(s_16_4, s_16_5)
        let s_16_6: Bits = Z_read(state, tracer, s_16_4, s_16_5);
        // D s_16_7: write-var operand <= s_16_6
        fn_state.operand = s_16_6;
        // N s_16_8: jump b3
        return block_3(state, tracer, fn_state);
    }
}
