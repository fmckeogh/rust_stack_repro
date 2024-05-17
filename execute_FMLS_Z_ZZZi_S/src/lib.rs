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
use FPCR_read::*;
use FPNeg::*;
use Elem_read::*;
use FPMulAdd::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_FMLS_Z_ZZZi_S<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    da: i64,
    esize: i64,
    index: i64,
    m: i64,
    n: i64,
    op1_neg: bool,
    op3_neg: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        element2: Bits,
        e: i64,
        element1: Bits,
        element3: Bits,
        VLshadow_2320: i64,
        VLshadow_2319: i64,
        ga_275408: i64,
        gs_729770: Bits,
        gs_181547: i64,
        result: Bits,
        operand1: Bits,
        eltspersegment: i64,
        operand2: Bits,
        VL: i64,
        da: i64,
        esize: i64,
        index: i64,
        m: i64,
        n: i64,
        op1_neg: bool,
        op3_neg: bool,
    }
    let fn_state = FunctionState {
        VL,
        da,
        esize,
        index,
        m,
        n,
        op1_neg,
        op3_neg,
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
        // D s_0_3: write-var VLshadow#2319 <= s_0_2
        fn_state.VLshadow_2319 = s_0_2;
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
        // D s_1_0: read-var VLshadow#2319:i64
        let s_1_0: i64 = fn_state.VLshadow_2319;
        // D s_1_1: write-var VLshadow#2320 <= s_1_0
        fn_state.VLshadow_2320 = s_1_0;
        // D s_1_2: read-var VLshadow#2320:i64
        let s_1_2: i64 = fn_state.VLshadow_2320;
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
        // C s_1_8: const #128s : i
        let s_1_8: i128 = 128;
        // D s_1_9: read-var esize:i64
        let s_1_9: i64 = fn_state.esize;
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_11: div s_1_8 s_1_10
        let s_1_11: i128 = ((s_1_8) / (s_1_10));
        // D s_1_12: cast reint s_1_11 -> i64
        let s_1_12: i64 = (s_1_11 as i64);
        // D s_1_13: write-var eltspersegment <= s_1_12
        fn_state.eltspersegment = s_1_12;
        // D s_1_14: read-var VLshadow#2320:i64
        let s_1_14: i64 = fn_state.VLshadow_2320;
        // D s_1_15: cast zx s_1_14 -> i
        let s_1_15: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_16: cast reint s_1_15 -> i64
        let s_1_16: i64 = (s_1_15 as i64);
        // D s_1_17: read-var n:i64
        let s_1_17: i64 = fn_state.n;
        // D s_1_18: cast zx s_1_17 -> i
        let s_1_18: i128 = (i128::try_from(s_1_17).unwrap());
        // D s_1_19: cast zx s_1_16 -> i
        let s_1_19: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_20: call Z_read(s_1_18, s_1_19)
        let s_1_20: Bits = Z_read(state, tracer, s_1_18, s_1_19);
        // D s_1_21: write-var operand1 <= s_1_20
        fn_state.operand1 = s_1_20;
        // D s_1_22: read-var VLshadow#2320:i64
        let s_1_22: i64 = fn_state.VLshadow_2320;
        // D s_1_23: cast zx s_1_22 -> i
        let s_1_23: i128 = (i128::try_from(s_1_22).unwrap());
        // D s_1_24: cast reint s_1_23 -> i64
        let s_1_24: i64 = (s_1_23 as i64);
        // D s_1_25: read-var m:i64
        let s_1_25: i64 = fn_state.m;
        // D s_1_26: cast zx s_1_25 -> i
        let s_1_26: i128 = (i128::try_from(s_1_25).unwrap());
        // D s_1_27: cast zx s_1_24 -> i
        let s_1_27: i128 = (i128::try_from(s_1_24).unwrap());
        // D s_1_28: call Z_read(s_1_26, s_1_27)
        let s_1_28: Bits = Z_read(state, tracer, s_1_26, s_1_27);
        // D s_1_29: write-var operand2 <= s_1_28
        fn_state.operand2 = s_1_28;
        // D s_1_30: read-var VLshadow#2320:i64
        let s_1_30: i64 = fn_state.VLshadow_2320;
        // D s_1_31: cast zx s_1_30 -> i
        let s_1_31: i128 = (i128::try_from(s_1_30).unwrap());
        // D s_1_32: cast reint s_1_31 -> i64
        let s_1_32: i64 = (s_1_31 as i64);
        // D s_1_33: read-var da:i64
        let s_1_33: i64 = fn_state.da;
        // D s_1_34: cast zx s_1_33 -> i
        let s_1_34: i128 = (i128::try_from(s_1_33).unwrap());
        // D s_1_35: cast zx s_1_32 -> i
        let s_1_35: i128 = (i128::try_from(s_1_32).unwrap());
        // D s_1_36: call Z_read(s_1_34, s_1_35)
        let s_1_36: Bits = Z_read(state, tracer, s_1_34, s_1_35);
        // D s_1_37: write-var result <= s_1_36
        fn_state.result = s_1_36;
        // C s_1_38: const #0s : i64
        let s_1_38: i64 = 0;
        // C s_1_39: const #1s : i
        let s_1_39: i128 = 1;
        // D s_1_40: cast zx s_1_7 -> i
        let s_1_40: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_41: sub s_1_40 s_1_39
        let s_1_41: i128 = ((s_1_40) - (s_1_39));
        // D s_1_42: cast reint s_1_41 -> i64
        let s_1_42: i64 = (s_1_41 as i64);
        // D s_1_43: write-var gs#181547 <= s_1_42
        fn_state.gs_181547 = s_1_42;
        // D s_1_44: write-var e <= s_1_38
        fn_state.e = s_1_38;
        // N s_1_45: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#181547:i64
        let s_2_1: i64 = fn_state.gs_181547;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b13 b3
        if s_2_2 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var e:i64
        let s_3_0: i64 = fn_state.e;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var eltspersegment:i64
        let s_3_2: i64 = fn_state.eltspersegment;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: mod s_3_1 s_3_3
        let s_3_4: i128 = ((s_3_1) % (s_3_3));
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: read-var e:i64
        let s_3_6: i64 = fn_state.e;
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_8: cast zx s_3_5 -> i
        let s_3_8: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_9: sub s_3_7 s_3_8
        let s_3_9: i128 = ((s_3_7) - (s_3_8));
        // D s_3_10: cast reint s_3_9 -> i64
        let s_3_10: i64 = (s_3_9 as i64);
        // D s_3_11: cast zx s_3_10 -> i
        let s_3_11: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_12: read-var index:i64
        let s_3_12: i64 = fn_state.index;
        // D s_3_13: cast zx s_3_12 -> i
        let s_3_13: i128 = (i128::try_from(s_3_12).unwrap());
        // D s_3_14: add s_3_11 s_3_13
        let s_3_14: i128 = (s_3_11 + s_3_13);
        // D s_3_15: cast reint s_3_14 -> i64
        let s_3_15: i64 = (s_3_14 as i64);
        // D s_3_16: read-var esize:i64
        let s_3_16: i64 = fn_state.esize;
        // D s_3_17: cast zx s_3_16 -> i
        let s_3_17: i128 = (i128::try_from(s_3_16).unwrap());
        // D s_3_18: cast reint s_3_17 -> i64
        let s_3_18: i64 = (s_3_17 as i64);
        // D s_3_19: read-var e:i64
        let s_3_19: i64 = fn_state.e;
        // D s_3_20: cast zx s_3_19 -> i
        let s_3_20: i128 = (i128::try_from(s_3_19).unwrap());
        // D s_3_21: cast zx s_3_18 -> i
        let s_3_21: i128 = (i128::try_from(s_3_18).unwrap());
        // D s_3_22: read-var operand1:bv
        let s_3_22: Bits = fn_state.operand1;
        // D s_3_23: call Elem_read(s_3_22, s_3_20, s_3_21)
        let s_3_23: Bits = Elem_read(state, tracer, s_3_22, s_3_20, s_3_21);
        // D s_3_24: write-var element1 <= s_3_23
        fn_state.element1 = s_3_23;
        // D s_3_25: read-var esize:i64
        let s_3_25: i64 = fn_state.esize;
        // D s_3_26: cast zx s_3_25 -> i
        let s_3_26: i128 = (i128::try_from(s_3_25).unwrap());
        // D s_3_27: cast reint s_3_26 -> i64
        let s_3_27: i64 = (s_3_26 as i64);
        // D s_3_28: cast zx s_3_15 -> i
        let s_3_28: i128 = (i128::try_from(s_3_15).unwrap());
        // D s_3_29: cast zx s_3_27 -> i
        let s_3_29: i128 = (i128::try_from(s_3_27).unwrap());
        // D s_3_30: read-var operand2:bv
        let s_3_30: Bits = fn_state.operand2;
        // D s_3_31: call Elem_read(s_3_30, s_3_28, s_3_29)
        let s_3_31: Bits = Elem_read(state, tracer, s_3_30, s_3_28, s_3_29);
        // D s_3_32: write-var element2 <= s_3_31
        fn_state.element2 = s_3_31;
        // D s_3_33: read-var esize:i64
        let s_3_33: i64 = fn_state.esize;
        // D s_3_34: cast zx s_3_33 -> i
        let s_3_34: i128 = (i128::try_from(s_3_33).unwrap());
        // D s_3_35: cast reint s_3_34 -> i64
        let s_3_35: i64 = (s_3_34 as i64);
        // D s_3_36: read-var e:i64
        let s_3_36: i64 = fn_state.e;
        // D s_3_37: cast zx s_3_36 -> i
        let s_3_37: i128 = (i128::try_from(s_3_36).unwrap());
        // D s_3_38: cast zx s_3_35 -> i
        let s_3_38: i128 = (i128::try_from(s_3_35).unwrap());
        // D s_3_39: read-var result:bv
        let s_3_39: Bits = fn_state.result;
        // D s_3_40: call Elem_read(s_3_39, s_3_37, s_3_38)
        let s_3_40: Bits = Elem_read(state, tracer, s_3_39, s_3_37, s_3_38);
        // D s_3_41: write-var element3 <= s_3_40
        fn_state.element3 = s_3_40;
        // D s_3_42: read-var op1_neg:u8
        let s_3_42: bool = fn_state.op1_neg;
        // N s_3_43: branch s_3_42 b11 b4
        if s_3_42 {
            return block_11(state, tracer, fn_state);
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
        // D s_5_0: read-var op3_neg:u8
        let s_5_0: bool = fn_state.op3_neg;
        // N s_5_1: branch s_5_0 b9 b6
        if s_5_0 {
            return block_9(state, tracer, fn_state);
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
        // D s_7_0: read-var esize:i64
        let s_7_0: i64 = fn_state.esize;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: cast reint s_7_1 -> i64
        let s_7_2: i64 = (s_7_1 as i64);
        // D s_7_3: write-var ga#275408 <= s_7_2
        fn_state.ga_275408 = s_7_2;
        // C s_7_4: const #() : ()
        let s_7_4: () = ();
        // S s_7_5: call FPCR_read(s_7_4)
        let s_7_5: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_7_4);
        // D s_7_6: read-var element3:bv
        let s_7_6: Bits = fn_state.element3;
        // D s_7_7: read-var element1:bv
        let s_7_7: Bits = fn_state.element1;
        // D s_7_8: read-var element2:bv
        let s_7_8: Bits = fn_state.element2;
        // D s_7_9: call FPMulAdd(s_7_6, s_7_7, s_7_8, s_7_5)
        let s_7_9: Bits = FPMulAdd(state, tracer, s_7_6, s_7_7, s_7_8, s_7_5);
        // D s_7_10: write-var gs#729770 <= s_7_9
        fn_state.gs_729770 = s_7_9;
        // N s_7_11: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#729770:bv
        let s_8_0: Bits = fn_state.gs_729770;
        // D s_8_1: cast reint s_8_0 -> u32
        let s_8_1: u32 = (s_8_0.value() as u32);
        // D s_8_2: read-var e:i64
        let s_8_2: i64 = fn_state.e;
        // D s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_4: read-var ga#275408:i64
        let s_8_4: i64 = fn_state.ga_275408;
        // D s_8_5: cast zx s_8_4 -> i
        let s_8_5: i128 = (i128::try_from(s_8_4).unwrap());
        // D s_8_6: cast zx s_8_1 -> bv
        let s_8_6: Bits = Bits::new(s_8_1 as u128, 32u16);
        // D s_8_7: read-var result:bv
        let s_8_7: Bits = fn_state.result;
        // D s_8_8: call Elem_set(s_8_7, s_8_3, s_8_5, s_8_6)
        let s_8_8: Bits = Elem_set(state, tracer, s_8_7, s_8_3, s_8_5, s_8_6);
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
        // N s_8_14: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var element3:bv
        let s_9_0: Bits = fn_state.element3;
        // D s_9_1: call FPNeg(s_9_0)
        let s_9_1: Bits = FPNeg(state, tracer, s_9_0);
        // D s_9_2: write-var element3 <= s_9_1
        fn_state.element3 = s_9_1;
        // N s_9_3: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var element1:bv
        let s_11_0: Bits = fn_state.element1;
        // D s_11_1: call FPNeg(s_11_0)
        let s_11_1: Bits = FPNeg(state, tracer, s_11_0);
        // D s_11_2: write-var element1 <= s_11_1
        fn_state.element1 = s_11_1;
        // N s_11_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var VLshadow#2320:i64
        let s_13_0: i64 = fn_state.VLshadow_2320;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: cast reint s_13_1 -> i64
        let s_13_2: i64 = (s_13_1 as i64);
        // D s_13_3: read-var da:i64
        let s_13_3: i64 = fn_state.da;
        // D s_13_4: cast zx s_13_3 -> i
        let s_13_4: i128 = (i128::try_from(s_13_3).unwrap());
        // D s_13_5: cast zx s_13_2 -> i
        let s_13_5: i128 = (i128::try_from(s_13_2).unwrap());
        // D s_13_6: read-var result:bv
        let s_13_6: Bits = fn_state.result;
        // D s_13_7: call Z_set(s_13_4, s_13_5, s_13_6)
        let s_13_7: () = Z_set(state, tracer, s_13_4, s_13_5, s_13_6);
        // N s_13_8: return
        return;
    }
}
