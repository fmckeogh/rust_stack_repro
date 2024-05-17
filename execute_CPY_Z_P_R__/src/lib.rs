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
use SP_read::*;
use CheckSVEEnabled::*;
use AnyActiveElement::*;
use ActivePredicateElement::*;
use P_read::*;
use X_read::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_CPY_Z_P_R__<T: Tracer>(
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
        e: i64,
        VLshadow_2979: i64,
        elements: i64,
        VLshadow_2978: i64,
        gs_196557: i64,
        result: Bits,
        operand1: u64,
        esizeshadow_2977: i64,
        mask: Bits,
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
        // D s_0_3: write-var esizeshadow#2977 <= s_0_2
        fn_state.esizeshadow_2977 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#2978 <= s_0_6
        fn_state.VLshadow_2978 = s_0_6;
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
        // D s_1_0: read-var VLshadow#2978:i64
        let s_1_0: i64 = fn_state.VLshadow_2978;
        // D s_1_1: write-var VLshadow#2979 <= s_1_0
        fn_state.VLshadow_2979 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#2979:i64
        let s_1_3: i64 = fn_state.VLshadow_2979;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#2979:i64
        let s_1_7: i64 = fn_state.VLshadow_2979;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: read-var esizeshadow#2977:i64
        let s_1_9: i64 = fn_state.esizeshadow_2977;
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
        // D s_1_21: read-var VLshadow#2979:i64
        let s_1_21: i64 = fn_state.VLshadow_2979;
        // D s_1_22: cast zx s_1_21 -> i
        let s_1_22: i128 = (i128::try_from(s_1_21).unwrap());
        // D s_1_23: cast reint s_1_22 -> i64
        let s_1_23: i64 = (s_1_22 as i64);
        // D s_1_24: read-var d:i64
        let s_1_24: i64 = fn_state.d;
        // D s_1_25: cast zx s_1_24 -> i
        let s_1_25: i128 = (i128::try_from(s_1_24).unwrap());
        // D s_1_26: cast zx s_1_23 -> i
        let s_1_26: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_27: call Z_read(s_1_25, s_1_26)
        let s_1_27: Bits = Z_read(state, tracer, s_1_25, s_1_26);
        // D s_1_28: write-var result <= s_1_27
        fn_state.result = s_1_27;
        // D s_1_29: read-var esizeshadow#2977:i64
        let s_1_29: i64 = fn_state.esizeshadow_2977;
        // D s_1_30: cast zx s_1_29 -> i
        let s_1_30: i128 = (i128::try_from(s_1_29).unwrap());
        // D s_1_31: read-var mask:bv
        let s_1_31: Bits = fn_state.mask;
        // D s_1_32: call AnyActiveElement(s_1_31, s_1_30)
        let s_1_32: bool = AnyActiveElement(state, tracer, s_1_31, s_1_30);
        // N s_1_33: branch s_1_32 b4 b2
        if s_1_32 {
            return block_4(state, tracer, fn_state);
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
        // D s_3_0: read-var VLshadow#2979:i64
        let s_3_0: i64 = fn_state.VLshadow_2979;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: cast reint s_3_1 -> i64
        let s_3_2: i64 = (s_3_1 as i64);
        // D s_3_3: read-var d:i64
        let s_3_3: i64 = fn_state.d;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: cast zx s_3_2 -> i
        let s_3_5: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_6: read-var result:bv
        let s_3_6: Bits = fn_state.result;
        // D s_3_7: call Z_set(s_3_4, s_3_5, s_3_6)
        let s_3_7: () = Z_set(state, tracer, s_3_4, s_3_5, s_3_6);
        // N s_3_8: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #31s : i
        let s_4_0: i128 = 31;
        // D s_4_1: read-var n:i64
        let s_4_1: i64 = fn_state.n;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: cmp-eq s_4_2 s_4_0
        let s_4_3: bool = ((s_4_2) == (s_4_0));
        // N s_4_4: branch s_4_3 b13 b5
        if s_4_3 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #64s : i64
        let s_5_0: i64 = 64;
        // D s_5_1: read-var n:i64
        let s_5_1: i64 = fn_state.n;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: call X_read(s_5_2, s_5_0)
        let s_5_3: Bits = X_read(state, tracer, s_5_2, s_5_0);
        // D s_5_4: cast reint s_5_3 -> u64
        let s_5_4: u64 = (s_5_3.value() as u64);
        // D s_5_5: write-var operand1 <= s_5_4
        fn_state.operand1 = s_5_4;
        // N s_5_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0s : i64
        let s_6_0: i64 = 0;
        // C s_6_1: const #1s : i
        let s_6_1: i128 = 1;
        // D s_6_2: read-var elements:i64
        let s_6_2: i64 = fn_state.elements;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: sub s_6_3 s_6_1
        let s_6_4: i128 = ((s_6_3) - (s_6_1));
        // D s_6_5: cast reint s_6_4 -> i64
        let s_6_5: i64 = (s_6_4 as i64);
        // D s_6_6: write-var gs#196557 <= s_6_5
        fn_state.gs_196557 = s_6_5;
        // D s_6_7: write-var e <= s_6_0
        fn_state.e = s_6_0;
        // N s_6_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var e:i64
        let s_7_0: i64 = fn_state.e;
        // D s_7_1: read-var gs#196557:i64
        let s_7_1: i64 = fn_state.gs_196557;
        // D s_7_2: cmp-gt s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) > (s_7_1));
        // N s_7_3: branch s_7_2 b12 b8
        if s_7_2 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var e:i64
        let s_8_0: i64 = fn_state.e;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: read-var esizeshadow#2977:i64
        let s_8_2: i64 = fn_state.esizeshadow_2977;
        // D s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_4: read-var mask:bv
        let s_8_4: Bits = fn_state.mask;
        // D s_8_5: call ActivePredicateElement(s_8_4, s_8_1, s_8_3)
        let s_8_5: bool = ActivePredicateElement(state, tracer, s_8_4, s_8_1, s_8_3);
        // N s_8_6: branch s_8_5 b11 b9
        if s_8_5 {
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
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var e:i64
        let s_10_0: i64 = fn_state.e;
        // C s_10_1: const #1s : i64
        let s_10_1: i64 = 1;
        // D s_10_2: add s_10_0 s_10_1
        let s_10_2: i64 = (s_10_0 + s_10_1);
        // D s_10_3: write-var e <= s_10_2
        fn_state.e = s_10_2;
        // N s_10_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var esizeshadow#2977:i64
        let s_11_0: i64 = fn_state.esizeshadow_2977;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: cast reint s_11_1 -> i64
        let s_11_2: i64 = (s_11_1 as i64);
        // C s_11_3: const #1s : i
        let s_11_3: i128 = 1;
        // D s_11_4: read-var esizeshadow#2977:i64
        let s_11_4: i64 = fn_state.esizeshadow_2977;
        // D s_11_5: cast zx s_11_4 -> i
        let s_11_5: i128 = (i128::try_from(s_11_4).unwrap());
        // D s_11_6: sub s_11_5 s_11_3
        let s_11_6: i128 = ((s_11_5) - (s_11_3));
        // D s_11_7: cast reint s_11_6 -> i64
        let s_11_7: i64 = (s_11_6 as i64);
        // C s_11_8: const #0s : i
        let s_11_8: i128 = 0;
        // D s_11_9: read-var operand1:u64
        let s_11_9: u64 = fn_state.operand1;
        // D s_11_10: cast zx s_11_9 -> bv
        let s_11_10: Bits = Bits::new(s_11_9 as u128, 64u16);
        // D s_11_11: cast zx s_11_7 -> i
        let s_11_11: i128 = (i128::try_from(s_11_7).unwrap());
        // C s_11_12: const #1s : i64
        let s_11_12: i64 = 1;
        // C s_11_13: cast zx s_11_12 -> i
        let s_11_13: i128 = (i128::try_from(s_11_12).unwrap());
        // D s_11_14: sub s_11_11 s_11_8
        let s_11_14: i128 = ((s_11_11) - (s_11_8));
        // D s_11_15: add s_11_14 s_11_13
        let s_11_15: i128 = (s_11_14 + s_11_13);
        // D s_11_16: bit-extract s_11_10 s_11_8 s_11_15
        let s_11_16: Bits = (Bits::new(
            ((s_11_10) >> (s_11_8)).value(),
            u16::try_from(s_11_15).unwrap(),
        ));
        // D s_11_17: read-var e:i64
        let s_11_17: i64 = fn_state.e;
        // D s_11_18: cast zx s_11_17 -> i
        let s_11_18: i128 = (i128::try_from(s_11_17).unwrap());
        // D s_11_19: cast zx s_11_2 -> i
        let s_11_19: i128 = (i128::try_from(s_11_2).unwrap());
        // D s_11_20: read-var result:bv
        let s_11_20: Bits = fn_state.result;
        // D s_11_21: call Elem_set(s_11_20, s_11_18, s_11_19, s_11_16)
        let s_11_21: Bits = Elem_set(state, tracer, s_11_20, s_11_18, s_11_19, s_11_16);
        // D s_11_22: write-var result <= s_11_21
        fn_state.result = s_11_21;
        // N s_11_23: jump b10
        return block_10(state, tracer, fn_state);
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
        // D s_13_2: write-var operand1 <= s_13_1
        fn_state.operand1 = s_13_1;
        // N s_13_3: jump b6
        return block_6(state, tracer, fn_state);
    }
}
