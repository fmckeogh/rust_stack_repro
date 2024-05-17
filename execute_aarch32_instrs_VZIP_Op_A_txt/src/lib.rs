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
use u__UNKNOWN_bits::*;
use Elem_set::*;
use Q_read::*;
use CheckAdvSIMDEnabled::*;
use Q_set::*;
use D_set::*;
use Elem_read::*;
use D_read::*;
use common::*;
pub fn execute_aarch32_instrs_VZIP_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    esize: i64,
    m: i64,
    quadword_operation: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        zipped_d: u128,
        zipped_q: u64,
        esizeshadow_7882: i64,
        gs_323062: i64,
        u_9570: i64,
        gs_323043: i64,
        d: i64,
        esize: i64,
        m: i64,
        quadword_operation: bool,
    }
    let fn_state = FunctionState {
        d,
        esize,
        m,
        quadword_operation,
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
        // D s_0_3: write-var esizeshadow#7882 <= s_0_2
        fn_state.esizeshadow_7882 = s_0_2;
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
        // D s_1_0: read-var quadword_operation:u8
        let s_1_0: bool = fn_state.quadword_operation;
        // N s_1_1: branch s_1_0 b8 b2
        if s_1_0 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var d:i64
        let s_2_0: i64 = fn_state.d;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: read-var m:i64
        let s_2_2: i64 = fn_state.m;
        // D s_2_3: cast zx s_2_2 -> i
        let s_2_3: i128 = (i128::try_from(s_2_2).unwrap());
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b7 b3
        if s_2_4 {
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
        // C s_3_0: const #0s : i64
        let s_3_0: i64 = 0;
        // C s_3_1: const #64s : i
        let s_3_1: i128 = 64;
        // D s_3_2: read-var esizeshadow#7882:i64
        let s_3_2: i64 = fn_state.esizeshadow_7882;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: div s_3_1 s_3_3
        let s_3_4: i128 = ((s_3_1) / (s_3_3));
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // C s_3_6: const #1s : i
        let s_3_6: i128 = 1;
        // D s_3_7: cast zx s_3_5 -> i
        let s_3_7: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_8: sub s_3_7 s_3_6
        let s_3_8: i128 = ((s_3_7) - (s_3_6));
        // D s_3_9: cast reint s_3_8 -> i64
        let s_3_9: i64 = (s_3_8 as i64);
        // D s_3_10: write-var gs#323043 <= s_3_9
        fn_state.gs_323043 = s_3_9;
        // D s_3_11: write-var u#9570 <= s_3_0
        fn_state.u_9570 = s_3_0;
        // N s_3_12: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var u#9570:i64
        let s_4_0: i64 = fn_state.u_9570;
        // D s_4_1: read-var gs#323043:i64
        let s_4_1: i64 = fn_state.gs_323043;
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
        // C s_5_0: const #2s : i
        let s_5_0: i128 = 2;
        // D s_5_1: read-var u#9570:i64
        let s_5_1: i64 = fn_state.u_9570;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: mul s_5_0 s_5_2
        let s_5_3: i128 = ((s_5_0) * (s_5_2));
        // D s_5_4: cast reint s_5_3 -> i64
        let s_5_4: i64 = (s_5_3 as i64);
        // D s_5_5: read-var esizeshadow#7882:i64
        let s_5_5: i64 = fn_state.esizeshadow_7882;
        // D s_5_6: cast zx s_5_5 -> i
        let s_5_6: i128 = (i128::try_from(s_5_5).unwrap());
        // D s_5_7: cast reint s_5_6 -> i64
        let s_5_7: i64 = (s_5_6 as i64);
        // D s_5_8: read-var d:i64
        let s_5_8: i64 = fn_state.d;
        // D s_5_9: cast zx s_5_8 -> i
        let s_5_9: i128 = (i128::try_from(s_5_8).unwrap());
        // D s_5_10: call D_read(s_5_9)
        let s_5_10: u64 = D_read(state, tracer, s_5_9);
        // D s_5_11: read-var esizeshadow#7882:i64
        let s_5_11: i64 = fn_state.esizeshadow_7882;
        // D s_5_12: cast zx s_5_11 -> i
        let s_5_12: i128 = (i128::try_from(s_5_11).unwrap());
        // D s_5_13: cast reint s_5_12 -> i64
        let s_5_13: i64 = (s_5_12 as i64);
        // D s_5_14: cast zx s_5_10 -> bv
        let s_5_14: Bits = Bits::new(s_5_10 as u128, 64u16);
        // D s_5_15: read-var u#9570:i64
        let s_5_15: i64 = fn_state.u_9570;
        // D s_5_16: cast zx s_5_15 -> i
        let s_5_16: i128 = (i128::try_from(s_5_15).unwrap());
        // D s_5_17: cast zx s_5_13 -> i
        let s_5_17: i128 = (i128::try_from(s_5_13).unwrap());
        // D s_5_18: call Elem_read(s_5_14, s_5_16, s_5_17)
        let s_5_18: Bits = Elem_read(state, tracer, s_5_14, s_5_16, s_5_17);
        // D s_5_19: read-var zipped_d:u128
        let s_5_19: u128 = fn_state.zipped_d;
        // D s_5_20: cast zx s_5_19 -> bv
        let s_5_20: Bits = Bits::new(s_5_19 as u128, 128u16);
        // D s_5_21: cast zx s_5_4 -> i
        let s_5_21: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_22: cast zx s_5_7 -> i
        let s_5_22: i128 = (i128::try_from(s_5_7).unwrap());
        // D s_5_23: call Elem_set(s_5_20, s_5_21, s_5_22, s_5_18)
        let s_5_23: Bits = Elem_set(state, tracer, s_5_20, s_5_21, s_5_22, s_5_18);
        // D s_5_24: cast reint s_5_23 -> u128
        let s_5_24: u128 = (s_5_23.value() as u128);
        // D s_5_25: write-var zipped_d <= s_5_24
        fn_state.zipped_d = s_5_24;
        // C s_5_26: const #2s : i
        let s_5_26: i128 = 2;
        // D s_5_27: read-var u#9570:i64
        let s_5_27: i64 = fn_state.u_9570;
        // D s_5_28: cast zx s_5_27 -> i
        let s_5_28: i128 = (i128::try_from(s_5_27).unwrap());
        // D s_5_29: mul s_5_26 s_5_28
        let s_5_29: i128 = ((s_5_26) * (s_5_28));
        // D s_5_30: cast reint s_5_29 -> i64
        let s_5_30: i64 = (s_5_29 as i64);
        // C s_5_31: const #1s : i
        let s_5_31: i128 = 1;
        // D s_5_32: cast zx s_5_30 -> i
        let s_5_32: i128 = (i128::try_from(s_5_30).unwrap());
        // D s_5_33: add s_5_32 s_5_31
        let s_5_33: i128 = (s_5_32 + s_5_31);
        // D s_5_34: cast reint s_5_33 -> i64
        let s_5_34: i64 = (s_5_33 as i64);
        // D s_5_35: read-var esizeshadow#7882:i64
        let s_5_35: i64 = fn_state.esizeshadow_7882;
        // D s_5_36: cast zx s_5_35 -> i
        let s_5_36: i128 = (i128::try_from(s_5_35).unwrap());
        // D s_5_37: cast reint s_5_36 -> i64
        let s_5_37: i64 = (s_5_36 as i64);
        // D s_5_38: read-var m:i64
        let s_5_38: i64 = fn_state.m;
        // D s_5_39: cast zx s_5_38 -> i
        let s_5_39: i128 = (i128::try_from(s_5_38).unwrap());
        // D s_5_40: call D_read(s_5_39)
        let s_5_40: u64 = D_read(state, tracer, s_5_39);
        // D s_5_41: read-var esizeshadow#7882:i64
        let s_5_41: i64 = fn_state.esizeshadow_7882;
        // D s_5_42: cast zx s_5_41 -> i
        let s_5_42: i128 = (i128::try_from(s_5_41).unwrap());
        // D s_5_43: cast reint s_5_42 -> i64
        let s_5_43: i64 = (s_5_42 as i64);
        // D s_5_44: cast zx s_5_40 -> bv
        let s_5_44: Bits = Bits::new(s_5_40 as u128, 64u16);
        // D s_5_45: read-var u#9570:i64
        let s_5_45: i64 = fn_state.u_9570;
        // D s_5_46: cast zx s_5_45 -> i
        let s_5_46: i128 = (i128::try_from(s_5_45).unwrap());
        // D s_5_47: cast zx s_5_43 -> i
        let s_5_47: i128 = (i128::try_from(s_5_43).unwrap());
        // D s_5_48: call Elem_read(s_5_44, s_5_46, s_5_47)
        let s_5_48: Bits = Elem_read(state, tracer, s_5_44, s_5_46, s_5_47);
        // D s_5_49: read-var zipped_d:u128
        let s_5_49: u128 = fn_state.zipped_d;
        // D s_5_50: cast zx s_5_49 -> bv
        let s_5_50: Bits = Bits::new(s_5_49 as u128, 128u16);
        // D s_5_51: cast zx s_5_34 -> i
        let s_5_51: i128 = (i128::try_from(s_5_34).unwrap());
        // D s_5_52: cast zx s_5_37 -> i
        let s_5_52: i128 = (i128::try_from(s_5_37).unwrap());
        // D s_5_53: call Elem_set(s_5_50, s_5_51, s_5_52, s_5_48)
        let s_5_53: Bits = Elem_set(state, tracer, s_5_50, s_5_51, s_5_52, s_5_48);
        // D s_5_54: cast reint s_5_53 -> u128
        let s_5_54: u128 = (s_5_53.value() as u128);
        // D s_5_55: write-var zipped_d <= s_5_54
        fn_state.zipped_d = s_5_54;
        // D s_5_56: read-var u#9570:i64
        let s_5_56: i64 = fn_state.u_9570;
        // C s_5_57: const #1s : i64
        let s_5_57: i64 = 1;
        // D s_5_58: add s_5_56 s_5_57
        let s_5_58: i64 = (s_5_56 + s_5_57);
        // D s_5_59: write-var u#9570 <= s_5_58
        fn_state.u_9570 = s_5_58;
        // N s_5_60: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0s : i
        let s_6_0: i128 = 0;
        // D s_6_1: read-var zipped_d:u128
        let s_6_1: u128 = fn_state.zipped_d;
        // D s_6_2: cast zx s_6_1 -> bv
        let s_6_2: Bits = Bits::new(s_6_1 as u128, 128u16);
        // C s_6_3: const #1s : i64
        let s_6_3: i64 = 1;
        // C s_6_4: cast zx s_6_3 -> i
        let s_6_4: i128 = (i128::try_from(s_6_3).unwrap());
        // C s_6_5: const #63s : i
        let s_6_5: i128 = 63;
        // C s_6_6: add s_6_5 s_6_4
        let s_6_6: i128 = (s_6_5 + s_6_4);
        // D s_6_7: bit-extract s_6_2 s_6_0 s_6_6
        let s_6_7: Bits = (Bits::new(
            ((s_6_2) >> (s_6_0)).value(),
            u16::try_from(s_6_6).unwrap(),
        ));
        // D s_6_8: cast reint s_6_7 -> u64
        let s_6_8: u64 = (s_6_7.value() as u64);
        // D s_6_9: read-var d:i64
        let s_6_9: i64 = fn_state.d;
        // D s_6_10: cast zx s_6_9 -> i
        let s_6_10: i128 = (i128::try_from(s_6_9).unwrap());
        // D s_6_11: call D_set(s_6_10, s_6_8)
        let s_6_11: () = D_set(state, tracer, s_6_10, s_6_8);
        // C s_6_12: const #64s : i
        let s_6_12: i128 = 64;
        // D s_6_13: read-var zipped_d:u128
        let s_6_13: u128 = fn_state.zipped_d;
        // D s_6_14: cast zx s_6_13 -> bv
        let s_6_14: Bits = Bits::new(s_6_13 as u128, 128u16);
        // C s_6_15: const #1s : i64
        let s_6_15: i64 = 1;
        // C s_6_16: cast zx s_6_15 -> i
        let s_6_16: i128 = (i128::try_from(s_6_15).unwrap());
        // C s_6_17: const #63s : i
        let s_6_17: i128 = 63;
        // C s_6_18: add s_6_17 s_6_16
        let s_6_18: i128 = (s_6_17 + s_6_16);
        // D s_6_19: bit-extract s_6_14 s_6_12 s_6_18
        let s_6_19: Bits = (Bits::new(
            ((s_6_14) >> (s_6_12)).value(),
            u16::try_from(s_6_18).unwrap(),
        ));
        // D s_6_20: cast reint s_6_19 -> u64
        let s_6_20: u64 = (s_6_19.value() as u64);
        // D s_6_21: read-var m:i64
        let s_6_21: i64 = fn_state.m;
        // D s_6_22: cast zx s_6_21 -> i
        let s_6_22: i128 = (i128::try_from(s_6_21).unwrap());
        // D s_6_23: call D_set(s_6_22, s_6_20)
        let s_6_23: () = D_set(state, tracer, s_6_22, s_6_20);
        // N s_6_24: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #64s : i64
        let s_7_0: i64 = 64;
        // C s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // S s_7_2: call __UNKNOWN_bits(s_7_1)
        let s_7_2: Bits = u__UNKNOWN_bits(state, tracer, s_7_1);
        // S s_7_3: cast reint s_7_2 -> u64
        let s_7_3: u64 = (s_7_2.value() as u64);
        // D s_7_4: read-var d:i64
        let s_7_4: i64 = fn_state.d;
        // D s_7_5: cast zx s_7_4 -> i
        let s_7_5: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_6: call D_set(s_7_5, s_7_3)
        let s_7_6: () = D_set(state, tracer, s_7_5, s_7_3);
        // N s_7_7: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var d:i64
        let s_8_0: i64 = fn_state.d;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: read-var m:i64
        let s_8_2: i64 = fn_state.m;
        // D s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // N s_8_5: branch s_8_4 b13 b9
        if s_8_4 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0s : i64
        let s_9_0: i64 = 0;
        // C s_9_1: const #128s : i
        let s_9_1: i128 = 128;
        // D s_9_2: read-var esizeshadow#7882:i64
        let s_9_2: i64 = fn_state.esizeshadow_7882;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: div s_9_1 s_9_3
        let s_9_4: i128 = ((s_9_1) / (s_9_3));
        // D s_9_5: cast reint s_9_4 -> i64
        let s_9_5: i64 = (s_9_4 as i64);
        // C s_9_6: const #1s : i
        let s_9_6: i128 = 1;
        // D s_9_7: cast zx s_9_5 -> i
        let s_9_7: i128 = (i128::try_from(s_9_5).unwrap());
        // D s_9_8: sub s_9_7 s_9_6
        let s_9_8: i128 = ((s_9_7) - (s_9_6));
        // D s_9_9: cast reint s_9_8 -> i64
        let s_9_9: i64 = (s_9_8 as i64);
        // D s_9_10: write-var gs#323062 <= s_9_9
        fn_state.gs_323062 = s_9_9;
        // D s_9_11: write-var e <= s_9_0
        fn_state.e = s_9_0;
        // N s_9_12: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var e:i64
        let s_10_0: i64 = fn_state.e;
        // D s_10_1: read-var gs#323062:i64
        let s_10_1: i64 = fn_state.gs_323062;
        // D s_10_2: cmp-gt s_10_0 s_10_1
        let s_10_2: bool = ((s_10_0) > (s_10_1));
        // N s_10_3: branch s_10_2 b12 b11
        if s_10_2 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #2s : i
        let s_11_0: i128 = 2;
        // D s_11_1: read-var e:i64
        let s_11_1: i64 = fn_state.e;
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // D s_11_3: mul s_11_0 s_11_2
        let s_11_3: i128 = ((s_11_0) * (s_11_2));
        // D s_11_4: cast reint s_11_3 -> i64
        let s_11_4: i64 = (s_11_3 as i64);
        // D s_11_5: read-var esizeshadow#7882:i64
        let s_11_5: i64 = fn_state.esizeshadow_7882;
        // D s_11_6: cast zx s_11_5 -> i
        let s_11_6: i128 = (i128::try_from(s_11_5).unwrap());
        // D s_11_7: cast reint s_11_6 -> i64
        let s_11_7: i64 = (s_11_6 as i64);
        // C s_11_8: const #1s : i64
        let s_11_8: i64 = 1;
        // D s_11_9: read-var d:i64
        let s_11_9: i64 = fn_state.d;
        // D s_11_10: lsr s_11_9 s_11_8
        let s_11_10: i64 = s_11_9 >> s_11_8;
        // D s_11_11: cast zx s_11_10 -> i
        let s_11_11: i128 = (i128::try_from(s_11_10).unwrap());
        // D s_11_12: call Q_read(s_11_11)
        let s_11_12: u128 = Q_read(state, tracer, s_11_11);
        // D s_11_13: read-var esizeshadow#7882:i64
        let s_11_13: i64 = fn_state.esizeshadow_7882;
        // D s_11_14: cast zx s_11_13 -> i
        let s_11_14: i128 = (i128::try_from(s_11_13).unwrap());
        // D s_11_15: cast reint s_11_14 -> i64
        let s_11_15: i64 = (s_11_14 as i64);
        // D s_11_16: cast zx s_11_12 -> bv
        let s_11_16: Bits = Bits::new(s_11_12 as u128, 128u16);
        // D s_11_17: read-var e:i64
        let s_11_17: i64 = fn_state.e;
        // D s_11_18: cast zx s_11_17 -> i
        let s_11_18: i128 = (i128::try_from(s_11_17).unwrap());
        // D s_11_19: cast zx s_11_15 -> i
        let s_11_19: i128 = (i128::try_from(s_11_15).unwrap());
        // D s_11_20: call Elem_read(s_11_16, s_11_18, s_11_19)
        let s_11_20: Bits = Elem_read(state, tracer, s_11_16, s_11_18, s_11_19);
        // D s_11_21: read-var zipped_q:u256
        let s_11_21: u64 = fn_state.zipped_q;
        // D s_11_22: cast zx s_11_21 -> bv
        let s_11_22: Bits = Bits::new(s_11_21 as u128, 256u16);
        // D s_11_23: cast zx s_11_4 -> i
        let s_11_23: i128 = (i128::try_from(s_11_4).unwrap());
        // D s_11_24: cast zx s_11_7 -> i
        let s_11_24: i128 = (i128::try_from(s_11_7).unwrap());
        // D s_11_25: call Elem_set(s_11_22, s_11_23, s_11_24, s_11_20)
        let s_11_25: Bits = Elem_set(state, tracer, s_11_22, s_11_23, s_11_24, s_11_20);
        // D s_11_26: cast reint s_11_25 -> u256
        let s_11_26: u64 = (s_11_25.value() as u64);
        // D s_11_27: write-var zipped_q <= s_11_26
        fn_state.zipped_q = s_11_26;
        // C s_11_28: const #2s : i
        let s_11_28: i128 = 2;
        // D s_11_29: read-var e:i64
        let s_11_29: i64 = fn_state.e;
        // D s_11_30: cast zx s_11_29 -> i
        let s_11_30: i128 = (i128::try_from(s_11_29).unwrap());
        // D s_11_31: mul s_11_28 s_11_30
        let s_11_31: i128 = ((s_11_28) * (s_11_30));
        // D s_11_32: cast reint s_11_31 -> i64
        let s_11_32: i64 = (s_11_31 as i64);
        // C s_11_33: const #1s : i
        let s_11_33: i128 = 1;
        // D s_11_34: cast zx s_11_32 -> i
        let s_11_34: i128 = (i128::try_from(s_11_32).unwrap());
        // D s_11_35: add s_11_34 s_11_33
        let s_11_35: i128 = (s_11_34 + s_11_33);
        // D s_11_36: cast reint s_11_35 -> i64
        let s_11_36: i64 = (s_11_35 as i64);
        // D s_11_37: read-var esizeshadow#7882:i64
        let s_11_37: i64 = fn_state.esizeshadow_7882;
        // D s_11_38: cast zx s_11_37 -> i
        let s_11_38: i128 = (i128::try_from(s_11_37).unwrap());
        // D s_11_39: cast reint s_11_38 -> i64
        let s_11_39: i64 = (s_11_38 as i64);
        // C s_11_40: const #1s : i64
        let s_11_40: i64 = 1;
        // D s_11_41: read-var m:i64
        let s_11_41: i64 = fn_state.m;
        // D s_11_42: lsr s_11_41 s_11_40
        let s_11_42: i64 = s_11_41 >> s_11_40;
        // D s_11_43: cast zx s_11_42 -> i
        let s_11_43: i128 = (i128::try_from(s_11_42).unwrap());
        // D s_11_44: call Q_read(s_11_43)
        let s_11_44: u128 = Q_read(state, tracer, s_11_43);
        // D s_11_45: read-var esizeshadow#7882:i64
        let s_11_45: i64 = fn_state.esizeshadow_7882;
        // D s_11_46: cast zx s_11_45 -> i
        let s_11_46: i128 = (i128::try_from(s_11_45).unwrap());
        // D s_11_47: cast reint s_11_46 -> i64
        let s_11_47: i64 = (s_11_46 as i64);
        // D s_11_48: cast zx s_11_44 -> bv
        let s_11_48: Bits = Bits::new(s_11_44 as u128, 128u16);
        // D s_11_49: read-var e:i64
        let s_11_49: i64 = fn_state.e;
        // D s_11_50: cast zx s_11_49 -> i
        let s_11_50: i128 = (i128::try_from(s_11_49).unwrap());
        // D s_11_51: cast zx s_11_47 -> i
        let s_11_51: i128 = (i128::try_from(s_11_47).unwrap());
        // D s_11_52: call Elem_read(s_11_48, s_11_50, s_11_51)
        let s_11_52: Bits = Elem_read(state, tracer, s_11_48, s_11_50, s_11_51);
        // D s_11_53: read-var zipped_q:u256
        let s_11_53: u64 = fn_state.zipped_q;
        // D s_11_54: cast zx s_11_53 -> bv
        let s_11_54: Bits = Bits::new(s_11_53 as u128, 256u16);
        // D s_11_55: cast zx s_11_36 -> i
        let s_11_55: i128 = (i128::try_from(s_11_36).unwrap());
        // D s_11_56: cast zx s_11_39 -> i
        let s_11_56: i128 = (i128::try_from(s_11_39).unwrap());
        // D s_11_57: call Elem_set(s_11_54, s_11_55, s_11_56, s_11_52)
        let s_11_57: Bits = Elem_set(state, tracer, s_11_54, s_11_55, s_11_56, s_11_52);
        // D s_11_58: cast reint s_11_57 -> u256
        let s_11_58: u64 = (s_11_57.value() as u64);
        // D s_11_59: write-var zipped_q <= s_11_58
        fn_state.zipped_q = s_11_58;
        // D s_11_60: read-var e:i64
        let s_11_60: i64 = fn_state.e;
        // C s_11_61: const #1s : i64
        let s_11_61: i64 = 1;
        // D s_11_62: add s_11_60 s_11_61
        let s_11_62: i64 = (s_11_60 + s_11_61);
        // D s_11_63: write-var e <= s_11_62
        fn_state.e = s_11_62;
        // N s_11_64: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1s : i64
        let s_12_0: i64 = 1;
        // D s_12_1: read-var d:i64
        let s_12_1: i64 = fn_state.d;
        // D s_12_2: lsr s_12_1 s_12_0
        let s_12_2: i64 = s_12_1 >> s_12_0;
        // C s_12_3: const #0s : i
        let s_12_3: i128 = 0;
        // D s_12_4: read-var zipped_q:u256
        let s_12_4: u64 = fn_state.zipped_q;
        // D s_12_5: cast zx s_12_4 -> bv
        let s_12_5: Bits = Bits::new(s_12_4 as u128, 256u16);
        // C s_12_6: const #1s : i64
        let s_12_6: i64 = 1;
        // C s_12_7: cast zx s_12_6 -> i
        let s_12_7: i128 = (i128::try_from(s_12_6).unwrap());
        // C s_12_8: const #127s : i
        let s_12_8: i128 = 127;
        // C s_12_9: add s_12_8 s_12_7
        let s_12_9: i128 = (s_12_8 + s_12_7);
        // D s_12_10: bit-extract s_12_5 s_12_3 s_12_9
        let s_12_10: Bits = (Bits::new(
            ((s_12_5) >> (s_12_3)).value(),
            u16::try_from(s_12_9).unwrap(),
        ));
        // D s_12_11: cast reint s_12_10 -> u128
        let s_12_11: u128 = (s_12_10.value() as u128);
        // D s_12_12: cast zx s_12_2 -> i
        let s_12_12: i128 = (i128::try_from(s_12_2).unwrap());
        // D s_12_13: call Q_set(s_12_12, s_12_11)
        let s_12_13: () = Q_set(state, tracer, s_12_12, s_12_11);
        // C s_12_14: const #1s : i64
        let s_12_14: i64 = 1;
        // D s_12_15: read-var m:i64
        let s_12_15: i64 = fn_state.m;
        // D s_12_16: lsr s_12_15 s_12_14
        let s_12_16: i64 = s_12_15 >> s_12_14;
        // C s_12_17: const #128s : i
        let s_12_17: i128 = 128;
        // D s_12_18: read-var zipped_q:u256
        let s_12_18: u64 = fn_state.zipped_q;
        // D s_12_19: cast zx s_12_18 -> bv
        let s_12_19: Bits = Bits::new(s_12_18 as u128, 256u16);
        // C s_12_20: const #1s : i64
        let s_12_20: i64 = 1;
        // C s_12_21: cast zx s_12_20 -> i
        let s_12_21: i128 = (i128::try_from(s_12_20).unwrap());
        // C s_12_22: const #127s : i
        let s_12_22: i128 = 127;
        // C s_12_23: add s_12_22 s_12_21
        let s_12_23: i128 = (s_12_22 + s_12_21);
        // D s_12_24: bit-extract s_12_19 s_12_17 s_12_23
        let s_12_24: Bits = (Bits::new(
            ((s_12_19) >> (s_12_17)).value(),
            u16::try_from(s_12_23).unwrap(),
        ));
        // D s_12_25: cast reint s_12_24 -> u128
        let s_12_25: u128 = (s_12_24.value() as u128);
        // D s_12_26: cast zx s_12_16 -> i
        let s_12_26: i128 = (i128::try_from(s_12_16).unwrap());
        // D s_12_27: call Q_set(s_12_26, s_12_25)
        let s_12_27: () = Q_set(state, tracer, s_12_26, s_12_25);
        // N s_12_28: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #1s : i64
        let s_13_0: i64 = 1;
        // D s_13_1: read-var d:i64
        let s_13_1: i64 = fn_state.d;
        // D s_13_2: lsr s_13_1 s_13_0
        let s_13_2: i64 = s_13_1 >> s_13_0;
        // C s_13_3: const #128s : i64
        let s_13_3: i64 = 128;
        // C s_13_4: cast zx s_13_3 -> i
        let s_13_4: i128 = (i128::try_from(s_13_3).unwrap());
        // S s_13_5: call __UNKNOWN_bits(s_13_4)
        let s_13_5: Bits = u__UNKNOWN_bits(state, tracer, s_13_4);
        // S s_13_6: cast reint s_13_5 -> u128
        let s_13_6: u128 = (s_13_5.value() as u128);
        // D s_13_7: cast zx s_13_2 -> i
        let s_13_7: i128 = (i128::try_from(s_13_2).unwrap());
        // D s_13_8: call Q_set(s_13_7, s_13_6)
        let s_13_8: () = Q_set(state, tracer, s_13_7, s_13_6);
        // N s_13_9: return
        return;
    }
}
