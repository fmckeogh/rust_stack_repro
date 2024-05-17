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
use AnyActiveElement::*;
use ActivePredicateElement::*;
use P_read::*;
use FPCR_read::*;
use BFMin::*;
use Elem_read::*;
use Zeros::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_BFMIN_Z_P_ZZ__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    dn: i64,
    g: i64,
    m: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        element1: u16,
        elements: i64,
        VLshadow_2457: i64,
        VLshadow_2458: i64,
        result: Bits,
        gs_185091: i64,
        operand1: Bits,
        mask: Bits,
        operand2: Bits,
        gs_734496: Bits,
        VL: i64,
        dn: i64,
        g: i64,
        m: i64,
    }
    let fn_state = FunctionState {
        VL,
        dn,
        g,
        m,
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
        // D s_0_3: write-var VLshadow#2457 <= s_0_2
        fn_state.VLshadow_2457 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckSVEEnabled(s_0_4)
        let s_0_5: () = CheckSVEEnabled(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#2457:i64
        let s_1_0: i64 = fn_state.VLshadow_2457;
        // D s_1_1: write-var VLshadow#2458 <= s_1_0
        fn_state.VLshadow_2458 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#2458:i64
        let s_1_3: i64 = fn_state.VLshadow_2458;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // C s_1_7: const #16s : i
        let s_1_7: i128 = 16;
        // D s_1_8: read-var VLshadow#2458:i64
        let s_1_8: i64 = fn_state.VLshadow_2458;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: div s_1_9 s_1_7
        let s_1_10: i128 = ((s_1_9) / (s_1_7));
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: write-var elements <= s_1_11
        fn_state.elements = s_1_11;
        // D s_1_13: cast zx s_1_6 -> i
        let s_1_13: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_14: cast reint s_1_13 -> i64
        let s_1_14: i64 = (s_1_13 as i64);
        // D s_1_15: read-var g:i64
        let s_1_15: i64 = fn_state.g;
        // D s_1_16: cast zx s_1_15 -> i
        let s_1_16: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_17: cast zx s_1_14 -> i
        let s_1_17: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_18: call P_read(s_1_16, s_1_17)
        let s_1_18: Bits = P_read(state, tracer, s_1_16, s_1_17);
        // D s_1_19: write-var mask <= s_1_18
        fn_state.mask = s_1_18;
        // D s_1_20: read-var VLshadow#2458:i64
        let s_1_20: i64 = fn_state.VLshadow_2458;
        // D s_1_21: cast zx s_1_20 -> i
        let s_1_21: i128 = (i128::try_from(s_1_20).unwrap());
        // D s_1_22: cast reint s_1_21 -> i64
        let s_1_22: i64 = (s_1_21 as i64);
        // D s_1_23: read-var dn:i64
        let s_1_23: i64 = fn_state.dn;
        // D s_1_24: cast zx s_1_23 -> i
        let s_1_24: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_25: cast zx s_1_22 -> i
        let s_1_25: i128 = (i128::try_from(s_1_22).unwrap());
        // D s_1_26: call Z_read(s_1_24, s_1_25)
        let s_1_26: Bits = Z_read(state, tracer, s_1_24, s_1_25);
        // D s_1_27: write-var operand1 <= s_1_26
        fn_state.operand1 = s_1_26;
        // C s_1_28: const #16s : i
        let s_1_28: i128 = 16;
        // D s_1_29: read-var mask:bv
        let s_1_29: Bits = fn_state.mask;
        // D s_1_30: call AnyActiveElement(s_1_29, s_1_28)
        let s_1_30: bool = AnyActiveElement(state, tracer, s_1_29, s_1_28);
        // N s_1_31: branch s_1_30 b11 b2
        if s_1_30 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var VLshadow#2458:i64
        let s_2_0: i64 = fn_state.VLshadow_2458;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: call Zeros(s_2_1)
        let s_2_2: Bits = Zeros(state, tracer, s_2_1);
        // D s_2_3: write-var operand2 <= s_2_2
        fn_state.operand2 = s_2_2;
        // N s_2_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0s : i64
        let s_3_0: i64 = 0;
        // C s_3_1: const #1s : i
        let s_3_1: i128 = 1;
        // D s_3_2: read-var elements:i64
        let s_3_2: i64 = fn_state.elements;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: sub s_3_3 s_3_1
        let s_3_4: i128 = ((s_3_3) - (s_3_1));
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: write-var gs#185091 <= s_3_5
        fn_state.gs_185091 = s_3_5;
        // D s_3_7: write-var e <= s_3_0
        fn_state.e = s_3_0;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#185091:i64
        let s_4_1: i64 = fn_state.gs_185091;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b10 b5
        if s_4_2 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #16s : i64
        let s_5_0: i64 = 16;
        // D s_5_1: read-var e:i64
        let s_5_1: i64 = fn_state.e;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // C s_5_3: cast zx s_5_0 -> i
        let s_5_3: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_4: read-var operand1:bv
        let s_5_4: Bits = fn_state.operand1;
        // D s_5_5: call Elem_read(s_5_4, s_5_2, s_5_3)
        let s_5_5: Bits = Elem_read(state, tracer, s_5_4, s_5_2, s_5_3);
        // D s_5_6: cast reint s_5_5 -> u16
        let s_5_6: u16 = (s_5_5.value() as u16);
        // D s_5_7: write-var element1 <= s_5_6
        fn_state.element1 = s_5_6;
        // C s_5_8: const #16s : i
        let s_5_8: i128 = 16;
        // D s_5_9: read-var e:i64
        let s_5_9: i64 = fn_state.e;
        // D s_5_10: cast zx s_5_9 -> i
        let s_5_10: i128 = (i128::try_from(s_5_9).unwrap());
        // D s_5_11: read-var mask:bv
        let s_5_11: Bits = fn_state.mask;
        // D s_5_12: call ActivePredicateElement(s_5_11, s_5_10, s_5_8)
        let s_5_12: bool = ActivePredicateElement(state, tracer, s_5_11, s_5_10, s_5_8);
        // N s_5_13: branch s_5_12 b8 b6
        if s_5_12 {
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
        // C s_6_0: const #16s : i64
        let s_6_0: i64 = 16;
        // D s_6_1: read-var e:i64
        let s_6_1: i64 = fn_state.e;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // C s_6_3: cast zx s_6_0 -> i
        let s_6_3: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_4: read-var element1:u16
        let s_6_4: u16 = fn_state.element1;
        // D s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 16u16);
        // D s_6_6: read-var result:bv
        let s_6_6: Bits = fn_state.result;
        // D s_6_7: call Elem_set(s_6_6, s_6_2, s_6_3, s_6_5)
        let s_6_7: Bits = Elem_set(state, tracer, s_6_6, s_6_2, s_6_3, s_6_5);
        // D s_6_8: write-var result <= s_6_7
        fn_state.result = s_6_7;
        // N s_6_9: jump b7
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
        // C s_8_0: const #16s : i64
        let s_8_0: i64 = 16;
        // D s_8_1: read-var e:i64
        let s_8_1: i64 = fn_state.e;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // C s_8_3: cast zx s_8_0 -> i
        let s_8_3: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_4: read-var operand2:bv
        let s_8_4: Bits = fn_state.operand2;
        // D s_8_5: call Elem_read(s_8_4, s_8_2, s_8_3)
        let s_8_5: Bits = Elem_read(state, tracer, s_8_4, s_8_2, s_8_3);
        // D s_8_6: cast reint s_8_5 -> u16
        let s_8_6: u16 = (s_8_5.value() as u16);
        // C s_8_7: const #() : ()
        let s_8_7: () = ();
        // S s_8_8: call FPCR_read(s_8_7)
        let s_8_8: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_8_7);
        // D s_8_9: read-var element1:u16
        let s_8_9: u16 = fn_state.element1;
        // D s_8_10: cast zx s_8_9 -> bv
        let s_8_10: Bits = Bits::new(s_8_9 as u128, 16u16);
        // D s_8_11: cast zx s_8_6 -> bv
        let s_8_11: Bits = Bits::new(s_8_6 as u128, 16u16);
        // D s_8_12: call BFMin(s_8_10, s_8_11, s_8_8)
        let s_8_12: Bits = BFMin(state, tracer, s_8_10, s_8_11, s_8_8);
        // D s_8_13: write-var gs#734496 <= s_8_12
        fn_state.gs_734496 = s_8_12;
        // N s_8_14: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#734496:bv
        let s_9_0: Bits = fn_state.gs_734496;
        // D s_9_1: cast reint s_9_0 -> u16
        let s_9_1: u16 = (s_9_0.value() as u16);
        // D s_9_2: read-var e:i64
        let s_9_2: i64 = fn_state.e;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // C s_9_4: const #16s : i64
        let s_9_4: i64 = 16;
        // C s_9_5: cast zx s_9_4 -> i
        let s_9_5: i128 = (i128::try_from(s_9_4).unwrap());
        // D s_9_6: cast zx s_9_1 -> bv
        let s_9_6: Bits = Bits::new(s_9_1 as u128, 16u16);
        // D s_9_7: read-var result:bv
        let s_9_7: Bits = fn_state.result;
        // D s_9_8: call Elem_set(s_9_7, s_9_3, s_9_5, s_9_6)
        let s_9_8: Bits = Elem_set(state, tracer, s_9_7, s_9_3, s_9_5, s_9_6);
        // D s_9_9: write-var result <= s_9_8
        fn_state.result = s_9_8;
        // N s_9_10: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var VLshadow#2458:i64
        let s_10_0: i64 = fn_state.VLshadow_2458;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: cast reint s_10_1 -> i64
        let s_10_2: i64 = (s_10_1 as i64);
        // D s_10_3: read-var dn:i64
        let s_10_3: i64 = fn_state.dn;
        // D s_10_4: cast zx s_10_3 -> i
        let s_10_4: i128 = (i128::try_from(s_10_3).unwrap());
        // D s_10_5: cast zx s_10_2 -> i
        let s_10_5: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_6: read-var result:bv
        let s_10_6: Bits = fn_state.result;
        // D s_10_7: call Z_set(s_10_4, s_10_5, s_10_6)
        let s_10_7: () = Z_set(state, tracer, s_10_4, s_10_5, s_10_6);
        // N s_10_8: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var VLshadow#2458:i64
        let s_11_0: i64 = fn_state.VLshadow_2458;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: cast reint s_11_1 -> i64
        let s_11_2: i64 = (s_11_1 as i64);
        // D s_11_3: read-var m:i64
        let s_11_3: i64 = fn_state.m;
        // D s_11_4: cast zx s_11_3 -> i
        let s_11_4: i128 = (i128::try_from(s_11_3).unwrap());
        // D s_11_5: cast zx s_11_2 -> i
        let s_11_5: i128 = (i128::try_from(s_11_2).unwrap());
        // D s_11_6: call Z_read(s_11_4, s_11_5)
        let s_11_6: Bits = Z_read(state, tracer, s_11_4, s_11_5);
        // D s_11_7: write-var operand2 <= s_11_6
        fn_state.operand2 = s_11_6;
        // N s_11_8: jump b3
        return block_3(state, tracer, fn_state);
    }
}
