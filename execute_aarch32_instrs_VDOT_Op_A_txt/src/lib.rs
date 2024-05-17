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
use D_set::*;
use Elem_set::*;
use Elem_read::*;
use CheckAdvSIMDEnabled::*;
use D_read::*;
use common::*;
pub fn execute_aarch32_instrs_VDOT_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    esize: i64,
    m: i64,
    n: i64,
    regs: i64,
    is_signed: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        e: i64,
        res: i128,
        gs_326004: i64,
        element2: i128,
        element1: i128,
        result: u64,
        i: i64,
        operand1: u64,
        operand2: u64,
        d: i64,
        esize: i64,
        m: i64,
        n: i64,
        regs: i64,
        is_signed: bool,
    }
    let fn_state = FunctionState {
        d,
        esize,
        m,
        n,
        regs,
        is_signed,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call CheckAdvSIMDEnabled(s_0_0)
        let s_0_1: () = CheckAdvSIMDEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0s : i64
        let s_1_0: i64 = 0;
        // C s_1_1: const #1s : i
        let s_1_1: i128 = 1;
        // D s_1_2: read-var regs:i64
        let s_1_2: i64 = fn_state.regs;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: sub s_1_3 s_1_1
        let s_1_4: i128 = ((s_1_3) - (s_1_1));
        // D s_1_5: cast reint s_1_4 -> i64
        let s_1_5: i64 = (s_1_4 as i64);
        // D s_1_6: write-var gs#326004 <= s_1_5
        fn_state.gs_326004 = s_1_5;
        // D s_1_7: write-var r <= s_1_0
        fn_state.r = s_1_0;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var r:i64
        let s_2_0: i64 = fn_state.r;
        // D s_2_1: read-var gs#326004:i64
        let s_2_1: i64 = fn_state.gs_326004;
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
        // D s_3_6: cast zx s_3_5 -> i
        let s_3_6: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_7: call D_read(s_3_6)
        let s_3_7: u64 = D_read(state, tracer, s_3_6);
        // D s_3_8: write-var operand1 <= s_3_7
        fn_state.operand1 = s_3_7;
        // D s_3_9: read-var m:i64
        let s_3_9: i64 = fn_state.m;
        // D s_3_10: cast zx s_3_9 -> i
        let s_3_10: i128 = (i128::try_from(s_3_9).unwrap());
        // D s_3_11: read-var r:i64
        let s_3_11: i64 = fn_state.r;
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_13: add s_3_10 s_3_12
        let s_3_13: i128 = (s_3_10 + s_3_12);
        // D s_3_14: cast reint s_3_13 -> i64
        let s_3_14: i64 = (s_3_13 as i64);
        // D s_3_15: cast zx s_3_14 -> i
        let s_3_15: i128 = (i128::try_from(s_3_14).unwrap());
        // D s_3_16: call D_read(s_3_15)
        let s_3_16: u64 = D_read(state, tracer, s_3_15);
        // D s_3_17: write-var operand2 <= s_3_16
        fn_state.operand2 = s_3_16;
        // D s_3_18: read-var d:i64
        let s_3_18: i64 = fn_state.d;
        // D s_3_19: cast zx s_3_18 -> i
        let s_3_19: i128 = (i128::try_from(s_3_18).unwrap());
        // D s_3_20: read-var r:i64
        let s_3_20: i64 = fn_state.r;
        // D s_3_21: cast zx s_3_20 -> i
        let s_3_21: i128 = (i128::try_from(s_3_20).unwrap());
        // D s_3_22: add s_3_19 s_3_21
        let s_3_22: i128 = (s_3_19 + s_3_21);
        // D s_3_23: cast reint s_3_22 -> i64
        let s_3_23: i64 = (s_3_22 as i64);
        // D s_3_24: cast zx s_3_23 -> i
        let s_3_24: i128 = (i128::try_from(s_3_23).unwrap());
        // D s_3_25: call D_read(s_3_24)
        let s_3_25: u64 = D_read(state, tracer, s_3_24);
        // D s_3_26: write-var result <= s_3_25
        fn_state.result = s_3_25;
        // C s_3_27: const #0s : i64
        let s_3_27: i64 = 0;
        // D s_3_28: write-var e <= s_3_27
        fn_state.e = s_3_27;
        // N s_3_29: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // C s_4_1: const #1s : i64
        let s_4_1: i64 = 1;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b12 b5
        if s_4_2 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0s : i
        let s_5_0: i128 = 0;
        // D s_5_1: write-var res <= s_5_0
        fn_state.res = s_5_0;
        // C s_5_2: const #0s : i64
        let s_5_2: i64 = 0;
        // D s_5_3: write-var i <= s_5_2
        fn_state.i = s_5_2;
        // N s_5_4: jump b6
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
        // N s_6_3: branch s_6_2 b11 b7
        if s_6_2 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var is_signed:u8
        let s_7_0: bool = fn_state.is_signed;
        // N s_7_1: branch s_7_0 b10 b8
        if s_7_0 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #4s : i
        let s_8_0: i128 = 4;
        // D s_8_1: read-var e:i64
        let s_8_1: i64 = fn_state.e;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // D s_8_3: mul s_8_0 s_8_2
        let s_8_3: i128 = ((s_8_0) * (s_8_2));
        // D s_8_4: cast reint s_8_3 -> i64
        let s_8_4: i64 = (s_8_3 as i64);
        // D s_8_5: cast zx s_8_4 -> i
        let s_8_5: i128 = (i128::try_from(s_8_4).unwrap());
        // D s_8_6: read-var i:i64
        let s_8_6: i64 = fn_state.i;
        // D s_8_7: cast zx s_8_6 -> i
        let s_8_7: i128 = (i128::try_from(s_8_6).unwrap());
        // D s_8_8: add s_8_5 s_8_7
        let s_8_8: i128 = (s_8_5 + s_8_7);
        // D s_8_9: cast reint s_8_8 -> i64
        let s_8_9: i64 = (s_8_8 as i64);
        // C s_8_10: const #4s : i
        let s_8_10: i128 = 4;
        // D s_8_11: read-var esize:i64
        let s_8_11: i64 = fn_state.esize;
        // D s_8_12: cast zx s_8_11 -> i
        let s_8_12: i128 = (i128::try_from(s_8_11).unwrap());
        // D s_8_13: div s_8_12 s_8_10
        let s_8_13: i128 = ((s_8_12) / (s_8_10));
        // D s_8_14: cast reint s_8_13 -> i64
        let s_8_14: i64 = (s_8_13 as i64);
        // D s_8_15: cast zx s_8_14 -> i
        let s_8_15: i128 = (i128::try_from(s_8_14).unwrap());
        // D s_8_16: cast reint s_8_15 -> i64
        let s_8_16: i64 = (s_8_15 as i64);
        // D s_8_17: read-var operand1:u64
        let s_8_17: u64 = fn_state.operand1;
        // D s_8_18: cast zx s_8_17 -> bv
        let s_8_18: Bits = Bits::new(s_8_17 as u128, 64u16);
        // D s_8_19: cast zx s_8_9 -> i
        let s_8_19: i128 = (i128::try_from(s_8_9).unwrap());
        // D s_8_20: cast zx s_8_16 -> i
        let s_8_20: i128 = (i128::try_from(s_8_16).unwrap());
        // D s_8_21: call Elem_read(s_8_18, s_8_19, s_8_20)
        let s_8_21: Bits = Elem_read(state, tracer, s_8_18, s_8_19, s_8_20);
        // D s_8_22: cast reint s_8_21 -> u8
        let s_8_22: u8 = (s_8_21.value() as u8);
        // D s_8_23: cast zx s_8_22 -> bv
        let s_8_23: Bits = Bits::new(s_8_22 as u128, 8u16);
        // D s_8_24: cast zx s_8_23 -> i
        let s_8_24: i128 = (s_8_23.value() as i128);
        // D s_8_25: write-var element1 <= s_8_24
        fn_state.element1 = s_8_24;
        // C s_8_26: const #4s : i
        let s_8_26: i128 = 4;
        // D s_8_27: read-var e:i64
        let s_8_27: i64 = fn_state.e;
        // D s_8_28: cast zx s_8_27 -> i
        let s_8_28: i128 = (i128::try_from(s_8_27).unwrap());
        // D s_8_29: mul s_8_26 s_8_28
        let s_8_29: i128 = ((s_8_26) * (s_8_28));
        // D s_8_30: cast reint s_8_29 -> i64
        let s_8_30: i64 = (s_8_29 as i64);
        // D s_8_31: cast zx s_8_30 -> i
        let s_8_31: i128 = (i128::try_from(s_8_30).unwrap());
        // D s_8_32: read-var i:i64
        let s_8_32: i64 = fn_state.i;
        // D s_8_33: cast zx s_8_32 -> i
        let s_8_33: i128 = (i128::try_from(s_8_32).unwrap());
        // D s_8_34: add s_8_31 s_8_33
        let s_8_34: i128 = (s_8_31 + s_8_33);
        // D s_8_35: cast reint s_8_34 -> i64
        let s_8_35: i64 = (s_8_34 as i64);
        // C s_8_36: const #4s : i
        let s_8_36: i128 = 4;
        // D s_8_37: read-var esize:i64
        let s_8_37: i64 = fn_state.esize;
        // D s_8_38: cast zx s_8_37 -> i
        let s_8_38: i128 = (i128::try_from(s_8_37).unwrap());
        // D s_8_39: div s_8_38 s_8_36
        let s_8_39: i128 = ((s_8_38) / (s_8_36));
        // D s_8_40: cast reint s_8_39 -> i64
        let s_8_40: i64 = (s_8_39 as i64);
        // D s_8_41: cast zx s_8_40 -> i
        let s_8_41: i128 = (i128::try_from(s_8_40).unwrap());
        // D s_8_42: cast reint s_8_41 -> i64
        let s_8_42: i64 = (s_8_41 as i64);
        // D s_8_43: read-var operand2:u64
        let s_8_43: u64 = fn_state.operand2;
        // D s_8_44: cast zx s_8_43 -> bv
        let s_8_44: Bits = Bits::new(s_8_43 as u128, 64u16);
        // D s_8_45: cast zx s_8_35 -> i
        let s_8_45: i128 = (i128::try_from(s_8_35).unwrap());
        // D s_8_46: cast zx s_8_42 -> i
        let s_8_46: i128 = (i128::try_from(s_8_42).unwrap());
        // D s_8_47: call Elem_read(s_8_44, s_8_45, s_8_46)
        let s_8_47: Bits = Elem_read(state, tracer, s_8_44, s_8_45, s_8_46);
        // D s_8_48: cast reint s_8_47 -> u8
        let s_8_48: u8 = (s_8_47.value() as u8);
        // D s_8_49: cast zx s_8_48 -> bv
        let s_8_49: Bits = Bits::new(s_8_48 as u128, 8u16);
        // D s_8_50: cast zx s_8_49 -> i
        let s_8_50: i128 = (s_8_49.value() as i128);
        // D s_8_51: write-var element2 <= s_8_50
        fn_state.element2 = s_8_50;
        // N s_8_52: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var element1:i
        let s_9_0: i128 = fn_state.element1;
        // D s_9_1: read-var element2:i
        let s_9_1: i128 = fn_state.element2;
        // D s_9_2: mul s_9_0 s_9_1
        let s_9_2: i128 = ((s_9_0) * (s_9_1));
        // D s_9_3: read-var res:i
        let s_9_3: i128 = fn_state.res;
        // D s_9_4: add s_9_3 s_9_2
        let s_9_4: i128 = (s_9_3 + s_9_2);
        // D s_9_5: write-var res <= s_9_4
        fn_state.res = s_9_4;
        // D s_9_6: read-var i:i64
        let s_9_6: i64 = fn_state.i;
        // C s_9_7: const #1s : i64
        let s_9_7: i64 = 1;
        // D s_9_8: add s_9_6 s_9_7
        let s_9_8: i64 = (s_9_6 + s_9_7);
        // D s_9_9: write-var i <= s_9_8
        fn_state.i = s_9_8;
        // N s_9_10: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #4s : i
        let s_10_0: i128 = 4;
        // D s_10_1: read-var e:i64
        let s_10_1: i64 = fn_state.e;
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // D s_10_3: mul s_10_0 s_10_2
        let s_10_3: i128 = ((s_10_0) * (s_10_2));
        // D s_10_4: cast reint s_10_3 -> i64
        let s_10_4: i64 = (s_10_3 as i64);
        // D s_10_5: cast zx s_10_4 -> i
        let s_10_5: i128 = (i128::try_from(s_10_4).unwrap());
        // D s_10_6: read-var i:i64
        let s_10_6: i64 = fn_state.i;
        // D s_10_7: cast zx s_10_6 -> i
        let s_10_7: i128 = (i128::try_from(s_10_6).unwrap());
        // D s_10_8: add s_10_5 s_10_7
        let s_10_8: i128 = (s_10_5 + s_10_7);
        // D s_10_9: cast reint s_10_8 -> i64
        let s_10_9: i64 = (s_10_8 as i64);
        // C s_10_10: const #4s : i
        let s_10_10: i128 = 4;
        // D s_10_11: read-var esize:i64
        let s_10_11: i64 = fn_state.esize;
        // D s_10_12: cast zx s_10_11 -> i
        let s_10_12: i128 = (i128::try_from(s_10_11).unwrap());
        // D s_10_13: div s_10_12 s_10_10
        let s_10_13: i128 = ((s_10_12) / (s_10_10));
        // D s_10_14: cast reint s_10_13 -> i64
        let s_10_14: i64 = (s_10_13 as i64);
        // D s_10_15: cast zx s_10_14 -> i
        let s_10_15: i128 = (i128::try_from(s_10_14).unwrap());
        // D s_10_16: cast reint s_10_15 -> i64
        let s_10_16: i64 = (s_10_15 as i64);
        // D s_10_17: read-var operand1:u64
        let s_10_17: u64 = fn_state.operand1;
        // D s_10_18: cast zx s_10_17 -> bv
        let s_10_18: Bits = Bits::new(s_10_17 as u128, 64u16);
        // D s_10_19: cast zx s_10_9 -> i
        let s_10_19: i128 = (i128::try_from(s_10_9).unwrap());
        // D s_10_20: cast zx s_10_16 -> i
        let s_10_20: i128 = (i128::try_from(s_10_16).unwrap());
        // D s_10_21: call Elem_read(s_10_18, s_10_19, s_10_20)
        let s_10_21: Bits = Elem_read(state, tracer, s_10_18, s_10_19, s_10_20);
        // D s_10_22: cast reint s_10_21 -> u8
        let s_10_22: u8 = (s_10_21.value() as u8);
        // D s_10_23: cast zx s_10_22 -> bv
        let s_10_23: Bits = Bits::new(s_10_22 as u128, 8u16);
        // D s_10_24: cast sx s_10_23 -> i
        let s_10_24: i128 = {
            let sign_bit = s_10_23.length() - 1;
            let mut result = s_10_23.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_10_25: write-var element1 <= s_10_24
        fn_state.element1 = s_10_24;
        // C s_10_26: const #4s : i
        let s_10_26: i128 = 4;
        // D s_10_27: read-var e:i64
        let s_10_27: i64 = fn_state.e;
        // D s_10_28: cast zx s_10_27 -> i
        let s_10_28: i128 = (i128::try_from(s_10_27).unwrap());
        // D s_10_29: mul s_10_26 s_10_28
        let s_10_29: i128 = ((s_10_26) * (s_10_28));
        // D s_10_30: cast reint s_10_29 -> i64
        let s_10_30: i64 = (s_10_29 as i64);
        // D s_10_31: cast zx s_10_30 -> i
        let s_10_31: i128 = (i128::try_from(s_10_30).unwrap());
        // D s_10_32: read-var i:i64
        let s_10_32: i64 = fn_state.i;
        // D s_10_33: cast zx s_10_32 -> i
        let s_10_33: i128 = (i128::try_from(s_10_32).unwrap());
        // D s_10_34: add s_10_31 s_10_33
        let s_10_34: i128 = (s_10_31 + s_10_33);
        // D s_10_35: cast reint s_10_34 -> i64
        let s_10_35: i64 = (s_10_34 as i64);
        // C s_10_36: const #4s : i
        let s_10_36: i128 = 4;
        // D s_10_37: read-var esize:i64
        let s_10_37: i64 = fn_state.esize;
        // D s_10_38: cast zx s_10_37 -> i
        let s_10_38: i128 = (i128::try_from(s_10_37).unwrap());
        // D s_10_39: div s_10_38 s_10_36
        let s_10_39: i128 = ((s_10_38) / (s_10_36));
        // D s_10_40: cast reint s_10_39 -> i64
        let s_10_40: i64 = (s_10_39 as i64);
        // D s_10_41: cast zx s_10_40 -> i
        let s_10_41: i128 = (i128::try_from(s_10_40).unwrap());
        // D s_10_42: cast reint s_10_41 -> i64
        let s_10_42: i64 = (s_10_41 as i64);
        // D s_10_43: read-var operand2:u64
        let s_10_43: u64 = fn_state.operand2;
        // D s_10_44: cast zx s_10_43 -> bv
        let s_10_44: Bits = Bits::new(s_10_43 as u128, 64u16);
        // D s_10_45: cast zx s_10_35 -> i
        let s_10_45: i128 = (i128::try_from(s_10_35).unwrap());
        // D s_10_46: cast zx s_10_42 -> i
        let s_10_46: i128 = (i128::try_from(s_10_42).unwrap());
        // D s_10_47: call Elem_read(s_10_44, s_10_45, s_10_46)
        let s_10_47: Bits = Elem_read(state, tracer, s_10_44, s_10_45, s_10_46);
        // D s_10_48: cast reint s_10_47 -> u8
        let s_10_48: u8 = (s_10_47.value() as u8);
        // D s_10_49: cast zx s_10_48 -> bv
        let s_10_49: Bits = Bits::new(s_10_48 as u128, 8u16);
        // D s_10_50: cast sx s_10_49 -> i
        let s_10_50: i128 = {
            let sign_bit = s_10_49.length() - 1;
            let mut result = s_10_49.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_10_51: write-var element2 <= s_10_50
        fn_state.element2 = s_10_50;
        // N s_10_52: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var res:i
        let s_11_0: i128 = fn_state.res;
        // D s_11_1: read-var esize:i64
        let s_11_1: i64 = fn_state.esize;
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // D s_11_3: cast reint s_11_2 -> i64
        let s_11_3: i64 = (s_11_2 as i64);
        // D s_11_4: read-var esize:i64
        let s_11_4: i64 = fn_state.esize;
        // D s_11_5: cast zx s_11_4 -> i
        let s_11_5: i128 = (i128::try_from(s_11_4).unwrap());
        // D s_11_6: cast reint s_11_5 -> i64
        let s_11_6: i64 = (s_11_5 as i64);
        // D s_11_7: read-var result:u64
        let s_11_7: u64 = fn_state.result;
        // D s_11_8: cast zx s_11_7 -> bv
        let s_11_8: Bits = Bits::new(s_11_7 as u128, 64u16);
        // D s_11_9: read-var e:i64
        let s_11_9: i64 = fn_state.e;
        // D s_11_10: cast zx s_11_9 -> i
        let s_11_10: i128 = (i128::try_from(s_11_9).unwrap());
        // D s_11_11: cast zx s_11_6 -> i
        let s_11_11: i128 = (i128::try_from(s_11_6).unwrap());
        // D s_11_12: call Elem_read(s_11_8, s_11_10, s_11_11)
        let s_11_12: Bits = Elem_read(state, tracer, s_11_8, s_11_10, s_11_11);
        // D s_11_13: cast reint s_11_12 -> u32
        let s_11_13: u32 = (s_11_12.value() as u32);
        // D s_11_14: cast zx s_11_13 -> bv
        let s_11_14: Bits = Bits::new(s_11_13 as u128, 32u16);
        // D s_11_15: cast cvt s_11_0 -> bv
        let s_11_15: Bits = Bits::new(s_11_0 as u128, 128);
        // D s_11_16: add s_11_14 s_11_15
        let s_11_16: Bits = (s_11_14 + s_11_15);
        // D s_11_17: cast reint s_11_16 -> u32
        let s_11_17: u32 = (s_11_16.value() as u32);
        // D s_11_18: read-var result:u64
        let s_11_18: u64 = fn_state.result;
        // D s_11_19: cast zx s_11_18 -> bv
        let s_11_19: Bits = Bits::new(s_11_18 as u128, 64u16);
        // D s_11_20: read-var e:i64
        let s_11_20: i64 = fn_state.e;
        // D s_11_21: cast zx s_11_20 -> i
        let s_11_21: i128 = (i128::try_from(s_11_20).unwrap());
        // D s_11_22: cast zx s_11_3 -> i
        let s_11_22: i128 = (i128::try_from(s_11_3).unwrap());
        // D s_11_23: cast zx s_11_17 -> bv
        let s_11_23: Bits = Bits::new(s_11_17 as u128, 32u16);
        // D s_11_24: call Elem_set(s_11_19, s_11_21, s_11_22, s_11_23)
        let s_11_24: Bits = Elem_set(state, tracer, s_11_19, s_11_21, s_11_22, s_11_23);
        // D s_11_25: cast reint s_11_24 -> u64
        let s_11_25: u64 = (s_11_24.value() as u64);
        // D s_11_26: write-var result <= s_11_25
        fn_state.result = s_11_25;
        // D s_11_27: read-var e:i64
        let s_11_27: i64 = fn_state.e;
        // C s_11_28: const #1s : i64
        let s_11_28: i64 = 1;
        // D s_11_29: add s_11_27 s_11_28
        let s_11_29: i64 = (s_11_27 + s_11_28);
        // D s_11_30: write-var e <= s_11_29
        fn_state.e = s_11_29;
        // N s_11_31: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var d:i64
        let s_12_0: i64 = fn_state.d;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: read-var r:i64
        let s_12_2: i64 = fn_state.r;
        // D s_12_3: cast zx s_12_2 -> i
        let s_12_3: i128 = (i128::try_from(s_12_2).unwrap());
        // D s_12_4: add s_12_1 s_12_3
        let s_12_4: i128 = (s_12_1 + s_12_3);
        // D s_12_5: cast reint s_12_4 -> i64
        let s_12_5: i64 = (s_12_4 as i64);
        // D s_12_6: cast zx s_12_5 -> i
        let s_12_6: i128 = (i128::try_from(s_12_5).unwrap());
        // D s_12_7: read-var result:u64
        let s_12_7: u64 = fn_state.result;
        // D s_12_8: call D_set(s_12_6, s_12_7)
        let s_12_8: () = D_set(state, tracer, s_12_6, s_12_7);
        // D s_12_9: read-var r:i64
        let s_12_9: i64 = fn_state.r;
        // C s_12_10: const #1s : i64
        let s_12_10: i64 = 1;
        // D s_12_11: add s_12_9 s_12_10
        let s_12_11: i64 = (s_12_9 + s_12_10);
        // D s_12_12: write-var r <= s_12_11
        fn_state.r = s_12_11;
        // N s_12_13: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: return
        return;
    }
}
