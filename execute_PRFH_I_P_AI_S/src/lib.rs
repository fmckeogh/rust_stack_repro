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
use ActivePredicateElement::*;
use Hint_Prefetch::*;
use Z_read::*;
use Elem_read::*;
use CheckNonStreamingSVEEnabled::*;
use P_read::*;
use AnyActiveElement::*;
use common::*;
pub fn execute_PRFH_I_P_AI_S<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    esize: i64,
    g: i64,
    level: i64,
    n: i64,
    offset: i64,
    pref_hint: u32,
    scale: i64,
    stream: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        VLshadow_4509: i64,
        base: Bits,
        VLshadow_4508: i64,
        elements: i64,
        gs_226386: i64,
        mask: Bits,
        baseshadow_4510: Bits,
        VL: i64,
        esize: i64,
        g: i64,
        level: i64,
        n: i64,
        offset: i64,
        pref_hint: u32,
        scale: i64,
        stream: bool,
    }
    let fn_state = FunctionState {
        VL,
        esize,
        g,
        level,
        n,
        offset,
        pref_hint,
        scale,
        stream,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var VL:i64
        let s_0_0: i64 = fn_state.VL;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var VLshadow#4508 <= s_0_2
        fn_state.VLshadow_4508 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckNonStreamingSVEEnabled(s_0_4)
        let s_0_5: () = CheckNonStreamingSVEEnabled(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#4508:i64
        let s_1_0: i64 = fn_state.VLshadow_4508;
        // D s_1_1: write-var VLshadow#4509 <= s_1_0
        fn_state.VLshadow_4509 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#4509:i64
        let s_1_3: i64 = fn_state.VLshadow_4509;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#4509:i64
        let s_1_7: i64 = fn_state.VLshadow_4509;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: read-var esize:i64
        let s_1_9: i64 = fn_state.esize;
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
        // D s_1_21: read-var esize:i64
        let s_1_21: i64 = fn_state.esize;
        // D s_1_22: cast zx s_1_21 -> i
        let s_1_22: i128 = (i128::try_from(s_1_21).unwrap());
        // D s_1_23: read-var mask:bv
        let s_1_23: Bits = fn_state.mask;
        // D s_1_24: call AnyActiveElement(s_1_23, s_1_22)
        let s_1_24: bool = AnyActiveElement(state, tracer, s_1_23, s_1_22);
        // N s_1_25: branch s_1_24 b10 b2
        if s_1_24 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_2_0: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var base:bv
        let s_3_0: Bits = fn_state.base;
        // D s_3_1: write-var baseshadow#4510 <= s_3_0
        fn_state.baseshadow_4510 = s_3_0;
        // C s_3_2: const #0s : i64
        let s_3_2: i64 = 0;
        // C s_3_3: const #1s : i
        let s_3_3: i128 = 1;
        // D s_3_4: read-var elements:i64
        let s_3_4: i64 = fn_state.elements;
        // D s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_6: sub s_3_5 s_3_3
        let s_3_6: i128 = ((s_3_5) - (s_3_3));
        // D s_3_7: cast reint s_3_6 -> i64
        let s_3_7: i64 = (s_3_6 as i64);
        // D s_3_8: write-var gs#226386 <= s_3_7
        fn_state.gs_226386 = s_3_7;
        // D s_3_9: write-var e <= s_3_2
        fn_state.e = s_3_2;
        // N s_3_10: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#226386:i64
        let s_4_1: i64 = fn_state.gs_226386;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b9 b5
        if s_4_2 {
            return block_9(state, tracer, fn_state);
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
        // D s_5_2: read-var esize:i64
        let s_5_2: i64 = fn_state.esize;
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
        // D s_8_0: read-var esize:i64
        let s_8_0: i64 = fn_state.esize;
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
        // D s_8_6: read-var baseshadow#4510:bv
        let s_8_6: Bits = fn_state.baseshadow_4510;
        // D s_8_7: call Elem_read(s_8_6, s_8_4, s_8_5)
        let s_8_7: Bits = Elem_read(state, tracer, s_8_6, s_8_4, s_8_5);
        // D s_8_8: cast reint s_8_7 -> u32
        let s_8_8: u32 = (s_8_7.value() as u32);
        // C s_8_9: const #64s : i
        let s_8_9: i128 = 64;
        // D s_8_10: cast zx s_8_8 -> bv
        let s_8_10: Bits = Bits::new(s_8_8 as u128, 32u16);
        // D s_8_11: bits-cast zx s_8_10 -> bv length s_8_9
        let s_8_11: Bits = s_8_10.zero_extend(s_8_9);
        // D s_8_12: cast reint s_8_11 -> u64
        let s_8_12: u64 = (s_8_11.value() as u64);
        // D s_8_13: read-var offset:i64
        let s_8_13: i64 = fn_state.offset;
        // D s_8_14: cast zx s_8_13 -> i
        let s_8_14: i128 = (i128::try_from(s_8_13).unwrap());
        // D s_8_15: read-var scale:i64
        let s_8_15: i64 = fn_state.scale;
        // D s_8_16: cast zx s_8_15 -> i
        let s_8_16: i128 = (i128::try_from(s_8_15).unwrap());
        // D s_8_17: lsl s_8_14 s_8_16
        let s_8_17: i128 = s_8_14 << s_8_16;
        // D s_8_18: cast zx s_8_12 -> bv
        let s_8_18: Bits = Bits::new(s_8_12 as u128, 64u16);
        // D s_8_19: cast cvt s_8_17 -> bv
        let s_8_19: Bits = Bits::new(s_8_17 as u128, 128);
        // D s_8_20: add s_8_18 s_8_19
        let s_8_20: Bits = (s_8_18 + s_8_19);
        // D s_8_21: cast reint s_8_20 -> u64
        let s_8_21: u64 = (s_8_20.value() as u64);
        // D s_8_22: read-var level:i64
        let s_8_22: i64 = fn_state.level;
        // D s_8_23: cast zx s_8_22 -> i
        let s_8_23: i128 = (i128::try_from(s_8_22).unwrap());
        // D s_8_24: read-var pref_hint:u32
        let s_8_24: u32 = fn_state.pref_hint;
        // D s_8_25: read-var stream:u8
        let s_8_25: bool = fn_state.stream;
        // D s_8_26: call Hint_Prefetch(s_8_21, s_8_24, s_8_23, s_8_25)
        let s_8_26: () = Hint_Prefetch(state, tracer, s_8_21, s_8_24, s_8_23, s_8_25);
        // N s_8_27: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var VLshadow#4509:i64
        let s_10_0: i64 = fn_state.VLshadow_4509;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: cast reint s_10_1 -> i64
        let s_10_2: i64 = (s_10_1 as i64);
        // D s_10_3: read-var n:i64
        let s_10_3: i64 = fn_state.n;
        // D s_10_4: cast zx s_10_3 -> i
        let s_10_4: i128 = (i128::try_from(s_10_3).unwrap());
        // D s_10_5: cast zx s_10_2 -> i
        let s_10_5: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_6: call Z_read(s_10_4, s_10_5)
        let s_10_6: Bits = Z_read(state, tracer, s_10_4, s_10_5);
        // D s_10_7: write-var base <= s_10_6
        fn_state.base = s_10_6;
        // N s_10_8: jump b3
        return block_3(state, tracer, fn_state);
    }
}
