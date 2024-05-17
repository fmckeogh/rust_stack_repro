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
use HaveSVE2p1::*;
use X_set::*;
use CheckStreamingSVEEnabled::*;
use CounterToPredicate::*;
use CheckSVEEnabled::*;
use P_read::*;
use ActivePredicateElement::*;
use common::*;
pub fn execute_CNTP_R_PN__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    n: i64,
    width: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        sum: u64,
        gs_221745: i64,
        mask: Bits,
        VL: i64,
        d: i64,
        esize: i64,
        n: i64,
        width: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        n,
        width,
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
        // S s_0_1: call HaveSVE2p1(s_0_0)
        let s_0_1: bool = HaveSVE2p1(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b9 b1
        if s_0_1 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call CheckStreamingSVEEnabled(s_1_0)
        let s_1_1: () = CheckStreamingSVEEnabled(state, tracer, s_1_0);
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var VL:i64
        let s_2_0: i64 = fn_state.VL;
        // C s_2_1: const #8s : i
        let s_2_1: i128 = 8;
        // D s_2_2: cast zx s_2_0 -> i
        let s_2_2: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_3: div s_2_2 s_2_1
        let s_2_3: i128 = ((s_2_2) / (s_2_1));
        // D s_2_4: cast reint s_2_3 -> i64
        let s_2_4: i64 = (s_2_3 as i64);
        // D s_2_5: cast zx s_2_0 -> i
        let s_2_5: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_6: read-var esize:i64
        let s_2_6: i64 = fn_state.esize;
        // D s_2_7: cast zx s_2_6 -> i
        let s_2_7: i128 = (i128::try_from(s_2_6).unwrap());
        // D s_2_8: div s_2_5 s_2_7
        let s_2_8: i128 = ((s_2_5) / (s_2_7));
        // D s_2_9: cast reint s_2_8 -> i64
        let s_2_9: i64 = (s_2_8 as i64);
        // D s_2_10: cast zx s_2_4 -> i
        let s_2_10: i128 = (i128::try_from(s_2_4).unwrap());
        // D s_2_11: cast reint s_2_10 -> i64
        let s_2_11: i64 = (s_2_10 as i64);
        // D s_2_12: read-var n:i64
        let s_2_12: i64 = fn_state.n;
        // D s_2_13: cast zx s_2_12 -> i
        let s_2_13: i128 = (i128::try_from(s_2_12).unwrap());
        // D s_2_14: cast zx s_2_11 -> i
        let s_2_14: i128 = (i128::try_from(s_2_11).unwrap());
        // D s_2_15: call P_read(s_2_13, s_2_14)
        let s_2_15: Bits = P_read(state, tracer, s_2_13, s_2_14);
        // C s_2_16: const #0s : i
        let s_2_16: i128 = 0;
        // C s_2_17: const #1s : i64
        let s_2_17: i64 = 1;
        // C s_2_18: cast zx s_2_17 -> i
        let s_2_18: i128 = (i128::try_from(s_2_17).unwrap());
        // C s_2_19: const #15s : i
        let s_2_19: i128 = 15;
        // C s_2_20: add s_2_19 s_2_18
        let s_2_20: i128 = (s_2_19 + s_2_18);
        // D s_2_21: bit-extract s_2_15 s_2_16 s_2_20
        let s_2_21: Bits = (Bits::new(
            ((s_2_15) >> (s_2_16)).value(),
            u16::try_from(s_2_20).unwrap(),
        ));
        // D s_2_22: cast reint s_2_21 -> u16
        let s_2_22: u16 = (s_2_21.value() as u16);
        // C s_2_23: const #4s : i
        let s_2_23: i128 = 4;
        // D s_2_24: cast zx s_2_4 -> i
        let s_2_24: i128 = (i128::try_from(s_2_4).unwrap());
        // D s_2_25: mul s_2_24 s_2_23
        let s_2_25: i128 = ((s_2_24) * (s_2_23));
        // D s_2_26: cast reint s_2_25 -> i64
        let s_2_26: i64 = (s_2_25 as i64);
        // D s_2_27: cast zx s_2_26 -> i
        let s_2_27: i128 = (i128::try_from(s_2_26).unwrap());
        // D s_2_28: cast reint s_2_27 -> i64
        let s_2_28: i64 = (s_2_27 as i64);
        // D s_2_29: cast zx s_2_28 -> i
        let s_2_29: i128 = (i128::try_from(s_2_28).unwrap());
        // D s_2_30: call CounterToPredicate(s_2_22, s_2_29)
        let s_2_30: Bits = CounterToPredicate(state, tracer, s_2_22, s_2_29);
        // D s_2_31: write-var mask <= s_2_30
        fn_state.mask = s_2_30;
        // C s_2_32: const #0u : u64
        let s_2_32: u64 = 0;
        // D s_2_33: write-var sum <= s_2_32
        fn_state.sum = s_2_32;
        // D s_2_34: cast zx s_2_9 -> i
        let s_2_34: i128 = (i128::try_from(s_2_9).unwrap());
        // D s_2_35: read-var width:i64
        let s_2_35: i64 = fn_state.width;
        // D s_2_36: cast zx s_2_35 -> i
        let s_2_36: i128 = (i128::try_from(s_2_35).unwrap());
        // D s_2_37: mul s_2_34 s_2_36
        let s_2_37: i128 = ((s_2_34) * (s_2_36));
        // D s_2_38: cast reint s_2_37 -> i64
        let s_2_38: i64 = (s_2_37 as i64);
        // C s_2_39: const #0s : i64
        let s_2_39: i64 = 0;
        // C s_2_40: const #1s : i
        let s_2_40: i128 = 1;
        // D s_2_41: cast zx s_2_38 -> i
        let s_2_41: i128 = (i128::try_from(s_2_38).unwrap());
        // D s_2_42: sub s_2_41 s_2_40
        let s_2_42: i128 = ((s_2_41) - (s_2_40));
        // D s_2_43: cast reint s_2_42 -> i64
        let s_2_43: i64 = (s_2_42 as i64);
        // D s_2_44: write-var gs#221745 <= s_2_43
        fn_state.gs_221745 = s_2_43;
        // D s_2_45: write-var e <= s_2_39
        fn_state.e = s_2_39;
        // N s_2_46: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var e:i64
        let s_3_0: i64 = fn_state.e;
        // D s_3_1: read-var gs#221745:i64
        let s_3_1: i64 = fn_state.gs_221745;
        // D s_3_2: cmp-gt s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) > (s_3_1));
        // N s_3_3: branch s_3_2 b8 b4
        if s_3_2 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: read-var esize:i64
        let s_4_2: i64 = fn_state.esize;
        // D s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_4: read-var mask:bv
        let s_4_4: Bits = fn_state.mask;
        // D s_4_5: call ActivePredicateElement(s_4_4, s_4_1, s_4_3)
        let s_4_5: bool = ActivePredicateElement(state, tracer, s_4_4, s_4_1, s_4_3);
        // N s_4_6: branch s_4_5 b7 b5
        if s_4_5 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var e:i64
        let s_6_0: i64 = fn_state.e;
        // C s_6_1: const #1s : i64
        let s_6_1: i64 = 1;
        // D s_6_2: add s_6_0 s_6_1
        let s_6_2: i64 = (s_6_0 + s_6_1);
        // D s_6_3: write-var e <= s_6_2
        fn_state.e = s_6_2;
        // N s_6_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #1s : i
        let s_7_0: i128 = 1;
        // D s_7_1: read-var sum:u64
        let s_7_1: u64 = fn_state.sum;
        // D s_7_2: cast zx s_7_1 -> bv
        let s_7_2: Bits = Bits::new(s_7_1 as u128, 64u16);
        // C s_7_3: cast cvt s_7_0 -> bv
        let s_7_3: Bits = Bits::new(s_7_0 as u128, 128);
        // D s_7_4: add s_7_2 s_7_3
        let s_7_4: Bits = (s_7_2 + s_7_3);
        // D s_7_5: cast reint s_7_4 -> u64
        let s_7_5: u64 = (s_7_4.value() as u64);
        // D s_7_6: write-var sum <= s_7_5
        fn_state.sum = s_7_5;
        // N s_7_7: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #64s : i64
        let s_8_0: i64 = 64;
        // D s_8_1: read-var d:i64
        let s_8_1: i64 = fn_state.d;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // D s_8_3: read-var sum:u64
        let s_8_3: u64 = fn_state.sum;
        // D s_8_4: cast zx s_8_3 -> bv
        let s_8_4: Bits = Bits::new(s_8_3 as u128, 64u16);
        // D s_8_5: call X_set(s_8_2, s_8_0, s_8_4)
        let s_8_5: () = X_set(state, tracer, s_8_2, s_8_0, s_8_4);
        // N s_8_6: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call CheckSVEEnabled(s_9_0)
        let s_9_1: () = CheckSVEEnabled(state, tracer, s_9_0);
        // N s_9_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
