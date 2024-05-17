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
use X_read::*;
use Elem_read::*;
use CheckStreamingSVEAndZAEnabled::*;
use Z_read::*;
use Elem_set::*;
use ZAvector_set::*;
use common::*;
pub fn execute_ADD_ZA_ZZW_2x2<T: Tracer>(
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
        vstride: i64,
        vec: i128,
        gs_271972: i64,
        gs_271978: i64,
        elements: i64,
        VLshadow_6095: i64,
        result: Bits,
        operand1: Bits,
        esizeshadow_6094: i64,
        VLshadow_6096: i64,
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
        // D s_0_3: write-var esizeshadow#6094 <= s_0_2
        fn_state.esizeshadow_6094 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#6095 <= s_0_6
        fn_state.VLshadow_6095 = s_0_6;
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
        // D s_1_0: read-var VLshadow#6095:i64
        let s_1_0: i64 = fn_state.VLshadow_6095;
        // D s_1_1: write-var VLshadow#6096 <= s_1_0
        fn_state.VLshadow_6096 = s_1_0;
        // D s_1_2: read-var VLshadow#6096:i64
        let s_1_2: i64 = fn_state.VLshadow_6096;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: read-var esizeshadow#6094:i64
        let s_1_4: i64 = fn_state.esizeshadow_6094;
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
        // D s_1_10: read-var VLshadow#6096:i64
        let s_1_10: i64 = fn_state.VLshadow_6096;
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
        // C s_1_38: const #0s : i64
        let s_1_38: i64 = 0;
        // C s_1_39: const #1s : i
        let s_1_39: i128 = 1;
        // D s_1_40: read-var nreg:i64
        let s_1_40: i64 = fn_state.nreg;
        // D s_1_41: cast zx s_1_40 -> i
        let s_1_41: i128 = (i128::try_from(s_1_40).unwrap());
        // D s_1_42: sub s_1_41 s_1_39
        let s_1_42: i128 = ((s_1_41) - (s_1_39));
        // D s_1_43: cast reint s_1_42 -> i64
        let s_1_43: i64 = (s_1_42 as i64);
        // D s_1_44: write-var gs#271972 <= s_1_43
        fn_state.gs_271972 = s_1_43;
        // D s_1_45: write-var r <= s_1_38
        fn_state.r = s_1_38;
        // N s_1_46: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var r:i64
        let s_2_0: i64 = fn_state.r;
        // D s_2_1: read-var gs#271972:i64
        let s_2_1: i64 = fn_state.gs_271972;
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
        // D s_3_6: read-var VLshadow#6096:i64
        let s_3_6: i64 = fn_state.VLshadow_6096;
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
        // D s_3_13: read-var m:i64
        let s_3_13: i64 = fn_state.m;
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (i128::try_from(s_3_13).unwrap());
        // D s_3_15: read-var r:i64
        let s_3_15: i64 = fn_state.r;
        // D s_3_16: cast zx s_3_15 -> i
        let s_3_16: i128 = (i128::try_from(s_3_15).unwrap());
        // D s_3_17: add s_3_14 s_3_16
        let s_3_17: i128 = (s_3_14 + s_3_16);
        // D s_3_18: cast reint s_3_17 -> i64
        let s_3_18: i64 = (s_3_17 as i64);
        // D s_3_19: read-var VLshadow#6096:i64
        let s_3_19: i64 = fn_state.VLshadow_6096;
        // D s_3_20: cast zx s_3_19 -> i
        let s_3_20: i128 = (i128::try_from(s_3_19).unwrap());
        // D s_3_21: cast reint s_3_20 -> i64
        let s_3_21: i64 = (s_3_20 as i64);
        // D s_3_22: cast zx s_3_18 -> i
        let s_3_22: i128 = (i128::try_from(s_3_18).unwrap());
        // D s_3_23: cast zx s_3_21 -> i
        let s_3_23: i128 = (i128::try_from(s_3_21).unwrap());
        // D s_3_24: call Z_read(s_3_22, s_3_23)
        let s_3_24: Bits = Z_read(state, tracer, s_3_22, s_3_23);
        // D s_3_25: write-var operand2 <= s_3_24
        fn_state.operand2 = s_3_24;
        // C s_3_26: const #0s : i64
        let s_3_26: i64 = 0;
        // C s_3_27: const #1s : i
        let s_3_27: i128 = 1;
        // D s_3_28: read-var elements:i64
        let s_3_28: i64 = fn_state.elements;
        // D s_3_29: cast zx s_3_28 -> i
        let s_3_29: i128 = (i128::try_from(s_3_28).unwrap());
        // D s_3_30: sub s_3_29 s_3_27
        let s_3_30: i128 = ((s_3_29) - (s_3_27));
        // D s_3_31: cast reint s_3_30 -> i64
        let s_3_31: i64 = (s_3_30 as i64);
        // D s_3_32: write-var gs#271978 <= s_3_31
        fn_state.gs_271978 = s_3_31;
        // D s_3_33: write-var e <= s_3_26
        fn_state.e = s_3_26;
        // N s_3_34: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#271978:i64
        let s_4_1: i64 = fn_state.gs_271978;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b6 b5
        if s_4_2 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var esizeshadow#6094:i64
        let s_5_0: i64 = fn_state.esizeshadow_6094;
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
        // D s_5_6: read-var operand1:bv
        let s_5_6: Bits = fn_state.operand1;
        // D s_5_7: call Elem_read(s_5_6, s_5_4, s_5_5)
        let s_5_7: Bits = Elem_read(state, tracer, s_5_6, s_5_4, s_5_5);
        // D s_5_8: read-var esizeshadow#6094:i64
        let s_5_8: i64 = fn_state.esizeshadow_6094;
        // D s_5_9: cast zx s_5_8 -> i
        let s_5_9: i128 = (i128::try_from(s_5_8).unwrap());
        // D s_5_10: cast reint s_5_9 -> i64
        let s_5_10: i64 = (s_5_9 as i64);
        // D s_5_11: read-var e:i64
        let s_5_11: i64 = fn_state.e;
        // D s_5_12: cast zx s_5_11 -> i
        let s_5_12: i128 = (i128::try_from(s_5_11).unwrap());
        // D s_5_13: cast zx s_5_10 -> i
        let s_5_13: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_14: read-var operand2:bv
        let s_5_14: Bits = fn_state.operand2;
        // D s_5_15: call Elem_read(s_5_14, s_5_12, s_5_13)
        let s_5_15: Bits = Elem_read(state, tracer, s_5_14, s_5_12, s_5_13);
        // D s_5_16: read-var esizeshadow#6094:i64
        let s_5_16: i64 = fn_state.esizeshadow_6094;
        // D s_5_17: cast zx s_5_16 -> i
        let s_5_17: i128 = (i128::try_from(s_5_16).unwrap());
        // D s_5_18: cast reint s_5_17 -> i64
        let s_5_18: i64 = (s_5_17 as i64);
        // D s_5_19: add s_5_7 s_5_15
        let s_5_19: Bits = (s_5_7 + s_5_15);
        // D s_5_20: read-var e:i64
        let s_5_20: i64 = fn_state.e;
        // D s_5_21: cast zx s_5_20 -> i
        let s_5_21: i128 = (i128::try_from(s_5_20).unwrap());
        // D s_5_22: cast zx s_5_18 -> i
        let s_5_22: i128 = (i128::try_from(s_5_18).unwrap());
        // D s_5_23: read-var result:bv
        let s_5_23: Bits = fn_state.result;
        // D s_5_24: call Elem_set(s_5_23, s_5_21, s_5_22, s_5_19)
        let s_5_24: Bits = Elem_set(state, tracer, s_5_23, s_5_21, s_5_22, s_5_19);
        // D s_5_25: write-var result <= s_5_24
        fn_state.result = s_5_24;
        // D s_5_26: read-var e:i64
        let s_5_26: i64 = fn_state.e;
        // C s_5_27: const #1s : i64
        let s_5_27: i64 = 1;
        // D s_5_28: add s_5_26 s_5_27
        let s_5_28: i64 = (s_5_26 + s_5_27);
        // D s_5_29: write-var e <= s_5_28
        fn_state.e = s_5_28;
        // N s_5_30: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var vec:i
        let s_6_0: i128 = fn_state.vec;
        // D s_6_1: read-var VLshadow#6096:i64
        let s_6_1: i64 = fn_state.VLshadow_6096;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: cast reint s_6_2 -> i64
        let s_6_3: i64 = (s_6_2 as i64);
        // D s_6_4: cast zx s_6_3 -> i
        let s_6_4: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_5: read-var result:bv
        let s_6_5: Bits = fn_state.result;
        // D s_6_6: call ZAvector_set(s_6_0, s_6_4, s_6_5)
        let s_6_6: () = ZAvector_set(state, tracer, s_6_0, s_6_4, s_6_5);
        // D s_6_7: read-var vstride:i64
        let s_6_7: i64 = fn_state.vstride;
        // D s_6_8: cast zx s_6_7 -> i
        let s_6_8: i128 = (i128::try_from(s_6_7).unwrap());
        // D s_6_9: read-var vec:i
        let s_6_9: i128 = fn_state.vec;
        // D s_6_10: add s_6_9 s_6_8
        let s_6_10: i128 = (s_6_9 + s_6_8);
        // D s_6_11: write-var vec <= s_6_10
        fn_state.vec = s_6_10;
        // D s_6_12: read-var r:i64
        let s_6_12: i64 = fn_state.r;
        // C s_6_13: const #1s : i64
        let s_6_13: i64 = 1;
        // D s_6_14: add s_6_12 s_6_13
        let s_6_14: i64 = (s_6_12 + s_6_13);
        // D s_6_15: write-var r <= s_6_14
        fn_state.r = s_6_14;
        // N s_6_16: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: return
        return;
    }
}
