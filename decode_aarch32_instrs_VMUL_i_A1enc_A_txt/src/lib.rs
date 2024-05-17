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
use execute_aarch32_instrs_VMUL_i_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VMUL_i_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    op: bool,
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
        polynomial: bool,
        esize: i64,
        n: i64,
        d: i64,
        gs_313732: bool,
        gs_313733: bool,
        elements: i64,
        gs_313724: bool,
        gs_313729: bool,
        ga_356807: i64,
        gs_313723: bool,
        op: bool,
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
        op,
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
        // N s_2_5: branch s_2_4 b24 b3
        if s_2_4 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var op:u8
        let s_3_0: bool = fn_state.op;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 1u16);
        // C s_3_2: const #1u : u8
        let s_3_2: bool = true;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 1u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // N s_3_5: branch s_3_4 b23 b4
        if s_3_4 {
            return block_23(state, tracer, fn_state);
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
        // D s_4_1: write-var gs#313723 <= s_4_0
        fn_state.gs_313723 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#313723:u8
        let s_5_0: bool = fn_state.gs_313723;
        // D s_5_1: write-var gs#313724 <= s_5_0
        fn_state.gs_313724 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#313724:u8
        let s_6_0: bool = fn_state.gs_313724;
        // N s_6_1: branch s_6_0 b22 b7
        if s_6_0 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var Q:u8
        let s_7_0: bool = fn_state.Q;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 1u16);
        // C s_7_2: const #1u : u8
        let s_7_2: bool = true;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 1u16);
        // D s_7_4: cmp-eq s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) == (s_7_3));
        // N s_7_5: branch s_7_4 b15 b8
        if s_7_4 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#313733 <= s_8_0
        fn_state.gs_313733 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#313733:u8
        let s_9_0: bool = fn_state.gs_313733;
        // N s_9_1: branch s_9_0 b14 b10
        if s_9_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var op:u8
        let s_10_0: bool = fn_state.op;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 1u16);
        // C s_10_2: const #1u : u8
        let s_10_2: bool = true;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // D s_10_5: write-var polynomial <= s_10_4
        fn_state.polynomial = s_10_4;
        // D s_10_6: read-var size:u8
        let s_10_6: u8 = fn_state.size;
        // D s_10_7: cast zx s_10_6 -> bv
        let s_10_7: Bits = Bits::new(s_10_6 as u128, 2u16);
        // D s_10_8: cast zx s_10_7 -> i
        let s_10_8: i128 = (s_10_7.value() as i128);
        // D s_10_9: cast reint s_10_8 -> i64
        let s_10_9: i64 = (s_10_8 as i64);
        // C s_10_10: const #8s : i64
        let s_10_10: i64 = 8;
        // D s_10_11: lsl s_10_10 s_10_9
        let s_10_11: i64 = s_10_10 << s_10_9;
        // D s_10_12: write-var esize <= s_10_11
        fn_state.esize = s_10_11;
        // C s_10_13: const #64s : i
        let s_10_13: i128 = 64;
        // D s_10_14: read-var esize:i64
        let s_10_14: i64 = fn_state.esize;
        // D s_10_15: cast zx s_10_14 -> i
        let s_10_15: i128 = (i128::try_from(s_10_14).unwrap());
        // D s_10_16: div s_10_13 s_10_15
        let s_10_16: i128 = ((s_10_13) / (s_10_15));
        // D s_10_17: cast reint s_10_16 -> i64
        let s_10_17: i64 = (s_10_16 as i64);
        // D s_10_18: write-var elements <= s_10_17
        fn_state.elements = s_10_17;
        // D s_10_19: read-var D:u8
        let s_10_19: bool = fn_state.D;
        // D s_10_20: cast zx s_10_19 -> bv
        let s_10_20: Bits = Bits::new(s_10_19 as u128, 1u16);
        // D s_10_21: read-var Vd:u8
        let s_10_21: u8 = fn_state.Vd;
        // D s_10_22: cast zx s_10_21 -> bv
        let s_10_22: Bits = Bits::new(s_10_21 as u128, 4u16);
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
        // D s_10_35: write-var d <= s_10_34
        fn_state.d = s_10_34;
        // D s_10_36: read-var N:u8
        let s_10_36: bool = fn_state.N;
        // D s_10_37: cast zx s_10_36 -> bv
        let s_10_37: Bits = Bits::new(s_10_36 as u128, 1u16);
        // D s_10_38: read-var Vn:u8
        let s_10_38: u8 = fn_state.Vn;
        // D s_10_39: cast zx s_10_38 -> bv
        let s_10_39: Bits = Bits::new(s_10_38 as u128, 4u16);
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
        // D s_10_52: write-var n <= s_10_51
        fn_state.n = s_10_51;
        // D s_10_53: read-var M:u8
        let s_10_53: bool = fn_state.M;
        // D s_10_54: cast zx s_10_53 -> bv
        let s_10_54: Bits = Bits::new(s_10_53 as u128, 1u16);
        // D s_10_55: read-var Vm:u8
        let s_10_55: u8 = fn_state.Vm;
        // D s_10_56: cast zx s_10_55 -> bv
        let s_10_56: Bits = Bits::new(s_10_55 as u128, 4u16);
        // D s_10_57: cast reint s_10_54 -> u128
        let s_10_57: u128 = (s_10_54.value() as u128);
        // D s_10_58: size-of s_10_54
        let s_10_58: u16 = s_10_54.length();
        // D s_10_59: cast reint s_10_56 -> u128
        let s_10_59: u128 = (s_10_56.value() as u128);
        // D s_10_60: size-of s_10_56
        let s_10_60: u16 = s_10_56.length();
        // D s_10_61: lsl s_10_57 s_10_60
        let s_10_61: u128 = s_10_57 << s_10_60;
        // D s_10_62: or s_10_61 s_10_59
        let s_10_62: u128 = ((s_10_61) | (s_10_59));
        // D s_10_63: add s_10_58 s_10_60
        let s_10_63: u16 = (s_10_58 + s_10_60);
        // D s_10_64: create-bits s_10_62 s_10_63
        let s_10_64: Bits = Bits::new(s_10_62, s_10_63);
        // D s_10_65: cast reint s_10_64 -> u8
        let s_10_65: u8 = (s_10_64.value() as u8);
        // D s_10_66: cast zx s_10_65 -> bv
        let s_10_66: Bits = Bits::new(s_10_65 as u128, 5u16);
        // D s_10_67: cast zx s_10_66 -> i
        let s_10_67: i128 = (s_10_66.value() as i128);
        // D s_10_68: cast reint s_10_67 -> i64
        let s_10_68: i64 = (s_10_67 as i64);
        // D s_10_69: write-var m <= s_10_68
        fn_state.m = s_10_68;
        // D s_10_70: read-var Q:u8
        let s_10_70: bool = fn_state.Q;
        // D s_10_71: cast zx s_10_70 -> bv
        let s_10_71: Bits = Bits::new(s_10_70 as u128, 1u16);
        // C s_10_72: const #0u : u8
        let s_10_72: bool = false;
        // C s_10_73: cast zx s_10_72 -> bv
        let s_10_73: Bits = Bits::new(s_10_72 as u128, 1u16);
        // D s_10_74: cmp-eq s_10_71 s_10_73
        let s_10_74: bool = ((s_10_71) == (s_10_73));
        // N s_10_75: branch s_10_74 b13 b11
        if s_10_74 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #2s : i64
        let s_11_0: i64 = 2;
        // D s_11_1: write-var ga#356807 <= s_11_0
        fn_state.ga_356807 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var ga#356807:i64
        let s_12_0: i64 = fn_state.ga_356807;
        // D s_12_1: read-var esize:i64
        let s_12_1: i64 = fn_state.esize;
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // D s_12_3: cast reint s_12_2 -> i64
        let s_12_3: i64 = (s_12_2 as i64);
        // D s_12_4: read-var elements:i64
        let s_12_4: i64 = fn_state.elements;
        // D s_12_5: cast zx s_12_4 -> i
        let s_12_5: i128 = (i128::try_from(s_12_4).unwrap());
        // D s_12_6: read-var d:i64
        let s_12_6: i64 = fn_state.d;
        // C s_12_7: const #0u : u8
        let s_12_7: bool = false;
        // D s_12_8: read-var m:i64
        let s_12_8: i64 = fn_state.m;
        // D s_12_9: read-var n:i64
        let s_12_9: i64 = fn_state.n;
        // D s_12_10: read-var polynomial:u8
        let s_12_10: bool = fn_state.polynomial;
        // C s_12_11: const #0u : u8
        let s_12_11: bool = false;
        // D s_12_12: call execute_aarch32_instrs_VMUL_i_Op_A_txt(s_12_6, s_12_5, s_12_3, s_12_7, s_12_8, s_12_9, s_12_10, s_12_0, s_12_11)
        let s_12_12: () = execute_aarch32_instrs_VMUL_i_Op_A_txt(
            state,
            tracer,
            s_12_6,
            s_12_5,
            s_12_3,
            s_12_7,
            s_12_8,
            s_12_9,
            s_12_10,
            s_12_0,
            s_12_11,
        );
        // N s_12_13: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #1s : i64
        let s_13_0: i64 = 1;
        // D s_13_1: write-var ga#356807 <= s_13_0
        fn_state.ga_356807 = s_13_0;
        // N s_13_2: jump b12
        return block_12(state, tracer, fn_state);
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
        // D s_16_22: write-var gs#313729 <= s_16_21
        fn_state.gs_313729 = s_16_21;
        // N s_16_23: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#313729:u8
        let s_17_0: bool = fn_state.gs_313729;
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
        // D s_18_22: write-var gs#313732 <= s_18_21
        fn_state.gs_313732 = s_18_21;
        // N s_18_23: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#313732:u8
        let s_19_0: bool = fn_state.gs_313732;
        // D s_19_1: write-var gs#313733 <= s_19_0
        fn_state.gs_313733 = s_19_0;
        // N s_19_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#313732 <= s_20_0
        fn_state.gs_313732 = s_20_0;
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
        // D s_21_1: write-var gs#313729 <= s_21_0
        fn_state.gs_313729 = s_21_0;
        // N s_21_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: panic
        panic!("{:?}", ());
        // N s_22_1: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var size:u8
        let s_23_0: u8 = fn_state.size;
        // D s_23_1: cast zx s_23_0 -> bv
        let s_23_1: Bits = Bits::new(s_23_0 as u128, 2u16);
        // C s_23_2: const #0u : u8
        let s_23_2: u8 = 0;
        // C s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 2u16);
        // D s_23_4: cmp-ne s_23_1 s_23_3
        let s_23_4: bool = ((s_23_1) != (s_23_3));
        // D s_23_5: write-var gs#313723 <= s_23_4
        fn_state.gs_313723 = s_23_4;
        // N s_23_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var gs#313724 <= s_24_0
        fn_state.gs_313724 = s_24_0;
        // N s_24_2: jump b6
        return block_6(state, tracer, fn_state);
    }
}
