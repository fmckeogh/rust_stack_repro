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
use u_get_SCTLR_Type_SPAN::*;
use Mk_DSPSR_Type::*;
use u_get_SCTLR_EL2_Type_SPAN::*;
use u_get_SCTLRType_EE::*;
use HSR_write::*;
use UpdateEDSCRFields::*;
use HaveGCS::*;
use Mk_DSPSR_EL0_Type::*;
use HaveSME::*;
use u__UNKNOWN_bits::*;
use DSPSR_write::*;
use u_get_SCR_EL3_Type_NMEA::*;
use SCTLR_read__1::*;
use ResetSVEState::*;
use LR_write::*;
use AArch32_WriteMode::*;
use SCTLR_read__2::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_SCTLR_EL1_Type_SPAN::*;
use EffectiveEA::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_TGE::*;
use AArch64_MaybeZeroRegisterUppers::*;
use MaybeZeroSVEUppers::*;
use u_get_HCR_EL2_Type_E2H::*;
use DLR_write::*;
use ConstrainUnpredictableBool::*;
use Mk_ESRType::*;
use HaveDoubleFaultExt::*;
use ELR_hyp_write::*;
use ESR_set__1::*;
use CurrentSecurityState::*;
use u_get_SCTLRType_IESB::*;
use IsSecureEL2Enabled::*;
use HavePANExt::*;
use Unreachable::*;
use ELR_set__1::*;
use SPSR_set::*;
use HaveUAOExt::*;
use ELUsingAArch32::*;
use HaveMTEExt::*;
use HaveIESB::*;
use SynchronizeErrors::*;
use UsingAArch32::*;
use SynchronizeContext::*;
use EDSCR_read::*;
use Mk_HSR_Type::*;
use common::*;
pub fn DCPSInstruction<T: Tracer>(state: &mut State, tracer: &T, target_el: u8) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_29387: bool,
        gs_29374: bool,
        gs_29386: bool,
        gs_29380: bool,
        gs_29367: bool,
        gs_29373: bool,
        gs_29441: bool,
        gs_29409: bool,
        from_secure: bool,
        handle_el: u8,
        from_32shadow_541: bool,
        gs_29382: bool,
        gs_29385: bool,
        gs_29372: bool,
        gs_29437: bool,
        gs_29443: bool,
        gs_29381: bool,
        gs_29384: bool,
        gs_29436: bool,
        sync_errors: bool,
        gs_29383: bool,
        gs_29368: bool,
        gs_29442: bool,
        gs_29370: bool,
        target_el: u8,
    }
    let fn_state = FunctionState {
        target_el,
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
        // D s_0_2: read-var target_el:u8
        let s_0_2: u8 = fn_state.target_el;
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 2u16);
        // C s_0_4: const #440u : u32
        let s_0_4: u32 = 440;
        // D s_0_5: read-reg s_0_4:u8
        let s_0_5: u8 = {
            let value = state.read_register::<u8>(s_0_4 as isize);
            tracer.read_register(s_0_4 as isize, value);
            value
        };
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 2u16);
        // D s_0_7: cmp-eq s_0_3 s_0_6
        let s_0_7: bool = ((s_0_3) == (s_0_6));
        // D s_0_8: not s_0_7
        let s_0_8: bool = !s_0_7;
        // N s_0_9: branch s_0_8 b110 b1
        if s_0_8 {
            return block_110(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #16975u : u32
        let s_1_0: u32 = 16975;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 2u16);
        // C s_1_3: const #432u : u32
        let s_1_3: u32 = 432;
        // D s_1_4: read-reg s_1_3:u8
        let s_1_4: u8 = {
            let value = state.read_register::<u8>(s_1_3 as isize);
            tracer.read_register(s_1_3 as isize, value);
            value
        };
        // D s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 2u16);
        // D s_1_6: cmp-eq s_1_2 s_1_5
        let s_1_6: bool = ((s_1_2) == (s_1_5));
        // N s_1_7: branch s_1_6 b109 b2
        if s_1_6 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #16975u : u32
        let s_2_0: u32 = 16975;
        // D s_2_1: read-reg s_2_0:u8
        let s_2_1: u8 = {
            let value = state.read_register::<u8>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 2u16);
        // C s_2_3: const #424u : u32
        let s_2_3: u32 = 424;
        // D s_2_4: read-reg s_2_3:u8
        let s_2_4: u8 = {
            let value = state.read_register::<u8>(s_2_3 as isize);
            tracer.read_register(s_2_3 as isize, value);
            value
        };
        // D s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 2u16);
        // D s_2_6: cmp-eq s_2_2 s_2_5
        let s_2_6: bool = ((s_2_2) == (s_2_5));
        // N s_2_7: branch s_2_6 b108 b3
        if s_2_6 {
            return block_108(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#29367 <= s_3_0
        fn_state.gs_29367 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#29367:u8
        let s_4_0: bool = fn_state.gs_29367;
        // D s_4_1: write-var gs#29368 <= s_4_0
        fn_state.gs_29368 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#29368:u8
        let s_5_0: bool = fn_state.gs_29368;
        // N s_5_1: branch s_5_0 b107 b6
        if s_5_0 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call EL2Enabled(s_6_0)
        let s_6_1: bool = EL2Enabled(state, tracer, s_6_0);
        // N s_6_2: branch s_6_1 b106 b7
        if s_6_1 {
            return block_106(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#29370 <= s_7_0
        fn_state.gs_29370 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#29370:u8
        let s_8_0: bool = fn_state.gs_29370;
        // N s_8_1: branch s_8_0 b105 b9
        if s_8_0 {
            return block_105(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #440u : u32
        let s_9_0: u32 = 440;
        // D s_9_1: read-reg s_9_0:u8
        let s_9_1: u8 = {
            let value = state.read_register::<u8>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // D s_9_2: write-var handle_el <= s_9_1
        fn_state.handle_el = s_9_1;
        // N s_9_3: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call CurrentSecurityState(s_10_0)
        let s_10_1: u32 = CurrentSecurityState(state, tracer, s_10_0);
        // C s_10_2: const #3u : u32
        let s_10_2: u32 = 3;
        // S s_10_3: cmp-eq s_10_1 s_10_2
        let s_10_3: bool = ((s_10_1) == (s_10_2));
        // D s_10_4: write-var from_secure <= s_10_3
        fn_state.from_secure = s_10_3;
        // D s_10_5: read-var handle_el:u8
        let s_10_5: u8 = fn_state.handle_el;
        // D s_10_6: call ELUsingAArch32(s_10_5)
        let s_10_6: bool = ELUsingAArch32(state, tracer, s_10_5);
        // N s_10_7: branch s_10_6 b78 b11
        if s_10_6 {
            return block_78(state, tracer, fn_state);
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
        // S s_11_1: call UsingAArch32(s_11_0)
        let s_11_1: bool = UsingAArch32(state, tracer, s_11_0);
        // D s_11_2: write-var from_32shadow#541 <= s_11_1
        fn_state.from_32shadow_541 = s_11_1;
        // D s_11_3: read-var from_32shadow#541:u8
        let s_11_3: bool = fn_state.from_32shadow_541;
        // N s_11_4: branch s_11_3 b77 b12
        if s_11_3 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var from_32shadow#541:u8
        let s_13_0: bool = fn_state.from_32shadow_541;
        // N s_13_1: branch s_13_0 b76 b14
        if s_13_0 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#29380 <= s_14_0
        fn_state.gs_29380 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#29380:u8
        let s_15_0: bool = fn_state.gs_29380;
        // N s_15_1: branch s_15_0 b75 b16
        if s_15_0 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#29381 <= s_16_0
        fn_state.gs_29381 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#29381:u8
        let s_17_0: bool = fn_state.gs_29381;
        // N s_17_1: branch s_17_0 b74 b18
        if s_17_0 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var target_el:u8
        let s_18_0: u8 = fn_state.target_el;
        // D s_18_1: call MaybeZeroSVEUppers(s_18_0)
        let s_18_1: () = MaybeZeroSVEUppers(state, tracer, s_18_0);
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // C s_19_1: const #16999u : u32
        let s_19_1: u32 = 16999;
        // N s_19_2: write-reg s_19_1 <= s_19_0
        let s_19_2: () = {
            state.write_register::<bool>(s_19_1 as isize, s_19_0);
            tracer.write_register(s_19_1 as isize, s_19_0);
        };
        // C s_19_3: const #1u : u8
        let s_19_3: bool = true;
        // C s_19_4: const #16990u : u32
        let s_19_4: u32 = 16990;
        // N s_19_5: write-reg s_19_4 <= s_19_3
        let s_19_5: () = {
            state.write_register::<bool>(s_19_4 as isize, s_19_3);
            tracer.write_register(s_19_4 as isize, s_19_3);
        };
        // D s_19_6: read-var handle_el:u8
        let s_19_6: u8 = fn_state.handle_el;
        // C s_19_7: const #16975u : u32
        let s_19_7: u32 = 16975;
        // N s_19_8: write-reg s_19_7 <= s_19_6
        let s_19_8: () = {
            state.write_register::<u8>(s_19_7 as isize, s_19_6);
            tracer.write_register(s_19_7 as isize, s_19_6);
        };
        // C s_19_9: const #() : ()
        let s_19_9: () = ();
        // S s_19_10: call HavePANExt(s_19_9)
        let s_19_10: bool = HavePANExt(state, tracer, s_19_9);
        // N s_19_11: branch s_19_10 b58 b20
        if s_19_10 {
            return block_58(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#29387 <= s_20_0
        fn_state.gs_29387 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#29387:u8
        let s_21_0: bool = fn_state.gs_29387;
        // N s_21_1: branch s_21_0 b57 b22
        if s_21_0 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #64s : i64
        let s_23_0: i64 = 64;
        // C s_23_1: cast zx s_23_0 -> i
        let s_23_1: i128 = (i128::try_from(s_23_0).unwrap());
        // S s_23_2: call __UNKNOWN_bits(s_23_1)
        let s_23_2: Bits = u__UNKNOWN_bits(state, tracer, s_23_1);
        // S s_23_3: cast reint s_23_2 -> u64
        let s_23_3: u64 = (s_23_2.value() as u64);
        // S s_23_4: call ELR_set__1(s_23_3)
        let s_23_4: () = ELR_set__1(state, tracer, s_23_3);
        // C s_23_5: const #64s : i64
        let s_23_5: i64 = 64;
        // C s_23_6: cast zx s_23_5 -> i
        let s_23_6: i128 = (i128::try_from(s_23_5).unwrap());
        // S s_23_7: call __UNKNOWN_bits(s_23_6)
        let s_23_7: Bits = u__UNKNOWN_bits(state, tracer, s_23_6);
        // S s_23_8: cast reint s_23_7 -> u64
        let s_23_8: u64 = (s_23_7.value() as u64);
        // C s_23_9: const #64s : i
        let s_23_9: i128 = 64;
        // S s_23_10: cast zx s_23_8 -> bv
        let s_23_10: Bits = Bits::new(s_23_8 as u128, 64u16);
        // S s_23_11: call SPSR_set(s_23_9, s_23_10)
        let s_23_11: () = SPSR_set(state, tracer, s_23_9, s_23_10);
        // C s_23_12: const #64s : i64
        let s_23_12: i64 = 64;
        // C s_23_13: cast zx s_23_12 -> i
        let s_23_13: i128 = (i128::try_from(s_23_12).unwrap());
        // S s_23_14: call __UNKNOWN_bits(s_23_13)
        let s_23_14: Bits = u__UNKNOWN_bits(state, tracer, s_23_13);
        // S s_23_15: cast reint s_23_14 -> u64
        let s_23_15: u64 = (s_23_14.value() as u64);
        // S s_23_16: call Mk_ESRType(s_23_15)
        let s_23_16: ProductType5c790c8ef59cc8b2 = Mk_ESRType(state, tracer, s_23_15);
        // S s_23_17: call ESR_set__1(s_23_16)
        let s_23_17: () = ESR_set__1(state, tracer, s_23_16);
        // C s_23_18: const #64s : i64
        let s_23_18: i64 = 64;
        // C s_23_19: cast zx s_23_18 -> i
        let s_23_19: i128 = (i128::try_from(s_23_18).unwrap());
        // S s_23_20: call __UNKNOWN_bits(s_23_19)
        let s_23_20: Bits = u__UNKNOWN_bits(state, tracer, s_23_19);
        // S s_23_21: cast reint s_23_20 -> u64
        let s_23_21: u64 = (s_23_20.value() as u64);
        // C s_23_22: const #13360u : u32
        let s_23_22: u32 = 13360;
        // N s_23_23: write-reg s_23_22 <= s_23_21
        let s_23_23: () = {
            state.write_register::<u64>(s_23_22 as isize, s_23_21);
            tracer.write_register(s_23_22 as isize, s_23_21);
        };
        // C s_23_24: const #64s : i64
        let s_23_24: i64 = 64;
        // C s_23_25: cast zx s_23_24 -> i
        let s_23_25: i128 = (i128::try_from(s_23_24).unwrap());
        // S s_23_26: call __UNKNOWN_bits(s_23_25)
        let s_23_26: Bits = u__UNKNOWN_bits(state, tracer, s_23_25);
        // S s_23_27: cast reint s_23_26 -> u64
        let s_23_27: u64 = (s_23_26.value() as u64);
        // S s_23_28: call Mk_DSPSR_EL0_Type(s_23_27)
        let s_23_28: ProductType5c790c8ef59cc8b2 = Mk_DSPSR_EL0_Type(
            state,
            tracer,
            s_23_27,
        );
        // C s_23_29: const #102584u : u32
        let s_23_29: u32 = 102584;
        // N s_23_30: write-reg s_23_29 <= s_23_28
        let s_23_30: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_23_29 as isize, s_23_28);
            tracer.write_register(s_23_29 as isize, s_23_28);
        };
        // C s_23_31: const #() : ()
        let s_23_31: () = ();
        // S s_23_32: call HaveUAOExt(s_23_31)
        let s_23_32: bool = HaveUAOExt(state, tracer, s_23_31);
        // N s_23_33: branch s_23_32 b56 b24
        if s_23_32 {
            return block_56(state, tracer, fn_state);
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
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call HaveMTEExt(s_25_0)
        let s_25_1: bool = HaveMTEExt(state, tracer, s_25_0);
        // N s_25_2: branch s_25_1 b55 b26
        if s_25_1 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_26_0: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #() : ()
        let s_27_0: () = ();
        // S s_27_1: call HaveGCS(s_27_0)
        let s_27_1: bool = HaveGCS(state, tracer, s_27_0);
        // N s_27_2: branch s_27_1 b54 b28
        if s_27_1 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_28_0: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_29_0: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #() : ()
        let s_30_0: () = ();
        // S s_30_1: call UpdateEDSCRFields(s_30_0)
        let s_30_1: () = UpdateEDSCRFields(state, tracer, s_30_0);
        // C s_30_2: const #() : ()
        let s_30_2: () = ();
        // S s_30_3: call HaveIESB(s_30_2)
        let s_30_3: bool = HaveIESB(state, tracer, s_30_2);
        // N s_30_4: branch s_30_3 b53 b31
        if s_30_3 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #0u : u8
        let s_31_0: bool = false;
        // D s_31_1: write-var gs#29436 <= s_31_0
        fn_state.gs_29436 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#29436:u8
        let s_32_0: bool = fn_state.gs_29436;
        // D s_32_1: write-var sync_errors <= s_32_0
        fn_state.sync_errors = s_32_0;
        // C s_32_2: const #() : ()
        let s_32_2: () = ();
        // S s_32_3: call HaveDoubleFaultExt(s_32_2)
        let s_32_3: bool = HaveDoubleFaultExt(state, tracer, s_32_2);
        // N s_32_4: branch s_32_3 b52 b33
        if s_32_3 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #0u : u8
        let s_33_0: bool = false;
        // D s_33_1: write-var gs#29437 <= s_33_0
        fn_state.gs_29437 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#29437:u8
        let s_34_0: bool = fn_state.gs_29437;
        // N s_34_1: branch s_34_0 b42 b35
        if s_34_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_35_0: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #59u : u32
        let s_36_0: u32 = 59;
        // S s_36_1: call ConstrainUnpredictableBool(s_36_0)
        let s_36_1: bool = ConstrainUnpredictableBool(state, tracer, s_36_0);
        // S s_36_2: not s_36_1
        let s_36_2: bool = !s_36_1;
        // N s_36_3: branch s_36_2 b41 b37
        if s_36_2 {
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
        // N s_37_0: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var sync_errors:u8
        let s_38_0: bool = fn_state.sync_errors;
        // N s_38_1: branch s_38_0 b40 b39
        if s_38_0 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_39_0: return
        return;
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #() : ()
        let s_40_0: () = ();
        // S s_40_1: call SynchronizeErrors(s_40_0)
        let s_40_1: () = SynchronizeErrors(state, tracer, s_40_0);
        // N s_40_2: return
        return;
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #0u : u8
        let s_41_0: bool = false;
        // D s_41_1: write-var sync_errors <= s_41_0
        fn_state.sync_errors = s_41_0;
        // N s_41_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var sync_errors:u8
        let s_42_0: bool = fn_state.sync_errors;
        // N s_42_1: branch s_42_0 b51 b43
        if s_42_0 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #() : ()
        let s_43_0: () = ();
        // S s_43_1: call EffectiveEA(s_43_0)
        let s_43_1: bool = EffectiveEA(state, tracer, s_43_0);
        // S s_43_2: cast zx s_43_1 -> bv
        let s_43_2: Bits = Bits::new(s_43_1 as u128, 1u16);
        // C s_43_3: const #1u : u8
        let s_43_3: bool = true;
        // C s_43_4: cast zx s_43_3 -> bv
        let s_43_4: Bits = Bits::new(s_43_3 as u128, 1u16);
        // S s_43_5: cmp-eq s_43_2 s_43_4
        let s_43_5: bool = ((s_43_2) == (s_43_4));
        // N s_43_6: branch s_43_5 b50 b44
        if s_43_5 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #0u : u8
        let s_44_0: bool = false;
        // D s_44_1: write-var gs#29441 <= s_44_0
        fn_state.gs_29441 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#29441:u8
        let s_45_0: bool = fn_state.gs_29441;
        // N s_45_1: branch s_45_0 b49 b46
        if s_45_0 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #0u : u8
        let s_46_0: bool = false;
        // D s_46_1: write-var gs#29442 <= s_46_0
        fn_state.gs_29442 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#29442:u8
        let s_47_0: bool = fn_state.gs_29442;
        // D s_47_1: write-var gs#29443 <= s_47_0
        fn_state.gs_29443 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#29443:u8
        let s_48_0: bool = fn_state.gs_29443;
        // D s_48_1: write-var sync_errors <= s_48_0
        fn_state.sync_errors = s_48_0;
        // N s_48_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #16975u : u32
        let s_49_0: u32 = 16975;
        // D s_49_1: read-reg s_49_0:u8
        let s_49_1: u8 = {
            let value = state.read_register::<u8>(s_49_0 as isize);
            tracer.read_register(s_49_0 as isize, value);
            value
        };
        // D s_49_2: cast zx s_49_1 -> bv
        let s_49_2: Bits = Bits::new(s_49_1 as u128, 2u16);
        // C s_49_3: const #424u : u32
        let s_49_3: u32 = 424;
        // D s_49_4: read-reg s_49_3:u8
        let s_49_4: u8 = {
            let value = state.read_register::<u8>(s_49_3 as isize);
            tracer.read_register(s_49_3 as isize, value);
            value
        };
        // D s_49_5: cast zx s_49_4 -> bv
        let s_49_5: Bits = Bits::new(s_49_4 as u128, 2u16);
        // D s_49_6: cmp-eq s_49_2 s_49_5
        let s_49_6: bool = ((s_49_2) == (s_49_5));
        // D s_49_7: write-var gs#29442 <= s_49_6
        fn_state.gs_29442 = s_49_6;
        // N s_49_8: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #90704u : u32
        let s_50_0: u32 = 90704;
        // D s_50_1: read-reg s_50_0:struct
        let s_50_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_50_0 as isize);
            tracer.read_register(s_50_0 as isize, value);
            value
        };
        // D s_50_2: call _get_SCR_EL3_Type_NMEA(s_50_1)
        let s_50_2: bool = u_get_SCR_EL3_Type_NMEA(state, tracer, s_50_1);
        // D s_50_3: cast zx s_50_2 -> bv
        let s_50_3: Bits = Bits::new(s_50_2 as u128, 1u16);
        // C s_50_4: const #1u : u8
        let s_50_4: bool = true;
        // C s_50_5: cast zx s_50_4 -> bv
        let s_50_5: Bits = Bits::new(s_50_4 as u128, 1u16);
        // D s_50_6: cmp-eq s_50_3 s_50_5
        let s_50_6: bool = ((s_50_3) == (s_50_5));
        // D s_50_7: write-var gs#29441 <= s_50_6
        fn_state.gs_29441 = s_50_6;
        // N s_50_8: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #1u : u8
        let s_51_0: bool = true;
        // D s_51_1: write-var gs#29443 <= s_51_0
        fn_state.gs_29443 = s_51_0;
        // N s_51_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #() : ()
        let s_52_0: () = ();
        // S s_52_1: call UsingAArch32(s_52_0)
        let s_52_1: bool = UsingAArch32(state, tracer, s_52_0);
        // S s_52_2: not s_52_1
        let s_52_2: bool = !s_52_1;
        // D s_52_3: write-var gs#29437 <= s_52_2
        fn_state.gs_29437 = s_52_2;
        // N s_52_4: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #() : ()
        let s_53_0: () = ();
        // S s_53_1: call SCTLR_read__1(s_53_0)
        let s_53_1: ProductType5c790c8ef59cc8b2 = SCTLR_read__1(state, tracer, s_53_0);
        // S s_53_2: call _get_SCTLRType_IESB(s_53_1)
        let s_53_2: bool = u_get_SCTLRType_IESB(state, tracer, s_53_1);
        // S s_53_3: cast zx s_53_2 -> bv
        let s_53_3: Bits = Bits::new(s_53_2 as u128, 1u16);
        // C s_53_4: const #1u : u8
        let s_53_4: bool = true;
        // C s_53_5: cast zx s_53_4 -> bv
        let s_53_5: Bits = Bits::new(s_53_4 as u128, 1u16);
        // S s_53_6: cmp-eq s_53_3 s_53_5
        let s_53_6: bool = ((s_53_3) == (s_53_5));
        // D s_53_7: write-var gs#29436 <= s_53_6
        fn_state.gs_29436 = s_53_6;
        // N s_53_8: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #0u : u8
        let s_54_0: bool = false;
        // C s_54_1: const #16976u : u32
        let s_54_1: u32 = 16976;
        // N s_54_2: write-reg s_54_1 <= s_54_0
        let s_54_2: () = {
            state.write_register::<bool>(s_54_1 as isize, s_54_0);
            tracer.write_register(s_54_1 as isize, s_54_0);
        };
        // N s_54_3: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #1u : u8
        let s_55_0: bool = true;
        // C s_55_1: const #16994u : u32
        let s_55_1: u32 = 16994;
        // N s_55_2: write-reg s_55_1 <= s_55_0
        let s_55_2: () = {
            state.write_register::<bool>(s_55_1 as isize, s_55_0);
            tracer.write_register(s_55_1 as isize, s_55_0);
        };
        // N s_55_3: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #0u : u8
        let s_56_0: bool = false;
        // C s_56_1: const #16995u : u32
        let s_56_1: u32 = 16995;
        // N s_56_2: write-reg s_56_1 <= s_56_0
        let s_56_2: () = {
            state.write_register::<bool>(s_56_1 as isize, s_56_0);
            tracer.write_register(s_56_1 as isize, s_56_0);
        };
        // N s_56_3: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #1u : u8
        let s_57_0: bool = true;
        // C s_57_1: const #16985u : u32
        let s_57_1: u32 = 16985;
        // N s_57_2: write-reg s_57_1 <= s_57_0
        let s_57_2: () = {
            state.write_register::<bool>(s_57_1 as isize, s_57_0);
            tracer.write_register(s_57_1 as isize, s_57_0);
        };
        // N s_57_3: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var handle_el:u8
        let s_58_0: u8 = fn_state.handle_el;
        // D s_58_1: cast zx s_58_0 -> bv
        let s_58_1: Bits = Bits::new(s_58_0 as u128, 2u16);
        // C s_58_2: const #440u : u32
        let s_58_2: u32 = 440;
        // D s_58_3: read-reg s_58_2:u8
        let s_58_3: u8 = {
            let value = state.read_register::<u8>(s_58_2 as isize);
            tracer.read_register(s_58_2 as isize, value);
            value
        };
        // D s_58_4: cast zx s_58_3 -> bv
        let s_58_4: Bits = Bits::new(s_58_3 as u128, 2u16);
        // D s_58_5: cmp-eq s_58_1 s_58_4
        let s_58_5: bool = ((s_58_1) == (s_58_4));
        // N s_58_6: branch s_58_5 b73 b59
        if s_58_5 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #0u : u8
        let s_59_0: bool = false;
        // D s_59_1: write-var gs#29382 <= s_59_0
        fn_state.gs_29382 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#29382:u8
        let s_60_0: bool = fn_state.gs_29382;
        // N s_60_1: branch s_60_0 b72 b61
        if s_60_0 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var handle_el:u8
        let s_61_0: u8 = fn_state.handle_el;
        // D s_61_1: cast zx s_61_0 -> bv
        let s_61_1: Bits = Bits::new(s_61_0 as u128, 2u16);
        // C s_61_2: const #432u : u32
        let s_61_2: u32 = 432;
        // D s_61_3: read-reg s_61_2:u8
        let s_61_3: u8 = {
            let value = state.read_register::<u8>(s_61_2 as isize);
            tracer.read_register(s_61_2 as isize, value);
            value
        };
        // D s_61_4: cast zx s_61_3 -> bv
        let s_61_4: Bits = Bits::new(s_61_3 as u128, 2u16);
        // D s_61_5: cmp-eq s_61_1 s_61_4
        let s_61_5: bool = ((s_61_1) == (s_61_4));
        // N s_61_6: branch s_61_5 b71 b62
        if s_61_5 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #0u : u8
        let s_62_0: bool = false;
        // D s_62_1: write-var gs#29383 <= s_62_0
        fn_state.gs_29383 = s_62_0;
        // N s_62_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var gs#29383:u8
        let s_63_0: bool = fn_state.gs_29383;
        // N s_63_1: branch s_63_0 b70 b64
        if s_63_0 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #0u : u8
        let s_64_0: bool = false;
        // D s_64_1: write-var gs#29384 <= s_64_0
        fn_state.gs_29384 = s_64_0;
        // N s_64_2: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var gs#29384:u8
        let s_65_0: bool = fn_state.gs_29384;
        // N s_65_1: branch s_65_0 b69 b66
        if s_65_0 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_66(state, tracer, fn_state);
        };
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #0u : u8
        let s_66_0: bool = false;
        // D s_66_1: write-var gs#29385 <= s_66_0
        fn_state.gs_29385 = s_66_0;
        // N s_66_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#29385:u8
        let s_67_0: bool = fn_state.gs_29385;
        // D s_67_1: write-var gs#29386 <= s_67_0
        fn_state.gs_29386 = s_67_0;
        // N s_67_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var gs#29386:u8
        let s_68_0: bool = fn_state.gs_29386;
        // D s_68_1: write-var gs#29387 <= s_68_0
        fn_state.gs_29387 = s_68_0;
        // N s_68_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #20784u : u32
        let s_69_0: u32 = 20784;
        // D s_69_1: read-reg s_69_0:struct
        let s_69_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_69_0 as isize);
            tracer.read_register(s_69_0 as isize, value);
            value
        };
        // D s_69_2: call _get_SCTLR_EL2_Type_SPAN(s_69_1)
        let s_69_2: bool = u_get_SCTLR_EL2_Type_SPAN(state, tracer, s_69_1);
        // D s_69_3: cast zx s_69_2 -> bv
        let s_69_3: Bits = Bits::new(s_69_2 as u128, 1u16);
        // C s_69_4: const #0u : u8
        let s_69_4: bool = false;
        // C s_69_5: cast zx s_69_4 -> bv
        let s_69_5: Bits = Bits::new(s_69_4 as u128, 1u16);
        // D s_69_6: cmp-eq s_69_3 s_69_5
        let s_69_6: bool = ((s_69_3) == (s_69_5));
        // D s_69_7: write-var gs#29385 <= s_69_6
        fn_state.gs_29385 = s_69_6;
        // N s_69_8: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #102552u : u32
        let s_70_0: u32 = 102552;
        // D s_70_1: read-reg s_70_0:struct
        let s_70_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_70_0 as isize);
            tracer.read_register(s_70_0 as isize, value);
            value
        };
        // D s_70_2: call _get_HCR_EL2_Type_TGE(s_70_1)
        let s_70_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_70_1);
        // D s_70_3: cast zx s_70_2 -> bv
        let s_70_3: Bits = Bits::new(s_70_2 as u128, 1u16);
        // C s_70_4: const #1u : u8
        let s_70_4: bool = true;
        // C s_70_5: cast zx s_70_4 -> bv
        let s_70_5: Bits = Bits::new(s_70_4 as u128, 1u16);
        // D s_70_6: cmp-eq s_70_3 s_70_5
        let s_70_6: bool = ((s_70_3) == (s_70_5));
        // D s_70_7: write-var gs#29384 <= s_70_6
        fn_state.gs_29384 = s_70_6;
        // N s_70_8: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #102552u : u32
        let s_71_0: u32 = 102552;
        // D s_71_1: read-reg s_71_0:struct
        let s_71_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_71_0 as isize);
            tracer.read_register(s_71_0 as isize, value);
            value
        };
        // D s_71_2: call _get_HCR_EL2_Type_E2H(s_71_1)
        let s_71_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_71_1);
        // D s_71_3: cast zx s_71_2 -> bv
        let s_71_3: Bits = Bits::new(s_71_2 as u128, 1u16);
        // C s_71_4: const #1u : u8
        let s_71_4: bool = true;
        // C s_71_5: cast zx s_71_4 -> bv
        let s_71_5: Bits = Bits::new(s_71_4 as u128, 1u16);
        // D s_71_6: cmp-eq s_71_3 s_71_5
        let s_71_6: bool = ((s_71_3) == (s_71_5));
        // D s_71_7: write-var gs#29383 <= s_71_6
        fn_state.gs_29383 = s_71_6;
        // N s_71_8: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #1u : u8
        let s_72_0: bool = true;
        // D s_72_1: write-var gs#29386 <= s_72_0
        fn_state.gs_29386 = s_72_0;
        // N s_72_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #90272u : u32
        let s_73_0: u32 = 90272;
        // D s_73_1: read-reg s_73_0:struct
        let s_73_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_73_0 as isize);
            tracer.read_register(s_73_0 as isize, value);
            value
        };
        // D s_73_2: call _get_SCTLR_EL1_Type_SPAN(s_73_1)
        let s_73_2: bool = u_get_SCTLR_EL1_Type_SPAN(state, tracer, s_73_1);
        // D s_73_3: cast zx s_73_2 -> bv
        let s_73_3: Bits = Bits::new(s_73_2 as u128, 1u16);
        // C s_73_4: const #0u : u8
        let s_73_4: bool = false;
        // C s_73_5: cast zx s_73_4 -> bv
        let s_73_5: Bits = Bits::new(s_73_4 as u128, 1u16);
        // D s_73_6: cmp-eq s_73_3 s_73_5
        let s_73_6: bool = ((s_73_3) == (s_73_5));
        // D s_73_7: write-var gs#29382 <= s_73_6
        fn_state.gs_29382 = s_73_6;
        // N s_73_8: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #() : ()
        let s_74_0: () = ();
        // S s_74_1: call ResetSVEState(s_74_0)
        let s_74_1: () = ResetSVEState(state, tracer, s_74_0);
        // N s_74_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #16989u : u32
        let s_75_0: u32 = 16989;
        // D s_75_1: read-reg s_75_0:u8
        let s_75_1: bool = {
            let value = state.read_register::<bool>(s_75_0 as isize);
            tracer.read_register(s_75_0 as isize, value);
            value
        };
        // D s_75_2: cast zx s_75_1 -> bv
        let s_75_2: Bits = Bits::new(s_75_1 as u128, 1u16);
        // C s_75_3: const #1u : u8
        let s_75_3: bool = true;
        // C s_75_4: cast zx s_75_3 -> bv
        let s_75_4: Bits = Bits::new(s_75_3 as u128, 1u16);
        // D s_75_5: cmp-eq s_75_2 s_75_4
        let s_75_5: bool = ((s_75_2) == (s_75_4));
        // D s_75_6: write-var gs#29381 <= s_75_5
        fn_state.gs_29381 = s_75_5;
        // N s_75_7: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #() : ()
        let s_76_0: () = ();
        // S s_76_1: call HaveSME(s_76_0)
        let s_76_1: bool = HaveSME(state, tracer, s_76_0);
        // D s_76_2: write-var gs#29380 <= s_76_1
        fn_state.gs_29380 = s_76_1;
        // N s_76_3: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #() : ()
        let s_77_0: () = ();
        // S s_77_1: call AArch64_MaybeZeroRegisterUppers(s_77_0)
        let s_77_1: () = AArch64_MaybeZeroRegisterUppers(state, tracer, s_77_0);
        // N s_77_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #16983u : u32
        let s_78_0: u32 = 16983;
        // D s_78_1: read-reg s_78_0:u8
        let s_78_1: u8 = {
            let value = state.read_register::<u8>(s_78_0 as isize);
            tracer.read_register(s_78_0 as isize, value);
            value
        };
        // D s_78_2: cast zx s_78_1 -> bv
        let s_78_2: Bits = Bits::new(s_78_1 as u128, 5u16);
        // C s_78_3: const #384u : u32
        let s_78_3: u32 = 384;
        // D s_78_4: read-reg s_78_3:u8
        let s_78_4: u8 = {
            let value = state.read_register::<u8>(s_78_3 as isize);
            tracer.read_register(s_78_3 as isize, value);
            value
        };
        // D s_78_5: cast zx s_78_4 -> bv
        let s_78_5: Bits = Bits::new(s_78_4 as u128, 5u16);
        // D s_78_6: cmp-eq s_78_2 s_78_5
        let s_78_6: bool = ((s_78_2) == (s_78_5));
        // N s_78_7: branch s_78_6 b104 b79
        if s_78_6 {
            return block_104(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_79_0: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #() : ()
        let s_80_0: () = ();
        // S s_80_1: call UsingAArch32(s_80_0)
        let s_80_1: bool = UsingAArch32(state, tracer, s_80_0);
        // N s_80_2: assert s_80_1
        let s_80_2: () = assert!(s_80_1);
        // D s_80_3: read-var handle_el:u8
        let s_80_3: u8 = fn_state.handle_el;
        // D s_80_4: cast zx s_80_3 -> bv
        let s_80_4: Bits = Bits::new(s_80_3 as u128, 2u16);
        // C s_80_5: const #440u : u32
        let s_80_5: u32 = 440;
        // D s_80_6: read-reg s_80_5:u8
        let s_80_6: u8 = {
            let value = state.read_register::<u8>(s_80_5 as isize);
            tracer.read_register(s_80_5 as isize, value);
            value
        };
        // D s_80_7: cast zx s_80_6 -> bv
        let s_80_7: Bits = Bits::new(s_80_6 as u128, 2u16);
        // D s_80_8: cmp-eq s_80_4 s_80_7
        let s_80_8: bool = ((s_80_4) == (s_80_7));
        // D s_80_9: not s_80_8
        let s_80_9: bool = !s_80_8;
        // N s_80_10: branch s_80_9 b92 b81
        if s_80_9 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #376u : u32
        let s_81_0: u32 = 376;
        // D s_81_1: read-reg s_81_0:u8
        let s_81_1: u8 = {
            let value = state.read_register::<u8>(s_81_0 as isize);
            tracer.read_register(s_81_0 as isize, value);
            value
        };
        // D s_81_2: call AArch32_WriteMode(s_81_1)
        let s_81_2: () = AArch32_WriteMode(state, tracer, s_81_1);
        // C s_81_3: const #() : ()
        let s_81_3: () = ();
        // S s_81_4: call HavePANExt(s_81_3)
        let s_81_4: bool = HavePANExt(state, tracer, s_81_3);
        // N s_81_5: branch s_81_4 b91 b82
        if s_81_4 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_82(state, tracer, fn_state);
        };
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #0u : u8
        let s_82_0: bool = false;
        // D s_82_1: write-var gs#29409 <= s_82_0
        fn_state.gs_29409 = s_82_0;
        // N s_82_2: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_83_0: read-var gs#29409:u8
        let s_83_0: bool = fn_state.gs_29409;
        // N s_83_1: branch s_83_0 b90 b84
        if s_83_0 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_84(state, tracer, fn_state);
        };
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_84_0: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_85_0: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var handle_el:u8
        let s_86_0: u8 = fn_state.handle_el;
        // D s_86_1: cast zx s_86_0 -> bv
        let s_86_1: Bits = Bits::new(s_86_0 as u128, 2u16);
        // C s_86_2: const #432u : u32
        let s_86_2: u32 = 432;
        // D s_86_3: read-reg s_86_2:u8
        let s_86_3: u8 = {
            let value = state.read_register::<u8>(s_86_2 as isize);
            tracer.read_register(s_86_2 as isize, value);
            value
        };
        // D s_86_4: cast zx s_86_3 -> bv
        let s_86_4: Bits = Bits::new(s_86_3 as u128, 2u16);
        // D s_86_5: cmp-eq s_86_1 s_86_4
        let s_86_5: bool = ((s_86_1) == (s_86_4));
        // N s_86_6: branch s_86_5 b89 b87
        if s_86_5 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #32s : i64
        let s_87_0: i64 = 32;
        // C s_87_1: cast zx s_87_0 -> i
        let s_87_1: i128 = (i128::try_from(s_87_0).unwrap());
        // S s_87_2: call __UNKNOWN_bits(s_87_1)
        let s_87_2: Bits = u__UNKNOWN_bits(state, tracer, s_87_1);
        // S s_87_3: cast reint s_87_2 -> u32
        let s_87_3: u32 = (s_87_2.value() as u32);
        // S s_87_4: call LR_write(s_87_3)
        let s_87_4: () = LR_write(state, tracer, s_87_3);
        // N s_87_5: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #32s : i64
        let s_88_0: i64 = 32;
        // C s_88_1: cast zx s_88_0 -> i
        let s_88_1: i128 = (i128::try_from(s_88_0).unwrap());
        // S s_88_2: call __UNKNOWN_bits(s_88_1)
        let s_88_2: Bits = u__UNKNOWN_bits(state, tracer, s_88_1);
        // S s_88_3: cast reint s_88_2 -> u32
        let s_88_3: u32 = (s_88_2.value() as u32);
        // C s_88_4: const #32s : i
        let s_88_4: i128 = 32;
        // S s_88_5: cast zx s_88_3 -> bv
        let s_88_5: Bits = Bits::new(s_88_3 as u128, 32u16);
        // S s_88_6: call SPSR_set(s_88_4, s_88_5)
        let s_88_6: () = SPSR_set(state, tracer, s_88_4, s_88_5);
        // C s_88_7: const #() : ()
        let s_88_7: () = ();
        // S s_88_8: call SCTLR_read__1(s_88_7)
        let s_88_8: ProductType5c790c8ef59cc8b2 = SCTLR_read__1(state, tracer, s_88_7);
        // S s_88_9: call _get_SCTLRType_EE(s_88_8)
        let s_88_9: bool = u_get_SCTLRType_EE(state, tracer, s_88_8);
        // C s_88_10: const #16974u : u32
        let s_88_10: u32 = 16974;
        // N s_88_11: write-reg s_88_10 <= s_88_9
        let s_88_11: () = {
            state.write_register::<bool>(s_88_10 as isize, s_88_9);
            tracer.write_register(s_88_10 as isize, s_88_9);
        };
        // C s_88_12: const #32s : i64
        let s_88_12: i64 = 32;
        // C s_88_13: cast zx s_88_12 -> i
        let s_88_13: i128 = (i128::try_from(s_88_12).unwrap());
        // S s_88_14: call __UNKNOWN_bits(s_88_13)
        let s_88_14: Bits = u__UNKNOWN_bits(state, tracer, s_88_13);
        // S s_88_15: cast reint s_88_14 -> u32
        let s_88_15: u32 = (s_88_14.value() as u32);
        // S s_88_16: call DLR_write(s_88_15)
        let s_88_16: () = DLR_write(state, tracer, s_88_15);
        // C s_88_17: const #32s : i64
        let s_88_17: i64 = 32;
        // C s_88_18: cast zx s_88_17 -> i
        let s_88_18: i128 = (i128::try_from(s_88_17).unwrap());
        // S s_88_19: call __UNKNOWN_bits(s_88_18)
        let s_88_19: Bits = u__UNKNOWN_bits(state, tracer, s_88_18);
        // S s_88_20: cast reint s_88_19 -> u32
        let s_88_20: u32 = (s_88_19.value() as u32);
        // S s_88_21: call Mk_DSPSR_Type(s_88_20)
        let s_88_21: ProductType700c18a878c5601b = Mk_DSPSR_Type(state, tracer, s_88_20);
        // S s_88_22: call DSPSR_write(s_88_21)
        let s_88_22: () = DSPSR_write(state, tracer, s_88_21);
        // N s_88_23: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #32s : i64
        let s_89_0: i64 = 32;
        // C s_89_1: cast zx s_89_0 -> i
        let s_89_1: i128 = (i128::try_from(s_89_0).unwrap());
        // S s_89_2: call __UNKNOWN_bits(s_89_1)
        let s_89_2: Bits = u__UNKNOWN_bits(state, tracer, s_89_1);
        // S s_89_3: cast reint s_89_2 -> u32
        let s_89_3: u32 = (s_89_2.value() as u32);
        // S s_89_4: call ELR_hyp_write(s_89_3)
        let s_89_4: () = ELR_hyp_write(state, tracer, s_89_3);
        // C s_89_5: const #32s : i64
        let s_89_5: i64 = 32;
        // C s_89_6: cast zx s_89_5 -> i
        let s_89_6: i128 = (i128::try_from(s_89_5).unwrap());
        // S s_89_7: call __UNKNOWN_bits(s_89_6)
        let s_89_7: Bits = u__UNKNOWN_bits(state, tracer, s_89_6);
        // S s_89_8: cast reint s_89_7 -> u32
        let s_89_8: u32 = (s_89_7.value() as u32);
        // S s_89_9: call Mk_HSR_Type(s_89_8)
        let s_89_9: ProductType700c18a878c5601b = Mk_HSR_Type(state, tracer, s_89_8);
        // S s_89_10: call HSR_write(s_89_9)
        let s_89_10: () = HSR_write(state, tracer, s_89_9);
        // N s_89_11: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #1u : u8
        let s_90_0: bool = true;
        // C s_90_1: const #16985u : u32
        let s_90_1: u32 = 16985;
        // N s_90_2: write-reg s_90_1 <= s_90_0
        let s_90_2: () = {
            state.write_register::<bool>(s_90_1 as isize, s_90_0);
            tracer.write_register(s_90_1 as isize, s_90_0);
        };
        // N s_90_3: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #() : ()
        let s_91_0: () = ();
        // S s_91_1: call SCTLR_read__2(s_91_0)
        let s_91_1: ProductType700c18a878c5601b = SCTLR_read__2(state, tracer, s_91_0);
        // S s_91_2: call _get_SCTLR_Type_SPAN(s_91_1)
        let s_91_2: bool = u_get_SCTLR_Type_SPAN(state, tracer, s_91_1);
        // S s_91_3: cast zx s_91_2 -> bv
        let s_91_3: Bits = Bits::new(s_91_2 as u128, 1u16);
        // C s_91_4: const #0u : u8
        let s_91_4: bool = false;
        // C s_91_5: cast zx s_91_4 -> bv
        let s_91_5: Bits = Bits::new(s_91_4 as u128, 1u16);
        // S s_91_6: cmp-eq s_91_3 s_91_5
        let s_91_6: bool = ((s_91_3) == (s_91_5));
        // D s_91_7: write-var gs#29409 <= s_91_6
        fn_state.gs_29409 = s_91_6;
        // N s_91_8: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var handle_el:u8
        let s_92_0: u8 = fn_state.handle_el;
        // D s_92_1: cast zx s_92_0 -> bv
        let s_92_1: Bits = Bits::new(s_92_0 as u128, 2u16);
        // C s_92_2: const #432u : u32
        let s_92_2: u32 = 432;
        // D s_92_3: read-reg s_92_2:u8
        let s_92_3: u8 = {
            let value = state.read_register::<u8>(s_92_2 as isize);
            tracer.read_register(s_92_2 as isize, value);
            value
        };
        // D s_92_4: cast zx s_92_3 -> bv
        let s_92_4: Bits = Bits::new(s_92_3 as u128, 2u16);
        // D s_92_5: cmp-eq s_92_1 s_92_4
        let s_92_5: bool = ((s_92_1) == (s_92_4));
        // D s_92_6: not s_92_5
        let s_92_6: bool = !s_92_5;
        // N s_92_7: branch s_92_6 b94 b93
        if s_92_6 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_93(state, tracer, fn_state);
        };
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #400u : u32
        let s_93_0: u32 = 400;
        // D s_93_1: read-reg s_93_0:u8
        let s_93_1: u8 = {
            let value = state.read_register::<u8>(s_93_0 as isize);
            tracer.read_register(s_93_0 as isize, value);
            value
        };
        // D s_93_2: call AArch32_WriteMode(s_93_1)
        let s_93_2: () = AArch32_WriteMode(state, tracer, s_93_1);
        // N s_93_3: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var handle_el:u8
        let s_94_0: u8 = fn_state.handle_el;
        // D s_94_1: cast zx s_94_0 -> bv
        let s_94_1: Bits = Bits::new(s_94_0 as u128, 2u16);
        // C s_94_2: const #424u : u32
        let s_94_2: u32 = 424;
        // D s_94_3: read-reg s_94_2:u8
        let s_94_3: u8 = {
            let value = state.read_register::<u8>(s_94_2 as isize);
            tracer.read_register(s_94_2 as isize, value);
            value
        };
        // D s_94_4: cast zx s_94_3 -> bv
        let s_94_4: Bits = Bits::new(s_94_3 as u128, 2u16);
        // D s_94_5: cmp-eq s_94_1 s_94_4
        let s_94_5: bool = ((s_94_1) == (s_94_4));
        // D s_94_6: not s_94_5
        let s_94_6: bool = !s_94_5;
        // N s_94_7: branch s_94_6 b103 b95
        if s_94_6 {
            return block_103(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #384u : u32
        let s_95_0: u32 = 384;
        // D s_95_1: read-reg s_95_0:u8
        let s_95_1: u8 = {
            let value = state.read_register::<u8>(s_95_0 as isize);
            tracer.read_register(s_95_0 as isize, value);
            value
        };
        // D s_95_2: call AArch32_WriteMode(s_95_1)
        let s_95_2: () = AArch32_WriteMode(state, tracer, s_95_1);
        // C s_95_3: const #() : ()
        let s_95_3: () = ();
        // S s_95_4: call HavePANExt(s_95_3)
        let s_95_4: bool = HavePANExt(state, tracer, s_95_3);
        // N s_95_5: branch s_95_4 b98 b96
        if s_95_4 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_96(state, tracer, fn_state);
        };
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_96_0: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_97_0: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var from_secure:u8
        let s_98_0: bool = fn_state.from_secure;
        // D s_98_1: not s_98_0
        let s_98_1: bool = !s_98_0;
        // N s_98_2: branch s_98_1 b102 b99
        if s_98_1 {
            return block_102(state, tracer, fn_state);
        } else {
            return block_99(state, tracer, fn_state);
        };
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #() : ()
        let s_99_0: () = ();
        // S s_99_1: call SCTLR_read__2(s_99_0)
        let s_99_1: ProductType700c18a878c5601b = SCTLR_read__2(state, tracer, s_99_0);
        // S s_99_2: call _get_SCTLR_Type_SPAN(s_99_1)
        let s_99_2: bool = u_get_SCTLR_Type_SPAN(state, tracer, s_99_1);
        // S s_99_3: cast zx s_99_2 -> bv
        let s_99_3: Bits = Bits::new(s_99_2 as u128, 1u16);
        // C s_99_4: const #0u : u8
        let s_99_4: bool = false;
        // C s_99_5: cast zx s_99_4 -> bv
        let s_99_5: Bits = Bits::new(s_99_4 as u128, 1u16);
        // S s_99_6: cmp-eq s_99_3 s_99_5
        let s_99_6: bool = ((s_99_3) == (s_99_5));
        // N s_99_7: branch s_99_6 b101 b100
        if s_99_6 {
            return block_101(state, tracer, fn_state);
        } else {
            return block_100(state, tracer, fn_state);
        };
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_100_0: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #1u : u8
        let s_101_0: bool = true;
        // C s_101_1: const #16985u : u32
        let s_101_1: u32 = 16985;
        // N s_101_2: write-reg s_101_1 <= s_101_0
        let s_101_2: () = {
            state.write_register::<bool>(s_101_1 as isize, s_101_0);
            tracer.write_register(s_101_1 as isize, s_101_0);
        };
        // N s_101_3: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #0u : u8
        let s_102_0: bool = false;
        // C s_102_1: const #16985u : u32
        let s_102_1: u32 = 16985;
        // N s_102_2: write-reg s_102_1 <= s_102_0
        let s_102_2: () = {
            state.write_register::<bool>(s_102_1 as isize, s_102_0);
            tracer.write_register(s_102_1 as isize, s_102_0);
        };
        // N s_102_3: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_103_0: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #20920u : u32
        let s_104_0: u32 = 20920;
        // D s_104_1: read-reg s_104_0:struct
        let s_104_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_104_0 as isize);
            tracer.read_register(s_104_0 as isize, value);
            value
        };
        // C s_104_2: const #20920u : u32
        let s_104_2: u32 = 20920;
        // N s_104_3: write-reg s_104_2 <= s_104_1
        let s_104_3: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_104_2 as isize, s_104_1);
            tracer.write_register(s_104_2 as isize, s_104_1);
        };
        // N s_104_4: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_105_0: panic
        panic!("{:?}", ());
        // N s_105_1: return
        return;
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #102552u : u32
        let s_106_0: u32 = 102552;
        // D s_106_1: read-reg s_106_0:struct
        let s_106_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_106_0 as isize);
            tracer.read_register(s_106_0 as isize, value);
            value
        };
        // D s_106_2: call _get_HCR_EL2_Type_TGE(s_106_1)
        let s_106_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_106_1);
        // D s_106_3: cast zx s_106_2 -> bv
        let s_106_3: Bits = Bits::new(s_106_2 as u128, 1u16);
        // C s_106_4: const #1u : u8
        let s_106_4: bool = true;
        // C s_106_5: cast zx s_106_4 -> bv
        let s_106_5: Bits = Bits::new(s_106_4 as u128, 1u16);
        // D s_106_6: cmp-eq s_106_3 s_106_5
        let s_106_6: bool = ((s_106_3) == (s_106_5));
        // D s_106_7: write-var gs#29370 <= s_106_6
        fn_state.gs_29370 = s_106_6;
        // N s_106_8: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #16975u : u32
        let s_107_0: u32 = 16975;
        // D s_107_1: read-reg s_107_0:u8
        let s_107_1: u8 = {
            let value = state.read_register::<u8>(s_107_0 as isize);
            tracer.read_register(s_107_0 as isize, value);
            value
        };
        // D s_107_2: write-var handle_el <= s_107_1
        fn_state.handle_el = s_107_1;
        // N s_107_3: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #() : ()
        let s_108_0: () = ();
        // S s_108_1: call UsingAArch32(s_108_0)
        let s_108_1: bool = UsingAArch32(state, tracer, s_108_0);
        // S s_108_2: not s_108_1
        let s_108_2: bool = !s_108_1;
        // D s_108_3: write-var gs#29367 <= s_108_2
        fn_state.gs_29367 = s_108_2;
        // N s_108_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #1u : u8
        let s_109_0: bool = true;
        // D s_109_1: write-var gs#29368 <= s_109_0
        fn_state.gs_29368 = s_109_0;
        // N s_109_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_110_0: read-var target_el:u8
        let s_110_0: u8 = fn_state.target_el;
        // D s_110_1: cast zx s_110_0 -> bv
        let s_110_1: Bits = Bits::new(s_110_0 as u128, 2u16);
        // C s_110_2: const #432u : u32
        let s_110_2: u32 = 432;
        // D s_110_3: read-reg s_110_2:u8
        let s_110_3: u8 = {
            let value = state.read_register::<u8>(s_110_2 as isize);
            tracer.read_register(s_110_2 as isize, value);
            value
        };
        // D s_110_4: cast zx s_110_3 -> bv
        let s_110_4: Bits = Bits::new(s_110_3 as u128, 2u16);
        // D s_110_5: cmp-eq s_110_1 s_110_4
        let s_110_5: bool = ((s_110_1) == (s_110_4));
        // D s_110_6: not s_110_5
        let s_110_6: bool = !s_110_5;
        // N s_110_7: branch s_110_6 b124 b111
        if s_110_6 {
            return block_124(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_111_0: const #432u : u32
        let s_111_0: u32 = 432;
        // D s_111_1: read-reg s_111_0:u8
        let s_111_1: u8 = {
            let value = state.read_register::<u8>(s_111_0 as isize);
            tracer.read_register(s_111_0 as isize, value);
            value
        };
        // C s_111_2: const #2u : u8
        let s_111_2: u8 = 2;
        // D s_111_3: cmp-lt s_111_1 s_111_2
        let s_111_3: bool = ((s_111_1) < (s_111_2));
        // D s_111_4: not s_111_3
        let s_111_4: bool = !s_111_3;
        // N s_111_5: branch s_111_4 b123 b112
        if s_111_4 {
            return block_123(state, tracer, fn_state);
        } else {
            return block_112(state, tracer, fn_state);
        };
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_112_0: const #16975u : u32
        let s_112_0: u32 = 16975;
        // D s_112_1: read-reg s_112_0:u8
        let s_112_1: u8 = {
            let value = state.read_register::<u8>(s_112_0 as isize);
            tracer.read_register(s_112_0 as isize, value);
            value
        };
        // D s_112_2: cast zx s_112_1 -> bv
        let s_112_2: Bits = Bits::new(s_112_1 as u128, 2u16);
        // C s_112_3: const #424u : u32
        let s_112_3: u32 = 424;
        // D s_112_4: read-reg s_112_3:u8
        let s_112_4: u8 = {
            let value = state.read_register::<u8>(s_112_3 as isize);
            tracer.read_register(s_112_3 as isize, value);
            value
        };
        // D s_112_5: cast zx s_112_4 -> bv
        let s_112_5: Bits = Bits::new(s_112_4 as u128, 2u16);
        // D s_112_6: cmp-eq s_112_2 s_112_5
        let s_112_6: bool = ((s_112_2) == (s_112_5));
        // N s_112_7: branch s_112_6 b122 b113
        if s_112_6 {
            return block_122(state, tracer, fn_state);
        } else {
            return block_113(state, tracer, fn_state);
        };
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_113_0: const #0u : u8
        let s_113_0: bool = false;
        // D s_113_1: write-var gs#29372 <= s_113_0
        fn_state.gs_29372 = s_113_0;
        // N s_113_2: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_114_0: read-var gs#29372:u8
        let s_114_0: bool = fn_state.gs_29372;
        // N s_114_1: branch s_114_0 b121 b115
        if s_114_0 {
            return block_121(state, tracer, fn_state);
        } else {
            return block_115(state, tracer, fn_state);
        };
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #() : ()
        let s_115_0: () = ();
        // S s_115_1: call IsSecureEL2Enabled(s_115_0)
        let s_115_1: bool = IsSecureEL2Enabled(state, tracer, s_115_0);
        // S s_115_2: not s_115_1
        let s_115_2: bool = !s_115_1;
        // N s_115_3: branch s_115_2 b120 b116
        if s_115_2 {
            return block_120(state, tracer, fn_state);
        } else {
            return block_116(state, tracer, fn_state);
        };
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #0u : u8
        let s_116_0: bool = false;
        // D s_116_1: write-var gs#29373 <= s_116_0
        fn_state.gs_29373 = s_116_0;
        // N s_116_2: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_117_0: read-var gs#29373:u8
        let s_117_0: bool = fn_state.gs_29373;
        // N s_117_1: branch s_117_0 b119 b118
        if s_117_0 {
            return block_119(state, tracer, fn_state);
        } else {
            return block_118(state, tracer, fn_state);
        };
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #432u : u32
        let s_118_0: u32 = 432;
        // D s_118_1: read-reg s_118_0:u8
        let s_118_1: u8 = {
            let value = state.read_register::<u8>(s_118_0 as isize);
            tracer.read_register(s_118_0 as isize, value);
            value
        };
        // D s_118_2: write-var handle_el <= s_118_1
        fn_state.handle_el = s_118_1;
        // N s_118_3: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_119_0: panic
        panic!("{:?}", ());
        // N s_119_1: return
        return;
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_120_0: const #() : ()
        let s_120_0: () = ();
        // S s_120_1: call CurrentSecurityState(s_120_0)
        let s_120_1: u32 = CurrentSecurityState(state, tracer, s_120_0);
        // C s_120_2: const #3u : u32
        let s_120_2: u32 = 3;
        // S s_120_3: cmp-eq s_120_1 s_120_2
        let s_120_3: bool = ((s_120_1) == (s_120_2));
        // D s_120_4: write-var gs#29373 <= s_120_3
        fn_state.gs_29373 = s_120_3;
        // N s_120_5: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #424u : u32
        let s_121_0: u32 = 424;
        // D s_121_1: read-reg s_121_0:u8
        let s_121_1: u8 = {
            let value = state.read_register::<u8>(s_121_0 as isize);
            tracer.read_register(s_121_0 as isize, value);
            value
        };
        // D s_121_2: write-var handle_el <= s_121_1
        fn_state.handle_el = s_121_1;
        // N s_121_3: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_122_0: const #() : ()
        let s_122_0: () = ();
        // S s_122_1: call UsingAArch32(s_122_0)
        let s_122_1: bool = UsingAArch32(state, tracer, s_122_0);
        // S s_122_2: not s_122_1
        let s_122_2: bool = !s_122_1;
        // D s_122_3: write-var gs#29372 <= s_122_2
        fn_state.gs_29372 = s_122_2;
        // N s_122_4: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_123_0: panic
        panic!("{:?}", ());
        // N s_123_1: return
        return;
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_124_0: read-var target_el:u8
        let s_124_0: u8 = fn_state.target_el;
        // D s_124_1: cast zx s_124_0 -> bv
        let s_124_1: Bits = Bits::new(s_124_0 as u128, 2u16);
        // C s_124_2: const #424u : u32
        let s_124_2: u32 = 424;
        // D s_124_3: read-reg s_124_2:u8
        let s_124_3: u8 = {
            let value = state.read_register::<u8>(s_124_2 as isize);
            tracer.read_register(s_124_2 as isize, value);
            value
        };
        // D s_124_4: cast zx s_124_3 -> bv
        let s_124_4: Bits = Bits::new(s_124_3 as u128, 2u16);
        // D s_124_5: cmp-eq s_124_1 s_124_4
        let s_124_5: bool = ((s_124_1) == (s_124_4));
        // D s_124_6: not s_124_5
        let s_124_6: bool = !s_124_5;
        // N s_124_7: branch s_124_6 b131 b125
        if s_124_6 {
            return block_131(state, tracer, fn_state);
        } else {
            return block_125(state, tracer, fn_state);
        };
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_125_0: const #() : ()
        let s_125_0: () = ();
        // S s_125_1: call EDSCR_read(s_125_0)
        let s_125_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_125_0);
        // S s_125_2: call _get_EDSCR_Type_SDD(s_125_1)
        let s_125_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_125_1);
        // S s_125_3: cast zx s_125_2 -> bv
        let s_125_3: Bits = Bits::new(s_125_2 as u128, 1u16);
        // C s_125_4: const #1u : u8
        let s_125_4: bool = true;
        // C s_125_5: cast zx s_125_4 -> bv
        let s_125_5: Bits = Bits::new(s_125_4 as u128, 1u16);
        // S s_125_6: cmp-eq s_125_3 s_125_5
        let s_125_6: bool = ((s_125_3) == (s_125_5));
        // N s_125_7: branch s_125_6 b130 b126
        if s_125_6 {
            return block_130(state, tracer, fn_state);
        } else {
            return block_126(state, tracer, fn_state);
        };
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_126_0: const #424u : u32
        let s_126_0: u32 = 424;
        // D s_126_1: read-reg s_126_0:u8
        let s_126_1: u8 = {
            let value = state.read_register::<u8>(s_126_0 as isize);
            tracer.read_register(s_126_0 as isize, value);
            value
        };
        // C s_126_2: const #2u : u8
        let s_126_2: u8 = 2;
        // D s_126_3: cmp-lt s_126_1 s_126_2
        let s_126_3: bool = ((s_126_1) < (s_126_2));
        // D s_126_4: not s_126_3
        let s_126_4: bool = !s_126_3;
        // D s_126_5: write-var gs#29374 <= s_126_4
        fn_state.gs_29374 = s_126_4;
        // N s_126_6: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_127_0: read-var gs#29374:u8
        let s_127_0: bool = fn_state.gs_29374;
        // N s_127_1: branch s_127_0 b129 b128
        if s_127_0 {
            return block_129(state, tracer, fn_state);
        } else {
            return block_128(state, tracer, fn_state);
        };
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_128_0: const #424u : u32
        let s_128_0: u32 = 424;
        // D s_128_1: read-reg s_128_0:u8
        let s_128_1: u8 = {
            let value = state.read_register::<u8>(s_128_0 as isize);
            tracer.read_register(s_128_0 as isize, value);
            value
        };
        // D s_128_2: write-var handle_el <= s_128_1
        fn_state.handle_el = s_128_1;
        // N s_128_3: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_129_0: panic
        panic!("{:?}", ());
        // N s_129_1: return
        return;
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_130_0: const #1u : u8
        let s_130_0: bool = true;
        // D s_130_1: write-var gs#29374 <= s_130_0
        fn_state.gs_29374 = s_130_0;
        // N s_130_2: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_131_0: const #() : ()
        let s_131_0: () = ();
        // S s_131_1: call Unreachable(s_131_0)
        let s_131_1: () = Unreachable(state, tracer, s_131_0);
        // N s_131_2: jump b10
        return block_10(state, tracer, fn_state);
    }
}
