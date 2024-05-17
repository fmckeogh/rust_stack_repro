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
use EDSCR_write::*;
use DLR_write::*;
use u_get_SCTLR_Type_SPAN::*;
use Mk_DSPSR_Type::*;
use u__UNKNOWN_bit::*;
use UpdateEDSCRFields::*;
use HavePANExt::*;
use u__UNKNOWN_bits::*;
use SPSR_set::*;
use DSPSR_write::*;
use R_set::*;
use ELUsingAArch32::*;
use SCTLR_read__2::*;
use AArch32_WriteMode::*;
use HaveSSBSExt::*;
use EndOfInstruction::*;
use SynchronizeContext::*;
use u_get_SCTLR_Type_EE::*;
use EDSCR_read::*;
use u_update_EDSCR_Type_ERR::*;
use common::*;
pub fn AArch32_EnterModeInDebugState<T: Tracer>(
    state: &mut State,
    tracer: &T,
    target_mode: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_9025: bool,
        gs_9006: bool,
        target_mode: u8,
    }
    let fn_state = FunctionState {
        target_mode,
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
        // S s_0_1: call SynchronizeContext(s_0_0)
        let s_0_1: () = SynchronizeContext(state, tracer, s_0_0);
        // C s_0_2: const #440u : u32
        let s_0_2: u32 = 440;
        // D s_0_3: read-reg s_0_2:u8
        let s_0_3: u8 = {
            let value = state.read_register::<u8>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // D s_0_4: call ELUsingAArch32(s_0_3)
        let s_0_4: bool = ELUsingAArch32(state, tracer, s_0_3);
        // N s_0_5: branch s_0_4 b15 b1
        if s_0_4 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#9006 <= s_1_0
        fn_state.gs_9006 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#9006:u8
        let s_2_0: bool = fn_state.gs_9006;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // C s_2_2: const #16983u : u32
        let s_2_2: u32 = 16983;
        // D s_2_3: read-reg s_2_2:u8
        let s_2_3: u8 = {
            let value = state.read_register::<u8>(s_2_2 as isize);
            tracer.read_register(s_2_2 as isize, value);
            value
        };
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 5u16);
        // C s_2_5: const #384u : u32
        let s_2_5: u32 = 384;
        // D s_2_6: read-reg s_2_5:u8
        let s_2_6: u8 = {
            let value = state.read_register::<u8>(s_2_5 as isize);
            tracer.read_register(s_2_5 as isize, value);
            value
        };
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 5u16);
        // D s_2_8: cmp-eq s_2_4 s_2_7
        let s_2_8: bool = ((s_2_4) == (s_2_7));
        // N s_2_9: branch s_2_8 b14 b3
        if s_2_8 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var target_mode:u8
        let s_4_0: u8 = fn_state.target_mode;
        // D s_4_1: call AArch32_WriteMode(s_4_0)
        let s_4_1: () = AArch32_WriteMode(state, tracer, s_4_0);
        // C s_4_2: const #32s : i64
        let s_4_2: i64 = 32;
        // C s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // S s_4_4: call __UNKNOWN_bits(s_4_3)
        let s_4_4: Bits = u__UNKNOWN_bits(state, tracer, s_4_3);
        // S s_4_5: cast reint s_4_4 -> u32
        let s_4_5: u32 = (s_4_4.value() as u32);
        // C s_4_6: const #32s : i
        let s_4_6: i128 = 32;
        // S s_4_7: cast zx s_4_5 -> bv
        let s_4_7: Bits = Bits::new(s_4_5 as u128, 32u16);
        // S s_4_8: call SPSR_set(s_4_6, s_4_7)
        let s_4_8: () = SPSR_set(state, tracer, s_4_6, s_4_7);
        // C s_4_9: const #32s : i64
        let s_4_9: i64 = 32;
        // C s_4_10: cast zx s_4_9 -> i
        let s_4_10: i128 = (i128::try_from(s_4_9).unwrap());
        // S s_4_11: call __UNKNOWN_bits(s_4_10)
        let s_4_11: Bits = u__UNKNOWN_bits(state, tracer, s_4_10);
        // S s_4_12: cast reint s_4_11 -> u32
        let s_4_12: u32 = (s_4_11.value() as u32);
        // C s_4_13: const #14s : i
        let s_4_13: i128 = 14;
        // S s_4_14: call R_set(s_4_13, s_4_12)
        let s_4_14: () = R_set(state, tracer, s_4_13, s_4_12);
        // C s_4_15: const #1u : u8
        let s_4_15: bool = true;
        // C s_4_16: const #16993u : u32
        let s_4_16: u32 = 16993;
        // N s_4_17: write-reg s_4_16 <= s_4_15
        let s_4_17: () = {
            state.write_register::<bool>(s_4_16 as isize, s_4_15);
            tracer.write_register(s_4_16 as isize, s_4_15);
        };
        // C s_4_18: const #4s : i64
        let s_4_18: i64 = 4;
        // C s_4_19: cast zx s_4_18 -> i
        let s_4_19: i128 = (i128::try_from(s_4_18).unwrap());
        // S s_4_20: call __UNKNOWN_bits(s_4_19)
        let s_4_20: Bits = u__UNKNOWN_bits(state, tracer, s_4_19);
        // S s_4_21: cast reint s_4_20 -> u8
        let s_4_21: u8 = (s_4_20.value() as u8);
        // C s_4_22: const #3s : i
        let s_4_22: i128 = 3;
        // S s_4_23: cast zx s_4_21 -> bv
        let s_4_23: Bits = Bits::new(s_4_21 as u128, 4u16);
        // C s_4_24: const #1s : i64
        let s_4_24: i64 = 1;
        // C s_4_25: cast zx s_4_24 -> i
        let s_4_25: i128 = (i128::try_from(s_4_24).unwrap());
        // C s_4_26: const #0s : i
        let s_4_26: i128 = 0;
        // C s_4_27: add s_4_26 s_4_25
        let s_4_27: i128 = (s_4_26 + s_4_25);
        // D s_4_28: bit-extract s_4_23 s_4_22 s_4_27
        let s_4_28: Bits = (Bits::new(
            ((s_4_23) >> (s_4_22)).value(),
            u16::try_from(s_4_27).unwrap(),
        ));
        // D s_4_29: cast reint s_4_28 -> u8
        let s_4_29: bool = ((s_4_28.value()) != 0);
        // C s_4_30: const #16991u : u32
        let s_4_30: u32 = 16991;
        // N s_4_31: write-reg s_4_30 <= s_4_29
        let s_4_31: () = {
            state.write_register::<bool>(s_4_30 as isize, s_4_29);
            tracer.write_register(s_4_30 as isize, s_4_29);
        };
        // C s_4_32: const #2s : i
        let s_4_32: i128 = 2;
        // S s_4_33: cast zx s_4_21 -> bv
        let s_4_33: Bits = Bits::new(s_4_21 as u128, 4u16);
        // C s_4_34: const #1s : i64
        let s_4_34: i64 = 1;
        // C s_4_35: cast zx s_4_34 -> i
        let s_4_35: i128 = (i128::try_from(s_4_34).unwrap());
        // C s_4_36: const #0s : i
        let s_4_36: i128 = 0;
        // C s_4_37: add s_4_36 s_4_35
        let s_4_37: i128 = (s_4_36 + s_4_35);
        // D s_4_38: bit-extract s_4_33 s_4_32 s_4_37
        let s_4_38: Bits = (Bits::new(
            ((s_4_33) >> (s_4_32)).value(),
            u16::try_from(s_4_37).unwrap(),
        ));
        // D s_4_39: cast reint s_4_38 -> u8
        let s_4_39: bool = ((s_4_38.value()) != 0);
        // C s_4_40: const #16968u : u32
        let s_4_40: u32 = 16968;
        // N s_4_41: write-reg s_4_40 <= s_4_39
        let s_4_41: () = {
            state.write_register::<bool>(s_4_40 as isize, s_4_39);
            tracer.write_register(s_4_40 as isize, s_4_39);
        };
        // C s_4_42: const #1s : i
        let s_4_42: i128 = 1;
        // S s_4_43: cast zx s_4_21 -> bv
        let s_4_43: Bits = Bits::new(s_4_21 as u128, 4u16);
        // C s_4_44: const #1s : i64
        let s_4_44: i64 = 1;
        // C s_4_45: cast zx s_4_44 -> i
        let s_4_45: i128 = (i128::try_from(s_4_44).unwrap());
        // C s_4_46: const #0s : i
        let s_4_46: i128 = 0;
        // C s_4_47: add s_4_46 s_4_45
        let s_4_47: i128 = (s_4_46 + s_4_45);
        // D s_4_48: bit-extract s_4_43 s_4_42 s_4_47
        let s_4_48: Bits = (Bits::new(
            ((s_4_43) >> (s_4_42)).value(),
            u16::try_from(s_4_47).unwrap(),
        ));
        // D s_4_49: cast reint s_4_48 -> u8
        let s_4_49: bool = ((s_4_48.value()) != 0);
        // C s_4_50: const #16979u : u32
        let s_4_50: u32 = 16979;
        // N s_4_51: write-reg s_4_50 <= s_4_49
        let s_4_51: () = {
            state.write_register::<bool>(s_4_50 as isize, s_4_49);
            tracer.write_register(s_4_50 as isize, s_4_49);
        };
        // C s_4_52: const #0s : i
        let s_4_52: i128 = 0;
        // S s_4_53: cast zx s_4_21 -> bv
        let s_4_53: Bits = Bits::new(s_4_21 as u128, 4u16);
        // C s_4_54: const #1s : i64
        let s_4_54: i64 = 1;
        // C s_4_55: cast zx s_4_54 -> i
        let s_4_55: i128 = (i128::try_from(s_4_54).unwrap());
        // C s_4_56: const #0s : i
        let s_4_56: i128 = 0;
        // C s_4_57: add s_4_56 s_4_55
        let s_4_57: i128 = (s_4_56 + s_4_55);
        // D s_4_58: bit-extract s_4_53 s_4_52 s_4_57
        let s_4_58: Bits = (Bits::new(
            ((s_4_53) >> (s_4_52)).value(),
            u16::try_from(s_4_57).unwrap(),
        ));
        // D s_4_59: cast reint s_4_58 -> u8
        let s_4_59: bool = ((s_4_58.value()) != 0);
        // C s_4_60: const #16977u : u32
        let s_4_60: u32 = 16977;
        // N s_4_61: write-reg s_4_60 <= s_4_59
        let s_4_61: () = {
            state.write_register::<bool>(s_4_60 as isize, s_4_59);
            tracer.write_register(s_4_60 as isize, s_4_59);
        };
        // C s_4_62: const #32s : i64
        let s_4_62: i64 = 32;
        // C s_4_63: cast zx s_4_62 -> i
        let s_4_63: i128 = (i128::try_from(s_4_62).unwrap());
        // S s_4_64: call __UNKNOWN_bits(s_4_63)
        let s_4_64: Bits = u__UNKNOWN_bits(state, tracer, s_4_63);
        // S s_4_65: cast reint s_4_64 -> u32
        let s_4_65: u32 = (s_4_64.value() as u32);
        // S s_4_66: call DLR_write(s_4_65)
        let s_4_66: () = DLR_write(state, tracer, s_4_65);
        // C s_4_67: const #32s : i64
        let s_4_67: i64 = 32;
        // C s_4_68: cast zx s_4_67 -> i
        let s_4_68: i128 = (i128::try_from(s_4_67).unwrap());
        // S s_4_69: call __UNKNOWN_bits(s_4_68)
        let s_4_69: Bits = u__UNKNOWN_bits(state, tracer, s_4_68);
        // S s_4_70: cast reint s_4_69 -> u32
        let s_4_70: u32 = (s_4_69.value() as u32);
        // S s_4_71: call Mk_DSPSR_Type(s_4_70)
        let s_4_71: ProductType700c18a878c5601b = Mk_DSPSR_Type(state, tracer, s_4_70);
        // S s_4_72: call DSPSR_write(s_4_71)
        let s_4_72: () = DSPSR_write(state, tracer, s_4_71);
        // C s_4_73: const #() : ()
        let s_4_73: () = ();
        // S s_4_74: call SCTLR_read__2(s_4_73)
        let s_4_74: ProductType700c18a878c5601b = SCTLR_read__2(state, tracer, s_4_73);
        // S s_4_75: call _get_SCTLR_Type_EE(s_4_74)
        let s_4_75: bool = u_get_SCTLR_Type_EE(state, tracer, s_4_74);
        // C s_4_76: const #16974u : u32
        let s_4_76: u32 = 16974;
        // N s_4_77: write-reg s_4_76 <= s_4_75
        let s_4_77: () = {
            state.write_register::<bool>(s_4_76 as isize, s_4_75);
            tracer.write_register(s_4_76 as isize, s_4_75);
        };
        // C s_4_78: const #0u : u8
        let s_4_78: bool = false;
        // C s_4_79: const #16980u : u32
        let s_4_79: u32 = 16980;
        // N s_4_80: write-reg s_4_79 <= s_4_78
        let s_4_80: () = {
            state.write_register::<bool>(s_4_79 as isize, s_4_78);
            tracer.write_register(s_4_79 as isize, s_4_78);
        };
        // C s_4_81: const #0u : u8
        let s_4_81: u8 = 0;
        // C s_4_82: const #16981u : u32
        let s_4_82: u32 = 16981;
        // N s_4_83: write-reg s_4_82 <= s_4_81
        let s_4_83: () = {
            state.write_register::<u8>(s_4_82 as isize, s_4_81);
            tracer.write_register(s_4_82 as isize, s_4_81);
        };
        // C s_4_84: const #() : ()
        let s_4_84: () = ();
        // S s_4_85: call HavePANExt(s_4_84)
        let s_4_85: bool = HavePANExt(state, tracer, s_4_84);
        // N s_4_86: branch s_4_85 b13 b5
        if s_4_85 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#9025 <= s_5_0
        fn_state.gs_9025 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#9025:u8
        let s_6_0: bool = fn_state.gs_9025;
        // N s_6_1: branch s_6_0 b12 b7
        if s_6_0 {
            return block_12(state, tracer, fn_state);
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
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call HaveSSBSExt(s_8_0)
        let s_8_1: bool = HaveSSBSExt(state, tracer, s_8_0);
        // N s_8_2: branch s_8_1 b11 b9
        if s_8_1 {
            return block_11(state, tracer, fn_state);
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
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call EDSCR_read(s_10_0)
        let s_10_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_10_0);
        // C s_10_2: const #1u : u8
        let s_10_2: bool = true;
        // S s_10_3: call _update_EDSCR_Type_ERR(s_10_1, s_10_2)
        let s_10_3: ProductType700c18a878c5601b = u_update_EDSCR_Type_ERR(
            state,
            tracer,
            s_10_1,
            s_10_2,
        );
        // S s_10_4: call EDSCR_write(s_10_3)
        let s_10_4: () = EDSCR_write(state, tracer, s_10_3);
        // C s_10_5: const #() : ()
        let s_10_5: () = ();
        // S s_10_6: call UpdateEDSCRFields(s_10_5)
        let s_10_6: () = UpdateEDSCRFields(state, tracer, s_10_5);
        // C s_10_7: const #() : ()
        let s_10_7: () = ();
        // S s_10_8: call EndOfInstruction(s_10_7)
        let s_10_8: () = EndOfInstruction(state, tracer, s_10_7);
        // N s_10_9: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call __UNKNOWN_bit(s_11_0)
        let s_11_1: bool = u__UNKNOWN_bit(state, tracer, s_11_0);
        // C s_11_2: const #16992u : u32
        let s_11_2: u32 = 16992;
        // N s_11_3: write-reg s_11_2 <= s_11_1
        let s_11_3: () = {
            state.write_register::<bool>(s_11_2 as isize, s_11_1);
            tracer.write_register(s_11_2 as isize, s_11_1);
        };
        // N s_11_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // C s_12_1: const #16985u : u32
        let s_12_1: u32 = 16985;
        // N s_12_2: write-reg s_12_1 <= s_12_0
        let s_12_2: () = {
            state.write_register::<bool>(s_12_1 as isize, s_12_0);
            tracer.write_register(s_12_1 as isize, s_12_0);
        };
        // N s_12_3: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call SCTLR_read__2(s_13_0)
        let s_13_1: ProductType700c18a878c5601b = SCTLR_read__2(state, tracer, s_13_0);
        // S s_13_2: call _get_SCTLR_Type_SPAN(s_13_1)
        let s_13_2: bool = u_get_SCTLR_Type_SPAN(state, tracer, s_13_1);
        // S s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 1u16);
        // C s_13_4: const #0u : u8
        let s_13_4: bool = false;
        // C s_13_5: cast zx s_13_4 -> bv
        let s_13_5: Bits = Bits::new(s_13_4 as u128, 1u16);
        // S s_13_6: cmp-eq s_13_3 s_13_5
        let s_13_6: bool = ((s_13_3) == (s_13_5));
        // D s_13_7: write-var gs#9025 <= s_13_6
        fn_state.gs_9025 = s_13_6;
        // N s_13_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #20920u : u32
        let s_14_0: u32 = 20920;
        // D s_14_1: read-reg s_14_0:struct
        let s_14_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // C s_14_2: const #20920u : u32
        let s_14_2: u32 = 20920;
        // N s_14_3: write-reg s_14_2 <= s_14_1
        let s_14_3: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_14_2 as isize, s_14_1);
            tracer.write_register(s_14_2 as isize, s_14_1);
        };
        // N s_14_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #16975u : u32
        let s_15_0: u32 = 16975;
        // D s_15_1: read-reg s_15_0:u8
        let s_15_1: u8 = {
            let value = state.read_register::<u8>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // D s_15_2: cast zx s_15_1 -> bv
        let s_15_2: Bits = Bits::new(s_15_1 as u128, 2u16);
        // C s_15_3: const #432u : u32
        let s_15_3: u32 = 432;
        // D s_15_4: read-reg s_15_3:u8
        let s_15_4: u8 = {
            let value = state.read_register::<u8>(s_15_3 as isize);
            tracer.read_register(s_15_3 as isize, value);
            value
        };
        // D s_15_5: cast zx s_15_4 -> bv
        let s_15_5: Bits = Bits::new(s_15_4 as u128, 2u16);
        // D s_15_6: cmp-ne s_15_2 s_15_5
        let s_15_6: bool = ((s_15_2) != (s_15_5));
        // D s_15_7: write-var gs#9006 <= s_15_6
        fn_state.gs_9006 = s_15_6;
        // N s_15_8: jump b2
        return block_2(state, tracer, fn_state);
    }
}
