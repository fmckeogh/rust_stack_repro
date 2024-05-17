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
pub fn execute_SUVDOT_ZA_ZZi_S4xi<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    esize: i64,
    index: i64,
    m: i64,
    n: i64,
    offset: i64,
    v: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand2: Bits,
        r: i64,
        e: i64,
        sum: Bits,
        VLshadow_6251: i64,
        operand3: Bits,
        vstride: i64,
        vec: i128,
        VLshadow_6250: i64,
        s: i64,
        gs_275045: i64,
        elements: i64,
        result: Bits,
        i: i64,
        eltspersegment: i64,
        VL: i64,
        esize: i64,
        index: i64,
        m: i64,
        n: i64,
        offset: i64,
        v: i64,
    }
    let fn_state = FunctionState {
        VL,
        esize,
        index,
        m,
        n,
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
        // D s_0_3: write-var VLshadow#6250 <= s_0_2
        fn_state.VLshadow_6250 = s_0_2;
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
        // D s_1_0: read-var VLshadow#6250:i64
        let s_1_0: i64 = fn_state.VLshadow_6250;
        // D s_1_1: write-var VLshadow#6251 <= s_1_0
        fn_state.VLshadow_6251 = s_1_0;
        // D s_1_2: read-var VLshadow#6251:i64
        let s_1_2: i64 = fn_state.VLshadow_6251;
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
        // D s_1_10: read-var VLshadow#6251:i64
        let s_1_10: i64 = fn_state.VLshadow_6251;
        // D s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_12: div s_1_11 s_1_9
        let s_1_12: i128 = ((s_1_11) / (s_1_9));
        // D s_1_13: cast reint s_1_12 -> i64
        let s_1_13: i64 = (s_1_12 as i64);
        // C s_1_14: const #4s : i
        let s_1_14: i128 = 4;
        // D s_1_15: cast zx s_1_13 -> i
        let s_1_15: i128 = (i128::try_from(s_1_13).unwrap());
        // D s_1_16: div s_1_15 s_1_14
        let s_1_16: i128 = ((s_1_15) / (s_1_14));
        // D s_1_17: cast reint s_1_16 -> i64
        let s_1_17: i64 = (s_1_16 as i64);
        // D s_1_18: write-var vstride <= s_1_17
        fn_state.vstride = s_1_17;
        // C s_1_19: const #128s : i
        let s_1_19: i128 = 128;
        // D s_1_20: read-var esize:i64
        let s_1_20: i64 = fn_state.esize;
        // D s_1_21: cast zx s_1_20 -> i
        let s_1_21: i128 = (i128::try_from(s_1_20).unwrap());
        // D s_1_22: div s_1_19 s_1_21
        let s_1_22: i128 = ((s_1_19) / (s_1_21));
        // D s_1_23: cast reint s_1_22 -> i64
        let s_1_23: i64 = (s_1_22 as i64);
        // D s_1_24: write-var eltspersegment <= s_1_23
        fn_state.eltspersegment = s_1_23;
        // C s_1_25: const #32s : i64
        let s_1_25: i64 = 32;
        // D s_1_26: read-var v:i64
        let s_1_26: i64 = fn_state.v;
        // D s_1_27: cast zx s_1_26 -> i
        let s_1_27: i128 = (i128::try_from(s_1_26).unwrap());
        // D s_1_28: call X_read(s_1_27, s_1_25)
        let s_1_28: Bits = X_read(state, tracer, s_1_27, s_1_25);
        // D s_1_29: cast reint s_1_28 -> u32
        let s_1_29: u32 = (s_1_28.value() as u32);
        // D s_1_30: cast zx s_1_29 -> bv
        let s_1_30: Bits = Bits::new(s_1_29 as u128, 32u16);
        // D s_1_31: cast zx s_1_30 -> i
        let s_1_31: i128 = (s_1_30.value() as i128);
        // D s_1_32: cast reint s_1_31 -> i64
        let s_1_32: i64 = (s_1_31 as i64);
        // D s_1_33: cast zx s_1_32 -> i
        let s_1_33: i128 = (i128::try_from(s_1_32).unwrap());
        // D s_1_34: read-var offset:i64
        let s_1_34: i64 = fn_state.offset;
        // D s_1_35: cast zx s_1_34 -> i
        let s_1_35: i128 = (i128::try_from(s_1_34).unwrap());
        // D s_1_36: add s_1_33 s_1_35
        let s_1_36: i128 = (s_1_33 + s_1_35);
        // D s_1_37: cast reint s_1_36 -> i64
        let s_1_37: i64 = (s_1_36 as i64);
        // D s_1_38: cast zx s_1_37 -> i
        let s_1_38: i128 = (i128::try_from(s_1_37).unwrap());
        // D s_1_39: read-var vstride:i64
        let s_1_39: i64 = fn_state.vstride;
        // D s_1_40: cast zx s_1_39 -> i
        let s_1_40: i128 = (i128::try_from(s_1_39).unwrap());
        // D s_1_41: mod s_1_38 s_1_40
        let s_1_41: i128 = ((s_1_38) % (s_1_40));
        // D s_1_42: write-var vec <= s_1_41
        fn_state.vec = s_1_41;
        // D s_1_43: read-var VLshadow#6251:i64
        let s_1_43: i64 = fn_state.VLshadow_6251;
        // D s_1_44: cast zx s_1_43 -> i
        let s_1_44: i128 = (i128::try_from(s_1_43).unwrap());
        // D s_1_45: cast reint s_1_44 -> i64
        let s_1_45: i64 = (s_1_44 as i64);
        // D s_1_46: read-var m:i64
        let s_1_46: i64 = fn_state.m;
        // D s_1_47: cast zx s_1_46 -> i
        let s_1_47: i128 = (i128::try_from(s_1_46).unwrap());
        // D s_1_48: cast zx s_1_45 -> i
        let s_1_48: i128 = (i128::try_from(s_1_45).unwrap());
        // D s_1_49: call Z_read(s_1_47, s_1_48)
        let s_1_49: Bits = Z_read(state, tracer, s_1_47, s_1_48);
        // D s_1_50: write-var operand2 <= s_1_49
        fn_state.operand2 = s_1_49;
        // C s_1_51: const #0s : i64
        let s_1_51: i64 = 0;
        // D s_1_52: write-var r <= s_1_51
        fn_state.r = s_1_51;
        // N s_1_53: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var r:i64
        let s_2_0: i64 = fn_state.r;
        // C s_2_1: const #3s : i64
        let s_2_1: i64 = 3;
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
        // D s_3_0: read-var vec:i
        let s_3_0: i128 = fn_state.vec;
        // D s_3_1: read-var VLshadow#6251:i64
        let s_3_1: i64 = fn_state.VLshadow_6251;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: call ZAvector_read(s_3_0, s_3_4)
        let s_3_5: Bits = ZAvector_read(state, tracer, s_3_0, s_3_4);
        // D s_3_6: write-var operand3 <= s_3_5
        fn_state.operand3 = s_3_5;
        // C s_3_7: const #0s : i64
        let s_3_7: i64 = 0;
        // C s_3_8: const #1s : i
        let s_3_8: i128 = 1;
        // D s_3_9: read-var elements:i64
        let s_3_9: i64 = fn_state.elements;
        // D s_3_10: cast zx s_3_9 -> i
        let s_3_10: i128 = (i128::try_from(s_3_9).unwrap());
        // D s_3_11: sub s_3_10 s_3_8
        let s_3_11: i128 = ((s_3_10) - (s_3_8));
        // D s_3_12: cast reint s_3_11 -> i64
        let s_3_12: i64 = (s_3_11 as i64);
        // D s_3_13: write-var gs#275045 <= s_3_12
        fn_state.gs_275045 = s_3_12;
        // D s_3_14: write-var e <= s_3_7
        fn_state.e = s_3_7;
        // N s_3_15: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#275045:i64
        let s_4_1: i64 = fn_state.gs_275045;
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
        // D s_5_0: read-var e:i64
        let s_5_0: i64 = fn_state.e;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: read-var eltspersegment:i64
        let s_5_2: i64 = fn_state.eltspersegment;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: mod s_5_1 s_5_3
        let s_5_4: i128 = ((s_5_1) % (s_5_3));
        // D s_5_5: cast reint s_5_4 -> i64
        let s_5_5: i64 = (s_5_4 as i64);
        // D s_5_6: read-var e:i64
        let s_5_6: i64 = fn_state.e;
        // D s_5_7: cast zx s_5_6 -> i
        let s_5_7: i128 = (i128::try_from(s_5_6).unwrap());
        // D s_5_8: cast zx s_5_5 -> i
        let s_5_8: i128 = (i128::try_from(s_5_5).unwrap());
        // D s_5_9: sub s_5_7 s_5_8
        let s_5_9: i128 = ((s_5_7) - (s_5_8));
        // D s_5_10: cast reint s_5_9 -> i64
        let s_5_10: i64 = (s_5_9 as i64);
        // D s_5_11: cast zx s_5_10 -> i
        let s_5_11: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_12: read-var index:i64
        let s_5_12: i64 = fn_state.index;
        // D s_5_13: cast zx s_5_12 -> i
        let s_5_13: i128 = (i128::try_from(s_5_12).unwrap());
        // D s_5_14: add s_5_11 s_5_13
        let s_5_14: i128 = (s_5_11 + s_5_13);
        // D s_5_15: cast reint s_5_14 -> i64
        let s_5_15: i64 = (s_5_14 as i64);
        // D s_5_16: write-var s <= s_5_15
        fn_state.s = s_5_15;
        // D s_5_17: read-var esize:i64
        let s_5_17: i64 = fn_state.esize;
        // D s_5_18: cast zx s_5_17 -> i
        let s_5_18: i128 = (i128::try_from(s_5_17).unwrap());
        // D s_5_19: cast reint s_5_18 -> i64
        let s_5_19: i64 = (s_5_18 as i64);
        // D s_5_20: read-var e:i64
        let s_5_20: i64 = fn_state.e;
        // D s_5_21: cast zx s_5_20 -> i
        let s_5_21: i128 = (i128::try_from(s_5_20).unwrap());
        // D s_5_22: cast zx s_5_19 -> i
        let s_5_22: i128 = (i128::try_from(s_5_19).unwrap());
        // D s_5_23: read-var operand3:bv
        let s_5_23: Bits = fn_state.operand3;
        // D s_5_24: call Elem_read(s_5_23, s_5_21, s_5_22)
        let s_5_24: Bits = Elem_read(state, tracer, s_5_23, s_5_21, s_5_22);
        // D s_5_25: write-var sum <= s_5_24
        fn_state.sum = s_5_24;
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
        // D s_7_0: read-var n:i64
        let s_7_0: i64 = fn_state.n;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: read-var i:i64
        let s_7_2: i64 = fn_state.i;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: add s_7_1 s_7_3
        let s_7_4: i128 = (s_7_1 + s_7_3);
        // D s_7_5: cast reint s_7_4 -> i64
        let s_7_5: i64 = (s_7_4 as i64);
        // D s_7_6: read-var VLshadow#6251:i64
        let s_7_6: i64 = fn_state.VLshadow_6251;
        // D s_7_7: cast zx s_7_6 -> i
        let s_7_7: i128 = (i128::try_from(s_7_6).unwrap());
        // D s_7_8: cast reint s_7_7 -> i64
        let s_7_8: i64 = (s_7_7 as i64);
        // D s_7_9: cast zx s_7_5 -> i
        let s_7_9: i128 = (i128::try_from(s_7_5).unwrap());
        // D s_7_10: cast zx s_7_8 -> i
        let s_7_10: i128 = (i128::try_from(s_7_8).unwrap());
        // D s_7_11: call Z_read(s_7_9, s_7_10)
        let s_7_11: Bits = Z_read(state, tracer, s_7_9, s_7_10);
        // C s_7_12: const #4s : i
        let s_7_12: i128 = 4;
        // D s_7_13: read-var e:i64
        let s_7_13: i64 = fn_state.e;
        // D s_7_14: cast zx s_7_13 -> i
        let s_7_14: i128 = (i128::try_from(s_7_13).unwrap());
        // D s_7_15: mul s_7_12 s_7_14
        let s_7_15: i128 = ((s_7_12) * (s_7_14));
        // D s_7_16: cast reint s_7_15 -> i64
        let s_7_16: i64 = (s_7_15 as i64);
        // D s_7_17: cast zx s_7_16 -> i
        let s_7_17: i128 = (i128::try_from(s_7_16).unwrap());
        // D s_7_18: read-var r:i64
        let s_7_18: i64 = fn_state.r;
        // D s_7_19: cast zx s_7_18 -> i
        let s_7_19: i128 = (i128::try_from(s_7_18).unwrap());
        // D s_7_20: add s_7_17 s_7_19
        let s_7_20: i128 = (s_7_17 + s_7_19);
        // D s_7_21: cast reint s_7_20 -> i64
        let s_7_21: i64 = (s_7_20 as i64);
        // C s_7_22: const #4s : i
        let s_7_22: i128 = 4;
        // D s_7_23: read-var esize:i64
        let s_7_23: i64 = fn_state.esize;
        // D s_7_24: cast zx s_7_23 -> i
        let s_7_24: i128 = (i128::try_from(s_7_23).unwrap());
        // D s_7_25: div s_7_24 s_7_22
        let s_7_25: i128 = ((s_7_24) / (s_7_22));
        // D s_7_26: cast reint s_7_25 -> i64
        let s_7_26: i64 = (s_7_25 as i64);
        // D s_7_27: cast zx s_7_26 -> i
        let s_7_27: i128 = (i128::try_from(s_7_26).unwrap());
        // D s_7_28: cast reint s_7_27 -> i64
        let s_7_28: i64 = (s_7_27 as i64);
        // D s_7_29: cast zx s_7_21 -> i
        let s_7_29: i128 = (i128::try_from(s_7_21).unwrap());
        // D s_7_30: cast zx s_7_28 -> i
        let s_7_30: i128 = (i128::try_from(s_7_28).unwrap());
        // D s_7_31: call Elem_read(s_7_11, s_7_29, s_7_30)
        let s_7_31: Bits = Elem_read(state, tracer, s_7_11, s_7_29, s_7_30);
        // D s_7_32: cast reint s_7_31 -> u8
        let s_7_32: u8 = (s_7_31.value() as u8);
        // D s_7_33: cast zx s_7_32 -> bv
        let s_7_33: Bits = Bits::new(s_7_32 as u128, 8u16);
        // D s_7_34: cast sx s_7_33 -> i
        let s_7_34: i128 = {
            let sign_bit = s_7_33.length() - 1;
            let mut result = s_7_33.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_7_35: cast reint s_7_34 -> i64
        let s_7_35: i64 = (s_7_34 as i64);
        // C s_7_36: const #4s : i
        let s_7_36: i128 = 4;
        // D s_7_37: read-var s:i64
        let s_7_37: i64 = fn_state.s;
        // D s_7_38: cast zx s_7_37 -> i
        let s_7_38: i128 = (i128::try_from(s_7_37).unwrap());
        // D s_7_39: mul s_7_36 s_7_38
        let s_7_39: i128 = ((s_7_36) * (s_7_38));
        // D s_7_40: cast reint s_7_39 -> i64
        let s_7_40: i64 = (s_7_39 as i64);
        // D s_7_41: cast zx s_7_40 -> i
        let s_7_41: i128 = (i128::try_from(s_7_40).unwrap());
        // D s_7_42: read-var i:i64
        let s_7_42: i64 = fn_state.i;
        // D s_7_43: cast zx s_7_42 -> i
        let s_7_43: i128 = (i128::try_from(s_7_42).unwrap());
        // D s_7_44: add s_7_41 s_7_43
        let s_7_44: i128 = (s_7_41 + s_7_43);
        // D s_7_45: cast reint s_7_44 -> i64
        let s_7_45: i64 = (s_7_44 as i64);
        // C s_7_46: const #4s : i
        let s_7_46: i128 = 4;
        // D s_7_47: read-var esize:i64
        let s_7_47: i64 = fn_state.esize;
        // D s_7_48: cast zx s_7_47 -> i
        let s_7_48: i128 = (i128::try_from(s_7_47).unwrap());
        // D s_7_49: div s_7_48 s_7_46
        let s_7_49: i128 = ((s_7_48) / (s_7_46));
        // D s_7_50: cast reint s_7_49 -> i64
        let s_7_50: i64 = (s_7_49 as i64);
        // D s_7_51: cast zx s_7_50 -> i
        let s_7_51: i128 = (i128::try_from(s_7_50).unwrap());
        // D s_7_52: cast reint s_7_51 -> i64
        let s_7_52: i64 = (s_7_51 as i64);
        // D s_7_53: cast zx s_7_45 -> i
        let s_7_53: i128 = (i128::try_from(s_7_45).unwrap());
        // D s_7_54: cast zx s_7_52 -> i
        let s_7_54: i128 = (i128::try_from(s_7_52).unwrap());
        // D s_7_55: read-var operand2:bv
        let s_7_55: Bits = fn_state.operand2;
        // D s_7_56: call Elem_read(s_7_55, s_7_53, s_7_54)
        let s_7_56: Bits = Elem_read(state, tracer, s_7_55, s_7_53, s_7_54);
        // D s_7_57: cast reint s_7_56 -> u8
        let s_7_57: u8 = (s_7_56.value() as u8);
        // D s_7_58: cast zx s_7_57 -> bv
        let s_7_58: Bits = Bits::new(s_7_57 as u128, 8u16);
        // D s_7_59: cast zx s_7_58 -> i
        let s_7_59: i128 = (s_7_58.value() as i128);
        // D s_7_60: cast reint s_7_59 -> i64
        let s_7_60: i64 = (s_7_59 as i64);
        // D s_7_61: cast zx s_7_35 -> i
        let s_7_61: i128 = (i128::try_from(s_7_35).unwrap());
        // D s_7_62: cast zx s_7_60 -> i
        let s_7_62: i128 = (i128::try_from(s_7_60).unwrap());
        // D s_7_63: mul s_7_61 s_7_62
        let s_7_63: i128 = ((s_7_61) * (s_7_62));
        // D s_7_64: cast reint s_7_63 -> i64
        let s_7_64: i64 = (s_7_63 as i64);
        // D s_7_65: cast zx s_7_64 -> i
        let s_7_65: i128 = (i128::try_from(s_7_64).unwrap());
        // D s_7_66: read-var sum:bv
        let s_7_66: Bits = fn_state.sum;
        // D s_7_67: cast cvt s_7_65 -> bv
        let s_7_67: Bits = Bits::new(s_7_65 as u128, 128);
        // D s_7_68: add s_7_66 s_7_67
        let s_7_68: Bits = (s_7_66 + s_7_67);
        // D s_7_69: write-var sum <= s_7_68
        fn_state.sum = s_7_68;
        // D s_7_70: read-var i:i64
        let s_7_70: i64 = fn_state.i;
        // C s_7_71: const #1s : i64
        let s_7_71: i64 = 1;
        // D s_7_72: add s_7_70 s_7_71
        let s_7_72: i64 = (s_7_70 + s_7_71);
        // D s_7_73: write-var i <= s_7_72
        fn_state.i = s_7_72;
        // N s_7_74: jump b6
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
        // D s_9_1: read-var VLshadow#6251:i64
        let s_9_1: i64 = fn_state.VLshadow_6251;
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
