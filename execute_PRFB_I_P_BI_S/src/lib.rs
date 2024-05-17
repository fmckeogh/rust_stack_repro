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
use Hint_Prefetch::*;
use X_read::*;
use SP_read::*;
use CheckSVEEnabled::*;
use AnyActiveElement::*;
use P_read::*;
use ActivePredicateElement::*;
use common::*;
pub fn execute_PRFB_I_P_BI_S<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    esize: i64,
    g: i64,
    level: i64,
    n: i64,
    offset: i128,
    pref_hint: u32,
    scale: i64,
    stream: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_225822: i64,
        e: i64,
        base: u64,
        elements: i64,
        mask: Bits,
        baseshadow_4486: u64,
        VL: i64,
        esize: i64,
        g: i64,
        level: i64,
        n: i64,
        offset: i128,
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
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call CheckSVEEnabled(s_0_0)
        let s_0_1: () = CheckSVEEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VL:i64
        let s_1_0: i64 = fn_state.VL;
        // C s_1_1: const #8s : i
        let s_1_1: i128 = 8;
        // D s_1_2: cast zx s_1_0 -> i
        let s_1_2: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_3: div s_1_2 s_1_1
        let s_1_3: i128 = ((s_1_2) / (s_1_1));
        // D s_1_4: cast reint s_1_3 -> i64
        let s_1_4: i64 = (s_1_3 as i64);
        // D s_1_5: cast zx s_1_0 -> i
        let s_1_5: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_6: read-var esize:i64
        let s_1_6: i64 = fn_state.esize;
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_8: div s_1_5 s_1_7
        let s_1_8: i128 = ((s_1_5) / (s_1_7));
        // D s_1_9: cast reint s_1_8 -> i64
        let s_1_9: i64 = (s_1_8 as i64);
        // D s_1_10: write-var elements <= s_1_9
        fn_state.elements = s_1_9;
        // D s_1_11: cast zx s_1_4 -> i
        let s_1_11: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_12: cast reint s_1_11 -> i64
        let s_1_12: i64 = (s_1_11 as i64);
        // D s_1_13: read-var g:i64
        let s_1_13: i64 = fn_state.g;
        // D s_1_14: cast zx s_1_13 -> i
        let s_1_14: i128 = (i128::try_from(s_1_13).unwrap());
        // D s_1_15: cast zx s_1_12 -> i
        let s_1_15: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_16: call P_read(s_1_14, s_1_15)
        let s_1_16: Bits = P_read(state, tracer, s_1_14, s_1_15);
        // D s_1_17: write-var mask <= s_1_16
        fn_state.mask = s_1_16;
        // D s_1_18: read-var esize:i64
        let s_1_18: i64 = fn_state.esize;
        // D s_1_19: cast zx s_1_18 -> i
        let s_1_19: i128 = (i128::try_from(s_1_18).unwrap());
        // D s_1_20: read-var mask:bv
        let s_1_20: Bits = fn_state.mask;
        // D s_1_21: call AnyActiveElement(s_1_20, s_1_19)
        let s_1_21: bool = AnyActiveElement(state, tracer, s_1_20, s_1_19);
        // N s_1_22: branch s_1_21 b10 b2
        if s_1_21 {
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
        // D s_3_0: read-var base:u64
        let s_3_0: u64 = fn_state.base;
        // D s_3_1: write-var baseshadow#4486 <= s_3_0
        fn_state.baseshadow_4486 = s_3_0;
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
        // D s_3_8: write-var gs#225822 <= s_3_7
        fn_state.gs_225822 = s_3_7;
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
        // D s_4_1: read-var gs#225822:i64
        let s_4_1: i64 = fn_state.gs_225822;
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
        // D s_8_0: read-var elements:i64
        let s_8_0: i64 = fn_state.elements;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: read-var offset:i
        let s_8_2: i128 = fn_state.offset;
        // D s_8_3: mul s_8_2 s_8_1
        let s_8_3: i128 = ((s_8_2) * (s_8_1));
        // D s_8_4: read-var e:i64
        let s_8_4: i64 = fn_state.e;
        // D s_8_5: cast zx s_8_4 -> i
        let s_8_5: i128 = (i128::try_from(s_8_4).unwrap());
        // D s_8_6: add s_8_3 s_8_5
        let s_8_6: i128 = (s_8_3 + s_8_5);
        // D s_8_7: read-var scale:i64
        let s_8_7: i64 = fn_state.scale;
        // D s_8_8: cast zx s_8_7 -> i
        let s_8_8: i128 = (i128::try_from(s_8_7).unwrap());
        // D s_8_9: lsl s_8_6 s_8_8
        let s_8_9: i128 = s_8_6 << s_8_8;
        // D s_8_10: read-var baseshadow#4486:u64
        let s_8_10: u64 = fn_state.baseshadow_4486;
        // D s_8_11: cast zx s_8_10 -> bv
        let s_8_11: Bits = Bits::new(s_8_10 as u128, 64u16);
        // D s_8_12: cast cvt s_8_9 -> bv
        let s_8_12: Bits = Bits::new(s_8_9 as u128, 128);
        // D s_8_13: add s_8_11 s_8_12
        let s_8_13: Bits = (s_8_11 + s_8_12);
        // D s_8_14: cast reint s_8_13 -> u64
        let s_8_14: u64 = (s_8_13.value() as u64);
        // D s_8_15: read-var level:i64
        let s_8_15: i64 = fn_state.level;
        // D s_8_16: cast zx s_8_15 -> i
        let s_8_16: i128 = (i128::try_from(s_8_15).unwrap());
        // D s_8_17: read-var pref_hint:u32
        let s_8_17: u32 = fn_state.pref_hint;
        // D s_8_18: read-var stream:u8
        let s_8_18: bool = fn_state.stream;
        // D s_8_19: call Hint_Prefetch(s_8_14, s_8_17, s_8_16, s_8_18)
        let s_8_19: () = Hint_Prefetch(state, tracer, s_8_14, s_8_17, s_8_16, s_8_18);
        // N s_8_20: jump b7
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
        // C s_10_0: const #31s : i
        let s_10_0: i128 = 31;
        // D s_10_1: read-var n:i64
        let s_10_1: i64 = fn_state.n;
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // D s_10_3: cmp-eq s_10_2 s_10_0
        let s_10_3: bool = ((s_10_2) == (s_10_0));
        // N s_10_4: branch s_10_3 b13 b11
        if s_10_3 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #64s : i64
        let s_11_0: i64 = 64;
        // D s_11_1: read-var n:i64
        let s_11_1: i64 = fn_state.n;
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // D s_11_3: call X_read(s_11_2, s_11_0)
        let s_11_3: Bits = X_read(state, tracer, s_11_2, s_11_0);
        // D s_11_4: cast reint s_11_3 -> u64
        let s_11_4: u64 = (s_11_3.value() as u64);
        // D s_11_5: write-var base <= s_11_4
        fn_state.base = s_11_4;
        // N s_11_6: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call SP_read(s_13_0)
        let s_13_1: u64 = SP_read(state, tracer, s_13_0);
        // D s_13_2: write-var base <= s_13_1
        fn_state.base = s_13_1;
        // N s_13_3: jump b12
        return block_12(state, tracer, fn_state);
    }
}
