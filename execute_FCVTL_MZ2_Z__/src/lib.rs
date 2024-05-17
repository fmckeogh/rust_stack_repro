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
pub fn execute_FCVTL_MZ2_Z__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        gs_877389: Bits,
        element2: u16,
        p: i64,
        gs_295095: i64,
        VLshadow_7100: i64,
        result1: Bits,
        gs_877386: Bits,
        VLshadow_7101: i64,
        result0: Bits,
        res1: u32,
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
        // D s_0_3: write-var VLshadow#7100 <= s_0_2
        fn_state.VLshadow_7100 = s_0_2;
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
        // D s_1_0: read-var VLshadow#7100:i64
        let s_1_0: i64 = fn_state.VLshadow_7100;
        // D s_1_1: write-var VLshadow#7101 <= s_1_0
        fn_state.VLshadow_7101 = s_1_0;
        // C s_1_2: const #32s : i
        let s_1_2: i128 = 32;
        // D s_1_3: read-var VLshadow#7101:i64
        let s_1_3: i64 = fn_state.VLshadow_7101;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#7101:i64
        let s_1_7: i64 = fn_state.VLshadow_7101;
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
        // D s_1_20: write-var gs#295095 <= s_1_19
        fn_state.gs_295095 = s_1_19;
        // D s_1_21: write-var p <= s_1_15
        fn_state.p = s_1_15;
        // N s_1_22: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var p:i64
        let s_2_0: i64 = fn_state.p;
        // D s_2_1: read-var gs#295095:i64
        let s_2_1: i64 = fn_state.gs_295095;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b6 b3
        if s_2_2 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #2s : i
        let s_3_0: i128 = 2;
        // D s_3_1: read-var p:i64
        let s_3_1: i64 = fn_state.p;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: mul s_3_0 s_3_2
        let s_3_3: i128 = ((s_3_0) * (s_3_2));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // C s_3_5: const #0s : i
        let s_3_5: i128 = 0;
        // D s_3_6: cast zx s_3_4 -> i
        let s_3_6: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_7: add s_3_6 s_3_5
        let s_3_7: i128 = (s_3_6 + s_3_5);
        // D s_3_8: cast reint s_3_7 -> i64
        let s_3_8: i64 = (s_3_7 as i64);
        // C s_3_9: const #16s : i64
        let s_3_9: i64 = 16;
        // D s_3_10: cast zx s_3_8 -> i
        let s_3_10: i128 = (i128::try_from(s_3_8).unwrap());
        // C s_3_11: cast zx s_3_9 -> i
        let s_3_11: i128 = (i128::try_from(s_3_9).unwrap());
        // D s_3_12: read-var operand:bv
        let s_3_12: Bits = fn_state.operand;
        // D s_3_13: call Elem_read(s_3_12, s_3_10, s_3_11)
        let s_3_13: Bits = Elem_read(state, tracer, s_3_12, s_3_10, s_3_11);
        // D s_3_14: cast reint s_3_13 -> u16
        let s_3_14: u16 = (s_3_13.value() as u16);
        // C s_3_15: const #2s : i
        let s_3_15: i128 = 2;
        // D s_3_16: read-var p:i64
        let s_3_16: i64 = fn_state.p;
        // D s_3_17: cast zx s_3_16 -> i
        let s_3_17: i128 = (i128::try_from(s_3_16).unwrap());
        // D s_3_18: mul s_3_15 s_3_17
        let s_3_18: i128 = ((s_3_15) * (s_3_17));
        // D s_3_19: cast reint s_3_18 -> i64
        let s_3_19: i64 = (s_3_18 as i64);
        // C s_3_20: const #1s : i
        let s_3_20: i128 = 1;
        // D s_3_21: cast zx s_3_19 -> i
        let s_3_21: i128 = (i128::try_from(s_3_19).unwrap());
        // D s_3_22: add s_3_21 s_3_20
        let s_3_22: i128 = (s_3_21 + s_3_20);
        // D s_3_23: cast reint s_3_22 -> i64
        let s_3_23: i64 = (s_3_22 as i64);
        // C s_3_24: const #16s : i64
        let s_3_24: i64 = 16;
        // D s_3_25: cast zx s_3_23 -> i
        let s_3_25: i128 = (i128::try_from(s_3_23).unwrap());
        // C s_3_26: cast zx s_3_24 -> i
        let s_3_26: i128 = (i128::try_from(s_3_24).unwrap());
        // D s_3_27: read-var operand:bv
        let s_3_27: Bits = fn_state.operand;
        // D s_3_28: call Elem_read(s_3_27, s_3_25, s_3_26)
        let s_3_28: Bits = Elem_read(state, tracer, s_3_27, s_3_25, s_3_26);
        // D s_3_29: cast reint s_3_28 -> u16
        let s_3_29: u16 = (s_3_28.value() as u16);
        // D s_3_30: write-var element2 <= s_3_29
        fn_state.element2 = s_3_29;
        // C s_3_31: const #() : ()
        let s_3_31: () = ();
        // S s_3_32: call FPCR_read(s_3_31)
        let s_3_32: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_3_31);
        // C s_3_33: const #32s : i64
        let s_3_33: i64 = 32;
        // D s_3_34: cast zx s_3_14 -> bv
        let s_3_34: Bits = Bits::new(s_3_14 as u128, 16u16);
        // D s_3_35: call FPConvertSVE__1(s_3_34, s_3_32, s_3_33)
        let s_3_35: Bits = FPConvertSVE__1(state, tracer, s_3_34, s_3_32, s_3_33);
        // D s_3_36: write-var gs#877386 <= s_3_35
        fn_state.gs_877386 = s_3_35;
        // N s_3_37: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#877386:bv
        let s_4_0: Bits = fn_state.gs_877386;
        // D s_4_1: cast reint s_4_0 -> u32
        let s_4_1: u32 = (s_4_0.value() as u32);
        // D s_4_2: write-var res1 <= s_4_1
        fn_state.res1 = s_4_1;
        // C s_4_3: const #() : ()
        let s_4_3: () = ();
        // S s_4_4: call FPCR_read(s_4_3)
        let s_4_4: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_4_3);
        // C s_4_5: const #32s : i64
        let s_4_5: i64 = 32;
        // D s_4_6: read-var element2:u16
        let s_4_6: u16 = fn_state.element2;
        // D s_4_7: cast zx s_4_6 -> bv
        let s_4_7: Bits = Bits::new(s_4_6 as u128, 16u16);
        // D s_4_8: call FPConvertSVE__1(s_4_7, s_4_4, s_4_5)
        let s_4_8: Bits = FPConvertSVE__1(state, tracer, s_4_7, s_4_4, s_4_5);
        // D s_4_9: write-var gs#877389 <= s_4_8
        fn_state.gs_877389 = s_4_8;
        // N s_4_10: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#877389:bv
        let s_5_0: Bits = fn_state.gs_877389;
        // D s_5_1: cast reint s_5_0 -> u32
        let s_5_1: u32 = (s_5_0.value() as u32);
        // C s_5_2: const #32s : i64
        let s_5_2: i64 = 32;
        // D s_5_3: read-var p:i64
        let s_5_3: i64 = fn_state.p;
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // C s_5_5: cast zx s_5_2 -> i
        let s_5_5: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_6: read-var res1:u32
        let s_5_6: u32 = fn_state.res1;
        // D s_5_7: cast zx s_5_6 -> bv
        let s_5_7: Bits = Bits::new(s_5_6 as u128, 32u16);
        // D s_5_8: read-var result0:bv
        let s_5_8: Bits = fn_state.result0;
        // D s_5_9: call Elem_set(s_5_8, s_5_4, s_5_5, s_5_7)
        let s_5_9: Bits = Elem_set(state, tracer, s_5_8, s_5_4, s_5_5, s_5_7);
        // D s_5_10: write-var result0 <= s_5_9
        fn_state.result0 = s_5_9;
        // C s_5_11: const #32s : i64
        let s_5_11: i64 = 32;
        // D s_5_12: read-var p:i64
        let s_5_12: i64 = fn_state.p;
        // D s_5_13: cast zx s_5_12 -> i
        let s_5_13: i128 = (i128::try_from(s_5_12).unwrap());
        // C s_5_14: cast zx s_5_11 -> i
        let s_5_14: i128 = (i128::try_from(s_5_11).unwrap());
        // D s_5_15: cast zx s_5_1 -> bv
        let s_5_15: Bits = Bits::new(s_5_1 as u128, 32u16);
        // D s_5_16: read-var result1:bv
        let s_5_16: Bits = fn_state.result1;
        // D s_5_17: call Elem_set(s_5_16, s_5_13, s_5_14, s_5_15)
        let s_5_17: Bits = Elem_set(state, tracer, s_5_16, s_5_13, s_5_14, s_5_15);
        // D s_5_18: write-var result1 <= s_5_17
        fn_state.result1 = s_5_17;
        // D s_5_19: read-var p:i64
        let s_5_19: i64 = fn_state.p;
        // C s_5_20: const #1s : i64
        let s_5_20: i64 = 1;
        // D s_5_21: add s_5_19 s_5_20
        let s_5_21: i64 = (s_5_19 + s_5_20);
        // D s_5_22: write-var p <= s_5_21
        fn_state.p = s_5_21;
        // N s_5_23: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0s : i
        let s_6_0: i128 = 0;
        // D s_6_1: read-var d:i64
        let s_6_1: i64 = fn_state.d;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: add s_6_2 s_6_0
        let s_6_3: i128 = (s_6_2 + s_6_0);
        // D s_6_4: cast reint s_6_3 -> i64
        let s_6_4: i64 = (s_6_3 as i64);
        // D s_6_5: read-var VLshadow#7101:i64
        let s_6_5: i64 = fn_state.VLshadow_7101;
        // D s_6_6: cast zx s_6_5 -> i
        let s_6_6: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_7: cast reint s_6_6 -> i64
        let s_6_7: i64 = (s_6_6 as i64);
        // D s_6_8: cast zx s_6_4 -> i
        let s_6_8: i128 = (i128::try_from(s_6_4).unwrap());
        // D s_6_9: cast zx s_6_7 -> i
        let s_6_9: i128 = (i128::try_from(s_6_7).unwrap());
        // D s_6_10: read-var result0:bv
        let s_6_10: Bits = fn_state.result0;
        // D s_6_11: call Z_set(s_6_8, s_6_9, s_6_10)
        let s_6_11: () = Z_set(state, tracer, s_6_8, s_6_9, s_6_10);
        // C s_6_12: const #1s : i
        let s_6_12: i128 = 1;
        // D s_6_13: read-var d:i64
        let s_6_13: i64 = fn_state.d;
        // D s_6_14: cast zx s_6_13 -> i
        let s_6_14: i128 = (i128::try_from(s_6_13).unwrap());
        // D s_6_15: add s_6_14 s_6_12
        let s_6_15: i128 = (s_6_14 + s_6_12);
        // D s_6_16: cast reint s_6_15 -> i64
        let s_6_16: i64 = (s_6_15 as i64);
        // D s_6_17: read-var VLshadow#7101:i64
        let s_6_17: i64 = fn_state.VLshadow_7101;
        // D s_6_18: cast zx s_6_17 -> i
        let s_6_18: i128 = (i128::try_from(s_6_17).unwrap());
        // D s_6_19: cast reint s_6_18 -> i64
        let s_6_19: i64 = (s_6_18 as i64);
        // D s_6_20: cast zx s_6_16 -> i
        let s_6_20: i128 = (i128::try_from(s_6_16).unwrap());
        // D s_6_21: cast zx s_6_19 -> i
        let s_6_21: i128 = (i128::try_from(s_6_19).unwrap());
        // D s_6_22: read-var result1:bv
        let s_6_22: Bits = fn_state.result1;
        // D s_6_23: call Z_set(s_6_20, s_6_21, s_6_22)
        let s_6_23: () = Z_set(state, tracer, s_6_20, s_6_21, s_6_22);
        // N s_6_24: return
        return;
    }
}
