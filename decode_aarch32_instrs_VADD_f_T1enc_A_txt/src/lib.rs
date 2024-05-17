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
use execute_aarch32_instrs_VADD_f_Op_A_txt::*;
use InITBlock::*;
use common::*;
pub fn decode_aarch32_instrs_VADD_f_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    sz: bool,
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
        esizeshadow_7356: i64,
        esize: i64,
        n: i64,
        gs_306199: bool,
        d: i64,
        gs_306198: bool,
        elements: i64,
        gs_306195: bool,
        gs_306200: bool,
        ga_350793: i64,
        elementsshadow_7357: i64,
        D: bool,
        sz: bool,
        Vn: u8,
        Vd: u8,
        N: bool,
        Q: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        D,
        sz,
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
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#306199 <= s_3_0
        fn_state.gs_306199 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#306199:u8
        let s_4_0: bool = fn_state.gs_306199;
        // N s_4_1: branch s_4_0 b17 b5
        if s_4_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var sz:u8
        let s_5_0: bool = fn_state.sz;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 1u16);
        // C s_5_2: const #1u : u8
        let s_5_2: bool = true;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // N s_5_5: branch s_5_4 b16 b6
        if s_5_4 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#306200 <= s_6_0
        fn_state.gs_306200 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#306200:u8
        let s_7_0: bool = fn_state.gs_306200;
        // N s_7_1: branch s_7_0 b15 b8
        if s_7_0 {
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
        // C s_8_0: const #16s : i64
        let s_8_0: i64 = 16;
        // D s_8_1: write-var esize <= s_8_0
        fn_state.esize = s_8_0;
        // C s_8_2: const #2s : i64
        let s_8_2: i64 = 2;
        // D s_8_3: write-var elements <= s_8_2
        fn_state.elements = s_8_2;
        // D s_8_4: read-var sz:u8
        let s_8_4: bool = fn_state.sz;
        // D s_8_5: cast zx s_8_4 -> bv
        let s_8_5: Bits = Bits::new(s_8_4 as u128, 1u16);
        // C s_8_6: const #0u : u8
        let s_8_6: bool = false;
        // C s_8_7: cast zx s_8_6 -> bv
        let s_8_7: Bits = Bits::new(s_8_6 as u128, 1u16);
        // D s_8_8: cmp-eq s_8_5 s_8_7
        let s_8_8: bool = ((s_8_5) == (s_8_7));
        // D s_8_9: not s_8_8
        let s_8_9: bool = !s_8_8;
        // N s_8_10: branch s_8_9 b14 b9
        if s_8_9 {
            return block_14(state, tracer, fn_state);
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
        // C s_9_2: const #2s : i64
        let s_9_2: i64 = 2;
        // D s_9_3: write-var elements <= s_9_2
        fn_state.elements = s_9_2;
        // N s_9_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var esize:i64
        let s_10_0: i64 = fn_state.esize;
        // D s_10_1: write-var esizeshadow#7356 <= s_10_0
        fn_state.esizeshadow_7356 = s_10_0;
        // D s_10_2: read-var elements:i64
        let s_10_2: i64 = fn_state.elements;
        // D s_10_3: write-var elementsshadow#7357 <= s_10_2
        fn_state.elementsshadow_7357 = s_10_2;
        // D s_10_4: read-var D:u8
        let s_10_4: bool = fn_state.D;
        // D s_10_5: cast zx s_10_4 -> bv
        let s_10_5: Bits = Bits::new(s_10_4 as u128, 1u16);
        // D s_10_6: read-var Vd:u8
        let s_10_6: u8 = fn_state.Vd;
        // D s_10_7: cast zx s_10_6 -> bv
        let s_10_7: Bits = Bits::new(s_10_6 as u128, 4u16);
        // D s_10_8: cast reint s_10_5 -> u128
        let s_10_8: u128 = (s_10_5.value() as u128);
        // D s_10_9: size-of s_10_5
        let s_10_9: u16 = s_10_5.length();
        // D s_10_10: cast reint s_10_7 -> u128
        let s_10_10: u128 = (s_10_7.value() as u128);
        // D s_10_11: size-of s_10_7
        let s_10_11: u16 = s_10_7.length();
        // D s_10_12: lsl s_10_8 s_10_11
        let s_10_12: u128 = s_10_8 << s_10_11;
        // D s_10_13: or s_10_12 s_10_10
        let s_10_13: u128 = ((s_10_12) | (s_10_10));
        // D s_10_14: add s_10_9 s_10_11
        let s_10_14: u16 = (s_10_9 + s_10_11);
        // D s_10_15: create-bits s_10_13 s_10_14
        let s_10_15: Bits = Bits::new(s_10_13, s_10_14);
        // D s_10_16: cast reint s_10_15 -> u8
        let s_10_16: u8 = (s_10_15.value() as u8);
        // D s_10_17: cast zx s_10_16 -> bv
        let s_10_17: Bits = Bits::new(s_10_16 as u128, 5u16);
        // D s_10_18: cast zx s_10_17 -> i
        let s_10_18: i128 = (s_10_17.value() as i128);
        // D s_10_19: cast reint s_10_18 -> i64
        let s_10_19: i64 = (s_10_18 as i64);
        // D s_10_20: write-var d <= s_10_19
        fn_state.d = s_10_19;
        // D s_10_21: read-var N:u8
        let s_10_21: bool = fn_state.N;
        // D s_10_22: cast zx s_10_21 -> bv
        let s_10_22: Bits = Bits::new(s_10_21 as u128, 1u16);
        // D s_10_23: read-var Vn:u8
        let s_10_23: u8 = fn_state.Vn;
        // D s_10_24: cast zx s_10_23 -> bv
        let s_10_24: Bits = Bits::new(s_10_23 as u128, 4u16);
        // D s_10_25: cast reint s_10_22 -> u128
        let s_10_25: u128 = (s_10_22.value() as u128);
        // D s_10_26: size-of s_10_22
        let s_10_26: u16 = s_10_22.length();
        // D s_10_27: cast reint s_10_24 -> u128
        let s_10_27: u128 = (s_10_24.value() as u128);
        // D s_10_28: size-of s_10_24
        let s_10_28: u16 = s_10_24.length();
        // D s_10_29: lsl s_10_25 s_10_28
        let s_10_29: u128 = s_10_25 << s_10_28;
        // D s_10_30: or s_10_29 s_10_27
        let s_10_30: u128 = ((s_10_29) | (s_10_27));
        // D s_10_31: add s_10_26 s_10_28
        let s_10_31: u16 = (s_10_26 + s_10_28);
        // D s_10_32: create-bits s_10_30 s_10_31
        let s_10_32: Bits = Bits::new(s_10_30, s_10_31);
        // D s_10_33: cast reint s_10_32 -> u8
        let s_10_33: u8 = (s_10_32.value() as u8);
        // D s_10_34: cast zx s_10_33 -> bv
        let s_10_34: Bits = Bits::new(s_10_33 as u128, 5u16);
        // D s_10_35: cast zx s_10_34 -> i
        let s_10_35: i128 = (s_10_34.value() as i128);
        // D s_10_36: cast reint s_10_35 -> i64
        let s_10_36: i64 = (s_10_35 as i64);
        // D s_10_37: write-var n <= s_10_36
        fn_state.n = s_10_36;
        // D s_10_38: read-var M:u8
        let s_10_38: bool = fn_state.M;
        // D s_10_39: cast zx s_10_38 -> bv
        let s_10_39: Bits = Bits::new(s_10_38 as u128, 1u16);
        // D s_10_40: read-var Vm:u8
        let s_10_40: u8 = fn_state.Vm;
        // D s_10_41: cast zx s_10_40 -> bv
        let s_10_41: Bits = Bits::new(s_10_40 as u128, 4u16);
        // D s_10_42: cast reint s_10_39 -> u128
        let s_10_42: u128 = (s_10_39.value() as u128);
        // D s_10_43: size-of s_10_39
        let s_10_43: u16 = s_10_39.length();
        // D s_10_44: cast reint s_10_41 -> u128
        let s_10_44: u128 = (s_10_41.value() as u128);
        // D s_10_45: size-of s_10_41
        let s_10_45: u16 = s_10_41.length();
        // D s_10_46: lsl s_10_42 s_10_45
        let s_10_46: u128 = s_10_42 << s_10_45;
        // D s_10_47: or s_10_46 s_10_44
        let s_10_47: u128 = ((s_10_46) | (s_10_44));
        // D s_10_48: add s_10_43 s_10_45
        let s_10_48: u16 = (s_10_43 + s_10_45);
        // D s_10_49: create-bits s_10_47 s_10_48
        let s_10_49: Bits = Bits::new(s_10_47, s_10_48);
        // D s_10_50: cast reint s_10_49 -> u8
        let s_10_50: u8 = (s_10_49.value() as u8);
        // D s_10_51: cast zx s_10_50 -> bv
        let s_10_51: Bits = Bits::new(s_10_50 as u128, 5u16);
        // D s_10_52: cast zx s_10_51 -> i
        let s_10_52: i128 = (s_10_51.value() as i128);
        // D s_10_53: cast reint s_10_52 -> i64
        let s_10_53: i64 = (s_10_52 as i64);
        // D s_10_54: write-var m <= s_10_53
        fn_state.m = s_10_53;
        // D s_10_55: read-var Q:u8
        let s_10_55: bool = fn_state.Q;
        // D s_10_56: cast zx s_10_55 -> bv
        let s_10_56: Bits = Bits::new(s_10_55 as u128, 1u16);
        // C s_10_57: const #0u : u8
        let s_10_57: bool = false;
        // C s_10_58: cast zx s_10_57 -> bv
        let s_10_58: Bits = Bits::new(s_10_57 as u128, 1u16);
        // D s_10_59: cmp-eq s_10_56 s_10_58
        let s_10_59: bool = ((s_10_56) == (s_10_58));
        // N s_10_60: branch s_10_59 b13 b11
        if s_10_59 {
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
        // D s_11_1: write-var ga#350793 <= s_11_0
        fn_state.ga_350793 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var ga#350793:i64
        let s_12_0: i64 = fn_state.ga_350793;
        // D s_12_1: read-var esizeshadow#7356:i64
        let s_12_1: i64 = fn_state.esizeshadow_7356;
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // D s_12_3: cast reint s_12_2 -> i64
        let s_12_3: i64 = (s_12_2 as i64);
        // D s_12_4: read-var elementsshadow#7357:i64
        let s_12_4: i64 = fn_state.elementsshadow_7357;
        // D s_12_5: cast zx s_12_4 -> i
        let s_12_5: i128 = (i128::try_from(s_12_4).unwrap());
        // D s_12_6: cast zx s_12_0 -> i
        let s_12_6: i128 = (i128::try_from(s_12_0).unwrap());
        // C s_12_7: const #1u : u8
        let s_12_7: bool = true;
        // D s_12_8: read-var d:i64
        let s_12_8: i64 = fn_state.d;
        // D s_12_9: read-var m:i64
        let s_12_9: i64 = fn_state.m;
        // D s_12_10: read-var n:i64
        let s_12_10: i64 = fn_state.n;
        // D s_12_11: call execute_aarch32_instrs_VADD_f_Op_A_txt(s_12_7, s_12_8, s_12_5, s_12_3, s_12_9, s_12_10, s_12_6)
        let s_12_11: () = execute_aarch32_instrs_VADD_f_Op_A_txt(
            state,
            tracer,
            s_12_7,
            s_12_8,
            s_12_5,
            s_12_3,
            s_12_9,
            s_12_10,
            s_12_6,
        );
        // N s_12_12: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #1s : i64
        let s_13_0: i64 = 1;
        // D s_13_1: write-var ga#350793 <= s_13_0
        fn_state.ga_350793 = s_13_0;
        // N s_13_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #16s : i64
        let s_14_0: i64 = 16;
        // D s_14_1: write-var esize <= s_14_0
        fn_state.esize = s_14_0;
        // C s_14_2: const #4s : i64
        let s_14_2: i64 = 4;
        // D s_14_3: write-var elements <= s_14_2
        fn_state.elements = s_14_2;
        // N s_14_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: panic
        panic!("{:?}", ());
        // N s_15_1: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call InITBlock(s_16_0)
        let s_16_1: bool = InITBlock(state, tracer, s_16_0);
        // D s_16_2: write-var gs#306200 <= s_16_1
        fn_state.gs_306200 = s_16_1;
        // N s_16_3: jump b7
        return block_7(state, tracer, fn_state);
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
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #0s : i
        let s_18_0: i128 = 0;
        // D s_18_1: read-var Vd:u8
        let s_18_1: u8 = fn_state.Vd;
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
        // N s_18_22: branch s_18_21 b24 b19
        if s_18_21 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #0s : i
        let s_19_0: i128 = 0;
        // D s_19_1: read-var Vn:u8
        let s_19_1: u8 = fn_state.Vn;
        // D s_19_2: cast zx s_19_1 -> bv
        let s_19_2: Bits = Bits::new(s_19_1 as u128, 4u16);
        // C s_19_3: const #1u : u64
        let s_19_3: u64 = 1;
        // D s_19_4: bit-extract s_19_2 s_19_0 s_19_3
        let s_19_4: Bits = (Bits::new(
            ((s_19_2) >> (s_19_0)).value(),
            u16::try_from(s_19_3).unwrap(),
        ));
        // D s_19_5: cast reint s_19_4 -> u8
        let s_19_5: bool = ((s_19_4.value()) != 0);
        // C s_19_6: const #0s : i
        let s_19_6: i128 = 0;
        // C s_19_7: const #0u : u64
        let s_19_7: u64 = 0;
        // D s_19_8: cast zx s_19_5 -> u64
        let s_19_8: u64 = (s_19_5 as u64);
        // C s_19_9: const #1u : u64
        let s_19_9: u64 = 1;
        // D s_19_10: and s_19_8 s_19_9
        let s_19_10: u64 = ((s_19_8) & (s_19_9));
        // D s_19_11: cmp-eq s_19_10 s_19_9
        let s_19_11: bool = ((s_19_10) == (s_19_9));
        // D s_19_12: lsl s_19_8 s_19_6
        let s_19_12: u64 = s_19_8 << s_19_6;
        // D s_19_13: or s_19_7 s_19_12
        let s_19_13: u64 = ((s_19_7) | (s_19_12));
        // D s_19_14: cmpl s_19_12
        let s_19_14: u64 = !s_19_12;
        // D s_19_15: and s_19_7 s_19_14
        let s_19_15: u64 = ((s_19_7) & (s_19_14));
        // D s_19_16: select s_19_11 s_19_13 s_19_15
        let s_19_16: u64 = if s_19_11 { s_19_13 } else { s_19_15 };
        // D s_19_17: cast trunc s_19_16 -> u8
        let s_19_17: bool = ((s_19_16) != 0);
        // D s_19_18: cast zx s_19_17 -> bv
        let s_19_18: Bits = Bits::new(s_19_17 as u128, 1u16);
        // C s_19_19: const #1u : u8
        let s_19_19: bool = true;
        // C s_19_20: cast zx s_19_19 -> bv
        let s_19_20: Bits = Bits::new(s_19_19 as u128, 1u16);
        // D s_19_21: cmp-eq s_19_18 s_19_20
        let s_19_21: bool = ((s_19_18) == (s_19_20));
        // D s_19_22: write-var gs#306195 <= s_19_21
        fn_state.gs_306195 = s_19_21;
        // N s_19_23: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#306195:u8
        let s_20_0: bool = fn_state.gs_306195;
        // N s_20_1: branch s_20_0 b23 b21
        if s_20_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0s : i
        let s_21_0: i128 = 0;
        // D s_21_1: read-var Vm:u8
        let s_21_1: u8 = fn_state.Vm;
        // D s_21_2: cast zx s_21_1 -> bv
        let s_21_2: Bits = Bits::new(s_21_1 as u128, 4u16);
        // C s_21_3: const #1u : u64
        let s_21_3: u64 = 1;
        // D s_21_4: bit-extract s_21_2 s_21_0 s_21_3
        let s_21_4: Bits = (Bits::new(
            ((s_21_2) >> (s_21_0)).value(),
            u16::try_from(s_21_3).unwrap(),
        ));
        // D s_21_5: cast reint s_21_4 -> u8
        let s_21_5: bool = ((s_21_4.value()) != 0);
        // C s_21_6: const #0s : i
        let s_21_6: i128 = 0;
        // C s_21_7: const #0u : u64
        let s_21_7: u64 = 0;
        // D s_21_8: cast zx s_21_5 -> u64
        let s_21_8: u64 = (s_21_5 as u64);
        // C s_21_9: const #1u : u64
        let s_21_9: u64 = 1;
        // D s_21_10: and s_21_8 s_21_9
        let s_21_10: u64 = ((s_21_8) & (s_21_9));
        // D s_21_11: cmp-eq s_21_10 s_21_9
        let s_21_11: bool = ((s_21_10) == (s_21_9));
        // D s_21_12: lsl s_21_8 s_21_6
        let s_21_12: u64 = s_21_8 << s_21_6;
        // D s_21_13: or s_21_7 s_21_12
        let s_21_13: u64 = ((s_21_7) | (s_21_12));
        // D s_21_14: cmpl s_21_12
        let s_21_14: u64 = !s_21_12;
        // D s_21_15: and s_21_7 s_21_14
        let s_21_15: u64 = ((s_21_7) & (s_21_14));
        // D s_21_16: select s_21_11 s_21_13 s_21_15
        let s_21_16: u64 = if s_21_11 { s_21_13 } else { s_21_15 };
        // D s_21_17: cast trunc s_21_16 -> u8
        let s_21_17: bool = ((s_21_16) != 0);
        // D s_21_18: cast zx s_21_17 -> bv
        let s_21_18: Bits = Bits::new(s_21_17 as u128, 1u16);
        // C s_21_19: const #1u : u8
        let s_21_19: bool = true;
        // C s_21_20: cast zx s_21_19 -> bv
        let s_21_20: Bits = Bits::new(s_21_19 as u128, 1u16);
        // D s_21_21: cmp-eq s_21_18 s_21_20
        let s_21_21: bool = ((s_21_18) == (s_21_20));
        // D s_21_22: write-var gs#306198 <= s_21_21
        fn_state.gs_306198 = s_21_21;
        // N s_21_23: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#306198:u8
        let s_22_0: bool = fn_state.gs_306198;
        // D s_22_1: write-var gs#306199 <= s_22_0
        fn_state.gs_306199 = s_22_0;
        // N s_22_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #1u : u8
        let s_23_0: bool = true;
        // D s_23_1: write-var gs#306198 <= s_23_0
        fn_state.gs_306198 = s_23_0;
        // N s_23_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var gs#306195 <= s_24_0
        fn_state.gs_306195 = s_24_0;
        // N s_24_2: jump b20
        return block_20(state, tracer, fn_state);
    }
}
