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
use execute_aarch32_instrs_VCGT_r_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VCGT_r_T1enc_A_txt<T: Tracer>(
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
        gs_307344: bool,
        esize: i64,
        ga_351819: i64,
        n: i64,
        d: i64,
        gs_307343: bool,
        elements: i64,
        vtype: u32,
        gs_307340: bool,
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
        // D s_3_1: write-var gs#307344 <= s_3_0
        fn_state.gs_307344 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#307344:u8
        let s_4_0: bool = fn_state.gs_307344;
        // N s_4_1: branch s_4_0 b14 b5
        if s_4_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var size:u8
        let s_5_0: u8 = fn_state.size;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // C s_5_2: const #3u : u8
        let s_5_2: u8 = 3;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 2u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // N s_5_5: branch s_5_4 b13 b6
        if s_5_4 {
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
        // N s_6_5: branch s_6_4 b12 b7
        if s_6_4 {
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
        // C s_7_0: const #0u : u32
        let s_7_0: u32 = 0;
        // D s_7_1: write-var vtype <= s_7_0
        fn_state.vtype = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
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
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (s_8_1.value() as i128);
        // D s_8_3: cast reint s_8_2 -> i64
        let s_8_3: i64 = (s_8_2 as i64);
        // C s_8_4: const #8s : i64
        let s_8_4: i64 = 8;
        // D s_8_5: lsl s_8_4 s_8_3
        let s_8_5: i64 = s_8_4 << s_8_3;
        // D s_8_6: write-var esize <= s_8_5
        fn_state.esize = s_8_5;
        // C s_8_7: const #64s : i
        let s_8_7: i128 = 64;
        // D s_8_8: read-var esize:i64
        let s_8_8: i64 = fn_state.esize;
        // D s_8_9: cast zx s_8_8 -> i
        let s_8_9: i128 = (i128::try_from(s_8_8).unwrap());
        // D s_8_10: div s_8_7 s_8_9
        let s_8_10: i128 = ((s_8_7) / (s_8_9));
        // D s_8_11: cast reint s_8_10 -> i64
        let s_8_11: i64 = (s_8_10 as i64);
        // D s_8_12: write-var elements <= s_8_11
        fn_state.elements = s_8_11;
        // D s_8_13: read-var D:u8
        let s_8_13: bool = fn_state.D;
        // D s_8_14: cast zx s_8_13 -> bv
        let s_8_14: Bits = Bits::new(s_8_13 as u128, 1u16);
        // D s_8_15: read-var Vd:u8
        let s_8_15: u8 = fn_state.Vd;
        // D s_8_16: cast zx s_8_15 -> bv
        let s_8_16: Bits = Bits::new(s_8_15 as u128, 4u16);
        // D s_8_17: cast reint s_8_14 -> u128
        let s_8_17: u128 = (s_8_14.value() as u128);
        // D s_8_18: size-of s_8_14
        let s_8_18: u16 = s_8_14.length();
        // D s_8_19: cast reint s_8_16 -> u128
        let s_8_19: u128 = (s_8_16.value() as u128);
        // D s_8_20: size-of s_8_16
        let s_8_20: u16 = s_8_16.length();
        // D s_8_21: lsl s_8_17 s_8_20
        let s_8_21: u128 = s_8_17 << s_8_20;
        // D s_8_22: or s_8_21 s_8_19
        let s_8_22: u128 = ((s_8_21) | (s_8_19));
        // D s_8_23: add s_8_18 s_8_20
        let s_8_23: u16 = (s_8_18 + s_8_20);
        // D s_8_24: create-bits s_8_22 s_8_23
        let s_8_24: Bits = Bits::new(s_8_22, s_8_23);
        // D s_8_25: cast reint s_8_24 -> u8
        let s_8_25: u8 = (s_8_24.value() as u8);
        // D s_8_26: cast zx s_8_25 -> bv
        let s_8_26: Bits = Bits::new(s_8_25 as u128, 5u16);
        // D s_8_27: cast zx s_8_26 -> i
        let s_8_27: i128 = (s_8_26.value() as i128);
        // D s_8_28: cast reint s_8_27 -> i64
        let s_8_28: i64 = (s_8_27 as i64);
        // D s_8_29: write-var d <= s_8_28
        fn_state.d = s_8_28;
        // D s_8_30: read-var N:u8
        let s_8_30: bool = fn_state.N;
        // D s_8_31: cast zx s_8_30 -> bv
        let s_8_31: Bits = Bits::new(s_8_30 as u128, 1u16);
        // D s_8_32: read-var Vn:u8
        let s_8_32: u8 = fn_state.Vn;
        // D s_8_33: cast zx s_8_32 -> bv
        let s_8_33: Bits = Bits::new(s_8_32 as u128, 4u16);
        // D s_8_34: cast reint s_8_31 -> u128
        let s_8_34: u128 = (s_8_31.value() as u128);
        // D s_8_35: size-of s_8_31
        let s_8_35: u16 = s_8_31.length();
        // D s_8_36: cast reint s_8_33 -> u128
        let s_8_36: u128 = (s_8_33.value() as u128);
        // D s_8_37: size-of s_8_33
        let s_8_37: u16 = s_8_33.length();
        // D s_8_38: lsl s_8_34 s_8_37
        let s_8_38: u128 = s_8_34 << s_8_37;
        // D s_8_39: or s_8_38 s_8_36
        let s_8_39: u128 = ((s_8_38) | (s_8_36));
        // D s_8_40: add s_8_35 s_8_37
        let s_8_40: u16 = (s_8_35 + s_8_37);
        // D s_8_41: create-bits s_8_39 s_8_40
        let s_8_41: Bits = Bits::new(s_8_39, s_8_40);
        // D s_8_42: cast reint s_8_41 -> u8
        let s_8_42: u8 = (s_8_41.value() as u8);
        // D s_8_43: cast zx s_8_42 -> bv
        let s_8_43: Bits = Bits::new(s_8_42 as u128, 5u16);
        // D s_8_44: cast zx s_8_43 -> i
        let s_8_44: i128 = (s_8_43.value() as i128);
        // D s_8_45: cast reint s_8_44 -> i64
        let s_8_45: i64 = (s_8_44 as i64);
        // D s_8_46: write-var n <= s_8_45
        fn_state.n = s_8_45;
        // D s_8_47: read-var M:u8
        let s_8_47: bool = fn_state.M;
        // D s_8_48: cast zx s_8_47 -> bv
        let s_8_48: Bits = Bits::new(s_8_47 as u128, 1u16);
        // D s_8_49: read-var Vm:u8
        let s_8_49: u8 = fn_state.Vm;
        // D s_8_50: cast zx s_8_49 -> bv
        let s_8_50: Bits = Bits::new(s_8_49 as u128, 4u16);
        // D s_8_51: cast reint s_8_48 -> u128
        let s_8_51: u128 = (s_8_48.value() as u128);
        // D s_8_52: size-of s_8_48
        let s_8_52: u16 = s_8_48.length();
        // D s_8_53: cast reint s_8_50 -> u128
        let s_8_53: u128 = (s_8_50.value() as u128);
        // D s_8_54: size-of s_8_50
        let s_8_54: u16 = s_8_50.length();
        // D s_8_55: lsl s_8_51 s_8_54
        let s_8_55: u128 = s_8_51 << s_8_54;
        // D s_8_56: or s_8_55 s_8_53
        let s_8_56: u128 = ((s_8_55) | (s_8_53));
        // D s_8_57: add s_8_52 s_8_54
        let s_8_57: u16 = (s_8_52 + s_8_54);
        // D s_8_58: create-bits s_8_56 s_8_57
        let s_8_58: Bits = Bits::new(s_8_56, s_8_57);
        // D s_8_59: cast reint s_8_58 -> u8
        let s_8_59: u8 = (s_8_58.value() as u8);
        // D s_8_60: cast zx s_8_59 -> bv
        let s_8_60: Bits = Bits::new(s_8_59 as u128, 5u16);
        // D s_8_61: cast zx s_8_60 -> i
        let s_8_61: i128 = (s_8_60.value() as i128);
        // D s_8_62: cast reint s_8_61 -> i64
        let s_8_62: i64 = (s_8_61 as i64);
        // D s_8_63: write-var m <= s_8_62
        fn_state.m = s_8_62;
        // D s_8_64: read-var Q:u8
        let s_8_64: bool = fn_state.Q;
        // D s_8_65: cast zx s_8_64 -> bv
        let s_8_65: Bits = Bits::new(s_8_64 as u128, 1u16);
        // C s_8_66: const #0u : u8
        let s_8_66: bool = false;
        // C s_8_67: cast zx s_8_66 -> bv
        let s_8_67: Bits = Bits::new(s_8_66 as u128, 1u16);
        // D s_8_68: cmp-eq s_8_65 s_8_67
        let s_8_68: bool = ((s_8_65) == (s_8_67));
        // N s_8_69: branch s_8_68 b11 b9
        if s_8_68 {
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
        // C s_9_0: const #2s : i64
        let s_9_0: i64 = 2;
        // D s_9_1: write-var ga#351819 <= s_9_0
        fn_state.ga_351819 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var ga#351819:i64
        let s_10_0: i64 = fn_state.ga_351819;
        // D s_10_1: read-var esize:i64
        let s_10_1: i64 = fn_state.esize;
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // D s_10_3: cast reint s_10_2 -> i64
        let s_10_3: i64 = (s_10_2 as i64);
        // D s_10_4: read-var elements:i64
        let s_10_4: i64 = fn_state.elements;
        // D s_10_5: cast zx s_10_4 -> i
        let s_10_5: i128 = (i128::try_from(s_10_4).unwrap());
        // D s_10_6: read-var d:i64
        let s_10_6: i64 = fn_state.d;
        // D s_10_7: read-var m:i64
        let s_10_7: i64 = fn_state.m;
        // D s_10_8: read-var n:i64
        let s_10_8: i64 = fn_state.n;
        // D s_10_9: read-var vtype:u32
        let s_10_9: u32 = fn_state.vtype;
        // D s_10_10: call execute_aarch32_instrs_VCGT_r_Op_A_txt(s_10_6, s_10_5, s_10_3, s_10_7, s_10_8, s_10_0, s_10_9)
        let s_10_10: () = execute_aarch32_instrs_VCGT_r_Op_A_txt(
            state,
            tracer,
            s_10_6,
            s_10_5,
            s_10_3,
            s_10_7,
            s_10_8,
            s_10_0,
            s_10_9,
        );
        // N s_10_11: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #1s : i64
        let s_11_0: i64 = 1;
        // D s_11_1: write-var ga#351819 <= s_11_0
        fn_state.ga_351819 = s_11_0;
        // N s_11_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1u : u32
        let s_12_0: u32 = 1;
        // D s_12_1: write-var vtype <= s_12_0
        fn_state.vtype = s_12_0;
        // N s_12_2: jump b8
        return block_8(state, tracer, fn_state);
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
        // C s_15_0: const #0s : i
        let s_15_0: i128 = 0;
        // D s_15_1: read-var Vd:u8
        let s_15_1: u8 = fn_state.Vd;
        // D s_15_2: cast zx s_15_1 -> bv
        let s_15_2: Bits = Bits::new(s_15_1 as u128, 4u16);
        // C s_15_3: const #1u : u64
        let s_15_3: u64 = 1;
        // D s_15_4: bit-extract s_15_2 s_15_0 s_15_3
        let s_15_4: Bits = (Bits::new(
            ((s_15_2) >> (s_15_0)).value(),
            u16::try_from(s_15_3).unwrap(),
        ));
        // D s_15_5: cast reint s_15_4 -> u8
        let s_15_5: bool = ((s_15_4.value()) != 0);
        // C s_15_6: const #0s : i
        let s_15_6: i128 = 0;
        // C s_15_7: const #0u : u64
        let s_15_7: u64 = 0;
        // D s_15_8: cast zx s_15_5 -> u64
        let s_15_8: u64 = (s_15_5 as u64);
        // C s_15_9: const #1u : u64
        let s_15_9: u64 = 1;
        // D s_15_10: and s_15_8 s_15_9
        let s_15_10: u64 = ((s_15_8) & (s_15_9));
        // D s_15_11: cmp-eq s_15_10 s_15_9
        let s_15_11: bool = ((s_15_10) == (s_15_9));
        // D s_15_12: lsl s_15_8 s_15_6
        let s_15_12: u64 = s_15_8 << s_15_6;
        // D s_15_13: or s_15_7 s_15_12
        let s_15_13: u64 = ((s_15_7) | (s_15_12));
        // D s_15_14: cmpl s_15_12
        let s_15_14: u64 = !s_15_12;
        // D s_15_15: and s_15_7 s_15_14
        let s_15_15: u64 = ((s_15_7) & (s_15_14));
        // D s_15_16: select s_15_11 s_15_13 s_15_15
        let s_15_16: u64 = if s_15_11 { s_15_13 } else { s_15_15 };
        // D s_15_17: cast trunc s_15_16 -> u8
        let s_15_17: bool = ((s_15_16) != 0);
        // D s_15_18: cast zx s_15_17 -> bv
        let s_15_18: Bits = Bits::new(s_15_17 as u128, 1u16);
        // C s_15_19: const #1u : u8
        let s_15_19: bool = true;
        // C s_15_20: cast zx s_15_19 -> bv
        let s_15_20: Bits = Bits::new(s_15_19 as u128, 1u16);
        // D s_15_21: cmp-eq s_15_18 s_15_20
        let s_15_21: bool = ((s_15_18) == (s_15_20));
        // N s_15_22: branch s_15_21 b21 b16
        if s_15_21 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0s : i
        let s_16_0: i128 = 0;
        // D s_16_1: read-var Vn:u8
        let s_16_1: u8 = fn_state.Vn;
        // D s_16_2: cast zx s_16_1 -> bv
        let s_16_2: Bits = Bits::new(s_16_1 as u128, 4u16);
        // C s_16_3: const #1u : u64
        let s_16_3: u64 = 1;
        // D s_16_4: bit-extract s_16_2 s_16_0 s_16_3
        let s_16_4: Bits = (Bits::new(
            ((s_16_2) >> (s_16_0)).value(),
            u16::try_from(s_16_3).unwrap(),
        ));
        // D s_16_5: cast reint s_16_4 -> u8
        let s_16_5: bool = ((s_16_4.value()) != 0);
        // C s_16_6: const #0s : i
        let s_16_6: i128 = 0;
        // C s_16_7: const #0u : u64
        let s_16_7: u64 = 0;
        // D s_16_8: cast zx s_16_5 -> u64
        let s_16_8: u64 = (s_16_5 as u64);
        // C s_16_9: const #1u : u64
        let s_16_9: u64 = 1;
        // D s_16_10: and s_16_8 s_16_9
        let s_16_10: u64 = ((s_16_8) & (s_16_9));
        // D s_16_11: cmp-eq s_16_10 s_16_9
        let s_16_11: bool = ((s_16_10) == (s_16_9));
        // D s_16_12: lsl s_16_8 s_16_6
        let s_16_12: u64 = s_16_8 << s_16_6;
        // D s_16_13: or s_16_7 s_16_12
        let s_16_13: u64 = ((s_16_7) | (s_16_12));
        // D s_16_14: cmpl s_16_12
        let s_16_14: u64 = !s_16_12;
        // D s_16_15: and s_16_7 s_16_14
        let s_16_15: u64 = ((s_16_7) & (s_16_14));
        // D s_16_16: select s_16_11 s_16_13 s_16_15
        let s_16_16: u64 = if s_16_11 { s_16_13 } else { s_16_15 };
        // D s_16_17: cast trunc s_16_16 -> u8
        let s_16_17: bool = ((s_16_16) != 0);
        // D s_16_18: cast zx s_16_17 -> bv
        let s_16_18: Bits = Bits::new(s_16_17 as u128, 1u16);
        // C s_16_19: const #1u : u8
        let s_16_19: bool = true;
        // C s_16_20: cast zx s_16_19 -> bv
        let s_16_20: Bits = Bits::new(s_16_19 as u128, 1u16);
        // D s_16_21: cmp-eq s_16_18 s_16_20
        let s_16_21: bool = ((s_16_18) == (s_16_20));
        // D s_16_22: write-var gs#307340 <= s_16_21
        fn_state.gs_307340 = s_16_21;
        // N s_16_23: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#307340:u8
        let s_17_0: bool = fn_state.gs_307340;
        // N s_17_1: branch s_17_0 b20 b18
        if s_17_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #0s : i
        let s_18_0: i128 = 0;
        // D s_18_1: read-var Vm:u8
        let s_18_1: u8 = fn_state.Vm;
        // D s_18_2: cast zx s_18_1 -> bv
        let s_18_2: Bits = Bits::new(s_18_1 as u128, 4u16);
        // C s_18_3: const #1u : u64
        let s_18_3: u64 = 1;
        // D s_18_4: bit-extract s_18_2 s_18_0 s_18_3
        let s_18_4: Bits = (Bits::new(
            ((s_18_2) >> (s_18_0)).value(),
            u16::try_from(s_18_3).unwrap(),
        ));
        // D s_18_5: cast reint s_18_4 -> u8
        let s_18_5: bool = ((s_18_4.value()) != 0);
        // C s_18_6: const #0s : i
        let s_18_6: i128 = 0;
        // C s_18_7: const #0u : u64
        let s_18_7: u64 = 0;
        // D s_18_8: cast zx s_18_5 -> u64
        let s_18_8: u64 = (s_18_5 as u64);
        // C s_18_9: const #1u : u64
        let s_18_9: u64 = 1;
        // D s_18_10: and s_18_8 s_18_9
        let s_18_10: u64 = ((s_18_8) & (s_18_9));
        // D s_18_11: cmp-eq s_18_10 s_18_9
        let s_18_11: bool = ((s_18_10) == (s_18_9));
        // D s_18_12: lsl s_18_8 s_18_6
        let s_18_12: u64 = s_18_8 << s_18_6;
        // D s_18_13: or s_18_7 s_18_12
        let s_18_13: u64 = ((s_18_7) | (s_18_12));
        // D s_18_14: cmpl s_18_12
        let s_18_14: u64 = !s_18_12;
        // D s_18_15: and s_18_7 s_18_14
        let s_18_15: u64 = ((s_18_7) & (s_18_14));
        // D s_18_16: select s_18_11 s_18_13 s_18_15
        let s_18_16: u64 = if s_18_11 { s_18_13 } else { s_18_15 };
        // D s_18_17: cast trunc s_18_16 -> u8
        let s_18_17: bool = ((s_18_16) != 0);
        // D s_18_18: cast zx s_18_17 -> bv
        let s_18_18: Bits = Bits::new(s_18_17 as u128, 1u16);
        // C s_18_19: const #1u : u8
        let s_18_19: bool = true;
        // C s_18_20: cast zx s_18_19 -> bv
        let s_18_20: Bits = Bits::new(s_18_19 as u128, 1u16);
        // D s_18_21: cmp-eq s_18_18 s_18_20
        let s_18_21: bool = ((s_18_18) == (s_18_20));
        // D s_18_22: write-var gs#307343 <= s_18_21
        fn_state.gs_307343 = s_18_21;
        // N s_18_23: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#307343:u8
        let s_19_0: bool = fn_state.gs_307343;
        // D s_19_1: write-var gs#307344 <= s_19_0
        fn_state.gs_307344 = s_19_0;
        // N s_19_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#307343 <= s_20_0
        fn_state.gs_307343 = s_20_0;
        // N s_20_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#307340 <= s_21_0
        fn_state.gs_307340 = s_21_0;
        // N s_21_2: jump b17
        return block_17(state, tracer, fn_state);
    }
}
