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
use execute_aarch32_instrs_VBIF_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VBIF_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    op: u8,
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
        gs_306671: bool,
        n: i64,
        gs_306672: bool,
        d: i64,
        operation: u32,
        gs_306668: bool,
        ga_351260: i64,
        D: bool,
        op: u8,
        Vn: u8,
        Vd: u8,
        N: bool,
        Q: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        D,
        op,
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
        // C s_2_0: const #0u : u32
        let s_2_0: u32 = 0;
        // D s_2_1: write-var operation <= s_2_0
        fn_state.operation = s_2_0;
        // D s_2_2: read-var Q:u8
        let s_2_2: bool = fn_state.Q;
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // C s_2_4: const #1u : u8
        let s_2_4: bool = true;
        // C s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 1u16);
        // D s_2_6: cmp-eq s_2_3 s_2_5
        let s_2_6: bool = ((s_2_3) == (s_2_5));
        // N s_2_7: branch s_2_6 b21 b3
        if s_2_6 {
            return block_21(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#306672 <= s_3_0
        fn_state.gs_306672 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#306672:u8
        let s_4_0: bool = fn_state.gs_306672;
        // N s_4_1: branch s_4_0 b20 b5
        if s_4_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var op:u8
        let s_5_0: u8 = fn_state.op;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // C s_5_2: const #0u : u8
        let s_5_2: u8 = 0;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 2u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // N s_5_5: branch s_5_4 b19 b6
        if s_5_4 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var op:u8
        let s_6_0: u8 = fn_state.op;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 2u16);
        // C s_6_2: const #1u : u8
        let s_6_2: u8 = 1;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 2u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // N s_6_5: branch s_6_4 b18 b7
        if s_6_4 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var op:u8
        let s_8_0: u8 = fn_state.op;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 2u16);
        // C s_8_2: const #2u : u8
        let s_8_2: u8 = 2;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 2u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // N s_8_5: branch s_8_4 b17 b9
        if s_8_4 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var op:u8
        let s_10_0: u8 = fn_state.op;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 2u16);
        // C s_10_2: const #3u : u8
        let s_10_2: u8 = 3;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 2u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // N s_10_5: branch s_10_4 b16 b11
        if s_10_4 {
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
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var D:u8
        let s_12_0: bool = fn_state.D;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 1u16);
        // D s_12_2: read-var Vd:u8
        let s_12_2: u8 = fn_state.Vd;
        // D s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 4u16);
        // D s_12_4: cast reint s_12_1 -> u128
        let s_12_4: u128 = (s_12_1.value() as u128);
        // D s_12_5: size-of s_12_1
        let s_12_5: u16 = s_12_1.length();
        // D s_12_6: cast reint s_12_3 -> u128
        let s_12_6: u128 = (s_12_3.value() as u128);
        // D s_12_7: size-of s_12_3
        let s_12_7: u16 = s_12_3.length();
        // D s_12_8: lsl s_12_4 s_12_7
        let s_12_8: u128 = s_12_4 << s_12_7;
        // D s_12_9: or s_12_8 s_12_6
        let s_12_9: u128 = ((s_12_8) | (s_12_6));
        // D s_12_10: add s_12_5 s_12_7
        let s_12_10: u16 = (s_12_5 + s_12_7);
        // D s_12_11: create-bits s_12_9 s_12_10
        let s_12_11: Bits = Bits::new(s_12_9, s_12_10);
        // D s_12_12: cast reint s_12_11 -> u8
        let s_12_12: u8 = (s_12_11.value() as u8);
        // D s_12_13: cast zx s_12_12 -> bv
        let s_12_13: Bits = Bits::new(s_12_12 as u128, 5u16);
        // D s_12_14: cast zx s_12_13 -> i
        let s_12_14: i128 = (s_12_13.value() as i128);
        // D s_12_15: cast reint s_12_14 -> i64
        let s_12_15: i64 = (s_12_14 as i64);
        // D s_12_16: write-var d <= s_12_15
        fn_state.d = s_12_15;
        // D s_12_17: read-var N:u8
        let s_12_17: bool = fn_state.N;
        // D s_12_18: cast zx s_12_17 -> bv
        let s_12_18: Bits = Bits::new(s_12_17 as u128, 1u16);
        // D s_12_19: read-var Vn:u8
        let s_12_19: u8 = fn_state.Vn;
        // D s_12_20: cast zx s_12_19 -> bv
        let s_12_20: Bits = Bits::new(s_12_19 as u128, 4u16);
        // D s_12_21: cast reint s_12_18 -> u128
        let s_12_21: u128 = (s_12_18.value() as u128);
        // D s_12_22: size-of s_12_18
        let s_12_22: u16 = s_12_18.length();
        // D s_12_23: cast reint s_12_20 -> u128
        let s_12_23: u128 = (s_12_20.value() as u128);
        // D s_12_24: size-of s_12_20
        let s_12_24: u16 = s_12_20.length();
        // D s_12_25: lsl s_12_21 s_12_24
        let s_12_25: u128 = s_12_21 << s_12_24;
        // D s_12_26: or s_12_25 s_12_23
        let s_12_26: u128 = ((s_12_25) | (s_12_23));
        // D s_12_27: add s_12_22 s_12_24
        let s_12_27: u16 = (s_12_22 + s_12_24);
        // D s_12_28: create-bits s_12_26 s_12_27
        let s_12_28: Bits = Bits::new(s_12_26, s_12_27);
        // D s_12_29: cast reint s_12_28 -> u8
        let s_12_29: u8 = (s_12_28.value() as u8);
        // D s_12_30: cast zx s_12_29 -> bv
        let s_12_30: Bits = Bits::new(s_12_29 as u128, 5u16);
        // D s_12_31: cast zx s_12_30 -> i
        let s_12_31: i128 = (s_12_30.value() as i128);
        // D s_12_32: cast reint s_12_31 -> i64
        let s_12_32: i64 = (s_12_31 as i64);
        // D s_12_33: write-var n <= s_12_32
        fn_state.n = s_12_32;
        // D s_12_34: read-var M:u8
        let s_12_34: bool = fn_state.M;
        // D s_12_35: cast zx s_12_34 -> bv
        let s_12_35: Bits = Bits::new(s_12_34 as u128, 1u16);
        // D s_12_36: read-var Vm:u8
        let s_12_36: u8 = fn_state.Vm;
        // D s_12_37: cast zx s_12_36 -> bv
        let s_12_37: Bits = Bits::new(s_12_36 as u128, 4u16);
        // D s_12_38: cast reint s_12_35 -> u128
        let s_12_38: u128 = (s_12_35.value() as u128);
        // D s_12_39: size-of s_12_35
        let s_12_39: u16 = s_12_35.length();
        // D s_12_40: cast reint s_12_37 -> u128
        let s_12_40: u128 = (s_12_37.value() as u128);
        // D s_12_41: size-of s_12_37
        let s_12_41: u16 = s_12_37.length();
        // D s_12_42: lsl s_12_38 s_12_41
        let s_12_42: u128 = s_12_38 << s_12_41;
        // D s_12_43: or s_12_42 s_12_40
        let s_12_43: u128 = ((s_12_42) | (s_12_40));
        // D s_12_44: add s_12_39 s_12_41
        let s_12_44: u16 = (s_12_39 + s_12_41);
        // D s_12_45: create-bits s_12_43 s_12_44
        let s_12_45: Bits = Bits::new(s_12_43, s_12_44);
        // D s_12_46: cast reint s_12_45 -> u8
        let s_12_46: u8 = (s_12_45.value() as u8);
        // D s_12_47: cast zx s_12_46 -> bv
        let s_12_47: Bits = Bits::new(s_12_46 as u128, 5u16);
        // D s_12_48: cast zx s_12_47 -> i
        let s_12_48: i128 = (s_12_47.value() as i128);
        // D s_12_49: cast reint s_12_48 -> i64
        let s_12_49: i64 = (s_12_48 as i64);
        // D s_12_50: write-var m <= s_12_49
        fn_state.m = s_12_49;
        // D s_12_51: read-var Q:u8
        let s_12_51: bool = fn_state.Q;
        // D s_12_52: cast zx s_12_51 -> bv
        let s_12_52: Bits = Bits::new(s_12_51 as u128, 1u16);
        // C s_12_53: const #0u : u8
        let s_12_53: bool = false;
        // C s_12_54: cast zx s_12_53 -> bv
        let s_12_54: Bits = Bits::new(s_12_53 as u128, 1u16);
        // D s_12_55: cmp-eq s_12_52 s_12_54
        let s_12_55: bool = ((s_12_52) == (s_12_54));
        // N s_12_56: branch s_12_55 b15 b13
        if s_12_55 {
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
        // C s_13_0: const #2s : i64
        let s_13_0: i64 = 2;
        // D s_13_1: write-var ga#351260 <= s_13_0
        fn_state.ga_351260 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var ga#351260:i64
        let s_14_0: i64 = fn_state.ga_351260;
        // D s_14_1: read-var d:i64
        let s_14_1: i64 = fn_state.d;
        // D s_14_2: read-var m:i64
        let s_14_2: i64 = fn_state.m;
        // D s_14_3: read-var n:i64
        let s_14_3: i64 = fn_state.n;
        // D s_14_4: read-var operation:u32
        let s_14_4: u32 = fn_state.operation;
        // D s_14_5: call execute_aarch32_instrs_VBIF_Op_A_txt(s_14_1, s_14_2, s_14_3, s_14_4, s_14_0)
        let s_14_5: () = execute_aarch32_instrs_VBIF_Op_A_txt(
            state,
            tracer,
            s_14_1,
            s_14_2,
            s_14_3,
            s_14_4,
            s_14_0,
        );
        // N s_14_6: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #1s : i64
        let s_15_0: i64 = 1;
        // D s_15_1: write-var ga#351260 <= s_15_0
        fn_state.ga_351260 = s_15_0;
        // N s_15_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0u : u32
        let s_16_0: u32 = 0;
        // D s_16_1: write-var operation <= s_16_0
        fn_state.operation = s_16_0;
        // N s_16_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1u : u32
        let s_17_0: u32 = 1;
        // D s_17_1: write-var operation <= s_17_0
        fn_state.operation = s_17_0;
        // N s_17_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #2u : u32
        let s_18_0: u32 = 2;
        // D s_18_1: write-var operation <= s_18_0
        fn_state.operation = s_18_0;
        // N s_18_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: panic
        panic!("{:?}", ());
        // N s_19_1: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: panic
        panic!("{:?}", ());
        // N s_20_1: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0s : i
        let s_21_0: i128 = 0;
        // D s_21_1: read-var Vd:u8
        let s_21_1: u8 = fn_state.Vd;
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
        // N s_21_22: branch s_21_21 b27 b22
        if s_21_21 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #0s : i
        let s_22_0: i128 = 0;
        // D s_22_1: read-var Vn:u8
        let s_22_1: u8 = fn_state.Vn;
        // D s_22_2: cast zx s_22_1 -> bv
        let s_22_2: Bits = Bits::new(s_22_1 as u128, 4u16);
        // C s_22_3: const #1u : u64
        let s_22_3: u64 = 1;
        // D s_22_4: bit-extract s_22_2 s_22_0 s_22_3
        let s_22_4: Bits = (Bits::new(
            ((s_22_2) >> (s_22_0)).value(),
            u16::try_from(s_22_3).unwrap(),
        ));
        // D s_22_5: cast reint s_22_4 -> u8
        let s_22_5: bool = ((s_22_4.value()) != 0);
        // C s_22_6: const #0s : i
        let s_22_6: i128 = 0;
        // C s_22_7: const #0u : u64
        let s_22_7: u64 = 0;
        // D s_22_8: cast zx s_22_5 -> u64
        let s_22_8: u64 = (s_22_5 as u64);
        // C s_22_9: const #1u : u64
        let s_22_9: u64 = 1;
        // D s_22_10: and s_22_8 s_22_9
        let s_22_10: u64 = ((s_22_8) & (s_22_9));
        // D s_22_11: cmp-eq s_22_10 s_22_9
        let s_22_11: bool = ((s_22_10) == (s_22_9));
        // D s_22_12: lsl s_22_8 s_22_6
        let s_22_12: u64 = s_22_8 << s_22_6;
        // D s_22_13: or s_22_7 s_22_12
        let s_22_13: u64 = ((s_22_7) | (s_22_12));
        // D s_22_14: cmpl s_22_12
        let s_22_14: u64 = !s_22_12;
        // D s_22_15: and s_22_7 s_22_14
        let s_22_15: u64 = ((s_22_7) & (s_22_14));
        // D s_22_16: select s_22_11 s_22_13 s_22_15
        let s_22_16: u64 = if s_22_11 { s_22_13 } else { s_22_15 };
        // D s_22_17: cast trunc s_22_16 -> u8
        let s_22_17: bool = ((s_22_16) != 0);
        // D s_22_18: cast zx s_22_17 -> bv
        let s_22_18: Bits = Bits::new(s_22_17 as u128, 1u16);
        // C s_22_19: const #1u : u8
        let s_22_19: bool = true;
        // C s_22_20: cast zx s_22_19 -> bv
        let s_22_20: Bits = Bits::new(s_22_19 as u128, 1u16);
        // D s_22_21: cmp-eq s_22_18 s_22_20
        let s_22_21: bool = ((s_22_18) == (s_22_20));
        // D s_22_22: write-var gs#306668 <= s_22_21
        fn_state.gs_306668 = s_22_21;
        // N s_22_23: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#306668:u8
        let s_23_0: bool = fn_state.gs_306668;
        // N s_23_1: branch s_23_0 b26 b24
        if s_23_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #0s : i
        let s_24_0: i128 = 0;
        // D s_24_1: read-var Vm:u8
        let s_24_1: u8 = fn_state.Vm;
        // D s_24_2: cast zx s_24_1 -> bv
        let s_24_2: Bits = Bits::new(s_24_1 as u128, 4u16);
        // C s_24_3: const #1u : u64
        let s_24_3: u64 = 1;
        // D s_24_4: bit-extract s_24_2 s_24_0 s_24_3
        let s_24_4: Bits = (Bits::new(
            ((s_24_2) >> (s_24_0)).value(),
            u16::try_from(s_24_3).unwrap(),
        ));
        // D s_24_5: cast reint s_24_4 -> u8
        let s_24_5: bool = ((s_24_4.value()) != 0);
        // C s_24_6: const #0s : i
        let s_24_6: i128 = 0;
        // C s_24_7: const #0u : u64
        let s_24_7: u64 = 0;
        // D s_24_8: cast zx s_24_5 -> u64
        let s_24_8: u64 = (s_24_5 as u64);
        // C s_24_9: const #1u : u64
        let s_24_9: u64 = 1;
        // D s_24_10: and s_24_8 s_24_9
        let s_24_10: u64 = ((s_24_8) & (s_24_9));
        // D s_24_11: cmp-eq s_24_10 s_24_9
        let s_24_11: bool = ((s_24_10) == (s_24_9));
        // D s_24_12: lsl s_24_8 s_24_6
        let s_24_12: u64 = s_24_8 << s_24_6;
        // D s_24_13: or s_24_7 s_24_12
        let s_24_13: u64 = ((s_24_7) | (s_24_12));
        // D s_24_14: cmpl s_24_12
        let s_24_14: u64 = !s_24_12;
        // D s_24_15: and s_24_7 s_24_14
        let s_24_15: u64 = ((s_24_7) & (s_24_14));
        // D s_24_16: select s_24_11 s_24_13 s_24_15
        let s_24_16: u64 = if s_24_11 { s_24_13 } else { s_24_15 };
        // D s_24_17: cast trunc s_24_16 -> u8
        let s_24_17: bool = ((s_24_16) != 0);
        // D s_24_18: cast zx s_24_17 -> bv
        let s_24_18: Bits = Bits::new(s_24_17 as u128, 1u16);
        // C s_24_19: const #1u : u8
        let s_24_19: bool = true;
        // C s_24_20: cast zx s_24_19 -> bv
        let s_24_20: Bits = Bits::new(s_24_19 as u128, 1u16);
        // D s_24_21: cmp-eq s_24_18 s_24_20
        let s_24_21: bool = ((s_24_18) == (s_24_20));
        // D s_24_22: write-var gs#306671 <= s_24_21
        fn_state.gs_306671 = s_24_21;
        // N s_24_23: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#306671:u8
        let s_25_0: bool = fn_state.gs_306671;
        // D s_25_1: write-var gs#306672 <= s_25_0
        fn_state.gs_306672 = s_25_0;
        // N s_25_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // D s_26_1: write-var gs#306671 <= s_26_0
        fn_state.gs_306671 = s_26_0;
        // N s_26_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // D s_27_1: write-var gs#306668 <= s_27_0
        fn_state.gs_306668 = s_27_0;
        // N s_27_2: jump b23
        return block_23(state, tracer, fn_state);
    }
}
