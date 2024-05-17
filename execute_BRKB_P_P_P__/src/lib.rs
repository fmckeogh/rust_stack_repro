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
use PredicateElement::*;
use PredTest::*;
use Elem_set::*;
use CheckSVEEnabled::*;
use P_set::*;
use P_read::*;
use ActivePredicateElement::*;
use common::*;
pub fn execute_BRKB_P_P_P__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    g: i64,
    merging: bool,
    n: i64,
    setflags: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        e: i64,
        gs_197928: bool,
        gs_197924: i64,
        PL: i64,
        mask: Bits,
        pbit: bool,
        psize: i64,
        _break: bool,
        element: bool,
        result: Bits,
        operand2: Bits,
        VL: i64,
        d: i64,
        esize: i64,
        g: i64,
        merging: bool,
        n: i64,
        setflags: bool,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        g,
        merging,
        n,
        setflags,
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
        // D s_1_11: read-var PL:i64
        let s_1_11: i64 = fn_state.PL;
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_13: cast reint s_1_12 -> i64
        let s_1_13: i64 = (s_1_12 as i64);
        // D s_1_14: read-var g:i64
        let s_1_14: i64 = fn_state.g;
        // D s_1_15: cast zx s_1_14 -> i
        let s_1_15: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_16: cast zx s_1_13 -> i
        let s_1_16: i128 = (i128::try_from(s_1_13).unwrap());
        // D s_1_17: call P_read(s_1_15, s_1_16)
        let s_1_17: Bits = P_read(state, tracer, s_1_15, s_1_16);
        // D s_1_18: write-var mask <= s_1_17
        fn_state.mask = s_1_17;
        // D s_1_19: read-var PL:i64
        let s_1_19: i64 = fn_state.PL;
        // D s_1_20: cast zx s_1_19 -> i
        let s_1_20: i128 = (i128::try_from(s_1_19).unwrap());
        // D s_1_21: cast reint s_1_20 -> i64
        let s_1_21: i64 = (s_1_20 as i64);
        // D s_1_22: read-var n:i64
        let s_1_22: i64 = fn_state.n;
        // D s_1_23: cast zx s_1_22 -> i
        let s_1_23: i128 = (i128::try_from(s_1_22).unwrap());
        // D s_1_24: cast zx s_1_21 -> i
        let s_1_24: i128 = (i128::try_from(s_1_21).unwrap());
        // D s_1_25: call P_read(s_1_23, s_1_24)
        let s_1_25: Bits = P_read(state, tracer, s_1_23, s_1_24);
        // D s_1_26: write-var operand <= s_1_25
        fn_state.operand = s_1_25;
        // D s_1_27: read-var PL:i64
        let s_1_27: i64 = fn_state.PL;
        // D s_1_28: cast zx s_1_27 -> i
        let s_1_28: i128 = (i128::try_from(s_1_27).unwrap());
        // D s_1_29: cast reint s_1_28 -> i64
        let s_1_29: i64 = (s_1_28 as i64);
        // D s_1_30: read-var d:i64
        let s_1_30: i64 = fn_state.d;
        // D s_1_31: cast zx s_1_30 -> i
        let s_1_31: i128 = (i128::try_from(s_1_30).unwrap());
        // D s_1_32: cast zx s_1_29 -> i
        let s_1_32: i128 = (i128::try_from(s_1_29).unwrap());
        // D s_1_33: call P_read(s_1_31, s_1_32)
        let s_1_33: Bits = P_read(state, tracer, s_1_31, s_1_32);
        // D s_1_34: write-var operand2 <= s_1_33
        fn_state.operand2 = s_1_33;
        // C s_1_35: const #0u : u8
        let s_1_35: bool = false;
        // D s_1_36: write-var break <= s_1_35
        fn_state._break = s_1_35;
        // C s_1_37: const #8s : i
        let s_1_37: i128 = 8;
        // D s_1_38: read-var esize:i64
        let s_1_38: i64 = fn_state.esize;
        // D s_1_39: cast zx s_1_38 -> i
        let s_1_39: i128 = (i128::try_from(s_1_38).unwrap());
        // D s_1_40: div s_1_39 s_1_37
        let s_1_40: i128 = ((s_1_39) / (s_1_37));
        // D s_1_41: cast reint s_1_40 -> i64
        let s_1_41: i64 = (s_1_40 as i64);
        // D s_1_42: write-var psize <= s_1_41
        fn_state.psize = s_1_41;
        // C s_1_43: const #0s : i64
        let s_1_43: i64 = 0;
        // C s_1_44: const #1s : i
        let s_1_44: i128 = 1;
        // D s_1_45: cast zx s_1_10 -> i
        let s_1_45: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_46: sub s_1_45 s_1_44
        let s_1_46: i128 = ((s_1_45) - (s_1_44));
        // D s_1_47: cast reint s_1_46 -> i64
        let s_1_47: i64 = (s_1_46 as i64);
        // D s_1_48: write-var gs#197924 <= s_1_47
        fn_state.gs_197924 = s_1_47;
        // D s_1_49: write-var e <= s_1_43
        fn_state.e = s_1_43;
        // N s_1_50: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#197924:i64
        let s_2_1: i64 = fn_state.gs_197924;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b15 b3
        if s_2_2 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var e:i64
        let s_3_0: i64 = fn_state.e;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var esize:i64
        let s_3_2: i64 = fn_state.esize;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: read-var operand:bv
        let s_3_4: Bits = fn_state.operand;
        // D s_3_5: call ActivePredicateElement(s_3_4, s_3_1, s_3_3)
        let s_3_5: bool = ActivePredicateElement(state, tracer, s_3_4, s_3_1, s_3_3);
        // D s_3_6: write-var element <= s_3_5
        fn_state.element = s_3_5;
        // D s_3_7: read-var e:i64
        let s_3_7: i64 = fn_state.e;
        // D s_3_8: cast zx s_3_7 -> i
        let s_3_8: i128 = (i128::try_from(s_3_7).unwrap());
        // D s_3_9: read-var esize:i64
        let s_3_9: i64 = fn_state.esize;
        // D s_3_10: cast zx s_3_9 -> i
        let s_3_10: i128 = (i128::try_from(s_3_9).unwrap());
        // D s_3_11: read-var mask:bv
        let s_3_11: Bits = fn_state.mask;
        // D s_3_12: call ActivePredicateElement(s_3_11, s_3_8, s_3_10)
        let s_3_12: bool = ActivePredicateElement(state, tracer, s_3_11, s_3_8, s_3_10);
        // N s_3_13: branch s_3_12 b8 b4
        if s_3_12 {
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
        // D s_4_0: read-var merging:u8
        let s_4_0: bool = fn_state.merging;
        // N s_4_1: branch s_4_0 b7 b5
        if s_4_0 {
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
        // D s_5_0: read-var psize:i64
        let s_5_0: i64 = fn_state.psize;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: cast reint s_5_1 -> i64
        let s_5_2: i64 = (s_5_1 as i64);
        // C s_5_3: const #0u : u8
        let s_5_3: bool = false;
        // C s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 1u16);
        // D s_5_5: read-var psize:i64
        let s_5_5: i64 = fn_state.psize;
        // D s_5_6: cast zx s_5_5 -> i
        let s_5_6: i128 = (i128::try_from(s_5_5).unwrap());
        // D s_5_7: bits-cast zx s_5_4 -> bv length s_5_6
        let s_5_7: Bits = s_5_4.zero_extend(s_5_6);
        // D s_5_8: cast reint s_5_7 -> u8
        let s_5_8: bool = ((s_5_7.value()) != 0);
        // D s_5_9: read-var e:i64
        let s_5_9: i64 = fn_state.e;
        // D s_5_10: cast zx s_5_9 -> i
        let s_5_10: i128 = (i128::try_from(s_5_9).unwrap());
        // D s_5_11: cast zx s_5_2 -> i
        let s_5_11: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_12: cast zx s_5_8 -> bv
        let s_5_12: Bits = Bits::new(s_5_8 as u128, 1u16);
        // D s_5_13: read-var result:bv
        let s_5_13: Bits = fn_state.result;
        // D s_5_14: call Elem_set(s_5_13, s_5_10, s_5_11, s_5_12)
        let s_5_14: Bits = Elem_set(state, tracer, s_5_13, s_5_10, s_5_11, s_5_12);
        // D s_5_15: write-var result <= s_5_14
        fn_state.result = s_5_14;
        // N s_5_16: jump b6
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
        // N s_6_4: jump b2
        return block_2(state, tracer, fn_state);
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
        // D s_7_2: read-var esize:i64
        let s_7_2: i64 = fn_state.esize;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: read-var operand2:bv
        let s_7_4: Bits = fn_state.operand2;
        // D s_7_5: call PredicateElement(s_7_4, s_7_1, s_7_3)
        let s_7_5: bool = PredicateElement(state, tracer, s_7_4, s_7_1, s_7_3);
        // D s_7_6: read-var psize:i64
        let s_7_6: i64 = fn_state.psize;
        // D s_7_7: cast zx s_7_6 -> i
        let s_7_7: i128 = (i128::try_from(s_7_6).unwrap());
        // D s_7_8: cast reint s_7_7 -> i64
        let s_7_8: i64 = (s_7_7 as i64);
        // D s_7_9: cast zx s_7_5 -> bv
        let s_7_9: Bits = Bits::new(s_7_5 as u128, 1u16);
        // D s_7_10: read-var psize:i64
        let s_7_10: i64 = fn_state.psize;
        // D s_7_11: cast zx s_7_10 -> i
        let s_7_11: i128 = (i128::try_from(s_7_10).unwrap());
        // D s_7_12: bits-cast zx s_7_9 -> bv length s_7_11
        let s_7_12: Bits = s_7_9.zero_extend(s_7_11);
        // D s_7_13: cast reint s_7_12 -> u8
        let s_7_13: bool = ((s_7_12.value()) != 0);
        // D s_7_14: read-var e:i64
        let s_7_14: i64 = fn_state.e;
        // D s_7_15: cast zx s_7_14 -> i
        let s_7_15: i128 = (i128::try_from(s_7_14).unwrap());
        // D s_7_16: cast zx s_7_8 -> i
        let s_7_16: i128 = (i128::try_from(s_7_8).unwrap());
        // D s_7_17: cast zx s_7_13 -> bv
        let s_7_17: Bits = Bits::new(s_7_13 as u128, 1u16);
        // D s_7_18: read-var result:bv
        let s_7_18: Bits = fn_state.result;
        // D s_7_19: call Elem_set(s_7_18, s_7_15, s_7_16, s_7_17)
        let s_7_19: Bits = Elem_set(state, tracer, s_7_18, s_7_15, s_7_16, s_7_17);
        // D s_7_20: write-var result <= s_7_19
        fn_state.result = s_7_19;
        // N s_7_21: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var break:u8
        let s_8_0: bool = fn_state._break;
        // N s_8_1: branch s_8_0 b14 b9
        if s_8_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var element:u8
        let s_9_0: bool = fn_state.element;
        // D s_9_1: write-var gs#197928 <= s_9_0
        fn_state.gs_197928 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#197928:u8
        let s_10_0: bool = fn_state.gs_197928;
        // D s_10_1: write-var break <= s_10_0
        fn_state._break = s_10_0;
        // D s_10_2: read-var break:u8
        let s_10_2: bool = fn_state._break;
        // D s_10_3: not s_10_2
        let s_10_3: bool = !s_10_2;
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
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var pbit <= s_11_0
        fn_state.pbit = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var psize:i64
        let s_12_0: i64 = fn_state.psize;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: cast reint s_12_1 -> i64
        let s_12_2: i64 = (s_12_1 as i64);
        // D s_12_3: read-var pbit:u8
        let s_12_3: bool = fn_state.pbit;
        // D s_12_4: cast zx s_12_3 -> bv
        let s_12_4: Bits = Bits::new(s_12_3 as u128, 1u16);
        // D s_12_5: read-var psize:i64
        let s_12_5: i64 = fn_state.psize;
        // D s_12_6: cast zx s_12_5 -> i
        let s_12_6: i128 = (i128::try_from(s_12_5).unwrap());
        // D s_12_7: bits-cast zx s_12_4 -> bv length s_12_6
        let s_12_7: Bits = s_12_4.zero_extend(s_12_6);
        // D s_12_8: cast reint s_12_7 -> u8
        let s_12_8: bool = ((s_12_7.value()) != 0);
        // D s_12_9: read-var e:i64
        let s_12_9: i64 = fn_state.e;
        // D s_12_10: cast zx s_12_9 -> i
        let s_12_10: i128 = (i128::try_from(s_12_9).unwrap());
        // D s_12_11: cast zx s_12_2 -> i
        let s_12_11: i128 = (i128::try_from(s_12_2).unwrap());
        // D s_12_12: cast zx s_12_8 -> bv
        let s_12_12: Bits = Bits::new(s_12_8 as u128, 1u16);
        // D s_12_13: read-var result:bv
        let s_12_13: Bits = fn_state.result;
        // D s_12_14: call Elem_set(s_12_13, s_12_10, s_12_11, s_12_12)
        let s_12_14: Bits = Elem_set(state, tracer, s_12_13, s_12_10, s_12_11, s_12_12);
        // D s_12_15: write-var result <= s_12_14
        fn_state.result = s_12_14;
        // N s_12_16: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var pbit <= s_13_0
        fn_state.pbit = s_13_0;
        // N s_13_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var gs#197928 <= s_14_0
        fn_state.gs_197928 = s_14_0;
        // N s_14_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var setflags:u8
        let s_15_0: bool = fn_state.setflags;
        // N s_15_1: branch s_15_0 b18 b16
        if s_15_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var PL:i64
        let s_17_0: i64 = fn_state.PL;
        // D s_17_1: cast zx s_17_0 -> i
        let s_17_1: i128 = (i128::try_from(s_17_0).unwrap());
        // D s_17_2: cast reint s_17_1 -> i64
        let s_17_2: i64 = (s_17_1 as i64);
        // D s_17_3: read-var d:i64
        let s_17_3: i64 = fn_state.d;
        // D s_17_4: cast zx s_17_3 -> i
        let s_17_4: i128 = (i128::try_from(s_17_3).unwrap());
        // D s_17_5: cast zx s_17_2 -> i
        let s_17_5: i128 = (i128::try_from(s_17_2).unwrap());
        // D s_17_6: read-var result:bv
        let s_17_6: Bits = fn_state.result;
        // D s_17_7: call P_set(s_17_4, s_17_5, s_17_6)
        let s_17_7: () = P_set(state, tracer, s_17_4, s_17_5, s_17_6);
        // N s_17_8: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var esize:i64
        let s_18_0: i64 = fn_state.esize;
        // D s_18_1: cast zx s_18_0 -> i
        let s_18_1: i128 = (i128::try_from(s_18_0).unwrap());
        // D s_18_2: read-var mask:bv
        let s_18_2: Bits = fn_state.mask;
        // D s_18_3: read-var result:bv
        let s_18_3: Bits = fn_state.result;
        // D s_18_4: call PredTest(s_18_2, s_18_3, s_18_1)
        let s_18_4: u8 = PredTest(state, tracer, s_18_2, s_18_3, s_18_1);
        // C s_18_5: const #3s : i
        let s_18_5: i128 = 3;
        // D s_18_6: cast zx s_18_4 -> bv
        let s_18_6: Bits = Bits::new(s_18_4 as u128, 4u16);
        // C s_18_7: const #1s : i64
        let s_18_7: i64 = 1;
        // C s_18_8: cast zx s_18_7 -> i
        let s_18_8: i128 = (i128::try_from(s_18_7).unwrap());
        // C s_18_9: const #0s : i
        let s_18_9: i128 = 0;
        // C s_18_10: add s_18_9 s_18_8
        let s_18_10: i128 = (s_18_9 + s_18_8);
        // D s_18_11: bit-extract s_18_6 s_18_5 s_18_10
        let s_18_11: Bits = (Bits::new(
            ((s_18_6) >> (s_18_5)).value(),
            u16::try_from(s_18_10).unwrap(),
        ));
        // D s_18_12: cast reint s_18_11 -> u8
        let s_18_12: bool = ((s_18_11.value()) != 0);
        // C s_18_13: const #16984u : u32
        let s_18_13: u32 = 16984;
        // N s_18_14: write-reg s_18_13 <= s_18_12
        let s_18_14: () = {
            state.write_register::<bool>(s_18_13 as isize, s_18_12);
            tracer.write_register(s_18_13 as isize, s_18_12);
        };
        // C s_18_15: const #2s : i
        let s_18_15: i128 = 2;
        // D s_18_16: cast zx s_18_4 -> bv
        let s_18_16: Bits = Bits::new(s_18_4 as u128, 4u16);
        // C s_18_17: const #1s : i64
        let s_18_17: i64 = 1;
        // C s_18_18: cast zx s_18_17 -> i
        let s_18_18: i128 = (i128::try_from(s_18_17).unwrap());
        // C s_18_19: const #0s : i
        let s_18_19: i128 = 0;
        // C s_18_20: add s_18_19 s_18_18
        let s_18_20: i128 = (s_18_19 + s_18_18);
        // D s_18_21: bit-extract s_18_16 s_18_15 s_18_20
        let s_18_21: Bits = (Bits::new(
            ((s_18_16) >> (s_18_15)).value(),
            u16::try_from(s_18_20).unwrap(),
        ));
        // D s_18_22: cast reint s_18_21 -> u8
        let s_18_22: bool = ((s_18_21.value()) != 0);
        // C s_18_23: const #16997u : u32
        let s_18_23: u32 = 16997;
        // N s_18_24: write-reg s_18_23 <= s_18_22
        let s_18_24: () = {
            state.write_register::<bool>(s_18_23 as isize, s_18_22);
            tracer.write_register(s_18_23 as isize, s_18_22);
        };
        // C s_18_25: const #1s : i
        let s_18_25: i128 = 1;
        // D s_18_26: cast zx s_18_4 -> bv
        let s_18_26: Bits = Bits::new(s_18_4 as u128, 4u16);
        // C s_18_27: const #1s : i64
        let s_18_27: i64 = 1;
        // C s_18_28: cast zx s_18_27 -> i
        let s_18_28: i128 = (i128::try_from(s_18_27).unwrap());
        // C s_18_29: const #0s : i
        let s_18_29: i128 = 0;
        // C s_18_30: add s_18_29 s_18_28
        let s_18_30: i128 = (s_18_29 + s_18_28);
        // D s_18_31: bit-extract s_18_26 s_18_25 s_18_30
        let s_18_31: Bits = (Bits::new(
            ((s_18_26) >> (s_18_25)).value(),
            u16::try_from(s_18_30).unwrap(),
        ));
        // D s_18_32: cast reint s_18_31 -> u8
        let s_18_32: bool = ((s_18_31.value()) != 0);
        // C s_18_33: const #16971u : u32
        let s_18_33: u32 = 16971;
        // N s_18_34: write-reg s_18_33 <= s_18_32
        let s_18_34: () = {
            state.write_register::<bool>(s_18_33 as isize, s_18_32);
            tracer.write_register(s_18_33 as isize, s_18_32);
        };
        // C s_18_35: const #0s : i
        let s_18_35: i128 = 0;
        // D s_18_36: cast zx s_18_4 -> bv
        let s_18_36: Bits = Bits::new(s_18_4 as u128, 4u16);
        // C s_18_37: const #1s : i64
        let s_18_37: i64 = 1;
        // C s_18_38: cast zx s_18_37 -> i
        let s_18_38: i128 = (i128::try_from(s_18_37).unwrap());
        // C s_18_39: const #0s : i
        let s_18_39: i128 = 0;
        // C s_18_40: add s_18_39 s_18_38
        let s_18_40: i128 = (s_18_39 + s_18_38);
        // D s_18_41: bit-extract s_18_36 s_18_35 s_18_40
        let s_18_41: Bits = (Bits::new(
            ((s_18_36) >> (s_18_35)).value(),
            u16::try_from(s_18_40).unwrap(),
        ));
        // D s_18_42: cast reint s_18_41 -> u8
        let s_18_42: bool = ((s_18_41.value()) != 0);
        // C s_18_43: const #16996u : u32
        let s_18_43: u32 = 16996;
        // N s_18_44: write-reg s_18_43 <= s_18_42
        let s_18_44: () = {
            state.write_register::<bool>(s_18_43 as isize, s_18_42);
            tracer.write_register(s_18_43 as isize, s_18_42);
        };
        // N s_18_45: jump b17
        return block_17(state, tracer, fn_state);
    }
}
