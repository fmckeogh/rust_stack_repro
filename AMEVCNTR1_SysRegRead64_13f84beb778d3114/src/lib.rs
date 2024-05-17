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
use u_get_AMUSERENR_Type_EN::*;
use u_get_SCR_EL3_Type_FGTEn::*;
use IsHighestEL::*;
use Halted::*;
use u_get_AMCR_Type_CG1RZ::*;
use u_get_HSTR_EL2_Type_T5::*;
use AMUSERENR_read::*;
use Zeros::*;
use u_get_HCPTR_Type_TAM::*;
use AMCR_read::*;
use IsG1ActivityMonitorImplemented::*;
use HCPTR_read::*;
use u_get_CPTR_EL2_Type_TAM::*;
use u_get_HAFGRTR_EL2_Type_AMEVCNTR12_EL0::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_E2H::*;
use AMEVCNTR1_read::*;
use HCR_read::*;
use u_get_AMCR_EL0_Type_CG1RZ::*;
use IsFeatureImplemented::*;
use u__IMPDEF_boolean::*;
use AArch64_AArch32SystemAccessTrap::*;
use HSTR_read::*;
use u_get_HCR_Type_TGE::*;
use u_get_HSTR_Type_T5::*;
use u_get_CPTR_EL3_Type_TAM::*;
use R_set::*;
use ELUsingAArch32::*;
use HaveAArch64::*;
use u_get_AMUSERENR_EL0_Type_EN::*;
use EDSCR_read::*;
use AArch32_TakeHypTrapException::*;
use common::*;
pub fn AMEVCNTR1_SysRegRead64_13f84beb778d3114<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    coproc: u8,
    opc1: u8,
    CRm: u8,
    t: i128,
    t2: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_120825: bool,
        gs_120788: bool,
        gs_120844: bool,
        gs_120847: bool,
        gs_120846: bool,
        gs_120781: bool,
        gs_120778: bool,
        u__CPTR_EL2_TAM: bool,
        ga_201038: ProductType5c790c8ef59cc8b2,
        gs_120840: bool,
        ga_201035: ProductType72d61775f103f7e0,
        gs_120748: bool,
        gs_120741: bool,
        u__HCR_TGE: bool,
        gs_120849: bool,
        gs_120744: bool,
        gs_120824: bool,
        ga_201154: ProductType72d61775f103f7e0,
        ga_201147: ProductType72d61775f103f7e0,
        ga_201150: ProductType5c790c8ef59cc8b2,
        gs_120743: bool,
        ga_201163: ProductType72d61775f103f7e0,
        gs_120783: bool,
        gs_120845: bool,
        gs_120875: bool,
        ga_201148: ProductType5c790c8ef59cc8b2,
        ga_201101: ProductType72d61775f103f7e0,
        u__HCR_EL2_TGE: bool,
        ga_201157: ProductType5c790c8ef59cc8b2,
        gs_120790: bool,
        ga_201102: ProductType5c790c8ef59cc8b2,
        gs_120796: bool,
        u__AMCR_CG1RZ: bool,
        u__SCR_EL3_FGTEn: bool,
        gs_120821: bool,
        u__AMUSERENR_EL0_EN: bool,
        gs_120823: bool,
        gs_120797: bool,
        gs_120848: bool,
        gs_120880: bool,
        gs_120786: bool,
        ga_201104: ProductType5c790c8ef59cc8b2,
        ga_201159: ProductType5c790c8ef59cc8b2,
        gs_120881: bool,
        gs_120879: bool,
        gs_120789: bool,
        ga_201092: ProductType72d61775f103f7e0,
        ga_201036: ProductType5c790c8ef59cc8b2,
        gs_120876: bool,
        gs_120842: bool,
        u__AMUSERENR_EN: bool,
        gs_120776: bool,
        gs_120774: bool,
        gs_120739: bool,
        u__AMCR_EL0_CG1RZ: bool,
        gs_120834: bool,
        gs_120794: bool,
        gs_120740: bool,
        gs_120841: bool,
        u__CPTR_EL3_TAM: bool,
        u__HCPTR_TAM: bool,
        gs_120793: bool,
        gs_120747: bool,
        gs_120772: bool,
        gs_120878: bool,
        gs_120777: bool,
        gs_120795: bool,
        gs_120828: bool,
        gs_120832: bool,
        gs_120746: bool,
        gs_120877: bool,
        gs_120829: bool,
        gs_120827: bool,
        ga_201029: ProductType72d61775f103f7e0,
        gs_120822: bool,
        ga_201108: ProductType72d61775f103f7e0,
        gs_120852: bool,
        gs_120775: bool,
        gs_120837: bool,
        gs_120820: bool,
        ga_201042: ProductType72d61775f103f7e0,
        gs_120742: bool,
        gs_120826: bool,
        gs_120792: bool,
        gs_120745: bool,
        gs_120843: bool,
        ga_201138: ProductType72d61775f103f7e0,
        gs_120851: bool,
        gs_120850: bool,
        u__HAFGRTR_EL2_AMEVCNTR12_EL0: bool,
        gs_120773: bool,
        gs_120791: bool,
        gs_120749: bool,
        gs_120839: bool,
        el: u8,
        coproc: u8,
        opc1: u8,
        CRm: u8,
        t: i128,
        t2: i128,
    }
    let fn_state = FunctionState {
        el,
        coproc,
        opc1,
        CRm,
        t,
        t2,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #16840u : u32
        let s_0_0: u32 = 16840;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call _get_CPTR_EL3_Type_TAM(s_0_1)
        let s_0_2: bool = u_get_CPTR_EL3_Type_TAM(state, tracer, s_0_1);
        // D s_0_3: write-var __CPTR_EL3_TAM <= s_0_2
        fn_state.u__CPTR_EL3_TAM = s_0_2;
        // C s_0_4: const #90496u : u32
        let s_0_4: u32 = 90496;
        // D s_0_5: read-reg s_0_4:struct
        let s_0_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_4 as isize);
            tracer.read_register(s_0_4 as isize, value);
            value
        };
        // D s_0_6: call _get_AMUSERENR_EL0_Type_EN(s_0_5)
        let s_0_6: bool = u_get_AMUSERENR_EL0_Type_EN(state, tracer, s_0_5);
        // D s_0_7: write-var __AMUSERENR_EL0_EN <= s_0_6
        fn_state.u__AMUSERENR_EL0_EN = s_0_6;
        // C s_0_8: const #102552u : u32
        let s_0_8: u32 = 102552;
        // D s_0_9: read-reg s_0_8:struct
        let s_0_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_8 as isize);
            tracer.read_register(s_0_8 as isize, value);
            value
        };
        // D s_0_10: call _get_HCR_EL2_Type_TGE(s_0_9)
        let s_0_10: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_0_9);
        // D s_0_11: write-var __HCR_EL2_TGE <= s_0_10
        fn_state.u__HCR_EL2_TGE = s_0_10;
        // C s_0_12: const #() : ()
        let s_0_12: () = ();
        // S s_0_13: call AMUSERENR_read(s_0_12)
        let s_0_13: ProductType700c18a878c5601b = AMUSERENR_read(state, tracer, s_0_12);
        // S s_0_14: call _get_AMUSERENR_Type_EN(s_0_13)
        let s_0_14: bool = u_get_AMUSERENR_Type_EN(state, tracer, s_0_13);
        // D s_0_15: write-var __AMUSERENR_EN <= s_0_14
        fn_state.u__AMUSERENR_EN = s_0_14;
        // C s_0_16: const #() : ()
        let s_0_16: () = ();
        // S s_0_17: call HCR_read(s_0_16)
        let s_0_17: ProductType700c18a878c5601b = HCR_read(state, tracer, s_0_16);
        // S s_0_18: call _get_HCR_Type_TGE(s_0_17)
        let s_0_18: bool = u_get_HCR_Type_TGE(state, tracer, s_0_17);
        // D s_0_19: write-var __HCR_TGE <= s_0_18
        fn_state.u__HCR_TGE = s_0_18;
        // C s_0_20: const #104936u : u32
        let s_0_20: u32 = 104936;
        // D s_0_21: read-reg s_0_20:struct
        let s_0_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_20 as isize);
            tracer.read_register(s_0_20 as isize, value);
            value
        };
        // D s_0_22: call _get_HSTR_EL2_Type_T5(s_0_21)
        let s_0_22: bool = u_get_HSTR_EL2_Type_T5(state, tracer, s_0_21);
        // C s_0_23: const #() : ()
        let s_0_23: () = ();
        // S s_0_24: call HSTR_read(s_0_23)
        let s_0_24: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_23);
        // S s_0_25: call _get_HSTR_Type_T5(s_0_24)
        let s_0_25: bool = u_get_HSTR_Type_T5(state, tracer, s_0_24);
        // C s_0_26: const #11088u : u32
        let s_0_26: u32 = 11088;
        // D s_0_27: read-reg s_0_26:struct
        let s_0_27: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_26 as isize);
            tracer.read_register(s_0_26 as isize, value);
            value
        };
        // D s_0_28: call _get_CPTR_EL2_Type_TAM(s_0_27)
        let s_0_28: bool = u_get_CPTR_EL2_Type_TAM(state, tracer, s_0_27);
        // D s_0_29: write-var __CPTR_EL2_TAM <= s_0_28
        fn_state.u__CPTR_EL2_TAM = s_0_28;
        // C s_0_30: const #() : ()
        let s_0_30: () = ();
        // S s_0_31: call HCPTR_read(s_0_30)
        let s_0_31: ProductType700c18a878c5601b = HCPTR_read(state, tracer, s_0_30);
        // S s_0_32: call _get_HCPTR_Type_TAM(s_0_31)
        let s_0_32: bool = u_get_HCPTR_Type_TAM(state, tracer, s_0_31);
        // D s_0_33: write-var __HCPTR_TAM <= s_0_32
        fn_state.u__HCPTR_TAM = s_0_32;
        // C s_0_34: const #90704u : u32
        let s_0_34: u32 = 90704;
        // D s_0_35: read-reg s_0_34:struct
        let s_0_35: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_34 as isize);
            tracer.read_register(s_0_34 as isize, value);
            value
        };
        // D s_0_36: call _get_SCR_EL3_Type_FGTEn(s_0_35)
        let s_0_36: bool = u_get_SCR_EL3_Type_FGTEn(state, tracer, s_0_35);
        // D s_0_37: write-var __SCR_EL3_FGTEn <= s_0_36
        fn_state.u__SCR_EL3_FGTEn = s_0_36;
        // C s_0_38: const #21760u : u32
        let s_0_38: u32 = 21760;
        // D s_0_39: read-reg s_0_38:struct
        let s_0_39: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_38 as isize);
            tracer.read_register(s_0_38 as isize, value);
            value
        };
        // D s_0_40: call _get_HAFGRTR_EL2_Type_AMEVCNTR12_EL0(s_0_39)
        let s_0_40: bool = u_get_HAFGRTR_EL2_Type_AMEVCNTR12_EL0(state, tracer, s_0_39);
        // D s_0_41: write-var __HAFGRTR_EL2_AMEVCNTR12_EL0 <= s_0_40
        fn_state.u__HAFGRTR_EL2_AMEVCNTR12_EL0 = s_0_40;
        // C s_0_42: const #15544u : u32
        let s_0_42: u32 = 15544;
        // D s_0_43: read-reg s_0_42:struct
        let s_0_43: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_42 as isize);
            tracer.read_register(s_0_42 as isize, value);
            value
        };
        // D s_0_44: call _get_AMCR_EL0_Type_CG1RZ(s_0_43)
        let s_0_44: bool = u_get_AMCR_EL0_Type_CG1RZ(state, tracer, s_0_43);
        // D s_0_45: write-var __AMCR_EL0_CG1RZ <= s_0_44
        fn_state.u__AMCR_EL0_CG1RZ = s_0_44;
        // C s_0_46: const #() : ()
        let s_0_46: () = ();
        // S s_0_47: call AMCR_read(s_0_46)
        let s_0_47: ProductType700c18a878c5601b = AMCR_read(state, tracer, s_0_46);
        // S s_0_48: call _get_AMCR_Type_CG1RZ(s_0_47)
        let s_0_48: bool = u_get_AMCR_Type_CG1RZ(state, tracer, s_0_47);
        // D s_0_49: write-var __AMCR_CG1RZ <= s_0_48
        fn_state.u__AMCR_CG1RZ = s_0_48;
        // C s_0_50: const #2s : i
        let s_0_50: i128 = 2;
        // S s_0_51: call IsG1ActivityMonitorImplemented(s_0_50)
        let s_0_51: bool = IsG1ActivityMonitorImplemented(state, tracer, s_0_50);
        // S s_0_52: not s_0_51
        let s_0_52: bool = !s_0_51;
        // N s_0_53: branch s_0_52 b267 b1
        if s_0_52 {
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
        // C s_1_3: const #448u : u32
        let s_1_3: u32 = 448;
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
        // N s_1_7: branch s_1_6 b135 b2
        if s_1_6 {
            return block_135(state, tracer, fn_state);
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
        // C s_2_3: const #440u : u32
        let s_2_3: u32 = 440;
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
        // N s_2_7: branch s_2_6 b54 b3
        if s_2_6 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #16975u : u32
        let s_3_0: u32 = 16975;
        // D s_3_1: read-reg s_3_0:u8
        let s_3_1: u8 = {
            let value = state.read_register::<u8>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 2u16);
        // C s_3_3: const #432u : u32
        let s_3_3: u32 = 432;
        // D s_3_4: read-reg s_3_3:u8
        let s_3_4: u8 = {
            let value = state.read_register::<u8>(s_3_3 as isize);
            tracer.read_register(s_3_3 as isize, value);
            value
        };
        // D s_3_5: cast zx s_3_4 -> bv
        let s_3_5: Bits = Bits::new(s_3_4 as u128, 2u16);
        // D s_3_6: cmp-eq s_3_2 s_3_5
        let s_3_6: bool = ((s_3_2) == (s_3_5));
        // N s_3_7: branch s_3_6 b7 b4
        if s_3_6 {
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
        // C s_4_0: const #16975u : u32
        let s_4_0: u32 = 16975;
        // D s_4_1: read-reg s_4_0:u8
        let s_4_1: u8 = {
            let value = state.read_register::<u8>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 2u16);
        // C s_4_3: const #424u : u32
        let s_4_3: u32 = 424;
        // D s_4_4: read-reg s_4_3:u8
        let s_4_4: u8 = {
            let value = state.read_register::<u8>(s_4_3 as isize);
            tracer.read_register(s_4_3 as isize, value);
            value
        };
        // D s_4_5: cast zx s_4_4 -> bv
        let s_4_5: Bits = Bits::new(s_4_4 as u128, 2u16);
        // D s_4_6: cmp-eq s_4_2 s_4_5
        let s_4_6: bool = ((s_4_2) == (s_4_5));
        // N s_4_7: branch s_4_6 b6 b5
        if s_4_6 {
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
        // C s_6_0: const #2s : i64
        let s_6_0: i64 = 2;
        // S s_6_1: call AMEVCNTR1_read(s_6_0)
        let s_6_1: ProductType5c790c8ef59cc8b2 = AMEVCNTR1_read(state, tracer, s_6_0);
        // D s_6_2: write-var ga#201157 <= s_6_1
        fn_state.ga_201157 = s_6_1;
        // D s_6_3: read-var ga#201157.0:struct
        let s_6_3: u64 = fn_state.ga_201157._0;
        // C s_6_4: const #32s : i
        let s_6_4: i128 = 32;
        // D s_6_5: cast zx s_6_3 -> bv
        let s_6_5: Bits = Bits::new(s_6_3 as u128, 64u16);
        // C s_6_6: const #1s : i64
        let s_6_6: i64 = 1;
        // C s_6_7: cast zx s_6_6 -> i
        let s_6_7: i128 = (i128::try_from(s_6_6).unwrap());
        // C s_6_8: const #31s : i
        let s_6_8: i128 = 31;
        // C s_6_9: add s_6_8 s_6_7
        let s_6_9: i128 = (s_6_8 + s_6_7);
        // D s_6_10: bit-extract s_6_5 s_6_4 s_6_9
        let s_6_10: Bits = (Bits::new(
            ((s_6_5) >> (s_6_4)).value(),
            u16::try_from(s_6_9).unwrap(),
        ));
        // D s_6_11: cast reint s_6_10 -> u32
        let s_6_11: u32 = (s_6_10.value() as u32);
        // C s_6_12: const #2s : i64
        let s_6_12: i64 = 2;
        // S s_6_13: call AMEVCNTR1_read(s_6_12)
        let s_6_13: ProductType5c790c8ef59cc8b2 = AMEVCNTR1_read(state, tracer, s_6_12);
        // D s_6_14: write-var ga#201159 <= s_6_13
        fn_state.ga_201159 = s_6_13;
        // D s_6_15: read-var ga#201159.0:struct
        let s_6_15: u64 = fn_state.ga_201159._0;
        // C s_6_16: const #0s : i
        let s_6_16: i128 = 0;
        // D s_6_17: cast zx s_6_15 -> bv
        let s_6_17: Bits = Bits::new(s_6_15 as u128, 64u16);
        // C s_6_18: const #1s : i64
        let s_6_18: i64 = 1;
        // C s_6_19: cast zx s_6_18 -> i
        let s_6_19: i128 = (i128::try_from(s_6_18).unwrap());
        // C s_6_20: const #31s : i
        let s_6_20: i128 = 31;
        // C s_6_21: add s_6_20 s_6_19
        let s_6_21: i128 = (s_6_20 + s_6_19);
        // D s_6_22: bit-extract s_6_17 s_6_16 s_6_21
        let s_6_22: Bits = (Bits::new(
            ((s_6_17) >> (s_6_16)).value(),
            u16::try_from(s_6_21).unwrap(),
        ));
        // D s_6_23: cast reint s_6_22 -> u32
        let s_6_23: u32 = (s_6_22.value() as u32);
        // D s_6_24: create-product struct = ["s_6_11", "s_6_23"]
        let s_6_24: ProductType72d61775f103f7e0 = ProductType72d61775f103f7e0 {
            _0: s_6_11,
            _1: s_6_23,
        };
        // D s_6_25: write-var ga#201163 <= s_6_24
        fn_state.ga_201163 = s_6_24;
        // D s_6_26: read-var ga#201163.0:struct
        let s_6_26: u32 = fn_state.ga_201163._0;
        // D s_6_27: read-var ga#201163.1:struct
        let s_6_27: u32 = fn_state.ga_201163._1;
        // D s_6_28: read-var t2:i
        let s_6_28: i128 = fn_state.t2;
        // D s_6_29: call R_set(s_6_28, s_6_26)
        let s_6_29: () = R_set(state, tracer, s_6_28, s_6_26);
        // D s_6_30: read-var t:i
        let s_6_30: i128 = fn_state.t;
        // D s_6_31: call R_set(s_6_30, s_6_27)
        let s_6_31: () = R_set(state, tracer, s_6_30, s_6_27);
        // N s_6_32: return
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
        // N s_7_2: branch s_7_1 b53 b8
        if s_7_1 {
            return block_53(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#120739 <= s_8_0
        fn_state.gs_120739 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#120739:u8
        let s_9_0: bool = fn_state.gs_120739;
        // N s_9_1: branch s_9_0 b52 b10
        if s_9_0 {
            return block_52(state, tracer, fn_state);
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
        // D s_10_1: write-var gs#120740 <= s_10_0
        fn_state.gs_120740 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#120740:u8
        let s_11_0: bool = fn_state.gs_120740;
        // N s_11_1: branch s_11_0 b51 b12
        if s_11_0 {
            return block_51(state, tracer, fn_state);
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
        // D s_12_1: write-var gs#120741 <= s_12_0
        fn_state.gs_120741 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#120741:u8
        let s_13_0: bool = fn_state.gs_120741;
        // N s_13_1: branch s_13_0 b50 b14
        if s_13_0 {
            return block_50(state, tracer, fn_state);
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
        // D s_14_1: write-var gs#120742 <= s_14_0
        fn_state.gs_120742 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#120742:u8
        let s_15_0: bool = fn_state.gs_120742;
        // N s_15_1: branch s_15_0 b49 b16
        if s_15_0 {
            return block_49(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#120743 <= s_16_0
        fn_state.gs_120743 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#120743:u8
        let s_17_0: bool = fn_state.gs_120743;
        // N s_17_1: branch s_17_0 b48 b18
        if s_17_0 {
            return block_48(state, tracer, fn_state);
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
        // N s_18_4: branch s_18_3 b47 b19
        if s_18_3 {
            return block_47(state, tracer, fn_state);
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
        // D s_19_1: write-var gs#120744 <= s_19_0
        fn_state.gs_120744 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#120744:u8
        let s_20_0: bool = fn_state.gs_120744;
        // N s_20_1: branch s_20_0 b46 b21
        if s_20_0 {
            return block_46(state, tracer, fn_state);
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
        // D s_21_1: write-var gs#120745 <= s_21_0
        fn_state.gs_120745 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#120745:u8
        let s_22_0: bool = fn_state.gs_120745;
        // N s_22_1: branch s_22_0 b40 b23
        if s_22_0 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #16975u : u32
        let s_23_0: u32 = 16975;
        // D s_23_1: read-reg s_23_0:u8
        let s_23_1: u8 = {
            let value = state.read_register::<u8>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // D s_23_2: call IsHighestEL(s_23_1)
        let s_23_2: bool = IsHighestEL(state, tracer, s_23_1);
        // D s_23_3: not s_23_2
        let s_23_3: bool = !s_23_2;
        // N s_23_4: branch s_23_3 b39 b24
        if s_23_3 {
            return block_39(state, tracer, fn_state);
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
        // D s_24_1: write-var gs#120746 <= s_24_0
        fn_state.gs_120746 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#120746:u8
        let s_25_0: bool = fn_state.gs_120746;
        // N s_25_1: branch s_25_0 b38 b26
        if s_25_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // D s_26_1: write-var gs#120747 <= s_26_0
        fn_state.gs_120747 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#120747:u8
        let s_27_0: bool = fn_state.gs_120747;
        // N s_27_1: branch s_27_0 b37 b28
        if s_27_0 {
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
        // C s_28_0: const #16975u : u32
        let s_28_0: u32 = 16975;
        // D s_28_1: read-reg s_28_0:u8
        let s_28_1: u8 = {
            let value = state.read_register::<u8>(s_28_0 as isize);
            tracer.read_register(s_28_0 as isize, value);
            value
        };
        // D s_28_2: call IsHighestEL(s_28_1)
        let s_28_2: bool = IsHighestEL(state, tracer, s_28_1);
        // D s_28_3: not s_28_2
        let s_28_3: bool = !s_28_2;
        // N s_28_4: branch s_28_3 b36 b29
        if s_28_3 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #0u : u8
        let s_29_0: bool = false;
        // D s_29_1: write-var gs#120748 <= s_29_0
        fn_state.gs_120748 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#120748:u8
        let s_30_0: bool = fn_state.gs_120748;
        // N s_30_1: branch s_30_0 b35 b31
        if s_30_0 {
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
        // D s_31_1: write-var gs#120749 <= s_31_0
        fn_state.gs_120749 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#120749:u8
        let s_32_0: bool = fn_state.gs_120749;
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
        // C s_33_0: const #2s : i64
        let s_33_0: i64 = 2;
        // S s_33_1: call AMEVCNTR1_read(s_33_0)
        let s_33_1: ProductType5c790c8ef59cc8b2 = AMEVCNTR1_read(state, tracer, s_33_0);
        // D s_33_2: write-var ga#201148 <= s_33_1
        fn_state.ga_201148 = s_33_1;
        // D s_33_3: read-var ga#201148.0:struct
        let s_33_3: u64 = fn_state.ga_201148._0;
        // C s_33_4: const #32s : i
        let s_33_4: i128 = 32;
        // D s_33_5: cast zx s_33_3 -> bv
        let s_33_5: Bits = Bits::new(s_33_3 as u128, 64u16);
        // C s_33_6: const #1s : i64
        let s_33_6: i64 = 1;
        // C s_33_7: cast zx s_33_6 -> i
        let s_33_7: i128 = (i128::try_from(s_33_6).unwrap());
        // C s_33_8: const #31s : i
        let s_33_8: i128 = 31;
        // C s_33_9: add s_33_8 s_33_7
        let s_33_9: i128 = (s_33_8 + s_33_7);
        // D s_33_10: bit-extract s_33_5 s_33_4 s_33_9
        let s_33_10: Bits = (Bits::new(
            ((s_33_5) >> (s_33_4)).value(),
            u16::try_from(s_33_9).unwrap(),
        ));
        // D s_33_11: cast reint s_33_10 -> u32
        let s_33_11: u32 = (s_33_10.value() as u32);
        // C s_33_12: const #2s : i64
        let s_33_12: i64 = 2;
        // S s_33_13: call AMEVCNTR1_read(s_33_12)
        let s_33_13: ProductType5c790c8ef59cc8b2 = AMEVCNTR1_read(
            state,
            tracer,
            s_33_12,
        );
        // D s_33_14: write-var ga#201150 <= s_33_13
        fn_state.ga_201150 = s_33_13;
        // D s_33_15: read-var ga#201150.0:struct
        let s_33_15: u64 = fn_state.ga_201150._0;
        // C s_33_16: const #0s : i
        let s_33_16: i128 = 0;
        // D s_33_17: cast zx s_33_15 -> bv
        let s_33_17: Bits = Bits::new(s_33_15 as u128, 64u16);
        // C s_33_18: const #1s : i64
        let s_33_18: i64 = 1;
        // C s_33_19: cast zx s_33_18 -> i
        let s_33_19: i128 = (i128::try_from(s_33_18).unwrap());
        // C s_33_20: const #31s : i
        let s_33_20: i128 = 31;
        // C s_33_21: add s_33_20 s_33_19
        let s_33_21: i128 = (s_33_20 + s_33_19);
        // D s_33_22: bit-extract s_33_17 s_33_16 s_33_21
        let s_33_22: Bits = (Bits::new(
            ((s_33_17) >> (s_33_16)).value(),
            u16::try_from(s_33_21).unwrap(),
        ));
        // D s_33_23: cast reint s_33_22 -> u32
        let s_33_23: u32 = (s_33_22.value() as u32);
        // D s_33_24: create-product struct = ["s_33_11", "s_33_23"]
        let s_33_24: ProductType72d61775f103f7e0 = ProductType72d61775f103f7e0 {
            _0: s_33_11,
            _1: s_33_23,
        };
        // D s_33_25: write-var ga#201154 <= s_33_24
        fn_state.ga_201154 = s_33_24;
        // D s_33_26: read-var ga#201154.0:struct
        let s_33_26: u32 = fn_state.ga_201154._0;
        // D s_33_27: read-var ga#201154.1:struct
        let s_33_27: u32 = fn_state.ga_201154._1;
        // D s_33_28: read-var t2:i
        let s_33_28: i128 = fn_state.t2;
        // D s_33_29: call R_set(s_33_28, s_33_26)
        let s_33_29: () = R_set(state, tracer, s_33_28, s_33_26);
        // D s_33_30: read-var t:i
        let s_33_30: i128 = fn_state.t;
        // D s_33_31: call R_set(s_33_30, s_33_27)
        let s_33_31: () = R_set(state, tracer, s_33_30, s_33_27);
        // N s_33_32: return
        return;
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #32s : i
        let s_34_0: i128 = 32;
        // S s_34_1: call Zeros(s_34_0)
        let s_34_1: Bits = Zeros(state, tracer, s_34_0);
        // S s_34_2: cast reint s_34_1 -> u32
        let s_34_2: u32 = (s_34_1.value() as u32);
        // C s_34_3: const #32s : i
        let s_34_3: i128 = 32;
        // S s_34_4: call Zeros(s_34_3)
        let s_34_4: Bits = Zeros(state, tracer, s_34_3);
        // S s_34_5: cast reint s_34_4 -> u32
        let s_34_5: u32 = (s_34_4.value() as u32);
        // D s_34_6: create-product struct = ["s_34_2", "s_34_5"]
        let s_34_6: ProductType72d61775f103f7e0 = ProductType72d61775f103f7e0 {
            _0: s_34_2,
            _1: s_34_5,
        };
        // D s_34_7: write-var ga#201147 <= s_34_6
        fn_state.ga_201147 = s_34_6;
        // D s_34_8: read-var ga#201147.0:struct
        let s_34_8: u32 = fn_state.ga_201147._0;
        // D s_34_9: read-var ga#201147.1:struct
        let s_34_9: u32 = fn_state.ga_201147._1;
        // D s_34_10: read-var t2:i
        let s_34_10: i128 = fn_state.t2;
        // D s_34_11: call R_set(s_34_10, s_34_8)
        let s_34_11: () = R_set(state, tracer, s_34_10, s_34_8);
        // D s_34_12: read-var t:i
        let s_34_12: i128 = fn_state.t;
        // D s_34_13: call R_set(s_34_12, s_34_9)
        let s_34_13: () = R_set(state, tracer, s_34_12, s_34_9);
        // N s_34_14: return
        return;
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var __AMCR_CG1RZ:u8
        let s_35_0: bool = fn_state.u__AMCR_CG1RZ;
        // D s_35_1: cast zx s_35_0 -> bv
        let s_35_1: Bits = Bits::new(s_35_0 as u128, 1u16);
        // C s_35_2: const #1u : u8
        let s_35_2: bool = true;
        // C s_35_3: cast zx s_35_2 -> bv
        let s_35_3: Bits = Bits::new(s_35_2 as u128, 1u16);
        // D s_35_4: cmp-eq s_35_1 s_35_3
        let s_35_4: bool = ((s_35_1) == (s_35_3));
        // D s_35_5: write-var gs#120749 <= s_35_4
        fn_state.gs_120749 = s_35_4;
        // N s_35_6: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #() : ()
        let s_36_0: () = ();
        // S s_36_1: call HaveAArch64(s_36_0)
        let s_36_1: bool = HaveAArch64(state, tracer, s_36_0);
        // S s_36_2: not s_36_1
        let s_36_2: bool = !s_36_1;
        // D s_36_3: write-var gs#120748 <= s_36_2
        fn_state.gs_120748 = s_36_2;
        // N s_36_4: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #32s : i
        let s_37_0: i128 = 32;
        // S s_37_1: call Zeros(s_37_0)
        let s_37_1: Bits = Zeros(state, tracer, s_37_0);
        // S s_37_2: cast reint s_37_1 -> u32
        let s_37_2: u32 = (s_37_1.value() as u32);
        // C s_37_3: const #32s : i
        let s_37_3: i128 = 32;
        // S s_37_4: call Zeros(s_37_3)
        let s_37_4: Bits = Zeros(state, tracer, s_37_3);
        // S s_37_5: cast reint s_37_4 -> u32
        let s_37_5: u32 = (s_37_4.value() as u32);
        // D s_37_6: create-product struct = ["s_37_2", "s_37_5"]
        let s_37_6: ProductType72d61775f103f7e0 = ProductType72d61775f103f7e0 {
            _0: s_37_2,
            _1: s_37_5,
        };
        // D s_37_7: write-var ga#201138 <= s_37_6
        fn_state.ga_201138 = s_37_6;
        // D s_37_8: read-var ga#201138.0:struct
        let s_37_8: u32 = fn_state.ga_201138._0;
        // D s_37_9: read-var ga#201138.1:struct
        let s_37_9: u32 = fn_state.ga_201138._1;
        // D s_37_10: read-var t2:i
        let s_37_10: i128 = fn_state.t2;
        // D s_37_11: call R_set(s_37_10, s_37_8)
        let s_37_11: () = R_set(state, tracer, s_37_10, s_37_8);
        // D s_37_12: read-var t:i
        let s_37_12: i128 = fn_state.t;
        // D s_37_13: call R_set(s_37_12, s_37_9)
        let s_37_13: () = R_set(state, tracer, s_37_12, s_37_9);
        // N s_37_14: return
        return;
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var __AMCR_EL0_CG1RZ:u8
        let s_38_0: bool = fn_state.u__AMCR_EL0_CG1RZ;
        // D s_38_1: cast zx s_38_0 -> bv
        let s_38_1: Bits = Bits::new(s_38_0 as u128, 1u16);
        // C s_38_2: const #1u : u8
        let s_38_2: bool = true;
        // C s_38_3: cast zx s_38_2 -> bv
        let s_38_3: Bits = Bits::new(s_38_2 as u128, 1u16);
        // D s_38_4: cmp-eq s_38_1 s_38_3
        let s_38_4: bool = ((s_38_1) == (s_38_3));
        // D s_38_5: write-var gs#120747 <= s_38_4
        fn_state.gs_120747 = s_38_4;
        // N s_38_6: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #() : ()
        let s_39_0: () = ();
        // S s_39_1: call HaveAArch64(s_39_0)
        let s_39_1: bool = HaveAArch64(state, tracer, s_39_0);
        // D s_39_2: write-var gs#120746 <= s_39_1
        fn_state.gs_120746 = s_39_1;
        // N s_39_3: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #() : ()
        let s_40_0: () = ();
        // S s_40_1: call Halted(s_40_0)
        let s_40_1: bool = Halted(state, tracer, s_40_0);
        // N s_40_2: branch s_40_1 b45 b41
        if s_40_1 {
            return block_45(state, tracer, fn_state);
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
        // D s_41_1: write-var gs#120772 <= s_41_0
        fn_state.gs_120772 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#120772:u8
        let s_42_0: bool = fn_state.gs_120772;
        // N s_42_1: branch s_42_0 b44 b43
        if s_42_0 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #4u : u8
        let s_43_0: u8 = 4;
        // C s_43_1: cast zx s_43_0 -> bv
        let s_43_1: Bits = Bits::new(s_43_0 as u128, 8u16);
        // C s_43_2: cast zx s_43_1 -> i
        let s_43_2: i128 = (s_43_1.value() as i128);
        // C s_43_3: cast reint s_43_2 -> i64
        let s_43_3: i64 = (s_43_2 as i64);
        // C s_43_4: cast zx s_43_3 -> i
        let s_43_4: i128 = (i128::try_from(s_43_3).unwrap());
        // C s_43_5: const #424u : u32
        let s_43_5: u32 = 424;
        // D s_43_6: read-reg s_43_5:u8
        let s_43_6: u8 = {
            let value = state.read_register::<u8>(s_43_5 as isize);
            tracer.read_register(s_43_5 as isize, value);
            value
        };
        // D s_43_7: call AArch64_AArch32SystemAccessTrap(s_43_6, s_43_4)
        let s_43_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_43_6, s_43_4);
        // N s_43_8: return
        return;
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_44_0: panic
        panic!("{:?}", ());
        // N s_44_1: return
        return;
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #() : ()
        let s_45_0: () = ();
        // S s_45_1: call EDSCR_read(s_45_0)
        let s_45_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_45_0);
        // S s_45_2: call _get_EDSCR_Type_SDD(s_45_1)
        let s_45_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_45_1);
        // S s_45_3: cast zx s_45_2 -> bv
        let s_45_3: Bits = Bits::new(s_45_2 as u128, 1u16);
        // C s_45_4: const #1u : u8
        let s_45_4: bool = true;
        // C s_45_5: cast zx s_45_4 -> bv
        let s_45_5: Bits = Bits::new(s_45_4 as u128, 1u16);
        // S s_45_6: cmp-eq s_45_3 s_45_5
        let s_45_6: bool = ((s_45_3) == (s_45_5));
        // D s_45_7: write-var gs#120772 <= s_45_6
        fn_state.gs_120772 = s_45_6;
        // N s_45_8: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var __CPTR_EL3_TAM:u8
        let s_46_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_46_1: cast zx s_46_0 -> bv
        let s_46_1: Bits = Bits::new(s_46_0 as u128, 1u16);
        // C s_46_2: const #1u : u8
        let s_46_2: bool = true;
        // C s_46_3: cast zx s_46_2 -> bv
        let s_46_3: Bits = Bits::new(s_46_2 as u128, 1u16);
        // D s_46_4: cmp-eq s_46_1 s_46_3
        let s_46_4: bool = ((s_46_1) == (s_46_3));
        // D s_46_5: write-var gs#120745 <= s_46_4
        fn_state.gs_120745 = s_46_4;
        // N s_46_6: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #424u : u32
        let s_47_0: u32 = 424;
        // D s_47_1: read-reg s_47_0:u8
        let s_47_1: u8 = {
            let value = state.read_register::<u8>(s_47_0 as isize);
            tracer.read_register(s_47_0 as isize, value);
            value
        };
        // D s_47_2: call ELUsingAArch32(s_47_1)
        let s_47_2: bool = ELUsingAArch32(state, tracer, s_47_1);
        // D s_47_3: not s_47_2
        let s_47_3: bool = !s_47_2;
        // D s_47_4: write-var gs#120744 <= s_47_3
        fn_state.gs_120744 = s_47_3;
        // N s_47_5: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_48_0: panic
        panic!("{:?}", ());
        // N s_48_1: return
        return;
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var __CPTR_EL3_TAM:u8
        let s_49_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_49_1: cast zx s_49_0 -> bv
        let s_49_1: Bits = Bits::new(s_49_0 as u128, 1u16);
        // C s_49_2: const #1u : u8
        let s_49_2: bool = true;
        // C s_49_3: cast zx s_49_2 -> bv
        let s_49_3: Bits = Bits::new(s_49_2 as u128, 1u16);
        // D s_49_4: cmp-eq s_49_1 s_49_3
        let s_49_4: bool = ((s_49_1) == (s_49_3));
        // D s_49_5: write-var gs#120743 <= s_49_4
        fn_state.gs_120743 = s_49_4;
        // N s_49_6: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #424u : u32
        let s_50_0: u32 = 424;
        // D s_50_1: read-reg s_50_0:u8
        let s_50_1: u8 = {
            let value = state.read_register::<u8>(s_50_0 as isize);
            tracer.read_register(s_50_0 as isize, value);
            value
        };
        // D s_50_2: call ELUsingAArch32(s_50_1)
        let s_50_2: bool = ELUsingAArch32(state, tracer, s_50_1);
        // D s_50_3: not s_50_2
        let s_50_3: bool = !s_50_2;
        // D s_50_4: write-var gs#120742 <= s_50_3
        fn_state.gs_120742 = s_50_3;
        // N s_50_5: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_51_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_51_1: call __IMPDEF_boolean(s_51_0)
        let s_51_1: bool = u__IMPDEF_boolean(state, tracer, s_51_0);
        // D s_51_2: write-var gs#120741 <= s_51_1
        fn_state.gs_120741 = s_51_1;
        // N s_51_3: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #() : ()
        let s_52_0: () = ();
        // S s_52_1: call EDSCR_read(s_52_0)
        let s_52_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_52_0);
        // S s_52_2: call _get_EDSCR_Type_SDD(s_52_1)
        let s_52_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_52_1);
        // S s_52_3: cast zx s_52_2 -> bv
        let s_52_3: Bits = Bits::new(s_52_2 as u128, 1u16);
        // C s_52_4: const #1u : u8
        let s_52_4: bool = true;
        // C s_52_5: cast zx s_52_4 -> bv
        let s_52_5: Bits = Bits::new(s_52_4 as u128, 1u16);
        // S s_52_6: cmp-eq s_52_3 s_52_5
        let s_52_6: bool = ((s_52_3) == (s_52_5));
        // D s_52_7: write-var gs#120740 <= s_52_6
        fn_state.gs_120740 = s_52_6;
        // N s_52_8: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #424u : u32
        let s_53_0: u32 = 424;
        // D s_53_1: read-reg s_53_0:u8
        let s_53_1: u8 = {
            let value = state.read_register::<u8>(s_53_0 as isize);
            tracer.read_register(s_53_0 as isize, value);
            value
        };
        // C s_53_2: const #2u : u8
        let s_53_2: u8 = 2;
        // D s_53_3: cmp-lt s_53_1 s_53_2
        let s_53_3: bool = ((s_53_1) < (s_53_2));
        // D s_53_4: write-var gs#120739 <= s_53_3
        fn_state.gs_120739 = s_53_3;
        // N s_53_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #() : ()
        let s_54_0: () = ();
        // S s_54_1: call Halted(s_54_0)
        let s_54_1: bool = Halted(state, tracer, s_54_0);
        // N s_54_2: branch s_54_1 b134 b55
        if s_54_1 {
            return block_134(state, tracer, fn_state);
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
        // D s_55_1: write-var gs#120773 <= s_55_0
        fn_state.gs_120773 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#120773:u8
        let s_56_0: bool = fn_state.gs_120773;
        // N s_56_1: branch s_56_0 b133 b57
        if s_56_0 {
            return block_133(state, tracer, fn_state);
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
        // D s_57_1: write-var gs#120774 <= s_57_0
        fn_state.gs_120774 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var gs#120774:u8
        let s_58_0: bool = fn_state.gs_120774;
        // N s_58_1: branch s_58_0 b132 b59
        if s_58_0 {
            return block_132(state, tracer, fn_state);
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
        // D s_59_1: write-var gs#120775 <= s_59_0
        fn_state.gs_120775 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#120775:u8
        let s_60_0: bool = fn_state.gs_120775;
        // N s_60_1: branch s_60_0 b131 b61
        if s_60_0 {
            return block_131(state, tracer, fn_state);
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
        // D s_61_1: write-var gs#120776 <= s_61_0
        fn_state.gs_120776 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#120776:u8
        let s_62_0: bool = fn_state.gs_120776;
        // N s_62_1: branch s_62_0 b130 b63
        if s_62_0 {
            return block_130(state, tracer, fn_state);
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
        // D s_63_1: write-var gs#120777 <= s_63_0
        fn_state.gs_120777 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#120777:u8
        let s_64_0: bool = fn_state.gs_120777;
        // N s_64_1: branch s_64_0 b129 b65
        if s_64_0 {
            return block_129(state, tracer, fn_state);
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
        // N s_65_2: branch s_65_1 b128 b66
        if s_65_1 {
            return block_128(state, tracer, fn_state);
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
        // D s_66_1: write-var gs#120778 <= s_66_0
        fn_state.gs_120778 = s_66_0;
        // N s_66_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#120778:u8
        let s_67_0: bool = fn_state.gs_120778;
        // N s_67_1: branch s_67_0 b127 b68
        if s_67_0 {
            return block_127(state, tracer, fn_state);
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
        // D s_68_1: write-var gs#120781 <= s_68_0
        fn_state.gs_120781 = s_68_0;
        // N s_68_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var gs#120781:u8
        let s_69_0: bool = fn_state.gs_120781;
        // N s_69_1: branch s_69_0 b126 b70
        if s_69_0 {
            return block_126(state, tracer, fn_state);
        } else {
            return block_70(state, tracer, fn_state);
        };
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_70_0: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #() : ()
        let s_71_0: () = ();
        // S s_71_1: call EL2Enabled(s_71_0)
        let s_71_1: bool = EL2Enabled(state, tracer, s_71_0);
        // N s_71_2: branch s_71_1 b125 b72
        if s_71_1 {
            return block_125(state, tracer, fn_state);
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
        // D s_72_1: write-var gs#120783 <= s_72_0
        fn_state.gs_120783 = s_72_0;
        // N s_72_2: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var gs#120783:u8
        let s_73_0: bool = fn_state.gs_120783;
        // N s_73_1: branch s_73_0 b124 b74
        if s_73_0 {
            return block_124(state, tracer, fn_state);
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
        // D s_74_1: write-var gs#120786 <= s_74_0
        fn_state.gs_120786 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var gs#120786:u8
        let s_75_0: bool = fn_state.gs_120786;
        // N s_75_1: branch s_75_0 b123 b76
        if s_75_0 {
            return block_123(state, tracer, fn_state);
        } else {
            return block_76(state, tracer, fn_state);
        };
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_76_0: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #() : ()
        let s_77_0: () = ();
        // S s_77_1: call EL2Enabled(s_77_0)
        let s_77_1: bool = EL2Enabled(state, tracer, s_77_0);
        // N s_77_2: branch s_77_1 b122 b78
        if s_77_1 {
            return block_122(state, tracer, fn_state);
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
        // D s_78_1: write-var gs#120788 <= s_78_0
        fn_state.gs_120788 = s_78_0;
        // N s_78_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var gs#120788:u8
        let s_79_0: bool = fn_state.gs_120788;
        // N s_79_1: branch s_79_0 b121 b80
        if s_79_0 {
            return block_121(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #0u : u8
        let s_80_0: bool = false;
        // D s_80_1: write-var gs#120789 <= s_80_0
        fn_state.gs_120789 = s_80_0;
        // N s_80_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#120789:u8
        let s_81_0: bool = fn_state.gs_120789;
        // N s_81_1: branch s_81_0 b120 b82
        if s_81_0 {
            return block_120(state, tracer, fn_state);
        } else {
            return block_82(state, tracer, fn_state);
        };
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #() : ()
        let s_82_0: () = ();
        // S s_82_1: call EL2Enabled(s_82_0)
        let s_82_1: bool = EL2Enabled(state, tracer, s_82_0);
        // N s_82_2: branch s_82_1 b119 b83
        if s_82_1 {
            return block_119(state, tracer, fn_state);
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
        // D s_83_1: write-var gs#120790 <= s_83_0
        fn_state.gs_120790 = s_83_0;
        // N s_83_2: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var gs#120790:u8
        let s_84_0: bool = fn_state.gs_120790;
        // N s_84_1: branch s_84_0 b118 b85
        if s_84_0 {
            return block_118(state, tracer, fn_state);
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
        // D s_85_1: write-var gs#120791 <= s_85_0
        fn_state.gs_120791 = s_85_0;
        // N s_85_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var gs#120791:u8
        let s_86_0: bool = fn_state.gs_120791;
        // N s_86_1: branch s_86_0 b117 b87
        if s_86_0 {
            return block_117(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #424u : u32
        let s_87_0: u32 = 424;
        // D s_87_1: read-reg s_87_0:u8
        let s_87_1: u8 = {
            let value = state.read_register::<u8>(s_87_0 as isize);
            tracer.read_register(s_87_0 as isize, value);
            value
        };
        // C s_87_2: const #2u : u8
        let s_87_2: u8 = 2;
        // D s_87_3: cmp-lt s_87_1 s_87_2
        let s_87_3: bool = ((s_87_1) < (s_87_2));
        // N s_87_4: branch s_87_3 b116 b88
        if s_87_3 {
            return block_116(state, tracer, fn_state);
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
        // D s_88_1: write-var gs#120792 <= s_88_0
        fn_state.gs_120792 = s_88_0;
        // N s_88_2: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_89_0: read-var gs#120792:u8
        let s_89_0: bool = fn_state.gs_120792;
        // N s_89_1: branch s_89_0 b115 b90
        if s_89_0 {
            return block_115(state, tracer, fn_state);
        } else {
            return block_90(state, tracer, fn_state);
        };
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #0u : u8
        let s_90_0: bool = false;
        // D s_90_1: write-var gs#120793 <= s_90_0
        fn_state.gs_120793 = s_90_0;
        // N s_90_2: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_91_0: read-var gs#120793:u8
        let s_91_0: bool = fn_state.gs_120793;
        // N s_91_1: branch s_91_0 b109 b92
        if s_91_0 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_92(state, tracer, fn_state);
        };
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #16975u : u32
        let s_92_0: u32 = 16975;
        // D s_92_1: read-reg s_92_0:u8
        let s_92_1: u8 = {
            let value = state.read_register::<u8>(s_92_0 as isize);
            tracer.read_register(s_92_0 as isize, value);
            value
        };
        // D s_92_2: call IsHighestEL(s_92_1)
        let s_92_2: bool = IsHighestEL(state, tracer, s_92_1);
        // D s_92_3: not s_92_2
        let s_92_3: bool = !s_92_2;
        // N s_92_4: branch s_92_3 b108 b93
        if s_92_3 {
            return block_108(state, tracer, fn_state);
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
        // D s_93_1: write-var gs#120794 <= s_93_0
        fn_state.gs_120794 = s_93_0;
        // N s_93_2: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var gs#120794:u8
        let s_94_0: bool = fn_state.gs_120794;
        // N s_94_1: branch s_94_0 b107 b95
        if s_94_0 {
            return block_107(state, tracer, fn_state);
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
        // D s_95_1: write-var gs#120795 <= s_95_0
        fn_state.gs_120795 = s_95_0;
        // N s_95_2: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var gs#120795:u8
        let s_96_0: bool = fn_state.gs_120795;
        // N s_96_1: branch s_96_0 b106 b97
        if s_96_0 {
            return block_106(state, tracer, fn_state);
        } else {
            return block_97(state, tracer, fn_state);
        };
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #16975u : u32
        let s_97_0: u32 = 16975;
        // D s_97_1: read-reg s_97_0:u8
        let s_97_1: u8 = {
            let value = state.read_register::<u8>(s_97_0 as isize);
            tracer.read_register(s_97_0 as isize, value);
            value
        };
        // D s_97_2: call IsHighestEL(s_97_1)
        let s_97_2: bool = IsHighestEL(state, tracer, s_97_1);
        // D s_97_3: not s_97_2
        let s_97_3: bool = !s_97_2;
        // N s_97_4: branch s_97_3 b105 b98
        if s_97_3 {
            return block_105(state, tracer, fn_state);
        } else {
            return block_98(state, tracer, fn_state);
        };
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #0u : u8
        let s_98_0: bool = false;
        // D s_98_1: write-var gs#120796 <= s_98_0
        fn_state.gs_120796 = s_98_0;
        // N s_98_2: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_99_0: read-var gs#120796:u8
        let s_99_0: bool = fn_state.gs_120796;
        // N s_99_1: branch s_99_0 b104 b100
        if s_99_0 {
            return block_104(state, tracer, fn_state);
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
        // D s_100_1: write-var gs#120797 <= s_100_0
        fn_state.gs_120797 = s_100_0;
        // N s_100_2: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_101_0: read-var gs#120797:u8
        let s_101_0: bool = fn_state.gs_120797;
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
        // C s_102_0: const #2s : i64
        let s_102_0: i64 = 2;
        // S s_102_1: call AMEVCNTR1_read(s_102_0)
        let s_102_1: ProductType5c790c8ef59cc8b2 = AMEVCNTR1_read(
            state,
            tracer,
            s_102_0,
        );
        // D s_102_2: write-var ga#201102 <= s_102_1
        fn_state.ga_201102 = s_102_1;
        // D s_102_3: read-var ga#201102.0:struct
        let s_102_3: u64 = fn_state.ga_201102._0;
        // C s_102_4: const #32s : i
        let s_102_4: i128 = 32;
        // D s_102_5: cast zx s_102_3 -> bv
        let s_102_5: Bits = Bits::new(s_102_3 as u128, 64u16);
        // C s_102_6: const #1s : i64
        let s_102_6: i64 = 1;
        // C s_102_7: cast zx s_102_6 -> i
        let s_102_7: i128 = (i128::try_from(s_102_6).unwrap());
        // C s_102_8: const #31s : i
        let s_102_8: i128 = 31;
        // C s_102_9: add s_102_8 s_102_7
        let s_102_9: i128 = (s_102_8 + s_102_7);
        // D s_102_10: bit-extract s_102_5 s_102_4 s_102_9
        let s_102_10: Bits = (Bits::new(
            ((s_102_5) >> (s_102_4)).value(),
            u16::try_from(s_102_9).unwrap(),
        ));
        // D s_102_11: cast reint s_102_10 -> u32
        let s_102_11: u32 = (s_102_10.value() as u32);
        // C s_102_12: const #2s : i64
        let s_102_12: i64 = 2;
        // S s_102_13: call AMEVCNTR1_read(s_102_12)
        let s_102_13: ProductType5c790c8ef59cc8b2 = AMEVCNTR1_read(
            state,
            tracer,
            s_102_12,
        );
        // D s_102_14: write-var ga#201104 <= s_102_13
        fn_state.ga_201104 = s_102_13;
        // D s_102_15: read-var ga#201104.0:struct
        let s_102_15: u64 = fn_state.ga_201104._0;
        // C s_102_16: const #0s : i
        let s_102_16: i128 = 0;
        // D s_102_17: cast zx s_102_15 -> bv
        let s_102_17: Bits = Bits::new(s_102_15 as u128, 64u16);
        // C s_102_18: const #1s : i64
        let s_102_18: i64 = 1;
        // C s_102_19: cast zx s_102_18 -> i
        let s_102_19: i128 = (i128::try_from(s_102_18).unwrap());
        // C s_102_20: const #31s : i
        let s_102_20: i128 = 31;
        // C s_102_21: add s_102_20 s_102_19
        let s_102_21: i128 = (s_102_20 + s_102_19);
        // D s_102_22: bit-extract s_102_17 s_102_16 s_102_21
        let s_102_22: Bits = (Bits::new(
            ((s_102_17) >> (s_102_16)).value(),
            u16::try_from(s_102_21).unwrap(),
        ));
        // D s_102_23: cast reint s_102_22 -> u32
        let s_102_23: u32 = (s_102_22.value() as u32);
        // D s_102_24: create-product struct = ["s_102_11", "s_102_23"]
        let s_102_24: ProductType72d61775f103f7e0 = ProductType72d61775f103f7e0 {
            _0: s_102_11,
            _1: s_102_23,
        };
        // D s_102_25: write-var ga#201108 <= s_102_24
        fn_state.ga_201108 = s_102_24;
        // D s_102_26: read-var ga#201108.0:struct
        let s_102_26: u32 = fn_state.ga_201108._0;
        // D s_102_27: read-var ga#201108.1:struct
        let s_102_27: u32 = fn_state.ga_201108._1;
        // D s_102_28: read-var t2:i
        let s_102_28: i128 = fn_state.t2;
        // D s_102_29: call R_set(s_102_28, s_102_26)
        let s_102_29: () = R_set(state, tracer, s_102_28, s_102_26);
        // D s_102_30: read-var t:i
        let s_102_30: i128 = fn_state.t;
        // D s_102_31: call R_set(s_102_30, s_102_27)
        let s_102_31: () = R_set(state, tracer, s_102_30, s_102_27);
        // N s_102_32: return
        return;
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #32s : i
        let s_103_0: i128 = 32;
        // S s_103_1: call Zeros(s_103_0)
        let s_103_1: Bits = Zeros(state, tracer, s_103_0);
        // S s_103_2: cast reint s_103_1 -> u32
        let s_103_2: u32 = (s_103_1.value() as u32);
        // C s_103_3: const #32s : i
        let s_103_3: i128 = 32;
        // S s_103_4: call Zeros(s_103_3)
        let s_103_4: Bits = Zeros(state, tracer, s_103_3);
        // S s_103_5: cast reint s_103_4 -> u32
        let s_103_5: u32 = (s_103_4.value() as u32);
        // D s_103_6: create-product struct = ["s_103_2", "s_103_5"]
        let s_103_6: ProductType72d61775f103f7e0 = ProductType72d61775f103f7e0 {
            _0: s_103_2,
            _1: s_103_5,
        };
        // D s_103_7: write-var ga#201101 <= s_103_6
        fn_state.ga_201101 = s_103_6;
        // D s_103_8: read-var ga#201101.0:struct
        let s_103_8: u32 = fn_state.ga_201101._0;
        // D s_103_9: read-var ga#201101.1:struct
        let s_103_9: u32 = fn_state.ga_201101._1;
        // D s_103_10: read-var t2:i
        let s_103_10: i128 = fn_state.t2;
        // D s_103_11: call R_set(s_103_10, s_103_8)
        let s_103_11: () = R_set(state, tracer, s_103_10, s_103_8);
        // D s_103_12: read-var t:i
        let s_103_12: i128 = fn_state.t;
        // D s_103_13: call R_set(s_103_12, s_103_9)
        let s_103_13: () = R_set(state, tracer, s_103_12, s_103_9);
        // N s_103_14: return
        return;
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_104_0: read-var __AMCR_CG1RZ:u8
        let s_104_0: bool = fn_state.u__AMCR_CG1RZ;
        // D s_104_1: cast zx s_104_0 -> bv
        let s_104_1: Bits = Bits::new(s_104_0 as u128, 1u16);
        // C s_104_2: const #1u : u8
        let s_104_2: bool = true;
        // C s_104_3: cast zx s_104_2 -> bv
        let s_104_3: Bits = Bits::new(s_104_2 as u128, 1u16);
        // D s_104_4: cmp-eq s_104_1 s_104_3
        let s_104_4: bool = ((s_104_1) == (s_104_3));
        // D s_104_5: write-var gs#120797 <= s_104_4
        fn_state.gs_120797 = s_104_4;
        // N s_104_6: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #() : ()
        let s_105_0: () = ();
        // S s_105_1: call HaveAArch64(s_105_0)
        let s_105_1: bool = HaveAArch64(state, tracer, s_105_0);
        // S s_105_2: not s_105_1
        let s_105_2: bool = !s_105_1;
        // D s_105_3: write-var gs#120796 <= s_105_2
        fn_state.gs_120796 = s_105_2;
        // N s_105_4: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #32s : i
        let s_106_0: i128 = 32;
        // S s_106_1: call Zeros(s_106_0)
        let s_106_1: Bits = Zeros(state, tracer, s_106_0);
        // S s_106_2: cast reint s_106_1 -> u32
        let s_106_2: u32 = (s_106_1.value() as u32);
        // C s_106_3: const #32s : i
        let s_106_3: i128 = 32;
        // S s_106_4: call Zeros(s_106_3)
        let s_106_4: Bits = Zeros(state, tracer, s_106_3);
        // S s_106_5: cast reint s_106_4 -> u32
        let s_106_5: u32 = (s_106_4.value() as u32);
        // D s_106_6: create-product struct = ["s_106_2", "s_106_5"]
        let s_106_6: ProductType72d61775f103f7e0 = ProductType72d61775f103f7e0 {
            _0: s_106_2,
            _1: s_106_5,
        };
        // D s_106_7: write-var ga#201092 <= s_106_6
        fn_state.ga_201092 = s_106_6;
        // D s_106_8: read-var ga#201092.0:struct
        let s_106_8: u32 = fn_state.ga_201092._0;
        // D s_106_9: read-var ga#201092.1:struct
        let s_106_9: u32 = fn_state.ga_201092._1;
        // D s_106_10: read-var t2:i
        let s_106_10: i128 = fn_state.t2;
        // D s_106_11: call R_set(s_106_10, s_106_8)
        let s_106_11: () = R_set(state, tracer, s_106_10, s_106_8);
        // D s_106_12: read-var t:i
        let s_106_12: i128 = fn_state.t;
        // D s_106_13: call R_set(s_106_12, s_106_9)
        let s_106_13: () = R_set(state, tracer, s_106_12, s_106_9);
        // N s_106_14: return
        return;
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_107_0: read-var __AMCR_EL0_CG1RZ:u8
        let s_107_0: bool = fn_state.u__AMCR_EL0_CG1RZ;
        // D s_107_1: cast zx s_107_0 -> bv
        let s_107_1: Bits = Bits::new(s_107_0 as u128, 1u16);
        // C s_107_2: const #1u : u8
        let s_107_2: bool = true;
        // C s_107_3: cast zx s_107_2 -> bv
        let s_107_3: Bits = Bits::new(s_107_2 as u128, 1u16);
        // D s_107_4: cmp-eq s_107_1 s_107_3
        let s_107_4: bool = ((s_107_1) == (s_107_3));
        // D s_107_5: write-var gs#120795 <= s_107_4
        fn_state.gs_120795 = s_107_4;
        // N s_107_6: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #() : ()
        let s_108_0: () = ();
        // S s_108_1: call HaveAArch64(s_108_0)
        let s_108_1: bool = HaveAArch64(state, tracer, s_108_0);
        // D s_108_2: write-var gs#120794 <= s_108_1
        fn_state.gs_120794 = s_108_1;
        // N s_108_3: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #() : ()
        let s_109_0: () = ();
        // S s_109_1: call Halted(s_109_0)
        let s_109_1: bool = Halted(state, tracer, s_109_0);
        // N s_109_2: branch s_109_1 b114 b110
        if s_109_1 {
            return block_114(state, tracer, fn_state);
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
        // D s_110_1: write-var gs#120820 <= s_110_0
        fn_state.gs_120820 = s_110_0;
        // N s_110_2: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var gs#120820:u8
        let s_111_0: bool = fn_state.gs_120820;
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
        // C s_112_0: const #4u : u8
        let s_112_0: u8 = 4;
        // C s_112_1: cast zx s_112_0 -> bv
        let s_112_1: Bits = Bits::new(s_112_0 as u128, 8u16);
        // C s_112_2: cast zx s_112_1 -> i
        let s_112_2: i128 = (s_112_1.value() as i128);
        // C s_112_3: cast reint s_112_2 -> i64
        let s_112_3: i64 = (s_112_2 as i64);
        // C s_112_4: cast zx s_112_3 -> i
        let s_112_4: i128 = (i128::try_from(s_112_3).unwrap());
        // C s_112_5: const #424u : u32
        let s_112_5: u32 = 424;
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
        // N s_113_0: panic
        panic!("{:?}", ());
        // N s_113_1: return
        return;
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
        // D s_114_7: write-var gs#120820 <= s_114_6
        fn_state.gs_120820 = s_114_6;
        // N s_114_8: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_115_0: read-var __CPTR_EL3_TAM:u8
        let s_115_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_115_1: cast zx s_115_0 -> bv
        let s_115_1: Bits = Bits::new(s_115_0 as u128, 1u16);
        // C s_115_2: const #1u : u8
        let s_115_2: bool = true;
        // C s_115_3: cast zx s_115_2 -> bv
        let s_115_3: Bits = Bits::new(s_115_2 as u128, 1u16);
        // D s_115_4: cmp-eq s_115_1 s_115_3
        let s_115_4: bool = ((s_115_1) == (s_115_3));
        // D s_115_5: write-var gs#120793 <= s_115_4
        fn_state.gs_120793 = s_115_4;
        // N s_115_6: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #424u : u32
        let s_116_0: u32 = 424;
        // D s_116_1: read-reg s_116_0:u8
        let s_116_1: u8 = {
            let value = state.read_register::<u8>(s_116_0 as isize);
            tracer.read_register(s_116_0 as isize, value);
            value
        };
        // D s_116_2: call ELUsingAArch32(s_116_1)
        let s_116_2: bool = ELUsingAArch32(state, tracer, s_116_1);
        // D s_116_3: not s_116_2
        let s_116_3: bool = !s_116_2;
        // D s_116_4: write-var gs#120792 <= s_116_3
        fn_state.gs_120792 = s_116_3;
        // N s_116_5: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #4u : u8
        let s_117_0: u8 = 4;
        // C s_117_1: cast zx s_117_0 -> bv
        let s_117_1: Bits = Bits::new(s_117_0 as u128, 8u16);
        // C s_117_2: cast zx s_117_1 -> i
        let s_117_2: i128 = (s_117_1.value() as i128);
        // C s_117_3: cast reint s_117_2 -> i64
        let s_117_3: i64 = (s_117_2 as i64);
        // C s_117_4: cast zx s_117_3 -> i
        let s_117_4: i128 = (i128::try_from(s_117_3).unwrap());
        // S s_117_5: call AArch32_TakeHypTrapException(s_117_4)
        let s_117_5: () = AArch32_TakeHypTrapException(state, tracer, s_117_4);
        // N s_117_6: return
        return;
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_118_0: read-var __HCPTR_TAM:u8
        let s_118_0: bool = fn_state.u__HCPTR_TAM;
        // D s_118_1: cast zx s_118_0 -> bv
        let s_118_1: Bits = Bits::new(s_118_0 as u128, 1u16);
        // C s_118_2: const #1u : u8
        let s_118_2: bool = true;
        // C s_118_3: cast zx s_118_2 -> bv
        let s_118_3: Bits = Bits::new(s_118_2 as u128, 1u16);
        // D s_118_4: cmp-eq s_118_1 s_118_3
        let s_118_4: bool = ((s_118_1) == (s_118_3));
        // D s_118_5: write-var gs#120791 <= s_118_4
        fn_state.gs_120791 = s_118_4;
        // N s_118_6: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #432u : u32
        let s_119_0: u32 = 432;
        // D s_119_1: read-reg s_119_0:u8
        let s_119_1: u8 = {
            let value = state.read_register::<u8>(s_119_0 as isize);
            tracer.read_register(s_119_0 as isize, value);
            value
        };
        // D s_119_2: call ELUsingAArch32(s_119_1)
        let s_119_2: bool = ELUsingAArch32(state, tracer, s_119_1);
        // D s_119_3: write-var gs#120790 <= s_119_2
        fn_state.gs_120790 = s_119_2;
        // N s_119_4: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_120_0: const #4u : u8
        let s_120_0: u8 = 4;
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
        // D s_120_7: call AArch64_AArch32SystemAccessTrap(s_120_6, s_120_4)
        let s_120_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_120_6,
            s_120_4,
        );
        // N s_120_8: return
        return;
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_121_0: read-var __CPTR_EL2_TAM:u8
        let s_121_0: bool = fn_state.u__CPTR_EL2_TAM;
        // D s_121_1: cast zx s_121_0 -> bv
        let s_121_1: Bits = Bits::new(s_121_0 as u128, 1u16);
        // C s_121_2: const #1u : u8
        let s_121_2: bool = true;
        // C s_121_3: cast zx s_121_2 -> bv
        let s_121_3: Bits = Bits::new(s_121_2 as u128, 1u16);
        // D s_121_4: cmp-eq s_121_1 s_121_3
        let s_121_4: bool = ((s_121_1) == (s_121_3));
        // D s_121_5: write-var gs#120789 <= s_121_4
        fn_state.gs_120789 = s_121_4;
        // N s_121_6: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_122_0: const #432u : u32
        let s_122_0: u32 = 432;
        // D s_122_1: read-reg s_122_0:u8
        let s_122_1: u8 = {
            let value = state.read_register::<u8>(s_122_0 as isize);
            tracer.read_register(s_122_0 as isize, value);
            value
        };
        // D s_122_2: call ELUsingAArch32(s_122_1)
        let s_122_2: bool = ELUsingAArch32(state, tracer, s_122_1);
        // D s_122_3: not s_122_2
        let s_122_3: bool = !s_122_2;
        // D s_122_4: write-var gs#120788 <= s_122_3
        fn_state.gs_120788 = s_122_3;
        // N s_122_5: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_123_0: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #0u : u8
        let s_124_0: bool = false;
        // D s_124_1: write-var gs#120786 <= s_124_0
        fn_state.gs_120786 = s_124_0;
        // N s_124_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_125_0: const #432u : u32
        let s_125_0: u32 = 432;
        // D s_125_1: read-reg s_125_0:u8
        let s_125_1: u8 = {
            let value = state.read_register::<u8>(s_125_0 as isize);
            tracer.read_register(s_125_0 as isize, value);
            value
        };
        // D s_125_2: call ELUsingAArch32(s_125_1)
        let s_125_2: bool = ELUsingAArch32(state, tracer, s_125_1);
        // D s_125_3: write-var gs#120783 <= s_125_2
        fn_state.gs_120783 = s_125_2;
        // N s_125_4: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_126_0: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #0u : u8
        let s_127_0: bool = false;
        // D s_127_1: write-var gs#120781 <= s_127_0
        fn_state.gs_120781 = s_127_0;
        // N s_127_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_128_0: const #432u : u32
        let s_128_0: u32 = 432;
        // D s_128_1: read-reg s_128_0:u8
        let s_128_1: u8 = {
            let value = state.read_register::<u8>(s_128_0 as isize);
            tracer.read_register(s_128_0 as isize, value);
            value
        };
        // D s_128_2: call ELUsingAArch32(s_128_1)
        let s_128_2: bool = ELUsingAArch32(state, tracer, s_128_1);
        // D s_128_3: not s_128_2
        let s_128_3: bool = !s_128_2;
        // D s_128_4: write-var gs#120778 <= s_128_3
        fn_state.gs_120778 = s_128_3;
        // N s_128_5: jump b67
        return block_67(state, tracer, fn_state);
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
        // D s_130_0: read-var __CPTR_EL3_TAM:u8
        let s_130_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_130_1: cast zx s_130_0 -> bv
        let s_130_1: Bits = Bits::new(s_130_0 as u128, 1u16);
        // C s_130_2: const #1u : u8
        let s_130_2: bool = true;
        // C s_130_3: cast zx s_130_2 -> bv
        let s_130_3: Bits = Bits::new(s_130_2 as u128, 1u16);
        // D s_130_4: cmp-eq s_130_1 s_130_3
        let s_130_4: bool = ((s_130_1) == (s_130_3));
        // D s_130_5: write-var gs#120777 <= s_130_4
        fn_state.gs_120777 = s_130_4;
        // N s_130_6: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_131_0: const #424u : u32
        let s_131_0: u32 = 424;
        // D s_131_1: read-reg s_131_0:u8
        let s_131_1: u8 = {
            let value = state.read_register::<u8>(s_131_0 as isize);
            tracer.read_register(s_131_0 as isize, value);
            value
        };
        // D s_131_2: call ELUsingAArch32(s_131_1)
        let s_131_2: bool = ELUsingAArch32(state, tracer, s_131_1);
        // D s_131_3: not s_131_2
        let s_131_3: bool = !s_131_2;
        // D s_131_4: write-var gs#120776 <= s_131_3
        fn_state.gs_120776 = s_131_3;
        // N s_131_5: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_132_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_132_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_132_1: call __IMPDEF_boolean(s_132_0)
        let s_132_1: bool = u__IMPDEF_boolean(state, tracer, s_132_0);
        // D s_132_2: write-var gs#120775 <= s_132_1
        fn_state.gs_120775 = s_132_1;
        // N s_132_3: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_133_0: const #() : ()
        let s_133_0: () = ();
        // S s_133_1: call EDSCR_read(s_133_0)
        let s_133_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_133_0);
        // S s_133_2: call _get_EDSCR_Type_SDD(s_133_1)
        let s_133_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_133_1);
        // S s_133_3: cast zx s_133_2 -> bv
        let s_133_3: Bits = Bits::new(s_133_2 as u128, 1u16);
        // C s_133_4: const #1u : u8
        let s_133_4: bool = true;
        // C s_133_5: cast zx s_133_4 -> bv
        let s_133_5: Bits = Bits::new(s_133_4 as u128, 1u16);
        // S s_133_6: cmp-eq s_133_3 s_133_5
        let s_133_6: bool = ((s_133_3) == (s_133_5));
        // D s_133_7: write-var gs#120774 <= s_133_6
        fn_state.gs_120774 = s_133_6;
        // N s_133_8: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_134_0: const #424u : u32
        let s_134_0: u32 = 424;
        // D s_134_1: read-reg s_134_0:u8
        let s_134_1: u8 = {
            let value = state.read_register::<u8>(s_134_0 as isize);
            tracer.read_register(s_134_0 as isize, value);
            value
        };
        // C s_134_2: const #2u : u8
        let s_134_2: u8 = 2;
        // D s_134_3: cmp-lt s_134_1 s_134_2
        let s_134_3: bool = ((s_134_1) < (s_134_2));
        // D s_134_4: write-var gs#120773 <= s_134_3
        fn_state.gs_120773 = s_134_3;
        // N s_134_5: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_135_0: const #() : ()
        let s_135_0: () = ();
        // S s_135_1: call Halted(s_135_0)
        let s_135_1: bool = Halted(state, tracer, s_135_0);
        // N s_135_2: branch s_135_1 b266 b136
        if s_135_1 {
            return block_266(state, tracer, fn_state);
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
        // D s_136_1: write-var gs#120821 <= s_136_0
        fn_state.gs_120821 = s_136_0;
        // N s_136_2: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_137_0: read-var gs#120821:u8
        let s_137_0: bool = fn_state.gs_120821;
        // N s_137_1: branch s_137_0 b265 b138
        if s_137_0 {
            return block_265(state, tracer, fn_state);
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
        // D s_138_1: write-var gs#120822 <= s_138_0
        fn_state.gs_120822 = s_138_0;
        // N s_138_2: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_139_0: read-var gs#120822:u8
        let s_139_0: bool = fn_state.gs_120822;
        // N s_139_1: branch s_139_0 b264 b140
        if s_139_0 {
            return block_264(state, tracer, fn_state);
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
        // D s_140_1: write-var gs#120823 <= s_140_0
        fn_state.gs_120823 = s_140_0;
        // N s_140_2: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_141_0: read-var gs#120823:u8
        let s_141_0: bool = fn_state.gs_120823;
        // N s_141_1: branch s_141_0 b263 b142
        if s_141_0 {
            return block_263(state, tracer, fn_state);
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
        // D s_142_1: write-var gs#120824 <= s_142_0
        fn_state.gs_120824 = s_142_0;
        // N s_142_2: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_143_0: read-var gs#120824:u8
        let s_143_0: bool = fn_state.gs_120824;
        // N s_143_1: branch s_143_0 b262 b144
        if s_143_0 {
            return block_262(state, tracer, fn_state);
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
        // D s_144_1: write-var gs#120825 <= s_144_0
        fn_state.gs_120825 = s_144_0;
        // N s_144_2: jump b145
        return block_145(state, tracer, fn_state);
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_145_0: read-var gs#120825:u8
        let s_145_0: bool = fn_state.gs_120825;
        // N s_145_1: branch s_145_0 b261 b146
        if s_145_0 {
            return block_261(state, tracer, fn_state);
        } else {
            return block_146(state, tracer, fn_state);
        };
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_146_0: const #440u : u32
        let s_146_0: u32 = 440;
        // D s_146_1: read-reg s_146_0:u8
        let s_146_1: u8 = {
            let value = state.read_register::<u8>(s_146_0 as isize);
            tracer.read_register(s_146_0 as isize, value);
            value
        };
        // D s_146_2: call ELUsingAArch32(s_146_1)
        let s_146_2: bool = ELUsingAArch32(state, tracer, s_146_1);
        // D s_146_3: not s_146_2
        let s_146_3: bool = !s_146_2;
        // N s_146_4: branch s_146_3 b260 b147
        if s_146_3 {
            return block_260(state, tracer, fn_state);
        } else {
            return block_147(state, tracer, fn_state);
        };
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_147_0: const #0u : u8
        let s_147_0: bool = false;
        // D s_147_1: write-var gs#120826 <= s_147_0
        fn_state.gs_120826 = s_147_0;
        // N s_147_2: jump b148
        return block_148(state, tracer, fn_state);
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_148_0: read-var gs#120826:u8
        let s_148_0: bool = fn_state.gs_120826;
        // N s_148_1: branch s_148_0 b251 b149
        if s_148_0 {
            return block_251(state, tracer, fn_state);
        } else {
            return block_149(state, tracer, fn_state);
        };
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_149_0: const #440u : u32
        let s_149_0: u32 = 440;
        // D s_149_1: read-reg s_149_0:u8
        let s_149_1: u8 = {
            let value = state.read_register::<u8>(s_149_0 as isize);
            tracer.read_register(s_149_0 as isize, value);
            value
        };
        // D s_149_2: call ELUsingAArch32(s_149_1)
        let s_149_2: bool = ELUsingAArch32(state, tracer, s_149_1);
        // N s_149_3: branch s_149_2 b250 b150
        if s_149_2 {
            return block_250(state, tracer, fn_state);
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
        // D s_150_1: write-var gs#120827 <= s_150_0
        fn_state.gs_120827 = s_150_0;
        // N s_150_2: jump b151
        return block_151(state, tracer, fn_state);
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_151_0: read-var gs#120827:u8
        let s_151_0: bool = fn_state.gs_120827;
        // N s_151_1: branch s_151_0 b233 b152
        if s_151_0 {
            return block_233(state, tracer, fn_state);
        } else {
            return block_152(state, tracer, fn_state);
        };
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_152_0: const #() : ()
        let s_152_0: () = ();
        // S s_152_1: call EL2Enabled(s_152_0)
        let s_152_1: bool = EL2Enabled(state, tracer, s_152_0);
        // N s_152_2: branch s_152_1 b232 b153
        if s_152_1 {
            return block_232(state, tracer, fn_state);
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
        // D s_153_1: write-var gs#120828 <= s_153_0
        fn_state.gs_120828 = s_153_0;
        // N s_153_2: jump b154
        return block_154(state, tracer, fn_state);
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_154_0: read-var gs#120828:u8
        let s_154_0: bool = fn_state.gs_120828;
        // N s_154_1: branch s_154_0 b231 b155
        if s_154_0 {
            return block_231(state, tracer, fn_state);
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
        // D s_155_1: write-var gs#120829 <= s_155_0
        fn_state.gs_120829 = s_155_0;
        // N s_155_2: jump b156
        return block_156(state, tracer, fn_state);
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_156_0: read-var gs#120829:u8
        let s_156_0: bool = fn_state.gs_120829;
        // N s_156_1: branch s_156_0 b230 b157
        if s_156_0 {
            return block_230(state, tracer, fn_state);
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
        // D s_157_1: write-var gs#120832 <= s_157_0
        fn_state.gs_120832 = s_157_0;
        // N s_157_2: jump b158
        return block_158(state, tracer, fn_state);
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_158_0: read-var gs#120832:u8
        let s_158_0: bool = fn_state.gs_120832;
        // N s_158_1: branch s_158_0 b229 b159
        if s_158_0 {
            return block_229(state, tracer, fn_state);
        } else {
            return block_159(state, tracer, fn_state);
        };
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_159_0: jump b160
        return block_160(state, tracer, fn_state);
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
        // N s_160_2: branch s_160_1 b228 b161
        if s_160_1 {
            return block_228(state, tracer, fn_state);
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
        // D s_161_1: write-var gs#120834 <= s_161_0
        fn_state.gs_120834 = s_161_0;
        // N s_161_2: jump b162
        return block_162(state, tracer, fn_state);
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_162_0: read-var gs#120834:u8
        let s_162_0: bool = fn_state.gs_120834;
        // N s_162_1: branch s_162_0 b227 b163
        if s_162_0 {
            return block_227(state, tracer, fn_state);
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
        // D s_163_1: write-var gs#120837 <= s_163_0
        fn_state.gs_120837 = s_163_0;
        // N s_163_2: jump b164
        return block_164(state, tracer, fn_state);
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_164_0: read-var gs#120837:u8
        let s_164_0: bool = fn_state.gs_120837;
        // N s_164_1: branch s_164_0 b226 b165
        if s_164_0 {
            return block_226(state, tracer, fn_state);
        } else {
            return block_165(state, tracer, fn_state);
        };
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_165_0: jump b166
        return block_166(state, tracer, fn_state);
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
        // N s_166_2: branch s_166_1 b225 b167
        if s_166_1 {
            return block_225(state, tracer, fn_state);
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
        // D s_167_1: write-var gs#120839 <= s_167_0
        fn_state.gs_120839 = s_167_0;
        // N s_167_2: jump b168
        return block_168(state, tracer, fn_state);
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_168_0: read-var gs#120839:u8
        let s_168_0: bool = fn_state.gs_120839;
        // N s_168_1: branch s_168_0 b224 b169
        if s_168_0 {
            return block_224(state, tracer, fn_state);
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
        // D s_169_1: write-var gs#120840 <= s_169_0
        fn_state.gs_120840 = s_169_0;
        // N s_169_2: jump b170
        return block_170(state, tracer, fn_state);
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_170_0: read-var gs#120840:u8
        let s_170_0: bool = fn_state.gs_120840;
        // N s_170_1: branch s_170_0 b223 b171
        if s_170_0 {
            return block_223(state, tracer, fn_state);
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
        // N s_171_2: branch s_171_1 b222 b172
        if s_171_1 {
            return block_222(state, tracer, fn_state);
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
        // D s_172_1: write-var gs#120841 <= s_172_0
        fn_state.gs_120841 = s_172_0;
        // N s_172_2: jump b173
        return block_173(state, tracer, fn_state);
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_173_0: read-var gs#120841:u8
        let s_173_0: bool = fn_state.gs_120841;
        // N s_173_1: branch s_173_0 b221 b174
        if s_173_0 {
            return block_221(state, tracer, fn_state);
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
        // D s_174_1: write-var gs#120842 <= s_174_0
        fn_state.gs_120842 = s_174_0;
        // N s_174_2: jump b175
        return block_175(state, tracer, fn_state);
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_175_0: read-var gs#120842:u8
        let s_175_0: bool = fn_state.gs_120842;
        // N s_175_1: branch s_175_0 b220 b176
        if s_175_0 {
            return block_220(state, tracer, fn_state);
        } else {
            return block_176(state, tracer, fn_state);
        };
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_176_0: const #() : ()
        let s_176_0: () = ();
        // S s_176_1: call EL2Enabled(s_176_0)
        let s_176_1: bool = EL2Enabled(state, tracer, s_176_0);
        // N s_176_2: branch s_176_1 b219 b177
        if s_176_1 {
            return block_219(state, tracer, fn_state);
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
        // D s_177_1: write-var gs#120843 <= s_177_0
        fn_state.gs_120843 = s_177_0;
        // N s_177_2: jump b178
        return block_178(state, tracer, fn_state);
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_178_0: read-var gs#120843:u8
        let s_178_0: bool = fn_state.gs_120843;
        // N s_178_1: branch s_178_0 b218 b179
        if s_178_0 {
            return block_218(state, tracer, fn_state);
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
        // D s_179_1: write-var gs#120844 <= s_179_0
        fn_state.gs_120844 = s_179_0;
        // N s_179_2: jump b180
        return block_180(state, tracer, fn_state);
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_180_0: read-var gs#120844:u8
        let s_180_0: bool = fn_state.gs_120844;
        // N s_180_1: branch s_180_0 b217 b181
        if s_180_0 {
            return block_217(state, tracer, fn_state);
        } else {
            return block_181(state, tracer, fn_state);
        };
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_181_0: const #0u : u8
        let s_181_0: bool = false;
        // D s_181_1: write-var gs#120845 <= s_181_0
        fn_state.gs_120845 = s_181_0;
        // N s_181_2: jump b182
        return block_182(state, tracer, fn_state);
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_182_0: read-var gs#120845:u8
        let s_182_0: bool = fn_state.gs_120845;
        // N s_182_1: branch s_182_0 b213 b183
        if s_182_0 {
            return block_213(state, tracer, fn_state);
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
        // D s_183_1: write-var gs#120847 <= s_183_0
        fn_state.gs_120847 = s_183_0;
        // N s_183_2: jump b184
        return block_184(state, tracer, fn_state);
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_184_0: read-var gs#120847:u8
        let s_184_0: bool = fn_state.gs_120847;
        // N s_184_1: branch s_184_0 b212 b185
        if s_184_0 {
            return block_212(state, tracer, fn_state);
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
        // D s_185_1: write-var gs#120848 <= s_185_0
        fn_state.gs_120848 = s_185_0;
        // N s_185_2: jump b186
        return block_186(state, tracer, fn_state);
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_186_0: read-var gs#120848:u8
        let s_186_0: bool = fn_state.gs_120848;
        // N s_186_1: branch s_186_0 b211 b187
        if s_186_0 {
            return block_211(state, tracer, fn_state);
        } else {
            return block_187(state, tracer, fn_state);
        };
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_187_0: const #424u : u32
        let s_187_0: u32 = 424;
        // D s_187_1: read-reg s_187_0:u8
        let s_187_1: u8 = {
            let value = state.read_register::<u8>(s_187_0 as isize);
            tracer.read_register(s_187_0 as isize, value);
            value
        };
        // C s_187_2: const #2u : u8
        let s_187_2: u8 = 2;
        // D s_187_3: cmp-lt s_187_1 s_187_2
        let s_187_3: bool = ((s_187_1) < (s_187_2));
        // N s_187_4: branch s_187_3 b210 b188
        if s_187_3 {
            return block_210(state, tracer, fn_state);
        } else {
            return block_188(state, tracer, fn_state);
        };
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_188_0: const #0u : u8
        let s_188_0: bool = false;
        // D s_188_1: write-var gs#120849 <= s_188_0
        fn_state.gs_120849 = s_188_0;
        // N s_188_2: jump b189
        return block_189(state, tracer, fn_state);
    }
    fn block_189<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_189_0: read-var gs#120849:u8
        let s_189_0: bool = fn_state.gs_120849;
        // N s_189_1: branch s_189_0 b209 b190
        if s_189_0 {
            return block_209(state, tracer, fn_state);
        } else {
            return block_190(state, tracer, fn_state);
        };
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_190_0: const #0u : u8
        let s_190_0: bool = false;
        // D s_190_1: write-var gs#120850 <= s_190_0
        fn_state.gs_120850 = s_190_0;
        // N s_190_2: jump b191
        return block_191(state, tracer, fn_state);
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_191_0: read-var gs#120850:u8
        let s_191_0: bool = fn_state.gs_120850;
        // N s_191_1: branch s_191_0 b203 b192
        if s_191_0 {
            return block_203(state, tracer, fn_state);
        } else {
            return block_192(state, tracer, fn_state);
        };
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_192_0: const #() : ()
        let s_192_0: () = ();
        // S s_192_1: call HaveAArch64(s_192_0)
        let s_192_1: bool = HaveAArch64(state, tracer, s_192_0);
        // N s_192_2: branch s_192_1 b202 b193
        if s_192_1 {
            return block_202(state, tracer, fn_state);
        } else {
            return block_193(state, tracer, fn_state);
        };
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_193_0: const #0u : u8
        let s_193_0: bool = false;
        // D s_193_1: write-var gs#120851 <= s_193_0
        fn_state.gs_120851 = s_193_0;
        // N s_193_2: jump b194
        return block_194(state, tracer, fn_state);
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_194_0: read-var gs#120851:u8
        let s_194_0: bool = fn_state.gs_120851;
        // N s_194_1: branch s_194_0 b201 b195
        if s_194_0 {
            return block_201(state, tracer, fn_state);
        } else {
            return block_195(state, tracer, fn_state);
        };
    }
    fn block_195<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_195_0: const #() : ()
        let s_195_0: () = ();
        // S s_195_1: call HaveAArch64(s_195_0)
        let s_195_1: bool = HaveAArch64(state, tracer, s_195_0);
        // S s_195_2: not s_195_1
        let s_195_2: bool = !s_195_1;
        // N s_195_3: branch s_195_2 b200 b196
        if s_195_2 {
            return block_200(state, tracer, fn_state);
        } else {
            return block_196(state, tracer, fn_state);
        };
    }
    fn block_196<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_196_0: const #0u : u8
        let s_196_0: bool = false;
        // D s_196_1: write-var gs#120852 <= s_196_0
        fn_state.gs_120852 = s_196_0;
        // N s_196_2: jump b197
        return block_197(state, tracer, fn_state);
    }
    fn block_197<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_197_0: read-var gs#120852:u8
        let s_197_0: bool = fn_state.gs_120852;
        // N s_197_1: branch s_197_0 b199 b198
        if s_197_0 {
            return block_199(state, tracer, fn_state);
        } else {
            return block_198(state, tracer, fn_state);
        };
    }
    fn block_198<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_198_0: const #2s : i64
        let s_198_0: i64 = 2;
        // S s_198_1: call AMEVCNTR1_read(s_198_0)
        let s_198_1: ProductType5c790c8ef59cc8b2 = AMEVCNTR1_read(
            state,
            tracer,
            s_198_0,
        );
        // D s_198_2: write-var ga#201036 <= s_198_1
        fn_state.ga_201036 = s_198_1;
        // D s_198_3: read-var ga#201036.0:struct
        let s_198_3: u64 = fn_state.ga_201036._0;
        // C s_198_4: const #32s : i
        let s_198_4: i128 = 32;
        // D s_198_5: cast zx s_198_3 -> bv
        let s_198_5: Bits = Bits::new(s_198_3 as u128, 64u16);
        // C s_198_6: const #1s : i64
        let s_198_6: i64 = 1;
        // C s_198_7: cast zx s_198_6 -> i
        let s_198_7: i128 = (i128::try_from(s_198_6).unwrap());
        // C s_198_8: const #31s : i
        let s_198_8: i128 = 31;
        // C s_198_9: add s_198_8 s_198_7
        let s_198_9: i128 = (s_198_8 + s_198_7);
        // D s_198_10: bit-extract s_198_5 s_198_4 s_198_9
        let s_198_10: Bits = (Bits::new(
            ((s_198_5) >> (s_198_4)).value(),
            u16::try_from(s_198_9).unwrap(),
        ));
        // D s_198_11: cast reint s_198_10 -> u32
        let s_198_11: u32 = (s_198_10.value() as u32);
        // C s_198_12: const #2s : i64
        let s_198_12: i64 = 2;
        // S s_198_13: call AMEVCNTR1_read(s_198_12)
        let s_198_13: ProductType5c790c8ef59cc8b2 = AMEVCNTR1_read(
            state,
            tracer,
            s_198_12,
        );
        // D s_198_14: write-var ga#201038 <= s_198_13
        fn_state.ga_201038 = s_198_13;
        // D s_198_15: read-var ga#201038.0:struct
        let s_198_15: u64 = fn_state.ga_201038._0;
        // C s_198_16: const #0s : i
        let s_198_16: i128 = 0;
        // D s_198_17: cast zx s_198_15 -> bv
        let s_198_17: Bits = Bits::new(s_198_15 as u128, 64u16);
        // C s_198_18: const #1s : i64
        let s_198_18: i64 = 1;
        // C s_198_19: cast zx s_198_18 -> i
        let s_198_19: i128 = (i128::try_from(s_198_18).unwrap());
        // C s_198_20: const #31s : i
        let s_198_20: i128 = 31;
        // C s_198_21: add s_198_20 s_198_19
        let s_198_21: i128 = (s_198_20 + s_198_19);
        // D s_198_22: bit-extract s_198_17 s_198_16 s_198_21
        let s_198_22: Bits = (Bits::new(
            ((s_198_17) >> (s_198_16)).value(),
            u16::try_from(s_198_21).unwrap(),
        ));
        // D s_198_23: cast reint s_198_22 -> u32
        let s_198_23: u32 = (s_198_22.value() as u32);
        // D s_198_24: create-product struct = ["s_198_11", "s_198_23"]
        let s_198_24: ProductType72d61775f103f7e0 = ProductType72d61775f103f7e0 {
            _0: s_198_11,
            _1: s_198_23,
        };
        // D s_198_25: write-var ga#201042 <= s_198_24
        fn_state.ga_201042 = s_198_24;
        // D s_198_26: read-var ga#201042.0:struct
        let s_198_26: u32 = fn_state.ga_201042._0;
        // D s_198_27: read-var ga#201042.1:struct
        let s_198_27: u32 = fn_state.ga_201042._1;
        // D s_198_28: read-var t2:i
        let s_198_28: i128 = fn_state.t2;
        // D s_198_29: call R_set(s_198_28, s_198_26)
        let s_198_29: () = R_set(state, tracer, s_198_28, s_198_26);
        // D s_198_30: read-var t:i
        let s_198_30: i128 = fn_state.t;
        // D s_198_31: call R_set(s_198_30, s_198_27)
        let s_198_31: () = R_set(state, tracer, s_198_30, s_198_27);
        // N s_198_32: return
        return;
    }
    fn block_199<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_199_0: const #32s : i
        let s_199_0: i128 = 32;
        // S s_199_1: call Zeros(s_199_0)
        let s_199_1: Bits = Zeros(state, tracer, s_199_0);
        // S s_199_2: cast reint s_199_1 -> u32
        let s_199_2: u32 = (s_199_1.value() as u32);
        // C s_199_3: const #32s : i
        let s_199_3: i128 = 32;
        // S s_199_4: call Zeros(s_199_3)
        let s_199_4: Bits = Zeros(state, tracer, s_199_3);
        // S s_199_5: cast reint s_199_4 -> u32
        let s_199_5: u32 = (s_199_4.value() as u32);
        // D s_199_6: create-product struct = ["s_199_2", "s_199_5"]
        let s_199_6: ProductType72d61775f103f7e0 = ProductType72d61775f103f7e0 {
            _0: s_199_2,
            _1: s_199_5,
        };
        // D s_199_7: write-var ga#201035 <= s_199_6
        fn_state.ga_201035 = s_199_6;
        // D s_199_8: read-var ga#201035.0:struct
        let s_199_8: u32 = fn_state.ga_201035._0;
        // D s_199_9: read-var ga#201035.1:struct
        let s_199_9: u32 = fn_state.ga_201035._1;
        // D s_199_10: read-var t2:i
        let s_199_10: i128 = fn_state.t2;
        // D s_199_11: call R_set(s_199_10, s_199_8)
        let s_199_11: () = R_set(state, tracer, s_199_10, s_199_8);
        // D s_199_12: read-var t:i
        let s_199_12: i128 = fn_state.t;
        // D s_199_13: call R_set(s_199_12, s_199_9)
        let s_199_13: () = R_set(state, tracer, s_199_12, s_199_9);
        // N s_199_14: return
        return;
    }
    fn block_200<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_200_0: read-var __AMCR_CG1RZ:u8
        let s_200_0: bool = fn_state.u__AMCR_CG1RZ;
        // D s_200_1: cast zx s_200_0 -> bv
        let s_200_1: Bits = Bits::new(s_200_0 as u128, 1u16);
        // C s_200_2: const #1u : u8
        let s_200_2: bool = true;
        // C s_200_3: cast zx s_200_2 -> bv
        let s_200_3: Bits = Bits::new(s_200_2 as u128, 1u16);
        // D s_200_4: cmp-eq s_200_1 s_200_3
        let s_200_4: bool = ((s_200_1) == (s_200_3));
        // D s_200_5: write-var gs#120852 <= s_200_4
        fn_state.gs_120852 = s_200_4;
        // N s_200_6: jump b197
        return block_197(state, tracer, fn_state);
    }
    fn block_201<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_201_0: const #32s : i
        let s_201_0: i128 = 32;
        // S s_201_1: call Zeros(s_201_0)
        let s_201_1: Bits = Zeros(state, tracer, s_201_0);
        // S s_201_2: cast reint s_201_1 -> u32
        let s_201_2: u32 = (s_201_1.value() as u32);
        // C s_201_3: const #32s : i
        let s_201_3: i128 = 32;
        // S s_201_4: call Zeros(s_201_3)
        let s_201_4: Bits = Zeros(state, tracer, s_201_3);
        // S s_201_5: cast reint s_201_4 -> u32
        let s_201_5: u32 = (s_201_4.value() as u32);
        // D s_201_6: create-product struct = ["s_201_2", "s_201_5"]
        let s_201_6: ProductType72d61775f103f7e0 = ProductType72d61775f103f7e0 {
            _0: s_201_2,
            _1: s_201_5,
        };
        // D s_201_7: write-var ga#201029 <= s_201_6
        fn_state.ga_201029 = s_201_6;
        // D s_201_8: read-var ga#201029.0:struct
        let s_201_8: u32 = fn_state.ga_201029._0;
        // D s_201_9: read-var ga#201029.1:struct
        let s_201_9: u32 = fn_state.ga_201029._1;
        // D s_201_10: read-var t2:i
        let s_201_10: i128 = fn_state.t2;
        // D s_201_11: call R_set(s_201_10, s_201_8)
        let s_201_11: () = R_set(state, tracer, s_201_10, s_201_8);
        // D s_201_12: read-var t:i
        let s_201_12: i128 = fn_state.t;
        // D s_201_13: call R_set(s_201_12, s_201_9)
        let s_201_13: () = R_set(state, tracer, s_201_12, s_201_9);
        // N s_201_14: return
        return;
    }
    fn block_202<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_202_0: read-var __AMCR_EL0_CG1RZ:u8
        let s_202_0: bool = fn_state.u__AMCR_EL0_CG1RZ;
        // D s_202_1: cast zx s_202_0 -> bv
        let s_202_1: Bits = Bits::new(s_202_0 as u128, 1u16);
        // C s_202_2: const #1u : u8
        let s_202_2: bool = true;
        // C s_202_3: cast zx s_202_2 -> bv
        let s_202_3: Bits = Bits::new(s_202_2 as u128, 1u16);
        // D s_202_4: cmp-eq s_202_1 s_202_3
        let s_202_4: bool = ((s_202_1) == (s_202_3));
        // D s_202_5: write-var gs#120851 <= s_202_4
        fn_state.gs_120851 = s_202_4;
        // N s_202_6: jump b194
        return block_194(state, tracer, fn_state);
    }
    fn block_203<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_203_0: const #() : ()
        let s_203_0: () = ();
        // S s_203_1: call Halted(s_203_0)
        let s_203_1: bool = Halted(state, tracer, s_203_0);
        // N s_203_2: branch s_203_1 b208 b204
        if s_203_1 {
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
        // D s_204_1: write-var gs#120875 <= s_204_0
        fn_state.gs_120875 = s_204_0;
        // N s_204_2: jump b205
        return block_205(state, tracer, fn_state);
    }
    fn block_205<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_205_0: read-var gs#120875:u8
        let s_205_0: bool = fn_state.gs_120875;
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
        // C s_206_0: const #4u : u8
        let s_206_0: u8 = 4;
        // C s_206_1: cast zx s_206_0 -> bv
        let s_206_1: Bits = Bits::new(s_206_0 as u128, 8u16);
        // C s_206_2: cast zx s_206_1 -> i
        let s_206_2: i128 = (s_206_1.value() as i128);
        // C s_206_3: cast reint s_206_2 -> i64
        let s_206_3: i64 = (s_206_2 as i64);
        // C s_206_4: cast zx s_206_3 -> i
        let s_206_4: i128 = (i128::try_from(s_206_3).unwrap());
        // C s_206_5: const #424u : u32
        let s_206_5: u32 = 424;
        // D s_206_6: read-reg s_206_5:u8
        let s_206_6: u8 = {
            let value = state.read_register::<u8>(s_206_5 as isize);
            tracer.read_register(s_206_5 as isize, value);
            value
        };
        // D s_206_7: call AArch64_AArch32SystemAccessTrap(s_206_6, s_206_4)
        let s_206_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_206_6,
            s_206_4,
        );
        // N s_206_8: return
        return;
    }
    fn block_207<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_207_0: panic
        panic!("{:?}", ());
        // N s_207_1: return
        return;
    }
    fn block_208<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_208_0: const #() : ()
        let s_208_0: () = ();
        // S s_208_1: call EDSCR_read(s_208_0)
        let s_208_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_208_0);
        // S s_208_2: call _get_EDSCR_Type_SDD(s_208_1)
        let s_208_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_208_1);
        // S s_208_3: cast zx s_208_2 -> bv
        let s_208_3: Bits = Bits::new(s_208_2 as u128, 1u16);
        // C s_208_4: const #1u : u8
        let s_208_4: bool = true;
        // C s_208_5: cast zx s_208_4 -> bv
        let s_208_5: Bits = Bits::new(s_208_4 as u128, 1u16);
        // S s_208_6: cmp-eq s_208_3 s_208_5
        let s_208_6: bool = ((s_208_3) == (s_208_5));
        // D s_208_7: write-var gs#120875 <= s_208_6
        fn_state.gs_120875 = s_208_6;
        // N s_208_8: jump b205
        return block_205(state, tracer, fn_state);
    }
    fn block_209<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_209_0: read-var __CPTR_EL3_TAM:u8
        let s_209_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_209_1: cast zx s_209_0 -> bv
        let s_209_1: Bits = Bits::new(s_209_0 as u128, 1u16);
        // C s_209_2: const #1u : u8
        let s_209_2: bool = true;
        // C s_209_3: cast zx s_209_2 -> bv
        let s_209_3: Bits = Bits::new(s_209_2 as u128, 1u16);
        // D s_209_4: cmp-eq s_209_1 s_209_3
        let s_209_4: bool = ((s_209_1) == (s_209_3));
        // D s_209_5: write-var gs#120850 <= s_209_4
        fn_state.gs_120850 = s_209_4;
        // N s_209_6: jump b191
        return block_191(state, tracer, fn_state);
    }
    fn block_210<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_210_0: const #424u : u32
        let s_210_0: u32 = 424;
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
        // D s_210_4: write-var gs#120849 <= s_210_3
        fn_state.gs_120849 = s_210_3;
        // N s_210_5: jump b189
        return block_189(state, tracer, fn_state);
    }
    fn block_211<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_211_0: const #4u : u8
        let s_211_0: u8 = 4;
        // C s_211_1: cast zx s_211_0 -> bv
        let s_211_1: Bits = Bits::new(s_211_0 as u128, 8u16);
        // C s_211_2: cast zx s_211_1 -> i
        let s_211_2: i128 = (s_211_1.value() as i128);
        // C s_211_3: cast reint s_211_2 -> i64
        let s_211_3: i64 = (s_211_2 as i64);
        // C s_211_4: cast zx s_211_3 -> i
        let s_211_4: i128 = (i128::try_from(s_211_3).unwrap());
        // C s_211_5: const #432u : u32
        let s_211_5: u32 = 432;
        // D s_211_6: read-reg s_211_5:u8
        let s_211_6: u8 = {
            let value = state.read_register::<u8>(s_211_5 as isize);
            tracer.read_register(s_211_5 as isize, value);
            value
        };
        // D s_211_7: call AArch64_AArch32SystemAccessTrap(s_211_6, s_211_4)
        let s_211_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_211_6,
            s_211_4,
        );
        // N s_211_8: return
        return;
    }
    fn block_212<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_212_0: read-var __HAFGRTR_EL2_AMEVCNTR12_EL0:u8
        let s_212_0: bool = fn_state.u__HAFGRTR_EL2_AMEVCNTR12_EL0;
        // D s_212_1: cast zx s_212_0 -> bv
        let s_212_1: Bits = Bits::new(s_212_0 as u128, 1u16);
        // C s_212_2: const #1u : u8
        let s_212_2: bool = true;
        // C s_212_3: cast zx s_212_2 -> bv
        let s_212_3: Bits = Bits::new(s_212_2 as u128, 1u16);
        // D s_212_4: cmp-eq s_212_1 s_212_3
        let s_212_4: bool = ((s_212_1) == (s_212_3));
        // D s_212_5: write-var gs#120848 <= s_212_4
        fn_state.gs_120848 = s_212_4;
        // N s_212_6: jump b186
        return block_186(state, tracer, fn_state);
    }
    fn block_213<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_213_0: const #424u : u32
        let s_213_0: u32 = 424;
        // D s_213_1: read-reg s_213_0:u8
        let s_213_1: u8 = {
            let value = state.read_register::<u8>(s_213_0 as isize);
            tracer.read_register(s_213_0 as isize, value);
            value
        };
        // C s_213_2: const #2u : u8
        let s_213_2: u8 = 2;
        // D s_213_3: cmp-lt s_213_1 s_213_2
        let s_213_3: bool = ((s_213_1) < (s_213_2));
        // D s_213_4: not s_213_3
        let s_213_4: bool = !s_213_3;
        // N s_213_5: branch s_213_4 b216 b214
        if s_213_4 {
            return block_216(state, tracer, fn_state);
        } else {
            return block_214(state, tracer, fn_state);
        };
    }
    fn block_214<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_214_0: read-var __SCR_EL3_FGTEn:u8
        let s_214_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_214_1: cast zx s_214_0 -> bv
        let s_214_1: Bits = Bits::new(s_214_0 as u128, 1u16);
        // C s_214_2: const #1u : u8
        let s_214_2: bool = true;
        // C s_214_3: cast zx s_214_2 -> bv
        let s_214_3: Bits = Bits::new(s_214_2 as u128, 1u16);
        // D s_214_4: cmp-eq s_214_1 s_214_3
        let s_214_4: bool = ((s_214_1) == (s_214_3));
        // D s_214_5: write-var gs#120846 <= s_214_4
        fn_state.gs_120846 = s_214_4;
        // N s_214_6: jump b215
        return block_215(state, tracer, fn_state);
    }
    fn block_215<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_215_0: read-var gs#120846:u8
        let s_215_0: bool = fn_state.gs_120846;
        // D s_215_1: write-var gs#120847 <= s_215_0
        fn_state.gs_120847 = s_215_0;
        // N s_215_2: jump b184
        return block_184(state, tracer, fn_state);
    }
    fn block_216<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_216_0: const #1u : u8
        let s_216_0: bool = true;
        // D s_216_1: write-var gs#120846 <= s_216_0
        fn_state.gs_120846 = s_216_0;
        // N s_216_2: jump b215
        return block_215(state, tracer, fn_state);
    }
    fn block_217<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_217_0: const #146u : u32
        let s_217_0: u32 = 146;
        // S s_217_1: call IsFeatureImplemented(s_217_0)
        let s_217_1: bool = IsFeatureImplemented(state, tracer, s_217_0);
        // D s_217_2: write-var gs#120845 <= s_217_1
        fn_state.gs_120845 = s_217_1;
        // N s_217_3: jump b182
        return block_182(state, tracer, fn_state);
    }
    fn block_218<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_218_0: const #102552u : u32
        let s_218_0: u32 = 102552;
        // D s_218_1: read-reg s_218_0:struct
        let s_218_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_218_0 as isize);
            tracer.read_register(s_218_0 as isize, value);
            value
        };
        // D s_218_2: call _get_HCR_EL2_Type_E2H(s_218_1)
        let s_218_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_218_1);
        // C s_218_3: const #102552u : u32
        let s_218_3: u32 = 102552;
        // D s_218_4: read-reg s_218_3:struct
        let s_218_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_218_3 as isize);
            tracer.read_register(s_218_3 as isize, value);
            value
        };
        // D s_218_5: call _get_HCR_EL2_Type_TGE(s_218_4)
        let s_218_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_218_4);
        // D s_218_6: cast zx s_218_2 -> bv
        let s_218_6: Bits = Bits::new(s_218_2 as u128, 1u16);
        // D s_218_7: cast zx s_218_5 -> bv
        let s_218_7: Bits = Bits::new(s_218_5 as u128, 1u16);
        // D s_218_8: cast reint s_218_6 -> u128
        let s_218_8: u128 = (s_218_6.value() as u128);
        // D s_218_9: size-of s_218_6
        let s_218_9: u16 = s_218_6.length();
        // D s_218_10: cast reint s_218_7 -> u128
        let s_218_10: u128 = (s_218_7.value() as u128);
        // D s_218_11: size-of s_218_7
        let s_218_11: u16 = s_218_7.length();
        // D s_218_12: lsl s_218_8 s_218_11
        let s_218_12: u128 = s_218_8 << s_218_11;
        // D s_218_13: or s_218_12 s_218_10
        let s_218_13: u128 = ((s_218_12) | (s_218_10));
        // D s_218_14: add s_218_9 s_218_11
        let s_218_14: u16 = (s_218_9 + s_218_11);
        // D s_218_15: create-bits s_218_13 s_218_14
        let s_218_15: Bits = Bits::new(s_218_13, s_218_14);
        // D s_218_16: cast reint s_218_15 -> u8
        let s_218_16: u8 = (s_218_15.value() as u8);
        // D s_218_17: cast zx s_218_16 -> bv
        let s_218_17: Bits = Bits::new(s_218_16 as u128, 2u16);
        // C s_218_18: const #3u : u8
        let s_218_18: u8 = 3;
        // C s_218_19: cast zx s_218_18 -> bv
        let s_218_19: Bits = Bits::new(s_218_18 as u128, 2u16);
        // D s_218_20: cmp-ne s_218_17 s_218_19
        let s_218_20: bool = ((s_218_17) != (s_218_19));
        // D s_218_21: write-var gs#120844 <= s_218_20
        fn_state.gs_120844 = s_218_20;
        // N s_218_22: jump b180
        return block_180(state, tracer, fn_state);
    }
    fn block_219<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_219_0: const #440u : u32
        let s_219_0: u32 = 440;
        // D s_219_1: read-reg s_219_0:u8
        let s_219_1: u8 = {
            let value = state.read_register::<u8>(s_219_0 as isize);
            tracer.read_register(s_219_0 as isize, value);
            value
        };
        // D s_219_2: call ELUsingAArch32(s_219_1)
        let s_219_2: bool = ELUsingAArch32(state, tracer, s_219_1);
        // D s_219_3: not s_219_2
        let s_219_3: bool = !s_219_2;
        // D s_219_4: write-var gs#120843 <= s_219_3
        fn_state.gs_120843 = s_219_3;
        // N s_219_5: jump b178
        return block_178(state, tracer, fn_state);
    }
    fn block_220<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_220_0: const #4u : u8
        let s_220_0: u8 = 4;
        // C s_220_1: cast zx s_220_0 -> bv
        let s_220_1: Bits = Bits::new(s_220_0 as u128, 8u16);
        // C s_220_2: cast zx s_220_1 -> i
        let s_220_2: i128 = (s_220_1.value() as i128);
        // C s_220_3: cast reint s_220_2 -> i64
        let s_220_3: i64 = (s_220_2 as i64);
        // C s_220_4: cast zx s_220_3 -> i
        let s_220_4: i128 = (i128::try_from(s_220_3).unwrap());
        // S s_220_5: call AArch32_TakeHypTrapException(s_220_4)
        let s_220_5: () = AArch32_TakeHypTrapException(state, tracer, s_220_4);
        // N s_220_6: return
        return;
    }
    fn block_221<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_221_0: read-var __HCPTR_TAM:u8
        let s_221_0: bool = fn_state.u__HCPTR_TAM;
        // D s_221_1: cast zx s_221_0 -> bv
        let s_221_1: Bits = Bits::new(s_221_0 as u128, 1u16);
        // C s_221_2: const #1u : u8
        let s_221_2: bool = true;
        // C s_221_3: cast zx s_221_2 -> bv
        let s_221_3: Bits = Bits::new(s_221_2 as u128, 1u16);
        // D s_221_4: cmp-eq s_221_1 s_221_3
        let s_221_4: bool = ((s_221_1) == (s_221_3));
        // D s_221_5: write-var gs#120842 <= s_221_4
        fn_state.gs_120842 = s_221_4;
        // N s_221_6: jump b175
        return block_175(state, tracer, fn_state);
    }
    fn block_222<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_222_0: const #432u : u32
        let s_222_0: u32 = 432;
        // D s_222_1: read-reg s_222_0:u8
        let s_222_1: u8 = {
            let value = state.read_register::<u8>(s_222_0 as isize);
            tracer.read_register(s_222_0 as isize, value);
            value
        };
        // D s_222_2: call ELUsingAArch32(s_222_1)
        let s_222_2: bool = ELUsingAArch32(state, tracer, s_222_1);
        // D s_222_3: write-var gs#120841 <= s_222_2
        fn_state.gs_120841 = s_222_2;
        // N s_222_4: jump b173
        return block_173(state, tracer, fn_state);
    }
    fn block_223<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_223_0: const #4u : u8
        let s_223_0: u8 = 4;
        // C s_223_1: cast zx s_223_0 -> bv
        let s_223_1: Bits = Bits::new(s_223_0 as u128, 8u16);
        // C s_223_2: cast zx s_223_1 -> i
        let s_223_2: i128 = (s_223_1.value() as i128);
        // C s_223_3: cast reint s_223_2 -> i64
        let s_223_3: i64 = (s_223_2 as i64);
        // C s_223_4: cast zx s_223_3 -> i
        let s_223_4: i128 = (i128::try_from(s_223_3).unwrap());
        // C s_223_5: const #432u : u32
        let s_223_5: u32 = 432;
        // D s_223_6: read-reg s_223_5:u8
        let s_223_6: u8 = {
            let value = state.read_register::<u8>(s_223_5 as isize);
            tracer.read_register(s_223_5 as isize, value);
            value
        };
        // D s_223_7: call AArch64_AArch32SystemAccessTrap(s_223_6, s_223_4)
        let s_223_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_223_6,
            s_223_4,
        );
        // N s_223_8: return
        return;
    }
    fn block_224<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_224_0: read-var __CPTR_EL2_TAM:u8
        let s_224_0: bool = fn_state.u__CPTR_EL2_TAM;
        // D s_224_1: cast zx s_224_0 -> bv
        let s_224_1: Bits = Bits::new(s_224_0 as u128, 1u16);
        // C s_224_2: const #1u : u8
        let s_224_2: bool = true;
        // C s_224_3: cast zx s_224_2 -> bv
        let s_224_3: Bits = Bits::new(s_224_2 as u128, 1u16);
        // D s_224_4: cmp-eq s_224_1 s_224_3
        let s_224_4: bool = ((s_224_1) == (s_224_3));
        // D s_224_5: write-var gs#120840 <= s_224_4
        fn_state.gs_120840 = s_224_4;
        // N s_224_6: jump b170
        return block_170(state, tracer, fn_state);
    }
    fn block_225<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_225_0: const #432u : u32
        let s_225_0: u32 = 432;
        // D s_225_1: read-reg s_225_0:u8
        let s_225_1: u8 = {
            let value = state.read_register::<u8>(s_225_0 as isize);
            tracer.read_register(s_225_0 as isize, value);
            value
        };
        // D s_225_2: call ELUsingAArch32(s_225_1)
        let s_225_2: bool = ELUsingAArch32(state, tracer, s_225_1);
        // D s_225_3: not s_225_2
        let s_225_3: bool = !s_225_2;
        // D s_225_4: write-var gs#120839 <= s_225_3
        fn_state.gs_120839 = s_225_3;
        // N s_225_5: jump b168
        return block_168(state, tracer, fn_state);
    }
    fn block_226<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_226_0: jump b166
        return block_166(state, tracer, fn_state);
    }
    fn block_227<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_227_0: const #0u : u8
        let s_227_0: bool = false;
        // D s_227_1: write-var gs#120837 <= s_227_0
        fn_state.gs_120837 = s_227_0;
        // N s_227_2: jump b164
        return block_164(state, tracer, fn_state);
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
        // D s_228_3: write-var gs#120834 <= s_228_2
        fn_state.gs_120834 = s_228_2;
        // N s_228_4: jump b162
        return block_162(state, tracer, fn_state);
    }
    fn block_229<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_229_0: jump b160
        return block_160(state, tracer, fn_state);
    }
    fn block_230<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_230_0: const #0u : u8
        let s_230_0: bool = false;
        // D s_230_1: write-var gs#120832 <= s_230_0
        fn_state.gs_120832 = s_230_0;
        // N s_230_2: jump b158
        return block_158(state, tracer, fn_state);
    }
    fn block_231<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_231_0: const #102552u : u32
        let s_231_0: u32 = 102552;
        // D s_231_1: read-reg s_231_0:struct
        let s_231_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_231_0 as isize);
            tracer.read_register(s_231_0 as isize, value);
            value
        };
        // D s_231_2: call _get_HCR_EL2_Type_E2H(s_231_1)
        let s_231_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_231_1);
        // C s_231_3: const #102552u : u32
        let s_231_3: u32 = 102552;
        // D s_231_4: read-reg s_231_3:struct
        let s_231_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_231_3 as isize);
            tracer.read_register(s_231_3 as isize, value);
            value
        };
        // D s_231_5: call _get_HCR_EL2_Type_TGE(s_231_4)
        let s_231_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_231_4);
        // D s_231_6: cast zx s_231_2 -> bv
        let s_231_6: Bits = Bits::new(s_231_2 as u128, 1u16);
        // D s_231_7: cast zx s_231_5 -> bv
        let s_231_7: Bits = Bits::new(s_231_5 as u128, 1u16);
        // D s_231_8: cast reint s_231_6 -> u128
        let s_231_8: u128 = (s_231_6.value() as u128);
        // D s_231_9: size-of s_231_6
        let s_231_9: u16 = s_231_6.length();
        // D s_231_10: cast reint s_231_7 -> u128
        let s_231_10: u128 = (s_231_7.value() as u128);
        // D s_231_11: size-of s_231_7
        let s_231_11: u16 = s_231_7.length();
        // D s_231_12: lsl s_231_8 s_231_11
        let s_231_12: u128 = s_231_8 << s_231_11;
        // D s_231_13: or s_231_12 s_231_10
        let s_231_13: u128 = ((s_231_12) | (s_231_10));
        // D s_231_14: add s_231_9 s_231_11
        let s_231_14: u16 = (s_231_9 + s_231_11);
        // D s_231_15: create-bits s_231_13 s_231_14
        let s_231_15: Bits = Bits::new(s_231_13, s_231_14);
        // D s_231_16: cast reint s_231_15 -> u8
        let s_231_16: u8 = (s_231_15.value() as u8);
        // D s_231_17: cast zx s_231_16 -> bv
        let s_231_17: Bits = Bits::new(s_231_16 as u128, 2u16);
        // C s_231_18: const #3u : u8
        let s_231_18: u8 = 3;
        // C s_231_19: cast zx s_231_18 -> bv
        let s_231_19: Bits = Bits::new(s_231_18 as u128, 2u16);
        // D s_231_20: cmp-ne s_231_17 s_231_19
        let s_231_20: bool = ((s_231_17) != (s_231_19));
        // D s_231_21: write-var gs#120829 <= s_231_20
        fn_state.gs_120829 = s_231_20;
        // N s_231_22: jump b156
        return block_156(state, tracer, fn_state);
    }
    fn block_232<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_232_0: const #432u : u32
        let s_232_0: u32 = 432;
        // D s_232_1: read-reg s_232_0:u8
        let s_232_1: u8 = {
            let value = state.read_register::<u8>(s_232_0 as isize);
            tracer.read_register(s_232_0 as isize, value);
            value
        };
        // D s_232_2: call ELUsingAArch32(s_232_1)
        let s_232_2: bool = ELUsingAArch32(state, tracer, s_232_1);
        // D s_232_3: not s_232_2
        let s_232_3: bool = !s_232_2;
        // D s_232_4: write-var gs#120828 <= s_232_3
        fn_state.gs_120828 = s_232_3;
        // N s_232_5: jump b154
        return block_154(state, tracer, fn_state);
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
        // N s_233_2: branch s_233_1 b249 b234
        if s_233_1 {
            return block_249(state, tracer, fn_state);
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
        // D s_234_1: write-var gs#120876 <= s_234_0
        fn_state.gs_120876 = s_234_0;
        // N s_234_2: jump b235
        return block_235(state, tracer, fn_state);
    }
    fn block_235<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_235_0: read-var gs#120876:u8
        let s_235_0: bool = fn_state.gs_120876;
        // N s_235_1: branch s_235_0 b248 b236
        if s_235_0 {
            return block_248(state, tracer, fn_state);
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
        // D s_236_1: write-var gs#120877 <= s_236_0
        fn_state.gs_120877 = s_236_0;
        // N s_236_2: jump b237
        return block_237(state, tracer, fn_state);
    }
    fn block_237<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_237_0: read-var gs#120877:u8
        let s_237_0: bool = fn_state.gs_120877;
        // N s_237_1: branch s_237_0 b247 b238
        if s_237_0 {
            return block_247(state, tracer, fn_state);
        } else {
            return block_238(state, tracer, fn_state);
        };
    }
    fn block_238<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_238_0: const #() : ()
        let s_238_0: () = ();
        // S s_238_1: call EL2Enabled(s_238_0)
        let s_238_1: bool = EL2Enabled(state, tracer, s_238_0);
        // N s_238_2: branch s_238_1 b246 b239
        if s_238_1 {
            return block_246(state, tracer, fn_state);
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
        // D s_239_1: write-var gs#120878 <= s_239_0
        fn_state.gs_120878 = s_239_0;
        // N s_239_2: jump b240
        return block_240(state, tracer, fn_state);
    }
    fn block_240<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_240_0: read-var gs#120878:u8
        let s_240_0: bool = fn_state.gs_120878;
        // N s_240_1: branch s_240_0 b245 b241
        if s_240_0 {
            return block_245(state, tracer, fn_state);
        } else {
            return block_241(state, tracer, fn_state);
        };
    }
    fn block_241<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_241_0: const #0u : u8
        let s_241_0: bool = false;
        // D s_241_1: write-var gs#120879 <= s_241_0
        fn_state.gs_120879 = s_241_0;
        // N s_241_2: jump b242
        return block_242(state, tracer, fn_state);
    }
    fn block_242<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_242_0: read-var gs#120879:u8
        let s_242_0: bool = fn_state.gs_120879;
        // N s_242_1: branch s_242_0 b244 b243
        if s_242_0 {
            return block_244(state, tracer, fn_state);
        } else {
            return block_243(state, tracer, fn_state);
        };
    }
    fn block_243<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_243_0: panic
        panic!("{:?}", ());
        // N s_243_1: return
        return;
    }
    fn block_244<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_244_0: const #0u : u8
        let s_244_0: u8 = 0;
        // C s_244_1: cast zx s_244_0 -> bv
        let s_244_1: Bits = Bits::new(s_244_0 as u128, 8u16);
        // C s_244_2: cast zx s_244_1 -> i
        let s_244_2: i128 = (s_244_1.value() as i128);
        // C s_244_3: cast reint s_244_2 -> i64
        let s_244_3: i64 = (s_244_2 as i64);
        // C s_244_4: cast zx s_244_3 -> i
        let s_244_4: i128 = (i128::try_from(s_244_3).unwrap());
        // S s_244_5: call AArch32_TakeHypTrapException(s_244_4)
        let s_244_5: () = AArch32_TakeHypTrapException(state, tracer, s_244_4);
        // N s_244_6: return
        return;
    }
    fn block_245<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_245_0: read-var __HCR_TGE:u8
        let s_245_0: bool = fn_state.u__HCR_TGE;
        // D s_245_1: cast zx s_245_0 -> bv
        let s_245_1: Bits = Bits::new(s_245_0 as u128, 1u16);
        // C s_245_2: const #1u : u8
        let s_245_2: bool = true;
        // C s_245_3: cast zx s_245_2 -> bv
        let s_245_3: Bits = Bits::new(s_245_2 as u128, 1u16);
        // D s_245_4: cmp-eq s_245_1 s_245_3
        let s_245_4: bool = ((s_245_1) == (s_245_3));
        // D s_245_5: write-var gs#120879 <= s_245_4
        fn_state.gs_120879 = s_245_4;
        // N s_245_6: jump b242
        return block_242(state, tracer, fn_state);
    }
    fn block_246<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_246_0: const #432u : u32
        let s_246_0: u32 = 432;
        // D s_246_1: read-reg s_246_0:u8
        let s_246_1: u8 = {
            let value = state.read_register::<u8>(s_246_0 as isize);
            tracer.read_register(s_246_0 as isize, value);
            value
        };
        // D s_246_2: call ELUsingAArch32(s_246_1)
        let s_246_2: bool = ELUsingAArch32(state, tracer, s_246_1);
        // D s_246_3: write-var gs#120878 <= s_246_2
        fn_state.gs_120878 = s_246_2;
        // N s_246_4: jump b240
        return block_240(state, tracer, fn_state);
    }
    fn block_247<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_247_0: const #4u : u8
        let s_247_0: u8 = 4;
        // C s_247_1: cast zx s_247_0 -> bv
        let s_247_1: Bits = Bits::new(s_247_0 as u128, 8u16);
        // C s_247_2: cast zx s_247_1 -> i
        let s_247_2: i128 = (s_247_1.value() as i128);
        // C s_247_3: cast reint s_247_2 -> i64
        let s_247_3: i64 = (s_247_2 as i64);
        // C s_247_4: cast zx s_247_3 -> i
        let s_247_4: i128 = (i128::try_from(s_247_3).unwrap());
        // C s_247_5: const #432u : u32
        let s_247_5: u32 = 432;
        // D s_247_6: read-reg s_247_5:u8
        let s_247_6: u8 = {
            let value = state.read_register::<u8>(s_247_5 as isize);
            tracer.read_register(s_247_5 as isize, value);
            value
        };
        // D s_247_7: call AArch64_AArch32SystemAccessTrap(s_247_6, s_247_4)
        let s_247_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_247_6,
            s_247_4,
        );
        // N s_247_8: return
        return;
    }
    fn block_248<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_248_0: read-var __HCR_EL2_TGE:u8
        let s_248_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_248_1: cast zx s_248_0 -> bv
        let s_248_1: Bits = Bits::new(s_248_0 as u128, 1u16);
        // C s_248_2: const #1u : u8
        let s_248_2: bool = true;
        // C s_248_3: cast zx s_248_2 -> bv
        let s_248_3: Bits = Bits::new(s_248_2 as u128, 1u16);
        // D s_248_4: cmp-eq s_248_1 s_248_3
        let s_248_4: bool = ((s_248_1) == (s_248_3));
        // D s_248_5: write-var gs#120877 <= s_248_4
        fn_state.gs_120877 = s_248_4;
        // N s_248_6: jump b237
        return block_237(state, tracer, fn_state);
    }
    fn block_249<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_249_0: const #432u : u32
        let s_249_0: u32 = 432;
        // D s_249_1: read-reg s_249_0:u8
        let s_249_1: u8 = {
            let value = state.read_register::<u8>(s_249_0 as isize);
            tracer.read_register(s_249_0 as isize, value);
            value
        };
        // D s_249_2: call ELUsingAArch32(s_249_1)
        let s_249_2: bool = ELUsingAArch32(state, tracer, s_249_1);
        // D s_249_3: not s_249_2
        let s_249_3: bool = !s_249_2;
        // D s_249_4: write-var gs#120876 <= s_249_3
        fn_state.gs_120876 = s_249_3;
        // N s_249_5: jump b235
        return block_235(state, tracer, fn_state);
    }
    fn block_250<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_250_0: read-var __AMUSERENR_EN:u8
        let s_250_0: bool = fn_state.u__AMUSERENR_EN;
        // D s_250_1: cast zx s_250_0 -> bv
        let s_250_1: Bits = Bits::new(s_250_0 as u128, 1u16);
        // C s_250_2: const #0u : u8
        let s_250_2: bool = false;
        // C s_250_3: cast zx s_250_2 -> bv
        let s_250_3: Bits = Bits::new(s_250_2 as u128, 1u16);
        // D s_250_4: cmp-eq s_250_1 s_250_3
        let s_250_4: bool = ((s_250_1) == (s_250_3));
        // D s_250_5: write-var gs#120827 <= s_250_4
        fn_state.gs_120827 = s_250_4;
        // N s_250_6: jump b151
        return block_151(state, tracer, fn_state);
    }
    fn block_251<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_251_0: const #() : ()
        let s_251_0: () = ();
        // S s_251_1: call EL2Enabled(s_251_0)
        let s_251_1: bool = EL2Enabled(state, tracer, s_251_0);
        // N s_251_2: branch s_251_1 b259 b252
        if s_251_1 {
            return block_259(state, tracer, fn_state);
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
        // D s_252_1: write-var gs#120880 <= s_252_0
        fn_state.gs_120880 = s_252_0;
        // N s_252_2: jump b253
        return block_253(state, tracer, fn_state);
    }
    fn block_253<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_253_0: read-var gs#120880:u8
        let s_253_0: bool = fn_state.gs_120880;
        // N s_253_1: branch s_253_0 b258 b254
        if s_253_0 {
            return block_258(state, tracer, fn_state);
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
        // D s_254_1: write-var gs#120881 <= s_254_0
        fn_state.gs_120881 = s_254_0;
        // N s_254_2: jump b255
        return block_255(state, tracer, fn_state);
    }
    fn block_255<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_255_0: read-var gs#120881:u8
        let s_255_0: bool = fn_state.gs_120881;
        // N s_255_1: branch s_255_0 b257 b256
        if s_255_0 {
            return block_257(state, tracer, fn_state);
        } else {
            return block_256(state, tracer, fn_state);
        };
    }
    fn block_256<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_256_0: const #4u : u8
        let s_256_0: u8 = 4;
        // C s_256_1: cast zx s_256_0 -> bv
        let s_256_1: Bits = Bits::new(s_256_0 as u128, 8u16);
        // C s_256_2: cast zx s_256_1 -> i
        let s_256_2: i128 = (s_256_1.value() as i128);
        // C s_256_3: cast reint s_256_2 -> i64
        let s_256_3: i64 = (s_256_2 as i64);
        // C s_256_4: cast zx s_256_3 -> i
        let s_256_4: i128 = (i128::try_from(s_256_3).unwrap());
        // C s_256_5: const #440u : u32
        let s_256_5: u32 = 440;
        // D s_256_6: read-reg s_256_5:u8
        let s_256_6: u8 = {
            let value = state.read_register::<u8>(s_256_5 as isize);
            tracer.read_register(s_256_5 as isize, value);
            value
        };
        // D s_256_7: call AArch64_AArch32SystemAccessTrap(s_256_6, s_256_4)
        let s_256_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_256_6,
            s_256_4,
        );
        // N s_256_8: return
        return;
    }
    fn block_257<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_257_0: const #4u : u8
        let s_257_0: u8 = 4;
        // C s_257_1: cast zx s_257_0 -> bv
        let s_257_1: Bits = Bits::new(s_257_0 as u128, 8u16);
        // C s_257_2: cast zx s_257_1 -> i
        let s_257_2: i128 = (s_257_1.value() as i128);
        // C s_257_3: cast reint s_257_2 -> i64
        let s_257_3: i64 = (s_257_2 as i64);
        // C s_257_4: cast zx s_257_3 -> i
        let s_257_4: i128 = (i128::try_from(s_257_3).unwrap());
        // C s_257_5: const #432u : u32
        let s_257_5: u32 = 432;
        // D s_257_6: read-reg s_257_5:u8
        let s_257_6: u8 = {
            let value = state.read_register::<u8>(s_257_5 as isize);
            tracer.read_register(s_257_5 as isize, value);
            value
        };
        // D s_257_7: call AArch64_AArch32SystemAccessTrap(s_257_6, s_257_4)
        let s_257_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_257_6,
            s_257_4,
        );
        // N s_257_8: return
        return;
    }
    fn block_258<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_258_0: read-var __HCR_EL2_TGE:u8
        let s_258_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_258_1: cast zx s_258_0 -> bv
        let s_258_1: Bits = Bits::new(s_258_0 as u128, 1u16);
        // C s_258_2: const #1u : u8
        let s_258_2: bool = true;
        // C s_258_3: cast zx s_258_2 -> bv
        let s_258_3: Bits = Bits::new(s_258_2 as u128, 1u16);
        // D s_258_4: cmp-eq s_258_1 s_258_3
        let s_258_4: bool = ((s_258_1) == (s_258_3));
        // D s_258_5: write-var gs#120881 <= s_258_4
        fn_state.gs_120881 = s_258_4;
        // N s_258_6: jump b255
        return block_255(state, tracer, fn_state);
    }
    fn block_259<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_259_0: const #432u : u32
        let s_259_0: u32 = 432;
        // D s_259_1: read-reg s_259_0:u8
        let s_259_1: u8 = {
            let value = state.read_register::<u8>(s_259_0 as isize);
            tracer.read_register(s_259_0 as isize, value);
            value
        };
        // D s_259_2: call ELUsingAArch32(s_259_1)
        let s_259_2: bool = ELUsingAArch32(state, tracer, s_259_1);
        // D s_259_3: not s_259_2
        let s_259_3: bool = !s_259_2;
        // D s_259_4: write-var gs#120880 <= s_259_3
        fn_state.gs_120880 = s_259_3;
        // N s_259_5: jump b253
        return block_253(state, tracer, fn_state);
    }
    fn block_260<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_260_0: read-var __AMUSERENR_EL0_EN:u8
        let s_260_0: bool = fn_state.u__AMUSERENR_EL0_EN;
        // D s_260_1: cast zx s_260_0 -> bv
        let s_260_1: Bits = Bits::new(s_260_0 as u128, 1u16);
        // C s_260_2: const #0u : u8
        let s_260_2: bool = false;
        // C s_260_3: cast zx s_260_2 -> bv
        let s_260_3: Bits = Bits::new(s_260_2 as u128, 1u16);
        // D s_260_4: cmp-eq s_260_1 s_260_3
        let s_260_4: bool = ((s_260_1) == (s_260_3));
        // D s_260_5: write-var gs#120826 <= s_260_4
        fn_state.gs_120826 = s_260_4;
        // N s_260_6: jump b148
        return block_148(state, tracer, fn_state);
    }
    fn block_261<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_261_0: panic
        panic!("{:?}", ());
        // N s_261_1: return
        return;
    }
    fn block_262<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_262_0: read-var __CPTR_EL3_TAM:u8
        let s_262_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_262_1: cast zx s_262_0 -> bv
        let s_262_1: Bits = Bits::new(s_262_0 as u128, 1u16);
        // C s_262_2: const #1u : u8
        let s_262_2: bool = true;
        // C s_262_3: cast zx s_262_2 -> bv
        let s_262_3: Bits = Bits::new(s_262_2 as u128, 1u16);
        // D s_262_4: cmp-eq s_262_1 s_262_3
        let s_262_4: bool = ((s_262_1) == (s_262_3));
        // D s_262_5: write-var gs#120825 <= s_262_4
        fn_state.gs_120825 = s_262_4;
        // N s_262_6: jump b145
        return block_145(state, tracer, fn_state);
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
        // D s_263_2: call ELUsingAArch32(s_263_1)
        let s_263_2: bool = ELUsingAArch32(state, tracer, s_263_1);
        // D s_263_3: not s_263_2
        let s_263_3: bool = !s_263_2;
        // D s_263_4: write-var gs#120824 <= s_263_3
        fn_state.gs_120824 = s_263_3;
        // N s_263_5: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_264<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_264_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_264_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_264_1: call __IMPDEF_boolean(s_264_0)
        let s_264_1: bool = u__IMPDEF_boolean(state, tracer, s_264_0);
        // D s_264_2: write-var gs#120823 <= s_264_1
        fn_state.gs_120823 = s_264_1;
        // N s_264_3: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_265<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_265_0: const #() : ()
        let s_265_0: () = ();
        // S s_265_1: call EDSCR_read(s_265_0)
        let s_265_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_265_0);
        // S s_265_2: call _get_EDSCR_Type_SDD(s_265_1)
        let s_265_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_265_1);
        // S s_265_3: cast zx s_265_2 -> bv
        let s_265_3: Bits = Bits::new(s_265_2 as u128, 1u16);
        // C s_265_4: const #1u : u8
        let s_265_4: bool = true;
        // C s_265_5: cast zx s_265_4 -> bv
        let s_265_5: Bits = Bits::new(s_265_4 as u128, 1u16);
        // S s_265_6: cmp-eq s_265_3 s_265_5
        let s_265_6: bool = ((s_265_3) == (s_265_5));
        // D s_265_7: write-var gs#120822 <= s_265_6
        fn_state.gs_120822 = s_265_6;
        // N s_265_8: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_266<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_266_0: const #424u : u32
        let s_266_0: u32 = 424;
        // D s_266_1: read-reg s_266_0:u8
        let s_266_1: u8 = {
            let value = state.read_register::<u8>(s_266_0 as isize);
            tracer.read_register(s_266_0 as isize, value);
            value
        };
        // C s_266_2: const #2u : u8
        let s_266_2: u8 = 2;
        // D s_266_3: cmp-lt s_266_1 s_266_2
        let s_266_3: bool = ((s_266_1) < (s_266_2));
        // D s_266_4: write-var gs#120821 <= s_266_3
        fn_state.gs_120821 = s_266_3;
        // N s_266_5: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_267<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_267_0: panic
        panic!("{:?}", ());
        // N s_267_1: return
        return;
    }
}
