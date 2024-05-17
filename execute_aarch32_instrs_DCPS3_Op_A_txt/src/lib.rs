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
use Halted::*;
use Mk_DSPSR_Type::*;
use Mk_SPSR_mon_Type::*;
use u_get_SCTLR_Type_SPAN::*;
use Mk_ESR_EL3_Type::*;
use UpdateEDSCRFields::*;
use Mk_DSPSR_EL0_Type::*;
use u__UNKNOWN_bits::*;
use SCTLR_read__2::*;
use u_get_SCR_EL3_Type_NMEA::*;
use DSPSR_write::*;
use AArch32_WriteMode::*;
use u_get_EDSCR_Type_SDD::*;
use EffectiveEA::*;
use AArch64_MaybeZeroRegisterUppers::*;
use MaybeZeroSVEUppers::*;
use DLR_write::*;
use ConstrainUnpredictableBool::*;
use HaveDoubleFaultExt::*;
use Mk_SPSR_EL3_Type::*;
use u_get_SCTLR_EL3_Type_IESB::*;
use CurrentSecurityState::*;
use HavePANExt::*;
use HaveUAOExt::*;
use ELUsingAArch32::*;
use HaveIESB::*;
use SynchronizeErrors::*;
use u_get_SCTLR_Type_EE::*;
use EDSCR_read::*;
use common::*;
pub fn execute_aarch32_instrs_DCPS3_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_324016: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_324017: bool,
        gs_324025: bool,
        from_secureshadow_7910: bool,
        gs_324024: bool,
        gs_324026: bool,
        sync_errors: bool,
        gs_324016: (),
    }
    let fn_state = FunctionState {
        gs_324016,
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
        // S s_0_1: call Halted(s_0_0)
        let s_0_1: bool = Halted(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b39 b1
        if s_0_2 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call EDSCR_read(s_1_0)
        let s_1_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_1_0);
        // S s_1_2: call _get_EDSCR_Type_SDD(s_1_1)
        let s_1_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_1_1);
        // S s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 1u16);
        // C s_1_4: const #1u : u8
        let s_1_4: bool = true;
        // C s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 1u16);
        // S s_1_6: cmp-eq s_1_3 s_1_5
        let s_1_6: bool = ((s_1_3) == (s_1_5));
        // D s_1_7: write-var gs#324017 <= s_1_6
        fn_state.gs_324017 = s_1_6;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#324017:u8
        let s_2_0: bool = fn_state.gs_324017;
        // N s_2_1: branch s_2_0 b38 b3
        if s_2_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #424u : u32
        let s_3_0: u32 = 424;
        // D s_3_1: read-reg s_3_0:u8
        let s_3_1: u8 = {
            let value = state.read_register::<u8>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // D s_3_2: call ELUsingAArch32(s_3_1)
        let s_3_2: bool = ELUsingAArch32(state, tracer, s_3_1);
        // N s_3_3: branch s_3_2 b27 b4
        if s_3_2 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call AArch64_MaybeZeroRegisterUppers(s_4_0)
        let s_4_1: () = AArch64_MaybeZeroRegisterUppers(state, tracer, s_4_0);
        // C s_4_2: const #424u : u32
        let s_4_2: u32 = 424;
        // D s_4_3: read-reg s_4_2:u8
        let s_4_3: u8 = {
            let value = state.read_register::<u8>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // D s_4_4: call MaybeZeroSVEUppers(s_4_3)
        let s_4_4: () = MaybeZeroSVEUppers(state, tracer, s_4_3);
        // C s_4_5: const #0u : u8
        let s_4_5: bool = false;
        // C s_4_6: const #16999u : u32
        let s_4_6: u32 = 16999;
        // N s_4_7: write-reg s_4_6 <= s_4_5
        let s_4_7: () = {
            state.write_register::<bool>(s_4_6 as isize, s_4_5);
            tracer.write_register(s_4_6 as isize, s_4_5);
        };
        // C s_4_8: const #1u : u8
        let s_4_8: bool = true;
        // C s_4_9: const #16990u : u32
        let s_4_9: u32 = 16990;
        // N s_4_10: write-reg s_4_9 <= s_4_8
        let s_4_10: () = {
            state.write_register::<bool>(s_4_9 as isize, s_4_8);
            tracer.write_register(s_4_9 as isize, s_4_8);
        };
        // C s_4_11: const #424u : u32
        let s_4_11: u32 = 424;
        // D s_4_12: read-reg s_4_11:u8
        let s_4_12: u8 = {
            let value = state.read_register::<u8>(s_4_11 as isize);
            tracer.read_register(s_4_11 as isize, value);
            value
        };
        // C s_4_13: const #16975u : u32
        let s_4_13: u32 = 16975;
        // N s_4_14: write-reg s_4_13 <= s_4_12
        let s_4_14: () = {
            state.write_register::<u8>(s_4_13 as isize, s_4_12);
            tracer.write_register(s_4_13 as isize, s_4_12);
        };
        // C s_4_15: const #() : ()
        let s_4_15: () = ();
        // S s_4_16: call HaveUAOExt(s_4_15)
        let s_4_16: bool = HaveUAOExt(state, tracer, s_4_15);
        // N s_4_17: branch s_4_16 b26 b5
        if s_4_16 {
            return block_26(state, tracer, fn_state);
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
        // C s_6_0: const #64s : i64
        let s_6_0: i64 = 64;
        // C s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // S s_6_2: call __UNKNOWN_bits(s_6_1)
        let s_6_2: Bits = u__UNKNOWN_bits(state, tracer, s_6_1);
        // S s_6_3: cast reint s_6_2 -> u64
        let s_6_3: u64 = (s_6_2.value() as u64);
        // C s_6_4: const #20128u : u32
        let s_6_4: u32 = 20128;
        // N s_6_5: write-reg s_6_4 <= s_6_3
        let s_6_5: () = {
            state.write_register::<u64>(s_6_4 as isize, s_6_3);
            tracer.write_register(s_6_4 as isize, s_6_3);
        };
        // C s_6_6: const #64s : i64
        let s_6_6: i64 = 64;
        // C s_6_7: cast zx s_6_6 -> i
        let s_6_7: i128 = (i128::try_from(s_6_6).unwrap());
        // S s_6_8: call __UNKNOWN_bits(s_6_7)
        let s_6_8: Bits = u__UNKNOWN_bits(state, tracer, s_6_7);
        // S s_6_9: cast reint s_6_8 -> u64
        let s_6_9: u64 = (s_6_8.value() as u64);
        // S s_6_10: call Mk_ESR_EL3_Type(s_6_9)
        let s_6_10: ProductType5c790c8ef59cc8b2 = Mk_ESR_EL3_Type(state, tracer, s_6_9);
        // C s_6_11: const #11992u : u32
        let s_6_11: u32 = 11992;
        // N s_6_12: write-reg s_6_11 <= s_6_10
        let s_6_12: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_6_11 as isize, s_6_10);
            tracer.write_register(s_6_11 as isize, s_6_10);
        };
        // C s_6_13: const #64s : i64
        let s_6_13: i64 = 64;
        // C s_6_14: cast zx s_6_13 -> i
        let s_6_14: i128 = (i128::try_from(s_6_13).unwrap());
        // S s_6_15: call __UNKNOWN_bits(s_6_14)
        let s_6_15: Bits = u__UNKNOWN_bits(state, tracer, s_6_14);
        // S s_6_16: cast reint s_6_15 -> u64
        let s_6_16: u64 = (s_6_15.value() as u64);
        // S s_6_17: call Mk_SPSR_EL3_Type(s_6_16)
        let s_6_17: ProductType5c790c8ef59cc8b2 = Mk_SPSR_EL3_Type(
            state,
            tracer,
            s_6_16,
        );
        // C s_6_18: const #20304u : u32
        let s_6_18: u32 = 20304;
        // N s_6_19: write-reg s_6_18 <= s_6_17
        let s_6_19: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_6_18 as isize, s_6_17);
            tracer.write_register(s_6_18 as isize, s_6_17);
        };
        // C s_6_20: const #64s : i64
        let s_6_20: i64 = 64;
        // C s_6_21: cast zx s_6_20 -> i
        let s_6_21: i128 = (i128::try_from(s_6_20).unwrap());
        // S s_6_22: call __UNKNOWN_bits(s_6_21)
        let s_6_22: Bits = u__UNKNOWN_bits(state, tracer, s_6_21);
        // S s_6_23: cast reint s_6_22 -> u64
        let s_6_23: u64 = (s_6_22.value() as u64);
        // C s_6_24: const #13360u : u32
        let s_6_24: u32 = 13360;
        // N s_6_25: write-reg s_6_24 <= s_6_23
        let s_6_25: () = {
            state.write_register::<u64>(s_6_24 as isize, s_6_23);
            tracer.write_register(s_6_24 as isize, s_6_23);
        };
        // C s_6_26: const #64s : i64
        let s_6_26: i64 = 64;
        // C s_6_27: cast zx s_6_26 -> i
        let s_6_27: i128 = (i128::try_from(s_6_26).unwrap());
        // S s_6_28: call __UNKNOWN_bits(s_6_27)
        let s_6_28: Bits = u__UNKNOWN_bits(state, tracer, s_6_27);
        // S s_6_29: cast reint s_6_28 -> u64
        let s_6_29: u64 = (s_6_28.value() as u64);
        // S s_6_30: call Mk_DSPSR_EL0_Type(s_6_29)
        let s_6_30: ProductType5c790c8ef59cc8b2 = Mk_DSPSR_EL0_Type(
            state,
            tracer,
            s_6_29,
        );
        // C s_6_31: const #102584u : u32
        let s_6_31: u32 = 102584;
        // N s_6_32: write-reg s_6_31 <= s_6_30
        let s_6_32: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_6_31 as isize, s_6_30);
            tracer.write_register(s_6_31 as isize, s_6_30);
        };
        // C s_6_33: const #() : ()
        let s_6_33: () = ();
        // S s_6_34: call HaveIESB(s_6_33)
        let s_6_34: bool = HaveIESB(state, tracer, s_6_33);
        // N s_6_35: branch s_6_34 b25 b7
        if s_6_34 {
            return block_25(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#324024 <= s_7_0
        fn_state.gs_324024 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#324024:u8
        let s_8_0: bool = fn_state.gs_324024;
        // D s_8_1: write-var sync_errors <= s_8_0
        fn_state.sync_errors = s_8_0;
        // C s_8_2: const #() : ()
        let s_8_2: () = ();
        // S s_8_3: call HaveDoubleFaultExt(s_8_2)
        let s_8_3: bool = HaveDoubleFaultExt(state, tracer, s_8_2);
        // N s_8_4: branch s_8_3 b24 b9
        if s_8_3 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#324025 <= s_9_0
        fn_state.gs_324025 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#324025:u8
        let s_10_0: bool = fn_state.gs_324025;
        // N s_10_1: branch s_10_0 b23 b11
        if s_10_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#324026 <= s_11_0
        fn_state.gs_324026 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#324026:u8
        let s_12_0: bool = fn_state.gs_324026;
        // N s_12_1: branch s_12_0 b22 b13
        if s_12_0 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #59u : u32
        let s_14_0: u32 = 59;
        // S s_14_1: call ConstrainUnpredictableBool(s_14_0)
        let s_14_1: bool = ConstrainUnpredictableBool(state, tracer, s_14_0);
        // S s_14_2: not s_14_1
        let s_14_2: bool = !s_14_1;
        // N s_14_3: branch s_14_2 b21 b15
        if s_14_2 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var sync_errors:u8
        let s_16_0: bool = fn_state.sync_errors;
        // N s_16_1: branch s_16_0 b20 b17
        if s_16_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call UpdateEDSCRFields(s_19_0)
        let s_19_1: () = UpdateEDSCRFields(state, tracer, s_19_0);
        // N s_19_2: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call SynchronizeErrors(s_20_0)
        let s_20_1: () = SynchronizeErrors(state, tracer, s_20_0);
        // N s_20_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var sync_errors <= s_21_0
        fn_state.sync_errors = s_21_0;
        // N s_21_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var sync_errors <= s_22_0
        fn_state.sync_errors = s_22_0;
        // N s_22_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #90704u : u32
        let s_23_0: u32 = 90704;
        // D s_23_1: read-reg s_23_0:struct
        let s_23_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // D s_23_2: call _get_SCR_EL3_Type_NMEA(s_23_1)
        let s_23_2: bool = u_get_SCR_EL3_Type_NMEA(state, tracer, s_23_1);
        // D s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 1u16);
        // C s_23_4: const #1u : u8
        let s_23_4: bool = true;
        // C s_23_5: cast zx s_23_4 -> bv
        let s_23_5: Bits = Bits::new(s_23_4 as u128, 1u16);
        // D s_23_6: cmp-eq s_23_3 s_23_5
        let s_23_6: bool = ((s_23_3) == (s_23_5));
        // D s_23_7: write-var gs#324026 <= s_23_6
        fn_state.gs_324026 = s_23_6;
        // N s_23_8: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call EffectiveEA(s_24_0)
        let s_24_1: bool = EffectiveEA(state, tracer, s_24_0);
        // S s_24_2: cast zx s_24_1 -> bv
        let s_24_2: Bits = Bits::new(s_24_1 as u128, 1u16);
        // C s_24_3: const #1u : u8
        let s_24_3: bool = true;
        // C s_24_4: cast zx s_24_3 -> bv
        let s_24_4: Bits = Bits::new(s_24_3 as u128, 1u16);
        // S s_24_5: cmp-eq s_24_2 s_24_4
        let s_24_5: bool = ((s_24_2) == (s_24_4));
        // D s_24_6: write-var gs#324025 <= s_24_5
        fn_state.gs_324025 = s_24_5;
        // N s_24_7: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #17072u : u32
        let s_25_0: u32 = 17072;
        // D s_25_1: read-reg s_25_0:struct
        let s_25_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_25_0 as isize);
            tracer.read_register(s_25_0 as isize, value);
            value
        };
        // D s_25_2: call _get_SCTLR_EL3_Type_IESB(s_25_1)
        let s_25_2: bool = u_get_SCTLR_EL3_Type_IESB(state, tracer, s_25_1);
        // D s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // C s_25_4: const #1u : u8
        let s_25_4: bool = true;
        // C s_25_5: cast zx s_25_4 -> bv
        let s_25_5: Bits = Bits::new(s_25_4 as u128, 1u16);
        // D s_25_6: cmp-eq s_25_3 s_25_5
        let s_25_6: bool = ((s_25_3) == (s_25_5));
        // D s_25_7: write-var gs#324024 <= s_25_6
        fn_state.gs_324024 = s_25_6;
        // N s_25_8: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // C s_26_1: const #16995u : u32
        let s_26_1: u32 = 16995;
        // N s_26_2: write-reg s_26_1 <= s_26_0
        let s_26_2: () = {
            state.write_register::<bool>(s_26_1 as isize, s_26_0);
            tracer.write_register(s_26_1 as isize, s_26_0);
        };
        // N s_26_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #() : ()
        let s_27_0: () = ();
        // S s_27_1: call CurrentSecurityState(s_27_0)
        let s_27_1: u32 = CurrentSecurityState(state, tracer, s_27_0);
        // C s_27_2: const #3u : u32
        let s_27_2: u32 = 3;
        // S s_27_3: cmp-eq s_27_1 s_27_2
        let s_27_3: bool = ((s_27_1) == (s_27_2));
        // D s_27_4: write-var from_secureshadow#7910 <= s_27_3
        fn_state.from_secureshadow_7910 = s_27_3;
        // C s_27_5: const #16983u : u32
        let s_27_5: u32 = 16983;
        // D s_27_6: read-reg s_27_5:u8
        let s_27_6: u8 = {
            let value = state.read_register::<u8>(s_27_5 as isize);
            tracer.read_register(s_27_5 as isize, value);
            value
        };
        // D s_27_7: cast zx s_27_6 -> bv
        let s_27_7: Bits = Bits::new(s_27_6 as u128, 5u16);
        // C s_27_8: const #384u : u32
        let s_27_8: u32 = 384;
        // D s_27_9: read-reg s_27_8:u8
        let s_27_9: u8 = {
            let value = state.read_register::<u8>(s_27_8 as isize);
            tracer.read_register(s_27_8 as isize, value);
            value
        };
        // D s_27_10: cast zx s_27_9 -> bv
        let s_27_10: Bits = Bits::new(s_27_9 as u128, 5u16);
        // D s_27_11: cmp-eq s_27_7 s_27_10
        let s_27_11: bool = ((s_27_7) == (s_27_10));
        // N s_27_12: branch s_27_11 b37 b28
        if s_27_11 {
            return block_37(state, tracer, fn_state);
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
        // C s_29_0: const #384u : u32
        let s_29_0: u32 = 384;
        // D s_29_1: read-reg s_29_0:u8
        let s_29_1: u8 = {
            let value = state.read_register::<u8>(s_29_0 as isize);
            tracer.read_register(s_29_0 as isize, value);
            value
        };
        // D s_29_2: call AArch32_WriteMode(s_29_1)
        let s_29_2: () = AArch32_WriteMode(state, tracer, s_29_1);
        // C s_29_3: const #() : ()
        let s_29_3: () = ();
        // S s_29_4: call HavePANExt(s_29_3)
        let s_29_4: bool = HavePANExt(state, tracer, s_29_3);
        // N s_29_5: branch s_29_4 b32 b30
        if s_29_4 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_30_0: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #() : ()
        let s_31_0: () = ();
        // S s_31_1: call SCTLR_read__2(s_31_0)
        let s_31_1: ProductType700c18a878c5601b = SCTLR_read__2(state, tracer, s_31_0);
        // S s_31_2: call _get_SCTLR_Type_EE(s_31_1)
        let s_31_2: bool = u_get_SCTLR_Type_EE(state, tracer, s_31_1);
        // C s_31_3: const #16974u : u32
        let s_31_3: u32 = 16974;
        // N s_31_4: write-reg s_31_3 <= s_31_2
        let s_31_4: () = {
            state.write_register::<bool>(s_31_3 as isize, s_31_2);
            tracer.write_register(s_31_3 as isize, s_31_2);
        };
        // C s_31_5: const #32s : i64
        let s_31_5: i64 = 32;
        // C s_31_6: cast zx s_31_5 -> i
        let s_31_6: i128 = (i128::try_from(s_31_5).unwrap());
        // S s_31_7: call __UNKNOWN_bits(s_31_6)
        let s_31_7: Bits = u__UNKNOWN_bits(state, tracer, s_31_6);
        // S s_31_8: cast reint s_31_7 -> u32
        let s_31_8: u32 = (s_31_7.value() as u32);
        // C s_31_9: const #11696u : u32
        let s_31_9: u32 = 11696;
        // N s_31_10: write-reg s_31_9 <= s_31_8
        let s_31_10: () = {
            state.write_register::<u32>(s_31_9 as isize, s_31_8);
            tracer.write_register(s_31_9 as isize, s_31_8);
        };
        // C s_31_11: const #32s : i64
        let s_31_11: i64 = 32;
        // C s_31_12: cast zx s_31_11 -> i
        let s_31_12: i128 = (i128::try_from(s_31_11).unwrap());
        // S s_31_13: call __UNKNOWN_bits(s_31_12)
        let s_31_13: Bits = u__UNKNOWN_bits(state, tracer, s_31_12);
        // S s_31_14: cast reint s_31_13 -> u32
        let s_31_14: u32 = (s_31_13.value() as u32);
        // S s_31_15: call Mk_SPSR_mon_Type(s_31_14)
        let s_31_15: ProductType700c18a878c5601b = Mk_SPSR_mon_Type(
            state,
            tracer,
            s_31_14,
        );
        // C s_31_16: const #21136u : u32
        let s_31_16: u32 = 21136;
        // N s_31_17: write-reg s_31_16 <= s_31_15
        let s_31_17: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_31_16 as isize, s_31_15);
            tracer.write_register(s_31_16 as isize, s_31_15);
        };
        // C s_31_18: const #32s : i64
        let s_31_18: i64 = 32;
        // C s_31_19: cast zx s_31_18 -> i
        let s_31_19: i128 = (i128::try_from(s_31_18).unwrap());
        // S s_31_20: call __UNKNOWN_bits(s_31_19)
        let s_31_20: Bits = u__UNKNOWN_bits(state, tracer, s_31_19);
        // S s_31_21: cast reint s_31_20 -> u32
        let s_31_21: u32 = (s_31_20.value() as u32);
        // S s_31_22: call DLR_write(s_31_21)
        let s_31_22: () = DLR_write(state, tracer, s_31_21);
        // C s_31_23: const #32s : i64
        let s_31_23: i64 = 32;
        // C s_31_24: cast zx s_31_23 -> i
        let s_31_24: i128 = (i128::try_from(s_31_23).unwrap());
        // S s_31_25: call __UNKNOWN_bits(s_31_24)
        let s_31_25: Bits = u__UNKNOWN_bits(state, tracer, s_31_24);
        // S s_31_26: cast reint s_31_25 -> u32
        let s_31_26: u32 = (s_31_25.value() as u32);
        // S s_31_27: call Mk_DSPSR_Type(s_31_26)
        let s_31_27: ProductType700c18a878c5601b = Mk_DSPSR_Type(state, tracer, s_31_26);
        // S s_31_28: call DSPSR_write(s_31_27)
        let s_31_28: () = DSPSR_write(state, tracer, s_31_27);
        // N s_31_29: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var from_secureshadow#7910:u8
        let s_32_0: bool = fn_state.from_secureshadow_7910;
        // D s_32_1: not s_32_0
        let s_32_1: bool = !s_32_0;
        // N s_32_2: branch s_32_1 b36 b33
        if s_32_1 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #() : ()
        let s_33_0: () = ();
        // S s_33_1: call SCTLR_read__2(s_33_0)
        let s_33_1: ProductType700c18a878c5601b = SCTLR_read__2(state, tracer, s_33_0);
        // S s_33_2: call _get_SCTLR_Type_SPAN(s_33_1)
        let s_33_2: bool = u_get_SCTLR_Type_SPAN(state, tracer, s_33_1);
        // S s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 1u16);
        // C s_33_4: const #0u : u8
        let s_33_4: bool = false;
        // C s_33_5: cast zx s_33_4 -> bv
        let s_33_5: Bits = Bits::new(s_33_4 as u128, 1u16);
        // S s_33_6: cmp-eq s_33_3 s_33_5
        let s_33_6: bool = ((s_33_3) == (s_33_5));
        // N s_33_7: branch s_33_6 b35 b34
        if s_33_6 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_34_0: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #1u : u8
        let s_35_0: bool = true;
        // C s_35_1: const #16985u : u32
        let s_35_1: u32 = 16985;
        // N s_35_2: write-reg s_35_1 <= s_35_0
        let s_35_2: () = {
            state.write_register::<bool>(s_35_1 as isize, s_35_0);
            tracer.write_register(s_35_1 as isize, s_35_0);
        };
        // N s_35_3: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #0u : u8
        let s_36_0: bool = false;
        // C s_36_1: const #16985u : u32
        let s_36_1: u32 = 16985;
        // N s_36_2: write-reg s_36_1 <= s_36_0
        let s_36_2: () = {
            state.write_register::<bool>(s_36_1 as isize, s_36_0);
            tracer.write_register(s_36_1 as isize, s_36_0);
        };
        // N s_36_3: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #20920u : u32
        let s_37_0: u32 = 20920;
        // D s_37_1: read-reg s_37_0:struct
        let s_37_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_37_0 as isize);
            tracer.read_register(s_37_0 as isize, value);
            value
        };
        // C s_37_2: const #20920u : u32
        let s_37_2: u32 = 20920;
        // N s_37_3: write-reg s_37_2 <= s_37_1
        let s_37_3: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_37_2 as isize, s_37_1);
            tracer.write_register(s_37_2 as isize, s_37_1);
        };
        // N s_37_4: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_38_0: panic
        panic!("{:?}", ());
        // N s_38_1: return
        return;
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #1u : u8
        let s_39_0: bool = true;
        // D s_39_1: write-var gs#324017 <= s_39_0
        fn_state.gs_324017 = s_39_0;
        // N s_39_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
