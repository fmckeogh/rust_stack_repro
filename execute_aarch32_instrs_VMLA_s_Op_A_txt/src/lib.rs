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
use Q_read::*;
use FPMul::*;
use Q_set::*;
use Elem_read::*;
use D_read::*;
use StandardFPSCRValue::*;
use Elem_set::*;
use CheckAdvSIMDEnabled::*;
use FPAdd::*;
use D_set::*;
use Din_read::*;
use asl_Int::*;
use FPNeg::*;
use Qin_read::*;
use common::*;
pub fn execute_aarch32_instrs_VMLA_s_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    add: bool,
    d__arg: i64,
    elements: i64,
    esize: i64,
    floating_point: bool,
    index: i64,
    long_destination: bool,
    m: i64,
    n: i64,
    regs: i64,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        ga_355701: u64,
        ga_355694: Bits,
        e: i64,
        op2val: i128,
        addend: i128,
        ga_355704: i128,
        op1val: i128,
        fp_addend: Bits,
        gs_312435: i64,
        ga_355702: i64,
        esizeshadow_7518: i64,
        op2: Bits,
        gs_312441: i64,
        d: i128,
        op1: Bits,
        ga_355703: Bits,
        add: bool,
        d__arg: i64,
        elements: i64,
        esize: i64,
        floating_point: bool,
        index: i64,
        long_destination: bool,
        m: i64,
        n: i64,
        regs: i64,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        add,
        d__arg,
        elements,
        esize,
        floating_point,
        index,
        long_destination,
        m,
        n,
        regs,
        is_unsigned,
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
        // D s_0_3: write-var esizeshadow#7518 <= s_0_2
        fn_state.esizeshadow_7518 = s_0_2;
        // D s_0_4: read-var d__arg:i64
        let s_0_4: i64 = fn_state.d__arg;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: write-var d <= s_0_5
        fn_state.d = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call CheckAdvSIMDEnabled(s_0_7)
        let s_0_8: () = CheckAdvSIMDEnabled(state, tracer, s_0_7);
        // N s_0_9: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var m:i64
        let s_1_0: i64 = fn_state.m;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: call Din_read(s_1_1)
        let s_1_2: u64 = Din_read(state, tracer, s_1_1);
        // D s_1_3: read-var esizeshadow#7518:i64
        let s_1_3: i64 = fn_state.esizeshadow_7518;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: cast reint s_1_4 -> i64
        let s_1_5: i64 = (s_1_4 as i64);
        // D s_1_6: cast zx s_1_2 -> bv
        let s_1_6: Bits = Bits::new(s_1_2 as u128, 64u16);
        // D s_1_7: read-var index:i64
        let s_1_7: i64 = fn_state.index;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast zx s_1_5 -> i
        let s_1_9: i128 = (i128::try_from(s_1_5).unwrap());
        // D s_1_10: call Elem_read(s_1_6, s_1_8, s_1_9)
        let s_1_10: Bits = Elem_read(state, tracer, s_1_6, s_1_8, s_1_9);
        // D s_1_11: write-var op2 <= s_1_10
        fn_state.op2 = s_1_10;
        // D s_1_12: read-var op2:bv
        let s_1_12: Bits = fn_state.op2;
        // D s_1_13: read-var is_unsigned:u8
        let s_1_13: bool = fn_state.is_unsigned;
        // D s_1_14: call asl_Int(s_1_12, s_1_13)
        let s_1_14: i128 = asl_Int(state, tracer, s_1_12, s_1_13);
        // D s_1_15: write-var op2val <= s_1_14
        fn_state.op2val = s_1_14;
        // C s_1_16: const #0s : i64
        let s_1_16: i64 = 0;
        // C s_1_17: const #1s : i
        let s_1_17: i128 = 1;
        // D s_1_18: read-var regs:i64
        let s_1_18: i64 = fn_state.regs;
        // D s_1_19: cast zx s_1_18 -> i
        let s_1_19: i128 = (i128::try_from(s_1_18).unwrap());
        // D s_1_20: sub s_1_19 s_1_17
        let s_1_20: i128 = ((s_1_19) - (s_1_17));
        // D s_1_21: cast reint s_1_20 -> i64
        let s_1_21: i64 = (s_1_20 as i64);
        // D s_1_22: write-var gs#312435 <= s_1_21
        fn_state.gs_312435 = s_1_21;
        // D s_1_23: write-var r <= s_1_16
        fn_state.r = s_1_16;
        // N s_1_24: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var r:i64
        let s_2_0: i64 = fn_state.r;
        // D s_2_1: read-var gs#312435:i64
        let s_2_1: i64 = fn_state.gs_312435;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b20 b3
        if s_2_2 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0s : i64
        let s_3_0: i64 = 0;
        // C s_3_1: const #1s : i
        let s_3_1: i128 = 1;
        // D s_3_2: read-var elements:i64
        let s_3_2: i64 = fn_state.elements;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: sub s_3_3 s_3_1
        let s_3_4: i128 = ((s_3_3) - (s_3_1));
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: write-var gs#312441 <= s_3_5
        fn_state.gs_312441 = s_3_5;
        // D s_3_7: write-var e <= s_3_0
        fn_state.e = s_3_0;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#312441:i64
        let s_4_1: i64 = fn_state.gs_312441;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b19 b5
        if s_4_2 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var n:i64
        let s_5_0: i64 = fn_state.n;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: read-var r:i64
        let s_5_2: i64 = fn_state.r;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: add s_5_1 s_5_3
        let s_5_4: i128 = (s_5_1 + s_5_3);
        // D s_5_5: cast reint s_5_4 -> i64
        let s_5_5: i64 = (s_5_4 as i64);
        // D s_5_6: cast zx s_5_5 -> i
        let s_5_6: i128 = (i128::try_from(s_5_5).unwrap());
        // D s_5_7: call Din_read(s_5_6)
        let s_5_7: u64 = Din_read(state, tracer, s_5_6);
        // D s_5_8: read-var esizeshadow#7518:i64
        let s_5_8: i64 = fn_state.esizeshadow_7518;
        // D s_5_9: cast zx s_5_8 -> i
        let s_5_9: i128 = (i128::try_from(s_5_8).unwrap());
        // D s_5_10: cast reint s_5_9 -> i64
        let s_5_10: i64 = (s_5_9 as i64);
        // D s_5_11: cast zx s_5_7 -> bv
        let s_5_11: Bits = Bits::new(s_5_7 as u128, 64u16);
        // D s_5_12: read-var e:i64
        let s_5_12: i64 = fn_state.e;
        // D s_5_13: cast zx s_5_12 -> i
        let s_5_13: i128 = (i128::try_from(s_5_12).unwrap());
        // D s_5_14: cast zx s_5_10 -> i
        let s_5_14: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_15: call Elem_read(s_5_11, s_5_13, s_5_14)
        let s_5_15: Bits = Elem_read(state, tracer, s_5_11, s_5_13, s_5_14);
        // D s_5_16: write-var op1 <= s_5_15
        fn_state.op1 = s_5_15;
        // D s_5_17: read-var op1:bv
        let s_5_17: Bits = fn_state.op1;
        // D s_5_18: read-var is_unsigned:u8
        let s_5_18: bool = fn_state.is_unsigned;
        // D s_5_19: call asl_Int(s_5_17, s_5_18)
        let s_5_19: i128 = asl_Int(state, tracer, s_5_17, s_5_18);
        // D s_5_20: write-var op1val <= s_5_19
        fn_state.op1val = s_5_19;
        // D s_5_21: read-var floating_point:u8
        let s_5_21: bool = fn_state.floating_point;
        // N s_5_22: branch s_5_21 b13 b6
        if s_5_21 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var add:u8
        let s_6_0: bool = fn_state.add;
        // N s_6_1: branch s_6_0 b12 b7
        if s_6_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var op1val:i
        let s_7_0: i128 = fn_state.op1val;
        // D s_7_1: neg s_7_0
        let s_7_1: i128 = -s_7_0;
        // D s_7_2: read-var op2val:i
        let s_7_2: i128 = fn_state.op2val;
        // D s_7_3: mul s_7_1 s_7_2
        let s_7_3: i128 = ((s_7_1) * (s_7_2));
        // D s_7_4: write-var addend <= s_7_3
        fn_state.addend = s_7_3;
        // N s_7_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var long_destination:u8
        let s_8_0: bool = fn_state.long_destination;
        // N s_8_1: branch s_8_0 b11 b9
        if s_8_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var r:i64
        let s_9_0: i64 = fn_state.r;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: read-var d:i
        let s_9_2: i128 = fn_state.d;
        // D s_9_3: add s_9_2 s_9_1
        let s_9_3: i128 = (s_9_2 + s_9_1);
        // D s_9_4: read-var r:i64
        let s_9_4: i64 = fn_state.r;
        // D s_9_5: cast zx s_9_4 -> i
        let s_9_5: i128 = (i128::try_from(s_9_4).unwrap());
        // D s_9_6: read-var d:i
        let s_9_6: i128 = fn_state.d;
        // D s_9_7: add s_9_6 s_9_5
        let s_9_7: i128 = (s_9_6 + s_9_5);
        // D s_9_8: call D_read(s_9_7)
        let s_9_8: u64 = D_read(state, tracer, s_9_7);
        // D s_9_9: read-var esizeshadow#7518:i64
        let s_9_9: i64 = fn_state.esizeshadow_7518;
        // D s_9_10: cast zx s_9_9 -> i
        let s_9_10: i128 = (i128::try_from(s_9_9).unwrap());
        // D s_9_11: cast reint s_9_10 -> i64
        let s_9_11: i64 = (s_9_10 as i64);
        // D s_9_12: read-var r:i64
        let s_9_12: i64 = fn_state.r;
        // D s_9_13: cast zx s_9_12 -> i
        let s_9_13: i128 = (i128::try_from(s_9_12).unwrap());
        // D s_9_14: read-var d:i
        let s_9_14: i128 = fn_state.d;
        // D s_9_15: add s_9_14 s_9_13
        let s_9_15: i128 = (s_9_14 + s_9_13);
        // D s_9_16: call Din_read(s_9_15)
        let s_9_16: u64 = Din_read(state, tracer, s_9_15);
        // D s_9_17: read-var esizeshadow#7518:i64
        let s_9_17: i64 = fn_state.esizeshadow_7518;
        // D s_9_18: cast zx s_9_17 -> i
        let s_9_18: i128 = (i128::try_from(s_9_17).unwrap());
        // D s_9_19: cast reint s_9_18 -> i64
        let s_9_19: i64 = (s_9_18 as i64);
        // D s_9_20: cast zx s_9_16 -> bv
        let s_9_20: Bits = Bits::new(s_9_16 as u128, 64u16);
        // D s_9_21: read-var e:i64
        let s_9_21: i64 = fn_state.e;
        // D s_9_22: cast zx s_9_21 -> i
        let s_9_22: i128 = (i128::try_from(s_9_21).unwrap());
        // D s_9_23: cast zx s_9_19 -> i
        let s_9_23: i128 = (i128::try_from(s_9_19).unwrap());
        // D s_9_24: call Elem_read(s_9_20, s_9_22, s_9_23)
        let s_9_24: Bits = Elem_read(state, tracer, s_9_20, s_9_22, s_9_23);
        // D s_9_25: read-var addend:i
        let s_9_25: i128 = fn_state.addend;
        // D s_9_26: cast cvt s_9_25 -> bv
        let s_9_26: Bits = Bits::new(s_9_25 as u128, 128);
        // D s_9_27: add s_9_24 s_9_26
        let s_9_27: Bits = (s_9_24 + s_9_26);
        // D s_9_28: cast zx s_9_8 -> bv
        let s_9_28: Bits = Bits::new(s_9_8 as u128, 64u16);
        // D s_9_29: read-var e:i64
        let s_9_29: i64 = fn_state.e;
        // D s_9_30: cast zx s_9_29 -> i
        let s_9_30: i128 = (i128::try_from(s_9_29).unwrap());
        // D s_9_31: cast zx s_9_11 -> i
        let s_9_31: i128 = (i128::try_from(s_9_11).unwrap());
        // D s_9_32: call Elem_set(s_9_28, s_9_30, s_9_31, s_9_27)
        let s_9_32: Bits = Elem_set(state, tracer, s_9_28, s_9_30, s_9_31, s_9_27);
        // D s_9_33: cast reint s_9_32 -> u64
        let s_9_33: u64 = (s_9_32.value() as u64);
        // D s_9_34: call D_set(s_9_3, s_9_33)
        let s_9_34: () = D_set(state, tracer, s_9_3, s_9_33);
        // N s_9_35: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var e:i64
        let s_10_0: i64 = fn_state.e;
        // C s_10_1: const #1s : i64
        let s_10_1: i64 = 1;
        // D s_10_2: add s_10_0 s_10_1
        let s_10_2: i64 = (s_10_0 + s_10_1);
        // D s_10_3: write-var e <= s_10_2
        fn_state.e = s_10_2;
        // N s_10_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #1s : i
        let s_11_0: i128 = 1;
        // D s_11_1: read-var d:i
        let s_11_1: i128 = fn_state.d;
        // D s_11_2: lsr s_11_1 s_11_0
        let s_11_2: i128 = s_11_1 >> s_11_0;
        // C s_11_3: const #1s : i
        let s_11_3: i128 = 1;
        // D s_11_4: read-var d:i
        let s_11_4: i128 = fn_state.d;
        // D s_11_5: lsr s_11_4 s_11_3
        let s_11_5: i128 = s_11_4 >> s_11_3;
        // D s_11_6: call Q_read(s_11_5)
        let s_11_6: u128 = Q_read(state, tracer, s_11_5);
        // C s_11_7: const #2s : i
        let s_11_7: i128 = 2;
        // D s_11_8: read-var esizeshadow#7518:i64
        let s_11_8: i64 = fn_state.esizeshadow_7518;
        // D s_11_9: cast zx s_11_8 -> i
        let s_11_9: i128 = (i128::try_from(s_11_8).unwrap());
        // D s_11_10: mul s_11_7 s_11_9
        let s_11_10: i128 = ((s_11_7) * (s_11_9));
        // D s_11_11: cast reint s_11_10 -> i64
        let s_11_11: i64 = (s_11_10 as i64);
        // D s_11_12: cast zx s_11_11 -> i
        let s_11_12: i128 = (i128::try_from(s_11_11).unwrap());
        // D s_11_13: cast reint s_11_12 -> i64
        let s_11_13: i64 = (s_11_12 as i64);
        // C s_11_14: const #1s : i
        let s_11_14: i128 = 1;
        // D s_11_15: read-var d:i
        let s_11_15: i128 = fn_state.d;
        // D s_11_16: lsr s_11_15 s_11_14
        let s_11_16: i128 = s_11_15 >> s_11_14;
        // D s_11_17: call Qin_read(s_11_16)
        let s_11_17: u128 = Qin_read(state, tracer, s_11_16);
        // C s_11_18: const #2s : i
        let s_11_18: i128 = 2;
        // D s_11_19: read-var esizeshadow#7518:i64
        let s_11_19: i64 = fn_state.esizeshadow_7518;
        // D s_11_20: cast zx s_11_19 -> i
        let s_11_20: i128 = (i128::try_from(s_11_19).unwrap());
        // D s_11_21: mul s_11_18 s_11_20
        let s_11_21: i128 = ((s_11_18) * (s_11_20));
        // D s_11_22: cast reint s_11_21 -> i64
        let s_11_22: i64 = (s_11_21 as i64);
        // D s_11_23: cast zx s_11_22 -> i
        let s_11_23: i128 = (i128::try_from(s_11_22).unwrap());
        // D s_11_24: cast reint s_11_23 -> i64
        let s_11_24: i64 = (s_11_23 as i64);
        // D s_11_25: cast zx s_11_17 -> bv
        let s_11_25: Bits = Bits::new(s_11_17 as u128, 128u16);
        // D s_11_26: read-var e:i64
        let s_11_26: i64 = fn_state.e;
        // D s_11_27: cast zx s_11_26 -> i
        let s_11_27: i128 = (i128::try_from(s_11_26).unwrap());
        // D s_11_28: cast zx s_11_24 -> i
        let s_11_28: i128 = (i128::try_from(s_11_24).unwrap());
        // D s_11_29: call Elem_read(s_11_25, s_11_27, s_11_28)
        let s_11_29: Bits = Elem_read(state, tracer, s_11_25, s_11_27, s_11_28);
        // D s_11_30: read-var addend:i
        let s_11_30: i128 = fn_state.addend;
        // D s_11_31: cast cvt s_11_30 -> bv
        let s_11_31: Bits = Bits::new(s_11_30 as u128, 128);
        // D s_11_32: add s_11_29 s_11_31
        let s_11_32: Bits = (s_11_29 + s_11_31);
        // D s_11_33: cast zx s_11_6 -> bv
        let s_11_33: Bits = Bits::new(s_11_6 as u128, 128u16);
        // D s_11_34: read-var e:i64
        let s_11_34: i64 = fn_state.e;
        // D s_11_35: cast zx s_11_34 -> i
        let s_11_35: i128 = (i128::try_from(s_11_34).unwrap());
        // D s_11_36: cast zx s_11_13 -> i
        let s_11_36: i128 = (i128::try_from(s_11_13).unwrap());
        // D s_11_37: call Elem_set(s_11_33, s_11_35, s_11_36, s_11_32)
        let s_11_37: Bits = Elem_set(state, tracer, s_11_33, s_11_35, s_11_36, s_11_32);
        // D s_11_38: cast reint s_11_37 -> u128
        let s_11_38: u128 = (s_11_37.value() as u128);
        // D s_11_39: call Q_set(s_11_2, s_11_38)
        let s_11_39: () = Q_set(state, tracer, s_11_2, s_11_38);
        // N s_11_40: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var op1val:i
        let s_12_0: i128 = fn_state.op1val;
        // D s_12_1: read-var op2val:i
        let s_12_1: i128 = fn_state.op2val;
        // D s_12_2: mul s_12_0 s_12_1
        let s_12_2: i128 = ((s_12_0) * (s_12_1));
        // D s_12_3: write-var addend <= s_12_2
        fn_state.addend = s_12_2;
        // N s_12_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var add:u8
        let s_13_0: bool = fn_state.add;
        // N s_13_1: branch s_13_0 b18 b14
        if s_13_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call StandardFPSCRValue(s_14_0)
        let s_14_1: ProductType5c790c8ef59cc8b2 = StandardFPSCRValue(
            state,
            tracer,
            s_14_0,
        );
        // D s_14_2: read-var op1:bv
        let s_14_2: Bits = fn_state.op1;
        // D s_14_3: read-var op2:bv
        let s_14_3: Bits = fn_state.op2;
        // D s_14_4: call FPMul(s_14_2, s_14_3, s_14_1)
        let s_14_4: Bits = FPMul(state, tracer, s_14_2, s_14_3, s_14_1);
        // D s_14_5: write-var ga#355694 <= s_14_4
        fn_state.ga_355694 = s_14_4;
        // N s_14_6: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var ga#355694:bv
        let s_15_0: Bits = fn_state.ga_355694;
        // D s_15_1: call FPNeg(s_15_0)
        let s_15_1: Bits = FPNeg(state, tracer, s_15_0);
        // D s_15_2: write-var fp_addend <= s_15_1
        fn_state.fp_addend = s_15_1;
        // N s_15_3: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var r:i64
        let s_16_0: i64 = fn_state.r;
        // D s_16_1: cast zx s_16_0 -> i
        let s_16_1: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_2: read-var d:i
        let s_16_2: i128 = fn_state.d;
        // D s_16_3: add s_16_2 s_16_1
        let s_16_3: i128 = (s_16_2 + s_16_1);
        // D s_16_4: write-var ga#355704 <= s_16_3
        fn_state.ga_355704 = s_16_3;
        // D s_16_5: read-var r:i64
        let s_16_5: i64 = fn_state.r;
        // D s_16_6: cast zx s_16_5 -> i
        let s_16_6: i128 = (i128::try_from(s_16_5).unwrap());
        // D s_16_7: read-var d:i
        let s_16_7: i128 = fn_state.d;
        // D s_16_8: add s_16_7 s_16_6
        let s_16_8: i128 = (s_16_7 + s_16_6);
        // D s_16_9: call D_read(s_16_8)
        let s_16_9: u64 = D_read(state, tracer, s_16_8);
        // D s_16_10: write-var ga#355701 <= s_16_9
        fn_state.ga_355701 = s_16_9;
        // D s_16_11: read-var esizeshadow#7518:i64
        let s_16_11: i64 = fn_state.esizeshadow_7518;
        // D s_16_12: cast zx s_16_11 -> i
        let s_16_12: i128 = (i128::try_from(s_16_11).unwrap());
        // D s_16_13: cast reint s_16_12 -> i64
        let s_16_13: i64 = (s_16_12 as i64);
        // D s_16_14: write-var ga#355702 <= s_16_13
        fn_state.ga_355702 = s_16_13;
        // D s_16_15: read-var r:i64
        let s_16_15: i64 = fn_state.r;
        // D s_16_16: cast zx s_16_15 -> i
        let s_16_16: i128 = (i128::try_from(s_16_15).unwrap());
        // D s_16_17: read-var d:i
        let s_16_17: i128 = fn_state.d;
        // D s_16_18: add s_16_17 s_16_16
        let s_16_18: i128 = (s_16_17 + s_16_16);
        // D s_16_19: call Din_read(s_16_18)
        let s_16_19: u64 = Din_read(state, tracer, s_16_18);
        // D s_16_20: read-var esizeshadow#7518:i64
        let s_16_20: i64 = fn_state.esizeshadow_7518;
        // D s_16_21: cast zx s_16_20 -> i
        let s_16_21: i128 = (i128::try_from(s_16_20).unwrap());
        // D s_16_22: cast reint s_16_21 -> i64
        let s_16_22: i64 = (s_16_21 as i64);
        // D s_16_23: cast zx s_16_19 -> bv
        let s_16_23: Bits = Bits::new(s_16_19 as u128, 64u16);
        // D s_16_24: read-var e:i64
        let s_16_24: i64 = fn_state.e;
        // D s_16_25: cast zx s_16_24 -> i
        let s_16_25: i128 = (i128::try_from(s_16_24).unwrap());
        // D s_16_26: cast zx s_16_22 -> i
        let s_16_26: i128 = (i128::try_from(s_16_22).unwrap());
        // D s_16_27: call Elem_read(s_16_23, s_16_25, s_16_26)
        let s_16_27: Bits = Elem_read(state, tracer, s_16_23, s_16_25, s_16_26);
        // C s_16_28: const #() : ()
        let s_16_28: () = ();
        // S s_16_29: call StandardFPSCRValue(s_16_28)
        let s_16_29: ProductType5c790c8ef59cc8b2 = StandardFPSCRValue(
            state,
            tracer,
            s_16_28,
        );
        // D s_16_30: read-var fp_addend:bv
        let s_16_30: Bits = fn_state.fp_addend;
        // D s_16_31: call FPAdd(s_16_27, s_16_30, s_16_29)
        let s_16_31: Bits = FPAdd(state, tracer, s_16_27, s_16_30, s_16_29);
        // D s_16_32: write-var ga#355703 <= s_16_31
        fn_state.ga_355703 = s_16_31;
        // N s_16_33: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var ga#355701:u64
        let s_17_0: u64 = fn_state.ga_355701;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 64u16);
        // D s_17_2: read-var e:i64
        let s_17_2: i64 = fn_state.e;
        // D s_17_3: cast zx s_17_2 -> i
        let s_17_3: i128 = (i128::try_from(s_17_2).unwrap());
        // D s_17_4: read-var ga#355702:i64
        let s_17_4: i64 = fn_state.ga_355702;
        // D s_17_5: cast zx s_17_4 -> i
        let s_17_5: i128 = (i128::try_from(s_17_4).unwrap());
        // D s_17_6: read-var ga#355703:bv
        let s_17_6: Bits = fn_state.ga_355703;
        // D s_17_7: call Elem_set(s_17_1, s_17_3, s_17_5, s_17_6)
        let s_17_7: Bits = Elem_set(state, tracer, s_17_1, s_17_3, s_17_5, s_17_6);
        // D s_17_8: cast reint s_17_7 -> u64
        let s_17_8: u64 = (s_17_7.value() as u64);
        // D s_17_9: read-var ga#355704:i
        let s_17_9: i128 = fn_state.ga_355704;
        // D s_17_10: call D_set(s_17_9, s_17_8)
        let s_17_10: () = D_set(state, tracer, s_17_9, s_17_8);
        // N s_17_11: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call StandardFPSCRValue(s_18_0)
        let s_18_1: ProductType5c790c8ef59cc8b2 = StandardFPSCRValue(
            state,
            tracer,
            s_18_0,
        );
        // D s_18_2: read-var op1:bv
        let s_18_2: Bits = fn_state.op1;
        // D s_18_3: read-var op2:bv
        let s_18_3: Bits = fn_state.op2;
        // D s_18_4: call FPMul(s_18_2, s_18_3, s_18_1)
        let s_18_4: Bits = FPMul(state, tracer, s_18_2, s_18_3, s_18_1);
        // D s_18_5: write-var fp_addend <= s_18_4
        fn_state.fp_addend = s_18_4;
        // N s_18_6: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var r:i64
        let s_19_0: i64 = fn_state.r;
        // C s_19_1: const #1s : i64
        let s_19_1: i64 = 1;
        // D s_19_2: add s_19_0 s_19_1
        let s_19_2: i64 = (s_19_0 + s_19_1);
        // D s_19_3: write-var r <= s_19_2
        fn_state.r = s_19_2;
        // N s_19_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: return
        return;
    }
}
