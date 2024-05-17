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
use FPCR_read::*;
use FPConvertSVE__1::*;
use Elem_read::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_FCVT_MZ2_Z__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        e: i64,
        VLshadow_7098: i64,
        gs_295047: i64,
        gs_877315: Bits,
        result: Bits,
        VLshadow_7099: i64,
        VL: i64,
        d: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
        n,
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
        // D s_0_3: write-var VLshadow#7098 <= s_0_2
        fn_state.VLshadow_7098 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckStreamingSVEEnabled(s_0_4)
        let s_0_5: () = CheckStreamingSVEEnabled(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#7098:i64
        let s_1_0: i64 = fn_state.VLshadow_7098;
        // D s_1_1: write-var VLshadow#7099 <= s_1_0
        fn_state.VLshadow_7099 = s_1_0;
        // C s_1_2: const #16s : i
        let s_1_2: i128 = 16;
        // D s_1_3: read-var VLshadow#7099:i64
        let s_1_3: i64 = fn_state.VLshadow_7099;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#7099:i64
        let s_1_7: i64 = fn_state.VLshadow_7099;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast reint s_1_8 -> i64
        let s_1_9: i64 = (s_1_8 as i64);
        // D s_1_10: read-var n:i64
        let s_1_10: i64 = fn_state.n;
        // D s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_12: cast zx s_1_9 -> i
        let s_1_12: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_13: call Z_read(s_1_11, s_1_12)
        let s_1_13: Bits = Z_read(state, tracer, s_1_11, s_1_12);
        // D s_1_14: write-var operand <= s_1_13
        fn_state.operand = s_1_13;
        // C s_1_15: const #0s : i64
        let s_1_15: i64 = 0;
        // C s_1_16: const #1s : i
        let s_1_16: i128 = 1;
        // D s_1_17: cast zx s_1_6 -> i
        let s_1_17: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_18: sub s_1_17 s_1_16
        let s_1_18: i128 = ((s_1_17) - (s_1_16));
        // D s_1_19: cast reint s_1_18 -> i64
        let s_1_19: i64 = (s_1_18 as i64);
        // D s_1_20: write-var gs#295047 <= s_1_19
        fn_state.gs_295047 = s_1_19;
        // D s_1_21: write-var e <= s_1_15
        fn_state.e = s_1_15;
        // N s_1_22: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#295047:i64
        let s_2_1: i64 = fn_state.gs_295047;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b5 b3
        if s_2_2 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #16s : i64
        let s_3_0: i64 = 16;
        // D s_3_1: read-var e:i64
        let s_3_1: i64 = fn_state.e;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // C s_3_3: cast zx s_3_0 -> i
        let s_3_3: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_4: read-var operand:bv
        let s_3_4: Bits = fn_state.operand;
        // D s_3_5: call Elem_read(s_3_4, s_3_2, s_3_3)
        let s_3_5: Bits = Elem_read(state, tracer, s_3_4, s_3_2, s_3_3);
        // D s_3_6: cast reint s_3_5 -> u16
        let s_3_6: u16 = (s_3_5.value() as u16);
        // C s_3_7: const #() : ()
        let s_3_7: () = ();
        // S s_3_8: call FPCR_read(s_3_7)
        let s_3_8: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_3_7);
        // C s_3_9: const #32s : i64
        let s_3_9: i64 = 32;
        // D s_3_10: cast zx s_3_6 -> bv
        let s_3_10: Bits = Bits::new(s_3_6 as u128, 16u16);
        // D s_3_11: call FPConvertSVE__1(s_3_10, s_3_8, s_3_9)
        let s_3_11: Bits = FPConvertSVE__1(state, tracer, s_3_10, s_3_8, s_3_9);
        // D s_3_12: write-var gs#877315 <= s_3_11
        fn_state.gs_877315 = s_3_11;
        // N s_3_13: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#877315:bv
        let s_4_0: Bits = fn_state.gs_877315;
        // D s_4_1: cast reint s_4_0 -> u32
        let s_4_1: u32 = (s_4_0.value() as u32);
        // C s_4_2: const #32s : i64
        let s_4_2: i64 = 32;
        // D s_4_3: read-var e:i64
        let s_4_3: i64 = fn_state.e;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // C s_4_5: cast zx s_4_2 -> i
        let s_4_5: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_6: cast zx s_4_1 -> bv
        let s_4_6: Bits = Bits::new(s_4_1 as u128, 32u16);
        // D s_4_7: read-var result:bv
        let s_4_7: Bits = fn_state.result;
        // D s_4_8: call Elem_set(s_4_7, s_4_4, s_4_5, s_4_6)
        let s_4_8: Bits = Elem_set(state, tracer, s_4_7, s_4_4, s_4_5, s_4_6);
        // D s_4_9: write-var result <= s_4_8
        fn_state.result = s_4_8;
        // D s_4_10: read-var e:i64
        let s_4_10: i64 = fn_state.e;
        // C s_4_11: const #1s : i64
        let s_4_11: i64 = 1;
        // D s_4_12: add s_4_10 s_4_11
        let s_4_12: i64 = (s_4_10 + s_4_11);
        // D s_4_13: write-var e <= s_4_12
        fn_state.e = s_4_12;
        // N s_4_14: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0s : i
        let s_5_0: i128 = 0;
        // D s_5_1: read-var d:i64
        let s_5_1: i64 = fn_state.d;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: add s_5_2 s_5_0
        let s_5_3: i128 = (s_5_2 + s_5_0);
        // D s_5_4: cast reint s_5_3 -> i64
        let s_5_4: i64 = (s_5_3 as i64);
        // D s_5_5: read-var VLshadow#7099:i64
        let s_5_5: i64 = fn_state.VLshadow_7099;
        // D s_5_6: cast zx s_5_5 -> i
        let s_5_6: i128 = (i128::try_from(s_5_5).unwrap());
        // D s_5_7: cast reint s_5_6 -> i64
        let s_5_7: i64 = (s_5_6 as i64);
        // C s_5_8: const #1s : i
        let s_5_8: i128 = 1;
        // D s_5_9: read-var VLshadow#7099:i64
        let s_5_9: i64 = fn_state.VLshadow_7099;
        // D s_5_10: cast zx s_5_9 -> i
        let s_5_10: i128 = (i128::try_from(s_5_9).unwrap());
        // D s_5_11: sub s_5_10 s_5_8
        let s_5_11: i128 = ((s_5_10) - (s_5_8));
        // D s_5_12: cast reint s_5_11 -> i64
        let s_5_12: i64 = (s_5_11 as i64);
        // C s_5_13: const #0s : i
        let s_5_13: i128 = 0;
        // D s_5_14: cast zx s_5_12 -> i
        let s_5_14: i128 = (i128::try_from(s_5_12).unwrap());
        // D s_5_15: read-var result:bv
        let s_5_15: Bits = fn_state.result;
        // C s_5_16: const #1s : i64
        let s_5_16: i64 = 1;
        // C s_5_17: cast zx s_5_16 -> i
        let s_5_17: i128 = (i128::try_from(s_5_16).unwrap());
        // D s_5_18: sub s_5_14 s_5_13
        let s_5_18: i128 = ((s_5_14) - (s_5_13));
        // D s_5_19: add s_5_18 s_5_17
        let s_5_19: i128 = (s_5_18 + s_5_17);
        // D s_5_20: bit-extract s_5_15 s_5_13 s_5_19
        let s_5_20: Bits = (Bits::new(
            ((s_5_15) >> (s_5_13)).value(),
            u16::try_from(s_5_19).unwrap(),
        ));
        // D s_5_21: cast zx s_5_4 -> i
        let s_5_21: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_22: cast zx s_5_7 -> i
        let s_5_22: i128 = (i128::try_from(s_5_7).unwrap());
        // D s_5_23: call Z_set(s_5_21, s_5_22, s_5_20)
        let s_5_23: () = Z_set(state, tracer, s_5_21, s_5_22, s_5_20);
        // C s_5_24: const #1s : i
        let s_5_24: i128 = 1;
        // D s_5_25: read-var d:i64
        let s_5_25: i64 = fn_state.d;
        // D s_5_26: cast zx s_5_25 -> i
        let s_5_26: i128 = (i128::try_from(s_5_25).unwrap());
        // D s_5_27: add s_5_26 s_5_24
        let s_5_27: i128 = (s_5_26 + s_5_24);
        // D s_5_28: cast reint s_5_27 -> i64
        let s_5_28: i64 = (s_5_27 as i64);
        // D s_5_29: read-var VLshadow#7099:i64
        let s_5_29: i64 = fn_state.VLshadow_7099;
        // D s_5_30: cast zx s_5_29 -> i
        let s_5_30: i128 = (i128::try_from(s_5_29).unwrap());
        // D s_5_31: cast reint s_5_30 -> i64
        let s_5_31: i64 = (s_5_30 as i64);
        // C s_5_32: const #2s : i
        let s_5_32: i128 = 2;
        // D s_5_33: read-var VLshadow#7099:i64
        let s_5_33: i64 = fn_state.VLshadow_7099;
        // D s_5_34: cast zx s_5_33 -> i
        let s_5_34: i128 = (i128::try_from(s_5_33).unwrap());
        // D s_5_35: mul s_5_32 s_5_34
        let s_5_35: i128 = ((s_5_32) * (s_5_34));
        // D s_5_36: cast reint s_5_35 -> i64
        let s_5_36: i64 = (s_5_35 as i64);
        // C s_5_37: const #1s : i
        let s_5_37: i128 = 1;
        // D s_5_38: cast zx s_5_36 -> i
        let s_5_38: i128 = (i128::try_from(s_5_36).unwrap());
        // D s_5_39: sub s_5_38 s_5_37
        let s_5_39: i128 = ((s_5_38) - (s_5_37));
        // D s_5_40: cast reint s_5_39 -> i64
        let s_5_40: i64 = (s_5_39 as i64);
        // D s_5_41: cast zx s_5_40 -> i
        let s_5_41: i128 = (i128::try_from(s_5_40).unwrap());
        // D s_5_42: read-var VLshadow#7099:i64
        let s_5_42: i64 = fn_state.VLshadow_7099;
        // D s_5_43: cast zx s_5_42 -> i
        let s_5_43: i128 = (i128::try_from(s_5_42).unwrap());
        // D s_5_44: read-var result:bv
        let s_5_44: Bits = fn_state.result;
        // C s_5_45: const #1s : i64
        let s_5_45: i64 = 1;
        // C s_5_46: cast zx s_5_45 -> i
        let s_5_46: i128 = (i128::try_from(s_5_45).unwrap());
        // D s_5_47: sub s_5_41 s_5_43
        let s_5_47: i128 = ((s_5_41) - (s_5_43));
        // D s_5_48: add s_5_47 s_5_46
        let s_5_48: i128 = (s_5_47 + s_5_46);
        // D s_5_49: bit-extract s_5_44 s_5_43 s_5_48
        let s_5_49: Bits = (Bits::new(
            ((s_5_44) >> (s_5_43)).value(),
            u16::try_from(s_5_48).unwrap(),
        ));
        // D s_5_50: cast zx s_5_28 -> i
        let s_5_50: i128 = (i128::try_from(s_5_28).unwrap());
        // D s_5_51: cast zx s_5_31 -> i
        let s_5_51: i128 = (i128::try_from(s_5_31).unwrap());
        // D s_5_52: call Z_set(s_5_50, s_5_51, s_5_49)
        let s_5_52: () = Z_set(state, tracer, s_5_50, s_5_51, s_5_49);
        // N s_5_53: return
        return;
    }
}
