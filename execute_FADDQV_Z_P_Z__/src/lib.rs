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
use CeilPow2::*;
use CheckSVEEnabled::*;
use AnyActiveElement::*;
use P_read::*;
use u__id::*;
use ActivePredicateElement::*;
use V_set::*;
use FPZero::*;
use Zeros::*;
use Elem_read::*;
use Z_read::*;
use Reduce::*;
use common::*;
pub fn execute_FADDQV_Z_P_Z__<T: Tracer>(
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
        p2elems: i128,
        VLshadow_2416: i64,
        e: i64,
        gs_183952: bool,
        gs_183955: bool,
        p2bits: i128,
        gs_183918: bool,
        gs_183958: bool,
        VLshadow_2415: i64,
        identity: Bits,
        mask: Bits,
        gs_183939: bool,
        gs_183951: bool,
        s: i64,
        gs_183957: bool,
        gs_183945: bool,
        elempersegment: i64,
        gs_183932: i64,
        gs_183935: bool,
        gs_183926: i64,
        esizeshadow_2414: i64,
        stmp: Bits,
        gs_183916: bool,
        dtmp: Bits,
        gs_183949: bool,
        gs_183943: bool,
        segments: i64,
        result: u128,
        gs_183941: bool,
        gs_183947: bool,
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
        // D s_0_3: write-var esizeshadow#2414 <= s_0_2
        fn_state.esizeshadow_2414 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#2415 <= s_0_6
        fn_state.VLshadow_2415 = s_0_6;
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
        // D s_1_0: read-var VLshadow#2415:i64
        let s_1_0: i64 = fn_state.VLshadow_2415;
        // D s_1_1: write-var VLshadow#2416 <= s_1_0
        fn_state.VLshadow_2416 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#2416:i64
        let s_1_3: i64 = fn_state.VLshadow_2416;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // C s_1_7: const #128s : i
        let s_1_7: i128 = 128;
        // D s_1_8: read-var VLshadow#2416:i64
        let s_1_8: i64 = fn_state.VLshadow_2416;
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
        // D s_1_14: read-var esizeshadow#2414:i64
        let s_1_14: i64 = fn_state.esizeshadow_2414;
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
        // D s_1_26: read-var esizeshadow#2414:i64
        let s_1_26: i64 = fn_state.esizeshadow_2414;
        // D s_1_27: cast zx s_1_26 -> i
        let s_1_27: i128 = (i128::try_from(s_1_26).unwrap());
        // D s_1_28: read-var mask:bv
        let s_1_28: Bits = fn_state.mask;
        // D s_1_29: call AnyActiveElement(s_1_28, s_1_27)
        let s_1_29: bool = AnyActiveElement(state, tracer, s_1_28, s_1_27);
        // N s_1_30: branch s_1_29 b56 b2
        if s_1_29 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var VLshadow#2416:i64
        let s_2_0: i64 = fn_state.VLshadow_2416;
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
        // D s_3_0: read-var esizeshadow#2414:i64
        let s_3_0: i64 = fn_state.esizeshadow_2414;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: call __id(s_3_1)
        let s_3_2: i128 = u__id(state, tracer, s_3_1);
        // D s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // C s_3_4: const #16s : i
        let s_3_4: i128 = 16;
        // D s_3_5: cast zx s_3_3 -> i
        let s_3_5: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_6: cmp-eq s_3_5 s_3_4
        let s_3_6: bool = ((s_3_5) == (s_3_4));
        // N s_3_7: branch s_3_6 b55 b4
        if s_3_6 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var esizeshadow#2414:i64
        let s_4_0: i64 = fn_state.esizeshadow_2414;
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
        // D s_4_7: write-var gs#183916 <= s_4_6
        fn_state.gs_183916 = s_4_6;
        // N s_4_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#183916:u8
        let s_5_0: bool = fn_state.gs_183916;
        // N s_5_1: branch s_5_0 b54 b6
        if s_5_0 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var esizeshadow#2414:i64
        let s_6_0: i64 = fn_state.esizeshadow_2414;
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
        // D s_6_7: write-var gs#183918 <= s_6_6
        fn_state.gs_183918 = s_6_6;
        // N s_6_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#183918:u8
        let s_7_0: bool = fn_state.gs_183918;
        // N s_7_1: assert s_7_0
        let s_7_1: () = assert!(s_7_0);
        // D s_7_2: read-var esizeshadow#2414:i64
        let s_7_2: i64 = fn_state.esizeshadow_2414;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: cast reint s_7_3 -> i64
        let s_7_4: i64 = (s_7_3 as i64);
        // C s_7_5: const #0u : u8
        let s_7_5: bool = false;
        // D s_7_6: call FPZero(s_7_5, s_7_4)
        let s_7_6: Bits = FPZero(state, tracer, s_7_5, s_7_4);
        // D s_7_7: write-var identity <= s_7_6
        fn_state.identity = s_7_6;
        // C s_7_8: const #0u : u8
        let s_7_8: u8 = 0;
        // C s_7_9: cast zx s_7_8 -> bv
        let s_7_9: Bits = Bits::new(s_7_8 as u128, 0u16);
        // C s_7_10: const #0u : u64
        let s_7_10: u64 = 0;
        // C s_7_11: cast zx s_7_10 -> bv
        let s_7_11: Bits = Bits::new(s_7_10 as u128, 64u16);
        // C s_7_12: cast reint s_7_9 -> u128
        let s_7_12: u128 = (s_7_9.value() as u128);
        // D s_7_13: size-of s_7_9
        let s_7_13: u16 = s_7_9.length();
        // C s_7_14: cast reint s_7_11 -> u128
        let s_7_14: u128 = (s_7_11.value() as u128);
        // D s_7_15: size-of s_7_11
        let s_7_15: u16 = s_7_11.length();
        // D s_7_16: lsl s_7_12 s_7_15
        let s_7_16: u128 = s_7_12 << s_7_15;
        // D s_7_17: or s_7_16 s_7_14
        let s_7_17: u128 = ((s_7_16) | (s_7_14));
        // D s_7_18: add s_7_13 s_7_15
        let s_7_18: u16 = (s_7_13 + s_7_15);
        // D s_7_19: create-bits s_7_17 s_7_18
        let s_7_19: Bits = Bits::new(s_7_17, s_7_18);
        // C s_7_20: const #0u : u64
        let s_7_20: u64 = 0;
        // C s_7_21: cast zx s_7_20 -> bv
        let s_7_21: Bits = Bits::new(s_7_20 as u128, 64u16);
        // D s_7_22: cast reint s_7_19 -> u128
        let s_7_22: u128 = (s_7_19.value() as u128);
        // D s_7_23: size-of s_7_19
        let s_7_23: u16 = s_7_19.length();
        // C s_7_24: cast reint s_7_21 -> u128
        let s_7_24: u128 = (s_7_21.value() as u128);
        // D s_7_25: size-of s_7_21
        let s_7_25: u16 = s_7_21.length();
        // D s_7_26: lsl s_7_22 s_7_25
        let s_7_26: u128 = s_7_22 << s_7_25;
        // D s_7_27: or s_7_26 s_7_24
        let s_7_27: u128 = ((s_7_26) | (s_7_24));
        // D s_7_28: add s_7_23 s_7_25
        let s_7_28: u16 = (s_7_23 + s_7_25);
        // D s_7_29: create-bits s_7_27 s_7_28
        let s_7_29: Bits = Bits::new(s_7_27, s_7_28);
        // D s_7_30: cast reint s_7_29 -> u128
        let s_7_30: u128 = (s_7_29.value() as u128);
        // D s_7_31: write-var result <= s_7_30
        fn_state.result = s_7_30;
        // D s_7_32: read-var segments:i64
        let s_7_32: i64 = fn_state.segments;
        // D s_7_33: cast zx s_7_32 -> i
        let s_7_33: i128 = (i128::try_from(s_7_32).unwrap());
        // D s_7_34: read-var esizeshadow#2414:i64
        let s_7_34: i64 = fn_state.esizeshadow_2414;
        // D s_7_35: cast zx s_7_34 -> i
        let s_7_35: i128 = (i128::try_from(s_7_34).unwrap());
        // D s_7_36: mul s_7_33 s_7_35
        let s_7_36: i128 = ((s_7_33) * (s_7_35));
        // D s_7_37: cast reint s_7_36 -> i64
        let s_7_37: i64 = (s_7_36 as i64);
        // D s_7_38: cast zx s_7_37 -> i
        let s_7_38: i128 = (i128::try_from(s_7_37).unwrap());
        // D s_7_39: call CeilPow2(s_7_38)
        let s_7_39: i128 = CeilPow2(state, tracer, s_7_38);
        // D s_7_40: write-var p2bits <= s_7_39
        fn_state.p2bits = s_7_39;
        // D s_7_41: read-var esizeshadow#2414:i64
        let s_7_41: i64 = fn_state.esizeshadow_2414;
        // D s_7_42: cast zx s_7_41 -> i
        let s_7_42: i128 = (i128::try_from(s_7_41).unwrap());
        // D s_7_43: read-var p2bits:i
        let s_7_43: i128 = fn_state.p2bits;
        // D s_7_44: div s_7_43 s_7_42
        let s_7_44: i128 = ((s_7_43) / (s_7_42));
        // D s_7_45: write-var p2elems <= s_7_44
        fn_state.p2elems = s_7_44;
        // C s_7_46: const #0s : i64
        let s_7_46: i64 = 0;
        // C s_7_47: const #1s : i
        let s_7_47: i128 = 1;
        // D s_7_48: read-var elempersegment:i64
        let s_7_48: i64 = fn_state.elempersegment;
        // D s_7_49: cast zx s_7_48 -> i
        let s_7_49: i128 = (i128::try_from(s_7_48).unwrap());
        // D s_7_50: sub s_7_49 s_7_47
        let s_7_50: i128 = ((s_7_49) - (s_7_47));
        // D s_7_51: cast reint s_7_50 -> i64
        let s_7_51: i64 = (s_7_50 as i64);
        // D s_7_52: write-var gs#183926 <= s_7_51
        fn_state.gs_183926 = s_7_51;
        // D s_7_53: write-var e <= s_7_46
        fn_state.e = s_7_46;
        // N s_7_54: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var e:i64
        let s_8_0: i64 = fn_state.e;
        // D s_8_1: read-var gs#183926:i64
        let s_8_1: i64 = fn_state.gs_183926;
        // D s_8_2: cmp-gt s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) > (s_8_1));
        // N s_8_3: branch s_8_2 b53 b9
        if s_8_2 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0s : i64
        let s_9_0: i64 = 0;
        // C s_9_1: const #1s : i
        let s_9_1: i128 = 1;
        // D s_9_2: read-var p2elems:i
        let s_9_2: i128 = fn_state.p2elems;
        // D s_9_3: sub s_9_2 s_9_1
        let s_9_3: i128 = ((s_9_2) - (s_9_1));
        // D s_9_4: cast reint s_9_3 -> i64
        let s_9_4: i64 = (s_9_3 as i64);
        // D s_9_5: write-var gs#183932 <= s_9_4
        fn_state.gs_183932 = s_9_4;
        // D s_9_6: write-var s <= s_9_0
        fn_state.s = s_9_0;
        // N s_9_7: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var s:i64
        let s_10_0: i64 = fn_state.s;
        // D s_10_1: read-var gs#183932:i64
        let s_10_1: i64 = fn_state.gs_183932;
        // D s_10_2: cmp-gt s_10_0 s_10_1
        let s_10_2: bool = ((s_10_0) > (s_10_1));
        // N s_10_3: branch s_10_2 b18 b11
        if s_10_2 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var s:i64
        let s_11_0: i64 = fn_state.s;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: read-var segments:i64
        let s_11_2: i64 = fn_state.segments;
        // D s_11_3: cast zx s_11_2 -> i
        let s_11_3: i128 = (i128::try_from(s_11_2).unwrap());
        // D s_11_4: cmp-lt s_11_1 s_11_3
        let s_11_4: bool = ((s_11_1) < (s_11_3));
        // N s_11_5: branch s_11_4 b17 b12
        if s_11_4 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#183935 <= s_12_0
        fn_state.gs_183935 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#183935:u8
        let s_13_0: bool = fn_state.gs_183935;
        // N s_13_1: branch s_13_0 b16 b14
        if s_13_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var esizeshadow#2414:i64
        let s_14_0: i64 = fn_state.esizeshadow_2414;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: cast reint s_14_1 -> i64
        let s_14_2: i64 = (s_14_1 as i64);
        // D s_14_3: read-var s:i64
        let s_14_3: i64 = fn_state.s;
        // D s_14_4: cast zx s_14_3 -> i
        let s_14_4: i128 = (i128::try_from(s_14_3).unwrap());
        // D s_14_5: cast zx s_14_2 -> i
        let s_14_5: i128 = (i128::try_from(s_14_2).unwrap());
        // D s_14_6: read-var stmp:bv
        let s_14_6: Bits = fn_state.stmp;
        // D s_14_7: read-var identity:bv
        let s_14_7: Bits = fn_state.identity;
        // D s_14_8: call Elem_set(s_14_6, s_14_4, s_14_5, s_14_7)
        let s_14_8: Bits = Elem_set(state, tracer, s_14_6, s_14_4, s_14_5, s_14_7);
        // D s_14_9: write-var stmp <= s_14_8
        fn_state.stmp = s_14_8;
        // N s_14_10: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var s:i64
        let s_15_0: i64 = fn_state.s;
        // C s_15_1: const #1s : i64
        let s_15_1: i64 = 1;
        // D s_15_2: add s_15_0 s_15_1
        let s_15_2: i64 = (s_15_0 + s_15_1);
        // D s_15_3: write-var s <= s_15_2
        fn_state.s = s_15_2;
        // N s_15_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var esizeshadow#2414:i64
        let s_16_0: i64 = fn_state.esizeshadow_2414;
        // D s_16_1: cast zx s_16_0 -> i
        let s_16_1: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_2: cast reint s_16_1 -> i64
        let s_16_2: i64 = (s_16_1 as i64);
        // D s_16_3: read-var s:i64
        let s_16_3: i64 = fn_state.s;
        // D s_16_4: cast zx s_16_3 -> i
        let s_16_4: i128 = (i128::try_from(s_16_3).unwrap());
        // D s_16_5: read-var elempersegment:i64
        let s_16_5: i64 = fn_state.elempersegment;
        // D s_16_6: cast zx s_16_5 -> i
        let s_16_6: i128 = (i128::try_from(s_16_5).unwrap());
        // D s_16_7: mul s_16_4 s_16_6
        let s_16_7: i128 = ((s_16_4) * (s_16_6));
        // D s_16_8: cast reint s_16_7 -> i64
        let s_16_8: i64 = (s_16_7 as i64);
        // D s_16_9: cast zx s_16_8 -> i
        let s_16_9: i128 = (i128::try_from(s_16_8).unwrap());
        // D s_16_10: read-var e:i64
        let s_16_10: i64 = fn_state.e;
        // D s_16_11: cast zx s_16_10 -> i
        let s_16_11: i128 = (i128::try_from(s_16_10).unwrap());
        // D s_16_12: add s_16_9 s_16_11
        let s_16_12: i128 = (s_16_9 + s_16_11);
        // D s_16_13: cast reint s_16_12 -> i64
        let s_16_13: i64 = (s_16_12 as i64);
        // D s_16_14: read-var esizeshadow#2414:i64
        let s_16_14: i64 = fn_state.esizeshadow_2414;
        // D s_16_15: cast zx s_16_14 -> i
        let s_16_15: i128 = (i128::try_from(s_16_14).unwrap());
        // D s_16_16: cast reint s_16_15 -> i64
        let s_16_16: i64 = (s_16_15 as i64);
        // D s_16_17: cast zx s_16_13 -> i
        let s_16_17: i128 = (i128::try_from(s_16_13).unwrap());
        // D s_16_18: cast zx s_16_16 -> i
        let s_16_18: i128 = (i128::try_from(s_16_16).unwrap());
        // D s_16_19: read-var operand:bv
        let s_16_19: Bits = fn_state.operand;
        // D s_16_20: call Elem_read(s_16_19, s_16_17, s_16_18)
        let s_16_20: Bits = Elem_read(state, tracer, s_16_19, s_16_17, s_16_18);
        // D s_16_21: read-var s:i64
        let s_16_21: i64 = fn_state.s;
        // D s_16_22: cast zx s_16_21 -> i
        let s_16_22: i128 = (i128::try_from(s_16_21).unwrap());
        // D s_16_23: cast zx s_16_2 -> i
        let s_16_23: i128 = (i128::try_from(s_16_2).unwrap());
        // D s_16_24: read-var stmp:bv
        let s_16_24: Bits = fn_state.stmp;
        // D s_16_25: call Elem_set(s_16_24, s_16_22, s_16_23, s_16_20)
        let s_16_25: Bits = Elem_set(state, tracer, s_16_24, s_16_22, s_16_23, s_16_20);
        // D s_16_26: write-var stmp <= s_16_25
        fn_state.stmp = s_16_25;
        // N s_16_27: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var s:i64
        let s_17_0: i64 = fn_state.s;
        // D s_17_1: cast zx s_17_0 -> i
        let s_17_1: i128 = (i128::try_from(s_17_0).unwrap());
        // D s_17_2: read-var elempersegment:i64
        let s_17_2: i64 = fn_state.elempersegment;
        // D s_17_3: cast zx s_17_2 -> i
        let s_17_3: i128 = (i128::try_from(s_17_2).unwrap());
        // D s_17_4: mul s_17_1 s_17_3
        let s_17_4: i128 = ((s_17_1) * (s_17_3));
        // D s_17_5: read-var e:i64
        let s_17_5: i64 = fn_state.e;
        // D s_17_6: cast zx s_17_5 -> i
        let s_17_6: i128 = (i128::try_from(s_17_5).unwrap());
        // D s_17_7: add s_17_4 s_17_6
        let s_17_7: i128 = (s_17_4 + s_17_6);
        // D s_17_8: read-var esizeshadow#2414:i64
        let s_17_8: i64 = fn_state.esizeshadow_2414;
        // D s_17_9: cast zx s_17_8 -> i
        let s_17_9: i128 = (i128::try_from(s_17_8).unwrap());
        // D s_17_10: read-var mask:bv
        let s_17_10: Bits = fn_state.mask;
        // D s_17_11: call ActivePredicateElement(s_17_10, s_17_7, s_17_9)
        let s_17_11: bool = ActivePredicateElement(
            state,
            tracer,
            s_17_10,
            s_17_7,
            s_17_9,
        );
        // D s_17_12: write-var gs#183935 <= s_17_11
        fn_state.gs_183935 = s_17_11;
        // N s_17_13: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var p2bits:i
        let s_18_0: i128 = fn_state.p2bits;
        // D s_18_1: call __id(s_18_0)
        let s_18_1: i128 = u__id(state, tracer, s_18_0);
        // C s_18_2: const #16s : i
        let s_18_2: i128 = 16;
        // D s_18_3: cmp-eq s_18_1 s_18_2
        let s_18_3: bool = ((s_18_1) == (s_18_2));
        // N s_18_4: branch s_18_3 b52 b19
        if s_18_3 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var p2bits:i
        let s_19_0: i128 = fn_state.p2bits;
        // D s_19_1: call __id(s_19_0)
        let s_19_1: i128 = u__id(state, tracer, s_19_0);
        // C s_19_2: const #32s : i
        let s_19_2: i128 = 32;
        // D s_19_3: cmp-eq s_19_1 s_19_2
        let s_19_3: bool = ((s_19_1) == (s_19_2));
        // D s_19_4: write-var gs#183939 <= s_19_3
        fn_state.gs_183939 = s_19_3;
        // N s_19_5: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#183939:u8
        let s_20_0: bool = fn_state.gs_183939;
        // N s_20_1: branch s_20_0 b51 b21
        if s_20_0 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var p2bits:i
        let s_21_0: i128 = fn_state.p2bits;
        // D s_21_1: call __id(s_21_0)
        let s_21_1: i128 = u__id(state, tracer, s_21_0);
        // C s_21_2: const #64s : i
        let s_21_2: i128 = 64;
        // D s_21_3: cmp-eq s_21_1 s_21_2
        let s_21_3: bool = ((s_21_1) == (s_21_2));
        // D s_21_4: write-var gs#183941 <= s_21_3
        fn_state.gs_183941 = s_21_3;
        // N s_21_5: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#183941:u8
        let s_22_0: bool = fn_state.gs_183941;
        // N s_22_1: branch s_22_0 b50 b23
        if s_22_0 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var p2bits:i
        let s_23_0: i128 = fn_state.p2bits;
        // D s_23_1: call __id(s_23_0)
        let s_23_1: i128 = u__id(state, tracer, s_23_0);
        // C s_23_2: const #128s : i
        let s_23_2: i128 = 128;
        // D s_23_3: cmp-eq s_23_1 s_23_2
        let s_23_3: bool = ((s_23_1) == (s_23_2));
        // D s_23_4: write-var gs#183943 <= s_23_3
        fn_state.gs_183943 = s_23_3;
        // N s_23_5: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#183943:u8
        let s_24_0: bool = fn_state.gs_183943;
        // N s_24_1: branch s_24_0 b49 b25
        if s_24_0 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var p2bits:i
        let s_25_0: i128 = fn_state.p2bits;
        // D s_25_1: call __id(s_25_0)
        let s_25_1: i128 = u__id(state, tracer, s_25_0);
        // C s_25_2: const #256s : i
        let s_25_2: i128 = 256;
        // D s_25_3: cmp-eq s_25_1 s_25_2
        let s_25_3: bool = ((s_25_1) == (s_25_2));
        // D s_25_4: write-var gs#183945 <= s_25_3
        fn_state.gs_183945 = s_25_3;
        // N s_25_5: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#183945:u8
        let s_26_0: bool = fn_state.gs_183945;
        // N s_26_1: branch s_26_0 b48 b27
        if s_26_0 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var p2bits:i
        let s_27_0: i128 = fn_state.p2bits;
        // D s_27_1: call __id(s_27_0)
        let s_27_1: i128 = u__id(state, tracer, s_27_0);
        // C s_27_2: const #512s : i
        let s_27_2: i128 = 512;
        // D s_27_3: cmp-eq s_27_1 s_27_2
        let s_27_3: bool = ((s_27_1) == (s_27_2));
        // D s_27_4: write-var gs#183947 <= s_27_3
        fn_state.gs_183947 = s_27_3;
        // N s_27_5: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#183947:u8
        let s_28_0: bool = fn_state.gs_183947;
        // N s_28_1: branch s_28_0 b47 b29
        if s_28_0 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var p2bits:i
        let s_29_0: i128 = fn_state.p2bits;
        // D s_29_1: call __id(s_29_0)
        let s_29_1: i128 = u__id(state, tracer, s_29_0);
        // C s_29_2: const #1024s : i
        let s_29_2: i128 = 1024;
        // D s_29_3: cmp-eq s_29_1 s_29_2
        let s_29_3: bool = ((s_29_1) == (s_29_2));
        // D s_29_4: write-var gs#183949 <= s_29_3
        fn_state.gs_183949 = s_29_3;
        // N s_29_5: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#183949:u8
        let s_30_0: bool = fn_state.gs_183949;
        // N s_30_1: branch s_30_0 b46 b31
        if s_30_0 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var p2bits:i
        let s_31_0: i128 = fn_state.p2bits;
        // D s_31_1: call __id(s_31_0)
        let s_31_1: i128 = u__id(state, tracer, s_31_0);
        // C s_31_2: const #2048s : i
        let s_31_2: i128 = 2048;
        // D s_31_3: cmp-eq s_31_1 s_31_2
        let s_31_3: bool = ((s_31_1) == (s_31_2));
        // D s_31_4: write-var gs#183951 <= s_31_3
        fn_state.gs_183951 = s_31_3;
        // N s_31_5: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#183951:u8
        let s_32_0: bool = fn_state.gs_183951;
        // N s_32_1: branch s_32_0 b45 b33
        if s_32_0 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #0u : u8
        let s_33_0: bool = false;
        // D s_33_1: write-var gs#183952 <= s_33_0
        fn_state.gs_183952 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#183952:u8
        let s_34_0: bool = fn_state.gs_183952;
        // N s_34_1: branch s_34_0 b44 b35
        if s_34_0 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var esizeshadow#2414:i64
        let s_35_0: i64 = fn_state.esizeshadow_2414;
        // D s_35_1: cast zx s_35_0 -> i
        let s_35_1: i128 = (i128::try_from(s_35_0).unwrap());
        // D s_35_2: call __id(s_35_1)
        let s_35_2: i128 = u__id(state, tracer, s_35_1);
        // D s_35_3: cast reint s_35_2 -> i64
        let s_35_3: i64 = (s_35_2 as i64);
        // C s_35_4: const #16s : i
        let s_35_4: i128 = 16;
        // D s_35_5: cast zx s_35_3 -> i
        let s_35_5: i128 = (i128::try_from(s_35_3).unwrap());
        // D s_35_6: cmp-eq s_35_5 s_35_4
        let s_35_6: bool = ((s_35_5) == (s_35_4));
        // N s_35_7: branch s_35_6 b43 b36
        if s_35_6 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var esizeshadow#2414:i64
        let s_36_0: i64 = fn_state.esizeshadow_2414;
        // D s_36_1: cast zx s_36_0 -> i
        let s_36_1: i128 = (i128::try_from(s_36_0).unwrap());
        // D s_36_2: call __id(s_36_1)
        let s_36_2: i128 = u__id(state, tracer, s_36_1);
        // D s_36_3: cast reint s_36_2 -> i64
        let s_36_3: i64 = (s_36_2 as i64);
        // C s_36_4: const #32s : i
        let s_36_4: i128 = 32;
        // D s_36_5: cast zx s_36_3 -> i
        let s_36_5: i128 = (i128::try_from(s_36_3).unwrap());
        // D s_36_6: cmp-eq s_36_5 s_36_4
        let s_36_6: bool = ((s_36_5) == (s_36_4));
        // D s_36_7: write-var gs#183955 <= s_36_6
        fn_state.gs_183955 = s_36_6;
        // N s_36_8: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#183955:u8
        let s_37_0: bool = fn_state.gs_183955;
        // N s_37_1: branch s_37_0 b42 b38
        if s_37_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var esizeshadow#2414:i64
        let s_38_0: i64 = fn_state.esizeshadow_2414;
        // D s_38_1: cast zx s_38_0 -> i
        let s_38_1: i128 = (i128::try_from(s_38_0).unwrap());
        // D s_38_2: call __id(s_38_1)
        let s_38_2: i128 = u__id(state, tracer, s_38_1);
        // D s_38_3: cast reint s_38_2 -> i64
        let s_38_3: i64 = (s_38_2 as i64);
        // C s_38_4: const #64s : i
        let s_38_4: i128 = 64;
        // D s_38_5: cast zx s_38_3 -> i
        let s_38_5: i128 = (i128::try_from(s_38_3).unwrap());
        // D s_38_6: cmp-eq s_38_5 s_38_4
        let s_38_6: bool = ((s_38_5) == (s_38_4));
        // D s_38_7: write-var gs#183957 <= s_38_6
        fn_state.gs_183957 = s_38_6;
        // N s_38_8: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#183957:u8
        let s_39_0: bool = fn_state.gs_183957;
        // D s_39_1: not s_39_0
        let s_39_1: bool = !s_39_0;
        // D s_39_2: write-var gs#183958 <= s_39_1
        fn_state.gs_183958 = s_39_1;
        // N s_39_3: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#183958:u8
        let s_40_0: bool = fn_state.gs_183958;
        // N s_40_1: assert s_40_0
        let s_40_1: () = assert!(s_40_0);
        // D s_40_2: read-var esizeshadow#2414:i64
        let s_40_2: i64 = fn_state.esizeshadow_2414;
        // D s_40_3: cast zx s_40_2 -> i
        let s_40_3: i128 = (i128::try_from(s_40_2).unwrap());
        // D s_40_4: cast reint s_40_3 -> i64
        let s_40_4: i64 = (s_40_3 as i64);
        // C s_40_5: const #4u : u32
        let s_40_5: u32 = 4;
        // D s_40_6: read-var stmp:bv
        let s_40_6: Bits = fn_state.stmp;
        // D s_40_7: call Reduce(s_40_5, s_40_6, s_40_4)
        let s_40_7: Bits = Reduce(state, tracer, s_40_5, s_40_6, s_40_4);
        // D s_40_8: write-var dtmp <= s_40_7
        fn_state.dtmp = s_40_7;
        // N s_40_9: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var esizeshadow#2414:i64
        let s_41_0: i64 = fn_state.esizeshadow_2414;
        // D s_41_1: cast zx s_41_0 -> i
        let s_41_1: i128 = (i128::try_from(s_41_0).unwrap());
        // D s_41_2: cast reint s_41_1 -> i64
        let s_41_2: i64 = (s_41_1 as i64);
        // D s_41_3: read-var result:u128
        let s_41_3: u128 = fn_state.result;
        // D s_41_4: cast zx s_41_3 -> bv
        let s_41_4: Bits = Bits::new(s_41_3 as u128, 128u16);
        // D s_41_5: read-var e:i64
        let s_41_5: i64 = fn_state.e;
        // D s_41_6: cast zx s_41_5 -> i
        let s_41_6: i128 = (i128::try_from(s_41_5).unwrap());
        // D s_41_7: cast zx s_41_2 -> i
        let s_41_7: i128 = (i128::try_from(s_41_2).unwrap());
        // D s_41_8: read-var dtmp:bv
        let s_41_8: Bits = fn_state.dtmp;
        // D s_41_9: call Elem_set(s_41_4, s_41_6, s_41_7, s_41_8)
        let s_41_9: Bits = Elem_set(state, tracer, s_41_4, s_41_6, s_41_7, s_41_8);
        // D s_41_10: cast reint s_41_9 -> u128
        let s_41_10: u128 = (s_41_9.value() as u128);
        // D s_41_11: write-var result <= s_41_10
        fn_state.result = s_41_10;
        // D s_41_12: read-var e:i64
        let s_41_12: i64 = fn_state.e;
        // C s_41_13: const #1s : i64
        let s_41_13: i64 = 1;
        // D s_41_14: add s_41_12 s_41_13
        let s_41_14: i64 = (s_41_12 + s_41_13);
        // D s_41_15: write-var e <= s_41_14
        fn_state.e = s_41_14;
        // N s_41_16: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #1u : u8
        let s_42_0: bool = true;
        // D s_42_1: write-var gs#183957 <= s_42_0
        fn_state.gs_183957 = s_42_0;
        // N s_42_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #1u : u8
        let s_43_0: bool = true;
        // D s_43_1: write-var gs#183955 <= s_43_0
        fn_state.gs_183955 = s_43_0;
        // N s_43_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #1u : u8
        let s_44_0: bool = true;
        // D s_44_1: write-var gs#183958 <= s_44_0
        fn_state.gs_183958 = s_44_0;
        // N s_44_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var p2bits:i
        let s_45_0: i128 = fn_state.p2bits;
        // D s_45_1: call __id(s_45_0)
        let s_45_1: i128 = u__id(state, tracer, s_45_0);
        // D s_45_2: read-var esizeshadow#2414:i64
        let s_45_2: i64 = fn_state.esizeshadow_2414;
        // D s_45_3: cast zx s_45_2 -> i
        let s_45_3: i128 = (i128::try_from(s_45_2).unwrap());
        // D s_45_4: call __id(s_45_3)
        let s_45_4: i128 = u__id(state, tracer, s_45_3);
        // D s_45_5: cast reint s_45_4 -> i64
        let s_45_5: i64 = (s_45_4 as i64);
        // D s_45_6: cast zx s_45_5 -> i
        let s_45_6: i128 = (i128::try_from(s_45_5).unwrap());
        // D s_45_7: cmp-ge s_45_1 s_45_6
        let s_45_7: bool = ((s_45_1) >= (s_45_6));
        // D s_45_8: write-var gs#183952 <= s_45_7
        fn_state.gs_183952 = s_45_7;
        // N s_45_9: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #1u : u8
        let s_46_0: bool = true;
        // D s_46_1: write-var gs#183951 <= s_46_0
        fn_state.gs_183951 = s_46_0;
        // N s_46_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #1u : u8
        let s_47_0: bool = true;
        // D s_47_1: write-var gs#183949 <= s_47_0
        fn_state.gs_183949 = s_47_0;
        // N s_47_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #1u : u8
        let s_48_0: bool = true;
        // D s_48_1: write-var gs#183947 <= s_48_0
        fn_state.gs_183947 = s_48_0;
        // N s_48_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #1u : u8
        let s_49_0: bool = true;
        // D s_49_1: write-var gs#183945 <= s_49_0
        fn_state.gs_183945 = s_49_0;
        // N s_49_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #1u : u8
        let s_50_0: bool = true;
        // D s_50_1: write-var gs#183943 <= s_50_0
        fn_state.gs_183943 = s_50_0;
        // N s_50_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #1u : u8
        let s_51_0: bool = true;
        // D s_51_1: write-var gs#183941 <= s_51_0
        fn_state.gs_183941 = s_51_0;
        // N s_51_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #1u : u8
        let s_52_0: bool = true;
        // D s_52_1: write-var gs#183939 <= s_52_0
        fn_state.gs_183939 = s_52_0;
        // N s_52_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #128s : i64
        let s_53_0: i64 = 128;
        // D s_53_1: read-var d:i64
        let s_53_1: i64 = fn_state.d;
        // D s_53_2: cast zx s_53_1 -> i
        let s_53_2: i128 = (i128::try_from(s_53_1).unwrap());
        // D s_53_3: read-var result:u128
        let s_53_3: u128 = fn_state.result;
        // D s_53_4: cast zx s_53_3 -> bv
        let s_53_4: Bits = Bits::new(s_53_3 as u128, 128u16);
        // D s_53_5: call V_set(s_53_2, s_53_0, s_53_4)
        let s_53_5: () = V_set(state, tracer, s_53_2, s_53_0, s_53_4);
        // N s_53_6: return
        return;
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #1u : u8
        let s_54_0: bool = true;
        // D s_54_1: write-var gs#183918 <= s_54_0
        fn_state.gs_183918 = s_54_0;
        // N s_54_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #1u : u8
        let s_55_0: bool = true;
        // D s_55_1: write-var gs#183916 <= s_55_0
        fn_state.gs_183916 = s_55_0;
        // N s_55_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var VLshadow#2416:i64
        let s_56_0: i64 = fn_state.VLshadow_2416;
        // D s_56_1: cast zx s_56_0 -> i
        let s_56_1: i128 = (i128::try_from(s_56_0).unwrap());
        // D s_56_2: cast reint s_56_1 -> i64
        let s_56_2: i64 = (s_56_1 as i64);
        // D s_56_3: read-var n:i64
        let s_56_3: i64 = fn_state.n;
        // D s_56_4: cast zx s_56_3 -> i
        let s_56_4: i128 = (i128::try_from(s_56_3).unwrap());
        // D s_56_5: cast zx s_56_2 -> i
        let s_56_5: i128 = (i128::try_from(s_56_2).unwrap());
        // D s_56_6: call Z_read(s_56_4, s_56_5)
        let s_56_6: Bits = Z_read(state, tracer, s_56_4, s_56_5);
        // D s_56_7: write-var operand <= s_56_6
        fn_state.operand = s_56_6;
        // N s_56_8: jump b3
        return block_3(state, tracer, fn_state);
    }
}
