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
use Halted::*;
use u_get_HSTR_Type_T9::*;
use u_get_MDCR_EL3_Type_TPM::*;
use u_get_PMUSERENR_Type_EN::*;
use u_get_PMUSERENR_EL0_Type_TID::*;
use u_get_PMUSERENR_Type_TID::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_HSTR_EL2_Type_T9::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use u_get_MDCR_EL2_Type_TPM::*;
use u_get_PMUSERENR_EL0_Type_UEN::*;
use u_get_HCR_EL2_Type_E2H::*;
use PMUSERENR_read::*;
use HCR_read::*;
use HDCR_read::*;
use IsFeatureImplemented::*;
use u__IMPDEF_boolean::*;
use AArch64_AArch32SystemAccessTrap::*;
use HSTR_read::*;
use u_get_HCR_Type_TGE::*;
use u_get_HDCR_Type_TPM::*;
use u_get_HDFGRTR_EL2_Type_PMCEIDn_EL0::*;
use PMCEID1_read::*;
use R_set::*;
use ELUsingAArch32::*;
use u_get_PMUSERENR_EL0_Type_EN::*;
use EDSCR_read::*;
use AArch32_TakeHypTrapException::*;
use common::*;
pub fn PMCEID1_SysRegRead32_dc9c0fe15ac731b4<T: Tracer>(
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
        gs_112901: bool,
        gs_112869: bool,
        gs_112864: bool,
        gs_112870: bool,
        ga_184921: ProductType700c18a878c5601b,
        gs_112840: bool,
        gs_112868: bool,
        gs_112889: bool,
        gs_112856: bool,
        gs_112874: bool,
        gs_112879: bool,
        gs_112849: bool,
        gs_112855: bool,
        gs_112848: bool,
        ga_184988: ProductType700c18a878c5601b,
        u__MDCR_EL3_TPM: bool,
        gs_112880: bool,
        gs_112888: bool,
        gs_112892: bool,
        gs_112885: bool,
        gs_112882: bool,
        gs_112890: bool,
        ga_184962: ProductType700c18a878c5601b,
        gs_112878: bool,
        gs_112843: bool,
        gs_112897: bool,
        gs_112877: bool,
        gs_112894: bool,
        u__HCR_TGE: bool,
        gs_112861: bool,
        gs_112904: bool,
        gs_112851: bool,
        gs_112876: bool,
        gs_112896: bool,
        gs_112883: bool,
        u__PMUSERENR_TID: bool,
        gs_112866: bool,
        gs_112875: bool,
        gs_112852: bool,
        gs_112838: bool,
        gs_112845: bool,
        gs_112853: bool,
        gs_112891: bool,
        u__PSTATE_EL: u8,
        gs_112905: bool,
        gs_112893: bool,
        u__MDCR_EL2_TPM: bool,
        gs_112884: bool,
        gs_112841: bool,
        gs_112898: bool,
        u__PMUSERENR_EN: bool,
        gs_112863: bool,
        gs_112846: bool,
        gs_112899: bool,
        gs_112859: bool,
        u__HCR_EL2_TGE: bool,
        gs_112887: bool,
        gs_112860: bool,
        gs_112850: bool,
        gs_112839: bool,
        u__SCR_EL3_FGTEn: bool,
        gs_112900: bool,
        u__PMUSERENR_EL0_EN: bool,
        u__HSTR_T9: bool,
        gs_112895: bool,
        gs_112871: bool,
        gs_112842: bool,
        gs_112867: bool,
        u__HDCR_TPM: bool,
        u__HDFGRTR_EL2_PMCEIDn_EL0: bool,
        gs_112844: bool,
        gs_112857: bool,
        gs_112872: bool,
        gs_112886: bool,
        gs_112847: bool,
        gs_112902: bool,
        gs_112854: bool,
        gs_112903: bool,
        gs_112881: bool,
        gs_112873: bool,
        gs_112865: bool,
        ga_184985: ProductType700c18a878c5601b,
        u__PMUSERENR_EL0_TID: bool,
        gs_112862: bool,
        gs_112858: bool,
        u__HSTR_EL2_T9: bool,
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
        // C s_0_15: const #102624u : u32
        let s_0_15: u32 = 102624;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_PMUSERENR_EL0_Type_TID(s_0_16)
        let s_0_17: bool = u_get_PMUSERENR_EL0_Type_TID(state, tracer, s_0_16);
        // D s_0_18: write-var __PMUSERENR_EL0_TID <= s_0_17
        fn_state.u__PMUSERENR_EL0_TID = s_0_17;
        // C s_0_19: const #() : ()
        let s_0_19: () = ();
        // S s_0_20: call PMUSERENR_read(s_0_19)
        let s_0_20: ProductType700c18a878c5601b = PMUSERENR_read(state, tracer, s_0_19);
        // S s_0_21: call _get_PMUSERENR_Type_EN(s_0_20)
        let s_0_21: bool = u_get_PMUSERENR_Type_EN(state, tracer, s_0_20);
        // D s_0_22: write-var __PMUSERENR_EN <= s_0_21
        fn_state.u__PMUSERENR_EN = s_0_21;
        // C s_0_23: const #() : ()
        let s_0_23: () = ();
        // S s_0_24: call HCR_read(s_0_23)
        let s_0_24: ProductType700c18a878c5601b = HCR_read(state, tracer, s_0_23);
        // S s_0_25: call _get_HCR_Type_TGE(s_0_24)
        let s_0_25: bool = u_get_HCR_Type_TGE(state, tracer, s_0_24);
        // D s_0_26: write-var __HCR_TGE <= s_0_25
        fn_state.u__HCR_TGE = s_0_25;
        // C s_0_27: const #() : ()
        let s_0_27: () = ();
        // S s_0_28: call PMUSERENR_read(s_0_27)
        let s_0_28: ProductType700c18a878c5601b = PMUSERENR_read(state, tracer, s_0_27);
        // S s_0_29: call _get_PMUSERENR_Type_TID(s_0_28)
        let s_0_29: bool = u_get_PMUSERENR_Type_TID(state, tracer, s_0_28);
        // D s_0_30: write-var __PMUSERENR_TID <= s_0_29
        fn_state.u__PMUSERENR_TID = s_0_29;
        // C s_0_31: const #104936u : u32
        let s_0_31: u32 = 104936;
        // D s_0_32: read-reg s_0_31:struct
        let s_0_32: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_31 as isize);
            tracer.read_register(s_0_31 as isize, value);
            value
        };
        // D s_0_33: call _get_HSTR_EL2_Type_T9(s_0_32)
        let s_0_33: bool = u_get_HSTR_EL2_Type_T9(state, tracer, s_0_32);
        // D s_0_34: write-var __HSTR_EL2_T9 <= s_0_33
        fn_state.u__HSTR_EL2_T9 = s_0_33;
        // C s_0_35: const #() : ()
        let s_0_35: () = ();
        // S s_0_36: call HSTR_read(s_0_35)
        let s_0_36: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_35);
        // S s_0_37: call _get_HSTR_Type_T9(s_0_36)
        let s_0_37: bool = u_get_HSTR_Type_T9(state, tracer, s_0_36);
        // D s_0_38: write-var __HSTR_T9 <= s_0_37
        fn_state.u__HSTR_T9 = s_0_37;
        // C s_0_39: const #90704u : u32
        let s_0_39: u32 = 90704;
        // D s_0_40: read-reg s_0_39:struct
        let s_0_40: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_39 as isize);
            tracer.read_register(s_0_39 as isize, value);
            value
        };
        // D s_0_41: call _get_SCR_EL3_Type_FGTEn(s_0_40)
        let s_0_41: bool = u_get_SCR_EL3_Type_FGTEn(state, tracer, s_0_40);
        // D s_0_42: write-var __SCR_EL3_FGTEn <= s_0_41
        fn_state.u__SCR_EL3_FGTEn = s_0_41;
        // C s_0_43: const #19144u : u32
        let s_0_43: u32 = 19144;
        // D s_0_44: read-reg s_0_43:struct
        let s_0_44: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_43 as isize);
            tracer.read_register(s_0_43 as isize, value);
            value
        };
        // D s_0_45: call _get_HDFGRTR_EL2_Type_PMCEIDn_EL0(s_0_44)
        let s_0_45: bool = u_get_HDFGRTR_EL2_Type_PMCEIDn_EL0(state, tracer, s_0_44);
        // D s_0_46: write-var __HDFGRTR_EL2_PMCEIDn_EL0 <= s_0_45
        fn_state.u__HDFGRTR_EL2_PMCEIDn_EL0 = s_0_45;
        // C s_0_47: const #104880u : u32
        let s_0_47: u32 = 104880;
        // D s_0_48: read-reg s_0_47:struct
        let s_0_48: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_47 as isize);
            tracer.read_register(s_0_47 as isize, value);
            value
        };
        // D s_0_49: call _get_MDCR_EL2_Type_TPM(s_0_48)
        let s_0_49: bool = u_get_MDCR_EL2_Type_TPM(state, tracer, s_0_48);
        // D s_0_50: write-var __MDCR_EL2_TPM <= s_0_49
        fn_state.u__MDCR_EL2_TPM = s_0_49;
        // C s_0_51: const #() : ()
        let s_0_51: () = ();
        // S s_0_52: call HDCR_read(s_0_51)
        let s_0_52: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_0_51);
        // S s_0_53: call _get_HDCR_Type_TPM(s_0_52)
        let s_0_53: bool = u_get_HDCR_Type_TPM(state, tracer, s_0_52);
        // D s_0_54: write-var __HDCR_TPM <= s_0_53
        fn_state.u__HDCR_TPM = s_0_53;
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
        // N s_0_61: branch s_0_60 b100 b1
        if s_0_60 {
            return block_100(state, tracer, fn_state);
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
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call PMCEID1_read(s_5_0)
        let s_5_1: ProductType700c18a878c5601b = PMCEID1_read(state, tracer, s_5_0);
        // D s_5_2: write-var ga#184988 <= s_5_1
        fn_state.ga_184988 = s_5_1;
        // D s_5_3: read-var ga#184988.0:struct
        let s_5_3: u32 = fn_state.ga_184988._0;
        // D s_5_4: read-var t:i
        let s_5_4: i128 = fn_state.t;
        // D s_5_5: call R_set(s_5_4, s_5_3)
        let s_5_5: () = R_set(state, tracer, s_5_4, s_5_3);
        // N s_5_6: return
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
        // D s_7_1: write-var gs#112838 <= s_7_0
        fn_state.gs_112838 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#112838:u8
        let s_8_0: bool = fn_state.gs_112838;
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
        // D s_9_1: write-var gs#112839 <= s_9_0
        fn_state.gs_112839 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#112839:u8
        let s_10_0: bool = fn_state.gs_112839;
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
        // D s_11_1: write-var gs#112840 <= s_11_0
        fn_state.gs_112840 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#112840:u8
        let s_12_0: bool = fn_state.gs_112840;
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
        // D s_13_1: write-var gs#112841 <= s_13_0
        fn_state.gs_112841 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#112841:u8
        let s_14_0: bool = fn_state.gs_112841;
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
        // D s_15_1: write-var gs#112842 <= s_15_0
        fn_state.gs_112842 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#112842:u8
        let s_16_0: bool = fn_state.gs_112842;
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
        // D s_18_1: write-var gs#112843 <= s_18_0
        fn_state.gs_112843 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#112843:u8
        let s_19_0: bool = fn_state.gs_112843;
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
        // D s_20_1: write-var gs#112844 <= s_20_0
        fn_state.gs_112844 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#112844:u8
        let s_21_0: bool = fn_state.gs_112844;
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
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call PMCEID1_read(s_22_0)
        let s_22_1: ProductType700c18a878c5601b = PMCEID1_read(state, tracer, s_22_0);
        // D s_22_2: write-var ga#184985 <= s_22_1
        fn_state.ga_184985 = s_22_1;
        // D s_22_3: read-var ga#184985.0:struct
        let s_22_3: u32 = fn_state.ga_184985._0;
        // D s_22_4: read-var t:i
        let s_22_4: i128 = fn_state.t;
        // D s_22_5: call R_set(s_22_4, s_22_3)
        let s_22_5: () = R_set(state, tracer, s_22_4, s_22_3);
        // N s_22_6: return
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
        // D s_24_1: write-var gs#112845 <= s_24_0
        fn_state.gs_112845 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#112845:u8
        let s_25_0: bool = fn_state.gs_112845;
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
        // D s_28_7: write-var gs#112845 <= s_28_6
        fn_state.gs_112845 = s_28_6;
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
        // D s_29_5: write-var gs#112844 <= s_29_4
        fn_state.gs_112844 = s_29_4;
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
        // D s_30_4: write-var gs#112843 <= s_30_3
        fn_state.gs_112843 = s_30_3;
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
        // D s_32_5: write-var gs#112842 <= s_32_4
        fn_state.gs_112842 = s_32_4;
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
        // D s_33_4: write-var gs#112841 <= s_33_3
        fn_state.gs_112841 = s_33_3;
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
        // D s_34_2: write-var gs#112840 <= s_34_1
        fn_state.gs_112840 = s_34_1;
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
        // D s_35_7: write-var gs#112839 <= s_35_6
        fn_state.gs_112839 = s_35_6;
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
        // D s_36_4: write-var gs#112838 <= s_36_3
        fn_state.gs_112838 = s_36_3;
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
        // N s_37_2: branch s_37_1 b99 b38
        if s_37_1 {
            return block_99(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#112846 <= s_38_0
        fn_state.gs_112846 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#112846:u8
        let s_39_0: bool = fn_state.gs_112846;
        // N s_39_1: branch s_39_0 b98 b40
        if s_39_0 {
            return block_98(state, tracer, fn_state);
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
        // D s_40_1: write-var gs#112847 <= s_40_0
        fn_state.gs_112847 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#112847:u8
        let s_41_0: bool = fn_state.gs_112847;
        // N s_41_1: branch s_41_0 b97 b42
        if s_41_0 {
            return block_97(state, tracer, fn_state);
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
        // D s_42_1: write-var gs#112848 <= s_42_0
        fn_state.gs_112848 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#112848:u8
        let s_43_0: bool = fn_state.gs_112848;
        // N s_43_1: branch s_43_0 b96 b44
        if s_43_0 {
            return block_96(state, tracer, fn_state);
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
        // D s_44_1: write-var gs#112849 <= s_44_0
        fn_state.gs_112849 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#112849:u8
        let s_45_0: bool = fn_state.gs_112849;
        // N s_45_1: branch s_45_0 b95 b46
        if s_45_0 {
            return block_95(state, tracer, fn_state);
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
        // D s_46_1: write-var gs#112850 <= s_46_0
        fn_state.gs_112850 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#112850:u8
        let s_47_0: bool = fn_state.gs_112850;
        // N s_47_1: branch s_47_0 b94 b48
        if s_47_0 {
            return block_94(state, tracer, fn_state);
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
        // N s_48_2: branch s_48_1 b93 b49
        if s_48_1 {
            return block_93(state, tracer, fn_state);
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
        // D s_49_1: write-var gs#112851 <= s_49_0
        fn_state.gs_112851 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#112851:u8
        let s_50_0: bool = fn_state.gs_112851;
        // N s_50_1: branch s_50_0 b92 b51
        if s_50_0 {
            return block_92(state, tracer, fn_state);
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
        // D s_51_1: write-var gs#112852 <= s_51_0
        fn_state.gs_112852 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#112852:u8
        let s_52_0: bool = fn_state.gs_112852;
        // N s_52_1: branch s_52_0 b91 b53
        if s_52_0 {
            return block_91(state, tracer, fn_state);
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
        // N s_53_2: branch s_53_1 b90 b54
        if s_53_1 {
            return block_90(state, tracer, fn_state);
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
        // D s_54_1: write-var gs#112853 <= s_54_0
        fn_state.gs_112853 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#112853:u8
        let s_55_0: bool = fn_state.gs_112853;
        // N s_55_1: branch s_55_0 b89 b56
        if s_55_0 {
            return block_89(state, tracer, fn_state);
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
        // D s_56_1: write-var gs#112854 <= s_56_0
        fn_state.gs_112854 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#112854:u8
        let s_57_0: bool = fn_state.gs_112854;
        // N s_57_1: branch s_57_0 b88 b58
        if s_57_0 {
            return block_88(state, tracer, fn_state);
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
        // N s_58_2: branch s_58_1 b87 b59
        if s_58_1 {
            return block_87(state, tracer, fn_state);
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
        // D s_59_1: write-var gs#112855 <= s_59_0
        fn_state.gs_112855 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#112855:u8
        let s_60_0: bool = fn_state.gs_112855;
        // N s_60_1: branch s_60_0 b86 b61
        if s_60_0 {
            return block_86(state, tracer, fn_state);
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
        // D s_61_1: write-var gs#112856 <= s_61_0
        fn_state.gs_112856 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#112856:u8
        let s_62_0: bool = fn_state.gs_112856;
        // N s_62_1: branch s_62_0 b85 b63
        if s_62_0 {
            return block_85(state, tracer, fn_state);
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
        // N s_63_2: branch s_63_1 b84 b64
        if s_63_1 {
            return block_84(state, tracer, fn_state);
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
        // D s_64_1: write-var gs#112857 <= s_64_0
        fn_state.gs_112857 = s_64_0;
        // N s_64_2: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var gs#112857:u8
        let s_65_0: bool = fn_state.gs_112857;
        // N s_65_1: branch s_65_0 b83 b66
        if s_65_0 {
            return block_83(state, tracer, fn_state);
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
        // D s_66_1: write-var gs#112858 <= s_66_0
        fn_state.gs_112858 = s_66_0;
        // N s_66_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#112858:u8
        let s_67_0: bool = fn_state.gs_112858;
        // N s_67_1: branch s_67_0 b82 b68
        if s_67_0 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #424u : u32
        let s_68_0: u32 = 424;
        // D s_68_1: read-reg s_68_0:u8
        let s_68_1: u8 = {
            let value = state.read_register::<u8>(s_68_0 as isize);
            tracer.read_register(s_68_0 as isize, value);
            value
        };
        // C s_68_2: const #2u : u8
        let s_68_2: u8 = 2;
        // D s_68_3: cmp-lt s_68_1 s_68_2
        let s_68_3: bool = ((s_68_1) < (s_68_2));
        // N s_68_4: branch s_68_3 b81 b69
        if s_68_3 {
            return block_81(state, tracer, fn_state);
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
        // D s_69_1: write-var gs#112859 <= s_69_0
        fn_state.gs_112859 = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var gs#112859:u8
        let s_70_0: bool = fn_state.gs_112859;
        // N s_70_1: branch s_70_0 b80 b71
        if s_70_0 {
            return block_80(state, tracer, fn_state);
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
        // D s_71_1: write-var gs#112860 <= s_71_0
        fn_state.gs_112860 = s_71_0;
        // N s_71_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var gs#112860:u8
        let s_72_0: bool = fn_state.gs_112860;
        // N s_72_1: branch s_72_0 b74 b73
        if s_72_0 {
            return block_74(state, tracer, fn_state);
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
        // S s_73_1: call PMCEID1_read(s_73_0)
        let s_73_1: ProductType700c18a878c5601b = PMCEID1_read(state, tracer, s_73_0);
        // D s_73_2: write-var ga#184962 <= s_73_1
        fn_state.ga_184962 = s_73_1;
        // D s_73_3: read-var ga#184962.0:struct
        let s_73_3: u32 = fn_state.ga_184962._0;
        // D s_73_4: read-var t:i
        let s_73_4: i128 = fn_state.t;
        // D s_73_5: call R_set(s_73_4, s_73_3)
        let s_73_5: () = R_set(state, tracer, s_73_4, s_73_3);
        // N s_73_6: return
        return;
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #() : ()
        let s_74_0: () = ();
        // S s_74_1: call Halted(s_74_0)
        let s_74_1: bool = Halted(state, tracer, s_74_0);
        // N s_74_2: branch s_74_1 b79 b75
        if s_74_1 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #0u : u8
        let s_75_0: bool = false;
        // D s_75_1: write-var gs#112861 <= s_75_0
        fn_state.gs_112861 = s_75_0;
        // N s_75_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var gs#112861:u8
        let s_76_0: bool = fn_state.gs_112861;
        // N s_76_1: branch s_76_0 b78 b77
        if s_76_0 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #3u : u8
        let s_77_0: u8 = 3;
        // C s_77_1: cast zx s_77_0 -> bv
        let s_77_1: Bits = Bits::new(s_77_0 as u128, 8u16);
        // C s_77_2: cast zx s_77_1 -> i
        let s_77_2: i128 = (s_77_1.value() as i128);
        // C s_77_3: cast reint s_77_2 -> i64
        let s_77_3: i64 = (s_77_2 as i64);
        // C s_77_4: cast zx s_77_3 -> i
        let s_77_4: i128 = (i128::try_from(s_77_3).unwrap());
        // C s_77_5: const #424u : u32
        let s_77_5: u32 = 424;
        // D s_77_6: read-reg s_77_5:u8
        let s_77_6: u8 = {
            let value = state.read_register::<u8>(s_77_5 as isize);
            tracer.read_register(s_77_5 as isize, value);
            value
        };
        // D s_77_7: call AArch64_AArch32SystemAccessTrap(s_77_6, s_77_4)
        let s_77_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_77_6, s_77_4);
        // N s_77_8: return
        return;
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_78_0: panic
        panic!("{:?}", ());
        // N s_78_1: return
        return;
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #() : ()
        let s_79_0: () = ();
        // S s_79_1: call EDSCR_read(s_79_0)
        let s_79_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_79_0);
        // S s_79_2: call _get_EDSCR_Type_SDD(s_79_1)
        let s_79_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_79_1);
        // S s_79_3: cast zx s_79_2 -> bv
        let s_79_3: Bits = Bits::new(s_79_2 as u128, 1u16);
        // C s_79_4: const #1u : u8
        let s_79_4: bool = true;
        // C s_79_5: cast zx s_79_4 -> bv
        let s_79_5: Bits = Bits::new(s_79_4 as u128, 1u16);
        // S s_79_6: cmp-eq s_79_3 s_79_5
        let s_79_6: bool = ((s_79_3) == (s_79_5));
        // D s_79_7: write-var gs#112861 <= s_79_6
        fn_state.gs_112861 = s_79_6;
        // N s_79_8: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var __MDCR_EL3_TPM:u8
        let s_80_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_80_1: cast zx s_80_0 -> bv
        let s_80_1: Bits = Bits::new(s_80_0 as u128, 1u16);
        // C s_80_2: const #1u : u8
        let s_80_2: bool = true;
        // C s_80_3: cast zx s_80_2 -> bv
        let s_80_3: Bits = Bits::new(s_80_2 as u128, 1u16);
        // D s_80_4: cmp-eq s_80_1 s_80_3
        let s_80_4: bool = ((s_80_1) == (s_80_3));
        // D s_80_5: write-var gs#112860 <= s_80_4
        fn_state.gs_112860 = s_80_4;
        // N s_80_6: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #424u : u32
        let s_81_0: u32 = 424;
        // D s_81_1: read-reg s_81_0:u8
        let s_81_1: u8 = {
            let value = state.read_register::<u8>(s_81_0 as isize);
            tracer.read_register(s_81_0 as isize, value);
            value
        };
        // D s_81_2: call ELUsingAArch32(s_81_1)
        let s_81_2: bool = ELUsingAArch32(state, tracer, s_81_1);
        // D s_81_3: not s_81_2
        let s_81_3: bool = !s_81_2;
        // D s_81_4: write-var gs#112859 <= s_81_3
        fn_state.gs_112859 = s_81_3;
        // N s_81_5: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #3u : u8
        let s_82_0: u8 = 3;
        // C s_82_1: cast zx s_82_0 -> bv
        let s_82_1: Bits = Bits::new(s_82_0 as u128, 8u16);
        // C s_82_2: cast zx s_82_1 -> i
        let s_82_2: i128 = (s_82_1.value() as i128);
        // C s_82_3: cast reint s_82_2 -> i64
        let s_82_3: i64 = (s_82_2 as i64);
        // C s_82_4: cast zx s_82_3 -> i
        let s_82_4: i128 = (i128::try_from(s_82_3).unwrap());
        // S s_82_5: call AArch32_TakeHypTrapException(s_82_4)
        let s_82_5: () = AArch32_TakeHypTrapException(state, tracer, s_82_4);
        // N s_82_6: return
        return;
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_83_0: read-var __HDCR_TPM:u8
        let s_83_0: bool = fn_state.u__HDCR_TPM;
        // D s_83_1: cast zx s_83_0 -> bv
        let s_83_1: Bits = Bits::new(s_83_0 as u128, 1u16);
        // C s_83_2: const #1u : u8
        let s_83_2: bool = true;
        // C s_83_3: cast zx s_83_2 -> bv
        let s_83_3: Bits = Bits::new(s_83_2 as u128, 1u16);
        // D s_83_4: cmp-eq s_83_1 s_83_3
        let s_83_4: bool = ((s_83_1) == (s_83_3));
        // D s_83_5: write-var gs#112858 <= s_83_4
        fn_state.gs_112858 = s_83_4;
        // N s_83_6: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #432u : u32
        let s_84_0: u32 = 432;
        // D s_84_1: read-reg s_84_0:u8
        let s_84_1: u8 = {
            let value = state.read_register::<u8>(s_84_0 as isize);
            tracer.read_register(s_84_0 as isize, value);
            value
        };
        // D s_84_2: call ELUsingAArch32(s_84_1)
        let s_84_2: bool = ELUsingAArch32(state, tracer, s_84_1);
        // D s_84_3: write-var gs#112857 <= s_84_2
        fn_state.gs_112857 = s_84_2;
        // N s_84_4: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #3u : u8
        let s_85_0: u8 = 3;
        // C s_85_1: cast zx s_85_0 -> bv
        let s_85_1: Bits = Bits::new(s_85_0 as u128, 8u16);
        // C s_85_2: cast zx s_85_1 -> i
        let s_85_2: i128 = (s_85_1.value() as i128);
        // C s_85_3: cast reint s_85_2 -> i64
        let s_85_3: i64 = (s_85_2 as i64);
        // C s_85_4: cast zx s_85_3 -> i
        let s_85_4: i128 = (i128::try_from(s_85_3).unwrap());
        // C s_85_5: const #432u : u32
        let s_85_5: u32 = 432;
        // D s_85_6: read-reg s_85_5:u8
        let s_85_6: u8 = {
            let value = state.read_register::<u8>(s_85_5 as isize);
            tracer.read_register(s_85_5 as isize, value);
            value
        };
        // D s_85_7: call AArch64_AArch32SystemAccessTrap(s_85_6, s_85_4)
        let s_85_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_85_6, s_85_4);
        // N s_85_8: return
        return;
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var __MDCR_EL2_TPM:u8
        let s_86_0: bool = fn_state.u__MDCR_EL2_TPM;
        // D s_86_1: cast zx s_86_0 -> bv
        let s_86_1: Bits = Bits::new(s_86_0 as u128, 1u16);
        // C s_86_2: const #1u : u8
        let s_86_2: bool = true;
        // C s_86_3: cast zx s_86_2 -> bv
        let s_86_3: Bits = Bits::new(s_86_2 as u128, 1u16);
        // D s_86_4: cmp-eq s_86_1 s_86_3
        let s_86_4: bool = ((s_86_1) == (s_86_3));
        // D s_86_5: write-var gs#112856 <= s_86_4
        fn_state.gs_112856 = s_86_4;
        // N s_86_6: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #432u : u32
        let s_87_0: u32 = 432;
        // D s_87_1: read-reg s_87_0:u8
        let s_87_1: u8 = {
            let value = state.read_register::<u8>(s_87_0 as isize);
            tracer.read_register(s_87_0 as isize, value);
            value
        };
        // D s_87_2: call ELUsingAArch32(s_87_1)
        let s_87_2: bool = ELUsingAArch32(state, tracer, s_87_1);
        // D s_87_3: not s_87_2
        let s_87_3: bool = !s_87_2;
        // D s_87_4: write-var gs#112855 <= s_87_3
        fn_state.gs_112855 = s_87_3;
        // N s_87_5: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #3u : u8
        let s_88_0: u8 = 3;
        // C s_88_1: cast zx s_88_0 -> bv
        let s_88_1: Bits = Bits::new(s_88_0 as u128, 8u16);
        // C s_88_2: cast zx s_88_1 -> i
        let s_88_2: i128 = (s_88_1.value() as i128);
        // C s_88_3: cast reint s_88_2 -> i64
        let s_88_3: i64 = (s_88_2 as i64);
        // C s_88_4: cast zx s_88_3 -> i
        let s_88_4: i128 = (i128::try_from(s_88_3).unwrap());
        // S s_88_5: call AArch32_TakeHypTrapException(s_88_4)
        let s_88_5: () = AArch32_TakeHypTrapException(state, tracer, s_88_4);
        // N s_88_6: return
        return;
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_89_0: read-var __HSTR_T9:u8
        let s_89_0: bool = fn_state.u__HSTR_T9;
        // D s_89_1: cast zx s_89_0 -> bv
        let s_89_1: Bits = Bits::new(s_89_0 as u128, 1u16);
        // C s_89_2: const #1u : u8
        let s_89_2: bool = true;
        // C s_89_3: cast zx s_89_2 -> bv
        let s_89_3: Bits = Bits::new(s_89_2 as u128, 1u16);
        // D s_89_4: cmp-eq s_89_1 s_89_3
        let s_89_4: bool = ((s_89_1) == (s_89_3));
        // D s_89_5: write-var gs#112854 <= s_89_4
        fn_state.gs_112854 = s_89_4;
        // N s_89_6: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #432u : u32
        let s_90_0: u32 = 432;
        // D s_90_1: read-reg s_90_0:u8
        let s_90_1: u8 = {
            let value = state.read_register::<u8>(s_90_0 as isize);
            tracer.read_register(s_90_0 as isize, value);
            value
        };
        // D s_90_2: call ELUsingAArch32(s_90_1)
        let s_90_2: bool = ELUsingAArch32(state, tracer, s_90_1);
        // D s_90_3: write-var gs#112853 <= s_90_2
        fn_state.gs_112853 = s_90_2;
        // N s_90_4: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #3u : u8
        let s_91_0: u8 = 3;
        // C s_91_1: cast zx s_91_0 -> bv
        let s_91_1: Bits = Bits::new(s_91_0 as u128, 8u16);
        // C s_91_2: cast zx s_91_1 -> i
        let s_91_2: i128 = (s_91_1.value() as i128);
        // C s_91_3: cast reint s_91_2 -> i64
        let s_91_3: i64 = (s_91_2 as i64);
        // C s_91_4: cast zx s_91_3 -> i
        let s_91_4: i128 = (i128::try_from(s_91_3).unwrap());
        // C s_91_5: const #432u : u32
        let s_91_5: u32 = 432;
        // D s_91_6: read-reg s_91_5:u8
        let s_91_6: u8 = {
            let value = state.read_register::<u8>(s_91_5 as isize);
            tracer.read_register(s_91_5 as isize, value);
            value
        };
        // D s_91_7: call AArch64_AArch32SystemAccessTrap(s_91_6, s_91_4)
        let s_91_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_91_6, s_91_4);
        // N s_91_8: return
        return;
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var __HSTR_EL2_T9:u8
        let s_92_0: bool = fn_state.u__HSTR_EL2_T9;
        // D s_92_1: cast zx s_92_0 -> bv
        let s_92_1: Bits = Bits::new(s_92_0 as u128, 1u16);
        // C s_92_2: const #1u : u8
        let s_92_2: bool = true;
        // C s_92_3: cast zx s_92_2 -> bv
        let s_92_3: Bits = Bits::new(s_92_2 as u128, 1u16);
        // D s_92_4: cmp-eq s_92_1 s_92_3
        let s_92_4: bool = ((s_92_1) == (s_92_3));
        // D s_92_5: write-var gs#112852 <= s_92_4
        fn_state.gs_112852 = s_92_4;
        // N s_92_6: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #432u : u32
        let s_93_0: u32 = 432;
        // D s_93_1: read-reg s_93_0:u8
        let s_93_1: u8 = {
            let value = state.read_register::<u8>(s_93_0 as isize);
            tracer.read_register(s_93_0 as isize, value);
            value
        };
        // D s_93_2: call ELUsingAArch32(s_93_1)
        let s_93_2: bool = ELUsingAArch32(state, tracer, s_93_1);
        // D s_93_3: not s_93_2
        let s_93_3: bool = !s_93_2;
        // D s_93_4: write-var gs#112851 <= s_93_3
        fn_state.gs_112851 = s_93_3;
        // N s_93_5: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_94_0: panic
        panic!("{:?}", ());
        // N s_94_1: return
        return;
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_95_0: read-var __MDCR_EL3_TPM:u8
        let s_95_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_95_1: cast zx s_95_0 -> bv
        let s_95_1: Bits = Bits::new(s_95_0 as u128, 1u16);
        // C s_95_2: const #1u : u8
        let s_95_2: bool = true;
        // C s_95_3: cast zx s_95_2 -> bv
        let s_95_3: Bits = Bits::new(s_95_2 as u128, 1u16);
        // D s_95_4: cmp-eq s_95_1 s_95_3
        let s_95_4: bool = ((s_95_1) == (s_95_3));
        // D s_95_5: write-var gs#112850 <= s_95_4
        fn_state.gs_112850 = s_95_4;
        // N s_95_6: jump b47
        return block_47(state, tracer, fn_state);
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
        // D s_96_2: call ELUsingAArch32(s_96_1)
        let s_96_2: bool = ELUsingAArch32(state, tracer, s_96_1);
        // D s_96_3: not s_96_2
        let s_96_3: bool = !s_96_2;
        // D s_96_4: write-var gs#112849 <= s_96_3
        fn_state.gs_112849 = s_96_3;
        // N s_96_5: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_97_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_97_1: call __IMPDEF_boolean(s_97_0)
        let s_97_1: bool = u__IMPDEF_boolean(state, tracer, s_97_0);
        // D s_97_2: write-var gs#112848 <= s_97_1
        fn_state.gs_112848 = s_97_1;
        // N s_97_3: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #() : ()
        let s_98_0: () = ();
        // S s_98_1: call EDSCR_read(s_98_0)
        let s_98_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_98_0);
        // S s_98_2: call _get_EDSCR_Type_SDD(s_98_1)
        let s_98_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_98_1);
        // S s_98_3: cast zx s_98_2 -> bv
        let s_98_3: Bits = Bits::new(s_98_2 as u128, 1u16);
        // C s_98_4: const #1u : u8
        let s_98_4: bool = true;
        // C s_98_5: cast zx s_98_4 -> bv
        let s_98_5: Bits = Bits::new(s_98_4 as u128, 1u16);
        // S s_98_6: cmp-eq s_98_3 s_98_5
        let s_98_6: bool = ((s_98_3) == (s_98_5));
        // D s_98_7: write-var gs#112847 <= s_98_6
        fn_state.gs_112847 = s_98_6;
        // N s_98_8: jump b41
        return block_41(state, tracer, fn_state);
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
        // D s_99_4: write-var gs#112846 <= s_99_3
        fn_state.gs_112846 = s_99_3;
        // N s_99_5: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #() : ()
        let s_100_0: () = ();
        // S s_100_1: call Halted(s_100_0)
        let s_100_1: bool = Halted(state, tracer, s_100_0);
        // N s_100_2: branch s_100_1 b268 b101
        if s_100_1 {
            return block_268(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #0u : u8
        let s_101_0: bool = false;
        // D s_101_1: write-var gs#112862 <= s_101_0
        fn_state.gs_112862 = s_101_0;
        // N s_101_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var gs#112862:u8
        let s_102_0: bool = fn_state.gs_112862;
        // N s_102_1: branch s_102_0 b267 b103
        if s_102_0 {
            return block_267(state, tracer, fn_state);
        } else {
            return block_103(state, tracer, fn_state);
        };
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #0u : u8
        let s_103_0: bool = false;
        // D s_103_1: write-var gs#112863 <= s_103_0
        fn_state.gs_112863 = s_103_0;
        // N s_103_2: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_104_0: read-var gs#112863:u8
        let s_104_0: bool = fn_state.gs_112863;
        // N s_104_1: branch s_104_0 b266 b105
        if s_104_0 {
            return block_266(state, tracer, fn_state);
        } else {
            return block_105(state, tracer, fn_state);
        };
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #0u : u8
        let s_105_0: bool = false;
        // D s_105_1: write-var gs#112864 <= s_105_0
        fn_state.gs_112864 = s_105_0;
        // N s_105_2: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var gs#112864:u8
        let s_106_0: bool = fn_state.gs_112864;
        // N s_106_1: branch s_106_0 b265 b107
        if s_106_0 {
            return block_265(state, tracer, fn_state);
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
        // D s_107_1: write-var gs#112865 <= s_107_0
        fn_state.gs_112865 = s_107_0;
        // N s_107_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var gs#112865:u8
        let s_108_0: bool = fn_state.gs_112865;
        // N s_108_1: branch s_108_0 b264 b109
        if s_108_0 {
            return block_264(state, tracer, fn_state);
        } else {
            return block_109(state, tracer, fn_state);
        };
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #0u : u8
        let s_109_0: bool = false;
        // D s_109_1: write-var gs#112866 <= s_109_0
        fn_state.gs_112866 = s_109_0;
        // N s_109_2: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_110_0: read-var gs#112866:u8
        let s_110_0: bool = fn_state.gs_112866;
        // N s_110_1: branch s_110_0 b263 b111
        if s_110_0 {
            return block_263(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_111_0: const #440u : u32
        let s_111_0: u32 = 440;
        // D s_111_1: read-reg s_111_0:u8
        let s_111_1: u8 = {
            let value = state.read_register::<u8>(s_111_0 as isize);
            tracer.read_register(s_111_0 as isize, value);
            value
        };
        // D s_111_2: call ELUsingAArch32(s_111_1)
        let s_111_2: bool = ELUsingAArch32(state, tracer, s_111_1);
        // D s_111_3: not s_111_2
        let s_111_3: bool = !s_111_2;
        // N s_111_4: branch s_111_3 b253 b112
        if s_111_3 {
            return block_253(state, tracer, fn_state);
        } else {
            return block_112(state, tracer, fn_state);
        };
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_112_0: const #0u : u8
        let s_112_0: bool = false;
        // D s_112_1: write-var gs#112870 <= s_112_0
        fn_state.gs_112870 = s_112_0;
        // N s_112_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var gs#112870:u8
        let s_113_0: bool = fn_state.gs_112870;
        // N s_113_1: branch s_113_0 b244 b114
        if s_113_0 {
            return block_244(state, tracer, fn_state);
        } else {
            return block_114(state, tracer, fn_state);
        };
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #440u : u32
        let s_114_0: u32 = 440;
        // D s_114_1: read-reg s_114_0:u8
        let s_114_1: u8 = {
            let value = state.read_register::<u8>(s_114_0 as isize);
            tracer.read_register(s_114_0 as isize, value);
            value
        };
        // D s_114_2: call ELUsingAArch32(s_114_1)
        let s_114_2: bool = ELUsingAArch32(state, tracer, s_114_1);
        // D s_114_3: not s_114_2
        let s_114_3: bool = !s_114_2;
        // N s_114_4: branch s_114_3 b243 b115
        if s_114_3 {
            return block_243(state, tracer, fn_state);
        } else {
            return block_115(state, tracer, fn_state);
        };
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #0u : u8
        let s_115_0: bool = false;
        // D s_115_1: write-var gs#112871 <= s_115_0
        fn_state.gs_112871 = s_115_0;
        // N s_115_2: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_116_0: read-var gs#112871:u8
        let s_116_0: bool = fn_state.gs_112871;
        // N s_116_1: branch s_116_0 b242 b117
        if s_116_0 {
            return block_242(state, tracer, fn_state);
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
        // D s_117_1: write-var gs#112872 <= s_117_0
        fn_state.gs_112872 = s_117_0;
        // N s_117_2: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_118_0: read-var gs#112872:u8
        let s_118_0: bool = fn_state.gs_112872;
        // N s_118_1: branch s_118_0 b233 b119
        if s_118_0 {
            return block_233(state, tracer, fn_state);
        } else {
            return block_119(state, tracer, fn_state);
        };
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #440u : u32
        let s_119_0: u32 = 440;
        // D s_119_1: read-reg s_119_0:u8
        let s_119_1: u8 = {
            let value = state.read_register::<u8>(s_119_0 as isize);
            tracer.read_register(s_119_0 as isize, value);
            value
        };
        // D s_119_2: call ELUsingAArch32(s_119_1)
        let s_119_2: bool = ELUsingAArch32(state, tracer, s_119_1);
        // N s_119_3: branch s_119_2 b232 b120
        if s_119_2 {
            return block_232(state, tracer, fn_state);
        } else {
            return block_120(state, tracer, fn_state);
        };
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_120_0: const #0u : u8
        let s_120_0: bool = false;
        // D s_120_1: write-var gs#112873 <= s_120_0
        fn_state.gs_112873 = s_120_0;
        // N s_120_2: jump b121
        return block_121(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_121_0: read-var gs#112873:u8
        let s_121_0: bool = fn_state.gs_112873;
        // N s_121_1: branch s_121_0 b215 b122
        if s_121_0 {
            return block_215(state, tracer, fn_state);
        } else {
            return block_122(state, tracer, fn_state);
        };
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_122_0: const #440u : u32
        let s_122_0: u32 = 440;
        // D s_122_1: read-reg s_122_0:u8
        let s_122_1: u8 = {
            let value = state.read_register::<u8>(s_122_0 as isize);
            tracer.read_register(s_122_0 as isize, value);
            value
        };
        // D s_122_2: call ELUsingAArch32(s_122_1)
        let s_122_2: bool = ELUsingAArch32(state, tracer, s_122_1);
        // N s_122_3: branch s_122_2 b214 b123
        if s_122_2 {
            return block_214(state, tracer, fn_state);
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
        // D s_123_1: write-var gs#112874 <= s_123_0
        fn_state.gs_112874 = s_123_0;
        // N s_123_2: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_124_0: read-var gs#112874:u8
        let s_124_0: bool = fn_state.gs_112874;
        // N s_124_1: branch s_124_0 b213 b125
        if s_124_0 {
            return block_213(state, tracer, fn_state);
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
        // D s_125_1: write-var gs#112875 <= s_125_0
        fn_state.gs_112875 = s_125_0;
        // N s_125_2: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_126_0: read-var gs#112875:u8
        let s_126_0: bool = fn_state.gs_112875;
        // N s_126_1: branch s_126_0 b196 b127
        if s_126_0 {
            return block_196(state, tracer, fn_state);
        } else {
            return block_127(state, tracer, fn_state);
        };
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #() : ()
        let s_127_0: () = ();
        // S s_127_1: call EL2Enabled(s_127_0)
        let s_127_1: bool = EL2Enabled(state, tracer, s_127_0);
        // N s_127_2: branch s_127_1 b195 b128
        if s_127_1 {
            return block_195(state, tracer, fn_state);
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
        // D s_128_1: write-var gs#112876 <= s_128_0
        fn_state.gs_112876 = s_128_0;
        // N s_128_2: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_129_0: read-var gs#112876:u8
        let s_129_0: bool = fn_state.gs_112876;
        // N s_129_1: branch s_129_0 b194 b130
        if s_129_0 {
            return block_194(state, tracer, fn_state);
        } else {
            return block_130(state, tracer, fn_state);
        };
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_130_0: const #0u : u8
        let s_130_0: bool = false;
        // D s_130_1: write-var gs#112877 <= s_130_0
        fn_state.gs_112877 = s_130_0;
        // N s_130_2: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_131_0: read-var gs#112877:u8
        let s_131_0: bool = fn_state.gs_112877;
        // N s_131_1: branch s_131_0 b193 b132
        if s_131_0 {
            return block_193(state, tracer, fn_state);
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
        // D s_132_1: write-var gs#112878 <= s_132_0
        fn_state.gs_112878 = s_132_0;
        // N s_132_2: jump b133
        return block_133(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_133_0: read-var gs#112878:u8
        let s_133_0: bool = fn_state.gs_112878;
        // N s_133_1: branch s_133_0 b192 b134
        if s_133_0 {
            return block_192(state, tracer, fn_state);
        } else {
            return block_134(state, tracer, fn_state);
        };
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_134_0: const #() : ()
        let s_134_0: () = ();
        // S s_134_1: call EL2Enabled(s_134_0)
        let s_134_1: bool = EL2Enabled(state, tracer, s_134_0);
        // N s_134_2: branch s_134_1 b191 b135
        if s_134_1 {
            return block_191(state, tracer, fn_state);
        } else {
            return block_135(state, tracer, fn_state);
        };
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_135_0: const #0u : u8
        let s_135_0: bool = false;
        // D s_135_1: write-var gs#112879 <= s_135_0
        fn_state.gs_112879 = s_135_0;
        // N s_135_2: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_136_0: read-var gs#112879:u8
        let s_136_0: bool = fn_state.gs_112879;
        // N s_136_1: branch s_136_0 b190 b137
        if s_136_0 {
            return block_190(state, tracer, fn_state);
        } else {
            return block_137(state, tracer, fn_state);
        };
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_137_0: const #0u : u8
        let s_137_0: bool = false;
        // D s_137_1: write-var gs#112880 <= s_137_0
        fn_state.gs_112880 = s_137_0;
        // N s_137_2: jump b138
        return block_138(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_138_0: read-var gs#112880:u8
        let s_138_0: bool = fn_state.gs_112880;
        // N s_138_1: branch s_138_0 b189 b139
        if s_138_0 {
            return block_189(state, tracer, fn_state);
        } else {
            return block_139(state, tracer, fn_state);
        };
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_139_0: const #() : ()
        let s_139_0: () = ();
        // S s_139_1: call EL2Enabled(s_139_0)
        let s_139_1: bool = EL2Enabled(state, tracer, s_139_0);
        // N s_139_2: branch s_139_1 b188 b140
        if s_139_1 {
            return block_188(state, tracer, fn_state);
        } else {
            return block_140(state, tracer, fn_state);
        };
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_140_0: const #0u : u8
        let s_140_0: bool = false;
        // D s_140_1: write-var gs#112881 <= s_140_0
        fn_state.gs_112881 = s_140_0;
        // N s_140_2: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_141_0: read-var gs#112881:u8
        let s_141_0: bool = fn_state.gs_112881;
        // N s_141_1: branch s_141_0 b187 b142
        if s_141_0 {
            return block_187(state, tracer, fn_state);
        } else {
            return block_142(state, tracer, fn_state);
        };
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_142_0: const #0u : u8
        let s_142_0: bool = false;
        // D s_142_1: write-var gs#112882 <= s_142_0
        fn_state.gs_112882 = s_142_0;
        // N s_142_2: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_143_0: read-var gs#112882:u8
        let s_143_0: bool = fn_state.gs_112882;
        // N s_143_1: branch s_143_0 b186 b144
        if s_143_0 {
            return block_186(state, tracer, fn_state);
        } else {
            return block_144(state, tracer, fn_state);
        };
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_144_0: const #0u : u8
        let s_144_0: bool = false;
        // D s_144_1: write-var gs#112883 <= s_144_0
        fn_state.gs_112883 = s_144_0;
        // N s_144_2: jump b145
        return block_145(state, tracer, fn_state);
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_145_0: read-var gs#112883:u8
        let s_145_0: bool = fn_state.gs_112883;
        // N s_145_1: branch s_145_0 b182 b146
        if s_145_0 {
            return block_182(state, tracer, fn_state);
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
        // D s_146_1: write-var gs#112885 <= s_146_0
        fn_state.gs_112885 = s_146_0;
        // N s_146_2: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_147_0: read-var gs#112885:u8
        let s_147_0: bool = fn_state.gs_112885;
        // N s_147_1: branch s_147_0 b181 b148
        if s_147_0 {
            return block_181(state, tracer, fn_state);
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
        // D s_148_1: write-var gs#112886 <= s_148_0
        fn_state.gs_112886 = s_148_0;
        // N s_148_2: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_149_0: read-var gs#112886:u8
        let s_149_0: bool = fn_state.gs_112886;
        // N s_149_1: branch s_149_0 b180 b150
        if s_149_0 {
            return block_180(state, tracer, fn_state);
        } else {
            return block_150(state, tracer, fn_state);
        };
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_150_0: const #() : ()
        let s_150_0: () = ();
        // S s_150_1: call EL2Enabled(s_150_0)
        let s_150_1: bool = EL2Enabled(state, tracer, s_150_0);
        // N s_150_2: branch s_150_1 b179 b151
        if s_150_1 {
            return block_179(state, tracer, fn_state);
        } else {
            return block_151(state, tracer, fn_state);
        };
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_151_0: const #0u : u8
        let s_151_0: bool = false;
        // D s_151_1: write-var gs#112887 <= s_151_0
        fn_state.gs_112887 = s_151_0;
        // N s_151_2: jump b152
        return block_152(state, tracer, fn_state);
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_152_0: read-var gs#112887:u8
        let s_152_0: bool = fn_state.gs_112887;
        // N s_152_1: branch s_152_0 b178 b153
        if s_152_0 {
            return block_178(state, tracer, fn_state);
        } else {
            return block_153(state, tracer, fn_state);
        };
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_153_0: const #0u : u8
        let s_153_0: bool = false;
        // D s_153_1: write-var gs#112888 <= s_153_0
        fn_state.gs_112888 = s_153_0;
        // N s_153_2: jump b154
        return block_154(state, tracer, fn_state);
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_154_0: read-var gs#112888:u8
        let s_154_0: bool = fn_state.gs_112888;
        // N s_154_1: branch s_154_0 b177 b155
        if s_154_0 {
            return block_177(state, tracer, fn_state);
        } else {
            return block_155(state, tracer, fn_state);
        };
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_155_0: const #() : ()
        let s_155_0: () = ();
        // S s_155_1: call EL2Enabled(s_155_0)
        let s_155_1: bool = EL2Enabled(state, tracer, s_155_0);
        // N s_155_2: branch s_155_1 b176 b156
        if s_155_1 {
            return block_176(state, tracer, fn_state);
        } else {
            return block_156(state, tracer, fn_state);
        };
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_156_0: const #0u : u8
        let s_156_0: bool = false;
        // D s_156_1: write-var gs#112889 <= s_156_0
        fn_state.gs_112889 = s_156_0;
        // N s_156_2: jump b157
        return block_157(state, tracer, fn_state);
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_157_0: read-var gs#112889:u8
        let s_157_0: bool = fn_state.gs_112889;
        // N s_157_1: branch s_157_0 b175 b158
        if s_157_0 {
            return block_175(state, tracer, fn_state);
        } else {
            return block_158(state, tracer, fn_state);
        };
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_158_0: const #0u : u8
        let s_158_0: bool = false;
        // D s_158_1: write-var gs#112890 <= s_158_0
        fn_state.gs_112890 = s_158_0;
        // N s_158_2: jump b159
        return block_159(state, tracer, fn_state);
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_159_0: read-var gs#112890:u8
        let s_159_0: bool = fn_state.gs_112890;
        // N s_159_1: branch s_159_0 b174 b160
        if s_159_0 {
            return block_174(state, tracer, fn_state);
        } else {
            return block_160(state, tracer, fn_state);
        };
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_160_0: const #424u : u32
        let s_160_0: u32 = 424;
        // D s_160_1: read-reg s_160_0:u8
        let s_160_1: u8 = {
            let value = state.read_register::<u8>(s_160_0 as isize);
            tracer.read_register(s_160_0 as isize, value);
            value
        };
        // C s_160_2: const #2u : u8
        let s_160_2: u8 = 2;
        // D s_160_3: cmp-lt s_160_1 s_160_2
        let s_160_3: bool = ((s_160_1) < (s_160_2));
        // N s_160_4: branch s_160_3 b173 b161
        if s_160_3 {
            return block_173(state, tracer, fn_state);
        } else {
            return block_161(state, tracer, fn_state);
        };
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_161_0: const #0u : u8
        let s_161_0: bool = false;
        // D s_161_1: write-var gs#112891 <= s_161_0
        fn_state.gs_112891 = s_161_0;
        // N s_161_2: jump b162
        return block_162(state, tracer, fn_state);
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_162_0: read-var gs#112891:u8
        let s_162_0: bool = fn_state.gs_112891;
        // N s_162_1: branch s_162_0 b172 b163
        if s_162_0 {
            return block_172(state, tracer, fn_state);
        } else {
            return block_163(state, tracer, fn_state);
        };
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_163_0: const #0u : u8
        let s_163_0: bool = false;
        // D s_163_1: write-var gs#112892 <= s_163_0
        fn_state.gs_112892 = s_163_0;
        // N s_163_2: jump b164
        return block_164(state, tracer, fn_state);
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_164_0: read-var gs#112892:u8
        let s_164_0: bool = fn_state.gs_112892;
        // N s_164_1: branch s_164_0 b166 b165
        if s_164_0 {
            return block_166(state, tracer, fn_state);
        } else {
            return block_165(state, tracer, fn_state);
        };
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_165_0: const #() : ()
        let s_165_0: () = ();
        // S s_165_1: call PMCEID1_read(s_165_0)
        let s_165_1: ProductType700c18a878c5601b = PMCEID1_read(state, tracer, s_165_0);
        // D s_165_2: write-var ga#184921 <= s_165_1
        fn_state.ga_184921 = s_165_1;
        // D s_165_3: read-var ga#184921.0:struct
        let s_165_3: u32 = fn_state.ga_184921._0;
        // D s_165_4: read-var t:i
        let s_165_4: i128 = fn_state.t;
        // D s_165_5: call R_set(s_165_4, s_165_3)
        let s_165_5: () = R_set(state, tracer, s_165_4, s_165_3);
        // N s_165_6: return
        return;
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_166_0: const #() : ()
        let s_166_0: () = ();
        // S s_166_1: call Halted(s_166_0)
        let s_166_1: bool = Halted(state, tracer, s_166_0);
        // N s_166_2: branch s_166_1 b171 b167
        if s_166_1 {
            return block_171(state, tracer, fn_state);
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
        // D s_167_1: write-var gs#112893 <= s_167_0
        fn_state.gs_112893 = s_167_0;
        // N s_167_2: jump b168
        return block_168(state, tracer, fn_state);
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_168_0: read-var gs#112893:u8
        let s_168_0: bool = fn_state.gs_112893;
        // N s_168_1: branch s_168_0 b170 b169
        if s_168_0 {
            return block_170(state, tracer, fn_state);
        } else {
            return block_169(state, tracer, fn_state);
        };
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_169_0: const #3u : u8
        let s_169_0: u8 = 3;
        // C s_169_1: cast zx s_169_0 -> bv
        let s_169_1: Bits = Bits::new(s_169_0 as u128, 8u16);
        // C s_169_2: cast zx s_169_1 -> i
        let s_169_2: i128 = (s_169_1.value() as i128);
        // C s_169_3: cast reint s_169_2 -> i64
        let s_169_3: i64 = (s_169_2 as i64);
        // C s_169_4: cast zx s_169_3 -> i
        let s_169_4: i128 = (i128::try_from(s_169_3).unwrap());
        // C s_169_5: const #424u : u32
        let s_169_5: u32 = 424;
        // D s_169_6: read-reg s_169_5:u8
        let s_169_6: u8 = {
            let value = state.read_register::<u8>(s_169_5 as isize);
            tracer.read_register(s_169_5 as isize, value);
            value
        };
        // D s_169_7: call AArch64_AArch32SystemAccessTrap(s_169_6, s_169_4)
        let s_169_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_169_6,
            s_169_4,
        );
        // N s_169_8: return
        return;
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_170_0: panic
        panic!("{:?}", ());
        // N s_170_1: return
        return;
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_171_0: const #() : ()
        let s_171_0: () = ();
        // S s_171_1: call EDSCR_read(s_171_0)
        let s_171_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_171_0);
        // S s_171_2: call _get_EDSCR_Type_SDD(s_171_1)
        let s_171_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_171_1);
        // S s_171_3: cast zx s_171_2 -> bv
        let s_171_3: Bits = Bits::new(s_171_2 as u128, 1u16);
        // C s_171_4: const #1u : u8
        let s_171_4: bool = true;
        // C s_171_5: cast zx s_171_4 -> bv
        let s_171_5: Bits = Bits::new(s_171_4 as u128, 1u16);
        // S s_171_6: cmp-eq s_171_3 s_171_5
        let s_171_6: bool = ((s_171_3) == (s_171_5));
        // D s_171_7: write-var gs#112893 <= s_171_6
        fn_state.gs_112893 = s_171_6;
        // N s_171_8: jump b168
        return block_168(state, tracer, fn_state);
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_172_0: read-var __MDCR_EL3_TPM:u8
        let s_172_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_172_1: cast zx s_172_0 -> bv
        let s_172_1: Bits = Bits::new(s_172_0 as u128, 1u16);
        // C s_172_2: const #1u : u8
        let s_172_2: bool = true;
        // C s_172_3: cast zx s_172_2 -> bv
        let s_172_3: Bits = Bits::new(s_172_2 as u128, 1u16);
        // D s_172_4: cmp-eq s_172_1 s_172_3
        let s_172_4: bool = ((s_172_1) == (s_172_3));
        // D s_172_5: write-var gs#112892 <= s_172_4
        fn_state.gs_112892 = s_172_4;
        // N s_172_6: jump b164
        return block_164(state, tracer, fn_state);
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_173_0: const #424u : u32
        let s_173_0: u32 = 424;
        // D s_173_1: read-reg s_173_0:u8
        let s_173_1: u8 = {
            let value = state.read_register::<u8>(s_173_0 as isize);
            tracer.read_register(s_173_0 as isize, value);
            value
        };
        // D s_173_2: call ELUsingAArch32(s_173_1)
        let s_173_2: bool = ELUsingAArch32(state, tracer, s_173_1);
        // D s_173_3: not s_173_2
        let s_173_3: bool = !s_173_2;
        // D s_173_4: write-var gs#112891 <= s_173_3
        fn_state.gs_112891 = s_173_3;
        // N s_173_5: jump b162
        return block_162(state, tracer, fn_state);
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_174_0: const #3u : u8
        let s_174_0: u8 = 3;
        // C s_174_1: cast zx s_174_0 -> bv
        let s_174_1: Bits = Bits::new(s_174_0 as u128, 8u16);
        // C s_174_2: cast zx s_174_1 -> i
        let s_174_2: i128 = (s_174_1.value() as i128);
        // C s_174_3: cast reint s_174_2 -> i64
        let s_174_3: i64 = (s_174_2 as i64);
        // C s_174_4: cast zx s_174_3 -> i
        let s_174_4: i128 = (i128::try_from(s_174_3).unwrap());
        // S s_174_5: call AArch32_TakeHypTrapException(s_174_4)
        let s_174_5: () = AArch32_TakeHypTrapException(state, tracer, s_174_4);
        // N s_174_6: return
        return;
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_175_0: read-var __HDCR_TPM:u8
        let s_175_0: bool = fn_state.u__HDCR_TPM;
        // D s_175_1: cast zx s_175_0 -> bv
        let s_175_1: Bits = Bits::new(s_175_0 as u128, 1u16);
        // C s_175_2: const #1u : u8
        let s_175_2: bool = true;
        // C s_175_3: cast zx s_175_2 -> bv
        let s_175_3: Bits = Bits::new(s_175_2 as u128, 1u16);
        // D s_175_4: cmp-eq s_175_1 s_175_3
        let s_175_4: bool = ((s_175_1) == (s_175_3));
        // D s_175_5: write-var gs#112890 <= s_175_4
        fn_state.gs_112890 = s_175_4;
        // N s_175_6: jump b159
        return block_159(state, tracer, fn_state);
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_176_0: const #432u : u32
        let s_176_0: u32 = 432;
        // D s_176_1: read-reg s_176_0:u8
        let s_176_1: u8 = {
            let value = state.read_register::<u8>(s_176_0 as isize);
            tracer.read_register(s_176_0 as isize, value);
            value
        };
        // D s_176_2: call ELUsingAArch32(s_176_1)
        let s_176_2: bool = ELUsingAArch32(state, tracer, s_176_1);
        // D s_176_3: write-var gs#112889 <= s_176_2
        fn_state.gs_112889 = s_176_2;
        // N s_176_4: jump b157
        return block_157(state, tracer, fn_state);
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_177_0: const #3u : u8
        let s_177_0: u8 = 3;
        // C s_177_1: cast zx s_177_0 -> bv
        let s_177_1: Bits = Bits::new(s_177_0 as u128, 8u16);
        // C s_177_2: cast zx s_177_1 -> i
        let s_177_2: i128 = (s_177_1.value() as i128);
        // C s_177_3: cast reint s_177_2 -> i64
        let s_177_3: i64 = (s_177_2 as i64);
        // C s_177_4: cast zx s_177_3 -> i
        let s_177_4: i128 = (i128::try_from(s_177_3).unwrap());
        // C s_177_5: const #432u : u32
        let s_177_5: u32 = 432;
        // D s_177_6: read-reg s_177_5:u8
        let s_177_6: u8 = {
            let value = state.read_register::<u8>(s_177_5 as isize);
            tracer.read_register(s_177_5 as isize, value);
            value
        };
        // D s_177_7: call AArch64_AArch32SystemAccessTrap(s_177_6, s_177_4)
        let s_177_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_177_6,
            s_177_4,
        );
        // N s_177_8: return
        return;
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_178_0: read-var __MDCR_EL2_TPM:u8
        let s_178_0: bool = fn_state.u__MDCR_EL2_TPM;
        // D s_178_1: cast zx s_178_0 -> bv
        let s_178_1: Bits = Bits::new(s_178_0 as u128, 1u16);
        // C s_178_2: const #1u : u8
        let s_178_2: bool = true;
        // C s_178_3: cast zx s_178_2 -> bv
        let s_178_3: Bits = Bits::new(s_178_2 as u128, 1u16);
        // D s_178_4: cmp-eq s_178_1 s_178_3
        let s_178_4: bool = ((s_178_1) == (s_178_3));
        // D s_178_5: write-var gs#112888 <= s_178_4
        fn_state.gs_112888 = s_178_4;
        // N s_178_6: jump b154
        return block_154(state, tracer, fn_state);
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_179_0: const #432u : u32
        let s_179_0: u32 = 432;
        // D s_179_1: read-reg s_179_0:u8
        let s_179_1: u8 = {
            let value = state.read_register::<u8>(s_179_0 as isize);
            tracer.read_register(s_179_0 as isize, value);
            value
        };
        // D s_179_2: call ELUsingAArch32(s_179_1)
        let s_179_2: bool = ELUsingAArch32(state, tracer, s_179_1);
        // D s_179_3: not s_179_2
        let s_179_3: bool = !s_179_2;
        // D s_179_4: write-var gs#112887 <= s_179_3
        fn_state.gs_112887 = s_179_3;
        // N s_179_5: jump b152
        return block_152(state, tracer, fn_state);
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_180_0: const #3u : u8
        let s_180_0: u8 = 3;
        // C s_180_1: cast zx s_180_0 -> bv
        let s_180_1: Bits = Bits::new(s_180_0 as u128, 8u16);
        // C s_180_2: cast zx s_180_1 -> i
        let s_180_2: i128 = (s_180_1.value() as i128);
        // C s_180_3: cast reint s_180_2 -> i64
        let s_180_3: i64 = (s_180_2 as i64);
        // C s_180_4: cast zx s_180_3 -> i
        let s_180_4: i128 = (i128::try_from(s_180_3).unwrap());
        // C s_180_5: const #432u : u32
        let s_180_5: u32 = 432;
        // D s_180_6: read-reg s_180_5:u8
        let s_180_6: u8 = {
            let value = state.read_register::<u8>(s_180_5 as isize);
            tracer.read_register(s_180_5 as isize, value);
            value
        };
        // D s_180_7: call AArch64_AArch32SystemAccessTrap(s_180_6, s_180_4)
        let s_180_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_180_6,
            s_180_4,
        );
        // N s_180_8: return
        return;
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_181_0: read-var __HDFGRTR_EL2_PMCEIDn_EL0:u8
        let s_181_0: bool = fn_state.u__HDFGRTR_EL2_PMCEIDn_EL0;
        // D s_181_1: cast zx s_181_0 -> bv
        let s_181_1: Bits = Bits::new(s_181_0 as u128, 1u16);
        // C s_181_2: const #1u : u8
        let s_181_2: bool = true;
        // C s_181_3: cast zx s_181_2 -> bv
        let s_181_3: Bits = Bits::new(s_181_2 as u128, 1u16);
        // D s_181_4: cmp-eq s_181_1 s_181_3
        let s_181_4: bool = ((s_181_1) == (s_181_3));
        // D s_181_5: write-var gs#112886 <= s_181_4
        fn_state.gs_112886 = s_181_4;
        // N s_181_6: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_182_0: const #424u : u32
        let s_182_0: u32 = 424;
        // D s_182_1: read-reg s_182_0:u8
        let s_182_1: u8 = {
            let value = state.read_register::<u8>(s_182_0 as isize);
            tracer.read_register(s_182_0 as isize, value);
            value
        };
        // C s_182_2: const #2u : u8
        let s_182_2: u8 = 2;
        // D s_182_3: cmp-lt s_182_1 s_182_2
        let s_182_3: bool = ((s_182_1) < (s_182_2));
        // D s_182_4: not s_182_3
        let s_182_4: bool = !s_182_3;
        // N s_182_5: branch s_182_4 b185 b183
        if s_182_4 {
            return block_185(state, tracer, fn_state);
        } else {
            return block_183(state, tracer, fn_state);
        };
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_183_0: read-var __SCR_EL3_FGTEn:u8
        let s_183_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_183_1: cast zx s_183_0 -> bv
        let s_183_1: Bits = Bits::new(s_183_0 as u128, 1u16);
        // C s_183_2: const #1u : u8
        let s_183_2: bool = true;
        // C s_183_3: cast zx s_183_2 -> bv
        let s_183_3: Bits = Bits::new(s_183_2 as u128, 1u16);
        // D s_183_4: cmp-eq s_183_1 s_183_3
        let s_183_4: bool = ((s_183_1) == (s_183_3));
        // D s_183_5: write-var gs#112884 <= s_183_4
        fn_state.gs_112884 = s_183_4;
        // N s_183_6: jump b184
        return block_184(state, tracer, fn_state);
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_184_0: read-var gs#112884:u8
        let s_184_0: bool = fn_state.gs_112884;
        // D s_184_1: write-var gs#112885 <= s_184_0
        fn_state.gs_112885 = s_184_0;
        // N s_184_2: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_185<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_185_0: const #1u : u8
        let s_185_0: bool = true;
        // D s_185_1: write-var gs#112884 <= s_185_0
        fn_state.gs_112884 = s_185_0;
        // N s_185_2: jump b184
        return block_184(state, tracer, fn_state);
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_186_0: const #146u : u32
        let s_186_0: u32 = 146;
        // S s_186_1: call IsFeatureImplemented(s_186_0)
        let s_186_1: bool = IsFeatureImplemented(state, tracer, s_186_0);
        // D s_186_2: write-var gs#112883 <= s_186_1
        fn_state.gs_112883 = s_186_1;
        // N s_186_3: jump b145
        return block_145(state, tracer, fn_state);
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_187_0: const #102552u : u32
        let s_187_0: u32 = 102552;
        // D s_187_1: read-reg s_187_0:struct
        let s_187_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_187_0 as isize);
            tracer.read_register(s_187_0 as isize, value);
            value
        };
        // D s_187_2: call _get_HCR_EL2_Type_E2H(s_187_1)
        let s_187_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_187_1);
        // C s_187_3: const #102552u : u32
        let s_187_3: u32 = 102552;
        // D s_187_4: read-reg s_187_3:struct
        let s_187_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_187_3 as isize);
            tracer.read_register(s_187_3 as isize, value);
            value
        };
        // D s_187_5: call _get_HCR_EL2_Type_TGE(s_187_4)
        let s_187_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_187_4);
        // D s_187_6: cast zx s_187_2 -> bv
        let s_187_6: Bits = Bits::new(s_187_2 as u128, 1u16);
        // D s_187_7: cast zx s_187_5 -> bv
        let s_187_7: Bits = Bits::new(s_187_5 as u128, 1u16);
        // D s_187_8: cast reint s_187_6 -> u128
        let s_187_8: u128 = (s_187_6.value() as u128);
        // D s_187_9: size-of s_187_6
        let s_187_9: u16 = s_187_6.length();
        // D s_187_10: cast reint s_187_7 -> u128
        let s_187_10: u128 = (s_187_7.value() as u128);
        // D s_187_11: size-of s_187_7
        let s_187_11: u16 = s_187_7.length();
        // D s_187_12: lsl s_187_8 s_187_11
        let s_187_12: u128 = s_187_8 << s_187_11;
        // D s_187_13: or s_187_12 s_187_10
        let s_187_13: u128 = ((s_187_12) | (s_187_10));
        // D s_187_14: add s_187_9 s_187_11
        let s_187_14: u16 = (s_187_9 + s_187_11);
        // D s_187_15: create-bits s_187_13 s_187_14
        let s_187_15: Bits = Bits::new(s_187_13, s_187_14);
        // D s_187_16: cast reint s_187_15 -> u8
        let s_187_16: u8 = (s_187_15.value() as u8);
        // D s_187_17: cast zx s_187_16 -> bv
        let s_187_17: Bits = Bits::new(s_187_16 as u128, 2u16);
        // C s_187_18: const #3u : u8
        let s_187_18: u8 = 3;
        // C s_187_19: cast zx s_187_18 -> bv
        let s_187_19: Bits = Bits::new(s_187_18 as u128, 2u16);
        // D s_187_20: cmp-ne s_187_17 s_187_19
        let s_187_20: bool = ((s_187_17) != (s_187_19));
        // D s_187_21: write-var gs#112882 <= s_187_20
        fn_state.gs_112882 = s_187_20;
        // N s_187_22: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_188_0: const #440u : u32
        let s_188_0: u32 = 440;
        // D s_188_1: read-reg s_188_0:u8
        let s_188_1: u8 = {
            let value = state.read_register::<u8>(s_188_0 as isize);
            tracer.read_register(s_188_0 as isize, value);
            value
        };
        // D s_188_2: call ELUsingAArch32(s_188_1)
        let s_188_2: bool = ELUsingAArch32(state, tracer, s_188_1);
        // D s_188_3: not s_188_2
        let s_188_3: bool = !s_188_2;
        // D s_188_4: write-var gs#112881 <= s_188_3
        fn_state.gs_112881 = s_188_3;
        // N s_188_5: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_189<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_189_0: const #3u : u8
        let s_189_0: u8 = 3;
        // C s_189_1: cast zx s_189_0 -> bv
        let s_189_1: Bits = Bits::new(s_189_0 as u128, 8u16);
        // C s_189_2: cast zx s_189_1 -> i
        let s_189_2: i128 = (s_189_1.value() as i128);
        // C s_189_3: cast reint s_189_2 -> i64
        let s_189_3: i64 = (s_189_2 as i64);
        // C s_189_4: cast zx s_189_3 -> i
        let s_189_4: i128 = (i128::try_from(s_189_3).unwrap());
        // S s_189_5: call AArch32_TakeHypTrapException(s_189_4)
        let s_189_5: () = AArch32_TakeHypTrapException(state, tracer, s_189_4);
        // N s_189_6: return
        return;
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_190_0: read-var __HSTR_T9:u8
        let s_190_0: bool = fn_state.u__HSTR_T9;
        // D s_190_1: cast zx s_190_0 -> bv
        let s_190_1: Bits = Bits::new(s_190_0 as u128, 1u16);
        // C s_190_2: const #1u : u8
        let s_190_2: bool = true;
        // C s_190_3: cast zx s_190_2 -> bv
        let s_190_3: Bits = Bits::new(s_190_2 as u128, 1u16);
        // D s_190_4: cmp-eq s_190_1 s_190_3
        let s_190_4: bool = ((s_190_1) == (s_190_3));
        // D s_190_5: write-var gs#112880 <= s_190_4
        fn_state.gs_112880 = s_190_4;
        // N s_190_6: jump b138
        return block_138(state, tracer, fn_state);
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_191_0: const #432u : u32
        let s_191_0: u32 = 432;
        // D s_191_1: read-reg s_191_0:u8
        let s_191_1: u8 = {
            let value = state.read_register::<u8>(s_191_0 as isize);
            tracer.read_register(s_191_0 as isize, value);
            value
        };
        // D s_191_2: call ELUsingAArch32(s_191_1)
        let s_191_2: bool = ELUsingAArch32(state, tracer, s_191_1);
        // D s_191_3: write-var gs#112879 <= s_191_2
        fn_state.gs_112879 = s_191_2;
        // N s_191_4: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_192_0: const #3u : u8
        let s_192_0: u8 = 3;
        // C s_192_1: cast zx s_192_0 -> bv
        let s_192_1: Bits = Bits::new(s_192_0 as u128, 8u16);
        // C s_192_2: cast zx s_192_1 -> i
        let s_192_2: i128 = (s_192_1.value() as i128);
        // C s_192_3: cast reint s_192_2 -> i64
        let s_192_3: i64 = (s_192_2 as i64);
        // C s_192_4: cast zx s_192_3 -> i
        let s_192_4: i128 = (i128::try_from(s_192_3).unwrap());
        // C s_192_5: const #432u : u32
        let s_192_5: u32 = 432;
        // D s_192_6: read-reg s_192_5:u8
        let s_192_6: u8 = {
            let value = state.read_register::<u8>(s_192_5 as isize);
            tracer.read_register(s_192_5 as isize, value);
            value
        };
        // D s_192_7: call AArch64_AArch32SystemAccessTrap(s_192_6, s_192_4)
        let s_192_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_192_6,
            s_192_4,
        );
        // N s_192_8: return
        return;
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_193_0: read-var __HSTR_EL2_T9:u8
        let s_193_0: bool = fn_state.u__HSTR_EL2_T9;
        // D s_193_1: cast zx s_193_0 -> bv
        let s_193_1: Bits = Bits::new(s_193_0 as u128, 1u16);
        // C s_193_2: const #1u : u8
        let s_193_2: bool = true;
        // C s_193_3: cast zx s_193_2 -> bv
        let s_193_3: Bits = Bits::new(s_193_2 as u128, 1u16);
        // D s_193_4: cmp-eq s_193_1 s_193_3
        let s_193_4: bool = ((s_193_1) == (s_193_3));
        // D s_193_5: write-var gs#112878 <= s_193_4
        fn_state.gs_112878 = s_193_4;
        // N s_193_6: jump b133
        return block_133(state, tracer, fn_state);
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_194_0: const #102552u : u32
        let s_194_0: u32 = 102552;
        // D s_194_1: read-reg s_194_0:struct
        let s_194_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_194_0 as isize);
            tracer.read_register(s_194_0 as isize, value);
            value
        };
        // D s_194_2: call _get_HCR_EL2_Type_E2H(s_194_1)
        let s_194_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_194_1);
        // C s_194_3: const #102552u : u32
        let s_194_3: u32 = 102552;
        // D s_194_4: read-reg s_194_3:struct
        let s_194_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_194_3 as isize);
            tracer.read_register(s_194_3 as isize, value);
            value
        };
        // D s_194_5: call _get_HCR_EL2_Type_TGE(s_194_4)
        let s_194_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_194_4);
        // D s_194_6: cast zx s_194_2 -> bv
        let s_194_6: Bits = Bits::new(s_194_2 as u128, 1u16);
        // D s_194_7: cast zx s_194_5 -> bv
        let s_194_7: Bits = Bits::new(s_194_5 as u128, 1u16);
        // D s_194_8: cast reint s_194_6 -> u128
        let s_194_8: u128 = (s_194_6.value() as u128);
        // D s_194_9: size-of s_194_6
        let s_194_9: u16 = s_194_6.length();
        // D s_194_10: cast reint s_194_7 -> u128
        let s_194_10: u128 = (s_194_7.value() as u128);
        // D s_194_11: size-of s_194_7
        let s_194_11: u16 = s_194_7.length();
        // D s_194_12: lsl s_194_8 s_194_11
        let s_194_12: u128 = s_194_8 << s_194_11;
        // D s_194_13: or s_194_12 s_194_10
        let s_194_13: u128 = ((s_194_12) | (s_194_10));
        // D s_194_14: add s_194_9 s_194_11
        let s_194_14: u16 = (s_194_9 + s_194_11);
        // D s_194_15: create-bits s_194_13 s_194_14
        let s_194_15: Bits = Bits::new(s_194_13, s_194_14);
        // D s_194_16: cast reint s_194_15 -> u8
        let s_194_16: u8 = (s_194_15.value() as u8);
        // D s_194_17: cast zx s_194_16 -> bv
        let s_194_17: Bits = Bits::new(s_194_16 as u128, 2u16);
        // C s_194_18: const #3u : u8
        let s_194_18: u8 = 3;
        // C s_194_19: cast zx s_194_18 -> bv
        let s_194_19: Bits = Bits::new(s_194_18 as u128, 2u16);
        // D s_194_20: cmp-ne s_194_17 s_194_19
        let s_194_20: bool = ((s_194_17) != (s_194_19));
        // D s_194_21: write-var gs#112877 <= s_194_20
        fn_state.gs_112877 = s_194_20;
        // N s_194_22: jump b131
        return block_131(state, tracer, fn_state);
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
        // D s_195_3: not s_195_2
        let s_195_3: bool = !s_195_2;
        // D s_195_4: write-var gs#112876 <= s_195_3
        fn_state.gs_112876 = s_195_3;
        // N s_195_5: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_196<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_196_0: const #() : ()
        let s_196_0: () = ();
        // S s_196_1: call EL2Enabled(s_196_0)
        let s_196_1: bool = EL2Enabled(state, tracer, s_196_0);
        // N s_196_2: branch s_196_1 b212 b197
        if s_196_1 {
            return block_212(state, tracer, fn_state);
        } else {
            return block_197(state, tracer, fn_state);
        };
    }
    fn block_197<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_197_0: const #0u : u8
        let s_197_0: bool = false;
        // D s_197_1: write-var gs#112894 <= s_197_0
        fn_state.gs_112894 = s_197_0;
        // N s_197_2: jump b198
        return block_198(state, tracer, fn_state);
    }
    fn block_198<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_198_0: read-var gs#112894:u8
        let s_198_0: bool = fn_state.gs_112894;
        // N s_198_1: branch s_198_0 b211 b199
        if s_198_0 {
            return block_211(state, tracer, fn_state);
        } else {
            return block_199(state, tracer, fn_state);
        };
    }
    fn block_199<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_199_0: const #0u : u8
        let s_199_0: bool = false;
        // D s_199_1: write-var gs#112895 <= s_199_0
        fn_state.gs_112895 = s_199_0;
        // N s_199_2: jump b200
        return block_200(state, tracer, fn_state);
    }
    fn block_200<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_200_0: read-var gs#112895:u8
        let s_200_0: bool = fn_state.gs_112895;
        // N s_200_1: branch s_200_0 b210 b201
        if s_200_0 {
            return block_210(state, tracer, fn_state);
        } else {
            return block_201(state, tracer, fn_state);
        };
    }
    fn block_201<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_201_0: const #() : ()
        let s_201_0: () = ();
        // S s_201_1: call EL2Enabled(s_201_0)
        let s_201_1: bool = EL2Enabled(state, tracer, s_201_0);
        // N s_201_2: branch s_201_1 b209 b202
        if s_201_1 {
            return block_209(state, tracer, fn_state);
        } else {
            return block_202(state, tracer, fn_state);
        };
    }
    fn block_202<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_202_0: const #0u : u8
        let s_202_0: bool = false;
        // D s_202_1: write-var gs#112896 <= s_202_0
        fn_state.gs_112896 = s_202_0;
        // N s_202_2: jump b203
        return block_203(state, tracer, fn_state);
    }
    fn block_203<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_203_0: read-var gs#112896:u8
        let s_203_0: bool = fn_state.gs_112896;
        // N s_203_1: branch s_203_0 b208 b204
        if s_203_0 {
            return block_208(state, tracer, fn_state);
        } else {
            return block_204(state, tracer, fn_state);
        };
    }
    fn block_204<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_204_0: const #0u : u8
        let s_204_0: bool = false;
        // D s_204_1: write-var gs#112897 <= s_204_0
        fn_state.gs_112897 = s_204_0;
        // N s_204_2: jump b205
        return block_205(state, tracer, fn_state);
    }
    fn block_205<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_205_0: read-var gs#112897:u8
        let s_205_0: bool = fn_state.gs_112897;
        // N s_205_1: branch s_205_0 b207 b206
        if s_205_0 {
            return block_207(state, tracer, fn_state);
        } else {
            return block_206(state, tracer, fn_state);
        };
    }
    fn block_206<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_206_0: panic
        panic!("{:?}", ());
        // N s_206_1: return
        return;
    }
    fn block_207<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_207_0: const #0u : u8
        let s_207_0: u8 = 0;
        // C s_207_1: cast zx s_207_0 -> bv
        let s_207_1: Bits = Bits::new(s_207_0 as u128, 8u16);
        // C s_207_2: cast zx s_207_1 -> i
        let s_207_2: i128 = (s_207_1.value() as i128);
        // C s_207_3: cast reint s_207_2 -> i64
        let s_207_3: i64 = (s_207_2 as i64);
        // C s_207_4: cast zx s_207_3 -> i
        let s_207_4: i128 = (i128::try_from(s_207_3).unwrap());
        // S s_207_5: call AArch32_TakeHypTrapException(s_207_4)
        let s_207_5: () = AArch32_TakeHypTrapException(state, tracer, s_207_4);
        // N s_207_6: return
        return;
    }
    fn block_208<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_208_0: read-var __HCR_TGE:u8
        let s_208_0: bool = fn_state.u__HCR_TGE;
        // D s_208_1: cast zx s_208_0 -> bv
        let s_208_1: Bits = Bits::new(s_208_0 as u128, 1u16);
        // C s_208_2: const #1u : u8
        let s_208_2: bool = true;
        // C s_208_3: cast zx s_208_2 -> bv
        let s_208_3: Bits = Bits::new(s_208_2 as u128, 1u16);
        // D s_208_4: cmp-eq s_208_1 s_208_3
        let s_208_4: bool = ((s_208_1) == (s_208_3));
        // D s_208_5: write-var gs#112897 <= s_208_4
        fn_state.gs_112897 = s_208_4;
        // N s_208_6: jump b205
        return block_205(state, tracer, fn_state);
    }
    fn block_209<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_209_0: const #432u : u32
        let s_209_0: u32 = 432;
        // D s_209_1: read-reg s_209_0:u8
        let s_209_1: u8 = {
            let value = state.read_register::<u8>(s_209_0 as isize);
            tracer.read_register(s_209_0 as isize, value);
            value
        };
        // D s_209_2: call ELUsingAArch32(s_209_1)
        let s_209_2: bool = ELUsingAArch32(state, tracer, s_209_1);
        // D s_209_3: write-var gs#112896 <= s_209_2
        fn_state.gs_112896 = s_209_2;
        // N s_209_4: jump b203
        return block_203(state, tracer, fn_state);
    }
    fn block_210<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_210_0: const #3u : u8
        let s_210_0: u8 = 3;
        // C s_210_1: cast zx s_210_0 -> bv
        let s_210_1: Bits = Bits::new(s_210_0 as u128, 8u16);
        // C s_210_2: cast zx s_210_1 -> i
        let s_210_2: i128 = (s_210_1.value() as i128);
        // C s_210_3: cast reint s_210_2 -> i64
        let s_210_3: i64 = (s_210_2 as i64);
        // C s_210_4: cast zx s_210_3 -> i
        let s_210_4: i128 = (i128::try_from(s_210_3).unwrap());
        // C s_210_5: const #432u : u32
        let s_210_5: u32 = 432;
        // D s_210_6: read-reg s_210_5:u8
        let s_210_6: u8 = {
            let value = state.read_register::<u8>(s_210_5 as isize);
            tracer.read_register(s_210_5 as isize, value);
            value
        };
        // D s_210_7: call AArch64_AArch32SystemAccessTrap(s_210_6, s_210_4)
        let s_210_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_210_6,
            s_210_4,
        );
        // N s_210_8: return
        return;
    }
    fn block_211<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_211_0: read-var __HCR_EL2_TGE:u8
        let s_211_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_211_1: cast zx s_211_0 -> bv
        let s_211_1: Bits = Bits::new(s_211_0 as u128, 1u16);
        // C s_211_2: const #1u : u8
        let s_211_2: bool = true;
        // C s_211_3: cast zx s_211_2 -> bv
        let s_211_3: Bits = Bits::new(s_211_2 as u128, 1u16);
        // D s_211_4: cmp-eq s_211_1 s_211_3
        let s_211_4: bool = ((s_211_1) == (s_211_3));
        // D s_211_5: write-var gs#112895 <= s_211_4
        fn_state.gs_112895 = s_211_4;
        // N s_211_6: jump b200
        return block_200(state, tracer, fn_state);
    }
    fn block_212<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_212_0: const #432u : u32
        let s_212_0: u32 = 432;
        // D s_212_1: read-reg s_212_0:u8
        let s_212_1: u8 = {
            let value = state.read_register::<u8>(s_212_0 as isize);
            tracer.read_register(s_212_0 as isize, value);
            value
        };
        // D s_212_2: call ELUsingAArch32(s_212_1)
        let s_212_2: bool = ELUsingAArch32(state, tracer, s_212_1);
        // D s_212_3: not s_212_2
        let s_212_3: bool = !s_212_2;
        // D s_212_4: write-var gs#112894 <= s_212_3
        fn_state.gs_112894 = s_212_3;
        // N s_212_5: jump b198
        return block_198(state, tracer, fn_state);
    }
    fn block_213<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_213_0: read-var __PMUSERENR_TID:u8
        let s_213_0: bool = fn_state.u__PMUSERENR_TID;
        // D s_213_1: cast zx s_213_0 -> bv
        let s_213_1: Bits = Bits::new(s_213_0 as u128, 1u16);
        // C s_213_2: const #1u : u8
        let s_213_2: bool = true;
        // C s_213_3: cast zx s_213_2 -> bv
        let s_213_3: Bits = Bits::new(s_213_2 as u128, 1u16);
        // D s_213_4: cmp-eq s_213_1 s_213_3
        let s_213_4: bool = ((s_213_1) == (s_213_3));
        // D s_213_5: write-var gs#112875 <= s_213_4
        fn_state.gs_112875 = s_213_4;
        // N s_213_6: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_214<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_214_0: const #204u : u32
        let s_214_0: u32 = 204;
        // S s_214_1: call IsFeatureImplemented(s_214_0)
        let s_214_1: bool = IsFeatureImplemented(state, tracer, s_214_0);
        // D s_214_2: write-var gs#112874 <= s_214_1
        fn_state.gs_112874 = s_214_1;
        // N s_214_3: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_215<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_215_0: const #() : ()
        let s_215_0: () = ();
        // S s_215_1: call EL2Enabled(s_215_0)
        let s_215_1: bool = EL2Enabled(state, tracer, s_215_0);
        // N s_215_2: branch s_215_1 b231 b216
        if s_215_1 {
            return block_231(state, tracer, fn_state);
        } else {
            return block_216(state, tracer, fn_state);
        };
    }
    fn block_216<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_216_0: const #0u : u8
        let s_216_0: bool = false;
        // D s_216_1: write-var gs#112898 <= s_216_0
        fn_state.gs_112898 = s_216_0;
        // N s_216_2: jump b217
        return block_217(state, tracer, fn_state);
    }
    fn block_217<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_217_0: read-var gs#112898:u8
        let s_217_0: bool = fn_state.gs_112898;
        // N s_217_1: branch s_217_0 b230 b218
        if s_217_0 {
            return block_230(state, tracer, fn_state);
        } else {
            return block_218(state, tracer, fn_state);
        };
    }
    fn block_218<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_218_0: const #0u : u8
        let s_218_0: bool = false;
        // D s_218_1: write-var gs#112899 <= s_218_0
        fn_state.gs_112899 = s_218_0;
        // N s_218_2: jump b219
        return block_219(state, tracer, fn_state);
    }
    fn block_219<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_219_0: read-var gs#112899:u8
        let s_219_0: bool = fn_state.gs_112899;
        // N s_219_1: branch s_219_0 b229 b220
        if s_219_0 {
            return block_229(state, tracer, fn_state);
        } else {
            return block_220(state, tracer, fn_state);
        };
    }
    fn block_220<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_220_0: const #() : ()
        let s_220_0: () = ();
        // S s_220_1: call EL2Enabled(s_220_0)
        let s_220_1: bool = EL2Enabled(state, tracer, s_220_0);
        // N s_220_2: branch s_220_1 b228 b221
        if s_220_1 {
            return block_228(state, tracer, fn_state);
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
        // D s_221_1: write-var gs#112900 <= s_221_0
        fn_state.gs_112900 = s_221_0;
        // N s_221_2: jump b222
        return block_222(state, tracer, fn_state);
    }
    fn block_222<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_222_0: read-var gs#112900:u8
        let s_222_0: bool = fn_state.gs_112900;
        // N s_222_1: branch s_222_0 b227 b223
        if s_222_0 {
            return block_227(state, tracer, fn_state);
        } else {
            return block_223(state, tracer, fn_state);
        };
    }
    fn block_223<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_223_0: const #0u : u8
        let s_223_0: bool = false;
        // D s_223_1: write-var gs#112901 <= s_223_0
        fn_state.gs_112901 = s_223_0;
        // N s_223_2: jump b224
        return block_224(state, tracer, fn_state);
    }
    fn block_224<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_224_0: read-var gs#112901:u8
        let s_224_0: bool = fn_state.gs_112901;
        // N s_224_1: branch s_224_0 b226 b225
        if s_224_0 {
            return block_226(state, tracer, fn_state);
        } else {
            return block_225(state, tracer, fn_state);
        };
    }
    fn block_225<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_225_0: panic
        panic!("{:?}", ());
        // N s_225_1: return
        return;
    }
    fn block_226<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_226_0: const #0u : u8
        let s_226_0: u8 = 0;
        // C s_226_1: cast zx s_226_0 -> bv
        let s_226_1: Bits = Bits::new(s_226_0 as u128, 8u16);
        // C s_226_2: cast zx s_226_1 -> i
        let s_226_2: i128 = (s_226_1.value() as i128);
        // C s_226_3: cast reint s_226_2 -> i64
        let s_226_3: i64 = (s_226_2 as i64);
        // C s_226_4: cast zx s_226_3 -> i
        let s_226_4: i128 = (i128::try_from(s_226_3).unwrap());
        // S s_226_5: call AArch32_TakeHypTrapException(s_226_4)
        let s_226_5: () = AArch32_TakeHypTrapException(state, tracer, s_226_4);
        // N s_226_6: return
        return;
    }
    fn block_227<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_227_0: read-var __HCR_TGE:u8
        let s_227_0: bool = fn_state.u__HCR_TGE;
        // D s_227_1: cast zx s_227_0 -> bv
        let s_227_1: Bits = Bits::new(s_227_0 as u128, 1u16);
        // C s_227_2: const #1u : u8
        let s_227_2: bool = true;
        // C s_227_3: cast zx s_227_2 -> bv
        let s_227_3: Bits = Bits::new(s_227_2 as u128, 1u16);
        // D s_227_4: cmp-eq s_227_1 s_227_3
        let s_227_4: bool = ((s_227_1) == (s_227_3));
        // D s_227_5: write-var gs#112901 <= s_227_4
        fn_state.gs_112901 = s_227_4;
        // N s_227_6: jump b224
        return block_224(state, tracer, fn_state);
    }
    fn block_228<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_228_0: const #432u : u32
        let s_228_0: u32 = 432;
        // D s_228_1: read-reg s_228_0:u8
        let s_228_1: u8 = {
            let value = state.read_register::<u8>(s_228_0 as isize);
            tracer.read_register(s_228_0 as isize, value);
            value
        };
        // D s_228_2: call ELUsingAArch32(s_228_1)
        let s_228_2: bool = ELUsingAArch32(state, tracer, s_228_1);
        // D s_228_3: write-var gs#112900 <= s_228_2
        fn_state.gs_112900 = s_228_2;
        // N s_228_4: jump b222
        return block_222(state, tracer, fn_state);
    }
    fn block_229<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_229_0: const #3u : u8
        let s_229_0: u8 = 3;
        // C s_229_1: cast zx s_229_0 -> bv
        let s_229_1: Bits = Bits::new(s_229_0 as u128, 8u16);
        // C s_229_2: cast zx s_229_1 -> i
        let s_229_2: i128 = (s_229_1.value() as i128);
        // C s_229_3: cast reint s_229_2 -> i64
        let s_229_3: i64 = (s_229_2 as i64);
        // C s_229_4: cast zx s_229_3 -> i
        let s_229_4: i128 = (i128::try_from(s_229_3).unwrap());
        // C s_229_5: const #432u : u32
        let s_229_5: u32 = 432;
        // D s_229_6: read-reg s_229_5:u8
        let s_229_6: u8 = {
            let value = state.read_register::<u8>(s_229_5 as isize);
            tracer.read_register(s_229_5 as isize, value);
            value
        };
        // D s_229_7: call AArch64_AArch32SystemAccessTrap(s_229_6, s_229_4)
        let s_229_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_229_6,
            s_229_4,
        );
        // N s_229_8: return
        return;
    }
    fn block_230<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_230_0: read-var __HCR_EL2_TGE:u8
        let s_230_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_230_1: cast zx s_230_0 -> bv
        let s_230_1: Bits = Bits::new(s_230_0 as u128, 1u16);
        // C s_230_2: const #1u : u8
        let s_230_2: bool = true;
        // C s_230_3: cast zx s_230_2 -> bv
        let s_230_3: Bits = Bits::new(s_230_2 as u128, 1u16);
        // D s_230_4: cmp-eq s_230_1 s_230_3
        let s_230_4: bool = ((s_230_1) == (s_230_3));
        // D s_230_5: write-var gs#112899 <= s_230_4
        fn_state.gs_112899 = s_230_4;
        // N s_230_6: jump b219
        return block_219(state, tracer, fn_state);
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
        // D s_231_3: not s_231_2
        let s_231_3: bool = !s_231_2;
        // D s_231_4: write-var gs#112898 <= s_231_3
        fn_state.gs_112898 = s_231_3;
        // N s_231_5: jump b217
        return block_217(state, tracer, fn_state);
    }
    fn block_232<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_232_0: read-var __PMUSERENR_EN:u8
        let s_232_0: bool = fn_state.u__PMUSERENR_EN;
        // D s_232_1: cast zx s_232_0 -> bv
        let s_232_1: Bits = Bits::new(s_232_0 as u128, 1u16);
        // C s_232_2: const #0u : u8
        let s_232_2: bool = false;
        // C s_232_3: cast zx s_232_2 -> bv
        let s_232_3: Bits = Bits::new(s_232_2 as u128, 1u16);
        // D s_232_4: cmp-eq s_232_1 s_232_3
        let s_232_4: bool = ((s_232_1) == (s_232_3));
        // D s_232_5: write-var gs#112873 <= s_232_4
        fn_state.gs_112873 = s_232_4;
        // N s_232_6: jump b121
        return block_121(state, tracer, fn_state);
    }
    fn block_233<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_233_0: const #() : ()
        let s_233_0: () = ();
        // S s_233_1: call EL2Enabled(s_233_0)
        let s_233_1: bool = EL2Enabled(state, tracer, s_233_0);
        // N s_233_2: branch s_233_1 b241 b234
        if s_233_1 {
            return block_241(state, tracer, fn_state);
        } else {
            return block_234(state, tracer, fn_state);
        };
    }
    fn block_234<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_234_0: const #0u : u8
        let s_234_0: bool = false;
        // D s_234_1: write-var gs#112902 <= s_234_0
        fn_state.gs_112902 = s_234_0;
        // N s_234_2: jump b235
        return block_235(state, tracer, fn_state);
    }
    fn block_235<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_235_0: read-var gs#112902:u8
        let s_235_0: bool = fn_state.gs_112902;
        // N s_235_1: branch s_235_0 b240 b236
        if s_235_0 {
            return block_240(state, tracer, fn_state);
        } else {
            return block_236(state, tracer, fn_state);
        };
    }
    fn block_236<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_236_0: const #0u : u8
        let s_236_0: bool = false;
        // D s_236_1: write-var gs#112903 <= s_236_0
        fn_state.gs_112903 = s_236_0;
        // N s_236_2: jump b237
        return block_237(state, tracer, fn_state);
    }
    fn block_237<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_237_0: read-var gs#112903:u8
        let s_237_0: bool = fn_state.gs_112903;
        // N s_237_1: branch s_237_0 b239 b238
        if s_237_0 {
            return block_239(state, tracer, fn_state);
        } else {
            return block_238(state, tracer, fn_state);
        };
    }
    fn block_238<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_238_0: const #3u : u8
        let s_238_0: u8 = 3;
        // C s_238_1: cast zx s_238_0 -> bv
        let s_238_1: Bits = Bits::new(s_238_0 as u128, 8u16);
        // C s_238_2: cast zx s_238_1 -> i
        let s_238_2: i128 = (s_238_1.value() as i128);
        // C s_238_3: cast reint s_238_2 -> i64
        let s_238_3: i64 = (s_238_2 as i64);
        // C s_238_4: cast zx s_238_3 -> i
        let s_238_4: i128 = (i128::try_from(s_238_3).unwrap());
        // C s_238_5: const #440u : u32
        let s_238_5: u32 = 440;
        // D s_238_6: read-reg s_238_5:u8
        let s_238_6: u8 = {
            let value = state.read_register::<u8>(s_238_5 as isize);
            tracer.read_register(s_238_5 as isize, value);
            value
        };
        // D s_238_7: call AArch64_AArch32SystemAccessTrap(s_238_6, s_238_4)
        let s_238_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_238_6,
            s_238_4,
        );
        // N s_238_8: return
        return;
    }
    fn block_239<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_239_0: const #3u : u8
        let s_239_0: u8 = 3;
        // C s_239_1: cast zx s_239_0 -> bv
        let s_239_1: Bits = Bits::new(s_239_0 as u128, 8u16);
        // C s_239_2: cast zx s_239_1 -> i
        let s_239_2: i128 = (s_239_1.value() as i128);
        // C s_239_3: cast reint s_239_2 -> i64
        let s_239_3: i64 = (s_239_2 as i64);
        // C s_239_4: cast zx s_239_3 -> i
        let s_239_4: i128 = (i128::try_from(s_239_3).unwrap());
        // C s_239_5: const #432u : u32
        let s_239_5: u32 = 432;
        // D s_239_6: read-reg s_239_5:u8
        let s_239_6: u8 = {
            let value = state.read_register::<u8>(s_239_5 as isize);
            tracer.read_register(s_239_5 as isize, value);
            value
        };
        // D s_239_7: call AArch64_AArch32SystemAccessTrap(s_239_6, s_239_4)
        let s_239_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_239_6,
            s_239_4,
        );
        // N s_239_8: return
        return;
    }
    fn block_240<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_240_0: read-var __HCR_EL2_TGE:u8
        let s_240_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_240_1: cast zx s_240_0 -> bv
        let s_240_1: Bits = Bits::new(s_240_0 as u128, 1u16);
        // C s_240_2: const #1u : u8
        let s_240_2: bool = true;
        // C s_240_3: cast zx s_240_2 -> bv
        let s_240_3: Bits = Bits::new(s_240_2 as u128, 1u16);
        // D s_240_4: cmp-eq s_240_1 s_240_3
        let s_240_4: bool = ((s_240_1) == (s_240_3));
        // D s_240_5: write-var gs#112903 <= s_240_4
        fn_state.gs_112903 = s_240_4;
        // N s_240_6: jump b237
        return block_237(state, tracer, fn_state);
    }
    fn block_241<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_241_0: const #432u : u32
        let s_241_0: u32 = 432;
        // D s_241_1: read-reg s_241_0:u8
        let s_241_1: u8 = {
            let value = state.read_register::<u8>(s_241_0 as isize);
            tracer.read_register(s_241_0 as isize, value);
            value
        };
        // D s_241_2: call ELUsingAArch32(s_241_1)
        let s_241_2: bool = ELUsingAArch32(state, tracer, s_241_1);
        // D s_241_3: not s_241_2
        let s_241_3: bool = !s_241_2;
        // D s_241_4: write-var gs#112902 <= s_241_3
        fn_state.gs_112902 = s_241_3;
        // N s_241_5: jump b235
        return block_235(state, tracer, fn_state);
    }
    fn block_242<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_242_0: read-var __PMUSERENR_EL0_TID:u8
        let s_242_0: bool = fn_state.u__PMUSERENR_EL0_TID;
        // D s_242_1: cast zx s_242_0 -> bv
        let s_242_1: Bits = Bits::new(s_242_0 as u128, 1u16);
        // C s_242_2: const #1u : u8
        let s_242_2: bool = true;
        // C s_242_3: cast zx s_242_2 -> bv
        let s_242_3: Bits = Bits::new(s_242_2 as u128, 1u16);
        // D s_242_4: cmp-eq s_242_1 s_242_3
        let s_242_4: bool = ((s_242_1) == (s_242_3));
        // D s_242_5: write-var gs#112872 <= s_242_4
        fn_state.gs_112872 = s_242_4;
        // N s_242_6: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_243<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_243_0: const #204u : u32
        let s_243_0: u32 = 204;
        // S s_243_1: call IsFeatureImplemented(s_243_0)
        let s_243_1: bool = IsFeatureImplemented(state, tracer, s_243_0);
        // D s_243_2: write-var gs#112871 <= s_243_1
        fn_state.gs_112871 = s_243_1;
        // N s_243_3: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_244<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_244_0: const #() : ()
        let s_244_0: () = ();
        // S s_244_1: call EL2Enabled(s_244_0)
        let s_244_1: bool = EL2Enabled(state, tracer, s_244_0);
        // N s_244_2: branch s_244_1 b252 b245
        if s_244_1 {
            return block_252(state, tracer, fn_state);
        } else {
            return block_245(state, tracer, fn_state);
        };
    }
    fn block_245<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_245_0: const #0u : u8
        let s_245_0: bool = false;
        // D s_245_1: write-var gs#112904 <= s_245_0
        fn_state.gs_112904 = s_245_0;
        // N s_245_2: jump b246
        return block_246(state, tracer, fn_state);
    }
    fn block_246<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_246_0: read-var gs#112904:u8
        let s_246_0: bool = fn_state.gs_112904;
        // N s_246_1: branch s_246_0 b251 b247
        if s_246_0 {
            return block_251(state, tracer, fn_state);
        } else {
            return block_247(state, tracer, fn_state);
        };
    }
    fn block_247<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_247_0: const #0u : u8
        let s_247_0: bool = false;
        // D s_247_1: write-var gs#112905 <= s_247_0
        fn_state.gs_112905 = s_247_0;
        // N s_247_2: jump b248
        return block_248(state, tracer, fn_state);
    }
    fn block_248<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_248_0: read-var gs#112905:u8
        let s_248_0: bool = fn_state.gs_112905;
        // N s_248_1: branch s_248_0 b250 b249
        if s_248_0 {
            return block_250(state, tracer, fn_state);
        } else {
            return block_249(state, tracer, fn_state);
        };
    }
    fn block_249<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_249_0: const #3u : u8
        let s_249_0: u8 = 3;
        // C s_249_1: cast zx s_249_0 -> bv
        let s_249_1: Bits = Bits::new(s_249_0 as u128, 8u16);
        // C s_249_2: cast zx s_249_1 -> i
        let s_249_2: i128 = (s_249_1.value() as i128);
        // C s_249_3: cast reint s_249_2 -> i64
        let s_249_3: i64 = (s_249_2 as i64);
        // C s_249_4: cast zx s_249_3 -> i
        let s_249_4: i128 = (i128::try_from(s_249_3).unwrap());
        // C s_249_5: const #440u : u32
        let s_249_5: u32 = 440;
        // D s_249_6: read-reg s_249_5:u8
        let s_249_6: u8 = {
            let value = state.read_register::<u8>(s_249_5 as isize);
            tracer.read_register(s_249_5 as isize, value);
            value
        };
        // D s_249_7: call AArch64_AArch32SystemAccessTrap(s_249_6, s_249_4)
        let s_249_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_249_6,
            s_249_4,
        );
        // N s_249_8: return
        return;
    }
    fn block_250<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_250_0: const #3u : u8
        let s_250_0: u8 = 3;
        // C s_250_1: cast zx s_250_0 -> bv
        let s_250_1: Bits = Bits::new(s_250_0 as u128, 8u16);
        // C s_250_2: cast zx s_250_1 -> i
        let s_250_2: i128 = (s_250_1.value() as i128);
        // C s_250_3: cast reint s_250_2 -> i64
        let s_250_3: i64 = (s_250_2 as i64);
        // C s_250_4: cast zx s_250_3 -> i
        let s_250_4: i128 = (i128::try_from(s_250_3).unwrap());
        // C s_250_5: const #432u : u32
        let s_250_5: u32 = 432;
        // D s_250_6: read-reg s_250_5:u8
        let s_250_6: u8 = {
            let value = state.read_register::<u8>(s_250_5 as isize);
            tracer.read_register(s_250_5 as isize, value);
            value
        };
        // D s_250_7: call AArch64_AArch32SystemAccessTrap(s_250_6, s_250_4)
        let s_250_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_250_6,
            s_250_4,
        );
        // N s_250_8: return
        return;
    }
    fn block_251<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_251_0: read-var __HCR_EL2_TGE:u8
        let s_251_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_251_1: cast zx s_251_0 -> bv
        let s_251_1: Bits = Bits::new(s_251_0 as u128, 1u16);
        // C s_251_2: const #1u : u8
        let s_251_2: bool = true;
        // C s_251_3: cast zx s_251_2 -> bv
        let s_251_3: Bits = Bits::new(s_251_2 as u128, 1u16);
        // D s_251_4: cmp-eq s_251_1 s_251_3
        let s_251_4: bool = ((s_251_1) == (s_251_3));
        // D s_251_5: write-var gs#112905 <= s_251_4
        fn_state.gs_112905 = s_251_4;
        // N s_251_6: jump b248
        return block_248(state, tracer, fn_state);
    }
    fn block_252<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_252_0: const #432u : u32
        let s_252_0: u32 = 432;
        // D s_252_1: read-reg s_252_0:u8
        let s_252_1: u8 = {
            let value = state.read_register::<u8>(s_252_0 as isize);
            tracer.read_register(s_252_0 as isize, value);
            value
        };
        // D s_252_2: call ELUsingAArch32(s_252_1)
        let s_252_2: bool = ELUsingAArch32(state, tracer, s_252_1);
        // D s_252_3: not s_252_2
        let s_252_3: bool = !s_252_2;
        // D s_252_4: write-var gs#112904 <= s_252_3
        fn_state.gs_112904 = s_252_3;
        // N s_252_5: jump b246
        return block_246(state, tracer, fn_state);
    }
    fn block_253<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_253_0: const #204u : u32
        let s_253_0: u32 = 204;
        // S s_253_1: call IsFeatureImplemented(s_253_0)
        let s_253_1: bool = IsFeatureImplemented(state, tracer, s_253_0);
        // N s_253_2: branch s_253_1 b262 b254
        if s_253_1 {
            return block_262(state, tracer, fn_state);
        } else {
            return block_254(state, tracer, fn_state);
        };
    }
    fn block_254<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_254_0: const #0u : u8
        let s_254_0: bool = false;
        // D s_254_1: write-var gs#112867 <= s_254_0
        fn_state.gs_112867 = s_254_0;
        // N s_254_2: jump b255
        return block_255(state, tracer, fn_state);
    }
    fn block_255<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_255_0: read-var gs#112867:u8
        let s_255_0: bool = fn_state.gs_112867;
        // N s_255_1: branch s_255_0 b261 b256
        if s_255_0 {
            return block_261(state, tracer, fn_state);
        } else {
            return block_256(state, tracer, fn_state);
        };
    }
    fn block_256<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_256_0: const #204u : u32
        let s_256_0: u32 = 204;
        // S s_256_1: call IsFeatureImplemented(s_256_0)
        let s_256_1: bool = IsFeatureImplemented(state, tracer, s_256_0);
        // S s_256_2: not s_256_1
        let s_256_2: bool = !s_256_1;
        // N s_256_3: branch s_256_2 b260 b257
        if s_256_2 {
            return block_260(state, tracer, fn_state);
        } else {
            return block_257(state, tracer, fn_state);
        };
    }
    fn block_257<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_257_0: const #0u : u8
        let s_257_0: bool = false;
        // D s_257_1: write-var gs#112868 <= s_257_0
        fn_state.gs_112868 = s_257_0;
        // N s_257_2: jump b258
        return block_258(state, tracer, fn_state);
    }
    fn block_258<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_258_0: read-var gs#112868:u8
        let s_258_0: bool = fn_state.gs_112868;
        // D s_258_1: write-var gs#112869 <= s_258_0
        fn_state.gs_112869 = s_258_0;
        // N s_258_2: jump b259
        return block_259(state, tracer, fn_state);
    }
    fn block_259<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_259_0: read-var gs#112869:u8
        let s_259_0: bool = fn_state.gs_112869;
        // D s_259_1: write-var gs#112870 <= s_259_0
        fn_state.gs_112870 = s_259_0;
        // N s_259_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_260<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_260_0: read-var __PMUSERENR_EL0_EN:u8
        let s_260_0: bool = fn_state.u__PMUSERENR_EL0_EN;
        // D s_260_1: cast zx s_260_0 -> bv
        let s_260_1: Bits = Bits::new(s_260_0 as u128, 1u16);
        // C s_260_2: const #0u : u8
        let s_260_2: bool = false;
        // C s_260_3: cast zx s_260_2 -> bv
        let s_260_3: Bits = Bits::new(s_260_2 as u128, 1u16);
        // D s_260_4: cmp-eq s_260_1 s_260_3
        let s_260_4: bool = ((s_260_1) == (s_260_3));
        // D s_260_5: write-var gs#112868 <= s_260_4
        fn_state.gs_112868 = s_260_4;
        // N s_260_6: jump b258
        return block_258(state, tracer, fn_state);
    }
    fn block_261<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_261_0: const #1u : u8
        let s_261_0: bool = true;
        // D s_261_1: write-var gs#112869 <= s_261_0
        fn_state.gs_112869 = s_261_0;
        // N s_261_2: jump b259
        return block_259(state, tracer, fn_state);
    }
    fn block_262<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_262_0: const #102624u : u32
        let s_262_0: u32 = 102624;
        // D s_262_1: read-reg s_262_0:struct
        let s_262_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_262_0 as isize);
            tracer.read_register(s_262_0 as isize, value);
            value
        };
        // D s_262_2: call _get_PMUSERENR_EL0_Type_UEN(s_262_1)
        let s_262_2: bool = u_get_PMUSERENR_EL0_Type_UEN(state, tracer, s_262_1);
        // C s_262_3: const #102624u : u32
        let s_262_3: u32 = 102624;
        // D s_262_4: read-reg s_262_3:struct
        let s_262_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_262_3 as isize);
            tracer.read_register(s_262_3 as isize, value);
            value
        };
        // D s_262_5: call _get_PMUSERENR_EL0_Type_EN(s_262_4)
        let s_262_5: bool = u_get_PMUSERENR_EL0_Type_EN(state, tracer, s_262_4);
        // D s_262_6: cast zx s_262_2 -> bv
        let s_262_6: Bits = Bits::new(s_262_2 as u128, 1u16);
        // D s_262_7: cast zx s_262_5 -> bv
        let s_262_7: Bits = Bits::new(s_262_5 as u128, 1u16);
        // D s_262_8: cast reint s_262_6 -> u128
        let s_262_8: u128 = (s_262_6.value() as u128);
        // D s_262_9: size-of s_262_6
        let s_262_9: u16 = s_262_6.length();
        // D s_262_10: cast reint s_262_7 -> u128
        let s_262_10: u128 = (s_262_7.value() as u128);
        // D s_262_11: size-of s_262_7
        let s_262_11: u16 = s_262_7.length();
        // D s_262_12: lsl s_262_8 s_262_11
        let s_262_12: u128 = s_262_8 << s_262_11;
        // D s_262_13: or s_262_12 s_262_10
        let s_262_13: u128 = ((s_262_12) | (s_262_10));
        // D s_262_14: add s_262_9 s_262_11
        let s_262_14: u16 = (s_262_9 + s_262_11);
        // D s_262_15: create-bits s_262_13 s_262_14
        let s_262_15: Bits = Bits::new(s_262_13, s_262_14);
        // D s_262_16: cast reint s_262_15 -> u8
        let s_262_16: u8 = (s_262_15.value() as u8);
        // D s_262_17: cast zx s_262_16 -> bv
        let s_262_17: Bits = Bits::new(s_262_16 as u128, 2u16);
        // C s_262_18: const #0u : u8
        let s_262_18: u8 = 0;
        // C s_262_19: cast zx s_262_18 -> bv
        let s_262_19: Bits = Bits::new(s_262_18 as u128, 2u16);
        // D s_262_20: cmp-eq s_262_17 s_262_19
        let s_262_20: bool = ((s_262_17) == (s_262_19));
        // D s_262_21: write-var gs#112867 <= s_262_20
        fn_state.gs_112867 = s_262_20;
        // N s_262_22: jump b255
        return block_255(state, tracer, fn_state);
    }
    fn block_263<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_263_0: panic
        panic!("{:?}", ());
        // N s_263_1: return
        return;
    }
    fn block_264<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_264_0: read-var __MDCR_EL3_TPM:u8
        let s_264_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_264_1: cast zx s_264_0 -> bv
        let s_264_1: Bits = Bits::new(s_264_0 as u128, 1u16);
        // C s_264_2: const #1u : u8
        let s_264_2: bool = true;
        // C s_264_3: cast zx s_264_2 -> bv
        let s_264_3: Bits = Bits::new(s_264_2 as u128, 1u16);
        // D s_264_4: cmp-eq s_264_1 s_264_3
        let s_264_4: bool = ((s_264_1) == (s_264_3));
        // D s_264_5: write-var gs#112866 <= s_264_4
        fn_state.gs_112866 = s_264_4;
        // N s_264_6: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_265<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_265_0: const #424u : u32
        let s_265_0: u32 = 424;
        // D s_265_1: read-reg s_265_0:u8
        let s_265_1: u8 = {
            let value = state.read_register::<u8>(s_265_0 as isize);
            tracer.read_register(s_265_0 as isize, value);
            value
        };
        // D s_265_2: call ELUsingAArch32(s_265_1)
        let s_265_2: bool = ELUsingAArch32(state, tracer, s_265_1);
        // D s_265_3: not s_265_2
        let s_265_3: bool = !s_265_2;
        // D s_265_4: write-var gs#112865 <= s_265_3
        fn_state.gs_112865 = s_265_3;
        // N s_265_5: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_266<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_266_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_266_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_266_1: call __IMPDEF_boolean(s_266_0)
        let s_266_1: bool = u__IMPDEF_boolean(state, tracer, s_266_0);
        // D s_266_2: write-var gs#112864 <= s_266_1
        fn_state.gs_112864 = s_266_1;
        // N s_266_3: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_267<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_267_0: const #() : ()
        let s_267_0: () = ();
        // S s_267_1: call EDSCR_read(s_267_0)
        let s_267_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_267_0);
        // S s_267_2: call _get_EDSCR_Type_SDD(s_267_1)
        let s_267_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_267_1);
        // S s_267_3: cast zx s_267_2 -> bv
        let s_267_3: Bits = Bits::new(s_267_2 as u128, 1u16);
        // C s_267_4: const #1u : u8
        let s_267_4: bool = true;
        // C s_267_5: cast zx s_267_4 -> bv
        let s_267_5: Bits = Bits::new(s_267_4 as u128, 1u16);
        // S s_267_6: cmp-eq s_267_3 s_267_5
        let s_267_6: bool = ((s_267_3) == (s_267_5));
        // D s_267_7: write-var gs#112863 <= s_267_6
        fn_state.gs_112863 = s_267_6;
        // N s_267_8: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_268<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_268_0: const #424u : u32
        let s_268_0: u32 = 424;
        // D s_268_1: read-reg s_268_0:u8
        let s_268_1: u8 = {
            let value = state.read_register::<u8>(s_268_0 as isize);
            tracer.read_register(s_268_0 as isize, value);
            value
        };
        // C s_268_2: const #2u : u8
        let s_268_2: u8 = 2;
        // D s_268_3: cmp-lt s_268_1 s_268_2
        let s_268_3: bool = ((s_268_1) < (s_268_2));
        // D s_268_4: write-var gs#112862 <= s_268_3
        fn_state.gs_112862 = s_268_3;
        // N s_268_5: jump b102
        return block_102(state, tracer, fn_state);
    }
}
