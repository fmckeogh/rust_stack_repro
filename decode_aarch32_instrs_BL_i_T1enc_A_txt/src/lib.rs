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
use LastInITBlock::*;
use ConditionPassed::*;
use InITBlock::*;
use execute_aarch32_instrs_BL_i_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_BL_i_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    S: bool,
    imm10: u16,
    J1: bool,
    J2: bool,
    imm11: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        imm32: u32,
        gs_296162: bool,
        S: bool,
        imm10: u16,
        J1: bool,
        J2: bool,
        imm11: u16,
    }
    let fn_state = FunctionState {
        S,
        imm10,
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
        // D s_2_0: read-var J1:u8
        let s_2_0: bool = fn_state.J1;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // D s_2_2: read-var S:u8
        let s_2_2: bool = fn_state.S;
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // D s_2_4: xor s_2_1 s_2_3
        let s_2_4: Bits = ((s_2_1) ^ (s_2_3));
        // D s_2_5: cast reint s_2_4 -> u8
        let s_2_5: bool = ((s_2_4.value()) != 0);
        // D s_2_6: cast zx s_2_5 -> bv
        let s_2_6: Bits = Bits::new(s_2_5 as u128, 1u16);
        // D s_2_7: not s_2_6
        let s_2_7: Bits = !s_2_6;
        // D s_2_8: cast reint s_2_7 -> u8
        let s_2_8: bool = ((s_2_7.value()) != 0);
        // D s_2_9: read-var J2:u8
        let s_2_9: bool = fn_state.J2;
        // D s_2_10: cast zx s_2_9 -> bv
        let s_2_10: Bits = Bits::new(s_2_9 as u128, 1u16);
        // D s_2_11: read-var S:u8
        let s_2_11: bool = fn_state.S;
        // D s_2_12: cast zx s_2_11 -> bv
        let s_2_12: Bits = Bits::new(s_2_11 as u128, 1u16);
        // D s_2_13: xor s_2_10 s_2_12
        let s_2_13: Bits = ((s_2_10) ^ (s_2_12));
        // D s_2_14: cast reint s_2_13 -> u8
        let s_2_14: bool = ((s_2_13.value()) != 0);
        // D s_2_15: cast zx s_2_14 -> bv
        let s_2_15: Bits = Bits::new(s_2_14 as u128, 1u16);
        // D s_2_16: not s_2_15
        let s_2_16: Bits = !s_2_15;
        // D s_2_17: cast reint s_2_16 -> u8
        let s_2_17: bool = ((s_2_16.value()) != 0);
        // D s_2_18: read-var S:u8
        let s_2_18: bool = fn_state.S;
        // D s_2_19: cast zx s_2_18 -> bv
        let s_2_19: Bits = Bits::new(s_2_18 as u128, 1u16);
        // D s_2_20: cast zx s_2_8 -> bv
        let s_2_20: Bits = Bits::new(s_2_8 as u128, 1u16);
        // D s_2_21: cast reint s_2_19 -> u128
        let s_2_21: u128 = (s_2_19.value() as u128);
        // D s_2_22: size-of s_2_19
        let s_2_22: u16 = s_2_19.length();
        // D s_2_23: cast reint s_2_20 -> u128
        let s_2_23: u128 = (s_2_20.value() as u128);
        // D s_2_24: size-of s_2_20
        let s_2_24: u16 = s_2_20.length();
        // D s_2_25: lsl s_2_21 s_2_24
        let s_2_25: u128 = s_2_21 << s_2_24;
        // D s_2_26: or s_2_25 s_2_23
        let s_2_26: u128 = ((s_2_25) | (s_2_23));
        // D s_2_27: add s_2_22 s_2_24
        let s_2_27: u16 = (s_2_22 + s_2_24);
        // D s_2_28: create-bits s_2_26 s_2_27
        let s_2_28: Bits = Bits::new(s_2_26, s_2_27);
        // D s_2_29: cast reint s_2_28 -> u8
        let s_2_29: u8 = (s_2_28.value() as u8);
        // D s_2_30: cast zx s_2_29 -> bv
        let s_2_30: Bits = Bits::new(s_2_29 as u128, 2u16);
        // D s_2_31: cast zx s_2_17 -> bv
        let s_2_31: Bits = Bits::new(s_2_17 as u128, 1u16);
        // D s_2_32: cast reint s_2_30 -> u128
        let s_2_32: u128 = (s_2_30.value() as u128);
        // D s_2_33: size-of s_2_30
        let s_2_33: u16 = s_2_30.length();
        // D s_2_34: cast reint s_2_31 -> u128
        let s_2_34: u128 = (s_2_31.value() as u128);
        // D s_2_35: size-of s_2_31
        let s_2_35: u16 = s_2_31.length();
        // D s_2_36: lsl s_2_32 s_2_35
        let s_2_36: u128 = s_2_32 << s_2_35;
        // D s_2_37: or s_2_36 s_2_34
        let s_2_37: u128 = ((s_2_36) | (s_2_34));
        // D s_2_38: add s_2_33 s_2_35
        let s_2_38: u16 = (s_2_33 + s_2_35);
        // D s_2_39: create-bits s_2_37 s_2_38
        let s_2_39: Bits = Bits::new(s_2_37, s_2_38);
        // D s_2_40: cast reint s_2_39 -> u8
        let s_2_40: u8 = (s_2_39.value() as u8);
        // D s_2_41: cast zx s_2_40 -> bv
        let s_2_41: Bits = Bits::new(s_2_40 as u128, 3u16);
        // D s_2_42: read-var imm10:u10
        let s_2_42: u16 = fn_state.imm10;
        // D s_2_43: cast zx s_2_42 -> bv
        let s_2_43: Bits = Bits::new(s_2_42 as u128, 10u16);
        // D s_2_44: cast reint s_2_41 -> u128
        let s_2_44: u128 = (s_2_41.value() as u128);
        // D s_2_45: size-of s_2_41
        let s_2_45: u16 = s_2_41.length();
        // D s_2_46: cast reint s_2_43 -> u128
        let s_2_46: u128 = (s_2_43.value() as u128);
        // D s_2_47: size-of s_2_43
        let s_2_47: u16 = s_2_43.length();
        // D s_2_48: lsl s_2_44 s_2_47
        let s_2_48: u128 = s_2_44 << s_2_47;
        // D s_2_49: or s_2_48 s_2_46
        let s_2_49: u128 = ((s_2_48) | (s_2_46));
        // D s_2_50: add s_2_45 s_2_47
        let s_2_50: u16 = (s_2_45 + s_2_47);
        // D s_2_51: create-bits s_2_49 s_2_50
        let s_2_51: Bits = Bits::new(s_2_49, s_2_50);
        // D s_2_52: cast reint s_2_51 -> u13
        let s_2_52: u16 = (s_2_51.value() as u16);
        // D s_2_53: cast zx s_2_52 -> bv
        let s_2_53: Bits = Bits::new(s_2_52 as u128, 13u16);
        // D s_2_54: read-var imm11:u11
        let s_2_54: u16 = fn_state.imm11;
        // D s_2_55: cast zx s_2_54 -> bv
        let s_2_55: Bits = Bits::new(s_2_54 as u128, 11u16);
        // D s_2_56: cast reint s_2_53 -> u128
        let s_2_56: u128 = (s_2_53.value() as u128);
        // D s_2_57: size-of s_2_53
        let s_2_57: u16 = s_2_53.length();
        // D s_2_58: cast reint s_2_55 -> u128
        let s_2_58: u128 = (s_2_55.value() as u128);
        // D s_2_59: size-of s_2_55
        let s_2_59: u16 = s_2_55.length();
        // D s_2_60: lsl s_2_56 s_2_59
        let s_2_60: u128 = s_2_56 << s_2_59;
        // D s_2_61: or s_2_60 s_2_58
        let s_2_61: u128 = ((s_2_60) | (s_2_58));
        // D s_2_62: add s_2_57 s_2_59
        let s_2_62: u16 = (s_2_57 + s_2_59);
        // D s_2_63: create-bits s_2_61 s_2_62
        let s_2_63: Bits = Bits::new(s_2_61, s_2_62);
        // D s_2_64: cast reint s_2_63 -> u24
        let s_2_64: u32 = (s_2_63.value() as u32);
        // D s_2_65: cast zx s_2_64 -> bv
        let s_2_65: Bits = Bits::new(s_2_64 as u128, 24u16);
        // C s_2_66: const #0u : u8
        let s_2_66: bool = false;
        // C s_2_67: cast zx s_2_66 -> bv
        let s_2_67: Bits = Bits::new(s_2_66 as u128, 1u16);
        // D s_2_68: cast reint s_2_65 -> u128
        let s_2_68: u128 = (s_2_65.value() as u128);
        // D s_2_69: size-of s_2_65
        let s_2_69: u16 = s_2_65.length();
        // C s_2_70: cast reint s_2_67 -> u128
        let s_2_70: u128 = (s_2_67.value() as u128);
        // D s_2_71: size-of s_2_67
        let s_2_71: u16 = s_2_67.length();
        // D s_2_72: lsl s_2_68 s_2_71
        let s_2_72: u128 = s_2_68 << s_2_71;
        // D s_2_73: or s_2_72 s_2_70
        let s_2_73: u128 = ((s_2_72) | (s_2_70));
        // D s_2_74: add s_2_69 s_2_71
        let s_2_74: u16 = (s_2_69 + s_2_71);
        // D s_2_75: create-bits s_2_73 s_2_74
        let s_2_75: Bits = Bits::new(s_2_73, s_2_74);
        // D s_2_76: cast reint s_2_75 -> u25
        let s_2_76: u32 = (s_2_75.value() as u32);
        // C s_2_77: const #32s : i
        let s_2_77: i128 = 32;
        // D s_2_78: cast zx s_2_76 -> bv
        let s_2_78: Bits = Bits::new(s_2_76 as u128, 25u16);
        // D s_2_79: bits-cast sx s_2_78 -> bv length s_2_77
        let s_2_79: Bits = s_2_78.sign_extend(s_2_77);
        // D s_2_80: cast reint s_2_79 -> u32
        let s_2_80: u32 = (s_2_79.value() as u32);
        // D s_2_81: write-var imm32 <= s_2_80
        fn_state.imm32 = s_2_80;
        // C s_2_82: const #() : ()
        let s_2_82: () = ();
        // S s_2_83: call InITBlock(s_2_82)
        let s_2_83: bool = InITBlock(state, tracer, s_2_82);
        // N s_2_84: branch s_2_83 b7 b3
        if s_2_83 {
            return block_7(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#296162 <= s_3_0
        fn_state.gs_296162 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#296162:u8
        let s_4_0: bool = fn_state.gs_296162;
        // N s_4_1: branch s_4_0 b6 b5
        if s_4_0 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var imm32:u32
        let s_5_0: u32 = fn_state.imm32;
        // C s_5_1: const #2u : u32
        let s_5_1: u32 = 2;
        // D s_5_2: call execute_aarch32_instrs_BL_i_Op_A_txt(s_5_0, s_5_1)
        let s_5_2: () = execute_aarch32_instrs_BL_i_Op_A_txt(
            state,
            tracer,
            s_5_0,
            s_5_1,
        );
        // N s_5_3: return
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
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call LastInITBlock(s_7_0)
        let s_7_1: bool = LastInITBlock(state, tracer, s_7_0);
        // S s_7_2: not s_7_1
        let s_7_2: bool = !s_7_1;
        // D s_7_3: write-var gs#296162 <= s_7_2
        fn_state.gs_296162 = s_7_2;
        // N s_7_4: jump b4
        return block_4(state, tracer, fn_state);
    }
}
