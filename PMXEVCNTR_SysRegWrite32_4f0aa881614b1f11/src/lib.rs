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
use PMSELR_read::*;
use u_get_SCR_EL3_Type_FGTEn::*;
use Halted::*;
use u_get_HSTR_Type_T9::*;
use u__set_selected_PMEVCNTR::*;
use u_get_MDCR_EL3_Type_TPM::*;
use AArch32_GetNumEventCountersAccessible::*;
use R_read::*;
use u_get_PMUSERENR_Type_EN::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_HSTR_EL2_Type_T9::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use u_get_MDCR_EL2_Type_TPM::*;
use u_get_PMUSERENR_EL0_Type_UEN::*;
use u_get_HCR_EL2_Type_E2H::*;
use HCR_read::*;
use PMUSERENR_read::*;
use HDCR_read::*;
use u_get_HDFGWTR_EL2_Type_PMEVCNTRn_EL0::*;
use IsFeatureImplemented::*;
use u_get_PMSELR_Type_SEL::*;
use u__IMPDEF_boolean::*;
use AArch64_AArch32SystemAccessTrap::*;
use HSTR_read::*;
use u_get_HCR_Type_TGE::*;
use u_get_HDCR_Type_TPM::*;
use ConstrainUnpredictableProcedure::*;
use ELUsingAArch32::*;
use u_get_PMUSERENR_EL0_Type_EN::*;
use EDSCR_read::*;
use AArch32_TakeHypTrapException::*;
use common::*;
pub fn PMXEVCNTR_SysRegWrite32_4f0aa881614b1f11<T: Tracer>(
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
        gs_133889: bool,
        gs_133883: bool,
        gs_133871: bool,
        gs_133916: bool,
        gs_133918: bool,
        gs_133911: bool,
        gs_133924: bool,
        gs_133895: bool,
        gs_133905: bool,
        u__MDCR_EL3_TPM: bool,
        gs_133915: bool,
        gs_133923: bool,
        gs_133881: bool,
        gs_133901: bool,
        gs_133921: bool,
        u__HCR_TGE: bool,
        gs_133927: bool,
        gs_133913: bool,
        gs_133902: bool,
        gs_133920: bool,
        u__HDFGWTR_EL2_PMEVCNTRn_EL0: bool,
        gs_133925: bool,
        gs_133884: bool,
        gs_133922: bool,
        gs_133882: bool,
        gs_133886: bool,
        u__PSTATE_EL: u8,
        gs_133910: bool,
        gs_133890: bool,
        gs_133904: bool,
        gs_133878: bool,
        u__MDCR_EL2_TPM: bool,
        gs_133906: bool,
        u__PMUSERENR_EN: bool,
        gs_133873: bool,
        gs_133876: bool,
        u__HCR_EL2_TGE: bool,
        gs_133894: bool,
        gs_133914: bool,
        gs_133926: bool,
        gs_133917: bool,
        gs_133912: bool,
        gs_133870: bool,
        u__SCR_EL3_FGTEn: bool,
        gs_133892: bool,
        gs_133877: bool,
        u__PMUSERENR_EL0_EN: bool,
        u__HSTR_T9: bool,
        gs_133896: bool,
        gs_133919: bool,
        u__HDCR_TPM: bool,
        gs_133875: bool,
        gs_133893: bool,
        gs_133897: bool,
        gs_133900: bool,
        gs_133880: bool,
        gs_133891: bool,
        gs_133899: bool,
        gs_133909: bool,
        gs_133879: bool,
        gs_133885: bool,
        gs_133903: bool,
        gs_133907: bool,
        gs_133874: bool,
        gs_133888: bool,
        gs_133887: bool,
        gs_133869: bool,
        gs_133908: bool,
        gs_133928: bool,
        gs_133898: bool,
        gs_133872: bool,
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
        // D s_0_37: call _get_HDFGWTR_EL2_Type_PMEVCNTRn_EL0(s_0_36)
        let s_0_37: bool = u_get_HDFGWTR_EL2_Type_PMEVCNTRn_EL0(state, tracer, s_0_36);
        // D s_0_38: write-var __HDFGWTR_EL2_PMEVCNTRn_EL0 <= s_0_37
        fn_state.u__HDFGWTR_EL2_PMEVCNTRn_EL0 = s_0_37;
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
        // C s_0_43: const #() : ()
        let s_0_43: () = ();
        // S s_0_44: call HDCR_read(s_0_43)
        let s_0_44: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_0_43);
        // S s_0_45: call _get_HDCR_Type_TPM(s_0_44)
        let s_0_45: bool = u_get_HDCR_Type_TPM(state, tracer, s_0_44);
        // D s_0_46: write-var __HDCR_TPM <= s_0_45
        fn_state.u__HDCR_TPM = s_0_45;
        // C s_0_47: const #() : ()
        let s_0_47: () = ();
        // S s_0_48: call PMSELR_read(s_0_47)
        let s_0_48: ProductType700c18a878c5601b = PMSELR_read(state, tracer, s_0_47);
        // S s_0_49: call _get_PMSELR_Type_SEL(s_0_48)
        let s_0_49: u8 = u_get_PMSELR_Type_SEL(state, tracer, s_0_48);
        // S s_0_50: cast zx s_0_49 -> bv
        let s_0_50: Bits = Bits::new(s_0_49 as u128, 5u16);
        // S s_0_51: cast zx s_0_50 -> i
        let s_0_51: i128 = (s_0_50.value() as i128);
        // S s_0_52: cast reint s_0_51 -> i64
        let s_0_52: i64 = (s_0_51 as i64);
        // C s_0_53: const #31s : i
        let s_0_53: i128 = 31;
        // S s_0_54: cast zx s_0_52 -> i
        let s_0_54: i128 = (i128::try_from(s_0_52).unwrap());
        // S s_0_55: cmp-ge s_0_54 s_0_53
        let s_0_55: bool = ((s_0_54) >= (s_0_53));
        // N s_0_56: branch s_0_55 b248 b1
        if s_0_55 {
            return block_248(state, tracer, fn_state);
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
        // C s_1_2: const #448u : u32
        let s_1_2: u32 = 448;
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
        // N s_1_6: branch s_1_5 b110 b2
        if s_1_5 {
            return block_110(state, tracer, fn_state);
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
        // C s_2_2: const #440u : u32
        let s_2_2: u32 = 440;
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
        // N s_2_6: branch s_2_5 b38 b3
        if s_2_5 {
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
        // D s_3_0: read-var __PSTATE_EL:u8
        let s_3_0: u8 = fn_state.u__PSTATE_EL;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_2: const #432u : u32
        let s_3_2: u32 = 432;
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
        // N s_3_6: branch s_3_5 b7 b4
        if s_3_5 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var __PSTATE_EL:u8
        let s_4_0: u8 = fn_state.u__PSTATE_EL;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 2u16);
        // C s_4_2: const #424u : u32
        let s_4_2: u32 = 424;
        // D s_4_3: read-reg s_4_2:u8
        let s_4_3: u8 = {
            let value = state.read_register::<u8>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 2u16);
        // D s_4_5: cmp-eq s_4_1 s_4_4
        let s_4_5: bool = ((s_4_1) == (s_4_4));
        // N s_4_6: branch s_4_5 b6 b5
        if s_4_5 {
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
        // N s_5_0: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var t:i
        let s_6_0: i128 = fn_state.t;
        // D s_6_1: call R_read(s_6_0)
        let s_6_1: u32 = R_read(state, tracer, s_6_0);
        // D s_6_2: call __set_selected_PMEVCNTR(s_6_1)
        let s_6_2: () = u__set_selected_PMEVCNTR(state, tracer, s_6_1);
        // N s_6_3: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call Halted(s_7_0)
        let s_7_1: bool = Halted(state, tracer, s_7_0);
        // N s_7_2: branch s_7_1 b37 b8
        if s_7_1 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#133869 <= s_8_0
        fn_state.gs_133869 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#133869:u8
        let s_9_0: bool = fn_state.gs_133869;
        // N s_9_1: branch s_9_0 b36 b10
        if s_9_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#133870 <= s_10_0
        fn_state.gs_133870 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#133870:u8
        let s_11_0: bool = fn_state.gs_133870;
        // N s_11_1: branch s_11_0 b35 b12
        if s_11_0 {
            return block_35(state, tracer, fn_state);
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
        // D s_12_1: write-var gs#133871 <= s_12_0
        fn_state.gs_133871 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#133871:u8
        let s_13_0: bool = fn_state.gs_133871;
        // N s_13_1: branch s_13_0 b34 b14
        if s_13_0 {
            return block_34(state, tracer, fn_state);
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
        // D s_14_1: write-var gs#133872 <= s_14_0
        fn_state.gs_133872 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#133872:u8
        let s_15_0: bool = fn_state.gs_133872;
        // N s_15_1: branch s_15_0 b33 b16
        if s_15_0 {
            return block_33(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#133873 <= s_16_0
        fn_state.gs_133873 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#133873:u8
        let s_17_0: bool = fn_state.gs_133873;
        // N s_17_1: branch s_17_0 b32 b18
        if s_17_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #424u : u32
        let s_18_0: u32 = 424;
        // D s_18_1: read-reg s_18_0:u8
        let s_18_1: u8 = {
            let value = state.read_register::<u8>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // C s_18_2: const #2u : u8
        let s_18_2: u8 = 2;
        // D s_18_3: cmp-lt s_18_1 s_18_2
        let s_18_3: bool = ((s_18_1) < (s_18_2));
        // N s_18_4: branch s_18_3 b31 b19
        if s_18_3 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#133874 <= s_19_0
        fn_state.gs_133874 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#133874:u8
        let s_20_0: bool = fn_state.gs_133874;
        // N s_20_1: branch s_20_0 b30 b21
        if s_20_0 {
            return block_30(state, tracer, fn_state);
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
        // D s_21_1: write-var gs#133875 <= s_21_0
        fn_state.gs_133875 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#133875:u8
        let s_22_0: bool = fn_state.gs_133875;
        // N s_22_1: branch s_22_0 b24 b23
        if s_22_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var t:i
        let s_23_0: i128 = fn_state.t;
        // D s_23_1: call R_read(s_23_0)
        let s_23_1: u32 = R_read(state, tracer, s_23_0);
        // D s_23_2: call __set_selected_PMEVCNTR(s_23_1)
        let s_23_2: () = u__set_selected_PMEVCNTR(state, tracer, s_23_1);
        // N s_23_3: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call Halted(s_24_0)
        let s_24_1: bool = Halted(state, tracer, s_24_0);
        // N s_24_2: branch s_24_1 b29 b25
        if s_24_1 {
            return block_29(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#133876 <= s_25_0
        fn_state.gs_133876 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#133876:u8
        let s_26_0: bool = fn_state.gs_133876;
        // N s_26_1: branch s_26_0 b28 b27
        if s_26_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #3u : u8
        let s_27_0: u8 = 3;
        // C s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 8u16);
        // C s_27_2: cast zx s_27_1 -> i
        let s_27_2: i128 = (s_27_1.value() as i128);
        // C s_27_3: cast reint s_27_2 -> i64
        let s_27_3: i64 = (s_27_2 as i64);
        // C s_27_4: cast zx s_27_3 -> i
        let s_27_4: i128 = (i128::try_from(s_27_3).unwrap());
        // C s_27_5: const #424u : u32
        let s_27_5: u32 = 424;
        // D s_27_6: read-reg s_27_5:u8
        let s_27_6: u8 = {
            let value = state.read_register::<u8>(s_27_5 as isize);
            tracer.read_register(s_27_5 as isize, value);
            value
        };
        // D s_27_7: call AArch64_AArch32SystemAccessTrap(s_27_6, s_27_4)
        let s_27_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_27_6, s_27_4);
        // N s_27_8: return
        return;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_28_0: panic
        panic!("{:?}", ());
        // N s_28_1: return
        return;
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #() : ()
        let s_29_0: () = ();
        // S s_29_1: call EDSCR_read(s_29_0)
        let s_29_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_29_0);
        // S s_29_2: call _get_EDSCR_Type_SDD(s_29_1)
        let s_29_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_29_1);
        // S s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // C s_29_4: const #1u : u8
        let s_29_4: bool = true;
        // C s_29_5: cast zx s_29_4 -> bv
        let s_29_5: Bits = Bits::new(s_29_4 as u128, 1u16);
        // S s_29_6: cmp-eq s_29_3 s_29_5
        let s_29_6: bool = ((s_29_3) == (s_29_5));
        // D s_29_7: write-var gs#133876 <= s_29_6
        fn_state.gs_133876 = s_29_6;
        // N s_29_8: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var __MDCR_EL3_TPM:u8
        let s_30_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 1u16);
        // C s_30_2: const #1u : u8
        let s_30_2: bool = true;
        // C s_30_3: cast zx s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 1u16);
        // D s_30_4: cmp-eq s_30_1 s_30_3
        let s_30_4: bool = ((s_30_1) == (s_30_3));
        // D s_30_5: write-var gs#133875 <= s_30_4
        fn_state.gs_133875 = s_30_4;
        // N s_30_6: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #424u : u32
        let s_31_0: u32 = 424;
        // D s_31_1: read-reg s_31_0:u8
        let s_31_1: u8 = {
            let value = state.read_register::<u8>(s_31_0 as isize);
            tracer.read_register(s_31_0 as isize, value);
            value
        };
        // D s_31_2: call ELUsingAArch32(s_31_1)
        let s_31_2: bool = ELUsingAArch32(state, tracer, s_31_1);
        // D s_31_3: not s_31_2
        let s_31_3: bool = !s_31_2;
        // D s_31_4: write-var gs#133874 <= s_31_3
        fn_state.gs_133874 = s_31_3;
        // N s_31_5: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_32_0: panic
        panic!("{:?}", ());
        // N s_32_1: return
        return;
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var __MDCR_EL3_TPM:u8
        let s_33_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 1u16);
        // C s_33_2: const #1u : u8
        let s_33_2: bool = true;
        // C s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 1u16);
        // D s_33_4: cmp-eq s_33_1 s_33_3
        let s_33_4: bool = ((s_33_1) == (s_33_3));
        // D s_33_5: write-var gs#133873 <= s_33_4
        fn_state.gs_133873 = s_33_4;
        // N s_33_6: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #424u : u32
        let s_34_0: u32 = 424;
        // D s_34_1: read-reg s_34_0:u8
        let s_34_1: u8 = {
            let value = state.read_register::<u8>(s_34_0 as isize);
            tracer.read_register(s_34_0 as isize, value);
            value
        };
        // D s_34_2: call ELUsingAArch32(s_34_1)
        let s_34_2: bool = ELUsingAArch32(state, tracer, s_34_1);
        // D s_34_3: not s_34_2
        let s_34_3: bool = !s_34_2;
        // D s_34_4: write-var gs#133872 <= s_34_3
        fn_state.gs_133872 = s_34_3;
        // N s_34_5: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_35_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_35_1: call __IMPDEF_boolean(s_35_0)
        let s_35_1: bool = u__IMPDEF_boolean(state, tracer, s_35_0);
        // D s_35_2: write-var gs#133871 <= s_35_1
        fn_state.gs_133871 = s_35_1;
        // N s_35_3: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #() : ()
        let s_36_0: () = ();
        // S s_36_1: call EDSCR_read(s_36_0)
        let s_36_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_36_0);
        // S s_36_2: call _get_EDSCR_Type_SDD(s_36_1)
        let s_36_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_36_1);
        // S s_36_3: cast zx s_36_2 -> bv
        let s_36_3: Bits = Bits::new(s_36_2 as u128, 1u16);
        // C s_36_4: const #1u : u8
        let s_36_4: bool = true;
        // C s_36_5: cast zx s_36_4 -> bv
        let s_36_5: Bits = Bits::new(s_36_4 as u128, 1u16);
        // S s_36_6: cmp-eq s_36_3 s_36_5
        let s_36_6: bool = ((s_36_3) == (s_36_5));
        // D s_36_7: write-var gs#133870 <= s_36_6
        fn_state.gs_133870 = s_36_6;
        // N s_36_8: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #424u : u32
        let s_37_0: u32 = 424;
        // D s_37_1: read-reg s_37_0:u8
        let s_37_1: u8 = {
            let value = state.read_register::<u8>(s_37_0 as isize);
            tracer.read_register(s_37_0 as isize, value);
            value
        };
        // C s_37_2: const #2u : u8
        let s_37_2: u8 = 2;
        // D s_37_3: cmp-lt s_37_1 s_37_2
        let s_37_3: bool = ((s_37_1) < (s_37_2));
        // D s_37_4: write-var gs#133869 <= s_37_3
        fn_state.gs_133869 = s_37_3;
        // N s_37_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #() : ()
        let s_38_0: () = ();
        // S s_38_1: call Halted(s_38_0)
        let s_38_1: bool = Halted(state, tracer, s_38_0);
        // N s_38_2: branch s_38_1 b109 b39
        if s_38_1 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #0u : u8
        let s_39_0: bool = false;
        // D s_39_1: write-var gs#133877 <= s_39_0
        fn_state.gs_133877 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#133877:u8
        let s_40_0: bool = fn_state.gs_133877;
        // N s_40_1: branch s_40_0 b108 b41
        if s_40_0 {
            return block_108(state, tracer, fn_state);
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
        // D s_41_1: write-var gs#133878 <= s_41_0
        fn_state.gs_133878 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#133878:u8
        let s_42_0: bool = fn_state.gs_133878;
        // N s_42_1: branch s_42_0 b107 b43
        if s_42_0 {
            return block_107(state, tracer, fn_state);
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
        // D s_43_1: write-var gs#133879 <= s_43_0
        fn_state.gs_133879 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#133879:u8
        let s_44_0: bool = fn_state.gs_133879;
        // N s_44_1: branch s_44_0 b106 b45
        if s_44_0 {
            return block_106(state, tracer, fn_state);
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
        // D s_45_1: write-var gs#133880 <= s_45_0
        fn_state.gs_133880 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#133880:u8
        let s_46_0: bool = fn_state.gs_133880;
        // N s_46_1: branch s_46_0 b105 b47
        if s_46_0 {
            return block_105(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #0u : u8
        let s_47_0: bool = false;
        // D s_47_1: write-var gs#133881 <= s_47_0
        fn_state.gs_133881 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#133881:u8
        let s_48_0: bool = fn_state.gs_133881;
        // N s_48_1: branch s_48_0 b104 b49
        if s_48_0 {
            return block_104(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #() : ()
        let s_49_0: () = ();
        // S s_49_1: call EL2Enabled(s_49_0)
        let s_49_1: bool = EL2Enabled(state, tracer, s_49_0);
        // N s_49_2: branch s_49_1 b103 b50
        if s_49_1 {
            return block_103(state, tracer, fn_state);
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
        // D s_50_1: write-var gs#133882 <= s_50_0
        fn_state.gs_133882 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#133882:u8
        let s_51_0: bool = fn_state.gs_133882;
        // N s_51_1: branch s_51_0 b102 b52
        if s_51_0 {
            return block_102(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #0u : u8
        let s_52_0: bool = false;
        // D s_52_1: write-var gs#133883 <= s_52_0
        fn_state.gs_133883 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#133883:u8
        let s_53_0: bool = fn_state.gs_133883;
        // N s_53_1: branch s_53_0 b101 b54
        if s_53_0 {
            return block_101(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #() : ()
        let s_54_0: () = ();
        // S s_54_1: call EL2Enabled(s_54_0)
        let s_54_1: bool = EL2Enabled(state, tracer, s_54_0);
        // N s_54_2: branch s_54_1 b100 b55
        if s_54_1 {
            return block_100(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #0u : u8
        let s_55_0: bool = false;
        // D s_55_1: write-var gs#133884 <= s_55_0
        fn_state.gs_133884 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#133884:u8
        let s_56_0: bool = fn_state.gs_133884;
        // N s_56_1: branch s_56_0 b99 b57
        if s_56_0 {
            return block_99(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #0u : u8
        let s_57_0: bool = false;
        // D s_57_1: write-var gs#133885 <= s_57_0
        fn_state.gs_133885 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var gs#133885:u8
        let s_58_0: bool = fn_state.gs_133885;
        // N s_58_1: branch s_58_0 b98 b59
        if s_58_0 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #() : ()
        let s_59_0: () = ();
        // S s_59_1: call EL2Enabled(s_59_0)
        let s_59_1: bool = EL2Enabled(state, tracer, s_59_0);
        // N s_59_2: branch s_59_1 b97 b60
        if s_59_1 {
            return block_97(state, tracer, fn_state);
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
        // D s_60_1: write-var gs#133886 <= s_60_0
        fn_state.gs_133886 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#133886:u8
        let s_61_0: bool = fn_state.gs_133886;
        // N s_61_1: branch s_61_0 b96 b62
        if s_61_0 {
            return block_96(state, tracer, fn_state);
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
        // D s_62_1: write-var gs#133887 <= s_62_0
        fn_state.gs_133887 = s_62_0;
        // N s_62_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var gs#133887:u8
        let s_63_0: bool = fn_state.gs_133887;
        // N s_63_1: branch s_63_0 b95 b64
        if s_63_0 {
            return block_95(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #() : ()
        let s_64_0: () = ();
        // S s_64_1: call EL2Enabled(s_64_0)
        let s_64_1: bool = EL2Enabled(state, tracer, s_64_0);
        // N s_64_2: branch s_64_1 b94 b65
        if s_64_1 {
            return block_94(state, tracer, fn_state);
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
        // D s_65_1: write-var gs#133888 <= s_65_0
        fn_state.gs_133888 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#133888:u8
        let s_66_0: bool = fn_state.gs_133888;
        // N s_66_1: branch s_66_0 b93 b67
        if s_66_0 {
            return block_93(state, tracer, fn_state);
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
        // D s_67_1: write-var gs#133889 <= s_67_0
        fn_state.gs_133889 = s_67_0;
        // N s_67_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var gs#133889:u8
        let s_68_0: bool = fn_state.gs_133889;
        // N s_68_1: branch s_68_0 b92 b69
        if s_68_0 {
            return block_92(state, tracer, fn_state);
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
        // S s_69_1: call EL2Enabled(s_69_0)
        let s_69_1: bool = EL2Enabled(state, tracer, s_69_0);
        // N s_69_2: branch s_69_1 b91 b70
        if s_69_1 {
            return block_91(state, tracer, fn_state);
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
        // D s_70_1: write-var gs#133890 <= s_70_0
        fn_state.gs_133890 = s_70_0;
        // N s_70_2: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var gs#133890:u8
        let s_71_0: bool = fn_state.gs_133890;
        // N s_71_1: branch s_71_0 b86 b72
        if s_71_0 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_72(state, tracer, fn_state);
        };
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #424u : u32
        let s_72_0: u32 = 424;
        // D s_72_1: read-reg s_72_0:u8
        let s_72_1: u8 = {
            let value = state.read_register::<u8>(s_72_0 as isize);
            tracer.read_register(s_72_0 as isize, value);
            value
        };
        // C s_72_2: const #2u : u8
        let s_72_2: u8 = 2;
        // D s_72_3: cmp-lt s_72_1 s_72_2
        let s_72_3: bool = ((s_72_1) < (s_72_2));
        // N s_72_4: branch s_72_3 b85 b73
        if s_72_3 {
            return block_85(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #0u : u8
        let s_73_0: bool = false;
        // D s_73_1: write-var gs#133891 <= s_73_0
        fn_state.gs_133891 = s_73_0;
        // N s_73_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var gs#133891:u8
        let s_74_0: bool = fn_state.gs_133891;
        // N s_74_1: branch s_74_0 b84 b75
        if s_74_0 {
            return block_84(state, tracer, fn_state);
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
        // D s_75_1: write-var gs#133892 <= s_75_0
        fn_state.gs_133892 = s_75_0;
        // N s_75_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var gs#133892:u8
        let s_76_0: bool = fn_state.gs_133892;
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
        // D s_77_0: read-var t:i
        let s_77_0: i128 = fn_state.t;
        // D s_77_1: call R_read(s_77_0)
        let s_77_1: u32 = R_read(state, tracer, s_77_0);
        // D s_77_2: call __set_selected_PMEVCNTR(s_77_1)
        let s_77_2: () = u__set_selected_PMEVCNTR(state, tracer, s_77_1);
        // N s_77_3: return
        return;
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #() : ()
        let s_78_0: () = ();
        // S s_78_1: call Halted(s_78_0)
        let s_78_1: bool = Halted(state, tracer, s_78_0);
        // N s_78_2: branch s_78_1 b83 b79
        if s_78_1 {
            return block_83(state, tracer, fn_state);
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
        // D s_79_1: write-var gs#133893 <= s_79_0
        fn_state.gs_133893 = s_79_0;
        // N s_79_2: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var gs#133893:u8
        let s_80_0: bool = fn_state.gs_133893;
        // N s_80_1: branch s_80_0 b82 b81
        if s_80_0 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #3u : u8
        let s_81_0: u8 = 3;
        // C s_81_1: cast zx s_81_0 -> bv
        let s_81_1: Bits = Bits::new(s_81_0 as u128, 8u16);
        // C s_81_2: cast zx s_81_1 -> i
        let s_81_2: i128 = (s_81_1.value() as i128);
        // C s_81_3: cast reint s_81_2 -> i64
        let s_81_3: i64 = (s_81_2 as i64);
        // C s_81_4: cast zx s_81_3 -> i
        let s_81_4: i128 = (i128::try_from(s_81_3).unwrap());
        // C s_81_5: const #424u : u32
        let s_81_5: u32 = 424;
        // D s_81_6: read-reg s_81_5:u8
        let s_81_6: u8 = {
            let value = state.read_register::<u8>(s_81_5 as isize);
            tracer.read_register(s_81_5 as isize, value);
            value
        };
        // D s_81_7: call AArch64_AArch32SystemAccessTrap(s_81_6, s_81_4)
        let s_81_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_81_6, s_81_4);
        // N s_81_8: return
        return;
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_82_0: panic
        panic!("{:?}", ());
        // N s_82_1: return
        return;
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #() : ()
        let s_83_0: () = ();
        // S s_83_1: call EDSCR_read(s_83_0)
        let s_83_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_83_0);
        // S s_83_2: call _get_EDSCR_Type_SDD(s_83_1)
        let s_83_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_83_1);
        // S s_83_3: cast zx s_83_2 -> bv
        let s_83_3: Bits = Bits::new(s_83_2 as u128, 1u16);
        // C s_83_4: const #1u : u8
        let s_83_4: bool = true;
        // C s_83_5: cast zx s_83_4 -> bv
        let s_83_5: Bits = Bits::new(s_83_4 as u128, 1u16);
        // S s_83_6: cmp-eq s_83_3 s_83_5
        let s_83_6: bool = ((s_83_3) == (s_83_5));
        // D s_83_7: write-var gs#133893 <= s_83_6
        fn_state.gs_133893 = s_83_6;
        // N s_83_8: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var __MDCR_EL3_TPM:u8
        let s_84_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_84_1: cast zx s_84_0 -> bv
        let s_84_1: Bits = Bits::new(s_84_0 as u128, 1u16);
        // C s_84_2: const #1u : u8
        let s_84_2: bool = true;
        // C s_84_3: cast zx s_84_2 -> bv
        let s_84_3: Bits = Bits::new(s_84_2 as u128, 1u16);
        // D s_84_4: cmp-eq s_84_1 s_84_3
        let s_84_4: bool = ((s_84_1) == (s_84_3));
        // D s_84_5: write-var gs#133892 <= s_84_4
        fn_state.gs_133892 = s_84_4;
        // N s_84_6: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #424u : u32
        let s_85_0: u32 = 424;
        // D s_85_1: read-reg s_85_0:u8
        let s_85_1: u8 = {
            let value = state.read_register::<u8>(s_85_0 as isize);
            tracer.read_register(s_85_0 as isize, value);
            value
        };
        // D s_85_2: call ELUsingAArch32(s_85_1)
        let s_85_2: bool = ELUsingAArch32(state, tracer, s_85_1);
        // D s_85_3: not s_85_2
        let s_85_3: bool = !s_85_2;
        // D s_85_4: write-var gs#133891 <= s_85_3
        fn_state.gs_133891 = s_85_3;
        // N s_85_5: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #146u : u32
        let s_86_0: u32 = 146;
        // S s_86_1: call IsFeatureImplemented(s_86_0)
        let s_86_1: bool = IsFeatureImplemented(state, tracer, s_86_0);
        // S s_86_2: not s_86_1
        let s_86_2: bool = !s_86_1;
        // N s_86_3: branch s_86_2 b90 b87
        if s_86_2 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
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
        // N s_87_3: branch s_87_2 b89 b88
        if s_87_2 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_88(state, tracer, fn_state);
        };
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
        // C s_88_5: const #432u : u32
        let s_88_5: u32 = 432;
        // D s_88_6: read-reg s_88_5:u8
        let s_88_6: u8 = {
            let value = state.read_register::<u8>(s_88_5 as isize);
            tracer.read_register(s_88_5 as isize, value);
            value
        };
        // D s_88_7: call AArch64_AArch32SystemAccessTrap(s_88_6, s_88_4)
        let s_88_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_88_6, s_88_4);
        // N s_88_8: return
        return;
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #3u : u8
        let s_89_0: u8 = 3;
        // C s_89_1: cast zx s_89_0 -> bv
        let s_89_1: Bits = Bits::new(s_89_0 as u128, 8u16);
        // C s_89_2: cast zx s_89_1 -> i
        let s_89_2: i128 = (s_89_1.value() as i128);
        // C s_89_3: cast reint s_89_2 -> i64
        let s_89_3: i64 = (s_89_2 as i64);
        // C s_89_4: cast zx s_89_3 -> i
        let s_89_4: i128 = (i128::try_from(s_89_3).unwrap());
        // S s_89_5: call AArch32_TakeHypTrapException(s_89_4)
        let s_89_5: () = AArch32_TakeHypTrapException(state, tracer, s_89_4);
        // N s_89_6: return
        return;
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #72u : u32
        let s_90_0: u32 = 72;
        // S s_90_1: call ConstrainUnpredictableProcedure(s_90_0)
        let s_90_1: () = ConstrainUnpredictableProcedure(state, tracer, s_90_0);
        // N s_90_2: return
        return;
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #() : ()
        let s_91_0: () = ();
        // S s_91_1: call PMSELR_read(s_91_0)
        let s_91_1: ProductType700c18a878c5601b = PMSELR_read(state, tracer, s_91_0);
        // S s_91_2: call _get_PMSELR_Type_SEL(s_91_1)
        let s_91_2: u8 = u_get_PMSELR_Type_SEL(state, tracer, s_91_1);
        // S s_91_3: cast zx s_91_2 -> bv
        let s_91_3: Bits = Bits::new(s_91_2 as u128, 5u16);
        // S s_91_4: cast zx s_91_3 -> i
        let s_91_4: i128 = (s_91_3.value() as i128);
        // S s_91_5: cast reint s_91_4 -> i64
        let s_91_5: i64 = (s_91_4 as i64);
        // C s_91_6: const #() : ()
        let s_91_6: () = ();
        // S s_91_7: call AArch32_GetNumEventCountersAccessible(s_91_6)
        let s_91_7: i128 = AArch32_GetNumEventCountersAccessible(state, tracer, s_91_6);
        // S s_91_8: cast zx s_91_5 -> i
        let s_91_8: i128 = (i128::try_from(s_91_5).unwrap());
        // S s_91_9: cmp-ge s_91_8 s_91_7
        let s_91_9: bool = ((s_91_8) >= (s_91_7));
        // D s_91_10: write-var gs#133890 <= s_91_9
        fn_state.gs_133890 = s_91_9;
        // N s_91_11: jump b71
        return block_71(state, tracer, fn_state);
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
        // D s_93_0: read-var __HDCR_TPM:u8
        let s_93_0: bool = fn_state.u__HDCR_TPM;
        // D s_93_1: cast zx s_93_0 -> bv
        let s_93_1: Bits = Bits::new(s_93_0 as u128, 1u16);
        // C s_93_2: const #1u : u8
        let s_93_2: bool = true;
        // C s_93_3: cast zx s_93_2 -> bv
        let s_93_3: Bits = Bits::new(s_93_2 as u128, 1u16);
        // D s_93_4: cmp-eq s_93_1 s_93_3
        let s_93_4: bool = ((s_93_1) == (s_93_3));
        // D s_93_5: write-var gs#133889 <= s_93_4
        fn_state.gs_133889 = s_93_4;
        // N s_93_6: jump b68
        return block_68(state, tracer, fn_state);
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
        // D s_94_3: write-var gs#133888 <= s_94_2
        fn_state.gs_133888 = s_94_2;
        // N s_94_4: jump b66
        return block_66(state, tracer, fn_state);
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
        // C s_95_5: const #432u : u32
        let s_95_5: u32 = 432;
        // D s_95_6: read-reg s_95_5:u8
        let s_95_6: u8 = {
            let value = state.read_register::<u8>(s_95_5 as isize);
            tracer.read_register(s_95_5 as isize, value);
            value
        };
        // D s_95_7: call AArch64_AArch32SystemAccessTrap(s_95_6, s_95_4)
        let s_95_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_95_6, s_95_4);
        // N s_95_8: return
        return;
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var __MDCR_EL2_TPM:u8
        let s_96_0: bool = fn_state.u__MDCR_EL2_TPM;
        // D s_96_1: cast zx s_96_0 -> bv
        let s_96_1: Bits = Bits::new(s_96_0 as u128, 1u16);
        // C s_96_2: const #1u : u8
        let s_96_2: bool = true;
        // C s_96_3: cast zx s_96_2 -> bv
        let s_96_3: Bits = Bits::new(s_96_2 as u128, 1u16);
        // D s_96_4: cmp-eq s_96_1 s_96_3
        let s_96_4: bool = ((s_96_1) == (s_96_3));
        // D s_96_5: write-var gs#133887 <= s_96_4
        fn_state.gs_133887 = s_96_4;
        // N s_96_6: jump b63
        return block_63(state, tracer, fn_state);
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
        // D s_97_3: not s_97_2
        let s_97_3: bool = !s_97_2;
        // D s_97_4: write-var gs#133886 <= s_97_3
        fn_state.gs_133886 = s_97_3;
        // N s_97_5: jump b61
        return block_61(state, tracer, fn_state);
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
        // S s_98_5: call AArch32_TakeHypTrapException(s_98_4)
        let s_98_5: () = AArch32_TakeHypTrapException(state, tracer, s_98_4);
        // N s_98_6: return
        return;
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_99_0: read-var __HSTR_T9:u8
        let s_99_0: bool = fn_state.u__HSTR_T9;
        // D s_99_1: cast zx s_99_0 -> bv
        let s_99_1: Bits = Bits::new(s_99_0 as u128, 1u16);
        // C s_99_2: const #1u : u8
        let s_99_2: bool = true;
        // C s_99_3: cast zx s_99_2 -> bv
        let s_99_3: Bits = Bits::new(s_99_2 as u128, 1u16);
        // D s_99_4: cmp-eq s_99_1 s_99_3
        let s_99_4: bool = ((s_99_1) == (s_99_3));
        // D s_99_5: write-var gs#133885 <= s_99_4
        fn_state.gs_133885 = s_99_4;
        // N s_99_6: jump b58
        return block_58(state, tracer, fn_state);
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
        // D s_100_3: write-var gs#133884 <= s_100_2
        fn_state.gs_133884 = s_100_2;
        // N s_100_4: jump b56
        return block_56(state, tracer, fn_state);
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
        // D s_102_0: read-var __HSTR_EL2_T9:u8
        let s_102_0: bool = fn_state.u__HSTR_EL2_T9;
        // D s_102_1: cast zx s_102_0 -> bv
        let s_102_1: Bits = Bits::new(s_102_0 as u128, 1u16);
        // C s_102_2: const #1u : u8
        let s_102_2: bool = true;
        // C s_102_3: cast zx s_102_2 -> bv
        let s_102_3: Bits = Bits::new(s_102_2 as u128, 1u16);
        // D s_102_4: cmp-eq s_102_1 s_102_3
        let s_102_4: bool = ((s_102_1) == (s_102_3));
        // D s_102_5: write-var gs#133883 <= s_102_4
        fn_state.gs_133883 = s_102_4;
        // N s_102_6: jump b53
        return block_53(state, tracer, fn_state);
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
        // D s_103_4: write-var gs#133882 <= s_103_3
        fn_state.gs_133882 = s_103_3;
        // N s_103_5: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_104_0: panic
        panic!("{:?}", ());
        // N s_104_1: return
        return;
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_105_0: read-var __MDCR_EL3_TPM:u8
        let s_105_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_105_1: cast zx s_105_0 -> bv
        let s_105_1: Bits = Bits::new(s_105_0 as u128, 1u16);
        // C s_105_2: const #1u : u8
        let s_105_2: bool = true;
        // C s_105_3: cast zx s_105_2 -> bv
        let s_105_3: Bits = Bits::new(s_105_2 as u128, 1u16);
        // D s_105_4: cmp-eq s_105_1 s_105_3
        let s_105_4: bool = ((s_105_1) == (s_105_3));
        // D s_105_5: write-var gs#133881 <= s_105_4
        fn_state.gs_133881 = s_105_4;
        // N s_105_6: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #424u : u32
        let s_106_0: u32 = 424;
        // D s_106_1: read-reg s_106_0:u8
        let s_106_1: u8 = {
            let value = state.read_register::<u8>(s_106_0 as isize);
            tracer.read_register(s_106_0 as isize, value);
            value
        };
        // D s_106_2: call ELUsingAArch32(s_106_1)
        let s_106_2: bool = ELUsingAArch32(state, tracer, s_106_1);
        // D s_106_3: not s_106_2
        let s_106_3: bool = !s_106_2;
        // D s_106_4: write-var gs#133880 <= s_106_3
        fn_state.gs_133880 = s_106_3;
        // N s_106_5: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_107_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_107_1: call __IMPDEF_boolean(s_107_0)
        let s_107_1: bool = u__IMPDEF_boolean(state, tracer, s_107_0);
        // D s_107_2: write-var gs#133879 <= s_107_1
        fn_state.gs_133879 = s_107_1;
        // N s_107_3: jump b44
        return block_44(state, tracer, fn_state);
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
        // D s_108_7: write-var gs#133878 <= s_108_6
        fn_state.gs_133878 = s_108_6;
        // N s_108_8: jump b42
        return block_42(state, tracer, fn_state);
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
        // D s_109_4: write-var gs#133877 <= s_109_3
        fn_state.gs_133877 = s_109_3;
        // N s_109_5: jump b40
        return block_40(state, tracer, fn_state);
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
        // N s_110_2: branch s_110_1 b247 b111
        if s_110_1 {
            return block_247(state, tracer, fn_state);
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
        // D s_111_1: write-var gs#133894 <= s_111_0
        fn_state.gs_133894 = s_111_0;
        // N s_111_2: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_112_0: read-var gs#133894:u8
        let s_112_0: bool = fn_state.gs_133894;
        // N s_112_1: branch s_112_0 b246 b113
        if s_112_0 {
            return block_246(state, tracer, fn_state);
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
        // D s_113_1: write-var gs#133895 <= s_113_0
        fn_state.gs_133895 = s_113_0;
        // N s_113_2: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_114_0: read-var gs#133895:u8
        let s_114_0: bool = fn_state.gs_133895;
        // N s_114_1: branch s_114_0 b245 b115
        if s_114_0 {
            return block_245(state, tracer, fn_state);
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
        // D s_115_1: write-var gs#133896 <= s_115_0
        fn_state.gs_133896 = s_115_0;
        // N s_115_2: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_116_0: read-var gs#133896:u8
        let s_116_0: bool = fn_state.gs_133896;
        // N s_116_1: branch s_116_0 b244 b117
        if s_116_0 {
            return block_244(state, tracer, fn_state);
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
        // D s_117_1: write-var gs#133897 <= s_117_0
        fn_state.gs_133897 = s_117_0;
        // N s_117_2: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_118_0: read-var gs#133897:u8
        let s_118_0: bool = fn_state.gs_133897;
        // N s_118_1: branch s_118_0 b243 b119
        if s_118_0 {
            return block_243(state, tracer, fn_state);
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
        // D s_119_1: write-var gs#133898 <= s_119_0
        fn_state.gs_133898 = s_119_0;
        // N s_119_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_120_0: read-var gs#133898:u8
        let s_120_0: bool = fn_state.gs_133898;
        // N s_120_1: branch s_120_0 b242 b121
        if s_120_0 {
            return block_242(state, tracer, fn_state);
        } else {
            return block_121(state, tracer, fn_state);
        };
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #440u : u32
        let s_121_0: u32 = 440;
        // D s_121_1: read-reg s_121_0:u8
        let s_121_1: u8 = {
            let value = state.read_register::<u8>(s_121_0 as isize);
            tracer.read_register(s_121_0 as isize, value);
            value
        };
        // D s_121_2: call ELUsingAArch32(s_121_1)
        let s_121_2: bool = ELUsingAArch32(state, tracer, s_121_1);
        // D s_121_3: not s_121_2
        let s_121_3: bool = !s_121_2;
        // N s_121_4: branch s_121_3 b232 b122
        if s_121_3 {
            return block_232(state, tracer, fn_state);
        } else {
            return block_122(state, tracer, fn_state);
        };
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_122_0: const #0u : u8
        let s_122_0: bool = false;
        // D s_122_1: write-var gs#133902 <= s_122_0
        fn_state.gs_133902 = s_122_0;
        // N s_122_2: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_123_0: read-var gs#133902:u8
        let s_123_0: bool = fn_state.gs_133902;
        // N s_123_1: branch s_123_0 b223 b124
        if s_123_0 {
            return block_223(state, tracer, fn_state);
        } else {
            return block_124(state, tracer, fn_state);
        };
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #440u : u32
        let s_124_0: u32 = 440;
        // D s_124_1: read-reg s_124_0:u8
        let s_124_1: u8 = {
            let value = state.read_register::<u8>(s_124_0 as isize);
            tracer.read_register(s_124_0 as isize, value);
            value
        };
        // D s_124_2: call ELUsingAArch32(s_124_1)
        let s_124_2: bool = ELUsingAArch32(state, tracer, s_124_1);
        // N s_124_3: branch s_124_2 b222 b125
        if s_124_2 {
            return block_222(state, tracer, fn_state);
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
        // D s_125_1: write-var gs#133903 <= s_125_0
        fn_state.gs_133903 = s_125_0;
        // N s_125_2: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_126_0: read-var gs#133903:u8
        let s_126_0: bool = fn_state.gs_133903;
        // N s_126_1: branch s_126_0 b205 b127
        if s_126_0 {
            return block_205(state, tracer, fn_state);
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
        // N s_127_2: branch s_127_1 b204 b128
        if s_127_1 {
            return block_204(state, tracer, fn_state);
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
        // D s_128_1: write-var gs#133904 <= s_128_0
        fn_state.gs_133904 = s_128_0;
        // N s_128_2: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_129_0: read-var gs#133904:u8
        let s_129_0: bool = fn_state.gs_133904;
        // N s_129_1: branch s_129_0 b203 b130
        if s_129_0 {
            return block_203(state, tracer, fn_state);
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
        // D s_130_1: write-var gs#133905 <= s_130_0
        fn_state.gs_133905 = s_130_0;
        // N s_130_2: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_131_0: read-var gs#133905:u8
        let s_131_0: bool = fn_state.gs_133905;
        // N s_131_1: branch s_131_0 b202 b132
        if s_131_0 {
            return block_202(state, tracer, fn_state);
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
        // D s_132_1: write-var gs#133906 <= s_132_0
        fn_state.gs_133906 = s_132_0;
        // N s_132_2: jump b133
        return block_133(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_133_0: read-var gs#133906:u8
        let s_133_0: bool = fn_state.gs_133906;
        // N s_133_1: branch s_133_0 b201 b134
        if s_133_0 {
            return block_201(state, tracer, fn_state);
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
        // N s_134_2: branch s_134_1 b200 b135
        if s_134_1 {
            return block_200(state, tracer, fn_state);
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
        // D s_135_1: write-var gs#133907 <= s_135_0
        fn_state.gs_133907 = s_135_0;
        // N s_135_2: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_136_0: read-var gs#133907:u8
        let s_136_0: bool = fn_state.gs_133907;
        // N s_136_1: branch s_136_0 b199 b137
        if s_136_0 {
            return block_199(state, tracer, fn_state);
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
        // D s_137_1: write-var gs#133908 <= s_137_0
        fn_state.gs_133908 = s_137_0;
        // N s_137_2: jump b138
        return block_138(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_138_0: read-var gs#133908:u8
        let s_138_0: bool = fn_state.gs_133908;
        // N s_138_1: branch s_138_0 b198 b139
        if s_138_0 {
            return block_198(state, tracer, fn_state);
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
        // N s_139_2: branch s_139_1 b197 b140
        if s_139_1 {
            return block_197(state, tracer, fn_state);
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
        // D s_140_1: write-var gs#133909 <= s_140_0
        fn_state.gs_133909 = s_140_0;
        // N s_140_2: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_141_0: read-var gs#133909:u8
        let s_141_0: bool = fn_state.gs_133909;
        // N s_141_1: branch s_141_0 b196 b142
        if s_141_0 {
            return block_196(state, tracer, fn_state);
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
        // D s_142_1: write-var gs#133910 <= s_142_0
        fn_state.gs_133910 = s_142_0;
        // N s_142_2: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_143_0: read-var gs#133910:u8
        let s_143_0: bool = fn_state.gs_133910;
        // N s_143_1: branch s_143_0 b195 b144
        if s_143_0 {
            return block_195(state, tracer, fn_state);
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
        // D s_144_1: write-var gs#133911 <= s_144_0
        fn_state.gs_133911 = s_144_0;
        // N s_144_2: jump b145
        return block_145(state, tracer, fn_state);
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_145_0: read-var gs#133911:u8
        let s_145_0: bool = fn_state.gs_133911;
        // N s_145_1: branch s_145_0 b191 b146
        if s_145_0 {
            return block_191(state, tracer, fn_state);
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
        // D s_146_1: write-var gs#133913 <= s_146_0
        fn_state.gs_133913 = s_146_0;
        // N s_146_2: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_147_0: read-var gs#133913:u8
        let s_147_0: bool = fn_state.gs_133913;
        // N s_147_1: branch s_147_0 b190 b148
        if s_147_0 {
            return block_190(state, tracer, fn_state);
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
        // D s_148_1: write-var gs#133914 <= s_148_0
        fn_state.gs_133914 = s_148_0;
        // N s_148_2: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_149_0: read-var gs#133914:u8
        let s_149_0: bool = fn_state.gs_133914;
        // N s_149_1: branch s_149_0 b189 b150
        if s_149_0 {
            return block_189(state, tracer, fn_state);
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
        // N s_150_2: branch s_150_1 b188 b151
        if s_150_1 {
            return block_188(state, tracer, fn_state);
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
        // D s_151_1: write-var gs#133915 <= s_151_0
        fn_state.gs_133915 = s_151_0;
        // N s_151_2: jump b152
        return block_152(state, tracer, fn_state);
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_152_0: read-var gs#133915:u8
        let s_152_0: bool = fn_state.gs_133915;
        // N s_152_1: branch s_152_0 b187 b153
        if s_152_0 {
            return block_187(state, tracer, fn_state);
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
        // D s_153_1: write-var gs#133916 <= s_153_0
        fn_state.gs_133916 = s_153_0;
        // N s_153_2: jump b154
        return block_154(state, tracer, fn_state);
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_154_0: read-var gs#133916:u8
        let s_154_0: bool = fn_state.gs_133916;
        // N s_154_1: branch s_154_0 b186 b155
        if s_154_0 {
            return block_186(state, tracer, fn_state);
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
        // N s_155_2: branch s_155_1 b185 b156
        if s_155_1 {
            return block_185(state, tracer, fn_state);
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
        // D s_156_1: write-var gs#133917 <= s_156_0
        fn_state.gs_133917 = s_156_0;
        // N s_156_2: jump b157
        return block_157(state, tracer, fn_state);
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_157_0: read-var gs#133917:u8
        let s_157_0: bool = fn_state.gs_133917;
        // N s_157_1: branch s_157_0 b184 b158
        if s_157_0 {
            return block_184(state, tracer, fn_state);
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
        // D s_158_1: write-var gs#133918 <= s_158_0
        fn_state.gs_133918 = s_158_0;
        // N s_158_2: jump b159
        return block_159(state, tracer, fn_state);
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_159_0: read-var gs#133918:u8
        let s_159_0: bool = fn_state.gs_133918;
        // N s_159_1: branch s_159_0 b183 b160
        if s_159_0 {
            return block_183(state, tracer, fn_state);
        } else {
            return block_160(state, tracer, fn_state);
        };
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_160_0: const #() : ()
        let s_160_0: () = ();
        // S s_160_1: call EL2Enabled(s_160_0)
        let s_160_1: bool = EL2Enabled(state, tracer, s_160_0);
        // N s_160_2: branch s_160_1 b182 b161
        if s_160_1 {
            return block_182(state, tracer, fn_state);
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
        // D s_161_1: write-var gs#133919 <= s_161_0
        fn_state.gs_133919 = s_161_0;
        // N s_161_2: jump b162
        return block_162(state, tracer, fn_state);
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_162_0: read-var gs#133919:u8
        let s_162_0: bool = fn_state.gs_133919;
        // N s_162_1: branch s_162_0 b177 b163
        if s_162_0 {
            return block_177(state, tracer, fn_state);
        } else {
            return block_163(state, tracer, fn_state);
        };
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_163_0: const #424u : u32
        let s_163_0: u32 = 424;
        // D s_163_1: read-reg s_163_0:u8
        let s_163_1: u8 = {
            let value = state.read_register::<u8>(s_163_0 as isize);
            tracer.read_register(s_163_0 as isize, value);
            value
        };
        // C s_163_2: const #2u : u8
        let s_163_2: u8 = 2;
        // D s_163_3: cmp-lt s_163_1 s_163_2
        let s_163_3: bool = ((s_163_1) < (s_163_2));
        // N s_163_4: branch s_163_3 b176 b164
        if s_163_3 {
            return block_176(state, tracer, fn_state);
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
        // D s_164_1: write-var gs#133920 <= s_164_0
        fn_state.gs_133920 = s_164_0;
        // N s_164_2: jump b165
        return block_165(state, tracer, fn_state);
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_165_0: read-var gs#133920:u8
        let s_165_0: bool = fn_state.gs_133920;
        // N s_165_1: branch s_165_0 b175 b166
        if s_165_0 {
            return block_175(state, tracer, fn_state);
        } else {
            return block_166(state, tracer, fn_state);
        };
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_166_0: const #0u : u8
        let s_166_0: bool = false;
        // D s_166_1: write-var gs#133921 <= s_166_0
        fn_state.gs_133921 = s_166_0;
        // N s_166_2: jump b167
        return block_167(state, tracer, fn_state);
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_167_0: read-var gs#133921:u8
        let s_167_0: bool = fn_state.gs_133921;
        // N s_167_1: branch s_167_0 b169 b168
        if s_167_0 {
            return block_169(state, tracer, fn_state);
        } else {
            return block_168(state, tracer, fn_state);
        };
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_168_0: read-var t:i
        let s_168_0: i128 = fn_state.t;
        // D s_168_1: call R_read(s_168_0)
        let s_168_1: u32 = R_read(state, tracer, s_168_0);
        // D s_168_2: call __set_selected_PMEVCNTR(s_168_1)
        let s_168_2: () = u__set_selected_PMEVCNTR(state, tracer, s_168_1);
        // N s_168_3: return
        return;
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_169_0: const #() : ()
        let s_169_0: () = ();
        // S s_169_1: call Halted(s_169_0)
        let s_169_1: bool = Halted(state, tracer, s_169_0);
        // N s_169_2: branch s_169_1 b174 b170
        if s_169_1 {
            return block_174(state, tracer, fn_state);
        } else {
            return block_170(state, tracer, fn_state);
        };
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_170_0: const #0u : u8
        let s_170_0: bool = false;
        // D s_170_1: write-var gs#133922 <= s_170_0
        fn_state.gs_133922 = s_170_0;
        // N s_170_2: jump b171
        return block_171(state, tracer, fn_state);
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_171_0: read-var gs#133922:u8
        let s_171_0: bool = fn_state.gs_133922;
        // N s_171_1: branch s_171_0 b173 b172
        if s_171_0 {
            return block_173(state, tracer, fn_state);
        } else {
            return block_172(state, tracer, fn_state);
        };
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_172_0: const #3u : u8
        let s_172_0: u8 = 3;
        // C s_172_1: cast zx s_172_0 -> bv
        let s_172_1: Bits = Bits::new(s_172_0 as u128, 8u16);
        // C s_172_2: cast zx s_172_1 -> i
        let s_172_2: i128 = (s_172_1.value() as i128);
        // C s_172_3: cast reint s_172_2 -> i64
        let s_172_3: i64 = (s_172_2 as i64);
        // C s_172_4: cast zx s_172_3 -> i
        let s_172_4: i128 = (i128::try_from(s_172_3).unwrap());
        // C s_172_5: const #424u : u32
        let s_172_5: u32 = 424;
        // D s_172_6: read-reg s_172_5:u8
        let s_172_6: u8 = {
            let value = state.read_register::<u8>(s_172_5 as isize);
            tracer.read_register(s_172_5 as isize, value);
            value
        };
        // D s_172_7: call AArch64_AArch32SystemAccessTrap(s_172_6, s_172_4)
        let s_172_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_172_6,
            s_172_4,
        );
        // N s_172_8: return
        return;
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_173_0: panic
        panic!("{:?}", ());
        // N s_173_1: return
        return;
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_174_0: const #() : ()
        let s_174_0: () = ();
        // S s_174_1: call EDSCR_read(s_174_0)
        let s_174_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_174_0);
        // S s_174_2: call _get_EDSCR_Type_SDD(s_174_1)
        let s_174_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_174_1);
        // S s_174_3: cast zx s_174_2 -> bv
        let s_174_3: Bits = Bits::new(s_174_2 as u128, 1u16);
        // C s_174_4: const #1u : u8
        let s_174_4: bool = true;
        // C s_174_5: cast zx s_174_4 -> bv
        let s_174_5: Bits = Bits::new(s_174_4 as u128, 1u16);
        // S s_174_6: cmp-eq s_174_3 s_174_5
        let s_174_6: bool = ((s_174_3) == (s_174_5));
        // D s_174_7: write-var gs#133922 <= s_174_6
        fn_state.gs_133922 = s_174_6;
        // N s_174_8: jump b171
        return block_171(state, tracer, fn_state);
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_175_0: read-var __MDCR_EL3_TPM:u8
        let s_175_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_175_1: cast zx s_175_0 -> bv
        let s_175_1: Bits = Bits::new(s_175_0 as u128, 1u16);
        // C s_175_2: const #1u : u8
        let s_175_2: bool = true;
        // C s_175_3: cast zx s_175_2 -> bv
        let s_175_3: Bits = Bits::new(s_175_2 as u128, 1u16);
        // D s_175_4: cmp-eq s_175_1 s_175_3
        let s_175_4: bool = ((s_175_1) == (s_175_3));
        // D s_175_5: write-var gs#133921 <= s_175_4
        fn_state.gs_133921 = s_175_4;
        // N s_175_6: jump b167
        return block_167(state, tracer, fn_state);
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
        // D s_176_2: call ELUsingAArch32(s_176_1)
        let s_176_2: bool = ELUsingAArch32(state, tracer, s_176_1);
        // D s_176_3: not s_176_2
        let s_176_3: bool = !s_176_2;
        // D s_176_4: write-var gs#133920 <= s_176_3
        fn_state.gs_133920 = s_176_3;
        // N s_176_5: jump b165
        return block_165(state, tracer, fn_state);
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_177_0: const #146u : u32
        let s_177_0: u32 = 146;
        // S s_177_1: call IsFeatureImplemented(s_177_0)
        let s_177_1: bool = IsFeatureImplemented(state, tracer, s_177_0);
        // S s_177_2: not s_177_1
        let s_177_2: bool = !s_177_1;
        // N s_177_3: branch s_177_2 b181 b178
        if s_177_2 {
            return block_181(state, tracer, fn_state);
        } else {
            return block_178(state, tracer, fn_state);
        };
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_178_0: const #440u : u32
        let s_178_0: u32 = 440;
        // D s_178_1: read-reg s_178_0:u8
        let s_178_1: u8 = {
            let value = state.read_register::<u8>(s_178_0 as isize);
            tracer.read_register(s_178_0 as isize, value);
            value
        };
        // D s_178_2: call ELUsingAArch32(s_178_1)
        let s_178_2: bool = ELUsingAArch32(state, tracer, s_178_1);
        // N s_178_3: branch s_178_2 b180 b179
        if s_178_2 {
            return block_180(state, tracer, fn_state);
        } else {
            return block_179(state, tracer, fn_state);
        };
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_179_0: const #3u : u8
        let s_179_0: u8 = 3;
        // C s_179_1: cast zx s_179_0 -> bv
        let s_179_1: Bits = Bits::new(s_179_0 as u128, 8u16);
        // C s_179_2: cast zx s_179_1 -> i
        let s_179_2: i128 = (s_179_1.value() as i128);
        // C s_179_3: cast reint s_179_2 -> i64
        let s_179_3: i64 = (s_179_2 as i64);
        // C s_179_4: cast zx s_179_3 -> i
        let s_179_4: i128 = (i128::try_from(s_179_3).unwrap());
        // C s_179_5: const #432u : u32
        let s_179_5: u32 = 432;
        // D s_179_6: read-reg s_179_5:u8
        let s_179_6: u8 = {
            let value = state.read_register::<u8>(s_179_5 as isize);
            tracer.read_register(s_179_5 as isize, value);
            value
        };
        // D s_179_7: call AArch64_AArch32SystemAccessTrap(s_179_6, s_179_4)
        let s_179_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_179_6,
            s_179_4,
        );
        // N s_179_8: return
        return;
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
        // S s_180_5: call AArch32_TakeHypTrapException(s_180_4)
        let s_180_5: () = AArch32_TakeHypTrapException(state, tracer, s_180_4);
        // N s_180_6: return
        return;
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_181_0: const #72u : u32
        let s_181_0: u32 = 72;
        // S s_181_1: call ConstrainUnpredictableProcedure(s_181_0)
        let s_181_1: () = ConstrainUnpredictableProcedure(state, tracer, s_181_0);
        // N s_181_2: return
        return;
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_182_0: const #() : ()
        let s_182_0: () = ();
        // S s_182_1: call PMSELR_read(s_182_0)
        let s_182_1: ProductType700c18a878c5601b = PMSELR_read(state, tracer, s_182_0);
        // S s_182_2: call _get_PMSELR_Type_SEL(s_182_1)
        let s_182_2: u8 = u_get_PMSELR_Type_SEL(state, tracer, s_182_1);
        // S s_182_3: cast zx s_182_2 -> bv
        let s_182_3: Bits = Bits::new(s_182_2 as u128, 5u16);
        // S s_182_4: cast zx s_182_3 -> i
        let s_182_4: i128 = (s_182_3.value() as i128);
        // S s_182_5: cast reint s_182_4 -> i64
        let s_182_5: i64 = (s_182_4 as i64);
        // C s_182_6: const #() : ()
        let s_182_6: () = ();
        // S s_182_7: call AArch32_GetNumEventCountersAccessible(s_182_6)
        let s_182_7: i128 = AArch32_GetNumEventCountersAccessible(
            state,
            tracer,
            s_182_6,
        );
        // S s_182_8: cast zx s_182_5 -> i
        let s_182_8: i128 = (i128::try_from(s_182_5).unwrap());
        // S s_182_9: cmp-ge s_182_8 s_182_7
        let s_182_9: bool = ((s_182_8) >= (s_182_7));
        // D s_182_10: write-var gs#133919 <= s_182_9
        fn_state.gs_133919 = s_182_9;
        // N s_182_11: jump b162
        return block_162(state, tracer, fn_state);
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_183_0: const #3u : u8
        let s_183_0: u8 = 3;
        // C s_183_1: cast zx s_183_0 -> bv
        let s_183_1: Bits = Bits::new(s_183_0 as u128, 8u16);
        // C s_183_2: cast zx s_183_1 -> i
        let s_183_2: i128 = (s_183_1.value() as i128);
        // C s_183_3: cast reint s_183_2 -> i64
        let s_183_3: i64 = (s_183_2 as i64);
        // C s_183_4: cast zx s_183_3 -> i
        let s_183_4: i128 = (i128::try_from(s_183_3).unwrap());
        // S s_183_5: call AArch32_TakeHypTrapException(s_183_4)
        let s_183_5: () = AArch32_TakeHypTrapException(state, tracer, s_183_4);
        // N s_183_6: return
        return;
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_184_0: read-var __HDCR_TPM:u8
        let s_184_0: bool = fn_state.u__HDCR_TPM;
        // D s_184_1: cast zx s_184_0 -> bv
        let s_184_1: Bits = Bits::new(s_184_0 as u128, 1u16);
        // C s_184_2: const #1u : u8
        let s_184_2: bool = true;
        // C s_184_3: cast zx s_184_2 -> bv
        let s_184_3: Bits = Bits::new(s_184_2 as u128, 1u16);
        // D s_184_4: cmp-eq s_184_1 s_184_3
        let s_184_4: bool = ((s_184_1) == (s_184_3));
        // D s_184_5: write-var gs#133918 <= s_184_4
        fn_state.gs_133918 = s_184_4;
        // N s_184_6: jump b159
        return block_159(state, tracer, fn_state);
    }
    fn block_185<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_185_0: const #432u : u32
        let s_185_0: u32 = 432;
        // D s_185_1: read-reg s_185_0:u8
        let s_185_1: u8 = {
            let value = state.read_register::<u8>(s_185_0 as isize);
            tracer.read_register(s_185_0 as isize, value);
            value
        };
        // D s_185_2: call ELUsingAArch32(s_185_1)
        let s_185_2: bool = ELUsingAArch32(state, tracer, s_185_1);
        // D s_185_3: write-var gs#133917 <= s_185_2
        fn_state.gs_133917 = s_185_2;
        // N s_185_4: jump b157
        return block_157(state, tracer, fn_state);
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_186_0: const #3u : u8
        let s_186_0: u8 = 3;
        // C s_186_1: cast zx s_186_0 -> bv
        let s_186_1: Bits = Bits::new(s_186_0 as u128, 8u16);
        // C s_186_2: cast zx s_186_1 -> i
        let s_186_2: i128 = (s_186_1.value() as i128);
        // C s_186_3: cast reint s_186_2 -> i64
        let s_186_3: i64 = (s_186_2 as i64);
        // C s_186_4: cast zx s_186_3 -> i
        let s_186_4: i128 = (i128::try_from(s_186_3).unwrap());
        // C s_186_5: const #432u : u32
        let s_186_5: u32 = 432;
        // D s_186_6: read-reg s_186_5:u8
        let s_186_6: u8 = {
            let value = state.read_register::<u8>(s_186_5 as isize);
            tracer.read_register(s_186_5 as isize, value);
            value
        };
        // D s_186_7: call AArch64_AArch32SystemAccessTrap(s_186_6, s_186_4)
        let s_186_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_186_6,
            s_186_4,
        );
        // N s_186_8: return
        return;
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_187_0: read-var __MDCR_EL2_TPM:u8
        let s_187_0: bool = fn_state.u__MDCR_EL2_TPM;
        // D s_187_1: cast zx s_187_0 -> bv
        let s_187_1: Bits = Bits::new(s_187_0 as u128, 1u16);
        // C s_187_2: const #1u : u8
        let s_187_2: bool = true;
        // C s_187_3: cast zx s_187_2 -> bv
        let s_187_3: Bits = Bits::new(s_187_2 as u128, 1u16);
        // D s_187_4: cmp-eq s_187_1 s_187_3
        let s_187_4: bool = ((s_187_1) == (s_187_3));
        // D s_187_5: write-var gs#133916 <= s_187_4
        fn_state.gs_133916 = s_187_4;
        // N s_187_6: jump b154
        return block_154(state, tracer, fn_state);
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_188_0: const #432u : u32
        let s_188_0: u32 = 432;
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
        // D s_188_4: write-var gs#133915 <= s_188_3
        fn_state.gs_133915 = s_188_3;
        // N s_188_5: jump b152
        return block_152(state, tracer, fn_state);
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
        // C s_189_5: const #432u : u32
        let s_189_5: u32 = 432;
        // D s_189_6: read-reg s_189_5:u8
        let s_189_6: u8 = {
            let value = state.read_register::<u8>(s_189_5 as isize);
            tracer.read_register(s_189_5 as isize, value);
            value
        };
        // D s_189_7: call AArch64_AArch32SystemAccessTrap(s_189_6, s_189_4)
        let s_189_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_189_6,
            s_189_4,
        );
        // N s_189_8: return
        return;
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_190_0: read-var __HDFGWTR_EL2_PMEVCNTRn_EL0:u8
        let s_190_0: bool = fn_state.u__HDFGWTR_EL2_PMEVCNTRn_EL0;
        // D s_190_1: cast zx s_190_0 -> bv
        let s_190_1: Bits = Bits::new(s_190_0 as u128, 1u16);
        // C s_190_2: const #1u : u8
        let s_190_2: bool = true;
        // C s_190_3: cast zx s_190_2 -> bv
        let s_190_3: Bits = Bits::new(s_190_2 as u128, 1u16);
        // D s_190_4: cmp-eq s_190_1 s_190_3
        let s_190_4: bool = ((s_190_1) == (s_190_3));
        // D s_190_5: write-var gs#133914 <= s_190_4
        fn_state.gs_133914 = s_190_4;
        // N s_190_6: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_191_0: const #424u : u32
        let s_191_0: u32 = 424;
        // D s_191_1: read-reg s_191_0:u8
        let s_191_1: u8 = {
            let value = state.read_register::<u8>(s_191_0 as isize);
            tracer.read_register(s_191_0 as isize, value);
            value
        };
        // C s_191_2: const #2u : u8
        let s_191_2: u8 = 2;
        // D s_191_3: cmp-lt s_191_1 s_191_2
        let s_191_3: bool = ((s_191_1) < (s_191_2));
        // D s_191_4: not s_191_3
        let s_191_4: bool = !s_191_3;
        // N s_191_5: branch s_191_4 b194 b192
        if s_191_4 {
            return block_194(state, tracer, fn_state);
        } else {
            return block_192(state, tracer, fn_state);
        };
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_192_0: read-var __SCR_EL3_FGTEn:u8
        let s_192_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_192_1: cast zx s_192_0 -> bv
        let s_192_1: Bits = Bits::new(s_192_0 as u128, 1u16);
        // C s_192_2: const #1u : u8
        let s_192_2: bool = true;
        // C s_192_3: cast zx s_192_2 -> bv
        let s_192_3: Bits = Bits::new(s_192_2 as u128, 1u16);
        // D s_192_4: cmp-eq s_192_1 s_192_3
        let s_192_4: bool = ((s_192_1) == (s_192_3));
        // D s_192_5: write-var gs#133912 <= s_192_4
        fn_state.gs_133912 = s_192_4;
        // N s_192_6: jump b193
        return block_193(state, tracer, fn_state);
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_193_0: read-var gs#133912:u8
        let s_193_0: bool = fn_state.gs_133912;
        // D s_193_1: write-var gs#133913 <= s_193_0
        fn_state.gs_133913 = s_193_0;
        // N s_193_2: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_194_0: const #1u : u8
        let s_194_0: bool = true;
        // D s_194_1: write-var gs#133912 <= s_194_0
        fn_state.gs_133912 = s_194_0;
        // N s_194_2: jump b193
        return block_193(state, tracer, fn_state);
    }
    fn block_195<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_195_0: const #146u : u32
        let s_195_0: u32 = 146;
        // S s_195_1: call IsFeatureImplemented(s_195_0)
        let s_195_1: bool = IsFeatureImplemented(state, tracer, s_195_0);
        // D s_195_2: write-var gs#133911 <= s_195_1
        fn_state.gs_133911 = s_195_1;
        // N s_195_3: jump b145
        return block_145(state, tracer, fn_state);
    }
    fn block_196<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_196_0: const #102552u : u32
        let s_196_0: u32 = 102552;
        // D s_196_1: read-reg s_196_0:struct
        let s_196_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_196_0 as isize);
            tracer.read_register(s_196_0 as isize, value);
            value
        };
        // D s_196_2: call _get_HCR_EL2_Type_E2H(s_196_1)
        let s_196_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_196_1);
        // C s_196_3: const #102552u : u32
        let s_196_3: u32 = 102552;
        // D s_196_4: read-reg s_196_3:struct
        let s_196_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_196_3 as isize);
            tracer.read_register(s_196_3 as isize, value);
            value
        };
        // D s_196_5: call _get_HCR_EL2_Type_TGE(s_196_4)
        let s_196_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_196_4);
        // D s_196_6: cast zx s_196_2 -> bv
        let s_196_6: Bits = Bits::new(s_196_2 as u128, 1u16);
        // D s_196_7: cast zx s_196_5 -> bv
        let s_196_7: Bits = Bits::new(s_196_5 as u128, 1u16);
        // D s_196_8: cast reint s_196_6 -> u128
        let s_196_8: u128 = (s_196_6.value() as u128);
        // D s_196_9: size-of s_196_6
        let s_196_9: u16 = s_196_6.length();
        // D s_196_10: cast reint s_196_7 -> u128
        let s_196_10: u128 = (s_196_7.value() as u128);
        // D s_196_11: size-of s_196_7
        let s_196_11: u16 = s_196_7.length();
        // D s_196_12: lsl s_196_8 s_196_11
        let s_196_12: u128 = s_196_8 << s_196_11;
        // D s_196_13: or s_196_12 s_196_10
        let s_196_13: u128 = ((s_196_12) | (s_196_10));
        // D s_196_14: add s_196_9 s_196_11
        let s_196_14: u16 = (s_196_9 + s_196_11);
        // D s_196_15: create-bits s_196_13 s_196_14
        let s_196_15: Bits = Bits::new(s_196_13, s_196_14);
        // D s_196_16: cast reint s_196_15 -> u8
        let s_196_16: u8 = (s_196_15.value() as u8);
        // D s_196_17: cast zx s_196_16 -> bv
        let s_196_17: Bits = Bits::new(s_196_16 as u128, 2u16);
        // C s_196_18: const #3u : u8
        let s_196_18: u8 = 3;
        // C s_196_19: cast zx s_196_18 -> bv
        let s_196_19: Bits = Bits::new(s_196_18 as u128, 2u16);
        // D s_196_20: cmp-ne s_196_17 s_196_19
        let s_196_20: bool = ((s_196_17) != (s_196_19));
        // D s_196_21: write-var gs#133910 <= s_196_20
        fn_state.gs_133910 = s_196_20;
        // N s_196_22: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_197<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_197_0: const #440u : u32
        let s_197_0: u32 = 440;
        // D s_197_1: read-reg s_197_0:u8
        let s_197_1: u8 = {
            let value = state.read_register::<u8>(s_197_0 as isize);
            tracer.read_register(s_197_0 as isize, value);
            value
        };
        // D s_197_2: call ELUsingAArch32(s_197_1)
        let s_197_2: bool = ELUsingAArch32(state, tracer, s_197_1);
        // D s_197_3: not s_197_2
        let s_197_3: bool = !s_197_2;
        // D s_197_4: write-var gs#133909 <= s_197_3
        fn_state.gs_133909 = s_197_3;
        // N s_197_5: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_198<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_198_0: const #3u : u8
        let s_198_0: u8 = 3;
        // C s_198_1: cast zx s_198_0 -> bv
        let s_198_1: Bits = Bits::new(s_198_0 as u128, 8u16);
        // C s_198_2: cast zx s_198_1 -> i
        let s_198_2: i128 = (s_198_1.value() as i128);
        // C s_198_3: cast reint s_198_2 -> i64
        let s_198_3: i64 = (s_198_2 as i64);
        // C s_198_4: cast zx s_198_3 -> i
        let s_198_4: i128 = (i128::try_from(s_198_3).unwrap());
        // S s_198_5: call AArch32_TakeHypTrapException(s_198_4)
        let s_198_5: () = AArch32_TakeHypTrapException(state, tracer, s_198_4);
        // N s_198_6: return
        return;
    }
    fn block_199<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_199_0: read-var __HSTR_T9:u8
        let s_199_0: bool = fn_state.u__HSTR_T9;
        // D s_199_1: cast zx s_199_0 -> bv
        let s_199_1: Bits = Bits::new(s_199_0 as u128, 1u16);
        // C s_199_2: const #1u : u8
        let s_199_2: bool = true;
        // C s_199_3: cast zx s_199_2 -> bv
        let s_199_3: Bits = Bits::new(s_199_2 as u128, 1u16);
        // D s_199_4: cmp-eq s_199_1 s_199_3
        let s_199_4: bool = ((s_199_1) == (s_199_3));
        // D s_199_5: write-var gs#133908 <= s_199_4
        fn_state.gs_133908 = s_199_4;
        // N s_199_6: jump b138
        return block_138(state, tracer, fn_state);
    }
    fn block_200<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_200_0: const #432u : u32
        let s_200_0: u32 = 432;
        // D s_200_1: read-reg s_200_0:u8
        let s_200_1: u8 = {
            let value = state.read_register::<u8>(s_200_0 as isize);
            tracer.read_register(s_200_0 as isize, value);
            value
        };
        // D s_200_2: call ELUsingAArch32(s_200_1)
        let s_200_2: bool = ELUsingAArch32(state, tracer, s_200_1);
        // D s_200_3: write-var gs#133907 <= s_200_2
        fn_state.gs_133907 = s_200_2;
        // N s_200_4: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_201<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_201_0: const #3u : u8
        let s_201_0: u8 = 3;
        // C s_201_1: cast zx s_201_0 -> bv
        let s_201_1: Bits = Bits::new(s_201_0 as u128, 8u16);
        // C s_201_2: cast zx s_201_1 -> i
        let s_201_2: i128 = (s_201_1.value() as i128);
        // C s_201_3: cast reint s_201_2 -> i64
        let s_201_3: i64 = (s_201_2 as i64);
        // C s_201_4: cast zx s_201_3 -> i
        let s_201_4: i128 = (i128::try_from(s_201_3).unwrap());
        // C s_201_5: const #432u : u32
        let s_201_5: u32 = 432;
        // D s_201_6: read-reg s_201_5:u8
        let s_201_6: u8 = {
            let value = state.read_register::<u8>(s_201_5 as isize);
            tracer.read_register(s_201_5 as isize, value);
            value
        };
        // D s_201_7: call AArch64_AArch32SystemAccessTrap(s_201_6, s_201_4)
        let s_201_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_201_6,
            s_201_4,
        );
        // N s_201_8: return
        return;
    }
    fn block_202<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_202_0: read-var __HSTR_EL2_T9:u8
        let s_202_0: bool = fn_state.u__HSTR_EL2_T9;
        // D s_202_1: cast zx s_202_0 -> bv
        let s_202_1: Bits = Bits::new(s_202_0 as u128, 1u16);
        // C s_202_2: const #1u : u8
        let s_202_2: bool = true;
        // C s_202_3: cast zx s_202_2 -> bv
        let s_202_3: Bits = Bits::new(s_202_2 as u128, 1u16);
        // D s_202_4: cmp-eq s_202_1 s_202_3
        let s_202_4: bool = ((s_202_1) == (s_202_3));
        // D s_202_5: write-var gs#133906 <= s_202_4
        fn_state.gs_133906 = s_202_4;
        // N s_202_6: jump b133
        return block_133(state, tracer, fn_state);
    }
    fn block_203<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_203_0: const #102552u : u32
        let s_203_0: u32 = 102552;
        // D s_203_1: read-reg s_203_0:struct
        let s_203_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_203_0 as isize);
            tracer.read_register(s_203_0 as isize, value);
            value
        };
        // D s_203_2: call _get_HCR_EL2_Type_E2H(s_203_1)
        let s_203_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_203_1);
        // C s_203_3: const #102552u : u32
        let s_203_3: u32 = 102552;
        // D s_203_4: read-reg s_203_3:struct
        let s_203_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_203_3 as isize);
            tracer.read_register(s_203_3 as isize, value);
            value
        };
        // D s_203_5: call _get_HCR_EL2_Type_TGE(s_203_4)
        let s_203_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_203_4);
        // D s_203_6: cast zx s_203_2 -> bv
        let s_203_6: Bits = Bits::new(s_203_2 as u128, 1u16);
        // D s_203_7: cast zx s_203_5 -> bv
        let s_203_7: Bits = Bits::new(s_203_5 as u128, 1u16);
        // D s_203_8: cast reint s_203_6 -> u128
        let s_203_8: u128 = (s_203_6.value() as u128);
        // D s_203_9: size-of s_203_6
        let s_203_9: u16 = s_203_6.length();
        // D s_203_10: cast reint s_203_7 -> u128
        let s_203_10: u128 = (s_203_7.value() as u128);
        // D s_203_11: size-of s_203_7
        let s_203_11: u16 = s_203_7.length();
        // D s_203_12: lsl s_203_8 s_203_11
        let s_203_12: u128 = s_203_8 << s_203_11;
        // D s_203_13: or s_203_12 s_203_10
        let s_203_13: u128 = ((s_203_12) | (s_203_10));
        // D s_203_14: add s_203_9 s_203_11
        let s_203_14: u16 = (s_203_9 + s_203_11);
        // D s_203_15: create-bits s_203_13 s_203_14
        let s_203_15: Bits = Bits::new(s_203_13, s_203_14);
        // D s_203_16: cast reint s_203_15 -> u8
        let s_203_16: u8 = (s_203_15.value() as u8);
        // D s_203_17: cast zx s_203_16 -> bv
        let s_203_17: Bits = Bits::new(s_203_16 as u128, 2u16);
        // C s_203_18: const #3u : u8
        let s_203_18: u8 = 3;
        // C s_203_19: cast zx s_203_18 -> bv
        let s_203_19: Bits = Bits::new(s_203_18 as u128, 2u16);
        // D s_203_20: cmp-ne s_203_17 s_203_19
        let s_203_20: bool = ((s_203_17) != (s_203_19));
        // D s_203_21: write-var gs#133905 <= s_203_20
        fn_state.gs_133905 = s_203_20;
        // N s_203_22: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_204<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_204_0: const #432u : u32
        let s_204_0: u32 = 432;
        // D s_204_1: read-reg s_204_0:u8
        let s_204_1: u8 = {
            let value = state.read_register::<u8>(s_204_0 as isize);
            tracer.read_register(s_204_0 as isize, value);
            value
        };
        // D s_204_2: call ELUsingAArch32(s_204_1)
        let s_204_2: bool = ELUsingAArch32(state, tracer, s_204_1);
        // D s_204_3: not s_204_2
        let s_204_3: bool = !s_204_2;
        // D s_204_4: write-var gs#133904 <= s_204_3
        fn_state.gs_133904 = s_204_3;
        // N s_204_5: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_205<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_205_0: const #() : ()
        let s_205_0: () = ();
        // S s_205_1: call EL2Enabled(s_205_0)
        let s_205_1: bool = EL2Enabled(state, tracer, s_205_0);
        // N s_205_2: branch s_205_1 b221 b206
        if s_205_1 {
            return block_221(state, tracer, fn_state);
        } else {
            return block_206(state, tracer, fn_state);
        };
    }
    fn block_206<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_206_0: const #0u : u8
        let s_206_0: bool = false;
        // D s_206_1: write-var gs#133923 <= s_206_0
        fn_state.gs_133923 = s_206_0;
        // N s_206_2: jump b207
        return block_207(state, tracer, fn_state);
    }
    fn block_207<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_207_0: read-var gs#133923:u8
        let s_207_0: bool = fn_state.gs_133923;
        // N s_207_1: branch s_207_0 b220 b208
        if s_207_0 {
            return block_220(state, tracer, fn_state);
        } else {
            return block_208(state, tracer, fn_state);
        };
    }
    fn block_208<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_208_0: const #0u : u8
        let s_208_0: bool = false;
        // D s_208_1: write-var gs#133924 <= s_208_0
        fn_state.gs_133924 = s_208_0;
        // N s_208_2: jump b209
        return block_209(state, tracer, fn_state);
    }
    fn block_209<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_209_0: read-var gs#133924:u8
        let s_209_0: bool = fn_state.gs_133924;
        // N s_209_1: branch s_209_0 b219 b210
        if s_209_0 {
            return block_219(state, tracer, fn_state);
        } else {
            return block_210(state, tracer, fn_state);
        };
    }
    fn block_210<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_210_0: const #() : ()
        let s_210_0: () = ();
        // S s_210_1: call EL2Enabled(s_210_0)
        let s_210_1: bool = EL2Enabled(state, tracer, s_210_0);
        // N s_210_2: branch s_210_1 b218 b211
        if s_210_1 {
            return block_218(state, tracer, fn_state);
        } else {
            return block_211(state, tracer, fn_state);
        };
    }
    fn block_211<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_211_0: const #0u : u8
        let s_211_0: bool = false;
        // D s_211_1: write-var gs#133925 <= s_211_0
        fn_state.gs_133925 = s_211_0;
        // N s_211_2: jump b212
        return block_212(state, tracer, fn_state);
    }
    fn block_212<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_212_0: read-var gs#133925:u8
        let s_212_0: bool = fn_state.gs_133925;
        // N s_212_1: branch s_212_0 b217 b213
        if s_212_0 {
            return block_217(state, tracer, fn_state);
        } else {
            return block_213(state, tracer, fn_state);
        };
    }
    fn block_213<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_213_0: const #0u : u8
        let s_213_0: bool = false;
        // D s_213_1: write-var gs#133926 <= s_213_0
        fn_state.gs_133926 = s_213_0;
        // N s_213_2: jump b214
        return block_214(state, tracer, fn_state);
    }
    fn block_214<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_214_0: read-var gs#133926:u8
        let s_214_0: bool = fn_state.gs_133926;
        // N s_214_1: branch s_214_0 b216 b215
        if s_214_0 {
            return block_216(state, tracer, fn_state);
        } else {
            return block_215(state, tracer, fn_state);
        };
    }
    fn block_215<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_215_0: panic
        panic!("{:?}", ());
        // N s_215_1: return
        return;
    }
    fn block_216<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_216_0: const #0u : u8
        let s_216_0: u8 = 0;
        // C s_216_1: cast zx s_216_0 -> bv
        let s_216_1: Bits = Bits::new(s_216_0 as u128, 8u16);
        // C s_216_2: cast zx s_216_1 -> i
        let s_216_2: i128 = (s_216_1.value() as i128);
        // C s_216_3: cast reint s_216_2 -> i64
        let s_216_3: i64 = (s_216_2 as i64);
        // C s_216_4: cast zx s_216_3 -> i
        let s_216_4: i128 = (i128::try_from(s_216_3).unwrap());
        // S s_216_5: call AArch32_TakeHypTrapException(s_216_4)
        let s_216_5: () = AArch32_TakeHypTrapException(state, tracer, s_216_4);
        // N s_216_6: return
        return;
    }
    fn block_217<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_217_0: read-var __HCR_TGE:u8
        let s_217_0: bool = fn_state.u__HCR_TGE;
        // D s_217_1: cast zx s_217_0 -> bv
        let s_217_1: Bits = Bits::new(s_217_0 as u128, 1u16);
        // C s_217_2: const #1u : u8
        let s_217_2: bool = true;
        // C s_217_3: cast zx s_217_2 -> bv
        let s_217_3: Bits = Bits::new(s_217_2 as u128, 1u16);
        // D s_217_4: cmp-eq s_217_1 s_217_3
        let s_217_4: bool = ((s_217_1) == (s_217_3));
        // D s_217_5: write-var gs#133926 <= s_217_4
        fn_state.gs_133926 = s_217_4;
        // N s_217_6: jump b214
        return block_214(state, tracer, fn_state);
    }
    fn block_218<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_218_0: const #432u : u32
        let s_218_0: u32 = 432;
        // D s_218_1: read-reg s_218_0:u8
        let s_218_1: u8 = {
            let value = state.read_register::<u8>(s_218_0 as isize);
            tracer.read_register(s_218_0 as isize, value);
            value
        };
        // D s_218_2: call ELUsingAArch32(s_218_1)
        let s_218_2: bool = ELUsingAArch32(state, tracer, s_218_1);
        // D s_218_3: write-var gs#133925 <= s_218_2
        fn_state.gs_133925 = s_218_2;
        // N s_218_4: jump b212
        return block_212(state, tracer, fn_state);
    }
    fn block_219<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_219_0: const #3u : u8
        let s_219_0: u8 = 3;
        // C s_219_1: cast zx s_219_0 -> bv
        let s_219_1: Bits = Bits::new(s_219_0 as u128, 8u16);
        // C s_219_2: cast zx s_219_1 -> i
        let s_219_2: i128 = (s_219_1.value() as i128);
        // C s_219_3: cast reint s_219_2 -> i64
        let s_219_3: i64 = (s_219_2 as i64);
        // C s_219_4: cast zx s_219_3 -> i
        let s_219_4: i128 = (i128::try_from(s_219_3).unwrap());
        // C s_219_5: const #432u : u32
        let s_219_5: u32 = 432;
        // D s_219_6: read-reg s_219_5:u8
        let s_219_6: u8 = {
            let value = state.read_register::<u8>(s_219_5 as isize);
            tracer.read_register(s_219_5 as isize, value);
            value
        };
        // D s_219_7: call AArch64_AArch32SystemAccessTrap(s_219_6, s_219_4)
        let s_219_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_219_6,
            s_219_4,
        );
        // N s_219_8: return
        return;
    }
    fn block_220<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_220_0: read-var __HCR_EL2_TGE:u8
        let s_220_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_220_1: cast zx s_220_0 -> bv
        let s_220_1: Bits = Bits::new(s_220_0 as u128, 1u16);
        // C s_220_2: const #1u : u8
        let s_220_2: bool = true;
        // C s_220_3: cast zx s_220_2 -> bv
        let s_220_3: Bits = Bits::new(s_220_2 as u128, 1u16);
        // D s_220_4: cmp-eq s_220_1 s_220_3
        let s_220_4: bool = ((s_220_1) == (s_220_3));
        // D s_220_5: write-var gs#133924 <= s_220_4
        fn_state.gs_133924 = s_220_4;
        // N s_220_6: jump b209
        return block_209(state, tracer, fn_state);
    }
    fn block_221<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_221_0: const #432u : u32
        let s_221_0: u32 = 432;
        // D s_221_1: read-reg s_221_0:u8
        let s_221_1: u8 = {
            let value = state.read_register::<u8>(s_221_0 as isize);
            tracer.read_register(s_221_0 as isize, value);
            value
        };
        // D s_221_2: call ELUsingAArch32(s_221_1)
        let s_221_2: bool = ELUsingAArch32(state, tracer, s_221_1);
        // D s_221_3: not s_221_2
        let s_221_3: bool = !s_221_2;
        // D s_221_4: write-var gs#133923 <= s_221_3
        fn_state.gs_133923 = s_221_3;
        // N s_221_5: jump b207
        return block_207(state, tracer, fn_state);
    }
    fn block_222<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_222_0: read-var __PMUSERENR_EN:u8
        let s_222_0: bool = fn_state.u__PMUSERENR_EN;
        // D s_222_1: cast zx s_222_0 -> bv
        let s_222_1: Bits = Bits::new(s_222_0 as u128, 1u16);
        // C s_222_2: const #0u : u8
        let s_222_2: bool = false;
        // C s_222_3: cast zx s_222_2 -> bv
        let s_222_3: Bits = Bits::new(s_222_2 as u128, 1u16);
        // D s_222_4: cmp-eq s_222_1 s_222_3
        let s_222_4: bool = ((s_222_1) == (s_222_3));
        // D s_222_5: write-var gs#133903 <= s_222_4
        fn_state.gs_133903 = s_222_4;
        // N s_222_6: jump b126
        return block_126(state, tracer, fn_state);
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
        // D s_224_1: write-var gs#133927 <= s_224_0
        fn_state.gs_133927 = s_224_0;
        // N s_224_2: jump b225
        return block_225(state, tracer, fn_state);
    }
    fn block_225<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_225_0: read-var gs#133927:u8
        let s_225_0: bool = fn_state.gs_133927;
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
        // D s_226_1: write-var gs#133928 <= s_226_0
        fn_state.gs_133928 = s_226_0;
        // N s_226_2: jump b227
        return block_227(state, tracer, fn_state);
    }
    fn block_227<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_227_0: read-var gs#133928:u8
        let s_227_0: bool = fn_state.gs_133928;
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
        // C s_228_0: const #3u : u8
        let s_228_0: u8 = 3;
        // C s_228_1: cast zx s_228_0 -> bv
        let s_228_1: Bits = Bits::new(s_228_0 as u128, 8u16);
        // C s_228_2: cast zx s_228_1 -> i
        let s_228_2: i128 = (s_228_1.value() as i128);
        // C s_228_3: cast reint s_228_2 -> i64
        let s_228_3: i64 = (s_228_2 as i64);
        // C s_228_4: cast zx s_228_3 -> i
        let s_228_4: i128 = (i128::try_from(s_228_3).unwrap());
        // C s_228_5: const #440u : u32
        let s_228_5: u32 = 440;
        // D s_228_6: read-reg s_228_5:u8
        let s_228_6: u8 = {
            let value = state.read_register::<u8>(s_228_5 as isize);
            tracer.read_register(s_228_5 as isize, value);
            value
        };
        // D s_228_7: call AArch64_AArch32SystemAccessTrap(s_228_6, s_228_4)
        let s_228_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_228_6,
            s_228_4,
        );
        // N s_228_8: return
        return;
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
        // D s_230_5: write-var gs#133928 <= s_230_4
        fn_state.gs_133928 = s_230_4;
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
        // D s_231_3: not s_231_2
        let s_231_3: bool = !s_231_2;
        // D s_231_4: write-var gs#133927 <= s_231_3
        fn_state.gs_133927 = s_231_3;
        // N s_231_5: jump b225
        return block_225(state, tracer, fn_state);
    }
    fn block_232<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_232_0: const #204u : u32
        let s_232_0: u32 = 204;
        // S s_232_1: call IsFeatureImplemented(s_232_0)
        let s_232_1: bool = IsFeatureImplemented(state, tracer, s_232_0);
        // N s_232_2: branch s_232_1 b241 b233
        if s_232_1 {
            return block_241(state, tracer, fn_state);
        } else {
            return block_233(state, tracer, fn_state);
        };
    }
    fn block_233<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_233_0: const #0u : u8
        let s_233_0: bool = false;
        // D s_233_1: write-var gs#133899 <= s_233_0
        fn_state.gs_133899 = s_233_0;
        // N s_233_2: jump b234
        return block_234(state, tracer, fn_state);
    }
    fn block_234<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_234_0: read-var gs#133899:u8
        let s_234_0: bool = fn_state.gs_133899;
        // N s_234_1: branch s_234_0 b240 b235
        if s_234_0 {
            return block_240(state, tracer, fn_state);
        } else {
            return block_235(state, tracer, fn_state);
        };
    }
    fn block_235<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_235_0: const #204u : u32
        let s_235_0: u32 = 204;
        // S s_235_1: call IsFeatureImplemented(s_235_0)
        let s_235_1: bool = IsFeatureImplemented(state, tracer, s_235_0);
        // S s_235_2: not s_235_1
        let s_235_2: bool = !s_235_1;
        // N s_235_3: branch s_235_2 b239 b236
        if s_235_2 {
            return block_239(state, tracer, fn_state);
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
        // D s_236_1: write-var gs#133900 <= s_236_0
        fn_state.gs_133900 = s_236_0;
        // N s_236_2: jump b237
        return block_237(state, tracer, fn_state);
    }
    fn block_237<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_237_0: read-var gs#133900:u8
        let s_237_0: bool = fn_state.gs_133900;
        // D s_237_1: write-var gs#133901 <= s_237_0
        fn_state.gs_133901 = s_237_0;
        // N s_237_2: jump b238
        return block_238(state, tracer, fn_state);
    }
    fn block_238<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_238_0: read-var gs#133901:u8
        let s_238_0: bool = fn_state.gs_133901;
        // D s_238_1: write-var gs#133902 <= s_238_0
        fn_state.gs_133902 = s_238_0;
        // N s_238_2: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_239<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_239_0: read-var __PMUSERENR_EL0_EN:u8
        let s_239_0: bool = fn_state.u__PMUSERENR_EL0_EN;
        // D s_239_1: cast zx s_239_0 -> bv
        let s_239_1: Bits = Bits::new(s_239_0 as u128, 1u16);
        // C s_239_2: const #0u : u8
        let s_239_2: bool = false;
        // C s_239_3: cast zx s_239_2 -> bv
        let s_239_3: Bits = Bits::new(s_239_2 as u128, 1u16);
        // D s_239_4: cmp-eq s_239_1 s_239_3
        let s_239_4: bool = ((s_239_1) == (s_239_3));
        // D s_239_5: write-var gs#133900 <= s_239_4
        fn_state.gs_133900 = s_239_4;
        // N s_239_6: jump b237
        return block_237(state, tracer, fn_state);
    }
    fn block_240<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_240_0: const #1u : u8
        let s_240_0: bool = true;
        // D s_240_1: write-var gs#133901 <= s_240_0
        fn_state.gs_133901 = s_240_0;
        // N s_240_2: jump b238
        return block_238(state, tracer, fn_state);
    }
    fn block_241<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_241_0: const #102624u : u32
        let s_241_0: u32 = 102624;
        // D s_241_1: read-reg s_241_0:struct
        let s_241_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_241_0 as isize);
            tracer.read_register(s_241_0 as isize, value);
            value
        };
        // D s_241_2: call _get_PMUSERENR_EL0_Type_UEN(s_241_1)
        let s_241_2: bool = u_get_PMUSERENR_EL0_Type_UEN(state, tracer, s_241_1);
        // C s_241_3: const #102624u : u32
        let s_241_3: u32 = 102624;
        // D s_241_4: read-reg s_241_3:struct
        let s_241_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_241_3 as isize);
            tracer.read_register(s_241_3 as isize, value);
            value
        };
        // D s_241_5: call _get_PMUSERENR_EL0_Type_EN(s_241_4)
        let s_241_5: bool = u_get_PMUSERENR_EL0_Type_EN(state, tracer, s_241_4);
        // D s_241_6: cast zx s_241_2 -> bv
        let s_241_6: Bits = Bits::new(s_241_2 as u128, 1u16);
        // D s_241_7: cast zx s_241_5 -> bv
        let s_241_7: Bits = Bits::new(s_241_5 as u128, 1u16);
        // D s_241_8: cast reint s_241_6 -> u128
        let s_241_8: u128 = (s_241_6.value() as u128);
        // D s_241_9: size-of s_241_6
        let s_241_9: u16 = s_241_6.length();
        // D s_241_10: cast reint s_241_7 -> u128
        let s_241_10: u128 = (s_241_7.value() as u128);
        // D s_241_11: size-of s_241_7
        let s_241_11: u16 = s_241_7.length();
        // D s_241_12: lsl s_241_8 s_241_11
        let s_241_12: u128 = s_241_8 << s_241_11;
        // D s_241_13: or s_241_12 s_241_10
        let s_241_13: u128 = ((s_241_12) | (s_241_10));
        // D s_241_14: add s_241_9 s_241_11
        let s_241_14: u16 = (s_241_9 + s_241_11);
        // D s_241_15: create-bits s_241_13 s_241_14
        let s_241_15: Bits = Bits::new(s_241_13, s_241_14);
        // D s_241_16: cast reint s_241_15 -> u8
        let s_241_16: u8 = (s_241_15.value() as u8);
        // D s_241_17: cast zx s_241_16 -> bv
        let s_241_17: Bits = Bits::new(s_241_16 as u128, 2u16);
        // C s_241_18: const #0u : u8
        let s_241_18: u8 = 0;
        // C s_241_19: cast zx s_241_18 -> bv
        let s_241_19: Bits = Bits::new(s_241_18 as u128, 2u16);
        // D s_241_20: cmp-eq s_241_17 s_241_19
        let s_241_20: bool = ((s_241_17) == (s_241_19));
        // D s_241_21: write-var gs#133899 <= s_241_20
        fn_state.gs_133899 = s_241_20;
        // N s_241_22: jump b234
        return block_234(state, tracer, fn_state);
    }
    fn block_242<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_242_0: panic
        panic!("{:?}", ());
        // N s_242_1: return
        return;
    }
    fn block_243<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_243_0: read-var __MDCR_EL3_TPM:u8
        let s_243_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_243_1: cast zx s_243_0 -> bv
        let s_243_1: Bits = Bits::new(s_243_0 as u128, 1u16);
        // C s_243_2: const #1u : u8
        let s_243_2: bool = true;
        // C s_243_3: cast zx s_243_2 -> bv
        let s_243_3: Bits = Bits::new(s_243_2 as u128, 1u16);
        // D s_243_4: cmp-eq s_243_1 s_243_3
        let s_243_4: bool = ((s_243_1) == (s_243_3));
        // D s_243_5: write-var gs#133898 <= s_243_4
        fn_state.gs_133898 = s_243_4;
        // N s_243_6: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_244<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_244_0: const #424u : u32
        let s_244_0: u32 = 424;
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
        // D s_244_4: write-var gs#133897 <= s_244_3
        fn_state.gs_133897 = s_244_3;
        // N s_244_5: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_245<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_245_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_245_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_245_1: call __IMPDEF_boolean(s_245_0)
        let s_245_1: bool = u__IMPDEF_boolean(state, tracer, s_245_0);
        // D s_245_2: write-var gs#133896 <= s_245_1
        fn_state.gs_133896 = s_245_1;
        // N s_245_3: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_246<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_246_0: const #() : ()
        let s_246_0: () = ();
        // S s_246_1: call EDSCR_read(s_246_0)
        let s_246_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_246_0);
        // S s_246_2: call _get_EDSCR_Type_SDD(s_246_1)
        let s_246_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_246_1);
        // S s_246_3: cast zx s_246_2 -> bv
        let s_246_3: Bits = Bits::new(s_246_2 as u128, 1u16);
        // C s_246_4: const #1u : u8
        let s_246_4: bool = true;
        // C s_246_5: cast zx s_246_4 -> bv
        let s_246_5: Bits = Bits::new(s_246_4 as u128, 1u16);
        // S s_246_6: cmp-eq s_246_3 s_246_5
        let s_246_6: bool = ((s_246_3) == (s_246_5));
        // D s_246_7: write-var gs#133895 <= s_246_6
        fn_state.gs_133895 = s_246_6;
        // N s_246_8: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_247<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_247_0: const #424u : u32
        let s_247_0: u32 = 424;
        // D s_247_1: read-reg s_247_0:u8
        let s_247_1: u8 = {
            let value = state.read_register::<u8>(s_247_0 as isize);
            tracer.read_register(s_247_0 as isize, value);
            value
        };
        // C s_247_2: const #2u : u8
        let s_247_2: u8 = 2;
        // D s_247_3: cmp-lt s_247_1 s_247_2
        let s_247_3: bool = ((s_247_1) < (s_247_2));
        // D s_247_4: write-var gs#133894 <= s_247_3
        fn_state.gs_133894 = s_247_3;
        // N s_247_5: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_248<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_248_0: const #146u : u32
        let s_248_0: u32 = 146;
        // S s_248_1: call IsFeatureImplemented(s_248_0)
        let s_248_1: bool = IsFeatureImplemented(state, tracer, s_248_0);
        // N s_248_2: branch s_248_1 b250 b249
        if s_248_1 {
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
        // C s_249_0: const #72u : u32
        let s_249_0: u32 = 72;
        // S s_249_1: call ConstrainUnpredictableProcedure(s_249_0)
        let s_249_1: () = ConstrainUnpredictableProcedure(state, tracer, s_249_0);
        // N s_249_2: return
        return;
    }
    fn block_250<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_250_0: panic
        panic!("{:?}", ());
        // N s_250_1: return
        return;
    }
}
