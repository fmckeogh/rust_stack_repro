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
pub fn decode_aarch32_instrs_BL_i_T2enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    S: bool,
    imm10H: u16,
    J1: bool,
    J2: bool,
    imm10L: u16,
    H: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        imm32: u32,
        gs_296167: bool,
        S: bool,
        imm10H: u16,
        J1: bool,
        J2: bool,
        imm10L: u16,
        H: bool,
    }
    let fn_state = FunctionState {
        S,
        imm10H,
        J1,
        J2,
        imm10L,
        H,
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
        // D s_2_0: read-var H:u8
        let s_2_0: bool = fn_state.H;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // C s_2_2: const #1u : u8
        let s_2_2: bool = true;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b9 b3
        if s_2_4 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var J1:u8
        let s_3_0: bool = fn_state.J1;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 1u16);
        // D s_3_2: read-var S:u8
        let s_3_2: bool = fn_state.S;
        // D s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 1u16);
        // D s_3_4: xor s_3_1 s_3_3
        let s_3_4: Bits = ((s_3_1) ^ (s_3_3));
        // D s_3_5: cast reint s_3_4 -> u8
        let s_3_5: bool = ((s_3_4.value()) != 0);
        // D s_3_6: cast zx s_3_5 -> bv
        let s_3_6: Bits = Bits::new(s_3_5 as u128, 1u16);
        // D s_3_7: not s_3_6
        let s_3_7: Bits = !s_3_6;
        // D s_3_8: cast reint s_3_7 -> u8
        let s_3_8: bool = ((s_3_7.value()) != 0);
        // D s_3_9: read-var J2:u8
        let s_3_9: bool = fn_state.J2;
        // D s_3_10: cast zx s_3_9 -> bv
        let s_3_10: Bits = Bits::new(s_3_9 as u128, 1u16);
        // D s_3_11: read-var S:u8
        let s_3_11: bool = fn_state.S;
        // D s_3_12: cast zx s_3_11 -> bv
        let s_3_12: Bits = Bits::new(s_3_11 as u128, 1u16);
        // D s_3_13: xor s_3_10 s_3_12
        let s_3_13: Bits = ((s_3_10) ^ (s_3_12));
        // D s_3_14: cast reint s_3_13 -> u8
        let s_3_14: bool = ((s_3_13.value()) != 0);
        // D s_3_15: cast zx s_3_14 -> bv
        let s_3_15: Bits = Bits::new(s_3_14 as u128, 1u16);
        // D s_3_16: not s_3_15
        let s_3_16: Bits = !s_3_15;
        // D s_3_17: cast reint s_3_16 -> u8
        let s_3_17: bool = ((s_3_16.value()) != 0);
        // D s_3_18: read-var S:u8
        let s_3_18: bool = fn_state.S;
        // D s_3_19: cast zx s_3_18 -> bv
        let s_3_19: Bits = Bits::new(s_3_18 as u128, 1u16);
        // D s_3_20: cast zx s_3_8 -> bv
        let s_3_20: Bits = Bits::new(s_3_8 as u128, 1u16);
        // D s_3_21: cast reint s_3_19 -> u128
        let s_3_21: u128 = (s_3_19.value() as u128);
        // D s_3_22: size-of s_3_19
        let s_3_22: u16 = s_3_19.length();
        // D s_3_23: cast reint s_3_20 -> u128
        let s_3_23: u128 = (s_3_20.value() as u128);
        // D s_3_24: size-of s_3_20
        let s_3_24: u16 = s_3_20.length();
        // D s_3_25: lsl s_3_21 s_3_24
        let s_3_25: u128 = s_3_21 << s_3_24;
        // D s_3_26: or s_3_25 s_3_23
        let s_3_26: u128 = ((s_3_25) | (s_3_23));
        // D s_3_27: add s_3_22 s_3_24
        let s_3_27: u16 = (s_3_22 + s_3_24);
        // D s_3_28: create-bits s_3_26 s_3_27
        let s_3_28: Bits = Bits::new(s_3_26, s_3_27);
        // D s_3_29: cast reint s_3_28 -> u8
        let s_3_29: u8 = (s_3_28.value() as u8);
        // D s_3_30: cast zx s_3_29 -> bv
        let s_3_30: Bits = Bits::new(s_3_29 as u128, 2u16);
        // D s_3_31: cast zx s_3_17 -> bv
        let s_3_31: Bits = Bits::new(s_3_17 as u128, 1u16);
        // D s_3_32: cast reint s_3_30 -> u128
        let s_3_32: u128 = (s_3_30.value() as u128);
        // D s_3_33: size-of s_3_30
        let s_3_33: u16 = s_3_30.length();
        // D s_3_34: cast reint s_3_31 -> u128
        let s_3_34: u128 = (s_3_31.value() as u128);
        // D s_3_35: size-of s_3_31
        let s_3_35: u16 = s_3_31.length();
        // D s_3_36: lsl s_3_32 s_3_35
        let s_3_36: u128 = s_3_32 << s_3_35;
        // D s_3_37: or s_3_36 s_3_34
        let s_3_37: u128 = ((s_3_36) | (s_3_34));
        // D s_3_38: add s_3_33 s_3_35
        let s_3_38: u16 = (s_3_33 + s_3_35);
        // D s_3_39: create-bits s_3_37 s_3_38
        let s_3_39: Bits = Bits::new(s_3_37, s_3_38);
        // D s_3_40: cast reint s_3_39 -> u8
        let s_3_40: u8 = (s_3_39.value() as u8);
        // D s_3_41: cast zx s_3_40 -> bv
        let s_3_41: Bits = Bits::new(s_3_40 as u128, 3u16);
        // D s_3_42: read-var imm10H:u10
        let s_3_42: u16 = fn_state.imm10H;
        // D s_3_43: cast zx s_3_42 -> bv
        let s_3_43: Bits = Bits::new(s_3_42 as u128, 10u16);
        // D s_3_44: cast reint s_3_41 -> u128
        let s_3_44: u128 = (s_3_41.value() as u128);
        // D s_3_45: size-of s_3_41
        let s_3_45: u16 = s_3_41.length();
        // D s_3_46: cast reint s_3_43 -> u128
        let s_3_46: u128 = (s_3_43.value() as u128);
        // D s_3_47: size-of s_3_43
        let s_3_47: u16 = s_3_43.length();
        // D s_3_48: lsl s_3_44 s_3_47
        let s_3_48: u128 = s_3_44 << s_3_47;
        // D s_3_49: or s_3_48 s_3_46
        let s_3_49: u128 = ((s_3_48) | (s_3_46));
        // D s_3_50: add s_3_45 s_3_47
        let s_3_50: u16 = (s_3_45 + s_3_47);
        // D s_3_51: create-bits s_3_49 s_3_50
        let s_3_51: Bits = Bits::new(s_3_49, s_3_50);
        // D s_3_52: cast reint s_3_51 -> u13
        let s_3_52: u16 = (s_3_51.value() as u16);
        // D s_3_53: cast zx s_3_52 -> bv
        let s_3_53: Bits = Bits::new(s_3_52 as u128, 13u16);
        // D s_3_54: read-var imm10L:u10
        let s_3_54: u16 = fn_state.imm10L;
        // D s_3_55: cast zx s_3_54 -> bv
        let s_3_55: Bits = Bits::new(s_3_54 as u128, 10u16);
        // D s_3_56: cast reint s_3_53 -> u128
        let s_3_56: u128 = (s_3_53.value() as u128);
        // D s_3_57: size-of s_3_53
        let s_3_57: u16 = s_3_53.length();
        // D s_3_58: cast reint s_3_55 -> u128
        let s_3_58: u128 = (s_3_55.value() as u128);
        // D s_3_59: size-of s_3_55
        let s_3_59: u16 = s_3_55.length();
        // D s_3_60: lsl s_3_56 s_3_59
        let s_3_60: u128 = s_3_56 << s_3_59;
        // D s_3_61: or s_3_60 s_3_58
        let s_3_61: u128 = ((s_3_60) | (s_3_58));
        // D s_3_62: add s_3_57 s_3_59
        let s_3_62: u16 = (s_3_57 + s_3_59);
        // D s_3_63: create-bits s_3_61 s_3_62
        let s_3_63: Bits = Bits::new(s_3_61, s_3_62);
        // D s_3_64: cast reint s_3_63 -> u23
        let s_3_64: u32 = (s_3_63.value() as u32);
        // D s_3_65: cast zx s_3_64 -> bv
        let s_3_65: Bits = Bits::new(s_3_64 as u128, 23u16);
        // C s_3_66: const #0u : u8
        let s_3_66: u8 = 0;
        // C s_3_67: cast zx s_3_66 -> bv
        let s_3_67: Bits = Bits::new(s_3_66 as u128, 2u16);
        // D s_3_68: cast reint s_3_65 -> u128
        let s_3_68: u128 = (s_3_65.value() as u128);
        // D s_3_69: size-of s_3_65
        let s_3_69: u16 = s_3_65.length();
        // C s_3_70: cast reint s_3_67 -> u128
        let s_3_70: u128 = (s_3_67.value() as u128);
        // D s_3_71: size-of s_3_67
        let s_3_71: u16 = s_3_67.length();
        // D s_3_72: lsl s_3_68 s_3_71
        let s_3_72: u128 = s_3_68 << s_3_71;
        // D s_3_73: or s_3_72 s_3_70
        let s_3_73: u128 = ((s_3_72) | (s_3_70));
        // D s_3_74: add s_3_69 s_3_71
        let s_3_74: u16 = (s_3_69 + s_3_71);
        // D s_3_75: create-bits s_3_73 s_3_74
        let s_3_75: Bits = Bits::new(s_3_73, s_3_74);
        // D s_3_76: cast reint s_3_75 -> u25
        let s_3_76: u32 = (s_3_75.value() as u32);
        // C s_3_77: const #32s : i
        let s_3_77: i128 = 32;
        // D s_3_78: cast zx s_3_76 -> bv
        let s_3_78: Bits = Bits::new(s_3_76 as u128, 25u16);
        // D s_3_79: bits-cast sx s_3_78 -> bv length s_3_77
        let s_3_79: Bits = s_3_78.sign_extend(s_3_77);
        // D s_3_80: cast reint s_3_79 -> u32
        let s_3_80: u32 = (s_3_79.value() as u32);
        // D s_3_81: write-var imm32 <= s_3_80
        fn_state.imm32 = s_3_80;
        // C s_3_82: const #() : ()
        let s_3_82: () = ();
        // S s_3_83: call InITBlock(s_3_82)
        let s_3_83: bool = InITBlock(state, tracer, s_3_82);
        // N s_3_84: branch s_3_83 b8 b4
        if s_3_83 {
            return block_8(state, tracer, fn_state);
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
        // D s_4_1: write-var gs#296167 <= s_4_0
        fn_state.gs_296167 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#296167:u8
        let s_5_0: bool = fn_state.gs_296167;
        // N s_5_1: branch s_5_0 b7 b6
        if s_5_0 {
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
        // D s_6_0: read-var imm32:u32
        let s_6_0: u32 = fn_state.imm32;
        // C s_6_1: const #1u : u32
        let s_6_1: u32 = 1;
        // D s_6_2: call execute_aarch32_instrs_BL_i_Op_A_txt(s_6_0, s_6_1)
        let s_6_2: () = execute_aarch32_instrs_BL_i_Op_A_txt(
            state,
            tracer,
            s_6_0,
            s_6_1,
        );
        // N s_6_3: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: panic
        panic!("{:?}", ());
        // N s_7_1: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call LastInITBlock(s_8_0)
        let s_8_1: bool = LastInITBlock(state, tracer, s_8_0);
        // S s_8_2: not s_8_1
        let s_8_2: bool = !s_8_1;
        // D s_8_3: write-var gs#296167 <= s_8_2
        fn_state.gs_296167 = s_8_2;
        // N s_8_4: jump b5
        return block_5(state, tracer, fn_state);
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
}
