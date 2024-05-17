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
use AArch64_SystemAccessTrap::*;
use IsHCRXEL2Enabled::*;
use u_get_HCRX_EL2_Type_TALLINT::*;
use CheckSMEAccess::*;
use SetPSTATE_SM::*;
use SetPSTATE_ZA::*;
use common::*;
pub fn execute_aarch64_instrs_system_register_cpsr<T: Tracer>(
    state: &mut State,
    tracer: &T,
    field: u32,
    operand: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_165326: bool,
        gs_165327: bool,
        gs_165330: bool,
        field: u32,
        operand: u8,
    }
    let fn_state = FunctionState {
        field,
        operand,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #5u : u32
        let s_0_0: u32 = 5;
        // D s_0_1: read-var field:u32
        let s_0_1: u32 = fn_state.field;
        // D s_0_2: cmp-eq s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) == (s_0_1));
        // D s_0_3: not s_0_2
        let s_0_3: bool = !s_0_2;
        // N s_0_4: branch s_0_3 b2 b1
        if s_0_3 {
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
        // C s_1_0: const #0s : i
        let s_1_0: i128 = 0;
        // D s_1_1: read-var operand:u8
        let s_1_1: u8 = fn_state.operand;
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 4u16);
        // C s_1_3: const #1u : u64
        let s_1_3: u64 = 1;
        // D s_1_4: bit-extract s_1_2 s_1_0 s_1_3
        let s_1_4: Bits = (Bits::new(
            ((s_1_2) >> (s_1_0)).value(),
            u16::try_from(s_1_3).unwrap(),
        ));
        // D s_1_5: cast reint s_1_4 -> u8
        let s_1_5: bool = ((s_1_4.value()) != 0);
        // C s_1_6: const #0s : i
        let s_1_6: i128 = 0;
        // C s_1_7: const #0u : u64
        let s_1_7: u64 = 0;
        // D s_1_8: cast zx s_1_5 -> u64
        let s_1_8: u64 = (s_1_5 as u64);
        // C s_1_9: const #1u : u64
        let s_1_9: u64 = 1;
        // D s_1_10: and s_1_8 s_1_9
        let s_1_10: u64 = ((s_1_8) & (s_1_9));
        // D s_1_11: cmp-eq s_1_10 s_1_9
        let s_1_11: bool = ((s_1_10) == (s_1_9));
        // D s_1_12: lsl s_1_8 s_1_6
        let s_1_12: u64 = s_1_8 << s_1_6;
        // D s_1_13: or s_1_7 s_1_12
        let s_1_13: u64 = ((s_1_7) | (s_1_12));
        // D s_1_14: cmpl s_1_12
        let s_1_14: u64 = !s_1_12;
        // D s_1_15: and s_1_7 s_1_14
        let s_1_15: u64 = ((s_1_7) & (s_1_14));
        // D s_1_16: select s_1_11 s_1_13 s_1_15
        let s_1_16: u64 = if s_1_11 { s_1_13 } else { s_1_15 };
        // D s_1_17: cast trunc s_1_16 -> u8
        let s_1_17: bool = ((s_1_16) != 0);
        // C s_1_18: const #16992u : u32
        let s_1_18: u32 = 16992;
        // N s_1_19: write-reg s_1_18 <= s_1_17
        let s_1_19: () = {
            state.write_register::<bool>(s_1_18 as isize, s_1_17);
            tracer.write_register(s_1_18 as isize, s_1_17);
        };
        // N s_1_20: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #12u : u32
        let s_2_0: u32 = 12;
        // D s_2_1: read-var field:u32
        let s_2_1: u32 = fn_state.field;
        // D s_2_2: cmp-eq s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) == (s_2_1));
        // D s_2_3: not s_2_2
        let s_2_3: bool = !s_2_2;
        // N s_2_4: branch s_2_3 b4 b3
        if s_2_3 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0s : i
        let s_3_0: i128 = 0;
        // D s_3_1: read-var operand:u8
        let s_3_1: u8 = fn_state.operand;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 4u16);
        // C s_3_3: const #1u : u64
        let s_3_3: u64 = 1;
        // D s_3_4: bit-extract s_3_2 s_3_0 s_3_3
        let s_3_4: Bits = (Bits::new(
            ((s_3_2) >> (s_3_0)).value(),
            u16::try_from(s_3_3).unwrap(),
        ));
        // D s_3_5: cast reint s_3_4 -> u8
        let s_3_5: bool = ((s_3_4.value()) != 0);
        // C s_3_6: const #0s : i
        let s_3_6: i128 = 0;
        // C s_3_7: const #0u : u64
        let s_3_7: u64 = 0;
        // D s_3_8: cast zx s_3_5 -> u64
        let s_3_8: u64 = (s_3_5 as u64);
        // C s_3_9: const #1u : u64
        let s_3_9: u64 = 1;
        // D s_3_10: and s_3_8 s_3_9
        let s_3_10: u64 = ((s_3_8) & (s_3_9));
        // D s_3_11: cmp-eq s_3_10 s_3_9
        let s_3_11: bool = ((s_3_10) == (s_3_9));
        // D s_3_12: lsl s_3_8 s_3_6
        let s_3_12: u64 = s_3_8 << s_3_6;
        // D s_3_13: or s_3_7 s_3_12
        let s_3_13: u64 = ((s_3_7) | (s_3_12));
        // D s_3_14: cmpl s_3_12
        let s_3_14: u64 = !s_3_12;
        // D s_3_15: and s_3_7 s_3_14
        let s_3_15: u64 = ((s_3_7) & (s_3_14));
        // D s_3_16: select s_3_11 s_3_13 s_3_15
        let s_3_16: u64 = if s_3_11 { s_3_13 } else { s_3_15 };
        // D s_3_17: cast trunc s_3_16 -> u8
        let s_3_17: bool = ((s_3_16) != 0);
        // C s_3_18: const #16990u : u32
        let s_3_18: u32 = 16990;
        // N s_3_19: write-reg s_3_18 <= s_3_17
        let s_3_19: () = {
            state.write_register::<bool>(s_3_18 as isize, s_3_17);
            tracer.write_register(s_3_18 as isize, s_3_17);
        };
        // N s_3_20: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u32
        let s_4_0: u32 = 0;
        // D s_4_1: read-var field:u32
        let s_4_1: u32 = fn_state.field;
        // D s_4_2: cmp-eq s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) == (s_4_1));
        // D s_4_3: not s_4_2
        let s_4_3: bool = !s_4_2;
        // N s_4_4: branch s_4_3 b6 b5
        if s_4_3 {
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
        // C s_5_0: const #16972u : u32
        let s_5_0: u32 = 16972;
        // D s_5_1: read-reg s_5_0:u8
        let s_5_1: bool = {
            let value = state.read_register::<bool>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // C s_5_2: const #3s : i
        let s_5_2: i128 = 3;
        // D s_5_3: read-var operand:u8
        let s_5_3: u8 = fn_state.operand;
        // D s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 4u16);
        // C s_5_5: const #1u : u64
        let s_5_5: u64 = 1;
        // D s_5_6: bit-extract s_5_4 s_5_2 s_5_5
        let s_5_6: Bits = (Bits::new(
            ((s_5_4) >> (s_5_2)).value(),
            u16::try_from(s_5_5).unwrap(),
        ));
        // D s_5_7: cast reint s_5_6 -> u8
        let s_5_7: bool = ((s_5_6.value()) != 0);
        // C s_5_8: const #0s : i
        let s_5_8: i128 = 0;
        // C s_5_9: const #0u : u64
        let s_5_9: u64 = 0;
        // D s_5_10: cast zx s_5_7 -> u64
        let s_5_10: u64 = (s_5_7 as u64);
        // C s_5_11: const #1u : u64
        let s_5_11: u64 = 1;
        // D s_5_12: and s_5_10 s_5_11
        let s_5_12: u64 = ((s_5_10) & (s_5_11));
        // D s_5_13: cmp-eq s_5_12 s_5_11
        let s_5_13: bool = ((s_5_12) == (s_5_11));
        // D s_5_14: lsl s_5_10 s_5_8
        let s_5_14: u64 = s_5_10 << s_5_8;
        // D s_5_15: or s_5_9 s_5_14
        let s_5_15: u64 = ((s_5_9) | (s_5_14));
        // D s_5_16: cmpl s_5_14
        let s_5_16: u64 = !s_5_14;
        // D s_5_17: and s_5_9 s_5_16
        let s_5_17: u64 = ((s_5_9) & (s_5_16));
        // D s_5_18: select s_5_13 s_5_15 s_5_17
        let s_5_18: u64 = if s_5_13 { s_5_15 } else { s_5_17 };
        // D s_5_19: cast trunc s_5_18 -> u8
        let s_5_19: bool = ((s_5_18) != 0);
        // D s_5_20: cast zx s_5_1 -> bv
        let s_5_20: Bits = Bits::new(s_5_1 as u128, 1u16);
        // D s_5_21: cast zx s_5_19 -> bv
        let s_5_21: Bits = Bits::new(s_5_19 as u128, 1u16);
        // D s_5_22: or s_5_20 s_5_21
        let s_5_22: Bits = ((s_5_20) | (s_5_21));
        // D s_5_23: cast reint s_5_22 -> u8
        let s_5_23: bool = ((s_5_22.value()) != 0);
        // C s_5_24: const #16972u : u32
        let s_5_24: u32 = 16972;
        // N s_5_25: write-reg s_5_24 <= s_5_23
        let s_5_25: () = {
            state.write_register::<bool>(s_5_24 as isize, s_5_23);
            tracer.write_register(s_5_24 as isize, s_5_23);
        };
        // C s_5_26: const #16968u : u32
        let s_5_26: u32 = 16968;
        // D s_5_27: read-reg s_5_26:u8
        let s_5_27: bool = {
            let value = state.read_register::<bool>(s_5_26 as isize);
            tracer.read_register(s_5_26 as isize, value);
            value
        };
        // C s_5_28: const #2s : i
        let s_5_28: i128 = 2;
        // D s_5_29: read-var operand:u8
        let s_5_29: u8 = fn_state.operand;
        // D s_5_30: cast zx s_5_29 -> bv
        let s_5_30: Bits = Bits::new(s_5_29 as u128, 4u16);
        // C s_5_31: const #1u : u64
        let s_5_31: u64 = 1;
        // D s_5_32: bit-extract s_5_30 s_5_28 s_5_31
        let s_5_32: Bits = (Bits::new(
            ((s_5_30) >> (s_5_28)).value(),
            u16::try_from(s_5_31).unwrap(),
        ));
        // D s_5_33: cast reint s_5_32 -> u8
        let s_5_33: bool = ((s_5_32.value()) != 0);
        // C s_5_34: const #0s : i
        let s_5_34: i128 = 0;
        // C s_5_35: const #0u : u64
        let s_5_35: u64 = 0;
        // D s_5_36: cast zx s_5_33 -> u64
        let s_5_36: u64 = (s_5_33 as u64);
        // C s_5_37: const #1u : u64
        let s_5_37: u64 = 1;
        // D s_5_38: and s_5_36 s_5_37
        let s_5_38: u64 = ((s_5_36) & (s_5_37));
        // D s_5_39: cmp-eq s_5_38 s_5_37
        let s_5_39: bool = ((s_5_38) == (s_5_37));
        // D s_5_40: lsl s_5_36 s_5_34
        let s_5_40: u64 = s_5_36 << s_5_34;
        // D s_5_41: or s_5_35 s_5_40
        let s_5_41: u64 = ((s_5_35) | (s_5_40));
        // D s_5_42: cmpl s_5_40
        let s_5_42: u64 = !s_5_40;
        // D s_5_43: and s_5_35 s_5_42
        let s_5_43: u64 = ((s_5_35) & (s_5_42));
        // D s_5_44: select s_5_39 s_5_41 s_5_43
        let s_5_44: u64 = if s_5_39 { s_5_41 } else { s_5_43 };
        // D s_5_45: cast trunc s_5_44 -> u8
        let s_5_45: bool = ((s_5_44) != 0);
        // D s_5_46: cast zx s_5_27 -> bv
        let s_5_46: Bits = Bits::new(s_5_27 as u128, 1u16);
        // D s_5_47: cast zx s_5_45 -> bv
        let s_5_47: Bits = Bits::new(s_5_45 as u128, 1u16);
        // D s_5_48: or s_5_46 s_5_47
        let s_5_48: Bits = ((s_5_46) | (s_5_47));
        // D s_5_49: cast reint s_5_48 -> u8
        let s_5_49: bool = ((s_5_48.value()) != 0);
        // C s_5_50: const #16968u : u32
        let s_5_50: u32 = 16968;
        // N s_5_51: write-reg s_5_50 <= s_5_49
        let s_5_51: () = {
            state.write_register::<bool>(s_5_50 as isize, s_5_49);
            tracer.write_register(s_5_50 as isize, s_5_49);
        };
        // C s_5_52: const #16979u : u32
        let s_5_52: u32 = 16979;
        // D s_5_53: read-reg s_5_52:u8
        let s_5_53: bool = {
            let value = state.read_register::<bool>(s_5_52 as isize);
            tracer.read_register(s_5_52 as isize, value);
            value
        };
        // C s_5_54: const #1s : i
        let s_5_54: i128 = 1;
        // D s_5_55: read-var operand:u8
        let s_5_55: u8 = fn_state.operand;
        // D s_5_56: cast zx s_5_55 -> bv
        let s_5_56: Bits = Bits::new(s_5_55 as u128, 4u16);
        // C s_5_57: const #1u : u64
        let s_5_57: u64 = 1;
        // D s_5_58: bit-extract s_5_56 s_5_54 s_5_57
        let s_5_58: Bits = (Bits::new(
            ((s_5_56) >> (s_5_54)).value(),
            u16::try_from(s_5_57).unwrap(),
        ));
        // D s_5_59: cast reint s_5_58 -> u8
        let s_5_59: bool = ((s_5_58.value()) != 0);
        // C s_5_60: const #0s : i
        let s_5_60: i128 = 0;
        // C s_5_61: const #0u : u64
        let s_5_61: u64 = 0;
        // D s_5_62: cast zx s_5_59 -> u64
        let s_5_62: u64 = (s_5_59 as u64);
        // C s_5_63: const #1u : u64
        let s_5_63: u64 = 1;
        // D s_5_64: and s_5_62 s_5_63
        let s_5_64: u64 = ((s_5_62) & (s_5_63));
        // D s_5_65: cmp-eq s_5_64 s_5_63
        let s_5_65: bool = ((s_5_64) == (s_5_63));
        // D s_5_66: lsl s_5_62 s_5_60
        let s_5_66: u64 = s_5_62 << s_5_60;
        // D s_5_67: or s_5_61 s_5_66
        let s_5_67: u64 = ((s_5_61) | (s_5_66));
        // D s_5_68: cmpl s_5_66
        let s_5_68: u64 = !s_5_66;
        // D s_5_69: and s_5_61 s_5_68
        let s_5_69: u64 = ((s_5_61) & (s_5_68));
        // D s_5_70: select s_5_65 s_5_67 s_5_69
        let s_5_70: u64 = if s_5_65 { s_5_67 } else { s_5_69 };
        // D s_5_71: cast trunc s_5_70 -> u8
        let s_5_71: bool = ((s_5_70) != 0);
        // D s_5_72: cast zx s_5_53 -> bv
        let s_5_72: Bits = Bits::new(s_5_53 as u128, 1u16);
        // D s_5_73: cast zx s_5_71 -> bv
        let s_5_73: Bits = Bits::new(s_5_71 as u128, 1u16);
        // D s_5_74: or s_5_72 s_5_73
        let s_5_74: Bits = ((s_5_72) | (s_5_73));
        // D s_5_75: cast reint s_5_74 -> u8
        let s_5_75: bool = ((s_5_74.value()) != 0);
        // C s_5_76: const #16979u : u32
        let s_5_76: u32 = 16979;
        // N s_5_77: write-reg s_5_76 <= s_5_75
        let s_5_77: () = {
            state.write_register::<bool>(s_5_76 as isize, s_5_75);
            tracer.write_register(s_5_76 as isize, s_5_75);
        };
        // C s_5_78: const #16977u : u32
        let s_5_78: u32 = 16977;
        // D s_5_79: read-reg s_5_78:u8
        let s_5_79: bool = {
            let value = state.read_register::<bool>(s_5_78 as isize);
            tracer.read_register(s_5_78 as isize, value);
            value
        };
        // C s_5_80: const #0s : i
        let s_5_80: i128 = 0;
        // D s_5_81: read-var operand:u8
        let s_5_81: u8 = fn_state.operand;
        // D s_5_82: cast zx s_5_81 -> bv
        let s_5_82: Bits = Bits::new(s_5_81 as u128, 4u16);
        // C s_5_83: const #1u : u64
        let s_5_83: u64 = 1;
        // D s_5_84: bit-extract s_5_82 s_5_80 s_5_83
        let s_5_84: Bits = (Bits::new(
            ((s_5_82) >> (s_5_80)).value(),
            u16::try_from(s_5_83).unwrap(),
        ));
        // D s_5_85: cast reint s_5_84 -> u8
        let s_5_85: bool = ((s_5_84.value()) != 0);
        // C s_5_86: const #0s : i
        let s_5_86: i128 = 0;
        // C s_5_87: const #0u : u64
        let s_5_87: u64 = 0;
        // D s_5_88: cast zx s_5_85 -> u64
        let s_5_88: u64 = (s_5_85 as u64);
        // C s_5_89: const #1u : u64
        let s_5_89: u64 = 1;
        // D s_5_90: and s_5_88 s_5_89
        let s_5_90: u64 = ((s_5_88) & (s_5_89));
        // D s_5_91: cmp-eq s_5_90 s_5_89
        let s_5_91: bool = ((s_5_90) == (s_5_89));
        // D s_5_92: lsl s_5_88 s_5_86
        let s_5_92: u64 = s_5_88 << s_5_86;
        // D s_5_93: or s_5_87 s_5_92
        let s_5_93: u64 = ((s_5_87) | (s_5_92));
        // D s_5_94: cmpl s_5_92
        let s_5_94: u64 = !s_5_92;
        // D s_5_95: and s_5_87 s_5_94
        let s_5_95: u64 = ((s_5_87) & (s_5_94));
        // D s_5_96: select s_5_91 s_5_93 s_5_95
        let s_5_96: u64 = if s_5_91 { s_5_93 } else { s_5_95 };
        // D s_5_97: cast trunc s_5_96 -> u8
        let s_5_97: bool = ((s_5_96) != 0);
        // D s_5_98: cast zx s_5_79 -> bv
        let s_5_98: Bits = Bits::new(s_5_79 as u128, 1u16);
        // D s_5_99: cast zx s_5_97 -> bv
        let s_5_99: Bits = Bits::new(s_5_97 as u128, 1u16);
        // D s_5_100: or s_5_98 s_5_99
        let s_5_100: Bits = ((s_5_98) | (s_5_99));
        // D s_5_101: cast reint s_5_100 -> u8
        let s_5_101: bool = ((s_5_100.value()) != 0);
        // C s_5_102: const #16977u : u32
        let s_5_102: u32 = 16977;
        // N s_5_103: write-reg s_5_102 <= s_5_101
        let s_5_103: () = {
            state.write_register::<bool>(s_5_102 as isize, s_5_101);
            tracer.write_register(s_5_102 as isize, s_5_101);
        };
        // N s_5_104: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #1u : u32
        let s_6_0: u32 = 1;
        // D s_6_1: read-var field:u32
        let s_6_1: u32 = fn_state.field;
        // D s_6_2: cmp-eq s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) == (s_6_1));
        // D s_6_3: not s_6_2
        let s_6_3: bool = !s_6_2;
        // N s_6_4: branch s_6_3 b8 b7
        if s_6_3 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #16972u : u32
        let s_7_0: u32 = 16972;
        // D s_7_1: read-reg s_7_0:u8
        let s_7_1: bool = {
            let value = state.read_register::<bool>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // C s_7_2: const #3s : i
        let s_7_2: i128 = 3;
        // D s_7_3: read-var operand:u8
        let s_7_3: u8 = fn_state.operand;
        // D s_7_4: cast zx s_7_3 -> bv
        let s_7_4: Bits = Bits::new(s_7_3 as u128, 4u16);
        // C s_7_5: const #1u : u64
        let s_7_5: u64 = 1;
        // D s_7_6: bit-extract s_7_4 s_7_2 s_7_5
        let s_7_6: Bits = (Bits::new(
            ((s_7_4) >> (s_7_2)).value(),
            u16::try_from(s_7_5).unwrap(),
        ));
        // D s_7_7: cast reint s_7_6 -> u8
        let s_7_7: bool = ((s_7_6.value()) != 0);
        // C s_7_8: const #0s : i
        let s_7_8: i128 = 0;
        // C s_7_9: const #0u : u64
        let s_7_9: u64 = 0;
        // D s_7_10: cast zx s_7_7 -> u64
        let s_7_10: u64 = (s_7_7 as u64);
        // C s_7_11: const #1u : u64
        let s_7_11: u64 = 1;
        // D s_7_12: and s_7_10 s_7_11
        let s_7_12: u64 = ((s_7_10) & (s_7_11));
        // D s_7_13: cmp-eq s_7_12 s_7_11
        let s_7_13: bool = ((s_7_12) == (s_7_11));
        // D s_7_14: lsl s_7_10 s_7_8
        let s_7_14: u64 = s_7_10 << s_7_8;
        // D s_7_15: or s_7_9 s_7_14
        let s_7_15: u64 = ((s_7_9) | (s_7_14));
        // D s_7_16: cmpl s_7_14
        let s_7_16: u64 = !s_7_14;
        // D s_7_17: and s_7_9 s_7_16
        let s_7_17: u64 = ((s_7_9) & (s_7_16));
        // D s_7_18: select s_7_13 s_7_15 s_7_17
        let s_7_18: u64 = if s_7_13 { s_7_15 } else { s_7_17 };
        // D s_7_19: cast trunc s_7_18 -> u8
        let s_7_19: bool = ((s_7_18) != 0);
        // D s_7_20: cast zx s_7_19 -> bv
        let s_7_20: Bits = Bits::new(s_7_19 as u128, 1u16);
        // D s_7_21: not s_7_20
        let s_7_21: Bits = !s_7_20;
        // D s_7_22: cast reint s_7_21 -> u8
        let s_7_22: bool = ((s_7_21.value()) != 0);
        // D s_7_23: cast zx s_7_1 -> bv
        let s_7_23: Bits = Bits::new(s_7_1 as u128, 1u16);
        // D s_7_24: cast zx s_7_22 -> bv
        let s_7_24: Bits = Bits::new(s_7_22 as u128, 1u16);
        // D s_7_25: and s_7_23 s_7_24
        let s_7_25: Bits = ((s_7_23) & (s_7_24));
        // D s_7_26: cast reint s_7_25 -> u8
        let s_7_26: bool = ((s_7_25.value()) != 0);
        // C s_7_27: const #16972u : u32
        let s_7_27: u32 = 16972;
        // N s_7_28: write-reg s_7_27 <= s_7_26
        let s_7_28: () = {
            state.write_register::<bool>(s_7_27 as isize, s_7_26);
            tracer.write_register(s_7_27 as isize, s_7_26);
        };
        // C s_7_29: const #16968u : u32
        let s_7_29: u32 = 16968;
        // D s_7_30: read-reg s_7_29:u8
        let s_7_30: bool = {
            let value = state.read_register::<bool>(s_7_29 as isize);
            tracer.read_register(s_7_29 as isize, value);
            value
        };
        // C s_7_31: const #2s : i
        let s_7_31: i128 = 2;
        // D s_7_32: read-var operand:u8
        let s_7_32: u8 = fn_state.operand;
        // D s_7_33: cast zx s_7_32 -> bv
        let s_7_33: Bits = Bits::new(s_7_32 as u128, 4u16);
        // C s_7_34: const #1u : u64
        let s_7_34: u64 = 1;
        // D s_7_35: bit-extract s_7_33 s_7_31 s_7_34
        let s_7_35: Bits = (Bits::new(
            ((s_7_33) >> (s_7_31)).value(),
            u16::try_from(s_7_34).unwrap(),
        ));
        // D s_7_36: cast reint s_7_35 -> u8
        let s_7_36: bool = ((s_7_35.value()) != 0);
        // C s_7_37: const #0s : i
        let s_7_37: i128 = 0;
        // C s_7_38: const #0u : u64
        let s_7_38: u64 = 0;
        // D s_7_39: cast zx s_7_36 -> u64
        let s_7_39: u64 = (s_7_36 as u64);
        // C s_7_40: const #1u : u64
        let s_7_40: u64 = 1;
        // D s_7_41: and s_7_39 s_7_40
        let s_7_41: u64 = ((s_7_39) & (s_7_40));
        // D s_7_42: cmp-eq s_7_41 s_7_40
        let s_7_42: bool = ((s_7_41) == (s_7_40));
        // D s_7_43: lsl s_7_39 s_7_37
        let s_7_43: u64 = s_7_39 << s_7_37;
        // D s_7_44: or s_7_38 s_7_43
        let s_7_44: u64 = ((s_7_38) | (s_7_43));
        // D s_7_45: cmpl s_7_43
        let s_7_45: u64 = !s_7_43;
        // D s_7_46: and s_7_38 s_7_45
        let s_7_46: u64 = ((s_7_38) & (s_7_45));
        // D s_7_47: select s_7_42 s_7_44 s_7_46
        let s_7_47: u64 = if s_7_42 { s_7_44 } else { s_7_46 };
        // D s_7_48: cast trunc s_7_47 -> u8
        let s_7_48: bool = ((s_7_47) != 0);
        // D s_7_49: cast zx s_7_48 -> bv
        let s_7_49: Bits = Bits::new(s_7_48 as u128, 1u16);
        // D s_7_50: not s_7_49
        let s_7_50: Bits = !s_7_49;
        // D s_7_51: cast reint s_7_50 -> u8
        let s_7_51: bool = ((s_7_50.value()) != 0);
        // D s_7_52: cast zx s_7_30 -> bv
        let s_7_52: Bits = Bits::new(s_7_30 as u128, 1u16);
        // D s_7_53: cast zx s_7_51 -> bv
        let s_7_53: Bits = Bits::new(s_7_51 as u128, 1u16);
        // D s_7_54: and s_7_52 s_7_53
        let s_7_54: Bits = ((s_7_52) & (s_7_53));
        // D s_7_55: cast reint s_7_54 -> u8
        let s_7_55: bool = ((s_7_54.value()) != 0);
        // C s_7_56: const #16968u : u32
        let s_7_56: u32 = 16968;
        // N s_7_57: write-reg s_7_56 <= s_7_55
        let s_7_57: () = {
            state.write_register::<bool>(s_7_56 as isize, s_7_55);
            tracer.write_register(s_7_56 as isize, s_7_55);
        };
        // C s_7_58: const #16979u : u32
        let s_7_58: u32 = 16979;
        // D s_7_59: read-reg s_7_58:u8
        let s_7_59: bool = {
            let value = state.read_register::<bool>(s_7_58 as isize);
            tracer.read_register(s_7_58 as isize, value);
            value
        };
        // C s_7_60: const #1s : i
        let s_7_60: i128 = 1;
        // D s_7_61: read-var operand:u8
        let s_7_61: u8 = fn_state.operand;
        // D s_7_62: cast zx s_7_61 -> bv
        let s_7_62: Bits = Bits::new(s_7_61 as u128, 4u16);
        // C s_7_63: const #1u : u64
        let s_7_63: u64 = 1;
        // D s_7_64: bit-extract s_7_62 s_7_60 s_7_63
        let s_7_64: Bits = (Bits::new(
            ((s_7_62) >> (s_7_60)).value(),
            u16::try_from(s_7_63).unwrap(),
        ));
        // D s_7_65: cast reint s_7_64 -> u8
        let s_7_65: bool = ((s_7_64.value()) != 0);
        // C s_7_66: const #0s : i
        let s_7_66: i128 = 0;
        // C s_7_67: const #0u : u64
        let s_7_67: u64 = 0;
        // D s_7_68: cast zx s_7_65 -> u64
        let s_7_68: u64 = (s_7_65 as u64);
        // C s_7_69: const #1u : u64
        let s_7_69: u64 = 1;
        // D s_7_70: and s_7_68 s_7_69
        let s_7_70: u64 = ((s_7_68) & (s_7_69));
        // D s_7_71: cmp-eq s_7_70 s_7_69
        let s_7_71: bool = ((s_7_70) == (s_7_69));
        // D s_7_72: lsl s_7_68 s_7_66
        let s_7_72: u64 = s_7_68 << s_7_66;
        // D s_7_73: or s_7_67 s_7_72
        let s_7_73: u64 = ((s_7_67) | (s_7_72));
        // D s_7_74: cmpl s_7_72
        let s_7_74: u64 = !s_7_72;
        // D s_7_75: and s_7_67 s_7_74
        let s_7_75: u64 = ((s_7_67) & (s_7_74));
        // D s_7_76: select s_7_71 s_7_73 s_7_75
        let s_7_76: u64 = if s_7_71 { s_7_73 } else { s_7_75 };
        // D s_7_77: cast trunc s_7_76 -> u8
        let s_7_77: bool = ((s_7_76) != 0);
        // D s_7_78: cast zx s_7_77 -> bv
        let s_7_78: Bits = Bits::new(s_7_77 as u128, 1u16);
        // D s_7_79: not s_7_78
        let s_7_79: Bits = !s_7_78;
        // D s_7_80: cast reint s_7_79 -> u8
        let s_7_80: bool = ((s_7_79.value()) != 0);
        // D s_7_81: cast zx s_7_59 -> bv
        let s_7_81: Bits = Bits::new(s_7_59 as u128, 1u16);
        // D s_7_82: cast zx s_7_80 -> bv
        let s_7_82: Bits = Bits::new(s_7_80 as u128, 1u16);
        // D s_7_83: and s_7_81 s_7_82
        let s_7_83: Bits = ((s_7_81) & (s_7_82));
        // D s_7_84: cast reint s_7_83 -> u8
        let s_7_84: bool = ((s_7_83.value()) != 0);
        // C s_7_85: const #16979u : u32
        let s_7_85: u32 = 16979;
        // N s_7_86: write-reg s_7_85 <= s_7_84
        let s_7_86: () = {
            state.write_register::<bool>(s_7_85 as isize, s_7_84);
            tracer.write_register(s_7_85 as isize, s_7_84);
        };
        // C s_7_87: const #16977u : u32
        let s_7_87: u32 = 16977;
        // D s_7_88: read-reg s_7_87:u8
        let s_7_88: bool = {
            let value = state.read_register::<bool>(s_7_87 as isize);
            tracer.read_register(s_7_87 as isize, value);
            value
        };
        // C s_7_89: const #0s : i
        let s_7_89: i128 = 0;
        // D s_7_90: read-var operand:u8
        let s_7_90: u8 = fn_state.operand;
        // D s_7_91: cast zx s_7_90 -> bv
        let s_7_91: Bits = Bits::new(s_7_90 as u128, 4u16);
        // C s_7_92: const #1u : u64
        let s_7_92: u64 = 1;
        // D s_7_93: bit-extract s_7_91 s_7_89 s_7_92
        let s_7_93: Bits = (Bits::new(
            ((s_7_91) >> (s_7_89)).value(),
            u16::try_from(s_7_92).unwrap(),
        ));
        // D s_7_94: cast reint s_7_93 -> u8
        let s_7_94: bool = ((s_7_93.value()) != 0);
        // C s_7_95: const #0s : i
        let s_7_95: i128 = 0;
        // C s_7_96: const #0u : u64
        let s_7_96: u64 = 0;
        // D s_7_97: cast zx s_7_94 -> u64
        let s_7_97: u64 = (s_7_94 as u64);
        // C s_7_98: const #1u : u64
        let s_7_98: u64 = 1;
        // D s_7_99: and s_7_97 s_7_98
        let s_7_99: u64 = ((s_7_97) & (s_7_98));
        // D s_7_100: cmp-eq s_7_99 s_7_98
        let s_7_100: bool = ((s_7_99) == (s_7_98));
        // D s_7_101: lsl s_7_97 s_7_95
        let s_7_101: u64 = s_7_97 << s_7_95;
        // D s_7_102: or s_7_96 s_7_101
        let s_7_102: u64 = ((s_7_96) | (s_7_101));
        // D s_7_103: cmpl s_7_101
        let s_7_103: u64 = !s_7_101;
        // D s_7_104: and s_7_96 s_7_103
        let s_7_104: u64 = ((s_7_96) & (s_7_103));
        // D s_7_105: select s_7_100 s_7_102 s_7_104
        let s_7_105: u64 = if s_7_100 { s_7_102 } else { s_7_104 };
        // D s_7_106: cast trunc s_7_105 -> u8
        let s_7_106: bool = ((s_7_105) != 0);
        // D s_7_107: cast zx s_7_106 -> bv
        let s_7_107: Bits = Bits::new(s_7_106 as u128, 1u16);
        // D s_7_108: not s_7_107
        let s_7_108: Bits = !s_7_107;
        // D s_7_109: cast reint s_7_108 -> u8
        let s_7_109: bool = ((s_7_108.value()) != 0);
        // D s_7_110: cast zx s_7_88 -> bv
        let s_7_110: Bits = Bits::new(s_7_88 as u128, 1u16);
        // D s_7_111: cast zx s_7_109 -> bv
        let s_7_111: Bits = Bits::new(s_7_109 as u128, 1u16);
        // D s_7_112: and s_7_110 s_7_111
        let s_7_112: Bits = ((s_7_110) & (s_7_111));
        // D s_7_113: cast reint s_7_112 -> u8
        let s_7_113: bool = ((s_7_112.value()) != 0);
        // C s_7_114: const #16977u : u32
        let s_7_114: u32 = 16977;
        // N s_7_115: write-reg s_7_114 <= s_7_113
        let s_7_115: () = {
            state.write_register::<bool>(s_7_114 as isize, s_7_113);
            tracer.write_register(s_7_114 as isize, s_7_113);
        };
        // N s_7_116: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #2u : u32
        let s_8_0: u32 = 2;
        // D s_8_1: read-var field:u32
        let s_8_1: u32 = fn_state.field;
        // D s_8_2: cmp-eq s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) == (s_8_1));
        // D s_8_3: not s_8_2
        let s_8_3: bool = !s_8_2;
        // N s_8_4: branch s_8_3 b10 b9
        if s_8_3 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0s : i
        let s_9_0: i128 = 0;
        // D s_9_1: read-var operand:u8
        let s_9_1: u8 = fn_state.operand;
        // D s_9_2: cast zx s_9_1 -> bv
        let s_9_2: Bits = Bits::new(s_9_1 as u128, 4u16);
        // C s_9_3: const #1u : u64
        let s_9_3: u64 = 1;
        // D s_9_4: bit-extract s_9_2 s_9_0 s_9_3
        let s_9_4: Bits = (Bits::new(
            ((s_9_2) >> (s_9_0)).value(),
            u16::try_from(s_9_3).unwrap(),
        ));
        // D s_9_5: cast reint s_9_4 -> u8
        let s_9_5: bool = ((s_9_4.value()) != 0);
        // C s_9_6: const #0s : i
        let s_9_6: i128 = 0;
        // C s_9_7: const #0u : u64
        let s_9_7: u64 = 0;
        // D s_9_8: cast zx s_9_5 -> u64
        let s_9_8: u64 = (s_9_5 as u64);
        // C s_9_9: const #1u : u64
        let s_9_9: u64 = 1;
        // D s_9_10: and s_9_8 s_9_9
        let s_9_10: u64 = ((s_9_8) & (s_9_9));
        // D s_9_11: cmp-eq s_9_10 s_9_9
        let s_9_11: bool = ((s_9_10) == (s_9_9));
        // D s_9_12: lsl s_9_8 s_9_6
        let s_9_12: u64 = s_9_8 << s_9_6;
        // D s_9_13: or s_9_7 s_9_12
        let s_9_13: u64 = ((s_9_7) | (s_9_12));
        // D s_9_14: cmpl s_9_12
        let s_9_14: u64 = !s_9_12;
        // D s_9_15: and s_9_7 s_9_14
        let s_9_15: u64 = ((s_9_7) & (s_9_14));
        // D s_9_16: select s_9_11 s_9_13 s_9_15
        let s_9_16: u64 = if s_9_11 { s_9_13 } else { s_9_15 };
        // D s_9_17: cast trunc s_9_16 -> u8
        let s_9_17: bool = ((s_9_16) != 0);
        // C s_9_18: const #16985u : u32
        let s_9_18: u32 = 16985;
        // N s_9_19: write-reg s_9_18 <= s_9_17
        let s_9_19: () = {
            state.write_register::<bool>(s_9_18 as isize, s_9_17);
            tracer.write_register(s_9_18 as isize, s_9_17);
        };
        // N s_9_20: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #3u : u32
        let s_10_0: u32 = 3;
        // D s_10_1: read-var field:u32
        let s_10_1: u32 = fn_state.field;
        // D s_10_2: cmp-eq s_10_0 s_10_1
        let s_10_2: bool = ((s_10_0) == (s_10_1));
        // D s_10_3: not s_10_2
        let s_10_3: bool = !s_10_2;
        // N s_10_4: branch s_10_3 b12 b11
        if s_10_3 {
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
        // C s_11_0: const #0s : i
        let s_11_0: i128 = 0;
        // D s_11_1: read-var operand:u8
        let s_11_1: u8 = fn_state.operand;
        // D s_11_2: cast zx s_11_1 -> bv
        let s_11_2: Bits = Bits::new(s_11_1 as u128, 4u16);
        // C s_11_3: const #1u : u64
        let s_11_3: u64 = 1;
        // D s_11_4: bit-extract s_11_2 s_11_0 s_11_3
        let s_11_4: Bits = (Bits::new(
            ((s_11_2) >> (s_11_0)).value(),
            u16::try_from(s_11_3).unwrap(),
        ));
        // D s_11_5: cast reint s_11_4 -> u8
        let s_11_5: bool = ((s_11_4.value()) != 0);
        // C s_11_6: const #0s : i
        let s_11_6: i128 = 0;
        // C s_11_7: const #0u : u64
        let s_11_7: u64 = 0;
        // D s_11_8: cast zx s_11_5 -> u64
        let s_11_8: u64 = (s_11_5 as u64);
        // C s_11_9: const #1u : u64
        let s_11_9: u64 = 1;
        // D s_11_10: and s_11_8 s_11_9
        let s_11_10: u64 = ((s_11_8) & (s_11_9));
        // D s_11_11: cmp-eq s_11_10 s_11_9
        let s_11_11: bool = ((s_11_10) == (s_11_9));
        // D s_11_12: lsl s_11_8 s_11_6
        let s_11_12: u64 = s_11_8 << s_11_6;
        // D s_11_13: or s_11_7 s_11_12
        let s_11_13: u64 = ((s_11_7) | (s_11_12));
        // D s_11_14: cmpl s_11_12
        let s_11_14: u64 = !s_11_12;
        // D s_11_15: and s_11_7 s_11_14
        let s_11_15: u64 = ((s_11_7) & (s_11_14));
        // D s_11_16: select s_11_11 s_11_13 s_11_15
        let s_11_16: u64 = if s_11_11 { s_11_13 } else { s_11_15 };
        // D s_11_17: cast trunc s_11_16 -> u8
        let s_11_17: bool = ((s_11_16) != 0);
        // C s_11_18: const #16995u : u32
        let s_11_18: u32 = 16995;
        // N s_11_19: write-reg s_11_18 <= s_11_17
        let s_11_19: () = {
            state.write_register::<bool>(s_11_18 as isize, s_11_17);
            tracer.write_register(s_11_18 as isize, s_11_17);
        };
        // N s_11_20: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #4u : u32
        let s_12_0: u32 = 4;
        // D s_12_1: read-var field:u32
        let s_12_1: u32 = fn_state.field;
        // D s_12_2: cmp-eq s_12_0 s_12_1
        let s_12_2: bool = ((s_12_0) == (s_12_1));
        // D s_12_3: not s_12_2
        let s_12_3: bool = !s_12_2;
        // N s_12_4: branch s_12_3 b14 b13
        if s_12_3 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0s : i
        let s_13_0: i128 = 0;
        // D s_13_1: read-var operand:u8
        let s_13_1: u8 = fn_state.operand;
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
        // C s_13_18: const #16973u : u32
        let s_13_18: u32 = 16973;
        // N s_13_19: write-reg s_13_18 <= s_13_17
        let s_13_19: () = {
            state.write_register::<bool>(s_13_18 as isize, s_13_17);
            tracer.write_register(s_13_18 as isize, s_13_17);
        };
        // N s_13_20: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #6u : u32
        let s_14_0: u32 = 6;
        // D s_14_1: read-var field:u32
        let s_14_1: u32 = fn_state.field;
        // D s_14_2: cmp-eq s_14_0 s_14_1
        let s_14_2: bool = ((s_14_0) == (s_14_1));
        // D s_14_3: not s_14_2
        let s_14_3: bool = !s_14_2;
        // N s_14_4: branch s_14_3 b16 b15
        if s_14_3 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #0s : i
        let s_15_0: i128 = 0;
        // D s_15_1: read-var operand:u8
        let s_15_1: u8 = fn_state.operand;
        // D s_15_2: cast zx s_15_1 -> bv
        let s_15_2: Bits = Bits::new(s_15_1 as u128, 4u16);
        // C s_15_3: const #1u : u64
        let s_15_3: u64 = 1;
        // D s_15_4: bit-extract s_15_2 s_15_0 s_15_3
        let s_15_4: Bits = (Bits::new(
            ((s_15_2) >> (s_15_0)).value(),
            u16::try_from(s_15_3).unwrap(),
        ));
        // D s_15_5: cast reint s_15_4 -> u8
        let s_15_5: bool = ((s_15_4.value()) != 0);
        // C s_15_6: const #0s : i
        let s_15_6: i128 = 0;
        // C s_15_7: const #0u : u64
        let s_15_7: u64 = 0;
        // D s_15_8: cast zx s_15_5 -> u64
        let s_15_8: u64 = (s_15_5 as u64);
        // C s_15_9: const #1u : u64
        let s_15_9: u64 = 1;
        // D s_15_10: and s_15_8 s_15_9
        let s_15_10: u64 = ((s_15_8) & (s_15_9));
        // D s_15_11: cmp-eq s_15_10 s_15_9
        let s_15_11: bool = ((s_15_10) == (s_15_9));
        // D s_15_12: lsl s_15_8 s_15_6
        let s_15_12: u64 = s_15_8 << s_15_6;
        // D s_15_13: or s_15_7 s_15_12
        let s_15_13: u64 = ((s_15_7) | (s_15_12));
        // D s_15_14: cmpl s_15_12
        let s_15_14: u64 = !s_15_12;
        // D s_15_15: and s_15_7 s_15_14
        let s_15_15: u64 = ((s_15_7) & (s_15_14));
        // D s_15_16: select s_15_11 s_15_13 s_15_15
        let s_15_16: u64 = if s_15_11 { s_15_13 } else { s_15_15 };
        // D s_15_17: cast trunc s_15_16 -> u8
        let s_15_17: bool = ((s_15_16) != 0);
        // C s_15_18: const #16994u : u32
        let s_15_18: u32 = 16994;
        // N s_15_19: write-reg s_15_18 <= s_15_17
        let s_15_19: () = {
            state.write_register::<bool>(s_15_18 as isize, s_15_17);
            tracer.write_register(s_15_18 as isize, s_15_17);
        };
        // N s_15_20: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #10u : u32
        let s_16_0: u32 = 10;
        // D s_16_1: read-var field:u32
        let s_16_1: u32 = fn_state.field;
        // D s_16_2: cmp-eq s_16_0 s_16_1
        let s_16_2: bool = ((s_16_0) == (s_16_1));
        // D s_16_3: not s_16_2
        let s_16_3: bool = !s_16_2;
        // N s_16_4: branch s_16_3 b30 b17
        if s_16_3 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #16975u : u32
        let s_17_0: u32 = 16975;
        // D s_17_1: read-reg s_17_0:u8
        let s_17_1: u8 = {
            let value = state.read_register::<u8>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // D s_17_2: cast zx s_17_1 -> bv
        let s_17_2: Bits = Bits::new(s_17_1 as u128, 2u16);
        // C s_17_3: const #440u : u32
        let s_17_3: u32 = 440;
        // D s_17_4: read-reg s_17_3:u8
        let s_17_4: u8 = {
            let value = state.read_register::<u8>(s_17_3 as isize);
            tracer.read_register(s_17_3 as isize, value);
            value
        };
        // D s_17_5: cast zx s_17_4 -> bv
        let s_17_5: Bits = Bits::new(s_17_4 as u128, 2u16);
        // D s_17_6: cmp-eq s_17_2 s_17_5
        let s_17_6: bool = ((s_17_2) == (s_17_5));
        // N s_17_7: branch s_17_6 b29 b18
        if s_17_6 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var gs#165326 <= s_18_0
        fn_state.gs_165326 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#165326:u8
        let s_19_0: bool = fn_state.gs_165326;
        // N s_19_1: branch s_19_0 b28 b20
        if s_19_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var gs#165327 <= s_20_0
        fn_state.gs_165327 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#165327:u8
        let s_21_0: bool = fn_state.gs_165327;
        // N s_21_1: branch s_21_0 b27 b22
        if s_21_0 {
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
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var gs#165330 <= s_22_0
        fn_state.gs_165330 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#165330:u8
        let s_23_0: bool = fn_state.gs_165330;
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
        // N s_24_0: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #0s : i
        let s_25_0: i128 = 0;
        // D s_25_1: read-var operand:u8
        let s_25_1: u8 = fn_state.operand;
        // D s_25_2: cast zx s_25_1 -> bv
        let s_25_2: Bits = Bits::new(s_25_1 as u128, 4u16);
        // C s_25_3: const #1u : u64
        let s_25_3: u64 = 1;
        // D s_25_4: bit-extract s_25_2 s_25_0 s_25_3
        let s_25_4: Bits = (Bits::new(
            ((s_25_2) >> (s_25_0)).value(),
            u16::try_from(s_25_3).unwrap(),
        ));
        // D s_25_5: cast reint s_25_4 -> u8
        let s_25_5: bool = ((s_25_4.value()) != 0);
        // C s_25_6: const #0s : i
        let s_25_6: i128 = 0;
        // C s_25_7: const #0u : u64
        let s_25_7: u64 = 0;
        // D s_25_8: cast zx s_25_5 -> u64
        let s_25_8: u64 = (s_25_5 as u64);
        // C s_25_9: const #1u : u64
        let s_25_9: u64 = 1;
        // D s_25_10: and s_25_8 s_25_9
        let s_25_10: u64 = ((s_25_8) & (s_25_9));
        // D s_25_11: cmp-eq s_25_10 s_25_9
        let s_25_11: bool = ((s_25_10) == (s_25_9));
        // D s_25_12: lsl s_25_8 s_25_6
        let s_25_12: u64 = s_25_8 << s_25_6;
        // D s_25_13: or s_25_7 s_25_12
        let s_25_13: u64 = ((s_25_7) | (s_25_12));
        // D s_25_14: cmpl s_25_12
        let s_25_14: u64 = !s_25_12;
        // D s_25_15: and s_25_7 s_25_14
        let s_25_15: u64 = ((s_25_7) & (s_25_14));
        // D s_25_16: select s_25_11 s_25_13 s_25_15
        let s_25_16: u64 = if s_25_11 { s_25_13 } else { s_25_15 };
        // D s_25_17: cast trunc s_25_16 -> u8
        let s_25_17: bool = ((s_25_16) != 0);
        // C s_25_18: const #16969u : u32
        let s_25_18: u32 = 16969;
        // N s_25_19: write-reg s_25_18 <= s_25_17
        let s_25_19: () = {
            state.write_register::<bool>(s_25_18 as isize, s_25_17);
            tracer.write_register(s_25_18 as isize, s_25_17);
        };
        // N s_25_20: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #24u : u8
        let s_26_0: u8 = 24;
        // C s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 8u16);
        // C s_26_2: cast zx s_26_1 -> i
        let s_26_2: i128 = (s_26_1.value() as i128);
        // C s_26_3: cast reint s_26_2 -> i64
        let s_26_3: i64 = (s_26_2 as i64);
        // C s_26_4: cast zx s_26_3 -> i
        let s_26_4: i128 = (i128::try_from(s_26_3).unwrap());
        // C s_26_5: const #432u : u32
        let s_26_5: u32 = 432;
        // D s_26_6: read-reg s_26_5:u8
        let s_26_6: u8 = {
            let value = state.read_register::<u8>(s_26_5 as isize);
            tracer.read_register(s_26_5 as isize, value);
            value
        };
        // D s_26_7: call AArch64_SystemAccessTrap(s_26_6, s_26_4)
        let s_26_7: () = AArch64_SystemAccessTrap(state, tracer, s_26_6, s_26_4);
        // N s_26_8: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #0s : i
        let s_27_0: i128 = 0;
        // D s_27_1: read-var operand:u8
        let s_27_1: u8 = fn_state.operand;
        // D s_27_2: cast zx s_27_1 -> bv
        let s_27_2: Bits = Bits::new(s_27_1 as u128, 4u16);
        // C s_27_3: const #1u : u64
        let s_27_3: u64 = 1;
        // D s_27_4: bit-extract s_27_2 s_27_0 s_27_3
        let s_27_4: Bits = (Bits::new(
            ((s_27_2) >> (s_27_0)).value(),
            u16::try_from(s_27_3).unwrap(),
        ));
        // D s_27_5: cast reint s_27_4 -> u8
        let s_27_5: bool = ((s_27_4.value()) != 0);
        // C s_27_6: const #0s : i
        let s_27_6: i128 = 0;
        // C s_27_7: const #0u : u64
        let s_27_7: u64 = 0;
        // D s_27_8: cast zx s_27_5 -> u64
        let s_27_8: u64 = (s_27_5 as u64);
        // C s_27_9: const #1u : u64
        let s_27_9: u64 = 1;
        // D s_27_10: and s_27_8 s_27_9
        let s_27_10: u64 = ((s_27_8) & (s_27_9));
        // D s_27_11: cmp-eq s_27_10 s_27_9
        let s_27_11: bool = ((s_27_10) == (s_27_9));
        // D s_27_12: lsl s_27_8 s_27_6
        let s_27_12: u64 = s_27_8 << s_27_6;
        // D s_27_13: or s_27_7 s_27_12
        let s_27_13: u64 = ((s_27_7) | (s_27_12));
        // D s_27_14: cmpl s_27_12
        let s_27_14: u64 = !s_27_12;
        // D s_27_15: and s_27_7 s_27_14
        let s_27_15: u64 = ((s_27_7) & (s_27_14));
        // D s_27_16: select s_27_11 s_27_13 s_27_15
        let s_27_16: u64 = if s_27_11 { s_27_13 } else { s_27_15 };
        // D s_27_17: cast trunc s_27_16 -> u8
        let s_27_17: bool = ((s_27_16) != 0);
        // D s_27_18: cast zx s_27_17 -> bv
        let s_27_18: Bits = Bits::new(s_27_17 as u128, 1u16);
        // C s_27_19: const #1u : u8
        let s_27_19: bool = true;
        // C s_27_20: cast zx s_27_19 -> bv
        let s_27_20: Bits = Bits::new(s_27_19 as u128, 1u16);
        // D s_27_21: cmp-eq s_27_18 s_27_20
        let s_27_21: bool = ((s_27_18) == (s_27_20));
        // D s_27_22: write-var gs#165330 <= s_27_21
        fn_state.gs_165330 = s_27_21;
        // N s_27_23: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #22528u : u32
        let s_28_0: u32 = 22528;
        // D s_28_1: read-reg s_28_0:struct
        let s_28_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_28_0 as isize);
            tracer.read_register(s_28_0 as isize, value);
            value
        };
        // D s_28_2: call _get_HCRX_EL2_Type_TALLINT(s_28_1)
        let s_28_2: bool = u_get_HCRX_EL2_Type_TALLINT(state, tracer, s_28_1);
        // D s_28_3: cast zx s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 1u16);
        // C s_28_4: const #1u : u8
        let s_28_4: bool = true;
        // C s_28_5: cast zx s_28_4 -> bv
        let s_28_5: Bits = Bits::new(s_28_4 as u128, 1u16);
        // D s_28_6: cmp-eq s_28_3 s_28_5
        let s_28_6: bool = ((s_28_3) == (s_28_5));
        // D s_28_7: write-var gs#165327 <= s_28_6
        fn_state.gs_165327 = s_28_6;
        // N s_28_8: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #() : ()
        let s_29_0: () = ();
        // S s_29_1: call IsHCRXEL2Enabled(s_29_0)
        let s_29_1: bool = IsHCRXEL2Enabled(state, tracer, s_29_0);
        // D s_29_2: write-var gs#165326 <= s_29_1
        fn_state.gs_165326 = s_29_1;
        // N s_29_3: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #7u : u32
        let s_30_0: u32 = 7;
        // D s_30_1: read-var field:u32
        let s_30_1: u32 = fn_state.field;
        // D s_30_2: cmp-eq s_30_0 s_30_1
        let s_30_2: bool = ((s_30_0) == (s_30_1));
        // D s_30_3: not s_30_2
        let s_30_3: bool = !s_30_2;
        // N s_30_4: branch s_30_3 b33 b31
        if s_30_3 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #() : ()
        let s_31_0: () = ();
        // S s_31_1: call CheckSMEAccess(s_31_0)
        let s_31_1: () = CheckSMEAccess(state, tracer, s_31_0);
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #0s : i
        let s_32_0: i128 = 0;
        // D s_32_1: read-var operand:u8
        let s_32_1: u8 = fn_state.operand;
        // D s_32_2: cast zx s_32_1 -> bv
        let s_32_2: Bits = Bits::new(s_32_1 as u128, 4u16);
        // C s_32_3: const #1u : u64
        let s_32_3: u64 = 1;
        // D s_32_4: bit-extract s_32_2 s_32_0 s_32_3
        let s_32_4: Bits = (Bits::new(
            ((s_32_2) >> (s_32_0)).value(),
            u16::try_from(s_32_3).unwrap(),
        ));
        // D s_32_5: cast reint s_32_4 -> u8
        let s_32_5: bool = ((s_32_4.value()) != 0);
        // C s_32_6: const #0s : i
        let s_32_6: i128 = 0;
        // C s_32_7: const #0u : u64
        let s_32_7: u64 = 0;
        // D s_32_8: cast zx s_32_5 -> u64
        let s_32_8: u64 = (s_32_5 as u64);
        // C s_32_9: const #1u : u64
        let s_32_9: u64 = 1;
        // D s_32_10: and s_32_8 s_32_9
        let s_32_10: u64 = ((s_32_8) & (s_32_9));
        // D s_32_11: cmp-eq s_32_10 s_32_9
        let s_32_11: bool = ((s_32_10) == (s_32_9));
        // D s_32_12: lsl s_32_8 s_32_6
        let s_32_12: u64 = s_32_8 << s_32_6;
        // D s_32_13: or s_32_7 s_32_12
        let s_32_13: u64 = ((s_32_7) | (s_32_12));
        // D s_32_14: cmpl s_32_12
        let s_32_14: u64 = !s_32_12;
        // D s_32_15: and s_32_7 s_32_14
        let s_32_15: u64 = ((s_32_7) & (s_32_14));
        // D s_32_16: select s_32_11 s_32_13 s_32_15
        let s_32_16: u64 = if s_32_11 { s_32_13 } else { s_32_15 };
        // D s_32_17: cast trunc s_32_16 -> u8
        let s_32_17: bool = ((s_32_16) != 0);
        // D s_32_18: call SetPSTATE_SM(s_32_17)
        let s_32_18: () = SetPSTATE_SM(state, tracer, s_32_17);
        // N s_32_19: return
        return;
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #8u : u32
        let s_33_0: u32 = 8;
        // D s_33_1: read-var field:u32
        let s_33_1: u32 = fn_state.field;
        // D s_33_2: cmp-eq s_33_0 s_33_1
        let s_33_2: bool = ((s_33_0) == (s_33_1));
        // D s_33_3: not s_33_2
        let s_33_3: bool = !s_33_2;
        // N s_33_4: branch s_33_3 b36 b34
        if s_33_3 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #() : ()
        let s_34_0: () = ();
        // S s_34_1: call CheckSMEAccess(s_34_0)
        let s_34_1: () = CheckSMEAccess(state, tracer, s_34_0);
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #0s : i
        let s_35_0: i128 = 0;
        // D s_35_1: read-var operand:u8
        let s_35_1: u8 = fn_state.operand;
        // D s_35_2: cast zx s_35_1 -> bv
        let s_35_2: Bits = Bits::new(s_35_1 as u128, 4u16);
        // C s_35_3: const #1u : u64
        let s_35_3: u64 = 1;
        // D s_35_4: bit-extract s_35_2 s_35_0 s_35_3
        let s_35_4: Bits = (Bits::new(
            ((s_35_2) >> (s_35_0)).value(),
            u16::try_from(s_35_3).unwrap(),
        ));
        // D s_35_5: cast reint s_35_4 -> u8
        let s_35_5: bool = ((s_35_4.value()) != 0);
        // C s_35_6: const #0s : i
        let s_35_6: i128 = 0;
        // C s_35_7: const #0u : u64
        let s_35_7: u64 = 0;
        // D s_35_8: cast zx s_35_5 -> u64
        let s_35_8: u64 = (s_35_5 as u64);
        // C s_35_9: const #1u : u64
        let s_35_9: u64 = 1;
        // D s_35_10: and s_35_8 s_35_9
        let s_35_10: u64 = ((s_35_8) & (s_35_9));
        // D s_35_11: cmp-eq s_35_10 s_35_9
        let s_35_11: bool = ((s_35_10) == (s_35_9));
        // D s_35_12: lsl s_35_8 s_35_6
        let s_35_12: u64 = s_35_8 << s_35_6;
        // D s_35_13: or s_35_7 s_35_12
        let s_35_13: u64 = ((s_35_7) | (s_35_12));
        // D s_35_14: cmpl s_35_12
        let s_35_14: u64 = !s_35_12;
        // D s_35_15: and s_35_7 s_35_14
        let s_35_15: u64 = ((s_35_7) & (s_35_14));
        // D s_35_16: select s_35_11 s_35_13 s_35_15
        let s_35_16: u64 = if s_35_11 { s_35_13 } else { s_35_15 };
        // D s_35_17: cast trunc s_35_16 -> u8
        let s_35_17: bool = ((s_35_16) != 0);
        // D s_35_18: call SetPSTATE_ZA(s_35_17)
        let s_35_18: () = SetPSTATE_ZA(state, tracer, s_35_17);
        // N s_35_19: return
        return;
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #9u : u32
        let s_36_0: u32 = 9;
        // D s_36_1: read-var field:u32
        let s_36_1: u32 = fn_state.field;
        // D s_36_2: cmp-eq s_36_0 s_36_1
        let s_36_2: bool = ((s_36_0) == (s_36_1));
        // D s_36_3: not s_36_2
        let s_36_3: bool = !s_36_2;
        // N s_36_4: branch s_36_3 b39 b37
        if s_36_3 {
            return block_39(state, tracer, fn_state);
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
        // S s_37_1: call CheckSMEAccess(s_37_0)
        let s_37_1: () = CheckSMEAccess(state, tracer, s_37_0);
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #0s : i
        let s_38_0: i128 = 0;
        // D s_38_1: read-var operand:u8
        let s_38_1: u8 = fn_state.operand;
        // D s_38_2: cast zx s_38_1 -> bv
        let s_38_2: Bits = Bits::new(s_38_1 as u128, 4u16);
        // C s_38_3: const #1u : u64
        let s_38_3: u64 = 1;
        // D s_38_4: bit-extract s_38_2 s_38_0 s_38_3
        let s_38_4: Bits = (Bits::new(
            ((s_38_2) >> (s_38_0)).value(),
            u16::try_from(s_38_3).unwrap(),
        ));
        // D s_38_5: cast reint s_38_4 -> u8
        let s_38_5: bool = ((s_38_4.value()) != 0);
        // C s_38_6: const #0s : i
        let s_38_6: i128 = 0;
        // C s_38_7: const #0u : u64
        let s_38_7: u64 = 0;
        // D s_38_8: cast zx s_38_5 -> u64
        let s_38_8: u64 = (s_38_5 as u64);
        // C s_38_9: const #1u : u64
        let s_38_9: u64 = 1;
        // D s_38_10: and s_38_8 s_38_9
        let s_38_10: u64 = ((s_38_8) & (s_38_9));
        // D s_38_11: cmp-eq s_38_10 s_38_9
        let s_38_11: bool = ((s_38_10) == (s_38_9));
        // D s_38_12: lsl s_38_8 s_38_6
        let s_38_12: u64 = s_38_8 << s_38_6;
        // D s_38_13: or s_38_7 s_38_12
        let s_38_13: u64 = ((s_38_7) | (s_38_12));
        // D s_38_14: cmpl s_38_12
        let s_38_14: u64 = !s_38_12;
        // D s_38_15: and s_38_7 s_38_14
        let s_38_15: u64 = ((s_38_7) & (s_38_14));
        // D s_38_16: select s_38_11 s_38_13 s_38_15
        let s_38_16: u64 = if s_38_11 { s_38_13 } else { s_38_15 };
        // D s_38_17: cast trunc s_38_16 -> u8
        let s_38_17: bool = ((s_38_16) != 0);
        // D s_38_18: call SetPSTATE_SM(s_38_17)
        let s_38_18: () = SetPSTATE_SM(state, tracer, s_38_17);
        // C s_38_19: const #0s : i
        let s_38_19: i128 = 0;
        // D s_38_20: read-var operand:u8
        let s_38_20: u8 = fn_state.operand;
        // D s_38_21: cast zx s_38_20 -> bv
        let s_38_21: Bits = Bits::new(s_38_20 as u128, 4u16);
        // C s_38_22: const #1u : u64
        let s_38_22: u64 = 1;
        // D s_38_23: bit-extract s_38_21 s_38_19 s_38_22
        let s_38_23: Bits = (Bits::new(
            ((s_38_21) >> (s_38_19)).value(),
            u16::try_from(s_38_22).unwrap(),
        ));
        // D s_38_24: cast reint s_38_23 -> u8
        let s_38_24: bool = ((s_38_23.value()) != 0);
        // C s_38_25: const #0s : i
        let s_38_25: i128 = 0;
        // C s_38_26: const #0u : u64
        let s_38_26: u64 = 0;
        // D s_38_27: cast zx s_38_24 -> u64
        let s_38_27: u64 = (s_38_24 as u64);
        // C s_38_28: const #1u : u64
        let s_38_28: u64 = 1;
        // D s_38_29: and s_38_27 s_38_28
        let s_38_29: u64 = ((s_38_27) & (s_38_28));
        // D s_38_30: cmp-eq s_38_29 s_38_28
        let s_38_30: bool = ((s_38_29) == (s_38_28));
        // D s_38_31: lsl s_38_27 s_38_25
        let s_38_31: u64 = s_38_27 << s_38_25;
        // D s_38_32: or s_38_26 s_38_31
        let s_38_32: u64 = ((s_38_26) | (s_38_31));
        // D s_38_33: cmpl s_38_31
        let s_38_33: u64 = !s_38_31;
        // D s_38_34: and s_38_26 s_38_33
        let s_38_34: u64 = ((s_38_26) & (s_38_33));
        // D s_38_35: select s_38_30 s_38_32 s_38_34
        let s_38_35: u64 = if s_38_30 { s_38_32 } else { s_38_34 };
        // D s_38_36: cast trunc s_38_35 -> u8
        let s_38_36: bool = ((s_38_35) != 0);
        // D s_38_37: call SetPSTATE_ZA(s_38_36)
        let s_38_37: () = SetPSTATE_ZA(state, tracer, s_38_36);
        // N s_38_38: return
        return;
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #11u : u32
        let s_39_0: u32 = 11;
        // D s_39_1: read-var field:u32
        let s_39_1: u32 = fn_state.field;
        // D s_39_2: cmp-eq s_39_0 s_39_1
        let s_39_2: bool = ((s_39_0) == (s_39_1));
        // D s_39_3: not s_39_2
        let s_39_3: bool = !s_39_2;
        // N s_39_4: branch s_39_3 b41 b40
        if s_39_3 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #0s : i
        let s_40_0: i128 = 0;
        // D s_40_1: read-var operand:u8
        let s_40_1: u8 = fn_state.operand;
        // D s_40_2: cast zx s_40_1 -> bv
        let s_40_2: Bits = Bits::new(s_40_1 as u128, 4u16);
        // C s_40_3: const #1u : u64
        let s_40_3: u64 = 1;
        // D s_40_4: bit-extract s_40_2 s_40_0 s_40_3
        let s_40_4: Bits = (Bits::new(
            ((s_40_2) >> (s_40_0)).value(),
            u16::try_from(s_40_3).unwrap(),
        ));
        // D s_40_5: cast reint s_40_4 -> u8
        let s_40_5: bool = ((s_40_4.value()) != 0);
        // C s_40_6: const #0s : i
        let s_40_6: i128 = 0;
        // C s_40_7: const #0u : u64
        let s_40_7: u64 = 0;
        // D s_40_8: cast zx s_40_5 -> u64
        let s_40_8: u64 = (s_40_5 as u64);
        // C s_40_9: const #1u : u64
        let s_40_9: u64 = 1;
        // D s_40_10: and s_40_8 s_40_9
        let s_40_10: u64 = ((s_40_8) & (s_40_9));
        // D s_40_11: cmp-eq s_40_10 s_40_9
        let s_40_11: bool = ((s_40_10) == (s_40_9));
        // D s_40_12: lsl s_40_8 s_40_6
        let s_40_12: u64 = s_40_8 << s_40_6;
        // D s_40_13: or s_40_7 s_40_12
        let s_40_13: u64 = ((s_40_7) | (s_40_12));
        // D s_40_14: cmpl s_40_12
        let s_40_14: u64 = !s_40_12;
        // D s_40_15: and s_40_7 s_40_14
        let s_40_15: u64 = ((s_40_7) & (s_40_14));
        // D s_40_16: select s_40_11 s_40_13 s_40_15
        let s_40_16: u64 = if s_40_11 { s_40_13 } else { s_40_15 };
        // D s_40_17: cast trunc s_40_16 -> u8
        let s_40_17: bool = ((s_40_16) != 0);
        // C s_40_18: const #16986u : u32
        let s_40_18: u32 = 16986;
        // N s_40_19: write-reg s_40_18 <= s_40_17
        let s_40_19: () = {
            state.write_register::<bool>(s_40_18 as isize, s_40_17);
            tracer.write_register(s_40_18 as isize, s_40_17);
        };
        // N s_40_20: return
        return;
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_41_0: return
        return;
    }
}
