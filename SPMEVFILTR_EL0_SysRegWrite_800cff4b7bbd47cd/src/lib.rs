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
use u_get_HCR_EL2_Type_E2H::*;
use Halted::*;
use IsFeatureImplemented::*;
use X_read::*;
use AArch64_SystemAccessTrap::*;
use SPMEVFILTR_EL0_set::*;
use u_get_MDCR_EL2_Type_EnSPM::*;
use u_get_SPMSELR_EL0_Type_SYSPMUSEL::*;
use u__IMPDEF_boolean::*;
use u_get_MDSCR_EL1_Type_EnSPM::*;
use u_get_HDFGWTR2_EL2_Type_nSPMEVTYPERn_EL0::*;
use u_get_SCR_EL3_Type_FGTEn2::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_SPMSELR_EL0_Type_BANK::*;
use u_get_HCR_EL2_Type_TGE::*;
use u_get_MDCR_EL3_Type_EnPM2::*;
use EDSCR_read::*;
use EL2Enabled::*;
use common::*;
pub fn SPMEVFILTR_EL0_SysRegWrite_800cff4b7bbd47cd<T: Tracer>(
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
        gs_94819: bool,
        u__MDCR_EL3_EnPM2: bool,
        gs_94824: bool,
        gs_94815: bool,
        gs_94813: bool,
        gs_94798: bool,
        gs_94784: bool,
        gs_94785: bool,
        gs_94794: bool,
        gs_94800: bool,
        gs_94823: bool,
        u__SCR_EL3_FGTEn2: bool,
        gs_94811: bool,
        gs_94795: bool,
        gs_94812: bool,
        u__HDFGWTR2_EL2_nSPMEVTYPERn_EL0: bool,
        gs_94799: bool,
        gs_94808: bool,
        u__PSTATE_EL: u8,
        gs_94791: bool,
        gs_94810: bool,
        gs_94793: bool,
        u__HCR_EL2_TGE: bool,
        u__EDSCR_SDD: bool,
        gs_94814: bool,
        gs_94802: bool,
        gs_94796: bool,
        gs_94816: bool,
        gs_94806: bool,
        u__MDSCR_EL1_EnSPM: bool,
        u__MDCR_EL2_EnSPM: bool,
        gs_94783: bool,
        gs_94786: bool,
        gs_94792: bool,
        gs_94797: bool,
        gs_94787: bool,
        gs_94818: bool,
        gs_94801: bool,
        gs_94807: bool,
        gs_94809: bool,
        gs_94817: bool,
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
        // C s_0_3: const #() : ()
        let s_0_3: () = ();
        // S s_0_4: call EDSCR_read(s_0_3)
        let s_0_4: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_0_3);
        // S s_0_5: call _get_EDSCR_Type_SDD(s_0_4)
        let s_0_5: bool = u_get_EDSCR_Type_SDD(state, tracer, s_0_4);
        // D s_0_6: write-var __EDSCR_SDD <= s_0_5
        fn_state.u__EDSCR_SDD = s_0_5;
        // C s_0_7: const #22712u : u32
        let s_0_7: u32 = 22712;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_MDCR_EL3_Type_EnPM2(s_0_8)
        let s_0_9: bool = u_get_MDCR_EL3_Type_EnPM2(state, tracer, s_0_8);
        // D s_0_10: write-var __MDCR_EL3_EnPM2 <= s_0_9
        fn_state.u__MDCR_EL3_EnPM2 = s_0_9;
        // C s_0_11: const #104648u : u32
        let s_0_11: u32 = 104648;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_MDSCR_EL1_Type_EnSPM(s_0_12)
        let s_0_13: bool = u_get_MDSCR_EL1_Type_EnSPM(state, tracer, s_0_12);
        // D s_0_14: write-var __MDSCR_EL1_EnSPM <= s_0_13
        fn_state.u__MDSCR_EL1_EnSPM = s_0_13;
        // C s_0_15: const #102552u : u32
        let s_0_15: u32 = 102552;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_HCR_EL2_Type_TGE(s_0_16)
        let s_0_17: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_0_16);
        // D s_0_18: write-var __HCR_EL2_TGE <= s_0_17
        fn_state.u__HCR_EL2_TGE = s_0_17;
        // C s_0_19: const #90704u : u32
        let s_0_19: u32 = 90704;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_SCR_EL3_Type_FGTEn2(s_0_20)
        let s_0_21: bool = u_get_SCR_EL3_Type_FGTEn2(state, tracer, s_0_20);
        // D s_0_22: write-var __SCR_EL3_FGTEn2 <= s_0_21
        fn_state.u__SCR_EL3_FGTEn2 = s_0_21;
        // C s_0_23: const #17664u : u32
        let s_0_23: u32 = 17664;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_HDFGWTR2_EL2_Type_nSPMEVTYPERn_EL0(s_0_24)
        let s_0_25: bool = u_get_HDFGWTR2_EL2_Type_nSPMEVTYPERn_EL0(
            state,
            tracer,
            s_0_24,
        );
        // D s_0_26: write-var __HDFGWTR2_EL2_nSPMEVTYPERn_EL0 <= s_0_25
        fn_state.u__HDFGWTR2_EL2_nSPMEVTYPERn_EL0 = s_0_25;
        // C s_0_27: const #104880u : u32
        let s_0_27: u32 = 104880;
        // D s_0_28: read-reg s_0_27:struct
        let s_0_28: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_27 as isize);
            tracer.read_register(s_0_27 as isize, value);
            value
        };
        // D s_0_29: call _get_MDCR_EL2_Type_EnSPM(s_0_28)
        let s_0_29: bool = u_get_MDCR_EL2_Type_EnSPM(state, tracer, s_0_28);
        // D s_0_30: write-var __MDCR_EL2_EnSPM <= s_0_29
        fn_state.u__MDCR_EL2_EnSPM = s_0_29;
        // D s_0_31: read-var __PSTATE_EL:u8
        let s_0_31: u8 = fn_state.u__PSTATE_EL;
        // D s_0_32: cast zx s_0_31 -> bv
        let s_0_32: Bits = Bits::new(s_0_31 as u128, 2u16);
        // C s_0_33: const #448u : u32
        let s_0_33: u32 = 448;
        // D s_0_34: read-reg s_0_33:u8
        let s_0_34: u8 = {
            let value = state.read_register::<u8>(s_0_33 as isize);
            tracer.read_register(s_0_33 as isize, value);
            value
        };
        // D s_0_35: cast zx s_0_34 -> bv
        let s_0_35: Bits = Bits::new(s_0_34 as u128, 2u16);
        // D s_0_36: cmp-eq s_0_32 s_0_35
        let s_0_36: bool = ((s_0_32) == (s_0_35));
        // N s_0_37: branch s_0_36 b80 b1
        if s_0_36 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var __PSTATE_EL:u8
        let s_1_0: u8 = fn_state.u__PSTATE_EL;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 2u16);
        // C s_1_2: const #440u : u32
        let s_1_2: u32 = 440;
        // D s_1_3: read-reg s_1_2:u8
        let s_1_3: u8 = {
            let value = state.read_register::<u8>(s_1_2 as isize);
            tracer.read_register(s_1_2 as isize, value);
            value
        };
        // D s_1_4: cast zx s_1_3 -> bv
        let s_1_4: Bits = Bits::new(s_1_3 as u128, 2u16);
        // D s_1_5: cmp-eq s_1_1 s_1_4
        let s_1_5: bool = ((s_1_1) == (s_1_4));
        // N s_1_6: branch s_1_5 b31 b2
        if s_1_5 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var __PSTATE_EL:u8
        let s_2_0: u8 = fn_state.u__PSTATE_EL;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // C s_2_2: const #432u : u32
        let s_2_2: u32 = 432;
        // D s_2_3: read-reg s_2_2:u8
        let s_2_3: u8 = {
            let value = state.read_register::<u8>(s_2_2 as isize);
            tracer.read_register(s_2_2 as isize, value);
            value
        };
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 2u16);
        // D s_2_5: cmp-eq s_2_1 s_2_4
        let s_2_5: bool = ((s_2_1) == (s_2_4));
        // N s_2_6: branch s_2_5 b6 b3
        if s_2_5 {
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
        // D s_3_0: read-var __PSTATE_EL:u8
        let s_3_0: u8 = fn_state.u__PSTATE_EL;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_2: const #424u : u32
        let s_3_2: u32 = 424;
        // D s_3_3: read-reg s_3_2:u8
        let s_3_3: u8 = {
            let value = state.read_register::<u8>(s_3_2 as isize);
            tracer.read_register(s_3_2 as isize, value);
            value
        };
        // D s_3_4: cast zx s_3_3 -> bv
        let s_3_4: Bits = Bits::new(s_3_3 as u128, 2u16);
        // D s_3_5: cmp-eq s_3_1 s_3_4
        let s_3_5: bool = ((s_3_1) == (s_3_4));
        // N s_3_6: branch s_3_5 b5 b4
        if s_3_5 {
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
        // N s_4_0: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #16552u : u32
        let s_5_0: u32 = 16552;
        // D s_5_1: read-reg s_5_0:struct
        let s_5_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: call _get_SPMSELR_EL0_Type_SYSPMUSEL(s_5_1)
        let s_5_2: u8 = u_get_SPMSELR_EL0_Type_SYSPMUSEL(state, tracer, s_5_1);
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 6u16);
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (s_5_3.value() as i128);
        // D s_5_5: cast reint s_5_4 -> i64
        let s_5_5: i64 = (s_5_4 as i64);
        // C s_5_6: const #16552u : u32
        let s_5_6: u32 = 16552;
        // D s_5_7: read-reg s_5_6:struct
        let s_5_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_6 as isize);
            tracer.read_register(s_5_6 as isize, value);
            value
        };
        // D s_5_8: call _get_SPMSELR_EL0_Type_BANK(s_5_7)
        let s_5_8: u8 = u_get_SPMSELR_EL0_Type_BANK(state, tracer, s_5_7);
        // D s_5_9: cast zx s_5_8 -> bv
        let s_5_9: Bits = Bits::new(s_5_8 as u128, 2u16);
        // D s_5_10: cast zx s_5_9 -> i
        let s_5_10: i128 = (s_5_9.value() as i128);
        // D s_5_11: cast reint s_5_10 -> i64
        let s_5_11: i64 = (s_5_10 as i64);
        // C s_5_12: const #16s : i
        let s_5_12: i128 = 16;
        // D s_5_13: cast zx s_5_11 -> i
        let s_5_13: i128 = (i128::try_from(s_5_11).unwrap());
        // D s_5_14: mul s_5_13 s_5_12
        let s_5_14: i128 = ((s_5_13) * (s_5_12));
        // D s_5_15: cast reint s_5_14 -> i64
        let s_5_15: i64 = (s_5_14 as i64);
        // C s_5_16: const #2s : i
        let s_5_16: i128 = 2;
        // D s_5_17: cast zx s_5_15 -> i
        let s_5_17: i128 = (i128::try_from(s_5_15).unwrap());
        // D s_5_18: add s_5_17 s_5_16
        let s_5_18: i128 = (s_5_17 + s_5_16);
        // D s_5_19: cast reint s_5_18 -> i64
        let s_5_19: i64 = (s_5_18 as i64);
        // C s_5_20: const #64s : i64
        let s_5_20: i64 = 64;
        // D s_5_21: read-var t:i
        let s_5_21: i128 = fn_state.t;
        // D s_5_22: call X_read(s_5_21, s_5_20)
        let s_5_22: Bits = X_read(state, tracer, s_5_21, s_5_20);
        // D s_5_23: cast reint s_5_22 -> u64
        let s_5_23: u64 = (s_5_22.value() as u64);
        // D s_5_24: cast zx s_5_5 -> i
        let s_5_24: i128 = (i128::try_from(s_5_5).unwrap());
        // D s_5_25: cast zx s_5_19 -> i
        let s_5_25: i128 = (i128::try_from(s_5_19).unwrap());
        // D s_5_26: call SPMEVFILTR_EL0_set(s_5_24, s_5_25, s_5_23)
        let s_5_26: () = SPMEVFILTR_EL0_set(state, tracer, s_5_24, s_5_25, s_5_23);
        // N s_5_27: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call Halted(s_6_0)
        let s_6_1: bool = Halted(state, tracer, s_6_0);
        // N s_6_2: branch s_6_1 b30 b7
        if s_6_1 {
            return block_30(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#94783 <= s_7_0
        fn_state.gs_94783 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#94783:u8
        let s_8_0: bool = fn_state.gs_94783;
        // N s_8_1: branch s_8_0 b29 b9
        if s_8_0 {
            return block_29(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#94784 <= s_9_0
        fn_state.gs_94784 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#94784:u8
        let s_10_0: bool = fn_state.gs_94784;
        // N s_10_1: branch s_10_0 b28 b11
        if s_10_0 {
            return block_28(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#94785 <= s_11_0
        fn_state.gs_94785 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#94785:u8
        let s_12_0: bool = fn_state.gs_94785;
        // N s_12_1: branch s_12_0 b27 b13
        if s_12_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#94786 <= s_13_0
        fn_state.gs_94786 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#94786:u8
        let s_14_0: bool = fn_state.gs_94786;
        // N s_14_1: branch s_14_0 b26 b15
        if s_14_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #424u : u32
        let s_15_0: u32 = 424;
        // D s_15_1: read-reg s_15_0:u8
        let s_15_1: u8 = {
            let value = state.read_register::<u8>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // C s_15_2: const #2u : u8
        let s_15_2: u8 = 2;
        // D s_15_3: cmp-lt s_15_1 s_15_2
        let s_15_3: bool = ((s_15_1) < (s_15_2));
        // N s_15_4: branch s_15_3 b25 b16
        if s_15_3 {
            return block_25(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#94787 <= s_16_0
        fn_state.gs_94787 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#94787:u8
        let s_17_0: bool = fn_state.gs_94787;
        // N s_17_1: branch s_17_0 b19 b18
        if s_17_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #16552u : u32
        let s_18_0: u32 = 16552;
        // D s_18_1: read-reg s_18_0:struct
        let s_18_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // D s_18_2: call _get_SPMSELR_EL0_Type_SYSPMUSEL(s_18_1)
        let s_18_2: u8 = u_get_SPMSELR_EL0_Type_SYSPMUSEL(state, tracer, s_18_1);
        // D s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 6u16);
        // D s_18_4: cast zx s_18_3 -> i
        let s_18_4: i128 = (s_18_3.value() as i128);
        // D s_18_5: cast reint s_18_4 -> i64
        let s_18_5: i64 = (s_18_4 as i64);
        // C s_18_6: const #16552u : u32
        let s_18_6: u32 = 16552;
        // D s_18_7: read-reg s_18_6:struct
        let s_18_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_18_6 as isize);
            tracer.read_register(s_18_6 as isize, value);
            value
        };
        // D s_18_8: call _get_SPMSELR_EL0_Type_BANK(s_18_7)
        let s_18_8: u8 = u_get_SPMSELR_EL0_Type_BANK(state, tracer, s_18_7);
        // D s_18_9: cast zx s_18_8 -> bv
        let s_18_9: Bits = Bits::new(s_18_8 as u128, 2u16);
        // D s_18_10: cast zx s_18_9 -> i
        let s_18_10: i128 = (s_18_9.value() as i128);
        // D s_18_11: cast reint s_18_10 -> i64
        let s_18_11: i64 = (s_18_10 as i64);
        // C s_18_12: const #16s : i
        let s_18_12: i128 = 16;
        // D s_18_13: cast zx s_18_11 -> i
        let s_18_13: i128 = (i128::try_from(s_18_11).unwrap());
        // D s_18_14: mul s_18_13 s_18_12
        let s_18_14: i128 = ((s_18_13) * (s_18_12));
        // D s_18_15: cast reint s_18_14 -> i64
        let s_18_15: i64 = (s_18_14 as i64);
        // C s_18_16: const #2s : i
        let s_18_16: i128 = 2;
        // D s_18_17: cast zx s_18_15 -> i
        let s_18_17: i128 = (i128::try_from(s_18_15).unwrap());
        // D s_18_18: add s_18_17 s_18_16
        let s_18_18: i128 = (s_18_17 + s_18_16);
        // D s_18_19: cast reint s_18_18 -> i64
        let s_18_19: i64 = (s_18_18 as i64);
        // C s_18_20: const #64s : i64
        let s_18_20: i64 = 64;
        // D s_18_21: read-var t:i
        let s_18_21: i128 = fn_state.t;
        // D s_18_22: call X_read(s_18_21, s_18_20)
        let s_18_22: Bits = X_read(state, tracer, s_18_21, s_18_20);
        // D s_18_23: cast reint s_18_22 -> u64
        let s_18_23: u64 = (s_18_22.value() as u64);
        // D s_18_24: cast zx s_18_5 -> i
        let s_18_24: i128 = (i128::try_from(s_18_5).unwrap());
        // D s_18_25: cast zx s_18_19 -> i
        let s_18_25: i128 = (i128::try_from(s_18_19).unwrap());
        // D s_18_26: call SPMEVFILTR_EL0_set(s_18_24, s_18_25, s_18_23)
        let s_18_26: () = SPMEVFILTR_EL0_set(state, tracer, s_18_24, s_18_25, s_18_23);
        // N s_18_27: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call Halted(s_19_0)
        let s_19_1: bool = Halted(state, tracer, s_19_0);
        // N s_19_2: branch s_19_1 b24 b20
        if s_19_1 {
            return block_24(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#94791 <= s_20_0
        fn_state.gs_94791 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#94791:u8
        let s_21_0: bool = fn_state.gs_94791;
        // N s_21_1: branch s_21_0 b23 b22
        if s_21_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #24u : u8
        let s_22_0: u8 = 24;
        // C s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 8u16);
        // C s_22_2: cast zx s_22_1 -> i
        let s_22_2: i128 = (s_22_1.value() as i128);
        // C s_22_3: cast reint s_22_2 -> i64
        let s_22_3: i64 = (s_22_2 as i64);
        // C s_22_4: cast zx s_22_3 -> i
        let s_22_4: i128 = (i128::try_from(s_22_3).unwrap());
        // C s_22_5: const #424u : u32
        let s_22_5: u32 = 424;
        // D s_22_6: read-reg s_22_5:u8
        let s_22_6: u8 = {
            let value = state.read_register::<u8>(s_22_5 as isize);
            tracer.read_register(s_22_5 as isize, value);
            value
        };
        // D s_22_7: call AArch64_SystemAccessTrap(s_22_6, s_22_4)
        let s_22_7: () = AArch64_SystemAccessTrap(state, tracer, s_22_6, s_22_4);
        // N s_22_8: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: panic
        panic!("{:?}", ());
        // N s_23_1: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var __EDSCR_SDD:u8
        let s_24_0: bool = fn_state.u__EDSCR_SDD;
        // D s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 1u16);
        // C s_24_2: const #1u : u8
        let s_24_2: bool = true;
        // C s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 1u16);
        // D s_24_4: cmp-eq s_24_1 s_24_3
        let s_24_4: bool = ((s_24_1) == (s_24_3));
        // D s_24_5: write-var gs#94791 <= s_24_4
        fn_state.gs_94791 = s_24_4;
        // N s_24_6: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var __MDCR_EL3_EnPM2:u8
        let s_25_0: bool = fn_state.u__MDCR_EL3_EnPM2;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 1u16);
        // C s_25_2: const #0u : u8
        let s_25_2: bool = false;
        // C s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // D s_25_4: cmp-eq s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) == (s_25_3));
        // D s_25_5: write-var gs#94787 <= s_25_4
        fn_state.gs_94787 = s_25_4;
        // N s_25_6: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_26_0: panic
        panic!("{:?}", ());
        // N s_26_1: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var __MDCR_EL3_EnPM2:u8
        let s_27_0: bool = fn_state.u__MDCR_EL3_EnPM2;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 1u16);
        // C s_27_2: const #0u : u8
        let s_27_2: bool = false;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: write-var gs#94786 <= s_27_4
        fn_state.gs_94786 = s_27_4;
        // N s_27_6: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_28_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_28_1: call __IMPDEF_boolean(s_28_0)
        let s_28_1: bool = u__IMPDEF_boolean(state, tracer, s_28_0);
        // D s_28_2: write-var gs#94785 <= s_28_1
        fn_state.gs_94785 = s_28_1;
        // N s_28_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var __EDSCR_SDD:u8
        let s_29_0: bool = fn_state.u__EDSCR_SDD;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 1u16);
        // C s_29_2: const #1u : u8
        let s_29_2: bool = true;
        // C s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // D s_29_4: cmp-eq s_29_1 s_29_3
        let s_29_4: bool = ((s_29_1) == (s_29_3));
        // D s_29_5: write-var gs#94784 <= s_29_4
        fn_state.gs_94784 = s_29_4;
        // N s_29_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #424u : u32
        let s_30_0: u32 = 424;
        // D s_30_1: read-reg s_30_0:u8
        let s_30_1: u8 = {
            let value = state.read_register::<u8>(s_30_0 as isize);
            tracer.read_register(s_30_0 as isize, value);
            value
        };
        // C s_30_2: const #2u : u8
        let s_30_2: u8 = 2;
        // D s_30_3: cmp-lt s_30_1 s_30_2
        let s_30_3: bool = ((s_30_1) < (s_30_2));
        // D s_30_4: write-var gs#94783 <= s_30_3
        fn_state.gs_94783 = s_30_3;
        // N s_30_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #() : ()
        let s_31_0: () = ();
        // S s_31_1: call Halted(s_31_0)
        let s_31_1: bool = Halted(state, tracer, s_31_0);
        // N s_31_2: branch s_31_1 b79 b32
        if s_31_1 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #0u : u8
        let s_32_0: bool = false;
        // D s_32_1: write-var gs#94792 <= s_32_0
        fn_state.gs_94792 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#94792:u8
        let s_33_0: bool = fn_state.gs_94792;
        // N s_33_1: branch s_33_0 b78 b34
        if s_33_0 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #0u : u8
        let s_34_0: bool = false;
        // D s_34_1: write-var gs#94793 <= s_34_0
        fn_state.gs_94793 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#94793:u8
        let s_35_0: bool = fn_state.gs_94793;
        // N s_35_1: branch s_35_0 b77 b36
        if s_35_0 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #0u : u8
        let s_36_0: bool = false;
        // D s_36_1: write-var gs#94794 <= s_36_0
        fn_state.gs_94794 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#94794:u8
        let s_37_0: bool = fn_state.gs_94794;
        // N s_37_1: branch s_37_0 b76 b38
        if s_37_0 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #0u : u8
        let s_38_0: bool = false;
        // D s_38_1: write-var gs#94795 <= s_38_0
        fn_state.gs_94795 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#94795:u8
        let s_39_0: bool = fn_state.gs_94795;
        // N s_39_1: branch s_39_0 b75 b40
        if s_39_0 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #() : ()
        let s_40_0: () = ();
        // S s_40_1: call EL2Enabled(s_40_0)
        let s_40_1: bool = EL2Enabled(state, tracer, s_40_0);
        // N s_40_2: branch s_40_1 b74 b41
        if s_40_1 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #0u : u8
        let s_41_0: bool = false;
        // D s_41_1: write-var gs#94796 <= s_41_0
        fn_state.gs_94796 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#94796:u8
        let s_42_0: bool = fn_state.gs_94796;
        // N s_42_1: branch s_42_0 b73 b43
        if s_42_0 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #0u : u8
        let s_43_0: bool = false;
        // D s_43_1: write-var gs#94797 <= s_43_0
        fn_state.gs_94797 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#94797:u8
        let s_44_0: bool = fn_state.gs_94797;
        // N s_44_1: branch s_44_0 b72 b45
        if s_44_0 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #0u : u8
        let s_45_0: bool = false;
        // D s_45_1: write-var gs#94798 <= s_45_0
        fn_state.gs_94798 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#94798:u8
        let s_46_0: bool = fn_state.gs_94798;
        // N s_46_1: branch s_46_0 b71 b47
        if s_46_0 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #() : ()
        let s_47_0: () = ();
        // S s_47_1: call EL2Enabled(s_47_0)
        let s_47_1: bool = EL2Enabled(state, tracer, s_47_0);
        // N s_47_2: branch s_47_1 b70 b48
        if s_47_1 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #0u : u8
        let s_48_0: bool = false;
        // D s_48_1: write-var gs#94799 <= s_48_0
        fn_state.gs_94799 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#94799:u8
        let s_49_0: bool = fn_state.gs_94799;
        // N s_49_1: branch s_49_0 b69 b50
        if s_49_0 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #0u : u8
        let s_50_0: bool = false;
        // D s_50_1: write-var gs#94800 <= s_50_0
        fn_state.gs_94800 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#94800:u8
        let s_51_0: bool = fn_state.gs_94800;
        // N s_51_1: branch s_51_0 b68 b52
        if s_51_0 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #() : ()
        let s_52_0: () = ();
        // S s_52_1: call EL2Enabled(s_52_0)
        let s_52_1: bool = EL2Enabled(state, tracer, s_52_0);
        // N s_52_2: branch s_52_1 b67 b53
        if s_52_1 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #0u : u8
        let s_53_0: bool = false;
        // D s_53_1: write-var gs#94801 <= s_53_0
        fn_state.gs_94801 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#94801:u8
        let s_54_0: bool = fn_state.gs_94801;
        // N s_54_1: branch s_54_0 b66 b55
        if s_54_0 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #424u : u32
        let s_55_0: u32 = 424;
        // D s_55_1: read-reg s_55_0:u8
        let s_55_1: u8 = {
            let value = state.read_register::<u8>(s_55_0 as isize);
            tracer.read_register(s_55_0 as isize, value);
            value
        };
        // C s_55_2: const #2u : u8
        let s_55_2: u8 = 2;
        // D s_55_3: cmp-lt s_55_1 s_55_2
        let s_55_3: bool = ((s_55_1) < (s_55_2));
        // N s_55_4: branch s_55_3 b65 b56
        if s_55_3 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #0u : u8
        let s_56_0: bool = false;
        // D s_56_1: write-var gs#94802 <= s_56_0
        fn_state.gs_94802 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#94802:u8
        let s_57_0: bool = fn_state.gs_94802;
        // N s_57_1: branch s_57_0 b59 b58
        if s_57_0 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #16552u : u32
        let s_58_0: u32 = 16552;
        // D s_58_1: read-reg s_58_0:struct
        let s_58_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_58_0 as isize);
            tracer.read_register(s_58_0 as isize, value);
            value
        };
        // D s_58_2: call _get_SPMSELR_EL0_Type_SYSPMUSEL(s_58_1)
        let s_58_2: u8 = u_get_SPMSELR_EL0_Type_SYSPMUSEL(state, tracer, s_58_1);
        // D s_58_3: cast zx s_58_2 -> bv
        let s_58_3: Bits = Bits::new(s_58_2 as u128, 6u16);
        // D s_58_4: cast zx s_58_3 -> i
        let s_58_4: i128 = (s_58_3.value() as i128);
        // D s_58_5: cast reint s_58_4 -> i64
        let s_58_5: i64 = (s_58_4 as i64);
        // C s_58_6: const #16552u : u32
        let s_58_6: u32 = 16552;
        // D s_58_7: read-reg s_58_6:struct
        let s_58_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_58_6 as isize);
            tracer.read_register(s_58_6 as isize, value);
            value
        };
        // D s_58_8: call _get_SPMSELR_EL0_Type_BANK(s_58_7)
        let s_58_8: u8 = u_get_SPMSELR_EL0_Type_BANK(state, tracer, s_58_7);
        // D s_58_9: cast zx s_58_8 -> bv
        let s_58_9: Bits = Bits::new(s_58_8 as u128, 2u16);
        // D s_58_10: cast zx s_58_9 -> i
        let s_58_10: i128 = (s_58_9.value() as i128);
        // D s_58_11: cast reint s_58_10 -> i64
        let s_58_11: i64 = (s_58_10 as i64);
        // C s_58_12: const #16s : i
        let s_58_12: i128 = 16;
        // D s_58_13: cast zx s_58_11 -> i
        let s_58_13: i128 = (i128::try_from(s_58_11).unwrap());
        // D s_58_14: mul s_58_13 s_58_12
        let s_58_14: i128 = ((s_58_13) * (s_58_12));
        // D s_58_15: cast reint s_58_14 -> i64
        let s_58_15: i64 = (s_58_14 as i64);
        // C s_58_16: const #2s : i
        let s_58_16: i128 = 2;
        // D s_58_17: cast zx s_58_15 -> i
        let s_58_17: i128 = (i128::try_from(s_58_15).unwrap());
        // D s_58_18: add s_58_17 s_58_16
        let s_58_18: i128 = (s_58_17 + s_58_16);
        // D s_58_19: cast reint s_58_18 -> i64
        let s_58_19: i64 = (s_58_18 as i64);
        // C s_58_20: const #64s : i64
        let s_58_20: i64 = 64;
        // D s_58_21: read-var t:i
        let s_58_21: i128 = fn_state.t;
        // D s_58_22: call X_read(s_58_21, s_58_20)
        let s_58_22: Bits = X_read(state, tracer, s_58_21, s_58_20);
        // D s_58_23: cast reint s_58_22 -> u64
        let s_58_23: u64 = (s_58_22.value() as u64);
        // D s_58_24: cast zx s_58_5 -> i
        let s_58_24: i128 = (i128::try_from(s_58_5).unwrap());
        // D s_58_25: cast zx s_58_19 -> i
        let s_58_25: i128 = (i128::try_from(s_58_19).unwrap());
        // D s_58_26: call SPMEVFILTR_EL0_set(s_58_24, s_58_25, s_58_23)
        let s_58_26: () = SPMEVFILTR_EL0_set(state, tracer, s_58_24, s_58_25, s_58_23);
        // N s_58_27: return
        return;
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #() : ()
        let s_59_0: () = ();
        // S s_59_1: call Halted(s_59_0)
        let s_59_1: bool = Halted(state, tracer, s_59_0);
        // N s_59_2: branch s_59_1 b64 b60
        if s_59_1 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #0u : u8
        let s_60_0: bool = false;
        // D s_60_1: write-var gs#94806 <= s_60_0
        fn_state.gs_94806 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#94806:u8
        let s_61_0: bool = fn_state.gs_94806;
        // N s_61_1: branch s_61_0 b63 b62
        if s_61_0 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #24u : u8
        let s_62_0: u8 = 24;
        // C s_62_1: cast zx s_62_0 -> bv
        let s_62_1: Bits = Bits::new(s_62_0 as u128, 8u16);
        // C s_62_2: cast zx s_62_1 -> i
        let s_62_2: i128 = (s_62_1.value() as i128);
        // C s_62_3: cast reint s_62_2 -> i64
        let s_62_3: i64 = (s_62_2 as i64);
        // C s_62_4: cast zx s_62_3 -> i
        let s_62_4: i128 = (i128::try_from(s_62_3).unwrap());
        // C s_62_5: const #424u : u32
        let s_62_5: u32 = 424;
        // D s_62_6: read-reg s_62_5:u8
        let s_62_6: u8 = {
            let value = state.read_register::<u8>(s_62_5 as isize);
            tracer.read_register(s_62_5 as isize, value);
            value
        };
        // D s_62_7: call AArch64_SystemAccessTrap(s_62_6, s_62_4)
        let s_62_7: () = AArch64_SystemAccessTrap(state, tracer, s_62_6, s_62_4);
        // N s_62_8: return
        return;
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_63_0: panic
        panic!("{:?}", ());
        // N s_63_1: return
        return;
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var __EDSCR_SDD:u8
        let s_64_0: bool = fn_state.u__EDSCR_SDD;
        // D s_64_1: cast zx s_64_0 -> bv
        let s_64_1: Bits = Bits::new(s_64_0 as u128, 1u16);
        // C s_64_2: const #1u : u8
        let s_64_2: bool = true;
        // C s_64_3: cast zx s_64_2 -> bv
        let s_64_3: Bits = Bits::new(s_64_2 as u128, 1u16);
        // D s_64_4: cmp-eq s_64_1 s_64_3
        let s_64_4: bool = ((s_64_1) == (s_64_3));
        // D s_64_5: write-var gs#94806 <= s_64_4
        fn_state.gs_94806 = s_64_4;
        // N s_64_6: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var __MDCR_EL3_EnPM2:u8
        let s_65_0: bool = fn_state.u__MDCR_EL3_EnPM2;
        // D s_65_1: cast zx s_65_0 -> bv
        let s_65_1: Bits = Bits::new(s_65_0 as u128, 1u16);
        // C s_65_2: const #0u : u8
        let s_65_2: bool = false;
        // C s_65_3: cast zx s_65_2 -> bv
        let s_65_3: Bits = Bits::new(s_65_2 as u128, 1u16);
        // D s_65_4: cmp-eq s_65_1 s_65_3
        let s_65_4: bool = ((s_65_1) == (s_65_3));
        // D s_65_5: write-var gs#94802 <= s_65_4
        fn_state.gs_94802 = s_65_4;
        // N s_65_6: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #24u : u8
        let s_66_0: u8 = 24;
        // C s_66_1: cast zx s_66_0 -> bv
        let s_66_1: Bits = Bits::new(s_66_0 as u128, 8u16);
        // C s_66_2: cast zx s_66_1 -> i
        let s_66_2: i128 = (s_66_1.value() as i128);
        // C s_66_3: cast reint s_66_2 -> i64
        let s_66_3: i64 = (s_66_2 as i64);
        // C s_66_4: cast zx s_66_3 -> i
        let s_66_4: i128 = (i128::try_from(s_66_3).unwrap());
        // C s_66_5: const #432u : u32
        let s_66_5: u32 = 432;
        // D s_66_6: read-reg s_66_5:u8
        let s_66_6: u8 = {
            let value = state.read_register::<u8>(s_66_5 as isize);
            tracer.read_register(s_66_5 as isize, value);
            value
        };
        // D s_66_7: call AArch64_SystemAccessTrap(s_66_6, s_66_4)
        let s_66_7: () = AArch64_SystemAccessTrap(state, tracer, s_66_6, s_66_4);
        // N s_66_8: return
        return;
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var __MDCR_EL2_EnSPM:u8
        let s_67_0: bool = fn_state.u__MDCR_EL2_EnSPM;
        // D s_67_1: cast zx s_67_0 -> bv
        let s_67_1: Bits = Bits::new(s_67_0 as u128, 1u16);
        // C s_67_2: const #0u : u8
        let s_67_2: bool = false;
        // C s_67_3: cast zx s_67_2 -> bv
        let s_67_3: Bits = Bits::new(s_67_2 as u128, 1u16);
        // D s_67_4: cmp-eq s_67_1 s_67_3
        let s_67_4: bool = ((s_67_1) == (s_67_3));
        // D s_67_5: write-var gs#94801 <= s_67_4
        fn_state.gs_94801 = s_67_4;
        // N s_67_6: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #24u : u8
        let s_68_0: u8 = 24;
        // C s_68_1: cast zx s_68_0 -> bv
        let s_68_1: Bits = Bits::new(s_68_0 as u128, 8u16);
        // C s_68_2: cast zx s_68_1 -> i
        let s_68_2: i128 = (s_68_1.value() as i128);
        // C s_68_3: cast reint s_68_2 -> i64
        let s_68_3: i64 = (s_68_2 as i64);
        // C s_68_4: cast zx s_68_3 -> i
        let s_68_4: i128 = (i128::try_from(s_68_3).unwrap());
        // C s_68_5: const #432u : u32
        let s_68_5: u32 = 432;
        // D s_68_6: read-reg s_68_5:u8
        let s_68_6: u8 = {
            let value = state.read_register::<u8>(s_68_5 as isize);
            tracer.read_register(s_68_5 as isize, value);
            value
        };
        // D s_68_7: call AArch64_SystemAccessTrap(s_68_6, s_68_4)
        let s_68_7: () = AArch64_SystemAccessTrap(state, tracer, s_68_6, s_68_4);
        // N s_68_8: return
        return;
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var __HDFGWTR2_EL2_nSPMEVTYPERn_EL0:u8
        let s_69_0: bool = fn_state.u__HDFGWTR2_EL2_nSPMEVTYPERn_EL0;
        // D s_69_1: cast zx s_69_0 -> bv
        let s_69_1: Bits = Bits::new(s_69_0 as u128, 1u16);
        // C s_69_2: const #0u : u8
        let s_69_2: bool = false;
        // C s_69_3: cast zx s_69_2 -> bv
        let s_69_3: Bits = Bits::new(s_69_2 as u128, 1u16);
        // D s_69_4: cmp-eq s_69_1 s_69_3
        let s_69_4: bool = ((s_69_1) == (s_69_3));
        // D s_69_5: write-var gs#94800 <= s_69_4
        fn_state.gs_94800 = s_69_4;
        // N s_69_6: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #188u : u32
        let s_70_0: u32 = 188;
        // S s_70_1: call IsFeatureImplemented(s_70_0)
        let s_70_1: bool = IsFeatureImplemented(state, tracer, s_70_0);
        // D s_70_2: write-var gs#94799 <= s_70_1
        fn_state.gs_94799 = s_70_1;
        // N s_70_3: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #24u : u8
        let s_71_0: u8 = 24;
        // C s_71_1: cast zx s_71_0 -> bv
        let s_71_1: Bits = Bits::new(s_71_0 as u128, 8u16);
        // C s_71_2: cast zx s_71_1 -> i
        let s_71_2: i128 = (s_71_1.value() as i128);
        // C s_71_3: cast reint s_71_2 -> i64
        let s_71_3: i64 = (s_71_2 as i64);
        // C s_71_4: cast zx s_71_3 -> i
        let s_71_4: i128 = (i128::try_from(s_71_3).unwrap());
        // C s_71_5: const #432u : u32
        let s_71_5: u32 = 432;
        // D s_71_6: read-reg s_71_5:u8
        let s_71_6: u8 = {
            let value = state.read_register::<u8>(s_71_5 as isize);
            tracer.read_register(s_71_5 as isize, value);
            value
        };
        // D s_71_7: call AArch64_SystemAccessTrap(s_71_6, s_71_4)
        let s_71_7: () = AArch64_SystemAccessTrap(state, tracer, s_71_6, s_71_4);
        // N s_71_8: return
        return;
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var __SCR_EL3_FGTEn2:u8
        let s_72_0: bool = fn_state.u__SCR_EL3_FGTEn2;
        // D s_72_1: cast zx s_72_0 -> bv
        let s_72_1: Bits = Bits::new(s_72_0 as u128, 1u16);
        // C s_72_2: const #0u : u8
        let s_72_2: bool = false;
        // C s_72_3: cast zx s_72_2 -> bv
        let s_72_3: Bits = Bits::new(s_72_2 as u128, 1u16);
        // D s_72_4: cmp-eq s_72_1 s_72_3
        let s_72_4: bool = ((s_72_1) == (s_72_3));
        // D s_72_5: write-var gs#94798 <= s_72_4
        fn_state.gs_94798 = s_72_4;
        // N s_72_6: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #424u : u32
        let s_73_0: u32 = 424;
        // D s_73_1: read-reg s_73_0:u8
        let s_73_1: u8 = {
            let value = state.read_register::<u8>(s_73_0 as isize);
            tracer.read_register(s_73_0 as isize, value);
            value
        };
        // C s_73_2: const #2u : u8
        let s_73_2: u8 = 2;
        // D s_73_3: cmp-lt s_73_1 s_73_2
        let s_73_3: bool = ((s_73_1) < (s_73_2));
        // D s_73_4: write-var gs#94797 <= s_73_3
        fn_state.gs_94797 = s_73_3;
        // N s_73_5: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #188u : u32
        let s_74_0: u32 = 188;
        // S s_74_1: call IsFeatureImplemented(s_74_0)
        let s_74_1: bool = IsFeatureImplemented(state, tracer, s_74_0);
        // D s_74_2: write-var gs#94796 <= s_74_1
        fn_state.gs_94796 = s_74_1;
        // N s_74_3: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_75_0: panic
        panic!("{:?}", ());
        // N s_75_1: return
        return;
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var __MDCR_EL3_EnPM2:u8
        let s_76_0: bool = fn_state.u__MDCR_EL3_EnPM2;
        // D s_76_1: cast zx s_76_0 -> bv
        let s_76_1: Bits = Bits::new(s_76_0 as u128, 1u16);
        // C s_76_2: const #0u : u8
        let s_76_2: bool = false;
        // C s_76_3: cast zx s_76_2 -> bv
        let s_76_3: Bits = Bits::new(s_76_2 as u128, 1u16);
        // D s_76_4: cmp-eq s_76_1 s_76_3
        let s_76_4: bool = ((s_76_1) == (s_76_3));
        // D s_76_5: write-var gs#94795 <= s_76_4
        fn_state.gs_94795 = s_76_4;
        // N s_76_6: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_77_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_77_1: call __IMPDEF_boolean(s_77_0)
        let s_77_1: bool = u__IMPDEF_boolean(state, tracer, s_77_0);
        // D s_77_2: write-var gs#94794 <= s_77_1
        fn_state.gs_94794 = s_77_1;
        // N s_77_3: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var __EDSCR_SDD:u8
        let s_78_0: bool = fn_state.u__EDSCR_SDD;
        // D s_78_1: cast zx s_78_0 -> bv
        let s_78_1: Bits = Bits::new(s_78_0 as u128, 1u16);
        // C s_78_2: const #1u : u8
        let s_78_2: bool = true;
        // C s_78_3: cast zx s_78_2 -> bv
        let s_78_3: Bits = Bits::new(s_78_2 as u128, 1u16);
        // D s_78_4: cmp-eq s_78_1 s_78_3
        let s_78_4: bool = ((s_78_1) == (s_78_3));
        // D s_78_5: write-var gs#94793 <= s_78_4
        fn_state.gs_94793 = s_78_4;
        // N s_78_6: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #424u : u32
        let s_79_0: u32 = 424;
        // D s_79_1: read-reg s_79_0:u8
        let s_79_1: u8 = {
            let value = state.read_register::<u8>(s_79_0 as isize);
            tracer.read_register(s_79_0 as isize, value);
            value
        };
        // C s_79_2: const #2u : u8
        let s_79_2: u8 = 2;
        // D s_79_3: cmp-lt s_79_1 s_79_2
        let s_79_3: bool = ((s_79_1) < (s_79_2));
        // D s_79_4: write-var gs#94792 <= s_79_3
        fn_state.gs_94792 = s_79_3;
        // N s_79_5: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #() : ()
        let s_80_0: () = ();
        // S s_80_1: call Halted(s_80_0)
        let s_80_1: bool = Halted(state, tracer, s_80_0);
        // N s_80_2: branch s_80_1 b141 b81
        if s_80_1 {
            return block_141(state, tracer, fn_state);
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
        // D s_81_1: write-var gs#94807 <= s_81_0
        fn_state.gs_94807 = s_81_0;
        // N s_81_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var gs#94807:u8
        let s_82_0: bool = fn_state.gs_94807;
        // N s_82_1: branch s_82_0 b140 b83
        if s_82_0 {
            return block_140(state, tracer, fn_state);
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
        // D s_83_1: write-var gs#94808 <= s_83_0
        fn_state.gs_94808 = s_83_0;
        // N s_83_2: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var gs#94808:u8
        let s_84_0: bool = fn_state.gs_94808;
        // N s_84_1: branch s_84_0 b139 b85
        if s_84_0 {
            return block_139(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #0u : u8
        let s_85_0: bool = false;
        // D s_85_1: write-var gs#94809 <= s_85_0
        fn_state.gs_94809 = s_85_0;
        // N s_85_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var gs#94809:u8
        let s_86_0: bool = fn_state.gs_94809;
        // N s_86_1: branch s_86_0 b138 b87
        if s_86_0 {
            return block_138(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #0u : u8
        let s_87_0: bool = false;
        // D s_87_1: write-var gs#94810 <= s_87_0
        fn_state.gs_94810 = s_87_0;
        // N s_87_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var gs#94810:u8
        let s_88_0: bool = fn_state.gs_94810;
        // N s_88_1: branch s_88_0 b137 b89
        if s_88_0 {
            return block_137(state, tracer, fn_state);
        } else {
            return block_89(state, tracer, fn_state);
        };
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_89_0: read-var __MDSCR_EL1_EnSPM:u8
        let s_89_0: bool = fn_state.u__MDSCR_EL1_EnSPM;
        // D s_89_1: cast zx s_89_0 -> bv
        let s_89_1: Bits = Bits::new(s_89_0 as u128, 1u16);
        // C s_89_2: const #0u : u8
        let s_89_2: bool = false;
        // C s_89_3: cast zx s_89_2 -> bv
        let s_89_3: Bits = Bits::new(s_89_2 as u128, 1u16);
        // D s_89_4: cmp-eq s_89_1 s_89_3
        let s_89_4: bool = ((s_89_1) == (s_89_3));
        // N s_89_5: branch s_89_4 b131 b90
        if s_89_4 {
            return block_131(state, tracer, fn_state);
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
        // N s_90_2: branch s_90_1 b130 b91
        if s_90_1 {
            return block_130(state, tracer, fn_state);
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
        // D s_91_1: write-var gs#94811 <= s_91_0
        fn_state.gs_94811 = s_91_0;
        // N s_91_2: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var gs#94811:u8
        let s_92_0: bool = fn_state.gs_94811;
        // N s_92_1: branch s_92_0 b129 b93
        if s_92_0 {
            return block_129(state, tracer, fn_state);
        } else {
            return block_93(state, tracer, fn_state);
        };
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #0u : u8
        let s_93_0: bool = false;
        // D s_93_1: write-var gs#94812 <= s_93_0
        fn_state.gs_94812 = s_93_0;
        // N s_93_2: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var gs#94812:u8
        let s_94_0: bool = fn_state.gs_94812;
        // N s_94_1: branch s_94_0 b128 b95
        if s_94_0 {
            return block_128(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #0u : u8
        let s_95_0: bool = false;
        // D s_95_1: write-var gs#94813 <= s_95_0
        fn_state.gs_94813 = s_95_0;
        // N s_95_2: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var gs#94813:u8
        let s_96_0: bool = fn_state.gs_94813;
        // N s_96_1: branch s_96_0 b127 b97
        if s_96_0 {
            return block_127(state, tracer, fn_state);
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
        // D s_97_1: write-var gs#94814 <= s_97_0
        fn_state.gs_94814 = s_97_0;
        // N s_97_2: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var gs#94814:u8
        let s_98_0: bool = fn_state.gs_94814;
        // N s_98_1: branch s_98_0 b126 b99
        if s_98_0 {
            return block_126(state, tracer, fn_state);
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
        // S s_99_1: call EL2Enabled(s_99_0)
        let s_99_1: bool = EL2Enabled(state, tracer, s_99_0);
        // N s_99_2: branch s_99_1 b125 b100
        if s_99_1 {
            return block_125(state, tracer, fn_state);
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
        // D s_100_1: write-var gs#94815 <= s_100_0
        fn_state.gs_94815 = s_100_0;
        // N s_100_2: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_101_0: read-var gs#94815:u8
        let s_101_0: bool = fn_state.gs_94815;
        // N s_101_1: branch s_101_0 b124 b102
        if s_101_0 {
            return block_124(state, tracer, fn_state);
        } else {
            return block_102(state, tracer, fn_state);
        };
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #0u : u8
        let s_102_0: bool = false;
        // D s_102_1: write-var gs#94816 <= s_102_0
        fn_state.gs_94816 = s_102_0;
        // N s_102_2: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_103_0: read-var gs#94816:u8
        let s_103_0: bool = fn_state.gs_94816;
        // N s_103_1: branch s_103_0 b123 b104
        if s_103_0 {
            return block_123(state, tracer, fn_state);
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
        // D s_104_1: write-var gs#94817 <= s_104_0
        fn_state.gs_94817 = s_104_0;
        // N s_104_2: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_105_0: read-var gs#94817:u8
        let s_105_0: bool = fn_state.gs_94817;
        // N s_105_1: branch s_105_0 b122 b106
        if s_105_0 {
            return block_122(state, tracer, fn_state);
        } else {
            return block_106(state, tracer, fn_state);
        };
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #() : ()
        let s_106_0: () = ();
        // S s_106_1: call EL2Enabled(s_106_0)
        let s_106_1: bool = EL2Enabled(state, tracer, s_106_0);
        // N s_106_2: branch s_106_1 b121 b107
        if s_106_1 {
            return block_121(state, tracer, fn_state);
        } else {
            return block_107(state, tracer, fn_state);
        };
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #0u : u8
        let s_107_0: bool = false;
        // D s_107_1: write-var gs#94818 <= s_107_0
        fn_state.gs_94818 = s_107_0;
        // N s_107_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var gs#94818:u8
        let s_108_0: bool = fn_state.gs_94818;
        // N s_108_1: branch s_108_0 b120 b109
        if s_108_0 {
            return block_120(state, tracer, fn_state);
        } else {
            return block_109(state, tracer, fn_state);
        };
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #424u : u32
        let s_109_0: u32 = 424;
        // D s_109_1: read-reg s_109_0:u8
        let s_109_1: u8 = {
            let value = state.read_register::<u8>(s_109_0 as isize);
            tracer.read_register(s_109_0 as isize, value);
            value
        };
        // C s_109_2: const #2u : u8
        let s_109_2: u8 = 2;
        // D s_109_3: cmp-lt s_109_1 s_109_2
        let s_109_3: bool = ((s_109_1) < (s_109_2));
        // N s_109_4: branch s_109_3 b119 b110
        if s_109_3 {
            return block_119(state, tracer, fn_state);
        } else {
            return block_110(state, tracer, fn_state);
        };
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_110_0: const #0u : u8
        let s_110_0: bool = false;
        // D s_110_1: write-var gs#94819 <= s_110_0
        fn_state.gs_94819 = s_110_0;
        // N s_110_2: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var gs#94819:u8
        let s_111_0: bool = fn_state.gs_94819;
        // N s_111_1: branch s_111_0 b113 b112
        if s_111_0 {
            return block_113(state, tracer, fn_state);
        } else {
            return block_112(state, tracer, fn_state);
        };
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_112_0: const #16552u : u32
        let s_112_0: u32 = 16552;
        // D s_112_1: read-reg s_112_0:struct
        let s_112_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_112_0 as isize);
            tracer.read_register(s_112_0 as isize, value);
            value
        };
        // D s_112_2: call _get_SPMSELR_EL0_Type_SYSPMUSEL(s_112_1)
        let s_112_2: u8 = u_get_SPMSELR_EL0_Type_SYSPMUSEL(state, tracer, s_112_1);
        // D s_112_3: cast zx s_112_2 -> bv
        let s_112_3: Bits = Bits::new(s_112_2 as u128, 6u16);
        // D s_112_4: cast zx s_112_3 -> i
        let s_112_4: i128 = (s_112_3.value() as i128);
        // D s_112_5: cast reint s_112_4 -> i64
        let s_112_5: i64 = (s_112_4 as i64);
        // C s_112_6: const #16552u : u32
        let s_112_6: u32 = 16552;
        // D s_112_7: read-reg s_112_6:struct
        let s_112_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_112_6 as isize);
            tracer.read_register(s_112_6 as isize, value);
            value
        };
        // D s_112_8: call _get_SPMSELR_EL0_Type_BANK(s_112_7)
        let s_112_8: u8 = u_get_SPMSELR_EL0_Type_BANK(state, tracer, s_112_7);
        // D s_112_9: cast zx s_112_8 -> bv
        let s_112_9: Bits = Bits::new(s_112_8 as u128, 2u16);
        // D s_112_10: cast zx s_112_9 -> i
        let s_112_10: i128 = (s_112_9.value() as i128);
        // D s_112_11: cast reint s_112_10 -> i64
        let s_112_11: i64 = (s_112_10 as i64);
        // C s_112_12: const #16s : i
        let s_112_12: i128 = 16;
        // D s_112_13: cast zx s_112_11 -> i
        let s_112_13: i128 = (i128::try_from(s_112_11).unwrap());
        // D s_112_14: mul s_112_13 s_112_12
        let s_112_14: i128 = ((s_112_13) * (s_112_12));
        // D s_112_15: cast reint s_112_14 -> i64
        let s_112_15: i64 = (s_112_14 as i64);
        // C s_112_16: const #2s : i
        let s_112_16: i128 = 2;
        // D s_112_17: cast zx s_112_15 -> i
        let s_112_17: i128 = (i128::try_from(s_112_15).unwrap());
        // D s_112_18: add s_112_17 s_112_16
        let s_112_18: i128 = (s_112_17 + s_112_16);
        // D s_112_19: cast reint s_112_18 -> i64
        let s_112_19: i64 = (s_112_18 as i64);
        // C s_112_20: const #64s : i64
        let s_112_20: i64 = 64;
        // D s_112_21: read-var t:i
        let s_112_21: i128 = fn_state.t;
        // D s_112_22: call X_read(s_112_21, s_112_20)
        let s_112_22: Bits = X_read(state, tracer, s_112_21, s_112_20);
        // D s_112_23: cast reint s_112_22 -> u64
        let s_112_23: u64 = (s_112_22.value() as u64);
        // D s_112_24: cast zx s_112_5 -> i
        let s_112_24: i128 = (i128::try_from(s_112_5).unwrap());
        // D s_112_25: cast zx s_112_19 -> i
        let s_112_25: i128 = (i128::try_from(s_112_19).unwrap());
        // D s_112_26: call SPMEVFILTR_EL0_set(s_112_24, s_112_25, s_112_23)
        let s_112_26: () = SPMEVFILTR_EL0_set(
            state,
            tracer,
            s_112_24,
            s_112_25,
            s_112_23,
        );
        // N s_112_27: return
        return;
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_113_0: const #() : ()
        let s_113_0: () = ();
        // S s_113_1: call Halted(s_113_0)
        let s_113_1: bool = Halted(state, tracer, s_113_0);
        // N s_113_2: branch s_113_1 b118 b114
        if s_113_1 {
            return block_118(state, tracer, fn_state);
        } else {
            return block_114(state, tracer, fn_state);
        };
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #0u : u8
        let s_114_0: bool = false;
        // D s_114_1: write-var gs#94823 <= s_114_0
        fn_state.gs_94823 = s_114_0;
        // N s_114_2: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_115_0: read-var gs#94823:u8
        let s_115_0: bool = fn_state.gs_94823;
        // N s_115_1: branch s_115_0 b117 b116
        if s_115_0 {
            return block_117(state, tracer, fn_state);
        } else {
            return block_116(state, tracer, fn_state);
        };
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #24u : u8
        let s_116_0: u8 = 24;
        // C s_116_1: cast zx s_116_0 -> bv
        let s_116_1: Bits = Bits::new(s_116_0 as u128, 8u16);
        // C s_116_2: cast zx s_116_1 -> i
        let s_116_2: i128 = (s_116_1.value() as i128);
        // C s_116_3: cast reint s_116_2 -> i64
        let s_116_3: i64 = (s_116_2 as i64);
        // C s_116_4: cast zx s_116_3 -> i
        let s_116_4: i128 = (i128::try_from(s_116_3).unwrap());
        // C s_116_5: const #424u : u32
        let s_116_5: u32 = 424;
        // D s_116_6: read-reg s_116_5:u8
        let s_116_6: u8 = {
            let value = state.read_register::<u8>(s_116_5 as isize);
            tracer.read_register(s_116_5 as isize, value);
            value
        };
        // D s_116_7: call AArch64_SystemAccessTrap(s_116_6, s_116_4)
        let s_116_7: () = AArch64_SystemAccessTrap(state, tracer, s_116_6, s_116_4);
        // N s_116_8: return
        return;
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_117_0: panic
        panic!("{:?}", ());
        // N s_117_1: return
        return;
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_118_0: read-var __EDSCR_SDD:u8
        let s_118_0: bool = fn_state.u__EDSCR_SDD;
        // D s_118_1: cast zx s_118_0 -> bv
        let s_118_1: Bits = Bits::new(s_118_0 as u128, 1u16);
        // C s_118_2: const #1u : u8
        let s_118_2: bool = true;
        // C s_118_3: cast zx s_118_2 -> bv
        let s_118_3: Bits = Bits::new(s_118_2 as u128, 1u16);
        // D s_118_4: cmp-eq s_118_1 s_118_3
        let s_118_4: bool = ((s_118_1) == (s_118_3));
        // D s_118_5: write-var gs#94823 <= s_118_4
        fn_state.gs_94823 = s_118_4;
        // N s_118_6: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_119_0: read-var __MDCR_EL3_EnPM2:u8
        let s_119_0: bool = fn_state.u__MDCR_EL3_EnPM2;
        // D s_119_1: cast zx s_119_0 -> bv
        let s_119_1: Bits = Bits::new(s_119_0 as u128, 1u16);
        // C s_119_2: const #0u : u8
        let s_119_2: bool = false;
        // C s_119_3: cast zx s_119_2 -> bv
        let s_119_3: Bits = Bits::new(s_119_2 as u128, 1u16);
        // D s_119_4: cmp-eq s_119_1 s_119_3
        let s_119_4: bool = ((s_119_1) == (s_119_3));
        // D s_119_5: write-var gs#94819 <= s_119_4
        fn_state.gs_94819 = s_119_4;
        // N s_119_6: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_120_0: const #24u : u8
        let s_120_0: u8 = 24;
        // C s_120_1: cast zx s_120_0 -> bv
        let s_120_1: Bits = Bits::new(s_120_0 as u128, 8u16);
        // C s_120_2: cast zx s_120_1 -> i
        let s_120_2: i128 = (s_120_1.value() as i128);
        // C s_120_3: cast reint s_120_2 -> i64
        let s_120_3: i64 = (s_120_2 as i64);
        // C s_120_4: cast zx s_120_3 -> i
        let s_120_4: i128 = (i128::try_from(s_120_3).unwrap());
        // C s_120_5: const #432u : u32
        let s_120_5: u32 = 432;
        // D s_120_6: read-reg s_120_5:u8
        let s_120_6: u8 = {
            let value = state.read_register::<u8>(s_120_5 as isize);
            tracer.read_register(s_120_5 as isize, value);
            value
        };
        // D s_120_7: call AArch64_SystemAccessTrap(s_120_6, s_120_4)
        let s_120_7: () = AArch64_SystemAccessTrap(state, tracer, s_120_6, s_120_4);
        // N s_120_8: return
        return;
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_121_0: read-var __MDCR_EL2_EnSPM:u8
        let s_121_0: bool = fn_state.u__MDCR_EL2_EnSPM;
        // D s_121_1: cast zx s_121_0 -> bv
        let s_121_1: Bits = Bits::new(s_121_0 as u128, 1u16);
        // C s_121_2: const #0u : u8
        let s_121_2: bool = false;
        // C s_121_3: cast zx s_121_2 -> bv
        let s_121_3: Bits = Bits::new(s_121_2 as u128, 1u16);
        // D s_121_4: cmp-eq s_121_1 s_121_3
        let s_121_4: bool = ((s_121_1) == (s_121_3));
        // D s_121_5: write-var gs#94818 <= s_121_4
        fn_state.gs_94818 = s_121_4;
        // N s_121_6: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_122_0: const #24u : u8
        let s_122_0: u8 = 24;
        // C s_122_1: cast zx s_122_0 -> bv
        let s_122_1: Bits = Bits::new(s_122_0 as u128, 8u16);
        // C s_122_2: cast zx s_122_1 -> i
        let s_122_2: i128 = (s_122_1.value() as i128);
        // C s_122_3: cast reint s_122_2 -> i64
        let s_122_3: i64 = (s_122_2 as i64);
        // C s_122_4: cast zx s_122_3 -> i
        let s_122_4: i128 = (i128::try_from(s_122_3).unwrap());
        // C s_122_5: const #432u : u32
        let s_122_5: u32 = 432;
        // D s_122_6: read-reg s_122_5:u8
        let s_122_6: u8 = {
            let value = state.read_register::<u8>(s_122_5 as isize);
            tracer.read_register(s_122_5 as isize, value);
            value
        };
        // D s_122_7: call AArch64_SystemAccessTrap(s_122_6, s_122_4)
        let s_122_7: () = AArch64_SystemAccessTrap(state, tracer, s_122_6, s_122_4);
        // N s_122_8: return
        return;
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_123_0: read-var __HDFGWTR2_EL2_nSPMEVTYPERn_EL0:u8
        let s_123_0: bool = fn_state.u__HDFGWTR2_EL2_nSPMEVTYPERn_EL0;
        // D s_123_1: cast zx s_123_0 -> bv
        let s_123_1: Bits = Bits::new(s_123_0 as u128, 1u16);
        // C s_123_2: const #0u : u8
        let s_123_2: bool = false;
        // C s_123_3: cast zx s_123_2 -> bv
        let s_123_3: Bits = Bits::new(s_123_2 as u128, 1u16);
        // D s_123_4: cmp-eq s_123_1 s_123_3
        let s_123_4: bool = ((s_123_1) == (s_123_3));
        // D s_123_5: write-var gs#94817 <= s_123_4
        fn_state.gs_94817 = s_123_4;
        // N s_123_6: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #188u : u32
        let s_124_0: u32 = 188;
        // S s_124_1: call IsFeatureImplemented(s_124_0)
        let s_124_1: bool = IsFeatureImplemented(state, tracer, s_124_0);
        // D s_124_2: write-var gs#94816 <= s_124_1
        fn_state.gs_94816 = s_124_1;
        // N s_124_3: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_125_0: const #102552u : u32
        let s_125_0: u32 = 102552;
        // D s_125_1: read-reg s_125_0:struct
        let s_125_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_125_0 as isize);
            tracer.read_register(s_125_0 as isize, value);
            value
        };
        // D s_125_2: call _get_HCR_EL2_Type_E2H(s_125_1)
        let s_125_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_125_1);
        // C s_125_3: const #102552u : u32
        let s_125_3: u32 = 102552;
        // D s_125_4: read-reg s_125_3:struct
        let s_125_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_125_3 as isize);
            tracer.read_register(s_125_3 as isize, value);
            value
        };
        // D s_125_5: call _get_HCR_EL2_Type_TGE(s_125_4)
        let s_125_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_125_4);
        // D s_125_6: cast zx s_125_2 -> bv
        let s_125_6: Bits = Bits::new(s_125_2 as u128, 1u16);
        // D s_125_7: cast zx s_125_5 -> bv
        let s_125_7: Bits = Bits::new(s_125_5 as u128, 1u16);
        // D s_125_8: cast reint s_125_6 -> u128
        let s_125_8: u128 = (s_125_6.value() as u128);
        // D s_125_9: size-of s_125_6
        let s_125_9: u16 = s_125_6.length();
        // D s_125_10: cast reint s_125_7 -> u128
        let s_125_10: u128 = (s_125_7.value() as u128);
        // D s_125_11: size-of s_125_7
        let s_125_11: u16 = s_125_7.length();
        // D s_125_12: lsl s_125_8 s_125_11
        let s_125_12: u128 = s_125_8 << s_125_11;
        // D s_125_13: or s_125_12 s_125_10
        let s_125_13: u128 = ((s_125_12) | (s_125_10));
        // D s_125_14: add s_125_9 s_125_11
        let s_125_14: u16 = (s_125_9 + s_125_11);
        // D s_125_15: create-bits s_125_13 s_125_14
        let s_125_15: Bits = Bits::new(s_125_13, s_125_14);
        // D s_125_16: cast reint s_125_15 -> u8
        let s_125_16: u8 = (s_125_15.value() as u8);
        // D s_125_17: cast zx s_125_16 -> bv
        let s_125_17: Bits = Bits::new(s_125_16 as u128, 2u16);
        // C s_125_18: const #3u : u8
        let s_125_18: u8 = 3;
        // C s_125_19: cast zx s_125_18 -> bv
        let s_125_19: Bits = Bits::new(s_125_18 as u128, 2u16);
        // D s_125_20: cmp-ne s_125_17 s_125_19
        let s_125_20: bool = ((s_125_17) != (s_125_19));
        // D s_125_21: write-var gs#94815 <= s_125_20
        fn_state.gs_94815 = s_125_20;
        // N s_125_22: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_126_0: const #24u : u8
        let s_126_0: u8 = 24;
        // C s_126_1: cast zx s_126_0 -> bv
        let s_126_1: Bits = Bits::new(s_126_0 as u128, 8u16);
        // C s_126_2: cast zx s_126_1 -> i
        let s_126_2: i128 = (s_126_1.value() as i128);
        // C s_126_3: cast reint s_126_2 -> i64
        let s_126_3: i64 = (s_126_2 as i64);
        // C s_126_4: cast zx s_126_3 -> i
        let s_126_4: i128 = (i128::try_from(s_126_3).unwrap());
        // C s_126_5: const #432u : u32
        let s_126_5: u32 = 432;
        // D s_126_6: read-reg s_126_5:u8
        let s_126_6: u8 = {
            let value = state.read_register::<u8>(s_126_5 as isize);
            tracer.read_register(s_126_5 as isize, value);
            value
        };
        // D s_126_7: call AArch64_SystemAccessTrap(s_126_6, s_126_4)
        let s_126_7: () = AArch64_SystemAccessTrap(state, tracer, s_126_6, s_126_4);
        // N s_126_8: return
        return;
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_127_0: read-var __SCR_EL3_FGTEn2:u8
        let s_127_0: bool = fn_state.u__SCR_EL3_FGTEn2;
        // D s_127_1: cast zx s_127_0 -> bv
        let s_127_1: Bits = Bits::new(s_127_0 as u128, 1u16);
        // C s_127_2: const #0u : u8
        let s_127_2: bool = false;
        // C s_127_3: cast zx s_127_2 -> bv
        let s_127_3: Bits = Bits::new(s_127_2 as u128, 1u16);
        // D s_127_4: cmp-eq s_127_1 s_127_3
        let s_127_4: bool = ((s_127_1) == (s_127_3));
        // D s_127_5: write-var gs#94814 <= s_127_4
        fn_state.gs_94814 = s_127_4;
        // N s_127_6: jump b98
        return block_98(state, tracer, fn_state);
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
        // C s_128_2: const #2u : u8
        let s_128_2: u8 = 2;
        // D s_128_3: cmp-lt s_128_1 s_128_2
        let s_128_3: bool = ((s_128_1) < (s_128_2));
        // D s_128_4: write-var gs#94813 <= s_128_3
        fn_state.gs_94813 = s_128_3;
        // N s_128_5: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_129_0: const #188u : u32
        let s_129_0: u32 = 188;
        // S s_129_1: call IsFeatureImplemented(s_129_0)
        let s_129_1: bool = IsFeatureImplemented(state, tracer, s_129_0);
        // D s_129_2: write-var gs#94812 <= s_129_1
        fn_state.gs_94812 = s_129_1;
        // N s_129_3: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_130_0: const #102552u : u32
        let s_130_0: u32 = 102552;
        // D s_130_1: read-reg s_130_0:struct
        let s_130_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_130_0 as isize);
            tracer.read_register(s_130_0 as isize, value);
            value
        };
        // D s_130_2: call _get_HCR_EL2_Type_E2H(s_130_1)
        let s_130_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_130_1);
        // C s_130_3: const #102552u : u32
        let s_130_3: u32 = 102552;
        // D s_130_4: read-reg s_130_3:struct
        let s_130_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_130_3 as isize);
            tracer.read_register(s_130_3 as isize, value);
            value
        };
        // D s_130_5: call _get_HCR_EL2_Type_TGE(s_130_4)
        let s_130_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_130_4);
        // D s_130_6: cast zx s_130_2 -> bv
        let s_130_6: Bits = Bits::new(s_130_2 as u128, 1u16);
        // D s_130_7: cast zx s_130_5 -> bv
        let s_130_7: Bits = Bits::new(s_130_5 as u128, 1u16);
        // D s_130_8: cast reint s_130_6 -> u128
        let s_130_8: u128 = (s_130_6.value() as u128);
        // D s_130_9: size-of s_130_6
        let s_130_9: u16 = s_130_6.length();
        // D s_130_10: cast reint s_130_7 -> u128
        let s_130_10: u128 = (s_130_7.value() as u128);
        // D s_130_11: size-of s_130_7
        let s_130_11: u16 = s_130_7.length();
        // D s_130_12: lsl s_130_8 s_130_11
        let s_130_12: u128 = s_130_8 << s_130_11;
        // D s_130_13: or s_130_12 s_130_10
        let s_130_13: u128 = ((s_130_12) | (s_130_10));
        // D s_130_14: add s_130_9 s_130_11
        let s_130_14: u16 = (s_130_9 + s_130_11);
        // D s_130_15: create-bits s_130_13 s_130_14
        let s_130_15: Bits = Bits::new(s_130_13, s_130_14);
        // D s_130_16: cast reint s_130_15 -> u8
        let s_130_16: u8 = (s_130_15.value() as u8);
        // D s_130_17: cast zx s_130_16 -> bv
        let s_130_17: Bits = Bits::new(s_130_16 as u128, 2u16);
        // C s_130_18: const #3u : u8
        let s_130_18: u8 = 3;
        // C s_130_19: cast zx s_130_18 -> bv
        let s_130_19: Bits = Bits::new(s_130_18 as u128, 2u16);
        // D s_130_20: cmp-ne s_130_17 s_130_19
        let s_130_20: bool = ((s_130_17) != (s_130_19));
        // D s_130_21: write-var gs#94811 <= s_130_20
        fn_state.gs_94811 = s_130_20;
        // N s_130_22: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_131_0: const #() : ()
        let s_131_0: () = ();
        // S s_131_1: call EL2Enabled(s_131_0)
        let s_131_1: bool = EL2Enabled(state, tracer, s_131_0);
        // N s_131_2: branch s_131_1 b136 b132
        if s_131_1 {
            return block_136(state, tracer, fn_state);
        } else {
            return block_132(state, tracer, fn_state);
        };
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_132_0: const #0u : u8
        let s_132_0: bool = false;
        // D s_132_1: write-var gs#94824 <= s_132_0
        fn_state.gs_94824 = s_132_0;
        // N s_132_2: jump b133
        return block_133(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_133_0: read-var gs#94824:u8
        let s_133_0: bool = fn_state.gs_94824;
        // N s_133_1: branch s_133_0 b135 b134
        if s_133_0 {
            return block_135(state, tracer, fn_state);
        } else {
            return block_134(state, tracer, fn_state);
        };
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_134_0: const #24u : u8
        let s_134_0: u8 = 24;
        // C s_134_1: cast zx s_134_0 -> bv
        let s_134_1: Bits = Bits::new(s_134_0 as u128, 8u16);
        // C s_134_2: cast zx s_134_1 -> i
        let s_134_2: i128 = (s_134_1.value() as i128);
        // C s_134_3: cast reint s_134_2 -> i64
        let s_134_3: i64 = (s_134_2 as i64);
        // C s_134_4: cast zx s_134_3 -> i
        let s_134_4: i128 = (i128::try_from(s_134_3).unwrap());
        // C s_134_5: const #440u : u32
        let s_134_5: u32 = 440;
        // D s_134_6: read-reg s_134_5:u8
        let s_134_6: u8 = {
            let value = state.read_register::<u8>(s_134_5 as isize);
            tracer.read_register(s_134_5 as isize, value);
            value
        };
        // D s_134_7: call AArch64_SystemAccessTrap(s_134_6, s_134_4)
        let s_134_7: () = AArch64_SystemAccessTrap(state, tracer, s_134_6, s_134_4);
        // N s_134_8: return
        return;
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_135_0: const #24u : u8
        let s_135_0: u8 = 24;
        // C s_135_1: cast zx s_135_0 -> bv
        let s_135_1: Bits = Bits::new(s_135_0 as u128, 8u16);
        // C s_135_2: cast zx s_135_1 -> i
        let s_135_2: i128 = (s_135_1.value() as i128);
        // C s_135_3: cast reint s_135_2 -> i64
        let s_135_3: i64 = (s_135_2 as i64);
        // C s_135_4: cast zx s_135_3 -> i
        let s_135_4: i128 = (i128::try_from(s_135_3).unwrap());
        // C s_135_5: const #432u : u32
        let s_135_5: u32 = 432;
        // D s_135_6: read-reg s_135_5:u8
        let s_135_6: u8 = {
            let value = state.read_register::<u8>(s_135_5 as isize);
            tracer.read_register(s_135_5 as isize, value);
            value
        };
        // D s_135_7: call AArch64_SystemAccessTrap(s_135_6, s_135_4)
        let s_135_7: () = AArch64_SystemAccessTrap(state, tracer, s_135_6, s_135_4);
        // N s_135_8: return
        return;
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_136_0: read-var __HCR_EL2_TGE:u8
        let s_136_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_136_1: cast zx s_136_0 -> bv
        let s_136_1: Bits = Bits::new(s_136_0 as u128, 1u16);
        // C s_136_2: const #1u : u8
        let s_136_2: bool = true;
        // C s_136_3: cast zx s_136_2 -> bv
        let s_136_3: Bits = Bits::new(s_136_2 as u128, 1u16);
        // D s_136_4: cmp-eq s_136_1 s_136_3
        let s_136_4: bool = ((s_136_1) == (s_136_3));
        // D s_136_5: write-var gs#94824 <= s_136_4
        fn_state.gs_94824 = s_136_4;
        // N s_136_6: jump b133
        return block_133(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_137_0: panic
        panic!("{:?}", ());
        // N s_137_1: return
        return;
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_138_0: read-var __MDCR_EL3_EnPM2:u8
        let s_138_0: bool = fn_state.u__MDCR_EL3_EnPM2;
        // D s_138_1: cast zx s_138_0 -> bv
        let s_138_1: Bits = Bits::new(s_138_0 as u128, 1u16);
        // C s_138_2: const #0u : u8
        let s_138_2: bool = false;
        // C s_138_3: cast zx s_138_2 -> bv
        let s_138_3: Bits = Bits::new(s_138_2 as u128, 1u16);
        // D s_138_4: cmp-eq s_138_1 s_138_3
        let s_138_4: bool = ((s_138_1) == (s_138_3));
        // D s_138_5: write-var gs#94810 <= s_138_4
        fn_state.gs_94810 = s_138_4;
        // N s_138_6: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_139_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_139_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_139_1: call __IMPDEF_boolean(s_139_0)
        let s_139_1: bool = u__IMPDEF_boolean(state, tracer, s_139_0);
        // D s_139_2: write-var gs#94809 <= s_139_1
        fn_state.gs_94809 = s_139_1;
        // N s_139_3: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_140_0: read-var __EDSCR_SDD:u8
        let s_140_0: bool = fn_state.u__EDSCR_SDD;
        // D s_140_1: cast zx s_140_0 -> bv
        let s_140_1: Bits = Bits::new(s_140_0 as u128, 1u16);
        // C s_140_2: const #1u : u8
        let s_140_2: bool = true;
        // C s_140_3: cast zx s_140_2 -> bv
        let s_140_3: Bits = Bits::new(s_140_2 as u128, 1u16);
        // D s_140_4: cmp-eq s_140_1 s_140_3
        let s_140_4: bool = ((s_140_1) == (s_140_3));
        // D s_140_5: write-var gs#94808 <= s_140_4
        fn_state.gs_94808 = s_140_4;
        // N s_140_6: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_141_0: const #424u : u32
        let s_141_0: u32 = 424;
        // D s_141_1: read-reg s_141_0:u8
        let s_141_1: u8 = {
            let value = state.read_register::<u8>(s_141_0 as isize);
            tracer.read_register(s_141_0 as isize, value);
            value
        };
        // C s_141_2: const #2u : u8
        let s_141_2: u8 = 2;
        // D s_141_3: cmp-lt s_141_1 s_141_2
        let s_141_3: bool = ((s_141_1) < (s_141_2));
        // D s_141_4: write-var gs#94807 <= s_141_3
        fn_state.gs_94807 = s_141_3;
        // N s_141_5: jump b82
        return block_82(state, tracer, fn_state);
    }
}
