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
use InITBlock::*;
use execute_aarch32_instrs_B_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_B_T3enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    S: bool,
    cond: u8,
    imm6: u8,
    J1: bool,
    J2: bool,
    imm11: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        imm32: u32,
        S: bool,
        cond: u8,
        imm6: u8,
        J1: bool,
        J2: bool,
        imm11: u16,
    }
    let fn_state = FunctionState {
        S,
        cond,
        imm6,
        J1,
        J2,
        imm11,
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
        // D s_2_0: read-var cond:u8
        let s_2_0: u8 = fn_state.cond;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // C s_2_2: const #15u : u8
        let s_2_2: u8 = 15;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 4u16);
        // D s_2_4: cmp-ne s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) != (s_2_3));
        // N s_2_5: assert s_2_4
        let s_2_5: () = assert!(s_2_4);
        // C s_2_6: const #1s : i
        let s_2_6: i128 = 1;
        // D s_2_7: read-var cond:u8
        let s_2_7: u8 = fn_state.cond;
        // D s_2_8: cast zx s_2_7 -> bv
        let s_2_8: Bits = Bits::new(s_2_7 as u128, 4u16);
        // C s_2_9: const #1s : i64
        let s_2_9: i64 = 1;
        // C s_2_10: cast zx s_2_9 -> i
        let s_2_10: i128 = (i128::try_from(s_2_9).unwrap());
        // C s_2_11: const #2s : i
        let s_2_11: i128 = 2;
        // C s_2_12: add s_2_11 s_2_10
        let s_2_12: i128 = (s_2_11 + s_2_10);
        // D s_2_13: bit-extract s_2_8 s_2_6 s_2_12
        let s_2_13: Bits = (Bits::new(
            ((s_2_8) >> (s_2_6)).value(),
            u16::try_from(s_2_12).unwrap(),
        ));
        // D s_2_14: cast reint s_2_13 -> u8
        let s_2_14: u8 = (s_2_13.value() as u8);
        // D s_2_15: cast zx s_2_14 -> bv
        let s_2_15: Bits = Bits::new(s_2_14 as u128, 3u16);
        // C s_2_16: const #7u : u8
        let s_2_16: u8 = 7;
        // C s_2_17: cast zx s_2_16 -> bv
        let s_2_17: Bits = Bits::new(s_2_16 as u128, 3u16);
        // D s_2_18: cmp-eq s_2_15 s_2_17
        let s_2_18: bool = ((s_2_15) == (s_2_17));
        // N s_2_19: branch s_2_18 b6 b3
        if s_2_18 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var S:u8
        let s_3_0: bool = fn_state.S;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 1u16);
        // D s_3_2: read-var J2:u8
        let s_3_2: bool = fn_state.J2;
        // D s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 1u16);
        // D s_3_4: cast reint s_3_1 -> u128
        let s_3_4: u128 = (s_3_1.value() as u128);
        // D s_3_5: size-of s_3_1
        let s_3_5: u16 = s_3_1.length();
        // D s_3_6: cast reint s_3_3 -> u128
        let s_3_6: u128 = (s_3_3.value() as u128);
        // D s_3_7: size-of s_3_3
        let s_3_7: u16 = s_3_3.length();
        // D s_3_8: lsl s_3_4 s_3_7
        let s_3_8: u128 = s_3_4 << s_3_7;
        // D s_3_9: or s_3_8 s_3_6
        let s_3_9: u128 = ((s_3_8) | (s_3_6));
        // D s_3_10: add s_3_5 s_3_7
        let s_3_10: u16 = (s_3_5 + s_3_7);
        // D s_3_11: create-bits s_3_9 s_3_10
        let s_3_11: Bits = Bits::new(s_3_9, s_3_10);
        // D s_3_12: cast reint s_3_11 -> u8
        let s_3_12: u8 = (s_3_11.value() as u8);
        // D s_3_13: cast zx s_3_12 -> bv
        let s_3_13: Bits = Bits::new(s_3_12 as u128, 2u16);
        // D s_3_14: read-var J1:u8
        let s_3_14: bool = fn_state.J1;
        // D s_3_15: cast zx s_3_14 -> bv
        let s_3_15: Bits = Bits::new(s_3_14 as u128, 1u16);
        // D s_3_16: cast reint s_3_13 -> u128
        let s_3_16: u128 = (s_3_13.value() as u128);
        // D s_3_17: size-of s_3_13
        let s_3_17: u16 = s_3_13.length();
        // D s_3_18: cast reint s_3_15 -> u128
        let s_3_18: u128 = (s_3_15.value() as u128);
        // D s_3_19: size-of s_3_15
        let s_3_19: u16 = s_3_15.length();
        // D s_3_20: lsl s_3_16 s_3_19
        let s_3_20: u128 = s_3_16 << s_3_19;
        // D s_3_21: or s_3_20 s_3_18
        let s_3_21: u128 = ((s_3_20) | (s_3_18));
        // D s_3_22: add s_3_17 s_3_19
        let s_3_22: u16 = (s_3_17 + s_3_19);
        // D s_3_23: create-bits s_3_21 s_3_22
        let s_3_23: Bits = Bits::new(s_3_21, s_3_22);
        // D s_3_24: cast reint s_3_23 -> u8
        let s_3_24: u8 = (s_3_23.value() as u8);
        // D s_3_25: cast zx s_3_24 -> bv
        let s_3_25: Bits = Bits::new(s_3_24 as u128, 3u16);
        // D s_3_26: read-var imm6:u8
        let s_3_26: u8 = fn_state.imm6;
        // D s_3_27: cast zx s_3_26 -> bv
        let s_3_27: Bits = Bits::new(s_3_26 as u128, 6u16);
        // D s_3_28: cast reint s_3_25 -> u128
        let s_3_28: u128 = (s_3_25.value() as u128);
        // D s_3_29: size-of s_3_25
        let s_3_29: u16 = s_3_25.length();
        // D s_3_30: cast reint s_3_27 -> u128
        let s_3_30: u128 = (s_3_27.value() as u128);
        // D s_3_31: size-of s_3_27
        let s_3_31: u16 = s_3_27.length();
        // D s_3_32: lsl s_3_28 s_3_31
        let s_3_32: u128 = s_3_28 << s_3_31;
        // D s_3_33: or s_3_32 s_3_30
        let s_3_33: u128 = ((s_3_32) | (s_3_30));
        // D s_3_34: add s_3_29 s_3_31
        let s_3_34: u16 = (s_3_29 + s_3_31);
        // D s_3_35: create-bits s_3_33 s_3_34
        let s_3_35: Bits = Bits::new(s_3_33, s_3_34);
        // D s_3_36: cast reint s_3_35 -> u9
        let s_3_36: u16 = (s_3_35.value() as u16);
        // D s_3_37: cast zx s_3_36 -> bv
        let s_3_37: Bits = Bits::new(s_3_36 as u128, 9u16);
        // D s_3_38: read-var imm11:u11
        let s_3_38: u16 = fn_state.imm11;
        // D s_3_39: cast zx s_3_38 -> bv
        let s_3_39: Bits = Bits::new(s_3_38 as u128, 11u16);
        // D s_3_40: cast reint s_3_37 -> u128
        let s_3_40: u128 = (s_3_37.value() as u128);
        // D s_3_41: size-of s_3_37
        let s_3_41: u16 = s_3_37.length();
        // D s_3_42: cast reint s_3_39 -> u128
        let s_3_42: u128 = (s_3_39.value() as u128);
        // D s_3_43: size-of s_3_39
        let s_3_43: u16 = s_3_39.length();
        // D s_3_44: lsl s_3_40 s_3_43
        let s_3_44: u128 = s_3_40 << s_3_43;
        // D s_3_45: or s_3_44 s_3_42
        let s_3_45: u128 = ((s_3_44) | (s_3_42));
        // D s_3_46: add s_3_41 s_3_43
        let s_3_46: u16 = (s_3_41 + s_3_43);
        // D s_3_47: create-bits s_3_45 s_3_46
        let s_3_47: Bits = Bits::new(s_3_45, s_3_46);
        // D s_3_48: cast reint s_3_47 -> u20
        let s_3_48: u32 = (s_3_47.value() as u32);
        // D s_3_49: cast zx s_3_48 -> bv
        let s_3_49: Bits = Bits::new(s_3_48 as u128, 20u16);
        // C s_3_50: const #0u : u8
        let s_3_50: bool = false;
        // C s_3_51: cast zx s_3_50 -> bv
        let s_3_51: Bits = Bits::new(s_3_50 as u128, 1u16);
        // D s_3_52: cast reint s_3_49 -> u128
        let s_3_52: u128 = (s_3_49.value() as u128);
        // D s_3_53: size-of s_3_49
        let s_3_53: u16 = s_3_49.length();
        // C s_3_54: cast reint s_3_51 -> u128
        let s_3_54: u128 = (s_3_51.value() as u128);
        // D s_3_55: size-of s_3_51
        let s_3_55: u16 = s_3_51.length();
        // D s_3_56: lsl s_3_52 s_3_55
        let s_3_56: u128 = s_3_52 << s_3_55;
        // D s_3_57: or s_3_56 s_3_54
        let s_3_57: u128 = ((s_3_56) | (s_3_54));
        // D s_3_58: add s_3_53 s_3_55
        let s_3_58: u16 = (s_3_53 + s_3_55);
        // D s_3_59: create-bits s_3_57 s_3_58
        let s_3_59: Bits = Bits::new(s_3_57, s_3_58);
        // D s_3_60: cast reint s_3_59 -> u21
        let s_3_60: u32 = (s_3_59.value() as u32);
        // C s_3_61: const #32s : i
        let s_3_61: i128 = 32;
        // D s_3_62: cast zx s_3_60 -> bv
        let s_3_62: Bits = Bits::new(s_3_60 as u128, 21u16);
        // D s_3_63: bits-cast sx s_3_62 -> bv length s_3_61
        let s_3_63: Bits = s_3_62.sign_extend(s_3_61);
        // D s_3_64: cast reint s_3_63 -> u32
        let s_3_64: u32 = (s_3_63.value() as u32);
        // D s_3_65: write-var imm32 <= s_3_64
        fn_state.imm32 = s_3_64;
        // C s_3_66: const #() : ()
        let s_3_66: () = ();
        // S s_3_67: call InITBlock(s_3_66)
        let s_3_67: bool = InITBlock(state, tracer, s_3_66);
        // N s_3_68: branch s_3_67 b5 b4
        if s_3_67 {
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
        // D s_4_0: read-var imm32:u32
        let s_4_0: u32 = fn_state.imm32;
        // D s_4_1: call execute_aarch32_instrs_B_Op_A_txt(s_4_0)
        let s_4_1: () = execute_aarch32_instrs_B_Op_A_txt(state, tracer, s_4_0);
        // N s_4_2: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: panic
        panic!("{:?}", ());
        // N s_5_1: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: panic
        panic!("{:?}", ());
        // N s_6_1: return
        return;
    }
}
