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
use execute_aarch32_instrs_VLDR_Op_A_txt::*;
use ConditionPassed::*;
use place_slice::*;
use InITBlock::*;
use HaveFP16Ext::*;
use common::*;
pub fn decode_aarch32_instrs_VLDR_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    U: bool,
    D: bool,
    Rn: u8,
    Vd: u8,
    size: u8,
    imm8: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_311936: bool,
        imm32: u32,
        gs_311937: bool,
        esize: i64,
        d: i64,
        add: bool,
        gs_311938: bool,
        U: bool,
        D: bool,
        Rn: u8,
        Vd: u8,
        size: u8,
        imm8: u8,
    }
    let fn_state = FunctionState {
        U,
        D,
        Rn,
        Vd,
        size,
        imm8,
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
        // C s_2_2: const #0u : u8
        let s_2_2: u8 = 0;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 2u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b25 b3
        if s_2_4 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var size:u8
        let s_3_0: u8 = fn_state.size;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_2: const #1u : u8
        let s_3_2: u8 = 1;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 2u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // N s_3_5: branch s_3_4 b24 b4
        if s_3_4 {
            return block_24(state, tracer, fn_state);
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
        // D s_4_1: write-var gs#311936 <= s_4_0
        fn_state.gs_311936 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#311936:u8
        let s_5_0: bool = fn_state.gs_311936;
        // D s_5_1: write-var gs#311937 <= s_5_0
        fn_state.gs_311937 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#311937:u8
        let s_6_0: bool = fn_state.gs_311937;
        // N s_6_1: branch s_6_0 b23 b7
        if s_6_0 {
            return block_23(state, tracer, fn_state);
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
        // C s_7_2: const #1u : u8
        let s_7_2: u8 = 1;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 2u16);
        // D s_7_4: cmp-eq s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) == (s_7_3));
        // N s_7_5: branch s_7_4 b22 b8
        if s_7_4 {
            return block_22(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#311938 <= s_8_0
        fn_state.gs_311938 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#311938:u8
        let s_9_0: bool = fn_state.gs_311938;
        // N s_9_1: branch s_9_0 b21 b10
        if s_9_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var size:u8
        let s_10_0: u8 = fn_state.size;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 2u16);
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (s_10_1.value() as i128);
        // D s_10_3: cast reint s_10_2 -> i64
        let s_10_3: i64 = (s_10_2 as i64);
        // C s_10_4: const #8s : i64
        let s_10_4: i64 = 8;
        // D s_10_5: lsl s_10_4 s_10_3
        let s_10_5: i64 = s_10_4 << s_10_3;
        // D s_10_6: write-var esize <= s_10_5
        fn_state.esize = s_10_5;
        // D s_10_7: read-var U:u8
        let s_10_7: bool = fn_state.U;
        // D s_10_8: cast zx s_10_7 -> bv
        let s_10_8: Bits = Bits::new(s_10_7 as u128, 1u16);
        // C s_10_9: const #1u : u8
        let s_10_9: bool = true;
        // C s_10_10: cast zx s_10_9 -> bv
        let s_10_10: Bits = Bits::new(s_10_9 as u128, 1u16);
        // D s_10_11: cmp-eq s_10_8 s_10_10
        let s_10_11: bool = ((s_10_8) == (s_10_10));
        // D s_10_12: write-var add <= s_10_11
        fn_state.add = s_10_11;
        // C s_10_13: const #16s : i
        let s_10_13: i128 = 16;
        // D s_10_14: read-var esize:i64
        let s_10_14: i64 = fn_state.esize;
        // D s_10_15: cast zx s_10_14 -> i
        let s_10_15: i128 = (i128::try_from(s_10_14).unwrap());
        // D s_10_16: cmp-eq s_10_15 s_10_13
        let s_10_16: bool = ((s_10_15) == (s_10_13));
        // N s_10_17: branch s_10_16 b20 b11
        if s_10_16 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #32s : i
        let s_11_0: i128 = 32;
        // C s_11_1: const #0s : i
        let s_11_1: i128 = 0;
        // C s_11_2: const #8s : i
        let s_11_2: i128 = 8;
        // C s_11_3: const #2s : i
        let s_11_3: i128 = 2;
        // D s_11_4: read-var imm8:u8
        let s_11_4: u8 = fn_state.imm8;
        // D s_11_5: cast zx s_11_4 -> bv
        let s_11_5: Bits = Bits::new(s_11_4 as u128, 8u16);
        // D s_11_6: call place_slice(s_11_0, s_11_5, s_11_1, s_11_2, s_11_3)
        let s_11_6: Bits = place_slice(
            state,
            tracer,
            s_11_0,
            s_11_5,
            s_11_1,
            s_11_2,
            s_11_3,
        );
        // D s_11_7: cast reint s_11_6 -> u32
        let s_11_7: u32 = (s_11_6.value() as u32);
        // D s_11_8: write-var imm32 <= s_11_7
        fn_state.imm32 = s_11_7;
        // N s_11_9: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var size:u8
        let s_12_0: u8 = fn_state.size;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 2u16);
        // C s_12_2: const #1u : u8
        let s_12_2: u8 = 1;
        // C s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 2u16);
        // D s_12_4: cmp-eq s_12_1 s_12_3
        let s_12_4: bool = ((s_12_1) == (s_12_3));
        // D s_12_5: not s_12_4
        let s_12_5: bool = !s_12_4;
        // N s_12_6: branch s_12_5 b15 b13
        if s_12_5 {
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
        // D s_13_0: read-var Vd:u8
        let s_13_0: u8 = fn_state.Vd;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 4u16);
        // D s_13_2: read-var D:u8
        let s_13_2: bool = fn_state.D;
        // D s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 1u16);
        // D s_13_4: cast reint s_13_1 -> u128
        let s_13_4: u128 = (s_13_1.value() as u128);
        // D s_13_5: size-of s_13_1
        let s_13_5: u16 = s_13_1.length();
        // D s_13_6: cast reint s_13_3 -> u128
        let s_13_6: u128 = (s_13_3.value() as u128);
        // D s_13_7: size-of s_13_3
        let s_13_7: u16 = s_13_3.length();
        // D s_13_8: lsl s_13_4 s_13_7
        let s_13_8: u128 = s_13_4 << s_13_7;
        // D s_13_9: or s_13_8 s_13_6
        let s_13_9: u128 = ((s_13_8) | (s_13_6));
        // D s_13_10: add s_13_5 s_13_7
        let s_13_10: u16 = (s_13_5 + s_13_7);
        // D s_13_11: create-bits s_13_9 s_13_10
        let s_13_11: Bits = Bits::new(s_13_9, s_13_10);
        // D s_13_12: cast reint s_13_11 -> u8
        let s_13_12: u8 = (s_13_11.value() as u8);
        // D s_13_13: cast zx s_13_12 -> bv
        let s_13_13: Bits = Bits::new(s_13_12 as u128, 5u16);
        // D s_13_14: cast zx s_13_13 -> i
        let s_13_14: i128 = (s_13_13.value() as i128);
        // D s_13_15: cast reint s_13_14 -> i64
        let s_13_15: i64 = (s_13_14 as i64);
        // D s_13_16: write-var d <= s_13_15
        fn_state.d = s_13_15;
        // N s_13_17: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var d:i64
        let s_14_0: i64 = fn_state.d;
        // D s_14_1: read-var Rn:u8
        let s_14_1: u8 = fn_state.Rn;
        // D s_14_2: cast zx s_14_1 -> bv
        let s_14_2: Bits = Bits::new(s_14_1 as u128, 4u16);
        // D s_14_3: cast zx s_14_2 -> i
        let s_14_3: i128 = (s_14_2.value() as i128);
        // D s_14_4: cast reint s_14_3 -> i64
        let s_14_4: i64 = (s_14_3 as i64);
        // D s_14_5: read-var add:u8
        let s_14_5: bool = fn_state.add;
        // D s_14_6: read-var esize:i64
        let s_14_6: i64 = fn_state.esize;
        // D s_14_7: read-var imm32:u32
        let s_14_7: u32 = fn_state.imm32;
        // D s_14_8: call execute_aarch32_instrs_VLDR_Op_A_txt(s_14_5, s_14_0, s_14_6, s_14_7, s_14_4)
        let s_14_8: () = execute_aarch32_instrs_VLDR_Op_A_txt(
            state,
            tracer,
            s_14_5,
            s_14_0,
            s_14_6,
            s_14_7,
            s_14_4,
        );
        // N s_14_9: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var size:u8
        let s_15_0: u8 = fn_state.size;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 2u16);
        // C s_15_2: const #2u : u8
        let s_15_2: u8 = 2;
        // C s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 2u16);
        // D s_15_4: cmp-eq s_15_1 s_15_3
        let s_15_4: bool = ((s_15_1) == (s_15_3));
        // D s_15_5: not s_15_4
        let s_15_5: bool = !s_15_4;
        // N s_15_6: branch s_15_5 b17 b16
        if s_15_5 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var Vd:u8
        let s_16_0: u8 = fn_state.Vd;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 4u16);
        // D s_16_2: read-var D:u8
        let s_16_2: bool = fn_state.D;
        // D s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 1u16);
        // D s_16_4: cast reint s_16_1 -> u128
        let s_16_4: u128 = (s_16_1.value() as u128);
        // D s_16_5: size-of s_16_1
        let s_16_5: u16 = s_16_1.length();
        // D s_16_6: cast reint s_16_3 -> u128
        let s_16_6: u128 = (s_16_3.value() as u128);
        // D s_16_7: size-of s_16_3
        let s_16_7: u16 = s_16_3.length();
        // D s_16_8: lsl s_16_4 s_16_7
        let s_16_8: u128 = s_16_4 << s_16_7;
        // D s_16_9: or s_16_8 s_16_6
        let s_16_9: u128 = ((s_16_8) | (s_16_6));
        // D s_16_10: add s_16_5 s_16_7
        let s_16_10: u16 = (s_16_5 + s_16_7);
        // D s_16_11: create-bits s_16_9 s_16_10
        let s_16_11: Bits = Bits::new(s_16_9, s_16_10);
        // D s_16_12: cast reint s_16_11 -> u8
        let s_16_12: u8 = (s_16_11.value() as u8);
        // D s_16_13: cast zx s_16_12 -> bv
        let s_16_13: Bits = Bits::new(s_16_12 as u128, 5u16);
        // D s_16_14: cast zx s_16_13 -> i
        let s_16_14: i128 = (s_16_13.value() as i128);
        // D s_16_15: cast reint s_16_14 -> i64
        let s_16_15: i64 = (s_16_14 as i64);
        // D s_16_16: write-var d <= s_16_15
        fn_state.d = s_16_15;
        // N s_16_17: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var size:u8
        let s_17_0: u8 = fn_state.size;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 2u16);
        // C s_17_2: const #3u : u8
        let s_17_2: u8 = 3;
        // C s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 2u16);
        // D s_17_4: cmp-eq s_17_1 s_17_3
        let s_17_4: bool = ((s_17_1) == (s_17_3));
        // D s_17_5: not s_17_4
        let s_17_5: bool = !s_17_4;
        // N s_17_6: branch s_17_5 b19 b18
        if s_17_5 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var D:u8
        let s_18_0: bool = fn_state.D;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 1u16);
        // D s_18_2: read-var Vd:u8
        let s_18_2: u8 = fn_state.Vd;
        // D s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 4u16);
        // D s_18_4: cast reint s_18_1 -> u128
        let s_18_4: u128 = (s_18_1.value() as u128);
        // D s_18_5: size-of s_18_1
        let s_18_5: u16 = s_18_1.length();
        // D s_18_6: cast reint s_18_3 -> u128
        let s_18_6: u128 = (s_18_3.value() as u128);
        // D s_18_7: size-of s_18_3
        let s_18_7: u16 = s_18_3.length();
        // D s_18_8: lsl s_18_4 s_18_7
        let s_18_8: u128 = s_18_4 << s_18_7;
        // D s_18_9: or s_18_8 s_18_6
        let s_18_9: u128 = ((s_18_8) | (s_18_6));
        // D s_18_10: add s_18_5 s_18_7
        let s_18_10: u16 = (s_18_5 + s_18_7);
        // D s_18_11: create-bits s_18_9 s_18_10
        let s_18_11: Bits = Bits::new(s_18_9, s_18_10);
        // D s_18_12: cast reint s_18_11 -> u8
        let s_18_12: u8 = (s_18_11.value() as u8);
        // D s_18_13: cast zx s_18_12 -> bv
        let s_18_13: Bits = Bits::new(s_18_12 as u128, 5u16);
        // D s_18_14: cast zx s_18_13 -> i
        let s_18_14: i128 = (s_18_13.value() as i128);
        // D s_18_15: cast reint s_18_14 -> i64
        let s_18_15: i64 = (s_18_14 as i64);
        // D s_18_16: write-var d <= s_18_15
        fn_state.d = s_18_15;
        // N s_18_17: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #32s : i
        let s_20_0: i128 = 32;
        // C s_20_1: const #0s : i
        let s_20_1: i128 = 0;
        // C s_20_2: const #8s : i
        let s_20_2: i128 = 8;
        // C s_20_3: const #1s : i
        let s_20_3: i128 = 1;
        // D s_20_4: read-var imm8:u8
        let s_20_4: u8 = fn_state.imm8;
        // D s_20_5: cast zx s_20_4 -> bv
        let s_20_5: Bits = Bits::new(s_20_4 as u128, 8u16);
        // D s_20_6: call place_slice(s_20_0, s_20_5, s_20_1, s_20_2, s_20_3)
        let s_20_6: Bits = place_slice(
            state,
            tracer,
            s_20_0,
            s_20_5,
            s_20_1,
            s_20_2,
            s_20_3,
        );
        // D s_20_7: cast reint s_20_6 -> u32
        let s_20_7: u32 = (s_20_6.value() as u32);
        // D s_20_8: write-var imm32 <= s_20_7
        fn_state.imm32 = s_20_7;
        // N s_20_9: jump b12
        return block_12(state, tracer, fn_state);
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
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call InITBlock(s_22_0)
        let s_22_1: bool = InITBlock(state, tracer, s_22_0);
        // D s_22_2: write-var gs#311938 <= s_22_1
        fn_state.gs_311938 = s_22_1;
        // N s_22_3: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: panic
        panic!("{:?}", ());
        // N s_23_1: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call HaveFP16Ext(s_24_0)
        let s_24_1: bool = HaveFP16Ext(state, tracer, s_24_0);
        // S s_24_2: not s_24_1
        let s_24_2: bool = !s_24_1;
        // D s_24_3: write-var gs#311936 <= s_24_2
        fn_state.gs_311936 = s_24_2;
        // N s_24_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #1u : u8
        let s_25_0: bool = true;
        // D s_25_1: write-var gs#311937 <= s_25_0
        fn_state.gs_311937 = s_25_0;
        // N s_25_2: jump b6
        return block_6(state, tracer, fn_state);
    }
}
