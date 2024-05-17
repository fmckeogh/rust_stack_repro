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
use CheckSVEEnabled::*;
use P_set::*;
use P_read::*;
use PredicateElement::*;
use common::*;
pub fn execute_PUNPKHI_P_P__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    hi: bool,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        e: i64,
        psize: i64,
        elements: i64,
        result: Bits,
        ga_284813: i64,
        PL: i64,
        gs_197028: i64,
        VL: i64,
        d: i64,
        esize: i64,
        hi: bool,
        n: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        hi,
        n,
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
        // D s_1_5: write-var PL <= s_1_4
        fn_state.PL = s_1_4;
        // D s_1_6: cast zx s_1_0 -> i
        let s_1_6: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_7: read-var esize:i64
        let s_1_7: i64 = fn_state.esize;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: div s_1_6 s_1_8
        let s_1_9: i128 = ((s_1_6) / (s_1_8));
        // D s_1_10: cast reint s_1_9 -> i64
        let s_1_10: i64 = (s_1_9 as i64);
        // D s_1_11: write-var elements <= s_1_10
        fn_state.elements = s_1_10;
        // D s_1_12: read-var PL:i64
        let s_1_12: i64 = fn_state.PL;
        // D s_1_13: cast zx s_1_12 -> i
        let s_1_13: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_14: cast reint s_1_13 -> i64
        let s_1_14: i64 = (s_1_13 as i64);
        // D s_1_15: read-var n:i64
        let s_1_15: i64 = fn_state.n;
        // D s_1_16: cast zx s_1_15 -> i
        let s_1_16: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_17: cast zx s_1_14 -> i
        let s_1_17: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_18: call P_read(s_1_16, s_1_17)
        let s_1_18: Bits = P_read(state, tracer, s_1_16, s_1_17);
        // D s_1_19: write-var operand <= s_1_18
        fn_state.operand = s_1_18;
        // C s_1_20: const #8s : i
        let s_1_20: i128 = 8;
        // D s_1_21: read-var esize:i64
        let s_1_21: i64 = fn_state.esize;
        // D s_1_22: cast zx s_1_21 -> i
        let s_1_22: i128 = (i128::try_from(s_1_21).unwrap());
        // D s_1_23: div s_1_22 s_1_20
        let s_1_23: i128 = ((s_1_22) / (s_1_20));
        // D s_1_24: cast reint s_1_23 -> i64
        let s_1_24: i64 = (s_1_23 as i64);
        // D s_1_25: write-var psize <= s_1_24
        fn_state.psize = s_1_24;
        // C s_1_26: const #0s : i64
        let s_1_26: i64 = 0;
        // C s_1_27: const #1s : i
        let s_1_27: i128 = 1;
        // D s_1_28: read-var elements:i64
        let s_1_28: i64 = fn_state.elements;
        // D s_1_29: cast zx s_1_28 -> i
        let s_1_29: i128 = (i128::try_from(s_1_28).unwrap());
        // D s_1_30: sub s_1_29 s_1_27
        let s_1_30: i128 = ((s_1_29) - (s_1_27));
        // D s_1_31: cast reint s_1_30 -> i64
        let s_1_31: i64 = (s_1_30 as i64);
        // D s_1_32: write-var gs#197028 <= s_1_31
        fn_state.gs_197028 = s_1_31;
        // D s_1_33: write-var e <= s_1_26
        fn_state.e = s_1_26;
        // N s_1_34: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#197028:i64
        let s_2_1: i64 = fn_state.gs_197028;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b7 b3
        if s_2_2 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var hi:u8
        let s_3_0: bool = fn_state.hi;
        // N s_3_1: branch s_3_0 b6 b4
        if s_3_0 {
            return block_6(state, tracer, fn_state);
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
        // D s_4_1: write-var ga#284813 <= s_4_0
        fn_state.ga_284813 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #2s : i
        let s_5_0: i128 = 2;
        // D s_5_1: read-var esize:i64
        let s_5_1: i64 = fn_state.esize;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: div s_5_2 s_5_0
        let s_5_3: i128 = ((s_5_2) / (s_5_0));
        // D s_5_4: cast reint s_5_3 -> i64
        let s_5_4: i64 = (s_5_3 as i64);
        // D s_5_5: read-var ga#284813:i64
        let s_5_5: i64 = fn_state.ga_284813;
        // D s_5_6: cast zx s_5_5 -> i
        let s_5_6: i128 = (i128::try_from(s_5_5).unwrap());
        // D s_5_7: cast zx s_5_4 -> i
        let s_5_7: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_8: read-var operand:bv
        let s_5_8: Bits = fn_state.operand;
        // D s_5_9: call PredicateElement(s_5_8, s_5_6, s_5_7)
        let s_5_9: bool = PredicateElement(state, tracer, s_5_8, s_5_6, s_5_7);
        // D s_5_10: read-var psize:i64
        let s_5_10: i64 = fn_state.psize;
        // D s_5_11: cast zx s_5_10 -> i
        let s_5_11: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_12: cast reint s_5_11 -> i64
        let s_5_12: i64 = (s_5_11 as i64);
        // D s_5_13: cast zx s_5_9 -> bv
        let s_5_13: Bits = Bits::new(s_5_9 as u128, 1u16);
        // D s_5_14: read-var psize:i64
        let s_5_14: i64 = fn_state.psize;
        // D s_5_15: cast zx s_5_14 -> i
        let s_5_15: i128 = (i128::try_from(s_5_14).unwrap());
        // D s_5_16: bits-cast zx s_5_13 -> bv length s_5_15
        let s_5_16: Bits = s_5_13.zero_extend(s_5_15);
        // D s_5_17: cast reint s_5_16 -> u8
        let s_5_17: u8 = (s_5_16.value() as u8);
        // D s_5_18: read-var e:i64
        let s_5_18: i64 = fn_state.e;
        // D s_5_19: cast zx s_5_18 -> i
        let s_5_19: i128 = (i128::try_from(s_5_18).unwrap());
        // D s_5_20: cast zx s_5_12 -> i
        let s_5_20: i128 = (i128::try_from(s_5_12).unwrap());
        // D s_5_21: cast zx s_5_17 -> bv
        let s_5_21: Bits = Bits::new(s_5_17 as u128, 2u16);
        // D s_5_22: read-var result:bv
        let s_5_22: Bits = fn_state.result;
        // D s_5_23: call Elem_set(s_5_22, s_5_19, s_5_20, s_5_21)
        let s_5_23: Bits = Elem_set(state, tracer, s_5_22, s_5_19, s_5_20, s_5_21);
        // D s_5_24: write-var result <= s_5_23
        fn_state.result = s_5_23;
        // D s_5_25: read-var e:i64
        let s_5_25: i64 = fn_state.e;
        // C s_5_26: const #1s : i64
        let s_5_26: i64 = 1;
        // D s_5_27: add s_5_25 s_5_26
        let s_5_27: i64 = (s_5_25 + s_5_26);
        // D s_5_28: write-var e <= s_5_27
        fn_state.e = s_5_27;
        // N s_5_29: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var e:i64
        let s_6_0: i64 = fn_state.e;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: read-var elements:i64
        let s_6_2: i64 = fn_state.elements;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: add s_6_1 s_6_3
        let s_6_4: i128 = (s_6_1 + s_6_3);
        // D s_6_5: cast reint s_6_4 -> i64
        let s_6_5: i64 = (s_6_4 as i64);
        // D s_6_6: write-var ga#284813 <= s_6_5
        fn_state.ga_284813 = s_6_5;
        // N s_6_7: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var PL:i64
        let s_7_0: i64 = fn_state.PL;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: cast reint s_7_1 -> i64
        let s_7_2: i64 = (s_7_1 as i64);
        // D s_7_3: read-var d:i64
        let s_7_3: i64 = fn_state.d;
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_5: cast zx s_7_2 -> i
        let s_7_5: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_6: read-var result:bv
        let s_7_6: Bits = fn_state.result;
        // D s_7_7: call P_set(s_7_4, s_7_5, s_7_6)
        let s_7_7: () = P_set(state, tracer, s_7_4, s_7_5, s_7_6);
        // N s_7_8: return
        return;
    }
}
