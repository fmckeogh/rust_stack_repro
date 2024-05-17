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
use CheckAdvSIMDEnabled::*;
use integer_subrange::*;
use D_set::*;
use asl_Int::*;
use Elem_read::*;
use D_read::*;
use fdiv_int::*;
use common::*;
pub fn execute_aarch32_instrs_VPMAX_i_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    elements: i128,
    esize: i64,
    m: i64,
    maximum: bool,
    n: i64,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        esizeshadow_7645: i64,
        e: i64,
        op2: i128,
        dest: u64,
        op1: i128,
        gs_315162: i64,
        result: i128,
        h: i128,
        d: i64,
        elements: i128,
        esize: i64,
        m: i64,
        maximum: bool,
        n: i64,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        d,
        elements,
        esize,
        m,
        maximum,
        n,
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
        // D s_0_3: write-var esizeshadow#7645 <= s_0_2
        fn_state.esizeshadow_7645 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckAdvSIMDEnabled(s_0_4)
        let s_0_5: () = CheckAdvSIMDEnabled(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #2s : i
        let s_1_0: i128 = 2;
        // D s_1_1: read-var elements:i
        let s_1_1: i128 = fn_state.elements;
        // D s_1_2: call fdiv_int(s_1_1, s_1_0)
        let s_1_2: i128 = fdiv_int(state, tracer, s_1_1, s_1_0);
        // D s_1_3: write-var h <= s_1_2
        fn_state.h = s_1_2;
        // C s_1_4: const #0s : i64
        let s_1_4: i64 = 0;
        // C s_1_5: const #1s : i
        let s_1_5: i128 = 1;
        // D s_1_6: read-var h:i
        let s_1_6: i128 = fn_state.h;
        // D s_1_7: sub s_1_6 s_1_5
        let s_1_7: i128 = ((s_1_6) - (s_1_5));
        // D s_1_8: cast reint s_1_7 -> i64
        let s_1_8: i64 = (s_1_7 as i64);
        // D s_1_9: write-var gs#315162 <= s_1_8
        fn_state.gs_315162 = s_1_8;
        // D s_1_10: write-var e <= s_1_4
        fn_state.e = s_1_4;
        // N s_1_11: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#315162:i64
        let s_2_1: i64 = fn_state.gs_315162;
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
        // D s_3_2: call D_read(s_3_1)
        let s_3_2: u64 = D_read(state, tracer, s_3_1);
        // C s_3_3: const #2s : i
        let s_3_3: i128 = 2;
        // D s_3_4: read-var e:i64
        let s_3_4: i64 = fn_state.e;
        // D s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_6: mul s_3_3 s_3_5
        let s_3_6: i128 = ((s_3_3) * (s_3_5));
        // D s_3_7: read-var esizeshadow#7645:i64
        let s_3_7: i64 = fn_state.esizeshadow_7645;
        // D s_3_8: cast zx s_3_7 -> i
        let s_3_8: i128 = (i128::try_from(s_3_7).unwrap());
        // D s_3_9: cast reint s_3_8 -> i64
        let s_3_9: i64 = (s_3_8 as i64);
        // D s_3_10: cast zx s_3_2 -> bv
        let s_3_10: Bits = Bits::new(s_3_2 as u128, 64u16);
        // D s_3_11: cast zx s_3_9 -> i
        let s_3_11: i128 = (i128::try_from(s_3_9).unwrap());
        // D s_3_12: call Elem_read(s_3_10, s_3_6, s_3_11)
        let s_3_12: Bits = Elem_read(state, tracer, s_3_10, s_3_6, s_3_11);
        // D s_3_13: read-var is_unsigned:u8
        let s_3_13: bool = fn_state.is_unsigned;
        // D s_3_14: call asl_Int(s_3_12, s_3_13)
        let s_3_14: i128 = asl_Int(state, tracer, s_3_12, s_3_13);
        // D s_3_15: write-var op1 <= s_3_14
        fn_state.op1 = s_3_14;
        // D s_3_16: read-var n:i64
        let s_3_16: i64 = fn_state.n;
        // D s_3_17: cast zx s_3_16 -> i
        let s_3_17: i128 = (i128::try_from(s_3_16).unwrap());
        // D s_3_18: call D_read(s_3_17)
        let s_3_18: u64 = D_read(state, tracer, s_3_17);
        // C s_3_19: const #2s : i
        let s_3_19: i128 = 2;
        // D s_3_20: read-var e:i64
        let s_3_20: i64 = fn_state.e;
        // D s_3_21: cast zx s_3_20 -> i
        let s_3_21: i128 = (i128::try_from(s_3_20).unwrap());
        // D s_3_22: mul s_3_19 s_3_21
        let s_3_22: i128 = ((s_3_19) * (s_3_21));
        // C s_3_23: const #1s : i
        let s_3_23: i128 = 1;
        // D s_3_24: add s_3_22 s_3_23
        let s_3_24: i128 = (s_3_22 + s_3_23);
        // D s_3_25: read-var esizeshadow#7645:i64
        let s_3_25: i64 = fn_state.esizeshadow_7645;
        // D s_3_26: cast zx s_3_25 -> i
        let s_3_26: i128 = (i128::try_from(s_3_25).unwrap());
        // D s_3_27: cast reint s_3_26 -> i64
        let s_3_27: i64 = (s_3_26 as i64);
        // D s_3_28: cast zx s_3_18 -> bv
        let s_3_28: Bits = Bits::new(s_3_18 as u128, 64u16);
        // D s_3_29: cast zx s_3_27 -> i
        let s_3_29: i128 = (i128::try_from(s_3_27).unwrap());
        // D s_3_30: call Elem_read(s_3_28, s_3_24, s_3_29)
        let s_3_30: Bits = Elem_read(state, tracer, s_3_28, s_3_24, s_3_29);
        // D s_3_31: read-var is_unsigned:u8
        let s_3_31: bool = fn_state.is_unsigned;
        // D s_3_32: call asl_Int(s_3_30, s_3_31)
        let s_3_32: i128 = asl_Int(state, tracer, s_3_30, s_3_31);
        // D s_3_33: write-var op2 <= s_3_32
        fn_state.op2 = s_3_32;
        // D s_3_34: read-var maximum:u8
        let s_3_34: bool = fn_state.maximum;
        // N s_3_35: branch s_3_34 b9 b4
        if s_3_34 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var op1:i
        let s_4_0: i128 = fn_state.op1;
        // D s_4_1: read-var op2:i
        let s_4_1: i128 = fn_state.op2;
        // D s_4_2: cmp-lt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) < (s_4_1));
        // D s_4_3: select s_4_2 s_4_0 s_4_1
        let s_4_3: i128 = if s_4_2 { s_4_0 } else { s_4_1 };
        // D s_4_4: write-var result <= s_4_3
        fn_state.result = s_4_3;
        // N s_4_5: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var esizeshadow#7645:i64
        let s_5_0: i64 = fn_state.esizeshadow_7645;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: cast reint s_5_1 -> i64
        let s_5_2: i64 = (s_5_1 as i64);
        // C s_5_3: const #1s : i
        let s_5_3: i128 = 1;
        // D s_5_4: read-var esizeshadow#7645:i64
        let s_5_4: i64 = fn_state.esizeshadow_7645;
        // D s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_6: sub s_5_5 s_5_3
        let s_5_6: i128 = ((s_5_5) - (s_5_3));
        // D s_5_7: cast reint s_5_6 -> i64
        let s_5_7: i64 = (s_5_6 as i64);
        // C s_5_8: const #0s : i
        let s_5_8: i128 = 0;
        // D s_5_9: cast zx s_5_7 -> i
        let s_5_9: i128 = (i128::try_from(s_5_7).unwrap());
        // D s_5_10: read-var result:i
        let s_5_10: i128 = fn_state.result;
        // D s_5_11: call integer_subrange(s_5_10, s_5_9, s_5_8)
        let s_5_11: Bits = integer_subrange(state, tracer, s_5_10, s_5_9, s_5_8);
        // D s_5_12: read-var dest:u64
        let s_5_12: u64 = fn_state.dest;
        // D s_5_13: cast zx s_5_12 -> bv
        let s_5_13: Bits = Bits::new(s_5_12 as u128, 64u16);
        // D s_5_14: read-var e:i64
        let s_5_14: i64 = fn_state.e;
        // D s_5_15: cast zx s_5_14 -> i
        let s_5_15: i128 = (i128::try_from(s_5_14).unwrap());
        // D s_5_16: cast zx s_5_2 -> i
        let s_5_16: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_17: call Elem_set(s_5_13, s_5_15, s_5_16, s_5_11)
        let s_5_17: Bits = Elem_set(state, tracer, s_5_13, s_5_15, s_5_16, s_5_11);
        // D s_5_18: cast reint s_5_17 -> u64
        let s_5_18: u64 = (s_5_17.value() as u64);
        // D s_5_19: write-var dest <= s_5_18
        fn_state.dest = s_5_18;
        // D s_5_20: read-var m:i64
        let s_5_20: i64 = fn_state.m;
        // D s_5_21: cast zx s_5_20 -> i
        let s_5_21: i128 = (i128::try_from(s_5_20).unwrap());
        // D s_5_22: call D_read(s_5_21)
        let s_5_22: u64 = D_read(state, tracer, s_5_21);
        // C s_5_23: const #2s : i
        let s_5_23: i128 = 2;
        // D s_5_24: read-var e:i64
        let s_5_24: i64 = fn_state.e;
        // D s_5_25: cast zx s_5_24 -> i
        let s_5_25: i128 = (i128::try_from(s_5_24).unwrap());
        // D s_5_26: mul s_5_23 s_5_25
        let s_5_26: i128 = ((s_5_23) * (s_5_25));
        // D s_5_27: read-var esizeshadow#7645:i64
        let s_5_27: i64 = fn_state.esizeshadow_7645;
        // D s_5_28: cast zx s_5_27 -> i
        let s_5_28: i128 = (i128::try_from(s_5_27).unwrap());
        // D s_5_29: cast reint s_5_28 -> i64
        let s_5_29: i64 = (s_5_28 as i64);
        // D s_5_30: cast zx s_5_22 -> bv
        let s_5_30: Bits = Bits::new(s_5_22 as u128, 64u16);
        // D s_5_31: cast zx s_5_29 -> i
        let s_5_31: i128 = (i128::try_from(s_5_29).unwrap());
        // D s_5_32: call Elem_read(s_5_30, s_5_26, s_5_31)
        let s_5_32: Bits = Elem_read(state, tracer, s_5_30, s_5_26, s_5_31);
        // D s_5_33: read-var is_unsigned:u8
        let s_5_33: bool = fn_state.is_unsigned;
        // D s_5_34: call asl_Int(s_5_32, s_5_33)
        let s_5_34: i128 = asl_Int(state, tracer, s_5_32, s_5_33);
        // D s_5_35: write-var op1 <= s_5_34
        fn_state.op1 = s_5_34;
        // D s_5_36: read-var m:i64
        let s_5_36: i64 = fn_state.m;
        // D s_5_37: cast zx s_5_36 -> i
        let s_5_37: i128 = (i128::try_from(s_5_36).unwrap());
        // D s_5_38: call D_read(s_5_37)
        let s_5_38: u64 = D_read(state, tracer, s_5_37);
        // C s_5_39: const #2s : i
        let s_5_39: i128 = 2;
        // D s_5_40: read-var e:i64
        let s_5_40: i64 = fn_state.e;
        // D s_5_41: cast zx s_5_40 -> i
        let s_5_41: i128 = (i128::try_from(s_5_40).unwrap());
        // D s_5_42: mul s_5_39 s_5_41
        let s_5_42: i128 = ((s_5_39) * (s_5_41));
        // C s_5_43: const #1s : i
        let s_5_43: i128 = 1;
        // D s_5_44: add s_5_42 s_5_43
        let s_5_44: i128 = (s_5_42 + s_5_43);
        // D s_5_45: read-var esizeshadow#7645:i64
        let s_5_45: i64 = fn_state.esizeshadow_7645;
        // D s_5_46: cast zx s_5_45 -> i
        let s_5_46: i128 = (i128::try_from(s_5_45).unwrap());
        // D s_5_47: cast reint s_5_46 -> i64
        let s_5_47: i64 = (s_5_46 as i64);
        // D s_5_48: cast zx s_5_38 -> bv
        let s_5_48: Bits = Bits::new(s_5_38 as u128, 64u16);
        // D s_5_49: cast zx s_5_47 -> i
        let s_5_49: i128 = (i128::try_from(s_5_47).unwrap());
        // D s_5_50: call Elem_read(s_5_48, s_5_44, s_5_49)
        let s_5_50: Bits = Elem_read(state, tracer, s_5_48, s_5_44, s_5_49);
        // D s_5_51: read-var is_unsigned:u8
        let s_5_51: bool = fn_state.is_unsigned;
        // D s_5_52: call asl_Int(s_5_50, s_5_51)
        let s_5_52: i128 = asl_Int(state, tracer, s_5_50, s_5_51);
        // D s_5_53: write-var op2 <= s_5_52
        fn_state.op2 = s_5_52;
        // D s_5_54: read-var maximum:u8
        let s_5_54: bool = fn_state.maximum;
        // N s_5_55: branch s_5_54 b8 b6
        if s_5_54 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var op1:i
        let s_6_0: i128 = fn_state.op1;
        // D s_6_1: read-var op2:i
        let s_6_1: i128 = fn_state.op2;
        // D s_6_2: cmp-lt s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) < (s_6_1));
        // D s_6_3: select s_6_2 s_6_0 s_6_1
        let s_6_3: i128 = if s_6_2 { s_6_0 } else { s_6_1 };
        // D s_6_4: write-var result <= s_6_3
        fn_state.result = s_6_3;
        // N s_6_5: jump b7
        return block_7(state, tracer, fn_state);
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
        // D s_7_2: read-var h:i
        let s_7_2: i128 = fn_state.h;
        // D s_7_3: add s_7_1 s_7_2
        let s_7_3: i128 = (s_7_1 + s_7_2);
        // D s_7_4: read-var esizeshadow#7645:i64
        let s_7_4: i64 = fn_state.esizeshadow_7645;
        // D s_7_5: cast zx s_7_4 -> i
        let s_7_5: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_6: cast reint s_7_5 -> i64
        let s_7_6: i64 = (s_7_5 as i64);
        // C s_7_7: const #1s : i
        let s_7_7: i128 = 1;
        // D s_7_8: read-var esizeshadow#7645:i64
        let s_7_8: i64 = fn_state.esizeshadow_7645;
        // D s_7_9: cast zx s_7_8 -> i
        let s_7_9: i128 = (i128::try_from(s_7_8).unwrap());
        // D s_7_10: sub s_7_9 s_7_7
        let s_7_10: i128 = ((s_7_9) - (s_7_7));
        // D s_7_11: cast reint s_7_10 -> i64
        let s_7_11: i64 = (s_7_10 as i64);
        // C s_7_12: const #0s : i
        let s_7_12: i128 = 0;
        // D s_7_13: cast zx s_7_11 -> i
        let s_7_13: i128 = (i128::try_from(s_7_11).unwrap());
        // D s_7_14: read-var result:i
        let s_7_14: i128 = fn_state.result;
        // D s_7_15: call integer_subrange(s_7_14, s_7_13, s_7_12)
        let s_7_15: Bits = integer_subrange(state, tracer, s_7_14, s_7_13, s_7_12);
        // D s_7_16: read-var dest:u64
        let s_7_16: u64 = fn_state.dest;
        // D s_7_17: cast zx s_7_16 -> bv
        let s_7_17: Bits = Bits::new(s_7_16 as u128, 64u16);
        // D s_7_18: cast zx s_7_6 -> i
        let s_7_18: i128 = (i128::try_from(s_7_6).unwrap());
        // D s_7_19: call Elem_set(s_7_17, s_7_3, s_7_18, s_7_15)
        let s_7_19: Bits = Elem_set(state, tracer, s_7_17, s_7_3, s_7_18, s_7_15);
        // D s_7_20: cast reint s_7_19 -> u64
        let s_7_20: u64 = (s_7_19.value() as u64);
        // D s_7_21: write-var dest <= s_7_20
        fn_state.dest = s_7_20;
        // D s_7_22: read-var e:i64
        let s_7_22: i64 = fn_state.e;
        // C s_7_23: const #1s : i64
        let s_7_23: i64 = 1;
        // D s_7_24: add s_7_22 s_7_23
        let s_7_24: i64 = (s_7_22 + s_7_23);
        // D s_7_25: write-var e <= s_7_24
        fn_state.e = s_7_24;
        // N s_7_26: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var op1:i
        let s_8_0: i128 = fn_state.op1;
        // D s_8_1: read-var op2:i
        let s_8_1: i128 = fn_state.op2;
        // D s_8_2: cmp-gt s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) > (s_8_1));
        // D s_8_3: select s_8_2 s_8_0 s_8_1
        let s_8_3: i128 = if s_8_2 { s_8_0 } else { s_8_1 };
        // D s_8_4: write-var result <= s_8_3
        fn_state.result = s_8_3;
        // N s_8_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var op1:i
        let s_9_0: i128 = fn_state.op1;
        // D s_9_1: read-var op2:i
        let s_9_1: i128 = fn_state.op2;
        // D s_9_2: cmp-gt s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) > (s_9_1));
        // D s_9_3: select s_9_2 s_9_0 s_9_1
        let s_9_3: i128 = if s_9_2 { s_9_0 } else { s_9_1 };
        // D s_9_4: write-var result <= s_9_3
        fn_state.result = s_9_3;
        // N s_9_5: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var d:i64
        let s_10_0: i64 = fn_state.d;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: read-var dest:u64
        let s_10_2: u64 = fn_state.dest;
        // D s_10_3: call D_set(s_10_1, s_10_2)
        let s_10_3: () = D_set(state, tracer, s_10_1, s_10_2);
        // N s_10_4: return
        return;
    }
}
