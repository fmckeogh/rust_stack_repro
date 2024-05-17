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
pub fn execute_SUMLALL_ZA_ZZi_S2xi<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    esize: i64,
    index: i64,
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
        gs_280172: i64,
        operand3: Bits,
        VLshadow_6465: i64,
        VLshadow_6464: i64,
        vecshadow_6466: i128,
        gs_280185: i64,
        vec: i128,
        vstride: i64,
        elements: i64,
        result: Bits,
        i: i64,
        operand1: Bits,
        eltspersegment: i64,
        operand2: Bits,
        VL: i64,
        esize: i64,
        index: i64,
        m: i64,
        n: i64,
        nreg: i64,
        offset: i64,
        v: i64,
    }
    let fn_state = FunctionState {
        VL,
        esize,
        index,
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
        // D s_0_0: read-var VL:i64
        let s_0_0: i64 = fn_state.VL;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var VLshadow#6464 <= s_0_2
        fn_state.VLshadow_6464 = s_0_2;
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
        // D s_1_0: read-var VLshadow#6464:i64
        let s_1_0: i64 = fn_state.VLshadow_6464;
        // D s_1_1: write-var VLshadow#6465 <= s_1_0
        fn_state.VLshadow_6465 = s_1_0;
        // D s_1_2: read-var VLshadow#6465:i64
        let s_1_2: i64 = fn_state.VLshadow_6465;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: read-var esize:i64
        let s_1_4: i64 = fn_state.esize;
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
        // D s_1_10: read-var VLshadow#6465:i64
        let s_1_10: i64 = fn_state.VLshadow_6465;
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
        // C s_1_20: const #128s : i
        let s_1_20: i128 = 128;
        // D s_1_21: read-var esize:i64
        let s_1_21: i64 = fn_state.esize;
        // D s_1_22: cast zx s_1_21 -> i
        let s_1_22: i128 = (i128::try_from(s_1_21).unwrap());
        // D s_1_23: div s_1_20 s_1_22
        let s_1_23: i128 = ((s_1_20) / (s_1_22));
        // D s_1_24: cast reint s_1_23 -> i64
        let s_1_24: i64 = (s_1_23 as i64);
        // D s_1_25: write-var eltspersegment <= s_1_24
        fn_state.eltspersegment = s_1_24;
        // C s_1_26: const #32s : i64
        let s_1_26: i64 = 32;
        // D s_1_27: read-var v:i64
        let s_1_27: i64 = fn_state.v;
        // D s_1_28: cast zx s_1_27 -> i
        let s_1_28: i128 = (i128::try_from(s_1_27).unwrap());
        // D s_1_29: call X_read(s_1_28, s_1_26)
        let s_1_29: Bits = X_read(state, tracer, s_1_28, s_1_26);
        // D s_1_30: cast reint s_1_29 -> u32
        let s_1_30: u32 = (s_1_29.value() as u32);
        // D s_1_31: cast zx s_1_30 -> bv
        let s_1_31: Bits = Bits::new(s_1_30 as u128, 32u16);
        // D s_1_32: cast zx s_1_31 -> i
        let s_1_32: i128 = (s_1_31.value() as i128);
        // D s_1_33: cast reint s_1_32 -> i64
        let s_1_33: i64 = (s_1_32 as i64);
        // D s_1_34: cast zx s_1_33 -> i
        let s_1_34: i128 = (i128::try_from(s_1_33).unwrap());
        // D s_1_35: read-var offset:i64
        let s_1_35: i64 = fn_state.offset;
        // D s_1_36: cast zx s_1_35 -> i
        let s_1_36: i128 = (i128::try_from(s_1_35).unwrap());
        // D s_1_37: add s_1_34 s_1_36
        let s_1_37: i128 = (s_1_34 + s_1_36);
        // D s_1_38: cast reint s_1_37 -> i64
        let s_1_38: i64 = (s_1_37 as i64);
        // D s_1_39: cast zx s_1_38 -> i
        let s_1_39: i128 = (i128::try_from(s_1_38).unwrap());
        // D s_1_40: read-var vstride:i64
        let s_1_40: i64 = fn_state.vstride;
        // D s_1_41: cast zx s_1_40 -> i
        let s_1_41: i128 = (i128::try_from(s_1_40).unwrap());
        // D s_1_42: mod s_1_39 s_1_41
        let s_1_42: i128 = ((s_1_39) % (s_1_41));
        // D s_1_43: write-var vec <= s_1_42
        fn_state.vec = s_1_42;
        // C s_1_44: const #4s : i
        let s_1_44: i128 = 4;
        // D s_1_45: read-var vec:i
        let s_1_45: i128 = fn_state.vec;
        // D s_1_46: call fmod_int(s_1_45, s_1_44)
        let s_1_46: i128 = fmod_int(state, tracer, s_1_45, s_1_44);
        // D s_1_47: read-var vec:i
        let s_1_47: i128 = fn_state.vec;
        // D s_1_48: sub s_1_47 s_1_46
        let s_1_48: i128 = ((s_1_47) - (s_1_46));
        // D s_1_49: write-var vec <= s_1_48
        fn_state.vec = s_1_48;
        // C s_1_50: const #0s : i64
        let s_1_50: i64 = 0;
        // C s_1_51: const #1s : i
        let s_1_51: i128 = 1;
        // D s_1_52: read-var nreg:i64
        let s_1_52: i64 = fn_state.nreg;
        // D s_1_53: cast zx s_1_52 -> i
        let s_1_53: i128 = (i128::try_from(s_1_52).unwrap());
        // D s_1_54: sub s_1_53 s_1_51
        let s_1_54: i128 = ((s_1_53) - (s_1_51));
        // D s_1_55: cast reint s_1_54 -> i64
        let s_1_55: i64 = (s_1_54 as i64);
        // D s_1_56: write-var gs#280172 <= s_1_55
        fn_state.gs_280172 = s_1_55;
        // D s_1_57: write-var r <= s_1_50
        fn_state.r = s_1_50;
        // N s_1_58: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var r:i64
        let s_2_0: i64 = fn_state.r;
        // D s_2_1: read-var gs#280172:i64
        let s_2_1: i64 = fn_state.gs_280172;
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
        // D s_3_6: read-var VLshadow#6465:i64
        let s_3_6: i64 = fn_state.VLshadow_6465;
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_8: cast reint s_3_7 -> i64
        let s_3_8: i64 = (s_3_7 as i64);
        // D s_3_9: cast zx s_3_5 -> i
        let s_3_9: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_10: cast zx s_3_8 -> i
        let s_3_10: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_11: call Z_read(s_3_9, s_3_10)
        let s_3_11: Bits = Z_read(state, tracer, s_3_9, s_3_10);
        // D s_3_12: write-var operand1 <= s_3_11
        fn_state.operand1 = s_3_11;
        // D s_3_13: read-var VLshadow#6465:i64
        let s_3_13: i64 = fn_state.VLshadow_6465;
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (i128::try_from(s_3_13).unwrap());
        // D s_3_15: cast reint s_3_14 -> i64
        let s_3_15: i64 = (s_3_14 as i64);
        // D s_3_16: read-var m:i64
        let s_3_16: i64 = fn_state.m;
        // D s_3_17: cast zx s_3_16 -> i
        let s_3_17: i128 = (i128::try_from(s_3_16).unwrap());
        // D s_3_18: cast zx s_3_15 -> i
        let s_3_18: i128 = (i128::try_from(s_3_15).unwrap());
        // D s_3_19: call Z_read(s_3_17, s_3_18)
        let s_3_19: Bits = Z_read(state, tracer, s_3_17, s_3_18);
        // D s_3_20: write-var operand2 <= s_3_19
        fn_state.operand2 = s_3_19;
        // D s_3_21: read-var vec:i
        let s_3_21: i128 = fn_state.vec;
        // D s_3_22: write-var vecshadow#6466 <= s_3_21
        fn_state.vecshadow_6466 = s_3_21;
        // C s_3_23: const #0s : i64
        let s_3_23: i64 = 0;
        // D s_3_24: write-var i <= s_3_23
        fn_state.i = s_3_23;
        // N s_3_25: jump b4
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
        // D s_5_2: read-var vecshadow#6466:i
        let s_5_2: i128 = fn_state.vecshadow_6466;
        // D s_5_3: add s_5_2 s_5_1
        let s_5_3: i128 = (s_5_2 + s_5_1);
        // D s_5_4: read-var VLshadow#6465:i64
        let s_5_4: i64 = fn_state.VLshadow_6465;
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
        // D s_5_16: write-var gs#280185 <= s_5_15
        fn_state.gs_280185 = s_5_15;
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
        // D s_6_1: read-var gs#280185:i64
        let s_6_1: i64 = fn_state.gs_280185;
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
        // D s_7_0: read-var e:i64
        let s_7_0: i64 = fn_state.e;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: read-var eltspersegment:i64
        let s_7_2: i64 = fn_state.eltspersegment;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: mod s_7_1 s_7_3
        let s_7_4: i128 = ((s_7_1) % (s_7_3));
        // D s_7_5: cast reint s_7_4 -> i64
        let s_7_5: i64 = (s_7_4 as i64);
        // D s_7_6: read-var e:i64
        let s_7_6: i64 = fn_state.e;
        // D s_7_7: cast zx s_7_6 -> i
        let s_7_7: i128 = (i128::try_from(s_7_6).unwrap());
        // D s_7_8: cast zx s_7_5 -> i
        let s_7_8: i128 = (i128::try_from(s_7_5).unwrap());
        // D s_7_9: sub s_7_7 s_7_8
        let s_7_9: i128 = ((s_7_7) - (s_7_8));
        // D s_7_10: cast reint s_7_9 -> i64
        let s_7_10: i64 = (s_7_9 as i64);
        // C s_7_11: const #4s : i
        let s_7_11: i128 = 4;
        // D s_7_12: cast zx s_7_10 -> i
        let s_7_12: i128 = (i128::try_from(s_7_10).unwrap());
        // D s_7_13: mul s_7_11 s_7_12
        let s_7_13: i128 = ((s_7_11) * (s_7_12));
        // D s_7_14: cast reint s_7_13 -> i64
        let s_7_14: i64 = (s_7_13 as i64);
        // D s_7_15: cast zx s_7_14 -> i
        let s_7_15: i128 = (i128::try_from(s_7_14).unwrap());
        // D s_7_16: read-var index:i64
        let s_7_16: i64 = fn_state.index;
        // D s_7_17: cast zx s_7_16 -> i
        let s_7_17: i128 = (i128::try_from(s_7_16).unwrap());
        // D s_7_18: add s_7_15 s_7_17
        let s_7_18: i128 = (s_7_15 + s_7_17);
        // D s_7_19: cast reint s_7_18 -> i64
        let s_7_19: i64 = (s_7_18 as i64);
        // C s_7_20: const #4s : i
        let s_7_20: i128 = 4;
        // D s_7_21: read-var e:i64
        let s_7_21: i64 = fn_state.e;
        // D s_7_22: cast zx s_7_21 -> i
        let s_7_22: i128 = (i128::try_from(s_7_21).unwrap());
        // D s_7_23: mul s_7_20 s_7_22
        let s_7_23: i128 = ((s_7_20) * (s_7_22));
        // D s_7_24: cast reint s_7_23 -> i64
        let s_7_24: i64 = (s_7_23 as i64);
        // D s_7_25: cast zx s_7_24 -> i
        let s_7_25: i128 = (i128::try_from(s_7_24).unwrap());
        // D s_7_26: read-var i:i64
        let s_7_26: i64 = fn_state.i;
        // D s_7_27: cast zx s_7_26 -> i
        let s_7_27: i128 = (i128::try_from(s_7_26).unwrap());
        // D s_7_28: add s_7_25 s_7_27
        let s_7_28: i128 = (s_7_25 + s_7_27);
        // D s_7_29: cast reint s_7_28 -> i64
        let s_7_29: i64 = (s_7_28 as i64);
        // C s_7_30: const #4s : i
        let s_7_30: i128 = 4;
        // D s_7_31: read-var esize:i64
        let s_7_31: i64 = fn_state.esize;
        // D s_7_32: cast zx s_7_31 -> i
        let s_7_32: i128 = (i128::try_from(s_7_31).unwrap());
        // D s_7_33: div s_7_32 s_7_30
        let s_7_33: i128 = ((s_7_32) / (s_7_30));
        // D s_7_34: cast reint s_7_33 -> i64
        let s_7_34: i64 = (s_7_33 as i64);
        // D s_7_35: cast zx s_7_34 -> i
        let s_7_35: i128 = (i128::try_from(s_7_34).unwrap());
        // D s_7_36: cast reint s_7_35 -> i64
        let s_7_36: i64 = (s_7_35 as i64);
        // D s_7_37: cast zx s_7_29 -> i
        let s_7_37: i128 = (i128::try_from(s_7_29).unwrap());
        // D s_7_38: cast zx s_7_36 -> i
        let s_7_38: i128 = (i128::try_from(s_7_36).unwrap());
        // D s_7_39: read-var operand1:bv
        let s_7_39: Bits = fn_state.operand1;
        // D s_7_40: call Elem_read(s_7_39, s_7_37, s_7_38)
        let s_7_40: Bits = Elem_read(state, tracer, s_7_39, s_7_37, s_7_38);
        // D s_7_41: cast reint s_7_40 -> u8
        let s_7_41: u8 = (s_7_40.value() as u8);
        // D s_7_42: cast zx s_7_41 -> bv
        let s_7_42: Bits = Bits::new(s_7_41 as u128, 8u16);
        // D s_7_43: cast sx s_7_42 -> i
        let s_7_43: i128 = {
            let sign_bit = s_7_42.length() - 1;
            let mut result = s_7_42.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_7_44: cast reint s_7_43 -> i64
        let s_7_44: i64 = (s_7_43 as i64);
        // C s_7_45: const #4s : i
        let s_7_45: i128 = 4;
        // D s_7_46: read-var esize:i64
        let s_7_46: i64 = fn_state.esize;
        // D s_7_47: cast zx s_7_46 -> i
        let s_7_47: i128 = (i128::try_from(s_7_46).unwrap());
        // D s_7_48: div s_7_47 s_7_45
        let s_7_48: i128 = ((s_7_47) / (s_7_45));
        // D s_7_49: cast reint s_7_48 -> i64
        let s_7_49: i64 = (s_7_48 as i64);
        // D s_7_50: cast zx s_7_49 -> i
        let s_7_50: i128 = (i128::try_from(s_7_49).unwrap());
        // D s_7_51: cast reint s_7_50 -> i64
        let s_7_51: i64 = (s_7_50 as i64);
        // D s_7_52: cast zx s_7_19 -> i
        let s_7_52: i128 = (i128::try_from(s_7_19).unwrap());
        // D s_7_53: cast zx s_7_51 -> i
        let s_7_53: i128 = (i128::try_from(s_7_51).unwrap());
        // D s_7_54: read-var operand2:bv
        let s_7_54: Bits = fn_state.operand2;
        // D s_7_55: call Elem_read(s_7_54, s_7_52, s_7_53)
        let s_7_55: Bits = Elem_read(state, tracer, s_7_54, s_7_52, s_7_53);
        // D s_7_56: cast reint s_7_55 -> u8
        let s_7_56: u8 = (s_7_55.value() as u8);
        // D s_7_57: cast zx s_7_56 -> bv
        let s_7_57: Bits = Bits::new(s_7_56 as u128, 8u16);
        // D s_7_58: cast zx s_7_57 -> i
        let s_7_58: i128 = (s_7_57.value() as i128);
        // D s_7_59: cast reint s_7_58 -> i64
        let s_7_59: i64 = (s_7_58 as i64);
        // D s_7_60: read-var esize:i64
        let s_7_60: i64 = fn_state.esize;
        // D s_7_61: cast zx s_7_60 -> i
        let s_7_61: i128 = (i128::try_from(s_7_60).unwrap());
        // D s_7_62: cast reint s_7_61 -> i64
        let s_7_62: i64 = (s_7_61 as i64);
        // D s_7_63: read-var esize:i64
        let s_7_63: i64 = fn_state.esize;
        // D s_7_64: cast zx s_7_63 -> i
        let s_7_64: i128 = (i128::try_from(s_7_63).unwrap());
        // D s_7_65: cast reint s_7_64 -> i64
        let s_7_65: i64 = (s_7_64 as i64);
        // D s_7_66: read-var e:i64
        let s_7_66: i64 = fn_state.e;
        // D s_7_67: cast zx s_7_66 -> i
        let s_7_67: i128 = (i128::try_from(s_7_66).unwrap());
        // D s_7_68: cast zx s_7_65 -> i
        let s_7_68: i128 = (i128::try_from(s_7_65).unwrap());
        // D s_7_69: read-var operand3:bv
        let s_7_69: Bits = fn_state.operand3;
        // D s_7_70: call Elem_read(s_7_69, s_7_67, s_7_68)
        let s_7_70: Bits = Elem_read(state, tracer, s_7_69, s_7_67, s_7_68);
        // D s_7_71: cast reint s_7_70 -> u32
        let s_7_71: u32 = (s_7_70.value() as u32);
        // D s_7_72: cast zx s_7_44 -> i
        let s_7_72: i128 = (i128::try_from(s_7_44).unwrap());
        // D s_7_73: cast zx s_7_59 -> i
        let s_7_73: i128 = (i128::try_from(s_7_59).unwrap());
        // D s_7_74: mul s_7_72 s_7_73
        let s_7_74: i128 = ((s_7_72) * (s_7_73));
        // D s_7_75: cast reint s_7_74 -> i64
        let s_7_75: i64 = (s_7_74 as i64);
        // C s_7_76: const #1s : i
        let s_7_76: i128 = 1;
        // D s_7_77: read-var esize:i64
        let s_7_77: i64 = fn_state.esize;
        // D s_7_78: cast zx s_7_77 -> i
        let s_7_78: i128 = (i128::try_from(s_7_77).unwrap());
        // D s_7_79: sub s_7_78 s_7_76
        let s_7_79: i128 = ((s_7_78) - (s_7_76));
        // D s_7_80: cast reint s_7_79 -> i64
        let s_7_80: i64 = (s_7_79 as i64);
        // C s_7_81: const #0s : i
        let s_7_81: i128 = 0;
        // D s_7_82: cast zx s_7_75 -> i
        let s_7_82: i128 = (i128::try_from(s_7_75).unwrap());
        // D s_7_83: cast zx s_7_80 -> i
        let s_7_83: i128 = (i128::try_from(s_7_80).unwrap());
        // D s_7_84: call integer_subrange(s_7_82, s_7_83, s_7_81)
        let s_7_84: Bits = integer_subrange(state, tracer, s_7_82, s_7_83, s_7_81);
        // D s_7_85: cast zx s_7_71 -> bv
        let s_7_85: Bits = Bits::new(s_7_71 as u128, 32u16);
        // D s_7_86: add s_7_85 s_7_84
        let s_7_86: Bits = (s_7_85 + s_7_84);
        // D s_7_87: cast reint s_7_86 -> u32
        let s_7_87: u32 = (s_7_86.value() as u32);
        // D s_7_88: read-var e:i64
        let s_7_88: i64 = fn_state.e;
        // D s_7_89: cast zx s_7_88 -> i
        let s_7_89: i128 = (i128::try_from(s_7_88).unwrap());
        // D s_7_90: cast zx s_7_62 -> i
        let s_7_90: i128 = (i128::try_from(s_7_62).unwrap());
        // D s_7_91: cast zx s_7_87 -> bv
        let s_7_91: Bits = Bits::new(s_7_87 as u128, 32u16);
        // D s_7_92: read-var result:bv
        let s_7_92: Bits = fn_state.result;
        // D s_7_93: call Elem_set(s_7_92, s_7_89, s_7_90, s_7_91)
        let s_7_93: Bits = Elem_set(state, tracer, s_7_92, s_7_89, s_7_90, s_7_91);
        // D s_7_94: write-var result <= s_7_93
        fn_state.result = s_7_93;
        // D s_7_95: read-var e:i64
        let s_7_95: i64 = fn_state.e;
        // C s_7_96: const #1s : i64
        let s_7_96: i64 = 1;
        // D s_7_97: add s_7_95 s_7_96
        let s_7_97: i64 = (s_7_95 + s_7_96);
        // D s_7_98: write-var e <= s_7_97
        fn_state.e = s_7_97;
        // N s_7_99: jump b6
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
        // D s_8_2: read-var vecshadow#6466:i
        let s_8_2: i128 = fn_state.vecshadow_6466;
        // D s_8_3: add s_8_2 s_8_1
        let s_8_3: i128 = (s_8_2 + s_8_1);
        // D s_8_4: read-var VLshadow#6465:i64
        let s_8_4: i64 = fn_state.VLshadow_6465;
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
