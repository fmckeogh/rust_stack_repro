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
use execute_aarch32_instrs_SUB_SP_i_Op_A_txt::*;
use T32ExpandImm::*;
use common::*;
pub fn decode_aarch32_instrs_SUB_SP_i_T2enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    i: bool,
    S: bool,
    imm3: u8,
    Rd: u8,
    imm8: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        imm32: u32,
        setflags: bool,
        gs_303348: bool,
        gs_303344: bool,
        d: i64,
        i: bool,
        S: bool,
        imm3: u8,
        Rd: u8,
        imm8: u8,
    }
    let fn_state = FunctionState {
        i,
        S,
        imm3,
        Rd,
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
        // D s_2_0: read-var Rd:u8
        let s_2_0: u8 = fn_state.Rd;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // C s_2_2: const #15u : u8
        let s_2_2: u8 = 15;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 4u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b12 b3
        if s_2_4 {
            return block_12(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#303344 <= s_3_0
        fn_state.gs_303344 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#303344:u8
        let s_4_0: bool = fn_state.gs_303344;
        // N s_4_1: branch s_4_0 b11 b5
        if s_4_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var Rd:u8
        let s_5_0: u8 = fn_state.Rd;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 4u16);
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (s_5_1.value() as i128);
        // D s_5_3: cast reint s_5_2 -> i64
        let s_5_3: i64 = (s_5_2 as i64);
        // D s_5_4: write-var d <= s_5_3
        fn_state.d = s_5_3;
        // D s_5_5: read-var S:u8
        let s_5_5: bool = fn_state.S;
        // D s_5_6: cast zx s_5_5 -> bv
        let s_5_6: Bits = Bits::new(s_5_5 as u128, 1u16);
        // C s_5_7: const #1u : u8
        let s_5_7: bool = true;
        // C s_5_8: cast zx s_5_7 -> bv
        let s_5_8: Bits = Bits::new(s_5_7 as u128, 1u16);
        // D s_5_9: cmp-eq s_5_6 s_5_8
        let s_5_9: bool = ((s_5_6) == (s_5_8));
        // D s_5_10: write-var setflags <= s_5_9
        fn_state.setflags = s_5_9;
        // D s_5_11: read-var i:u8
        let s_5_11: bool = fn_state.i;
        // D s_5_12: cast zx s_5_11 -> bv
        let s_5_12: Bits = Bits::new(s_5_11 as u128, 1u16);
        // D s_5_13: read-var imm3:u8
        let s_5_13: u8 = fn_state.imm3;
        // D s_5_14: cast zx s_5_13 -> bv
        let s_5_14: Bits = Bits::new(s_5_13 as u128, 3u16);
        // D s_5_15: cast reint s_5_12 -> u128
        let s_5_15: u128 = (s_5_12.value() as u128);
        // D s_5_16: size-of s_5_12
        let s_5_16: u16 = s_5_12.length();
        // D s_5_17: cast reint s_5_14 -> u128
        let s_5_17: u128 = (s_5_14.value() as u128);
        // D s_5_18: size-of s_5_14
        let s_5_18: u16 = s_5_14.length();
        // D s_5_19: lsl s_5_15 s_5_18
        let s_5_19: u128 = s_5_15 << s_5_18;
        // D s_5_20: or s_5_19 s_5_17
        let s_5_20: u128 = ((s_5_19) | (s_5_17));
        // D s_5_21: add s_5_16 s_5_18
        let s_5_21: u16 = (s_5_16 + s_5_18);
        // D s_5_22: create-bits s_5_20 s_5_21
        let s_5_22: Bits = Bits::new(s_5_20, s_5_21);
        // D s_5_23: cast reint s_5_22 -> u8
        let s_5_23: u8 = (s_5_22.value() as u8);
        // D s_5_24: cast zx s_5_23 -> bv
        let s_5_24: Bits = Bits::new(s_5_23 as u128, 4u16);
        // D s_5_25: read-var imm8:u8
        let s_5_25: u8 = fn_state.imm8;
        // D s_5_26: cast zx s_5_25 -> bv
        let s_5_26: Bits = Bits::new(s_5_25 as u128, 8u16);
        // D s_5_27: cast reint s_5_24 -> u128
        let s_5_27: u128 = (s_5_24.value() as u128);
        // D s_5_28: size-of s_5_24
        let s_5_28: u16 = s_5_24.length();
        // D s_5_29: cast reint s_5_26 -> u128
        let s_5_29: u128 = (s_5_26.value() as u128);
        // D s_5_30: size-of s_5_26
        let s_5_30: u16 = s_5_26.length();
        // D s_5_31: lsl s_5_27 s_5_30
        let s_5_31: u128 = s_5_27 << s_5_30;
        // D s_5_32: or s_5_31 s_5_29
        let s_5_32: u128 = ((s_5_31) | (s_5_29));
        // D s_5_33: add s_5_28 s_5_30
        let s_5_33: u16 = (s_5_28 + s_5_30);
        // D s_5_34: create-bits s_5_32 s_5_33
        let s_5_34: Bits = Bits::new(s_5_32, s_5_33);
        // D s_5_35: cast reint s_5_34 -> u12
        let s_5_35: u16 = (s_5_34.value() as u16);
        // D s_5_36: call T32ExpandImm(s_5_35)
        let s_5_36: u32 = T32ExpandImm(state, tracer, s_5_35);
        // D s_5_37: write-var imm32 <= s_5_36
        fn_state.imm32 = s_5_36;
        // C s_5_38: const #15s : i
        let s_5_38: i128 = 15;
        // D s_5_39: read-var d:i64
        let s_5_39: i64 = fn_state.d;
        // D s_5_40: cast zx s_5_39 -> i
        let s_5_40: i128 = (i128::try_from(s_5_39).unwrap());
        // D s_5_41: cmp-eq s_5_40 s_5_38
        let s_5_41: bool = ((s_5_40) == (s_5_38));
        // N s_5_42: branch s_5_41 b10 b6
        if s_5_41 {
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
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#303348 <= s_6_0
        fn_state.gs_303348 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#303348:u8
        let s_7_0: bool = fn_state.gs_303348;
        // N s_7_1: branch s_7_0 b9 b8
        if s_7_0 {
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
        // D s_8_0: read-var d:i64
        let s_8_0: i64 = fn_state.d;
        // D s_8_1: read-var imm32:u32
        let s_8_1: u32 = fn_state.imm32;
        // D s_8_2: read-var setflags:u8
        let s_8_2: bool = fn_state.setflags;
        // D s_8_3: call execute_aarch32_instrs_SUB_SP_i_Op_A_txt(s_8_0, s_8_1, s_8_2)
        let s_8_3: () = execute_aarch32_instrs_SUB_SP_i_Op_A_txt(
            state,
            tracer,
            s_8_0,
            s_8_1,
            s_8_2,
        );
        // N s_8_4: return
        return;
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
        // D s_10_0: read-var setflags:u8
        let s_10_0: bool = fn_state.setflags;
        // D s_10_1: not s_10_0
        let s_10_1: bool = !s_10_0;
        // D s_10_2: write-var gs#303348 <= s_10_1
        fn_state.gs_303348 = s_10_1;
        // N s_10_3: jump b7
        return block_7(state, tracer, fn_state);
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
        // D s_12_0: read-var S:u8
        let s_12_0: bool = fn_state.S;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 1u16);
        // C s_12_2: const #1u : u8
        let s_12_2: bool = true;
        // C s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 1u16);
        // D s_12_4: cmp-eq s_12_1 s_12_3
        let s_12_4: bool = ((s_12_1) == (s_12_3));
        // D s_12_5: write-var gs#303344 <= s_12_4
        fn_state.gs_303344 = s_12_4;
        // N s_12_6: jump b4
        return block_4(state, tracer, fn_state);
    }
}
