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
use Elem_set::*;
use CheckAdvSIMDEnabled::*;
use integer_subrange::*;
use Q_set::*;
use Din_read::*;
use asl_Int::*;
use Elem_read::*;
use Qin_read::*;
use common::*;
pub fn execute_aarch32_instrs_VADDL_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d__arg: i64,
    elements: i128,
    esize: i64,
    is_vaddw: bool,
    m: i64,
    n: i64,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        esizeshadow_7364: i64,
        gs_306392: i64,
        d: i128,
        op1: i128,
        d__arg: i64,
        elements: i128,
        esize: i64,
        is_vaddw: bool,
        m: i64,
        n: i64,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        d__arg,
        elements,
        esize,
        is_vaddw,
        m,
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
        // D s_0_3: write-var esizeshadow#7364 <= s_0_2
        fn_state.esizeshadow_7364 = s_0_2;
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
        // C s_1_0: const #0s : i64
        let s_1_0: i64 = 0;
        // C s_1_1: const #1s : i
        let s_1_1: i128 = 1;
        // D s_1_2: read-var elements:i
        let s_1_2: i128 = fn_state.elements;
        // D s_1_3: sub s_1_2 s_1_1
        let s_1_3: i128 = ((s_1_2) - (s_1_1));
        // D s_1_4: cast reint s_1_3 -> i64
        let s_1_4: i64 = (s_1_3 as i64);
        // D s_1_5: write-var gs#306392 <= s_1_4
        fn_state.gs_306392 = s_1_4;
        // D s_1_6: write-var e <= s_1_0
        fn_state.e = s_1_0;
        // N s_1_7: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#306392:i64
        let s_2_1: i64 = fn_state.gs_306392;
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
        // D s_3_0: read-var is_vaddw:u8
        let s_3_0: bool = fn_state.is_vaddw;
        // N s_3_1: branch s_3_0 b6 b4
        if s_3_0 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var n:i64
        let s_4_0: i64 = fn_state.n;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: call Din_read(s_4_1)
        let s_4_2: u64 = Din_read(state, tracer, s_4_1);
        // D s_4_3: read-var esizeshadow#7364:i64
        let s_4_3: i64 = fn_state.esizeshadow_7364;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: cast reint s_4_4 -> i64
        let s_4_5: i64 = (s_4_4 as i64);
        // D s_4_6: cast zx s_4_2 -> bv
        let s_4_6: Bits = Bits::new(s_4_2 as u128, 64u16);
        // D s_4_7: read-var e:i64
        let s_4_7: i64 = fn_state.e;
        // D s_4_8: cast zx s_4_7 -> i
        let s_4_8: i128 = (i128::try_from(s_4_7).unwrap());
        // D s_4_9: cast zx s_4_5 -> i
        let s_4_9: i128 = (i128::try_from(s_4_5).unwrap());
        // D s_4_10: call Elem_read(s_4_6, s_4_8, s_4_9)
        let s_4_10: Bits = Elem_read(state, tracer, s_4_6, s_4_8, s_4_9);
        // D s_4_11: read-var is_unsigned:u8
        let s_4_11: bool = fn_state.is_unsigned;
        // D s_4_12: call asl_Int(s_4_10, s_4_11)
        let s_4_12: i128 = asl_Int(state, tracer, s_4_10, s_4_11);
        // D s_4_13: write-var op1 <= s_4_12
        fn_state.op1 = s_4_12;
        // N s_4_14: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var op1:i
        let s_5_0: i128 = fn_state.op1;
        // D s_5_1: read-var m:i64
        let s_5_1: i64 = fn_state.m;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: call Din_read(s_5_2)
        let s_5_3: u64 = Din_read(state, tracer, s_5_2);
        // D s_5_4: read-var esizeshadow#7364:i64
        let s_5_4: i64 = fn_state.esizeshadow_7364;
        // D s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_6: cast reint s_5_5 -> i64
        let s_5_6: i64 = (s_5_5 as i64);
        // D s_5_7: cast zx s_5_3 -> bv
        let s_5_7: Bits = Bits::new(s_5_3 as u128, 64u16);
        // D s_5_8: read-var e:i64
        let s_5_8: i64 = fn_state.e;
        // D s_5_9: cast zx s_5_8 -> i
        let s_5_9: i128 = (i128::try_from(s_5_8).unwrap());
        // D s_5_10: cast zx s_5_6 -> i
        let s_5_10: i128 = (i128::try_from(s_5_6).unwrap());
        // D s_5_11: call Elem_read(s_5_7, s_5_9, s_5_10)
        let s_5_11: Bits = Elem_read(state, tracer, s_5_7, s_5_9, s_5_10);
        // D s_5_12: read-var is_unsigned:u8
        let s_5_12: bool = fn_state.is_unsigned;
        // D s_5_13: call asl_Int(s_5_11, s_5_12)
        let s_5_13: i128 = asl_Int(state, tracer, s_5_11, s_5_12);
        // D s_5_14: add s_5_0 s_5_13
        let s_5_14: i128 = (s_5_0 + s_5_13);
        // C s_5_15: const #1s : i
        let s_5_15: i128 = 1;
        // D s_5_16: read-var d:i
        let s_5_16: i128 = fn_state.d;
        // D s_5_17: lsr s_5_16 s_5_15
        let s_5_17: i128 = s_5_16 >> s_5_15;
        // C s_5_18: const #1s : i
        let s_5_18: i128 = 1;
        // D s_5_19: read-var d:i
        let s_5_19: i128 = fn_state.d;
        // D s_5_20: lsr s_5_19 s_5_18
        let s_5_20: i128 = s_5_19 >> s_5_18;
        // D s_5_21: call Q_read(s_5_20)
        let s_5_21: u128 = Q_read(state, tracer, s_5_20);
        // C s_5_22: const #2s : i
        let s_5_22: i128 = 2;
        // D s_5_23: read-var esizeshadow#7364:i64
        let s_5_23: i64 = fn_state.esizeshadow_7364;
        // D s_5_24: cast zx s_5_23 -> i
        let s_5_24: i128 = (i128::try_from(s_5_23).unwrap());
        // D s_5_25: mul s_5_22 s_5_24
        let s_5_25: i128 = ((s_5_22) * (s_5_24));
        // D s_5_26: cast reint s_5_25 -> i64
        let s_5_26: i64 = (s_5_25 as i64);
        // D s_5_27: cast zx s_5_26 -> i
        let s_5_27: i128 = (i128::try_from(s_5_26).unwrap());
        // D s_5_28: cast reint s_5_27 -> i64
        let s_5_28: i64 = (s_5_27 as i64);
        // C s_5_29: const #2s : i
        let s_5_29: i128 = 2;
        // D s_5_30: read-var esizeshadow#7364:i64
        let s_5_30: i64 = fn_state.esizeshadow_7364;
        // D s_5_31: cast zx s_5_30 -> i
        let s_5_31: i128 = (i128::try_from(s_5_30).unwrap());
        // D s_5_32: mul s_5_29 s_5_31
        let s_5_32: i128 = ((s_5_29) * (s_5_31));
        // D s_5_33: cast reint s_5_32 -> i64
        let s_5_33: i64 = (s_5_32 as i64);
        // C s_5_34: const #1s : i
        let s_5_34: i128 = 1;
        // D s_5_35: cast zx s_5_33 -> i
        let s_5_35: i128 = (i128::try_from(s_5_33).unwrap());
        // D s_5_36: sub s_5_35 s_5_34
        let s_5_36: i128 = ((s_5_35) - (s_5_34));
        // D s_5_37: cast reint s_5_36 -> i64
        let s_5_37: i64 = (s_5_36 as i64);
        // C s_5_38: const #0s : i
        let s_5_38: i128 = 0;
        // D s_5_39: cast zx s_5_37 -> i
        let s_5_39: i128 = (i128::try_from(s_5_37).unwrap());
        // D s_5_40: call integer_subrange(s_5_14, s_5_39, s_5_38)
        let s_5_40: Bits = integer_subrange(state, tracer, s_5_14, s_5_39, s_5_38);
        // D s_5_41: cast zx s_5_21 -> bv
        let s_5_41: Bits = Bits::new(s_5_21 as u128, 128u16);
        // D s_5_42: read-var e:i64
        let s_5_42: i64 = fn_state.e;
        // D s_5_43: cast zx s_5_42 -> i
        let s_5_43: i128 = (i128::try_from(s_5_42).unwrap());
        // D s_5_44: cast zx s_5_28 -> i
        let s_5_44: i128 = (i128::try_from(s_5_28).unwrap());
        // D s_5_45: call Elem_set(s_5_41, s_5_43, s_5_44, s_5_40)
        let s_5_45: Bits = Elem_set(state, tracer, s_5_41, s_5_43, s_5_44, s_5_40);
        // D s_5_46: cast reint s_5_45 -> u128
        let s_5_46: u128 = (s_5_45.value() as u128);
        // D s_5_47: call Q_set(s_5_17, s_5_46)
        let s_5_47: () = Q_set(state, tracer, s_5_17, s_5_46);
        // D s_5_48: read-var e:i64
        let s_5_48: i64 = fn_state.e;
        // C s_5_49: const #1s : i64
        let s_5_49: i64 = 1;
        // D s_5_50: add s_5_48 s_5_49
        let s_5_50: i64 = (s_5_48 + s_5_49);
        // D s_5_51: write-var e <= s_5_50
        fn_state.e = s_5_50;
        // N s_5_52: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #1s : i64
        let s_6_0: i64 = 1;
        // D s_6_1: read-var n:i64
        let s_6_1: i64 = fn_state.n;
        // D s_6_2: lsr s_6_1 s_6_0
        let s_6_2: i64 = s_6_1 >> s_6_0;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: call Qin_read(s_6_3)
        let s_6_4: u128 = Qin_read(state, tracer, s_6_3);
        // C s_6_5: const #2s : i
        let s_6_5: i128 = 2;
        // D s_6_6: read-var esizeshadow#7364:i64
        let s_6_6: i64 = fn_state.esizeshadow_7364;
        // D s_6_7: cast zx s_6_6 -> i
        let s_6_7: i128 = (i128::try_from(s_6_6).unwrap());
        // D s_6_8: mul s_6_5 s_6_7
        let s_6_8: i128 = ((s_6_5) * (s_6_7));
        // D s_6_9: cast reint s_6_8 -> i64
        let s_6_9: i64 = (s_6_8 as i64);
        // D s_6_10: cast zx s_6_9 -> i
        let s_6_10: i128 = (i128::try_from(s_6_9).unwrap());
        // D s_6_11: cast reint s_6_10 -> i64
        let s_6_11: i64 = (s_6_10 as i64);
        // D s_6_12: cast zx s_6_4 -> bv
        let s_6_12: Bits = Bits::new(s_6_4 as u128, 128u16);
        // D s_6_13: read-var e:i64
        let s_6_13: i64 = fn_state.e;
        // D s_6_14: cast zx s_6_13 -> i
        let s_6_14: i128 = (i128::try_from(s_6_13).unwrap());
        // D s_6_15: cast zx s_6_11 -> i
        let s_6_15: i128 = (i128::try_from(s_6_11).unwrap());
        // D s_6_16: call Elem_read(s_6_12, s_6_14, s_6_15)
        let s_6_16: Bits = Elem_read(state, tracer, s_6_12, s_6_14, s_6_15);
        // D s_6_17: read-var is_unsigned:u8
        let s_6_17: bool = fn_state.is_unsigned;
        // D s_6_18: call asl_Int(s_6_16, s_6_17)
        let s_6_18: i128 = asl_Int(state, tracer, s_6_16, s_6_17);
        // D s_6_19: write-var op1 <= s_6_18
        fn_state.op1 = s_6_18;
        // N s_6_20: jump b5
        return block_5(state, tracer, fn_state);
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
