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
use HaveQRDMLAHExt::*;
use execute_aarch32_instrs_VQRDMLSH_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VQRDMLSH_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
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
        ga_359329: i128,
        m: i64,
        gs_316657: bool,
        gs_316655: bool,
        gs_316652: bool,
        esize: i64,
        ga_359327: i64,
        n: i64,
        gs_316656: bool,
        d: i64,
        elements: i64,
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
        // S s_0_1: call HaveQRDMLAHExt(s_0_0)
        let s_0_1: bool = HaveQRDMLAHExt(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b21 b1
        if s_0_2 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var Q:u8
        let s_1_0: bool = fn_state.Q;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 1u16);
        // C s_1_2: const #1u : u8
        let s_1_2: bool = true;
        // C s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 1u16);
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
        // D s_2_1: write-var gs#316656 <= s_2_0
        fn_state.gs_316656 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#316656:u8
        let s_3_0: bool = fn_state.gs_316656;
        // N s_3_1: branch s_3_0 b13 b4
        if s_3_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var size:u8
        let s_4_0: u8 = fn_state.size;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 2u16);
        // C s_4_2: const #0u : u8
        let s_4_2: u8 = 0;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 2u16);
        // D s_4_4: cmp-eq s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) == (s_4_3));
        // N s_4_5: branch s_4_4 b12 b5
        if s_4_4 {
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
        // D s_5_5: write-var gs#316657 <= s_5_4
        fn_state.gs_316657 = s_5_4;
        // N s_5_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#316657:u8
        let s_6_0: bool = fn_state.gs_316657;
        // N s_6_1: branch s_6_0 b11 b7
        if s_6_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var size:u8
        let s_7_0: u8 = fn_state.size;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 2u16);
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (s_7_1.value() as i128);
        // D s_7_3: cast reint s_7_2 -> i64
        let s_7_3: i64 = (s_7_2 as i64);
        // C s_7_4: const #8s : i64
        let s_7_4: i64 = 8;
        // D s_7_5: lsl s_7_4 s_7_3
        let s_7_5: i64 = s_7_4 << s_7_3;
        // D s_7_6: write-var esize <= s_7_5
        fn_state.esize = s_7_5;
        // C s_7_7: const #64s : i
        let s_7_7: i128 = 64;
        // D s_7_8: read-var esize:i64
        let s_7_8: i64 = fn_state.esize;
        // D s_7_9: cast zx s_7_8 -> i
        let s_7_9: i128 = (i128::try_from(s_7_8).unwrap());
        // D s_7_10: div s_7_7 s_7_9
        let s_7_10: i128 = ((s_7_7) / (s_7_9));
        // D s_7_11: cast reint s_7_10 -> i64
        let s_7_11: i64 = (s_7_10 as i64);
        // D s_7_12: write-var elements <= s_7_11
        fn_state.elements = s_7_11;
        // D s_7_13: read-var D:u8
        let s_7_13: bool = fn_state.D;
        // D s_7_14: cast zx s_7_13 -> bv
        let s_7_14: Bits = Bits::new(s_7_13 as u128, 1u16);
        // D s_7_15: read-var Vd:u8
        let s_7_15: u8 = fn_state.Vd;
        // D s_7_16: cast zx s_7_15 -> bv
        let s_7_16: Bits = Bits::new(s_7_15 as u128, 4u16);
        // D s_7_17: cast reint s_7_14 -> u128
        let s_7_17: u128 = (s_7_14.value() as u128);
        // D s_7_18: size-of s_7_14
        let s_7_18: u16 = s_7_14.length();
        // D s_7_19: cast reint s_7_16 -> u128
        let s_7_19: u128 = (s_7_16.value() as u128);
        // D s_7_20: size-of s_7_16
        let s_7_20: u16 = s_7_16.length();
        // D s_7_21: lsl s_7_17 s_7_20
        let s_7_21: u128 = s_7_17 << s_7_20;
        // D s_7_22: or s_7_21 s_7_19
        let s_7_22: u128 = ((s_7_21) | (s_7_19));
        // D s_7_23: add s_7_18 s_7_20
        let s_7_23: u16 = (s_7_18 + s_7_20);
        // D s_7_24: create-bits s_7_22 s_7_23
        let s_7_24: Bits = Bits::new(s_7_22, s_7_23);
        // D s_7_25: cast reint s_7_24 -> u8
        let s_7_25: u8 = (s_7_24.value() as u8);
        // D s_7_26: cast zx s_7_25 -> bv
        let s_7_26: Bits = Bits::new(s_7_25 as u128, 5u16);
        // D s_7_27: cast zx s_7_26 -> i
        let s_7_27: i128 = (s_7_26.value() as i128);
        // D s_7_28: cast reint s_7_27 -> i64
        let s_7_28: i64 = (s_7_27 as i64);
        // D s_7_29: write-var d <= s_7_28
        fn_state.d = s_7_28;
        // D s_7_30: read-var N:u8
        let s_7_30: bool = fn_state.N;
        // D s_7_31: cast zx s_7_30 -> bv
        let s_7_31: Bits = Bits::new(s_7_30 as u128, 1u16);
        // D s_7_32: read-var Vn:u8
        let s_7_32: u8 = fn_state.Vn;
        // D s_7_33: cast zx s_7_32 -> bv
        let s_7_33: Bits = Bits::new(s_7_32 as u128, 4u16);
        // D s_7_34: cast reint s_7_31 -> u128
        let s_7_34: u128 = (s_7_31.value() as u128);
        // D s_7_35: size-of s_7_31
        let s_7_35: u16 = s_7_31.length();
        // D s_7_36: cast reint s_7_33 -> u128
        let s_7_36: u128 = (s_7_33.value() as u128);
        // D s_7_37: size-of s_7_33
        let s_7_37: u16 = s_7_33.length();
        // D s_7_38: lsl s_7_34 s_7_37
        let s_7_38: u128 = s_7_34 << s_7_37;
        // D s_7_39: or s_7_38 s_7_36
        let s_7_39: u128 = ((s_7_38) | (s_7_36));
        // D s_7_40: add s_7_35 s_7_37
        let s_7_40: u16 = (s_7_35 + s_7_37);
        // D s_7_41: create-bits s_7_39 s_7_40
        let s_7_41: Bits = Bits::new(s_7_39, s_7_40);
        // D s_7_42: cast reint s_7_41 -> u8
        let s_7_42: u8 = (s_7_41.value() as u8);
        // D s_7_43: cast zx s_7_42 -> bv
        let s_7_43: Bits = Bits::new(s_7_42 as u128, 5u16);
        // D s_7_44: cast zx s_7_43 -> i
        let s_7_44: i128 = (s_7_43.value() as i128);
        // D s_7_45: cast reint s_7_44 -> i64
        let s_7_45: i64 = (s_7_44 as i64);
        // D s_7_46: write-var n <= s_7_45
        fn_state.n = s_7_45;
        // D s_7_47: read-var M:u8
        let s_7_47: bool = fn_state.M;
        // D s_7_48: cast zx s_7_47 -> bv
        let s_7_48: Bits = Bits::new(s_7_47 as u128, 1u16);
        // D s_7_49: read-var Vm:u8
        let s_7_49: u8 = fn_state.Vm;
        // D s_7_50: cast zx s_7_49 -> bv
        let s_7_50: Bits = Bits::new(s_7_49 as u128, 4u16);
        // D s_7_51: cast reint s_7_48 -> u128
        let s_7_51: u128 = (s_7_48.value() as u128);
        // D s_7_52: size-of s_7_48
        let s_7_52: u16 = s_7_48.length();
        // D s_7_53: cast reint s_7_50 -> u128
        let s_7_53: u128 = (s_7_50.value() as u128);
        // D s_7_54: size-of s_7_50
        let s_7_54: u16 = s_7_50.length();
        // D s_7_55: lsl s_7_51 s_7_54
        let s_7_55: u128 = s_7_51 << s_7_54;
        // D s_7_56: or s_7_55 s_7_53
        let s_7_56: u128 = ((s_7_55) | (s_7_53));
        // D s_7_57: add s_7_52 s_7_54
        let s_7_57: u16 = (s_7_52 + s_7_54);
        // D s_7_58: create-bits s_7_56 s_7_57
        let s_7_58: Bits = Bits::new(s_7_56, s_7_57);
        // D s_7_59: cast reint s_7_58 -> u8
        let s_7_59: u8 = (s_7_58.value() as u8);
        // D s_7_60: cast zx s_7_59 -> bv
        let s_7_60: Bits = Bits::new(s_7_59 as u128, 5u16);
        // D s_7_61: cast zx s_7_60 -> i
        let s_7_61: i128 = (s_7_60.value() as i128);
        // D s_7_62: cast reint s_7_61 -> i64
        let s_7_62: i64 = (s_7_61 as i64);
        // D s_7_63: write-var m <= s_7_62
        fn_state.m = s_7_62;
        // D s_7_64: read-var Q:u8
        let s_7_64: bool = fn_state.Q;
        // D s_7_65: cast zx s_7_64 -> bv
        let s_7_65: Bits = Bits::new(s_7_64 as u128, 1u16);
        // C s_7_66: const #0u : u8
        let s_7_66: bool = false;
        // C s_7_67: cast zx s_7_66 -> bv
        let s_7_67: Bits = Bits::new(s_7_66 as u128, 1u16);
        // D s_7_68: cmp-eq s_7_65 s_7_67
        let s_7_68: bool = ((s_7_65) == (s_7_67));
        // N s_7_69: branch s_7_68 b10 b8
        if s_7_68 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #2s : i64
        let s_8_0: i64 = 2;
        // D s_8_1: write-var ga#359327 <= s_8_0
        fn_state.ga_359327 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var ga#359327:i64
        let s_9_0: i64 = fn_state.ga_359327;
        // D s_9_1: read-var esize:i64
        let s_9_1: i64 = fn_state.esize;
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (i128::try_from(s_9_1).unwrap());
        // D s_9_3: cast reint s_9_2 -> i64
        let s_9_3: i64 = (s_9_2 as i64);
        // D s_9_4: read-var elements:i64
        let s_9_4: i64 = fn_state.elements;
        // D s_9_5: cast zx s_9_4 -> i
        let s_9_5: i128 = (i128::try_from(s_9_4).unwrap());
        // D s_9_6: read-var d:i64
        let s_9_6: i64 = fn_state.d;
        // D s_9_7: read-var ga#359329:i
        let s_9_7: i128 = fn_state.ga_359329;
        // D s_9_8: read-var m:i64
        let s_9_8: i64 = fn_state.m;
        // D s_9_9: read-var n:i64
        let s_9_9: i64 = fn_state.n;
        // C s_9_10: const #0u : u8
        let s_9_10: bool = false;
        // D s_9_11: call execute_aarch32_instrs_VQRDMLSH_Op_A_txt(s_9_6, s_9_5, s_9_3, s_9_7, s_9_8, s_9_9, s_9_0, s_9_10)
        let s_9_11: () = execute_aarch32_instrs_VQRDMLSH_Op_A_txt(
            state,
            tracer,
            s_9_6,
            s_9_5,
            s_9_3,
            s_9_7,
            s_9_8,
            s_9_9,
            s_9_0,
            s_9_10,
        );
        // N s_9_12: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #1s : i64
        let s_10_0: i64 = 1;
        // D s_10_1: write-var ga#359327 <= s_10_0
        fn_state.ga_359327 = s_10_0;
        // N s_10_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: panic
        panic!("{:?}", ());
        // N s_11_1: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var gs#316657 <= s_12_0
        fn_state.gs_316657 = s_12_0;
        // N s_12_2: jump b6
        return block_6(state, tracer, fn_state);
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
        // C s_14_0: const #0s : i
        let s_14_0: i128 = 0;
        // D s_14_1: read-var Vd:u8
        let s_14_1: u8 = fn_state.Vd;
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
        // N s_14_22: branch s_14_21 b20 b15
        if s_14_21 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #0s : i
        let s_15_0: i128 = 0;
        // D s_15_1: read-var Vn:u8
        let s_15_1: u8 = fn_state.Vn;
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
        // D s_15_22: write-var gs#316652 <= s_15_21
        fn_state.gs_316652 = s_15_21;
        // N s_15_23: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#316652:u8
        let s_16_0: bool = fn_state.gs_316652;
        // N s_16_1: branch s_16_0 b19 b17
        if s_16_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #0s : i
        let s_17_0: i128 = 0;
        // D s_17_1: read-var Vm:u8
        let s_17_1: u8 = fn_state.Vm;
        // D s_17_2: cast zx s_17_1 -> bv
        let s_17_2: Bits = Bits::new(s_17_1 as u128, 4u16);
        // C s_17_3: const #1u : u64
        let s_17_3: u64 = 1;
        // D s_17_4: bit-extract s_17_2 s_17_0 s_17_3
        let s_17_4: Bits = (Bits::new(
            ((s_17_2) >> (s_17_0)).value(),
            u16::try_from(s_17_3).unwrap(),
        ));
        // D s_17_5: cast reint s_17_4 -> u8
        let s_17_5: bool = ((s_17_4.value()) != 0);
        // C s_17_6: const #0s : i
        let s_17_6: i128 = 0;
        // C s_17_7: const #0u : u64
        let s_17_7: u64 = 0;
        // D s_17_8: cast zx s_17_5 -> u64
        let s_17_8: u64 = (s_17_5 as u64);
        // C s_17_9: const #1u : u64
        let s_17_9: u64 = 1;
        // D s_17_10: and s_17_8 s_17_9
        let s_17_10: u64 = ((s_17_8) & (s_17_9));
        // D s_17_11: cmp-eq s_17_10 s_17_9
        let s_17_11: bool = ((s_17_10) == (s_17_9));
        // D s_17_12: lsl s_17_8 s_17_6
        let s_17_12: u64 = s_17_8 << s_17_6;
        // D s_17_13: or s_17_7 s_17_12
        let s_17_13: u64 = ((s_17_7) | (s_17_12));
        // D s_17_14: cmpl s_17_12
        let s_17_14: u64 = !s_17_12;
        // D s_17_15: and s_17_7 s_17_14
        let s_17_15: u64 = ((s_17_7) & (s_17_14));
        // D s_17_16: select s_17_11 s_17_13 s_17_15
        let s_17_16: u64 = if s_17_11 { s_17_13 } else { s_17_15 };
        // D s_17_17: cast trunc s_17_16 -> u8
        let s_17_17: bool = ((s_17_16) != 0);
        // D s_17_18: cast zx s_17_17 -> bv
        let s_17_18: Bits = Bits::new(s_17_17 as u128, 1u16);
        // C s_17_19: const #1u : u8
        let s_17_19: bool = true;
        // C s_17_20: cast zx s_17_19 -> bv
        let s_17_20: Bits = Bits::new(s_17_19 as u128, 1u16);
        // D s_17_21: cmp-eq s_17_18 s_17_20
        let s_17_21: bool = ((s_17_18) == (s_17_20));
        // D s_17_22: write-var gs#316655 <= s_17_21
        fn_state.gs_316655 = s_17_21;
        // N s_17_23: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#316655:u8
        let s_18_0: bool = fn_state.gs_316655;
        // D s_18_1: write-var gs#316656 <= s_18_0
        fn_state.gs_316656 = s_18_0;
        // N s_18_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #1u : u8
        let s_19_0: bool = true;
        // D s_19_1: write-var gs#316655 <= s_19_0
        fn_state.gs_316655 = s_19_0;
        // N s_19_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#316652 <= s_20_0
        fn_state.gs_316652 = s_20_0;
        // N s_20_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_21_0: panic
        panic!("{:?}", ());
        // N s_21_1: return
        return;
    }
}
