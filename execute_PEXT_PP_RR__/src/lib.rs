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
pub fn execute_PEXT_PP_RR__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d0: i64,
    d1: i128,
    esize: i64,
    n: i64,
    part: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        u_5179: i64,
        gs_221645: i64,
        result1: Bits,
        result0: Bits,
        PL: i64,
        mask: Bits,
        psize: i64,
        elements: i64,
        gs_221653: i64,
        VL: i64,
        d0: i64,
        d1: i128,
        esize: i64,
        n: i64,
        part: i64,
    }
    let fn_state = FunctionState {
        VL,
        d0,
        d1,
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
        // D s_2_48: write-var gs#221645 <= s_2_47
        fn_state.gs_221645 = s_2_47;
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
        // D s_3_1: read-var gs#221645:i64
        let s_3_1: i64 = fn_state.gs_221645;
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
        // C s_4_0: const #2s : i
        let s_4_0: i128 = 2;
        // D s_4_1: read-var part:i64
        let s_4_1: i64 = fn_state.part;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: mul s_4_2 s_4_0
        let s_4_3: i128 = ((s_4_2) * (s_4_0));
        // D s_4_4: cast reint s_4_3 -> i64
        let s_4_4: i64 = (s_4_3 as i64);
        // D s_4_5: cast zx s_4_4 -> i
        let s_4_5: i128 = (i128::try_from(s_4_4).unwrap());
        // D s_4_6: read-var elements:i64
        let s_4_6: i64 = fn_state.elements;
        // D s_4_7: cast zx s_4_6 -> i
        let s_4_7: i128 = (i128::try_from(s_4_6).unwrap());
        // D s_4_8: mul s_4_5 s_4_7
        let s_4_8: i128 = ((s_4_5) * (s_4_7));
        // D s_4_9: cast reint s_4_8 -> i64
        let s_4_9: i64 = (s_4_8 as i64);
        // D s_4_10: cast zx s_4_9 -> i
        let s_4_10: i128 = (i128::try_from(s_4_9).unwrap());
        // D s_4_11: read-var e:i64
        let s_4_11: i64 = fn_state.e;
        // D s_4_12: cast zx s_4_11 -> i
        let s_4_12: i128 = (i128::try_from(s_4_11).unwrap());
        // D s_4_13: add s_4_10 s_4_12
        let s_4_13: i128 = (s_4_10 + s_4_12);
        // D s_4_14: cast reint s_4_13 -> i64
        let s_4_14: i64 = (s_4_13 as i64);
        // D s_4_15: cast zx s_4_14 -> i
        let s_4_15: i128 = (i128::try_from(s_4_14).unwrap());
        // D s_4_16: read-var esize:i64
        let s_4_16: i64 = fn_state.esize;
        // D s_4_17: cast zx s_4_16 -> i
        let s_4_17: i128 = (i128::try_from(s_4_16).unwrap());
        // D s_4_18: read-var mask:bv
        let s_4_18: Bits = fn_state.mask;
        // D s_4_19: call PredicateElement(s_4_18, s_4_15, s_4_17)
        let s_4_19: bool = PredicateElement(state, tracer, s_4_18, s_4_15, s_4_17);
        // D s_4_20: read-var psize:i64
        let s_4_20: i64 = fn_state.psize;
        // D s_4_21: cast zx s_4_20 -> i
        let s_4_21: i128 = (i128::try_from(s_4_20).unwrap());
        // D s_4_22: cast reint s_4_21 -> i64
        let s_4_22: i64 = (s_4_21 as i64);
        // D s_4_23: cast zx s_4_19 -> bv
        let s_4_23: Bits = Bits::new(s_4_19 as u128, 1u16);
        // D s_4_24: read-var psize:i64
        let s_4_24: i64 = fn_state.psize;
        // D s_4_25: cast zx s_4_24 -> i
        let s_4_25: i128 = (i128::try_from(s_4_24).unwrap());
        // D s_4_26: bits-cast zx s_4_23 -> bv length s_4_25
        let s_4_26: Bits = s_4_23.zero_extend(s_4_25);
        // D s_4_27: read-var e:i64
        let s_4_27: i64 = fn_state.e;
        // D s_4_28: cast zx s_4_27 -> i
        let s_4_28: i128 = (i128::try_from(s_4_27).unwrap());
        // D s_4_29: cast zx s_4_22 -> i
        let s_4_29: i128 = (i128::try_from(s_4_22).unwrap());
        // D s_4_30: read-var result0:bv
        let s_4_30: Bits = fn_state.result0;
        // D s_4_31: call Elem_set(s_4_30, s_4_28, s_4_29, s_4_26)
        let s_4_31: Bits = Elem_set(state, tracer, s_4_30, s_4_28, s_4_29, s_4_26);
        // D s_4_32: write-var result0 <= s_4_31
        fn_state.result0 = s_4_31;
        // D s_4_33: read-var e:i64
        let s_4_33: i64 = fn_state.e;
        // C s_4_34: const #1s : i64
        let s_4_34: i64 = 1;
        // D s_4_35: add s_4_33 s_4_34
        let s_4_35: i64 = (s_4_33 + s_4_34);
        // D s_4_36: write-var e <= s_4_35
        fn_state.e = s_4_35;
        // N s_4_37: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0s : i64
        let s_5_0: i64 = 0;
        // C s_5_1: const #1s : i
        let s_5_1: i128 = 1;
        // D s_5_2: read-var elements:i64
        let s_5_2: i64 = fn_state.elements;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: sub s_5_3 s_5_1
        let s_5_4: i128 = ((s_5_3) - (s_5_1));
        // D s_5_5: cast reint s_5_4 -> i64
        let s_5_5: i64 = (s_5_4 as i64);
        // D s_5_6: write-var gs#221653 <= s_5_5
        fn_state.gs_221653 = s_5_5;
        // D s_5_7: write-var u#5179 <= s_5_0
        fn_state.u_5179 = s_5_0;
        // N s_5_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var u#5179:i64
        let s_6_0: i64 = fn_state.u_5179;
        // D s_6_1: read-var gs#221653:i64
        let s_6_1: i64 = fn_state.gs_221653;
        // D s_6_2: cmp-gt s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) > (s_6_1));
        // N s_6_3: branch s_6_2 b8 b7
        if s_6_2 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #2s : i
        let s_7_0: i128 = 2;
        // D s_7_1: read-var part:i64
        let s_7_1: i64 = fn_state.part;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: mul s_7_2 s_7_0
        let s_7_3: i128 = ((s_7_2) * (s_7_0));
        // D s_7_4: cast reint s_7_3 -> i64
        let s_7_4: i64 = (s_7_3 as i64);
        // D s_7_5: cast zx s_7_4 -> i
        let s_7_5: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_6: read-var elements:i64
        let s_7_6: i64 = fn_state.elements;
        // D s_7_7: cast zx s_7_6 -> i
        let s_7_7: i128 = (i128::try_from(s_7_6).unwrap());
        // D s_7_8: mul s_7_5 s_7_7
        let s_7_8: i128 = ((s_7_5) * (s_7_7));
        // D s_7_9: cast reint s_7_8 -> i64
        let s_7_9: i64 = (s_7_8 as i64);
        // D s_7_10: cast zx s_7_9 -> i
        let s_7_10: i128 = (i128::try_from(s_7_9).unwrap());
        // D s_7_11: read-var elements:i64
        let s_7_11: i64 = fn_state.elements;
        // D s_7_12: cast zx s_7_11 -> i
        let s_7_12: i128 = (i128::try_from(s_7_11).unwrap());
        // D s_7_13: add s_7_10 s_7_12
        let s_7_13: i128 = (s_7_10 + s_7_12);
        // D s_7_14: cast reint s_7_13 -> i64
        let s_7_14: i64 = (s_7_13 as i64);
        // D s_7_15: cast zx s_7_14 -> i
        let s_7_15: i128 = (i128::try_from(s_7_14).unwrap());
        // D s_7_16: read-var u#5179:i64
        let s_7_16: i64 = fn_state.u_5179;
        // D s_7_17: cast zx s_7_16 -> i
        let s_7_17: i128 = (i128::try_from(s_7_16).unwrap());
        // D s_7_18: add s_7_15 s_7_17
        let s_7_18: i128 = (s_7_15 + s_7_17);
        // D s_7_19: cast reint s_7_18 -> i64
        let s_7_19: i64 = (s_7_18 as i64);
        // D s_7_20: cast zx s_7_19 -> i
        let s_7_20: i128 = (i128::try_from(s_7_19).unwrap());
        // D s_7_21: read-var esize:i64
        let s_7_21: i64 = fn_state.esize;
        // D s_7_22: cast zx s_7_21 -> i
        let s_7_22: i128 = (i128::try_from(s_7_21).unwrap());
        // D s_7_23: read-var mask:bv
        let s_7_23: Bits = fn_state.mask;
        // D s_7_24: call PredicateElement(s_7_23, s_7_20, s_7_22)
        let s_7_24: bool = PredicateElement(state, tracer, s_7_23, s_7_20, s_7_22);
        // D s_7_25: read-var psize:i64
        let s_7_25: i64 = fn_state.psize;
        // D s_7_26: cast zx s_7_25 -> i
        let s_7_26: i128 = (i128::try_from(s_7_25).unwrap());
        // D s_7_27: cast reint s_7_26 -> i64
        let s_7_27: i64 = (s_7_26 as i64);
        // D s_7_28: cast zx s_7_24 -> bv
        let s_7_28: Bits = Bits::new(s_7_24 as u128, 1u16);
        // D s_7_29: read-var psize:i64
        let s_7_29: i64 = fn_state.psize;
        // D s_7_30: cast zx s_7_29 -> i
        let s_7_30: i128 = (i128::try_from(s_7_29).unwrap());
        // D s_7_31: bits-cast zx s_7_28 -> bv length s_7_30
        let s_7_31: Bits = s_7_28.zero_extend(s_7_30);
        // D s_7_32: read-var u#5179:i64
        let s_7_32: i64 = fn_state.u_5179;
        // D s_7_33: cast zx s_7_32 -> i
        let s_7_33: i128 = (i128::try_from(s_7_32).unwrap());
        // D s_7_34: cast zx s_7_27 -> i
        let s_7_34: i128 = (i128::try_from(s_7_27).unwrap());
        // D s_7_35: read-var result1:bv
        let s_7_35: Bits = fn_state.result1;
        // D s_7_36: call Elem_set(s_7_35, s_7_33, s_7_34, s_7_31)
        let s_7_36: Bits = Elem_set(state, tracer, s_7_35, s_7_33, s_7_34, s_7_31);
        // D s_7_37: write-var result1 <= s_7_36
        fn_state.result1 = s_7_36;
        // D s_7_38: read-var u#5179:i64
        let s_7_38: i64 = fn_state.u_5179;
        // C s_7_39: const #1s : i64
        let s_7_39: i64 = 1;
        // D s_7_40: add s_7_38 s_7_39
        let s_7_40: i64 = (s_7_38 + s_7_39);
        // D s_7_41: write-var u#5179 <= s_7_40
        fn_state.u_5179 = s_7_40;
        // N s_7_42: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var PL:i64
        let s_8_0: i64 = fn_state.PL;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: cast reint s_8_1 -> i64
        let s_8_2: i64 = (s_8_1 as i64);
        // D s_8_3: read-var d0:i64
        let s_8_3: i64 = fn_state.d0;
        // D s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // D s_8_5: cast zx s_8_2 -> i
        let s_8_5: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_6: read-var result0:bv
        let s_8_6: Bits = fn_state.result0;
        // D s_8_7: call P_set(s_8_4, s_8_5, s_8_6)
        let s_8_7: () = P_set(state, tracer, s_8_4, s_8_5, s_8_6);
        // D s_8_8: read-var PL:i64
        let s_8_8: i64 = fn_state.PL;
        // D s_8_9: cast zx s_8_8 -> i
        let s_8_9: i128 = (i128::try_from(s_8_8).unwrap());
        // D s_8_10: cast reint s_8_9 -> i64
        let s_8_10: i64 = (s_8_9 as i64);
        // D s_8_11: cast zx s_8_10 -> i
        let s_8_11: i128 = (i128::try_from(s_8_10).unwrap());
        // D s_8_12: read-var d1:i
        let s_8_12: i128 = fn_state.d1;
        // D s_8_13: read-var result1:bv
        let s_8_13: Bits = fn_state.result1;
        // D s_8_14: call P_set(s_8_12, s_8_11, s_8_13)
        let s_8_14: () = P_set(state, tracer, s_8_12, s_8_11, s_8_13);
        // N s_8_15: return
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
