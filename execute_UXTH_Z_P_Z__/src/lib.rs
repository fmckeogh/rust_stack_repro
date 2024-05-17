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
use u__id::*;
use CheckSVEEnabled::*;
use AnyActiveElement::*;
use ActivePredicateElement::*;
use P_read::*;
use Elem_read::*;
use Zeros::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_UXTH_Z_P_Z__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    g: i64,
    n: i64,
    s_esize: i64,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        e: i64,
        ga_282395: Bits,
        gs_193185: i64,
        esizeshadow_2801: i64,
        mask: Bits,
        gs_193192: bool,
        ga_282394: i64,
        VLshadow_2803: i64,
        elements: i64,
        element: Bits,
        result: Bits,
        VLshadow_2802: i64,
        VL: i64,
        d: i64,
        esize: i64,
        g: i64,
        n: i64,
        s_esize: i64,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        g,
        n,
        s_esize,
        is_unsigned,
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
        // D s_0_3: write-var esizeshadow#2801 <= s_0_2
        fn_state.esizeshadow_2801 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#2802 <= s_0_6
        fn_state.VLshadow_2802 = s_0_6;
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
        // D s_1_0: read-var VLshadow#2802:i64
        let s_1_0: i64 = fn_state.VLshadow_2802;
        // D s_1_1: write-var VLshadow#2803 <= s_1_0
        fn_state.VLshadow_2803 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#2803:i64
        let s_1_3: i64 = fn_state.VLshadow_2803;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#2803:i64
        let s_1_7: i64 = fn_state.VLshadow_2803;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: read-var esizeshadow#2801:i64
        let s_1_9: i64 = fn_state.esizeshadow_2801;
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
        // D s_1_21: read-var esizeshadow#2801:i64
        let s_1_21: i64 = fn_state.esizeshadow_2801;
        // D s_1_22: cast zx s_1_21 -> i
        let s_1_22: i128 = (i128::try_from(s_1_21).unwrap());
        // D s_1_23: read-var mask:bv
        let s_1_23: Bits = fn_state.mask;
        // D s_1_24: call AnyActiveElement(s_1_23, s_1_22)
        let s_1_24: bool = AnyActiveElement(state, tracer, s_1_23, s_1_22);
        // N s_1_25: branch s_1_24 b16 b2
        if s_1_24 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var VLshadow#2803:i64
        let s_2_0: i64 = fn_state.VLshadow_2803;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: call Zeros(s_2_1)
        let s_2_2: Bits = Zeros(state, tracer, s_2_1);
        // D s_2_3: write-var operand <= s_2_2
        fn_state.operand = s_2_2;
        // N s_2_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var VLshadow#2803:i64
        let s_3_0: i64 = fn_state.VLshadow_2803;
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
        // D s_3_6: call Z_read(s_3_4, s_3_5)
        let s_3_6: Bits = Z_read(state, tracer, s_3_4, s_3_5);
        // D s_3_7: write-var result <= s_3_6
        fn_state.result = s_3_6;
        // C s_3_8: const #0s : i64
        let s_3_8: i64 = 0;
        // C s_3_9: const #1s : i
        let s_3_9: i128 = 1;
        // D s_3_10: read-var elements:i64
        let s_3_10: i64 = fn_state.elements;
        // D s_3_11: cast zx s_3_10 -> i
        let s_3_11: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_12: sub s_3_11 s_3_9
        let s_3_12: i128 = ((s_3_11) - (s_3_9));
        // D s_3_13: cast reint s_3_12 -> i64
        let s_3_13: i64 = (s_3_12 as i64);
        // D s_3_14: write-var gs#193185 <= s_3_13
        fn_state.gs_193185 = s_3_13;
        // D s_3_15: write-var e <= s_3_8
        fn_state.e = s_3_8;
        // N s_3_16: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#193185:i64
        let s_4_1: i64 = fn_state.gs_193185;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b15 b5
        if s_4_2 {
            return block_15(state, tracer, fn_state);
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
        // D s_5_2: read-var esizeshadow#2801:i64
        let s_5_2: i64 = fn_state.esizeshadow_2801;
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
        // D s_8_0: read-var esizeshadow#2801:i64
        let s_8_0: i64 = fn_state.esizeshadow_2801;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: cast reint s_8_1 -> i64
        let s_8_2: i64 = (s_8_1 as i64);
        // D s_8_3: read-var e:i64
        let s_8_3: i64 = fn_state.e;
        // D s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // D s_8_5: cast zx s_8_2 -> i
        let s_8_5: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_6: read-var operand:bv
        let s_8_6: Bits = fn_state.operand;
        // D s_8_7: call Elem_read(s_8_6, s_8_4, s_8_5)
        let s_8_7: Bits = Elem_read(state, tracer, s_8_6, s_8_4, s_8_5);
        // D s_8_8: write-var element <= s_8_7
        fn_state.element = s_8_7;
        // D s_8_9: read-var s_esize:i64
        let s_8_9: i64 = fn_state.s_esize;
        // D s_8_10: cast zx s_8_9 -> i
        let s_8_10: i128 = (i128::try_from(s_8_9).unwrap());
        // D s_8_11: call __id(s_8_10)
        let s_8_11: i128 = u__id(state, tracer, s_8_10);
        // D s_8_12: cast reint s_8_11 -> i64
        let s_8_12: i64 = (s_8_11 as i64);
        // C s_8_13: const #1s : i
        let s_8_13: i128 = 1;
        // D s_8_14: cast zx s_8_12 -> i
        let s_8_14: i128 = (i128::try_from(s_8_12).unwrap());
        // D s_8_15: sub s_8_14 s_8_13
        let s_8_15: i128 = ((s_8_14) - (s_8_13));
        // D s_8_16: cast reint s_8_15 -> i64
        let s_8_16: i64 = (s_8_15 as i64);
        // C s_8_17: const #0s : i
        let s_8_17: i128 = 0;
        // D s_8_18: cast zx s_8_16 -> i
        let s_8_18: i128 = (i128::try_from(s_8_16).unwrap());
        // D s_8_19: cmp-le s_8_17 s_8_18
        let s_8_19: bool = ((s_8_17) <= (s_8_18));
        // N s_8_20: branch s_8_19 b14 b9
        if s_8_19 {
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
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#193192 <= s_9_0
        fn_state.gs_193192 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#193192:u8
        let s_10_0: bool = fn_state.gs_193192;
        // N s_10_1: assert s_10_0
        let s_10_1: () = assert!(s_10_0);
        // D s_10_2: read-var esizeshadow#2801:i64
        let s_10_2: i64 = fn_state.esizeshadow_2801;
        // D s_10_3: cast zx s_10_2 -> i
        let s_10_3: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_4: cast reint s_10_3 -> i64
        let s_10_4: i64 = (s_10_3 as i64);
        // D s_10_5: write-var ga#282394 <= s_10_4
        fn_state.ga_282394 = s_10_4;
        // D s_10_6: read-var is_unsigned:u8
        let s_10_6: bool = fn_state.is_unsigned;
        // N s_10_7: branch s_10_6 b13 b11
        if s_10_6 {
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
        // C s_11_0: const #0s : i
        let s_11_0: i128 = 0;
        // C s_11_1: const #16s : i
        let s_11_1: i128 = 16;
        // D s_11_2: read-var element:bv
        let s_11_2: Bits = fn_state.element;
        // D s_11_3: bit-extract s_11_2 s_11_0 s_11_1
        let s_11_3: Bits = (Bits::new(
            ((s_11_2) >> (s_11_0)).value(),
            u16::try_from(s_11_1).unwrap(),
        ));
        // D s_11_4: cast reint s_11_3 -> u16
        let s_11_4: u16 = (s_11_3.value() as u16);
        // D s_11_5: cast zx s_11_4 -> bv
        let s_11_5: Bits = Bits::new(s_11_4 as u128, 16u16);
        // D s_11_6: read-var esizeshadow#2801:i64
        let s_11_6: i64 = fn_state.esizeshadow_2801;
        // D s_11_7: cast zx s_11_6 -> i
        let s_11_7: i128 = (i128::try_from(s_11_6).unwrap());
        // D s_11_8: bits-cast sx s_11_5 -> bv length s_11_7
        let s_11_8: Bits = s_11_5.sign_extend(s_11_7);
        // D s_11_9: write-var ga#282395 <= s_11_8
        fn_state.ga_282395 = s_11_8;
        // N s_11_10: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var e:i64
        let s_12_0: i64 = fn_state.e;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: read-var ga#282394:i64
        let s_12_2: i64 = fn_state.ga_282394;
        // D s_12_3: cast zx s_12_2 -> i
        let s_12_3: i128 = (i128::try_from(s_12_2).unwrap());
        // D s_12_4: read-var result:bv
        let s_12_4: Bits = fn_state.result;
        // D s_12_5: read-var ga#282395:bv
        let s_12_5: Bits = fn_state.ga_282395;
        // D s_12_6: call Elem_set(s_12_4, s_12_1, s_12_3, s_12_5)
        let s_12_6: Bits = Elem_set(state, tracer, s_12_4, s_12_1, s_12_3, s_12_5);
        // D s_12_7: write-var result <= s_12_6
        fn_state.result = s_12_6;
        // N s_12_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0s : i
        let s_13_0: i128 = 0;
        // C s_13_1: const #16s : i
        let s_13_1: i128 = 16;
        // D s_13_2: read-var element:bv
        let s_13_2: Bits = fn_state.element;
        // D s_13_3: bit-extract s_13_2 s_13_0 s_13_1
        let s_13_3: Bits = (Bits::new(
            ((s_13_2) >> (s_13_0)).value(),
            u16::try_from(s_13_1).unwrap(),
        ));
        // D s_13_4: cast reint s_13_3 -> u16
        let s_13_4: u16 = (s_13_3.value() as u16);
        // D s_13_5: cast zx s_13_4 -> bv
        let s_13_5: Bits = Bits::new(s_13_4 as u128, 16u16);
        // D s_13_6: read-var esizeshadow#2801:i64
        let s_13_6: i64 = fn_state.esizeshadow_2801;
        // D s_13_7: cast zx s_13_6 -> i
        let s_13_7: i128 = (i128::try_from(s_13_6).unwrap());
        // D s_13_8: bits-cast zx s_13_5 -> bv length s_13_7
        let s_13_8: Bits = s_13_5.zero_extend(s_13_7);
        // D s_13_9: write-var ga#282395 <= s_13_8
        fn_state.ga_282395 = s_13_8;
        // N s_13_10: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var s_esize:i64
        let s_14_0: i64 = fn_state.s_esize;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: call __id(s_14_1)
        let s_14_2: i128 = u__id(state, tracer, s_14_1);
        // D s_14_3: cast reint s_14_2 -> i64
        let s_14_3: i64 = (s_14_2 as i64);
        // C s_14_4: const #1s : i
        let s_14_4: i128 = 1;
        // D s_14_5: cast zx s_14_3 -> i
        let s_14_5: i128 = (i128::try_from(s_14_3).unwrap());
        // D s_14_6: sub s_14_5 s_14_4
        let s_14_6: i128 = ((s_14_5) - (s_14_4));
        // D s_14_7: cast reint s_14_6 -> i64
        let s_14_7: i64 = (s_14_6 as i64);
        // D s_14_8: read-var esizeshadow#2801:i64
        let s_14_8: i64 = fn_state.esizeshadow_2801;
        // D s_14_9: cast zx s_14_8 -> i
        let s_14_9: i128 = (i128::try_from(s_14_8).unwrap());
        // D s_14_10: call __id(s_14_9)
        let s_14_10: i128 = u__id(state, tracer, s_14_9);
        // D s_14_11: cast reint s_14_10 -> i64
        let s_14_11: i64 = (s_14_10 as i64);
        // D s_14_12: cast zx s_14_7 -> i
        let s_14_12: i128 = (i128::try_from(s_14_7).unwrap());
        // D s_14_13: cast zx s_14_11 -> i
        let s_14_13: i128 = (i128::try_from(s_14_11).unwrap());
        // D s_14_14: cmp-lt s_14_12 s_14_13
        let s_14_14: bool = ((s_14_12) < (s_14_13));
        // D s_14_15: write-var gs#193192 <= s_14_14
        fn_state.gs_193192 = s_14_14;
        // N s_14_16: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var VLshadow#2803:i64
        let s_15_0: i64 = fn_state.VLshadow_2803;
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
        // D s_15_7: call Z_set(s_15_4, s_15_5, s_15_6)
        let s_15_7: () = Z_set(state, tracer, s_15_4, s_15_5, s_15_6);
        // N s_15_8: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var VLshadow#2803:i64
        let s_16_0: i64 = fn_state.VLshadow_2803;
        // D s_16_1: cast zx s_16_0 -> i
        let s_16_1: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_2: cast reint s_16_1 -> i64
        let s_16_2: i64 = (s_16_1 as i64);
        // D s_16_3: read-var n:i64
        let s_16_3: i64 = fn_state.n;
        // D s_16_4: cast zx s_16_3 -> i
        let s_16_4: i128 = (i128::try_from(s_16_3).unwrap());
        // D s_16_5: cast zx s_16_2 -> i
        let s_16_5: i128 = (i128::try_from(s_16_2).unwrap());
        // D s_16_6: call Z_read(s_16_4, s_16_5)
        let s_16_6: Bits = Z_read(state, tracer, s_16_4, s_16_5);
        // D s_16_7: write-var operand <= s_16_6
        fn_state.operand = s_16_6;
        // N s_16_8: jump b3
        return block_3(state, tracer, fn_state);
    }
}
