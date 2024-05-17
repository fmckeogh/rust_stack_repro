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
use CheckStreamingSVEEnabled::*;
use Elem_set::*;
use CheckSVEEnabled::*;
use P_read::*;
use HaveSVE2p1::*;
use CounterToPredicate::*;
use P_set::*;
use PredicateElement::*;
use common::*;
pub fn execute_PEXT_PN_RR__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    n: i64,
    part: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        psize: i64,
        elements: i64,
        result: Bits,
        mask: Bits,
        PL: i64,
        gs_221593: i64,
        VL: i64,
        d: i64,
        esize: i64,
        n: i64,
        part: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        n,
        part,
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
        // N s_0_2: branch s_0_1 b6 b1
        if s_0_1 {
            return block_6(state, tracer, fn_state);
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
        // D s_2_5: write-var PL <= s_2_4
        fn_state.PL = s_2_4;
        // D s_2_6: cast zx s_2_0 -> i
        let s_2_6: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_7: read-var esize:i64
        let s_2_7: i64 = fn_state.esize;
        // D s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (i128::try_from(s_2_7).unwrap());
        // D s_2_9: div s_2_6 s_2_8
        let s_2_9: i128 = ((s_2_6) / (s_2_8));
        // D s_2_10: cast reint s_2_9 -> i64
        let s_2_10: i64 = (s_2_9 as i64);
        // D s_2_11: write-var elements <= s_2_10
        fn_state.elements = s_2_10;
        // D s_2_12: read-var PL:i64
        let s_2_12: i64 = fn_state.PL;
        // D s_2_13: cast zx s_2_12 -> i
        let s_2_13: i128 = (i128::try_from(s_2_12).unwrap());
        // D s_2_14: cast reint s_2_13 -> i64
        let s_2_14: i64 = (s_2_13 as i64);
        // D s_2_15: read-var n:i64
        let s_2_15: i64 = fn_state.n;
        // D s_2_16: cast zx s_2_15 -> i
        let s_2_16: i128 = (i128::try_from(s_2_15).unwrap());
        // D s_2_17: cast zx s_2_14 -> i
        let s_2_17: i128 = (i128::try_from(s_2_14).unwrap());
        // D s_2_18: call P_read(s_2_16, s_2_17)
        let s_2_18: Bits = P_read(state, tracer, s_2_16, s_2_17);
        // C s_2_19: const #0s : i
        let s_2_19: i128 = 0;
        // C s_2_20: const #1s : i64
        let s_2_20: i64 = 1;
        // C s_2_21: cast zx s_2_20 -> i
        let s_2_21: i128 = (i128::try_from(s_2_20).unwrap());
        // C s_2_22: const #15s : i
        let s_2_22: i128 = 15;
        // C s_2_23: add s_2_22 s_2_21
        let s_2_23: i128 = (s_2_22 + s_2_21);
        // D s_2_24: bit-extract s_2_18 s_2_19 s_2_23
        let s_2_24: Bits = (Bits::new(
            ((s_2_18) >> (s_2_19)).value(),
            u16::try_from(s_2_23).unwrap(),
        ));
        // D s_2_25: cast reint s_2_24 -> u16
        let s_2_25: u16 = (s_2_24.value() as u16);
        // C s_2_26: const #4s : i
        let s_2_26: i128 = 4;
        // D s_2_27: read-var PL:i64
        let s_2_27: i64 = fn_state.PL;
        // D s_2_28: cast zx s_2_27 -> i
        let s_2_28: i128 = (i128::try_from(s_2_27).unwrap());
        // D s_2_29: mul s_2_28 s_2_26
        let s_2_29: i128 = ((s_2_28) * (s_2_26));
        // D s_2_30: cast reint s_2_29 -> i64
        let s_2_30: i64 = (s_2_29 as i64);
        // D s_2_31: cast zx s_2_30 -> i
        let s_2_31: i128 = (i128::try_from(s_2_30).unwrap());
        // D s_2_32: cast reint s_2_31 -> i64
        let s_2_32: i64 = (s_2_31 as i64);
        // D s_2_33: cast zx s_2_32 -> i
        let s_2_33: i128 = (i128::try_from(s_2_32).unwrap());
        // D s_2_34: call CounterToPredicate(s_2_25, s_2_33)
        let s_2_34: Bits = CounterToPredicate(state, tracer, s_2_25, s_2_33);
        // D s_2_35: write-var mask <= s_2_34
        fn_state.mask = s_2_34;
        // C s_2_36: const #8s : i
        let s_2_36: i128 = 8;
        // D s_2_37: read-var esize:i64
        let s_2_37: i64 = fn_state.esize;
        // D s_2_38: cast zx s_2_37 -> i
        let s_2_38: i128 = (i128::try_from(s_2_37).unwrap());
        // D s_2_39: div s_2_38 s_2_36
        let s_2_39: i128 = ((s_2_38) / (s_2_36));
        // D s_2_40: cast reint s_2_39 -> i64
        let s_2_40: i64 = (s_2_39 as i64);
        // D s_2_41: write-var psize <= s_2_40
        fn_state.psize = s_2_40;
        // C s_2_42: const #0s : i64
        let s_2_42: i64 = 0;
        // C s_2_43: const #1s : i
        let s_2_43: i128 = 1;
        // D s_2_44: read-var elements:i64
        let s_2_44: i64 = fn_state.elements;
        // D s_2_45: cast zx s_2_44 -> i
        let s_2_45: i128 = (i128::try_from(s_2_44).unwrap());
        // D s_2_46: sub s_2_45 s_2_43
        let s_2_46: i128 = ((s_2_45) - (s_2_43));
        // D s_2_47: cast reint s_2_46 -> i64
        let s_2_47: i64 = (s_2_46 as i64);
        // D s_2_48: write-var gs#221593 <= s_2_47
        fn_state.gs_221593 = s_2_47;
        // D s_2_49: write-var e <= s_2_42
        fn_state.e = s_2_42;
        // N s_2_50: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var e:i64
        let s_3_0: i64 = fn_state.e;
        // D s_3_1: read-var gs#221593:i64
        let s_3_1: i64 = fn_state.gs_221593;
        // D s_3_2: cmp-gt s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) > (s_3_1));
        // N s_3_3: branch s_3_2 b5 b4
        if s_3_2 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var part:i64
        let s_4_0: i64 = fn_state.part;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: read-var elements:i64
        let s_4_2: i64 = fn_state.elements;
        // D s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_4: mul s_4_1 s_4_3
        let s_4_4: i128 = ((s_4_1) * (s_4_3));
        // D s_4_5: cast reint s_4_4 -> i64
        let s_4_5: i64 = (s_4_4 as i64);
        // D s_4_6: cast zx s_4_5 -> i
        let s_4_6: i128 = (i128::try_from(s_4_5).unwrap());
        // D s_4_7: read-var e:i64
        let s_4_7: i64 = fn_state.e;
        // D s_4_8: cast zx s_4_7 -> i
        let s_4_8: i128 = (i128::try_from(s_4_7).unwrap());
        // D s_4_9: add s_4_6 s_4_8
        let s_4_9: i128 = (s_4_6 + s_4_8);
        // D s_4_10: cast reint s_4_9 -> i64
        let s_4_10: i64 = (s_4_9 as i64);
        // D s_4_11: cast zx s_4_10 -> i
        let s_4_11: i128 = (i128::try_from(s_4_10).unwrap());
        // D s_4_12: read-var esize:i64
        let s_4_12: i64 = fn_state.esize;
        // D s_4_13: cast zx s_4_12 -> i
        let s_4_13: i128 = (i128::try_from(s_4_12).unwrap());
        // D s_4_14: read-var mask:bv
        let s_4_14: Bits = fn_state.mask;
        // D s_4_15: call PredicateElement(s_4_14, s_4_11, s_4_13)
        let s_4_15: bool = PredicateElement(state, tracer, s_4_14, s_4_11, s_4_13);
        // D s_4_16: read-var psize:i64
        let s_4_16: i64 = fn_state.psize;
        // D s_4_17: cast zx s_4_16 -> i
        let s_4_17: i128 = (i128::try_from(s_4_16).unwrap());
        // D s_4_18: cast reint s_4_17 -> i64
        let s_4_18: i64 = (s_4_17 as i64);
        // D s_4_19: cast zx s_4_15 -> bv
        let s_4_19: Bits = Bits::new(s_4_15 as u128, 1u16);
        // D s_4_20: read-var psize:i64
        let s_4_20: i64 = fn_state.psize;
        // D s_4_21: cast zx s_4_20 -> i
        let s_4_21: i128 = (i128::try_from(s_4_20).unwrap());
        // D s_4_22: bits-cast zx s_4_19 -> bv length s_4_21
        let s_4_22: Bits = s_4_19.zero_extend(s_4_21);
        // D s_4_23: read-var e:i64
        let s_4_23: i64 = fn_state.e;
        // D s_4_24: cast zx s_4_23 -> i
        let s_4_24: i128 = (i128::try_from(s_4_23).unwrap());
        // D s_4_25: cast zx s_4_18 -> i
        let s_4_25: i128 = (i128::try_from(s_4_18).unwrap());
        // D s_4_26: read-var result:bv
        let s_4_26: Bits = fn_state.result;
        // D s_4_27: call Elem_set(s_4_26, s_4_24, s_4_25, s_4_22)
        let s_4_27: Bits = Elem_set(state, tracer, s_4_26, s_4_24, s_4_25, s_4_22);
        // D s_4_28: write-var result <= s_4_27
        fn_state.result = s_4_27;
        // D s_4_29: read-var e:i64
        let s_4_29: i64 = fn_state.e;
        // C s_4_30: const #1s : i64
        let s_4_30: i64 = 1;
        // D s_4_31: add s_4_29 s_4_30
        let s_4_31: i64 = (s_4_29 + s_4_30);
        // D s_4_32: write-var e <= s_4_31
        fn_state.e = s_4_31;
        // N s_4_33: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var PL:i64
        let s_5_0: i64 = fn_state.PL;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: cast reint s_5_1 -> i64
        let s_5_2: i64 = (s_5_1 as i64);
        // D s_5_3: read-var d:i64
        let s_5_3: i64 = fn_state.d;
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_5: cast zx s_5_2 -> i
        let s_5_5: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_6: read-var result:bv
        let s_5_6: Bits = fn_state.result;
        // D s_5_7: call P_set(s_5_4, s_5_5, s_5_6)
        let s_5_7: () = P_set(state, tracer, s_5_4, s_5_5, s_5_6);
        // N s_5_8: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call CheckSVEEnabled(s_6_0)
        let s_6_1: () = CheckSVEEnabled(state, tracer, s_6_0);
        // N s_6_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
