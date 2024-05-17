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
use InITBlock::*;
use execute_aarch32_instrs_VSEL_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VSEL_T1enc_A_txt<T: Tracer>(
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
        n: i64,
        gs_325919: bool,
        gs_325920: bool,
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
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call InITBlock(s_0_0)
        let s_0_1: bool = InITBlock(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b17 b1
        if s_0_1 {
            return block_17(state, tracer, fn_state);
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
        // C s_1_2: const #0u : u8
        let s_1_2: u8 = 0;
        // C s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 2u16);
        // D s_1_4: cmp-eq s_1_1 s_1_3
        let s_1_4: bool = ((s_1_1) == (s_1_3));
        // N s_1_5: branch s_1_4 b16 b2
        if s_1_4 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var size:u8
        let s_2_0: u8 = fn_state.size;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // C s_2_2: const #1u : u8
        let s_2_2: u8 = 1;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 2u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b15 b3
        if s_2_4 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#325919 <= s_3_0
        fn_state.gs_325919 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#325919:u8
        let s_4_0: bool = fn_state.gs_325919;
        // D s_4_1: write-var gs#325920 <= s_4_0
        fn_state.gs_325920 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#325920:u8
        let s_5_0: bool = fn_state.gs_325920;
        // N s_5_1: branch s_5_0 b14 b6
        if s_5_0 {
            return block_14(state, tracer, fn_state);
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
        // D s_6_2: read-var size:u8
        let s_6_2: u8 = fn_state.size;
        // D s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 2u16);
        // C s_6_4: const #1u : u8
        let s_6_4: u8 = 1;
        // C s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 2u16);
        // D s_6_6: cmp-eq s_6_3 s_6_5
        let s_6_6: bool = ((s_6_3) == (s_6_5));
        // D s_6_7: not s_6_6
        let s_6_7: bool = !s_6_6;
        // N s_6_8: branch s_6_7 b9 b7
        if s_6_7 {
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
        // C s_7_0: const #16s : i64
        let s_7_0: i64 = 16;
        // D s_7_1: write-var esize <= s_7_0
        fn_state.esize = s_7_0;
        // D s_7_2: read-var Vd:u8
        let s_7_2: u8 = fn_state.Vd;
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 4u16);
        // D s_7_4: read-var D:u8
        let s_7_4: bool = fn_state.D;
        // D s_7_5: cast zx s_7_4 -> bv
        let s_7_5: Bits = Bits::new(s_7_4 as u128, 1u16);
        // D s_7_6: cast reint s_7_3 -> u128
        let s_7_6: u128 = (s_7_3.value() as u128);
        // D s_7_7: size-of s_7_3
        let s_7_7: u16 = s_7_3.length();
        // D s_7_8: cast reint s_7_5 -> u128
        let s_7_8: u128 = (s_7_5.value() as u128);
        // D s_7_9: size-of s_7_5
        let s_7_9: u16 = s_7_5.length();
        // D s_7_10: lsl s_7_6 s_7_9
        let s_7_10: u128 = s_7_6 << s_7_9;
        // D s_7_11: or s_7_10 s_7_8
        let s_7_11: u128 = ((s_7_10) | (s_7_8));
        // D s_7_12: add s_7_7 s_7_9
        let s_7_12: u16 = (s_7_7 + s_7_9);
        // D s_7_13: create-bits s_7_11 s_7_12
        let s_7_13: Bits = Bits::new(s_7_11, s_7_12);
        // D s_7_14: cast reint s_7_13 -> u8
        let s_7_14: u8 = (s_7_13.value() as u8);
        // D s_7_15: cast zx s_7_14 -> bv
        let s_7_15: Bits = Bits::new(s_7_14 as u128, 5u16);
        // D s_7_16: cast zx s_7_15 -> i
        let s_7_16: i128 = (s_7_15.value() as i128);
        // D s_7_17: cast reint s_7_16 -> i64
        let s_7_17: i64 = (s_7_16 as i64);
        // D s_7_18: write-var d <= s_7_17
        fn_state.d = s_7_17;
        // D s_7_19: read-var Vn:u8
        let s_7_19: u8 = fn_state.Vn;
        // D s_7_20: cast zx s_7_19 -> bv
        let s_7_20: Bits = Bits::new(s_7_19 as u128, 4u16);
        // D s_7_21: read-var N:u8
        let s_7_21: bool = fn_state.N;
        // D s_7_22: cast zx s_7_21 -> bv
        let s_7_22: Bits = Bits::new(s_7_21 as u128, 1u16);
        // D s_7_23: cast reint s_7_20 -> u128
        let s_7_23: u128 = (s_7_20.value() as u128);
        // D s_7_24: size-of s_7_20
        let s_7_24: u16 = s_7_20.length();
        // D s_7_25: cast reint s_7_22 -> u128
        let s_7_25: u128 = (s_7_22.value() as u128);
        // D s_7_26: size-of s_7_22
        let s_7_26: u16 = s_7_22.length();
        // D s_7_27: lsl s_7_23 s_7_26
        let s_7_27: u128 = s_7_23 << s_7_26;
        // D s_7_28: or s_7_27 s_7_25
        let s_7_28: u128 = ((s_7_27) | (s_7_25));
        // D s_7_29: add s_7_24 s_7_26
        let s_7_29: u16 = (s_7_24 + s_7_26);
        // D s_7_30: create-bits s_7_28 s_7_29
        let s_7_30: Bits = Bits::new(s_7_28, s_7_29);
        // D s_7_31: cast reint s_7_30 -> u8
        let s_7_31: u8 = (s_7_30.value() as u8);
        // D s_7_32: cast zx s_7_31 -> bv
        let s_7_32: Bits = Bits::new(s_7_31 as u128, 5u16);
        // D s_7_33: cast zx s_7_32 -> i
        let s_7_33: i128 = (s_7_32.value() as i128);
        // D s_7_34: cast reint s_7_33 -> i64
        let s_7_34: i64 = (s_7_33 as i64);
        // D s_7_35: write-var n <= s_7_34
        fn_state.n = s_7_34;
        // D s_7_36: read-var Vm:u8
        let s_7_36: u8 = fn_state.Vm;
        // D s_7_37: cast zx s_7_36 -> bv
        let s_7_37: Bits = Bits::new(s_7_36 as u128, 4u16);
        // D s_7_38: read-var M:u8
        let s_7_38: bool = fn_state.M;
        // D s_7_39: cast zx s_7_38 -> bv
        let s_7_39: Bits = Bits::new(s_7_38 as u128, 1u16);
        // D s_7_40: cast reint s_7_37 -> u128
        let s_7_40: u128 = (s_7_37.value() as u128);
        // D s_7_41: size-of s_7_37
        let s_7_41: u16 = s_7_37.length();
        // D s_7_42: cast reint s_7_39 -> u128
        let s_7_42: u128 = (s_7_39.value() as u128);
        // D s_7_43: size-of s_7_39
        let s_7_43: u16 = s_7_39.length();
        // D s_7_44: lsl s_7_40 s_7_43
        let s_7_44: u128 = s_7_40 << s_7_43;
        // D s_7_45: or s_7_44 s_7_42
        let s_7_45: u128 = ((s_7_44) | (s_7_42));
        // D s_7_46: add s_7_41 s_7_43
        let s_7_46: u16 = (s_7_41 + s_7_43);
        // D s_7_47: create-bits s_7_45 s_7_46
        let s_7_47: Bits = Bits::new(s_7_45, s_7_46);
        // D s_7_48: cast reint s_7_47 -> u8
        let s_7_48: u8 = (s_7_47.value() as u8);
        // D s_7_49: cast zx s_7_48 -> bv
        let s_7_49: Bits = Bits::new(s_7_48 as u128, 5u16);
        // D s_7_50: cast zx s_7_49 -> i
        let s_7_50: i128 = (s_7_49.value() as i128);
        // D s_7_51: cast reint s_7_50 -> i64
        let s_7_51: i64 = (s_7_50 as i64);
        // D s_7_52: write-var m <= s_7_51
        fn_state.m = s_7_51;
        // N s_7_53: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var n:i64
        let s_8_0: i64 = fn_state.n;
        // D s_8_1: read-var m:i64
        let s_8_1: i64 = fn_state.m;
        // D s_8_2: read-var esize:i64
        let s_8_2: i64 = fn_state.esize;
        // D s_8_3: read-var d:i64
        let s_8_3: i64 = fn_state.d;
        // C s_8_4: const #1s : i
        let s_8_4: i128 = 1;
        // D s_8_5: read-var cc:u8
        let s_8_5: u8 = fn_state.cc;
        // D s_8_6: cast zx s_8_5 -> bv
        let s_8_6: Bits = Bits::new(s_8_5 as u128, 2u16);
        // C s_8_7: const #1u : u64
        let s_8_7: u64 = 1;
        // D s_8_8: bit-extract s_8_6 s_8_4 s_8_7
        let s_8_8: Bits = (Bits::new(
            ((s_8_6) >> (s_8_4)).value(),
            u16::try_from(s_8_7).unwrap(),
        ));
        // D s_8_9: cast reint s_8_8 -> u8
        let s_8_9: bool = ((s_8_8.value()) != 0);
        // C s_8_10: const #0s : i
        let s_8_10: i128 = 0;
        // C s_8_11: const #0u : u64
        let s_8_11: u64 = 0;
        // D s_8_12: cast zx s_8_9 -> u64
        let s_8_12: u64 = (s_8_9 as u64);
        // C s_8_13: const #1u : u64
        let s_8_13: u64 = 1;
        // D s_8_14: and s_8_12 s_8_13
        let s_8_14: u64 = ((s_8_12) & (s_8_13));
        // D s_8_15: cmp-eq s_8_14 s_8_13
        let s_8_15: bool = ((s_8_14) == (s_8_13));
        // D s_8_16: lsl s_8_12 s_8_10
        let s_8_16: u64 = s_8_12 << s_8_10;
        // D s_8_17: or s_8_11 s_8_16
        let s_8_17: u64 = ((s_8_11) | (s_8_16));
        // D s_8_18: cmpl s_8_16
        let s_8_18: u64 = !s_8_16;
        // D s_8_19: and s_8_11 s_8_18
        let s_8_19: u64 = ((s_8_11) & (s_8_18));
        // D s_8_20: select s_8_15 s_8_17 s_8_19
        let s_8_20: u64 = if s_8_15 { s_8_17 } else { s_8_19 };
        // D s_8_21: cast trunc s_8_20 -> u8
        let s_8_21: bool = ((s_8_20) != 0);
        // C s_8_22: const #0s : i
        let s_8_22: i128 = 0;
        // D s_8_23: read-var cc:u8
        let s_8_23: u8 = fn_state.cc;
        // D s_8_24: cast zx s_8_23 -> bv
        let s_8_24: Bits = Bits::new(s_8_23 as u128, 2u16);
        // C s_8_25: const #1u : u64
        let s_8_25: u64 = 1;
        // D s_8_26: bit-extract s_8_24 s_8_22 s_8_25
        let s_8_26: Bits = (Bits::new(
            ((s_8_24) >> (s_8_22)).value(),
            u16::try_from(s_8_25).unwrap(),
        ));
        // D s_8_27: cast reint s_8_26 -> u8
        let s_8_27: bool = ((s_8_26.value()) != 0);
        // C s_8_28: const #0s : i
        let s_8_28: i128 = 0;
        // C s_8_29: const #0u : u64
        let s_8_29: u64 = 0;
        // D s_8_30: cast zx s_8_27 -> u64
        let s_8_30: u64 = (s_8_27 as u64);
        // C s_8_31: const #1u : u64
        let s_8_31: u64 = 1;
        // D s_8_32: and s_8_30 s_8_31
        let s_8_32: u64 = ((s_8_30) & (s_8_31));
        // D s_8_33: cmp-eq s_8_32 s_8_31
        let s_8_33: bool = ((s_8_32) == (s_8_31));
        // D s_8_34: lsl s_8_30 s_8_28
        let s_8_34: u64 = s_8_30 << s_8_28;
        // D s_8_35: or s_8_29 s_8_34
        let s_8_35: u64 = ((s_8_29) | (s_8_34));
        // D s_8_36: cmpl s_8_34
        let s_8_36: u64 = !s_8_34;
        // D s_8_37: and s_8_29 s_8_36
        let s_8_37: u64 = ((s_8_29) & (s_8_36));
        // D s_8_38: select s_8_33 s_8_35 s_8_37
        let s_8_38: u64 = if s_8_33 { s_8_35 } else { s_8_37 };
        // D s_8_39: cast trunc s_8_38 -> u8
        let s_8_39: bool = ((s_8_38) != 0);
        // D s_8_40: cast zx s_8_21 -> bv
        let s_8_40: Bits = Bits::new(s_8_21 as u128, 1u16);
        // D s_8_41: cast zx s_8_39 -> bv
        let s_8_41: Bits = Bits::new(s_8_39 as u128, 1u16);
        // D s_8_42: xor s_8_40 s_8_41
        let s_8_42: Bits = ((s_8_40) ^ (s_8_41));
        // D s_8_43: cast reint s_8_42 -> u8
        let s_8_43: bool = ((s_8_42.value()) != 0);
        // D s_8_44: read-var cc:u8
        let s_8_44: u8 = fn_state.cc;
        // D s_8_45: cast zx s_8_44 -> bv
        let s_8_45: Bits = Bits::new(s_8_44 as u128, 2u16);
        // D s_8_46: cast zx s_8_43 -> bv
        let s_8_46: Bits = Bits::new(s_8_43 as u128, 1u16);
        // D s_8_47: cast reint s_8_45 -> u128
        let s_8_47: u128 = (s_8_45.value() as u128);
        // D s_8_48: size-of s_8_45
        let s_8_48: u16 = s_8_45.length();
        // D s_8_49: cast reint s_8_46 -> u128
        let s_8_49: u128 = (s_8_46.value() as u128);
        // D s_8_50: size-of s_8_46
        let s_8_50: u16 = s_8_46.length();
        // D s_8_51: lsl s_8_47 s_8_50
        let s_8_51: u128 = s_8_47 << s_8_50;
        // D s_8_52: or s_8_51 s_8_49
        let s_8_52: u128 = ((s_8_51) | (s_8_49));
        // D s_8_53: add s_8_48 s_8_50
        let s_8_53: u16 = (s_8_48 + s_8_50);
        // D s_8_54: create-bits s_8_52 s_8_53
        let s_8_54: Bits = Bits::new(s_8_52, s_8_53);
        // D s_8_55: cast reint s_8_54 -> u8
        let s_8_55: u8 = (s_8_54.value() as u8);
        // D s_8_56: cast zx s_8_55 -> bv
        let s_8_56: Bits = Bits::new(s_8_55 as u128, 3u16);
        // C s_8_57: const #0u : u8
        let s_8_57: bool = false;
        // C s_8_58: cast zx s_8_57 -> bv
        let s_8_58: Bits = Bits::new(s_8_57 as u128, 1u16);
        // D s_8_59: cast reint s_8_56 -> u128
        let s_8_59: u128 = (s_8_56.value() as u128);
        // D s_8_60: size-of s_8_56
        let s_8_60: u16 = s_8_56.length();
        // C s_8_61: cast reint s_8_58 -> u128
        let s_8_61: u128 = (s_8_58.value() as u128);
        // D s_8_62: size-of s_8_58
        let s_8_62: u16 = s_8_58.length();
        // D s_8_63: lsl s_8_59 s_8_62
        let s_8_63: u128 = s_8_59 << s_8_62;
        // D s_8_64: or s_8_63 s_8_61
        let s_8_64: u128 = ((s_8_63) | (s_8_61));
        // D s_8_65: add s_8_60 s_8_62
        let s_8_65: u16 = (s_8_60 + s_8_62);
        // D s_8_66: create-bits s_8_64 s_8_65
        let s_8_66: Bits = Bits::new(s_8_64, s_8_65);
        // D s_8_67: cast reint s_8_66 -> u8
        let s_8_67: u8 = (s_8_66.value() as u8);
        // D s_8_68: call execute_aarch32_instrs_VSEL_Op_A_txt(s_8_67, s_8_3, s_8_2, s_8_1, s_8_0)
        let s_8_68: () = execute_aarch32_instrs_VSEL_Op_A_txt(
            state,
            tracer,
            s_8_67,
            s_8_3,
            s_8_2,
            s_8_1,
            s_8_0,
        );
        // N s_8_69: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var size:u8
        let s_9_0: u8 = fn_state.size;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 2u16);
        // C s_9_2: const #2u : u8
        let s_9_2: u8 = 2;
        // C s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 2u16);
        // D s_9_4: cmp-eq s_9_1 s_9_3
        let s_9_4: bool = ((s_9_1) == (s_9_3));
        // D s_9_5: not s_9_4
        let s_9_5: bool = !s_9_4;
        // N s_9_6: branch s_9_5 b11 b10
        if s_9_5 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #32s : i64
        let s_10_0: i64 = 32;
        // D s_10_1: write-var esize <= s_10_0
        fn_state.esize = s_10_0;
        // D s_10_2: read-var Vd:u8
        let s_10_2: u8 = fn_state.Vd;
        // D s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 4u16);
        // D s_10_4: read-var D:u8
        let s_10_4: bool = fn_state.D;
        // D s_10_5: cast zx s_10_4 -> bv
        let s_10_5: Bits = Bits::new(s_10_4 as u128, 1u16);
        // D s_10_6: cast reint s_10_3 -> u128
        let s_10_6: u128 = (s_10_3.value() as u128);
        // D s_10_7: size-of s_10_3
        let s_10_7: u16 = s_10_3.length();
        // D s_10_8: cast reint s_10_5 -> u128
        let s_10_8: u128 = (s_10_5.value() as u128);
        // D s_10_9: size-of s_10_5
        let s_10_9: u16 = s_10_5.length();
        // D s_10_10: lsl s_10_6 s_10_9
        let s_10_10: u128 = s_10_6 << s_10_9;
        // D s_10_11: or s_10_10 s_10_8
        let s_10_11: u128 = ((s_10_10) | (s_10_8));
        // D s_10_12: add s_10_7 s_10_9
        let s_10_12: u16 = (s_10_7 + s_10_9);
        // D s_10_13: create-bits s_10_11 s_10_12
        let s_10_13: Bits = Bits::new(s_10_11, s_10_12);
        // D s_10_14: cast reint s_10_13 -> u8
        let s_10_14: u8 = (s_10_13.value() as u8);
        // D s_10_15: cast zx s_10_14 -> bv
        let s_10_15: Bits = Bits::new(s_10_14 as u128, 5u16);
        // D s_10_16: cast zx s_10_15 -> i
        let s_10_16: i128 = (s_10_15.value() as i128);
        // D s_10_17: cast reint s_10_16 -> i64
        let s_10_17: i64 = (s_10_16 as i64);
        // D s_10_18: write-var d <= s_10_17
        fn_state.d = s_10_17;
        // D s_10_19: read-var Vn:u8
        let s_10_19: u8 = fn_state.Vn;
        // D s_10_20: cast zx s_10_19 -> bv
        let s_10_20: Bits = Bits::new(s_10_19 as u128, 4u16);
        // D s_10_21: read-var N:u8
        let s_10_21: bool = fn_state.N;
        // D s_10_22: cast zx s_10_21 -> bv
        let s_10_22: Bits = Bits::new(s_10_21 as u128, 1u16);
        // D s_10_23: cast reint s_10_20 -> u128
        let s_10_23: u128 = (s_10_20.value() as u128);
        // D s_10_24: size-of s_10_20
        let s_10_24: u16 = s_10_20.length();
        // D s_10_25: cast reint s_10_22 -> u128
        let s_10_25: u128 = (s_10_22.value() as u128);
        // D s_10_26: size-of s_10_22
        let s_10_26: u16 = s_10_22.length();
        // D s_10_27: lsl s_10_23 s_10_26
        let s_10_27: u128 = s_10_23 << s_10_26;
        // D s_10_28: or s_10_27 s_10_25
        let s_10_28: u128 = ((s_10_27) | (s_10_25));
        // D s_10_29: add s_10_24 s_10_26
        let s_10_29: u16 = (s_10_24 + s_10_26);
        // D s_10_30: create-bits s_10_28 s_10_29
        let s_10_30: Bits = Bits::new(s_10_28, s_10_29);
        // D s_10_31: cast reint s_10_30 -> u8
        let s_10_31: u8 = (s_10_30.value() as u8);
        // D s_10_32: cast zx s_10_31 -> bv
        let s_10_32: Bits = Bits::new(s_10_31 as u128, 5u16);
        // D s_10_33: cast zx s_10_32 -> i
        let s_10_33: i128 = (s_10_32.value() as i128);
        // D s_10_34: cast reint s_10_33 -> i64
        let s_10_34: i64 = (s_10_33 as i64);
        // D s_10_35: write-var n <= s_10_34
        fn_state.n = s_10_34;
        // D s_10_36: read-var Vm:u8
        let s_10_36: u8 = fn_state.Vm;
        // D s_10_37: cast zx s_10_36 -> bv
        let s_10_37: Bits = Bits::new(s_10_36 as u128, 4u16);
        // D s_10_38: read-var M:u8
        let s_10_38: bool = fn_state.M;
        // D s_10_39: cast zx s_10_38 -> bv
        let s_10_39: Bits = Bits::new(s_10_38 as u128, 1u16);
        // D s_10_40: cast reint s_10_37 -> u128
        let s_10_40: u128 = (s_10_37.value() as u128);
        // D s_10_41: size-of s_10_37
        let s_10_41: u16 = s_10_37.length();
        // D s_10_42: cast reint s_10_39 -> u128
        let s_10_42: u128 = (s_10_39.value() as u128);
        // D s_10_43: size-of s_10_39
        let s_10_43: u16 = s_10_39.length();
        // D s_10_44: lsl s_10_40 s_10_43
        let s_10_44: u128 = s_10_40 << s_10_43;
        // D s_10_45: or s_10_44 s_10_42
        let s_10_45: u128 = ((s_10_44) | (s_10_42));
        // D s_10_46: add s_10_41 s_10_43
        let s_10_46: u16 = (s_10_41 + s_10_43);
        // D s_10_47: create-bits s_10_45 s_10_46
        let s_10_47: Bits = Bits::new(s_10_45, s_10_46);
        // D s_10_48: cast reint s_10_47 -> u8
        let s_10_48: u8 = (s_10_47.value() as u8);
        // D s_10_49: cast zx s_10_48 -> bv
        let s_10_49: Bits = Bits::new(s_10_48 as u128, 5u16);
        // D s_10_50: cast zx s_10_49 -> i
        let s_10_50: i128 = (s_10_49.value() as i128);
        // D s_10_51: cast reint s_10_50 -> i64
        let s_10_51: i64 = (s_10_50 as i64);
        // D s_10_52: write-var m <= s_10_51
        fn_state.m = s_10_51;
        // N s_10_53: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var size:u8
        let s_11_0: u8 = fn_state.size;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 2u16);
        // C s_11_2: const #3u : u8
        let s_11_2: u8 = 3;
        // C s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 2u16);
        // D s_11_4: cmp-eq s_11_1 s_11_3
        let s_11_4: bool = ((s_11_1) == (s_11_3));
        // D s_11_5: not s_11_4
        let s_11_5: bool = !s_11_4;
        // N s_11_6: branch s_11_5 b13 b12
        if s_11_5 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #64s : i64
        let s_12_0: i64 = 64;
        // D s_12_1: write-var esize <= s_12_0
        fn_state.esize = s_12_0;
        // D s_12_2: read-var D:u8
        let s_12_2: bool = fn_state.D;
        // D s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 1u16);
        // D s_12_4: read-var Vd:u8
        let s_12_4: u8 = fn_state.Vd;
        // D s_12_5: cast zx s_12_4 -> bv
        let s_12_5: Bits = Bits::new(s_12_4 as u128, 4u16);
        // D s_12_6: cast reint s_12_3 -> u128
        let s_12_6: u128 = (s_12_3.value() as u128);
        // D s_12_7: size-of s_12_3
        let s_12_7: u16 = s_12_3.length();
        // D s_12_8: cast reint s_12_5 -> u128
        let s_12_8: u128 = (s_12_5.value() as u128);
        // D s_12_9: size-of s_12_5
        let s_12_9: u16 = s_12_5.length();
        // D s_12_10: lsl s_12_6 s_12_9
        let s_12_10: u128 = s_12_6 << s_12_9;
        // D s_12_11: or s_12_10 s_12_8
        let s_12_11: u128 = ((s_12_10) | (s_12_8));
        // D s_12_12: add s_12_7 s_12_9
        let s_12_12: u16 = (s_12_7 + s_12_9);
        // D s_12_13: create-bits s_12_11 s_12_12
        let s_12_13: Bits = Bits::new(s_12_11, s_12_12);
        // D s_12_14: cast reint s_12_13 -> u8
        let s_12_14: u8 = (s_12_13.value() as u8);
        // D s_12_15: cast zx s_12_14 -> bv
        let s_12_15: Bits = Bits::new(s_12_14 as u128, 5u16);
        // D s_12_16: cast zx s_12_15 -> i
        let s_12_16: i128 = (s_12_15.value() as i128);
        // D s_12_17: cast reint s_12_16 -> i64
        let s_12_17: i64 = (s_12_16 as i64);
        // D s_12_18: write-var d <= s_12_17
        fn_state.d = s_12_17;
        // D s_12_19: read-var N:u8
        let s_12_19: bool = fn_state.N;
        // D s_12_20: cast zx s_12_19 -> bv
        let s_12_20: Bits = Bits::new(s_12_19 as u128, 1u16);
        // D s_12_21: read-var Vn:u8
        let s_12_21: u8 = fn_state.Vn;
        // D s_12_22: cast zx s_12_21 -> bv
        let s_12_22: Bits = Bits::new(s_12_21 as u128, 4u16);
        // D s_12_23: cast reint s_12_20 -> u128
        let s_12_23: u128 = (s_12_20.value() as u128);
        // D s_12_24: size-of s_12_20
        let s_12_24: u16 = s_12_20.length();
        // D s_12_25: cast reint s_12_22 -> u128
        let s_12_25: u128 = (s_12_22.value() as u128);
        // D s_12_26: size-of s_12_22
        let s_12_26: u16 = s_12_22.length();
        // D s_12_27: lsl s_12_23 s_12_26
        let s_12_27: u128 = s_12_23 << s_12_26;
        // D s_12_28: or s_12_27 s_12_25
        let s_12_28: u128 = ((s_12_27) | (s_12_25));
        // D s_12_29: add s_12_24 s_12_26
        let s_12_29: u16 = (s_12_24 + s_12_26);
        // D s_12_30: create-bits s_12_28 s_12_29
        let s_12_30: Bits = Bits::new(s_12_28, s_12_29);
        // D s_12_31: cast reint s_12_30 -> u8
        let s_12_31: u8 = (s_12_30.value() as u8);
        // D s_12_32: cast zx s_12_31 -> bv
        let s_12_32: Bits = Bits::new(s_12_31 as u128, 5u16);
        // D s_12_33: cast zx s_12_32 -> i
        let s_12_33: i128 = (s_12_32.value() as i128);
        // D s_12_34: cast reint s_12_33 -> i64
        let s_12_34: i64 = (s_12_33 as i64);
        // D s_12_35: write-var n <= s_12_34
        fn_state.n = s_12_34;
        // D s_12_36: read-var M:u8
        let s_12_36: bool = fn_state.M;
        // D s_12_37: cast zx s_12_36 -> bv
        let s_12_37: Bits = Bits::new(s_12_36 as u128, 1u16);
        // D s_12_38: read-var Vm:u8
        let s_12_38: u8 = fn_state.Vm;
        // D s_12_39: cast zx s_12_38 -> bv
        let s_12_39: Bits = Bits::new(s_12_38 as u128, 4u16);
        // D s_12_40: cast reint s_12_37 -> u128
        let s_12_40: u128 = (s_12_37.value() as u128);
        // D s_12_41: size-of s_12_37
        let s_12_41: u16 = s_12_37.length();
        // D s_12_42: cast reint s_12_39 -> u128
        let s_12_42: u128 = (s_12_39.value() as u128);
        // D s_12_43: size-of s_12_39
        let s_12_43: u16 = s_12_39.length();
        // D s_12_44: lsl s_12_40 s_12_43
        let s_12_44: u128 = s_12_40 << s_12_43;
        // D s_12_45: or s_12_44 s_12_42
        let s_12_45: u128 = ((s_12_44) | (s_12_42));
        // D s_12_46: add s_12_41 s_12_43
        let s_12_46: u16 = (s_12_41 + s_12_43);
        // D s_12_47: create-bits s_12_45 s_12_46
        let s_12_47: Bits = Bits::new(s_12_45, s_12_46);
        // D s_12_48: cast reint s_12_47 -> u8
        let s_12_48: u8 = (s_12_47.value() as u8);
        // D s_12_49: cast zx s_12_48 -> bv
        let s_12_49: Bits = Bits::new(s_12_48 as u128, 5u16);
        // D s_12_50: cast zx s_12_49 -> i
        let s_12_50: i128 = (s_12_49.value() as i128);
        // D s_12_51: cast reint s_12_50 -> i64
        let s_12_51: i64 = (s_12_50 as i64);
        // D s_12_52: write-var m <= s_12_51
        fn_state.m = s_12_51;
        // N s_12_53: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: panic
        panic!("{:?}", ());
        // N s_14_1: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call HaveFP16Ext(s_15_0)
        let s_15_1: bool = HaveFP16Ext(state, tracer, s_15_0);
        // S s_15_2: not s_15_1
        let s_15_2: bool = !s_15_1;
        // D s_15_3: write-var gs#325919 <= s_15_2
        fn_state.gs_325919 = s_15_2;
        // N s_15_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#325920 <= s_16_0
        fn_state.gs_325920 = s_16_0;
        // N s_16_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: panic
        panic!("{:?}", ());
        // N s_17_1: return
        return;
    }
}
