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
use StandardFPSCRValue::*;
use Q_read::*;
use Elem_set::*;
use FPMul::*;
use CheckAdvSIMDEnabled::*;
use integer_subrange::*;
use Q_set::*;
use D_set::*;
use Din_read::*;
use asl_Int::*;
use Elem_read::*;
use D_read::*;
use common::*;
pub fn execute_aarch32_instrs_VMUL_s_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
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
        e: i64,
        gs_313840: i64,
        op2val: i128,
        ga_356892: i128,
        ga_356889: u64,
        op1val: i128,
        esizeshadow_7589: i64,
        op2: Bits,
        ga_356891: Bits,
        d: i128,
        gs_313846: i64,
        op1: Bits,
        ga_356890: i64,
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
        // D s_0_3: write-var esizeshadow#7589 <= s_0_2
        fn_state.esizeshadow_7589 = s_0_2;
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
        // D s_1_3: read-var esizeshadow#7589:i64
        let s_1_3: i64 = fn_state.esizeshadow_7589;
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
        // D s_1_22: write-var gs#313840 <= s_1_21
        fn_state.gs_313840 = s_1_21;
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
        // D s_2_1: read-var gs#313840:i64
        let s_2_1: i64 = fn_state.gs_313840;
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
        // D s_3_6: write-var gs#313846 <= s_3_5
        fn_state.gs_313846 = s_3_5;
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
        // D s_4_1: read-var gs#313846:i64
        let s_4_1: i64 = fn_state.gs_313846;
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
        // D s_5_8: read-var esizeshadow#7589:i64
        let s_5_8: i64 = fn_state.esizeshadow_7589;
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
        // N s_5_22: branch s_5_21 b10 b6
        if s_5_21 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var long_destination:u8
        let s_6_0: bool = fn_state.long_destination;
        // N s_6_1: branch s_6_0 b9 b7
        if s_6_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var r:i64
        let s_7_0: i64 = fn_state.r;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: read-var d:i
        let s_7_2: i128 = fn_state.d;
        // D s_7_3: add s_7_2 s_7_1
        let s_7_3: i128 = (s_7_2 + s_7_1);
        // D s_7_4: read-var r:i64
        let s_7_4: i64 = fn_state.r;
        // D s_7_5: cast zx s_7_4 -> i
        let s_7_5: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_6: read-var d:i
        let s_7_6: i128 = fn_state.d;
        // D s_7_7: add s_7_6 s_7_5
        let s_7_7: i128 = (s_7_6 + s_7_5);
        // D s_7_8: call D_read(s_7_7)
        let s_7_8: u64 = D_read(state, tracer, s_7_7);
        // D s_7_9: read-var esizeshadow#7589:i64
        let s_7_9: i64 = fn_state.esizeshadow_7589;
        // D s_7_10: cast zx s_7_9 -> i
        let s_7_10: i128 = (i128::try_from(s_7_9).unwrap());
        // D s_7_11: cast reint s_7_10 -> i64
        let s_7_11: i64 = (s_7_10 as i64);
        // D s_7_12: read-var op1val:i
        let s_7_12: i128 = fn_state.op1val;
        // D s_7_13: read-var op2val:i
        let s_7_13: i128 = fn_state.op2val;
        // D s_7_14: mul s_7_12 s_7_13
        let s_7_14: i128 = ((s_7_12) * (s_7_13));
        // C s_7_15: const #1s : i
        let s_7_15: i128 = 1;
        // D s_7_16: read-var esizeshadow#7589:i64
        let s_7_16: i64 = fn_state.esizeshadow_7589;
        // D s_7_17: cast zx s_7_16 -> i
        let s_7_17: i128 = (i128::try_from(s_7_16).unwrap());
        // D s_7_18: sub s_7_17 s_7_15
        let s_7_18: i128 = ((s_7_17) - (s_7_15));
        // D s_7_19: cast reint s_7_18 -> i64
        let s_7_19: i64 = (s_7_18 as i64);
        // C s_7_20: const #0s : i
        let s_7_20: i128 = 0;
        // D s_7_21: cast zx s_7_19 -> i
        let s_7_21: i128 = (i128::try_from(s_7_19).unwrap());
        // D s_7_22: call integer_subrange(s_7_14, s_7_21, s_7_20)
        let s_7_22: Bits = integer_subrange(state, tracer, s_7_14, s_7_21, s_7_20);
        // D s_7_23: cast zx s_7_8 -> bv
        let s_7_23: Bits = Bits::new(s_7_8 as u128, 64u16);
        // D s_7_24: read-var e:i64
        let s_7_24: i64 = fn_state.e;
        // D s_7_25: cast zx s_7_24 -> i
        let s_7_25: i128 = (i128::try_from(s_7_24).unwrap());
        // D s_7_26: cast zx s_7_11 -> i
        let s_7_26: i128 = (i128::try_from(s_7_11).unwrap());
        // D s_7_27: call Elem_set(s_7_23, s_7_25, s_7_26, s_7_22)
        let s_7_27: Bits = Elem_set(state, tracer, s_7_23, s_7_25, s_7_26, s_7_22);
        // D s_7_28: cast reint s_7_27 -> u64
        let s_7_28: u64 = (s_7_27.value() as u64);
        // D s_7_29: call D_set(s_7_3, s_7_28)
        let s_7_29: () = D_set(state, tracer, s_7_3, s_7_28);
        // N s_7_30: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var e:i64
        let s_8_0: i64 = fn_state.e;
        // C s_8_1: const #1s : i64
        let s_8_1: i64 = 1;
        // D s_8_2: add s_8_0 s_8_1
        let s_8_2: i64 = (s_8_0 + s_8_1);
        // D s_8_3: write-var e <= s_8_2
        fn_state.e = s_8_2;
        // N s_8_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #1s : i
        let s_9_0: i128 = 1;
        // D s_9_1: read-var d:i
        let s_9_1: i128 = fn_state.d;
        // D s_9_2: lsr s_9_1 s_9_0
        let s_9_2: i128 = s_9_1 >> s_9_0;
        // C s_9_3: const #1s : i
        let s_9_3: i128 = 1;
        // D s_9_4: read-var d:i
        let s_9_4: i128 = fn_state.d;
        // D s_9_5: lsr s_9_4 s_9_3
        let s_9_5: i128 = s_9_4 >> s_9_3;
        // D s_9_6: call Q_read(s_9_5)
        let s_9_6: u128 = Q_read(state, tracer, s_9_5);
        // C s_9_7: const #2s : i
        let s_9_7: i128 = 2;
        // D s_9_8: read-var esizeshadow#7589:i64
        let s_9_8: i64 = fn_state.esizeshadow_7589;
        // D s_9_9: cast zx s_9_8 -> i
        let s_9_9: i128 = (i128::try_from(s_9_8).unwrap());
        // D s_9_10: mul s_9_7 s_9_9
        let s_9_10: i128 = ((s_9_7) * (s_9_9));
        // D s_9_11: cast reint s_9_10 -> i64
        let s_9_11: i64 = (s_9_10 as i64);
        // D s_9_12: cast zx s_9_11 -> i
        let s_9_12: i128 = (i128::try_from(s_9_11).unwrap());
        // D s_9_13: cast reint s_9_12 -> i64
        let s_9_13: i64 = (s_9_12 as i64);
        // D s_9_14: read-var op1val:i
        let s_9_14: i128 = fn_state.op1val;
        // D s_9_15: read-var op2val:i
        let s_9_15: i128 = fn_state.op2val;
        // D s_9_16: mul s_9_14 s_9_15
        let s_9_16: i128 = ((s_9_14) * (s_9_15));
        // C s_9_17: const #2s : i
        let s_9_17: i128 = 2;
        // D s_9_18: read-var esizeshadow#7589:i64
        let s_9_18: i64 = fn_state.esizeshadow_7589;
        // D s_9_19: cast zx s_9_18 -> i
        let s_9_19: i128 = (i128::try_from(s_9_18).unwrap());
        // D s_9_20: mul s_9_17 s_9_19
        let s_9_20: i128 = ((s_9_17) * (s_9_19));
        // D s_9_21: cast reint s_9_20 -> i64
        let s_9_21: i64 = (s_9_20 as i64);
        // C s_9_22: const #1s : i
        let s_9_22: i128 = 1;
        // D s_9_23: cast zx s_9_21 -> i
        let s_9_23: i128 = (i128::try_from(s_9_21).unwrap());
        // D s_9_24: sub s_9_23 s_9_22
        let s_9_24: i128 = ((s_9_23) - (s_9_22));
        // D s_9_25: cast reint s_9_24 -> i64
        let s_9_25: i64 = (s_9_24 as i64);
        // C s_9_26: const #0s : i
        let s_9_26: i128 = 0;
        // D s_9_27: cast zx s_9_25 -> i
        let s_9_27: i128 = (i128::try_from(s_9_25).unwrap());
        // D s_9_28: call integer_subrange(s_9_16, s_9_27, s_9_26)
        let s_9_28: Bits = integer_subrange(state, tracer, s_9_16, s_9_27, s_9_26);
        // D s_9_29: cast zx s_9_6 -> bv
        let s_9_29: Bits = Bits::new(s_9_6 as u128, 128u16);
        // D s_9_30: read-var e:i64
        let s_9_30: i64 = fn_state.e;
        // D s_9_31: cast zx s_9_30 -> i
        let s_9_31: i128 = (i128::try_from(s_9_30).unwrap());
        // D s_9_32: cast zx s_9_13 -> i
        let s_9_32: i128 = (i128::try_from(s_9_13).unwrap());
        // D s_9_33: call Elem_set(s_9_29, s_9_31, s_9_32, s_9_28)
        let s_9_33: Bits = Elem_set(state, tracer, s_9_29, s_9_31, s_9_32, s_9_28);
        // D s_9_34: cast reint s_9_33 -> u128
        let s_9_34: u128 = (s_9_33.value() as u128);
        // D s_9_35: call Q_set(s_9_2, s_9_34)
        let s_9_35: () = Q_set(state, tracer, s_9_2, s_9_34);
        // N s_9_36: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var r:i64
        let s_10_0: i64 = fn_state.r;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: read-var d:i
        let s_10_2: i128 = fn_state.d;
        // D s_10_3: add s_10_2 s_10_1
        let s_10_3: i128 = (s_10_2 + s_10_1);
        // D s_10_4: write-var ga#356892 <= s_10_3
        fn_state.ga_356892 = s_10_3;
        // D s_10_5: read-var r:i64
        let s_10_5: i64 = fn_state.r;
        // D s_10_6: cast zx s_10_5 -> i
        let s_10_6: i128 = (i128::try_from(s_10_5).unwrap());
        // D s_10_7: read-var d:i
        let s_10_7: i128 = fn_state.d;
        // D s_10_8: add s_10_7 s_10_6
        let s_10_8: i128 = (s_10_7 + s_10_6);
        // D s_10_9: call D_read(s_10_8)
        let s_10_9: u64 = D_read(state, tracer, s_10_8);
        // D s_10_10: write-var ga#356889 <= s_10_9
        fn_state.ga_356889 = s_10_9;
        // D s_10_11: read-var esizeshadow#7589:i64
        let s_10_11: i64 = fn_state.esizeshadow_7589;
        // D s_10_12: cast zx s_10_11 -> i
        let s_10_12: i128 = (i128::try_from(s_10_11).unwrap());
        // D s_10_13: cast reint s_10_12 -> i64
        let s_10_13: i64 = (s_10_12 as i64);
        // D s_10_14: write-var ga#356890 <= s_10_13
        fn_state.ga_356890 = s_10_13;
        // C s_10_15: const #() : ()
        let s_10_15: () = ();
        // S s_10_16: call StandardFPSCRValue(s_10_15)
        let s_10_16: ProductType5c790c8ef59cc8b2 = StandardFPSCRValue(
            state,
            tracer,
            s_10_15,
        );
        // D s_10_17: read-var op1:bv
        let s_10_17: Bits = fn_state.op1;
        // D s_10_18: read-var op2:bv
        let s_10_18: Bits = fn_state.op2;
        // D s_10_19: call FPMul(s_10_17, s_10_18, s_10_16)
        let s_10_19: Bits = FPMul(state, tracer, s_10_17, s_10_18, s_10_16);
        // D s_10_20: write-var ga#356891 <= s_10_19
        fn_state.ga_356891 = s_10_19;
        // N s_10_21: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var ga#356889:u64
        let s_11_0: u64 = fn_state.ga_356889;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 64u16);
        // D s_11_2: read-var e:i64
        let s_11_2: i64 = fn_state.e;
        // D s_11_3: cast zx s_11_2 -> i
        let s_11_3: i128 = (i128::try_from(s_11_2).unwrap());
        // D s_11_4: read-var ga#356890:i64
        let s_11_4: i64 = fn_state.ga_356890;
        // D s_11_5: cast zx s_11_4 -> i
        let s_11_5: i128 = (i128::try_from(s_11_4).unwrap());
        // D s_11_6: read-var ga#356891:bv
        let s_11_6: Bits = fn_state.ga_356891;
        // D s_11_7: call Elem_set(s_11_1, s_11_3, s_11_5, s_11_6)
        let s_11_7: Bits = Elem_set(state, tracer, s_11_1, s_11_3, s_11_5, s_11_6);
        // D s_11_8: cast reint s_11_7 -> u64
        let s_11_8: u64 = (s_11_7.value() as u64);
        // D s_11_9: read-var ga#356892:i
        let s_11_9: i128 = fn_state.ga_356892;
        // D s_11_10: call D_set(s_11_9, s_11_8)
        let s_11_10: () = D_set(state, tracer, s_11_9, s_11_8);
        // N s_11_11: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var r:i64
        let s_12_0: i64 = fn_state.r;
        // C s_12_1: const #1s : i64
        let s_12_1: i64 = 1;
        // D s_12_2: add s_12_0 s_12_1
        let s_12_2: i64 = (s_12_0 + s_12_1);
        // D s_12_3: write-var r <= s_12_2
        fn_state.r = s_12_2;
        // N s_12_4: jump b2
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
