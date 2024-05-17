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
use ZAvector_set::*;
use X_read::*;
use Elem_read::*;
use CheckStreamingSVEAndZAEnabled::*;
use Z_read::*;
use Elem_set::*;
use ZAvector_read::*;
use common::*;
pub fn execute_SDOT_ZA_ZZi_D2xi<T: Tracer>(
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
        sum: Bits,
        gs_274147: i64,
        operand3: Bits,
        gs_274139: i64,
        vec: i128,
        vstride: i64,
        s: i64,
        VLshadow_6211: i64,
        elements: i64,
        VLshadow_6210: i64,
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
        // D s_0_3: write-var VLshadow#6210 <= s_0_2
        fn_state.VLshadow_6210 = s_0_2;
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
        // D s_1_0: read-var VLshadow#6210:i64
        let s_1_0: i64 = fn_state.VLshadow_6210;
        // D s_1_1: write-var VLshadow#6211 <= s_1_0
        fn_state.VLshadow_6211 = s_1_0;
        // D s_1_2: read-var VLshadow#6211:i64
        let s_1_2: i64 = fn_state.VLshadow_6211;
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
        // D s_1_10: read-var VLshadow#6211:i64
        let s_1_10: i64 = fn_state.VLshadow_6211;
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
        // D s_1_50: write-var gs#274139 <= s_1_49
        fn_state.gs_274139 = s_1_49;
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
        // D s_2_1: read-var gs#274139:i64
        let s_2_1: i64 = fn_state.gs_274139;
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
        // D s_3_6: read-var VLshadow#6211:i64
        let s_3_6: i64 = fn_state.VLshadow_6211;
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
        // D s_3_13: read-var VLshadow#6211:i64
        let s_3_13: i64 = fn_state.VLshadow_6211;
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
        // D s_3_22: read-var VLshadow#6211:i64
        let s_3_22: i64 = fn_state.VLshadow_6211;
        // D s_3_23: cast zx s_3_22 -> i
        let s_3_23: i128 = (i128::try_from(s_3_22).unwrap());
        // D s_3_24: cast reint s_3_23 -> i64
        let s_3_24: i64 = (s_3_23 as i64);
        // D s_3_25: cast zx s_3_24 -> i
        let s_3_25: i128 = (i128::try_from(s_3_24).unwrap());
        // D s_3_26: call ZAvector_read(s_3_21, s_3_25)
        let s_3_26: Bits = ZAvector_read(state, tracer, s_3_21, s_3_25);
        // D s_3_27: write-var operand3 <= s_3_26
        fn_state.operand3 = s_3_26;
        // C s_3_28: const #0s : i64
        let s_3_28: i64 = 0;
        // C s_3_29: const #1s : i
        let s_3_29: i128 = 1;
        // D s_3_30: read-var elements:i64
        let s_3_30: i64 = fn_state.elements;
        // D s_3_31: cast zx s_3_30 -> i
        let s_3_31: i128 = (i128::try_from(s_3_30).unwrap());
        // D s_3_32: sub s_3_31 s_3_29
        let s_3_32: i128 = ((s_3_31) - (s_3_29));
        // D s_3_33: cast reint s_3_32 -> i64
        let s_3_33: i64 = (s_3_32 as i64);
        // D s_3_34: write-var gs#274147 <= s_3_33
        fn_state.gs_274147 = s_3_33;
        // D s_3_35: write-var e <= s_3_28
        fn_state.e = s_3_28;
        // N s_3_36: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#274147:i64
        let s_4_1: i64 = fn_state.gs_274147;
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
        // D s_5_0: read-var esize:i64
        let s_5_0: i64 = fn_state.esize;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: cast reint s_5_1 -> i64
        let s_5_2: i64 = (s_5_1 as i64);
        // D s_5_3: read-var e:i64
        let s_5_3: i64 = fn_state.e;
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_5: cast zx s_5_2 -> i
        let s_5_5: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_6: read-var operand3:bv
        let s_5_6: Bits = fn_state.operand3;
        // D s_5_7: call Elem_read(s_5_6, s_5_4, s_5_5)
        let s_5_7: Bits = Elem_read(state, tracer, s_5_6, s_5_4, s_5_5);
        // D s_5_8: write-var sum <= s_5_7
        fn_state.sum = s_5_7;
        // D s_5_9: read-var e:i64
        let s_5_9: i64 = fn_state.e;
        // D s_5_10: cast zx s_5_9 -> i
        let s_5_10: i128 = (i128::try_from(s_5_9).unwrap());
        // D s_5_11: read-var eltspersegment:i64
        let s_5_11: i64 = fn_state.eltspersegment;
        // D s_5_12: cast zx s_5_11 -> i
        let s_5_12: i128 = (i128::try_from(s_5_11).unwrap());
        // D s_5_13: mod s_5_10 s_5_12
        let s_5_13: i128 = ((s_5_10) % (s_5_12));
        // D s_5_14: cast reint s_5_13 -> i64
        let s_5_14: i64 = (s_5_13 as i64);
        // D s_5_15: read-var e:i64
        let s_5_15: i64 = fn_state.e;
        // D s_5_16: cast zx s_5_15 -> i
        let s_5_16: i128 = (i128::try_from(s_5_15).unwrap());
        // D s_5_17: cast zx s_5_14 -> i
        let s_5_17: i128 = (i128::try_from(s_5_14).unwrap());
        // D s_5_18: sub s_5_16 s_5_17
        let s_5_18: i128 = ((s_5_16) - (s_5_17));
        // D s_5_19: cast reint s_5_18 -> i64
        let s_5_19: i64 = (s_5_18 as i64);
        // D s_5_20: cast zx s_5_19 -> i
        let s_5_20: i128 = (i128::try_from(s_5_19).unwrap());
        // D s_5_21: read-var index:i64
        let s_5_21: i64 = fn_state.index;
        // D s_5_22: cast zx s_5_21 -> i
        let s_5_22: i128 = (i128::try_from(s_5_21).unwrap());
        // D s_5_23: add s_5_20 s_5_22
        let s_5_23: i128 = (s_5_20 + s_5_22);
        // D s_5_24: cast reint s_5_23 -> i64
        let s_5_24: i64 = (s_5_23 as i64);
        // D s_5_25: write-var s <= s_5_24
        fn_state.s = s_5_24;
        // C s_5_26: const #0s : i64
        let s_5_26: i64 = 0;
        // D s_5_27: write-var i <= s_5_26
        fn_state.i = s_5_26;
        // N s_5_28: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var i:i64
        let s_6_0: i64 = fn_state.i;
        // C s_6_1: const #3s : i64
        let s_6_1: i64 = 3;
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
        // D s_7_11: read-var esize:i64
        let s_7_11: i64 = fn_state.esize;
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
        // D s_7_21: cast reint s_7_20 -> u16
        let s_7_21: u16 = (s_7_20.value() as u16);
        // D s_7_22: cast zx s_7_21 -> bv
        let s_7_22: Bits = Bits::new(s_7_21 as u128, 16u16);
        // D s_7_23: cast sx s_7_22 -> i
        let s_7_23: i128 = {
            let sign_bit = s_7_22.length() - 1;
            let mut result = s_7_22.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_7_24: cast reint s_7_23 -> i64
        let s_7_24: i64 = (s_7_23 as i64);
        // C s_7_25: const #4s : i
        let s_7_25: i128 = 4;
        // D s_7_26: read-var s:i64
        let s_7_26: i64 = fn_state.s;
        // D s_7_27: cast zx s_7_26 -> i
        let s_7_27: i128 = (i128::try_from(s_7_26).unwrap());
        // D s_7_28: mul s_7_25 s_7_27
        let s_7_28: i128 = ((s_7_25) * (s_7_27));
        // D s_7_29: cast reint s_7_28 -> i64
        let s_7_29: i64 = (s_7_28 as i64);
        // D s_7_30: cast zx s_7_29 -> i
        let s_7_30: i128 = (i128::try_from(s_7_29).unwrap());
        // D s_7_31: read-var i:i64
        let s_7_31: i64 = fn_state.i;
        // D s_7_32: cast zx s_7_31 -> i
        let s_7_32: i128 = (i128::try_from(s_7_31).unwrap());
        // D s_7_33: add s_7_30 s_7_32
        let s_7_33: i128 = (s_7_30 + s_7_32);
        // D s_7_34: cast reint s_7_33 -> i64
        let s_7_34: i64 = (s_7_33 as i64);
        // C s_7_35: const #4s : i
        let s_7_35: i128 = 4;
        // D s_7_36: read-var esize:i64
        let s_7_36: i64 = fn_state.esize;
        // D s_7_37: cast zx s_7_36 -> i
        let s_7_37: i128 = (i128::try_from(s_7_36).unwrap());
        // D s_7_38: div s_7_37 s_7_35
        let s_7_38: i128 = ((s_7_37) / (s_7_35));
        // D s_7_39: cast reint s_7_38 -> i64
        let s_7_39: i64 = (s_7_38 as i64);
        // D s_7_40: cast zx s_7_39 -> i
        let s_7_40: i128 = (i128::try_from(s_7_39).unwrap());
        // D s_7_41: cast reint s_7_40 -> i64
        let s_7_41: i64 = (s_7_40 as i64);
        // D s_7_42: cast zx s_7_34 -> i
        let s_7_42: i128 = (i128::try_from(s_7_34).unwrap());
        // D s_7_43: cast zx s_7_41 -> i
        let s_7_43: i128 = (i128::try_from(s_7_41).unwrap());
        // D s_7_44: read-var operand2:bv
        let s_7_44: Bits = fn_state.operand2;
        // D s_7_45: call Elem_read(s_7_44, s_7_42, s_7_43)
        let s_7_45: Bits = Elem_read(state, tracer, s_7_44, s_7_42, s_7_43);
        // D s_7_46: cast reint s_7_45 -> u16
        let s_7_46: u16 = (s_7_45.value() as u16);
        // D s_7_47: cast zx s_7_46 -> bv
        let s_7_47: Bits = Bits::new(s_7_46 as u128, 16u16);
        // D s_7_48: cast sx s_7_47 -> i
        let s_7_48: i128 = {
            let sign_bit = s_7_47.length() - 1;
            let mut result = s_7_47.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_7_49: cast reint s_7_48 -> i64
        let s_7_49: i64 = (s_7_48 as i64);
        // D s_7_50: cast zx s_7_24 -> i
        let s_7_50: i128 = (i128::try_from(s_7_24).unwrap());
        // D s_7_51: cast zx s_7_49 -> i
        let s_7_51: i128 = (i128::try_from(s_7_49).unwrap());
        // D s_7_52: mul s_7_50 s_7_51
        let s_7_52: i128 = ((s_7_50) * (s_7_51));
        // D s_7_53: cast reint s_7_52 -> i64
        let s_7_53: i64 = (s_7_52 as i64);
        // D s_7_54: cast zx s_7_53 -> i
        let s_7_54: i128 = (i128::try_from(s_7_53).unwrap());
        // D s_7_55: read-var sum:bv
        let s_7_55: Bits = fn_state.sum;
        // D s_7_56: cast cvt s_7_54 -> bv
        let s_7_56: Bits = Bits::new(s_7_54 as u128, 128);
        // D s_7_57: add s_7_55 s_7_56
        let s_7_57: Bits = (s_7_55 + s_7_56);
        // D s_7_58: write-var sum <= s_7_57
        fn_state.sum = s_7_57;
        // D s_7_59: read-var i:i64
        let s_7_59: i64 = fn_state.i;
        // C s_7_60: const #1s : i64
        let s_7_60: i64 = 1;
        // D s_7_61: add s_7_59 s_7_60
        let s_7_61: i64 = (s_7_59 + s_7_60);
        // D s_7_62: write-var i <= s_7_61
        fn_state.i = s_7_61;
        // N s_7_63: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var esize:i64
        let s_8_0: i64 = fn_state.esize;
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
        // D s_8_6: read-var result:bv
        let s_8_6: Bits = fn_state.result;
        // D s_8_7: read-var sum:bv
        let s_8_7: Bits = fn_state.sum;
        // D s_8_8: call Elem_set(s_8_6, s_8_4, s_8_5, s_8_7)
        let s_8_8: Bits = Elem_set(state, tracer, s_8_6, s_8_4, s_8_5, s_8_7);
        // D s_8_9: write-var result <= s_8_8
        fn_state.result = s_8_8;
        // D s_8_10: read-var e:i64
        let s_8_10: i64 = fn_state.e;
        // C s_8_11: const #1s : i64
        let s_8_11: i64 = 1;
        // D s_8_12: add s_8_10 s_8_11
        let s_8_12: i64 = (s_8_10 + s_8_11);
        // D s_8_13: write-var e <= s_8_12
        fn_state.e = s_8_12;
        // N s_8_14: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var vec:i
        let s_9_0: i128 = fn_state.vec;
        // D s_9_1: read-var VLshadow#6211:i64
        let s_9_1: i64 = fn_state.VLshadow_6211;
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (i128::try_from(s_9_1).unwrap());
        // D s_9_3: cast reint s_9_2 -> i64
        let s_9_3: i64 = (s_9_2 as i64);
        // D s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_5: read-var result:bv
        let s_9_5: Bits = fn_state.result;
        // D s_9_6: call ZAvector_set(s_9_0, s_9_4, s_9_5)
        let s_9_6: () = ZAvector_set(state, tracer, s_9_0, s_9_4, s_9_5);
        // D s_9_7: read-var vstride:i64
        let s_9_7: i64 = fn_state.vstride;
        // D s_9_8: cast zx s_9_7 -> i
        let s_9_8: i128 = (i128::try_from(s_9_7).unwrap());
        // D s_9_9: read-var vec:i
        let s_9_9: i128 = fn_state.vec;
        // D s_9_10: add s_9_9 s_9_8
        let s_9_10: i128 = (s_9_9 + s_9_8);
        // D s_9_11: write-var vec <= s_9_10
        fn_state.vec = s_9_10;
        // D s_9_12: read-var r:i64
        let s_9_12: i64 = fn_state.r;
        // C s_9_13: const #1s : i64
        let s_9_13: i64 = 1;
        // D s_9_14: add s_9_12 s_9_13
        let s_9_14: i64 = (s_9_12 + s_9_13);
        // D s_9_15: write-var r <= s_9_14
        fn_state.r = s_9_14;
        // N s_9_16: jump b2
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
