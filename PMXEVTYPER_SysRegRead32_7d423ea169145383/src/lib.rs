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
use AArch32_TakeHypTrapException::*;
use u_get_SCR_EL3_Type_FGTEn::*;
use Halted::*;
use u_get_HSTR_Type_T9::*;
use u_get_MDCR_EL3_Type_TPM::*;
use u__get_selected_PMEVTYPER::*;
use AArch32_GetNumEventCountersAccessible::*;
use u_get_PMUSERENR_Type_EN::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_HSTR_EL2_Type_T9::*;
use u_get_HDFGRTR_EL2_Type_PMEVTYPERn_EL0::*;
use u_get_HCR_EL2_Type_TGE::*;
use u__get_PMEVTYPER::*;
use EL2Enabled::*;
use u_get_MDCR_EL2_Type_TPM::*;
use u_get_PMUSERENR_EL0_Type_UEN::*;
use u_get_HCR_EL2_Type_E2H::*;
use PMCCFILTR_read::*;
use HCR_read::*;
use PMUSERENR_read::*;
use HDCR_read::*;
use IsFeatureImplemented::*;
use u_get_PMSELR_Type_SEL::*;
use u__get_PMCCFILTR::*;
use AArch64_AArch32SystemAccessTrap::*;
use HSTR_read::*;
use u_get_HCR_Type_TGE::*;
use u_get_HDCR_Type_TPM::*;
use neq_int::*;
use u__IMPDEF_boolean::*;
use ConstrainUnpredictableProcedure::*;
use R_set::*;
use ELUsingAArch32::*;
use u_get_PMUSERENR_EL0_Type_EN::*;
use EDSCR_read::*;
use PMSELR_read::*;
use common::*;
pub fn PMXEVTYPER_SysRegRead32_7d423ea169145383<T: Tracer>(
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
        ga_196844: ProductType700c18a878c5601b,
        ga_196833: ProductType700c18a878c5601b,
        gs_117271: bool,
        gs_117263: bool,
        gs_117249: bool,
        gs_117253: bool,
        u__HCR_TGE: bool,
        gs_117297: bool,
        gs_117251: bool,
        gs_117242: bool,
        gs_117283: bool,
        gs_117289: bool,
        gs_117243: bool,
        gs_117252: bool,
        gs_117258: bool,
        gs_117276: bool,
        gs_117257: bool,
        gs_117272: bool,
        u__HCR_EL2_TGE: bool,
        ga_196735: ProductType700c18a878c5601b,
        u__SCR_EL3_FGTEn: bool,
        gs_117275: bool,
        u__PMUSERENR_EL0_EN: bool,
        u__HSTR_T9: bool,
        gs_117279: bool,
        gs_117232: bool,
        gs_117285: bool,
        u__HDCR_TPM: bool,
        gs_117287: bool,
        gs_117235: bool,
        gs_117238: bool,
        gs_117237: bool,
        ga_196841: ProductType700c18a878c5601b,
        gs_117259: bool,
        gs_117298: bool,
        gs_117296: bool,
        gs_117286: bool,
        u__HDFGRTR_EL2_PMEVTYPERn_EL0: bool,
        gs_117277: bool,
        gs_117282: bool,
        gs_117278: bool,
        gs_117246: bool,
        gs_117248: bool,
        gs_117244: bool,
        gs_117280: bool,
        ga_196802: ProductType700c18a878c5601b,
        gs_117294: bool,
        gs_117265: bool,
        gs_117284: bool,
        u__MDCR_EL3_TPM: bool,
        ga_196799: ProductType700c18a878c5601b,
        gs_117234: bool,
        gs_117300: bool,
        gs_117239: bool,
        gs_117267: bool,
        gs_117247: bool,
        gs_117260: bool,
        gs_117292: bool,
        gs_117290: bool,
        gs_117270: bool,
        gs_117262: bool,
        ga_196738: ProductType700c18a878c5601b,
        gs_117236: bool,
        gs_117255: bool,
        u__PSTATE_EL: u8,
        u__MDCR_EL2_TPM: bool,
        u__PMUSERENR_EN: bool,
        gs_117250: bool,
        gs_117299: bool,
        gs_117268: bool,
        gs_117266: bool,
        gs_117254: bool,
        gs_117240: bool,
        gs_117291: bool,
        gs_117264: bool,
        gs_117269: bool,
        gs_117295: bool,
        gs_117274: bool,
        ga_196830: ProductType700c18a878c5601b,
        gs_117245: bool,
        gs_117273: bool,
        gs_117281: bool,
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
        // C s_0_35: const #19144u : u32
        let s_0_35: u32 = 19144;
        // D s_0_36: read-reg s_0_35:struct
        let s_0_36: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_35 as isize);
            tracer.read_register(s_0_35 as isize, value);
            value
        };
        // D s_0_37: call _get_HDFGRTR_EL2_Type_PMEVTYPERn_EL0(s_0_36)
        let s_0_37: bool = u_get_HDFGRTR_EL2_Type_PMEVTYPERn_EL0(state, tracer, s_0_36);
        // D s_0_38: write-var __HDFGRTR_EL2_PMEVTYPERn_EL0 <= s_0_37
        fn_state.u__HDFGRTR_EL2_PMEVTYPERn_EL0 = s_0_37;
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
        // S s_0_55: call neq_int(s_0_54, s_0_53)
        let s_0_55: bool = neq_int(state, tracer, s_0_54, s_0_53);
        // N s_0_56: branch s_0_55 b267 b1
        if s_0_55 {
            return block_267(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#117232 <= s_1_0
        fn_state.gs_117232 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#117232:u8
        let s_2_0: bool = fn_state.gs_117232;
        // N s_2_1: branch s_2_0 b264 b3
        if s_2_0 {
            return block_264(state, tracer, fn_state);
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
        // C s_3_2: const #448u : u32
        let s_3_2: u32 = 448;
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
        // N s_3_6: branch s_3_5 b121 b4
        if s_3_5 {
            return block_121(state, tracer, fn_state);
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
        // C s_4_2: const #440u : u32
        let s_4_2: u32 = 440;
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
        // N s_4_6: branch s_4_5 b44 b5
        if s_4_5 {
            return block_44(state, tracer, fn_state);
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
        // C s_5_2: const #432u : u32
        let s_5_2: u32 = 432;
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
        // N s_5_6: branch s_5_5 b11 b6
        if s_5_5 {
            return block_11(state, tracer, fn_state);
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
        // C s_6_2: const #424u : u32
        let s_6_2: u32 = 424;
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
        // N s_6_6: branch s_6_5 b8 b7
        if s_6_5 {
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
        // N s_7_0: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call PMSELR_read(s_8_0)
        let s_8_1: ProductType700c18a878c5601b = PMSELR_read(state, tracer, s_8_0);
        // S s_8_2: call _get_PMSELR_Type_SEL(s_8_1)
        let s_8_2: u8 = u_get_PMSELR_Type_SEL(state, tracer, s_8_1);
        // S s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 5u16);
        // S s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (s_8_3.value() as i128);
        // S s_8_5: cast reint s_8_4 -> i64
        let s_8_5: i64 = (s_8_4 as i64);
        // C s_8_6: const #31s : i
        let s_8_6: i128 = 31;
        // S s_8_7: cast zx s_8_5 -> i
        let s_8_7: i128 = (i128::try_from(s_8_5).unwrap());
        // S s_8_8: cmp-eq s_8_7 s_8_6
        let s_8_8: bool = ((s_8_7) == (s_8_6));
        // N s_8_9: branch s_8_8 b10 b9
        if s_8_8 {
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
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call __get_selected_PMEVTYPER(s_9_0)
        let s_9_1: ProductType700c18a878c5601b = u__get_selected_PMEVTYPER(
            state,
            tracer,
            s_9_0,
        );
        // S s_9_2: call __get_PMEVTYPER(s_9_1)
        let s_9_2: ProductType700c18a878c5601b = u__get_PMEVTYPER(state, tracer, s_9_1);
        // D s_9_3: write-var ga#196844 <= s_9_2
        fn_state.ga_196844 = s_9_2;
        // D s_9_4: read-var ga#196844.0:struct
        let s_9_4: u32 = fn_state.ga_196844._0;
        // D s_9_5: read-var t:i
        let s_9_5: i128 = fn_state.t;
        // D s_9_6: call R_set(s_9_5, s_9_4)
        let s_9_6: () = R_set(state, tracer, s_9_5, s_9_4);
        // N s_9_7: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call PMCCFILTR_read(s_10_0)
        let s_10_1: ProductType700c18a878c5601b = PMCCFILTR_read(state, tracer, s_10_0);
        // S s_10_2: call __get_PMCCFILTR(s_10_1)
        let s_10_2: ProductType700c18a878c5601b = u__get_PMCCFILTR(
            state,
            tracer,
            s_10_1,
        );
        // D s_10_3: write-var ga#196841 <= s_10_2
        fn_state.ga_196841 = s_10_2;
        // D s_10_4: read-var ga#196841.0:struct
        let s_10_4: u32 = fn_state.ga_196841._0;
        // D s_10_5: read-var t:i
        let s_10_5: i128 = fn_state.t;
        // D s_10_6: call R_set(s_10_5, s_10_4)
        let s_10_6: () = R_set(state, tracer, s_10_5, s_10_4);
        // N s_10_7: return
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
        // N s_11_2: branch s_11_1 b43 b12
        if s_11_1 {
            return block_43(state, tracer, fn_state);
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
        // D s_12_1: write-var gs#117234 <= s_12_0
        fn_state.gs_117234 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#117234:u8
        let s_13_0: bool = fn_state.gs_117234;
        // N s_13_1: branch s_13_0 b42 b14
        if s_13_0 {
            return block_42(state, tracer, fn_state);
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
        // D s_14_1: write-var gs#117235 <= s_14_0
        fn_state.gs_117235 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#117235:u8
        let s_15_0: bool = fn_state.gs_117235;
        // N s_15_1: branch s_15_0 b41 b16
        if s_15_0 {
            return block_41(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#117236 <= s_16_0
        fn_state.gs_117236 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#117236:u8
        let s_17_0: bool = fn_state.gs_117236;
        // N s_17_1: branch s_17_0 b40 b18
        if s_17_0 {
            return block_40(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#117237 <= s_18_0
        fn_state.gs_117237 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#117237:u8
        let s_19_0: bool = fn_state.gs_117237;
        // N s_19_1: branch s_19_0 b39 b20
        if s_19_0 {
            return block_39(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#117238 <= s_20_0
        fn_state.gs_117238 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#117238:u8
        let s_21_0: bool = fn_state.gs_117238;
        // N s_21_1: branch s_21_0 b38 b22
        if s_21_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #424u : u32
        let s_22_0: u32 = 424;
        // D s_22_1: read-reg s_22_0:u8
        let s_22_1: u8 = {
            let value = state.read_register::<u8>(s_22_0 as isize);
            tracer.read_register(s_22_0 as isize, value);
            value
        };
        // C s_22_2: const #2u : u8
        let s_22_2: u8 = 2;
        // D s_22_3: cmp-lt s_22_1 s_22_2
        let s_22_3: bool = ((s_22_1) < (s_22_2));
        // N s_22_4: branch s_22_3 b37 b23
        if s_22_3 {
            return block_37(state, tracer, fn_state);
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
        // D s_23_1: write-var gs#117239 <= s_23_0
        fn_state.gs_117239 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#117239:u8
        let s_24_0: bool = fn_state.gs_117239;
        // N s_24_1: branch s_24_0 b36 b25
        if s_24_0 {
            return block_36(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#117240 <= s_25_0
        fn_state.gs_117240 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#117240:u8
        let s_26_0: bool = fn_state.gs_117240;
        // N s_26_1: branch s_26_0 b30 b27
        if s_26_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #() : ()
        let s_27_0: () = ();
        // S s_27_1: call PMSELR_read(s_27_0)
        let s_27_1: ProductType700c18a878c5601b = PMSELR_read(state, tracer, s_27_0);
        // S s_27_2: call _get_PMSELR_Type_SEL(s_27_1)
        let s_27_2: u8 = u_get_PMSELR_Type_SEL(state, tracer, s_27_1);
        // S s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 5u16);
        // S s_27_4: cast zx s_27_3 -> i
        let s_27_4: i128 = (s_27_3.value() as i128);
        // S s_27_5: cast reint s_27_4 -> i64
        let s_27_5: i64 = (s_27_4 as i64);
        // C s_27_6: const #31s : i
        let s_27_6: i128 = 31;
        // S s_27_7: cast zx s_27_5 -> i
        let s_27_7: i128 = (i128::try_from(s_27_5).unwrap());
        // S s_27_8: cmp-eq s_27_7 s_27_6
        let s_27_8: bool = ((s_27_7) == (s_27_6));
        // N s_27_9: branch s_27_8 b29 b28
        if s_27_8 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call __get_selected_PMEVTYPER(s_28_0)
        let s_28_1: ProductType700c18a878c5601b = u__get_selected_PMEVTYPER(
            state,
            tracer,
            s_28_0,
        );
        // S s_28_2: call __get_PMEVTYPER(s_28_1)
        let s_28_2: ProductType700c18a878c5601b = u__get_PMEVTYPER(
            state,
            tracer,
            s_28_1,
        );
        // D s_28_3: write-var ga#196833 <= s_28_2
        fn_state.ga_196833 = s_28_2;
        // D s_28_4: read-var ga#196833.0:struct
        let s_28_4: u32 = fn_state.ga_196833._0;
        // D s_28_5: read-var t:i
        let s_28_5: i128 = fn_state.t;
        // D s_28_6: call R_set(s_28_5, s_28_4)
        let s_28_6: () = R_set(state, tracer, s_28_5, s_28_4);
        // N s_28_7: return
        return;
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #() : ()
        let s_29_0: () = ();
        // S s_29_1: call PMCCFILTR_read(s_29_0)
        let s_29_1: ProductType700c18a878c5601b = PMCCFILTR_read(state, tracer, s_29_0);
        // S s_29_2: call __get_PMCCFILTR(s_29_1)
        let s_29_2: ProductType700c18a878c5601b = u__get_PMCCFILTR(
            state,
            tracer,
            s_29_1,
        );
        // D s_29_3: write-var ga#196830 <= s_29_2
        fn_state.ga_196830 = s_29_2;
        // D s_29_4: read-var ga#196830.0:struct
        let s_29_4: u32 = fn_state.ga_196830._0;
        // D s_29_5: read-var t:i
        let s_29_5: i128 = fn_state.t;
        // D s_29_6: call R_set(s_29_5, s_29_4)
        let s_29_6: () = R_set(state, tracer, s_29_5, s_29_4);
        // N s_29_7: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #() : ()
        let s_30_0: () = ();
        // S s_30_1: call Halted(s_30_0)
        let s_30_1: bool = Halted(state, tracer, s_30_0);
        // N s_30_2: branch s_30_1 b35 b31
        if s_30_1 {
            return block_35(state, tracer, fn_state);
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
        // D s_31_1: write-var gs#117242 <= s_31_0
        fn_state.gs_117242 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#117242:u8
        let s_32_0: bool = fn_state.gs_117242;
        // N s_32_1: branch s_32_0 b34 b33
        if s_32_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #3u : u8
        let s_33_0: u8 = 3;
        // C s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 8u16);
        // C s_33_2: cast zx s_33_1 -> i
        let s_33_2: i128 = (s_33_1.value() as i128);
        // C s_33_3: cast reint s_33_2 -> i64
        let s_33_3: i64 = (s_33_2 as i64);
        // C s_33_4: cast zx s_33_3 -> i
        let s_33_4: i128 = (i128::try_from(s_33_3).unwrap());
        // C s_33_5: const #424u : u32
        let s_33_5: u32 = 424;
        // D s_33_6: read-reg s_33_5:u8
        let s_33_6: u8 = {
            let value = state.read_register::<u8>(s_33_5 as isize);
            tracer.read_register(s_33_5 as isize, value);
            value
        };
        // D s_33_7: call AArch64_AArch32SystemAccessTrap(s_33_6, s_33_4)
        let s_33_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_33_6, s_33_4);
        // N s_33_8: return
        return;
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_34_0: panic
        panic!("{:?}", ());
        // N s_34_1: return
        return;
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
        // D s_35_7: write-var gs#117242 <= s_35_6
        fn_state.gs_117242 = s_35_6;
        // N s_35_8: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var __MDCR_EL3_TPM:u8
        let s_36_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_36_1: cast zx s_36_0 -> bv
        let s_36_1: Bits = Bits::new(s_36_0 as u128, 1u16);
        // C s_36_2: const #1u : u8
        let s_36_2: bool = true;
        // C s_36_3: cast zx s_36_2 -> bv
        let s_36_3: Bits = Bits::new(s_36_2 as u128, 1u16);
        // D s_36_4: cmp-eq s_36_1 s_36_3
        let s_36_4: bool = ((s_36_1) == (s_36_3));
        // D s_36_5: write-var gs#117240 <= s_36_4
        fn_state.gs_117240 = s_36_4;
        // N s_36_6: jump b26
        return block_26(state, tracer, fn_state);
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
        // D s_37_2: call ELUsingAArch32(s_37_1)
        let s_37_2: bool = ELUsingAArch32(state, tracer, s_37_1);
        // D s_37_3: not s_37_2
        let s_37_3: bool = !s_37_2;
        // D s_37_4: write-var gs#117239 <= s_37_3
        fn_state.gs_117239 = s_37_3;
        // N s_37_5: jump b24
        return block_24(state, tracer, fn_state);
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
        // D s_39_0: read-var __MDCR_EL3_TPM:u8
        let s_39_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_39_1: cast zx s_39_0 -> bv
        let s_39_1: Bits = Bits::new(s_39_0 as u128, 1u16);
        // C s_39_2: const #1u : u8
        let s_39_2: bool = true;
        // C s_39_3: cast zx s_39_2 -> bv
        let s_39_3: Bits = Bits::new(s_39_2 as u128, 1u16);
        // D s_39_4: cmp-eq s_39_1 s_39_3
        let s_39_4: bool = ((s_39_1) == (s_39_3));
        // D s_39_5: write-var gs#117238 <= s_39_4
        fn_state.gs_117238 = s_39_4;
        // N s_39_6: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #424u : u32
        let s_40_0: u32 = 424;
        // D s_40_1: read-reg s_40_0:u8
        let s_40_1: u8 = {
            let value = state.read_register::<u8>(s_40_0 as isize);
            tracer.read_register(s_40_0 as isize, value);
            value
        };
        // D s_40_2: call ELUsingAArch32(s_40_1)
        let s_40_2: bool = ELUsingAArch32(state, tracer, s_40_1);
        // D s_40_3: not s_40_2
        let s_40_3: bool = !s_40_2;
        // D s_40_4: write-var gs#117237 <= s_40_3
        fn_state.gs_117237 = s_40_3;
        // N s_40_5: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_41_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_41_1: call __IMPDEF_boolean(s_41_0)
        let s_41_1: bool = u__IMPDEF_boolean(state, tracer, s_41_0);
        // D s_41_2: write-var gs#117236 <= s_41_1
        fn_state.gs_117236 = s_41_1;
        // N s_41_3: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #() : ()
        let s_42_0: () = ();
        // S s_42_1: call EDSCR_read(s_42_0)
        let s_42_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_42_0);
        // S s_42_2: call _get_EDSCR_Type_SDD(s_42_1)
        let s_42_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_42_1);
        // S s_42_3: cast zx s_42_2 -> bv
        let s_42_3: Bits = Bits::new(s_42_2 as u128, 1u16);
        // C s_42_4: const #1u : u8
        let s_42_4: bool = true;
        // C s_42_5: cast zx s_42_4 -> bv
        let s_42_5: Bits = Bits::new(s_42_4 as u128, 1u16);
        // S s_42_6: cmp-eq s_42_3 s_42_5
        let s_42_6: bool = ((s_42_3) == (s_42_5));
        // D s_42_7: write-var gs#117235 <= s_42_6
        fn_state.gs_117235 = s_42_6;
        // N s_42_8: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #424u : u32
        let s_43_0: u32 = 424;
        // D s_43_1: read-reg s_43_0:u8
        let s_43_1: u8 = {
            let value = state.read_register::<u8>(s_43_0 as isize);
            tracer.read_register(s_43_0 as isize, value);
            value
        };
        // C s_43_2: const #2u : u8
        let s_43_2: u8 = 2;
        // D s_43_3: cmp-lt s_43_1 s_43_2
        let s_43_3: bool = ((s_43_1) < (s_43_2));
        // D s_43_4: write-var gs#117234 <= s_43_3
        fn_state.gs_117234 = s_43_3;
        // N s_43_5: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #() : ()
        let s_44_0: () = ();
        // S s_44_1: call Halted(s_44_0)
        let s_44_1: bool = Halted(state, tracer, s_44_0);
        // N s_44_2: branch s_44_1 b120 b45
        if s_44_1 {
            return block_120(state, tracer, fn_state);
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
        // D s_45_1: write-var gs#117243 <= s_45_0
        fn_state.gs_117243 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#117243:u8
        let s_46_0: bool = fn_state.gs_117243;
        // N s_46_1: branch s_46_0 b119 b47
        if s_46_0 {
            return block_119(state, tracer, fn_state);
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
        // D s_47_1: write-var gs#117244 <= s_47_0
        fn_state.gs_117244 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#117244:u8
        let s_48_0: bool = fn_state.gs_117244;
        // N s_48_1: branch s_48_0 b118 b49
        if s_48_0 {
            return block_118(state, tracer, fn_state);
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
        // D s_49_1: write-var gs#117245 <= s_49_0
        fn_state.gs_117245 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#117245:u8
        let s_50_0: bool = fn_state.gs_117245;
        // N s_50_1: branch s_50_0 b117 b51
        if s_50_0 {
            return block_117(state, tracer, fn_state);
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
        // D s_51_1: write-var gs#117246 <= s_51_0
        fn_state.gs_117246 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#117246:u8
        let s_52_0: bool = fn_state.gs_117246;
        // N s_52_1: branch s_52_0 b116 b53
        if s_52_0 {
            return block_116(state, tracer, fn_state);
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
        // D s_53_1: write-var gs#117247 <= s_53_0
        fn_state.gs_117247 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#117247:u8
        let s_54_0: bool = fn_state.gs_117247;
        // N s_54_1: branch s_54_0 b115 b55
        if s_54_0 {
            return block_115(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #() : ()
        let s_55_0: () = ();
        // S s_55_1: call EL2Enabled(s_55_0)
        let s_55_1: bool = EL2Enabled(state, tracer, s_55_0);
        // N s_55_2: branch s_55_1 b114 b56
        if s_55_1 {
            return block_114(state, tracer, fn_state);
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
        // D s_56_1: write-var gs#117248 <= s_56_0
        fn_state.gs_117248 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#117248:u8
        let s_57_0: bool = fn_state.gs_117248;
        // N s_57_1: branch s_57_0 b113 b58
        if s_57_0 {
            return block_113(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #0u : u8
        let s_58_0: bool = false;
        // D s_58_1: write-var gs#117249 <= s_58_0
        fn_state.gs_117249 = s_58_0;
        // N s_58_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var gs#117249:u8
        let s_59_0: bool = fn_state.gs_117249;
        // N s_59_1: branch s_59_0 b112 b60
        if s_59_0 {
            return block_112(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #() : ()
        let s_60_0: () = ();
        // S s_60_1: call EL2Enabled(s_60_0)
        let s_60_1: bool = EL2Enabled(state, tracer, s_60_0);
        // N s_60_2: branch s_60_1 b111 b61
        if s_60_1 {
            return block_111(state, tracer, fn_state);
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
        // D s_61_1: write-var gs#117250 <= s_61_0
        fn_state.gs_117250 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#117250:u8
        let s_62_0: bool = fn_state.gs_117250;
        // N s_62_1: branch s_62_0 b110 b63
        if s_62_0 {
            return block_110(state, tracer, fn_state);
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
        // D s_63_1: write-var gs#117251 <= s_63_0
        fn_state.gs_117251 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#117251:u8
        let s_64_0: bool = fn_state.gs_117251;
        // N s_64_1: branch s_64_0 b109 b65
        if s_64_0 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #() : ()
        let s_65_0: () = ();
        // S s_65_1: call EL2Enabled(s_65_0)
        let s_65_1: bool = EL2Enabled(state, tracer, s_65_0);
        // N s_65_2: branch s_65_1 b108 b66
        if s_65_1 {
            return block_108(state, tracer, fn_state);
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
        // D s_66_1: write-var gs#117252 <= s_66_0
        fn_state.gs_117252 = s_66_0;
        // N s_66_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#117252:u8
        let s_67_0: bool = fn_state.gs_117252;
        // N s_67_1: branch s_67_0 b107 b68
        if s_67_0 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #0u : u8
        let s_68_0: bool = false;
        // D s_68_1: write-var gs#117253 <= s_68_0
        fn_state.gs_117253 = s_68_0;
        // N s_68_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var gs#117253:u8
        let s_69_0: bool = fn_state.gs_117253;
        // N s_69_1: branch s_69_0 b106 b70
        if s_69_0 {
            return block_106(state, tracer, fn_state);
        } else {
            return block_70(state, tracer, fn_state);
        };
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #() : ()
        let s_70_0: () = ();
        // S s_70_1: call EL2Enabled(s_70_0)
        let s_70_1: bool = EL2Enabled(state, tracer, s_70_0);
        // N s_70_2: branch s_70_1 b105 b71
        if s_70_1 {
            return block_105(state, tracer, fn_state);
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
        // D s_71_1: write-var gs#117254 <= s_71_0
        fn_state.gs_117254 = s_71_0;
        // N s_71_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var gs#117254:u8
        let s_72_0: bool = fn_state.gs_117254;
        // N s_72_1: branch s_72_0 b104 b73
        if s_72_0 {
            return block_104(state, tracer, fn_state);
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
        // D s_73_1: write-var gs#117255 <= s_73_0
        fn_state.gs_117255 = s_73_0;
        // N s_73_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var gs#117255:u8
        let s_74_0: bool = fn_state.gs_117255;
        // N s_74_1: branch s_74_0 b103 b75
        if s_74_0 {
            return block_103(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #() : ()
        let s_75_0: () = ();
        // S s_75_1: call EL2Enabled(s_75_0)
        let s_75_1: bool = EL2Enabled(state, tracer, s_75_0);
        // N s_75_2: branch s_75_1 b102 b76
        if s_75_1 {
            return block_102(state, tracer, fn_state);
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
        // D s_76_1: write-var gs#117257 <= s_76_0
        fn_state.gs_117257 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var gs#117257:u8
        let s_77_0: bool = fn_state.gs_117257;
        // N s_77_1: branch s_77_0 b101 b78
        if s_77_0 {
            return block_101(state, tracer, fn_state);
        } else {
            return block_78(state, tracer, fn_state);
        };
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #0u : u8
        let s_78_0: bool = false;
        // D s_78_1: write-var gs#117258 <= s_78_0
        fn_state.gs_117258 = s_78_0;
        // N s_78_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var gs#117258:u8
        let s_79_0: bool = fn_state.gs_117258;
        // N s_79_1: branch s_79_0 b96 b80
        if s_79_0 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #424u : u32
        let s_80_0: u32 = 424;
        // D s_80_1: read-reg s_80_0:u8
        let s_80_1: u8 = {
            let value = state.read_register::<u8>(s_80_0 as isize);
            tracer.read_register(s_80_0 as isize, value);
            value
        };
        // C s_80_2: const #2u : u8
        let s_80_2: u8 = 2;
        // D s_80_3: cmp-lt s_80_1 s_80_2
        let s_80_3: bool = ((s_80_1) < (s_80_2));
        // N s_80_4: branch s_80_3 b95 b81
        if s_80_3 {
            return block_95(state, tracer, fn_state);
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
        // D s_81_1: write-var gs#117259 <= s_81_0
        fn_state.gs_117259 = s_81_0;
        // N s_81_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var gs#117259:u8
        let s_82_0: bool = fn_state.gs_117259;
        // N s_82_1: branch s_82_0 b94 b83
        if s_82_0 {
            return block_94(state, tracer, fn_state);
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
        // D s_83_1: write-var gs#117260 <= s_83_0
        fn_state.gs_117260 = s_83_0;
        // N s_83_2: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var gs#117260:u8
        let s_84_0: bool = fn_state.gs_117260;
        // N s_84_1: branch s_84_0 b88 b85
        if s_84_0 {
            return block_88(state, tracer, fn_state);
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
        // S s_85_1: call PMSELR_read(s_85_0)
        let s_85_1: ProductType700c18a878c5601b = PMSELR_read(state, tracer, s_85_0);
        // S s_85_2: call _get_PMSELR_Type_SEL(s_85_1)
        let s_85_2: u8 = u_get_PMSELR_Type_SEL(state, tracer, s_85_1);
        // S s_85_3: cast zx s_85_2 -> bv
        let s_85_3: Bits = Bits::new(s_85_2 as u128, 5u16);
        // S s_85_4: cast zx s_85_3 -> i
        let s_85_4: i128 = (s_85_3.value() as i128);
        // S s_85_5: cast reint s_85_4 -> i64
        let s_85_5: i64 = (s_85_4 as i64);
        // C s_85_6: const #31s : i
        let s_85_6: i128 = 31;
        // S s_85_7: cast zx s_85_5 -> i
        let s_85_7: i128 = (i128::try_from(s_85_5).unwrap());
        // S s_85_8: cmp-eq s_85_7 s_85_6
        let s_85_8: bool = ((s_85_7) == (s_85_6));
        // N s_85_9: branch s_85_8 b87 b86
        if s_85_8 {
            return block_87(state, tracer, fn_state);
        } else {
            return block_86(state, tracer, fn_state);
        };
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #() : ()
        let s_86_0: () = ();
        // S s_86_1: call __get_selected_PMEVTYPER(s_86_0)
        let s_86_1: ProductType700c18a878c5601b = u__get_selected_PMEVTYPER(
            state,
            tracer,
            s_86_0,
        );
        // S s_86_2: call __get_PMEVTYPER(s_86_1)
        let s_86_2: ProductType700c18a878c5601b = u__get_PMEVTYPER(
            state,
            tracer,
            s_86_1,
        );
        // D s_86_3: write-var ga#196802 <= s_86_2
        fn_state.ga_196802 = s_86_2;
        // D s_86_4: read-var ga#196802.0:struct
        let s_86_4: u32 = fn_state.ga_196802._0;
        // D s_86_5: read-var t:i
        let s_86_5: i128 = fn_state.t;
        // D s_86_6: call R_set(s_86_5, s_86_4)
        let s_86_6: () = R_set(state, tracer, s_86_5, s_86_4);
        // N s_86_7: return
        return;
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #() : ()
        let s_87_0: () = ();
        // S s_87_1: call PMCCFILTR_read(s_87_0)
        let s_87_1: ProductType700c18a878c5601b = PMCCFILTR_read(state, tracer, s_87_0);
        // S s_87_2: call __get_PMCCFILTR(s_87_1)
        let s_87_2: ProductType700c18a878c5601b = u__get_PMCCFILTR(
            state,
            tracer,
            s_87_1,
        );
        // D s_87_3: write-var ga#196799 <= s_87_2
        fn_state.ga_196799 = s_87_2;
        // D s_87_4: read-var ga#196799.0:struct
        let s_87_4: u32 = fn_state.ga_196799._0;
        // D s_87_5: read-var t:i
        let s_87_5: i128 = fn_state.t;
        // D s_87_6: call R_set(s_87_5, s_87_4)
        let s_87_6: () = R_set(state, tracer, s_87_5, s_87_4);
        // N s_87_7: return
        return;
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #() : ()
        let s_88_0: () = ();
        // S s_88_1: call Halted(s_88_0)
        let s_88_1: bool = Halted(state, tracer, s_88_0);
        // N s_88_2: branch s_88_1 b93 b89
        if s_88_1 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_89(state, tracer, fn_state);
        };
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #0u : u8
        let s_89_0: bool = false;
        // D s_89_1: write-var gs#117262 <= s_89_0
        fn_state.gs_117262 = s_89_0;
        // N s_89_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var gs#117262:u8
        let s_90_0: bool = fn_state.gs_117262;
        // N s_90_1: branch s_90_0 b92 b91
        if s_90_0 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_91(state, tracer, fn_state);
        };
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
        // C s_91_5: const #424u : u32
        let s_91_5: u32 = 424;
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
        // N s_92_0: panic
        panic!("{:?}", ());
        // N s_92_1: return
        return;
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #() : ()
        let s_93_0: () = ();
        // S s_93_1: call EDSCR_read(s_93_0)
        let s_93_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_93_0);
        // S s_93_2: call _get_EDSCR_Type_SDD(s_93_1)
        let s_93_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_93_1);
        // S s_93_3: cast zx s_93_2 -> bv
        let s_93_3: Bits = Bits::new(s_93_2 as u128, 1u16);
        // C s_93_4: const #1u : u8
        let s_93_4: bool = true;
        // C s_93_5: cast zx s_93_4 -> bv
        let s_93_5: Bits = Bits::new(s_93_4 as u128, 1u16);
        // S s_93_6: cmp-eq s_93_3 s_93_5
        let s_93_6: bool = ((s_93_3) == (s_93_5));
        // D s_93_7: write-var gs#117262 <= s_93_6
        fn_state.gs_117262 = s_93_6;
        // N s_93_8: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var __MDCR_EL3_TPM:u8
        let s_94_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_94_1: cast zx s_94_0 -> bv
        let s_94_1: Bits = Bits::new(s_94_0 as u128, 1u16);
        // C s_94_2: const #1u : u8
        let s_94_2: bool = true;
        // C s_94_3: cast zx s_94_2 -> bv
        let s_94_3: Bits = Bits::new(s_94_2 as u128, 1u16);
        // D s_94_4: cmp-eq s_94_1 s_94_3
        let s_94_4: bool = ((s_94_1) == (s_94_3));
        // D s_94_5: write-var gs#117260 <= s_94_4
        fn_state.gs_117260 = s_94_4;
        // N s_94_6: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #424u : u32
        let s_95_0: u32 = 424;
        // D s_95_1: read-reg s_95_0:u8
        let s_95_1: u8 = {
            let value = state.read_register::<u8>(s_95_0 as isize);
            tracer.read_register(s_95_0 as isize, value);
            value
        };
        // D s_95_2: call ELUsingAArch32(s_95_1)
        let s_95_2: bool = ELUsingAArch32(state, tracer, s_95_1);
        // D s_95_3: not s_95_2
        let s_95_3: bool = !s_95_2;
        // D s_95_4: write-var gs#117259 <= s_95_3
        fn_state.gs_117259 = s_95_3;
        // N s_95_5: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #146u : u32
        let s_96_0: u32 = 146;
        // S s_96_1: call IsFeatureImplemented(s_96_0)
        let s_96_1: bool = IsFeatureImplemented(state, tracer, s_96_0);
        // S s_96_2: not s_96_1
        let s_96_2: bool = !s_96_1;
        // N s_96_3: branch s_96_2 b100 b97
        if s_96_2 {
            return block_100(state, tracer, fn_state);
        } else {
            return block_97(state, tracer, fn_state);
        };
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
        // N s_97_3: branch s_97_2 b99 b98
        if s_97_2 {
            return block_99(state, tracer, fn_state);
        } else {
            return block_98(state, tracer, fn_state);
        };
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
        // C s_99_0: const #3u : u8
        let s_99_0: u8 = 3;
        // C s_99_1: cast zx s_99_0 -> bv
        let s_99_1: Bits = Bits::new(s_99_0 as u128, 8u16);
        // C s_99_2: cast zx s_99_1 -> i
        let s_99_2: i128 = (s_99_1.value() as i128);
        // C s_99_3: cast reint s_99_2 -> i64
        let s_99_3: i64 = (s_99_2 as i64);
        // C s_99_4: cast zx s_99_3 -> i
        let s_99_4: i128 = (i128::try_from(s_99_3).unwrap());
        // S s_99_5: call AArch32_TakeHypTrapException(s_99_4)
        let s_99_5: () = AArch32_TakeHypTrapException(state, tracer, s_99_4);
        // N s_99_6: return
        return;
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #72u : u32
        let s_100_0: u32 = 72;
        // S s_100_1: call ConstrainUnpredictableProcedure(s_100_0)
        let s_100_1: () = ConstrainUnpredictableProcedure(state, tracer, s_100_0);
        // N s_100_2: return
        return;
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #() : ()
        let s_101_0: () = ();
        // S s_101_1: call PMSELR_read(s_101_0)
        let s_101_1: ProductType700c18a878c5601b = PMSELR_read(state, tracer, s_101_0);
        // S s_101_2: call _get_PMSELR_Type_SEL(s_101_1)
        let s_101_2: u8 = u_get_PMSELR_Type_SEL(state, tracer, s_101_1);
        // S s_101_3: cast zx s_101_2 -> bv
        let s_101_3: Bits = Bits::new(s_101_2 as u128, 5u16);
        // S s_101_4: cast zx s_101_3 -> i
        let s_101_4: i128 = (s_101_3.value() as i128);
        // S s_101_5: cast reint s_101_4 -> i64
        let s_101_5: i64 = (s_101_4 as i64);
        // C s_101_6: const #() : ()
        let s_101_6: () = ();
        // S s_101_7: call AArch32_GetNumEventCountersAccessible(s_101_6)
        let s_101_7: i128 = AArch32_GetNumEventCountersAccessible(
            state,
            tracer,
            s_101_6,
        );
        // S s_101_8: cast zx s_101_5 -> i
        let s_101_8: i128 = (i128::try_from(s_101_5).unwrap());
        // S s_101_9: cmp-ge s_101_8 s_101_7
        let s_101_9: bool = ((s_101_8) >= (s_101_7));
        // D s_101_10: write-var gs#117258 <= s_101_9
        fn_state.gs_117258 = s_101_9;
        // N s_101_11: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #() : ()
        let s_102_0: () = ();
        // S s_102_1: call PMSELR_read(s_102_0)
        let s_102_1: ProductType700c18a878c5601b = PMSELR_read(state, tracer, s_102_0);
        // S s_102_2: call _get_PMSELR_Type_SEL(s_102_1)
        let s_102_2: u8 = u_get_PMSELR_Type_SEL(state, tracer, s_102_1);
        // S s_102_3: cast zx s_102_2 -> bv
        let s_102_3: Bits = Bits::new(s_102_2 as u128, 5u16);
        // S s_102_4: cast zx s_102_3 -> i
        let s_102_4: i128 = (s_102_3.value() as i128);
        // S s_102_5: cast reint s_102_4 -> i64
        let s_102_5: i64 = (s_102_4 as i64);
        // C s_102_6: const #31s : i
        let s_102_6: i128 = 31;
        // S s_102_7: cast zx s_102_5 -> i
        let s_102_7: i128 = (i128::try_from(s_102_5).unwrap());
        // S s_102_8: call neq_int(s_102_7, s_102_6)
        let s_102_8: bool = neq_int(state, tracer, s_102_7, s_102_6);
        // D s_102_9: write-var gs#117257 <= s_102_8
        fn_state.gs_117257 = s_102_8;
        // N s_102_10: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #3u : u8
        let s_103_0: u8 = 3;
        // C s_103_1: cast zx s_103_0 -> bv
        let s_103_1: Bits = Bits::new(s_103_0 as u128, 8u16);
        // C s_103_2: cast zx s_103_1 -> i
        let s_103_2: i128 = (s_103_1.value() as i128);
        // C s_103_3: cast reint s_103_2 -> i64
        let s_103_3: i64 = (s_103_2 as i64);
        // C s_103_4: cast zx s_103_3 -> i
        let s_103_4: i128 = (i128::try_from(s_103_3).unwrap());
        // S s_103_5: call AArch32_TakeHypTrapException(s_103_4)
        let s_103_5: () = AArch32_TakeHypTrapException(state, tracer, s_103_4);
        // N s_103_6: return
        return;
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_104_0: read-var __HDCR_TPM:u8
        let s_104_0: bool = fn_state.u__HDCR_TPM;
        // D s_104_1: cast zx s_104_0 -> bv
        let s_104_1: Bits = Bits::new(s_104_0 as u128, 1u16);
        // C s_104_2: const #1u : u8
        let s_104_2: bool = true;
        // C s_104_3: cast zx s_104_2 -> bv
        let s_104_3: Bits = Bits::new(s_104_2 as u128, 1u16);
        // D s_104_4: cmp-eq s_104_1 s_104_3
        let s_104_4: bool = ((s_104_1) == (s_104_3));
        // D s_104_5: write-var gs#117255 <= s_104_4
        fn_state.gs_117255 = s_104_4;
        // N s_104_6: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #432u : u32
        let s_105_0: u32 = 432;
        // D s_105_1: read-reg s_105_0:u8
        let s_105_1: u8 = {
            let value = state.read_register::<u8>(s_105_0 as isize);
            tracer.read_register(s_105_0 as isize, value);
            value
        };
        // D s_105_2: call ELUsingAArch32(s_105_1)
        let s_105_2: bool = ELUsingAArch32(state, tracer, s_105_1);
        // D s_105_3: write-var gs#117254 <= s_105_2
        fn_state.gs_117254 = s_105_2;
        // N s_105_4: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #3u : u8
        let s_106_0: u8 = 3;
        // C s_106_1: cast zx s_106_0 -> bv
        let s_106_1: Bits = Bits::new(s_106_0 as u128, 8u16);
        // C s_106_2: cast zx s_106_1 -> i
        let s_106_2: i128 = (s_106_1.value() as i128);
        // C s_106_3: cast reint s_106_2 -> i64
        let s_106_3: i64 = (s_106_2 as i64);
        // C s_106_4: cast zx s_106_3 -> i
        let s_106_4: i128 = (i128::try_from(s_106_3).unwrap());
        // C s_106_5: const #432u : u32
        let s_106_5: u32 = 432;
        // D s_106_6: read-reg s_106_5:u8
        let s_106_6: u8 = {
            let value = state.read_register::<u8>(s_106_5 as isize);
            tracer.read_register(s_106_5 as isize, value);
            value
        };
        // D s_106_7: call AArch64_AArch32SystemAccessTrap(s_106_6, s_106_4)
        let s_106_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_106_6,
            s_106_4,
        );
        // N s_106_8: return
        return;
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_107_0: read-var __MDCR_EL2_TPM:u8
        let s_107_0: bool = fn_state.u__MDCR_EL2_TPM;
        // D s_107_1: cast zx s_107_0 -> bv
        let s_107_1: Bits = Bits::new(s_107_0 as u128, 1u16);
        // C s_107_2: const #1u : u8
        let s_107_2: bool = true;
        // C s_107_3: cast zx s_107_2 -> bv
        let s_107_3: Bits = Bits::new(s_107_2 as u128, 1u16);
        // D s_107_4: cmp-eq s_107_1 s_107_3
        let s_107_4: bool = ((s_107_1) == (s_107_3));
        // D s_107_5: write-var gs#117253 <= s_107_4
        fn_state.gs_117253 = s_107_4;
        // N s_107_6: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #432u : u32
        let s_108_0: u32 = 432;
        // D s_108_1: read-reg s_108_0:u8
        let s_108_1: u8 = {
            let value = state.read_register::<u8>(s_108_0 as isize);
            tracer.read_register(s_108_0 as isize, value);
            value
        };
        // D s_108_2: call ELUsingAArch32(s_108_1)
        let s_108_2: bool = ELUsingAArch32(state, tracer, s_108_1);
        // D s_108_3: not s_108_2
        let s_108_3: bool = !s_108_2;
        // D s_108_4: write-var gs#117252 <= s_108_3
        fn_state.gs_117252 = s_108_3;
        // N s_108_5: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #3u : u8
        let s_109_0: u8 = 3;
        // C s_109_1: cast zx s_109_0 -> bv
        let s_109_1: Bits = Bits::new(s_109_0 as u128, 8u16);
        // C s_109_2: cast zx s_109_1 -> i
        let s_109_2: i128 = (s_109_1.value() as i128);
        // C s_109_3: cast reint s_109_2 -> i64
        let s_109_3: i64 = (s_109_2 as i64);
        // C s_109_4: cast zx s_109_3 -> i
        let s_109_4: i128 = (i128::try_from(s_109_3).unwrap());
        // S s_109_5: call AArch32_TakeHypTrapException(s_109_4)
        let s_109_5: () = AArch32_TakeHypTrapException(state, tracer, s_109_4);
        // N s_109_6: return
        return;
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_110_0: read-var __HSTR_T9:u8
        let s_110_0: bool = fn_state.u__HSTR_T9;
        // D s_110_1: cast zx s_110_0 -> bv
        let s_110_1: Bits = Bits::new(s_110_0 as u128, 1u16);
        // C s_110_2: const #1u : u8
        let s_110_2: bool = true;
        // C s_110_3: cast zx s_110_2 -> bv
        let s_110_3: Bits = Bits::new(s_110_2 as u128, 1u16);
        // D s_110_4: cmp-eq s_110_1 s_110_3
        let s_110_4: bool = ((s_110_1) == (s_110_3));
        // D s_110_5: write-var gs#117251 <= s_110_4
        fn_state.gs_117251 = s_110_4;
        // N s_110_6: jump b64
        return block_64(state, tracer, fn_state);
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
        // D s_111_2: call ELUsingAArch32(s_111_1)
        let s_111_2: bool = ELUsingAArch32(state, tracer, s_111_1);
        // D s_111_3: write-var gs#117250 <= s_111_2
        fn_state.gs_117250 = s_111_2;
        // N s_111_4: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_112_0: const #3u : u8
        let s_112_0: u8 = 3;
        // C s_112_1: cast zx s_112_0 -> bv
        let s_112_1: Bits = Bits::new(s_112_0 as u128, 8u16);
        // C s_112_2: cast zx s_112_1 -> i
        let s_112_2: i128 = (s_112_1.value() as i128);
        // C s_112_3: cast reint s_112_2 -> i64
        let s_112_3: i64 = (s_112_2 as i64);
        // C s_112_4: cast zx s_112_3 -> i
        let s_112_4: i128 = (i128::try_from(s_112_3).unwrap());
        // C s_112_5: const #432u : u32
        let s_112_5: u32 = 432;
        // D s_112_6: read-reg s_112_5:u8
        let s_112_6: u8 = {
            let value = state.read_register::<u8>(s_112_5 as isize);
            tracer.read_register(s_112_5 as isize, value);
            value
        };
        // D s_112_7: call AArch64_AArch32SystemAccessTrap(s_112_6, s_112_4)
        let s_112_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_112_6,
            s_112_4,
        );
        // N s_112_8: return
        return;
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var __HSTR_EL2_T9:u8
        let s_113_0: bool = fn_state.u__HSTR_EL2_T9;
        // D s_113_1: cast zx s_113_0 -> bv
        let s_113_1: Bits = Bits::new(s_113_0 as u128, 1u16);
        // C s_113_2: const #1u : u8
        let s_113_2: bool = true;
        // C s_113_3: cast zx s_113_2 -> bv
        let s_113_3: Bits = Bits::new(s_113_2 as u128, 1u16);
        // D s_113_4: cmp-eq s_113_1 s_113_3
        let s_113_4: bool = ((s_113_1) == (s_113_3));
        // D s_113_5: write-var gs#117249 <= s_113_4
        fn_state.gs_117249 = s_113_4;
        // N s_113_6: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #432u : u32
        let s_114_0: u32 = 432;
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
        // D s_114_4: write-var gs#117248 <= s_114_3
        fn_state.gs_117248 = s_114_3;
        // N s_114_5: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_115_0: panic
        panic!("{:?}", ());
        // N s_115_1: return
        return;
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_116_0: read-var __MDCR_EL3_TPM:u8
        let s_116_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_116_1: cast zx s_116_0 -> bv
        let s_116_1: Bits = Bits::new(s_116_0 as u128, 1u16);
        // C s_116_2: const #1u : u8
        let s_116_2: bool = true;
        // C s_116_3: cast zx s_116_2 -> bv
        let s_116_3: Bits = Bits::new(s_116_2 as u128, 1u16);
        // D s_116_4: cmp-eq s_116_1 s_116_3
        let s_116_4: bool = ((s_116_1) == (s_116_3));
        // D s_116_5: write-var gs#117247 <= s_116_4
        fn_state.gs_117247 = s_116_4;
        // N s_116_6: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #424u : u32
        let s_117_0: u32 = 424;
        // D s_117_1: read-reg s_117_0:u8
        let s_117_1: u8 = {
            let value = state.read_register::<u8>(s_117_0 as isize);
            tracer.read_register(s_117_0 as isize, value);
            value
        };
        // D s_117_2: call ELUsingAArch32(s_117_1)
        let s_117_2: bool = ELUsingAArch32(state, tracer, s_117_1);
        // D s_117_3: not s_117_2
        let s_117_3: bool = !s_117_2;
        // D s_117_4: write-var gs#117246 <= s_117_3
        fn_state.gs_117246 = s_117_3;
        // N s_117_5: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_118_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_118_1: call __IMPDEF_boolean(s_118_0)
        let s_118_1: bool = u__IMPDEF_boolean(state, tracer, s_118_0);
        // D s_118_2: write-var gs#117245 <= s_118_1
        fn_state.gs_117245 = s_118_1;
        // N s_118_3: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #() : ()
        let s_119_0: () = ();
        // S s_119_1: call EDSCR_read(s_119_0)
        let s_119_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_119_0);
        // S s_119_2: call _get_EDSCR_Type_SDD(s_119_1)
        let s_119_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_119_1);
        // S s_119_3: cast zx s_119_2 -> bv
        let s_119_3: Bits = Bits::new(s_119_2 as u128, 1u16);
        // C s_119_4: const #1u : u8
        let s_119_4: bool = true;
        // C s_119_5: cast zx s_119_4 -> bv
        let s_119_5: Bits = Bits::new(s_119_4 as u128, 1u16);
        // S s_119_6: cmp-eq s_119_3 s_119_5
        let s_119_6: bool = ((s_119_3) == (s_119_5));
        // D s_119_7: write-var gs#117244 <= s_119_6
        fn_state.gs_117244 = s_119_6;
        // N s_119_8: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_120_0: const #424u : u32
        let s_120_0: u32 = 424;
        // D s_120_1: read-reg s_120_0:u8
        let s_120_1: u8 = {
            let value = state.read_register::<u8>(s_120_0 as isize);
            tracer.read_register(s_120_0 as isize, value);
            value
        };
        // C s_120_2: const #2u : u8
        let s_120_2: u8 = 2;
        // D s_120_3: cmp-lt s_120_1 s_120_2
        let s_120_3: bool = ((s_120_1) < (s_120_2));
        // D s_120_4: write-var gs#117243 <= s_120_3
        fn_state.gs_117243 = s_120_3;
        // N s_120_5: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #() : ()
        let s_121_0: () = ();
        // S s_121_1: call Halted(s_121_0)
        let s_121_1: bool = Halted(state, tracer, s_121_0);
        // N s_121_2: branch s_121_1 b263 b122
        if s_121_1 {
            return block_263(state, tracer, fn_state);
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
        // D s_122_1: write-var gs#117263 <= s_122_0
        fn_state.gs_117263 = s_122_0;
        // N s_122_2: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_123_0: read-var gs#117263:u8
        let s_123_0: bool = fn_state.gs_117263;
        // N s_123_1: branch s_123_0 b262 b124
        if s_123_0 {
            return block_262(state, tracer, fn_state);
        } else {
            return block_124(state, tracer, fn_state);
        };
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #0u : u8
        let s_124_0: bool = false;
        // D s_124_1: write-var gs#117264 <= s_124_0
        fn_state.gs_117264 = s_124_0;
        // N s_124_2: jump b125
        return block_125(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_125_0: read-var gs#117264:u8
        let s_125_0: bool = fn_state.gs_117264;
        // N s_125_1: branch s_125_0 b261 b126
        if s_125_0 {
            return block_261(state, tracer, fn_state);
        } else {
            return block_126(state, tracer, fn_state);
        };
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_126_0: const #0u : u8
        let s_126_0: bool = false;
        // D s_126_1: write-var gs#117265 <= s_126_0
        fn_state.gs_117265 = s_126_0;
        // N s_126_2: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_127_0: read-var gs#117265:u8
        let s_127_0: bool = fn_state.gs_117265;
        // N s_127_1: branch s_127_0 b260 b128
        if s_127_0 {
            return block_260(state, tracer, fn_state);
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
        // D s_128_1: write-var gs#117266 <= s_128_0
        fn_state.gs_117266 = s_128_0;
        // N s_128_2: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_129_0: read-var gs#117266:u8
        let s_129_0: bool = fn_state.gs_117266;
        // N s_129_1: branch s_129_0 b259 b130
        if s_129_0 {
            return block_259(state, tracer, fn_state);
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
        // D s_130_1: write-var gs#117267 <= s_130_0
        fn_state.gs_117267 = s_130_0;
        // N s_130_2: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_131_0: read-var gs#117267:u8
        let s_131_0: bool = fn_state.gs_117267;
        // N s_131_1: branch s_131_0 b258 b132
        if s_131_0 {
            return block_258(state, tracer, fn_state);
        } else {
            return block_132(state, tracer, fn_state);
        };
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_132_0: const #440u : u32
        let s_132_0: u32 = 440;
        // D s_132_1: read-reg s_132_0:u8
        let s_132_1: u8 = {
            let value = state.read_register::<u8>(s_132_0 as isize);
            tracer.read_register(s_132_0 as isize, value);
            value
        };
        // D s_132_2: call ELUsingAArch32(s_132_1)
        let s_132_2: bool = ELUsingAArch32(state, tracer, s_132_1);
        // D s_132_3: not s_132_2
        let s_132_3: bool = !s_132_2;
        // N s_132_4: branch s_132_3 b248 b133
        if s_132_3 {
            return block_248(state, tracer, fn_state);
        } else {
            return block_133(state, tracer, fn_state);
        };
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_133_0: const #0u : u8
        let s_133_0: bool = false;
        // D s_133_1: write-var gs#117271 <= s_133_0
        fn_state.gs_117271 = s_133_0;
        // N s_133_2: jump b134
        return block_134(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_134_0: read-var gs#117271:u8
        let s_134_0: bool = fn_state.gs_117271;
        // N s_134_1: branch s_134_0 b239 b135
        if s_134_0 {
            return block_239(state, tracer, fn_state);
        } else {
            return block_135(state, tracer, fn_state);
        };
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_135_0: const #440u : u32
        let s_135_0: u32 = 440;
        // D s_135_1: read-reg s_135_0:u8
        let s_135_1: u8 = {
            let value = state.read_register::<u8>(s_135_0 as isize);
            tracer.read_register(s_135_0 as isize, value);
            value
        };
        // D s_135_2: call ELUsingAArch32(s_135_1)
        let s_135_2: bool = ELUsingAArch32(state, tracer, s_135_1);
        // N s_135_3: branch s_135_2 b238 b136
        if s_135_2 {
            return block_238(state, tracer, fn_state);
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
        // D s_136_1: write-var gs#117272 <= s_136_0
        fn_state.gs_117272 = s_136_0;
        // N s_136_2: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_137_0: read-var gs#117272:u8
        let s_137_0: bool = fn_state.gs_117272;
        // N s_137_1: branch s_137_0 b221 b138
        if s_137_0 {
            return block_221(state, tracer, fn_state);
        } else {
            return block_138(state, tracer, fn_state);
        };
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_138_0: const #() : ()
        let s_138_0: () = ();
        // S s_138_1: call EL2Enabled(s_138_0)
        let s_138_1: bool = EL2Enabled(state, tracer, s_138_0);
        // N s_138_2: branch s_138_1 b220 b139
        if s_138_1 {
            return block_220(state, tracer, fn_state);
        } else {
            return block_139(state, tracer, fn_state);
        };
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_139_0: const #0u : u8
        let s_139_0: bool = false;
        // D s_139_1: write-var gs#117273 <= s_139_0
        fn_state.gs_117273 = s_139_0;
        // N s_139_2: jump b140
        return block_140(state, tracer, fn_state);
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_140_0: read-var gs#117273:u8
        let s_140_0: bool = fn_state.gs_117273;
        // N s_140_1: branch s_140_0 b219 b141
        if s_140_0 {
            return block_219(state, tracer, fn_state);
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
        // D s_141_1: write-var gs#117274 <= s_141_0
        fn_state.gs_117274 = s_141_0;
        // N s_141_2: jump b142
        return block_142(state, tracer, fn_state);
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_142_0: read-var gs#117274:u8
        let s_142_0: bool = fn_state.gs_117274;
        // N s_142_1: branch s_142_0 b218 b143
        if s_142_0 {
            return block_218(state, tracer, fn_state);
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
        // D s_143_1: write-var gs#117275 <= s_143_0
        fn_state.gs_117275 = s_143_0;
        // N s_143_2: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_144_0: read-var gs#117275:u8
        let s_144_0: bool = fn_state.gs_117275;
        // N s_144_1: branch s_144_0 b217 b145
        if s_144_0 {
            return block_217(state, tracer, fn_state);
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
        // N s_145_2: branch s_145_1 b216 b146
        if s_145_1 {
            return block_216(state, tracer, fn_state);
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
        // D s_146_1: write-var gs#117276 <= s_146_0
        fn_state.gs_117276 = s_146_0;
        // N s_146_2: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_147_0: read-var gs#117276:u8
        let s_147_0: bool = fn_state.gs_117276;
        // N s_147_1: branch s_147_0 b215 b148
        if s_147_0 {
            return block_215(state, tracer, fn_state);
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
        // D s_148_1: write-var gs#117277 <= s_148_0
        fn_state.gs_117277 = s_148_0;
        // N s_148_2: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_149_0: read-var gs#117277:u8
        let s_149_0: bool = fn_state.gs_117277;
        // N s_149_1: branch s_149_0 b214 b150
        if s_149_0 {
            return block_214(state, tracer, fn_state);
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
        // N s_150_2: branch s_150_1 b213 b151
        if s_150_1 {
            return block_213(state, tracer, fn_state);
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
        // D s_151_1: write-var gs#117278 <= s_151_0
        fn_state.gs_117278 = s_151_0;
        // N s_151_2: jump b152
        return block_152(state, tracer, fn_state);
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_152_0: read-var gs#117278:u8
        let s_152_0: bool = fn_state.gs_117278;
        // N s_152_1: branch s_152_0 b212 b153
        if s_152_0 {
            return block_212(state, tracer, fn_state);
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
        // D s_153_1: write-var gs#117279 <= s_153_0
        fn_state.gs_117279 = s_153_0;
        // N s_153_2: jump b154
        return block_154(state, tracer, fn_state);
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_154_0: read-var gs#117279:u8
        let s_154_0: bool = fn_state.gs_117279;
        // N s_154_1: branch s_154_0 b211 b155
        if s_154_0 {
            return block_211(state, tracer, fn_state);
        } else {
            return block_155(state, tracer, fn_state);
        };
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_155_0: const #0u : u8
        let s_155_0: bool = false;
        // D s_155_1: write-var gs#117280 <= s_155_0
        fn_state.gs_117280 = s_155_0;
        // N s_155_2: jump b156
        return block_156(state, tracer, fn_state);
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_156_0: read-var gs#117280:u8
        let s_156_0: bool = fn_state.gs_117280;
        // N s_156_1: branch s_156_0 b207 b157
        if s_156_0 {
            return block_207(state, tracer, fn_state);
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
        // D s_157_1: write-var gs#117282 <= s_157_0
        fn_state.gs_117282 = s_157_0;
        // N s_157_2: jump b158
        return block_158(state, tracer, fn_state);
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_158_0: read-var gs#117282:u8
        let s_158_0: bool = fn_state.gs_117282;
        // N s_158_1: branch s_158_0 b206 b159
        if s_158_0 {
            return block_206(state, tracer, fn_state);
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
        // D s_159_1: write-var gs#117283 <= s_159_0
        fn_state.gs_117283 = s_159_0;
        // N s_159_2: jump b160
        return block_160(state, tracer, fn_state);
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_160_0: read-var gs#117283:u8
        let s_160_0: bool = fn_state.gs_117283;
        // N s_160_1: branch s_160_0 b205 b161
        if s_160_0 {
            return block_205(state, tracer, fn_state);
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
        // N s_161_2: branch s_161_1 b204 b162
        if s_161_1 {
            return block_204(state, tracer, fn_state);
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
        // D s_162_1: write-var gs#117284 <= s_162_0
        fn_state.gs_117284 = s_162_0;
        // N s_162_2: jump b163
        return block_163(state, tracer, fn_state);
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_163_0: read-var gs#117284:u8
        let s_163_0: bool = fn_state.gs_117284;
        // N s_163_1: branch s_163_0 b203 b164
        if s_163_0 {
            return block_203(state, tracer, fn_state);
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
        // D s_164_1: write-var gs#117285 <= s_164_0
        fn_state.gs_117285 = s_164_0;
        // N s_164_2: jump b165
        return block_165(state, tracer, fn_state);
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_165_0: read-var gs#117285:u8
        let s_165_0: bool = fn_state.gs_117285;
        // N s_165_1: branch s_165_0 b202 b166
        if s_165_0 {
            return block_202(state, tracer, fn_state);
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
        // N s_166_2: branch s_166_1 b201 b167
        if s_166_1 {
            return block_201(state, tracer, fn_state);
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
        // D s_167_1: write-var gs#117286 <= s_167_0
        fn_state.gs_117286 = s_167_0;
        // N s_167_2: jump b168
        return block_168(state, tracer, fn_state);
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_168_0: read-var gs#117286:u8
        let s_168_0: bool = fn_state.gs_117286;
        // N s_168_1: branch s_168_0 b200 b169
        if s_168_0 {
            return block_200(state, tracer, fn_state);
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
        // D s_169_1: write-var gs#117287 <= s_169_0
        fn_state.gs_117287 = s_169_0;
        // N s_169_2: jump b170
        return block_170(state, tracer, fn_state);
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_170_0: read-var gs#117287:u8
        let s_170_0: bool = fn_state.gs_117287;
        // N s_170_1: branch s_170_0 b199 b171
        if s_170_0 {
            return block_199(state, tracer, fn_state);
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
        // N s_171_2: branch s_171_1 b198 b172
        if s_171_1 {
            return block_198(state, tracer, fn_state);
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
        // D s_172_1: write-var gs#117289 <= s_172_0
        fn_state.gs_117289 = s_172_0;
        // N s_172_2: jump b173
        return block_173(state, tracer, fn_state);
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_173_0: read-var gs#117289:u8
        let s_173_0: bool = fn_state.gs_117289;
        // N s_173_1: branch s_173_0 b197 b174
        if s_173_0 {
            return block_197(state, tracer, fn_state);
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
        // D s_174_1: write-var gs#117290 <= s_174_0
        fn_state.gs_117290 = s_174_0;
        // N s_174_2: jump b175
        return block_175(state, tracer, fn_state);
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_175_0: read-var gs#117290:u8
        let s_175_0: bool = fn_state.gs_117290;
        // N s_175_1: branch s_175_0 b192 b176
        if s_175_0 {
            return block_192(state, tracer, fn_state);
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
        // N s_176_4: branch s_176_3 b191 b177
        if s_176_3 {
            return block_191(state, tracer, fn_state);
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
        // D s_177_1: write-var gs#117291 <= s_177_0
        fn_state.gs_117291 = s_177_0;
        // N s_177_2: jump b178
        return block_178(state, tracer, fn_state);
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_178_0: read-var gs#117291:u8
        let s_178_0: bool = fn_state.gs_117291;
        // N s_178_1: branch s_178_0 b190 b179
        if s_178_0 {
            return block_190(state, tracer, fn_state);
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
        // D s_179_1: write-var gs#117292 <= s_179_0
        fn_state.gs_117292 = s_179_0;
        // N s_179_2: jump b180
        return block_180(state, tracer, fn_state);
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_180_0: read-var gs#117292:u8
        let s_180_0: bool = fn_state.gs_117292;
        // N s_180_1: branch s_180_0 b184 b181
        if s_180_0 {
            return block_184(state, tracer, fn_state);
        } else {
            return block_181(state, tracer, fn_state);
        };
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_181_0: const #() : ()
        let s_181_0: () = ();
        // S s_181_1: call PMSELR_read(s_181_0)
        let s_181_1: ProductType700c18a878c5601b = PMSELR_read(state, tracer, s_181_0);
        // S s_181_2: call _get_PMSELR_Type_SEL(s_181_1)
        let s_181_2: u8 = u_get_PMSELR_Type_SEL(state, tracer, s_181_1);
        // S s_181_3: cast zx s_181_2 -> bv
        let s_181_3: Bits = Bits::new(s_181_2 as u128, 5u16);
        // S s_181_4: cast zx s_181_3 -> i
        let s_181_4: i128 = (s_181_3.value() as i128);
        // S s_181_5: cast reint s_181_4 -> i64
        let s_181_5: i64 = (s_181_4 as i64);
        // C s_181_6: const #31s : i
        let s_181_6: i128 = 31;
        // S s_181_7: cast zx s_181_5 -> i
        let s_181_7: i128 = (i128::try_from(s_181_5).unwrap());
        // S s_181_8: cmp-eq s_181_7 s_181_6
        let s_181_8: bool = ((s_181_7) == (s_181_6));
        // N s_181_9: branch s_181_8 b183 b182
        if s_181_8 {
            return block_183(state, tracer, fn_state);
        } else {
            return block_182(state, tracer, fn_state);
        };
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_182_0: const #() : ()
        let s_182_0: () = ();
        // S s_182_1: call __get_selected_PMEVTYPER(s_182_0)
        let s_182_1: ProductType700c18a878c5601b = u__get_selected_PMEVTYPER(
            state,
            tracer,
            s_182_0,
        );
        // S s_182_2: call __get_PMEVTYPER(s_182_1)
        let s_182_2: ProductType700c18a878c5601b = u__get_PMEVTYPER(
            state,
            tracer,
            s_182_1,
        );
        // D s_182_3: write-var ga#196738 <= s_182_2
        fn_state.ga_196738 = s_182_2;
        // D s_182_4: read-var ga#196738.0:struct
        let s_182_4: u32 = fn_state.ga_196738._0;
        // D s_182_5: read-var t:i
        let s_182_5: i128 = fn_state.t;
        // D s_182_6: call R_set(s_182_5, s_182_4)
        let s_182_6: () = R_set(state, tracer, s_182_5, s_182_4);
        // N s_182_7: return
        return;
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_183_0: const #() : ()
        let s_183_0: () = ();
        // S s_183_1: call PMCCFILTR_read(s_183_0)
        let s_183_1: ProductType700c18a878c5601b = PMCCFILTR_read(
            state,
            tracer,
            s_183_0,
        );
        // S s_183_2: call __get_PMCCFILTR(s_183_1)
        let s_183_2: ProductType700c18a878c5601b = u__get_PMCCFILTR(
            state,
            tracer,
            s_183_1,
        );
        // D s_183_3: write-var ga#196735 <= s_183_2
        fn_state.ga_196735 = s_183_2;
        // D s_183_4: read-var ga#196735.0:struct
        let s_183_4: u32 = fn_state.ga_196735._0;
        // D s_183_5: read-var t:i
        let s_183_5: i128 = fn_state.t;
        // D s_183_6: call R_set(s_183_5, s_183_4)
        let s_183_6: () = R_set(state, tracer, s_183_5, s_183_4);
        // N s_183_7: return
        return;
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_184_0: const #() : ()
        let s_184_0: () = ();
        // S s_184_1: call Halted(s_184_0)
        let s_184_1: bool = Halted(state, tracer, s_184_0);
        // N s_184_2: branch s_184_1 b189 b185
        if s_184_1 {
            return block_189(state, tracer, fn_state);
        } else {
            return block_185(state, tracer, fn_state);
        };
    }
    fn block_185<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_185_0: const #0u : u8
        let s_185_0: bool = false;
        // D s_185_1: write-var gs#117294 <= s_185_0
        fn_state.gs_117294 = s_185_0;
        // N s_185_2: jump b186
        return block_186(state, tracer, fn_state);
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_186_0: read-var gs#117294:u8
        let s_186_0: bool = fn_state.gs_117294;
        // N s_186_1: branch s_186_0 b188 b187
        if s_186_0 {
            return block_188(state, tracer, fn_state);
        } else {
            return block_187(state, tracer, fn_state);
        };
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_187_0: const #3u : u8
        let s_187_0: u8 = 3;
        // C s_187_1: cast zx s_187_0 -> bv
        let s_187_1: Bits = Bits::new(s_187_0 as u128, 8u16);
        // C s_187_2: cast zx s_187_1 -> i
        let s_187_2: i128 = (s_187_1.value() as i128);
        // C s_187_3: cast reint s_187_2 -> i64
        let s_187_3: i64 = (s_187_2 as i64);
        // C s_187_4: cast zx s_187_3 -> i
        let s_187_4: i128 = (i128::try_from(s_187_3).unwrap());
        // C s_187_5: const #424u : u32
        let s_187_5: u32 = 424;
        // D s_187_6: read-reg s_187_5:u8
        let s_187_6: u8 = {
            let value = state.read_register::<u8>(s_187_5 as isize);
            tracer.read_register(s_187_5 as isize, value);
            value
        };
        // D s_187_7: call AArch64_AArch32SystemAccessTrap(s_187_6, s_187_4)
        let s_187_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_187_6,
            s_187_4,
        );
        // N s_187_8: return
        return;
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_188_0: panic
        panic!("{:?}", ());
        // N s_188_1: return
        return;
    }
    fn block_189<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_189_0: const #() : ()
        let s_189_0: () = ();
        // S s_189_1: call EDSCR_read(s_189_0)
        let s_189_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_189_0);
        // S s_189_2: call _get_EDSCR_Type_SDD(s_189_1)
        let s_189_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_189_1);
        // S s_189_3: cast zx s_189_2 -> bv
        let s_189_3: Bits = Bits::new(s_189_2 as u128, 1u16);
        // C s_189_4: const #1u : u8
        let s_189_4: bool = true;
        // C s_189_5: cast zx s_189_4 -> bv
        let s_189_5: Bits = Bits::new(s_189_4 as u128, 1u16);
        // S s_189_6: cmp-eq s_189_3 s_189_5
        let s_189_6: bool = ((s_189_3) == (s_189_5));
        // D s_189_7: write-var gs#117294 <= s_189_6
        fn_state.gs_117294 = s_189_6;
        // N s_189_8: jump b186
        return block_186(state, tracer, fn_state);
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_190_0: read-var __MDCR_EL3_TPM:u8
        let s_190_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_190_1: cast zx s_190_0 -> bv
        let s_190_1: Bits = Bits::new(s_190_0 as u128, 1u16);
        // C s_190_2: const #1u : u8
        let s_190_2: bool = true;
        // C s_190_3: cast zx s_190_2 -> bv
        let s_190_3: Bits = Bits::new(s_190_2 as u128, 1u16);
        // D s_190_4: cmp-eq s_190_1 s_190_3
        let s_190_4: bool = ((s_190_1) == (s_190_3));
        // D s_190_5: write-var gs#117292 <= s_190_4
        fn_state.gs_117292 = s_190_4;
        // N s_190_6: jump b180
        return block_180(state, tracer, fn_state);
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
        // D s_191_2: call ELUsingAArch32(s_191_1)
        let s_191_2: bool = ELUsingAArch32(state, tracer, s_191_1);
        // D s_191_3: not s_191_2
        let s_191_3: bool = !s_191_2;
        // D s_191_4: write-var gs#117291 <= s_191_3
        fn_state.gs_117291 = s_191_3;
        // N s_191_5: jump b178
        return block_178(state, tracer, fn_state);
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_192_0: const #146u : u32
        let s_192_0: u32 = 146;
        // S s_192_1: call IsFeatureImplemented(s_192_0)
        let s_192_1: bool = IsFeatureImplemented(state, tracer, s_192_0);
        // S s_192_2: not s_192_1
        let s_192_2: bool = !s_192_1;
        // N s_192_3: branch s_192_2 b196 b193
        if s_192_2 {
            return block_196(state, tracer, fn_state);
        } else {
            return block_193(state, tracer, fn_state);
        };
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_193_0: const #440u : u32
        let s_193_0: u32 = 440;
        // D s_193_1: read-reg s_193_0:u8
        let s_193_1: u8 = {
            let value = state.read_register::<u8>(s_193_0 as isize);
            tracer.read_register(s_193_0 as isize, value);
            value
        };
        // D s_193_2: call ELUsingAArch32(s_193_1)
        let s_193_2: bool = ELUsingAArch32(state, tracer, s_193_1);
        // N s_193_3: branch s_193_2 b195 b194
        if s_193_2 {
            return block_195(state, tracer, fn_state);
        } else {
            return block_194(state, tracer, fn_state);
        };
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_194_0: const #3u : u8
        let s_194_0: u8 = 3;
        // C s_194_1: cast zx s_194_0 -> bv
        let s_194_1: Bits = Bits::new(s_194_0 as u128, 8u16);
        // C s_194_2: cast zx s_194_1 -> i
        let s_194_2: i128 = (s_194_1.value() as i128);
        // C s_194_3: cast reint s_194_2 -> i64
        let s_194_3: i64 = (s_194_2 as i64);
        // C s_194_4: cast zx s_194_3 -> i
        let s_194_4: i128 = (i128::try_from(s_194_3).unwrap());
        // C s_194_5: const #432u : u32
        let s_194_5: u32 = 432;
        // D s_194_6: read-reg s_194_5:u8
        let s_194_6: u8 = {
            let value = state.read_register::<u8>(s_194_5 as isize);
            tracer.read_register(s_194_5 as isize, value);
            value
        };
        // D s_194_7: call AArch64_AArch32SystemAccessTrap(s_194_6, s_194_4)
        let s_194_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_194_6,
            s_194_4,
        );
        // N s_194_8: return
        return;
    }
    fn block_195<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_195_0: const #3u : u8
        let s_195_0: u8 = 3;
        // C s_195_1: cast zx s_195_0 -> bv
        let s_195_1: Bits = Bits::new(s_195_0 as u128, 8u16);
        // C s_195_2: cast zx s_195_1 -> i
        let s_195_2: i128 = (s_195_1.value() as i128);
        // C s_195_3: cast reint s_195_2 -> i64
        let s_195_3: i64 = (s_195_2 as i64);
        // C s_195_4: cast zx s_195_3 -> i
        let s_195_4: i128 = (i128::try_from(s_195_3).unwrap());
        // S s_195_5: call AArch32_TakeHypTrapException(s_195_4)
        let s_195_5: () = AArch32_TakeHypTrapException(state, tracer, s_195_4);
        // N s_195_6: return
        return;
    }
    fn block_196<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_196_0: const #72u : u32
        let s_196_0: u32 = 72;
        // S s_196_1: call ConstrainUnpredictableProcedure(s_196_0)
        let s_196_1: () = ConstrainUnpredictableProcedure(state, tracer, s_196_0);
        // N s_196_2: return
        return;
    }
    fn block_197<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_197_0: const #() : ()
        let s_197_0: () = ();
        // S s_197_1: call PMSELR_read(s_197_0)
        let s_197_1: ProductType700c18a878c5601b = PMSELR_read(state, tracer, s_197_0);
        // S s_197_2: call _get_PMSELR_Type_SEL(s_197_1)
        let s_197_2: u8 = u_get_PMSELR_Type_SEL(state, tracer, s_197_1);
        // S s_197_3: cast zx s_197_2 -> bv
        let s_197_3: Bits = Bits::new(s_197_2 as u128, 5u16);
        // S s_197_4: cast zx s_197_3 -> i
        let s_197_4: i128 = (s_197_3.value() as i128);
        // S s_197_5: cast reint s_197_4 -> i64
        let s_197_5: i64 = (s_197_4 as i64);
        // C s_197_6: const #() : ()
        let s_197_6: () = ();
        // S s_197_7: call AArch32_GetNumEventCountersAccessible(s_197_6)
        let s_197_7: i128 = AArch32_GetNumEventCountersAccessible(
            state,
            tracer,
            s_197_6,
        );
        // S s_197_8: cast zx s_197_5 -> i
        let s_197_8: i128 = (i128::try_from(s_197_5).unwrap());
        // S s_197_9: cmp-ge s_197_8 s_197_7
        let s_197_9: bool = ((s_197_8) >= (s_197_7));
        // D s_197_10: write-var gs#117290 <= s_197_9
        fn_state.gs_117290 = s_197_9;
        // N s_197_11: jump b175
        return block_175(state, tracer, fn_state);
    }
    fn block_198<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_198_0: const #() : ()
        let s_198_0: () = ();
        // S s_198_1: call PMSELR_read(s_198_0)
        let s_198_1: ProductType700c18a878c5601b = PMSELR_read(state, tracer, s_198_0);
        // S s_198_2: call _get_PMSELR_Type_SEL(s_198_1)
        let s_198_2: u8 = u_get_PMSELR_Type_SEL(state, tracer, s_198_1);
        // S s_198_3: cast zx s_198_2 -> bv
        let s_198_3: Bits = Bits::new(s_198_2 as u128, 5u16);
        // S s_198_4: cast zx s_198_3 -> i
        let s_198_4: i128 = (s_198_3.value() as i128);
        // S s_198_5: cast reint s_198_4 -> i64
        let s_198_5: i64 = (s_198_4 as i64);
        // C s_198_6: const #31s : i
        let s_198_6: i128 = 31;
        // S s_198_7: cast zx s_198_5 -> i
        let s_198_7: i128 = (i128::try_from(s_198_5).unwrap());
        // S s_198_8: call neq_int(s_198_7, s_198_6)
        let s_198_8: bool = neq_int(state, tracer, s_198_7, s_198_6);
        // D s_198_9: write-var gs#117289 <= s_198_8
        fn_state.gs_117289 = s_198_8;
        // N s_198_10: jump b173
        return block_173(state, tracer, fn_state);
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
        // S s_199_5: call AArch32_TakeHypTrapException(s_199_4)
        let s_199_5: () = AArch32_TakeHypTrapException(state, tracer, s_199_4);
        // N s_199_6: return
        return;
    }
    fn block_200<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_200_0: read-var __HDCR_TPM:u8
        let s_200_0: bool = fn_state.u__HDCR_TPM;
        // D s_200_1: cast zx s_200_0 -> bv
        let s_200_1: Bits = Bits::new(s_200_0 as u128, 1u16);
        // C s_200_2: const #1u : u8
        let s_200_2: bool = true;
        // C s_200_3: cast zx s_200_2 -> bv
        let s_200_3: Bits = Bits::new(s_200_2 as u128, 1u16);
        // D s_200_4: cmp-eq s_200_1 s_200_3
        let s_200_4: bool = ((s_200_1) == (s_200_3));
        // D s_200_5: write-var gs#117287 <= s_200_4
        fn_state.gs_117287 = s_200_4;
        // N s_200_6: jump b170
        return block_170(state, tracer, fn_state);
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
        // D s_201_3: write-var gs#117286 <= s_201_2
        fn_state.gs_117286 = s_201_2;
        // N s_201_4: jump b168
        return block_168(state, tracer, fn_state);
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
        // D s_203_0: read-var __MDCR_EL2_TPM:u8
        let s_203_0: bool = fn_state.u__MDCR_EL2_TPM;
        // D s_203_1: cast zx s_203_0 -> bv
        let s_203_1: Bits = Bits::new(s_203_0 as u128, 1u16);
        // C s_203_2: const #1u : u8
        let s_203_2: bool = true;
        // C s_203_3: cast zx s_203_2 -> bv
        let s_203_3: Bits = Bits::new(s_203_2 as u128, 1u16);
        // D s_203_4: cmp-eq s_203_1 s_203_3
        let s_203_4: bool = ((s_203_1) == (s_203_3));
        // D s_203_5: write-var gs#117285 <= s_203_4
        fn_state.gs_117285 = s_203_4;
        // N s_203_6: jump b165
        return block_165(state, tracer, fn_state);
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
        // D s_204_4: write-var gs#117284 <= s_204_3
        fn_state.gs_117284 = s_204_3;
        // N s_204_5: jump b163
        return block_163(state, tracer, fn_state);
    }
    fn block_205<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_205_0: const #3u : u8
        let s_205_0: u8 = 3;
        // C s_205_1: cast zx s_205_0 -> bv
        let s_205_1: Bits = Bits::new(s_205_0 as u128, 8u16);
        // C s_205_2: cast zx s_205_1 -> i
        let s_205_2: i128 = (s_205_1.value() as i128);
        // C s_205_3: cast reint s_205_2 -> i64
        let s_205_3: i64 = (s_205_2 as i64);
        // C s_205_4: cast zx s_205_3 -> i
        let s_205_4: i128 = (i128::try_from(s_205_3).unwrap());
        // C s_205_5: const #432u : u32
        let s_205_5: u32 = 432;
        // D s_205_6: read-reg s_205_5:u8
        let s_205_6: u8 = {
            let value = state.read_register::<u8>(s_205_5 as isize);
            tracer.read_register(s_205_5 as isize, value);
            value
        };
        // D s_205_7: call AArch64_AArch32SystemAccessTrap(s_205_6, s_205_4)
        let s_205_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_205_6,
            s_205_4,
        );
        // N s_205_8: return
        return;
    }
    fn block_206<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_206_0: read-var __HDFGRTR_EL2_PMEVTYPERn_EL0:u8
        let s_206_0: bool = fn_state.u__HDFGRTR_EL2_PMEVTYPERn_EL0;
        // D s_206_1: cast zx s_206_0 -> bv
        let s_206_1: Bits = Bits::new(s_206_0 as u128, 1u16);
        // C s_206_2: const #1u : u8
        let s_206_2: bool = true;
        // C s_206_3: cast zx s_206_2 -> bv
        let s_206_3: Bits = Bits::new(s_206_2 as u128, 1u16);
        // D s_206_4: cmp-eq s_206_1 s_206_3
        let s_206_4: bool = ((s_206_1) == (s_206_3));
        // D s_206_5: write-var gs#117283 <= s_206_4
        fn_state.gs_117283 = s_206_4;
        // N s_206_6: jump b160
        return block_160(state, tracer, fn_state);
    }
    fn block_207<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_207_0: const #424u : u32
        let s_207_0: u32 = 424;
        // D s_207_1: read-reg s_207_0:u8
        let s_207_1: u8 = {
            let value = state.read_register::<u8>(s_207_0 as isize);
            tracer.read_register(s_207_0 as isize, value);
            value
        };
        // C s_207_2: const #2u : u8
        let s_207_2: u8 = 2;
        // D s_207_3: cmp-lt s_207_1 s_207_2
        let s_207_3: bool = ((s_207_1) < (s_207_2));
        // D s_207_4: not s_207_3
        let s_207_4: bool = !s_207_3;
        // N s_207_5: branch s_207_4 b210 b208
        if s_207_4 {
            return block_210(state, tracer, fn_state);
        } else {
            return block_208(state, tracer, fn_state);
        };
    }
    fn block_208<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_208_0: read-var __SCR_EL3_FGTEn:u8
        let s_208_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_208_1: cast zx s_208_0 -> bv
        let s_208_1: Bits = Bits::new(s_208_0 as u128, 1u16);
        // C s_208_2: const #1u : u8
        let s_208_2: bool = true;
        // C s_208_3: cast zx s_208_2 -> bv
        let s_208_3: Bits = Bits::new(s_208_2 as u128, 1u16);
        // D s_208_4: cmp-eq s_208_1 s_208_3
        let s_208_4: bool = ((s_208_1) == (s_208_3));
        // D s_208_5: write-var gs#117281 <= s_208_4
        fn_state.gs_117281 = s_208_4;
        // N s_208_6: jump b209
        return block_209(state, tracer, fn_state);
    }
    fn block_209<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_209_0: read-var gs#117281:u8
        let s_209_0: bool = fn_state.gs_117281;
        // D s_209_1: write-var gs#117282 <= s_209_0
        fn_state.gs_117282 = s_209_0;
        // N s_209_2: jump b158
        return block_158(state, tracer, fn_state);
    }
    fn block_210<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_210_0: const #1u : u8
        let s_210_0: bool = true;
        // D s_210_1: write-var gs#117281 <= s_210_0
        fn_state.gs_117281 = s_210_0;
        // N s_210_2: jump b209
        return block_209(state, tracer, fn_state);
    }
    fn block_211<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_211_0: const #146u : u32
        let s_211_0: u32 = 146;
        // S s_211_1: call IsFeatureImplemented(s_211_0)
        let s_211_1: bool = IsFeatureImplemented(state, tracer, s_211_0);
        // D s_211_2: write-var gs#117280 <= s_211_1
        fn_state.gs_117280 = s_211_1;
        // N s_211_3: jump b156
        return block_156(state, tracer, fn_state);
    }
    fn block_212<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_212_0: const #102552u : u32
        let s_212_0: u32 = 102552;
        // D s_212_1: read-reg s_212_0:struct
        let s_212_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_212_0 as isize);
            tracer.read_register(s_212_0 as isize, value);
            value
        };
        // D s_212_2: call _get_HCR_EL2_Type_E2H(s_212_1)
        let s_212_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_212_1);
        // C s_212_3: const #102552u : u32
        let s_212_3: u32 = 102552;
        // D s_212_4: read-reg s_212_3:struct
        let s_212_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_212_3 as isize);
            tracer.read_register(s_212_3 as isize, value);
            value
        };
        // D s_212_5: call _get_HCR_EL2_Type_TGE(s_212_4)
        let s_212_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_212_4);
        // D s_212_6: cast zx s_212_2 -> bv
        let s_212_6: Bits = Bits::new(s_212_2 as u128, 1u16);
        // D s_212_7: cast zx s_212_5 -> bv
        let s_212_7: Bits = Bits::new(s_212_5 as u128, 1u16);
        // D s_212_8: cast reint s_212_6 -> u128
        let s_212_8: u128 = (s_212_6.value() as u128);
        // D s_212_9: size-of s_212_6
        let s_212_9: u16 = s_212_6.length();
        // D s_212_10: cast reint s_212_7 -> u128
        let s_212_10: u128 = (s_212_7.value() as u128);
        // D s_212_11: size-of s_212_7
        let s_212_11: u16 = s_212_7.length();
        // D s_212_12: lsl s_212_8 s_212_11
        let s_212_12: u128 = s_212_8 << s_212_11;
        // D s_212_13: or s_212_12 s_212_10
        let s_212_13: u128 = ((s_212_12) | (s_212_10));
        // D s_212_14: add s_212_9 s_212_11
        let s_212_14: u16 = (s_212_9 + s_212_11);
        // D s_212_15: create-bits s_212_13 s_212_14
        let s_212_15: Bits = Bits::new(s_212_13, s_212_14);
        // D s_212_16: cast reint s_212_15 -> u8
        let s_212_16: u8 = (s_212_15.value() as u8);
        // D s_212_17: cast zx s_212_16 -> bv
        let s_212_17: Bits = Bits::new(s_212_16 as u128, 2u16);
        // C s_212_18: const #3u : u8
        let s_212_18: u8 = 3;
        // C s_212_19: cast zx s_212_18 -> bv
        let s_212_19: Bits = Bits::new(s_212_18 as u128, 2u16);
        // D s_212_20: cmp-ne s_212_17 s_212_19
        let s_212_20: bool = ((s_212_17) != (s_212_19));
        // D s_212_21: write-var gs#117279 <= s_212_20
        fn_state.gs_117279 = s_212_20;
        // N s_212_22: jump b154
        return block_154(state, tracer, fn_state);
    }
    fn block_213<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_213_0: const #440u : u32
        let s_213_0: u32 = 440;
        // D s_213_1: read-reg s_213_0:u8
        let s_213_1: u8 = {
            let value = state.read_register::<u8>(s_213_0 as isize);
            tracer.read_register(s_213_0 as isize, value);
            value
        };
        // D s_213_2: call ELUsingAArch32(s_213_1)
        let s_213_2: bool = ELUsingAArch32(state, tracer, s_213_1);
        // D s_213_3: not s_213_2
        let s_213_3: bool = !s_213_2;
        // D s_213_4: write-var gs#117278 <= s_213_3
        fn_state.gs_117278 = s_213_3;
        // N s_213_5: jump b152
        return block_152(state, tracer, fn_state);
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
        // S s_214_5: call AArch32_TakeHypTrapException(s_214_4)
        let s_214_5: () = AArch32_TakeHypTrapException(state, tracer, s_214_4);
        // N s_214_6: return
        return;
    }
    fn block_215<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_215_0: read-var __HSTR_T9:u8
        let s_215_0: bool = fn_state.u__HSTR_T9;
        // D s_215_1: cast zx s_215_0 -> bv
        let s_215_1: Bits = Bits::new(s_215_0 as u128, 1u16);
        // C s_215_2: const #1u : u8
        let s_215_2: bool = true;
        // C s_215_3: cast zx s_215_2 -> bv
        let s_215_3: Bits = Bits::new(s_215_2 as u128, 1u16);
        // D s_215_4: cmp-eq s_215_1 s_215_3
        let s_215_4: bool = ((s_215_1) == (s_215_3));
        // D s_215_5: write-var gs#117277 <= s_215_4
        fn_state.gs_117277 = s_215_4;
        // N s_215_6: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_216<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_216_0: const #432u : u32
        let s_216_0: u32 = 432;
        // D s_216_1: read-reg s_216_0:u8
        let s_216_1: u8 = {
            let value = state.read_register::<u8>(s_216_0 as isize);
            tracer.read_register(s_216_0 as isize, value);
            value
        };
        // D s_216_2: call ELUsingAArch32(s_216_1)
        let s_216_2: bool = ELUsingAArch32(state, tracer, s_216_1);
        // D s_216_3: write-var gs#117276 <= s_216_2
        fn_state.gs_117276 = s_216_2;
        // N s_216_4: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_217<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_217_0: const #3u : u8
        let s_217_0: u8 = 3;
        // C s_217_1: cast zx s_217_0 -> bv
        let s_217_1: Bits = Bits::new(s_217_0 as u128, 8u16);
        // C s_217_2: cast zx s_217_1 -> i
        let s_217_2: i128 = (s_217_1.value() as i128);
        // C s_217_3: cast reint s_217_2 -> i64
        let s_217_3: i64 = (s_217_2 as i64);
        // C s_217_4: cast zx s_217_3 -> i
        let s_217_4: i128 = (i128::try_from(s_217_3).unwrap());
        // C s_217_5: const #432u : u32
        let s_217_5: u32 = 432;
        // D s_217_6: read-reg s_217_5:u8
        let s_217_6: u8 = {
            let value = state.read_register::<u8>(s_217_5 as isize);
            tracer.read_register(s_217_5 as isize, value);
            value
        };
        // D s_217_7: call AArch64_AArch32SystemAccessTrap(s_217_6, s_217_4)
        let s_217_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_217_6,
            s_217_4,
        );
        // N s_217_8: return
        return;
    }
    fn block_218<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_218_0: read-var __HSTR_EL2_T9:u8
        let s_218_0: bool = fn_state.u__HSTR_EL2_T9;
        // D s_218_1: cast zx s_218_0 -> bv
        let s_218_1: Bits = Bits::new(s_218_0 as u128, 1u16);
        // C s_218_2: const #1u : u8
        let s_218_2: bool = true;
        // C s_218_3: cast zx s_218_2 -> bv
        let s_218_3: Bits = Bits::new(s_218_2 as u128, 1u16);
        // D s_218_4: cmp-eq s_218_1 s_218_3
        let s_218_4: bool = ((s_218_1) == (s_218_3));
        // D s_218_5: write-var gs#117275 <= s_218_4
        fn_state.gs_117275 = s_218_4;
        // N s_218_6: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_219<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_219_0: const #102552u : u32
        let s_219_0: u32 = 102552;
        // D s_219_1: read-reg s_219_0:struct
        let s_219_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_219_0 as isize);
            tracer.read_register(s_219_0 as isize, value);
            value
        };
        // D s_219_2: call _get_HCR_EL2_Type_E2H(s_219_1)
        let s_219_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_219_1);
        // C s_219_3: const #102552u : u32
        let s_219_3: u32 = 102552;
        // D s_219_4: read-reg s_219_3:struct
        let s_219_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_219_3 as isize);
            tracer.read_register(s_219_3 as isize, value);
            value
        };
        // D s_219_5: call _get_HCR_EL2_Type_TGE(s_219_4)
        let s_219_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_219_4);
        // D s_219_6: cast zx s_219_2 -> bv
        let s_219_6: Bits = Bits::new(s_219_2 as u128, 1u16);
        // D s_219_7: cast zx s_219_5 -> bv
        let s_219_7: Bits = Bits::new(s_219_5 as u128, 1u16);
        // D s_219_8: cast reint s_219_6 -> u128
        let s_219_8: u128 = (s_219_6.value() as u128);
        // D s_219_9: size-of s_219_6
        let s_219_9: u16 = s_219_6.length();
        // D s_219_10: cast reint s_219_7 -> u128
        let s_219_10: u128 = (s_219_7.value() as u128);
        // D s_219_11: size-of s_219_7
        let s_219_11: u16 = s_219_7.length();
        // D s_219_12: lsl s_219_8 s_219_11
        let s_219_12: u128 = s_219_8 << s_219_11;
        // D s_219_13: or s_219_12 s_219_10
        let s_219_13: u128 = ((s_219_12) | (s_219_10));
        // D s_219_14: add s_219_9 s_219_11
        let s_219_14: u16 = (s_219_9 + s_219_11);
        // D s_219_15: create-bits s_219_13 s_219_14
        let s_219_15: Bits = Bits::new(s_219_13, s_219_14);
        // D s_219_16: cast reint s_219_15 -> u8
        let s_219_16: u8 = (s_219_15.value() as u8);
        // D s_219_17: cast zx s_219_16 -> bv
        let s_219_17: Bits = Bits::new(s_219_16 as u128, 2u16);
        // C s_219_18: const #3u : u8
        let s_219_18: u8 = 3;
        // C s_219_19: cast zx s_219_18 -> bv
        let s_219_19: Bits = Bits::new(s_219_18 as u128, 2u16);
        // D s_219_20: cmp-ne s_219_17 s_219_19
        let s_219_20: bool = ((s_219_17) != (s_219_19));
        // D s_219_21: write-var gs#117274 <= s_219_20
        fn_state.gs_117274 = s_219_20;
        // N s_219_22: jump b142
        return block_142(state, tracer, fn_state);
    }
    fn block_220<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_220_0: const #432u : u32
        let s_220_0: u32 = 432;
        // D s_220_1: read-reg s_220_0:u8
        let s_220_1: u8 = {
            let value = state.read_register::<u8>(s_220_0 as isize);
            tracer.read_register(s_220_0 as isize, value);
            value
        };
        // D s_220_2: call ELUsingAArch32(s_220_1)
        let s_220_2: bool = ELUsingAArch32(state, tracer, s_220_1);
        // D s_220_3: not s_220_2
        let s_220_3: bool = !s_220_2;
        // D s_220_4: write-var gs#117273 <= s_220_3
        fn_state.gs_117273 = s_220_3;
        // N s_220_5: jump b140
        return block_140(state, tracer, fn_state);
    }
    fn block_221<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_221_0: const #() : ()
        let s_221_0: () = ();
        // S s_221_1: call EL2Enabled(s_221_0)
        let s_221_1: bool = EL2Enabled(state, tracer, s_221_0);
        // N s_221_2: branch s_221_1 b237 b222
        if s_221_1 {
            return block_237(state, tracer, fn_state);
        } else {
            return block_222(state, tracer, fn_state);
        };
    }
    fn block_222<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_222_0: const #0u : u8
        let s_222_0: bool = false;
        // D s_222_1: write-var gs#117295 <= s_222_0
        fn_state.gs_117295 = s_222_0;
        // N s_222_2: jump b223
        return block_223(state, tracer, fn_state);
    }
    fn block_223<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_223_0: read-var gs#117295:u8
        let s_223_0: bool = fn_state.gs_117295;
        // N s_223_1: branch s_223_0 b236 b224
        if s_223_0 {
            return block_236(state, tracer, fn_state);
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
        // D s_224_1: write-var gs#117296 <= s_224_0
        fn_state.gs_117296 = s_224_0;
        // N s_224_2: jump b225
        return block_225(state, tracer, fn_state);
    }
    fn block_225<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_225_0: read-var gs#117296:u8
        let s_225_0: bool = fn_state.gs_117296;
        // N s_225_1: branch s_225_0 b235 b226
        if s_225_0 {
            return block_235(state, tracer, fn_state);
        } else {
            return block_226(state, tracer, fn_state);
        };
    }
    fn block_226<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_226_0: const #() : ()
        let s_226_0: () = ();
        // S s_226_1: call EL2Enabled(s_226_0)
        let s_226_1: bool = EL2Enabled(state, tracer, s_226_0);
        // N s_226_2: branch s_226_1 b234 b227
        if s_226_1 {
            return block_234(state, tracer, fn_state);
        } else {
            return block_227(state, tracer, fn_state);
        };
    }
    fn block_227<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_227_0: const #0u : u8
        let s_227_0: bool = false;
        // D s_227_1: write-var gs#117297 <= s_227_0
        fn_state.gs_117297 = s_227_0;
        // N s_227_2: jump b228
        return block_228(state, tracer, fn_state);
    }
    fn block_228<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_228_0: read-var gs#117297:u8
        let s_228_0: bool = fn_state.gs_117297;
        // N s_228_1: branch s_228_0 b233 b229
        if s_228_0 {
            return block_233(state, tracer, fn_state);
        } else {
            return block_229(state, tracer, fn_state);
        };
    }
    fn block_229<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_229_0: const #0u : u8
        let s_229_0: bool = false;
        // D s_229_1: write-var gs#117298 <= s_229_0
        fn_state.gs_117298 = s_229_0;
        // N s_229_2: jump b230
        return block_230(state, tracer, fn_state);
    }
    fn block_230<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_230_0: read-var gs#117298:u8
        let s_230_0: bool = fn_state.gs_117298;
        // N s_230_1: branch s_230_0 b232 b231
        if s_230_0 {
            return block_232(state, tracer, fn_state);
        } else {
            return block_231(state, tracer, fn_state);
        };
    }
    fn block_231<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_231_0: panic
        panic!("{:?}", ());
        // N s_231_1: return
        return;
    }
    fn block_232<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_232_0: const #0u : u8
        let s_232_0: u8 = 0;
        // C s_232_1: cast zx s_232_0 -> bv
        let s_232_1: Bits = Bits::new(s_232_0 as u128, 8u16);
        // C s_232_2: cast zx s_232_1 -> i
        let s_232_2: i128 = (s_232_1.value() as i128);
        // C s_232_3: cast reint s_232_2 -> i64
        let s_232_3: i64 = (s_232_2 as i64);
        // C s_232_4: cast zx s_232_3 -> i
        let s_232_4: i128 = (i128::try_from(s_232_3).unwrap());
        // S s_232_5: call AArch32_TakeHypTrapException(s_232_4)
        let s_232_5: () = AArch32_TakeHypTrapException(state, tracer, s_232_4);
        // N s_232_6: return
        return;
    }
    fn block_233<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_233_0: read-var __HCR_TGE:u8
        let s_233_0: bool = fn_state.u__HCR_TGE;
        // D s_233_1: cast zx s_233_0 -> bv
        let s_233_1: Bits = Bits::new(s_233_0 as u128, 1u16);
        // C s_233_2: const #1u : u8
        let s_233_2: bool = true;
        // C s_233_3: cast zx s_233_2 -> bv
        let s_233_3: Bits = Bits::new(s_233_2 as u128, 1u16);
        // D s_233_4: cmp-eq s_233_1 s_233_3
        let s_233_4: bool = ((s_233_1) == (s_233_3));
        // D s_233_5: write-var gs#117298 <= s_233_4
        fn_state.gs_117298 = s_233_4;
        // N s_233_6: jump b230
        return block_230(state, tracer, fn_state);
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
        // D s_234_3: write-var gs#117297 <= s_234_2
        fn_state.gs_117297 = s_234_2;
        // N s_234_4: jump b228
        return block_228(state, tracer, fn_state);
    }
    fn block_235<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_235_0: const #3u : u8
        let s_235_0: u8 = 3;
        // C s_235_1: cast zx s_235_0 -> bv
        let s_235_1: Bits = Bits::new(s_235_0 as u128, 8u16);
        // C s_235_2: cast zx s_235_1 -> i
        let s_235_2: i128 = (s_235_1.value() as i128);
        // C s_235_3: cast reint s_235_2 -> i64
        let s_235_3: i64 = (s_235_2 as i64);
        // C s_235_4: cast zx s_235_3 -> i
        let s_235_4: i128 = (i128::try_from(s_235_3).unwrap());
        // C s_235_5: const #432u : u32
        let s_235_5: u32 = 432;
        // D s_235_6: read-reg s_235_5:u8
        let s_235_6: u8 = {
            let value = state.read_register::<u8>(s_235_5 as isize);
            tracer.read_register(s_235_5 as isize, value);
            value
        };
        // D s_235_7: call AArch64_AArch32SystemAccessTrap(s_235_6, s_235_4)
        let s_235_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_235_6,
            s_235_4,
        );
        // N s_235_8: return
        return;
    }
    fn block_236<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_236_0: read-var __HCR_EL2_TGE:u8
        let s_236_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_236_1: cast zx s_236_0 -> bv
        let s_236_1: Bits = Bits::new(s_236_0 as u128, 1u16);
        // C s_236_2: const #1u : u8
        let s_236_2: bool = true;
        // C s_236_3: cast zx s_236_2 -> bv
        let s_236_3: Bits = Bits::new(s_236_2 as u128, 1u16);
        // D s_236_4: cmp-eq s_236_1 s_236_3
        let s_236_4: bool = ((s_236_1) == (s_236_3));
        // D s_236_5: write-var gs#117296 <= s_236_4
        fn_state.gs_117296 = s_236_4;
        // N s_236_6: jump b225
        return block_225(state, tracer, fn_state);
    }
    fn block_237<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_237_0: const #432u : u32
        let s_237_0: u32 = 432;
        // D s_237_1: read-reg s_237_0:u8
        let s_237_1: u8 = {
            let value = state.read_register::<u8>(s_237_0 as isize);
            tracer.read_register(s_237_0 as isize, value);
            value
        };
        // D s_237_2: call ELUsingAArch32(s_237_1)
        let s_237_2: bool = ELUsingAArch32(state, tracer, s_237_1);
        // D s_237_3: not s_237_2
        let s_237_3: bool = !s_237_2;
        // D s_237_4: write-var gs#117295 <= s_237_3
        fn_state.gs_117295 = s_237_3;
        // N s_237_5: jump b223
        return block_223(state, tracer, fn_state);
    }
    fn block_238<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_238_0: read-var __PMUSERENR_EN:u8
        let s_238_0: bool = fn_state.u__PMUSERENR_EN;
        // D s_238_1: cast zx s_238_0 -> bv
        let s_238_1: Bits = Bits::new(s_238_0 as u128, 1u16);
        // C s_238_2: const #0u : u8
        let s_238_2: bool = false;
        // C s_238_3: cast zx s_238_2 -> bv
        let s_238_3: Bits = Bits::new(s_238_2 as u128, 1u16);
        // D s_238_4: cmp-eq s_238_1 s_238_3
        let s_238_4: bool = ((s_238_1) == (s_238_3));
        // D s_238_5: write-var gs#117272 <= s_238_4
        fn_state.gs_117272 = s_238_4;
        // N s_238_6: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_239<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_239_0: const #() : ()
        let s_239_0: () = ();
        // S s_239_1: call EL2Enabled(s_239_0)
        let s_239_1: bool = EL2Enabled(state, tracer, s_239_0);
        // N s_239_2: branch s_239_1 b247 b240
        if s_239_1 {
            return block_247(state, tracer, fn_state);
        } else {
            return block_240(state, tracer, fn_state);
        };
    }
    fn block_240<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_240_0: const #0u : u8
        let s_240_0: bool = false;
        // D s_240_1: write-var gs#117299 <= s_240_0
        fn_state.gs_117299 = s_240_0;
        // N s_240_2: jump b241
        return block_241(state, tracer, fn_state);
    }
    fn block_241<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_241_0: read-var gs#117299:u8
        let s_241_0: bool = fn_state.gs_117299;
        // N s_241_1: branch s_241_0 b246 b242
        if s_241_0 {
            return block_246(state, tracer, fn_state);
        } else {
            return block_242(state, tracer, fn_state);
        };
    }
    fn block_242<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_242_0: const #0u : u8
        let s_242_0: bool = false;
        // D s_242_1: write-var gs#117300 <= s_242_0
        fn_state.gs_117300 = s_242_0;
        // N s_242_2: jump b243
        return block_243(state, tracer, fn_state);
    }
    fn block_243<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_243_0: read-var gs#117300:u8
        let s_243_0: bool = fn_state.gs_117300;
        // N s_243_1: branch s_243_0 b245 b244
        if s_243_0 {
            return block_245(state, tracer, fn_state);
        } else {
            return block_244(state, tracer, fn_state);
        };
    }
    fn block_244<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_244_0: const #3u : u8
        let s_244_0: u8 = 3;
        // C s_244_1: cast zx s_244_0 -> bv
        let s_244_1: Bits = Bits::new(s_244_0 as u128, 8u16);
        // C s_244_2: cast zx s_244_1 -> i
        let s_244_2: i128 = (s_244_1.value() as i128);
        // C s_244_3: cast reint s_244_2 -> i64
        let s_244_3: i64 = (s_244_2 as i64);
        // C s_244_4: cast zx s_244_3 -> i
        let s_244_4: i128 = (i128::try_from(s_244_3).unwrap());
        // C s_244_5: const #440u : u32
        let s_244_5: u32 = 440;
        // D s_244_6: read-reg s_244_5:u8
        let s_244_6: u8 = {
            let value = state.read_register::<u8>(s_244_5 as isize);
            tracer.read_register(s_244_5 as isize, value);
            value
        };
        // D s_244_7: call AArch64_AArch32SystemAccessTrap(s_244_6, s_244_4)
        let s_244_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_244_6,
            s_244_4,
        );
        // N s_244_8: return
        return;
    }
    fn block_245<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_245_0: const #3u : u8
        let s_245_0: u8 = 3;
        // C s_245_1: cast zx s_245_0 -> bv
        let s_245_1: Bits = Bits::new(s_245_0 as u128, 8u16);
        // C s_245_2: cast zx s_245_1 -> i
        let s_245_2: i128 = (s_245_1.value() as i128);
        // C s_245_3: cast reint s_245_2 -> i64
        let s_245_3: i64 = (s_245_2 as i64);
        // C s_245_4: cast zx s_245_3 -> i
        let s_245_4: i128 = (i128::try_from(s_245_3).unwrap());
        // C s_245_5: const #432u : u32
        let s_245_5: u32 = 432;
        // D s_245_6: read-reg s_245_5:u8
        let s_245_6: u8 = {
            let value = state.read_register::<u8>(s_245_5 as isize);
            tracer.read_register(s_245_5 as isize, value);
            value
        };
        // D s_245_7: call AArch64_AArch32SystemAccessTrap(s_245_6, s_245_4)
        let s_245_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_245_6,
            s_245_4,
        );
        // N s_245_8: return
        return;
    }
    fn block_246<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_246_0: read-var __HCR_EL2_TGE:u8
        let s_246_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_246_1: cast zx s_246_0 -> bv
        let s_246_1: Bits = Bits::new(s_246_0 as u128, 1u16);
        // C s_246_2: const #1u : u8
        let s_246_2: bool = true;
        // C s_246_3: cast zx s_246_2 -> bv
        let s_246_3: Bits = Bits::new(s_246_2 as u128, 1u16);
        // D s_246_4: cmp-eq s_246_1 s_246_3
        let s_246_4: bool = ((s_246_1) == (s_246_3));
        // D s_246_5: write-var gs#117300 <= s_246_4
        fn_state.gs_117300 = s_246_4;
        // N s_246_6: jump b243
        return block_243(state, tracer, fn_state);
    }
    fn block_247<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_247_0: const #432u : u32
        let s_247_0: u32 = 432;
        // D s_247_1: read-reg s_247_0:u8
        let s_247_1: u8 = {
            let value = state.read_register::<u8>(s_247_0 as isize);
            tracer.read_register(s_247_0 as isize, value);
            value
        };
        // D s_247_2: call ELUsingAArch32(s_247_1)
        let s_247_2: bool = ELUsingAArch32(state, tracer, s_247_1);
        // D s_247_3: not s_247_2
        let s_247_3: bool = !s_247_2;
        // D s_247_4: write-var gs#117299 <= s_247_3
        fn_state.gs_117299 = s_247_3;
        // N s_247_5: jump b241
        return block_241(state, tracer, fn_state);
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
        // N s_248_2: branch s_248_1 b257 b249
        if s_248_1 {
            return block_257(state, tracer, fn_state);
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
        // D s_249_1: write-var gs#117268 <= s_249_0
        fn_state.gs_117268 = s_249_0;
        // N s_249_2: jump b250
        return block_250(state, tracer, fn_state);
    }
    fn block_250<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_250_0: read-var gs#117268:u8
        let s_250_0: bool = fn_state.gs_117268;
        // N s_250_1: branch s_250_0 b256 b251
        if s_250_0 {
            return block_256(state, tracer, fn_state);
        } else {
            return block_251(state, tracer, fn_state);
        };
    }
    fn block_251<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_251_0: const #204u : u32
        let s_251_0: u32 = 204;
        // S s_251_1: call IsFeatureImplemented(s_251_0)
        let s_251_1: bool = IsFeatureImplemented(state, tracer, s_251_0);
        // S s_251_2: not s_251_1
        let s_251_2: bool = !s_251_1;
        // N s_251_3: branch s_251_2 b255 b252
        if s_251_2 {
            return block_255(state, tracer, fn_state);
        } else {
            return block_252(state, tracer, fn_state);
        };
    }
    fn block_252<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_252_0: const #0u : u8
        let s_252_0: bool = false;
        // D s_252_1: write-var gs#117269 <= s_252_0
        fn_state.gs_117269 = s_252_0;
        // N s_252_2: jump b253
        return block_253(state, tracer, fn_state);
    }
    fn block_253<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_253_0: read-var gs#117269:u8
        let s_253_0: bool = fn_state.gs_117269;
        // D s_253_1: write-var gs#117270 <= s_253_0
        fn_state.gs_117270 = s_253_0;
        // N s_253_2: jump b254
        return block_254(state, tracer, fn_state);
    }
    fn block_254<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_254_0: read-var gs#117270:u8
        let s_254_0: bool = fn_state.gs_117270;
        // D s_254_1: write-var gs#117271 <= s_254_0
        fn_state.gs_117271 = s_254_0;
        // N s_254_2: jump b134
        return block_134(state, tracer, fn_state);
    }
    fn block_255<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_255_0: read-var __PMUSERENR_EL0_EN:u8
        let s_255_0: bool = fn_state.u__PMUSERENR_EL0_EN;
        // D s_255_1: cast zx s_255_0 -> bv
        let s_255_1: Bits = Bits::new(s_255_0 as u128, 1u16);
        // C s_255_2: const #0u : u8
        let s_255_2: bool = false;
        // C s_255_3: cast zx s_255_2 -> bv
        let s_255_3: Bits = Bits::new(s_255_2 as u128, 1u16);
        // D s_255_4: cmp-eq s_255_1 s_255_3
        let s_255_4: bool = ((s_255_1) == (s_255_3));
        // D s_255_5: write-var gs#117269 <= s_255_4
        fn_state.gs_117269 = s_255_4;
        // N s_255_6: jump b253
        return block_253(state, tracer, fn_state);
    }
    fn block_256<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_256_0: const #1u : u8
        let s_256_0: bool = true;
        // D s_256_1: write-var gs#117270 <= s_256_0
        fn_state.gs_117270 = s_256_0;
        // N s_256_2: jump b254
        return block_254(state, tracer, fn_state);
    }
    fn block_257<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_257_0: const #102624u : u32
        let s_257_0: u32 = 102624;
        // D s_257_1: read-reg s_257_0:struct
        let s_257_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_257_0 as isize);
            tracer.read_register(s_257_0 as isize, value);
            value
        };
        // D s_257_2: call _get_PMUSERENR_EL0_Type_UEN(s_257_1)
        let s_257_2: bool = u_get_PMUSERENR_EL0_Type_UEN(state, tracer, s_257_1);
        // C s_257_3: const #102624u : u32
        let s_257_3: u32 = 102624;
        // D s_257_4: read-reg s_257_3:struct
        let s_257_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_257_3 as isize);
            tracer.read_register(s_257_3 as isize, value);
            value
        };
        // D s_257_5: call _get_PMUSERENR_EL0_Type_EN(s_257_4)
        let s_257_5: bool = u_get_PMUSERENR_EL0_Type_EN(state, tracer, s_257_4);
        // D s_257_6: cast zx s_257_2 -> bv
        let s_257_6: Bits = Bits::new(s_257_2 as u128, 1u16);
        // D s_257_7: cast zx s_257_5 -> bv
        let s_257_7: Bits = Bits::new(s_257_5 as u128, 1u16);
        // D s_257_8: cast reint s_257_6 -> u128
        let s_257_8: u128 = (s_257_6.value() as u128);
        // D s_257_9: size-of s_257_6
        let s_257_9: u16 = s_257_6.length();
        // D s_257_10: cast reint s_257_7 -> u128
        let s_257_10: u128 = (s_257_7.value() as u128);
        // D s_257_11: size-of s_257_7
        let s_257_11: u16 = s_257_7.length();
        // D s_257_12: lsl s_257_8 s_257_11
        let s_257_12: u128 = s_257_8 << s_257_11;
        // D s_257_13: or s_257_12 s_257_10
        let s_257_13: u128 = ((s_257_12) | (s_257_10));
        // D s_257_14: add s_257_9 s_257_11
        let s_257_14: u16 = (s_257_9 + s_257_11);
        // D s_257_15: create-bits s_257_13 s_257_14
        let s_257_15: Bits = Bits::new(s_257_13, s_257_14);
        // D s_257_16: cast reint s_257_15 -> u8
        let s_257_16: u8 = (s_257_15.value() as u8);
        // D s_257_17: cast zx s_257_16 -> bv
        let s_257_17: Bits = Bits::new(s_257_16 as u128, 2u16);
        // C s_257_18: const #0u : u8
        let s_257_18: u8 = 0;
        // C s_257_19: cast zx s_257_18 -> bv
        let s_257_19: Bits = Bits::new(s_257_18 as u128, 2u16);
        // D s_257_20: cmp-eq s_257_17 s_257_19
        let s_257_20: bool = ((s_257_17) == (s_257_19));
        // D s_257_21: write-var gs#117268 <= s_257_20
        fn_state.gs_117268 = s_257_20;
        // N s_257_22: jump b250
        return block_250(state, tracer, fn_state);
    }
    fn block_258<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_258_0: panic
        panic!("{:?}", ());
        // N s_258_1: return
        return;
    }
    fn block_259<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_259_0: read-var __MDCR_EL3_TPM:u8
        let s_259_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_259_1: cast zx s_259_0 -> bv
        let s_259_1: Bits = Bits::new(s_259_0 as u128, 1u16);
        // C s_259_2: const #1u : u8
        let s_259_2: bool = true;
        // C s_259_3: cast zx s_259_2 -> bv
        let s_259_3: Bits = Bits::new(s_259_2 as u128, 1u16);
        // D s_259_4: cmp-eq s_259_1 s_259_3
        let s_259_4: bool = ((s_259_1) == (s_259_3));
        // D s_259_5: write-var gs#117267 <= s_259_4
        fn_state.gs_117267 = s_259_4;
        // N s_259_6: jump b131
        return block_131(state, tracer, fn_state);
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
        // D s_260_2: call ELUsingAArch32(s_260_1)
        let s_260_2: bool = ELUsingAArch32(state, tracer, s_260_1);
        // D s_260_3: not s_260_2
        let s_260_3: bool = !s_260_2;
        // D s_260_4: write-var gs#117266 <= s_260_3
        fn_state.gs_117266 = s_260_3;
        // N s_260_5: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_261<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_261_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_261_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_261_1: call __IMPDEF_boolean(s_261_0)
        let s_261_1: bool = u__IMPDEF_boolean(state, tracer, s_261_0);
        // D s_261_2: write-var gs#117265 <= s_261_1
        fn_state.gs_117265 = s_261_1;
        // N s_261_3: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_262<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_262_0: const #() : ()
        let s_262_0: () = ();
        // S s_262_1: call EDSCR_read(s_262_0)
        let s_262_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_262_0);
        // S s_262_2: call _get_EDSCR_Type_SDD(s_262_1)
        let s_262_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_262_1);
        // S s_262_3: cast zx s_262_2 -> bv
        let s_262_3: Bits = Bits::new(s_262_2 as u128, 1u16);
        // C s_262_4: const #1u : u8
        let s_262_4: bool = true;
        // C s_262_5: cast zx s_262_4 -> bv
        let s_262_5: Bits = Bits::new(s_262_4 as u128, 1u16);
        // S s_262_6: cmp-eq s_262_3 s_262_5
        let s_262_6: bool = ((s_262_3) == (s_262_5));
        // D s_262_7: write-var gs#117264 <= s_262_6
        fn_state.gs_117264 = s_262_6;
        // N s_262_8: jump b125
        return block_125(state, tracer, fn_state);
    }
    fn block_263<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_263_0: const #424u : u32
        let s_263_0: u32 = 424;
        // D s_263_1: read-reg s_263_0:u8
        let s_263_1: u8 = {
            let value = state.read_register::<u8>(s_263_0 as isize);
            tracer.read_register(s_263_0 as isize, value);
            value
        };
        // C s_263_2: const #2u : u8
        let s_263_2: u8 = 2;
        // D s_263_3: cmp-lt s_263_1 s_263_2
        let s_263_3: bool = ((s_263_1) < (s_263_2));
        // D s_263_4: write-var gs#117263 <= s_263_3
        fn_state.gs_117263 = s_263_3;
        // N s_263_5: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_264<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_264_0: const #146u : u32
        let s_264_0: u32 = 146;
        // S s_264_1: call IsFeatureImplemented(s_264_0)
        let s_264_1: bool = IsFeatureImplemented(state, tracer, s_264_0);
        // N s_264_2: branch s_264_1 b266 b265
        if s_264_1 {
            return block_266(state, tracer, fn_state);
        } else {
            return block_265(state, tracer, fn_state);
        };
    }
    fn block_265<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_265_0: const #72u : u32
        let s_265_0: u32 = 72;
        // S s_265_1: call ConstrainUnpredictableProcedure(s_265_0)
        let s_265_1: () = ConstrainUnpredictableProcedure(state, tracer, s_265_0);
        // N s_265_2: return
        return;
    }
    fn block_266<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_266_0: panic
        panic!("{:?}", ());
        // N s_266_1: return
        return;
    }
    fn block_267<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_267_0: const #() : ()
        let s_267_0: () = ();
        // S s_267_1: call PMSELR_read(s_267_0)
        let s_267_1: ProductType700c18a878c5601b = PMSELR_read(state, tracer, s_267_0);
        // S s_267_2: call _get_PMSELR_Type_SEL(s_267_1)
        let s_267_2: u8 = u_get_PMSELR_Type_SEL(state, tracer, s_267_1);
        // S s_267_3: cast zx s_267_2 -> bv
        let s_267_3: Bits = Bits::new(s_267_2 as u128, 5u16);
        // S s_267_4: cast zx s_267_3 -> i
        let s_267_4: i128 = (s_267_3.value() as i128);
        // S s_267_5: cast reint s_267_4 -> i64
        let s_267_5: i64 = (s_267_4 as i64);
        // C s_267_6: const #31s : i
        let s_267_6: i128 = 31;
        // S s_267_7: cast zx s_267_5 -> i
        let s_267_7: i128 = (i128::try_from(s_267_5).unwrap());
        // S s_267_8: cmp-ge s_267_7 s_267_6
        let s_267_8: bool = ((s_267_7) >= (s_267_6));
        // D s_267_9: write-var gs#117232 <= s_267_8
        fn_state.gs_117232 = s_267_8;
        // N s_267_10: jump b2
        return block_2(state, tracer, fn_state);
    }
}
