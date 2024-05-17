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
use execute_aarch32_instrs_ADD_i_OpT_A_txt::*;
use T32ExpandImm::*;
use common::*;
pub fn decode_aarch32_instrs_ADD_i_T3enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    i: bool,
    S: bool,
    Rn: u8,
    imm3: u8,
    Rd: u8,
    imm8: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        imm32: u32,
        setflags: bool,
        gs_295463: bool,
        gs_295465: bool,
        gs_295457: bool,
        n: i64,
        d: i64,
        i: bool,
        S: bool,
        Rn: u8,
        imm3: u8,
        Rd: u8,
        imm8: u8,
    }
    let fn_state = FunctionState {
        i,
        S,
        Rn,
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
        // N s_2_5: branch s_2_4 b17 b3
        if s_2_4 {
            return block_17(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#295457 <= s_3_0
        fn_state.gs_295457 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#295457:u8
        let s_4_0: bool = fn_state.gs_295457;
        // N s_4_1: branch s_4_0 b16 b5
        if s_4_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var Rn:u8
        let s_5_0: u8 = fn_state.Rn;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 4u16);
        // C s_5_2: const #13u : u8
        let s_5_2: u8 = 13;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 4u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // N s_5_5: branch s_5_4 b15 b6
        if s_5_4 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var Rd:u8
        let s_6_0: u8 = fn_state.Rd;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 4u16);
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (s_6_1.value() as i128);
        // D s_6_3: cast reint s_6_2 -> i64
        let s_6_3: i64 = (s_6_2 as i64);
        // D s_6_4: write-var d <= s_6_3
        fn_state.d = s_6_3;
        // D s_6_5: read-var Rn:u8
        let s_6_5: u8 = fn_state.Rn;
        // D s_6_6: cast zx s_6_5 -> bv
        let s_6_6: Bits = Bits::new(s_6_5 as u128, 4u16);
        // D s_6_7: cast zx s_6_6 -> i
        let s_6_7: i128 = (s_6_6.value() as i128);
        // D s_6_8: cast reint s_6_7 -> i64
        let s_6_8: i64 = (s_6_7 as i64);
        // D s_6_9: write-var n <= s_6_8
        fn_state.n = s_6_8;
        // D s_6_10: read-var S:u8
        let s_6_10: bool = fn_state.S;
        // D s_6_11: cast zx s_6_10 -> bv
        let s_6_11: Bits = Bits::new(s_6_10 as u128, 1u16);
        // C s_6_12: const #1u : u8
        let s_6_12: bool = true;
        // C s_6_13: cast zx s_6_12 -> bv
        let s_6_13: Bits = Bits::new(s_6_12 as u128, 1u16);
        // D s_6_14: cmp-eq s_6_11 s_6_13
        let s_6_14: bool = ((s_6_11) == (s_6_13));
        // D s_6_15: write-var setflags <= s_6_14
        fn_state.setflags = s_6_14;
        // D s_6_16: read-var i:u8
        let s_6_16: bool = fn_state.i;
        // D s_6_17: cast zx s_6_16 -> bv
        let s_6_17: Bits = Bits::new(s_6_16 as u128, 1u16);
        // D s_6_18: read-var imm3:u8
        let s_6_18: u8 = fn_state.imm3;
        // D s_6_19: cast zx s_6_18 -> bv
        let s_6_19: Bits = Bits::new(s_6_18 as u128, 3u16);
        // D s_6_20: cast reint s_6_17 -> u128
        let s_6_20: u128 = (s_6_17.value() as u128);
        // D s_6_21: size-of s_6_17
        let s_6_21: u16 = s_6_17.length();
        // D s_6_22: cast reint s_6_19 -> u128
        let s_6_22: u128 = (s_6_19.value() as u128);
        // D s_6_23: size-of s_6_19
        let s_6_23: u16 = s_6_19.length();
        // D s_6_24: lsl s_6_20 s_6_23
        let s_6_24: u128 = s_6_20 << s_6_23;
        // D s_6_25: or s_6_24 s_6_22
        let s_6_25: u128 = ((s_6_24) | (s_6_22));
        // D s_6_26: add s_6_21 s_6_23
        let s_6_26: u16 = (s_6_21 + s_6_23);
        // D s_6_27: create-bits s_6_25 s_6_26
        let s_6_27: Bits = Bits::new(s_6_25, s_6_26);
        // D s_6_28: cast reint s_6_27 -> u8
        let s_6_28: u8 = (s_6_27.value() as u8);
        // D s_6_29: cast zx s_6_28 -> bv
        let s_6_29: Bits = Bits::new(s_6_28 as u128, 4u16);
        // D s_6_30: read-var imm8:u8
        let s_6_30: u8 = fn_state.imm8;
        // D s_6_31: cast zx s_6_30 -> bv
        let s_6_31: Bits = Bits::new(s_6_30 as u128, 8u16);
        // D s_6_32: cast reint s_6_29 -> u128
        let s_6_32: u128 = (s_6_29.value() as u128);
        // D s_6_33: size-of s_6_29
        let s_6_33: u16 = s_6_29.length();
        // D s_6_34: cast reint s_6_31 -> u128
        let s_6_34: u128 = (s_6_31.value() as u128);
        // D s_6_35: size-of s_6_31
        let s_6_35: u16 = s_6_31.length();
        // D s_6_36: lsl s_6_32 s_6_35
        let s_6_36: u128 = s_6_32 << s_6_35;
        // D s_6_37: or s_6_36 s_6_34
        let s_6_37: u128 = ((s_6_36) | (s_6_34));
        // D s_6_38: add s_6_33 s_6_35
        let s_6_38: u16 = (s_6_33 + s_6_35);
        // D s_6_39: create-bits s_6_37 s_6_38
        let s_6_39: Bits = Bits::new(s_6_37, s_6_38);
        // D s_6_40: cast reint s_6_39 -> u12
        let s_6_40: u16 = (s_6_39.value() as u16);
        // D s_6_41: call T32ExpandImm(s_6_40)
        let s_6_41: u32 = T32ExpandImm(state, tracer, s_6_40);
        // D s_6_42: write-var imm32 <= s_6_41
        fn_state.imm32 = s_6_41;
        // C s_6_43: const #15s : i
        let s_6_43: i128 = 15;
        // D s_6_44: read-var d:i64
        let s_6_44: i64 = fn_state.d;
        // D s_6_45: cast zx s_6_44 -> i
        let s_6_45: i128 = (i128::try_from(s_6_44).unwrap());
        // D s_6_46: cmp-eq s_6_45 s_6_43
        let s_6_46: bool = ((s_6_45) == (s_6_43));
        // N s_6_47: branch s_6_46 b14 b7
        if s_6_46 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#295463 <= s_7_0
        fn_state.gs_295463 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#295463:u8
        let s_8_0: bool = fn_state.gs_295463;
        // N s_8_1: branch s_8_0 b13 b9
        if s_8_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #15s : i
        let s_9_0: i128 = 15;
        // D s_9_1: read-var n:i64
        let s_9_1: i64 = fn_state.n;
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (i128::try_from(s_9_1).unwrap());
        // D s_9_3: cmp-eq s_9_2 s_9_0
        let s_9_3: bool = ((s_9_2) == (s_9_0));
        // D s_9_4: write-var gs#295465 <= s_9_3
        fn_state.gs_295465 = s_9_3;
        // N s_9_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#295465:u8
        let s_10_0: bool = fn_state.gs_295465;
        // N s_10_1: branch s_10_0 b12 b11
        if s_10_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var d:i64
        let s_11_0: i64 = fn_state.d;
        // D s_11_1: read-var imm32:u32
        let s_11_1: u32 = fn_state.imm32;
        // D s_11_2: read-var n:i64
        let s_11_2: i64 = fn_state.n;
        // D s_11_3: read-var setflags:u8
        let s_11_3: bool = fn_state.setflags;
        // D s_11_4: call execute_aarch32_instrs_ADD_i_OpT_A_txt(s_11_0, s_11_1, s_11_2, s_11_3)
        let s_11_4: () = execute_aarch32_instrs_ADD_i_OpT_A_txt(
            state,
            tracer,
            s_11_0,
            s_11_1,
            s_11_2,
            s_11_3,
        );
        // N s_11_5: return
        return;
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
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var gs#295465 <= s_13_0
        fn_state.gs_295465 = s_13_0;
        // N s_13_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var setflags:u8
        let s_14_0: bool = fn_state.setflags;
        // D s_14_1: not s_14_0
        let s_14_1: bool = !s_14_0;
        // D s_14_2: write-var gs#295463 <= s_14_1
        fn_state.gs_295463 = s_14_1;
        // N s_14_3: jump b8
        return block_8(state, tracer, fn_state);
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
        // N s_16_0: panic
        panic!("{:?}", ());
        // N s_16_1: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var S:u8
        let s_17_0: bool = fn_state.S;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 1u16);
        // C s_17_2: const #1u : u8
        let s_17_2: bool = true;
        // C s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 1u16);
        // D s_17_4: cmp-eq s_17_1 s_17_3
        let s_17_4: bool = ((s_17_1) == (s_17_3));
        // D s_17_5: write-var gs#295457 <= s_17_4
        fn_state.gs_295457 = s_17_4;
        // N s_17_6: jump b4
        return block_4(state, tracer, fn_state);
    }
}
