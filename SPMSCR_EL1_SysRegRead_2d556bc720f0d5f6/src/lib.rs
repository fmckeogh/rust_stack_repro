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
use SPMSCR_EL1_read::*;
use Halted::*;
use IsFeatureImplemented::*;
use u_get_MDCR_EL2_Type_EnSPM::*;
use AArch64_SystemAccessTrap::*;
use u_get_SPMSELR_EL0_Type_SYSPMUSEL::*;
use u__IMPDEF_boolean::*;
use u__get_selected_SPMACCESSR_EL2_field::*;
use X_set::*;
use u_get_HDFGRTR2_EL2_Type_nSPMSCR_EL1::*;
use u_get_SCR_EL3_Type_FGTEn2::*;
use u_get_EDSCR_Type_SDD::*;
use IsCurrentSecurityState::*;
use EL2Enabled::*;
use u_get_MDCR_EL3_Type_EnPM2::*;
use u__get_selected_SPMACCESSR_EL3_field::*;
use EDSCR_read::*;
use common::*;
pub fn SPMSCR_EL1_SysRegRead_2d556bc720f0d5f6<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    op0: u8,
    op1: u8,
    CRn: u8,
    op2: u8,
    CRm: u8,
    t: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_73996: bool,
        gs_73993: bool,
        u__MDCR_EL3_EnPM2: bool,
        gs_74003: bool,
        gs_73999: bool,
        gs_74016: bool,
        gs_73982: bool,
        gs_73991: bool,
        gs_74009: bool,
        u__HDFGRTR2_EL2_nSPMSCR_EL1: bool,
        gs_73997: bool,
        u__SCR_EL3_FGTEn2: bool,
        gs_73998: bool,
        gs_74006: bool,
        gs_73990: bool,
        gs_74001: bool,
        gs_73994: bool,
        gs_74012: bool,
        u__PSTATE_EL: u8,
        gs_73985: bool,
        gs_73992: bool,
        gs_73983: bool,
        gs_74002: bool,
        gs_74014: bool,
        gs_74013: bool,
        gs_74008: bool,
        gs_73989: bool,
        gs_74005: bool,
        gs_74007: bool,
        u__MDCR_EL2_EnSPM: bool,
        gs_73987: bool,
        gs_74017: bool,
        gs_73988: bool,
        gs_74004: bool,
        gs_74010: bool,
        gs_73986: bool,
        gs_74011: bool,
        gs_74000: bool,
        el: u8,
        op0: u8,
        op1: u8,
        CRn: u8,
        op2: u8,
        CRm: u8,
        t: i128,
    }
    let fn_state = FunctionState {
        el,
        op0,
        op1,
        CRn,
        op2,
        CRm,
        t,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: write-var __PSTATE_EL <= s_0_1
        fn_state.u__PSTATE_EL = s_0_1;
        // C s_0_3: const #22712u : u32
        let s_0_3: u32 = 22712;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_MDCR_EL3_Type_EnPM2(s_0_4)
        let s_0_5: bool = u_get_MDCR_EL3_Type_EnPM2(state, tracer, s_0_4);
        // D s_0_6: write-var __MDCR_EL3_EnPM2 <= s_0_5
        fn_state.u__MDCR_EL3_EnPM2 = s_0_5;
        // C s_0_7: const #90704u : u32
        let s_0_7: u32 = 90704;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_SCR_EL3_Type_FGTEn2(s_0_8)
        let s_0_9: bool = u_get_SCR_EL3_Type_FGTEn2(state, tracer, s_0_8);
        // D s_0_10: write-var __SCR_EL3_FGTEn2 <= s_0_9
        fn_state.u__SCR_EL3_FGTEn2 = s_0_9;
        // C s_0_11: const #101224u : u32
        let s_0_11: u32 = 101224;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_HDFGRTR2_EL2_Type_nSPMSCR_EL1(s_0_12)
        let s_0_13: bool = u_get_HDFGRTR2_EL2_Type_nSPMSCR_EL1(state, tracer, s_0_12);
        // D s_0_14: write-var __HDFGRTR2_EL2_nSPMSCR_EL1 <= s_0_13
        fn_state.u__HDFGRTR2_EL2_nSPMSCR_EL1 = s_0_13;
        // C s_0_15: const #104880u : u32
        let s_0_15: u32 = 104880;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_MDCR_EL2_Type_EnSPM(s_0_16)
        let s_0_17: bool = u_get_MDCR_EL2_Type_EnSPM(state, tracer, s_0_16);
        // D s_0_18: write-var __MDCR_EL2_EnSPM <= s_0_17
        fn_state.u__MDCR_EL2_EnSPM = s_0_17;
        // C s_0_19: const #0u : u32
        let s_0_19: u32 = 0;
        // S s_0_20: call IsCurrentSecurityState(s_0_19)
        let s_0_20: bool = IsCurrentSecurityState(state, tracer, s_0_19);
        // N s_0_21: branch s_0_20 b141 b1
        if s_0_20 {
            return block_141(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #232u : u32
        let s_1_0: u32 = 232;
        // S s_1_1: call IsFeatureImplemented(s_1_0)
        let s_1_1: bool = IsFeatureImplemented(state, tracer, s_1_0);
        // N s_1_2: branch s_1_1 b140 b2
        if s_1_1 {
            return block_140(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#73982 <= s_2_0
        fn_state.gs_73982 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#73982:u8
        let s_3_0: bool = fn_state.gs_73982;
        // D s_3_1: write-var gs#73983 <= s_3_0
        fn_state.gs_73983 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#73983:u8
        let s_4_0: bool = fn_state.gs_73983;
        // N s_4_1: branch s_4_0 b139 b5
        if s_4_0 {
            return block_139(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var __PSTATE_EL:u8
        let s_5_0: u8 = fn_state.u__PSTATE_EL;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // C s_5_2: const #448u : u32
        let s_5_2: u32 = 448;
        // D s_5_3: read-reg s_5_2:u8
        let s_5_3: u8 = {
            let value = state.read_register::<u8>(s_5_2 as isize);
            tracer.read_register(s_5_2 as isize, value);
            value
        };
        // D s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 2u16);
        // D s_5_5: cmp-eq s_5_1 s_5_4
        let s_5_5: bool = ((s_5_1) == (s_5_4));
        // N s_5_6: branch s_5_5 b138 b6
        if s_5_5 {
            return block_138(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var __PSTATE_EL:u8
        let s_6_0: u8 = fn_state.u__PSTATE_EL;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 2u16);
        // C s_6_2: const #440u : u32
        let s_6_2: u32 = 440;
        // D s_6_3: read-reg s_6_2:u8
        let s_6_3: u8 = {
            let value = state.read_register::<u8>(s_6_2 as isize);
            tracer.read_register(s_6_2 as isize, value);
            value
        };
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 2u16);
        // D s_6_5: cmp-eq s_6_1 s_6_4
        let s_6_5: bool = ((s_6_1) == (s_6_4));
        // N s_6_6: branch s_6_5 b60 b7
        if s_6_5 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var __PSTATE_EL:u8
        let s_7_0: u8 = fn_state.u__PSTATE_EL;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 2u16);
        // C s_7_2: const #432u : u32
        let s_7_2: u32 = 432;
        // D s_7_3: read-reg s_7_2:u8
        let s_7_3: u8 = {
            let value = state.read_register::<u8>(s_7_2 as isize);
            tracer.read_register(s_7_2 as isize, value);
            value
        };
        // D s_7_4: cast zx s_7_3 -> bv
        let s_7_4: Bits = Bits::new(s_7_3 as u128, 2u16);
        // D s_7_5: cmp-eq s_7_1 s_7_4
        let s_7_5: bool = ((s_7_1) == (s_7_4));
        // N s_7_6: branch s_7_5 b11 b8
        if s_7_5 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var __PSTATE_EL:u8
        let s_8_0: u8 = fn_state.u__PSTATE_EL;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 2u16);
        // C s_8_2: const #424u : u32
        let s_8_2: u32 = 424;
        // D s_8_3: read-reg s_8_2:u8
        let s_8_3: u8 = {
            let value = state.read_register::<u8>(s_8_2 as isize);
            tracer.read_register(s_8_2 as isize, value);
            value
        };
        // D s_8_4: cast zx s_8_3 -> bv
        let s_8_4: Bits = Bits::new(s_8_3 as u128, 2u16);
        // D s_8_5: cmp-eq s_8_1 s_8_4
        let s_8_5: bool = ((s_8_1) == (s_8_4));
        // N s_8_6: branch s_8_5 b10 b9
        if s_8_5 {
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
        // N s_9_0: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #64s : i64
        let s_10_0: i64 = 64;
        // C s_10_1: const #16552u : u32
        let s_10_1: u32 = 16552;
        // D s_10_2: read-reg s_10_1:struct
        let s_10_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_10_1 as isize);
            tracer.read_register(s_10_1 as isize, value);
            value
        };
        // D s_10_3: call _get_SPMSELR_EL0_Type_SYSPMUSEL(s_10_2)
        let s_10_3: u8 = u_get_SPMSELR_EL0_Type_SYSPMUSEL(state, tracer, s_10_2);
        // D s_10_4: cast zx s_10_3 -> bv
        let s_10_4: Bits = Bits::new(s_10_3 as u128, 6u16);
        // D s_10_5: cast zx s_10_4 -> i
        let s_10_5: i128 = (s_10_4.value() as i128);
        // D s_10_6: cast reint s_10_5 -> i64
        let s_10_6: i64 = (s_10_5 as i64);
        // D s_10_7: cast zx s_10_6 -> i
        let s_10_7: i128 = (i128::try_from(s_10_6).unwrap());
        // D s_10_8: call SPMSCR_EL1_read(s_10_7)
        let s_10_8: u64 = SPMSCR_EL1_read(state, tracer, s_10_7);
        // D s_10_9: cast zx s_10_8 -> bv
        let s_10_9: Bits = Bits::new(s_10_8 as u128, 64u16);
        // D s_10_10: read-var t:i
        let s_10_10: i128 = fn_state.t;
        // D s_10_11: call X_set(s_10_10, s_10_0, s_10_9)
        let s_10_11: () = X_set(state, tracer, s_10_10, s_10_0, s_10_9);
        // N s_10_12: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call Halted(s_11_0)
        let s_11_1: bool = Halted(state, tracer, s_11_0);
        // N s_11_2: branch s_11_1 b59 b12
        if s_11_1 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#73985 <= s_12_0
        fn_state.gs_73985 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#73985:u8
        let s_13_0: bool = fn_state.gs_73985;
        // N s_13_1: branch s_13_0 b58 b14
        if s_13_0 {
            return block_58(state, tracer, fn_state);
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
        // D s_14_1: write-var gs#73986 <= s_14_0
        fn_state.gs_73986 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#73986:u8
        let s_15_0: bool = fn_state.gs_73986;
        // N s_15_1: branch s_15_0 b57 b16
        if s_15_0 {
            return block_57(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#73987 <= s_16_0
        fn_state.gs_73987 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#73987:u8
        let s_17_0: bool = fn_state.gs_73987;
        // N s_17_1: branch s_17_0 b56 b18
        if s_17_0 {
            return block_56(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#73988 <= s_18_0
        fn_state.gs_73988 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#73988:u8
        let s_19_0: bool = fn_state.gs_73988;
        // N s_19_1: branch s_19_0 b55 b20
        if s_19_0 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call Halted(s_20_0)
        let s_20_1: bool = Halted(state, tracer, s_20_0);
        // N s_20_2: branch s_20_1 b54 b21
        if s_20_1 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#73989 <= s_21_0
        fn_state.gs_73989 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#73989:u8
        let s_22_0: bool = fn_state.gs_73989;
        // N s_22_1: branch s_22_0 b53 b23
        if s_22_0 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#73990 <= s_23_0
        fn_state.gs_73990 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#73990:u8
        let s_24_0: bool = fn_state.gs_73990;
        // N s_24_1: branch s_24_0 b52 b25
        if s_24_0 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#73991 <= s_25_0
        fn_state.gs_73991 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#73991:u8
        let s_26_0: bool = fn_state.gs_73991;
        // N s_26_1: branch s_26_0 b51 b27
        if s_26_0 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // D s_27_1: write-var gs#73992 <= s_27_0
        fn_state.gs_73992 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#73992:u8
        let s_28_0: bool = fn_state.gs_73992;
        // N s_28_1: branch s_28_0 b50 b29
        if s_28_0 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #424u : u32
        let s_29_0: u32 = 424;
        // D s_29_1: read-reg s_29_0:u8
        let s_29_1: u8 = {
            let value = state.read_register::<u8>(s_29_0 as isize);
            tracer.read_register(s_29_0 as isize, value);
            value
        };
        // C s_29_2: const #2u : u8
        let s_29_2: u8 = 2;
        // D s_29_3: cmp-lt s_29_1 s_29_2
        let s_29_3: bool = ((s_29_1) < (s_29_2));
        // N s_29_4: branch s_29_3 b49 b30
        if s_29_3 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #0u : u8
        let s_30_0: bool = false;
        // D s_30_1: write-var gs#73993 <= s_30_0
        fn_state.gs_73993 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#73993:u8
        let s_31_0: bool = fn_state.gs_73993;
        // N s_31_1: branch s_31_0 b43 b32
        if s_31_0 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #424u : u32
        let s_32_0: u32 = 424;
        // D s_32_1: read-reg s_32_0:u8
        let s_32_1: u8 = {
            let value = state.read_register::<u8>(s_32_0 as isize);
            tracer.read_register(s_32_0 as isize, value);
            value
        };
        // C s_32_2: const #2u : u8
        let s_32_2: u8 = 2;
        // D s_32_3: cmp-lt s_32_1 s_32_2
        let s_32_3: bool = ((s_32_1) < (s_32_2));
        // N s_32_4: branch s_32_3 b42 b33
        if s_32_3 {
            return block_42(state, tracer, fn_state);
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
        // D s_33_1: write-var gs#73994 <= s_33_0
        fn_state.gs_73994 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#73994:u8
        let s_34_0: bool = fn_state.gs_73994;
        // N s_34_1: branch s_34_0 b36 b35
        if s_34_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #64s : i64
        let s_35_0: i64 = 64;
        // C s_35_1: const #16552u : u32
        let s_35_1: u32 = 16552;
        // D s_35_2: read-reg s_35_1:struct
        let s_35_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_35_1 as isize);
            tracer.read_register(s_35_1 as isize, value);
            value
        };
        // D s_35_3: call _get_SPMSELR_EL0_Type_SYSPMUSEL(s_35_2)
        let s_35_3: u8 = u_get_SPMSELR_EL0_Type_SYSPMUSEL(state, tracer, s_35_2);
        // D s_35_4: cast zx s_35_3 -> bv
        let s_35_4: Bits = Bits::new(s_35_3 as u128, 6u16);
        // D s_35_5: cast zx s_35_4 -> i
        let s_35_5: i128 = (s_35_4.value() as i128);
        // D s_35_6: cast reint s_35_5 -> i64
        let s_35_6: i64 = (s_35_5 as i64);
        // D s_35_7: cast zx s_35_6 -> i
        let s_35_7: i128 = (i128::try_from(s_35_6).unwrap());
        // D s_35_8: call SPMSCR_EL1_read(s_35_7)
        let s_35_8: u64 = SPMSCR_EL1_read(state, tracer, s_35_7);
        // D s_35_9: cast zx s_35_8 -> bv
        let s_35_9: Bits = Bits::new(s_35_8 as u128, 64u16);
        // D s_35_10: read-var t:i
        let s_35_10: i128 = fn_state.t;
        // D s_35_11: call X_set(s_35_10, s_35_0, s_35_9)
        let s_35_11: () = X_set(state, tracer, s_35_10, s_35_0, s_35_9);
        // N s_35_12: return
        return;
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #() : ()
        let s_36_0: () = ();
        // S s_36_1: call Halted(s_36_0)
        let s_36_1: bool = Halted(state, tracer, s_36_0);
        // N s_36_2: branch s_36_1 b41 b37
        if s_36_1 {
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
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var gs#73996 <= s_37_0
        fn_state.gs_73996 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#73996:u8
        let s_38_0: bool = fn_state.gs_73996;
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
        // C s_39_0: const #24u : u8
        let s_39_0: u8 = 24;
        // C s_39_1: cast zx s_39_0 -> bv
        let s_39_1: Bits = Bits::new(s_39_0 as u128, 8u16);
        // C s_39_2: cast zx s_39_1 -> i
        let s_39_2: i128 = (s_39_1.value() as i128);
        // C s_39_3: cast reint s_39_2 -> i64
        let s_39_3: i64 = (s_39_2 as i64);
        // C s_39_4: cast zx s_39_3 -> i
        let s_39_4: i128 = (i128::try_from(s_39_3).unwrap());
        // C s_39_5: const #424u : u32
        let s_39_5: u32 = 424;
        // D s_39_6: read-reg s_39_5:u8
        let s_39_6: u8 = {
            let value = state.read_register::<u8>(s_39_5 as isize);
            tracer.read_register(s_39_5 as isize, value);
            value
        };
        // D s_39_7: call AArch64_SystemAccessTrap(s_39_6, s_39_4)
        let s_39_7: () = AArch64_SystemAccessTrap(state, tracer, s_39_6, s_39_4);
        // N s_39_8: return
        return;
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_40_0: panic
        panic!("{:?}", ());
        // N s_40_1: return
        return;
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #() : ()
        let s_41_0: () = ();
        // S s_41_1: call EDSCR_read(s_41_0)
        let s_41_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_41_0);
        // S s_41_2: call _get_EDSCR_Type_SDD(s_41_1)
        let s_41_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_41_1);
        // S s_41_3: cast zx s_41_2 -> bv
        let s_41_3: Bits = Bits::new(s_41_2 as u128, 1u16);
        // C s_41_4: const #1u : u8
        let s_41_4: bool = true;
        // C s_41_5: cast zx s_41_4 -> bv
        let s_41_5: Bits = Bits::new(s_41_4 as u128, 1u16);
        // S s_41_6: cmp-eq s_41_3 s_41_5
        let s_41_6: bool = ((s_41_3) == (s_41_5));
        // D s_41_7: write-var gs#73996 <= s_41_6
        fn_state.gs_73996 = s_41_6;
        // N s_41_8: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #() : ()
        let s_42_0: () = ();
        // S s_42_1: call __get_selected_SPMACCESSR_EL3_field(s_42_0)
        let s_42_1: u8 = u__get_selected_SPMACCESSR_EL3_field(state, tracer, s_42_0);
        // S s_42_2: cast zx s_42_1 -> bv
        let s_42_2: Bits = Bits::new(s_42_1 as u128, 2u16);
        // C s_42_3: const #0u : u8
        let s_42_3: u8 = 0;
        // C s_42_4: cast zx s_42_3 -> bv
        let s_42_4: Bits = Bits::new(s_42_3 as u128, 2u16);
        // S s_42_5: cmp-eq s_42_2 s_42_4
        let s_42_5: bool = ((s_42_2) == (s_42_4));
        // D s_42_6: write-var gs#73994 <= s_42_5
        fn_state.gs_73994 = s_42_5;
        // N s_42_7: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #() : ()
        let s_43_0: () = ();
        // S s_43_1: call Halted(s_43_0)
        let s_43_1: bool = Halted(state, tracer, s_43_0);
        // N s_43_2: branch s_43_1 b48 b44
        if s_43_1 {
            return block_48(state, tracer, fn_state);
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
        // D s_44_1: write-var gs#73997 <= s_44_0
        fn_state.gs_73997 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#73997:u8
        let s_45_0: bool = fn_state.gs_73997;
        // N s_45_1: branch s_45_0 b47 b46
        if s_45_0 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #24u : u8
        let s_46_0: u8 = 24;
        // C s_46_1: cast zx s_46_0 -> bv
        let s_46_1: Bits = Bits::new(s_46_0 as u128, 8u16);
        // C s_46_2: cast zx s_46_1 -> i
        let s_46_2: i128 = (s_46_1.value() as i128);
        // C s_46_3: cast reint s_46_2 -> i64
        let s_46_3: i64 = (s_46_2 as i64);
        // C s_46_4: cast zx s_46_3 -> i
        let s_46_4: i128 = (i128::try_from(s_46_3).unwrap());
        // C s_46_5: const #424u : u32
        let s_46_5: u32 = 424;
        // D s_46_6: read-reg s_46_5:u8
        let s_46_6: u8 = {
            let value = state.read_register::<u8>(s_46_5 as isize);
            tracer.read_register(s_46_5 as isize, value);
            value
        };
        // D s_46_7: call AArch64_SystemAccessTrap(s_46_6, s_46_4)
        let s_46_7: () = AArch64_SystemAccessTrap(state, tracer, s_46_6, s_46_4);
        // N s_46_8: return
        return;
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_47_0: panic
        panic!("{:?}", ());
        // N s_47_1: return
        return;
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #() : ()
        let s_48_0: () = ();
        // S s_48_1: call EDSCR_read(s_48_0)
        let s_48_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_48_0);
        // S s_48_2: call _get_EDSCR_Type_SDD(s_48_1)
        let s_48_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_48_1);
        // S s_48_3: cast zx s_48_2 -> bv
        let s_48_3: Bits = Bits::new(s_48_2 as u128, 1u16);
        // C s_48_4: const #1u : u8
        let s_48_4: bool = true;
        // C s_48_5: cast zx s_48_4 -> bv
        let s_48_5: Bits = Bits::new(s_48_4 as u128, 1u16);
        // S s_48_6: cmp-eq s_48_3 s_48_5
        let s_48_6: bool = ((s_48_3) == (s_48_5));
        // D s_48_7: write-var gs#73997 <= s_48_6
        fn_state.gs_73997 = s_48_6;
        // N s_48_8: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var __MDCR_EL3_EnPM2:u8
        let s_49_0: bool = fn_state.u__MDCR_EL3_EnPM2;
        // D s_49_1: cast zx s_49_0 -> bv
        let s_49_1: Bits = Bits::new(s_49_0 as u128, 1u16);
        // C s_49_2: const #0u : u8
        let s_49_2: bool = false;
        // C s_49_3: cast zx s_49_2 -> bv
        let s_49_3: Bits = Bits::new(s_49_2 as u128, 1u16);
        // D s_49_4: cmp-eq s_49_1 s_49_3
        let s_49_4: bool = ((s_49_1) == (s_49_3));
        // D s_49_5: write-var gs#73993 <= s_49_4
        fn_state.gs_73993 = s_49_4;
        // N s_49_6: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_50_0: panic
        panic!("{:?}", ());
        // N s_50_1: return
        return;
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #() : ()
        let s_51_0: () = ();
        // S s_51_1: call __get_selected_SPMACCESSR_EL3_field(s_51_0)
        let s_51_1: u8 = u__get_selected_SPMACCESSR_EL3_field(state, tracer, s_51_0);
        // S s_51_2: cast zx s_51_1 -> bv
        let s_51_2: Bits = Bits::new(s_51_1 as u128, 2u16);
        // C s_51_3: const #0u : u8
        let s_51_3: u8 = 0;
        // C s_51_4: cast zx s_51_3 -> bv
        let s_51_4: Bits = Bits::new(s_51_3 as u128, 2u16);
        // S s_51_5: cmp-eq s_51_2 s_51_4
        let s_51_5: bool = ((s_51_2) == (s_51_4));
        // D s_51_6: write-var gs#73992 <= s_51_5
        fn_state.gs_73992 = s_51_5;
        // N s_51_7: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_52_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_52_1: call __IMPDEF_boolean(s_52_0)
        let s_52_1: bool = u__IMPDEF_boolean(state, tracer, s_52_0);
        // D s_52_2: write-var gs#73991 <= s_52_1
        fn_state.gs_73991 = s_52_1;
        // N s_52_3: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #() : ()
        let s_53_0: () = ();
        // S s_53_1: call EDSCR_read(s_53_0)
        let s_53_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_53_0);
        // S s_53_2: call _get_EDSCR_Type_SDD(s_53_1)
        let s_53_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_53_1);
        // S s_53_3: cast zx s_53_2 -> bv
        let s_53_3: Bits = Bits::new(s_53_2 as u128, 1u16);
        // C s_53_4: const #1u : u8
        let s_53_4: bool = true;
        // C s_53_5: cast zx s_53_4 -> bv
        let s_53_5: Bits = Bits::new(s_53_4 as u128, 1u16);
        // S s_53_6: cmp-eq s_53_3 s_53_5
        let s_53_6: bool = ((s_53_3) == (s_53_5));
        // D s_53_7: write-var gs#73990 <= s_53_6
        fn_state.gs_73990 = s_53_6;
        // N s_53_8: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #424u : u32
        let s_54_0: u32 = 424;
        // D s_54_1: read-reg s_54_0:u8
        let s_54_1: u8 = {
            let value = state.read_register::<u8>(s_54_0 as isize);
            tracer.read_register(s_54_0 as isize, value);
            value
        };
        // C s_54_2: const #2u : u8
        let s_54_2: u8 = 2;
        // D s_54_3: cmp-lt s_54_1 s_54_2
        let s_54_3: bool = ((s_54_1) < (s_54_2));
        // D s_54_4: write-var gs#73989 <= s_54_3
        fn_state.gs_73989 = s_54_3;
        // N s_54_5: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_55_0: panic
        panic!("{:?}", ());
        // N s_55_1: return
        return;
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var __MDCR_EL3_EnPM2:u8
        let s_56_0: bool = fn_state.u__MDCR_EL3_EnPM2;
        // D s_56_1: cast zx s_56_0 -> bv
        let s_56_1: Bits = Bits::new(s_56_0 as u128, 1u16);
        // C s_56_2: const #0u : u8
        let s_56_2: bool = false;
        // C s_56_3: cast zx s_56_2 -> bv
        let s_56_3: Bits = Bits::new(s_56_2 as u128, 1u16);
        // D s_56_4: cmp-eq s_56_1 s_56_3
        let s_56_4: bool = ((s_56_1) == (s_56_3));
        // D s_56_5: write-var gs#73988 <= s_56_4
        fn_state.gs_73988 = s_56_4;
        // N s_56_6: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_57_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_57_1: call __IMPDEF_boolean(s_57_0)
        let s_57_1: bool = u__IMPDEF_boolean(state, tracer, s_57_0);
        // D s_57_2: write-var gs#73987 <= s_57_1
        fn_state.gs_73987 = s_57_1;
        // N s_57_3: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #() : ()
        let s_58_0: () = ();
        // S s_58_1: call EDSCR_read(s_58_0)
        let s_58_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_58_0);
        // S s_58_2: call _get_EDSCR_Type_SDD(s_58_1)
        let s_58_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_58_1);
        // S s_58_3: cast zx s_58_2 -> bv
        let s_58_3: Bits = Bits::new(s_58_2 as u128, 1u16);
        // C s_58_4: const #1u : u8
        let s_58_4: bool = true;
        // C s_58_5: cast zx s_58_4 -> bv
        let s_58_5: Bits = Bits::new(s_58_4 as u128, 1u16);
        // S s_58_6: cmp-eq s_58_3 s_58_5
        let s_58_6: bool = ((s_58_3) == (s_58_5));
        // D s_58_7: write-var gs#73986 <= s_58_6
        fn_state.gs_73986 = s_58_6;
        // N s_58_8: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #424u : u32
        let s_59_0: u32 = 424;
        // D s_59_1: read-reg s_59_0:u8
        let s_59_1: u8 = {
            let value = state.read_register::<u8>(s_59_0 as isize);
            tracer.read_register(s_59_0 as isize, value);
            value
        };
        // C s_59_2: const #2u : u8
        let s_59_2: u8 = 2;
        // D s_59_3: cmp-lt s_59_1 s_59_2
        let s_59_3: bool = ((s_59_1) < (s_59_2));
        // D s_59_4: write-var gs#73985 <= s_59_3
        fn_state.gs_73985 = s_59_3;
        // N s_59_5: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #() : ()
        let s_60_0: () = ();
        // S s_60_1: call Halted(s_60_0)
        let s_60_1: bool = Halted(state, tracer, s_60_0);
        // N s_60_2: branch s_60_1 b137 b61
        if s_60_1 {
            return block_137(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #0u : u8
        let s_61_0: bool = false;
        // D s_61_1: write-var gs#73998 <= s_61_0
        fn_state.gs_73998 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#73998:u8
        let s_62_0: bool = fn_state.gs_73998;
        // N s_62_1: branch s_62_0 b136 b63
        if s_62_0 {
            return block_136(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #0u : u8
        let s_63_0: bool = false;
        // D s_63_1: write-var gs#73999 <= s_63_0
        fn_state.gs_73999 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#73999:u8
        let s_64_0: bool = fn_state.gs_73999;
        // N s_64_1: branch s_64_0 b135 b65
        if s_64_0 {
            return block_135(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #0u : u8
        let s_65_0: bool = false;
        // D s_65_1: write-var gs#74000 <= s_65_0
        fn_state.gs_74000 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#74000:u8
        let s_66_0: bool = fn_state.gs_74000;
        // N s_66_1: branch s_66_0 b134 b67
        if s_66_0 {
            return block_134(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #0u : u8
        let s_67_0: bool = false;
        // D s_67_1: write-var gs#74001 <= s_67_0
        fn_state.gs_74001 = s_67_0;
        // N s_67_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var gs#74001:u8
        let s_68_0: bool = fn_state.gs_74001;
        // N s_68_1: branch s_68_0 b133 b69
        if s_68_0 {
            return block_133(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #() : ()
        let s_69_0: () = ();
        // S s_69_1: call Halted(s_69_0)
        let s_69_1: bool = Halted(state, tracer, s_69_0);
        // N s_69_2: branch s_69_1 b132 b70
        if s_69_1 {
            return block_132(state, tracer, fn_state);
        } else {
            return block_70(state, tracer, fn_state);
        };
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #0u : u8
        let s_70_0: bool = false;
        // D s_70_1: write-var gs#74002 <= s_70_0
        fn_state.gs_74002 = s_70_0;
        // N s_70_2: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var gs#74002:u8
        let s_71_0: bool = fn_state.gs_74002;
        // N s_71_1: branch s_71_0 b131 b72
        if s_71_0 {
            return block_131(state, tracer, fn_state);
        } else {
            return block_72(state, tracer, fn_state);
        };
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #0u : u8
        let s_72_0: bool = false;
        // D s_72_1: write-var gs#74003 <= s_72_0
        fn_state.gs_74003 = s_72_0;
        // N s_72_2: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var gs#74003:u8
        let s_73_0: bool = fn_state.gs_74003;
        // N s_73_1: branch s_73_0 b130 b74
        if s_73_0 {
            return block_130(state, tracer, fn_state);
        } else {
            return block_74(state, tracer, fn_state);
        };
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #0u : u8
        let s_74_0: bool = false;
        // D s_74_1: write-var gs#74004 <= s_74_0
        fn_state.gs_74004 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var gs#74004:u8
        let s_75_0: bool = fn_state.gs_74004;
        // N s_75_1: branch s_75_0 b129 b76
        if s_75_0 {
            return block_129(state, tracer, fn_state);
        } else {
            return block_76(state, tracer, fn_state);
        };
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #0u : u8
        let s_76_0: bool = false;
        // D s_76_1: write-var gs#74005 <= s_76_0
        fn_state.gs_74005 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var gs#74005:u8
        let s_77_0: bool = fn_state.gs_74005;
        // N s_77_1: branch s_77_0 b128 b78
        if s_77_0 {
            return block_128(state, tracer, fn_state);
        } else {
            return block_78(state, tracer, fn_state);
        };
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #() : ()
        let s_78_0: () = ();
        // S s_78_1: call EL2Enabled(s_78_0)
        let s_78_1: bool = EL2Enabled(state, tracer, s_78_0);
        // N s_78_2: branch s_78_1 b127 b79
        if s_78_1 {
            return block_127(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #0u : u8
        let s_79_0: bool = false;
        // D s_79_1: write-var gs#74006 <= s_79_0
        fn_state.gs_74006 = s_79_0;
        // N s_79_2: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var gs#74006:u8
        let s_80_0: bool = fn_state.gs_74006;
        // N s_80_1: branch s_80_0 b126 b81
        if s_80_0 {
            return block_126(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #0u : u8
        let s_81_0: bool = false;
        // D s_81_1: write-var gs#74007 <= s_81_0
        fn_state.gs_74007 = s_81_0;
        // N s_81_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var gs#74007:u8
        let s_82_0: bool = fn_state.gs_74007;
        // N s_82_1: branch s_82_0 b125 b83
        if s_82_0 {
            return block_125(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #0u : u8
        let s_83_0: bool = false;
        // D s_83_1: write-var gs#74008 <= s_83_0
        fn_state.gs_74008 = s_83_0;
        // N s_83_2: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var gs#74008:u8
        let s_84_0: bool = fn_state.gs_74008;
        // N s_84_1: branch s_84_0 b124 b85
        if s_84_0 {
            return block_124(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #() : ()
        let s_85_0: () = ();
        // S s_85_1: call EL2Enabled(s_85_0)
        let s_85_1: bool = EL2Enabled(state, tracer, s_85_0);
        // N s_85_2: branch s_85_1 b123 b86
        if s_85_1 {
            return block_123(state, tracer, fn_state);
        } else {
            return block_86(state, tracer, fn_state);
        };
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #0u : u8
        let s_86_0: bool = false;
        // D s_86_1: write-var gs#74009 <= s_86_0
        fn_state.gs_74009 = s_86_0;
        // N s_86_2: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var gs#74009:u8
        let s_87_0: bool = fn_state.gs_74009;
        // N s_87_1: branch s_87_0 b122 b88
        if s_87_0 {
            return block_122(state, tracer, fn_state);
        } else {
            return block_88(state, tracer, fn_state);
        };
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #0u : u8
        let s_88_0: bool = false;
        // D s_88_1: write-var gs#74010 <= s_88_0
        fn_state.gs_74010 = s_88_0;
        // N s_88_2: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_89_0: read-var gs#74010:u8
        let s_89_0: bool = fn_state.gs_74010;
        // N s_89_1: branch s_89_0 b121 b90
        if s_89_0 {
            return block_121(state, tracer, fn_state);
        } else {
            return block_90(state, tracer, fn_state);
        };
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #() : ()
        let s_90_0: () = ();
        // S s_90_1: call EL2Enabled(s_90_0)
        let s_90_1: bool = EL2Enabled(state, tracer, s_90_0);
        // N s_90_2: branch s_90_1 b120 b91
        if s_90_1 {
            return block_120(state, tracer, fn_state);
        } else {
            return block_91(state, tracer, fn_state);
        };
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #0u : u8
        let s_91_0: bool = false;
        // D s_91_1: write-var gs#74011 <= s_91_0
        fn_state.gs_74011 = s_91_0;
        // N s_91_2: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var gs#74011:u8
        let s_92_0: bool = fn_state.gs_74011;
        // N s_92_1: branch s_92_0 b119 b93
        if s_92_0 {
            return block_119(state, tracer, fn_state);
        } else {
            return block_93(state, tracer, fn_state);
        };
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #() : ()
        let s_93_0: () = ();
        // S s_93_1: call EL2Enabled(s_93_0)
        let s_93_1: bool = EL2Enabled(state, tracer, s_93_0);
        // N s_93_2: branch s_93_1 b118 b94
        if s_93_1 {
            return block_118(state, tracer, fn_state);
        } else {
            return block_94(state, tracer, fn_state);
        };
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #0u : u8
        let s_94_0: bool = false;
        // D s_94_1: write-var gs#74012 <= s_94_0
        fn_state.gs_74012 = s_94_0;
        // N s_94_2: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_95_0: read-var gs#74012:u8
        let s_95_0: bool = fn_state.gs_74012;
        // N s_95_1: branch s_95_0 b117 b96
        if s_95_0 {
            return block_117(state, tracer, fn_state);
        } else {
            return block_96(state, tracer, fn_state);
        };
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #424u : u32
        let s_96_0: u32 = 424;
        // D s_96_1: read-reg s_96_0:u8
        let s_96_1: u8 = {
            let value = state.read_register::<u8>(s_96_0 as isize);
            tracer.read_register(s_96_0 as isize, value);
            value
        };
        // C s_96_2: const #2u : u8
        let s_96_2: u8 = 2;
        // D s_96_3: cmp-lt s_96_1 s_96_2
        let s_96_3: bool = ((s_96_1) < (s_96_2));
        // N s_96_4: branch s_96_3 b116 b97
        if s_96_3 {
            return block_116(state, tracer, fn_state);
        } else {
            return block_97(state, tracer, fn_state);
        };
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #0u : u8
        let s_97_0: bool = false;
        // D s_97_1: write-var gs#74013 <= s_97_0
        fn_state.gs_74013 = s_97_0;
        // N s_97_2: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var gs#74013:u8
        let s_98_0: bool = fn_state.gs_74013;
        // N s_98_1: branch s_98_0 b110 b99
        if s_98_0 {
            return block_110(state, tracer, fn_state);
        } else {
            return block_99(state, tracer, fn_state);
        };
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #424u : u32
        let s_99_0: u32 = 424;
        // D s_99_1: read-reg s_99_0:u8
        let s_99_1: u8 = {
            let value = state.read_register::<u8>(s_99_0 as isize);
            tracer.read_register(s_99_0 as isize, value);
            value
        };
        // C s_99_2: const #2u : u8
        let s_99_2: u8 = 2;
        // D s_99_3: cmp-lt s_99_1 s_99_2
        let s_99_3: bool = ((s_99_1) < (s_99_2));
        // N s_99_4: branch s_99_3 b109 b100
        if s_99_3 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_100(state, tracer, fn_state);
        };
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #0u : u8
        let s_100_0: bool = false;
        // D s_100_1: write-var gs#74014 <= s_100_0
        fn_state.gs_74014 = s_100_0;
        // N s_100_2: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_101_0: read-var gs#74014:u8
        let s_101_0: bool = fn_state.gs_74014;
        // N s_101_1: branch s_101_0 b103 b102
        if s_101_0 {
            return block_103(state, tracer, fn_state);
        } else {
            return block_102(state, tracer, fn_state);
        };
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #64s : i64
        let s_102_0: i64 = 64;
        // C s_102_1: const #16552u : u32
        let s_102_1: u32 = 16552;
        // D s_102_2: read-reg s_102_1:struct
        let s_102_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_102_1 as isize);
            tracer.read_register(s_102_1 as isize, value);
            value
        };
        // D s_102_3: call _get_SPMSELR_EL0_Type_SYSPMUSEL(s_102_2)
        let s_102_3: u8 = u_get_SPMSELR_EL0_Type_SYSPMUSEL(state, tracer, s_102_2);
        // D s_102_4: cast zx s_102_3 -> bv
        let s_102_4: Bits = Bits::new(s_102_3 as u128, 6u16);
        // D s_102_5: cast zx s_102_4 -> i
        let s_102_5: i128 = (s_102_4.value() as i128);
        // D s_102_6: cast reint s_102_5 -> i64
        let s_102_6: i64 = (s_102_5 as i64);
        // D s_102_7: cast zx s_102_6 -> i
        let s_102_7: i128 = (i128::try_from(s_102_6).unwrap());
        // D s_102_8: call SPMSCR_EL1_read(s_102_7)
        let s_102_8: u64 = SPMSCR_EL1_read(state, tracer, s_102_7);
        // D s_102_9: cast zx s_102_8 -> bv
        let s_102_9: Bits = Bits::new(s_102_8 as u128, 64u16);
        // D s_102_10: read-var t:i
        let s_102_10: i128 = fn_state.t;
        // D s_102_11: call X_set(s_102_10, s_102_0, s_102_9)
        let s_102_11: () = X_set(state, tracer, s_102_10, s_102_0, s_102_9);
        // N s_102_12: return
        return;
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #() : ()
        let s_103_0: () = ();
        // S s_103_1: call Halted(s_103_0)
        let s_103_1: bool = Halted(state, tracer, s_103_0);
        // N s_103_2: branch s_103_1 b108 b104
        if s_103_1 {
            return block_108(state, tracer, fn_state);
        } else {
            return block_104(state, tracer, fn_state);
        };
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #0u : u8
        let s_104_0: bool = false;
        // D s_104_1: write-var gs#74016 <= s_104_0
        fn_state.gs_74016 = s_104_0;
        // N s_104_2: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_105_0: read-var gs#74016:u8
        let s_105_0: bool = fn_state.gs_74016;
        // N s_105_1: branch s_105_0 b107 b106
        if s_105_0 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_106(state, tracer, fn_state);
        };
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #24u : u8
        let s_106_0: u8 = 24;
        // C s_106_1: cast zx s_106_0 -> bv
        let s_106_1: Bits = Bits::new(s_106_0 as u128, 8u16);
        // C s_106_2: cast zx s_106_1 -> i
        let s_106_2: i128 = (s_106_1.value() as i128);
        // C s_106_3: cast reint s_106_2 -> i64
        let s_106_3: i64 = (s_106_2 as i64);
        // C s_106_4: cast zx s_106_3 -> i
        let s_106_4: i128 = (i128::try_from(s_106_3).unwrap());
        // C s_106_5: const #424u : u32
        let s_106_5: u32 = 424;
        // D s_106_6: read-reg s_106_5:u8
        let s_106_6: u8 = {
            let value = state.read_register::<u8>(s_106_5 as isize);
            tracer.read_register(s_106_5 as isize, value);
            value
        };
        // D s_106_7: call AArch64_SystemAccessTrap(s_106_6, s_106_4)
        let s_106_7: () = AArch64_SystemAccessTrap(state, tracer, s_106_6, s_106_4);
        // N s_106_8: return
        return;
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_107_0: panic
        panic!("{:?}", ());
        // N s_107_1: return
        return;
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #() : ()
        let s_108_0: () = ();
        // S s_108_1: call EDSCR_read(s_108_0)
        let s_108_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_108_0);
        // S s_108_2: call _get_EDSCR_Type_SDD(s_108_1)
        let s_108_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_108_1);
        // S s_108_3: cast zx s_108_2 -> bv
        let s_108_3: Bits = Bits::new(s_108_2 as u128, 1u16);
        // C s_108_4: const #1u : u8
        let s_108_4: bool = true;
        // C s_108_5: cast zx s_108_4 -> bv
        let s_108_5: Bits = Bits::new(s_108_4 as u128, 1u16);
        // S s_108_6: cmp-eq s_108_3 s_108_5
        let s_108_6: bool = ((s_108_3) == (s_108_5));
        // D s_108_7: write-var gs#74016 <= s_108_6
        fn_state.gs_74016 = s_108_6;
        // N s_108_8: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #() : ()
        let s_109_0: () = ();
        // S s_109_1: call __get_selected_SPMACCESSR_EL3_field(s_109_0)
        let s_109_1: u8 = u__get_selected_SPMACCESSR_EL3_field(state, tracer, s_109_0);
        // S s_109_2: cast zx s_109_1 -> bv
        let s_109_2: Bits = Bits::new(s_109_1 as u128, 2u16);
        // C s_109_3: const #0u : u8
        let s_109_3: u8 = 0;
        // C s_109_4: cast zx s_109_3 -> bv
        let s_109_4: Bits = Bits::new(s_109_3 as u128, 2u16);
        // S s_109_5: cmp-eq s_109_2 s_109_4
        let s_109_5: bool = ((s_109_2) == (s_109_4));
        // D s_109_6: write-var gs#74014 <= s_109_5
        fn_state.gs_74014 = s_109_5;
        // N s_109_7: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_110_0: const #() : ()
        let s_110_0: () = ();
        // S s_110_1: call Halted(s_110_0)
        let s_110_1: bool = Halted(state, tracer, s_110_0);
        // N s_110_2: branch s_110_1 b115 b111
        if s_110_1 {
            return block_115(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_111_0: const #0u : u8
        let s_111_0: bool = false;
        // D s_111_1: write-var gs#74017 <= s_111_0
        fn_state.gs_74017 = s_111_0;
        // N s_111_2: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_112_0: read-var gs#74017:u8
        let s_112_0: bool = fn_state.gs_74017;
        // N s_112_1: branch s_112_0 b114 b113
        if s_112_0 {
            return block_114(state, tracer, fn_state);
        } else {
            return block_113(state, tracer, fn_state);
        };
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_113_0: const #24u : u8
        let s_113_0: u8 = 24;
        // C s_113_1: cast zx s_113_0 -> bv
        let s_113_1: Bits = Bits::new(s_113_0 as u128, 8u16);
        // C s_113_2: cast zx s_113_1 -> i
        let s_113_2: i128 = (s_113_1.value() as i128);
        // C s_113_3: cast reint s_113_2 -> i64
        let s_113_3: i64 = (s_113_2 as i64);
        // C s_113_4: cast zx s_113_3 -> i
        let s_113_4: i128 = (i128::try_from(s_113_3).unwrap());
        // C s_113_5: const #424u : u32
        let s_113_5: u32 = 424;
        // D s_113_6: read-reg s_113_5:u8
        let s_113_6: u8 = {
            let value = state.read_register::<u8>(s_113_5 as isize);
            tracer.read_register(s_113_5 as isize, value);
            value
        };
        // D s_113_7: call AArch64_SystemAccessTrap(s_113_6, s_113_4)
        let s_113_7: () = AArch64_SystemAccessTrap(state, tracer, s_113_6, s_113_4);
        // N s_113_8: return
        return;
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_114_0: panic
        panic!("{:?}", ());
        // N s_114_1: return
        return;
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #() : ()
        let s_115_0: () = ();
        // S s_115_1: call EDSCR_read(s_115_0)
        let s_115_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_115_0);
        // S s_115_2: call _get_EDSCR_Type_SDD(s_115_1)
        let s_115_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_115_1);
        // S s_115_3: cast zx s_115_2 -> bv
        let s_115_3: Bits = Bits::new(s_115_2 as u128, 1u16);
        // C s_115_4: const #1u : u8
        let s_115_4: bool = true;
        // C s_115_5: cast zx s_115_4 -> bv
        let s_115_5: Bits = Bits::new(s_115_4 as u128, 1u16);
        // S s_115_6: cmp-eq s_115_3 s_115_5
        let s_115_6: bool = ((s_115_3) == (s_115_5));
        // D s_115_7: write-var gs#74017 <= s_115_6
        fn_state.gs_74017 = s_115_6;
        // N s_115_8: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_116_0: read-var __MDCR_EL3_EnPM2:u8
        let s_116_0: bool = fn_state.u__MDCR_EL3_EnPM2;
        // D s_116_1: cast zx s_116_0 -> bv
        let s_116_1: Bits = Bits::new(s_116_0 as u128, 1u16);
        // C s_116_2: const #0u : u8
        let s_116_2: bool = false;
        // C s_116_3: cast zx s_116_2 -> bv
        let s_116_3: Bits = Bits::new(s_116_2 as u128, 1u16);
        // D s_116_4: cmp-eq s_116_1 s_116_3
        let s_116_4: bool = ((s_116_1) == (s_116_3));
        // D s_116_5: write-var gs#74013 <= s_116_4
        fn_state.gs_74013 = s_116_4;
        // N s_116_6: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #24u : u8
        let s_117_0: u8 = 24;
        // C s_117_1: cast zx s_117_0 -> bv
        let s_117_1: Bits = Bits::new(s_117_0 as u128, 8u16);
        // C s_117_2: cast zx s_117_1 -> i
        let s_117_2: i128 = (s_117_1.value() as i128);
        // C s_117_3: cast reint s_117_2 -> i64
        let s_117_3: i64 = (s_117_2 as i64);
        // C s_117_4: cast zx s_117_3 -> i
        let s_117_4: i128 = (i128::try_from(s_117_3).unwrap());
        // C s_117_5: const #432u : u32
        let s_117_5: u32 = 432;
        // D s_117_6: read-reg s_117_5:u8
        let s_117_6: u8 = {
            let value = state.read_register::<u8>(s_117_5 as isize);
            tracer.read_register(s_117_5 as isize, value);
            value
        };
        // D s_117_7: call AArch64_SystemAccessTrap(s_117_6, s_117_4)
        let s_117_7: () = AArch64_SystemAccessTrap(state, tracer, s_117_6, s_117_4);
        // N s_117_8: return
        return;
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #() : ()
        let s_118_0: () = ();
        // S s_118_1: call __get_selected_SPMACCESSR_EL2_field(s_118_0)
        let s_118_1: u8 = u__get_selected_SPMACCESSR_EL2_field(state, tracer, s_118_0);
        // S s_118_2: cast zx s_118_1 -> bv
        let s_118_2: Bits = Bits::new(s_118_1 as u128, 2u16);
        // C s_118_3: const #0u : u8
        let s_118_3: u8 = 0;
        // C s_118_4: cast zx s_118_3 -> bv
        let s_118_4: Bits = Bits::new(s_118_3 as u128, 2u16);
        // S s_118_5: cmp-eq s_118_2 s_118_4
        let s_118_5: bool = ((s_118_2) == (s_118_4));
        // D s_118_6: write-var gs#74012 <= s_118_5
        fn_state.gs_74012 = s_118_5;
        // N s_118_7: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #24u : u8
        let s_119_0: u8 = 24;
        // C s_119_1: cast zx s_119_0 -> bv
        let s_119_1: Bits = Bits::new(s_119_0 as u128, 8u16);
        // C s_119_2: cast zx s_119_1 -> i
        let s_119_2: i128 = (s_119_1.value() as i128);
        // C s_119_3: cast reint s_119_2 -> i64
        let s_119_3: i64 = (s_119_2 as i64);
        // C s_119_4: cast zx s_119_3 -> i
        let s_119_4: i128 = (i128::try_from(s_119_3).unwrap());
        // C s_119_5: const #432u : u32
        let s_119_5: u32 = 432;
        // D s_119_6: read-reg s_119_5:u8
        let s_119_6: u8 = {
            let value = state.read_register::<u8>(s_119_5 as isize);
            tracer.read_register(s_119_5 as isize, value);
            value
        };
        // D s_119_7: call AArch64_SystemAccessTrap(s_119_6, s_119_4)
        let s_119_7: () = AArch64_SystemAccessTrap(state, tracer, s_119_6, s_119_4);
        // N s_119_8: return
        return;
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_120_0: read-var __MDCR_EL2_EnSPM:u8
        let s_120_0: bool = fn_state.u__MDCR_EL2_EnSPM;
        // D s_120_1: cast zx s_120_0 -> bv
        let s_120_1: Bits = Bits::new(s_120_0 as u128, 1u16);
        // C s_120_2: const #0u : u8
        let s_120_2: bool = false;
        // C s_120_3: cast zx s_120_2 -> bv
        let s_120_3: Bits = Bits::new(s_120_2 as u128, 1u16);
        // D s_120_4: cmp-eq s_120_1 s_120_3
        let s_120_4: bool = ((s_120_1) == (s_120_3));
        // D s_120_5: write-var gs#74011 <= s_120_4
        fn_state.gs_74011 = s_120_4;
        // N s_120_6: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #24u : u8
        let s_121_0: u8 = 24;
        // C s_121_1: cast zx s_121_0 -> bv
        let s_121_1: Bits = Bits::new(s_121_0 as u128, 8u16);
        // C s_121_2: cast zx s_121_1 -> i
        let s_121_2: i128 = (s_121_1.value() as i128);
        // C s_121_3: cast reint s_121_2 -> i64
        let s_121_3: i64 = (s_121_2 as i64);
        // C s_121_4: cast zx s_121_3 -> i
        let s_121_4: i128 = (i128::try_from(s_121_3).unwrap());
        // C s_121_5: const #432u : u32
        let s_121_5: u32 = 432;
        // D s_121_6: read-reg s_121_5:u8
        let s_121_6: u8 = {
            let value = state.read_register::<u8>(s_121_5 as isize);
            tracer.read_register(s_121_5 as isize, value);
            value
        };
        // D s_121_7: call AArch64_SystemAccessTrap(s_121_6, s_121_4)
        let s_121_7: () = AArch64_SystemAccessTrap(state, tracer, s_121_6, s_121_4);
        // N s_121_8: return
        return;
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_122_0: read-var __HDFGRTR2_EL2_nSPMSCR_EL1:u8
        let s_122_0: bool = fn_state.u__HDFGRTR2_EL2_nSPMSCR_EL1;
        // D s_122_1: cast zx s_122_0 -> bv
        let s_122_1: Bits = Bits::new(s_122_0 as u128, 1u16);
        // C s_122_2: const #0u : u8
        let s_122_2: bool = false;
        // C s_122_3: cast zx s_122_2 -> bv
        let s_122_3: Bits = Bits::new(s_122_2 as u128, 1u16);
        // D s_122_4: cmp-eq s_122_1 s_122_3
        let s_122_4: bool = ((s_122_1) == (s_122_3));
        // D s_122_5: write-var gs#74010 <= s_122_4
        fn_state.gs_74010 = s_122_4;
        // N s_122_6: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_123_0: const #188u : u32
        let s_123_0: u32 = 188;
        // S s_123_1: call IsFeatureImplemented(s_123_0)
        let s_123_1: bool = IsFeatureImplemented(state, tracer, s_123_0);
        // D s_123_2: write-var gs#74009 <= s_123_1
        fn_state.gs_74009 = s_123_1;
        // N s_123_3: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #24u : u8
        let s_124_0: u8 = 24;
        // C s_124_1: cast zx s_124_0 -> bv
        let s_124_1: Bits = Bits::new(s_124_0 as u128, 8u16);
        // C s_124_2: cast zx s_124_1 -> i
        let s_124_2: i128 = (s_124_1.value() as i128);
        // C s_124_3: cast reint s_124_2 -> i64
        let s_124_3: i64 = (s_124_2 as i64);
        // C s_124_4: cast zx s_124_3 -> i
        let s_124_4: i128 = (i128::try_from(s_124_3).unwrap());
        // C s_124_5: const #432u : u32
        let s_124_5: u32 = 432;
        // D s_124_6: read-reg s_124_5:u8
        let s_124_6: u8 = {
            let value = state.read_register::<u8>(s_124_5 as isize);
            tracer.read_register(s_124_5 as isize, value);
            value
        };
        // D s_124_7: call AArch64_SystemAccessTrap(s_124_6, s_124_4)
        let s_124_7: () = AArch64_SystemAccessTrap(state, tracer, s_124_6, s_124_4);
        // N s_124_8: return
        return;
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_125_0: read-var __SCR_EL3_FGTEn2:u8
        let s_125_0: bool = fn_state.u__SCR_EL3_FGTEn2;
        // D s_125_1: cast zx s_125_0 -> bv
        let s_125_1: Bits = Bits::new(s_125_0 as u128, 1u16);
        // C s_125_2: const #0u : u8
        let s_125_2: bool = false;
        // C s_125_3: cast zx s_125_2 -> bv
        let s_125_3: Bits = Bits::new(s_125_2 as u128, 1u16);
        // D s_125_4: cmp-eq s_125_1 s_125_3
        let s_125_4: bool = ((s_125_1) == (s_125_3));
        // D s_125_5: write-var gs#74008 <= s_125_4
        fn_state.gs_74008 = s_125_4;
        // N s_125_6: jump b84
        return block_84(state, tracer, fn_state);
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
        // D s_126_4: write-var gs#74007 <= s_126_3
        fn_state.gs_74007 = s_126_3;
        // N s_126_5: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #188u : u32
        let s_127_0: u32 = 188;
        // S s_127_1: call IsFeatureImplemented(s_127_0)
        let s_127_1: bool = IsFeatureImplemented(state, tracer, s_127_0);
        // D s_127_2: write-var gs#74006 <= s_127_1
        fn_state.gs_74006 = s_127_1;
        // N s_127_3: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_128_0: panic
        panic!("{:?}", ());
        // N s_128_1: return
        return;
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_129_0: const #() : ()
        let s_129_0: () = ();
        // S s_129_1: call __get_selected_SPMACCESSR_EL3_field(s_129_0)
        let s_129_1: u8 = u__get_selected_SPMACCESSR_EL3_field(state, tracer, s_129_0);
        // S s_129_2: cast zx s_129_1 -> bv
        let s_129_2: Bits = Bits::new(s_129_1 as u128, 2u16);
        // C s_129_3: const #0u : u8
        let s_129_3: u8 = 0;
        // C s_129_4: cast zx s_129_3 -> bv
        let s_129_4: Bits = Bits::new(s_129_3 as u128, 2u16);
        // S s_129_5: cmp-eq s_129_2 s_129_4
        let s_129_5: bool = ((s_129_2) == (s_129_4));
        // D s_129_6: write-var gs#74005 <= s_129_5
        fn_state.gs_74005 = s_129_5;
        // N s_129_7: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_130_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_130_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_130_1: call __IMPDEF_boolean(s_130_0)
        let s_130_1: bool = u__IMPDEF_boolean(state, tracer, s_130_0);
        // D s_130_2: write-var gs#74004 <= s_130_1
        fn_state.gs_74004 = s_130_1;
        // N s_130_3: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_131_0: const #() : ()
        let s_131_0: () = ();
        // S s_131_1: call EDSCR_read(s_131_0)
        let s_131_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_131_0);
        // S s_131_2: call _get_EDSCR_Type_SDD(s_131_1)
        let s_131_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_131_1);
        // S s_131_3: cast zx s_131_2 -> bv
        let s_131_3: Bits = Bits::new(s_131_2 as u128, 1u16);
        // C s_131_4: const #1u : u8
        let s_131_4: bool = true;
        // C s_131_5: cast zx s_131_4 -> bv
        let s_131_5: Bits = Bits::new(s_131_4 as u128, 1u16);
        // S s_131_6: cmp-eq s_131_3 s_131_5
        let s_131_6: bool = ((s_131_3) == (s_131_5));
        // D s_131_7: write-var gs#74003 <= s_131_6
        fn_state.gs_74003 = s_131_6;
        // N s_131_8: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_132_0: const #424u : u32
        let s_132_0: u32 = 424;
        // D s_132_1: read-reg s_132_0:u8
        let s_132_1: u8 = {
            let value = state.read_register::<u8>(s_132_0 as isize);
            tracer.read_register(s_132_0 as isize, value);
            value
        };
        // C s_132_2: const #2u : u8
        let s_132_2: u8 = 2;
        // D s_132_3: cmp-lt s_132_1 s_132_2
        let s_132_3: bool = ((s_132_1) < (s_132_2));
        // D s_132_4: write-var gs#74002 <= s_132_3
        fn_state.gs_74002 = s_132_3;
        // N s_132_5: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_133_0: panic
        panic!("{:?}", ());
        // N s_133_1: return
        return;
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_134_0: read-var __MDCR_EL3_EnPM2:u8
        let s_134_0: bool = fn_state.u__MDCR_EL3_EnPM2;
        // D s_134_1: cast zx s_134_0 -> bv
        let s_134_1: Bits = Bits::new(s_134_0 as u128, 1u16);
        // C s_134_2: const #0u : u8
        let s_134_2: bool = false;
        // C s_134_3: cast zx s_134_2 -> bv
        let s_134_3: Bits = Bits::new(s_134_2 as u128, 1u16);
        // D s_134_4: cmp-eq s_134_1 s_134_3
        let s_134_4: bool = ((s_134_1) == (s_134_3));
        // D s_134_5: write-var gs#74001 <= s_134_4
        fn_state.gs_74001 = s_134_4;
        // N s_134_6: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_135_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_135_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_135_1: call __IMPDEF_boolean(s_135_0)
        let s_135_1: bool = u__IMPDEF_boolean(state, tracer, s_135_0);
        // D s_135_2: write-var gs#74000 <= s_135_1
        fn_state.gs_74000 = s_135_1;
        // N s_135_3: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_136_0: const #() : ()
        let s_136_0: () = ();
        // S s_136_1: call EDSCR_read(s_136_0)
        let s_136_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_136_0);
        // S s_136_2: call _get_EDSCR_Type_SDD(s_136_1)
        let s_136_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_136_1);
        // S s_136_3: cast zx s_136_2 -> bv
        let s_136_3: Bits = Bits::new(s_136_2 as u128, 1u16);
        // C s_136_4: const #1u : u8
        let s_136_4: bool = true;
        // C s_136_5: cast zx s_136_4 -> bv
        let s_136_5: Bits = Bits::new(s_136_4 as u128, 1u16);
        // S s_136_6: cmp-eq s_136_3 s_136_5
        let s_136_6: bool = ((s_136_3) == (s_136_5));
        // D s_136_7: write-var gs#73999 <= s_136_6
        fn_state.gs_73999 = s_136_6;
        // N s_136_8: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_137_0: const #424u : u32
        let s_137_0: u32 = 424;
        // D s_137_1: read-reg s_137_0:u8
        let s_137_1: u8 = {
            let value = state.read_register::<u8>(s_137_0 as isize);
            tracer.read_register(s_137_0 as isize, value);
            value
        };
        // C s_137_2: const #2u : u8
        let s_137_2: u8 = 2;
        // D s_137_3: cmp-lt s_137_1 s_137_2
        let s_137_3: bool = ((s_137_1) < (s_137_2));
        // D s_137_4: write-var gs#73998 <= s_137_3
        fn_state.gs_73998 = s_137_3;
        // N s_137_5: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_138_0: panic
        panic!("{:?}", ());
        // N s_138_1: return
        return;
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_139_0: panic
        panic!("{:?}", ());
        // N s_139_1: return
        return;
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_140_0: const #2u : u32
        let s_140_0: u32 = 2;
        // S s_140_1: call IsCurrentSecurityState(s_140_0)
        let s_140_1: bool = IsCurrentSecurityState(state, tracer, s_140_0);
        // D s_140_2: write-var gs#73982 <= s_140_1
        fn_state.gs_73982 = s_140_1;
        // N s_140_3: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_141_0: const #1u : u8
        let s_141_0: bool = true;
        // D s_141_1: write-var gs#73983 <= s_141_0
        fn_state.gs_73983 = s_141_0;
        // N s_141_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
