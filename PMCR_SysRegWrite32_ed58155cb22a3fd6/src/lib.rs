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
use u_get_SCR_EL3_Type_FGTEn::*;
use u_get_HDCR_Type_TPMCR::*;
use Halted::*;
use u_get_HSTR_Type_T9::*;
use u_get_MDCR_EL2_Type_TPMCR::*;
use u_get_MDCR_EL3_Type_TPM::*;
use u__set_PMCR::*;
use R_read::*;
use u_get_PMUSERENR_Type_EN::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_HSTR_EL2_Type_T9::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use u_get_MDCR_EL2_Type_TPM::*;
use u_get_PMUSERENR_EL0_Type_UEN::*;
use u_get_HCR_EL2_Type_E2H::*;
use u_get_HDFGWTR_EL2_Type_PMCR_EL0::*;
use HCR_read::*;
use PMUSERENR_read::*;
use HDCR_read::*;
use IsFeatureImplemented::*;
use u__IMPDEF_boolean::*;
use AArch64_AArch32SystemAccessTrap::*;
use HSTR_read::*;
use u_get_HCR_Type_TGE::*;
use u_get_HDCR_Type_TPM::*;
use Mk_PMCR_Type::*;
use ELUsingAArch32::*;
use u_get_PMUSERENR_EL0_Type_EN::*;
use EDSCR_read::*;
use AArch32_TakeHypTrapException::*;
use common::*;
pub fn PMCR_SysRegWrite32_ed58155cb22a3fd6<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    coproc: u8,
    opc1: u8,
    CRn: u8,
    opc2: u8,
    CRm: u8,
    t: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        u__HSTR_EL2_T9: bool,
        gs_129855: bool,
        gs_129843: bool,
        gs_129881: bool,
        gs_129854: bool,
        gs_129846: bool,
        u__HDFGWTR_EL2_PMCR_EL0: bool,
        gs_129847: bool,
        u__MDCR_EL2_TPMCR: bool,
        gs_129844: bool,
        gs_129886: bool,
        gs_129897: bool,
        gs_129853: bool,
        gs_129900: bool,
        gs_129859: bool,
        u__MDCR_EL3_TPM: bool,
        gs_129892: bool,
        gs_129874: bool,
        gs_129902: bool,
        gs_129872: bool,
        gs_129895: bool,
        gs_129880: bool,
        u__HCR_TGE: bool,
        gs_129871: bool,
        gs_129869: bool,
        gs_129883: bool,
        gs_129865: bool,
        gs_129899: bool,
        gs_129888: bool,
        gs_129861: bool,
        gs_129845: bool,
        gs_129858: bool,
        u__HDCR_TPMCR: bool,
        gs_129884: bool,
        gs_129856: bool,
        gs_129850: bool,
        gs_129862: bool,
        gs_129903: bool,
        u__PSTATE_EL: u8,
        gs_129852: bool,
        gs_129893: bool,
        gs_129904: bool,
        u__MDCR_EL2_TPM: bool,
        gs_129889: bool,
        u__PMUSERENR_EN: bool,
        gs_129877: bool,
        gs_129851: bool,
        gs_129842: bool,
        gs_129885: bool,
        u__HCR_EL2_TGE: bool,
        gs_129840: bool,
        gs_129891: bool,
        gs_129887: bool,
        gs_129882: bool,
        u__SCR_EL3_FGTEn: bool,
        u__PMUSERENR_EL0_EN: bool,
        u__HSTR_T9: bool,
        gs_129867: bool,
        gs_129879: bool,
        gs_129841: bool,
        gs_129868: bool,
        u__HDCR_TPM: bool,
        gs_129901: bool,
        gs_129863: bool,
        gs_129873: bool,
        gs_129857: bool,
        gs_129875: bool,
        gs_129896: bool,
        gs_129848: bool,
        gs_129898: bool,
        gs_129864: bool,
        gs_129894: bool,
        gs_129905: bool,
        gs_129849: bool,
        gs_129866: bool,
        gs_129860: bool,
        gs_129870: bool,
        gs_129890: bool,
        gs_129876: bool,
        gs_129878: bool,
        el: u8,
        coproc: u8,
        opc1: u8,
        CRn: u8,
        opc2: u8,
        CRm: u8,
        t: i128,
    }
    let fn_state = FunctionState {
        el,
        coproc,
        opc1,
        CRn,
        opc2,
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
        // D s_0_5: call _get_MDCR_EL3_Type_TPM(s_0_4)
        let s_0_5: bool = u_get_MDCR_EL3_Type_TPM(state, tracer, s_0_4);
        // D s_0_6: write-var __MDCR_EL3_TPM <= s_0_5
        fn_state.u__MDCR_EL3_TPM = s_0_5;
        // C s_0_7: const #102624u : u32
        let s_0_7: u32 = 102624;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_PMUSERENR_EL0_Type_EN(s_0_8)
        let s_0_9: bool = u_get_PMUSERENR_EL0_Type_EN(state, tracer, s_0_8);
        // D s_0_10: write-var __PMUSERENR_EL0_EN <= s_0_9
        fn_state.u__PMUSERENR_EL0_EN = s_0_9;
        // C s_0_11: const #102552u : u32
        let s_0_11: u32 = 102552;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_HCR_EL2_Type_TGE(s_0_12)
        let s_0_13: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_0_12);
        // D s_0_14: write-var __HCR_EL2_TGE <= s_0_13
        fn_state.u__HCR_EL2_TGE = s_0_13;
        // C s_0_15: const #() : ()
        let s_0_15: () = ();
        // S s_0_16: call PMUSERENR_read(s_0_15)
        let s_0_16: ProductType700c18a878c5601b = PMUSERENR_read(state, tracer, s_0_15);
        // S s_0_17: call _get_PMUSERENR_Type_EN(s_0_16)
        let s_0_17: bool = u_get_PMUSERENR_Type_EN(state, tracer, s_0_16);
        // D s_0_18: write-var __PMUSERENR_EN <= s_0_17
        fn_state.u__PMUSERENR_EN = s_0_17;
        // C s_0_19: const #() : ()
        let s_0_19: () = ();
        // S s_0_20: call HCR_read(s_0_19)
        let s_0_20: ProductType700c18a878c5601b = HCR_read(state, tracer, s_0_19);
        // S s_0_21: call _get_HCR_Type_TGE(s_0_20)
        let s_0_21: bool = u_get_HCR_Type_TGE(state, tracer, s_0_20);
        // D s_0_22: write-var __HCR_TGE <= s_0_21
        fn_state.u__HCR_TGE = s_0_21;
        // C s_0_23: const #104936u : u32
        let s_0_23: u32 = 104936;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_HSTR_EL2_Type_T9(s_0_24)
        let s_0_25: bool = u_get_HSTR_EL2_Type_T9(state, tracer, s_0_24);
        // D s_0_26: write-var __HSTR_EL2_T9 <= s_0_25
        fn_state.u__HSTR_EL2_T9 = s_0_25;
        // C s_0_27: const #() : ()
        let s_0_27: () = ();
        // S s_0_28: call HSTR_read(s_0_27)
        let s_0_28: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_27);
        // S s_0_29: call _get_HSTR_Type_T9(s_0_28)
        let s_0_29: bool = u_get_HSTR_Type_T9(state, tracer, s_0_28);
        // D s_0_30: write-var __HSTR_T9 <= s_0_29
        fn_state.u__HSTR_T9 = s_0_29;
        // C s_0_31: const #90704u : u32
        let s_0_31: u32 = 90704;
        // D s_0_32: read-reg s_0_31:struct
        let s_0_32: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_31 as isize);
            tracer.read_register(s_0_31 as isize, value);
            value
        };
        // D s_0_33: call _get_SCR_EL3_Type_FGTEn(s_0_32)
        let s_0_33: bool = u_get_SCR_EL3_Type_FGTEn(state, tracer, s_0_32);
        // D s_0_34: write-var __SCR_EL3_FGTEn <= s_0_33
        fn_state.u__SCR_EL3_FGTEn = s_0_33;
        // C s_0_35: const #17360u : u32
        let s_0_35: u32 = 17360;
        // D s_0_36: read-reg s_0_35:struct
        let s_0_36: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_35 as isize);
            tracer.read_register(s_0_35 as isize, value);
            value
        };
        // D s_0_37: call _get_HDFGWTR_EL2_Type_PMCR_EL0(s_0_36)
        let s_0_37: bool = u_get_HDFGWTR_EL2_Type_PMCR_EL0(state, tracer, s_0_36);
        // D s_0_38: write-var __HDFGWTR_EL2_PMCR_EL0 <= s_0_37
        fn_state.u__HDFGWTR_EL2_PMCR_EL0 = s_0_37;
        // C s_0_39: const #104880u : u32
        let s_0_39: u32 = 104880;
        // D s_0_40: read-reg s_0_39:struct
        let s_0_40: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_39 as isize);
            tracer.read_register(s_0_39 as isize, value);
            value
        };
        // D s_0_41: call _get_MDCR_EL2_Type_TPM(s_0_40)
        let s_0_41: bool = u_get_MDCR_EL2_Type_TPM(state, tracer, s_0_40);
        // D s_0_42: write-var __MDCR_EL2_TPM <= s_0_41
        fn_state.u__MDCR_EL2_TPM = s_0_41;
        // C s_0_43: const #104880u : u32
        let s_0_43: u32 = 104880;
        // D s_0_44: read-reg s_0_43:struct
        let s_0_44: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_43 as isize);
            tracer.read_register(s_0_43 as isize, value);
            value
        };
        // D s_0_45: call _get_MDCR_EL2_Type_TPMCR(s_0_44)
        let s_0_45: bool = u_get_MDCR_EL2_Type_TPMCR(state, tracer, s_0_44);
        // D s_0_46: write-var __MDCR_EL2_TPMCR <= s_0_45
        fn_state.u__MDCR_EL2_TPMCR = s_0_45;
        // C s_0_47: const #() : ()
        let s_0_47: () = ();
        // S s_0_48: call HDCR_read(s_0_47)
        let s_0_48: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_0_47);
        // S s_0_49: call _get_HDCR_Type_TPM(s_0_48)
        let s_0_49: bool = u_get_HDCR_Type_TPM(state, tracer, s_0_48);
        // D s_0_50: write-var __HDCR_TPM <= s_0_49
        fn_state.u__HDCR_TPM = s_0_49;
        // C s_0_51: const #() : ()
        let s_0_51: () = ();
        // S s_0_52: call HDCR_read(s_0_51)
        let s_0_52: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_0_51);
        // S s_0_53: call _get_HDCR_Type_TPMCR(s_0_52)
        let s_0_53: bool = u_get_HDCR_Type_TPMCR(state, tracer, s_0_52);
        // D s_0_54: write-var __HDCR_TPMCR <= s_0_53
        fn_state.u__HDCR_TPMCR = s_0_53;
        // D s_0_55: read-var __PSTATE_EL:u8
        let s_0_55: u8 = fn_state.u__PSTATE_EL;
        // D s_0_56: cast zx s_0_55 -> bv
        let s_0_56: Bits = Bits::new(s_0_55 as u128, 2u16);
        // C s_0_57: const #448u : u32
        let s_0_57: u32 = 448;
        // D s_0_58: read-reg s_0_57:u8
        let s_0_58: u8 = {
            let value = state.read_register::<u8>(s_0_57 as isize);
            tracer.read_register(s_0_57 as isize, value);
            value
        };
        // D s_0_59: cast zx s_0_58 -> bv
        let s_0_59: Bits = Bits::new(s_0_58 as u128, 2u16);
        // D s_0_60: cmp-eq s_0_56 s_0_59
        let s_0_60: bool = ((s_0_56) == (s_0_59));
        // N s_0_61: branch s_0_60 b116 b1
        if s_0_60 {
            return block_116(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b37 b2
        if s_1_5 {
            return block_37(state, tracer, fn_state);
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
        // D s_5_0: read-var t:i
        let s_5_0: i128 = fn_state.t;
        // D s_5_1: call R_read(s_5_0)
        let s_5_1: u32 = R_read(state, tracer, s_5_0);
        // D s_5_2: call Mk_PMCR_Type(s_5_1)
        let s_5_2: ProductType700c18a878c5601b = Mk_PMCR_Type(state, tracer, s_5_1);
        // D s_5_3: call __set_PMCR(s_5_2)
        let s_5_3: () = u__set_PMCR(state, tracer, s_5_2);
        // N s_5_4: return
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
        // N s_6_2: branch s_6_1 b36 b7
        if s_6_1 {
            return block_36(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#129840 <= s_7_0
        fn_state.gs_129840 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#129840:u8
        let s_8_0: bool = fn_state.gs_129840;
        // N s_8_1: branch s_8_0 b35 b9
        if s_8_0 {
            return block_35(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#129841 <= s_9_0
        fn_state.gs_129841 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#129841:u8
        let s_10_0: bool = fn_state.gs_129841;
        // N s_10_1: branch s_10_0 b34 b11
        if s_10_0 {
            return block_34(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#129842 <= s_11_0
        fn_state.gs_129842 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#129842:u8
        let s_12_0: bool = fn_state.gs_129842;
        // N s_12_1: branch s_12_0 b33 b13
        if s_12_0 {
            return block_33(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#129843 <= s_13_0
        fn_state.gs_129843 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#129843:u8
        let s_14_0: bool = fn_state.gs_129843;
        // N s_14_1: branch s_14_0 b32 b15
        if s_14_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#129844 <= s_15_0
        fn_state.gs_129844 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#129844:u8
        let s_16_0: bool = fn_state.gs_129844;
        // N s_16_1: branch s_16_0 b31 b17
        if s_16_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #424u : u32
        let s_17_0: u32 = 424;
        // D s_17_1: read-reg s_17_0:u8
        let s_17_1: u8 = {
            let value = state.read_register::<u8>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // C s_17_2: const #2u : u8
        let s_17_2: u8 = 2;
        // D s_17_3: cmp-lt s_17_1 s_17_2
        let s_17_3: bool = ((s_17_1) < (s_17_2));
        // N s_17_4: branch s_17_3 b30 b18
        if s_17_3 {
            return block_30(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#129845 <= s_18_0
        fn_state.gs_129845 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#129845:u8
        let s_19_0: bool = fn_state.gs_129845;
        // N s_19_1: branch s_19_0 b29 b20
        if s_19_0 {
            return block_29(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#129846 <= s_20_0
        fn_state.gs_129846 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#129846:u8
        let s_21_0: bool = fn_state.gs_129846;
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
        // D s_22_0: read-var t:i
        let s_22_0: i128 = fn_state.t;
        // D s_22_1: call R_read(s_22_0)
        let s_22_1: u32 = R_read(state, tracer, s_22_0);
        // D s_22_2: call Mk_PMCR_Type(s_22_1)
        let s_22_2: ProductType700c18a878c5601b = Mk_PMCR_Type(state, tracer, s_22_1);
        // D s_22_3: call __set_PMCR(s_22_2)
        let s_22_3: () = u__set_PMCR(state, tracer, s_22_2);
        // N s_22_4: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #() : ()
        let s_23_0: () = ();
        // S s_23_1: call Halted(s_23_0)
        let s_23_1: bool = Halted(state, tracer, s_23_0);
        // N s_23_2: branch s_23_1 b28 b24
        if s_23_1 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #0u : u8
        let s_24_0: bool = false;
        // D s_24_1: write-var gs#129847 <= s_24_0
        fn_state.gs_129847 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#129847:u8
        let s_25_0: bool = fn_state.gs_129847;
        // N s_25_1: branch s_25_0 b27 b26
        if s_25_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #3u : u8
        let s_26_0: u8 = 3;
        // C s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 8u16);
        // C s_26_2: cast zx s_26_1 -> i
        let s_26_2: i128 = (s_26_1.value() as i128);
        // C s_26_3: cast reint s_26_2 -> i64
        let s_26_3: i64 = (s_26_2 as i64);
        // C s_26_4: cast zx s_26_3 -> i
        let s_26_4: i128 = (i128::try_from(s_26_3).unwrap());
        // C s_26_5: const #424u : u32
        let s_26_5: u32 = 424;
        // D s_26_6: read-reg s_26_5:u8
        let s_26_6: u8 = {
            let value = state.read_register::<u8>(s_26_5 as isize);
            tracer.read_register(s_26_5 as isize, value);
            value
        };
        // D s_26_7: call AArch64_AArch32SystemAccessTrap(s_26_6, s_26_4)
        let s_26_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_26_6, s_26_4);
        // N s_26_8: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_27_0: panic
        panic!("{:?}", ());
        // N s_27_1: return
        return;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call EDSCR_read(s_28_0)
        let s_28_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_28_0);
        // S s_28_2: call _get_EDSCR_Type_SDD(s_28_1)
        let s_28_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_28_1);
        // S s_28_3: cast zx s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 1u16);
        // C s_28_4: const #1u : u8
        let s_28_4: bool = true;
        // C s_28_5: cast zx s_28_4 -> bv
        let s_28_5: Bits = Bits::new(s_28_4 as u128, 1u16);
        // S s_28_6: cmp-eq s_28_3 s_28_5
        let s_28_6: bool = ((s_28_3) == (s_28_5));
        // D s_28_7: write-var gs#129847 <= s_28_6
        fn_state.gs_129847 = s_28_6;
        // N s_28_8: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var __MDCR_EL3_TPM:u8
        let s_29_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 1u16);
        // C s_29_2: const #1u : u8
        let s_29_2: bool = true;
        // C s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // D s_29_4: cmp-eq s_29_1 s_29_3
        let s_29_4: bool = ((s_29_1) == (s_29_3));
        // D s_29_5: write-var gs#129846 <= s_29_4
        fn_state.gs_129846 = s_29_4;
        // N s_29_6: jump b21
        return block_21(state, tracer, fn_state);
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
        // D s_30_2: call ELUsingAArch32(s_30_1)
        let s_30_2: bool = ELUsingAArch32(state, tracer, s_30_1);
        // D s_30_3: not s_30_2
        let s_30_3: bool = !s_30_2;
        // D s_30_4: write-var gs#129845 <= s_30_3
        fn_state.gs_129845 = s_30_3;
        // N s_30_5: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_31_0: panic
        panic!("{:?}", ());
        // N s_31_1: return
        return;
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var __MDCR_EL3_TPM:u8
        let s_32_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 1u16);
        // C s_32_2: const #1u : u8
        let s_32_2: bool = true;
        // C s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 1u16);
        // D s_32_4: cmp-eq s_32_1 s_32_3
        let s_32_4: bool = ((s_32_1) == (s_32_3));
        // D s_32_5: write-var gs#129844 <= s_32_4
        fn_state.gs_129844 = s_32_4;
        // N s_32_6: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #424u : u32
        let s_33_0: u32 = 424;
        // D s_33_1: read-reg s_33_0:u8
        let s_33_1: u8 = {
            let value = state.read_register::<u8>(s_33_0 as isize);
            tracer.read_register(s_33_0 as isize, value);
            value
        };
        // D s_33_2: call ELUsingAArch32(s_33_1)
        let s_33_2: bool = ELUsingAArch32(state, tracer, s_33_1);
        // D s_33_3: not s_33_2
        let s_33_3: bool = !s_33_2;
        // D s_33_4: write-var gs#129843 <= s_33_3
        fn_state.gs_129843 = s_33_3;
        // N s_33_5: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_34_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_34_1: call __IMPDEF_boolean(s_34_0)
        let s_34_1: bool = u__IMPDEF_boolean(state, tracer, s_34_0);
        // D s_34_2: write-var gs#129842 <= s_34_1
        fn_state.gs_129842 = s_34_1;
        // N s_34_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #() : ()
        let s_35_0: () = ();
        // S s_35_1: call EDSCR_read(s_35_0)
        let s_35_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_35_0);
        // S s_35_2: call _get_EDSCR_Type_SDD(s_35_1)
        let s_35_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_35_1);
        // S s_35_3: cast zx s_35_2 -> bv
        let s_35_3: Bits = Bits::new(s_35_2 as u128, 1u16);
        // C s_35_4: const #1u : u8
        let s_35_4: bool = true;
        // C s_35_5: cast zx s_35_4 -> bv
        let s_35_5: Bits = Bits::new(s_35_4 as u128, 1u16);
        // S s_35_6: cmp-eq s_35_3 s_35_5
        let s_35_6: bool = ((s_35_3) == (s_35_5));
        // D s_35_7: write-var gs#129841 <= s_35_6
        fn_state.gs_129841 = s_35_6;
        // N s_35_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #424u : u32
        let s_36_0: u32 = 424;
        // D s_36_1: read-reg s_36_0:u8
        let s_36_1: u8 = {
            let value = state.read_register::<u8>(s_36_0 as isize);
            tracer.read_register(s_36_0 as isize, value);
            value
        };
        // C s_36_2: const #2u : u8
        let s_36_2: u8 = 2;
        // D s_36_3: cmp-lt s_36_1 s_36_2
        let s_36_3: bool = ((s_36_1) < (s_36_2));
        // D s_36_4: write-var gs#129840 <= s_36_3
        fn_state.gs_129840 = s_36_3;
        // N s_36_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #() : ()
        let s_37_0: () = ();
        // S s_37_1: call Halted(s_37_0)
        let s_37_1: bool = Halted(state, tracer, s_37_0);
        // N s_37_2: branch s_37_1 b115 b38
        if s_37_1 {
            return block_115(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#129848 <= s_38_0
        fn_state.gs_129848 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#129848:u8
        let s_39_0: bool = fn_state.gs_129848;
        // N s_39_1: branch s_39_0 b114 b40
        if s_39_0 {
            return block_114(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #0u : u8
        let s_40_0: bool = false;
        // D s_40_1: write-var gs#129849 <= s_40_0
        fn_state.gs_129849 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#129849:u8
        let s_41_0: bool = fn_state.gs_129849;
        // N s_41_1: branch s_41_0 b113 b42
        if s_41_0 {
            return block_113(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #0u : u8
        let s_42_0: bool = false;
        // D s_42_1: write-var gs#129850 <= s_42_0
        fn_state.gs_129850 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#129850:u8
        let s_43_0: bool = fn_state.gs_129850;
        // N s_43_1: branch s_43_0 b112 b44
        if s_43_0 {
            return block_112(state, tracer, fn_state);
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
        // D s_44_1: write-var gs#129851 <= s_44_0
        fn_state.gs_129851 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#129851:u8
        let s_45_0: bool = fn_state.gs_129851;
        // N s_45_1: branch s_45_0 b111 b46
        if s_45_0 {
            return block_111(state, tracer, fn_state);
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
        // D s_46_1: write-var gs#129852 <= s_46_0
        fn_state.gs_129852 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#129852:u8
        let s_47_0: bool = fn_state.gs_129852;
        // N s_47_1: branch s_47_0 b110 b48
        if s_47_0 {
            return block_110(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #() : ()
        let s_48_0: () = ();
        // S s_48_1: call EL2Enabled(s_48_0)
        let s_48_1: bool = EL2Enabled(state, tracer, s_48_0);
        // N s_48_2: branch s_48_1 b109 b49
        if s_48_1 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #0u : u8
        let s_49_0: bool = false;
        // D s_49_1: write-var gs#129853 <= s_49_0
        fn_state.gs_129853 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#129853:u8
        let s_50_0: bool = fn_state.gs_129853;
        // N s_50_1: branch s_50_0 b108 b51
        if s_50_0 {
            return block_108(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #0u : u8
        let s_51_0: bool = false;
        // D s_51_1: write-var gs#129854 <= s_51_0
        fn_state.gs_129854 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#129854:u8
        let s_52_0: bool = fn_state.gs_129854;
        // N s_52_1: branch s_52_0 b107 b53
        if s_52_0 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #() : ()
        let s_53_0: () = ();
        // S s_53_1: call EL2Enabled(s_53_0)
        let s_53_1: bool = EL2Enabled(state, tracer, s_53_0);
        // N s_53_2: branch s_53_1 b106 b54
        if s_53_1 {
            return block_106(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #0u : u8
        let s_54_0: bool = false;
        // D s_54_1: write-var gs#129855 <= s_54_0
        fn_state.gs_129855 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#129855:u8
        let s_55_0: bool = fn_state.gs_129855;
        // N s_55_1: branch s_55_0 b105 b56
        if s_55_0 {
            return block_105(state, tracer, fn_state);
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
        // D s_56_1: write-var gs#129856 <= s_56_0
        fn_state.gs_129856 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#129856:u8
        let s_57_0: bool = fn_state.gs_129856;
        // N s_57_1: branch s_57_0 b104 b58
        if s_57_0 {
            return block_104(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #() : ()
        let s_58_0: () = ();
        // S s_58_1: call EL2Enabled(s_58_0)
        let s_58_1: bool = EL2Enabled(state, tracer, s_58_0);
        // N s_58_2: branch s_58_1 b103 b59
        if s_58_1 {
            return block_103(state, tracer, fn_state);
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
        // D s_59_1: write-var gs#129857 <= s_59_0
        fn_state.gs_129857 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#129857:u8
        let s_60_0: bool = fn_state.gs_129857;
        // N s_60_1: branch s_60_0 b102 b61
        if s_60_0 {
            return block_102(state, tracer, fn_state);
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
        // D s_61_1: write-var gs#129858 <= s_61_0
        fn_state.gs_129858 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#129858:u8
        let s_62_0: bool = fn_state.gs_129858;
        // N s_62_1: branch s_62_0 b101 b63
        if s_62_0 {
            return block_101(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #() : ()
        let s_63_0: () = ();
        // S s_63_1: call EL2Enabled(s_63_0)
        let s_63_1: bool = EL2Enabled(state, tracer, s_63_0);
        // N s_63_2: branch s_63_1 b100 b64
        if s_63_1 {
            return block_100(state, tracer, fn_state);
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
        // D s_64_1: write-var gs#129859 <= s_64_0
        fn_state.gs_129859 = s_64_0;
        // N s_64_2: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var gs#129859:u8
        let s_65_0: bool = fn_state.gs_129859;
        // N s_65_1: branch s_65_0 b99 b66
        if s_65_0 {
            return block_99(state, tracer, fn_state);
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
        // D s_66_1: write-var gs#129860 <= s_66_0
        fn_state.gs_129860 = s_66_0;
        // N s_66_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#129860:u8
        let s_67_0: bool = fn_state.gs_129860;
        // N s_67_1: branch s_67_0 b98 b68
        if s_67_0 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #() : ()
        let s_68_0: () = ();
        // S s_68_1: call EL2Enabled(s_68_0)
        let s_68_1: bool = EL2Enabled(state, tracer, s_68_0);
        // N s_68_2: branch s_68_1 b97 b69
        if s_68_1 {
            return block_97(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #0u : u8
        let s_69_0: bool = false;
        // D s_69_1: write-var gs#129861 <= s_69_0
        fn_state.gs_129861 = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var gs#129861:u8
        let s_70_0: bool = fn_state.gs_129861;
        // N s_70_1: branch s_70_0 b96 b71
        if s_70_0 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #0u : u8
        let s_71_0: bool = false;
        // D s_71_1: write-var gs#129862 <= s_71_0
        fn_state.gs_129862 = s_71_0;
        // N s_71_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var gs#129862:u8
        let s_72_0: bool = fn_state.gs_129862;
        // N s_72_1: branch s_72_0 b95 b73
        if s_72_0 {
            return block_95(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #() : ()
        let s_73_0: () = ();
        // S s_73_1: call EL2Enabled(s_73_0)
        let s_73_1: bool = EL2Enabled(state, tracer, s_73_0);
        // N s_73_2: branch s_73_1 b94 b74
        if s_73_1 {
            return block_94(state, tracer, fn_state);
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
        // D s_74_1: write-var gs#129863 <= s_74_0
        fn_state.gs_129863 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var gs#129863:u8
        let s_75_0: bool = fn_state.gs_129863;
        // N s_75_1: branch s_75_0 b93 b76
        if s_75_0 {
            return block_93(state, tracer, fn_state);
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
        // D s_76_1: write-var gs#129864 <= s_76_0
        fn_state.gs_129864 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var gs#129864:u8
        let s_77_0: bool = fn_state.gs_129864;
        // N s_77_1: branch s_77_0 b92 b78
        if s_77_0 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_78(state, tracer, fn_state);
        };
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #424u : u32
        let s_78_0: u32 = 424;
        // D s_78_1: read-reg s_78_0:u8
        let s_78_1: u8 = {
            let value = state.read_register::<u8>(s_78_0 as isize);
            tracer.read_register(s_78_0 as isize, value);
            value
        };
        // C s_78_2: const #2u : u8
        let s_78_2: u8 = 2;
        // D s_78_3: cmp-lt s_78_1 s_78_2
        let s_78_3: bool = ((s_78_1) < (s_78_2));
        // N s_78_4: branch s_78_3 b91 b79
        if s_78_3 {
            return block_91(state, tracer, fn_state);
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
        // D s_79_1: write-var gs#129865 <= s_79_0
        fn_state.gs_129865 = s_79_0;
        // N s_79_2: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var gs#129865:u8
        let s_80_0: bool = fn_state.gs_129865;
        // N s_80_1: branch s_80_0 b90 b81
        if s_80_0 {
            return block_90(state, tracer, fn_state);
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
        // D s_81_1: write-var gs#129866 <= s_81_0
        fn_state.gs_129866 = s_81_0;
        // N s_81_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var gs#129866:u8
        let s_82_0: bool = fn_state.gs_129866;
        // N s_82_1: branch s_82_0 b84 b83
        if s_82_0 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_83_0: read-var t:i
        let s_83_0: i128 = fn_state.t;
        // D s_83_1: call R_read(s_83_0)
        let s_83_1: u32 = R_read(state, tracer, s_83_0);
        // D s_83_2: call Mk_PMCR_Type(s_83_1)
        let s_83_2: ProductType700c18a878c5601b = Mk_PMCR_Type(state, tracer, s_83_1);
        // D s_83_3: call __set_PMCR(s_83_2)
        let s_83_3: () = u__set_PMCR(state, tracer, s_83_2);
        // N s_83_4: return
        return;
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #() : ()
        let s_84_0: () = ();
        // S s_84_1: call Halted(s_84_0)
        let s_84_1: bool = Halted(state, tracer, s_84_0);
        // N s_84_2: branch s_84_1 b89 b85
        if s_84_1 {
            return block_89(state, tracer, fn_state);
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
        // D s_85_1: write-var gs#129867 <= s_85_0
        fn_state.gs_129867 = s_85_0;
        // N s_85_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var gs#129867:u8
        let s_86_0: bool = fn_state.gs_129867;
        // N s_86_1: branch s_86_0 b88 b87
        if s_86_0 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #3u : u8
        let s_87_0: u8 = 3;
        // C s_87_1: cast zx s_87_0 -> bv
        let s_87_1: Bits = Bits::new(s_87_0 as u128, 8u16);
        // C s_87_2: cast zx s_87_1 -> i
        let s_87_2: i128 = (s_87_1.value() as i128);
        // C s_87_3: cast reint s_87_2 -> i64
        let s_87_3: i64 = (s_87_2 as i64);
        // C s_87_4: cast zx s_87_3 -> i
        let s_87_4: i128 = (i128::try_from(s_87_3).unwrap());
        // C s_87_5: const #424u : u32
        let s_87_5: u32 = 424;
        // D s_87_6: read-reg s_87_5:u8
        let s_87_6: u8 = {
            let value = state.read_register::<u8>(s_87_5 as isize);
            tracer.read_register(s_87_5 as isize, value);
            value
        };
        // D s_87_7: call AArch64_AArch32SystemAccessTrap(s_87_6, s_87_4)
        let s_87_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_87_6, s_87_4);
        // N s_87_8: return
        return;
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_88_0: panic
        panic!("{:?}", ());
        // N s_88_1: return
        return;
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #() : ()
        let s_89_0: () = ();
        // S s_89_1: call EDSCR_read(s_89_0)
        let s_89_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_89_0);
        // S s_89_2: call _get_EDSCR_Type_SDD(s_89_1)
        let s_89_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_89_1);
        // S s_89_3: cast zx s_89_2 -> bv
        let s_89_3: Bits = Bits::new(s_89_2 as u128, 1u16);
        // C s_89_4: const #1u : u8
        let s_89_4: bool = true;
        // C s_89_5: cast zx s_89_4 -> bv
        let s_89_5: Bits = Bits::new(s_89_4 as u128, 1u16);
        // S s_89_6: cmp-eq s_89_3 s_89_5
        let s_89_6: bool = ((s_89_3) == (s_89_5));
        // D s_89_7: write-var gs#129867 <= s_89_6
        fn_state.gs_129867 = s_89_6;
        // N s_89_8: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var __MDCR_EL3_TPM:u8
        let s_90_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_90_1: cast zx s_90_0 -> bv
        let s_90_1: Bits = Bits::new(s_90_0 as u128, 1u16);
        // C s_90_2: const #1u : u8
        let s_90_2: bool = true;
        // C s_90_3: cast zx s_90_2 -> bv
        let s_90_3: Bits = Bits::new(s_90_2 as u128, 1u16);
        // D s_90_4: cmp-eq s_90_1 s_90_3
        let s_90_4: bool = ((s_90_1) == (s_90_3));
        // D s_90_5: write-var gs#129866 <= s_90_4
        fn_state.gs_129866 = s_90_4;
        // N s_90_6: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #424u : u32
        let s_91_0: u32 = 424;
        // D s_91_1: read-reg s_91_0:u8
        let s_91_1: u8 = {
            let value = state.read_register::<u8>(s_91_0 as isize);
            tracer.read_register(s_91_0 as isize, value);
            value
        };
        // D s_91_2: call ELUsingAArch32(s_91_1)
        let s_91_2: bool = ELUsingAArch32(state, tracer, s_91_1);
        // D s_91_3: not s_91_2
        let s_91_3: bool = !s_91_2;
        // D s_91_4: write-var gs#129865 <= s_91_3
        fn_state.gs_129865 = s_91_3;
        // N s_91_5: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #3u : u8
        let s_92_0: u8 = 3;
        // C s_92_1: cast zx s_92_0 -> bv
        let s_92_1: Bits = Bits::new(s_92_0 as u128, 8u16);
        // C s_92_2: cast zx s_92_1 -> i
        let s_92_2: i128 = (s_92_1.value() as i128);
        // C s_92_3: cast reint s_92_2 -> i64
        let s_92_3: i64 = (s_92_2 as i64);
        // C s_92_4: cast zx s_92_3 -> i
        let s_92_4: i128 = (i128::try_from(s_92_3).unwrap());
        // S s_92_5: call AArch32_TakeHypTrapException(s_92_4)
        let s_92_5: () = AArch32_TakeHypTrapException(state, tracer, s_92_4);
        // N s_92_6: return
        return;
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_93_0: read-var __HDCR_TPMCR:u8
        let s_93_0: bool = fn_state.u__HDCR_TPMCR;
        // D s_93_1: cast zx s_93_0 -> bv
        let s_93_1: Bits = Bits::new(s_93_0 as u128, 1u16);
        // C s_93_2: const #1u : u8
        let s_93_2: bool = true;
        // C s_93_3: cast zx s_93_2 -> bv
        let s_93_3: Bits = Bits::new(s_93_2 as u128, 1u16);
        // D s_93_4: cmp-eq s_93_1 s_93_3
        let s_93_4: bool = ((s_93_1) == (s_93_3));
        // D s_93_5: write-var gs#129864 <= s_93_4
        fn_state.gs_129864 = s_93_4;
        // N s_93_6: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #432u : u32
        let s_94_0: u32 = 432;
        // D s_94_1: read-reg s_94_0:u8
        let s_94_1: u8 = {
            let value = state.read_register::<u8>(s_94_0 as isize);
            tracer.read_register(s_94_0 as isize, value);
            value
        };
        // D s_94_2: call ELUsingAArch32(s_94_1)
        let s_94_2: bool = ELUsingAArch32(state, tracer, s_94_1);
        // D s_94_3: write-var gs#129863 <= s_94_2
        fn_state.gs_129863 = s_94_2;
        // N s_94_4: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #3u : u8
        let s_95_0: u8 = 3;
        // C s_95_1: cast zx s_95_0 -> bv
        let s_95_1: Bits = Bits::new(s_95_0 as u128, 8u16);
        // C s_95_2: cast zx s_95_1 -> i
        let s_95_2: i128 = (s_95_1.value() as i128);
        // C s_95_3: cast reint s_95_2 -> i64
        let s_95_3: i64 = (s_95_2 as i64);
        // C s_95_4: cast zx s_95_3 -> i
        let s_95_4: i128 = (i128::try_from(s_95_3).unwrap());
        // S s_95_5: call AArch32_TakeHypTrapException(s_95_4)
        let s_95_5: () = AArch32_TakeHypTrapException(state, tracer, s_95_4);
        // N s_95_6: return
        return;
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var __HDCR_TPM:u8
        let s_96_0: bool = fn_state.u__HDCR_TPM;
        // D s_96_1: cast zx s_96_0 -> bv
        let s_96_1: Bits = Bits::new(s_96_0 as u128, 1u16);
        // C s_96_2: const #1u : u8
        let s_96_2: bool = true;
        // C s_96_3: cast zx s_96_2 -> bv
        let s_96_3: Bits = Bits::new(s_96_2 as u128, 1u16);
        // D s_96_4: cmp-eq s_96_1 s_96_3
        let s_96_4: bool = ((s_96_1) == (s_96_3));
        // D s_96_5: write-var gs#129862 <= s_96_4
        fn_state.gs_129862 = s_96_4;
        // N s_96_6: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #432u : u32
        let s_97_0: u32 = 432;
        // D s_97_1: read-reg s_97_0:u8
        let s_97_1: u8 = {
            let value = state.read_register::<u8>(s_97_0 as isize);
            tracer.read_register(s_97_0 as isize, value);
            value
        };
        // D s_97_2: call ELUsingAArch32(s_97_1)
        let s_97_2: bool = ELUsingAArch32(state, tracer, s_97_1);
        // D s_97_3: write-var gs#129861 <= s_97_2
        fn_state.gs_129861 = s_97_2;
        // N s_97_4: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #3u : u8
        let s_98_0: u8 = 3;
        // C s_98_1: cast zx s_98_0 -> bv
        let s_98_1: Bits = Bits::new(s_98_0 as u128, 8u16);
        // C s_98_2: cast zx s_98_1 -> i
        let s_98_2: i128 = (s_98_1.value() as i128);
        // C s_98_3: cast reint s_98_2 -> i64
        let s_98_3: i64 = (s_98_2 as i64);
        // C s_98_4: cast zx s_98_3 -> i
        let s_98_4: i128 = (i128::try_from(s_98_3).unwrap());
        // C s_98_5: const #432u : u32
        let s_98_5: u32 = 432;
        // D s_98_6: read-reg s_98_5:u8
        let s_98_6: u8 = {
            let value = state.read_register::<u8>(s_98_5 as isize);
            tracer.read_register(s_98_5 as isize, value);
            value
        };
        // D s_98_7: call AArch64_AArch32SystemAccessTrap(s_98_6, s_98_4)
        let s_98_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_98_6, s_98_4);
        // N s_98_8: return
        return;
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_99_0: read-var __MDCR_EL2_TPMCR:u8
        let s_99_0: bool = fn_state.u__MDCR_EL2_TPMCR;
        // D s_99_1: cast zx s_99_0 -> bv
        let s_99_1: Bits = Bits::new(s_99_0 as u128, 1u16);
        // C s_99_2: const #1u : u8
        let s_99_2: bool = true;
        // C s_99_3: cast zx s_99_2 -> bv
        let s_99_3: Bits = Bits::new(s_99_2 as u128, 1u16);
        // D s_99_4: cmp-eq s_99_1 s_99_3
        let s_99_4: bool = ((s_99_1) == (s_99_3));
        // D s_99_5: write-var gs#129860 <= s_99_4
        fn_state.gs_129860 = s_99_4;
        // N s_99_6: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #432u : u32
        let s_100_0: u32 = 432;
        // D s_100_1: read-reg s_100_0:u8
        let s_100_1: u8 = {
            let value = state.read_register::<u8>(s_100_0 as isize);
            tracer.read_register(s_100_0 as isize, value);
            value
        };
        // D s_100_2: call ELUsingAArch32(s_100_1)
        let s_100_2: bool = ELUsingAArch32(state, tracer, s_100_1);
        // D s_100_3: not s_100_2
        let s_100_3: bool = !s_100_2;
        // D s_100_4: write-var gs#129859 <= s_100_3
        fn_state.gs_129859 = s_100_3;
        // N s_100_5: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #3u : u8
        let s_101_0: u8 = 3;
        // C s_101_1: cast zx s_101_0 -> bv
        let s_101_1: Bits = Bits::new(s_101_0 as u128, 8u16);
        // C s_101_2: cast zx s_101_1 -> i
        let s_101_2: i128 = (s_101_1.value() as i128);
        // C s_101_3: cast reint s_101_2 -> i64
        let s_101_3: i64 = (s_101_2 as i64);
        // C s_101_4: cast zx s_101_3 -> i
        let s_101_4: i128 = (i128::try_from(s_101_3).unwrap());
        // C s_101_5: const #432u : u32
        let s_101_5: u32 = 432;
        // D s_101_6: read-reg s_101_5:u8
        let s_101_6: u8 = {
            let value = state.read_register::<u8>(s_101_5 as isize);
            tracer.read_register(s_101_5 as isize, value);
            value
        };
        // D s_101_7: call AArch64_AArch32SystemAccessTrap(s_101_6, s_101_4)
        let s_101_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_101_6,
            s_101_4,
        );
        // N s_101_8: return
        return;
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var __MDCR_EL2_TPM:u8
        let s_102_0: bool = fn_state.u__MDCR_EL2_TPM;
        // D s_102_1: cast zx s_102_0 -> bv
        let s_102_1: Bits = Bits::new(s_102_0 as u128, 1u16);
        // C s_102_2: const #1u : u8
        let s_102_2: bool = true;
        // C s_102_3: cast zx s_102_2 -> bv
        let s_102_3: Bits = Bits::new(s_102_2 as u128, 1u16);
        // D s_102_4: cmp-eq s_102_1 s_102_3
        let s_102_4: bool = ((s_102_1) == (s_102_3));
        // D s_102_5: write-var gs#129858 <= s_102_4
        fn_state.gs_129858 = s_102_4;
        // N s_102_6: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #432u : u32
        let s_103_0: u32 = 432;
        // D s_103_1: read-reg s_103_0:u8
        let s_103_1: u8 = {
            let value = state.read_register::<u8>(s_103_0 as isize);
            tracer.read_register(s_103_0 as isize, value);
            value
        };
        // D s_103_2: call ELUsingAArch32(s_103_1)
        let s_103_2: bool = ELUsingAArch32(state, tracer, s_103_1);
        // D s_103_3: not s_103_2
        let s_103_3: bool = !s_103_2;
        // D s_103_4: write-var gs#129857 <= s_103_3
        fn_state.gs_129857 = s_103_3;
        // N s_103_5: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #3u : u8
        let s_104_0: u8 = 3;
        // C s_104_1: cast zx s_104_0 -> bv
        let s_104_1: Bits = Bits::new(s_104_0 as u128, 8u16);
        // C s_104_2: cast zx s_104_1 -> i
        let s_104_2: i128 = (s_104_1.value() as i128);
        // C s_104_3: cast reint s_104_2 -> i64
        let s_104_3: i64 = (s_104_2 as i64);
        // C s_104_4: cast zx s_104_3 -> i
        let s_104_4: i128 = (i128::try_from(s_104_3).unwrap());
        // S s_104_5: call AArch32_TakeHypTrapException(s_104_4)
        let s_104_5: () = AArch32_TakeHypTrapException(state, tracer, s_104_4);
        // N s_104_6: return
        return;
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_105_0: read-var __HSTR_T9:u8
        let s_105_0: bool = fn_state.u__HSTR_T9;
        // D s_105_1: cast zx s_105_0 -> bv
        let s_105_1: Bits = Bits::new(s_105_0 as u128, 1u16);
        // C s_105_2: const #1u : u8
        let s_105_2: bool = true;
        // C s_105_3: cast zx s_105_2 -> bv
        let s_105_3: Bits = Bits::new(s_105_2 as u128, 1u16);
        // D s_105_4: cmp-eq s_105_1 s_105_3
        let s_105_4: bool = ((s_105_1) == (s_105_3));
        // D s_105_5: write-var gs#129856 <= s_105_4
        fn_state.gs_129856 = s_105_4;
        // N s_105_6: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #432u : u32
        let s_106_0: u32 = 432;
        // D s_106_1: read-reg s_106_0:u8
        let s_106_1: u8 = {
            let value = state.read_register::<u8>(s_106_0 as isize);
            tracer.read_register(s_106_0 as isize, value);
            value
        };
        // D s_106_2: call ELUsingAArch32(s_106_1)
        let s_106_2: bool = ELUsingAArch32(state, tracer, s_106_1);
        // D s_106_3: write-var gs#129855 <= s_106_2
        fn_state.gs_129855 = s_106_2;
        // N s_106_4: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #3u : u8
        let s_107_0: u8 = 3;
        // C s_107_1: cast zx s_107_0 -> bv
        let s_107_1: Bits = Bits::new(s_107_0 as u128, 8u16);
        // C s_107_2: cast zx s_107_1 -> i
        let s_107_2: i128 = (s_107_1.value() as i128);
        // C s_107_3: cast reint s_107_2 -> i64
        let s_107_3: i64 = (s_107_2 as i64);
        // C s_107_4: cast zx s_107_3 -> i
        let s_107_4: i128 = (i128::try_from(s_107_3).unwrap());
        // C s_107_5: const #432u : u32
        let s_107_5: u32 = 432;
        // D s_107_6: read-reg s_107_5:u8
        let s_107_6: u8 = {
            let value = state.read_register::<u8>(s_107_5 as isize);
            tracer.read_register(s_107_5 as isize, value);
            value
        };
        // D s_107_7: call AArch64_AArch32SystemAccessTrap(s_107_6, s_107_4)
        let s_107_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_107_6,
            s_107_4,
        );
        // N s_107_8: return
        return;
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var __HSTR_EL2_T9:u8
        let s_108_0: bool = fn_state.u__HSTR_EL2_T9;
        // D s_108_1: cast zx s_108_0 -> bv
        let s_108_1: Bits = Bits::new(s_108_0 as u128, 1u16);
        // C s_108_2: const #1u : u8
        let s_108_2: bool = true;
        // C s_108_3: cast zx s_108_2 -> bv
        let s_108_3: Bits = Bits::new(s_108_2 as u128, 1u16);
        // D s_108_4: cmp-eq s_108_1 s_108_3
        let s_108_4: bool = ((s_108_1) == (s_108_3));
        // D s_108_5: write-var gs#129854 <= s_108_4
        fn_state.gs_129854 = s_108_4;
        // N s_108_6: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #432u : u32
        let s_109_0: u32 = 432;
        // D s_109_1: read-reg s_109_0:u8
        let s_109_1: u8 = {
            let value = state.read_register::<u8>(s_109_0 as isize);
            tracer.read_register(s_109_0 as isize, value);
            value
        };
        // D s_109_2: call ELUsingAArch32(s_109_1)
        let s_109_2: bool = ELUsingAArch32(state, tracer, s_109_1);
        // D s_109_3: not s_109_2
        let s_109_3: bool = !s_109_2;
        // D s_109_4: write-var gs#129853 <= s_109_3
        fn_state.gs_129853 = s_109_3;
        // N s_109_5: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_110_0: panic
        panic!("{:?}", ());
        // N s_110_1: return
        return;
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var __MDCR_EL3_TPM:u8
        let s_111_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_111_1: cast zx s_111_0 -> bv
        let s_111_1: Bits = Bits::new(s_111_0 as u128, 1u16);
        // C s_111_2: const #1u : u8
        let s_111_2: bool = true;
        // C s_111_3: cast zx s_111_2 -> bv
        let s_111_3: Bits = Bits::new(s_111_2 as u128, 1u16);
        // D s_111_4: cmp-eq s_111_1 s_111_3
        let s_111_4: bool = ((s_111_1) == (s_111_3));
        // D s_111_5: write-var gs#129852 <= s_111_4
        fn_state.gs_129852 = s_111_4;
        // N s_111_6: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_112_0: const #424u : u32
        let s_112_0: u32 = 424;
        // D s_112_1: read-reg s_112_0:u8
        let s_112_1: u8 = {
            let value = state.read_register::<u8>(s_112_0 as isize);
            tracer.read_register(s_112_0 as isize, value);
            value
        };
        // D s_112_2: call ELUsingAArch32(s_112_1)
        let s_112_2: bool = ELUsingAArch32(state, tracer, s_112_1);
        // D s_112_3: not s_112_2
        let s_112_3: bool = !s_112_2;
        // D s_112_4: write-var gs#129851 <= s_112_3
        fn_state.gs_129851 = s_112_3;
        // N s_112_5: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_113_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_113_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_113_1: call __IMPDEF_boolean(s_113_0)
        let s_113_1: bool = u__IMPDEF_boolean(state, tracer, s_113_0);
        // D s_113_2: write-var gs#129850 <= s_113_1
        fn_state.gs_129850 = s_113_1;
        // N s_113_3: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #() : ()
        let s_114_0: () = ();
        // S s_114_1: call EDSCR_read(s_114_0)
        let s_114_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_114_0);
        // S s_114_2: call _get_EDSCR_Type_SDD(s_114_1)
        let s_114_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_114_1);
        // S s_114_3: cast zx s_114_2 -> bv
        let s_114_3: Bits = Bits::new(s_114_2 as u128, 1u16);
        // C s_114_4: const #1u : u8
        let s_114_4: bool = true;
        // C s_114_5: cast zx s_114_4 -> bv
        let s_114_5: Bits = Bits::new(s_114_4 as u128, 1u16);
        // S s_114_6: cmp-eq s_114_3 s_114_5
        let s_114_6: bool = ((s_114_3) == (s_114_5));
        // D s_114_7: write-var gs#129849 <= s_114_6
        fn_state.gs_129849 = s_114_6;
        // N s_114_8: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #424u : u32
        let s_115_0: u32 = 424;
        // D s_115_1: read-reg s_115_0:u8
        let s_115_1: u8 = {
            let value = state.read_register::<u8>(s_115_0 as isize);
            tracer.read_register(s_115_0 as isize, value);
            value
        };
        // C s_115_2: const #2u : u8
        let s_115_2: u8 = 2;
        // D s_115_3: cmp-lt s_115_1 s_115_2
        let s_115_3: bool = ((s_115_1) < (s_115_2));
        // D s_115_4: write-var gs#129848 <= s_115_3
        fn_state.gs_129848 = s_115_3;
        // N s_115_5: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #() : ()
        let s_116_0: () = ();
        // S s_116_1: call Halted(s_116_0)
        let s_116_1: bool = Halted(state, tracer, s_116_0);
        // N s_116_2: branch s_116_1 b260 b117
        if s_116_1 {
            return block_260(state, tracer, fn_state);
        } else {
            return block_117(state, tracer, fn_state);
        };
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #0u : u8
        let s_117_0: bool = false;
        // D s_117_1: write-var gs#129868 <= s_117_0
        fn_state.gs_129868 = s_117_0;
        // N s_117_2: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_118_0: read-var gs#129868:u8
        let s_118_0: bool = fn_state.gs_129868;
        // N s_118_1: branch s_118_0 b259 b119
        if s_118_0 {
            return block_259(state, tracer, fn_state);
        } else {
            return block_119(state, tracer, fn_state);
        };
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #0u : u8
        let s_119_0: bool = false;
        // D s_119_1: write-var gs#129869 <= s_119_0
        fn_state.gs_129869 = s_119_0;
        // N s_119_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_120_0: read-var gs#129869:u8
        let s_120_0: bool = fn_state.gs_129869;
        // N s_120_1: branch s_120_0 b258 b121
        if s_120_0 {
            return block_258(state, tracer, fn_state);
        } else {
            return block_121(state, tracer, fn_state);
        };
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #0u : u8
        let s_121_0: bool = false;
        // D s_121_1: write-var gs#129870 <= s_121_0
        fn_state.gs_129870 = s_121_0;
        // N s_121_2: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_122_0: read-var gs#129870:u8
        let s_122_0: bool = fn_state.gs_129870;
        // N s_122_1: branch s_122_0 b257 b123
        if s_122_0 {
            return block_257(state, tracer, fn_state);
        } else {
            return block_123(state, tracer, fn_state);
        };
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_123_0: const #0u : u8
        let s_123_0: bool = false;
        // D s_123_1: write-var gs#129871 <= s_123_0
        fn_state.gs_129871 = s_123_0;
        // N s_123_2: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_124_0: read-var gs#129871:u8
        let s_124_0: bool = fn_state.gs_129871;
        // N s_124_1: branch s_124_0 b256 b125
        if s_124_0 {
            return block_256(state, tracer, fn_state);
        } else {
            return block_125(state, tracer, fn_state);
        };
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_125_0: const #0u : u8
        let s_125_0: bool = false;
        // D s_125_1: write-var gs#129872 <= s_125_0
        fn_state.gs_129872 = s_125_0;
        // N s_125_2: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_126_0: read-var gs#129872:u8
        let s_126_0: bool = fn_state.gs_129872;
        // N s_126_1: branch s_126_0 b255 b127
        if s_126_0 {
            return block_255(state, tracer, fn_state);
        } else {
            return block_127(state, tracer, fn_state);
        };
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #440u : u32
        let s_127_0: u32 = 440;
        // D s_127_1: read-reg s_127_0:u8
        let s_127_1: u8 = {
            let value = state.read_register::<u8>(s_127_0 as isize);
            tracer.read_register(s_127_0 as isize, value);
            value
        };
        // D s_127_2: call ELUsingAArch32(s_127_1)
        let s_127_2: bool = ELUsingAArch32(state, tracer, s_127_1);
        // D s_127_3: not s_127_2
        let s_127_3: bool = !s_127_2;
        // N s_127_4: branch s_127_3 b245 b128
        if s_127_3 {
            return block_245(state, tracer, fn_state);
        } else {
            return block_128(state, tracer, fn_state);
        };
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_128_0: const #0u : u8
        let s_128_0: bool = false;
        // D s_128_1: write-var gs#129876 <= s_128_0
        fn_state.gs_129876 = s_128_0;
        // N s_128_2: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_129_0: read-var gs#129876:u8
        let s_129_0: bool = fn_state.gs_129876;
        // N s_129_1: branch s_129_0 b236 b130
        if s_129_0 {
            return block_236(state, tracer, fn_state);
        } else {
            return block_130(state, tracer, fn_state);
        };
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_130_0: const #440u : u32
        let s_130_0: u32 = 440;
        // D s_130_1: read-reg s_130_0:u8
        let s_130_1: u8 = {
            let value = state.read_register::<u8>(s_130_0 as isize);
            tracer.read_register(s_130_0 as isize, value);
            value
        };
        // D s_130_2: call ELUsingAArch32(s_130_1)
        let s_130_2: bool = ELUsingAArch32(state, tracer, s_130_1);
        // N s_130_3: branch s_130_2 b235 b131
        if s_130_2 {
            return block_235(state, tracer, fn_state);
        } else {
            return block_131(state, tracer, fn_state);
        };
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_131_0: const #0u : u8
        let s_131_0: bool = false;
        // D s_131_1: write-var gs#129877 <= s_131_0
        fn_state.gs_129877 = s_131_0;
        // N s_131_2: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_132_0: read-var gs#129877:u8
        let s_132_0: bool = fn_state.gs_129877;
        // N s_132_1: branch s_132_0 b218 b133
        if s_132_0 {
            return block_218(state, tracer, fn_state);
        } else {
            return block_133(state, tracer, fn_state);
        };
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_133_0: const #() : ()
        let s_133_0: () = ();
        // S s_133_1: call EL2Enabled(s_133_0)
        let s_133_1: bool = EL2Enabled(state, tracer, s_133_0);
        // N s_133_2: branch s_133_1 b217 b134
        if s_133_1 {
            return block_217(state, tracer, fn_state);
        } else {
            return block_134(state, tracer, fn_state);
        };
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_134_0: const #0u : u8
        let s_134_0: bool = false;
        // D s_134_1: write-var gs#129878 <= s_134_0
        fn_state.gs_129878 = s_134_0;
        // N s_134_2: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_135_0: read-var gs#129878:u8
        let s_135_0: bool = fn_state.gs_129878;
        // N s_135_1: branch s_135_0 b216 b136
        if s_135_0 {
            return block_216(state, tracer, fn_state);
        } else {
            return block_136(state, tracer, fn_state);
        };
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_136_0: const #0u : u8
        let s_136_0: bool = false;
        // D s_136_1: write-var gs#129879 <= s_136_0
        fn_state.gs_129879 = s_136_0;
        // N s_136_2: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_137_0: read-var gs#129879:u8
        let s_137_0: bool = fn_state.gs_129879;
        // N s_137_1: branch s_137_0 b215 b138
        if s_137_0 {
            return block_215(state, tracer, fn_state);
        } else {
            return block_138(state, tracer, fn_state);
        };
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_138_0: const #0u : u8
        let s_138_0: bool = false;
        // D s_138_1: write-var gs#129880 <= s_138_0
        fn_state.gs_129880 = s_138_0;
        // N s_138_2: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_139_0: read-var gs#129880:u8
        let s_139_0: bool = fn_state.gs_129880;
        // N s_139_1: branch s_139_0 b214 b140
        if s_139_0 {
            return block_214(state, tracer, fn_state);
        } else {
            return block_140(state, tracer, fn_state);
        };
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_140_0: const #() : ()
        let s_140_0: () = ();
        // S s_140_1: call EL2Enabled(s_140_0)
        let s_140_1: bool = EL2Enabled(state, tracer, s_140_0);
        // N s_140_2: branch s_140_1 b213 b141
        if s_140_1 {
            return block_213(state, tracer, fn_state);
        } else {
            return block_141(state, tracer, fn_state);
        };
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_141_0: const #0u : u8
        let s_141_0: bool = false;
        // D s_141_1: write-var gs#129881 <= s_141_0
        fn_state.gs_129881 = s_141_0;
        // N s_141_2: jump b142
        return block_142(state, tracer, fn_state);
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_142_0: read-var gs#129881:u8
        let s_142_0: bool = fn_state.gs_129881;
        // N s_142_1: branch s_142_0 b212 b143
        if s_142_0 {
            return block_212(state, tracer, fn_state);
        } else {
            return block_143(state, tracer, fn_state);
        };
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_143_0: const #0u : u8
        let s_143_0: bool = false;
        // D s_143_1: write-var gs#129882 <= s_143_0
        fn_state.gs_129882 = s_143_0;
        // N s_143_2: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_144_0: read-var gs#129882:u8
        let s_144_0: bool = fn_state.gs_129882;
        // N s_144_1: branch s_144_0 b211 b145
        if s_144_0 {
            return block_211(state, tracer, fn_state);
        } else {
            return block_145(state, tracer, fn_state);
        };
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_145_0: const #() : ()
        let s_145_0: () = ();
        // S s_145_1: call EL2Enabled(s_145_0)
        let s_145_1: bool = EL2Enabled(state, tracer, s_145_0);
        // N s_145_2: branch s_145_1 b210 b146
        if s_145_1 {
            return block_210(state, tracer, fn_state);
        } else {
            return block_146(state, tracer, fn_state);
        };
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_146_0: const #0u : u8
        let s_146_0: bool = false;
        // D s_146_1: write-var gs#129883 <= s_146_0
        fn_state.gs_129883 = s_146_0;
        // N s_146_2: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_147_0: read-var gs#129883:u8
        let s_147_0: bool = fn_state.gs_129883;
        // N s_147_1: branch s_147_0 b209 b148
        if s_147_0 {
            return block_209(state, tracer, fn_state);
        } else {
            return block_148(state, tracer, fn_state);
        };
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_148_0: const #0u : u8
        let s_148_0: bool = false;
        // D s_148_1: write-var gs#129884 <= s_148_0
        fn_state.gs_129884 = s_148_0;
        // N s_148_2: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_149_0: read-var gs#129884:u8
        let s_149_0: bool = fn_state.gs_129884;
        // N s_149_1: branch s_149_0 b208 b150
        if s_149_0 {
            return block_208(state, tracer, fn_state);
        } else {
            return block_150(state, tracer, fn_state);
        };
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_150_0: const #0u : u8
        let s_150_0: bool = false;
        // D s_150_1: write-var gs#129885 <= s_150_0
        fn_state.gs_129885 = s_150_0;
        // N s_150_2: jump b151
        return block_151(state, tracer, fn_state);
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_151_0: read-var gs#129885:u8
        let s_151_0: bool = fn_state.gs_129885;
        // N s_151_1: branch s_151_0 b204 b152
        if s_151_0 {
            return block_204(state, tracer, fn_state);
        } else {
            return block_152(state, tracer, fn_state);
        };
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_152_0: const #0u : u8
        let s_152_0: bool = false;
        // D s_152_1: write-var gs#129887 <= s_152_0
        fn_state.gs_129887 = s_152_0;
        // N s_152_2: jump b153
        return block_153(state, tracer, fn_state);
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_153_0: read-var gs#129887:u8
        let s_153_0: bool = fn_state.gs_129887;
        // N s_153_1: branch s_153_0 b203 b154
        if s_153_0 {
            return block_203(state, tracer, fn_state);
        } else {
            return block_154(state, tracer, fn_state);
        };
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_154_0: const #0u : u8
        let s_154_0: bool = false;
        // D s_154_1: write-var gs#129888 <= s_154_0
        fn_state.gs_129888 = s_154_0;
        // N s_154_2: jump b155
        return block_155(state, tracer, fn_state);
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_155_0: read-var gs#129888:u8
        let s_155_0: bool = fn_state.gs_129888;
        // N s_155_1: branch s_155_0 b202 b156
        if s_155_0 {
            return block_202(state, tracer, fn_state);
        } else {
            return block_156(state, tracer, fn_state);
        };
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_156_0: const #() : ()
        let s_156_0: () = ();
        // S s_156_1: call EL2Enabled(s_156_0)
        let s_156_1: bool = EL2Enabled(state, tracer, s_156_0);
        // N s_156_2: branch s_156_1 b201 b157
        if s_156_1 {
            return block_201(state, tracer, fn_state);
        } else {
            return block_157(state, tracer, fn_state);
        };
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_157_0: const #0u : u8
        let s_157_0: bool = false;
        // D s_157_1: write-var gs#129889 <= s_157_0
        fn_state.gs_129889 = s_157_0;
        // N s_157_2: jump b158
        return block_158(state, tracer, fn_state);
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_158_0: read-var gs#129889:u8
        let s_158_0: bool = fn_state.gs_129889;
        // N s_158_1: branch s_158_0 b200 b159
        if s_158_0 {
            return block_200(state, tracer, fn_state);
        } else {
            return block_159(state, tracer, fn_state);
        };
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_159_0: const #0u : u8
        let s_159_0: bool = false;
        // D s_159_1: write-var gs#129890 <= s_159_0
        fn_state.gs_129890 = s_159_0;
        // N s_159_2: jump b160
        return block_160(state, tracer, fn_state);
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_160_0: read-var gs#129890:u8
        let s_160_0: bool = fn_state.gs_129890;
        // N s_160_1: branch s_160_0 b199 b161
        if s_160_0 {
            return block_199(state, tracer, fn_state);
        } else {
            return block_161(state, tracer, fn_state);
        };
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_161_0: const #() : ()
        let s_161_0: () = ();
        // S s_161_1: call EL2Enabled(s_161_0)
        let s_161_1: bool = EL2Enabled(state, tracer, s_161_0);
        // N s_161_2: branch s_161_1 b198 b162
        if s_161_1 {
            return block_198(state, tracer, fn_state);
        } else {
            return block_162(state, tracer, fn_state);
        };
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_162_0: const #0u : u8
        let s_162_0: bool = false;
        // D s_162_1: write-var gs#129891 <= s_162_0
        fn_state.gs_129891 = s_162_0;
        // N s_162_2: jump b163
        return block_163(state, tracer, fn_state);
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_163_0: read-var gs#129891:u8
        let s_163_0: bool = fn_state.gs_129891;
        // N s_163_1: branch s_163_0 b197 b164
        if s_163_0 {
            return block_197(state, tracer, fn_state);
        } else {
            return block_164(state, tracer, fn_state);
        };
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_164_0: const #0u : u8
        let s_164_0: bool = false;
        // D s_164_1: write-var gs#129892 <= s_164_0
        fn_state.gs_129892 = s_164_0;
        // N s_164_2: jump b165
        return block_165(state, tracer, fn_state);
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_165_0: read-var gs#129892:u8
        let s_165_0: bool = fn_state.gs_129892;
        // N s_165_1: branch s_165_0 b196 b166
        if s_165_0 {
            return block_196(state, tracer, fn_state);
        } else {
            return block_166(state, tracer, fn_state);
        };
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_166_0: const #() : ()
        let s_166_0: () = ();
        // S s_166_1: call EL2Enabled(s_166_0)
        let s_166_1: bool = EL2Enabled(state, tracer, s_166_0);
        // N s_166_2: branch s_166_1 b195 b167
        if s_166_1 {
            return block_195(state, tracer, fn_state);
        } else {
            return block_167(state, tracer, fn_state);
        };
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_167_0: const #0u : u8
        let s_167_0: bool = false;
        // D s_167_1: write-var gs#129893 <= s_167_0
        fn_state.gs_129893 = s_167_0;
        // N s_167_2: jump b168
        return block_168(state, tracer, fn_state);
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_168_0: read-var gs#129893:u8
        let s_168_0: bool = fn_state.gs_129893;
        // N s_168_1: branch s_168_0 b194 b169
        if s_168_0 {
            return block_194(state, tracer, fn_state);
        } else {
            return block_169(state, tracer, fn_state);
        };
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_169_0: const #0u : u8
        let s_169_0: bool = false;
        // D s_169_1: write-var gs#129894 <= s_169_0
        fn_state.gs_129894 = s_169_0;
        // N s_169_2: jump b170
        return block_170(state, tracer, fn_state);
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_170_0: read-var gs#129894:u8
        let s_170_0: bool = fn_state.gs_129894;
        // N s_170_1: branch s_170_0 b193 b171
        if s_170_0 {
            return block_193(state, tracer, fn_state);
        } else {
            return block_171(state, tracer, fn_state);
        };
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_171_0: const #() : ()
        let s_171_0: () = ();
        // S s_171_1: call EL2Enabled(s_171_0)
        let s_171_1: bool = EL2Enabled(state, tracer, s_171_0);
        // N s_171_2: branch s_171_1 b192 b172
        if s_171_1 {
            return block_192(state, tracer, fn_state);
        } else {
            return block_172(state, tracer, fn_state);
        };
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_172_0: const #0u : u8
        let s_172_0: bool = false;
        // D s_172_1: write-var gs#129895 <= s_172_0
        fn_state.gs_129895 = s_172_0;
        // N s_172_2: jump b173
        return block_173(state, tracer, fn_state);
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_173_0: read-var gs#129895:u8
        let s_173_0: bool = fn_state.gs_129895;
        // N s_173_1: branch s_173_0 b191 b174
        if s_173_0 {
            return block_191(state, tracer, fn_state);
        } else {
            return block_174(state, tracer, fn_state);
        };
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_174_0: const #0u : u8
        let s_174_0: bool = false;
        // D s_174_1: write-var gs#129896 <= s_174_0
        fn_state.gs_129896 = s_174_0;
        // N s_174_2: jump b175
        return block_175(state, tracer, fn_state);
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_175_0: read-var gs#129896:u8
        let s_175_0: bool = fn_state.gs_129896;
        // N s_175_1: branch s_175_0 b190 b176
        if s_175_0 {
            return block_190(state, tracer, fn_state);
        } else {
            return block_176(state, tracer, fn_state);
        };
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_176_0: const #424u : u32
        let s_176_0: u32 = 424;
        // D s_176_1: read-reg s_176_0:u8
        let s_176_1: u8 = {
            let value = state.read_register::<u8>(s_176_0 as isize);
            tracer.read_register(s_176_0 as isize, value);
            value
        };
        // C s_176_2: const #2u : u8
        let s_176_2: u8 = 2;
        // D s_176_3: cmp-lt s_176_1 s_176_2
        let s_176_3: bool = ((s_176_1) < (s_176_2));
        // N s_176_4: branch s_176_3 b189 b177
        if s_176_3 {
            return block_189(state, tracer, fn_state);
        } else {
            return block_177(state, tracer, fn_state);
        };
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_177_0: const #0u : u8
        let s_177_0: bool = false;
        // D s_177_1: write-var gs#129897 <= s_177_0
        fn_state.gs_129897 = s_177_0;
        // N s_177_2: jump b178
        return block_178(state, tracer, fn_state);
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_178_0: read-var gs#129897:u8
        let s_178_0: bool = fn_state.gs_129897;
        // N s_178_1: branch s_178_0 b188 b179
        if s_178_0 {
            return block_188(state, tracer, fn_state);
        } else {
            return block_179(state, tracer, fn_state);
        };
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_179_0: const #0u : u8
        let s_179_0: bool = false;
        // D s_179_1: write-var gs#129898 <= s_179_0
        fn_state.gs_129898 = s_179_0;
        // N s_179_2: jump b180
        return block_180(state, tracer, fn_state);
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_180_0: read-var gs#129898:u8
        let s_180_0: bool = fn_state.gs_129898;
        // N s_180_1: branch s_180_0 b182 b181
        if s_180_0 {
            return block_182(state, tracer, fn_state);
        } else {
            return block_181(state, tracer, fn_state);
        };
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_181_0: read-var t:i
        let s_181_0: i128 = fn_state.t;
        // D s_181_1: call R_read(s_181_0)
        let s_181_1: u32 = R_read(state, tracer, s_181_0);
        // D s_181_2: call Mk_PMCR_Type(s_181_1)
        let s_181_2: ProductType700c18a878c5601b = Mk_PMCR_Type(state, tracer, s_181_1);
        // D s_181_3: call __set_PMCR(s_181_2)
        let s_181_3: () = u__set_PMCR(state, tracer, s_181_2);
        // N s_181_4: return
        return;
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_182_0: const #() : ()
        let s_182_0: () = ();
        // S s_182_1: call Halted(s_182_0)
        let s_182_1: bool = Halted(state, tracer, s_182_0);
        // N s_182_2: branch s_182_1 b187 b183
        if s_182_1 {
            return block_187(state, tracer, fn_state);
        } else {
            return block_183(state, tracer, fn_state);
        };
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_183_0: const #0u : u8
        let s_183_0: bool = false;
        // D s_183_1: write-var gs#129899 <= s_183_0
        fn_state.gs_129899 = s_183_0;
        // N s_183_2: jump b184
        return block_184(state, tracer, fn_state);
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_184_0: read-var gs#129899:u8
        let s_184_0: bool = fn_state.gs_129899;
        // N s_184_1: branch s_184_0 b186 b185
        if s_184_0 {
            return block_186(state, tracer, fn_state);
        } else {
            return block_185(state, tracer, fn_state);
        };
    }
    fn block_185<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_185_0: const #3u : u8
        let s_185_0: u8 = 3;
        // C s_185_1: cast zx s_185_0 -> bv
        let s_185_1: Bits = Bits::new(s_185_0 as u128, 8u16);
        // C s_185_2: cast zx s_185_1 -> i
        let s_185_2: i128 = (s_185_1.value() as i128);
        // C s_185_3: cast reint s_185_2 -> i64
        let s_185_3: i64 = (s_185_2 as i64);
        // C s_185_4: cast zx s_185_3 -> i
        let s_185_4: i128 = (i128::try_from(s_185_3).unwrap());
        // C s_185_5: const #424u : u32
        let s_185_5: u32 = 424;
        // D s_185_6: read-reg s_185_5:u8
        let s_185_6: u8 = {
            let value = state.read_register::<u8>(s_185_5 as isize);
            tracer.read_register(s_185_5 as isize, value);
            value
        };
        // D s_185_7: call AArch64_AArch32SystemAccessTrap(s_185_6, s_185_4)
        let s_185_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_185_6,
            s_185_4,
        );
        // N s_185_8: return
        return;
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_186_0: panic
        panic!("{:?}", ());
        // N s_186_1: return
        return;
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_187_0: const #() : ()
        let s_187_0: () = ();
        // S s_187_1: call EDSCR_read(s_187_0)
        let s_187_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_187_0);
        // S s_187_2: call _get_EDSCR_Type_SDD(s_187_1)
        let s_187_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_187_1);
        // S s_187_3: cast zx s_187_2 -> bv
        let s_187_3: Bits = Bits::new(s_187_2 as u128, 1u16);
        // C s_187_4: const #1u : u8
        let s_187_4: bool = true;
        // C s_187_5: cast zx s_187_4 -> bv
        let s_187_5: Bits = Bits::new(s_187_4 as u128, 1u16);
        // S s_187_6: cmp-eq s_187_3 s_187_5
        let s_187_6: bool = ((s_187_3) == (s_187_5));
        // D s_187_7: write-var gs#129899 <= s_187_6
        fn_state.gs_129899 = s_187_6;
        // N s_187_8: jump b184
        return block_184(state, tracer, fn_state);
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_188_0: read-var __MDCR_EL3_TPM:u8
        let s_188_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_188_1: cast zx s_188_0 -> bv
        let s_188_1: Bits = Bits::new(s_188_0 as u128, 1u16);
        // C s_188_2: const #1u : u8
        let s_188_2: bool = true;
        // C s_188_3: cast zx s_188_2 -> bv
        let s_188_3: Bits = Bits::new(s_188_2 as u128, 1u16);
        // D s_188_4: cmp-eq s_188_1 s_188_3
        let s_188_4: bool = ((s_188_1) == (s_188_3));
        // D s_188_5: write-var gs#129898 <= s_188_4
        fn_state.gs_129898 = s_188_4;
        // N s_188_6: jump b180
        return block_180(state, tracer, fn_state);
    }
    fn block_189<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_189_0: const #424u : u32
        let s_189_0: u32 = 424;
        // D s_189_1: read-reg s_189_0:u8
        let s_189_1: u8 = {
            let value = state.read_register::<u8>(s_189_0 as isize);
            tracer.read_register(s_189_0 as isize, value);
            value
        };
        // D s_189_2: call ELUsingAArch32(s_189_1)
        let s_189_2: bool = ELUsingAArch32(state, tracer, s_189_1);
        // D s_189_3: not s_189_2
        let s_189_3: bool = !s_189_2;
        // D s_189_4: write-var gs#129897 <= s_189_3
        fn_state.gs_129897 = s_189_3;
        // N s_189_5: jump b178
        return block_178(state, tracer, fn_state);
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_190_0: const #3u : u8
        let s_190_0: u8 = 3;
        // C s_190_1: cast zx s_190_0 -> bv
        let s_190_1: Bits = Bits::new(s_190_0 as u128, 8u16);
        // C s_190_2: cast zx s_190_1 -> i
        let s_190_2: i128 = (s_190_1.value() as i128);
        // C s_190_3: cast reint s_190_2 -> i64
        let s_190_3: i64 = (s_190_2 as i64);
        // C s_190_4: cast zx s_190_3 -> i
        let s_190_4: i128 = (i128::try_from(s_190_3).unwrap());
        // S s_190_5: call AArch32_TakeHypTrapException(s_190_4)
        let s_190_5: () = AArch32_TakeHypTrapException(state, tracer, s_190_4);
        // N s_190_6: return
        return;
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_191_0: read-var __HDCR_TPMCR:u8
        let s_191_0: bool = fn_state.u__HDCR_TPMCR;
        // D s_191_1: cast zx s_191_0 -> bv
        let s_191_1: Bits = Bits::new(s_191_0 as u128, 1u16);
        // C s_191_2: const #1u : u8
        let s_191_2: bool = true;
        // C s_191_3: cast zx s_191_2 -> bv
        let s_191_3: Bits = Bits::new(s_191_2 as u128, 1u16);
        // D s_191_4: cmp-eq s_191_1 s_191_3
        let s_191_4: bool = ((s_191_1) == (s_191_3));
        // D s_191_5: write-var gs#129896 <= s_191_4
        fn_state.gs_129896 = s_191_4;
        // N s_191_6: jump b175
        return block_175(state, tracer, fn_state);
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_192_0: const #432u : u32
        let s_192_0: u32 = 432;
        // D s_192_1: read-reg s_192_0:u8
        let s_192_1: u8 = {
            let value = state.read_register::<u8>(s_192_0 as isize);
            tracer.read_register(s_192_0 as isize, value);
            value
        };
        // D s_192_2: call ELUsingAArch32(s_192_1)
        let s_192_2: bool = ELUsingAArch32(state, tracer, s_192_1);
        // D s_192_3: write-var gs#129895 <= s_192_2
        fn_state.gs_129895 = s_192_2;
        // N s_192_4: jump b173
        return block_173(state, tracer, fn_state);
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_193_0: const #3u : u8
        let s_193_0: u8 = 3;
        // C s_193_1: cast zx s_193_0 -> bv
        let s_193_1: Bits = Bits::new(s_193_0 as u128, 8u16);
        // C s_193_2: cast zx s_193_1 -> i
        let s_193_2: i128 = (s_193_1.value() as i128);
        // C s_193_3: cast reint s_193_2 -> i64
        let s_193_3: i64 = (s_193_2 as i64);
        // C s_193_4: cast zx s_193_3 -> i
        let s_193_4: i128 = (i128::try_from(s_193_3).unwrap());
        // S s_193_5: call AArch32_TakeHypTrapException(s_193_4)
        let s_193_5: () = AArch32_TakeHypTrapException(state, tracer, s_193_4);
        // N s_193_6: return
        return;
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_194_0: read-var __HDCR_TPM:u8
        let s_194_0: bool = fn_state.u__HDCR_TPM;
        // D s_194_1: cast zx s_194_0 -> bv
        let s_194_1: Bits = Bits::new(s_194_0 as u128, 1u16);
        // C s_194_2: const #1u : u8
        let s_194_2: bool = true;
        // C s_194_3: cast zx s_194_2 -> bv
        let s_194_3: Bits = Bits::new(s_194_2 as u128, 1u16);
        // D s_194_4: cmp-eq s_194_1 s_194_3
        let s_194_4: bool = ((s_194_1) == (s_194_3));
        // D s_194_5: write-var gs#129894 <= s_194_4
        fn_state.gs_129894 = s_194_4;
        // N s_194_6: jump b170
        return block_170(state, tracer, fn_state);
    }
    fn block_195<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_195_0: const #432u : u32
        let s_195_0: u32 = 432;
        // D s_195_1: read-reg s_195_0:u8
        let s_195_1: u8 = {
            let value = state.read_register::<u8>(s_195_0 as isize);
            tracer.read_register(s_195_0 as isize, value);
            value
        };
        // D s_195_2: call ELUsingAArch32(s_195_1)
        let s_195_2: bool = ELUsingAArch32(state, tracer, s_195_1);
        // D s_195_3: write-var gs#129893 <= s_195_2
        fn_state.gs_129893 = s_195_2;
        // N s_195_4: jump b168
        return block_168(state, tracer, fn_state);
    }
    fn block_196<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_196_0: const #3u : u8
        let s_196_0: u8 = 3;
        // C s_196_1: cast zx s_196_0 -> bv
        let s_196_1: Bits = Bits::new(s_196_0 as u128, 8u16);
        // C s_196_2: cast zx s_196_1 -> i
        let s_196_2: i128 = (s_196_1.value() as i128);
        // C s_196_3: cast reint s_196_2 -> i64
        let s_196_3: i64 = (s_196_2 as i64);
        // C s_196_4: cast zx s_196_3 -> i
        let s_196_4: i128 = (i128::try_from(s_196_3).unwrap());
        // C s_196_5: const #432u : u32
        let s_196_5: u32 = 432;
        // D s_196_6: read-reg s_196_5:u8
        let s_196_6: u8 = {
            let value = state.read_register::<u8>(s_196_5 as isize);
            tracer.read_register(s_196_5 as isize, value);
            value
        };
        // D s_196_7: call AArch64_AArch32SystemAccessTrap(s_196_6, s_196_4)
        let s_196_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_196_6,
            s_196_4,
        );
        // N s_196_8: return
        return;
    }
    fn block_197<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_197_0: read-var __MDCR_EL2_TPMCR:u8
        let s_197_0: bool = fn_state.u__MDCR_EL2_TPMCR;
        // D s_197_1: cast zx s_197_0 -> bv
        let s_197_1: Bits = Bits::new(s_197_0 as u128, 1u16);
        // C s_197_2: const #1u : u8
        let s_197_2: bool = true;
        // C s_197_3: cast zx s_197_2 -> bv
        let s_197_3: Bits = Bits::new(s_197_2 as u128, 1u16);
        // D s_197_4: cmp-eq s_197_1 s_197_3
        let s_197_4: bool = ((s_197_1) == (s_197_3));
        // D s_197_5: write-var gs#129892 <= s_197_4
        fn_state.gs_129892 = s_197_4;
        // N s_197_6: jump b165
        return block_165(state, tracer, fn_state);
    }
    fn block_198<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_198_0: const #432u : u32
        let s_198_0: u32 = 432;
        // D s_198_1: read-reg s_198_0:u8
        let s_198_1: u8 = {
            let value = state.read_register::<u8>(s_198_0 as isize);
            tracer.read_register(s_198_0 as isize, value);
            value
        };
        // D s_198_2: call ELUsingAArch32(s_198_1)
        let s_198_2: bool = ELUsingAArch32(state, tracer, s_198_1);
        // D s_198_3: not s_198_2
        let s_198_3: bool = !s_198_2;
        // D s_198_4: write-var gs#129891 <= s_198_3
        fn_state.gs_129891 = s_198_3;
        // N s_198_5: jump b163
        return block_163(state, tracer, fn_state);
    }
    fn block_199<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_199_0: const #3u : u8
        let s_199_0: u8 = 3;
        // C s_199_1: cast zx s_199_0 -> bv
        let s_199_1: Bits = Bits::new(s_199_0 as u128, 8u16);
        // C s_199_2: cast zx s_199_1 -> i
        let s_199_2: i128 = (s_199_1.value() as i128);
        // C s_199_3: cast reint s_199_2 -> i64
        let s_199_3: i64 = (s_199_2 as i64);
        // C s_199_4: cast zx s_199_3 -> i
        let s_199_4: i128 = (i128::try_from(s_199_3).unwrap());
        // C s_199_5: const #432u : u32
        let s_199_5: u32 = 432;
        // D s_199_6: read-reg s_199_5:u8
        let s_199_6: u8 = {
            let value = state.read_register::<u8>(s_199_5 as isize);
            tracer.read_register(s_199_5 as isize, value);
            value
        };
        // D s_199_7: call AArch64_AArch32SystemAccessTrap(s_199_6, s_199_4)
        let s_199_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_199_6,
            s_199_4,
        );
        // N s_199_8: return
        return;
    }
    fn block_200<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_200_0: read-var __MDCR_EL2_TPM:u8
        let s_200_0: bool = fn_state.u__MDCR_EL2_TPM;
        // D s_200_1: cast zx s_200_0 -> bv
        let s_200_1: Bits = Bits::new(s_200_0 as u128, 1u16);
        // C s_200_2: const #1u : u8
        let s_200_2: bool = true;
        // C s_200_3: cast zx s_200_2 -> bv
        let s_200_3: Bits = Bits::new(s_200_2 as u128, 1u16);
        // D s_200_4: cmp-eq s_200_1 s_200_3
        let s_200_4: bool = ((s_200_1) == (s_200_3));
        // D s_200_5: write-var gs#129890 <= s_200_4
        fn_state.gs_129890 = s_200_4;
        // N s_200_6: jump b160
        return block_160(state, tracer, fn_state);
    }
    fn block_201<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_201_0: const #432u : u32
        let s_201_0: u32 = 432;
        // D s_201_1: read-reg s_201_0:u8
        let s_201_1: u8 = {
            let value = state.read_register::<u8>(s_201_0 as isize);
            tracer.read_register(s_201_0 as isize, value);
            value
        };
        // D s_201_2: call ELUsingAArch32(s_201_1)
        let s_201_2: bool = ELUsingAArch32(state, tracer, s_201_1);
        // D s_201_3: not s_201_2
        let s_201_3: bool = !s_201_2;
        // D s_201_4: write-var gs#129889 <= s_201_3
        fn_state.gs_129889 = s_201_3;
        // N s_201_5: jump b158
        return block_158(state, tracer, fn_state);
    }
    fn block_202<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_202_0: const #3u : u8
        let s_202_0: u8 = 3;
        // C s_202_1: cast zx s_202_0 -> bv
        let s_202_1: Bits = Bits::new(s_202_0 as u128, 8u16);
        // C s_202_2: cast zx s_202_1 -> i
        let s_202_2: i128 = (s_202_1.value() as i128);
        // C s_202_3: cast reint s_202_2 -> i64
        let s_202_3: i64 = (s_202_2 as i64);
        // C s_202_4: cast zx s_202_3 -> i
        let s_202_4: i128 = (i128::try_from(s_202_3).unwrap());
        // C s_202_5: const #432u : u32
        let s_202_5: u32 = 432;
        // D s_202_6: read-reg s_202_5:u8
        let s_202_6: u8 = {
            let value = state.read_register::<u8>(s_202_5 as isize);
            tracer.read_register(s_202_5 as isize, value);
            value
        };
        // D s_202_7: call AArch64_AArch32SystemAccessTrap(s_202_6, s_202_4)
        let s_202_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_202_6,
            s_202_4,
        );
        // N s_202_8: return
        return;
    }
    fn block_203<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_203_0: read-var __HDFGWTR_EL2_PMCR_EL0:u8
        let s_203_0: bool = fn_state.u__HDFGWTR_EL2_PMCR_EL0;
        // D s_203_1: cast zx s_203_0 -> bv
        let s_203_1: Bits = Bits::new(s_203_0 as u128, 1u16);
        // C s_203_2: const #1u : u8
        let s_203_2: bool = true;
        // C s_203_3: cast zx s_203_2 -> bv
        let s_203_3: Bits = Bits::new(s_203_2 as u128, 1u16);
        // D s_203_4: cmp-eq s_203_1 s_203_3
        let s_203_4: bool = ((s_203_1) == (s_203_3));
        // D s_203_5: write-var gs#129888 <= s_203_4
        fn_state.gs_129888 = s_203_4;
        // N s_203_6: jump b155
        return block_155(state, tracer, fn_state);
    }
    fn block_204<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_204_0: const #424u : u32
        let s_204_0: u32 = 424;
        // D s_204_1: read-reg s_204_0:u8
        let s_204_1: u8 = {
            let value = state.read_register::<u8>(s_204_0 as isize);
            tracer.read_register(s_204_0 as isize, value);
            value
        };
        // C s_204_2: const #2u : u8
        let s_204_2: u8 = 2;
        // D s_204_3: cmp-lt s_204_1 s_204_2
        let s_204_3: bool = ((s_204_1) < (s_204_2));
        // D s_204_4: not s_204_3
        let s_204_4: bool = !s_204_3;
        // N s_204_5: branch s_204_4 b207 b205
        if s_204_4 {
            return block_207(state, tracer, fn_state);
        } else {
            return block_205(state, tracer, fn_state);
        };
    }
    fn block_205<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_205_0: read-var __SCR_EL3_FGTEn:u8
        let s_205_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_205_1: cast zx s_205_0 -> bv
        let s_205_1: Bits = Bits::new(s_205_0 as u128, 1u16);
        // C s_205_2: const #1u : u8
        let s_205_2: bool = true;
        // C s_205_3: cast zx s_205_2 -> bv
        let s_205_3: Bits = Bits::new(s_205_2 as u128, 1u16);
        // D s_205_4: cmp-eq s_205_1 s_205_3
        let s_205_4: bool = ((s_205_1) == (s_205_3));
        // D s_205_5: write-var gs#129886 <= s_205_4
        fn_state.gs_129886 = s_205_4;
        // N s_205_6: jump b206
        return block_206(state, tracer, fn_state);
    }
    fn block_206<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_206_0: read-var gs#129886:u8
        let s_206_0: bool = fn_state.gs_129886;
        // D s_206_1: write-var gs#129887 <= s_206_0
        fn_state.gs_129887 = s_206_0;
        // N s_206_2: jump b153
        return block_153(state, tracer, fn_state);
    }
    fn block_207<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_207_0: const #1u : u8
        let s_207_0: bool = true;
        // D s_207_1: write-var gs#129886 <= s_207_0
        fn_state.gs_129886 = s_207_0;
        // N s_207_2: jump b206
        return block_206(state, tracer, fn_state);
    }
    fn block_208<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_208_0: const #146u : u32
        let s_208_0: u32 = 146;
        // S s_208_1: call IsFeatureImplemented(s_208_0)
        let s_208_1: bool = IsFeatureImplemented(state, tracer, s_208_0);
        // D s_208_2: write-var gs#129885 <= s_208_1
        fn_state.gs_129885 = s_208_1;
        // N s_208_3: jump b151
        return block_151(state, tracer, fn_state);
    }
    fn block_209<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_209_0: const #102552u : u32
        let s_209_0: u32 = 102552;
        // D s_209_1: read-reg s_209_0:struct
        let s_209_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_209_0 as isize);
            tracer.read_register(s_209_0 as isize, value);
            value
        };
        // D s_209_2: call _get_HCR_EL2_Type_E2H(s_209_1)
        let s_209_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_209_1);
        // C s_209_3: const #102552u : u32
        let s_209_3: u32 = 102552;
        // D s_209_4: read-reg s_209_3:struct
        let s_209_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_209_3 as isize);
            tracer.read_register(s_209_3 as isize, value);
            value
        };
        // D s_209_5: call _get_HCR_EL2_Type_TGE(s_209_4)
        let s_209_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_209_4);
        // D s_209_6: cast zx s_209_2 -> bv
        let s_209_6: Bits = Bits::new(s_209_2 as u128, 1u16);
        // D s_209_7: cast zx s_209_5 -> bv
        let s_209_7: Bits = Bits::new(s_209_5 as u128, 1u16);
        // D s_209_8: cast reint s_209_6 -> u128
        let s_209_8: u128 = (s_209_6.value() as u128);
        // D s_209_9: size-of s_209_6
        let s_209_9: u16 = s_209_6.length();
        // D s_209_10: cast reint s_209_7 -> u128
        let s_209_10: u128 = (s_209_7.value() as u128);
        // D s_209_11: size-of s_209_7
        let s_209_11: u16 = s_209_7.length();
        // D s_209_12: lsl s_209_8 s_209_11
        let s_209_12: u128 = s_209_8 << s_209_11;
        // D s_209_13: or s_209_12 s_209_10
        let s_209_13: u128 = ((s_209_12) | (s_209_10));
        // D s_209_14: add s_209_9 s_209_11
        let s_209_14: u16 = (s_209_9 + s_209_11);
        // D s_209_15: create-bits s_209_13 s_209_14
        let s_209_15: Bits = Bits::new(s_209_13, s_209_14);
        // D s_209_16: cast reint s_209_15 -> u8
        let s_209_16: u8 = (s_209_15.value() as u8);
        // D s_209_17: cast zx s_209_16 -> bv
        let s_209_17: Bits = Bits::new(s_209_16 as u128, 2u16);
        // C s_209_18: const #3u : u8
        let s_209_18: u8 = 3;
        // C s_209_19: cast zx s_209_18 -> bv
        let s_209_19: Bits = Bits::new(s_209_18 as u128, 2u16);
        // D s_209_20: cmp-ne s_209_17 s_209_19
        let s_209_20: bool = ((s_209_17) != (s_209_19));
        // D s_209_21: write-var gs#129884 <= s_209_20
        fn_state.gs_129884 = s_209_20;
        // N s_209_22: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_210<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_210_0: const #440u : u32
        let s_210_0: u32 = 440;
        // D s_210_1: read-reg s_210_0:u8
        let s_210_1: u8 = {
            let value = state.read_register::<u8>(s_210_0 as isize);
            tracer.read_register(s_210_0 as isize, value);
            value
        };
        // D s_210_2: call ELUsingAArch32(s_210_1)
        let s_210_2: bool = ELUsingAArch32(state, tracer, s_210_1);
        // D s_210_3: not s_210_2
        let s_210_3: bool = !s_210_2;
        // D s_210_4: write-var gs#129883 <= s_210_3
        fn_state.gs_129883 = s_210_3;
        // N s_210_5: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_211<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_211_0: const #3u : u8
        let s_211_0: u8 = 3;
        // C s_211_1: cast zx s_211_0 -> bv
        let s_211_1: Bits = Bits::new(s_211_0 as u128, 8u16);
        // C s_211_2: cast zx s_211_1 -> i
        let s_211_2: i128 = (s_211_1.value() as i128);
        // C s_211_3: cast reint s_211_2 -> i64
        let s_211_3: i64 = (s_211_2 as i64);
        // C s_211_4: cast zx s_211_3 -> i
        let s_211_4: i128 = (i128::try_from(s_211_3).unwrap());
        // S s_211_5: call AArch32_TakeHypTrapException(s_211_4)
        let s_211_5: () = AArch32_TakeHypTrapException(state, tracer, s_211_4);
        // N s_211_6: return
        return;
    }
    fn block_212<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_212_0: read-var __HSTR_T9:u8
        let s_212_0: bool = fn_state.u__HSTR_T9;
        // D s_212_1: cast zx s_212_0 -> bv
        let s_212_1: Bits = Bits::new(s_212_0 as u128, 1u16);
        // C s_212_2: const #1u : u8
        let s_212_2: bool = true;
        // C s_212_3: cast zx s_212_2 -> bv
        let s_212_3: Bits = Bits::new(s_212_2 as u128, 1u16);
        // D s_212_4: cmp-eq s_212_1 s_212_3
        let s_212_4: bool = ((s_212_1) == (s_212_3));
        // D s_212_5: write-var gs#129882 <= s_212_4
        fn_state.gs_129882 = s_212_4;
        // N s_212_6: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_213<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_213_0: const #432u : u32
        let s_213_0: u32 = 432;
        // D s_213_1: read-reg s_213_0:u8
        let s_213_1: u8 = {
            let value = state.read_register::<u8>(s_213_0 as isize);
            tracer.read_register(s_213_0 as isize, value);
            value
        };
        // D s_213_2: call ELUsingAArch32(s_213_1)
        let s_213_2: bool = ELUsingAArch32(state, tracer, s_213_1);
        // D s_213_3: write-var gs#129881 <= s_213_2
        fn_state.gs_129881 = s_213_2;
        // N s_213_4: jump b142
        return block_142(state, tracer, fn_state);
    }
    fn block_214<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_214_0: const #3u : u8
        let s_214_0: u8 = 3;
        // C s_214_1: cast zx s_214_0 -> bv
        let s_214_1: Bits = Bits::new(s_214_0 as u128, 8u16);
        // C s_214_2: cast zx s_214_1 -> i
        let s_214_2: i128 = (s_214_1.value() as i128);
        // C s_214_3: cast reint s_214_2 -> i64
        let s_214_3: i64 = (s_214_2 as i64);
        // C s_214_4: cast zx s_214_3 -> i
        let s_214_4: i128 = (i128::try_from(s_214_3).unwrap());
        // C s_214_5: const #432u : u32
        let s_214_5: u32 = 432;
        // D s_214_6: read-reg s_214_5:u8
        let s_214_6: u8 = {
            let value = state.read_register::<u8>(s_214_5 as isize);
            tracer.read_register(s_214_5 as isize, value);
            value
        };
        // D s_214_7: call AArch64_AArch32SystemAccessTrap(s_214_6, s_214_4)
        let s_214_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_214_6,
            s_214_4,
        );
        // N s_214_8: return
        return;
    }
    fn block_215<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_215_0: read-var __HSTR_EL2_T9:u8
        let s_215_0: bool = fn_state.u__HSTR_EL2_T9;
        // D s_215_1: cast zx s_215_0 -> bv
        let s_215_1: Bits = Bits::new(s_215_0 as u128, 1u16);
        // C s_215_2: const #1u : u8
        let s_215_2: bool = true;
        // C s_215_3: cast zx s_215_2 -> bv
        let s_215_3: Bits = Bits::new(s_215_2 as u128, 1u16);
        // D s_215_4: cmp-eq s_215_1 s_215_3
        let s_215_4: bool = ((s_215_1) == (s_215_3));
        // D s_215_5: write-var gs#129880 <= s_215_4
        fn_state.gs_129880 = s_215_4;
        // N s_215_6: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_216<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_216_0: const #102552u : u32
        let s_216_0: u32 = 102552;
        // D s_216_1: read-reg s_216_0:struct
        let s_216_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_216_0 as isize);
            tracer.read_register(s_216_0 as isize, value);
            value
        };
        // D s_216_2: call _get_HCR_EL2_Type_E2H(s_216_1)
        let s_216_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_216_1);
        // C s_216_3: const #102552u : u32
        let s_216_3: u32 = 102552;
        // D s_216_4: read-reg s_216_3:struct
        let s_216_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_216_3 as isize);
            tracer.read_register(s_216_3 as isize, value);
            value
        };
        // D s_216_5: call _get_HCR_EL2_Type_TGE(s_216_4)
        let s_216_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_216_4);
        // D s_216_6: cast zx s_216_2 -> bv
        let s_216_6: Bits = Bits::new(s_216_2 as u128, 1u16);
        // D s_216_7: cast zx s_216_5 -> bv
        let s_216_7: Bits = Bits::new(s_216_5 as u128, 1u16);
        // D s_216_8: cast reint s_216_6 -> u128
        let s_216_8: u128 = (s_216_6.value() as u128);
        // D s_216_9: size-of s_216_6
        let s_216_9: u16 = s_216_6.length();
        // D s_216_10: cast reint s_216_7 -> u128
        let s_216_10: u128 = (s_216_7.value() as u128);
        // D s_216_11: size-of s_216_7
        let s_216_11: u16 = s_216_7.length();
        // D s_216_12: lsl s_216_8 s_216_11
        let s_216_12: u128 = s_216_8 << s_216_11;
        // D s_216_13: or s_216_12 s_216_10
        let s_216_13: u128 = ((s_216_12) | (s_216_10));
        // D s_216_14: add s_216_9 s_216_11
        let s_216_14: u16 = (s_216_9 + s_216_11);
        // D s_216_15: create-bits s_216_13 s_216_14
        let s_216_15: Bits = Bits::new(s_216_13, s_216_14);
        // D s_216_16: cast reint s_216_15 -> u8
        let s_216_16: u8 = (s_216_15.value() as u8);
        // D s_216_17: cast zx s_216_16 -> bv
        let s_216_17: Bits = Bits::new(s_216_16 as u128, 2u16);
        // C s_216_18: const #3u : u8
        let s_216_18: u8 = 3;
        // C s_216_19: cast zx s_216_18 -> bv
        let s_216_19: Bits = Bits::new(s_216_18 as u128, 2u16);
        // D s_216_20: cmp-ne s_216_17 s_216_19
        let s_216_20: bool = ((s_216_17) != (s_216_19));
        // D s_216_21: write-var gs#129879 <= s_216_20
        fn_state.gs_129879 = s_216_20;
        // N s_216_22: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_217<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_217_0: const #432u : u32
        let s_217_0: u32 = 432;
        // D s_217_1: read-reg s_217_0:u8
        let s_217_1: u8 = {
            let value = state.read_register::<u8>(s_217_0 as isize);
            tracer.read_register(s_217_0 as isize, value);
            value
        };
        // D s_217_2: call ELUsingAArch32(s_217_1)
        let s_217_2: bool = ELUsingAArch32(state, tracer, s_217_1);
        // D s_217_3: not s_217_2
        let s_217_3: bool = !s_217_2;
        // D s_217_4: write-var gs#129878 <= s_217_3
        fn_state.gs_129878 = s_217_3;
        // N s_217_5: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_218<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_218_0: const #() : ()
        let s_218_0: () = ();
        // S s_218_1: call EL2Enabled(s_218_0)
        let s_218_1: bool = EL2Enabled(state, tracer, s_218_0);
        // N s_218_2: branch s_218_1 b234 b219
        if s_218_1 {
            return block_234(state, tracer, fn_state);
        } else {
            return block_219(state, tracer, fn_state);
        };
    }
    fn block_219<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_219_0: const #0u : u8
        let s_219_0: bool = false;
        // D s_219_1: write-var gs#129900 <= s_219_0
        fn_state.gs_129900 = s_219_0;
        // N s_219_2: jump b220
        return block_220(state, tracer, fn_state);
    }
    fn block_220<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_220_0: read-var gs#129900:u8
        let s_220_0: bool = fn_state.gs_129900;
        // N s_220_1: branch s_220_0 b233 b221
        if s_220_0 {
            return block_233(state, tracer, fn_state);
        } else {
            return block_221(state, tracer, fn_state);
        };
    }
    fn block_221<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_221_0: const #0u : u8
        let s_221_0: bool = false;
        // D s_221_1: write-var gs#129901 <= s_221_0
        fn_state.gs_129901 = s_221_0;
        // N s_221_2: jump b222
        return block_222(state, tracer, fn_state);
    }
    fn block_222<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_222_0: read-var gs#129901:u8
        let s_222_0: bool = fn_state.gs_129901;
        // N s_222_1: branch s_222_0 b232 b223
        if s_222_0 {
            return block_232(state, tracer, fn_state);
        } else {
            return block_223(state, tracer, fn_state);
        };
    }
    fn block_223<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_223_0: const #() : ()
        let s_223_0: () = ();
        // S s_223_1: call EL2Enabled(s_223_0)
        let s_223_1: bool = EL2Enabled(state, tracer, s_223_0);
        // N s_223_2: branch s_223_1 b231 b224
        if s_223_1 {
            return block_231(state, tracer, fn_state);
        } else {
            return block_224(state, tracer, fn_state);
        };
    }
    fn block_224<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_224_0: const #0u : u8
        let s_224_0: bool = false;
        // D s_224_1: write-var gs#129902 <= s_224_0
        fn_state.gs_129902 = s_224_0;
        // N s_224_2: jump b225
        return block_225(state, tracer, fn_state);
    }
    fn block_225<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_225_0: read-var gs#129902:u8
        let s_225_0: bool = fn_state.gs_129902;
        // N s_225_1: branch s_225_0 b230 b226
        if s_225_0 {
            return block_230(state, tracer, fn_state);
        } else {
            return block_226(state, tracer, fn_state);
        };
    }
    fn block_226<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_226_0: const #0u : u8
        let s_226_0: bool = false;
        // D s_226_1: write-var gs#129903 <= s_226_0
        fn_state.gs_129903 = s_226_0;
        // N s_226_2: jump b227
        return block_227(state, tracer, fn_state);
    }
    fn block_227<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_227_0: read-var gs#129903:u8
        let s_227_0: bool = fn_state.gs_129903;
        // N s_227_1: branch s_227_0 b229 b228
        if s_227_0 {
            return block_229(state, tracer, fn_state);
        } else {
            return block_228(state, tracer, fn_state);
        };
    }
    fn block_228<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_228_0: panic
        panic!("{:?}", ());
        // N s_228_1: return
        return;
    }
    fn block_229<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_229_0: const #0u : u8
        let s_229_0: u8 = 0;
        // C s_229_1: cast zx s_229_0 -> bv
        let s_229_1: Bits = Bits::new(s_229_0 as u128, 8u16);
        // C s_229_2: cast zx s_229_1 -> i
        let s_229_2: i128 = (s_229_1.value() as i128);
        // C s_229_3: cast reint s_229_2 -> i64
        let s_229_3: i64 = (s_229_2 as i64);
        // C s_229_4: cast zx s_229_3 -> i
        let s_229_4: i128 = (i128::try_from(s_229_3).unwrap());
        // S s_229_5: call AArch32_TakeHypTrapException(s_229_4)
        let s_229_5: () = AArch32_TakeHypTrapException(state, tracer, s_229_4);
        // N s_229_6: return
        return;
    }
    fn block_230<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_230_0: read-var __HCR_TGE:u8
        let s_230_0: bool = fn_state.u__HCR_TGE;
        // D s_230_1: cast zx s_230_0 -> bv
        let s_230_1: Bits = Bits::new(s_230_0 as u128, 1u16);
        // C s_230_2: const #1u : u8
        let s_230_2: bool = true;
        // C s_230_3: cast zx s_230_2 -> bv
        let s_230_3: Bits = Bits::new(s_230_2 as u128, 1u16);
        // D s_230_4: cmp-eq s_230_1 s_230_3
        let s_230_4: bool = ((s_230_1) == (s_230_3));
        // D s_230_5: write-var gs#129903 <= s_230_4
        fn_state.gs_129903 = s_230_4;
        // N s_230_6: jump b227
        return block_227(state, tracer, fn_state);
    }
    fn block_231<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_231_0: const #432u : u32
        let s_231_0: u32 = 432;
        // D s_231_1: read-reg s_231_0:u8
        let s_231_1: u8 = {
            let value = state.read_register::<u8>(s_231_0 as isize);
            tracer.read_register(s_231_0 as isize, value);
            value
        };
        // D s_231_2: call ELUsingAArch32(s_231_1)
        let s_231_2: bool = ELUsingAArch32(state, tracer, s_231_1);
        // D s_231_3: write-var gs#129902 <= s_231_2
        fn_state.gs_129902 = s_231_2;
        // N s_231_4: jump b225
        return block_225(state, tracer, fn_state);
    }
    fn block_232<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_232_0: const #3u : u8
        let s_232_0: u8 = 3;
        // C s_232_1: cast zx s_232_0 -> bv
        let s_232_1: Bits = Bits::new(s_232_0 as u128, 8u16);
        // C s_232_2: cast zx s_232_1 -> i
        let s_232_2: i128 = (s_232_1.value() as i128);
        // C s_232_3: cast reint s_232_2 -> i64
        let s_232_3: i64 = (s_232_2 as i64);
        // C s_232_4: cast zx s_232_3 -> i
        let s_232_4: i128 = (i128::try_from(s_232_3).unwrap());
        // C s_232_5: const #432u : u32
        let s_232_5: u32 = 432;
        // D s_232_6: read-reg s_232_5:u8
        let s_232_6: u8 = {
            let value = state.read_register::<u8>(s_232_5 as isize);
            tracer.read_register(s_232_5 as isize, value);
            value
        };
        // D s_232_7: call AArch64_AArch32SystemAccessTrap(s_232_6, s_232_4)
        let s_232_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_232_6,
            s_232_4,
        );
        // N s_232_8: return
        return;
    }
    fn block_233<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_233_0: read-var __HCR_EL2_TGE:u8
        let s_233_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_233_1: cast zx s_233_0 -> bv
        let s_233_1: Bits = Bits::new(s_233_0 as u128, 1u16);
        // C s_233_2: const #1u : u8
        let s_233_2: bool = true;
        // C s_233_3: cast zx s_233_2 -> bv
        let s_233_3: Bits = Bits::new(s_233_2 as u128, 1u16);
        // D s_233_4: cmp-eq s_233_1 s_233_3
        let s_233_4: bool = ((s_233_1) == (s_233_3));
        // D s_233_5: write-var gs#129901 <= s_233_4
        fn_state.gs_129901 = s_233_4;
        // N s_233_6: jump b222
        return block_222(state, tracer, fn_state);
    }
    fn block_234<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_234_0: const #432u : u32
        let s_234_0: u32 = 432;
        // D s_234_1: read-reg s_234_0:u8
        let s_234_1: u8 = {
            let value = state.read_register::<u8>(s_234_0 as isize);
            tracer.read_register(s_234_0 as isize, value);
            value
        };
        // D s_234_2: call ELUsingAArch32(s_234_1)
        let s_234_2: bool = ELUsingAArch32(state, tracer, s_234_1);
        // D s_234_3: not s_234_2
        let s_234_3: bool = !s_234_2;
        // D s_234_4: write-var gs#129900 <= s_234_3
        fn_state.gs_129900 = s_234_3;
        // N s_234_5: jump b220
        return block_220(state, tracer, fn_state);
    }
    fn block_235<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_235_0: read-var __PMUSERENR_EN:u8
        let s_235_0: bool = fn_state.u__PMUSERENR_EN;
        // D s_235_1: cast zx s_235_0 -> bv
        let s_235_1: Bits = Bits::new(s_235_0 as u128, 1u16);
        // C s_235_2: const #0u : u8
        let s_235_2: bool = false;
        // C s_235_3: cast zx s_235_2 -> bv
        let s_235_3: Bits = Bits::new(s_235_2 as u128, 1u16);
        // D s_235_4: cmp-eq s_235_1 s_235_3
        let s_235_4: bool = ((s_235_1) == (s_235_3));
        // D s_235_5: write-var gs#129877 <= s_235_4
        fn_state.gs_129877 = s_235_4;
        // N s_235_6: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_236<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_236_0: const #() : ()
        let s_236_0: () = ();
        // S s_236_1: call EL2Enabled(s_236_0)
        let s_236_1: bool = EL2Enabled(state, tracer, s_236_0);
        // N s_236_2: branch s_236_1 b244 b237
        if s_236_1 {
            return block_244(state, tracer, fn_state);
        } else {
            return block_237(state, tracer, fn_state);
        };
    }
    fn block_237<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_237_0: const #0u : u8
        let s_237_0: bool = false;
        // D s_237_1: write-var gs#129904 <= s_237_0
        fn_state.gs_129904 = s_237_0;
        // N s_237_2: jump b238
        return block_238(state, tracer, fn_state);
    }
    fn block_238<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_238_0: read-var gs#129904:u8
        let s_238_0: bool = fn_state.gs_129904;
        // N s_238_1: branch s_238_0 b243 b239
        if s_238_0 {
            return block_243(state, tracer, fn_state);
        } else {
            return block_239(state, tracer, fn_state);
        };
    }
    fn block_239<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_239_0: const #0u : u8
        let s_239_0: bool = false;
        // D s_239_1: write-var gs#129905 <= s_239_0
        fn_state.gs_129905 = s_239_0;
        // N s_239_2: jump b240
        return block_240(state, tracer, fn_state);
    }
    fn block_240<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_240_0: read-var gs#129905:u8
        let s_240_0: bool = fn_state.gs_129905;
        // N s_240_1: branch s_240_0 b242 b241
        if s_240_0 {
            return block_242(state, tracer, fn_state);
        } else {
            return block_241(state, tracer, fn_state);
        };
    }
    fn block_241<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_241_0: const #3u : u8
        let s_241_0: u8 = 3;
        // C s_241_1: cast zx s_241_0 -> bv
        let s_241_1: Bits = Bits::new(s_241_0 as u128, 8u16);
        // C s_241_2: cast zx s_241_1 -> i
        let s_241_2: i128 = (s_241_1.value() as i128);
        // C s_241_3: cast reint s_241_2 -> i64
        let s_241_3: i64 = (s_241_2 as i64);
        // C s_241_4: cast zx s_241_3 -> i
        let s_241_4: i128 = (i128::try_from(s_241_3).unwrap());
        // C s_241_5: const #440u : u32
        let s_241_5: u32 = 440;
        // D s_241_6: read-reg s_241_5:u8
        let s_241_6: u8 = {
            let value = state.read_register::<u8>(s_241_5 as isize);
            tracer.read_register(s_241_5 as isize, value);
            value
        };
        // D s_241_7: call AArch64_AArch32SystemAccessTrap(s_241_6, s_241_4)
        let s_241_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_241_6,
            s_241_4,
        );
        // N s_241_8: return
        return;
    }
    fn block_242<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_242_0: const #3u : u8
        let s_242_0: u8 = 3;
        // C s_242_1: cast zx s_242_0 -> bv
        let s_242_1: Bits = Bits::new(s_242_0 as u128, 8u16);
        // C s_242_2: cast zx s_242_1 -> i
        let s_242_2: i128 = (s_242_1.value() as i128);
        // C s_242_3: cast reint s_242_2 -> i64
        let s_242_3: i64 = (s_242_2 as i64);
        // C s_242_4: cast zx s_242_3 -> i
        let s_242_4: i128 = (i128::try_from(s_242_3).unwrap());
        // C s_242_5: const #432u : u32
        let s_242_5: u32 = 432;
        // D s_242_6: read-reg s_242_5:u8
        let s_242_6: u8 = {
            let value = state.read_register::<u8>(s_242_5 as isize);
            tracer.read_register(s_242_5 as isize, value);
            value
        };
        // D s_242_7: call AArch64_AArch32SystemAccessTrap(s_242_6, s_242_4)
        let s_242_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_242_6,
            s_242_4,
        );
        // N s_242_8: return
        return;
    }
    fn block_243<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_243_0: read-var __HCR_EL2_TGE:u8
        let s_243_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_243_1: cast zx s_243_0 -> bv
        let s_243_1: Bits = Bits::new(s_243_0 as u128, 1u16);
        // C s_243_2: const #1u : u8
        let s_243_2: bool = true;
        // C s_243_3: cast zx s_243_2 -> bv
        let s_243_3: Bits = Bits::new(s_243_2 as u128, 1u16);
        // D s_243_4: cmp-eq s_243_1 s_243_3
        let s_243_4: bool = ((s_243_1) == (s_243_3));
        // D s_243_5: write-var gs#129905 <= s_243_4
        fn_state.gs_129905 = s_243_4;
        // N s_243_6: jump b240
        return block_240(state, tracer, fn_state);
    }
    fn block_244<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_244_0: const #432u : u32
        let s_244_0: u32 = 432;
        // D s_244_1: read-reg s_244_0:u8
        let s_244_1: u8 = {
            let value = state.read_register::<u8>(s_244_0 as isize);
            tracer.read_register(s_244_0 as isize, value);
            value
        };
        // D s_244_2: call ELUsingAArch32(s_244_1)
        let s_244_2: bool = ELUsingAArch32(state, tracer, s_244_1);
        // D s_244_3: not s_244_2
        let s_244_3: bool = !s_244_2;
        // D s_244_4: write-var gs#129904 <= s_244_3
        fn_state.gs_129904 = s_244_3;
        // N s_244_5: jump b238
        return block_238(state, tracer, fn_state);
    }
    fn block_245<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_245_0: const #204u : u32
        let s_245_0: u32 = 204;
        // S s_245_1: call IsFeatureImplemented(s_245_0)
        let s_245_1: bool = IsFeatureImplemented(state, tracer, s_245_0);
        // N s_245_2: branch s_245_1 b254 b246
        if s_245_1 {
            return block_254(state, tracer, fn_state);
        } else {
            return block_246(state, tracer, fn_state);
        };
    }
    fn block_246<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_246_0: const #0u : u8
        let s_246_0: bool = false;
        // D s_246_1: write-var gs#129873 <= s_246_0
        fn_state.gs_129873 = s_246_0;
        // N s_246_2: jump b247
        return block_247(state, tracer, fn_state);
    }
    fn block_247<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_247_0: read-var gs#129873:u8
        let s_247_0: bool = fn_state.gs_129873;
        // N s_247_1: branch s_247_0 b253 b248
        if s_247_0 {
            return block_253(state, tracer, fn_state);
        } else {
            return block_248(state, tracer, fn_state);
        };
    }
    fn block_248<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_248_0: const #204u : u32
        let s_248_0: u32 = 204;
        // S s_248_1: call IsFeatureImplemented(s_248_0)
        let s_248_1: bool = IsFeatureImplemented(state, tracer, s_248_0);
        // S s_248_2: not s_248_1
        let s_248_2: bool = !s_248_1;
        // N s_248_3: branch s_248_2 b252 b249
        if s_248_2 {
            return block_252(state, tracer, fn_state);
        } else {
            return block_249(state, tracer, fn_state);
        };
    }
    fn block_249<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_249_0: const #0u : u8
        let s_249_0: bool = false;
        // D s_249_1: write-var gs#129874 <= s_249_0
        fn_state.gs_129874 = s_249_0;
        // N s_249_2: jump b250
        return block_250(state, tracer, fn_state);
    }
    fn block_250<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_250_0: read-var gs#129874:u8
        let s_250_0: bool = fn_state.gs_129874;
        // D s_250_1: write-var gs#129875 <= s_250_0
        fn_state.gs_129875 = s_250_0;
        // N s_250_2: jump b251
        return block_251(state, tracer, fn_state);
    }
    fn block_251<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_251_0: read-var gs#129875:u8
        let s_251_0: bool = fn_state.gs_129875;
        // D s_251_1: write-var gs#129876 <= s_251_0
        fn_state.gs_129876 = s_251_0;
        // N s_251_2: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_252<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_252_0: read-var __PMUSERENR_EL0_EN:u8
        let s_252_0: bool = fn_state.u__PMUSERENR_EL0_EN;
        // D s_252_1: cast zx s_252_0 -> bv
        let s_252_1: Bits = Bits::new(s_252_0 as u128, 1u16);
        // C s_252_2: const #0u : u8
        let s_252_2: bool = false;
        // C s_252_3: cast zx s_252_2 -> bv
        let s_252_3: Bits = Bits::new(s_252_2 as u128, 1u16);
        // D s_252_4: cmp-eq s_252_1 s_252_3
        let s_252_4: bool = ((s_252_1) == (s_252_3));
        // D s_252_5: write-var gs#129874 <= s_252_4
        fn_state.gs_129874 = s_252_4;
        // N s_252_6: jump b250
        return block_250(state, tracer, fn_state);
    }
    fn block_253<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_253_0: const #1u : u8
        let s_253_0: bool = true;
        // D s_253_1: write-var gs#129875 <= s_253_0
        fn_state.gs_129875 = s_253_0;
        // N s_253_2: jump b251
        return block_251(state, tracer, fn_state);
    }
    fn block_254<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_254_0: const #102624u : u32
        let s_254_0: u32 = 102624;
        // D s_254_1: read-reg s_254_0:struct
        let s_254_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_254_0 as isize);
            tracer.read_register(s_254_0 as isize, value);
            value
        };
        // D s_254_2: call _get_PMUSERENR_EL0_Type_UEN(s_254_1)
        let s_254_2: bool = u_get_PMUSERENR_EL0_Type_UEN(state, tracer, s_254_1);
        // C s_254_3: const #102624u : u32
        let s_254_3: u32 = 102624;
        // D s_254_4: read-reg s_254_3:struct
        let s_254_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_254_3 as isize);
            tracer.read_register(s_254_3 as isize, value);
            value
        };
        // D s_254_5: call _get_PMUSERENR_EL0_Type_EN(s_254_4)
        let s_254_5: bool = u_get_PMUSERENR_EL0_Type_EN(state, tracer, s_254_4);
        // D s_254_6: cast zx s_254_2 -> bv
        let s_254_6: Bits = Bits::new(s_254_2 as u128, 1u16);
        // D s_254_7: cast zx s_254_5 -> bv
        let s_254_7: Bits = Bits::new(s_254_5 as u128, 1u16);
        // D s_254_8: cast reint s_254_6 -> u128
        let s_254_8: u128 = (s_254_6.value() as u128);
        // D s_254_9: size-of s_254_6
        let s_254_9: u16 = s_254_6.length();
        // D s_254_10: cast reint s_254_7 -> u128
        let s_254_10: u128 = (s_254_7.value() as u128);
        // D s_254_11: size-of s_254_7
        let s_254_11: u16 = s_254_7.length();
        // D s_254_12: lsl s_254_8 s_254_11
        let s_254_12: u128 = s_254_8 << s_254_11;
        // D s_254_13: or s_254_12 s_254_10
        let s_254_13: u128 = ((s_254_12) | (s_254_10));
        // D s_254_14: add s_254_9 s_254_11
        let s_254_14: u16 = (s_254_9 + s_254_11);
        // D s_254_15: create-bits s_254_13 s_254_14
        let s_254_15: Bits = Bits::new(s_254_13, s_254_14);
        // D s_254_16: cast reint s_254_15 -> u8
        let s_254_16: u8 = (s_254_15.value() as u8);
        // D s_254_17: cast zx s_254_16 -> bv
        let s_254_17: Bits = Bits::new(s_254_16 as u128, 2u16);
        // C s_254_18: const #1u : u8
        let s_254_18: u8 = 1;
        // C s_254_19: cast zx s_254_18 -> bv
        let s_254_19: Bits = Bits::new(s_254_18 as u128, 2u16);
        // D s_254_20: cmp-ne s_254_17 s_254_19
        let s_254_20: bool = ((s_254_17) != (s_254_19));
        // D s_254_21: write-var gs#129873 <= s_254_20
        fn_state.gs_129873 = s_254_20;
        // N s_254_22: jump b247
        return block_247(state, tracer, fn_state);
    }
    fn block_255<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_255_0: panic
        panic!("{:?}", ());
        // N s_255_1: return
        return;
    }
    fn block_256<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_256_0: read-var __MDCR_EL3_TPM:u8
        let s_256_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_256_1: cast zx s_256_0 -> bv
        let s_256_1: Bits = Bits::new(s_256_0 as u128, 1u16);
        // C s_256_2: const #1u : u8
        let s_256_2: bool = true;
        // C s_256_3: cast zx s_256_2 -> bv
        let s_256_3: Bits = Bits::new(s_256_2 as u128, 1u16);
        // D s_256_4: cmp-eq s_256_1 s_256_3
        let s_256_4: bool = ((s_256_1) == (s_256_3));
        // D s_256_5: write-var gs#129872 <= s_256_4
        fn_state.gs_129872 = s_256_4;
        // N s_256_6: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_257<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_257_0: const #424u : u32
        let s_257_0: u32 = 424;
        // D s_257_1: read-reg s_257_0:u8
        let s_257_1: u8 = {
            let value = state.read_register::<u8>(s_257_0 as isize);
            tracer.read_register(s_257_0 as isize, value);
            value
        };
        // D s_257_2: call ELUsingAArch32(s_257_1)
        let s_257_2: bool = ELUsingAArch32(state, tracer, s_257_1);
        // D s_257_3: not s_257_2
        let s_257_3: bool = !s_257_2;
        // D s_257_4: write-var gs#129871 <= s_257_3
        fn_state.gs_129871 = s_257_3;
        // N s_257_5: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_258<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_258_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_258_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_258_1: call __IMPDEF_boolean(s_258_0)
        let s_258_1: bool = u__IMPDEF_boolean(state, tracer, s_258_0);
        // D s_258_2: write-var gs#129870 <= s_258_1
        fn_state.gs_129870 = s_258_1;
        // N s_258_3: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_259<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_259_0: const #() : ()
        let s_259_0: () = ();
        // S s_259_1: call EDSCR_read(s_259_0)
        let s_259_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_259_0);
        // S s_259_2: call _get_EDSCR_Type_SDD(s_259_1)
        let s_259_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_259_1);
        // S s_259_3: cast zx s_259_2 -> bv
        let s_259_3: Bits = Bits::new(s_259_2 as u128, 1u16);
        // C s_259_4: const #1u : u8
        let s_259_4: bool = true;
        // C s_259_5: cast zx s_259_4 -> bv
        let s_259_5: Bits = Bits::new(s_259_4 as u128, 1u16);
        // S s_259_6: cmp-eq s_259_3 s_259_5
        let s_259_6: bool = ((s_259_3) == (s_259_5));
        // D s_259_7: write-var gs#129869 <= s_259_6
        fn_state.gs_129869 = s_259_6;
        // N s_259_8: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_260<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_260_0: const #424u : u32
        let s_260_0: u32 = 424;
        // D s_260_1: read-reg s_260_0:u8
        let s_260_1: u8 = {
            let value = state.read_register::<u8>(s_260_0 as isize);
            tracer.read_register(s_260_0 as isize, value);
            value
        };
        // C s_260_2: const #2u : u8
        let s_260_2: u8 = 2;
        // D s_260_3: cmp-lt s_260_1 s_260_2
        let s_260_3: bool = ((s_260_1) < (s_260_2));
        // D s_260_4: write-var gs#129868 <= s_260_3
        fn_state.gs_129868 = s_260_3;
        // N s_260_5: jump b118
        return block_118(state, tracer, fn_state);
    }
}
