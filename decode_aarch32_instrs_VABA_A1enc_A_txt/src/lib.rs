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
use execute_aarch32_instrs_VABA_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VABA_A1enc_A_txt<T: Tracer>(
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
        ga_350263: i64,
        gs_305549: bool,
        esize: i64,
        n: i64,
        d: i64,
        is_unsigned: bool,
        elements: i64,
        gs_305550: bool,
        gs_305546: bool,
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
        // D s_2_0: read-var size:u8
        let s_2_0: u8 = fn_state.size;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // C s_2_2: const #3u : u8
        let s_2_2: u8 = 3;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 2u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b18 b3
        if s_2_4 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var Q:u8
        let s_3_0: bool = fn_state.Q;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 1u16);
        // C s_3_2: const #1u : u8
        let s_3_2: bool = true;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 1u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // N s_3_5: branch s_3_4 b11 b4
        if s_3_4 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#305550 <= s_4_0
        fn_state.gs_305550 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#305550:u8
        let s_5_0: bool = fn_state.gs_305550;
        // N s_5_1: branch s_5_0 b10 b6
        if s_5_0 {
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
        // D s_6_0: read-var U:u8
        let s_6_0: bool = fn_state.U;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 1u16);
        // C s_6_2: const #1u : u8
        let s_6_2: bool = true;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // D s_6_5: write-var is_unsigned <= s_6_4
        fn_state.is_unsigned = s_6_4;
        // D s_6_6: read-var size:u8
        let s_6_6: u8 = fn_state.size;
        // D s_6_7: cast zx s_6_6 -> bv
        let s_6_7: Bits = Bits::new(s_6_6 as u128, 2u16);
        // D s_6_8: cast zx s_6_7 -> i
        let s_6_8: i128 = (s_6_7.value() as i128);
        // D s_6_9: cast reint s_6_8 -> i64
        let s_6_9: i64 = (s_6_8 as i64);
        // C s_6_10: const #8s : i64
        let s_6_10: i64 = 8;
        // D s_6_11: lsl s_6_10 s_6_9
        let s_6_11: i64 = s_6_10 << s_6_9;
        // D s_6_12: write-var esize <= s_6_11
        fn_state.esize = s_6_11;
        // C s_6_13: const #64s : i
        let s_6_13: i128 = 64;
        // D s_6_14: read-var esize:i64
        let s_6_14: i64 = fn_state.esize;
        // D s_6_15: cast zx s_6_14 -> i
        let s_6_15: i128 = (i128::try_from(s_6_14).unwrap());
        // D s_6_16: div s_6_13 s_6_15
        let s_6_16: i128 = ((s_6_13) / (s_6_15));
        // D s_6_17: cast reint s_6_16 -> i64
        let s_6_17: i64 = (s_6_16 as i64);
        // D s_6_18: write-var elements <= s_6_17
        fn_state.elements = s_6_17;
        // D s_6_19: read-var D:u8
        let s_6_19: bool = fn_state.D;
        // D s_6_20: cast zx s_6_19 -> bv
        let s_6_20: Bits = Bits::new(s_6_19 as u128, 1u16);
        // D s_6_21: read-var Vd:u8
        let s_6_21: u8 = fn_state.Vd;
        // D s_6_22: cast zx s_6_21 -> bv
        let s_6_22: Bits = Bits::new(s_6_21 as u128, 4u16);
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
        // D s_6_35: write-var d <= s_6_34
        fn_state.d = s_6_34;
        // D s_6_36: read-var N:u8
        let s_6_36: bool = fn_state.N;
        // D s_6_37: cast zx s_6_36 -> bv
        let s_6_37: Bits = Bits::new(s_6_36 as u128, 1u16);
        // D s_6_38: read-var Vn:u8
        let s_6_38: u8 = fn_state.Vn;
        // D s_6_39: cast zx s_6_38 -> bv
        let s_6_39: Bits = Bits::new(s_6_38 as u128, 4u16);
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
        // D s_6_52: write-var n <= s_6_51
        fn_state.n = s_6_51;
        // D s_6_53: read-var M:u8
        let s_6_53: bool = fn_state.M;
        // D s_6_54: cast zx s_6_53 -> bv
        let s_6_54: Bits = Bits::new(s_6_53 as u128, 1u16);
        // D s_6_55: read-var Vm:u8
        let s_6_55: u8 = fn_state.Vm;
        // D s_6_56: cast zx s_6_55 -> bv
        let s_6_56: Bits = Bits::new(s_6_55 as u128, 4u16);
        // D s_6_57: cast reint s_6_54 -> u128
        let s_6_57: u128 = (s_6_54.value() as u128);
        // D s_6_58: size-of s_6_54
        let s_6_58: u16 = s_6_54.length();
        // D s_6_59: cast reint s_6_56 -> u128
        let s_6_59: u128 = (s_6_56.value() as u128);
        // D s_6_60: size-of s_6_56
        let s_6_60: u16 = s_6_56.length();
        // D s_6_61: lsl s_6_57 s_6_60
        let s_6_61: u128 = s_6_57 << s_6_60;
        // D s_6_62: or s_6_61 s_6_59
        let s_6_62: u128 = ((s_6_61) | (s_6_59));
        // D s_6_63: add s_6_58 s_6_60
        let s_6_63: u16 = (s_6_58 + s_6_60);
        // D s_6_64: create-bits s_6_62 s_6_63
        let s_6_64: Bits = Bits::new(s_6_62, s_6_63);
        // D s_6_65: cast reint s_6_64 -> u8
        let s_6_65: u8 = (s_6_64.value() as u8);
        // D s_6_66: cast zx s_6_65 -> bv
        let s_6_66: Bits = Bits::new(s_6_65 as u128, 5u16);
        // D s_6_67: cast zx s_6_66 -> i
        let s_6_67: i128 = (s_6_66.value() as i128);
        // D s_6_68: cast reint s_6_67 -> i64
        let s_6_68: i64 = (s_6_67 as i64);
        // D s_6_69: write-var m <= s_6_68
        fn_state.m = s_6_68;
        // D s_6_70: read-var Q:u8
        let s_6_70: bool = fn_state.Q;
        // D s_6_71: cast zx s_6_70 -> bv
        let s_6_71: Bits = Bits::new(s_6_70 as u128, 1u16);
        // C s_6_72: const #0u : u8
        let s_6_72: bool = false;
        // C s_6_73: cast zx s_6_72 -> bv
        let s_6_73: Bits = Bits::new(s_6_72 as u128, 1u16);
        // D s_6_74: cmp-eq s_6_71 s_6_73
        let s_6_74: bool = ((s_6_71) == (s_6_73));
        // N s_6_75: branch s_6_74 b9 b7
        if s_6_74 {
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
        // C s_7_0: const #2s : i64
        let s_7_0: i64 = 2;
        // D s_7_1: write-var ga#350263 <= s_7_0
        fn_state.ga_350263 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var ga#350263:i64
        let s_8_0: i64 = fn_state.ga_350263;
        // D s_8_1: read-var esize:i64
        let s_8_1: i64 = fn_state.esize;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // D s_8_3: cast reint s_8_2 -> i64
        let s_8_3: i64 = (s_8_2 as i64);
        // D s_8_4: read-var elements:i64
        let s_8_4: i64 = fn_state.elements;
        // D s_8_5: cast zx s_8_4 -> i
        let s_8_5: i128 = (i128::try_from(s_8_4).unwrap());
        // D s_8_6: read-var d:i64
        let s_8_6: i64 = fn_state.d;
        // C s_8_7: const #0u : u8
        let s_8_7: bool = false;
        // D s_8_8: read-var m:i64
        let s_8_8: i64 = fn_state.m;
        // D s_8_9: read-var n:i64
        let s_8_9: i64 = fn_state.n;
        // D s_8_10: read-var is_unsigned:u8
        let s_8_10: bool = fn_state.is_unsigned;
        // D s_8_11: call execute_aarch32_instrs_VABA_Op_A_txt(s_8_6, s_8_5, s_8_3, s_8_7, s_8_8, s_8_9, s_8_0, s_8_10)
        let s_8_11: () = execute_aarch32_instrs_VABA_Op_A_txt(
            state,
            tracer,
            s_8_6,
            s_8_5,
            s_8_3,
            s_8_7,
            s_8_8,
            s_8_9,
            s_8_0,
            s_8_10,
        );
        // N s_8_12: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #1s : i64
        let s_9_0: i64 = 1;
        // D s_9_1: write-var ga#350263 <= s_9_0
        fn_state.ga_350263 = s_9_0;
        // N s_9_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: panic
        panic!("{:?}", ());
        // N s_10_1: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0s : i
        let s_11_0: i128 = 0;
        // D s_11_1: read-var Vd:u8
        let s_11_1: u8 = fn_state.Vd;
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
        // N s_11_22: branch s_11_21 b17 b12
        if s_11_21 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0s : i
        let s_12_0: i128 = 0;
        // D s_12_1: read-var Vn:u8
        let s_12_1: u8 = fn_state.Vn;
        // D s_12_2: cast zx s_12_1 -> bv
        let s_12_2: Bits = Bits::new(s_12_1 as u128, 4u16);
        // C s_12_3: const #1u : u64
        let s_12_3: u64 = 1;
        // D s_12_4: bit-extract s_12_2 s_12_0 s_12_3
        let s_12_4: Bits = (Bits::new(
            ((s_12_2) >> (s_12_0)).value(),
            u16::try_from(s_12_3).unwrap(),
        ));
        // D s_12_5: cast reint s_12_4 -> u8
        let s_12_5: bool = ((s_12_4.value()) != 0);
        // C s_12_6: const #0s : i
        let s_12_6: i128 = 0;
        // C s_12_7: const #0u : u64
        let s_12_7: u64 = 0;
        // D s_12_8: cast zx s_12_5 -> u64
        let s_12_8: u64 = (s_12_5 as u64);
        // C s_12_9: const #1u : u64
        let s_12_9: u64 = 1;
        // D s_12_10: and s_12_8 s_12_9
        let s_12_10: u64 = ((s_12_8) & (s_12_9));
        // D s_12_11: cmp-eq s_12_10 s_12_9
        let s_12_11: bool = ((s_12_10) == (s_12_9));
        // D s_12_12: lsl s_12_8 s_12_6
        let s_12_12: u64 = s_12_8 << s_12_6;
        // D s_12_13: or s_12_7 s_12_12
        let s_12_13: u64 = ((s_12_7) | (s_12_12));
        // D s_12_14: cmpl s_12_12
        let s_12_14: u64 = !s_12_12;
        // D s_12_15: and s_12_7 s_12_14
        let s_12_15: u64 = ((s_12_7) & (s_12_14));
        // D s_12_16: select s_12_11 s_12_13 s_12_15
        let s_12_16: u64 = if s_12_11 { s_12_13 } else { s_12_15 };
        // D s_12_17: cast trunc s_12_16 -> u8
        let s_12_17: bool = ((s_12_16) != 0);
        // D s_12_18: cast zx s_12_17 -> bv
        let s_12_18: Bits = Bits::new(s_12_17 as u128, 1u16);
        // C s_12_19: const #1u : u8
        let s_12_19: bool = true;
        // C s_12_20: cast zx s_12_19 -> bv
        let s_12_20: Bits = Bits::new(s_12_19 as u128, 1u16);
        // D s_12_21: cmp-eq s_12_18 s_12_20
        let s_12_21: bool = ((s_12_18) == (s_12_20));
        // D s_12_22: write-var gs#305546 <= s_12_21
        fn_state.gs_305546 = s_12_21;
        // N s_12_23: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#305546:u8
        let s_13_0: bool = fn_state.gs_305546;
        // N s_13_1: branch s_13_0 b16 b14
        if s_13_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0s : i
        let s_14_0: i128 = 0;
        // D s_14_1: read-var Vm:u8
        let s_14_1: u8 = fn_state.Vm;
        // D s_14_2: cast zx s_14_1 -> bv
        let s_14_2: Bits = Bits::new(s_14_1 as u128, 4u16);
        // C s_14_3: const #1u : u64
        let s_14_3: u64 = 1;
        // D s_14_4: bit-extract s_14_2 s_14_0 s_14_3
        let s_14_4: Bits = (Bits::new(
            ((s_14_2) >> (s_14_0)).value(),
            u16::try_from(s_14_3).unwrap(),
        ));
        // D s_14_5: cast reint s_14_4 -> u8
        let s_14_5: bool = ((s_14_4.value()) != 0);
        // C s_14_6: const #0s : i
        let s_14_6: i128 = 0;
        // C s_14_7: const #0u : u64
        let s_14_7: u64 = 0;
        // D s_14_8: cast zx s_14_5 -> u64
        let s_14_8: u64 = (s_14_5 as u64);
        // C s_14_9: const #1u : u64
        let s_14_9: u64 = 1;
        // D s_14_10: and s_14_8 s_14_9
        let s_14_10: u64 = ((s_14_8) & (s_14_9));
        // D s_14_11: cmp-eq s_14_10 s_14_9
        let s_14_11: bool = ((s_14_10) == (s_14_9));
        // D s_14_12: lsl s_14_8 s_14_6
        let s_14_12: u64 = s_14_8 << s_14_6;
        // D s_14_13: or s_14_7 s_14_12
        let s_14_13: u64 = ((s_14_7) | (s_14_12));
        // D s_14_14: cmpl s_14_12
        let s_14_14: u64 = !s_14_12;
        // D s_14_15: and s_14_7 s_14_14
        let s_14_15: u64 = ((s_14_7) & (s_14_14));
        // D s_14_16: select s_14_11 s_14_13 s_14_15
        let s_14_16: u64 = if s_14_11 { s_14_13 } else { s_14_15 };
        // D s_14_17: cast trunc s_14_16 -> u8
        let s_14_17: bool = ((s_14_16) != 0);
        // D s_14_18: cast zx s_14_17 -> bv
        let s_14_18: Bits = Bits::new(s_14_17 as u128, 1u16);
        // C s_14_19: const #1u : u8
        let s_14_19: bool = true;
        // C s_14_20: cast zx s_14_19 -> bv
        let s_14_20: Bits = Bits::new(s_14_19 as u128, 1u16);
        // D s_14_21: cmp-eq s_14_18 s_14_20
        let s_14_21: bool = ((s_14_18) == (s_14_20));
        // D s_14_22: write-var gs#305549 <= s_14_21
        fn_state.gs_305549 = s_14_21;
        // N s_14_23: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#305549:u8
        let s_15_0: bool = fn_state.gs_305549;
        // D s_15_1: write-var gs#305550 <= s_15_0
        fn_state.gs_305550 = s_15_0;
        // N s_15_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#305549 <= s_16_0
        fn_state.gs_305549 = s_16_0;
        // N s_16_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var gs#305546 <= s_17_0
        fn_state.gs_305546 = s_17_0;
        // N s_17_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: panic
        panic!("{:?}", ());
        // N s_18_1: return
        return;
    }
}
