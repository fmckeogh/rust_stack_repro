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
use execute_aarch32_instrs_VCGE_r_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VCGE_r_A2enc_A_txt<T: Tracer>(
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
        esizeshadow_7375: i64,
        esize: i64,
        gs_307065: bool,
        n: i64,
        gs_307066: bool,
        d: i64,
        elements: i64,
        elementsshadow_7376: i64,
        gs_307062: bool,
        ga_351596: i64,
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
        // N s_2_5: branch s_2_4 b13 b3
        if s_2_4 {
            return block_13(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#307066 <= s_3_0
        fn_state.gs_307066 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#307066:u8
        let s_4_0: bool = fn_state.gs_307066;
        // N s_4_1: branch s_4_0 b12 b5
        if s_4_0 {
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
        // C s_5_0: const #16s : i64
        let s_5_0: i64 = 16;
        // D s_5_1: write-var esize <= s_5_0
        fn_state.esize = s_5_0;
        // C s_5_2: const #2s : i64
        let s_5_2: i64 = 2;
        // D s_5_3: write-var elements <= s_5_2
        fn_state.elements = s_5_2;
        // D s_5_4: read-var sz:u8
        let s_5_4: bool = fn_state.sz;
        // D s_5_5: cast zx s_5_4 -> bv
        let s_5_5: Bits = Bits::new(s_5_4 as u128, 1u16);
        // C s_5_6: const #0u : u8
        let s_5_6: bool = false;
        // C s_5_7: cast zx s_5_6 -> bv
        let s_5_7: Bits = Bits::new(s_5_6 as u128, 1u16);
        // D s_5_8: cmp-eq s_5_5 s_5_7
        let s_5_8: bool = ((s_5_5) == (s_5_7));
        // D s_5_9: not s_5_8
        let s_5_9: bool = !s_5_8;
        // N s_5_10: branch s_5_9 b11 b6
        if s_5_9 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #32s : i64
        let s_6_0: i64 = 32;
        // D s_6_1: write-var esize <= s_6_0
        fn_state.esize = s_6_0;
        // C s_6_2: const #2s : i64
        let s_6_2: i64 = 2;
        // D s_6_3: write-var elements <= s_6_2
        fn_state.elements = s_6_2;
        // N s_6_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var esize:i64
        let s_7_0: i64 = fn_state.esize;
        // D s_7_1: write-var esizeshadow#7375 <= s_7_0
        fn_state.esizeshadow_7375 = s_7_0;
        // D s_7_2: read-var elements:i64
        let s_7_2: i64 = fn_state.elements;
        // D s_7_3: write-var elementsshadow#7376 <= s_7_2
        fn_state.elementsshadow_7376 = s_7_2;
        // D s_7_4: read-var D:u8
        let s_7_4: bool = fn_state.D;
        // D s_7_5: cast zx s_7_4 -> bv
        let s_7_5: Bits = Bits::new(s_7_4 as u128, 1u16);
        // D s_7_6: read-var Vd:u8
        let s_7_6: u8 = fn_state.Vd;
        // D s_7_7: cast zx s_7_6 -> bv
        let s_7_7: Bits = Bits::new(s_7_6 as u128, 4u16);
        // D s_7_8: cast reint s_7_5 -> u128
        let s_7_8: u128 = (s_7_5.value() as u128);
        // D s_7_9: size-of s_7_5
        let s_7_9: u16 = s_7_5.length();
        // D s_7_10: cast reint s_7_7 -> u128
        let s_7_10: u128 = (s_7_7.value() as u128);
        // D s_7_11: size-of s_7_7
        let s_7_11: u16 = s_7_7.length();
        // D s_7_12: lsl s_7_8 s_7_11
        let s_7_12: u128 = s_7_8 << s_7_11;
        // D s_7_13: or s_7_12 s_7_10
        let s_7_13: u128 = ((s_7_12) | (s_7_10));
        // D s_7_14: add s_7_9 s_7_11
        let s_7_14: u16 = (s_7_9 + s_7_11);
        // D s_7_15: create-bits s_7_13 s_7_14
        let s_7_15: Bits = Bits::new(s_7_13, s_7_14);
        // D s_7_16: cast reint s_7_15 -> u8
        let s_7_16: u8 = (s_7_15.value() as u8);
        // D s_7_17: cast zx s_7_16 -> bv
        let s_7_17: Bits = Bits::new(s_7_16 as u128, 5u16);
        // D s_7_18: cast zx s_7_17 -> i
        let s_7_18: i128 = (s_7_17.value() as i128);
        // D s_7_19: cast reint s_7_18 -> i64
        let s_7_19: i64 = (s_7_18 as i64);
        // D s_7_20: write-var d <= s_7_19
        fn_state.d = s_7_19;
        // D s_7_21: read-var N:u8
        let s_7_21: bool = fn_state.N;
        // D s_7_22: cast zx s_7_21 -> bv
        let s_7_22: Bits = Bits::new(s_7_21 as u128, 1u16);
        // D s_7_23: read-var Vn:u8
        let s_7_23: u8 = fn_state.Vn;
        // D s_7_24: cast zx s_7_23 -> bv
        let s_7_24: Bits = Bits::new(s_7_23 as u128, 4u16);
        // D s_7_25: cast reint s_7_22 -> u128
        let s_7_25: u128 = (s_7_22.value() as u128);
        // D s_7_26: size-of s_7_22
        let s_7_26: u16 = s_7_22.length();
        // D s_7_27: cast reint s_7_24 -> u128
        let s_7_27: u128 = (s_7_24.value() as u128);
        // D s_7_28: size-of s_7_24
        let s_7_28: u16 = s_7_24.length();
        // D s_7_29: lsl s_7_25 s_7_28
        let s_7_29: u128 = s_7_25 << s_7_28;
        // D s_7_30: or s_7_29 s_7_27
        let s_7_30: u128 = ((s_7_29) | (s_7_27));
        // D s_7_31: add s_7_26 s_7_28
        let s_7_31: u16 = (s_7_26 + s_7_28);
        // D s_7_32: create-bits s_7_30 s_7_31
        let s_7_32: Bits = Bits::new(s_7_30, s_7_31);
        // D s_7_33: cast reint s_7_32 -> u8
        let s_7_33: u8 = (s_7_32.value() as u8);
        // D s_7_34: cast zx s_7_33 -> bv
        let s_7_34: Bits = Bits::new(s_7_33 as u128, 5u16);
        // D s_7_35: cast zx s_7_34 -> i
        let s_7_35: i128 = (s_7_34.value() as i128);
        // D s_7_36: cast reint s_7_35 -> i64
        let s_7_36: i64 = (s_7_35 as i64);
        // D s_7_37: write-var n <= s_7_36
        fn_state.n = s_7_36;
        // D s_7_38: read-var M:u8
        let s_7_38: bool = fn_state.M;
        // D s_7_39: cast zx s_7_38 -> bv
        let s_7_39: Bits = Bits::new(s_7_38 as u128, 1u16);
        // D s_7_40: read-var Vm:u8
        let s_7_40: u8 = fn_state.Vm;
        // D s_7_41: cast zx s_7_40 -> bv
        let s_7_41: Bits = Bits::new(s_7_40 as u128, 4u16);
        // D s_7_42: cast reint s_7_39 -> u128
        let s_7_42: u128 = (s_7_39.value() as u128);
        // D s_7_43: size-of s_7_39
        let s_7_43: u16 = s_7_39.length();
        // D s_7_44: cast reint s_7_41 -> u128
        let s_7_44: u128 = (s_7_41.value() as u128);
        // D s_7_45: size-of s_7_41
        let s_7_45: u16 = s_7_41.length();
        // D s_7_46: lsl s_7_42 s_7_45
        let s_7_46: u128 = s_7_42 << s_7_45;
        // D s_7_47: or s_7_46 s_7_44
        let s_7_47: u128 = ((s_7_46) | (s_7_44));
        // D s_7_48: add s_7_43 s_7_45
        let s_7_48: u16 = (s_7_43 + s_7_45);
        // D s_7_49: create-bits s_7_47 s_7_48
        let s_7_49: Bits = Bits::new(s_7_47, s_7_48);
        // D s_7_50: cast reint s_7_49 -> u8
        let s_7_50: u8 = (s_7_49.value() as u8);
        // D s_7_51: cast zx s_7_50 -> bv
        let s_7_51: Bits = Bits::new(s_7_50 as u128, 5u16);
        // D s_7_52: cast zx s_7_51 -> i
        let s_7_52: i128 = (s_7_51.value() as i128);
        // D s_7_53: cast reint s_7_52 -> i64
        let s_7_53: i64 = (s_7_52 as i64);
        // D s_7_54: write-var m <= s_7_53
        fn_state.m = s_7_53;
        // D s_7_55: read-var Q:u8
        let s_7_55: bool = fn_state.Q;
        // D s_7_56: cast zx s_7_55 -> bv
        let s_7_56: Bits = Bits::new(s_7_55 as u128, 1u16);
        // C s_7_57: const #0u : u8
        let s_7_57: bool = false;
        // C s_7_58: cast zx s_7_57 -> bv
        let s_7_58: Bits = Bits::new(s_7_57 as u128, 1u16);
        // D s_7_59: cmp-eq s_7_56 s_7_58
        let s_7_59: bool = ((s_7_56) == (s_7_58));
        // N s_7_60: branch s_7_59 b10 b8
        if s_7_59 {
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
        // D s_8_1: write-var ga#351596 <= s_8_0
        fn_state.ga_351596 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var ga#351596:i64
        let s_9_0: i64 = fn_state.ga_351596;
        // D s_9_1: read-var esizeshadow#7375:i64
        let s_9_1: i64 = fn_state.esizeshadow_7375;
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (i128::try_from(s_9_1).unwrap());
        // D s_9_3: cast reint s_9_2 -> i64
        let s_9_3: i64 = (s_9_2 as i64);
        // D s_9_4: read-var elementsshadow#7376:i64
        let s_9_4: i64 = fn_state.elementsshadow_7376;
        // D s_9_5: cast zx s_9_4 -> i
        let s_9_5: i128 = (i128::try_from(s_9_4).unwrap());
        // D s_9_6: read-var d:i64
        let s_9_6: i64 = fn_state.d;
        // D s_9_7: read-var m:i64
        let s_9_7: i64 = fn_state.m;
        // D s_9_8: read-var n:i64
        let s_9_8: i64 = fn_state.n;
        // C s_9_9: const #2u : u32
        let s_9_9: u32 = 2;
        // D s_9_10: call execute_aarch32_instrs_VCGE_r_Op_A_txt(s_9_6, s_9_5, s_9_3, s_9_7, s_9_8, s_9_0, s_9_9)
        let s_9_10: () = execute_aarch32_instrs_VCGE_r_Op_A_txt(
            state,
            tracer,
            s_9_6,
            s_9_5,
            s_9_3,
            s_9_7,
            s_9_8,
            s_9_0,
            s_9_9,
        );
        // N s_9_11: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #1s : i64
        let s_10_0: i64 = 1;
        // D s_10_1: write-var ga#351596 <= s_10_0
        fn_state.ga_351596 = s_10_0;
        // N s_10_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #16s : i64
        let s_11_0: i64 = 16;
        // D s_11_1: write-var esize <= s_11_0
        fn_state.esize = s_11_0;
        // C s_11_2: const #4s : i64
        let s_11_2: i64 = 4;
        // D s_11_3: write-var elements <= s_11_2
        fn_state.elements = s_11_2;
        // N s_11_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: panic
        panic!("{:?}", ());
        // N s_12_1: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0s : i
        let s_13_0: i128 = 0;
        // D s_13_1: read-var Vd:u8
        let s_13_1: u8 = fn_state.Vd;
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
        // N s_13_22: branch s_13_21 b19 b14
        if s_13_21 {
            return block_19(state, tracer, fn_state);
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
        // D s_14_1: read-var Vn:u8
        let s_14_1: u8 = fn_state.Vn;
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
        // D s_14_22: write-var gs#307062 <= s_14_21
        fn_state.gs_307062 = s_14_21;
        // N s_14_23: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#307062:u8
        let s_15_0: bool = fn_state.gs_307062;
        // N s_15_1: branch s_15_0 b18 b16
        if s_15_0 {
            return block_18(state, tracer, fn_state);
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
        // D s_16_1: read-var Vm:u8
        let s_16_1: u8 = fn_state.Vm;
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
        // D s_16_22: write-var gs#307065 <= s_16_21
        fn_state.gs_307065 = s_16_21;
        // N s_16_23: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#307065:u8
        let s_17_0: bool = fn_state.gs_307065;
        // D s_17_1: write-var gs#307066 <= s_17_0
        fn_state.gs_307066 = s_17_0;
        // N s_17_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#307065 <= s_18_0
        fn_state.gs_307065 = s_18_0;
        // N s_18_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #1u : u8
        let s_19_0: bool = true;
        // D s_19_1: write-var gs#307062 <= s_19_0
        fn_state.gs_307062 = s_19_0;
        // N s_19_2: jump b15
        return block_15(state, tracer, fn_state);
    }
}
