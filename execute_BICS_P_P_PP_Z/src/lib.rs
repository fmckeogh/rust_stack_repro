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
use PredicateElement::*;
use common::*;
pub fn execute_BICS_P_P_PP_Z<T: Tracer>(
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
        e: i64,
        PL: i64,
        mask: Bits,
        element2: bool,
        element1: bool,
        psize: i64,
        gs_198239: i64,
        result: Bits,
        operand1: Bits,
        operand2: Bits,
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
        // D s_1_26: write-var operand1 <= s_1_25
        fn_state.operand1 = s_1_25;
        // D s_1_27: read-var PL:i64
        let s_1_27: i64 = fn_state.PL;
        // D s_1_28: cast zx s_1_27 -> i
        let s_1_28: i128 = (i128::try_from(s_1_27).unwrap());
        // D s_1_29: cast reint s_1_28 -> i64
        let s_1_29: i64 = (s_1_28 as i64);
        // D s_1_30: read-var m:i64
        let s_1_30: i64 = fn_state.m;
        // D s_1_31: cast zx s_1_30 -> i
        let s_1_31: i128 = (i128::try_from(s_1_30).unwrap());
        // D s_1_32: cast zx s_1_29 -> i
        let s_1_32: i128 = (i128::try_from(s_1_29).unwrap());
        // D s_1_33: call P_read(s_1_31, s_1_32)
        let s_1_33: Bits = P_read(state, tracer, s_1_31, s_1_32);
        // D s_1_34: write-var operand2 <= s_1_33
        fn_state.operand2 = s_1_33;
        // C s_1_35: const #8s : i
        let s_1_35: i128 = 8;
        // D s_1_36: read-var esize:i64
        let s_1_36: i64 = fn_state.esize;
        // D s_1_37: cast zx s_1_36 -> i
        let s_1_37: i128 = (i128::try_from(s_1_36).unwrap());
        // D s_1_38: div s_1_37 s_1_35
        let s_1_38: i128 = ((s_1_37) / (s_1_35));
        // D s_1_39: cast reint s_1_38 -> i64
        let s_1_39: i64 = (s_1_38 as i64);
        // D s_1_40: write-var psize <= s_1_39
        fn_state.psize = s_1_39;
        // C s_1_41: const #0s : i64
        let s_1_41: i64 = 0;
        // C s_1_42: const #1s : i
        let s_1_42: i128 = 1;
        // D s_1_43: cast zx s_1_10 -> i
        let s_1_43: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_44: sub s_1_43 s_1_42
        let s_1_44: i128 = ((s_1_43) - (s_1_42));
        // D s_1_45: cast reint s_1_44 -> i64
        let s_1_45: i64 = (s_1_44 as i64);
        // D s_1_46: write-var gs#198239 <= s_1_45
        fn_state.gs_198239 = s_1_45;
        // D s_1_47: write-var e <= s_1_41
        fn_state.e = s_1_41;
        // N s_1_48: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#198239:i64
        let s_2_1: i64 = fn_state.gs_198239;
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
        // D s_3_0: read-var e:i64
        let s_3_0: i64 = fn_state.e;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var esize:i64
        let s_3_2: i64 = fn_state.esize;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: read-var operand1:bv
        let s_3_4: Bits = fn_state.operand1;
        // D s_3_5: call PredicateElement(s_3_4, s_3_1, s_3_3)
        let s_3_5: bool = PredicateElement(state, tracer, s_3_4, s_3_1, s_3_3);
        // D s_3_6: write-var element1 <= s_3_5
        fn_state.element1 = s_3_5;
        // D s_3_7: read-var e:i64
        let s_3_7: i64 = fn_state.e;
        // D s_3_8: cast zx s_3_7 -> i
        let s_3_8: i128 = (i128::try_from(s_3_7).unwrap());
        // D s_3_9: read-var esize:i64
        let s_3_9: i64 = fn_state.esize;
        // D s_3_10: cast zx s_3_9 -> i
        let s_3_10: i128 = (i128::try_from(s_3_9).unwrap());
        // D s_3_11: read-var operand2:bv
        let s_3_11: Bits = fn_state.operand2;
        // D s_3_12: call PredicateElement(s_3_11, s_3_8, s_3_10)
        let s_3_12: bool = PredicateElement(state, tracer, s_3_11, s_3_8, s_3_10);
        // D s_3_13: write-var element2 <= s_3_12
        fn_state.element2 = s_3_12;
        // D s_3_14: read-var e:i64
        let s_3_14: i64 = fn_state.e;
        // D s_3_15: cast zx s_3_14 -> i
        let s_3_15: i128 = (i128::try_from(s_3_14).unwrap());
        // D s_3_16: read-var esize:i64
        let s_3_16: i64 = fn_state.esize;
        // D s_3_17: cast zx s_3_16 -> i
        let s_3_17: i128 = (i128::try_from(s_3_16).unwrap());
        // D s_3_18: read-var mask:bv
        let s_3_18: Bits = fn_state.mask;
        // D s_3_19: call ActivePredicateElement(s_3_18, s_3_15, s_3_17)
        let s_3_19: bool = ActivePredicateElement(state, tracer, s_3_18, s_3_15, s_3_17);
        // N s_3_20: branch s_3_19 b6 b4
        if s_3_19 {
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
        // D s_4_0: read-var psize:i64
        let s_4_0: i64 = fn_state.psize;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // C s_4_3: const #0u : u8
        let s_4_3: bool = false;
        // C s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 1u16);
        // D s_4_5: read-var psize:i64
        let s_4_5: i64 = fn_state.psize;
        // D s_4_6: cast zx s_4_5 -> i
        let s_4_6: i128 = (i128::try_from(s_4_5).unwrap());
        // D s_4_7: bits-cast zx s_4_4 -> bv length s_4_6
        let s_4_7: Bits = s_4_4.zero_extend(s_4_6);
        // D s_4_8: cast reint s_4_7 -> u8
        let s_4_8: bool = ((s_4_7.value()) != 0);
        // D s_4_9: read-var e:i64
        let s_4_9: i64 = fn_state.e;
        // D s_4_10: cast zx s_4_9 -> i
        let s_4_10: i128 = (i128::try_from(s_4_9).unwrap());
        // D s_4_11: cast zx s_4_2 -> i
        let s_4_11: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_12: cast zx s_4_8 -> bv
        let s_4_12: Bits = Bits::new(s_4_8 as u128, 1u16);
        // D s_4_13: read-var result:bv
        let s_4_13: Bits = fn_state.result;
        // D s_4_14: call Elem_set(s_4_13, s_4_10, s_4_11, s_4_12)
        let s_4_14: Bits = Elem_set(state, tracer, s_4_13, s_4_10, s_4_11, s_4_12);
        // D s_4_15: write-var result <= s_4_14
        fn_state.result = s_4_14;
        // N s_4_16: jump b5
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
        // D s_6_0: read-var psize:i64
        let s_6_0: i64 = fn_state.psize;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: cast reint s_6_1 -> i64
        let s_6_2: i64 = (s_6_1 as i64);
        // D s_6_3: read-var element2:u8
        let s_6_3: bool = fn_state.element2;
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 1u16);
        // D s_6_5: not s_6_4
        let s_6_5: Bits = !s_6_4;
        // D s_6_6: cast reint s_6_5 -> u8
        let s_6_6: bool = ((s_6_5.value()) != 0);
        // D s_6_7: read-var element1:u8
        let s_6_7: bool = fn_state.element1;
        // D s_6_8: cast zx s_6_7 -> bv
        let s_6_8: Bits = Bits::new(s_6_7 as u128, 1u16);
        // D s_6_9: cast zx s_6_6 -> bv
        let s_6_9: Bits = Bits::new(s_6_6 as u128, 1u16);
        // D s_6_10: and s_6_8 s_6_9
        let s_6_10: Bits = ((s_6_8) & (s_6_9));
        // D s_6_11: cast reint s_6_10 -> u8
        let s_6_11: bool = ((s_6_10.value()) != 0);
        // D s_6_12: cast zx s_6_11 -> bv
        let s_6_12: Bits = Bits::new(s_6_11 as u128, 1u16);
        // D s_6_13: read-var psize:i64
        let s_6_13: i64 = fn_state.psize;
        // D s_6_14: cast zx s_6_13 -> i
        let s_6_14: i128 = (i128::try_from(s_6_13).unwrap());
        // D s_6_15: bits-cast zx s_6_12 -> bv length s_6_14
        let s_6_15: Bits = s_6_12.zero_extend(s_6_14);
        // D s_6_16: cast reint s_6_15 -> u8
        let s_6_16: bool = ((s_6_15.value()) != 0);
        // D s_6_17: read-var e:i64
        let s_6_17: i64 = fn_state.e;
        // D s_6_18: cast zx s_6_17 -> i
        let s_6_18: i128 = (i128::try_from(s_6_17).unwrap());
        // D s_6_19: cast zx s_6_2 -> i
        let s_6_19: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_20: cast zx s_6_16 -> bv
        let s_6_20: Bits = Bits::new(s_6_16 as u128, 1u16);
        // D s_6_21: read-var result:bv
        let s_6_21: Bits = fn_state.result;
        // D s_6_22: call Elem_set(s_6_21, s_6_18, s_6_19, s_6_20)
        let s_6_22: Bits = Elem_set(state, tracer, s_6_21, s_6_18, s_6_19, s_6_20);
        // D s_6_23: write-var result <= s_6_22
        fn_state.result = s_6_22;
        // N s_6_24: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var setflags:u8
        let s_7_0: bool = fn_state.setflags;
        // N s_7_1: branch s_7_0 b10 b8
        if s_7_0 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var PL:i64
        let s_9_0: i64 = fn_state.PL;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: cast reint s_9_1 -> i64
        let s_9_2: i64 = (s_9_1 as i64);
        // D s_9_3: read-var d:i64
        let s_9_3: i64 = fn_state.d;
        // D s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_5: cast zx s_9_2 -> i
        let s_9_5: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_6: read-var result:bv
        let s_9_6: Bits = fn_state.result;
        // D s_9_7: call P_set(s_9_4, s_9_5, s_9_6)
        let s_9_7: () = P_set(state, tracer, s_9_4, s_9_5, s_9_6);
        // N s_9_8: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var esize:i64
        let s_10_0: i64 = fn_state.esize;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: read-var mask:bv
        let s_10_2: Bits = fn_state.mask;
        // D s_10_3: read-var result:bv
        let s_10_3: Bits = fn_state.result;
        // D s_10_4: call PredTest(s_10_2, s_10_3, s_10_1)
        let s_10_4: u8 = PredTest(state, tracer, s_10_2, s_10_3, s_10_1);
        // C s_10_5: const #3s : i
        let s_10_5: i128 = 3;
        // D s_10_6: cast zx s_10_4 -> bv
        let s_10_6: Bits = Bits::new(s_10_4 as u128, 4u16);
        // C s_10_7: const #1s : i64
        let s_10_7: i64 = 1;
        // C s_10_8: cast zx s_10_7 -> i
        let s_10_8: i128 = (i128::try_from(s_10_7).unwrap());
        // C s_10_9: const #0s : i
        let s_10_9: i128 = 0;
        // C s_10_10: add s_10_9 s_10_8
        let s_10_10: i128 = (s_10_9 + s_10_8);
        // D s_10_11: bit-extract s_10_6 s_10_5 s_10_10
        let s_10_11: Bits = (Bits::new(
            ((s_10_6) >> (s_10_5)).value(),
            u16::try_from(s_10_10).unwrap(),
        ));
        // D s_10_12: cast reint s_10_11 -> u8
        let s_10_12: bool = ((s_10_11.value()) != 0);
        // C s_10_13: const #16984u : u32
        let s_10_13: u32 = 16984;
        // N s_10_14: write-reg s_10_13 <= s_10_12
        let s_10_14: () = {
            state.write_register::<bool>(s_10_13 as isize, s_10_12);
            tracer.write_register(s_10_13 as isize, s_10_12);
        };
        // C s_10_15: const #2s : i
        let s_10_15: i128 = 2;
        // D s_10_16: cast zx s_10_4 -> bv
        let s_10_16: Bits = Bits::new(s_10_4 as u128, 4u16);
        // C s_10_17: const #1s : i64
        let s_10_17: i64 = 1;
        // C s_10_18: cast zx s_10_17 -> i
        let s_10_18: i128 = (i128::try_from(s_10_17).unwrap());
        // C s_10_19: const #0s : i
        let s_10_19: i128 = 0;
        // C s_10_20: add s_10_19 s_10_18
        let s_10_20: i128 = (s_10_19 + s_10_18);
        // D s_10_21: bit-extract s_10_16 s_10_15 s_10_20
        let s_10_21: Bits = (Bits::new(
            ((s_10_16) >> (s_10_15)).value(),
            u16::try_from(s_10_20).unwrap(),
        ));
        // D s_10_22: cast reint s_10_21 -> u8
        let s_10_22: bool = ((s_10_21.value()) != 0);
        // C s_10_23: const #16997u : u32
        let s_10_23: u32 = 16997;
        // N s_10_24: write-reg s_10_23 <= s_10_22
        let s_10_24: () = {
            state.write_register::<bool>(s_10_23 as isize, s_10_22);
            tracer.write_register(s_10_23 as isize, s_10_22);
        };
        // C s_10_25: const #1s : i
        let s_10_25: i128 = 1;
        // D s_10_26: cast zx s_10_4 -> bv
        let s_10_26: Bits = Bits::new(s_10_4 as u128, 4u16);
        // C s_10_27: const #1s : i64
        let s_10_27: i64 = 1;
        // C s_10_28: cast zx s_10_27 -> i
        let s_10_28: i128 = (i128::try_from(s_10_27).unwrap());
        // C s_10_29: const #0s : i
        let s_10_29: i128 = 0;
        // C s_10_30: add s_10_29 s_10_28
        let s_10_30: i128 = (s_10_29 + s_10_28);
        // D s_10_31: bit-extract s_10_26 s_10_25 s_10_30
        let s_10_31: Bits = (Bits::new(
            ((s_10_26) >> (s_10_25)).value(),
            u16::try_from(s_10_30).unwrap(),
        ));
        // D s_10_32: cast reint s_10_31 -> u8
        let s_10_32: bool = ((s_10_31.value()) != 0);
        // C s_10_33: const #16971u : u32
        let s_10_33: u32 = 16971;
        // N s_10_34: write-reg s_10_33 <= s_10_32
        let s_10_34: () = {
            state.write_register::<bool>(s_10_33 as isize, s_10_32);
            tracer.write_register(s_10_33 as isize, s_10_32);
        };
        // C s_10_35: const #0s : i
        let s_10_35: i128 = 0;
        // D s_10_36: cast zx s_10_4 -> bv
        let s_10_36: Bits = Bits::new(s_10_4 as u128, 4u16);
        // C s_10_37: const #1s : i64
        let s_10_37: i64 = 1;
        // C s_10_38: cast zx s_10_37 -> i
        let s_10_38: i128 = (i128::try_from(s_10_37).unwrap());
        // C s_10_39: const #0s : i
        let s_10_39: i128 = 0;
        // C s_10_40: add s_10_39 s_10_38
        let s_10_40: i128 = (s_10_39 + s_10_38);
        // D s_10_41: bit-extract s_10_36 s_10_35 s_10_40
        let s_10_41: Bits = (Bits::new(
            ((s_10_36) >> (s_10_35)).value(),
            u16::try_from(s_10_40).unwrap(),
        ));
        // D s_10_42: cast reint s_10_41 -> u8
        let s_10_42: bool = ((s_10_41.value()) != 0);
        // C s_10_43: const #16996u : u32
        let s_10_43: u32 = 16996;
        // N s_10_44: write-reg s_10_43 <= s_10_42
        let s_10_44: () = {
            state.write_register::<bool>(s_10_43 as isize, s_10_42);
            tracer.write_register(s_10_43 as isize, s_10_42);
        };
        // N s_10_45: jump b9
        return block_9(state, tracer, fn_state);
    }
}
