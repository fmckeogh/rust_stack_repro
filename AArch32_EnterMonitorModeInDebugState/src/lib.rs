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
use CurrentSecurityState::*;
use UpdateEDSCRFields::*;
use HavePANExt::*;
use u__UNKNOWN_bits::*;
use SPSR_set::*;
use SCTLR_read__2::*;
use R_set::*;
use HaveSSBSExt::*;
use DSPSR_write::*;
use AArch32_WriteMode::*;
use EndOfInstruction::*;
use ELUsingAArch32::*;
use SynchronizeContext::*;
use u_get_SCTLR_Type_EE::*;
use EDSCR_read::*;
use u_update_EDSCR_Type_ERR::*;
use common::*;
pub fn AArch32_EnterMonitorModeInDebugState<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_9096: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_9097: bool,
        from_secure: bool,
        gs_9096: (),
    }
    let fn_state = FunctionState {
        gs_9096,
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
        // C s_0_2: const #424u : u32
        let s_0_2: u32 = 424;
        // D s_0_3: read-reg s_0_2:u8
        let s_0_3: u8 = {
            let value = state.read_register::<u8>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // C s_0_4: const #2u : u8
        let s_0_4: u8 = 2;
        // D s_0_5: cmp-lt s_0_3 s_0_4
        let s_0_5: bool = ((s_0_3) < (s_0_4));
        // N s_0_6: branch s_0_5 b16 b1
        if s_0_5 {
            return block_16(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#9097 <= s_1_0
        fn_state.gs_9097 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#9097:u8
        let s_2_0: bool = fn_state.gs_9097;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // C s_2_2: const #() : ()
        let s_2_2: () = ();
        // S s_2_3: call CurrentSecurityState(s_2_2)
        let s_2_3: u32 = CurrentSecurityState(state, tracer, s_2_2);
        // C s_2_4: const #3u : u32
        let s_2_4: u32 = 3;
        // S s_2_5: cmp-eq s_2_3 s_2_4
        let s_2_5: bool = ((s_2_3) == (s_2_4));
        // D s_2_6: write-var from_secure <= s_2_5
        fn_state.from_secure = s_2_5;
        // C s_2_7: const #16983u : u32
        let s_2_7: u32 = 16983;
        // D s_2_8: read-reg s_2_7:u8
        let s_2_8: u8 = {
            let value = state.read_register::<u8>(s_2_7 as isize);
            tracer.read_register(s_2_7 as isize, value);
            value
        };
        // D s_2_9: cast zx s_2_8 -> bv
        let s_2_9: Bits = Bits::new(s_2_8 as u128, 5u16);
        // C s_2_10: const #384u : u32
        let s_2_10: u32 = 384;
        // D s_2_11: read-reg s_2_10:u8
        let s_2_11: u8 = {
            let value = state.read_register::<u8>(s_2_10 as isize);
            tracer.read_register(s_2_10 as isize, value);
            value
        };
        // D s_2_12: cast zx s_2_11 -> bv
        let s_2_12: Bits = Bits::new(s_2_11 as u128, 5u16);
        // D s_2_13: cmp-eq s_2_9 s_2_12
        let s_2_13: bool = ((s_2_9) == (s_2_12));
        // N s_2_14: branch s_2_13 b15 b3
        if s_2_13 {
            return block_15(state, tracer, fn_state);
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
        // C s_4_0: const #384u : u32
        let s_4_0: u32 = 384;
        // D s_4_1: read-reg s_4_0:u8
        let s_4_1: u8 = {
            let value = state.read_register::<u8>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: call AArch32_WriteMode(s_4_1)
        let s_4_2: () = AArch32_WriteMode(state, tracer, s_4_1);
        // C s_4_3: const #32s : i64
        let s_4_3: i64 = 32;
        // C s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // S s_4_5: call __UNKNOWN_bits(s_4_4)
        let s_4_5: Bits = u__UNKNOWN_bits(state, tracer, s_4_4);
        // S s_4_6: cast reint s_4_5 -> u32
        let s_4_6: u32 = (s_4_5.value() as u32);
        // C s_4_7: const #32s : i
        let s_4_7: i128 = 32;
        // S s_4_8: cast zx s_4_6 -> bv
        let s_4_8: Bits = Bits::new(s_4_6 as u128, 32u16);
        // S s_4_9: call SPSR_set(s_4_7, s_4_8)
        let s_4_9: () = SPSR_set(state, tracer, s_4_7, s_4_8);
        // C s_4_10: const #32s : i64
        let s_4_10: i64 = 32;
        // C s_4_11: cast zx s_4_10 -> i
        let s_4_11: i128 = (i128::try_from(s_4_10).unwrap());
        // S s_4_12: call __UNKNOWN_bits(s_4_11)
        let s_4_12: Bits = u__UNKNOWN_bits(state, tracer, s_4_11);
        // S s_4_13: cast reint s_4_12 -> u32
        let s_4_13: u32 = (s_4_12.value() as u32);
        // C s_4_14: const #14s : i
        let s_4_14: i128 = 14;
        // S s_4_15: call R_set(s_4_14, s_4_13)
        let s_4_15: () = R_set(state, tracer, s_4_14, s_4_13);
        // C s_4_16: const #1u : u8
        let s_4_16: bool = true;
        // C s_4_17: const #16993u : u32
        let s_4_17: u32 = 16993;
        // N s_4_18: write-reg s_4_17 <= s_4_16
        let s_4_18: () = {
            state.write_register::<bool>(s_4_17 as isize, s_4_16);
            tracer.write_register(s_4_17 as isize, s_4_16);
        };
        // C s_4_19: const #4s : i64
        let s_4_19: i64 = 4;
        // C s_4_20: cast zx s_4_19 -> i
        let s_4_20: i128 = (i128::try_from(s_4_19).unwrap());
        // S s_4_21: call __UNKNOWN_bits(s_4_20)
        let s_4_21: Bits = u__UNKNOWN_bits(state, tracer, s_4_20);
        // S s_4_22: cast reint s_4_21 -> u8
        let s_4_22: u8 = (s_4_21.value() as u8);
        // C s_4_23: const #3s : i
        let s_4_23: i128 = 3;
        // S s_4_24: cast zx s_4_22 -> bv
        let s_4_24: Bits = Bits::new(s_4_22 as u128, 4u16);
        // C s_4_25: const #1s : i64
        let s_4_25: i64 = 1;
        // C s_4_26: cast zx s_4_25 -> i
        let s_4_26: i128 = (i128::try_from(s_4_25).unwrap());
        // C s_4_27: const #0s : i
        let s_4_27: i128 = 0;
        // C s_4_28: add s_4_27 s_4_26
        let s_4_28: i128 = (s_4_27 + s_4_26);
        // D s_4_29: bit-extract s_4_24 s_4_23 s_4_28
        let s_4_29: Bits = (Bits::new(
            ((s_4_24) >> (s_4_23)).value(),
            u16::try_from(s_4_28).unwrap(),
        ));
        // D s_4_30: cast reint s_4_29 -> u8
        let s_4_30: bool = ((s_4_29.value()) != 0);
        // C s_4_31: const #16991u : u32
        let s_4_31: u32 = 16991;
        // N s_4_32: write-reg s_4_31 <= s_4_30
        let s_4_32: () = {
            state.write_register::<bool>(s_4_31 as isize, s_4_30);
            tracer.write_register(s_4_31 as isize, s_4_30);
        };
        // C s_4_33: const #2s : i
        let s_4_33: i128 = 2;
        // S s_4_34: cast zx s_4_22 -> bv
        let s_4_34: Bits = Bits::new(s_4_22 as u128, 4u16);
        // C s_4_35: const #1s : i64
        let s_4_35: i64 = 1;
        // C s_4_36: cast zx s_4_35 -> i
        let s_4_36: i128 = (i128::try_from(s_4_35).unwrap());
        // C s_4_37: const #0s : i
        let s_4_37: i128 = 0;
        // C s_4_38: add s_4_37 s_4_36
        let s_4_38: i128 = (s_4_37 + s_4_36);
        // D s_4_39: bit-extract s_4_34 s_4_33 s_4_38
        let s_4_39: Bits = (Bits::new(
            ((s_4_34) >> (s_4_33)).value(),
            u16::try_from(s_4_38).unwrap(),
        ));
        // D s_4_40: cast reint s_4_39 -> u8
        let s_4_40: bool = ((s_4_39.value()) != 0);
        // C s_4_41: const #16968u : u32
        let s_4_41: u32 = 16968;
        // N s_4_42: write-reg s_4_41 <= s_4_40
        let s_4_42: () = {
            state.write_register::<bool>(s_4_41 as isize, s_4_40);
            tracer.write_register(s_4_41 as isize, s_4_40);
        };
        // C s_4_43: const #1s : i
        let s_4_43: i128 = 1;
        // S s_4_44: cast zx s_4_22 -> bv
        let s_4_44: Bits = Bits::new(s_4_22 as u128, 4u16);
        // C s_4_45: const #1s : i64
        let s_4_45: i64 = 1;
        // C s_4_46: cast zx s_4_45 -> i
        let s_4_46: i128 = (i128::try_from(s_4_45).unwrap());
        // C s_4_47: const #0s : i
        let s_4_47: i128 = 0;
        // C s_4_48: add s_4_47 s_4_46
        let s_4_48: i128 = (s_4_47 + s_4_46);
        // D s_4_49: bit-extract s_4_44 s_4_43 s_4_48
        let s_4_49: Bits = (Bits::new(
            ((s_4_44) >> (s_4_43)).value(),
            u16::try_from(s_4_48).unwrap(),
        ));
        // D s_4_50: cast reint s_4_49 -> u8
        let s_4_50: bool = ((s_4_49.value()) != 0);
        // C s_4_51: const #16979u : u32
        let s_4_51: u32 = 16979;
        // N s_4_52: write-reg s_4_51 <= s_4_50
        let s_4_52: () = {
            state.write_register::<bool>(s_4_51 as isize, s_4_50);
            tracer.write_register(s_4_51 as isize, s_4_50);
        };
        // C s_4_53: const #0s : i
        let s_4_53: i128 = 0;
        // S s_4_54: cast zx s_4_22 -> bv
        let s_4_54: Bits = Bits::new(s_4_22 as u128, 4u16);
        // C s_4_55: const #1s : i64
        let s_4_55: i64 = 1;
        // C s_4_56: cast zx s_4_55 -> i
        let s_4_56: i128 = (i128::try_from(s_4_55).unwrap());
        // C s_4_57: const #0s : i
        let s_4_57: i128 = 0;
        // C s_4_58: add s_4_57 s_4_56
        let s_4_58: i128 = (s_4_57 + s_4_56);
        // D s_4_59: bit-extract s_4_54 s_4_53 s_4_58
        let s_4_59: Bits = (Bits::new(
            ((s_4_54) >> (s_4_53)).value(),
            u16::try_from(s_4_58).unwrap(),
        ));
        // D s_4_60: cast reint s_4_59 -> u8
        let s_4_60: bool = ((s_4_59.value()) != 0);
        // C s_4_61: const #16977u : u32
        let s_4_61: u32 = 16977;
        // N s_4_62: write-reg s_4_61 <= s_4_60
        let s_4_62: () = {
            state.write_register::<bool>(s_4_61 as isize, s_4_60);
            tracer.write_register(s_4_61 as isize, s_4_60);
        };
        // C s_4_63: const #() : ()
        let s_4_63: () = ();
        // S s_4_64: call SCTLR_read__2(s_4_63)
        let s_4_64: ProductType700c18a878c5601b = SCTLR_read__2(state, tracer, s_4_63);
        // S s_4_65: call _get_SCTLR_Type_EE(s_4_64)
        let s_4_65: bool = u_get_SCTLR_Type_EE(state, tracer, s_4_64);
        // C s_4_66: const #16974u : u32
        let s_4_66: u32 = 16974;
        // N s_4_67: write-reg s_4_66 <= s_4_65
        let s_4_67: () = {
            state.write_register::<bool>(s_4_66 as isize, s_4_65);
            tracer.write_register(s_4_66 as isize, s_4_65);
        };
        // C s_4_68: const #0u : u8
        let s_4_68: bool = false;
        // C s_4_69: const #16980u : u32
        let s_4_69: u32 = 16980;
        // N s_4_70: write-reg s_4_69 <= s_4_68
        let s_4_70: () = {
            state.write_register::<bool>(s_4_69 as isize, s_4_68);
            tracer.write_register(s_4_69 as isize, s_4_68);
        };
        // C s_4_71: const #0u : u8
        let s_4_71: u8 = 0;
        // C s_4_72: const #16981u : u32
        let s_4_72: u32 = 16981;
        // N s_4_73: write-reg s_4_72 <= s_4_71
        let s_4_73: () = {
            state.write_register::<u8>(s_4_72 as isize, s_4_71);
            tracer.write_register(s_4_72 as isize, s_4_71);
        };
        // C s_4_74: const #() : ()
        let s_4_74: () = ();
        // S s_4_75: call HavePANExt(s_4_74)
        let s_4_75: bool = HavePANExt(state, tracer, s_4_74);
        // N s_4_76: branch s_4_75 b10 b5
        if s_4_75 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call HaveSSBSExt(s_6_0)
        let s_6_1: bool = HaveSSBSExt(state, tracer, s_6_0);
        // N s_6_2: branch s_6_1 b9 b7
        if s_6_1 {
            return block_9(state, tracer, fn_state);
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
        // C s_8_0: const #32s : i64
        let s_8_0: i64 = 32;
        // C s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // S s_8_2: call __UNKNOWN_bits(s_8_1)
        let s_8_2: Bits = u__UNKNOWN_bits(state, tracer, s_8_1);
        // S s_8_3: cast reint s_8_2 -> u32
        let s_8_3: u32 = (s_8_2.value() as u32);
        // S s_8_4: call DLR_write(s_8_3)
        let s_8_4: () = DLR_write(state, tracer, s_8_3);
        // C s_8_5: const #32s : i64
        let s_8_5: i64 = 32;
        // C s_8_6: cast zx s_8_5 -> i
        let s_8_6: i128 = (i128::try_from(s_8_5).unwrap());
        // S s_8_7: call __UNKNOWN_bits(s_8_6)
        let s_8_7: Bits = u__UNKNOWN_bits(state, tracer, s_8_6);
        // S s_8_8: cast reint s_8_7 -> u32
        let s_8_8: u32 = (s_8_7.value() as u32);
        // S s_8_9: call Mk_DSPSR_Type(s_8_8)
        let s_8_9: ProductType700c18a878c5601b = Mk_DSPSR_Type(state, tracer, s_8_8);
        // S s_8_10: call DSPSR_write(s_8_9)
        let s_8_10: () = DSPSR_write(state, tracer, s_8_9);
        // C s_8_11: const #() : ()
        let s_8_11: () = ();
        // S s_8_12: call EDSCR_read(s_8_11)
        let s_8_12: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_8_11);
        // C s_8_13: const #1u : u8
        let s_8_13: bool = true;
        // S s_8_14: call _update_EDSCR_Type_ERR(s_8_12, s_8_13)
        let s_8_14: ProductType700c18a878c5601b = u_update_EDSCR_Type_ERR(
            state,
            tracer,
            s_8_12,
            s_8_13,
        );
        // S s_8_15: call EDSCR_write(s_8_14)
        let s_8_15: () = EDSCR_write(state, tracer, s_8_14);
        // C s_8_16: const #() : ()
        let s_8_16: () = ();
        // S s_8_17: call UpdateEDSCRFields(s_8_16)
        let s_8_17: () = UpdateEDSCRFields(state, tracer, s_8_16);
        // C s_8_18: const #() : ()
        let s_8_18: () = ();
        // S s_8_19: call EndOfInstruction(s_8_18)
        let s_8_19: () = EndOfInstruction(state, tracer, s_8_18);
        // N s_8_20: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call __UNKNOWN_bit(s_9_0)
        let s_9_1: bool = u__UNKNOWN_bit(state, tracer, s_9_0);
        // C s_9_2: const #16992u : u32
        let s_9_2: u32 = 16992;
        // N s_9_3: write-reg s_9_2 <= s_9_1
        let s_9_3: () = {
            state.write_register::<bool>(s_9_2 as isize, s_9_1);
            tracer.write_register(s_9_2 as isize, s_9_1);
        };
        // N s_9_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var from_secure:u8
        let s_10_0: bool = fn_state.from_secure;
        // D s_10_1: not s_10_0
        let s_10_1: bool = !s_10_0;
        // N s_10_2: branch s_10_1 b14 b11
        if s_10_1 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call SCTLR_read__2(s_11_0)
        let s_11_1: ProductType700c18a878c5601b = SCTLR_read__2(state, tracer, s_11_0);
        // S s_11_2: call _get_SCTLR_Type_SPAN(s_11_1)
        let s_11_2: bool = u_get_SCTLR_Type_SPAN(state, tracer, s_11_1);
        // S s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 1u16);
        // C s_11_4: const #0u : u8
        let s_11_4: bool = false;
        // C s_11_5: cast zx s_11_4 -> bv
        let s_11_5: Bits = Bits::new(s_11_4 as u128, 1u16);
        // S s_11_6: cmp-eq s_11_3 s_11_5
        let s_11_6: bool = ((s_11_3) == (s_11_5));
        // N s_11_7: branch s_11_6 b13 b12
        if s_11_6 {
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
        // N s_12_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // C s_13_1: const #16985u : u32
        let s_13_1: u32 = 16985;
        // N s_13_2: write-reg s_13_1 <= s_13_0
        let s_13_2: () = {
            state.write_register::<bool>(s_13_1 as isize, s_13_0);
            tracer.write_register(s_13_1 as isize, s_13_0);
        };
        // N s_13_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // C s_14_1: const #16985u : u32
        let s_14_1: u32 = 16985;
        // N s_14_2: write-reg s_14_1 <= s_14_0
        let s_14_2: () = {
            state.write_register::<bool>(s_14_1 as isize, s_14_0);
            tracer.write_register(s_14_1 as isize, s_14_0);
        };
        // N s_14_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #20920u : u32
        let s_15_0: u32 = 20920;
        // D s_15_1: read-reg s_15_0:struct
        let s_15_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // C s_15_2: const #20920u : u32
        let s_15_2: u32 = 20920;
        // N s_15_3: write-reg s_15_2 <= s_15_1
        let s_15_3: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_15_2 as isize, s_15_1);
            tracer.write_register(s_15_2 as isize, s_15_1);
        };
        // N s_15_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #424u : u32
        let s_16_0: u32 = 424;
        // D s_16_1: read-reg s_16_0:u8
        let s_16_1: u8 = {
            let value = state.read_register::<u8>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // D s_16_2: call ELUsingAArch32(s_16_1)
        let s_16_2: bool = ELUsingAArch32(state, tracer, s_16_1);
        // D s_16_3: write-var gs#9097 <= s_16_2
        fn_state.gs_9097 = s_16_2;
        // N s_16_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
