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
use CheckStreamingSVEAndZAEnabled::*;
use integer_subrange::*;
use fmod_int::*;
use X_read::*;
use Elem_read::*;
use ZAvector_set::*;
use Z_read::*;
use ZAvector_read::*;
use common::*;
pub fn execute_SMLSLL_ZA_ZZV_2x1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    esize: i64,
    m: i64,
    n: i64,
    nreg: i64,
    offset: i64,
    v: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        e: i64,
        esizeshadow_6368: i64,
        operand3: Bits,
        gs_277622: i64,
        gs_277608: i64,
        vec: i128,
        vstride: i64,
        VLshadow_6369: i64,
        vecshadow_6371: i128,
        VLshadow_6370: i64,
        elements: i64,
        result: Bits,
        i: i64,
        operand1: Bits,
        operand2: Bits,
        VL: i64,
        esize: i64,
        m: i64,
        n: i64,
        nreg: i64,
        offset: i64,
        v: i64,
    }
    let fn_state = FunctionState {
        VL,
        esize,
        m,
        n,
        nreg,
        offset,
        v,
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
        // D s_0_3: write-var esizeshadow#6368 <= s_0_2
        fn_state.esizeshadow_6368 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#6369 <= s_0_6
        fn_state.VLshadow_6369 = s_0_6;
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call CheckStreamingSVEAndZAEnabled(s_0_8)
        let s_0_9: () = CheckStreamingSVEAndZAEnabled(state, tracer, s_0_8);
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#6369:i64
        let s_1_0: i64 = fn_state.VLshadow_6369;
        // D s_1_1: write-var VLshadow#6370 <= s_1_0
        fn_state.VLshadow_6370 = s_1_0;
        // D s_1_2: read-var VLshadow#6370:i64
        let s_1_2: i64 = fn_state.VLshadow_6370;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: read-var esizeshadow#6368:i64
        let s_1_4: i64 = fn_state.esizeshadow_6368;
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: div s_1_3 s_1_5
        let s_1_6: i128 = ((s_1_3) / (s_1_5));
        // D s_1_7: cast reint s_1_6 -> i64
        let s_1_7: i64 = (s_1_6 as i64);
        // D s_1_8: write-var elements <= s_1_7
        fn_state.elements = s_1_7;
        // C s_1_9: const #8s : i
        let s_1_9: i128 = 8;
        // D s_1_10: read-var VLshadow#6370:i64
        let s_1_10: i64 = fn_state.VLshadow_6370;
        // D s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_12: div s_1_11 s_1_9
        let s_1_12: i128 = ((s_1_11) / (s_1_9));
        // D s_1_13: cast reint s_1_12 -> i64
        let s_1_13: i64 = (s_1_12 as i64);
        // D s_1_14: cast zx s_1_13 -> i
        let s_1_14: i128 = (i128::try_from(s_1_13).unwrap());
        // D s_1_15: read-var nreg:i64
        let s_1_15: i64 = fn_state.nreg;
        // D s_1_16: cast zx s_1_15 -> i
        let s_1_16: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_17: div s_1_14 s_1_16
        let s_1_17: i128 = ((s_1_14) / (s_1_16));
        // D s_1_18: cast reint s_1_17 -> i64
        let s_1_18: i64 = (s_1_17 as i64);
        // D s_1_19: write-var vstride <= s_1_18
        fn_state.vstride = s_1_18;
        // C s_1_20: const #32s : i64
        let s_1_20: i64 = 32;
        // D s_1_21: read-var v:i64
        let s_1_21: i64 = fn_state.v;
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
        // D s_1_34: read-var vstride:i64
        let s_1_34: i64 = fn_state.vstride;
        // D s_1_35: cast zx s_1_34 -> i
        let s_1_35: i128 = (i128::try_from(s_1_34).unwrap());
        // D s_1_36: mod s_1_33 s_1_35
        let s_1_36: i128 = ((s_1_33) % (s_1_35));
        // D s_1_37: write-var vec <= s_1_36
        fn_state.vec = s_1_36;
        // C s_1_38: const #4s : i
        let s_1_38: i128 = 4;
        // D s_1_39: read-var vec:i
        let s_1_39: i128 = fn_state.vec;
        // D s_1_40: call fmod_int(s_1_39, s_1_38)
        let s_1_40: i128 = fmod_int(state, tracer, s_1_39, s_1_38);
        // D s_1_41: read-var vec:i
        let s_1_41: i128 = fn_state.vec;
        // D s_1_42: sub s_1_41 s_1_40
        let s_1_42: i128 = ((s_1_41) - (s_1_40));
        // D s_1_43: write-var vec <= s_1_42
        fn_state.vec = s_1_42;
        // C s_1_44: const #0s : i64
        let s_1_44: i64 = 0;
        // C s_1_45: const #1s : i
        let s_1_45: i128 = 1;
        // D s_1_46: read-var nreg:i64
        let s_1_46: i64 = fn_state.nreg;
        // D s_1_47: cast zx s_1_46 -> i
        let s_1_47: i128 = (i128::try_from(s_1_46).unwrap());
        // D s_1_48: sub s_1_47 s_1_45
        let s_1_48: i128 = ((s_1_47) - (s_1_45));
        // D s_1_49: cast reint s_1_48 -> i64
        let s_1_49: i64 = (s_1_48 as i64);
        // D s_1_50: write-var gs#277608 <= s_1_49
        fn_state.gs_277608 = s_1_49;
        // D s_1_51: write-var r <= s_1_44
        fn_state.r = s_1_44;
        // N s_1_52: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var r:i64
        let s_2_0: i64 = fn_state.r;
        // D s_2_1: read-var gs#277608:i64
        let s_2_1: i64 = fn_state.gs_277608;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b10 b3
        if s_2_2 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var n:i64
        let s_3_0: i64 = fn_state.n;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var r:i64
        let s_3_2: i64 = fn_state.r;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: add s_3_1 s_3_3
        let s_3_4: i128 = (s_3_1 + s_3_3);
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // C s_3_6: const #32s : i
        let s_3_6: i128 = 32;
        // D s_3_7: cast zx s_3_5 -> i
        let s_3_7: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_8: mod s_3_7 s_3_6
        let s_3_8: i128 = ((s_3_7) % (s_3_6));
        // D s_3_9: cast reint s_3_8 -> i64
        let s_3_9: i64 = (s_3_8 as i64);
        // D s_3_10: read-var VLshadow#6370:i64
        let s_3_10: i64 = fn_state.VLshadow_6370;
        // D s_3_11: cast zx s_3_10 -> i
        let s_3_11: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_12: cast reint s_3_11 -> i64
        let s_3_12: i64 = (s_3_11 as i64);
        // D s_3_13: cast zx s_3_9 -> i
        let s_3_13: i128 = (i128::try_from(s_3_9).unwrap());
        // D s_3_14: cast zx s_3_12 -> i
        let s_3_14: i128 = (i128::try_from(s_3_12).unwrap());
        // D s_3_15: call Z_read(s_3_13, s_3_14)
        let s_3_15: Bits = Z_read(state, tracer, s_3_13, s_3_14);
        // D s_3_16: write-var operand1 <= s_3_15
        fn_state.operand1 = s_3_15;
        // D s_3_17: read-var VLshadow#6370:i64
        let s_3_17: i64 = fn_state.VLshadow_6370;
        // D s_3_18: cast zx s_3_17 -> i
        let s_3_18: i128 = (i128::try_from(s_3_17).unwrap());
        // D s_3_19: cast reint s_3_18 -> i64
        let s_3_19: i64 = (s_3_18 as i64);
        // D s_3_20: read-var m:i64
        let s_3_20: i64 = fn_state.m;
        // D s_3_21: cast zx s_3_20 -> i
        let s_3_21: i128 = (i128::try_from(s_3_20).unwrap());
        // D s_3_22: cast zx s_3_19 -> i
        let s_3_22: i128 = (i128::try_from(s_3_19).unwrap());
        // D s_3_23: call Z_read(s_3_21, s_3_22)
        let s_3_23: Bits = Z_read(state, tracer, s_3_21, s_3_22);
        // D s_3_24: write-var operand2 <= s_3_23
        fn_state.operand2 = s_3_23;
        // D s_3_25: read-var vec:i
        let s_3_25: i128 = fn_state.vec;
        // D s_3_26: write-var vecshadow#6371 <= s_3_25
        fn_state.vecshadow_6371 = s_3_25;
        // C s_3_27: const #0s : i64
        let s_3_27: i64 = 0;
        // D s_3_28: write-var i <= s_3_27
        fn_state.i = s_3_27;
        // N s_3_29: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var i:i64
        let s_4_0: i64 = fn_state.i;
        // C s_4_1: const #3s : i64
        let s_4_1: i64 = 3;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b9 b5
        if s_4_2 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var i:i64
        let s_5_0: i64 = fn_state.i;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: read-var vecshadow#6371:i
        let s_5_2: i128 = fn_state.vecshadow_6371;
        // D s_5_3: add s_5_2 s_5_1
        let s_5_3: i128 = (s_5_2 + s_5_1);
        // D s_5_4: read-var VLshadow#6370:i64
        let s_5_4: i64 = fn_state.VLshadow_6370;
        // D s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_6: cast reint s_5_5 -> i64
        let s_5_6: i64 = (s_5_5 as i64);
        // D s_5_7: cast zx s_5_6 -> i
        let s_5_7: i128 = (i128::try_from(s_5_6).unwrap());
        // D s_5_8: call ZAvector_read(s_5_3, s_5_7)
        let s_5_8: Bits = ZAvector_read(state, tracer, s_5_3, s_5_7);
        // D s_5_9: write-var operand3 <= s_5_8
        fn_state.operand3 = s_5_8;
        // C s_5_10: const #0s : i64
        let s_5_10: i64 = 0;
        // C s_5_11: const #1s : i
        let s_5_11: i128 = 1;
        // D s_5_12: read-var elements:i64
        let s_5_12: i64 = fn_state.elements;
        // D s_5_13: cast zx s_5_12 -> i
        let s_5_13: i128 = (i128::try_from(s_5_12).unwrap());
        // D s_5_14: sub s_5_13 s_5_11
        let s_5_14: i128 = ((s_5_13) - (s_5_11));
        // D s_5_15: cast reint s_5_14 -> i64
        let s_5_15: i64 = (s_5_14 as i64);
        // D s_5_16: write-var gs#277622 <= s_5_15
        fn_state.gs_277622 = s_5_15;
        // D s_5_17: write-var e <= s_5_10
        fn_state.e = s_5_10;
        // N s_5_18: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var e:i64
        let s_6_0: i64 = fn_state.e;
        // D s_6_1: read-var gs#277622:i64
        let s_6_1: i64 = fn_state.gs_277622;
        // D s_6_2: cmp-gt s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) > (s_6_1));
        // N s_6_3: branch s_6_2 b8 b7
        if s_6_2 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #4s : i
        let s_7_0: i128 = 4;
        // D s_7_1: read-var e:i64
        let s_7_1: i64 = fn_state.e;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: mul s_7_0 s_7_2
        let s_7_3: i128 = ((s_7_0) * (s_7_2));
        // D s_7_4: cast reint s_7_3 -> i64
        let s_7_4: i64 = (s_7_3 as i64);
        // D s_7_5: cast zx s_7_4 -> i
        let s_7_5: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_6: read-var i:i64
        let s_7_6: i64 = fn_state.i;
        // D s_7_7: cast zx s_7_6 -> i
        let s_7_7: i128 = (i128::try_from(s_7_6).unwrap());
        // D s_7_8: add s_7_5 s_7_7
        let s_7_8: i128 = (s_7_5 + s_7_7);
        // D s_7_9: cast reint s_7_8 -> i64
        let s_7_9: i64 = (s_7_8 as i64);
        // C s_7_10: const #4s : i
        let s_7_10: i128 = 4;
        // D s_7_11: read-var esizeshadow#6368:i64
        let s_7_11: i64 = fn_state.esizeshadow_6368;
        // D s_7_12: cast zx s_7_11 -> i
        let s_7_12: i128 = (i128::try_from(s_7_11).unwrap());
        // D s_7_13: div s_7_12 s_7_10
        let s_7_13: i128 = ((s_7_12) / (s_7_10));
        // D s_7_14: cast reint s_7_13 -> i64
        let s_7_14: i64 = (s_7_13 as i64);
        // D s_7_15: cast zx s_7_14 -> i
        let s_7_15: i128 = (i128::try_from(s_7_14).unwrap());
        // D s_7_16: cast reint s_7_15 -> i64
        let s_7_16: i64 = (s_7_15 as i64);
        // D s_7_17: cast zx s_7_9 -> i
        let s_7_17: i128 = (i128::try_from(s_7_9).unwrap());
        // D s_7_18: cast zx s_7_16 -> i
        let s_7_18: i128 = (i128::try_from(s_7_16).unwrap());
        // D s_7_19: read-var operand1:bv
        let s_7_19: Bits = fn_state.operand1;
        // D s_7_20: call Elem_read(s_7_19, s_7_17, s_7_18)
        let s_7_20: Bits = Elem_read(state, tracer, s_7_19, s_7_17, s_7_18);
        // D s_7_21: cast sx s_7_20 -> i
        let s_7_21: i128 = {
            let sign_bit = s_7_20.length() - 1;
            let mut result = s_7_20.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_7_22: cast reint s_7_21 -> i64
        let s_7_22: i64 = (s_7_21 as i64);
        // C s_7_23: const #4s : i
        let s_7_23: i128 = 4;
        // D s_7_24: read-var e:i64
        let s_7_24: i64 = fn_state.e;
        // D s_7_25: cast zx s_7_24 -> i
        let s_7_25: i128 = (i128::try_from(s_7_24).unwrap());
        // D s_7_26: mul s_7_23 s_7_25
        let s_7_26: i128 = ((s_7_23) * (s_7_25));
        // D s_7_27: cast reint s_7_26 -> i64
        let s_7_27: i64 = (s_7_26 as i64);
        // D s_7_28: cast zx s_7_27 -> i
        let s_7_28: i128 = (i128::try_from(s_7_27).unwrap());
        // D s_7_29: read-var i:i64
        let s_7_29: i64 = fn_state.i;
        // D s_7_30: cast zx s_7_29 -> i
        let s_7_30: i128 = (i128::try_from(s_7_29).unwrap());
        // D s_7_31: add s_7_28 s_7_30
        let s_7_31: i128 = (s_7_28 + s_7_30);
        // D s_7_32: cast reint s_7_31 -> i64
        let s_7_32: i64 = (s_7_31 as i64);
        // C s_7_33: const #4s : i
        let s_7_33: i128 = 4;
        // D s_7_34: read-var esizeshadow#6368:i64
        let s_7_34: i64 = fn_state.esizeshadow_6368;
        // D s_7_35: cast zx s_7_34 -> i
        let s_7_35: i128 = (i128::try_from(s_7_34).unwrap());
        // D s_7_36: div s_7_35 s_7_33
        let s_7_36: i128 = ((s_7_35) / (s_7_33));
        // D s_7_37: cast reint s_7_36 -> i64
        let s_7_37: i64 = (s_7_36 as i64);
        // D s_7_38: cast zx s_7_37 -> i
        let s_7_38: i128 = (i128::try_from(s_7_37).unwrap());
        // D s_7_39: cast reint s_7_38 -> i64
        let s_7_39: i64 = (s_7_38 as i64);
        // D s_7_40: cast zx s_7_32 -> i
        let s_7_40: i128 = (i128::try_from(s_7_32).unwrap());
        // D s_7_41: cast zx s_7_39 -> i
        let s_7_41: i128 = (i128::try_from(s_7_39).unwrap());
        // D s_7_42: read-var operand2:bv
        let s_7_42: Bits = fn_state.operand2;
        // D s_7_43: call Elem_read(s_7_42, s_7_40, s_7_41)
        let s_7_43: Bits = Elem_read(state, tracer, s_7_42, s_7_40, s_7_41);
        // D s_7_44: cast sx s_7_43 -> i
        let s_7_44: i128 = {
            let sign_bit = s_7_43.length() - 1;
            let mut result = s_7_43.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_7_45: cast reint s_7_44 -> i64
        let s_7_45: i64 = (s_7_44 as i64);
        // D s_7_46: read-var esizeshadow#6368:i64
        let s_7_46: i64 = fn_state.esizeshadow_6368;
        // D s_7_47: cast zx s_7_46 -> i
        let s_7_47: i128 = (i128::try_from(s_7_46).unwrap());
        // D s_7_48: cast reint s_7_47 -> i64
        let s_7_48: i64 = (s_7_47 as i64);
        // D s_7_49: read-var esizeshadow#6368:i64
        let s_7_49: i64 = fn_state.esizeshadow_6368;
        // D s_7_50: cast zx s_7_49 -> i
        let s_7_50: i128 = (i128::try_from(s_7_49).unwrap());
        // D s_7_51: cast reint s_7_50 -> i64
        let s_7_51: i64 = (s_7_50 as i64);
        // D s_7_52: read-var e:i64
        let s_7_52: i64 = fn_state.e;
        // D s_7_53: cast zx s_7_52 -> i
        let s_7_53: i128 = (i128::try_from(s_7_52).unwrap());
        // D s_7_54: cast zx s_7_51 -> i
        let s_7_54: i128 = (i128::try_from(s_7_51).unwrap());
        // D s_7_55: read-var operand3:bv
        let s_7_55: Bits = fn_state.operand3;
        // D s_7_56: call Elem_read(s_7_55, s_7_53, s_7_54)
        let s_7_56: Bits = Elem_read(state, tracer, s_7_55, s_7_53, s_7_54);
        // D s_7_57: cast zx s_7_22 -> i
        let s_7_57: i128 = (i128::try_from(s_7_22).unwrap());
        // D s_7_58: cast zx s_7_45 -> i
        let s_7_58: i128 = (i128::try_from(s_7_45).unwrap());
        // D s_7_59: mul s_7_57 s_7_58
        let s_7_59: i128 = ((s_7_57) * (s_7_58));
        // D s_7_60: cast reint s_7_59 -> i64
        let s_7_60: i64 = (s_7_59 as i64);
        // C s_7_61: const #1s : i
        let s_7_61: i128 = 1;
        // D s_7_62: read-var esizeshadow#6368:i64
        let s_7_62: i64 = fn_state.esizeshadow_6368;
        // D s_7_63: cast zx s_7_62 -> i
        let s_7_63: i128 = (i128::try_from(s_7_62).unwrap());
        // D s_7_64: sub s_7_63 s_7_61
        let s_7_64: i128 = ((s_7_63) - (s_7_61));
        // D s_7_65: cast reint s_7_64 -> i64
        let s_7_65: i64 = (s_7_64 as i64);
        // C s_7_66: const #0s : i
        let s_7_66: i128 = 0;
        // D s_7_67: cast zx s_7_60 -> i
        let s_7_67: i128 = (i128::try_from(s_7_60).unwrap());
        // D s_7_68: cast zx s_7_65 -> i
        let s_7_68: i128 = (i128::try_from(s_7_65).unwrap());
        // D s_7_69: call integer_subrange(s_7_67, s_7_68, s_7_66)
        let s_7_69: Bits = integer_subrange(state, tracer, s_7_67, s_7_68, s_7_66);
        // D s_7_70: sub s_7_56 s_7_69
        let s_7_70: Bits = ((s_7_56) - (s_7_69));
        // D s_7_71: read-var e:i64
        let s_7_71: i64 = fn_state.e;
        // D s_7_72: cast zx s_7_71 -> i
        let s_7_72: i128 = (i128::try_from(s_7_71).unwrap());
        // D s_7_73: cast zx s_7_48 -> i
        let s_7_73: i128 = (i128::try_from(s_7_48).unwrap());
        // D s_7_74: read-var result:bv
        let s_7_74: Bits = fn_state.result;
        // D s_7_75: call Elem_set(s_7_74, s_7_72, s_7_73, s_7_70)
        let s_7_75: Bits = Elem_set(state, tracer, s_7_74, s_7_72, s_7_73, s_7_70);
        // D s_7_76: write-var result <= s_7_75
        fn_state.result = s_7_75;
        // D s_7_77: read-var e:i64
        let s_7_77: i64 = fn_state.e;
        // C s_7_78: const #1s : i64
        let s_7_78: i64 = 1;
        // D s_7_79: add s_7_77 s_7_78
        let s_7_79: i64 = (s_7_77 + s_7_78);
        // D s_7_80: write-var e <= s_7_79
        fn_state.e = s_7_79;
        // N s_7_81: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var i:i64
        let s_8_0: i64 = fn_state.i;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: read-var vecshadow#6371:i
        let s_8_2: i128 = fn_state.vecshadow_6371;
        // D s_8_3: add s_8_2 s_8_1
        let s_8_3: i128 = (s_8_2 + s_8_1);
        // D s_8_4: read-var VLshadow#6370:i64
        let s_8_4: i64 = fn_state.VLshadow_6370;
        // D s_8_5: cast zx s_8_4 -> i
        let s_8_5: i128 = (i128::try_from(s_8_4).unwrap());
        // D s_8_6: cast reint s_8_5 -> i64
        let s_8_6: i64 = (s_8_5 as i64);
        // D s_8_7: cast zx s_8_6 -> i
        let s_8_7: i128 = (i128::try_from(s_8_6).unwrap());
        // D s_8_8: read-var result:bv
        let s_8_8: Bits = fn_state.result;
        // D s_8_9: call ZAvector_set(s_8_3, s_8_7, s_8_8)
        let s_8_9: () = ZAvector_set(state, tracer, s_8_3, s_8_7, s_8_8);
        // D s_8_10: read-var i:i64
        let s_8_10: i64 = fn_state.i;
        // C s_8_11: const #1s : i64
        let s_8_11: i64 = 1;
        // D s_8_12: add s_8_10 s_8_11
        let s_8_12: i64 = (s_8_10 + s_8_11);
        // D s_8_13: write-var i <= s_8_12
        fn_state.i = s_8_12;
        // N s_8_14: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var vstride:i64
        let s_9_0: i64 = fn_state.vstride;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: read-var vec:i
        let s_9_2: i128 = fn_state.vec;
        // D s_9_3: add s_9_2 s_9_1
        let s_9_3: i128 = (s_9_2 + s_9_1);
        // D s_9_4: write-var vec <= s_9_3
        fn_state.vec = s_9_3;
        // D s_9_5: read-var r:i64
        let s_9_5: i64 = fn_state.r;
        // C s_9_6: const #1s : i64
        let s_9_6: i64 = 1;
        // D s_9_7: add s_9_5 s_9_6
        let s_9_7: i64 = (s_9_5 + s_9_6);
        // D s_9_8: write-var r <= s_9_7
        fn_state.r = s_9_7;
        // N s_9_9: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: return
        return;
    }
}
