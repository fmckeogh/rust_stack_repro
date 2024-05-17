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
pub fn execute_aarch32_instrs_VUZP_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d__arg: i64,
    esize: i64,
    m__arg: i64,
    quadword_operation: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i128,
        e: i64,
        zipped_qshadow_7881: u64,
        gs_322973: i64,
        u_9569: i64,
        zipped_dshadow_7880: u128,
        esizeshadow_7879: i64,
        gs_322988: i64,
        d: i128,
        d__arg: i64,
        esize: i64,
        m__arg: i64,
        quadword_operation: bool,
    }
    let fn_state = FunctionState {
        d__arg,
        esize,
        m__arg,
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
        // D s_0_3: write-var esizeshadow#7879 <= s_0_2
        fn_state.esizeshadow_7879 = s_0_2;
        // D s_0_4: read-var d__arg:i64
        let s_0_4: i64 = fn_state.d__arg;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: write-var d <= s_0_5
        fn_state.d = s_0_5;
        // D s_0_7: read-var m__arg:i64
        let s_0_7: i64 = fn_state.m__arg;
        // D s_0_8: cast zx s_0_7 -> i
        let s_0_8: i128 = (i128::try_from(s_0_7).unwrap());
        // D s_0_9: write-var m <= s_0_8
        fn_state.m = s_0_8;
        // C s_0_10: const #() : ()
        let s_0_10: () = ();
        // S s_0_11: call CheckAdvSIMDEnabled(s_0_10)
        let s_0_11: () = CheckAdvSIMDEnabled(state, tracer, s_0_10);
        // N s_0_12: jump b1
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
        // D s_2_0: read-var d:i
        let s_2_0: i128 = fn_state.d;
        // D s_2_1: read-var m:i
        let s_2_1: i128 = fn_state.m;
        // D s_2_2: cmp-eq s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) == (s_2_1));
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
        // D s_3_0: read-var m:i
        let s_3_0: i128 = fn_state.m;
        // D s_3_1: call D_read(s_3_0)
        let s_3_1: u64 = D_read(state, tracer, s_3_0);
        // D s_3_2: read-var d:i
        let s_3_2: i128 = fn_state.d;
        // D s_3_3: call D_read(s_3_2)
        let s_3_3: u64 = D_read(state, tracer, s_3_2);
        // D s_3_4: cast zx s_3_1 -> bv
        let s_3_4: Bits = Bits::new(s_3_1 as u128, 64u16);
        // D s_3_5: cast zx s_3_3 -> bv
        let s_3_5: Bits = Bits::new(s_3_3 as u128, 64u16);
        // D s_3_6: cast reint s_3_4 -> u128
        let s_3_6: u128 = (s_3_4.value() as u128);
        // D s_3_7: size-of s_3_4
        let s_3_7: u16 = s_3_4.length();
        // D s_3_8: cast reint s_3_5 -> u128
        let s_3_8: u128 = (s_3_5.value() as u128);
        // D s_3_9: size-of s_3_5
        let s_3_9: u16 = s_3_5.length();
        // D s_3_10: lsl s_3_6 s_3_9
        let s_3_10: u128 = s_3_6 << s_3_9;
        // D s_3_11: or s_3_10 s_3_8
        let s_3_11: u128 = ((s_3_10) | (s_3_8));
        // D s_3_12: add s_3_7 s_3_9
        let s_3_12: u16 = (s_3_7 + s_3_9);
        // D s_3_13: create-bits s_3_11 s_3_12
        let s_3_13: Bits = Bits::new(s_3_11, s_3_12);
        // D s_3_14: cast reint s_3_13 -> u128
        let s_3_14: u128 = (s_3_13.value() as u128);
        // D s_3_15: write-var zipped_dshadow#7880 <= s_3_14
        fn_state.zipped_dshadow_7880 = s_3_14;
        // C s_3_16: const #0s : i64
        let s_3_16: i64 = 0;
        // C s_3_17: const #64s : i
        let s_3_17: i128 = 64;
        // D s_3_18: read-var esizeshadow#7879:i64
        let s_3_18: i64 = fn_state.esizeshadow_7879;
        // D s_3_19: cast zx s_3_18 -> i
        let s_3_19: i128 = (i128::try_from(s_3_18).unwrap());
        // D s_3_20: div s_3_17 s_3_19
        let s_3_20: i128 = ((s_3_17) / (s_3_19));
        // D s_3_21: cast reint s_3_20 -> i64
        let s_3_21: i64 = (s_3_20 as i64);
        // C s_3_22: const #1s : i
        let s_3_22: i128 = 1;
        // D s_3_23: cast zx s_3_21 -> i
        let s_3_23: i128 = (i128::try_from(s_3_21).unwrap());
        // D s_3_24: sub s_3_23 s_3_22
        let s_3_24: i128 = ((s_3_23) - (s_3_22));
        // D s_3_25: cast reint s_3_24 -> i64
        let s_3_25: i64 = (s_3_24 as i64);
        // D s_3_26: write-var gs#322973 <= s_3_25
        fn_state.gs_322973 = s_3_25;
        // D s_3_27: write-var u#9569 <= s_3_16
        fn_state.u_9569 = s_3_16;
        // N s_3_28: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var u#9569:i64
        let s_4_0: i64 = fn_state.u_9569;
        // D s_4_1: read-var gs#322973:i64
        let s_4_1: i64 = fn_state.gs_322973;
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
        // D s_5_0: read-var d:i
        let s_5_0: i128 = fn_state.d;
        // D s_5_1: call D_read(s_5_0)
        let s_5_1: u64 = D_read(state, tracer, s_5_0);
        // D s_5_2: read-var esizeshadow#7879:i64
        let s_5_2: i64 = fn_state.esizeshadow_7879;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: cast reint s_5_3 -> i64
        let s_5_4: i64 = (s_5_3 as i64);
        // C s_5_5: const #2s : i
        let s_5_5: i128 = 2;
        // D s_5_6: read-var u#9569:i64
        let s_5_6: i64 = fn_state.u_9569;
        // D s_5_7: cast zx s_5_6 -> i
        let s_5_7: i128 = (i128::try_from(s_5_6).unwrap());
        // D s_5_8: mul s_5_5 s_5_7
        let s_5_8: i128 = ((s_5_5) * (s_5_7));
        // D s_5_9: cast reint s_5_8 -> i64
        let s_5_9: i64 = (s_5_8 as i64);
        // D s_5_10: read-var esizeshadow#7879:i64
        let s_5_10: i64 = fn_state.esizeshadow_7879;
        // D s_5_11: cast zx s_5_10 -> i
        let s_5_11: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_12: cast reint s_5_11 -> i64
        let s_5_12: i64 = (s_5_11 as i64);
        // D s_5_13: read-var zipped_dshadow#7880:u128
        let s_5_13: u128 = fn_state.zipped_dshadow_7880;
        // D s_5_14: cast zx s_5_13 -> bv
        let s_5_14: Bits = Bits::new(s_5_13 as u128, 128u16);
        // D s_5_15: cast zx s_5_9 -> i
        let s_5_15: i128 = (i128::try_from(s_5_9).unwrap());
        // D s_5_16: cast zx s_5_12 -> i
        let s_5_16: i128 = (i128::try_from(s_5_12).unwrap());
        // D s_5_17: call Elem_read(s_5_14, s_5_15, s_5_16)
        let s_5_17: Bits = Elem_read(state, tracer, s_5_14, s_5_15, s_5_16);
        // D s_5_18: cast zx s_5_1 -> bv
        let s_5_18: Bits = Bits::new(s_5_1 as u128, 64u16);
        // D s_5_19: read-var u#9569:i64
        let s_5_19: i64 = fn_state.u_9569;
        // D s_5_20: cast zx s_5_19 -> i
        let s_5_20: i128 = (i128::try_from(s_5_19).unwrap());
        // D s_5_21: cast zx s_5_4 -> i
        let s_5_21: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_22: call Elem_set(s_5_18, s_5_20, s_5_21, s_5_17)
        let s_5_22: Bits = Elem_set(state, tracer, s_5_18, s_5_20, s_5_21, s_5_17);
        // D s_5_23: cast reint s_5_22 -> u64
        let s_5_23: u64 = (s_5_22.value() as u64);
        // D s_5_24: read-var d:i
        let s_5_24: i128 = fn_state.d;
        // D s_5_25: call D_set(s_5_24, s_5_23)
        let s_5_25: () = D_set(state, tracer, s_5_24, s_5_23);
        // D s_5_26: read-var m:i
        let s_5_26: i128 = fn_state.m;
        // D s_5_27: call D_read(s_5_26)
        let s_5_27: u64 = D_read(state, tracer, s_5_26);
        // D s_5_28: read-var esizeshadow#7879:i64
        let s_5_28: i64 = fn_state.esizeshadow_7879;
        // D s_5_29: cast zx s_5_28 -> i
        let s_5_29: i128 = (i128::try_from(s_5_28).unwrap());
        // D s_5_30: cast reint s_5_29 -> i64
        let s_5_30: i64 = (s_5_29 as i64);
        // C s_5_31: const #2s : i
        let s_5_31: i128 = 2;
        // D s_5_32: read-var u#9569:i64
        let s_5_32: i64 = fn_state.u_9569;
        // D s_5_33: cast zx s_5_32 -> i
        let s_5_33: i128 = (i128::try_from(s_5_32).unwrap());
        // D s_5_34: mul s_5_31 s_5_33
        let s_5_34: i128 = ((s_5_31) * (s_5_33));
        // D s_5_35: cast reint s_5_34 -> i64
        let s_5_35: i64 = (s_5_34 as i64);
        // C s_5_36: const #1s : i
        let s_5_36: i128 = 1;
        // D s_5_37: cast zx s_5_35 -> i
        let s_5_37: i128 = (i128::try_from(s_5_35).unwrap());
        // D s_5_38: add s_5_37 s_5_36
        let s_5_38: i128 = (s_5_37 + s_5_36);
        // D s_5_39: cast reint s_5_38 -> i64
        let s_5_39: i64 = (s_5_38 as i64);
        // D s_5_40: read-var esizeshadow#7879:i64
        let s_5_40: i64 = fn_state.esizeshadow_7879;
        // D s_5_41: cast zx s_5_40 -> i
        let s_5_41: i128 = (i128::try_from(s_5_40).unwrap());
        // D s_5_42: cast reint s_5_41 -> i64
        let s_5_42: i64 = (s_5_41 as i64);
        // D s_5_43: read-var zipped_dshadow#7880:u128
        let s_5_43: u128 = fn_state.zipped_dshadow_7880;
        // D s_5_44: cast zx s_5_43 -> bv
        let s_5_44: Bits = Bits::new(s_5_43 as u128, 128u16);
        // D s_5_45: cast zx s_5_39 -> i
        let s_5_45: i128 = (i128::try_from(s_5_39).unwrap());
        // D s_5_46: cast zx s_5_42 -> i
        let s_5_46: i128 = (i128::try_from(s_5_42).unwrap());
        // D s_5_47: call Elem_read(s_5_44, s_5_45, s_5_46)
        let s_5_47: Bits = Elem_read(state, tracer, s_5_44, s_5_45, s_5_46);
        // D s_5_48: cast zx s_5_27 -> bv
        let s_5_48: Bits = Bits::new(s_5_27 as u128, 64u16);
        // D s_5_49: read-var u#9569:i64
        let s_5_49: i64 = fn_state.u_9569;
        // D s_5_50: cast zx s_5_49 -> i
        let s_5_50: i128 = (i128::try_from(s_5_49).unwrap());
        // D s_5_51: cast zx s_5_30 -> i
        let s_5_51: i128 = (i128::try_from(s_5_30).unwrap());
        // D s_5_52: call Elem_set(s_5_48, s_5_50, s_5_51, s_5_47)
        let s_5_52: Bits = Elem_set(state, tracer, s_5_48, s_5_50, s_5_51, s_5_47);
        // D s_5_53: cast reint s_5_52 -> u64
        let s_5_53: u64 = (s_5_52.value() as u64);
        // D s_5_54: read-var m:i
        let s_5_54: i128 = fn_state.m;
        // D s_5_55: call D_set(s_5_54, s_5_53)
        let s_5_55: () = D_set(state, tracer, s_5_54, s_5_53);
        // D s_5_56: read-var u#9569:i64
        let s_5_56: i64 = fn_state.u_9569;
        // C s_5_57: const #1s : i64
        let s_5_57: i64 = 1;
        // D s_5_58: add s_5_56 s_5_57
        let s_5_58: i64 = (s_5_56 + s_5_57);
        // D s_5_59: write-var u#9569 <= s_5_58
        fn_state.u_9569 = s_5_58;
        // N s_5_60: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: return
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
        // D s_7_4: read-var d:i
        let s_7_4: i128 = fn_state.d;
        // D s_7_5: call D_set(s_7_4, s_7_3)
        let s_7_5: () = D_set(state, tracer, s_7_4, s_7_3);
        // N s_7_6: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var d:i
        let s_8_0: i128 = fn_state.d;
        // D s_8_1: read-var m:i
        let s_8_1: i128 = fn_state.m;
        // D s_8_2: cmp-eq s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) == (s_8_1));
        // N s_8_3: branch s_8_2 b13 b9
        if s_8_2 {
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
        // C s_9_0: const #1s : i
        let s_9_0: i128 = 1;
        // D s_9_1: read-var m:i
        let s_9_1: i128 = fn_state.m;
        // D s_9_2: lsr s_9_1 s_9_0
        let s_9_2: i128 = s_9_1 >> s_9_0;
        // D s_9_3: call Q_read(s_9_2)
        let s_9_3: u128 = Q_read(state, tracer, s_9_2);
        // C s_9_4: const #1s : i
        let s_9_4: i128 = 1;
        // D s_9_5: read-var d:i
        let s_9_5: i128 = fn_state.d;
        // D s_9_6: lsr s_9_5 s_9_4
        let s_9_6: i128 = s_9_5 >> s_9_4;
        // D s_9_7: call Q_read(s_9_6)
        let s_9_7: u128 = Q_read(state, tracer, s_9_6);
        // D s_9_8: cast zx s_9_3 -> bv
        let s_9_8: Bits = Bits::new(s_9_3 as u128, 128u16);
        // D s_9_9: cast zx s_9_7 -> bv
        let s_9_9: Bits = Bits::new(s_9_7 as u128, 128u16);
        // D s_9_10: cast reint s_9_8 -> u128
        let s_9_10: u128 = (s_9_8.value() as u128);
        // D s_9_11: size-of s_9_8
        let s_9_11: u16 = s_9_8.length();
        // D s_9_12: cast reint s_9_9 -> u128
        let s_9_12: u128 = (s_9_9.value() as u128);
        // D s_9_13: size-of s_9_9
        let s_9_13: u16 = s_9_9.length();
        // D s_9_14: lsl s_9_10 s_9_13
        let s_9_14: u128 = s_9_10 << s_9_13;
        // D s_9_15: or s_9_14 s_9_12
        let s_9_15: u128 = ((s_9_14) | (s_9_12));
        // D s_9_16: add s_9_11 s_9_13
        let s_9_16: u16 = (s_9_11 + s_9_13);
        // D s_9_17: create-bits s_9_15 s_9_16
        let s_9_17: Bits = Bits::new(s_9_15, s_9_16);
        // D s_9_18: cast reint s_9_17 -> u256
        let s_9_18: u64 = (s_9_17.value() as u64);
        // D s_9_19: write-var zipped_qshadow#7881 <= s_9_18
        fn_state.zipped_qshadow_7881 = s_9_18;
        // C s_9_20: const #0s : i64
        let s_9_20: i64 = 0;
        // C s_9_21: const #128s : i
        let s_9_21: i128 = 128;
        // D s_9_22: read-var esizeshadow#7879:i64
        let s_9_22: i64 = fn_state.esizeshadow_7879;
        // D s_9_23: cast zx s_9_22 -> i
        let s_9_23: i128 = (i128::try_from(s_9_22).unwrap());
        // D s_9_24: div s_9_21 s_9_23
        let s_9_24: i128 = ((s_9_21) / (s_9_23));
        // D s_9_25: cast reint s_9_24 -> i64
        let s_9_25: i64 = (s_9_24 as i64);
        // C s_9_26: const #1s : i
        let s_9_26: i128 = 1;
        // D s_9_27: cast zx s_9_25 -> i
        let s_9_27: i128 = (i128::try_from(s_9_25).unwrap());
        // D s_9_28: sub s_9_27 s_9_26
        let s_9_28: i128 = ((s_9_27) - (s_9_26));
        // D s_9_29: cast reint s_9_28 -> i64
        let s_9_29: i64 = (s_9_28 as i64);
        // D s_9_30: write-var gs#322988 <= s_9_29
        fn_state.gs_322988 = s_9_29;
        // D s_9_31: write-var e <= s_9_20
        fn_state.e = s_9_20;
        // N s_9_32: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var e:i64
        let s_10_0: i64 = fn_state.e;
        // D s_10_1: read-var gs#322988:i64
        let s_10_1: i64 = fn_state.gs_322988;
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
        // D s_11_7: read-var esizeshadow#7879:i64
        let s_11_7: i64 = fn_state.esizeshadow_7879;
        // D s_11_8: cast zx s_11_7 -> i
        let s_11_8: i128 = (i128::try_from(s_11_7).unwrap());
        // D s_11_9: cast reint s_11_8 -> i64
        let s_11_9: i64 = (s_11_8 as i64);
        // C s_11_10: const #2s : i
        let s_11_10: i128 = 2;
        // D s_11_11: read-var e:i64
        let s_11_11: i64 = fn_state.e;
        // D s_11_12: cast zx s_11_11 -> i
        let s_11_12: i128 = (i128::try_from(s_11_11).unwrap());
        // D s_11_13: mul s_11_10 s_11_12
        let s_11_13: i128 = ((s_11_10) * (s_11_12));
        // D s_11_14: cast reint s_11_13 -> i64
        let s_11_14: i64 = (s_11_13 as i64);
        // D s_11_15: read-var esizeshadow#7879:i64
        let s_11_15: i64 = fn_state.esizeshadow_7879;
        // D s_11_16: cast zx s_11_15 -> i
        let s_11_16: i128 = (i128::try_from(s_11_15).unwrap());
        // D s_11_17: cast reint s_11_16 -> i64
        let s_11_17: i64 = (s_11_16 as i64);
        // D s_11_18: read-var zipped_qshadow#7881:u256
        let s_11_18: u64 = fn_state.zipped_qshadow_7881;
        // D s_11_19: cast zx s_11_18 -> bv
        let s_11_19: Bits = Bits::new(s_11_18 as u128, 256u16);
        // D s_11_20: cast zx s_11_14 -> i
        let s_11_20: i128 = (i128::try_from(s_11_14).unwrap());
        // D s_11_21: cast zx s_11_17 -> i
        let s_11_21: i128 = (i128::try_from(s_11_17).unwrap());
        // D s_11_22: call Elem_read(s_11_19, s_11_20, s_11_21)
        let s_11_22: Bits = Elem_read(state, tracer, s_11_19, s_11_20, s_11_21);
        // D s_11_23: cast zx s_11_6 -> bv
        let s_11_23: Bits = Bits::new(s_11_6 as u128, 128u16);
        // D s_11_24: read-var e:i64
        let s_11_24: i64 = fn_state.e;
        // D s_11_25: cast zx s_11_24 -> i
        let s_11_25: i128 = (i128::try_from(s_11_24).unwrap());
        // D s_11_26: cast zx s_11_9 -> i
        let s_11_26: i128 = (i128::try_from(s_11_9).unwrap());
        // D s_11_27: call Elem_set(s_11_23, s_11_25, s_11_26, s_11_22)
        let s_11_27: Bits = Elem_set(state, tracer, s_11_23, s_11_25, s_11_26, s_11_22);
        // D s_11_28: cast reint s_11_27 -> u128
        let s_11_28: u128 = (s_11_27.value() as u128);
        // D s_11_29: call Q_set(s_11_2, s_11_28)
        let s_11_29: () = Q_set(state, tracer, s_11_2, s_11_28);
        // C s_11_30: const #1s : i
        let s_11_30: i128 = 1;
        // D s_11_31: read-var m:i
        let s_11_31: i128 = fn_state.m;
        // D s_11_32: lsr s_11_31 s_11_30
        let s_11_32: i128 = s_11_31 >> s_11_30;
        // C s_11_33: const #1s : i
        let s_11_33: i128 = 1;
        // D s_11_34: read-var m:i
        let s_11_34: i128 = fn_state.m;
        // D s_11_35: lsr s_11_34 s_11_33
        let s_11_35: i128 = s_11_34 >> s_11_33;
        // D s_11_36: call Q_read(s_11_35)
        let s_11_36: u128 = Q_read(state, tracer, s_11_35);
        // D s_11_37: read-var esizeshadow#7879:i64
        let s_11_37: i64 = fn_state.esizeshadow_7879;
        // D s_11_38: cast zx s_11_37 -> i
        let s_11_38: i128 = (i128::try_from(s_11_37).unwrap());
        // D s_11_39: cast reint s_11_38 -> i64
        let s_11_39: i64 = (s_11_38 as i64);
        // C s_11_40: const #2s : i
        let s_11_40: i128 = 2;
        // D s_11_41: read-var e:i64
        let s_11_41: i64 = fn_state.e;
        // D s_11_42: cast zx s_11_41 -> i
        let s_11_42: i128 = (i128::try_from(s_11_41).unwrap());
        // D s_11_43: mul s_11_40 s_11_42
        let s_11_43: i128 = ((s_11_40) * (s_11_42));
        // D s_11_44: cast reint s_11_43 -> i64
        let s_11_44: i64 = (s_11_43 as i64);
        // C s_11_45: const #1s : i
        let s_11_45: i128 = 1;
        // D s_11_46: cast zx s_11_44 -> i
        let s_11_46: i128 = (i128::try_from(s_11_44).unwrap());
        // D s_11_47: add s_11_46 s_11_45
        let s_11_47: i128 = (s_11_46 + s_11_45);
        // D s_11_48: cast reint s_11_47 -> i64
        let s_11_48: i64 = (s_11_47 as i64);
        // D s_11_49: read-var esizeshadow#7879:i64
        let s_11_49: i64 = fn_state.esizeshadow_7879;
        // D s_11_50: cast zx s_11_49 -> i
        let s_11_50: i128 = (i128::try_from(s_11_49).unwrap());
        // D s_11_51: cast reint s_11_50 -> i64
        let s_11_51: i64 = (s_11_50 as i64);
        // D s_11_52: read-var zipped_qshadow#7881:u256
        let s_11_52: u64 = fn_state.zipped_qshadow_7881;
        // D s_11_53: cast zx s_11_52 -> bv
        let s_11_53: Bits = Bits::new(s_11_52 as u128, 256u16);
        // D s_11_54: cast zx s_11_48 -> i
        let s_11_54: i128 = (i128::try_from(s_11_48).unwrap());
        // D s_11_55: cast zx s_11_51 -> i
        let s_11_55: i128 = (i128::try_from(s_11_51).unwrap());
        // D s_11_56: call Elem_read(s_11_53, s_11_54, s_11_55)
        let s_11_56: Bits = Elem_read(state, tracer, s_11_53, s_11_54, s_11_55);
        // D s_11_57: cast zx s_11_36 -> bv
        let s_11_57: Bits = Bits::new(s_11_36 as u128, 128u16);
        // D s_11_58: read-var e:i64
        let s_11_58: i64 = fn_state.e;
        // D s_11_59: cast zx s_11_58 -> i
        let s_11_59: i128 = (i128::try_from(s_11_58).unwrap());
        // D s_11_60: cast zx s_11_39 -> i
        let s_11_60: i128 = (i128::try_from(s_11_39).unwrap());
        // D s_11_61: call Elem_set(s_11_57, s_11_59, s_11_60, s_11_56)
        let s_11_61: Bits = Elem_set(state, tracer, s_11_57, s_11_59, s_11_60, s_11_56);
        // D s_11_62: cast reint s_11_61 -> u128
        let s_11_62: u128 = (s_11_61.value() as u128);
        // D s_11_63: call Q_set(s_11_32, s_11_62)
        let s_11_63: () = Q_set(state, tracer, s_11_32, s_11_62);
        // D s_11_64: read-var e:i64
        let s_11_64: i64 = fn_state.e;
        // C s_11_65: const #1s : i64
        let s_11_65: i64 = 1;
        // D s_11_66: add s_11_64 s_11_65
        let s_11_66: i64 = (s_11_64 + s_11_65);
        // D s_11_67: write-var e <= s_11_66
        fn_state.e = s_11_66;
        // N s_11_68: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #1s : i
        let s_13_0: i128 = 1;
        // D s_13_1: read-var d:i
        let s_13_1: i128 = fn_state.d;
        // D s_13_2: lsr s_13_1 s_13_0
        let s_13_2: i128 = s_13_1 >> s_13_0;
        // C s_13_3: const #128s : i64
        let s_13_3: i64 = 128;
        // C s_13_4: cast zx s_13_3 -> i
        let s_13_4: i128 = (i128::try_from(s_13_3).unwrap());
        // S s_13_5: call __UNKNOWN_bits(s_13_4)
        let s_13_5: Bits = u__UNKNOWN_bits(state, tracer, s_13_4);
        // S s_13_6: cast reint s_13_5 -> u128
        let s_13_6: u128 = (s_13_5.value() as u128);
        // D s_13_7: call Q_set(s_13_2, s_13_6)
        let s_13_7: () = Q_set(state, tracer, s_13_2, s_13_6);
        // N s_13_8: return
        return;
    }
}
