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
use HaveDGHExt::*;
use HaveSelfHostedTrace::*;
use execute_aarch64_instrs_system_hints::*;
use EndOfInstruction::*;
use SetBTypeCompatible::*;
use IsFeatureImplemented::*;
use BTypeCompatible_BTI::*;
use HaveStatisticalProfiling::*;
use HaveRASExt::*;
use HaveFeatCLRBHB::*;
use common::*;
pub fn decode_yield_aarch64_instrs_system_hints<T: Tracer>(
    state: &mut State,
    tracer: &T,
    op2: u8,
    CRm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_145736: bool,
        b__27: u8,
        op: u32,
        ga_250880: u8,
        op2: u8,
        CRm: u8,
    }
    let fn_state = FunctionState {
        op2,
        CRm,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var CRm:u8
        let s_0_0: u8 = fn_state.CRm;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 4u16);
        // D s_0_2: read-var op2:u8
        let s_0_2: u8 = fn_state.op2;
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 3u16);
        // D s_0_4: cast reint s_0_1 -> u128
        let s_0_4: u128 = (s_0_1.value() as u128);
        // D s_0_5: size-of s_0_1
        let s_0_5: u16 = s_0_1.length();
        // D s_0_6: cast reint s_0_3 -> u128
        let s_0_6: u128 = (s_0_3.value() as u128);
        // D s_0_7: size-of s_0_3
        let s_0_7: u16 = s_0_3.length();
        // D s_0_8: lsl s_0_4 s_0_7
        let s_0_8: u128 = s_0_4 << s_0_7;
        // D s_0_9: or s_0_8 s_0_6
        let s_0_9: u128 = ((s_0_8) | (s_0_6));
        // D s_0_10: add s_0_5 s_0_7
        let s_0_10: u16 = (s_0_5 + s_0_7);
        // D s_0_11: create-bits s_0_9 s_0_10
        let s_0_11: Bits = Bits::new(s_0_9, s_0_10);
        // D s_0_12: cast reint s_0_11 -> u8
        let s_0_12: u8 = (s_0_11.value() as u8);
        // D s_0_13: write-var ga#250880 <= s_0_12
        fn_state.ga_250880 = s_0_12;
        // D s_0_14: read-var ga#250880:u8
        let s_0_14: u8 = fn_state.ga_250880;
        // D s_0_15: cast zx s_0_14 -> bv
        let s_0_15: Bits = Bits::new(s_0_14 as u128, 7u16);
        // C s_0_16: const #0u : u8
        let s_0_16: u8 = 0;
        // C s_0_17: cast zx s_0_16 -> bv
        let s_0_17: Bits = Bits::new(s_0_16 as u128, 7u16);
        // D s_0_18: cmp-eq s_0_15 s_0_17
        let s_0_18: bool = ((s_0_15) == (s_0_17));
        // D s_0_19: not s_0_18
        let s_0_19: bool = !s_0_18;
        // N s_0_20: branch s_0_19 b3 b1
        if s_0_19 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u32
        let s_1_0: u32 = 0;
        // D s_1_1: write-var op <= s_1_0
        fn_state.op = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var op:u32
        let s_2_0: u32 = fn_state.op;
        // D s_2_1: call execute_aarch64_instrs_system_hints(s_2_0)
        let s_2_1: () = execute_aarch64_instrs_system_hints(state, tracer, s_2_0);
        // N s_2_2: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var ga#250880:u8
        let s_3_0: u8 = fn_state.ga_250880;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 7u16);
        // C s_3_2: const #1u : u8
        let s_3_2: u8 = 1;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 7u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // D s_3_5: not s_3_4
        let s_3_5: bool = !s_3_4;
        // N s_3_6: branch s_3_5 b5 b4
        if s_3_5 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #1u : u32
        let s_4_0: u32 = 1;
        // D s_4_1: write-var op <= s_4_0
        fn_state.op = s_4_0;
        // N s_4_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var ga#250880:u8
        let s_5_0: u8 = fn_state.ga_250880;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 7u16);
        // C s_5_2: const #2u : u8
        let s_5_2: u8 = 2;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 7u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // D s_5_5: not s_5_4
        let s_5_5: bool = !s_5_4;
        // N s_5_6: branch s_5_5 b7 b6
        if s_5_5 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #2u : u32
        let s_6_0: u32 = 2;
        // D s_6_1: write-var op <= s_6_0
        fn_state.op = s_6_0;
        // N s_6_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var ga#250880:u8
        let s_7_0: u8 = fn_state.ga_250880;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 7u16);
        // C s_7_2: const #3u : u8
        let s_7_2: u8 = 3;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 7u16);
        // D s_7_4: cmp-eq s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) == (s_7_3));
        // D s_7_5: not s_7_4
        let s_7_5: bool = !s_7_4;
        // N s_7_6: branch s_7_5 b9 b8
        if s_7_5 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #3u : u32
        let s_8_0: u32 = 3;
        // D s_8_1: write-var op <= s_8_0
        fn_state.op = s_8_0;
        // N s_8_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var ga#250880:u8
        let s_9_0: u8 = fn_state.ga_250880;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 7u16);
        // C s_9_2: const #4u : u8
        let s_9_2: u8 = 4;
        // C s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 7u16);
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
        // C s_10_0: const #4u : u32
        let s_10_0: u32 = 4;
        // D s_10_1: write-var op <= s_10_0
        fn_state.op = s_10_0;
        // N s_10_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var ga#250880:u8
        let s_11_0: u8 = fn_state.ga_250880;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 7u16);
        // C s_11_2: const #5u : u8
        let s_11_2: u8 = 5;
        // C s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 7u16);
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
        // C s_12_0: const #5u : u32
        let s_12_0: u32 = 5;
        // D s_12_1: write-var op <= s_12_0
        fn_state.op = s_12_0;
        // N s_12_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var ga#250880:u8
        let s_13_0: u8 = fn_state.ga_250880;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 7u16);
        // C s_13_2: const #6u : u8
        let s_13_2: u8 = 6;
        // C s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 7u16);
        // D s_13_4: cmp-eq s_13_1 s_13_3
        let s_13_4: bool = ((s_13_1) == (s_13_3));
        // D s_13_5: not s_13_4
        let s_13_5: bool = !s_13_4;
        // N s_13_6: branch s_13_5 b18 b14
        if s_13_5 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call HaveDGHExt(s_14_0)
        let s_14_1: bool = HaveDGHExt(state, tracer, s_14_0);
        // S s_14_2: not s_14_1
        let s_14_2: bool = !s_14_1;
        // N s_14_3: branch s_14_2 b17 b15
        if s_14_2 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #6u : u32
        let s_16_0: u32 = 6;
        // D s_16_1: write-var op <= s_16_0
        fn_state.op = s_16_0;
        // N s_16_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call EndOfInstruction(s_17_0)
        let s_17_1: () = EndOfInstruction(state, tracer, s_17_0);
        // N s_17_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var ga#250880:u8
        let s_18_0: u8 = fn_state.ga_250880;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 7u16);
        // C s_18_2: const #7u : u8
        let s_18_2: u8 = 7;
        // C s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 7u16);
        // D s_18_4: cmp-eq s_18_1 s_18_3
        let s_18_4: bool = ((s_18_1) == (s_18_3));
        // D s_18_5: not s_18_4
        let s_18_5: bool = !s_18_4;
        // N s_18_6: branch s_18_5 b20 b19
        if s_18_5 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
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
        // D s_20_0: read-var ga#250880:u8
        let s_20_0: u8 = fn_state.ga_250880;
        // C s_20_1: const #3s : i
        let s_20_1: i128 = 3;
        // D s_20_2: cast zx s_20_0 -> bv
        let s_20_2: Bits = Bits::new(s_20_0 as u128, 7u16);
        // C s_20_3: const #1s : i64
        let s_20_3: i64 = 1;
        // C s_20_4: cast zx s_20_3 -> i
        let s_20_4: i128 = (i128::try_from(s_20_3).unwrap());
        // C s_20_5: const #3s : i
        let s_20_5: i128 = 3;
        // C s_20_6: add s_20_5 s_20_4
        let s_20_6: i128 = (s_20_5 + s_20_4);
        // D s_20_7: bit-extract s_20_2 s_20_1 s_20_6
        let s_20_7: Bits = (Bits::new(
            ((s_20_2) >> (s_20_1)).value(),
            u16::try_from(s_20_6).unwrap(),
        ));
        // D s_20_8: cast reint s_20_7 -> u8
        let s_20_8: u8 = (s_20_7.value() as u8);
        // D s_20_9: cast zx s_20_8 -> bv
        let s_20_9: Bits = Bits::new(s_20_8 as u128, 4u16);
        // C s_20_10: const #1u : u8
        let s_20_10: u8 = 1;
        // C s_20_11: cast zx s_20_10 -> bv
        let s_20_11: Bits = Bits::new(s_20_10 as u128, 4u16);
        // D s_20_12: cmp-eq s_20_9 s_20_11
        let s_20_12: bool = ((s_20_9) == (s_20_11));
        // D s_20_13: not s_20_12
        let s_20_13: bool = !s_20_12;
        // N s_20_14: branch s_20_13 b31 b21
        if s_20_13 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var op2:u8
        let s_21_0: u8 = fn_state.op2;
        // D s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 3u16);
        // C s_21_2: const #0u : u8
        let s_21_2: u8 = 0;
        // C s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 3u16);
        // D s_21_4: cmp-eq s_21_1 s_21_3
        let s_21_4: bool = ((s_21_1) == (s_21_3));
        // D s_21_5: not s_21_4
        let s_21_5: bool = !s_21_4;
        // N s_21_6: branch s_21_5 b23 b22
        if s_21_5 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
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
        // D s_23_0: read-var op2:u8
        let s_23_0: u8 = fn_state.op2;
        // D s_23_1: cast zx s_23_0 -> bv
        let s_23_1: Bits = Bits::new(s_23_0 as u128, 3u16);
        // C s_23_2: const #2u : u8
        let s_23_2: u8 = 2;
        // C s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 3u16);
        // D s_23_4: cmp-eq s_23_1 s_23_3
        let s_23_4: bool = ((s_23_1) == (s_23_3));
        // D s_23_5: not s_23_4
        let s_23_5: bool = !s_23_4;
        // N s_23_6: branch s_23_5 b25 b24
        if s_23_5 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_24_0: panic
        panic!("{:?}", ());
        // N s_24_1: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var op2:u8
        let s_25_0: u8 = fn_state.op2;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 3u16);
        // C s_25_2: const #4u : u8
        let s_25_2: u8 = 4;
        // C s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 3u16);
        // D s_25_4: cmp-eq s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) == (s_25_3));
        // D s_25_5: not s_25_4
        let s_25_5: bool = !s_25_4;
        // N s_25_6: branch s_25_5 b27 b26
        if s_25_5 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_26_0: panic
        panic!("{:?}", ());
        // N s_26_1: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var op2:u8
        let s_27_0: u8 = fn_state.op2;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 3u16);
        // C s_27_2: const #6u : u8
        let s_27_2: u8 = 6;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 3u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: not s_27_4
        let s_27_5: bool = !s_27_4;
        // N s_27_6: branch s_27_5 b29 b28
        if s_27_5 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_28_0: panic
        panic!("{:?}", ());
        // N s_28_1: return
        return;
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #() : ()
        let s_29_0: () = ();
        // S s_29_1: call EndOfInstruction(s_29_0)
        let s_29_1: () = EndOfInstruction(state, tracer, s_29_0);
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_30_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var ga#250880:u8
        let s_31_0: u8 = fn_state.ga_250880;
        // D s_31_1: cast zx s_31_0 -> bv
        let s_31_1: Bits = Bits::new(s_31_0 as u128, 7u16);
        // C s_31_2: const #16u : u8
        let s_31_2: u8 = 16;
        // C s_31_3: cast zx s_31_2 -> bv
        let s_31_3: Bits = Bits::new(s_31_2 as u128, 7u16);
        // D s_31_4: cmp-eq s_31_1 s_31_3
        let s_31_4: bool = ((s_31_1) == (s_31_3));
        // D s_31_5: not s_31_4
        let s_31_5: bool = !s_31_4;
        // N s_31_6: branch s_31_5 b36 b32
        if s_31_5 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #() : ()
        let s_32_0: () = ();
        // S s_32_1: call HaveRASExt(s_32_0)
        let s_32_1: bool = HaveRASExt(state, tracer, s_32_0);
        // S s_32_2: not s_32_1
        let s_32_2: bool = !s_32_1;
        // N s_32_3: branch s_32_2 b35 b33
        if s_32_2 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_33_0: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #7u : u32
        let s_34_0: u32 = 7;
        // D s_34_1: write-var op <= s_34_0
        fn_state.op = s_34_0;
        // N s_34_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #() : ()
        let s_35_0: () = ();
        // S s_35_1: call EndOfInstruction(s_35_0)
        let s_35_1: () = EndOfInstruction(state, tracer, s_35_0);
        // N s_35_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var ga#250880:u8
        let s_36_0: u8 = fn_state.ga_250880;
        // D s_36_1: cast zx s_36_0 -> bv
        let s_36_1: Bits = Bits::new(s_36_0 as u128, 7u16);
        // C s_36_2: const #17u : u8
        let s_36_2: u8 = 17;
        // C s_36_3: cast zx s_36_2 -> bv
        let s_36_3: Bits = Bits::new(s_36_2 as u128, 7u16);
        // D s_36_4: cmp-eq s_36_1 s_36_3
        let s_36_4: bool = ((s_36_1) == (s_36_3));
        // D s_36_5: not s_36_4
        let s_36_5: bool = !s_36_4;
        // N s_36_6: branch s_36_5 b41 b37
        if s_36_5 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #() : ()
        let s_37_0: () = ();
        // S s_37_1: call HaveStatisticalProfiling(s_37_0)
        let s_37_1: bool = HaveStatisticalProfiling(state, tracer, s_37_0);
        // S s_37_2: not s_37_1
        let s_37_2: bool = !s_37_1;
        // N s_37_3: branch s_37_2 b40 b38
        if s_37_2 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_38_0: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #8u : u32
        let s_39_0: u32 = 8;
        // D s_39_1: write-var op <= s_39_0
        fn_state.op = s_39_0;
        // N s_39_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #() : ()
        let s_40_0: () = ();
        // S s_40_1: call EndOfInstruction(s_40_0)
        let s_40_1: () = EndOfInstruction(state, tracer, s_40_0);
        // N s_40_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var ga#250880:u8
        let s_41_0: u8 = fn_state.ga_250880;
        // D s_41_1: cast zx s_41_0 -> bv
        let s_41_1: Bits = Bits::new(s_41_0 as u128, 7u16);
        // C s_41_2: const #18u : u8
        let s_41_2: u8 = 18;
        // C s_41_3: cast zx s_41_2 -> bv
        let s_41_3: Bits = Bits::new(s_41_2 as u128, 7u16);
        // D s_41_4: cmp-eq s_41_1 s_41_3
        let s_41_4: bool = ((s_41_1) == (s_41_3));
        // D s_41_5: not s_41_4
        let s_41_5: bool = !s_41_4;
        // N s_41_6: branch s_41_5 b46 b42
        if s_41_5 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #() : ()
        let s_42_0: () = ();
        // S s_42_1: call HaveSelfHostedTrace(s_42_0)
        let s_42_1: bool = HaveSelfHostedTrace(state, tracer, s_42_0);
        // S s_42_2: not s_42_1
        let s_42_2: bool = !s_42_1;
        // N s_42_3: branch s_42_2 b45 b43
        if s_42_2 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_43_0: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #9u : u32
        let s_44_0: u32 = 9;
        // D s_44_1: write-var op <= s_44_0
        fn_state.op = s_44_0;
        // N s_44_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #() : ()
        let s_45_0: () = ();
        // S s_45_1: call EndOfInstruction(s_45_0)
        let s_45_1: () = EndOfInstruction(state, tracer, s_45_0);
        // N s_45_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var ga#250880:u8
        let s_46_0: u8 = fn_state.ga_250880;
        // D s_46_1: cast zx s_46_0 -> bv
        let s_46_1: Bits = Bits::new(s_46_0 as u128, 7u16);
        // C s_46_2: const #19u : u8
        let s_46_2: u8 = 19;
        // C s_46_3: cast zx s_46_2 -> bv
        let s_46_3: Bits = Bits::new(s_46_2 as u128, 7u16);
        // D s_46_4: cmp-eq s_46_1 s_46_3
        let s_46_4: bool = ((s_46_1) == (s_46_3));
        // D s_46_5: not s_46_4
        let s_46_5: bool = !s_46_4;
        // N s_46_6: branch s_46_5 b51 b47
        if s_46_5 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #245u : u32
        let s_47_0: u32 = 245;
        // S s_47_1: call IsFeatureImplemented(s_47_0)
        let s_47_1: bool = IsFeatureImplemented(state, tracer, s_47_0);
        // S s_47_2: not s_47_1
        let s_47_2: bool = !s_47_1;
        // N s_47_3: branch s_47_2 b50 b48
        if s_47_2 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_48_0: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #14u : u32
        let s_49_0: u32 = 14;
        // D s_49_1: write-var op <= s_49_0
        fn_state.op = s_49_0;
        // N s_49_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #() : ()
        let s_50_0: () = ();
        // S s_50_1: call EndOfInstruction(s_50_0)
        let s_50_1: () = EndOfInstruction(state, tracer, s_50_0);
        // N s_50_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var ga#250880:u8
        let s_51_0: u8 = fn_state.ga_250880;
        // D s_51_1: cast zx s_51_0 -> bv
        let s_51_1: Bits = Bits::new(s_51_0 as u128, 7u16);
        // C s_51_2: const #20u : u8
        let s_51_2: u8 = 20;
        // C s_51_3: cast zx s_51_2 -> bv
        let s_51_3: Bits = Bits::new(s_51_2 as u128, 7u16);
        // D s_51_4: cmp-eq s_51_1 s_51_3
        let s_51_4: bool = ((s_51_1) == (s_51_3));
        // D s_51_5: not s_51_4
        let s_51_5: bool = !s_51_4;
        // N s_51_6: branch s_51_5 b53 b52
        if s_51_5 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #16u : u32
        let s_52_0: u32 = 16;
        // D s_52_1: write-var op <= s_52_0
        fn_state.op = s_52_0;
        // N s_52_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var ga#250880:u8
        let s_53_0: u8 = fn_state.ga_250880;
        // D s_53_1: cast zx s_53_0 -> bv
        let s_53_1: Bits = Bits::new(s_53_0 as u128, 7u16);
        // C s_53_2: const #22u : u8
        let s_53_2: u8 = 22;
        // C s_53_3: cast zx s_53_2 -> bv
        let s_53_3: Bits = Bits::new(s_53_2 as u128, 7u16);
        // D s_53_4: cmp-eq s_53_1 s_53_3
        let s_53_4: bool = ((s_53_1) == (s_53_3));
        // D s_53_5: not s_53_4
        let s_53_5: bool = !s_53_4;
        // N s_53_6: branch s_53_5 b58 b54
        if s_53_5 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #() : ()
        let s_54_0: () = ();
        // S s_54_1: call HaveFeatCLRBHB(s_54_0)
        let s_54_1: bool = HaveFeatCLRBHB(state, tracer, s_54_0);
        // S s_54_2: not s_54_1
        let s_54_2: bool = !s_54_1;
        // N s_54_3: branch s_54_2 b57 b55
        if s_54_2 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_55_0: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #13u : u32
        let s_56_0: u32 = 13;
        // D s_56_1: write-var op <= s_56_0
        fn_state.op = s_56_0;
        // N s_56_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #() : ()
        let s_57_0: () = ();
        // S s_57_1: call EndOfInstruction(s_57_0)
        let s_57_1: () = EndOfInstruction(state, tracer, s_57_0);
        // N s_57_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var ga#250880:u8
        let s_58_0: u8 = fn_state.ga_250880;
        // C s_58_1: const #3s : i
        let s_58_1: i128 = 3;
        // D s_58_2: cast zx s_58_0 -> bv
        let s_58_2: Bits = Bits::new(s_58_0 as u128, 7u16);
        // C s_58_3: const #1s : i64
        let s_58_3: i64 = 1;
        // C s_58_4: cast zx s_58_3 -> i
        let s_58_4: i128 = (i128::try_from(s_58_3).unwrap());
        // C s_58_5: const #3s : i
        let s_58_5: i128 = 3;
        // C s_58_6: add s_58_5 s_58_4
        let s_58_6: i128 = (s_58_5 + s_58_4);
        // D s_58_7: bit-extract s_58_2 s_58_1 s_58_6
        let s_58_7: Bits = (Bits::new(
            ((s_58_2) >> (s_58_1)).value(),
            u16::try_from(s_58_6).unwrap(),
        ));
        // D s_58_8: cast reint s_58_7 -> u8
        let s_58_8: u8 = (s_58_7.value() as u8);
        // D s_58_9: cast zx s_58_8 -> bv
        let s_58_9: Bits = Bits::new(s_58_8 as u128, 4u16);
        // C s_58_10: const #3u : u8
        let s_58_10: u8 = 3;
        // C s_58_11: cast zx s_58_10 -> bv
        let s_58_11: Bits = Bits::new(s_58_10 as u128, 4u16);
        // D s_58_12: cmp-eq s_58_9 s_58_11
        let s_58_12: bool = ((s_58_9) == (s_58_11));
        // D s_58_13: not s_58_12
        let s_58_13: bool = !s_58_12;
        // N s_58_14: branch s_58_13 b74 b59
        if s_58_13 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var op2:u8
        let s_59_0: u8 = fn_state.op2;
        // D s_59_1: cast zx s_59_0 -> bv
        let s_59_1: Bits = Bits::new(s_59_0 as u128, 3u16);
        // C s_59_2: const #0u : u8
        let s_59_2: u8 = 0;
        // C s_59_3: cast zx s_59_2 -> bv
        let s_59_3: Bits = Bits::new(s_59_2 as u128, 3u16);
        // D s_59_4: cmp-eq s_59_1 s_59_3
        let s_59_4: bool = ((s_59_1) == (s_59_3));
        // D s_59_5: not s_59_4
        let s_59_5: bool = !s_59_4;
        // N s_59_6: branch s_59_5 b61 b60
        if s_59_5 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_60_0: panic
        panic!("{:?}", ());
        // N s_60_1: return
        return;
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var op2:u8
        let s_61_0: u8 = fn_state.op2;
        // D s_61_1: cast zx s_61_0 -> bv
        let s_61_1: Bits = Bits::new(s_61_0 as u128, 3u16);
        // C s_61_2: const #1u : u8
        let s_61_2: u8 = 1;
        // C s_61_3: cast zx s_61_2 -> bv
        let s_61_3: Bits = Bits::new(s_61_2 as u128, 3u16);
        // D s_61_4: cmp-eq s_61_1 s_61_3
        let s_61_4: bool = ((s_61_1) == (s_61_3));
        // D s_61_5: not s_61_4
        let s_61_5: bool = !s_61_4;
        // N s_61_6: branch s_61_5 b63 b62
        if s_61_5 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_62_0: panic
        panic!("{:?}", ());
        // N s_62_1: return
        return;
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var op2:u8
        let s_63_0: u8 = fn_state.op2;
        // D s_63_1: cast zx s_63_0 -> bv
        let s_63_1: Bits = Bits::new(s_63_0 as u128, 3u16);
        // C s_63_2: const #2u : u8
        let s_63_2: u8 = 2;
        // C s_63_3: cast zx s_63_2 -> bv
        let s_63_3: Bits = Bits::new(s_63_2 as u128, 3u16);
        // D s_63_4: cmp-eq s_63_1 s_63_3
        let s_63_4: bool = ((s_63_1) == (s_63_3));
        // D s_63_5: not s_63_4
        let s_63_5: bool = !s_63_4;
        // N s_63_6: branch s_63_5 b65 b64
        if s_63_5 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_64_0: panic
        panic!("{:?}", ());
        // N s_64_1: return
        return;
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var op2:u8
        let s_65_0: u8 = fn_state.op2;
        // D s_65_1: cast zx s_65_0 -> bv
        let s_65_1: Bits = Bits::new(s_65_0 as u128, 3u16);
        // C s_65_2: const #3u : u8
        let s_65_2: u8 = 3;
        // C s_65_3: cast zx s_65_2 -> bv
        let s_65_3: Bits = Bits::new(s_65_2 as u128, 3u16);
        // D s_65_4: cmp-eq s_65_1 s_65_3
        let s_65_4: bool = ((s_65_1) == (s_65_3));
        // D s_65_5: not s_65_4
        let s_65_5: bool = !s_65_4;
        // N s_65_6: branch s_65_5 b67 b66
        if s_65_5 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_66(state, tracer, fn_state);
        };
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_66_0: panic
        panic!("{:?}", ());
        // N s_66_1: return
        return;
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var op2:u8
        let s_67_0: u8 = fn_state.op2;
        // D s_67_1: cast zx s_67_0 -> bv
        let s_67_1: Bits = Bits::new(s_67_0 as u128, 3u16);
        // C s_67_2: const #4u : u8
        let s_67_2: u8 = 4;
        // C s_67_3: cast zx s_67_2 -> bv
        let s_67_3: Bits = Bits::new(s_67_2 as u128, 3u16);
        // D s_67_4: cmp-eq s_67_1 s_67_3
        let s_67_4: bool = ((s_67_1) == (s_67_3));
        // D s_67_5: not s_67_4
        let s_67_5: bool = !s_67_4;
        // N s_67_6: branch s_67_5 b69 b68
        if s_67_5 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_68_0: panic
        panic!("{:?}", ());
        // N s_68_1: return
        return;
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var op2:u8
        let s_69_0: u8 = fn_state.op2;
        // D s_69_1: cast zx s_69_0 -> bv
        let s_69_1: Bits = Bits::new(s_69_0 as u128, 3u16);
        // C s_69_2: const #5u : u8
        let s_69_2: u8 = 5;
        // C s_69_3: cast zx s_69_2 -> bv
        let s_69_3: Bits = Bits::new(s_69_2 as u128, 3u16);
        // D s_69_4: cmp-eq s_69_1 s_69_3
        let s_69_4: bool = ((s_69_1) == (s_69_3));
        // D s_69_5: not s_69_4
        let s_69_5: bool = !s_69_4;
        // N s_69_6: branch s_69_5 b71 b70
        if s_69_5 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_70(state, tracer, fn_state);
        };
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_70_0: panic
        panic!("{:?}", ());
        // N s_70_1: return
        return;
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var op2:u8
        let s_71_0: u8 = fn_state.op2;
        // D s_71_1: cast zx s_71_0 -> bv
        let s_71_1: Bits = Bits::new(s_71_0 as u128, 3u16);
        // C s_71_2: const #6u : u8
        let s_71_2: u8 = 6;
        // C s_71_3: cast zx s_71_2 -> bv
        let s_71_3: Bits = Bits::new(s_71_2 as u128, 3u16);
        // D s_71_4: cmp-eq s_71_1 s_71_3
        let s_71_4: bool = ((s_71_1) == (s_71_3));
        // D s_71_5: not s_71_4
        let s_71_5: bool = !s_71_4;
        // N s_71_6: branch s_71_5 b73 b72
        if s_71_5 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_72(state, tracer, fn_state);
        };
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_72_0: panic
        panic!("{:?}", ());
        // N s_72_1: return
        return;
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_73_0: panic
        panic!("{:?}", ());
        // N s_73_1: return
        return;
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var ga#250880:u8
        let s_74_0: u8 = fn_state.ga_250880;
        // D s_74_1: write-var b__27 <= s_74_0
        fn_state.b__27 = s_74_0;
        // C s_74_2: const #3s : i
        let s_74_2: i128 = 3;
        // D s_74_3: read-var b__27:u8
        let s_74_3: u8 = fn_state.b__27;
        // D s_74_4: cast zx s_74_3 -> bv
        let s_74_4: Bits = Bits::new(s_74_3 as u128, 7u16);
        // C s_74_5: const #1s : i64
        let s_74_5: i64 = 1;
        // C s_74_6: cast zx s_74_5 -> i
        let s_74_6: i128 = (i128::try_from(s_74_5).unwrap());
        // C s_74_7: const #3s : i
        let s_74_7: i128 = 3;
        // C s_74_8: add s_74_7 s_74_6
        let s_74_8: i128 = (s_74_7 + s_74_6);
        // D s_74_9: bit-extract s_74_4 s_74_2 s_74_8
        let s_74_9: Bits = (Bits::new(
            ((s_74_4) >> (s_74_2)).value(),
            u16::try_from(s_74_8).unwrap(),
        ));
        // D s_74_10: cast reint s_74_9 -> u8
        let s_74_10: u8 = (s_74_9.value() as u8);
        // D s_74_11: cast zx s_74_10 -> bv
        let s_74_11: Bits = Bits::new(s_74_10 as u128, 4u16);
        // C s_74_12: const #4u : u8
        let s_74_12: u8 = 4;
        // C s_74_13: cast zx s_74_12 -> bv
        let s_74_13: Bits = Bits::new(s_74_12 as u128, 4u16);
        // D s_74_14: cmp-eq s_74_11 s_74_13
        let s_74_14: bool = ((s_74_11) == (s_74_13));
        // N s_74_15: branch s_74_14 b84 b75
        if s_74_14 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #0u : u8
        let s_75_0: bool = false;
        // D s_75_1: write-var gs#145736 <= s_75_0
        fn_state.gs_145736 = s_75_0;
        // N s_75_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var gs#145736:u8
        let s_76_0: bool = fn_state.gs_145736;
        // D s_76_1: not s_76_0
        let s_76_1: bool = !s_76_0;
        // N s_76_2: branch s_76_1 b78 b77
        if s_76_1 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #10u : u32
        let s_77_0: u32 = 10;
        // D s_77_1: write-var op <= s_77_0
        fn_state.op = s_77_0;
        // C s_77_2: const #1s : i
        let s_77_2: i128 = 1;
        // D s_77_3: read-var op2:u8
        let s_77_3: u8 = fn_state.op2;
        // D s_77_4: cast zx s_77_3 -> bv
        let s_77_4: Bits = Bits::new(s_77_3 as u128, 3u16);
        // C s_77_5: const #1s : i64
        let s_77_5: i64 = 1;
        // C s_77_6: cast zx s_77_5 -> i
        let s_77_6: i128 = (i128::try_from(s_77_5).unwrap());
        // C s_77_7: const #1s : i
        let s_77_7: i128 = 1;
        // C s_77_8: add s_77_7 s_77_6
        let s_77_8: i128 = (s_77_7 + s_77_6);
        // D s_77_9: bit-extract s_77_4 s_77_2 s_77_8
        let s_77_9: Bits = (Bits::new(
            ((s_77_4) >> (s_77_2)).value(),
            u16::try_from(s_77_8).unwrap(),
        ));
        // D s_77_10: cast reint s_77_9 -> u8
        let s_77_10: u8 = (s_77_9.value() as u8);
        // D s_77_11: call BTypeCompatible_BTI(s_77_10)
        let s_77_11: bool = BTypeCompatible_BTI(state, tracer, s_77_10);
        // D s_77_12: call SetBTypeCompatible(s_77_11)
        let s_77_12: () = SetBTypeCompatible(state, tracer, s_77_11);
        // N s_77_13: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var ga#250880:u8
        let s_78_0: u8 = fn_state.ga_250880;
        // D s_78_1: cast zx s_78_0 -> bv
        let s_78_1: Bits = Bits::new(s_78_0 as u128, 7u16);
        // C s_78_2: const #40u : u8
        let s_78_2: u8 = 40;
        // C s_78_3: cast zx s_78_2 -> bv
        let s_78_3: Bits = Bits::new(s_78_2 as u128, 7u16);
        // D s_78_4: cmp-eq s_78_1 s_78_3
        let s_78_4: bool = ((s_78_1) == (s_78_3));
        // D s_78_5: not s_78_4
        let s_78_5: bool = !s_78_4;
        // N s_78_6: branch s_78_5 b83 b79
        if s_78_5 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #241u : u32
        let s_79_0: u32 = 241;
        // S s_79_1: call IsFeatureImplemented(s_79_0)
        let s_79_1: bool = IsFeatureImplemented(state, tracer, s_79_0);
        // S s_79_2: not s_79_1
        let s_79_2: bool = !s_79_1;
        // N s_79_3: branch s_79_2 b82 b80
        if s_79_2 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_80_0: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #15u : u32
        let s_81_0: u32 = 15;
        // D s_81_1: write-var op <= s_81_0
        fn_state.op = s_81_0;
        // N s_81_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #() : ()
        let s_82_0: () = ();
        // S s_82_1: call EndOfInstruction(s_82_0)
        let s_82_1: () = EndOfInstruction(state, tracer, s_82_0);
        // N s_82_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #() : ()
        let s_83_0: () = ();
        // S s_83_1: call EndOfInstruction(s_83_0)
        let s_83_1: () = EndOfInstruction(state, tracer, s_83_0);
        // N s_83_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #0s : i
        let s_84_0: i128 = 0;
        // D s_84_1: read-var b__27:u8
        let s_84_1: u8 = fn_state.b__27;
        // D s_84_2: cast zx s_84_1 -> bv
        let s_84_2: Bits = Bits::new(s_84_1 as u128, 7u16);
        // C s_84_3: const #1s : i64
        let s_84_3: i64 = 1;
        // C s_84_4: cast zx s_84_3 -> i
        let s_84_4: i128 = (i128::try_from(s_84_3).unwrap());
        // C s_84_5: const #0s : i
        let s_84_5: i128 = 0;
        // C s_84_6: add s_84_5 s_84_4
        let s_84_6: i128 = (s_84_5 + s_84_4);
        // D s_84_7: bit-extract s_84_2 s_84_0 s_84_6
        let s_84_7: Bits = (Bits::new(
            ((s_84_2) >> (s_84_0)).value(),
            u16::try_from(s_84_6).unwrap(),
        ));
        // D s_84_8: cast reint s_84_7 -> u8
        let s_84_8: bool = ((s_84_7.value()) != 0);
        // D s_84_9: cast zx s_84_8 -> bv
        let s_84_9: Bits = Bits::new(s_84_8 as u128, 1u16);
        // C s_84_10: const #0u : u8
        let s_84_10: bool = false;
        // C s_84_11: cast zx s_84_10 -> bv
        let s_84_11: Bits = Bits::new(s_84_10 as u128, 1u16);
        // D s_84_12: cmp-eq s_84_9 s_84_11
        let s_84_12: bool = ((s_84_9) == (s_84_11));
        // D s_84_13: write-var gs#145736 <= s_84_12
        fn_state.gs_145736 = s_84_12;
        // N s_84_14: jump b76
        return block_76(state, tracer, fn_state);
    }
}
