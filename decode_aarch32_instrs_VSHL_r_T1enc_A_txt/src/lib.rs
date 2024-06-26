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
use ConditionPassed::*;
use execute_aarch32_instrs_VSHL_r_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VSHL_r_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    U: bool,
    D: bool,
    size: u8,
    Vn: u8,
    Vd: u8,
    N: bool,
    Q: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        gs_319068: bool,
        esize: i64,
        n: i64,
        d: i64,
        is_unsigned: bool,
        elements: i64,
        ga_360833: i64,
        gs_319072: bool,
        gs_319071: bool,
        U: bool,
        D: bool,
        size: u8,
        Vn: u8,
        Vd: u8,
        N: bool,
        Q: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        U,
        D,
        size,
        Vn,
        Vd,
        N,
        Q,
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
        // S s_0_1: call ConditionPassed(s_0_0)
        let s_0_1: bool = ConditionPassed(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b2 b1
        if s_0_1 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var Q:u8
        let s_2_0: bool = fn_state.Q;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // C s_2_2: const #1u : u8
        let s_2_2: bool = true;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b10 b3
        if s_2_4 {
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
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#319072 <= s_3_0
        fn_state.gs_319072 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#319072:u8
        let s_4_0: bool = fn_state.gs_319072;
        // N s_4_1: branch s_4_0 b9 b5
        if s_4_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var U:u8
        let s_5_0: bool = fn_state.U;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 1u16);
        // C s_5_2: const #1u : u8
        let s_5_2: bool = true;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // D s_5_5: write-var is_unsigned <= s_5_4
        fn_state.is_unsigned = s_5_4;
        // D s_5_6: read-var size:u8
        let s_5_6: u8 = fn_state.size;
        // D s_5_7: cast zx s_5_6 -> bv
        let s_5_7: Bits = Bits::new(s_5_6 as u128, 2u16);
        // D s_5_8: cast zx s_5_7 -> i
        let s_5_8: i128 = (s_5_7.value() as i128);
        // D s_5_9: cast reint s_5_8 -> i64
        let s_5_9: i64 = (s_5_8 as i64);
        // C s_5_10: const #8s : i64
        let s_5_10: i64 = 8;
        // D s_5_11: lsl s_5_10 s_5_9
        let s_5_11: i64 = s_5_10 << s_5_9;
        // D s_5_12: write-var esize <= s_5_11
        fn_state.esize = s_5_11;
        // C s_5_13: const #64s : i
        let s_5_13: i128 = 64;
        // D s_5_14: read-var esize:i64
        let s_5_14: i64 = fn_state.esize;
        // D s_5_15: cast zx s_5_14 -> i
        let s_5_15: i128 = (i128::try_from(s_5_14).unwrap());
        // D s_5_16: div s_5_13 s_5_15
        let s_5_16: i128 = ((s_5_13) / (s_5_15));
        // D s_5_17: cast reint s_5_16 -> i64
        let s_5_17: i64 = (s_5_16 as i64);
        // D s_5_18: write-var elements <= s_5_17
        fn_state.elements = s_5_17;
        // D s_5_19: read-var D:u8
        let s_5_19: bool = fn_state.D;
        // D s_5_20: cast zx s_5_19 -> bv
        let s_5_20: Bits = Bits::new(s_5_19 as u128, 1u16);
        // D s_5_21: read-var Vd:u8
        let s_5_21: u8 = fn_state.Vd;
        // D s_5_22: cast zx s_5_21 -> bv
        let s_5_22: Bits = Bits::new(s_5_21 as u128, 4u16);
        // D s_5_23: cast reint s_5_20 -> u128
        let s_5_23: u128 = (s_5_20.value() as u128);
        // D s_5_24: size-of s_5_20
        let s_5_24: u16 = s_5_20.length();
        // D s_5_25: cast reint s_5_22 -> u128
        let s_5_25: u128 = (s_5_22.value() as u128);
        // D s_5_26: size-of s_5_22
        let s_5_26: u16 = s_5_22.length();
        // D s_5_27: lsl s_5_23 s_5_26
        let s_5_27: u128 = s_5_23 << s_5_26;
        // D s_5_28: or s_5_27 s_5_25
        let s_5_28: u128 = ((s_5_27) | (s_5_25));
        // D s_5_29: add s_5_24 s_5_26
        let s_5_29: u16 = (s_5_24 + s_5_26);
        // D s_5_30: create-bits s_5_28 s_5_29
        let s_5_30: Bits = Bits::new(s_5_28, s_5_29);
        // D s_5_31: cast reint s_5_30 -> u8
        let s_5_31: u8 = (s_5_30.value() as u8);
        // D s_5_32: cast zx s_5_31 -> bv
        let s_5_32: Bits = Bits::new(s_5_31 as u128, 5u16);
        // D s_5_33: cast zx s_5_32 -> i
        let s_5_33: i128 = (s_5_32.value() as i128);
        // D s_5_34: cast reint s_5_33 -> i64
        let s_5_34: i64 = (s_5_33 as i64);
        // D s_5_35: write-var d <= s_5_34
        fn_state.d = s_5_34;
        // D s_5_36: read-var M:u8
        let s_5_36: bool = fn_state.M;
        // D s_5_37: cast zx s_5_36 -> bv
        let s_5_37: Bits = Bits::new(s_5_36 as u128, 1u16);
        // D s_5_38: read-var Vm:u8
        let s_5_38: u8 = fn_state.Vm;
        // D s_5_39: cast zx s_5_38 -> bv
        let s_5_39: Bits = Bits::new(s_5_38 as u128, 4u16);
        // D s_5_40: cast reint s_5_37 -> u128
        let s_5_40: u128 = (s_5_37.value() as u128);
        // D s_5_41: size-of s_5_37
        let s_5_41: u16 = s_5_37.length();
        // D s_5_42: cast reint s_5_39 -> u128
        let s_5_42: u128 = (s_5_39.value() as u128);
        // D s_5_43: size-of s_5_39
        let s_5_43: u16 = s_5_39.length();
        // D s_5_44: lsl s_5_40 s_5_43
        let s_5_44: u128 = s_5_40 << s_5_43;
        // D s_5_45: or s_5_44 s_5_42
        let s_5_45: u128 = ((s_5_44) | (s_5_42));
        // D s_5_46: add s_5_41 s_5_43
        let s_5_46: u16 = (s_5_41 + s_5_43);
        // D s_5_47: create-bits s_5_45 s_5_46
        let s_5_47: Bits = Bits::new(s_5_45, s_5_46);
        // D s_5_48: cast reint s_5_47 -> u8
        let s_5_48: u8 = (s_5_47.value() as u8);
        // D s_5_49: cast zx s_5_48 -> bv
        let s_5_49: Bits = Bits::new(s_5_48 as u128, 5u16);
        // D s_5_50: cast zx s_5_49 -> i
        let s_5_50: i128 = (s_5_49.value() as i128);
        // D s_5_51: cast reint s_5_50 -> i64
        let s_5_51: i64 = (s_5_50 as i64);
        // D s_5_52: write-var m <= s_5_51
        fn_state.m = s_5_51;
        // D s_5_53: read-var N:u8
        let s_5_53: bool = fn_state.N;
        // D s_5_54: cast zx s_5_53 -> bv
        let s_5_54: Bits = Bits::new(s_5_53 as u128, 1u16);
        // D s_5_55: read-var Vn:u8
        let s_5_55: u8 = fn_state.Vn;
        // D s_5_56: cast zx s_5_55 -> bv
        let s_5_56: Bits = Bits::new(s_5_55 as u128, 4u16);
        // D s_5_57: cast reint s_5_54 -> u128
        let s_5_57: u128 = (s_5_54.value() as u128);
        // D s_5_58: size-of s_5_54
        let s_5_58: u16 = s_5_54.length();
        // D s_5_59: cast reint s_5_56 -> u128
        let s_5_59: u128 = (s_5_56.value() as u128);
        // D s_5_60: size-of s_5_56
        let s_5_60: u16 = s_5_56.length();
        // D s_5_61: lsl s_5_57 s_5_60
        let s_5_61: u128 = s_5_57 << s_5_60;
        // D s_5_62: or s_5_61 s_5_59
        let s_5_62: u128 = ((s_5_61) | (s_5_59));
        // D s_5_63: add s_5_58 s_5_60
        let s_5_63: u16 = (s_5_58 + s_5_60);
        // D s_5_64: create-bits s_5_62 s_5_63
        let s_5_64: Bits = Bits::new(s_5_62, s_5_63);
        // D s_5_65: cast reint s_5_64 -> u8
        let s_5_65: u8 = (s_5_64.value() as u8);
        // D s_5_66: cast zx s_5_65 -> bv
        let s_5_66: Bits = Bits::new(s_5_65 as u128, 5u16);
        // D s_5_67: cast zx s_5_66 -> i
        let s_5_67: i128 = (s_5_66.value() as i128);
        // D s_5_68: cast reint s_5_67 -> i64
        let s_5_68: i64 = (s_5_67 as i64);
        // D s_5_69: write-var n <= s_5_68
        fn_state.n = s_5_68;
        // D s_5_70: read-var Q:u8
        let s_5_70: bool = fn_state.Q;
        // D s_5_71: cast zx s_5_70 -> bv
        let s_5_71: Bits = Bits::new(s_5_70 as u128, 1u16);
        // C s_5_72: const #0u : u8
        let s_5_72: bool = false;
        // C s_5_73: cast zx s_5_72 -> bv
        let s_5_73: Bits = Bits::new(s_5_72 as u128, 1u16);
        // D s_5_74: cmp-eq s_5_71 s_5_73
        let s_5_74: bool = ((s_5_71) == (s_5_73));
        // N s_5_75: branch s_5_74 b8 b6
        if s_5_74 {
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
        // C s_6_0: const #2s : i64
        let s_6_0: i64 = 2;
        // D s_6_1: write-var ga#360833 <= s_6_0
        fn_state.ga_360833 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var ga#360833:i64
        let s_7_0: i64 = fn_state.ga_360833;
        // D s_7_1: read-var esize:i64
        let s_7_1: i64 = fn_state.esize;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: cast reint s_7_2 -> i64
        let s_7_3: i64 = (s_7_2 as i64);
        // D s_7_4: read-var elements:i64
        let s_7_4: i64 = fn_state.elements;
        // D s_7_5: cast zx s_7_4 -> i
        let s_7_5: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_6: read-var d:i64
        let s_7_6: i64 = fn_state.d;
        // D s_7_7: read-var m:i64
        let s_7_7: i64 = fn_state.m;
        // D s_7_8: read-var n:i64
        let s_7_8: i64 = fn_state.n;
        // D s_7_9: read-var is_unsigned:u8
        let s_7_9: bool = fn_state.is_unsigned;
        // D s_7_10: call execute_aarch32_instrs_VSHL_r_Op_A_txt(s_7_6, s_7_5, s_7_3, s_7_7, s_7_8, s_7_0, s_7_9)
        let s_7_10: () = execute_aarch32_instrs_VSHL_r_Op_A_txt(
            state,
            tracer,
            s_7_6,
            s_7_5,
            s_7_3,
            s_7_7,
            s_7_8,
            s_7_0,
            s_7_9,
        );
        // N s_7_11: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #1s : i64
        let s_8_0: i64 = 1;
        // D s_8_1: write-var ga#360833 <= s_8_0
        fn_state.ga_360833 = s_8_0;
        // N s_8_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: panic
        panic!("{:?}", ());
        // N s_9_1: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0s : i
        let s_10_0: i128 = 0;
        // D s_10_1: read-var Vd:u8
        let s_10_1: u8 = fn_state.Vd;
        // D s_10_2: cast zx s_10_1 -> bv
        let s_10_2: Bits = Bits::new(s_10_1 as u128, 4u16);
        // C s_10_3: const #1u : u64
        let s_10_3: u64 = 1;
        // D s_10_4: bit-extract s_10_2 s_10_0 s_10_3
        let s_10_4: Bits = (Bits::new(
            ((s_10_2) >> (s_10_0)).value(),
            u16::try_from(s_10_3).unwrap(),
        ));
        // D s_10_5: cast reint s_10_4 -> u8
        let s_10_5: bool = ((s_10_4.value()) != 0);
        // C s_10_6: const #0s : i
        let s_10_6: i128 = 0;
        // C s_10_7: const #0u : u64
        let s_10_7: u64 = 0;
        // D s_10_8: cast zx s_10_5 -> u64
        let s_10_8: u64 = (s_10_5 as u64);
        // C s_10_9: const #1u : u64
        let s_10_9: u64 = 1;
        // D s_10_10: and s_10_8 s_10_9
        let s_10_10: u64 = ((s_10_8) & (s_10_9));
        // D s_10_11: cmp-eq s_10_10 s_10_9
        let s_10_11: bool = ((s_10_10) == (s_10_9));
        // D s_10_12: lsl s_10_8 s_10_6
        let s_10_12: u64 = s_10_8 << s_10_6;
        // D s_10_13: or s_10_7 s_10_12
        let s_10_13: u64 = ((s_10_7) | (s_10_12));
        // D s_10_14: cmpl s_10_12
        let s_10_14: u64 = !s_10_12;
        // D s_10_15: and s_10_7 s_10_14
        let s_10_15: u64 = ((s_10_7) & (s_10_14));
        // D s_10_16: select s_10_11 s_10_13 s_10_15
        let s_10_16: u64 = if s_10_11 { s_10_13 } else { s_10_15 };
        // D s_10_17: cast trunc s_10_16 -> u8
        let s_10_17: bool = ((s_10_16) != 0);
        // D s_10_18: cast zx s_10_17 -> bv
        let s_10_18: Bits = Bits::new(s_10_17 as u128, 1u16);
        // C s_10_19: const #1u : u8
        let s_10_19: bool = true;
        // C s_10_20: cast zx s_10_19 -> bv
        let s_10_20: Bits = Bits::new(s_10_19 as u128, 1u16);
        // D s_10_21: cmp-eq s_10_18 s_10_20
        let s_10_21: bool = ((s_10_18) == (s_10_20));
        // N s_10_22: branch s_10_21 b16 b11
        if s_10_21 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0s : i
        let s_11_0: i128 = 0;
        // D s_11_1: read-var Vm:u8
        let s_11_1: u8 = fn_state.Vm;
        // D s_11_2: cast zx s_11_1 -> bv
        let s_11_2: Bits = Bits::new(s_11_1 as u128, 4u16);
        // C s_11_3: const #1u : u64
        let s_11_3: u64 = 1;
        // D s_11_4: bit-extract s_11_2 s_11_0 s_11_3
        let s_11_4: Bits = (Bits::new(
            ((s_11_2) >> (s_11_0)).value(),
            u16::try_from(s_11_3).unwrap(),
        ));
        // D s_11_5: cast reint s_11_4 -> u8
        let s_11_5: bool = ((s_11_4.value()) != 0);
        // C s_11_6: const #0s : i
        let s_11_6: i128 = 0;
        // C s_11_7: const #0u : u64
        let s_11_7: u64 = 0;
        // D s_11_8: cast zx s_11_5 -> u64
        let s_11_8: u64 = (s_11_5 as u64);
        // C s_11_9: const #1u : u64
        let s_11_9: u64 = 1;
        // D s_11_10: and s_11_8 s_11_9
        let s_11_10: u64 = ((s_11_8) & (s_11_9));
        // D s_11_11: cmp-eq s_11_10 s_11_9
        let s_11_11: bool = ((s_11_10) == (s_11_9));
        // D s_11_12: lsl s_11_8 s_11_6
        let s_11_12: u64 = s_11_8 << s_11_6;
        // D s_11_13: or s_11_7 s_11_12
        let s_11_13: u64 = ((s_11_7) | (s_11_12));
        // D s_11_14: cmpl s_11_12
        let s_11_14: u64 = !s_11_12;
        // D s_11_15: and s_11_7 s_11_14
        let s_11_15: u64 = ((s_11_7) & (s_11_14));
        // D s_11_16: select s_11_11 s_11_13 s_11_15
        let s_11_16: u64 = if s_11_11 { s_11_13 } else { s_11_15 };
        // D s_11_17: cast trunc s_11_16 -> u8
        let s_11_17: bool = ((s_11_16) != 0);
        // D s_11_18: cast zx s_11_17 -> bv
        let s_11_18: Bits = Bits::new(s_11_17 as u128, 1u16);
        // C s_11_19: const #1u : u8
        let s_11_19: bool = true;
        // C s_11_20: cast zx s_11_19 -> bv
        let s_11_20: Bits = Bits::new(s_11_19 as u128, 1u16);
        // D s_11_21: cmp-eq s_11_18 s_11_20
        let s_11_21: bool = ((s_11_18) == (s_11_20));
        // D s_11_22: write-var gs#319068 <= s_11_21
        fn_state.gs_319068 = s_11_21;
        // N s_11_23: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#319068:u8
        let s_12_0: bool = fn_state.gs_319068;
        // N s_12_1: branch s_12_0 b15 b13
        if s_12_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0s : i
        let s_13_0: i128 = 0;
        // D s_13_1: read-var Vn:u8
        let s_13_1: u8 = fn_state.Vn;
        // D s_13_2: cast zx s_13_1 -> bv
        let s_13_2: Bits = Bits::new(s_13_1 as u128, 4u16);
        // C s_13_3: const #1u : u64
        let s_13_3: u64 = 1;
        // D s_13_4: bit-extract s_13_2 s_13_0 s_13_3
        let s_13_4: Bits = (Bits::new(
            ((s_13_2) >> (s_13_0)).value(),
            u16::try_from(s_13_3).unwrap(),
        ));
        // D s_13_5: cast reint s_13_4 -> u8
        let s_13_5: bool = ((s_13_4.value()) != 0);
        // C s_13_6: const #0s : i
        let s_13_6: i128 = 0;
        // C s_13_7: const #0u : u64
        let s_13_7: u64 = 0;
        // D s_13_8: cast zx s_13_5 -> u64
        let s_13_8: u64 = (s_13_5 as u64);
        // C s_13_9: const #1u : u64
        let s_13_9: u64 = 1;
        // D s_13_10: and s_13_8 s_13_9
        let s_13_10: u64 = ((s_13_8) & (s_13_9));
        // D s_13_11: cmp-eq s_13_10 s_13_9
        let s_13_11: bool = ((s_13_10) == (s_13_9));
        // D s_13_12: lsl s_13_8 s_13_6
        let s_13_12: u64 = s_13_8 << s_13_6;
        // D s_13_13: or s_13_7 s_13_12
        let s_13_13: u64 = ((s_13_7) | (s_13_12));
        // D s_13_14: cmpl s_13_12
        let s_13_14: u64 = !s_13_12;
        // D s_13_15: and s_13_7 s_13_14
        let s_13_15: u64 = ((s_13_7) & (s_13_14));
        // D s_13_16: select s_13_11 s_13_13 s_13_15
        let s_13_16: u64 = if s_13_11 { s_13_13 } else { s_13_15 };
        // D s_13_17: cast trunc s_13_16 -> u8
        let s_13_17: bool = ((s_13_16) != 0);
        // D s_13_18: cast zx s_13_17 -> bv
        let s_13_18: Bits = Bits::new(s_13_17 as u128, 1u16);
        // C s_13_19: const #1u : u8
        let s_13_19: bool = true;
        // C s_13_20: cast zx s_13_19 -> bv
        let s_13_20: Bits = Bits::new(s_13_19 as u128, 1u16);
        // D s_13_21: cmp-eq s_13_18 s_13_20
        let s_13_21: bool = ((s_13_18) == (s_13_20));
        // D s_13_22: write-var gs#319071 <= s_13_21
        fn_state.gs_319071 = s_13_21;
        // N s_13_23: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#319071:u8
        let s_14_0: bool = fn_state.gs_319071;
        // D s_14_1: write-var gs#319072 <= s_14_0
        fn_state.gs_319072 = s_14_0;
        // N s_14_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var gs#319071 <= s_15_0
        fn_state.gs_319071 = s_15_0;
        // N s_15_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#319068 <= s_16_0
        fn_state.gs_319068 = s_16_0;
        // N s_16_2: jump b12
        return block_12(state, tracer, fn_state);
    }
}
