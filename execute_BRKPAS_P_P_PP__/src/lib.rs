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
use PredTest::*;
use Elem_set::*;
use CheckSVEEnabled::*;
use P_set::*;
use P_read::*;
use LastActive::*;
use common::*;
pub fn execute_BRKPAS_P_P_PP__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    g: i64,
    m: i64,
    n: i64,
    setflags: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        last: bool,
        e: i64,
        pbit: bool,
        result: Bits,
        PL: i64,
        mask: Bits,
        gs_201449: i64,
        operand2: Bits,
        gs_201459: bool,
        VL: i64,
        d: i64,
        esize: i64,
        g: i64,
        m: i64,
        n: i64,
        setflags: bool,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        g,
        m,
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
        // D s_1_26: read-var PL:i64
        let s_1_26: i64 = fn_state.PL;
        // D s_1_27: cast zx s_1_26 -> i
        let s_1_27: i128 = (i128::try_from(s_1_26).unwrap());
        // D s_1_28: cast reint s_1_27 -> i64
        let s_1_28: i64 = (s_1_27 as i64);
        // D s_1_29: read-var m:i64
        let s_1_29: i64 = fn_state.m;
        // D s_1_30: cast zx s_1_29 -> i
        let s_1_30: i128 = (i128::try_from(s_1_29).unwrap());
        // D s_1_31: cast zx s_1_28 -> i
        let s_1_31: i128 = (i128::try_from(s_1_28).unwrap());
        // D s_1_32: call P_read(s_1_30, s_1_31)
        let s_1_32: Bits = P_read(state, tracer, s_1_30, s_1_31);
        // D s_1_33: write-var operand2 <= s_1_32
        fn_state.operand2 = s_1_32;
        // C s_1_34: const #8s : i
        let s_1_34: i128 = 8;
        // D s_1_35: read-var mask:bv
        let s_1_35: Bits = fn_state.mask;
        // D s_1_36: call LastActive(s_1_35, s_1_25, s_1_34)
        let s_1_36: bool = LastActive(state, tracer, s_1_35, s_1_25, s_1_34);
        // D s_1_37: cast zx s_1_36 -> bv
        let s_1_37: Bits = Bits::new(s_1_36 as u128, 1u16);
        // C s_1_38: const #1u : u8
        let s_1_38: bool = true;
        // C s_1_39: cast zx s_1_38 -> bv
        let s_1_39: Bits = Bits::new(s_1_38 as u128, 1u16);
        // D s_1_40: cmp-eq s_1_37 s_1_39
        let s_1_40: bool = ((s_1_37) == (s_1_39));
        // D s_1_41: write-var last <= s_1_40
        fn_state.last = s_1_40;
        // C s_1_42: const #0s : i64
        let s_1_42: i64 = 0;
        // C s_1_43: const #1s : i
        let s_1_43: i128 = 1;
        // D s_1_44: cast zx s_1_10 -> i
        let s_1_44: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_45: sub s_1_44 s_1_43
        let s_1_45: i128 = ((s_1_44) - (s_1_43));
        // D s_1_46: cast reint s_1_45 -> i64
        let s_1_46: i64 = (s_1_45 as i64);
        // D s_1_47: write-var gs#201449 <= s_1_46
        fn_state.gs_201449 = s_1_46;
        // D s_1_48: write-var e <= s_1_42
        fn_state.e = s_1_42;
        // N s_1_49: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#201449:i64
        let s_2_1: i64 = fn_state.gs_201449;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b13 b3
        if s_2_2 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #8s : i
        let s_3_0: i128 = 8;
        // D s_3_1: read-var e:i64
        let s_3_1: i64 = fn_state.e;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: read-var mask:bv
        let s_3_3: Bits = fn_state.mask;
        // D s_3_4: call ActivePredicateElement(s_3_3, s_3_2, s_3_0)
        let s_3_4: bool = ActivePredicateElement(state, tracer, s_3_3, s_3_2, s_3_0);
        // N s_3_5: branch s_3_4 b6 b4
        if s_3_4 {
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
        // C s_4_0: const #1s : i64
        let s_4_0: i64 = 1;
        // D s_4_1: read-var e:i64
        let s_4_1: i64 = fn_state.e;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // C s_4_3: cast zx s_4_0 -> i
        let s_4_3: i128 = (i128::try_from(s_4_0).unwrap());
        // C s_4_4: const #0u : u8
        let s_4_4: bool = false;
        // C s_4_5: cast zx s_4_4 -> bv
        let s_4_5: Bits = Bits::new(s_4_4 as u128, 1u16);
        // D s_4_6: read-var result:bv
        let s_4_6: Bits = fn_state.result;
        // D s_4_7: call Elem_set(s_4_6, s_4_2, s_4_3, s_4_5)
        let s_4_7: Bits = Elem_set(state, tracer, s_4_6, s_4_2, s_4_3, s_4_5);
        // D s_4_8: write-var result <= s_4_7
        fn_state.result = s_4_7;
        // N s_4_9: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var e:i64
        let s_5_0: i64 = fn_state.e;
        // C s_5_1: const #1s : i64
        let s_5_1: i64 = 1;
        // D s_5_2: add s_5_0 s_5_1
        let s_5_2: i64 = (s_5_0 + s_5_1);
        // D s_5_3: write-var e <= s_5_2
        fn_state.e = s_5_2;
        // N s_5_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var last:u8
        let s_6_0: bool = fn_state.last;
        // N s_6_1: branch s_6_0 b12 b7
        if s_6_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var pbit <= s_7_0
        fn_state.pbit = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #1s : i64
        let s_8_0: i64 = 1;
        // C s_8_1: const #1s : i
        let s_8_1: i128 = 1;
        // D s_8_2: read-var pbit:u8
        let s_8_2: bool = fn_state.pbit;
        // D s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 1u16);
        // D s_8_4: bits-cast zx s_8_3 -> bv length s_8_1
        let s_8_4: Bits = s_8_3.zero_extend(s_8_1);
        // D s_8_5: cast reint s_8_4 -> u8
        let s_8_5: bool = ((s_8_4.value()) != 0);
        // D s_8_6: read-var e:i64
        let s_8_6: i64 = fn_state.e;
        // D s_8_7: cast zx s_8_6 -> i
        let s_8_7: i128 = (i128::try_from(s_8_6).unwrap());
        // C s_8_8: cast zx s_8_0 -> i
        let s_8_8: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_9: cast zx s_8_5 -> bv
        let s_8_9: Bits = Bits::new(s_8_5 as u128, 1u16);
        // D s_8_10: read-var result:bv
        let s_8_10: Bits = fn_state.result;
        // D s_8_11: call Elem_set(s_8_10, s_8_7, s_8_8, s_8_9)
        let s_8_11: Bits = Elem_set(state, tracer, s_8_10, s_8_7, s_8_8, s_8_9);
        // D s_8_12: write-var result <= s_8_11
        fn_state.result = s_8_11;
        // D s_8_13: read-var last:u8
        let s_8_13: bool = fn_state.last;
        // N s_8_14: branch s_8_13 b11 b9
        if s_8_13 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#201459 <= s_9_0
        fn_state.gs_201459 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#201459:u8
        let s_10_0: bool = fn_state.gs_201459;
        // D s_10_1: write-var last <= s_10_0
        fn_state.last = s_10_0;
        // N s_10_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #8s : i
        let s_11_0: i128 = 8;
        // D s_11_1: read-var e:i64
        let s_11_1: i64 = fn_state.e;
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // D s_11_3: read-var operand2:bv
        let s_11_3: Bits = fn_state.operand2;
        // D s_11_4: call ActivePredicateElement(s_11_3, s_11_2, s_11_0)
        let s_11_4: bool = ActivePredicateElement(state, tracer, s_11_3, s_11_2, s_11_0);
        // D s_11_5: not s_11_4
        let s_11_5: bool = !s_11_4;
        // D s_11_6: write-var gs#201459 <= s_11_5
        fn_state.gs_201459 = s_11_5;
        // N s_11_7: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var pbit <= s_12_0
        fn_state.pbit = s_12_0;
        // N s_12_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var setflags:u8
        let s_13_0: bool = fn_state.setflags;
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
        // N s_14_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var PL:i64
        let s_15_0: i64 = fn_state.PL;
        // D s_15_1: cast zx s_15_0 -> i
        let s_15_1: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_2: cast reint s_15_1 -> i64
        let s_15_2: i64 = (s_15_1 as i64);
        // D s_15_3: read-var d:i64
        let s_15_3: i64 = fn_state.d;
        // D s_15_4: cast zx s_15_3 -> i
        let s_15_4: i128 = (i128::try_from(s_15_3).unwrap());
        // D s_15_5: cast zx s_15_2 -> i
        let s_15_5: i128 = (i128::try_from(s_15_2).unwrap());
        // D s_15_6: read-var result:bv
        let s_15_6: Bits = fn_state.result;
        // D s_15_7: call P_set(s_15_4, s_15_5, s_15_6)
        let s_15_7: () = P_set(state, tracer, s_15_4, s_15_5, s_15_6);
        // N s_15_8: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var esize:i64
        let s_16_0: i64 = fn_state.esize;
        // D s_16_1: cast zx s_16_0 -> i
        let s_16_1: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_2: read-var mask:bv
        let s_16_2: Bits = fn_state.mask;
        // D s_16_3: read-var result:bv
        let s_16_3: Bits = fn_state.result;
        // D s_16_4: call PredTest(s_16_2, s_16_3, s_16_1)
        let s_16_4: u8 = PredTest(state, tracer, s_16_2, s_16_3, s_16_1);
        // C s_16_5: const #3s : i
        let s_16_5: i128 = 3;
        // D s_16_6: cast zx s_16_4 -> bv
        let s_16_6: Bits = Bits::new(s_16_4 as u128, 4u16);
        // C s_16_7: const #1s : i64
        let s_16_7: i64 = 1;
        // C s_16_8: cast zx s_16_7 -> i
        let s_16_8: i128 = (i128::try_from(s_16_7).unwrap());
        // C s_16_9: const #0s : i
        let s_16_9: i128 = 0;
        // C s_16_10: add s_16_9 s_16_8
        let s_16_10: i128 = (s_16_9 + s_16_8);
        // D s_16_11: bit-extract s_16_6 s_16_5 s_16_10
        let s_16_11: Bits = (Bits::new(
            ((s_16_6) >> (s_16_5)).value(),
            u16::try_from(s_16_10).unwrap(),
        ));
        // D s_16_12: cast reint s_16_11 -> u8
        let s_16_12: bool = ((s_16_11.value()) != 0);
        // C s_16_13: const #16984u : u32
        let s_16_13: u32 = 16984;
        // N s_16_14: write-reg s_16_13 <= s_16_12
        let s_16_14: () = {
            state.write_register::<bool>(s_16_13 as isize, s_16_12);
            tracer.write_register(s_16_13 as isize, s_16_12);
        };
        // C s_16_15: const #2s : i
        let s_16_15: i128 = 2;
        // D s_16_16: cast zx s_16_4 -> bv
        let s_16_16: Bits = Bits::new(s_16_4 as u128, 4u16);
        // C s_16_17: const #1s : i64
        let s_16_17: i64 = 1;
        // C s_16_18: cast zx s_16_17 -> i
        let s_16_18: i128 = (i128::try_from(s_16_17).unwrap());
        // C s_16_19: const #0s : i
        let s_16_19: i128 = 0;
        // C s_16_20: add s_16_19 s_16_18
        let s_16_20: i128 = (s_16_19 + s_16_18);
        // D s_16_21: bit-extract s_16_16 s_16_15 s_16_20
        let s_16_21: Bits = (Bits::new(
            ((s_16_16) >> (s_16_15)).value(),
            u16::try_from(s_16_20).unwrap(),
        ));
        // D s_16_22: cast reint s_16_21 -> u8
        let s_16_22: bool = ((s_16_21.value()) != 0);
        // C s_16_23: const #16997u : u32
        let s_16_23: u32 = 16997;
        // N s_16_24: write-reg s_16_23 <= s_16_22
        let s_16_24: () = {
            state.write_register::<bool>(s_16_23 as isize, s_16_22);
            tracer.write_register(s_16_23 as isize, s_16_22);
        };
        // C s_16_25: const #1s : i
        let s_16_25: i128 = 1;
        // D s_16_26: cast zx s_16_4 -> bv
        let s_16_26: Bits = Bits::new(s_16_4 as u128, 4u16);
        // C s_16_27: const #1s : i64
        let s_16_27: i64 = 1;
        // C s_16_28: cast zx s_16_27 -> i
        let s_16_28: i128 = (i128::try_from(s_16_27).unwrap());
        // C s_16_29: const #0s : i
        let s_16_29: i128 = 0;
        // C s_16_30: add s_16_29 s_16_28
        let s_16_30: i128 = (s_16_29 + s_16_28);
        // D s_16_31: bit-extract s_16_26 s_16_25 s_16_30
        let s_16_31: Bits = (Bits::new(
            ((s_16_26) >> (s_16_25)).value(),
            u16::try_from(s_16_30).unwrap(),
        ));
        // D s_16_32: cast reint s_16_31 -> u8
        let s_16_32: bool = ((s_16_31.value()) != 0);
        // C s_16_33: const #16971u : u32
        let s_16_33: u32 = 16971;
        // N s_16_34: write-reg s_16_33 <= s_16_32
        let s_16_34: () = {
            state.write_register::<bool>(s_16_33 as isize, s_16_32);
            tracer.write_register(s_16_33 as isize, s_16_32);
        };
        // C s_16_35: const #0s : i
        let s_16_35: i128 = 0;
        // D s_16_36: cast zx s_16_4 -> bv
        let s_16_36: Bits = Bits::new(s_16_4 as u128, 4u16);
        // C s_16_37: const #1s : i64
        let s_16_37: i64 = 1;
        // C s_16_38: cast zx s_16_37 -> i
        let s_16_38: i128 = (i128::try_from(s_16_37).unwrap());
        // C s_16_39: const #0s : i
        let s_16_39: i128 = 0;
        // C s_16_40: add s_16_39 s_16_38
        let s_16_40: i128 = (s_16_39 + s_16_38);
        // D s_16_41: bit-extract s_16_36 s_16_35 s_16_40
        let s_16_41: Bits = (Bits::new(
            ((s_16_36) >> (s_16_35)).value(),
            u16::try_from(s_16_40).unwrap(),
        ));
        // D s_16_42: cast reint s_16_41 -> u8
        let s_16_42: bool = ((s_16_41.value()) != 0);
        // C s_16_43: const #16996u : u32
        let s_16_43: u32 = 16996;
        // N s_16_44: write-reg s_16_43 <= s_16_42
        let s_16_44: () = {
            state.write_register::<bool>(s_16_43 as isize, s_16_42);
            tracer.write_register(s_16_43 as isize, s_16_42);
        };
        // N s_16_45: jump b15
        return block_15(state, tracer, fn_state);
    }
}
