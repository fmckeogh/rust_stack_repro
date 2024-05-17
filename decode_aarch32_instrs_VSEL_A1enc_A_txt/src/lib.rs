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
use HaveFP16Ext::*;
use execute_aarch32_instrs_VSEL_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VSEL_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    cc: u8,
    Vn: u8,
    Vd: u8,
    size: u8,
    N: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        esize: i64,
        gs_325885: bool,
        n: i64,
        gs_325886: bool,
        d: i64,
        D: bool,
        cc: u8,
        Vn: u8,
        Vd: u8,
        size: u8,
        N: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        D,
        cc,
        Vn,
        Vd,
        size,
        N,
        M,
        Vm,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var size:u8
        let s_0_0: u8 = fn_state.size;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 2u16);
        // C s_0_2: const #0u : u8
        let s_0_2: u8 = 0;
        // C s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 2u16);
        // D s_0_4: cmp-eq s_0_1 s_0_3
        let s_0_4: bool = ((s_0_1) == (s_0_3));
        // N s_0_5: branch s_0_4 b15 b1
        if s_0_4 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var size:u8
        let s_1_0: u8 = fn_state.size;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 2u16);
        // C s_1_2: const #1u : u8
        let s_1_2: u8 = 1;
        // C s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 2u16);
        // D s_1_4: cmp-eq s_1_1 s_1_3
        let s_1_4: bool = ((s_1_1) == (s_1_3));
        // N s_1_5: branch s_1_4 b14 b2
        if s_1_4 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#325885 <= s_2_0
        fn_state.gs_325885 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#325885:u8
        let s_3_0: bool = fn_state.gs_325885;
        // D s_3_1: write-var gs#325886 <= s_3_0
        fn_state.gs_325886 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#325886:u8
        let s_4_0: bool = fn_state.gs_325886;
        // N s_4_1: branch s_4_0 b13 b5
        if s_4_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #16s : i64
        let s_5_0: i64 = 16;
        // D s_5_1: write-var esize <= s_5_0
        fn_state.esize = s_5_0;
        // D s_5_2: read-var size:u8
        let s_5_2: u8 = fn_state.size;
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 2u16);
        // C s_5_4: const #1u : u8
        let s_5_4: u8 = 1;
        // C s_5_5: cast zx s_5_4 -> bv
        let s_5_5: Bits = Bits::new(s_5_4 as u128, 2u16);
        // D s_5_6: cmp-eq s_5_3 s_5_5
        let s_5_6: bool = ((s_5_3) == (s_5_5));
        // D s_5_7: not s_5_6
        let s_5_7: bool = !s_5_6;
        // N s_5_8: branch s_5_7 b8 b6
        if s_5_7 {
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
        // C s_6_0: const #16s : i64
        let s_6_0: i64 = 16;
        // D s_6_1: write-var esize <= s_6_0
        fn_state.esize = s_6_0;
        // D s_6_2: read-var Vd:u8
        let s_6_2: u8 = fn_state.Vd;
        // D s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 4u16);
        // D s_6_4: read-var D:u8
        let s_6_4: bool = fn_state.D;
        // D s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 1u16);
        // D s_6_6: cast reint s_6_3 -> u128
        let s_6_6: u128 = (s_6_3.value() as u128);
        // D s_6_7: size-of s_6_3
        let s_6_7: u16 = s_6_3.length();
        // D s_6_8: cast reint s_6_5 -> u128
        let s_6_8: u128 = (s_6_5.value() as u128);
        // D s_6_9: size-of s_6_5
        let s_6_9: u16 = s_6_5.length();
        // D s_6_10: lsl s_6_6 s_6_9
        let s_6_10: u128 = s_6_6 << s_6_9;
        // D s_6_11: or s_6_10 s_6_8
        let s_6_11: u128 = ((s_6_10) | (s_6_8));
        // D s_6_12: add s_6_7 s_6_9
        let s_6_12: u16 = (s_6_7 + s_6_9);
        // D s_6_13: create-bits s_6_11 s_6_12
        let s_6_13: Bits = Bits::new(s_6_11, s_6_12);
        // D s_6_14: cast reint s_6_13 -> u8
        let s_6_14: u8 = (s_6_13.value() as u8);
        // D s_6_15: cast zx s_6_14 -> bv
        let s_6_15: Bits = Bits::new(s_6_14 as u128, 5u16);
        // D s_6_16: cast zx s_6_15 -> i
        let s_6_16: i128 = (s_6_15.value() as i128);
        // D s_6_17: cast reint s_6_16 -> i64
        let s_6_17: i64 = (s_6_16 as i64);
        // D s_6_18: write-var d <= s_6_17
        fn_state.d = s_6_17;
        // D s_6_19: read-var Vn:u8
        let s_6_19: u8 = fn_state.Vn;
        // D s_6_20: cast zx s_6_19 -> bv
        let s_6_20: Bits = Bits::new(s_6_19 as u128, 4u16);
        // D s_6_21: read-var N:u8
        let s_6_21: bool = fn_state.N;
        // D s_6_22: cast zx s_6_21 -> bv
        let s_6_22: Bits = Bits::new(s_6_21 as u128, 1u16);
        // D s_6_23: cast reint s_6_20 -> u128
        let s_6_23: u128 = (s_6_20.value() as u128);
        // D s_6_24: size-of s_6_20
        let s_6_24: u16 = s_6_20.length();
        // D s_6_25: cast reint s_6_22 -> u128
        let s_6_25: u128 = (s_6_22.value() as u128);
        // D s_6_26: size-of s_6_22
        let s_6_26: u16 = s_6_22.length();
        // D s_6_27: lsl s_6_23 s_6_26
        let s_6_27: u128 = s_6_23 << s_6_26;
        // D s_6_28: or s_6_27 s_6_25
        let s_6_28: u128 = ((s_6_27) | (s_6_25));
        // D s_6_29: add s_6_24 s_6_26
        let s_6_29: u16 = (s_6_24 + s_6_26);
        // D s_6_30: create-bits s_6_28 s_6_29
        let s_6_30: Bits = Bits::new(s_6_28, s_6_29);
        // D s_6_31: cast reint s_6_30 -> u8
        let s_6_31: u8 = (s_6_30.value() as u8);
        // D s_6_32: cast zx s_6_31 -> bv
        let s_6_32: Bits = Bits::new(s_6_31 as u128, 5u16);
        // D s_6_33: cast zx s_6_32 -> i
        let s_6_33: i128 = (s_6_32.value() as i128);
        // D s_6_34: cast reint s_6_33 -> i64
        let s_6_34: i64 = (s_6_33 as i64);
        // D s_6_35: write-var n <= s_6_34
        fn_state.n = s_6_34;
        // D s_6_36: read-var Vm:u8
        let s_6_36: u8 = fn_state.Vm;
        // D s_6_37: cast zx s_6_36 -> bv
        let s_6_37: Bits = Bits::new(s_6_36 as u128, 4u16);
        // D s_6_38: read-var M:u8
        let s_6_38: bool = fn_state.M;
        // D s_6_39: cast zx s_6_38 -> bv
        let s_6_39: Bits = Bits::new(s_6_38 as u128, 1u16);
        // D s_6_40: cast reint s_6_37 -> u128
        let s_6_40: u128 = (s_6_37.value() as u128);
        // D s_6_41: size-of s_6_37
        let s_6_41: u16 = s_6_37.length();
        // D s_6_42: cast reint s_6_39 -> u128
        let s_6_42: u128 = (s_6_39.value() as u128);
        // D s_6_43: size-of s_6_39
        let s_6_43: u16 = s_6_39.length();
        // D s_6_44: lsl s_6_40 s_6_43
        let s_6_44: u128 = s_6_40 << s_6_43;
        // D s_6_45: or s_6_44 s_6_42
        let s_6_45: u128 = ((s_6_44) | (s_6_42));
        // D s_6_46: add s_6_41 s_6_43
        let s_6_46: u16 = (s_6_41 + s_6_43);
        // D s_6_47: create-bits s_6_45 s_6_46
        let s_6_47: Bits = Bits::new(s_6_45, s_6_46);
        // D s_6_48: cast reint s_6_47 -> u8
        let s_6_48: u8 = (s_6_47.value() as u8);
        // D s_6_49: cast zx s_6_48 -> bv
        let s_6_49: Bits = Bits::new(s_6_48 as u128, 5u16);
        // D s_6_50: cast zx s_6_49 -> i
        let s_6_50: i128 = (s_6_49.value() as i128);
        // D s_6_51: cast reint s_6_50 -> i64
        let s_6_51: i64 = (s_6_50 as i64);
        // D s_6_52: write-var m <= s_6_51
        fn_state.m = s_6_51;
        // N s_6_53: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var n:i64
        let s_7_0: i64 = fn_state.n;
        // D s_7_1: read-var m:i64
        let s_7_1: i64 = fn_state.m;
        // D s_7_2: read-var esize:i64
        let s_7_2: i64 = fn_state.esize;
        // D s_7_3: read-var d:i64
        let s_7_3: i64 = fn_state.d;
        // C s_7_4: const #1s : i
        let s_7_4: i128 = 1;
        // D s_7_5: read-var cc:u8
        let s_7_5: u8 = fn_state.cc;
        // D s_7_6: cast zx s_7_5 -> bv
        let s_7_6: Bits = Bits::new(s_7_5 as u128, 2u16);
        // C s_7_7: const #1u : u64
        let s_7_7: u64 = 1;
        // D s_7_8: bit-extract s_7_6 s_7_4 s_7_7
        let s_7_8: Bits = (Bits::new(
            ((s_7_6) >> (s_7_4)).value(),
            u16::try_from(s_7_7).unwrap(),
        ));
        // D s_7_9: cast reint s_7_8 -> u8
        let s_7_9: bool = ((s_7_8.value()) != 0);
        // C s_7_10: const #0s : i
        let s_7_10: i128 = 0;
        // C s_7_11: const #0u : u64
        let s_7_11: u64 = 0;
        // D s_7_12: cast zx s_7_9 -> u64
        let s_7_12: u64 = (s_7_9 as u64);
        // C s_7_13: const #1u : u64
        let s_7_13: u64 = 1;
        // D s_7_14: and s_7_12 s_7_13
        let s_7_14: u64 = ((s_7_12) & (s_7_13));
        // D s_7_15: cmp-eq s_7_14 s_7_13
        let s_7_15: bool = ((s_7_14) == (s_7_13));
        // D s_7_16: lsl s_7_12 s_7_10
        let s_7_16: u64 = s_7_12 << s_7_10;
        // D s_7_17: or s_7_11 s_7_16
        let s_7_17: u64 = ((s_7_11) | (s_7_16));
        // D s_7_18: cmpl s_7_16
        let s_7_18: u64 = !s_7_16;
        // D s_7_19: and s_7_11 s_7_18
        let s_7_19: u64 = ((s_7_11) & (s_7_18));
        // D s_7_20: select s_7_15 s_7_17 s_7_19
        let s_7_20: u64 = if s_7_15 { s_7_17 } else { s_7_19 };
        // D s_7_21: cast trunc s_7_20 -> u8
        let s_7_21: bool = ((s_7_20) != 0);
        // C s_7_22: const #0s : i
        let s_7_22: i128 = 0;
        // D s_7_23: read-var cc:u8
        let s_7_23: u8 = fn_state.cc;
        // D s_7_24: cast zx s_7_23 -> bv
        let s_7_24: Bits = Bits::new(s_7_23 as u128, 2u16);
        // C s_7_25: const #1u : u64
        let s_7_25: u64 = 1;
        // D s_7_26: bit-extract s_7_24 s_7_22 s_7_25
        let s_7_26: Bits = (Bits::new(
            ((s_7_24) >> (s_7_22)).value(),
            u16::try_from(s_7_25).unwrap(),
        ));
        // D s_7_27: cast reint s_7_26 -> u8
        let s_7_27: bool = ((s_7_26.value()) != 0);
        // C s_7_28: const #0s : i
        let s_7_28: i128 = 0;
        // C s_7_29: const #0u : u64
        let s_7_29: u64 = 0;
        // D s_7_30: cast zx s_7_27 -> u64
        let s_7_30: u64 = (s_7_27 as u64);
        // C s_7_31: const #1u : u64
        let s_7_31: u64 = 1;
        // D s_7_32: and s_7_30 s_7_31
        let s_7_32: u64 = ((s_7_30) & (s_7_31));
        // D s_7_33: cmp-eq s_7_32 s_7_31
        let s_7_33: bool = ((s_7_32) == (s_7_31));
        // D s_7_34: lsl s_7_30 s_7_28
        let s_7_34: u64 = s_7_30 << s_7_28;
        // D s_7_35: or s_7_29 s_7_34
        let s_7_35: u64 = ((s_7_29) | (s_7_34));
        // D s_7_36: cmpl s_7_34
        let s_7_36: u64 = !s_7_34;
        // D s_7_37: and s_7_29 s_7_36
        let s_7_37: u64 = ((s_7_29) & (s_7_36));
        // D s_7_38: select s_7_33 s_7_35 s_7_37
        let s_7_38: u64 = if s_7_33 { s_7_35 } else { s_7_37 };
        // D s_7_39: cast trunc s_7_38 -> u8
        let s_7_39: bool = ((s_7_38) != 0);
        // D s_7_40: cast zx s_7_21 -> bv
        let s_7_40: Bits = Bits::new(s_7_21 as u128, 1u16);
        // D s_7_41: cast zx s_7_39 -> bv
        let s_7_41: Bits = Bits::new(s_7_39 as u128, 1u16);
        // D s_7_42: xor s_7_40 s_7_41
        let s_7_42: Bits = ((s_7_40) ^ (s_7_41));
        // D s_7_43: cast reint s_7_42 -> u8
        let s_7_43: bool = ((s_7_42.value()) != 0);
        // D s_7_44: read-var cc:u8
        let s_7_44: u8 = fn_state.cc;
        // D s_7_45: cast zx s_7_44 -> bv
        let s_7_45: Bits = Bits::new(s_7_44 as u128, 2u16);
        // D s_7_46: cast zx s_7_43 -> bv
        let s_7_46: Bits = Bits::new(s_7_43 as u128, 1u16);
        // D s_7_47: cast reint s_7_45 -> u128
        let s_7_47: u128 = (s_7_45.value() as u128);
        // D s_7_48: size-of s_7_45
        let s_7_48: u16 = s_7_45.length();
        // D s_7_49: cast reint s_7_46 -> u128
        let s_7_49: u128 = (s_7_46.value() as u128);
        // D s_7_50: size-of s_7_46
        let s_7_50: u16 = s_7_46.length();
        // D s_7_51: lsl s_7_47 s_7_50
        let s_7_51: u128 = s_7_47 << s_7_50;
        // D s_7_52: or s_7_51 s_7_49
        let s_7_52: u128 = ((s_7_51) | (s_7_49));
        // D s_7_53: add s_7_48 s_7_50
        let s_7_53: u16 = (s_7_48 + s_7_50);
        // D s_7_54: create-bits s_7_52 s_7_53
        let s_7_54: Bits = Bits::new(s_7_52, s_7_53);
        // D s_7_55: cast reint s_7_54 -> u8
        let s_7_55: u8 = (s_7_54.value() as u8);
        // D s_7_56: cast zx s_7_55 -> bv
        let s_7_56: Bits = Bits::new(s_7_55 as u128, 3u16);
        // C s_7_57: const #0u : u8
        let s_7_57: bool = false;
        // C s_7_58: cast zx s_7_57 -> bv
        let s_7_58: Bits = Bits::new(s_7_57 as u128, 1u16);
        // D s_7_59: cast reint s_7_56 -> u128
        let s_7_59: u128 = (s_7_56.value() as u128);
        // D s_7_60: size-of s_7_56
        let s_7_60: u16 = s_7_56.length();
        // C s_7_61: cast reint s_7_58 -> u128
        let s_7_61: u128 = (s_7_58.value() as u128);
        // D s_7_62: size-of s_7_58
        let s_7_62: u16 = s_7_58.length();
        // D s_7_63: lsl s_7_59 s_7_62
        let s_7_63: u128 = s_7_59 << s_7_62;
        // D s_7_64: or s_7_63 s_7_61
        let s_7_64: u128 = ((s_7_63) | (s_7_61));
        // D s_7_65: add s_7_60 s_7_62
        let s_7_65: u16 = (s_7_60 + s_7_62);
        // D s_7_66: create-bits s_7_64 s_7_65
        let s_7_66: Bits = Bits::new(s_7_64, s_7_65);
        // D s_7_67: cast reint s_7_66 -> u8
        let s_7_67: u8 = (s_7_66.value() as u8);
        // D s_7_68: call execute_aarch32_instrs_VSEL_Op_A_txt(s_7_67, s_7_3, s_7_2, s_7_1, s_7_0)
        let s_7_68: () = execute_aarch32_instrs_VSEL_Op_A_txt(
            state,
            tracer,
            s_7_67,
            s_7_3,
            s_7_2,
            s_7_1,
            s_7_0,
        );
        // N s_7_69: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var size:u8
        let s_8_0: u8 = fn_state.size;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 2u16);
        // C s_8_2: const #2u : u8
        let s_8_2: u8 = 2;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 2u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // D s_8_5: not s_8_4
        let s_8_5: bool = !s_8_4;
        // N s_8_6: branch s_8_5 b10 b9
        if s_8_5 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #32s : i64
        let s_9_0: i64 = 32;
        // D s_9_1: write-var esize <= s_9_0
        fn_state.esize = s_9_0;
        // D s_9_2: read-var Vd:u8
        let s_9_2: u8 = fn_state.Vd;
        // D s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 4u16);
        // D s_9_4: read-var D:u8
        let s_9_4: bool = fn_state.D;
        // D s_9_5: cast zx s_9_4 -> bv
        let s_9_5: Bits = Bits::new(s_9_4 as u128, 1u16);
        // D s_9_6: cast reint s_9_3 -> u128
        let s_9_6: u128 = (s_9_3.value() as u128);
        // D s_9_7: size-of s_9_3
        let s_9_7: u16 = s_9_3.length();
        // D s_9_8: cast reint s_9_5 -> u128
        let s_9_8: u128 = (s_9_5.value() as u128);
        // D s_9_9: size-of s_9_5
        let s_9_9: u16 = s_9_5.length();
        // D s_9_10: lsl s_9_6 s_9_9
        let s_9_10: u128 = s_9_6 << s_9_9;
        // D s_9_11: or s_9_10 s_9_8
        let s_9_11: u128 = ((s_9_10) | (s_9_8));
        // D s_9_12: add s_9_7 s_9_9
        let s_9_12: u16 = (s_9_7 + s_9_9);
        // D s_9_13: create-bits s_9_11 s_9_12
        let s_9_13: Bits = Bits::new(s_9_11, s_9_12);
        // D s_9_14: cast reint s_9_13 -> u8
        let s_9_14: u8 = (s_9_13.value() as u8);
        // D s_9_15: cast zx s_9_14 -> bv
        let s_9_15: Bits = Bits::new(s_9_14 as u128, 5u16);
        // D s_9_16: cast zx s_9_15 -> i
        let s_9_16: i128 = (s_9_15.value() as i128);
        // D s_9_17: cast reint s_9_16 -> i64
        let s_9_17: i64 = (s_9_16 as i64);
        // D s_9_18: write-var d <= s_9_17
        fn_state.d = s_9_17;
        // D s_9_19: read-var Vn:u8
        let s_9_19: u8 = fn_state.Vn;
        // D s_9_20: cast zx s_9_19 -> bv
        let s_9_20: Bits = Bits::new(s_9_19 as u128, 4u16);
        // D s_9_21: read-var N:u8
        let s_9_21: bool = fn_state.N;
        // D s_9_22: cast zx s_9_21 -> bv
        let s_9_22: Bits = Bits::new(s_9_21 as u128, 1u16);
        // D s_9_23: cast reint s_9_20 -> u128
        let s_9_23: u128 = (s_9_20.value() as u128);
        // D s_9_24: size-of s_9_20
        let s_9_24: u16 = s_9_20.length();
        // D s_9_25: cast reint s_9_22 -> u128
        let s_9_25: u128 = (s_9_22.value() as u128);
        // D s_9_26: size-of s_9_22
        let s_9_26: u16 = s_9_22.length();
        // D s_9_27: lsl s_9_23 s_9_26
        let s_9_27: u128 = s_9_23 << s_9_26;
        // D s_9_28: or s_9_27 s_9_25
        let s_9_28: u128 = ((s_9_27) | (s_9_25));
        // D s_9_29: add s_9_24 s_9_26
        let s_9_29: u16 = (s_9_24 + s_9_26);
        // D s_9_30: create-bits s_9_28 s_9_29
        let s_9_30: Bits = Bits::new(s_9_28, s_9_29);
        // D s_9_31: cast reint s_9_30 -> u8
        let s_9_31: u8 = (s_9_30.value() as u8);
        // D s_9_32: cast zx s_9_31 -> bv
        let s_9_32: Bits = Bits::new(s_9_31 as u128, 5u16);
        // D s_9_33: cast zx s_9_32 -> i
        let s_9_33: i128 = (s_9_32.value() as i128);
        // D s_9_34: cast reint s_9_33 -> i64
        let s_9_34: i64 = (s_9_33 as i64);
        // D s_9_35: write-var n <= s_9_34
        fn_state.n = s_9_34;
        // D s_9_36: read-var Vm:u8
        let s_9_36: u8 = fn_state.Vm;
        // D s_9_37: cast zx s_9_36 -> bv
        let s_9_37: Bits = Bits::new(s_9_36 as u128, 4u16);
        // D s_9_38: read-var M:u8
        let s_9_38: bool = fn_state.M;
        // D s_9_39: cast zx s_9_38 -> bv
        let s_9_39: Bits = Bits::new(s_9_38 as u128, 1u16);
        // D s_9_40: cast reint s_9_37 -> u128
        let s_9_40: u128 = (s_9_37.value() as u128);
        // D s_9_41: size-of s_9_37
        let s_9_41: u16 = s_9_37.length();
        // D s_9_42: cast reint s_9_39 -> u128
        let s_9_42: u128 = (s_9_39.value() as u128);
        // D s_9_43: size-of s_9_39
        let s_9_43: u16 = s_9_39.length();
        // D s_9_44: lsl s_9_40 s_9_43
        let s_9_44: u128 = s_9_40 << s_9_43;
        // D s_9_45: or s_9_44 s_9_42
        let s_9_45: u128 = ((s_9_44) | (s_9_42));
        // D s_9_46: add s_9_41 s_9_43
        let s_9_46: u16 = (s_9_41 + s_9_43);
        // D s_9_47: create-bits s_9_45 s_9_46
        let s_9_47: Bits = Bits::new(s_9_45, s_9_46);
        // D s_9_48: cast reint s_9_47 -> u8
        let s_9_48: u8 = (s_9_47.value() as u8);
        // D s_9_49: cast zx s_9_48 -> bv
        let s_9_49: Bits = Bits::new(s_9_48 as u128, 5u16);
        // D s_9_50: cast zx s_9_49 -> i
        let s_9_50: i128 = (s_9_49.value() as i128);
        // D s_9_51: cast reint s_9_50 -> i64
        let s_9_51: i64 = (s_9_50 as i64);
        // D s_9_52: write-var m <= s_9_51
        fn_state.m = s_9_51;
        // N s_9_53: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var size:u8
        let s_10_0: u8 = fn_state.size;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 2u16);
        // C s_10_2: const #3u : u8
        let s_10_2: u8 = 3;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 2u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // D s_10_5: not s_10_4
        let s_10_5: bool = !s_10_4;
        // N s_10_6: branch s_10_5 b12 b11
        if s_10_5 {
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
        // C s_11_0: const #64s : i64
        let s_11_0: i64 = 64;
        // D s_11_1: write-var esize <= s_11_0
        fn_state.esize = s_11_0;
        // D s_11_2: read-var D:u8
        let s_11_2: bool = fn_state.D;
        // D s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 1u16);
        // D s_11_4: read-var Vd:u8
        let s_11_4: u8 = fn_state.Vd;
        // D s_11_5: cast zx s_11_4 -> bv
        let s_11_5: Bits = Bits::new(s_11_4 as u128, 4u16);
        // D s_11_6: cast reint s_11_3 -> u128
        let s_11_6: u128 = (s_11_3.value() as u128);
        // D s_11_7: size-of s_11_3
        let s_11_7: u16 = s_11_3.length();
        // D s_11_8: cast reint s_11_5 -> u128
        let s_11_8: u128 = (s_11_5.value() as u128);
        // D s_11_9: size-of s_11_5
        let s_11_9: u16 = s_11_5.length();
        // D s_11_10: lsl s_11_6 s_11_9
        let s_11_10: u128 = s_11_6 << s_11_9;
        // D s_11_11: or s_11_10 s_11_8
        let s_11_11: u128 = ((s_11_10) | (s_11_8));
        // D s_11_12: add s_11_7 s_11_9
        let s_11_12: u16 = (s_11_7 + s_11_9);
        // D s_11_13: create-bits s_11_11 s_11_12
        let s_11_13: Bits = Bits::new(s_11_11, s_11_12);
        // D s_11_14: cast reint s_11_13 -> u8
        let s_11_14: u8 = (s_11_13.value() as u8);
        // D s_11_15: cast zx s_11_14 -> bv
        let s_11_15: Bits = Bits::new(s_11_14 as u128, 5u16);
        // D s_11_16: cast zx s_11_15 -> i
        let s_11_16: i128 = (s_11_15.value() as i128);
        // D s_11_17: cast reint s_11_16 -> i64
        let s_11_17: i64 = (s_11_16 as i64);
        // D s_11_18: write-var d <= s_11_17
        fn_state.d = s_11_17;
        // D s_11_19: read-var N:u8
        let s_11_19: bool = fn_state.N;
        // D s_11_20: cast zx s_11_19 -> bv
        let s_11_20: Bits = Bits::new(s_11_19 as u128, 1u16);
        // D s_11_21: read-var Vn:u8
        let s_11_21: u8 = fn_state.Vn;
        // D s_11_22: cast zx s_11_21 -> bv
        let s_11_22: Bits = Bits::new(s_11_21 as u128, 4u16);
        // D s_11_23: cast reint s_11_20 -> u128
        let s_11_23: u128 = (s_11_20.value() as u128);
        // D s_11_24: size-of s_11_20
        let s_11_24: u16 = s_11_20.length();
        // D s_11_25: cast reint s_11_22 -> u128
        let s_11_25: u128 = (s_11_22.value() as u128);
        // D s_11_26: size-of s_11_22
        let s_11_26: u16 = s_11_22.length();
        // D s_11_27: lsl s_11_23 s_11_26
        let s_11_27: u128 = s_11_23 << s_11_26;
        // D s_11_28: or s_11_27 s_11_25
        let s_11_28: u128 = ((s_11_27) | (s_11_25));
        // D s_11_29: add s_11_24 s_11_26
        let s_11_29: u16 = (s_11_24 + s_11_26);
        // D s_11_30: create-bits s_11_28 s_11_29
        let s_11_30: Bits = Bits::new(s_11_28, s_11_29);
        // D s_11_31: cast reint s_11_30 -> u8
        let s_11_31: u8 = (s_11_30.value() as u8);
        // D s_11_32: cast zx s_11_31 -> bv
        let s_11_32: Bits = Bits::new(s_11_31 as u128, 5u16);
        // D s_11_33: cast zx s_11_32 -> i
        let s_11_33: i128 = (s_11_32.value() as i128);
        // D s_11_34: cast reint s_11_33 -> i64
        let s_11_34: i64 = (s_11_33 as i64);
        // D s_11_35: write-var n <= s_11_34
        fn_state.n = s_11_34;
        // D s_11_36: read-var M:u8
        let s_11_36: bool = fn_state.M;
        // D s_11_37: cast zx s_11_36 -> bv
        let s_11_37: Bits = Bits::new(s_11_36 as u128, 1u16);
        // D s_11_38: read-var Vm:u8
        let s_11_38: u8 = fn_state.Vm;
        // D s_11_39: cast zx s_11_38 -> bv
        let s_11_39: Bits = Bits::new(s_11_38 as u128, 4u16);
        // D s_11_40: cast reint s_11_37 -> u128
        let s_11_40: u128 = (s_11_37.value() as u128);
        // D s_11_41: size-of s_11_37
        let s_11_41: u16 = s_11_37.length();
        // D s_11_42: cast reint s_11_39 -> u128
        let s_11_42: u128 = (s_11_39.value() as u128);
        // D s_11_43: size-of s_11_39
        let s_11_43: u16 = s_11_39.length();
        // D s_11_44: lsl s_11_40 s_11_43
        let s_11_44: u128 = s_11_40 << s_11_43;
        // D s_11_45: or s_11_44 s_11_42
        let s_11_45: u128 = ((s_11_44) | (s_11_42));
        // D s_11_46: add s_11_41 s_11_43
        let s_11_46: u16 = (s_11_41 + s_11_43);
        // D s_11_47: create-bits s_11_45 s_11_46
        let s_11_47: Bits = Bits::new(s_11_45, s_11_46);
        // D s_11_48: cast reint s_11_47 -> u8
        let s_11_48: u8 = (s_11_47.value() as u8);
        // D s_11_49: cast zx s_11_48 -> bv
        let s_11_49: Bits = Bits::new(s_11_48 as u128, 5u16);
        // D s_11_50: cast zx s_11_49 -> i
        let s_11_50: i128 = (s_11_49.value() as i128);
        // D s_11_51: cast reint s_11_50 -> i64
        let s_11_51: i64 = (s_11_50 as i64);
        // D s_11_52: write-var m <= s_11_51
        fn_state.m = s_11_51;
        // N s_11_53: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: panic
        panic!("{:?}", ());
        // N s_13_1: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call HaveFP16Ext(s_14_0)
        let s_14_1: bool = HaveFP16Ext(state, tracer, s_14_0);
        // S s_14_2: not s_14_1
        let s_14_2: bool = !s_14_1;
        // D s_14_3: write-var gs#325885 <= s_14_2
        fn_state.gs_325885 = s_14_2;
        // N s_14_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var gs#325886 <= s_15_0
        fn_state.gs_325886 = s_15_0;
        // N s_15_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
