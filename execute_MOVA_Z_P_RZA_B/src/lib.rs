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
use ZAslice_read::*;
use Elem_set::*;
use CheckStreamingSVEAndZAEnabled::*;
use P_read::*;
use ActivePredicateElement::*;
use X_read::*;
use Elem_read::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_MOVA_Z_P_RZA_B<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    g: i64,
    n: i64,
    offset: i64,
    s: i64,
    vertical: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        VLshadow_5499: i64,
        e: i64,
        gs_256837: i64,
        result: Bits,
        element: Bits,
        VLshadow_5500: i64,
        mask: Bits,
        VL: i64,
        d: i64,
        esize: i64,
        g: i64,
        n: i64,
        offset: i64,
        s: i64,
        vertical: bool,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        g,
        n,
        offset,
        s,
        vertical,
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
        // D s_0_3: write-var VLshadow#5499 <= s_0_2
        fn_state.VLshadow_5499 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckStreamingSVEAndZAEnabled(s_0_4)
        let s_0_5: () = CheckStreamingSVEAndZAEnabled(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#5499:i64
        let s_1_0: i64 = fn_state.VLshadow_5499;
        // D s_1_1: write-var VLshadow#5500 <= s_1_0
        fn_state.VLshadow_5500 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#5500:i64
        let s_1_3: i64 = fn_state.VLshadow_5500;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#5500:i64
        let s_1_7: i64 = fn_state.VLshadow_5500;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: read-var esize:i64
        let s_1_9: i64 = fn_state.esize;
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_11: div s_1_8 s_1_10
        let s_1_11: i128 = ((s_1_8) / (s_1_10));
        // D s_1_12: cast reint s_1_11 -> i64
        let s_1_12: i64 = (s_1_11 as i64);
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
        // C s_1_20: const #32s : i64
        let s_1_20: i64 = 32;
        // D s_1_21: read-var s:i64
        let s_1_21: i64 = fn_state.s;
        // D s_1_22: cast zx s_1_21 -> i
        let s_1_22: i128 = (i128::try_from(s_1_21).unwrap());
        // D s_1_23: call X_read(s_1_22, s_1_20)
        let s_1_23: Bits = X_read(state, tracer, s_1_22, s_1_20);
        // D s_1_24: cast reint s_1_23 -> u32
        let s_1_24: u32 = (s_1_23.value() as u32);
        // D s_1_25: cast zx s_1_24 -> bv
        let s_1_25: Bits = Bits::new(s_1_24 as u128, 32u16);
        // D s_1_26: cast zx s_1_25 -> i
        let s_1_26: i128 = (s_1_25.value() as i128);
        // D s_1_27: cast reint s_1_26 -> i64
        let s_1_27: i64 = (s_1_26 as i64);
        // D s_1_28: cast zx s_1_27 -> i
        let s_1_28: i128 = (i128::try_from(s_1_27).unwrap());
        // D s_1_29: read-var offset:i64
        let s_1_29: i64 = fn_state.offset;
        // D s_1_30: cast zx s_1_29 -> i
        let s_1_30: i128 = (i128::try_from(s_1_29).unwrap());
        // D s_1_31: add s_1_28 s_1_30
        let s_1_31: i128 = (s_1_28 + s_1_30);
        // D s_1_32: cast reint s_1_31 -> i64
        let s_1_32: i64 = (s_1_31 as i64);
        // D s_1_33: cast zx s_1_32 -> i
        let s_1_33: i128 = (i128::try_from(s_1_32).unwrap());
        // D s_1_34: cast zx s_1_12 -> i
        let s_1_34: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_35: mod s_1_33 s_1_34
        let s_1_35: i128 = ((s_1_33) % (s_1_34));
        // D s_1_36: cast reint s_1_35 -> i64
        let s_1_36: i64 = (s_1_35 as i64);
        // D s_1_37: read-var esize:i64
        let s_1_37: i64 = fn_state.esize;
        // D s_1_38: cast zx s_1_37 -> i
        let s_1_38: i128 = (i128::try_from(s_1_37).unwrap());
        // D s_1_39: cast reint s_1_38 -> i64
        let s_1_39: i64 = (s_1_38 as i64);
        // D s_1_40: read-var VLshadow#5500:i64
        let s_1_40: i64 = fn_state.VLshadow_5500;
        // D s_1_41: cast zx s_1_40 -> i
        let s_1_41: i128 = (i128::try_from(s_1_40).unwrap());
        // D s_1_42: cast reint s_1_41 -> i64
        let s_1_42: i64 = (s_1_41 as i64);
        // D s_1_43: read-var n:i64
        let s_1_43: i64 = fn_state.n;
        // D s_1_44: cast zx s_1_43 -> i
        let s_1_44: i128 = (i128::try_from(s_1_43).unwrap());
        // D s_1_45: cast zx s_1_39 -> i
        let s_1_45: i128 = (i128::try_from(s_1_39).unwrap());
        // D s_1_46: cast zx s_1_36 -> i
        let s_1_46: i128 = (i128::try_from(s_1_36).unwrap());
        // D s_1_47: cast zx s_1_42 -> i
        let s_1_47: i128 = (i128::try_from(s_1_42).unwrap());
        // D s_1_48: read-var vertical:u8
        let s_1_48: bool = fn_state.vertical;
        // D s_1_49: call ZAslice_read(s_1_44, s_1_45, s_1_48, s_1_46, s_1_47)
        let s_1_49: Bits = ZAslice_read(
            state,
            tracer,
            s_1_44,
            s_1_45,
            s_1_48,
            s_1_46,
            s_1_47,
        );
        // D s_1_50: write-var operand <= s_1_49
        fn_state.operand = s_1_49;
        // D s_1_51: read-var VLshadow#5500:i64
        let s_1_51: i64 = fn_state.VLshadow_5500;
        // D s_1_52: cast zx s_1_51 -> i
        let s_1_52: i128 = (i128::try_from(s_1_51).unwrap());
        // D s_1_53: cast reint s_1_52 -> i64
        let s_1_53: i64 = (s_1_52 as i64);
        // D s_1_54: read-var d:i64
        let s_1_54: i64 = fn_state.d;
        // D s_1_55: cast zx s_1_54 -> i
        let s_1_55: i128 = (i128::try_from(s_1_54).unwrap());
        // D s_1_56: cast zx s_1_53 -> i
        let s_1_56: i128 = (i128::try_from(s_1_53).unwrap());
        // D s_1_57: call Z_read(s_1_55, s_1_56)
        let s_1_57: Bits = Z_read(state, tracer, s_1_55, s_1_56);
        // D s_1_58: write-var result <= s_1_57
        fn_state.result = s_1_57;
        // C s_1_59: const #0s : i64
        let s_1_59: i64 = 0;
        // C s_1_60: const #1s : i
        let s_1_60: i128 = 1;
        // D s_1_61: cast zx s_1_12 -> i
        let s_1_61: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_62: sub s_1_61 s_1_60
        let s_1_62: i128 = ((s_1_61) - (s_1_60));
        // D s_1_63: cast reint s_1_62 -> i64
        let s_1_63: i64 = (s_1_62 as i64);
        // D s_1_64: write-var gs#256837 <= s_1_63
        fn_state.gs_256837 = s_1_63;
        // D s_1_65: write-var e <= s_1_59
        fn_state.e = s_1_59;
        // N s_1_66: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#256837:i64
        let s_2_1: i64 = fn_state.gs_256837;
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
        // D s_3_0: read-var esize:i64
        let s_3_0: i64 = fn_state.esize;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: cast reint s_3_1 -> i64
        let s_3_2: i64 = (s_3_1 as i64);
        // D s_3_3: read-var e:i64
        let s_3_3: i64 = fn_state.e;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: cast zx s_3_2 -> i
        let s_3_5: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_6: read-var operand:bv
        let s_3_6: Bits = fn_state.operand;
        // D s_3_7: call Elem_read(s_3_6, s_3_4, s_3_5)
        let s_3_7: Bits = Elem_read(state, tracer, s_3_6, s_3_4, s_3_5);
        // D s_3_8: write-var element <= s_3_7
        fn_state.element = s_3_7;
        // D s_3_9: read-var e:i64
        let s_3_9: i64 = fn_state.e;
        // D s_3_10: cast zx s_3_9 -> i
        let s_3_10: i128 = (i128::try_from(s_3_9).unwrap());
        // D s_3_11: read-var esize:i64
        let s_3_11: i64 = fn_state.esize;
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_13: read-var mask:bv
        let s_3_13: Bits = fn_state.mask;
        // D s_3_14: call ActivePredicateElement(s_3_13, s_3_10, s_3_12)
        let s_3_14: bool = ActivePredicateElement(state, tracer, s_3_13, s_3_10, s_3_12);
        // N s_3_15: branch s_3_14 b6 b4
        if s_3_14 {
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
        // N s_4_0: jump b5
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
        // D s_6_0: read-var esize:i64
        let s_6_0: i64 = fn_state.esize;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: cast reint s_6_1 -> i64
        let s_6_2: i64 = (s_6_1 as i64);
        // D s_6_3: read-var e:i64
        let s_6_3: i64 = fn_state.e;
        // D s_6_4: cast zx s_6_3 -> i
        let s_6_4: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_5: cast zx s_6_2 -> i
        let s_6_5: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_6: read-var result:bv
        let s_6_6: Bits = fn_state.result;
        // D s_6_7: read-var element:bv
        let s_6_7: Bits = fn_state.element;
        // D s_6_8: call Elem_set(s_6_6, s_6_4, s_6_5, s_6_7)
        let s_6_8: Bits = Elem_set(state, tracer, s_6_6, s_6_4, s_6_5, s_6_7);
        // D s_6_9: write-var result <= s_6_8
        fn_state.result = s_6_8;
        // N s_6_10: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var VLshadow#5500:i64
        let s_7_0: i64 = fn_state.VLshadow_5500;
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
        // D s_7_7: call Z_set(s_7_4, s_7_5, s_7_6)
        let s_7_7: () = Z_set(state, tracer, s_7_4, s_7_5, s_7_6);
        // N s_7_8: return
        return;
    }
}
