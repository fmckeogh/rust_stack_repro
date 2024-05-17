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
use u_get_HAFGRTR_EL2_Type_AMEVCNTR114_EL0::*;
use Zeros::*;
use u_get_HCPTR_Type_TAM::*;
use AMCR_read::*;
use IsG1ActivityMonitorImplemented::*;
use HCPTR_read::*;
use u_get_CPTR_EL2_Type_TAM::*;
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
pub fn AMEVCNTR1_SysRegRead64_cdf416f7d354cbf5<T: Tracer>(
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
        gs_122380: bool,
        ga_203438: ProductType5c790c8ef59cc8b2,
        gs_122398: bool,
        u__HSTR_EL2_T5: bool,
        gs_122387: bool,
        ga_203328: ProductType5c790c8ef59cc8b2,
        ga_203449: ProductType5c790c8ef59cc8b2,
        gs_122341: bool,
        gs_122385: bool,
        ga_203428: ProductType72d61775f103f7e0,
        gs_122437: bool,
        gs_122343: bool,
        u__CPTR_EL2_TAM: bool,
        ga_203398: ProductType72d61775f103f7e0,
        gs_122400: bool,
        gs_122402: bool,
        u__HCR_TGE: bool,
        gs_122303: bool,
        gs_122435: bool,
        gs_122304: bool,
        gs_122349: bool,
        gs_122300: bool,
        ga_203447: ProductType5c790c8ef59cc8b2,
        gs_122299: bool,
        gs_122347: bool,
        ga_203391: ProductType72d61775f103f7e0,
        gs_122407: bool,
        gs_122389: bool,
        gs_122408: bool,
        ga_203437: ProductType72d61775f103f7e0,
        gs_122302: bool,
        u__HCR_EL2_TGE: bool,
        gs_122436: bool,
        u__AMCR_CG1RZ: bool,
        gs_122392: bool,
        u__SCR_EL3_FGTEn: bool,
        gs_122336: bool,
        u__AMUSERENR_EL0_EN: bool,
        u__HSTR_T5: bool,
        gs_122308: bool,
        gs_122381: bool,
        gs_122305: bool,
        gs_122393: bool,
        gs_122441: bool,
        gs_122399: bool,
        gs_122403: bool,
        gs_122411: bool,
        gs_122332: bool,
        u__AMUSERENR_EN: bool,
        gs_122348: bool,
        gs_122301: bool,
        ga_203392: ProductType5c790c8ef59cc8b2,
        gs_122356: bool,
        u__AMCR_EL0_CG1RZ: bool,
        gs_122384: bool,
        gs_122307: bool,
        gs_122410: bool,
        gs_122405: bool,
        gs_122409: bool,
        gs_122397: bool,
        u__CPTR_EL3_TAM: bool,
        ga_203453: ProductType72d61775f103f7e0,
        ga_203326: ProductType5c790c8ef59cc8b2,
        gs_122412: bool,
        u__HCPTR_TAM: bool,
        gs_122346: bool,
        gs_122401: bool,
        gs_122354: bool,
        ga_203319: ProductType72d61775f103f7e0,
        gs_122306: bool,
        ga_203440: ProductType5c790c8ef59cc8b2,
        gs_122382: bool,
        gs_122394: bool,
        gs_122350: bool,
        gs_122351: bool,
        u__HAFGRTR_EL2_AMEVCNTR114_EL0: bool,
        ga_203382: ProductType72d61775f103f7e0,
        ga_203325: ProductType72d61775f103f7e0,
        gs_122334: bool,
        gs_122337: bool,
        gs_122388: bool,
        gs_122357: bool,
        ga_203332: ProductType72d61775f103f7e0,
        gs_122353: bool,
        ga_203394: ProductType5c790c8ef59cc8b2,
        gs_122383: bool,
        gs_122309: bool,
        gs_122338: bool,
        gs_122404: bool,
        gs_122355: bool,
        gs_122386: bool,
        gs_122440: bool,
        gs_122342: bool,
        gs_122352: bool,
        gs_122438: bool,
        gs_122333: bool,
        gs_122406: bool,
        gs_122335: bool,
        ga_203444: ProductType72d61775f103f7e0,
        gs_122439: bool,
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
        // D s_0_23: write-var __HSTR_EL2_T5 <= s_0_22
        fn_state.u__HSTR_EL2_T5 = s_0_22;
        // C s_0_24: const #() : ()
        let s_0_24: () = ();
        // S s_0_25: call HSTR_read(s_0_24)
        let s_0_25: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_24);
        // S s_0_26: call _get_HSTR_Type_T5(s_0_25)
        let s_0_26: bool = u_get_HSTR_Type_T5(state, tracer, s_0_25);
        // D s_0_27: write-var __HSTR_T5 <= s_0_26
        fn_state.u__HSTR_T5 = s_0_26;
        // C s_0_28: const #11088u : u32
        let s_0_28: u32 = 11088;
        // D s_0_29: read-reg s_0_28:struct
        let s_0_29: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_28 as isize);
            tracer.read_register(s_0_28 as isize, value);
            value
        };
        // D s_0_30: call _get_CPTR_EL2_Type_TAM(s_0_29)
        let s_0_30: bool = u_get_CPTR_EL2_Type_TAM(state, tracer, s_0_29);
        // D s_0_31: write-var __CPTR_EL2_TAM <= s_0_30
        fn_state.u__CPTR_EL2_TAM = s_0_30;
        // C s_0_32: const #() : ()
        let s_0_32: () = ();
        // S s_0_33: call HCPTR_read(s_0_32)
        let s_0_33: ProductType700c18a878c5601b = HCPTR_read(state, tracer, s_0_32);
        // S s_0_34: call _get_HCPTR_Type_TAM(s_0_33)
        let s_0_34: bool = u_get_HCPTR_Type_TAM(state, tracer, s_0_33);
        // D s_0_35: write-var __HCPTR_TAM <= s_0_34
        fn_state.u__HCPTR_TAM = s_0_34;
        // C s_0_36: const #90704u : u32
        let s_0_36: u32 = 90704;
        // D s_0_37: read-reg s_0_36:struct
        let s_0_37: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_36 as isize);
            tracer.read_register(s_0_36 as isize, value);
            value
        };
        // D s_0_38: call _get_SCR_EL3_Type_FGTEn(s_0_37)
        let s_0_38: bool = u_get_SCR_EL3_Type_FGTEn(state, tracer, s_0_37);
        // D s_0_39: write-var __SCR_EL3_FGTEn <= s_0_38
        fn_state.u__SCR_EL3_FGTEn = s_0_38;
        // C s_0_40: const #21760u : u32
        let s_0_40: u32 = 21760;
        // D s_0_41: read-reg s_0_40:struct
        let s_0_41: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_40 as isize);
            tracer.read_register(s_0_40 as isize, value);
            value
        };
        // D s_0_42: call _get_HAFGRTR_EL2_Type_AMEVCNTR114_EL0(s_0_41)
        let s_0_42: bool = u_get_HAFGRTR_EL2_Type_AMEVCNTR114_EL0(state, tracer, s_0_41);
        // D s_0_43: write-var __HAFGRTR_EL2_AMEVCNTR114_EL0 <= s_0_42
        fn_state.u__HAFGRTR_EL2_AMEVCNTR114_EL0 = s_0_42;
        // C s_0_44: const #15544u : u32
        let s_0_44: u32 = 15544;
        // D s_0_45: read-reg s_0_44:struct
        let s_0_45: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_44 as isize);
            tracer.read_register(s_0_44 as isize, value);
            value
        };
        // D s_0_46: call _get_AMCR_EL0_Type_CG1RZ(s_0_45)
        let s_0_46: bool = u_get_AMCR_EL0_Type_CG1RZ(state, tracer, s_0_45);
        // D s_0_47: write-var __AMCR_EL0_CG1RZ <= s_0_46
        fn_state.u__AMCR_EL0_CG1RZ = s_0_46;
        // C s_0_48: const #() : ()
        let s_0_48: () = ();
        // S s_0_49: call AMCR_read(s_0_48)
        let s_0_49: ProductType700c18a878c5601b = AMCR_read(state, tracer, s_0_48);
        // S s_0_50: call _get_AMCR_Type_CG1RZ(s_0_49)
        let s_0_50: bool = u_get_AMCR_Type_CG1RZ(state, tracer, s_0_49);
        // D s_0_51: write-var __AMCR_CG1RZ <= s_0_50
        fn_state.u__AMCR_CG1RZ = s_0_50;
        // C s_0_52: const #14s : i
        let s_0_52: i128 = 14;
        // S s_0_53: call IsG1ActivityMonitorImplemented(s_0_52)
        let s_0_53: bool = IsG1ActivityMonitorImplemented(state, tracer, s_0_52);
        // S s_0_54: not s_0_53
        let s_0_54: bool = !s_0_53;
        // N s_0_55: branch s_0_54 b275 b1
        if s_0_54 {
            return block_275(state, tracer, fn_state);
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
        // N s_1_7: branch s_1_6 b139 b2
        if s_1_6 {
            return block_139(state, tracer, fn_state);
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
        // C s_6_0: const #14s : i64
        let s_6_0: i64 = 14;
        // S s_6_1: call AMEVCNTR1_read(s_6_0)
        let s_6_1: ProductType5c790c8ef59cc8b2 = AMEVCNTR1_read(state, tracer, s_6_0);
        // D s_6_2: write-var ga#203447 <= s_6_1
        fn_state.ga_203447 = s_6_1;
        // D s_6_3: read-var ga#203447.0:struct
        let s_6_3: u64 = fn_state.ga_203447._0;
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
        // C s_6_12: const #14s : i64
        let s_6_12: i64 = 14;
        // S s_6_13: call AMEVCNTR1_read(s_6_12)
        let s_6_13: ProductType5c790c8ef59cc8b2 = AMEVCNTR1_read(state, tracer, s_6_12);
        // D s_6_14: write-var ga#203449 <= s_6_13
        fn_state.ga_203449 = s_6_13;
        // D s_6_15: read-var ga#203449.0:struct
        let s_6_15: u64 = fn_state.ga_203449._0;
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
        // D s_6_25: write-var ga#203453 <= s_6_24
        fn_state.ga_203453 = s_6_24;
        // D s_6_26: read-var ga#203453.0:struct
        let s_6_26: u32 = fn_state.ga_203453._0;
        // D s_6_27: read-var ga#203453.1:struct
        let s_6_27: u32 = fn_state.ga_203453._1;
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
        // D s_8_1: write-var gs#122299 <= s_8_0
        fn_state.gs_122299 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#122299:u8
        let s_9_0: bool = fn_state.gs_122299;
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
        // D s_10_1: write-var gs#122300 <= s_10_0
        fn_state.gs_122300 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#122300:u8
        let s_11_0: bool = fn_state.gs_122300;
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
        // D s_12_1: write-var gs#122301 <= s_12_0
        fn_state.gs_122301 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#122301:u8
        let s_13_0: bool = fn_state.gs_122301;
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
        // D s_14_1: write-var gs#122302 <= s_14_0
        fn_state.gs_122302 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#122302:u8
        let s_15_0: bool = fn_state.gs_122302;
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
        // D s_16_1: write-var gs#122303 <= s_16_0
        fn_state.gs_122303 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#122303:u8
        let s_17_0: bool = fn_state.gs_122303;
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
        // D s_19_1: write-var gs#122304 <= s_19_0
        fn_state.gs_122304 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#122304:u8
        let s_20_0: bool = fn_state.gs_122304;
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
        // D s_21_1: write-var gs#122305 <= s_21_0
        fn_state.gs_122305 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#122305:u8
        let s_22_0: bool = fn_state.gs_122305;
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
        // D s_24_1: write-var gs#122306 <= s_24_0
        fn_state.gs_122306 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#122306:u8
        let s_25_0: bool = fn_state.gs_122306;
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
        // D s_26_1: write-var gs#122307 <= s_26_0
        fn_state.gs_122307 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#122307:u8
        let s_27_0: bool = fn_state.gs_122307;
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
        // D s_29_1: write-var gs#122308 <= s_29_0
        fn_state.gs_122308 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#122308:u8
        let s_30_0: bool = fn_state.gs_122308;
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
        // D s_31_1: write-var gs#122309 <= s_31_0
        fn_state.gs_122309 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#122309:u8
        let s_32_0: bool = fn_state.gs_122309;
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
        // C s_33_0: const #14s : i64
        let s_33_0: i64 = 14;
        // S s_33_1: call AMEVCNTR1_read(s_33_0)
        let s_33_1: ProductType5c790c8ef59cc8b2 = AMEVCNTR1_read(state, tracer, s_33_0);
        // D s_33_2: write-var ga#203438 <= s_33_1
        fn_state.ga_203438 = s_33_1;
        // D s_33_3: read-var ga#203438.0:struct
        let s_33_3: u64 = fn_state.ga_203438._0;
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
        // C s_33_12: const #14s : i64
        let s_33_12: i64 = 14;
        // S s_33_13: call AMEVCNTR1_read(s_33_12)
        let s_33_13: ProductType5c790c8ef59cc8b2 = AMEVCNTR1_read(
            state,
            tracer,
            s_33_12,
        );
        // D s_33_14: write-var ga#203440 <= s_33_13
        fn_state.ga_203440 = s_33_13;
        // D s_33_15: read-var ga#203440.0:struct
        let s_33_15: u64 = fn_state.ga_203440._0;
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
        // D s_33_25: write-var ga#203444 <= s_33_24
        fn_state.ga_203444 = s_33_24;
        // D s_33_26: read-var ga#203444.0:struct
        let s_33_26: u32 = fn_state.ga_203444._0;
        // D s_33_27: read-var ga#203444.1:struct
        let s_33_27: u32 = fn_state.ga_203444._1;
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
        // D s_34_7: write-var ga#203437 <= s_34_6
        fn_state.ga_203437 = s_34_6;
        // D s_34_8: read-var ga#203437.0:struct
        let s_34_8: u32 = fn_state.ga_203437._0;
        // D s_34_9: read-var ga#203437.1:struct
        let s_34_9: u32 = fn_state.ga_203437._1;
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
        // D s_35_5: write-var gs#122309 <= s_35_4
        fn_state.gs_122309 = s_35_4;
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
        // D s_36_3: write-var gs#122308 <= s_36_2
        fn_state.gs_122308 = s_36_2;
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
        // D s_37_7: write-var ga#203428 <= s_37_6
        fn_state.ga_203428 = s_37_6;
        // D s_37_8: read-var ga#203428.0:struct
        let s_37_8: u32 = fn_state.ga_203428._0;
        // D s_37_9: read-var ga#203428.1:struct
        let s_37_9: u32 = fn_state.ga_203428._1;
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
        // D s_38_5: write-var gs#122307 <= s_38_4
        fn_state.gs_122307 = s_38_4;
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
        // D s_39_2: write-var gs#122306 <= s_39_1
        fn_state.gs_122306 = s_39_1;
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
        // D s_41_1: write-var gs#122332 <= s_41_0
        fn_state.gs_122332 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#122332:u8
        let s_42_0: bool = fn_state.gs_122332;
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
        // D s_45_7: write-var gs#122332 <= s_45_6
        fn_state.gs_122332 = s_45_6;
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
        // D s_46_5: write-var gs#122305 <= s_46_4
        fn_state.gs_122305 = s_46_4;
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
        // D s_47_4: write-var gs#122304 <= s_47_3
        fn_state.gs_122304 = s_47_3;
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
        // D s_49_5: write-var gs#122303 <= s_49_4
        fn_state.gs_122303 = s_49_4;
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
        // D s_50_4: write-var gs#122302 <= s_50_3
        fn_state.gs_122302 = s_50_3;
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
        // D s_51_2: write-var gs#122301 <= s_51_1
        fn_state.gs_122301 = s_51_1;
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
        // D s_52_7: write-var gs#122300 <= s_52_6
        fn_state.gs_122300 = s_52_6;
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
        // D s_53_4: write-var gs#122299 <= s_53_3
        fn_state.gs_122299 = s_53_3;
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
        // N s_54_2: branch s_54_1 b138 b55
        if s_54_1 {
            return block_138(state, tracer, fn_state);
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
        // D s_55_1: write-var gs#122333 <= s_55_0
        fn_state.gs_122333 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#122333:u8
        let s_56_0: bool = fn_state.gs_122333;
        // N s_56_1: branch s_56_0 b137 b57
        if s_56_0 {
            return block_137(state, tracer, fn_state);
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
        // D s_57_1: write-var gs#122334 <= s_57_0
        fn_state.gs_122334 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var gs#122334:u8
        let s_58_0: bool = fn_state.gs_122334;
        // N s_58_1: branch s_58_0 b136 b59
        if s_58_0 {
            return block_136(state, tracer, fn_state);
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
        // D s_59_1: write-var gs#122335 <= s_59_0
        fn_state.gs_122335 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#122335:u8
        let s_60_0: bool = fn_state.gs_122335;
        // N s_60_1: branch s_60_0 b135 b61
        if s_60_0 {
            return block_135(state, tracer, fn_state);
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
        // D s_61_1: write-var gs#122336 <= s_61_0
        fn_state.gs_122336 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#122336:u8
        let s_62_0: bool = fn_state.gs_122336;
        // N s_62_1: branch s_62_0 b134 b63
        if s_62_0 {
            return block_134(state, tracer, fn_state);
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
        // D s_63_1: write-var gs#122337 <= s_63_0
        fn_state.gs_122337 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#122337:u8
        let s_64_0: bool = fn_state.gs_122337;
        // N s_64_1: branch s_64_0 b133 b65
        if s_64_0 {
            return block_133(state, tracer, fn_state);
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
        // N s_65_2: branch s_65_1 b132 b66
        if s_65_1 {
            return block_132(state, tracer, fn_state);
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
        // D s_66_1: write-var gs#122338 <= s_66_0
        fn_state.gs_122338 = s_66_0;
        // N s_66_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#122338:u8
        let s_67_0: bool = fn_state.gs_122338;
        // N s_67_1: branch s_67_0 b131 b68
        if s_67_0 {
            return block_131(state, tracer, fn_state);
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
        // D s_68_1: write-var gs#122341 <= s_68_0
        fn_state.gs_122341 = s_68_0;
        // N s_68_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var gs#122341:u8
        let s_69_0: bool = fn_state.gs_122341;
        // N s_69_1: branch s_69_0 b130 b70
        if s_69_0 {
            return block_130(state, tracer, fn_state);
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
        // D s_70_1: write-var gs#122342 <= s_70_0
        fn_state.gs_122342 = s_70_0;
        // N s_70_2: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var gs#122342:u8
        let s_71_0: bool = fn_state.gs_122342;
        // N s_71_1: branch s_71_0 b129 b72
        if s_71_0 {
            return block_129(state, tracer, fn_state);
        } else {
            return block_72(state, tracer, fn_state);
        };
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #() : ()
        let s_72_0: () = ();
        // S s_72_1: call EL2Enabled(s_72_0)
        let s_72_1: bool = EL2Enabled(state, tracer, s_72_0);
        // N s_72_2: branch s_72_1 b128 b73
        if s_72_1 {
            return block_128(state, tracer, fn_state);
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
        // D s_73_1: write-var gs#122343 <= s_73_0
        fn_state.gs_122343 = s_73_0;
        // N s_73_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var gs#122343:u8
        let s_74_0: bool = fn_state.gs_122343;
        // N s_74_1: branch s_74_0 b127 b75
        if s_74_0 {
            return block_127(state, tracer, fn_state);
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
        // D s_75_1: write-var gs#122346 <= s_75_0
        fn_state.gs_122346 = s_75_0;
        // N s_75_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var gs#122346:u8
        let s_76_0: bool = fn_state.gs_122346;
        // N s_76_1: branch s_76_0 b126 b77
        if s_76_0 {
            return block_126(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #0u : u8
        let s_77_0: bool = false;
        // D s_77_1: write-var gs#122347 <= s_77_0
        fn_state.gs_122347 = s_77_0;
        // N s_77_2: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var gs#122347:u8
        let s_78_0: bool = fn_state.gs_122347;
        // N s_78_1: branch s_78_0 b125 b79
        if s_78_0 {
            return block_125(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #() : ()
        let s_79_0: () = ();
        // S s_79_1: call EL2Enabled(s_79_0)
        let s_79_1: bool = EL2Enabled(state, tracer, s_79_0);
        // N s_79_2: branch s_79_1 b124 b80
        if s_79_1 {
            return block_124(state, tracer, fn_state);
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
        // D s_80_1: write-var gs#122348 <= s_80_0
        fn_state.gs_122348 = s_80_0;
        // N s_80_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#122348:u8
        let s_81_0: bool = fn_state.gs_122348;
        // N s_81_1: branch s_81_0 b123 b82
        if s_81_0 {
            return block_123(state, tracer, fn_state);
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
        // D s_82_1: write-var gs#122349 <= s_82_0
        fn_state.gs_122349 = s_82_0;
        // N s_82_2: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_83_0: read-var gs#122349:u8
        let s_83_0: bool = fn_state.gs_122349;
        // N s_83_1: branch s_83_0 b122 b84
        if s_83_0 {
            return block_122(state, tracer, fn_state);
        } else {
            return block_84(state, tracer, fn_state);
        };
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #() : ()
        let s_84_0: () = ();
        // S s_84_1: call EL2Enabled(s_84_0)
        let s_84_1: bool = EL2Enabled(state, tracer, s_84_0);
        // N s_84_2: branch s_84_1 b121 b85
        if s_84_1 {
            return block_121(state, tracer, fn_state);
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
        // D s_85_1: write-var gs#122350 <= s_85_0
        fn_state.gs_122350 = s_85_0;
        // N s_85_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var gs#122350:u8
        let s_86_0: bool = fn_state.gs_122350;
        // N s_86_1: branch s_86_0 b120 b87
        if s_86_0 {
            return block_120(state, tracer, fn_state);
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
        // D s_87_1: write-var gs#122351 <= s_87_0
        fn_state.gs_122351 = s_87_0;
        // N s_87_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var gs#122351:u8
        let s_88_0: bool = fn_state.gs_122351;
        // N s_88_1: branch s_88_0 b119 b89
        if s_88_0 {
            return block_119(state, tracer, fn_state);
        } else {
            return block_89(state, tracer, fn_state);
        };
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #424u : u32
        let s_89_0: u32 = 424;
        // D s_89_1: read-reg s_89_0:u8
        let s_89_1: u8 = {
            let value = state.read_register::<u8>(s_89_0 as isize);
            tracer.read_register(s_89_0 as isize, value);
            value
        };
        // C s_89_2: const #2u : u8
        let s_89_2: u8 = 2;
        // D s_89_3: cmp-lt s_89_1 s_89_2
        let s_89_3: bool = ((s_89_1) < (s_89_2));
        // N s_89_4: branch s_89_3 b118 b90
        if s_89_3 {
            return block_118(state, tracer, fn_state);
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
        // D s_90_1: write-var gs#122352 <= s_90_0
        fn_state.gs_122352 = s_90_0;
        // N s_90_2: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_91_0: read-var gs#122352:u8
        let s_91_0: bool = fn_state.gs_122352;
        // N s_91_1: branch s_91_0 b117 b92
        if s_91_0 {
            return block_117(state, tracer, fn_state);
        } else {
            return block_92(state, tracer, fn_state);
        };
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #0u : u8
        let s_92_0: bool = false;
        // D s_92_1: write-var gs#122353 <= s_92_0
        fn_state.gs_122353 = s_92_0;
        // N s_92_2: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_93_0: read-var gs#122353:u8
        let s_93_0: bool = fn_state.gs_122353;
        // N s_93_1: branch s_93_0 b111 b94
        if s_93_0 {
            return block_111(state, tracer, fn_state);
        } else {
            return block_94(state, tracer, fn_state);
        };
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #16975u : u32
        let s_94_0: u32 = 16975;
        // D s_94_1: read-reg s_94_0:u8
        let s_94_1: u8 = {
            let value = state.read_register::<u8>(s_94_0 as isize);
            tracer.read_register(s_94_0 as isize, value);
            value
        };
        // D s_94_2: call IsHighestEL(s_94_1)
        let s_94_2: bool = IsHighestEL(state, tracer, s_94_1);
        // D s_94_3: not s_94_2
        let s_94_3: bool = !s_94_2;
        // N s_94_4: branch s_94_3 b110 b95
        if s_94_3 {
            return block_110(state, tracer, fn_state);
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
        // D s_95_1: write-var gs#122354 <= s_95_0
        fn_state.gs_122354 = s_95_0;
        // N s_95_2: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var gs#122354:u8
        let s_96_0: bool = fn_state.gs_122354;
        // N s_96_1: branch s_96_0 b109 b97
        if s_96_0 {
            return block_109(state, tracer, fn_state);
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
        // D s_97_1: write-var gs#122355 <= s_97_0
        fn_state.gs_122355 = s_97_0;
        // N s_97_2: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var gs#122355:u8
        let s_98_0: bool = fn_state.gs_122355;
        // N s_98_1: branch s_98_0 b108 b99
        if s_98_0 {
            return block_108(state, tracer, fn_state);
        } else {
            return block_99(state, tracer, fn_state);
        };
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #16975u : u32
        let s_99_0: u32 = 16975;
        // D s_99_1: read-reg s_99_0:u8
        let s_99_1: u8 = {
            let value = state.read_register::<u8>(s_99_0 as isize);
            tracer.read_register(s_99_0 as isize, value);
            value
        };
        // D s_99_2: call IsHighestEL(s_99_1)
        let s_99_2: bool = IsHighestEL(state, tracer, s_99_1);
        // D s_99_3: not s_99_2
        let s_99_3: bool = !s_99_2;
        // N s_99_4: branch s_99_3 b107 b100
        if s_99_3 {
            return block_107(state, tracer, fn_state);
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
        // D s_100_1: write-var gs#122356 <= s_100_0
        fn_state.gs_122356 = s_100_0;
        // N s_100_2: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_101_0: read-var gs#122356:u8
        let s_101_0: bool = fn_state.gs_122356;
        // N s_101_1: branch s_101_0 b106 b102
        if s_101_0 {
            return block_106(state, tracer, fn_state);
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
        // D s_102_1: write-var gs#122357 <= s_102_0
        fn_state.gs_122357 = s_102_0;
        // N s_102_2: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_103_0: read-var gs#122357:u8
        let s_103_0: bool = fn_state.gs_122357;
        // N s_103_1: branch s_103_0 b105 b104
        if s_103_0 {
            return block_105(state, tracer, fn_state);
        } else {
            return block_104(state, tracer, fn_state);
        };
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #14s : i64
        let s_104_0: i64 = 14;
        // S s_104_1: call AMEVCNTR1_read(s_104_0)
        let s_104_1: ProductType5c790c8ef59cc8b2 = AMEVCNTR1_read(
            state,
            tracer,
            s_104_0,
        );
        // D s_104_2: write-var ga#203392 <= s_104_1
        fn_state.ga_203392 = s_104_1;
        // D s_104_3: read-var ga#203392.0:struct
        let s_104_3: u64 = fn_state.ga_203392._0;
        // C s_104_4: const #32s : i
        let s_104_4: i128 = 32;
        // D s_104_5: cast zx s_104_3 -> bv
        let s_104_5: Bits = Bits::new(s_104_3 as u128, 64u16);
        // C s_104_6: const #1s : i64
        let s_104_6: i64 = 1;
        // C s_104_7: cast zx s_104_6 -> i
        let s_104_7: i128 = (i128::try_from(s_104_6).unwrap());
        // C s_104_8: const #31s : i
        let s_104_8: i128 = 31;
        // C s_104_9: add s_104_8 s_104_7
        let s_104_9: i128 = (s_104_8 + s_104_7);
        // D s_104_10: bit-extract s_104_5 s_104_4 s_104_9
        let s_104_10: Bits = (Bits::new(
            ((s_104_5) >> (s_104_4)).value(),
            u16::try_from(s_104_9).unwrap(),
        ));
        // D s_104_11: cast reint s_104_10 -> u32
        let s_104_11: u32 = (s_104_10.value() as u32);
        // C s_104_12: const #14s : i64
        let s_104_12: i64 = 14;
        // S s_104_13: call AMEVCNTR1_read(s_104_12)
        let s_104_13: ProductType5c790c8ef59cc8b2 = AMEVCNTR1_read(
            state,
            tracer,
            s_104_12,
        );
        // D s_104_14: write-var ga#203394 <= s_104_13
        fn_state.ga_203394 = s_104_13;
        // D s_104_15: read-var ga#203394.0:struct
        let s_104_15: u64 = fn_state.ga_203394._0;
        // C s_104_16: const #0s : i
        let s_104_16: i128 = 0;
        // D s_104_17: cast zx s_104_15 -> bv
        let s_104_17: Bits = Bits::new(s_104_15 as u128, 64u16);
        // C s_104_18: const #1s : i64
        let s_104_18: i64 = 1;
        // C s_104_19: cast zx s_104_18 -> i
        let s_104_19: i128 = (i128::try_from(s_104_18).unwrap());
        // C s_104_20: const #31s : i
        let s_104_20: i128 = 31;
        // C s_104_21: add s_104_20 s_104_19
        let s_104_21: i128 = (s_104_20 + s_104_19);
        // D s_104_22: bit-extract s_104_17 s_104_16 s_104_21
        let s_104_22: Bits = (Bits::new(
            ((s_104_17) >> (s_104_16)).value(),
            u16::try_from(s_104_21).unwrap(),
        ));
        // D s_104_23: cast reint s_104_22 -> u32
        let s_104_23: u32 = (s_104_22.value() as u32);
        // D s_104_24: create-product struct = ["s_104_11", "s_104_23"]
        let s_104_24: ProductType72d61775f103f7e0 = ProductType72d61775f103f7e0 {
            _0: s_104_11,
            _1: s_104_23,
        };
        // D s_104_25: write-var ga#203398 <= s_104_24
        fn_state.ga_203398 = s_104_24;
        // D s_104_26: read-var ga#203398.0:struct
        let s_104_26: u32 = fn_state.ga_203398._0;
        // D s_104_27: read-var ga#203398.1:struct
        let s_104_27: u32 = fn_state.ga_203398._1;
        // D s_104_28: read-var t2:i
        let s_104_28: i128 = fn_state.t2;
        // D s_104_29: call R_set(s_104_28, s_104_26)
        let s_104_29: () = R_set(state, tracer, s_104_28, s_104_26);
        // D s_104_30: read-var t:i
        let s_104_30: i128 = fn_state.t;
        // D s_104_31: call R_set(s_104_30, s_104_27)
        let s_104_31: () = R_set(state, tracer, s_104_30, s_104_27);
        // N s_104_32: return
        return;
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #32s : i
        let s_105_0: i128 = 32;
        // S s_105_1: call Zeros(s_105_0)
        let s_105_1: Bits = Zeros(state, tracer, s_105_0);
        // S s_105_2: cast reint s_105_1 -> u32
        let s_105_2: u32 = (s_105_1.value() as u32);
        // C s_105_3: const #32s : i
        let s_105_3: i128 = 32;
        // S s_105_4: call Zeros(s_105_3)
        let s_105_4: Bits = Zeros(state, tracer, s_105_3);
        // S s_105_5: cast reint s_105_4 -> u32
        let s_105_5: u32 = (s_105_4.value() as u32);
        // D s_105_6: create-product struct = ["s_105_2", "s_105_5"]
        let s_105_6: ProductType72d61775f103f7e0 = ProductType72d61775f103f7e0 {
            _0: s_105_2,
            _1: s_105_5,
        };
        // D s_105_7: write-var ga#203391 <= s_105_6
        fn_state.ga_203391 = s_105_6;
        // D s_105_8: read-var ga#203391.0:struct
        let s_105_8: u32 = fn_state.ga_203391._0;
        // D s_105_9: read-var ga#203391.1:struct
        let s_105_9: u32 = fn_state.ga_203391._1;
        // D s_105_10: read-var t2:i
        let s_105_10: i128 = fn_state.t2;
        // D s_105_11: call R_set(s_105_10, s_105_8)
        let s_105_11: () = R_set(state, tracer, s_105_10, s_105_8);
        // D s_105_12: read-var t:i
        let s_105_12: i128 = fn_state.t;
        // D s_105_13: call R_set(s_105_12, s_105_9)
        let s_105_13: () = R_set(state, tracer, s_105_12, s_105_9);
        // N s_105_14: return
        return;
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var __AMCR_CG1RZ:u8
        let s_106_0: bool = fn_state.u__AMCR_CG1RZ;
        // D s_106_1: cast zx s_106_0 -> bv
        let s_106_1: Bits = Bits::new(s_106_0 as u128, 1u16);
        // C s_106_2: const #1u : u8
        let s_106_2: bool = true;
        // C s_106_3: cast zx s_106_2 -> bv
        let s_106_3: Bits = Bits::new(s_106_2 as u128, 1u16);
        // D s_106_4: cmp-eq s_106_1 s_106_3
        let s_106_4: bool = ((s_106_1) == (s_106_3));
        // D s_106_5: write-var gs#122357 <= s_106_4
        fn_state.gs_122357 = s_106_4;
        // N s_106_6: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #() : ()
        let s_107_0: () = ();
        // S s_107_1: call HaveAArch64(s_107_0)
        let s_107_1: bool = HaveAArch64(state, tracer, s_107_0);
        // S s_107_2: not s_107_1
        let s_107_2: bool = !s_107_1;
        // D s_107_3: write-var gs#122356 <= s_107_2
        fn_state.gs_122356 = s_107_2;
        // N s_107_4: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #32s : i
        let s_108_0: i128 = 32;
        // S s_108_1: call Zeros(s_108_0)
        let s_108_1: Bits = Zeros(state, tracer, s_108_0);
        // S s_108_2: cast reint s_108_1 -> u32
        let s_108_2: u32 = (s_108_1.value() as u32);
        // C s_108_3: const #32s : i
        let s_108_3: i128 = 32;
        // S s_108_4: call Zeros(s_108_3)
        let s_108_4: Bits = Zeros(state, tracer, s_108_3);
        // S s_108_5: cast reint s_108_4 -> u32
        let s_108_5: u32 = (s_108_4.value() as u32);
        // D s_108_6: create-product struct = ["s_108_2", "s_108_5"]
        let s_108_6: ProductType72d61775f103f7e0 = ProductType72d61775f103f7e0 {
            _0: s_108_2,
            _1: s_108_5,
        };
        // D s_108_7: write-var ga#203382 <= s_108_6
        fn_state.ga_203382 = s_108_6;
        // D s_108_8: read-var ga#203382.0:struct
        let s_108_8: u32 = fn_state.ga_203382._0;
        // D s_108_9: read-var ga#203382.1:struct
        let s_108_9: u32 = fn_state.ga_203382._1;
        // D s_108_10: read-var t2:i
        let s_108_10: i128 = fn_state.t2;
        // D s_108_11: call R_set(s_108_10, s_108_8)
        let s_108_11: () = R_set(state, tracer, s_108_10, s_108_8);
        // D s_108_12: read-var t:i
        let s_108_12: i128 = fn_state.t;
        // D s_108_13: call R_set(s_108_12, s_108_9)
        let s_108_13: () = R_set(state, tracer, s_108_12, s_108_9);
        // N s_108_14: return
        return;
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_109_0: read-var __AMCR_EL0_CG1RZ:u8
        let s_109_0: bool = fn_state.u__AMCR_EL0_CG1RZ;
        // D s_109_1: cast zx s_109_0 -> bv
        let s_109_1: Bits = Bits::new(s_109_0 as u128, 1u16);
        // C s_109_2: const #1u : u8
        let s_109_2: bool = true;
        // C s_109_3: cast zx s_109_2 -> bv
        let s_109_3: Bits = Bits::new(s_109_2 as u128, 1u16);
        // D s_109_4: cmp-eq s_109_1 s_109_3
        let s_109_4: bool = ((s_109_1) == (s_109_3));
        // D s_109_5: write-var gs#122355 <= s_109_4
        fn_state.gs_122355 = s_109_4;
        // N s_109_6: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_110_0: const #() : ()
        let s_110_0: () = ();
        // S s_110_1: call HaveAArch64(s_110_0)
        let s_110_1: bool = HaveAArch64(state, tracer, s_110_0);
        // D s_110_2: write-var gs#122354 <= s_110_1
        fn_state.gs_122354 = s_110_1;
        // N s_110_3: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_111_0: const #() : ()
        let s_111_0: () = ();
        // S s_111_1: call Halted(s_111_0)
        let s_111_1: bool = Halted(state, tracer, s_111_0);
        // N s_111_2: branch s_111_1 b116 b112
        if s_111_1 {
            return block_116(state, tracer, fn_state);
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
        // D s_112_1: write-var gs#122380 <= s_112_0
        fn_state.gs_122380 = s_112_0;
        // N s_112_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var gs#122380:u8
        let s_113_0: bool = fn_state.gs_122380;
        // N s_113_1: branch s_113_0 b115 b114
        if s_113_0 {
            return block_115(state, tracer, fn_state);
        } else {
            return block_114(state, tracer, fn_state);
        };
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #4u : u8
        let s_114_0: u8 = 4;
        // C s_114_1: cast zx s_114_0 -> bv
        let s_114_1: Bits = Bits::new(s_114_0 as u128, 8u16);
        // C s_114_2: cast zx s_114_1 -> i
        let s_114_2: i128 = (s_114_1.value() as i128);
        // C s_114_3: cast reint s_114_2 -> i64
        let s_114_3: i64 = (s_114_2 as i64);
        // C s_114_4: cast zx s_114_3 -> i
        let s_114_4: i128 = (i128::try_from(s_114_3).unwrap());
        // C s_114_5: const #424u : u32
        let s_114_5: u32 = 424;
        // D s_114_6: read-reg s_114_5:u8
        let s_114_6: u8 = {
            let value = state.read_register::<u8>(s_114_5 as isize);
            tracer.read_register(s_114_5 as isize, value);
            value
        };
        // D s_114_7: call AArch64_AArch32SystemAccessTrap(s_114_6, s_114_4)
        let s_114_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_114_6,
            s_114_4,
        );
        // N s_114_8: return
        return;
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
        // C s_116_0: const #() : ()
        let s_116_0: () = ();
        // S s_116_1: call EDSCR_read(s_116_0)
        let s_116_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_116_0);
        // S s_116_2: call _get_EDSCR_Type_SDD(s_116_1)
        let s_116_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_116_1);
        // S s_116_3: cast zx s_116_2 -> bv
        let s_116_3: Bits = Bits::new(s_116_2 as u128, 1u16);
        // C s_116_4: const #1u : u8
        let s_116_4: bool = true;
        // C s_116_5: cast zx s_116_4 -> bv
        let s_116_5: Bits = Bits::new(s_116_4 as u128, 1u16);
        // S s_116_6: cmp-eq s_116_3 s_116_5
        let s_116_6: bool = ((s_116_3) == (s_116_5));
        // D s_116_7: write-var gs#122380 <= s_116_6
        fn_state.gs_122380 = s_116_6;
        // N s_116_8: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_117_0: read-var __CPTR_EL3_TAM:u8
        let s_117_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_117_1: cast zx s_117_0 -> bv
        let s_117_1: Bits = Bits::new(s_117_0 as u128, 1u16);
        // C s_117_2: const #1u : u8
        let s_117_2: bool = true;
        // C s_117_3: cast zx s_117_2 -> bv
        let s_117_3: Bits = Bits::new(s_117_2 as u128, 1u16);
        // D s_117_4: cmp-eq s_117_1 s_117_3
        let s_117_4: bool = ((s_117_1) == (s_117_3));
        // D s_117_5: write-var gs#122353 <= s_117_4
        fn_state.gs_122353 = s_117_4;
        // N s_117_6: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #424u : u32
        let s_118_0: u32 = 424;
        // D s_118_1: read-reg s_118_0:u8
        let s_118_1: u8 = {
            let value = state.read_register::<u8>(s_118_0 as isize);
            tracer.read_register(s_118_0 as isize, value);
            value
        };
        // D s_118_2: call ELUsingAArch32(s_118_1)
        let s_118_2: bool = ELUsingAArch32(state, tracer, s_118_1);
        // D s_118_3: not s_118_2
        let s_118_3: bool = !s_118_2;
        // D s_118_4: write-var gs#122352 <= s_118_3
        fn_state.gs_122352 = s_118_3;
        // N s_118_5: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #4u : u8
        let s_119_0: u8 = 4;
        // C s_119_1: cast zx s_119_0 -> bv
        let s_119_1: Bits = Bits::new(s_119_0 as u128, 8u16);
        // C s_119_2: cast zx s_119_1 -> i
        let s_119_2: i128 = (s_119_1.value() as i128);
        // C s_119_3: cast reint s_119_2 -> i64
        let s_119_3: i64 = (s_119_2 as i64);
        // C s_119_4: cast zx s_119_3 -> i
        let s_119_4: i128 = (i128::try_from(s_119_3).unwrap());
        // S s_119_5: call AArch32_TakeHypTrapException(s_119_4)
        let s_119_5: () = AArch32_TakeHypTrapException(state, tracer, s_119_4);
        // N s_119_6: return
        return;
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_120_0: read-var __HCPTR_TAM:u8
        let s_120_0: bool = fn_state.u__HCPTR_TAM;
        // D s_120_1: cast zx s_120_0 -> bv
        let s_120_1: Bits = Bits::new(s_120_0 as u128, 1u16);
        // C s_120_2: const #1u : u8
        let s_120_2: bool = true;
        // C s_120_3: cast zx s_120_2 -> bv
        let s_120_3: Bits = Bits::new(s_120_2 as u128, 1u16);
        // D s_120_4: cmp-eq s_120_1 s_120_3
        let s_120_4: bool = ((s_120_1) == (s_120_3));
        // D s_120_5: write-var gs#122351 <= s_120_4
        fn_state.gs_122351 = s_120_4;
        // N s_120_6: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #432u : u32
        let s_121_0: u32 = 432;
        // D s_121_1: read-reg s_121_0:u8
        let s_121_1: u8 = {
            let value = state.read_register::<u8>(s_121_0 as isize);
            tracer.read_register(s_121_0 as isize, value);
            value
        };
        // D s_121_2: call ELUsingAArch32(s_121_1)
        let s_121_2: bool = ELUsingAArch32(state, tracer, s_121_1);
        // D s_121_3: write-var gs#122350 <= s_121_2
        fn_state.gs_122350 = s_121_2;
        // N s_121_4: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_122_0: const #4u : u8
        let s_122_0: u8 = 4;
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
        // D s_122_7: call AArch64_AArch32SystemAccessTrap(s_122_6, s_122_4)
        let s_122_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_122_6,
            s_122_4,
        );
        // N s_122_8: return
        return;
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_123_0: read-var __CPTR_EL2_TAM:u8
        let s_123_0: bool = fn_state.u__CPTR_EL2_TAM;
        // D s_123_1: cast zx s_123_0 -> bv
        let s_123_1: Bits = Bits::new(s_123_0 as u128, 1u16);
        // C s_123_2: const #1u : u8
        let s_123_2: bool = true;
        // C s_123_3: cast zx s_123_2 -> bv
        let s_123_3: Bits = Bits::new(s_123_2 as u128, 1u16);
        // D s_123_4: cmp-eq s_123_1 s_123_3
        let s_123_4: bool = ((s_123_1) == (s_123_3));
        // D s_123_5: write-var gs#122349 <= s_123_4
        fn_state.gs_122349 = s_123_4;
        // N s_123_6: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #432u : u32
        let s_124_0: u32 = 432;
        // D s_124_1: read-reg s_124_0:u8
        let s_124_1: u8 = {
            let value = state.read_register::<u8>(s_124_0 as isize);
            tracer.read_register(s_124_0 as isize, value);
            value
        };
        // D s_124_2: call ELUsingAArch32(s_124_1)
        let s_124_2: bool = ELUsingAArch32(state, tracer, s_124_1);
        // D s_124_3: not s_124_2
        let s_124_3: bool = !s_124_2;
        // D s_124_4: write-var gs#122348 <= s_124_3
        fn_state.gs_122348 = s_124_3;
        // N s_124_5: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_125_0: const #4u : u8
        let s_125_0: u8 = 4;
        // C s_125_1: cast zx s_125_0 -> bv
        let s_125_1: Bits = Bits::new(s_125_0 as u128, 8u16);
        // C s_125_2: cast zx s_125_1 -> i
        let s_125_2: i128 = (s_125_1.value() as i128);
        // C s_125_3: cast reint s_125_2 -> i64
        let s_125_3: i64 = (s_125_2 as i64);
        // C s_125_4: cast zx s_125_3 -> i
        let s_125_4: i128 = (i128::try_from(s_125_3).unwrap());
        // S s_125_5: call AArch32_TakeHypTrapException(s_125_4)
        let s_125_5: () = AArch32_TakeHypTrapException(state, tracer, s_125_4);
        // N s_125_6: return
        return;
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_126_0: read-var __HSTR_T5:u8
        let s_126_0: bool = fn_state.u__HSTR_T5;
        // D s_126_1: cast zx s_126_0 -> bv
        let s_126_1: Bits = Bits::new(s_126_0 as u128, 1u16);
        // C s_126_2: const #1u : u8
        let s_126_2: bool = true;
        // C s_126_3: cast zx s_126_2 -> bv
        let s_126_3: Bits = Bits::new(s_126_2 as u128, 1u16);
        // D s_126_4: cmp-eq s_126_1 s_126_3
        let s_126_4: bool = ((s_126_1) == (s_126_3));
        // D s_126_5: write-var gs#122347 <= s_126_4
        fn_state.gs_122347 = s_126_4;
        // N s_126_6: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #1u : u8
        let s_127_0: bool = true;
        // D s_127_1: write-var gs#122346 <= s_127_0
        fn_state.gs_122346 = s_127_0;
        // N s_127_2: jump b76
        return block_76(state, tracer, fn_state);
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
        // D s_128_3: write-var gs#122343 <= s_128_2
        fn_state.gs_122343 = s_128_2;
        // N s_128_4: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_129_0: const #4u : u8
        let s_129_0: u8 = 4;
        // C s_129_1: cast zx s_129_0 -> bv
        let s_129_1: Bits = Bits::new(s_129_0 as u128, 8u16);
        // C s_129_2: cast zx s_129_1 -> i
        let s_129_2: i128 = (s_129_1.value() as i128);
        // C s_129_3: cast reint s_129_2 -> i64
        let s_129_3: i64 = (s_129_2 as i64);
        // C s_129_4: cast zx s_129_3 -> i
        let s_129_4: i128 = (i128::try_from(s_129_3).unwrap());
        // C s_129_5: const #432u : u32
        let s_129_5: u32 = 432;
        // D s_129_6: read-reg s_129_5:u8
        let s_129_6: u8 = {
            let value = state.read_register::<u8>(s_129_5 as isize);
            tracer.read_register(s_129_5 as isize, value);
            value
        };
        // D s_129_7: call AArch64_AArch32SystemAccessTrap(s_129_6, s_129_4)
        let s_129_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_129_6,
            s_129_4,
        );
        // N s_129_8: return
        return;
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_130_0: read-var __HSTR_EL2_T5:u8
        let s_130_0: bool = fn_state.u__HSTR_EL2_T5;
        // D s_130_1: cast zx s_130_0 -> bv
        let s_130_1: Bits = Bits::new(s_130_0 as u128, 1u16);
        // C s_130_2: const #1u : u8
        let s_130_2: bool = true;
        // C s_130_3: cast zx s_130_2 -> bv
        let s_130_3: Bits = Bits::new(s_130_2 as u128, 1u16);
        // D s_130_4: cmp-eq s_130_1 s_130_3
        let s_130_4: bool = ((s_130_1) == (s_130_3));
        // D s_130_5: write-var gs#122342 <= s_130_4
        fn_state.gs_122342 = s_130_4;
        // N s_130_6: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_131_0: const #1u : u8
        let s_131_0: bool = true;
        // D s_131_1: write-var gs#122341 <= s_131_0
        fn_state.gs_122341 = s_131_0;
        // N s_131_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_132_0: const #432u : u32
        let s_132_0: u32 = 432;
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
        // D s_132_4: write-var gs#122338 <= s_132_3
        fn_state.gs_122338 = s_132_3;
        // N s_132_5: jump b67
        return block_67(state, tracer, fn_state);
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
        // D s_134_0: read-var __CPTR_EL3_TAM:u8
        let s_134_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_134_1: cast zx s_134_0 -> bv
        let s_134_1: Bits = Bits::new(s_134_0 as u128, 1u16);
        // C s_134_2: const #1u : u8
        let s_134_2: bool = true;
        // C s_134_3: cast zx s_134_2 -> bv
        let s_134_3: Bits = Bits::new(s_134_2 as u128, 1u16);
        // D s_134_4: cmp-eq s_134_1 s_134_3
        let s_134_4: bool = ((s_134_1) == (s_134_3));
        // D s_134_5: write-var gs#122337 <= s_134_4
        fn_state.gs_122337 = s_134_4;
        // N s_134_6: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_135_0: const #424u : u32
        let s_135_0: u32 = 424;
        // D s_135_1: read-reg s_135_0:u8
        let s_135_1: u8 = {
            let value = state.read_register::<u8>(s_135_0 as isize);
            tracer.read_register(s_135_0 as isize, value);
            value
        };
        // D s_135_2: call ELUsingAArch32(s_135_1)
        let s_135_2: bool = ELUsingAArch32(state, tracer, s_135_1);
        // D s_135_3: not s_135_2
        let s_135_3: bool = !s_135_2;
        // D s_135_4: write-var gs#122336 <= s_135_3
        fn_state.gs_122336 = s_135_3;
        // N s_135_5: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_136_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_136_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_136_1: call __IMPDEF_boolean(s_136_0)
        let s_136_1: bool = u__IMPDEF_boolean(state, tracer, s_136_0);
        // D s_136_2: write-var gs#122335 <= s_136_1
        fn_state.gs_122335 = s_136_1;
        // N s_136_3: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_137_0: const #() : ()
        let s_137_0: () = ();
        // S s_137_1: call EDSCR_read(s_137_0)
        let s_137_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_137_0);
        // S s_137_2: call _get_EDSCR_Type_SDD(s_137_1)
        let s_137_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_137_1);
        // S s_137_3: cast zx s_137_2 -> bv
        let s_137_3: Bits = Bits::new(s_137_2 as u128, 1u16);
        // C s_137_4: const #1u : u8
        let s_137_4: bool = true;
        // C s_137_5: cast zx s_137_4 -> bv
        let s_137_5: Bits = Bits::new(s_137_4 as u128, 1u16);
        // S s_137_6: cmp-eq s_137_3 s_137_5
        let s_137_6: bool = ((s_137_3) == (s_137_5));
        // D s_137_7: write-var gs#122334 <= s_137_6
        fn_state.gs_122334 = s_137_6;
        // N s_137_8: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_138_0: const #424u : u32
        let s_138_0: u32 = 424;
        // D s_138_1: read-reg s_138_0:u8
        let s_138_1: u8 = {
            let value = state.read_register::<u8>(s_138_0 as isize);
            tracer.read_register(s_138_0 as isize, value);
            value
        };
        // C s_138_2: const #2u : u8
        let s_138_2: u8 = 2;
        // D s_138_3: cmp-lt s_138_1 s_138_2
        let s_138_3: bool = ((s_138_1) < (s_138_2));
        // D s_138_4: write-var gs#122333 <= s_138_3
        fn_state.gs_122333 = s_138_3;
        // N s_138_5: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_139_0: const #() : ()
        let s_139_0: () = ();
        // S s_139_1: call Halted(s_139_0)
        let s_139_1: bool = Halted(state, tracer, s_139_0);
        // N s_139_2: branch s_139_1 b274 b140
        if s_139_1 {
            return block_274(state, tracer, fn_state);
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
        // D s_140_1: write-var gs#122381 <= s_140_0
        fn_state.gs_122381 = s_140_0;
        // N s_140_2: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_141_0: read-var gs#122381:u8
        let s_141_0: bool = fn_state.gs_122381;
        // N s_141_1: branch s_141_0 b273 b142
        if s_141_0 {
            return block_273(state, tracer, fn_state);
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
        // D s_142_1: write-var gs#122382 <= s_142_0
        fn_state.gs_122382 = s_142_0;
        // N s_142_2: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_143_0: read-var gs#122382:u8
        let s_143_0: bool = fn_state.gs_122382;
        // N s_143_1: branch s_143_0 b272 b144
        if s_143_0 {
            return block_272(state, tracer, fn_state);
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
        // D s_144_1: write-var gs#122383 <= s_144_0
        fn_state.gs_122383 = s_144_0;
        // N s_144_2: jump b145
        return block_145(state, tracer, fn_state);
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_145_0: read-var gs#122383:u8
        let s_145_0: bool = fn_state.gs_122383;
        // N s_145_1: branch s_145_0 b271 b146
        if s_145_0 {
            return block_271(state, tracer, fn_state);
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
        // D s_146_1: write-var gs#122384 <= s_146_0
        fn_state.gs_122384 = s_146_0;
        // N s_146_2: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_147_0: read-var gs#122384:u8
        let s_147_0: bool = fn_state.gs_122384;
        // N s_147_1: branch s_147_0 b270 b148
        if s_147_0 {
            return block_270(state, tracer, fn_state);
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
        // D s_148_1: write-var gs#122385 <= s_148_0
        fn_state.gs_122385 = s_148_0;
        // N s_148_2: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_149_0: read-var gs#122385:u8
        let s_149_0: bool = fn_state.gs_122385;
        // N s_149_1: branch s_149_0 b269 b150
        if s_149_0 {
            return block_269(state, tracer, fn_state);
        } else {
            return block_150(state, tracer, fn_state);
        };
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_150_0: const #440u : u32
        let s_150_0: u32 = 440;
        // D s_150_1: read-reg s_150_0:u8
        let s_150_1: u8 = {
            let value = state.read_register::<u8>(s_150_0 as isize);
            tracer.read_register(s_150_0 as isize, value);
            value
        };
        // D s_150_2: call ELUsingAArch32(s_150_1)
        let s_150_2: bool = ELUsingAArch32(state, tracer, s_150_1);
        // D s_150_3: not s_150_2
        let s_150_3: bool = !s_150_2;
        // N s_150_4: branch s_150_3 b268 b151
        if s_150_3 {
            return block_268(state, tracer, fn_state);
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
        // D s_151_1: write-var gs#122386 <= s_151_0
        fn_state.gs_122386 = s_151_0;
        // N s_151_2: jump b152
        return block_152(state, tracer, fn_state);
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_152_0: read-var gs#122386:u8
        let s_152_0: bool = fn_state.gs_122386;
        // N s_152_1: branch s_152_0 b259 b153
        if s_152_0 {
            return block_259(state, tracer, fn_state);
        } else {
            return block_153(state, tracer, fn_state);
        };
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_153_0: const #440u : u32
        let s_153_0: u32 = 440;
        // D s_153_1: read-reg s_153_0:u8
        let s_153_1: u8 = {
            let value = state.read_register::<u8>(s_153_0 as isize);
            tracer.read_register(s_153_0 as isize, value);
            value
        };
        // D s_153_2: call ELUsingAArch32(s_153_1)
        let s_153_2: bool = ELUsingAArch32(state, tracer, s_153_1);
        // N s_153_3: branch s_153_2 b258 b154
        if s_153_2 {
            return block_258(state, tracer, fn_state);
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
        // D s_154_1: write-var gs#122387 <= s_154_0
        fn_state.gs_122387 = s_154_0;
        // N s_154_2: jump b155
        return block_155(state, tracer, fn_state);
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_155_0: read-var gs#122387:u8
        let s_155_0: bool = fn_state.gs_122387;
        // N s_155_1: branch s_155_0 b241 b156
        if s_155_0 {
            return block_241(state, tracer, fn_state);
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
        // N s_156_2: branch s_156_1 b240 b157
        if s_156_1 {
            return block_240(state, tracer, fn_state);
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
        // D s_157_1: write-var gs#122388 <= s_157_0
        fn_state.gs_122388 = s_157_0;
        // N s_157_2: jump b158
        return block_158(state, tracer, fn_state);
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_158_0: read-var gs#122388:u8
        let s_158_0: bool = fn_state.gs_122388;
        // N s_158_1: branch s_158_0 b239 b159
        if s_158_0 {
            return block_239(state, tracer, fn_state);
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
        // D s_159_1: write-var gs#122389 <= s_159_0
        fn_state.gs_122389 = s_159_0;
        // N s_159_2: jump b160
        return block_160(state, tracer, fn_state);
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_160_0: read-var gs#122389:u8
        let s_160_0: bool = fn_state.gs_122389;
        // N s_160_1: branch s_160_0 b238 b161
        if s_160_0 {
            return block_238(state, tracer, fn_state);
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
        // D s_161_1: write-var gs#122392 <= s_161_0
        fn_state.gs_122392 = s_161_0;
        // N s_161_2: jump b162
        return block_162(state, tracer, fn_state);
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_162_0: read-var gs#122392:u8
        let s_162_0: bool = fn_state.gs_122392;
        // N s_162_1: branch s_162_0 b237 b163
        if s_162_0 {
            return block_237(state, tracer, fn_state);
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
        // D s_163_1: write-var gs#122393 <= s_163_0
        fn_state.gs_122393 = s_163_0;
        // N s_163_2: jump b164
        return block_164(state, tracer, fn_state);
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_164_0: read-var gs#122393:u8
        let s_164_0: bool = fn_state.gs_122393;
        // N s_164_1: branch s_164_0 b236 b165
        if s_164_0 {
            return block_236(state, tracer, fn_state);
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
        // S s_165_1: call EL2Enabled(s_165_0)
        let s_165_1: bool = EL2Enabled(state, tracer, s_165_0);
        // N s_165_2: branch s_165_1 b235 b166
        if s_165_1 {
            return block_235(state, tracer, fn_state);
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
        // D s_166_1: write-var gs#122394 <= s_166_0
        fn_state.gs_122394 = s_166_0;
        // N s_166_2: jump b167
        return block_167(state, tracer, fn_state);
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_167_0: read-var gs#122394:u8
        let s_167_0: bool = fn_state.gs_122394;
        // N s_167_1: branch s_167_0 b234 b168
        if s_167_0 {
            return block_234(state, tracer, fn_state);
        } else {
            return block_168(state, tracer, fn_state);
        };
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_168_0: const #0u : u8
        let s_168_0: bool = false;
        // D s_168_1: write-var gs#122397 <= s_168_0
        fn_state.gs_122397 = s_168_0;
        // N s_168_2: jump b169
        return block_169(state, tracer, fn_state);
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_169_0: read-var gs#122397:u8
        let s_169_0: bool = fn_state.gs_122397;
        // N s_169_1: branch s_169_0 b233 b170
        if s_169_0 {
            return block_233(state, tracer, fn_state);
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
        // D s_170_1: write-var gs#122398 <= s_170_0
        fn_state.gs_122398 = s_170_0;
        // N s_170_2: jump b171
        return block_171(state, tracer, fn_state);
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_171_0: read-var gs#122398:u8
        let s_171_0: bool = fn_state.gs_122398;
        // N s_171_1: branch s_171_0 b232 b172
        if s_171_0 {
            return block_232(state, tracer, fn_state);
        } else {
            return block_172(state, tracer, fn_state);
        };
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_172_0: const #() : ()
        let s_172_0: () = ();
        // S s_172_1: call EL2Enabled(s_172_0)
        let s_172_1: bool = EL2Enabled(state, tracer, s_172_0);
        // N s_172_2: branch s_172_1 b231 b173
        if s_172_1 {
            return block_231(state, tracer, fn_state);
        } else {
            return block_173(state, tracer, fn_state);
        };
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_173_0: const #0u : u8
        let s_173_0: bool = false;
        // D s_173_1: write-var gs#122399 <= s_173_0
        fn_state.gs_122399 = s_173_0;
        // N s_173_2: jump b174
        return block_174(state, tracer, fn_state);
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_174_0: read-var gs#122399:u8
        let s_174_0: bool = fn_state.gs_122399;
        // N s_174_1: branch s_174_0 b230 b175
        if s_174_0 {
            return block_230(state, tracer, fn_state);
        } else {
            return block_175(state, tracer, fn_state);
        };
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_175_0: const #0u : u8
        let s_175_0: bool = false;
        // D s_175_1: write-var gs#122400 <= s_175_0
        fn_state.gs_122400 = s_175_0;
        // N s_175_2: jump b176
        return block_176(state, tracer, fn_state);
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_176_0: read-var gs#122400:u8
        let s_176_0: bool = fn_state.gs_122400;
        // N s_176_1: branch s_176_0 b229 b177
        if s_176_0 {
            return block_229(state, tracer, fn_state);
        } else {
            return block_177(state, tracer, fn_state);
        };
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_177_0: const #() : ()
        let s_177_0: () = ();
        // S s_177_1: call EL2Enabled(s_177_0)
        let s_177_1: bool = EL2Enabled(state, tracer, s_177_0);
        // N s_177_2: branch s_177_1 b228 b178
        if s_177_1 {
            return block_228(state, tracer, fn_state);
        } else {
            return block_178(state, tracer, fn_state);
        };
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_178_0: const #0u : u8
        let s_178_0: bool = false;
        // D s_178_1: write-var gs#122401 <= s_178_0
        fn_state.gs_122401 = s_178_0;
        // N s_178_2: jump b179
        return block_179(state, tracer, fn_state);
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_179_0: read-var gs#122401:u8
        let s_179_0: bool = fn_state.gs_122401;
        // N s_179_1: branch s_179_0 b227 b180
        if s_179_0 {
            return block_227(state, tracer, fn_state);
        } else {
            return block_180(state, tracer, fn_state);
        };
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_180_0: const #0u : u8
        let s_180_0: bool = false;
        // D s_180_1: write-var gs#122402 <= s_180_0
        fn_state.gs_122402 = s_180_0;
        // N s_180_2: jump b181
        return block_181(state, tracer, fn_state);
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_181_0: read-var gs#122402:u8
        let s_181_0: bool = fn_state.gs_122402;
        // N s_181_1: branch s_181_0 b226 b182
        if s_181_0 {
            return block_226(state, tracer, fn_state);
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
        // S s_182_1: call EL2Enabled(s_182_0)
        let s_182_1: bool = EL2Enabled(state, tracer, s_182_0);
        // N s_182_2: branch s_182_1 b225 b183
        if s_182_1 {
            return block_225(state, tracer, fn_state);
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
        // D s_183_1: write-var gs#122403 <= s_183_0
        fn_state.gs_122403 = s_183_0;
        // N s_183_2: jump b184
        return block_184(state, tracer, fn_state);
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_184_0: read-var gs#122403:u8
        let s_184_0: bool = fn_state.gs_122403;
        // N s_184_1: branch s_184_0 b224 b185
        if s_184_0 {
            return block_224(state, tracer, fn_state);
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
        // D s_185_1: write-var gs#122404 <= s_185_0
        fn_state.gs_122404 = s_185_0;
        // N s_185_2: jump b186
        return block_186(state, tracer, fn_state);
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_186_0: read-var gs#122404:u8
        let s_186_0: bool = fn_state.gs_122404;
        // N s_186_1: branch s_186_0 b223 b187
        if s_186_0 {
            return block_223(state, tracer, fn_state);
        } else {
            return block_187(state, tracer, fn_state);
        };
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_187_0: const #0u : u8
        let s_187_0: bool = false;
        // D s_187_1: write-var gs#122405 <= s_187_0
        fn_state.gs_122405 = s_187_0;
        // N s_187_2: jump b188
        return block_188(state, tracer, fn_state);
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_188_0: read-var gs#122405:u8
        let s_188_0: bool = fn_state.gs_122405;
        // N s_188_1: branch s_188_0 b219 b189
        if s_188_0 {
            return block_219(state, tracer, fn_state);
        } else {
            return block_189(state, tracer, fn_state);
        };
    }
    fn block_189<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_189_0: const #0u : u8
        let s_189_0: bool = false;
        // D s_189_1: write-var gs#122407 <= s_189_0
        fn_state.gs_122407 = s_189_0;
        // N s_189_2: jump b190
        return block_190(state, tracer, fn_state);
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_190_0: read-var gs#122407:u8
        let s_190_0: bool = fn_state.gs_122407;
        // N s_190_1: branch s_190_0 b218 b191
        if s_190_0 {
            return block_218(state, tracer, fn_state);
        } else {
            return block_191(state, tracer, fn_state);
        };
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_191_0: const #0u : u8
        let s_191_0: bool = false;
        // D s_191_1: write-var gs#122408 <= s_191_0
        fn_state.gs_122408 = s_191_0;
        // N s_191_2: jump b192
        return block_192(state, tracer, fn_state);
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_192_0: read-var gs#122408:u8
        let s_192_0: bool = fn_state.gs_122408;
        // N s_192_1: branch s_192_0 b217 b193
        if s_192_0 {
            return block_217(state, tracer, fn_state);
        } else {
            return block_193(state, tracer, fn_state);
        };
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_193_0: const #424u : u32
        let s_193_0: u32 = 424;
        // D s_193_1: read-reg s_193_0:u8
        let s_193_1: u8 = {
            let value = state.read_register::<u8>(s_193_0 as isize);
            tracer.read_register(s_193_0 as isize, value);
            value
        };
        // C s_193_2: const #2u : u8
        let s_193_2: u8 = 2;
        // D s_193_3: cmp-lt s_193_1 s_193_2
        let s_193_3: bool = ((s_193_1) < (s_193_2));
        // N s_193_4: branch s_193_3 b216 b194
        if s_193_3 {
            return block_216(state, tracer, fn_state);
        } else {
            return block_194(state, tracer, fn_state);
        };
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_194_0: const #0u : u8
        let s_194_0: bool = false;
        // D s_194_1: write-var gs#122409 <= s_194_0
        fn_state.gs_122409 = s_194_0;
        // N s_194_2: jump b195
        return block_195(state, tracer, fn_state);
    }
    fn block_195<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_195_0: read-var gs#122409:u8
        let s_195_0: bool = fn_state.gs_122409;
        // N s_195_1: branch s_195_0 b215 b196
        if s_195_0 {
            return block_215(state, tracer, fn_state);
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
        // D s_196_1: write-var gs#122410 <= s_196_0
        fn_state.gs_122410 = s_196_0;
        // N s_196_2: jump b197
        return block_197(state, tracer, fn_state);
    }
    fn block_197<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_197_0: read-var gs#122410:u8
        let s_197_0: bool = fn_state.gs_122410;
        // N s_197_1: branch s_197_0 b209 b198
        if s_197_0 {
            return block_209(state, tracer, fn_state);
        } else {
            return block_198(state, tracer, fn_state);
        };
    }
    fn block_198<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_198_0: const #() : ()
        let s_198_0: () = ();
        // S s_198_1: call HaveAArch64(s_198_0)
        let s_198_1: bool = HaveAArch64(state, tracer, s_198_0);
        // N s_198_2: branch s_198_1 b208 b199
        if s_198_1 {
            return block_208(state, tracer, fn_state);
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
        // D s_199_1: write-var gs#122411 <= s_199_0
        fn_state.gs_122411 = s_199_0;
        // N s_199_2: jump b200
        return block_200(state, tracer, fn_state);
    }
    fn block_200<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_200_0: read-var gs#122411:u8
        let s_200_0: bool = fn_state.gs_122411;
        // N s_200_1: branch s_200_0 b207 b201
        if s_200_0 {
            return block_207(state, tracer, fn_state);
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
        // S s_201_1: call HaveAArch64(s_201_0)
        let s_201_1: bool = HaveAArch64(state, tracer, s_201_0);
        // S s_201_2: not s_201_1
        let s_201_2: bool = !s_201_1;
        // N s_201_3: branch s_201_2 b206 b202
        if s_201_2 {
            return block_206(state, tracer, fn_state);
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
        // D s_202_1: write-var gs#122412 <= s_202_0
        fn_state.gs_122412 = s_202_0;
        // N s_202_2: jump b203
        return block_203(state, tracer, fn_state);
    }
    fn block_203<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_203_0: read-var gs#122412:u8
        let s_203_0: bool = fn_state.gs_122412;
        // N s_203_1: branch s_203_0 b205 b204
        if s_203_0 {
            return block_205(state, tracer, fn_state);
        } else {
            return block_204(state, tracer, fn_state);
        };
    }
    fn block_204<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_204_0: const #14s : i64
        let s_204_0: i64 = 14;
        // S s_204_1: call AMEVCNTR1_read(s_204_0)
        let s_204_1: ProductType5c790c8ef59cc8b2 = AMEVCNTR1_read(
            state,
            tracer,
            s_204_0,
        );
        // D s_204_2: write-var ga#203326 <= s_204_1
        fn_state.ga_203326 = s_204_1;
        // D s_204_3: read-var ga#203326.0:struct
        let s_204_3: u64 = fn_state.ga_203326._0;
        // C s_204_4: const #32s : i
        let s_204_4: i128 = 32;
        // D s_204_5: cast zx s_204_3 -> bv
        let s_204_5: Bits = Bits::new(s_204_3 as u128, 64u16);
        // C s_204_6: const #1s : i64
        let s_204_6: i64 = 1;
        // C s_204_7: cast zx s_204_6 -> i
        let s_204_7: i128 = (i128::try_from(s_204_6).unwrap());
        // C s_204_8: const #31s : i
        let s_204_8: i128 = 31;
        // C s_204_9: add s_204_8 s_204_7
        let s_204_9: i128 = (s_204_8 + s_204_7);
        // D s_204_10: bit-extract s_204_5 s_204_4 s_204_9
        let s_204_10: Bits = (Bits::new(
            ((s_204_5) >> (s_204_4)).value(),
            u16::try_from(s_204_9).unwrap(),
        ));
        // D s_204_11: cast reint s_204_10 -> u32
        let s_204_11: u32 = (s_204_10.value() as u32);
        // C s_204_12: const #14s : i64
        let s_204_12: i64 = 14;
        // S s_204_13: call AMEVCNTR1_read(s_204_12)
        let s_204_13: ProductType5c790c8ef59cc8b2 = AMEVCNTR1_read(
            state,
            tracer,
            s_204_12,
        );
        // D s_204_14: write-var ga#203328 <= s_204_13
        fn_state.ga_203328 = s_204_13;
        // D s_204_15: read-var ga#203328.0:struct
        let s_204_15: u64 = fn_state.ga_203328._0;
        // C s_204_16: const #0s : i
        let s_204_16: i128 = 0;
        // D s_204_17: cast zx s_204_15 -> bv
        let s_204_17: Bits = Bits::new(s_204_15 as u128, 64u16);
        // C s_204_18: const #1s : i64
        let s_204_18: i64 = 1;
        // C s_204_19: cast zx s_204_18 -> i
        let s_204_19: i128 = (i128::try_from(s_204_18).unwrap());
        // C s_204_20: const #31s : i
        let s_204_20: i128 = 31;
        // C s_204_21: add s_204_20 s_204_19
        let s_204_21: i128 = (s_204_20 + s_204_19);
        // D s_204_22: bit-extract s_204_17 s_204_16 s_204_21
        let s_204_22: Bits = (Bits::new(
            ((s_204_17) >> (s_204_16)).value(),
            u16::try_from(s_204_21).unwrap(),
        ));
        // D s_204_23: cast reint s_204_22 -> u32
        let s_204_23: u32 = (s_204_22.value() as u32);
        // D s_204_24: create-product struct = ["s_204_11", "s_204_23"]
        let s_204_24: ProductType72d61775f103f7e0 = ProductType72d61775f103f7e0 {
            _0: s_204_11,
            _1: s_204_23,
        };
        // D s_204_25: write-var ga#203332 <= s_204_24
        fn_state.ga_203332 = s_204_24;
        // D s_204_26: read-var ga#203332.0:struct
        let s_204_26: u32 = fn_state.ga_203332._0;
        // D s_204_27: read-var ga#203332.1:struct
        let s_204_27: u32 = fn_state.ga_203332._1;
        // D s_204_28: read-var t2:i
        let s_204_28: i128 = fn_state.t2;
        // D s_204_29: call R_set(s_204_28, s_204_26)
        let s_204_29: () = R_set(state, tracer, s_204_28, s_204_26);
        // D s_204_30: read-var t:i
        let s_204_30: i128 = fn_state.t;
        // D s_204_31: call R_set(s_204_30, s_204_27)
        let s_204_31: () = R_set(state, tracer, s_204_30, s_204_27);
        // N s_204_32: return
        return;
    }
    fn block_205<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_205_0: const #32s : i
        let s_205_0: i128 = 32;
        // S s_205_1: call Zeros(s_205_0)
        let s_205_1: Bits = Zeros(state, tracer, s_205_0);
        // S s_205_2: cast reint s_205_1 -> u32
        let s_205_2: u32 = (s_205_1.value() as u32);
        // C s_205_3: const #32s : i
        let s_205_3: i128 = 32;
        // S s_205_4: call Zeros(s_205_3)
        let s_205_4: Bits = Zeros(state, tracer, s_205_3);
        // S s_205_5: cast reint s_205_4 -> u32
        let s_205_5: u32 = (s_205_4.value() as u32);
        // D s_205_6: create-product struct = ["s_205_2", "s_205_5"]
        let s_205_6: ProductType72d61775f103f7e0 = ProductType72d61775f103f7e0 {
            _0: s_205_2,
            _1: s_205_5,
        };
        // D s_205_7: write-var ga#203325 <= s_205_6
        fn_state.ga_203325 = s_205_6;
        // D s_205_8: read-var ga#203325.0:struct
        let s_205_8: u32 = fn_state.ga_203325._0;
        // D s_205_9: read-var ga#203325.1:struct
        let s_205_9: u32 = fn_state.ga_203325._1;
        // D s_205_10: read-var t2:i
        let s_205_10: i128 = fn_state.t2;
        // D s_205_11: call R_set(s_205_10, s_205_8)
        let s_205_11: () = R_set(state, tracer, s_205_10, s_205_8);
        // D s_205_12: read-var t:i
        let s_205_12: i128 = fn_state.t;
        // D s_205_13: call R_set(s_205_12, s_205_9)
        let s_205_13: () = R_set(state, tracer, s_205_12, s_205_9);
        // N s_205_14: return
        return;
    }
    fn block_206<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_206_0: read-var __AMCR_CG1RZ:u8
        let s_206_0: bool = fn_state.u__AMCR_CG1RZ;
        // D s_206_1: cast zx s_206_0 -> bv
        let s_206_1: Bits = Bits::new(s_206_0 as u128, 1u16);
        // C s_206_2: const #1u : u8
        let s_206_2: bool = true;
        // C s_206_3: cast zx s_206_2 -> bv
        let s_206_3: Bits = Bits::new(s_206_2 as u128, 1u16);
        // D s_206_4: cmp-eq s_206_1 s_206_3
        let s_206_4: bool = ((s_206_1) == (s_206_3));
        // D s_206_5: write-var gs#122412 <= s_206_4
        fn_state.gs_122412 = s_206_4;
        // N s_206_6: jump b203
        return block_203(state, tracer, fn_state);
    }
    fn block_207<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_207_0: const #32s : i
        let s_207_0: i128 = 32;
        // S s_207_1: call Zeros(s_207_0)
        let s_207_1: Bits = Zeros(state, tracer, s_207_0);
        // S s_207_2: cast reint s_207_1 -> u32
        let s_207_2: u32 = (s_207_1.value() as u32);
        // C s_207_3: const #32s : i
        let s_207_3: i128 = 32;
        // S s_207_4: call Zeros(s_207_3)
        let s_207_4: Bits = Zeros(state, tracer, s_207_3);
        // S s_207_5: cast reint s_207_4 -> u32
        let s_207_5: u32 = (s_207_4.value() as u32);
        // D s_207_6: create-product struct = ["s_207_2", "s_207_5"]
        let s_207_6: ProductType72d61775f103f7e0 = ProductType72d61775f103f7e0 {
            _0: s_207_2,
            _1: s_207_5,
        };
        // D s_207_7: write-var ga#203319 <= s_207_6
        fn_state.ga_203319 = s_207_6;
        // D s_207_8: read-var ga#203319.0:struct
        let s_207_8: u32 = fn_state.ga_203319._0;
        // D s_207_9: read-var ga#203319.1:struct
        let s_207_9: u32 = fn_state.ga_203319._1;
        // D s_207_10: read-var t2:i
        let s_207_10: i128 = fn_state.t2;
        // D s_207_11: call R_set(s_207_10, s_207_8)
        let s_207_11: () = R_set(state, tracer, s_207_10, s_207_8);
        // D s_207_12: read-var t:i
        let s_207_12: i128 = fn_state.t;
        // D s_207_13: call R_set(s_207_12, s_207_9)
        let s_207_13: () = R_set(state, tracer, s_207_12, s_207_9);
        // N s_207_14: return
        return;
    }
    fn block_208<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_208_0: read-var __AMCR_EL0_CG1RZ:u8
        let s_208_0: bool = fn_state.u__AMCR_EL0_CG1RZ;
        // D s_208_1: cast zx s_208_0 -> bv
        let s_208_1: Bits = Bits::new(s_208_0 as u128, 1u16);
        // C s_208_2: const #1u : u8
        let s_208_2: bool = true;
        // C s_208_3: cast zx s_208_2 -> bv
        let s_208_3: Bits = Bits::new(s_208_2 as u128, 1u16);
        // D s_208_4: cmp-eq s_208_1 s_208_3
        let s_208_4: bool = ((s_208_1) == (s_208_3));
        // D s_208_5: write-var gs#122411 <= s_208_4
        fn_state.gs_122411 = s_208_4;
        // N s_208_6: jump b200
        return block_200(state, tracer, fn_state);
    }
    fn block_209<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_209_0: const #() : ()
        let s_209_0: () = ();
        // S s_209_1: call Halted(s_209_0)
        let s_209_1: bool = Halted(state, tracer, s_209_0);
        // N s_209_2: branch s_209_1 b214 b210
        if s_209_1 {
            return block_214(state, tracer, fn_state);
        } else {
            return block_210(state, tracer, fn_state);
        };
    }
    fn block_210<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_210_0: const #0u : u8
        let s_210_0: bool = false;
        // D s_210_1: write-var gs#122435 <= s_210_0
        fn_state.gs_122435 = s_210_0;
        // N s_210_2: jump b211
        return block_211(state, tracer, fn_state);
    }
    fn block_211<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_211_0: read-var gs#122435:u8
        let s_211_0: bool = fn_state.gs_122435;
        // N s_211_1: branch s_211_0 b213 b212
        if s_211_0 {
            return block_213(state, tracer, fn_state);
        } else {
            return block_212(state, tracer, fn_state);
        };
    }
    fn block_212<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_212_0: const #4u : u8
        let s_212_0: u8 = 4;
        // C s_212_1: cast zx s_212_0 -> bv
        let s_212_1: Bits = Bits::new(s_212_0 as u128, 8u16);
        // C s_212_2: cast zx s_212_1 -> i
        let s_212_2: i128 = (s_212_1.value() as i128);
        // C s_212_3: cast reint s_212_2 -> i64
        let s_212_3: i64 = (s_212_2 as i64);
        // C s_212_4: cast zx s_212_3 -> i
        let s_212_4: i128 = (i128::try_from(s_212_3).unwrap());
        // C s_212_5: const #424u : u32
        let s_212_5: u32 = 424;
        // D s_212_6: read-reg s_212_5:u8
        let s_212_6: u8 = {
            let value = state.read_register::<u8>(s_212_5 as isize);
            tracer.read_register(s_212_5 as isize, value);
            value
        };
        // D s_212_7: call AArch64_AArch32SystemAccessTrap(s_212_6, s_212_4)
        let s_212_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_212_6,
            s_212_4,
        );
        // N s_212_8: return
        return;
    }
    fn block_213<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_213_0: panic
        panic!("{:?}", ());
        // N s_213_1: return
        return;
    }
    fn block_214<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_214_0: const #() : ()
        let s_214_0: () = ();
        // S s_214_1: call EDSCR_read(s_214_0)
        let s_214_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_214_0);
        // S s_214_2: call _get_EDSCR_Type_SDD(s_214_1)
        let s_214_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_214_1);
        // S s_214_3: cast zx s_214_2 -> bv
        let s_214_3: Bits = Bits::new(s_214_2 as u128, 1u16);
        // C s_214_4: const #1u : u8
        let s_214_4: bool = true;
        // C s_214_5: cast zx s_214_4 -> bv
        let s_214_5: Bits = Bits::new(s_214_4 as u128, 1u16);
        // S s_214_6: cmp-eq s_214_3 s_214_5
        let s_214_6: bool = ((s_214_3) == (s_214_5));
        // D s_214_7: write-var gs#122435 <= s_214_6
        fn_state.gs_122435 = s_214_6;
        // N s_214_8: jump b211
        return block_211(state, tracer, fn_state);
    }
    fn block_215<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_215_0: read-var __CPTR_EL3_TAM:u8
        let s_215_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_215_1: cast zx s_215_0 -> bv
        let s_215_1: Bits = Bits::new(s_215_0 as u128, 1u16);
        // C s_215_2: const #1u : u8
        let s_215_2: bool = true;
        // C s_215_3: cast zx s_215_2 -> bv
        let s_215_3: Bits = Bits::new(s_215_2 as u128, 1u16);
        // D s_215_4: cmp-eq s_215_1 s_215_3
        let s_215_4: bool = ((s_215_1) == (s_215_3));
        // D s_215_5: write-var gs#122410 <= s_215_4
        fn_state.gs_122410 = s_215_4;
        // N s_215_6: jump b197
        return block_197(state, tracer, fn_state);
    }
    fn block_216<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_216_0: const #424u : u32
        let s_216_0: u32 = 424;
        // D s_216_1: read-reg s_216_0:u8
        let s_216_1: u8 = {
            let value = state.read_register::<u8>(s_216_0 as isize);
            tracer.read_register(s_216_0 as isize, value);
            value
        };
        // D s_216_2: call ELUsingAArch32(s_216_1)
        let s_216_2: bool = ELUsingAArch32(state, tracer, s_216_1);
        // D s_216_3: not s_216_2
        let s_216_3: bool = !s_216_2;
        // D s_216_4: write-var gs#122409 <= s_216_3
        fn_state.gs_122409 = s_216_3;
        // N s_216_5: jump b195
        return block_195(state, tracer, fn_state);
    }
    fn block_217<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_217_0: const #4u : u8
        let s_217_0: u8 = 4;
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
        // D s_218_0: read-var __HAFGRTR_EL2_AMEVCNTR114_EL0:u8
        let s_218_0: bool = fn_state.u__HAFGRTR_EL2_AMEVCNTR114_EL0;
        // D s_218_1: cast zx s_218_0 -> bv
        let s_218_1: Bits = Bits::new(s_218_0 as u128, 1u16);
        // C s_218_2: const #1u : u8
        let s_218_2: bool = true;
        // C s_218_3: cast zx s_218_2 -> bv
        let s_218_3: Bits = Bits::new(s_218_2 as u128, 1u16);
        // D s_218_4: cmp-eq s_218_1 s_218_3
        let s_218_4: bool = ((s_218_1) == (s_218_3));
        // D s_218_5: write-var gs#122408 <= s_218_4
        fn_state.gs_122408 = s_218_4;
        // N s_218_6: jump b192
        return block_192(state, tracer, fn_state);
    }
    fn block_219<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_219_0: const #424u : u32
        let s_219_0: u32 = 424;
        // D s_219_1: read-reg s_219_0:u8
        let s_219_1: u8 = {
            let value = state.read_register::<u8>(s_219_0 as isize);
            tracer.read_register(s_219_0 as isize, value);
            value
        };
        // C s_219_2: const #2u : u8
        let s_219_2: u8 = 2;
        // D s_219_3: cmp-lt s_219_1 s_219_2
        let s_219_3: bool = ((s_219_1) < (s_219_2));
        // D s_219_4: not s_219_3
        let s_219_4: bool = !s_219_3;
        // N s_219_5: branch s_219_4 b222 b220
        if s_219_4 {
            return block_222(state, tracer, fn_state);
        } else {
            return block_220(state, tracer, fn_state);
        };
    }
    fn block_220<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_220_0: read-var __SCR_EL3_FGTEn:u8
        let s_220_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_220_1: cast zx s_220_0 -> bv
        let s_220_1: Bits = Bits::new(s_220_0 as u128, 1u16);
        // C s_220_2: const #1u : u8
        let s_220_2: bool = true;
        // C s_220_3: cast zx s_220_2 -> bv
        let s_220_3: Bits = Bits::new(s_220_2 as u128, 1u16);
        // D s_220_4: cmp-eq s_220_1 s_220_3
        let s_220_4: bool = ((s_220_1) == (s_220_3));
        // D s_220_5: write-var gs#122406 <= s_220_4
        fn_state.gs_122406 = s_220_4;
        // N s_220_6: jump b221
        return block_221(state, tracer, fn_state);
    }
    fn block_221<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_221_0: read-var gs#122406:u8
        let s_221_0: bool = fn_state.gs_122406;
        // D s_221_1: write-var gs#122407 <= s_221_0
        fn_state.gs_122407 = s_221_0;
        // N s_221_2: jump b190
        return block_190(state, tracer, fn_state);
    }
    fn block_222<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_222_0: const #1u : u8
        let s_222_0: bool = true;
        // D s_222_1: write-var gs#122406 <= s_222_0
        fn_state.gs_122406 = s_222_0;
        // N s_222_2: jump b221
        return block_221(state, tracer, fn_state);
    }
    fn block_223<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_223_0: const #146u : u32
        let s_223_0: u32 = 146;
        // S s_223_1: call IsFeatureImplemented(s_223_0)
        let s_223_1: bool = IsFeatureImplemented(state, tracer, s_223_0);
        // D s_223_2: write-var gs#122405 <= s_223_1
        fn_state.gs_122405 = s_223_1;
        // N s_223_3: jump b188
        return block_188(state, tracer, fn_state);
    }
    fn block_224<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_224_0: const #102552u : u32
        let s_224_0: u32 = 102552;
        // D s_224_1: read-reg s_224_0:struct
        let s_224_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_224_0 as isize);
            tracer.read_register(s_224_0 as isize, value);
            value
        };
        // D s_224_2: call _get_HCR_EL2_Type_E2H(s_224_1)
        let s_224_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_224_1);
        // C s_224_3: const #102552u : u32
        let s_224_3: u32 = 102552;
        // D s_224_4: read-reg s_224_3:struct
        let s_224_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_224_3 as isize);
            tracer.read_register(s_224_3 as isize, value);
            value
        };
        // D s_224_5: call _get_HCR_EL2_Type_TGE(s_224_4)
        let s_224_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_224_4);
        // D s_224_6: cast zx s_224_2 -> bv
        let s_224_6: Bits = Bits::new(s_224_2 as u128, 1u16);
        // D s_224_7: cast zx s_224_5 -> bv
        let s_224_7: Bits = Bits::new(s_224_5 as u128, 1u16);
        // D s_224_8: cast reint s_224_6 -> u128
        let s_224_8: u128 = (s_224_6.value() as u128);
        // D s_224_9: size-of s_224_6
        let s_224_9: u16 = s_224_6.length();
        // D s_224_10: cast reint s_224_7 -> u128
        let s_224_10: u128 = (s_224_7.value() as u128);
        // D s_224_11: size-of s_224_7
        let s_224_11: u16 = s_224_7.length();
        // D s_224_12: lsl s_224_8 s_224_11
        let s_224_12: u128 = s_224_8 << s_224_11;
        // D s_224_13: or s_224_12 s_224_10
        let s_224_13: u128 = ((s_224_12) | (s_224_10));
        // D s_224_14: add s_224_9 s_224_11
        let s_224_14: u16 = (s_224_9 + s_224_11);
        // D s_224_15: create-bits s_224_13 s_224_14
        let s_224_15: Bits = Bits::new(s_224_13, s_224_14);
        // D s_224_16: cast reint s_224_15 -> u8
        let s_224_16: u8 = (s_224_15.value() as u8);
        // D s_224_17: cast zx s_224_16 -> bv
        let s_224_17: Bits = Bits::new(s_224_16 as u128, 2u16);
        // C s_224_18: const #3u : u8
        let s_224_18: u8 = 3;
        // C s_224_19: cast zx s_224_18 -> bv
        let s_224_19: Bits = Bits::new(s_224_18 as u128, 2u16);
        // D s_224_20: cmp-ne s_224_17 s_224_19
        let s_224_20: bool = ((s_224_17) != (s_224_19));
        // D s_224_21: write-var gs#122404 <= s_224_20
        fn_state.gs_122404 = s_224_20;
        // N s_224_22: jump b186
        return block_186(state, tracer, fn_state);
    }
    fn block_225<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_225_0: const #440u : u32
        let s_225_0: u32 = 440;
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
        // D s_225_4: write-var gs#122403 <= s_225_3
        fn_state.gs_122403 = s_225_3;
        // N s_225_5: jump b184
        return block_184(state, tracer, fn_state);
    }
    fn block_226<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_226_0: const #4u : u8
        let s_226_0: u8 = 4;
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
        // D s_227_0: read-var __HCPTR_TAM:u8
        let s_227_0: bool = fn_state.u__HCPTR_TAM;
        // D s_227_1: cast zx s_227_0 -> bv
        let s_227_1: Bits = Bits::new(s_227_0 as u128, 1u16);
        // C s_227_2: const #1u : u8
        let s_227_2: bool = true;
        // C s_227_3: cast zx s_227_2 -> bv
        let s_227_3: Bits = Bits::new(s_227_2 as u128, 1u16);
        // D s_227_4: cmp-eq s_227_1 s_227_3
        let s_227_4: bool = ((s_227_1) == (s_227_3));
        // D s_227_5: write-var gs#122402 <= s_227_4
        fn_state.gs_122402 = s_227_4;
        // N s_227_6: jump b181
        return block_181(state, tracer, fn_state);
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
        // D s_228_3: write-var gs#122401 <= s_228_2
        fn_state.gs_122401 = s_228_2;
        // N s_228_4: jump b179
        return block_179(state, tracer, fn_state);
    }
    fn block_229<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_229_0: const #4u : u8
        let s_229_0: u8 = 4;
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
        // D s_230_0: read-var __CPTR_EL2_TAM:u8
        let s_230_0: bool = fn_state.u__CPTR_EL2_TAM;
        // D s_230_1: cast zx s_230_0 -> bv
        let s_230_1: Bits = Bits::new(s_230_0 as u128, 1u16);
        // C s_230_2: const #1u : u8
        let s_230_2: bool = true;
        // C s_230_3: cast zx s_230_2 -> bv
        let s_230_3: Bits = Bits::new(s_230_2 as u128, 1u16);
        // D s_230_4: cmp-eq s_230_1 s_230_3
        let s_230_4: bool = ((s_230_1) == (s_230_3));
        // D s_230_5: write-var gs#122400 <= s_230_4
        fn_state.gs_122400 = s_230_4;
        // N s_230_6: jump b176
        return block_176(state, tracer, fn_state);
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
        // D s_231_4: write-var gs#122399 <= s_231_3
        fn_state.gs_122399 = s_231_3;
        // N s_231_5: jump b174
        return block_174(state, tracer, fn_state);
    }
    fn block_232<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_232_0: const #4u : u8
        let s_232_0: u8 = 4;
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
        // D s_233_0: read-var __HSTR_T5:u8
        let s_233_0: bool = fn_state.u__HSTR_T5;
        // D s_233_1: cast zx s_233_0 -> bv
        let s_233_1: Bits = Bits::new(s_233_0 as u128, 1u16);
        // C s_233_2: const #1u : u8
        let s_233_2: bool = true;
        // C s_233_3: cast zx s_233_2 -> bv
        let s_233_3: Bits = Bits::new(s_233_2 as u128, 1u16);
        // D s_233_4: cmp-eq s_233_1 s_233_3
        let s_233_4: bool = ((s_233_1) == (s_233_3));
        // D s_233_5: write-var gs#122398 <= s_233_4
        fn_state.gs_122398 = s_233_4;
        // N s_233_6: jump b171
        return block_171(state, tracer, fn_state);
    }
    fn block_234<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_234_0: const #1u : u8
        let s_234_0: bool = true;
        // D s_234_1: write-var gs#122397 <= s_234_0
        fn_state.gs_122397 = s_234_0;
        // N s_234_2: jump b169
        return block_169(state, tracer, fn_state);
    }
    fn block_235<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_235_0: const #432u : u32
        let s_235_0: u32 = 432;
        // D s_235_1: read-reg s_235_0:u8
        let s_235_1: u8 = {
            let value = state.read_register::<u8>(s_235_0 as isize);
            tracer.read_register(s_235_0 as isize, value);
            value
        };
        // D s_235_2: call ELUsingAArch32(s_235_1)
        let s_235_2: bool = ELUsingAArch32(state, tracer, s_235_1);
        // D s_235_3: write-var gs#122394 <= s_235_2
        fn_state.gs_122394 = s_235_2;
        // N s_235_4: jump b167
        return block_167(state, tracer, fn_state);
    }
    fn block_236<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_236_0: const #4u : u8
        let s_236_0: u8 = 4;
        // C s_236_1: cast zx s_236_0 -> bv
        let s_236_1: Bits = Bits::new(s_236_0 as u128, 8u16);
        // C s_236_2: cast zx s_236_1 -> i
        let s_236_2: i128 = (s_236_1.value() as i128);
        // C s_236_3: cast reint s_236_2 -> i64
        let s_236_3: i64 = (s_236_2 as i64);
        // C s_236_4: cast zx s_236_3 -> i
        let s_236_4: i128 = (i128::try_from(s_236_3).unwrap());
        // C s_236_5: const #432u : u32
        let s_236_5: u32 = 432;
        // D s_236_6: read-reg s_236_5:u8
        let s_236_6: u8 = {
            let value = state.read_register::<u8>(s_236_5 as isize);
            tracer.read_register(s_236_5 as isize, value);
            value
        };
        // D s_236_7: call AArch64_AArch32SystemAccessTrap(s_236_6, s_236_4)
        let s_236_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_236_6,
            s_236_4,
        );
        // N s_236_8: return
        return;
    }
    fn block_237<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_237_0: read-var __HSTR_EL2_T5:u8
        let s_237_0: bool = fn_state.u__HSTR_EL2_T5;
        // D s_237_1: cast zx s_237_0 -> bv
        let s_237_1: Bits = Bits::new(s_237_0 as u128, 1u16);
        // C s_237_2: const #1u : u8
        let s_237_2: bool = true;
        // C s_237_3: cast zx s_237_2 -> bv
        let s_237_3: Bits = Bits::new(s_237_2 as u128, 1u16);
        // D s_237_4: cmp-eq s_237_1 s_237_3
        let s_237_4: bool = ((s_237_1) == (s_237_3));
        // D s_237_5: write-var gs#122393 <= s_237_4
        fn_state.gs_122393 = s_237_4;
        // N s_237_6: jump b164
        return block_164(state, tracer, fn_state);
    }
    fn block_238<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_238_0: const #1u : u8
        let s_238_0: bool = true;
        // D s_238_1: write-var gs#122392 <= s_238_0
        fn_state.gs_122392 = s_238_0;
        // N s_238_2: jump b162
        return block_162(state, tracer, fn_state);
    }
    fn block_239<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_239_0: const #102552u : u32
        let s_239_0: u32 = 102552;
        // D s_239_1: read-reg s_239_0:struct
        let s_239_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_239_0 as isize);
            tracer.read_register(s_239_0 as isize, value);
            value
        };
        // D s_239_2: call _get_HCR_EL2_Type_E2H(s_239_1)
        let s_239_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_239_1);
        // C s_239_3: const #102552u : u32
        let s_239_3: u32 = 102552;
        // D s_239_4: read-reg s_239_3:struct
        let s_239_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_239_3 as isize);
            tracer.read_register(s_239_3 as isize, value);
            value
        };
        // D s_239_5: call _get_HCR_EL2_Type_TGE(s_239_4)
        let s_239_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_239_4);
        // D s_239_6: cast zx s_239_2 -> bv
        let s_239_6: Bits = Bits::new(s_239_2 as u128, 1u16);
        // D s_239_7: cast zx s_239_5 -> bv
        let s_239_7: Bits = Bits::new(s_239_5 as u128, 1u16);
        // D s_239_8: cast reint s_239_6 -> u128
        let s_239_8: u128 = (s_239_6.value() as u128);
        // D s_239_9: size-of s_239_6
        let s_239_9: u16 = s_239_6.length();
        // D s_239_10: cast reint s_239_7 -> u128
        let s_239_10: u128 = (s_239_7.value() as u128);
        // D s_239_11: size-of s_239_7
        let s_239_11: u16 = s_239_7.length();
        // D s_239_12: lsl s_239_8 s_239_11
        let s_239_12: u128 = s_239_8 << s_239_11;
        // D s_239_13: or s_239_12 s_239_10
        let s_239_13: u128 = ((s_239_12) | (s_239_10));
        // D s_239_14: add s_239_9 s_239_11
        let s_239_14: u16 = (s_239_9 + s_239_11);
        // D s_239_15: create-bits s_239_13 s_239_14
        let s_239_15: Bits = Bits::new(s_239_13, s_239_14);
        // D s_239_16: cast reint s_239_15 -> u8
        let s_239_16: u8 = (s_239_15.value() as u8);
        // D s_239_17: cast zx s_239_16 -> bv
        let s_239_17: Bits = Bits::new(s_239_16 as u128, 2u16);
        // C s_239_18: const #3u : u8
        let s_239_18: u8 = 3;
        // C s_239_19: cast zx s_239_18 -> bv
        let s_239_19: Bits = Bits::new(s_239_18 as u128, 2u16);
        // D s_239_20: cmp-ne s_239_17 s_239_19
        let s_239_20: bool = ((s_239_17) != (s_239_19));
        // D s_239_21: write-var gs#122389 <= s_239_20
        fn_state.gs_122389 = s_239_20;
        // N s_239_22: jump b160
        return block_160(state, tracer, fn_state);
    }
    fn block_240<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_240_0: const #432u : u32
        let s_240_0: u32 = 432;
        // D s_240_1: read-reg s_240_0:u8
        let s_240_1: u8 = {
            let value = state.read_register::<u8>(s_240_0 as isize);
            tracer.read_register(s_240_0 as isize, value);
            value
        };
        // D s_240_2: call ELUsingAArch32(s_240_1)
        let s_240_2: bool = ELUsingAArch32(state, tracer, s_240_1);
        // D s_240_3: not s_240_2
        let s_240_3: bool = !s_240_2;
        // D s_240_4: write-var gs#122388 <= s_240_3
        fn_state.gs_122388 = s_240_3;
        // N s_240_5: jump b158
        return block_158(state, tracer, fn_state);
    }
    fn block_241<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_241_0: const #() : ()
        let s_241_0: () = ();
        // S s_241_1: call EL2Enabled(s_241_0)
        let s_241_1: bool = EL2Enabled(state, tracer, s_241_0);
        // N s_241_2: branch s_241_1 b257 b242
        if s_241_1 {
            return block_257(state, tracer, fn_state);
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
        // D s_242_1: write-var gs#122436 <= s_242_0
        fn_state.gs_122436 = s_242_0;
        // N s_242_2: jump b243
        return block_243(state, tracer, fn_state);
    }
    fn block_243<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_243_0: read-var gs#122436:u8
        let s_243_0: bool = fn_state.gs_122436;
        // N s_243_1: branch s_243_0 b256 b244
        if s_243_0 {
            return block_256(state, tracer, fn_state);
        } else {
            return block_244(state, tracer, fn_state);
        };
    }
    fn block_244<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_244_0: const #0u : u8
        let s_244_0: bool = false;
        // D s_244_1: write-var gs#122437 <= s_244_0
        fn_state.gs_122437 = s_244_0;
        // N s_244_2: jump b245
        return block_245(state, tracer, fn_state);
    }
    fn block_245<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_245_0: read-var gs#122437:u8
        let s_245_0: bool = fn_state.gs_122437;
        // N s_245_1: branch s_245_0 b255 b246
        if s_245_0 {
            return block_255(state, tracer, fn_state);
        } else {
            return block_246(state, tracer, fn_state);
        };
    }
    fn block_246<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_246_0: const #() : ()
        let s_246_0: () = ();
        // S s_246_1: call EL2Enabled(s_246_0)
        let s_246_1: bool = EL2Enabled(state, tracer, s_246_0);
        // N s_246_2: branch s_246_1 b254 b247
        if s_246_1 {
            return block_254(state, tracer, fn_state);
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
        // D s_247_1: write-var gs#122438 <= s_247_0
        fn_state.gs_122438 = s_247_0;
        // N s_247_2: jump b248
        return block_248(state, tracer, fn_state);
    }
    fn block_248<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_248_0: read-var gs#122438:u8
        let s_248_0: bool = fn_state.gs_122438;
        // N s_248_1: branch s_248_0 b253 b249
        if s_248_0 {
            return block_253(state, tracer, fn_state);
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
        // D s_249_1: write-var gs#122439 <= s_249_0
        fn_state.gs_122439 = s_249_0;
        // N s_249_2: jump b250
        return block_250(state, tracer, fn_state);
    }
    fn block_250<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_250_0: read-var gs#122439:u8
        let s_250_0: bool = fn_state.gs_122439;
        // N s_250_1: branch s_250_0 b252 b251
        if s_250_0 {
            return block_252(state, tracer, fn_state);
        } else {
            return block_251(state, tracer, fn_state);
        };
    }
    fn block_251<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_251_0: panic
        panic!("{:?}", ());
        // N s_251_1: return
        return;
    }
    fn block_252<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_252_0: const #0u : u8
        let s_252_0: u8 = 0;
        // C s_252_1: cast zx s_252_0 -> bv
        let s_252_1: Bits = Bits::new(s_252_0 as u128, 8u16);
        // C s_252_2: cast zx s_252_1 -> i
        let s_252_2: i128 = (s_252_1.value() as i128);
        // C s_252_3: cast reint s_252_2 -> i64
        let s_252_3: i64 = (s_252_2 as i64);
        // C s_252_4: cast zx s_252_3 -> i
        let s_252_4: i128 = (i128::try_from(s_252_3).unwrap());
        // S s_252_5: call AArch32_TakeHypTrapException(s_252_4)
        let s_252_5: () = AArch32_TakeHypTrapException(state, tracer, s_252_4);
        // N s_252_6: return
        return;
    }
    fn block_253<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_253_0: read-var __HCR_TGE:u8
        let s_253_0: bool = fn_state.u__HCR_TGE;
        // D s_253_1: cast zx s_253_0 -> bv
        let s_253_1: Bits = Bits::new(s_253_0 as u128, 1u16);
        // C s_253_2: const #1u : u8
        let s_253_2: bool = true;
        // C s_253_3: cast zx s_253_2 -> bv
        let s_253_3: Bits = Bits::new(s_253_2 as u128, 1u16);
        // D s_253_4: cmp-eq s_253_1 s_253_3
        let s_253_4: bool = ((s_253_1) == (s_253_3));
        // D s_253_5: write-var gs#122439 <= s_253_4
        fn_state.gs_122439 = s_253_4;
        // N s_253_6: jump b250
        return block_250(state, tracer, fn_state);
    }
    fn block_254<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_254_0: const #432u : u32
        let s_254_0: u32 = 432;
        // D s_254_1: read-reg s_254_0:u8
        let s_254_1: u8 = {
            let value = state.read_register::<u8>(s_254_0 as isize);
            tracer.read_register(s_254_0 as isize, value);
            value
        };
        // D s_254_2: call ELUsingAArch32(s_254_1)
        let s_254_2: bool = ELUsingAArch32(state, tracer, s_254_1);
        // D s_254_3: write-var gs#122438 <= s_254_2
        fn_state.gs_122438 = s_254_2;
        // N s_254_4: jump b248
        return block_248(state, tracer, fn_state);
    }
    fn block_255<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_255_0: const #4u : u8
        let s_255_0: u8 = 4;
        // C s_255_1: cast zx s_255_0 -> bv
        let s_255_1: Bits = Bits::new(s_255_0 as u128, 8u16);
        // C s_255_2: cast zx s_255_1 -> i
        let s_255_2: i128 = (s_255_1.value() as i128);
        // C s_255_3: cast reint s_255_2 -> i64
        let s_255_3: i64 = (s_255_2 as i64);
        // C s_255_4: cast zx s_255_3 -> i
        let s_255_4: i128 = (i128::try_from(s_255_3).unwrap());
        // C s_255_5: const #432u : u32
        let s_255_5: u32 = 432;
        // D s_255_6: read-reg s_255_5:u8
        let s_255_6: u8 = {
            let value = state.read_register::<u8>(s_255_5 as isize);
            tracer.read_register(s_255_5 as isize, value);
            value
        };
        // D s_255_7: call AArch64_AArch32SystemAccessTrap(s_255_6, s_255_4)
        let s_255_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_255_6,
            s_255_4,
        );
        // N s_255_8: return
        return;
    }
    fn block_256<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_256_0: read-var __HCR_EL2_TGE:u8
        let s_256_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_256_1: cast zx s_256_0 -> bv
        let s_256_1: Bits = Bits::new(s_256_0 as u128, 1u16);
        // C s_256_2: const #1u : u8
        let s_256_2: bool = true;
        // C s_256_3: cast zx s_256_2 -> bv
        let s_256_3: Bits = Bits::new(s_256_2 as u128, 1u16);
        // D s_256_4: cmp-eq s_256_1 s_256_3
        let s_256_4: bool = ((s_256_1) == (s_256_3));
        // D s_256_5: write-var gs#122437 <= s_256_4
        fn_state.gs_122437 = s_256_4;
        // N s_256_6: jump b245
        return block_245(state, tracer, fn_state);
    }
    fn block_257<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_257_0: const #432u : u32
        let s_257_0: u32 = 432;
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
        // D s_257_4: write-var gs#122436 <= s_257_3
        fn_state.gs_122436 = s_257_3;
        // N s_257_5: jump b243
        return block_243(state, tracer, fn_state);
    }
    fn block_258<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_258_0: read-var __AMUSERENR_EN:u8
        let s_258_0: bool = fn_state.u__AMUSERENR_EN;
        // D s_258_1: cast zx s_258_0 -> bv
        let s_258_1: Bits = Bits::new(s_258_0 as u128, 1u16);
        // C s_258_2: const #0u : u8
        let s_258_2: bool = false;
        // C s_258_3: cast zx s_258_2 -> bv
        let s_258_3: Bits = Bits::new(s_258_2 as u128, 1u16);
        // D s_258_4: cmp-eq s_258_1 s_258_3
        let s_258_4: bool = ((s_258_1) == (s_258_3));
        // D s_258_5: write-var gs#122387 <= s_258_4
        fn_state.gs_122387 = s_258_4;
        // N s_258_6: jump b155
        return block_155(state, tracer, fn_state);
    }
    fn block_259<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_259_0: const #() : ()
        let s_259_0: () = ();
        // S s_259_1: call EL2Enabled(s_259_0)
        let s_259_1: bool = EL2Enabled(state, tracer, s_259_0);
        // N s_259_2: branch s_259_1 b267 b260
        if s_259_1 {
            return block_267(state, tracer, fn_state);
        } else {
            return block_260(state, tracer, fn_state);
        };
    }
    fn block_260<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_260_0: const #0u : u8
        let s_260_0: bool = false;
        // D s_260_1: write-var gs#122440 <= s_260_0
        fn_state.gs_122440 = s_260_0;
        // N s_260_2: jump b261
        return block_261(state, tracer, fn_state);
    }
    fn block_261<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_261_0: read-var gs#122440:u8
        let s_261_0: bool = fn_state.gs_122440;
        // N s_261_1: branch s_261_0 b266 b262
        if s_261_0 {
            return block_266(state, tracer, fn_state);
        } else {
            return block_262(state, tracer, fn_state);
        };
    }
    fn block_262<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_262_0: const #0u : u8
        let s_262_0: bool = false;
        // D s_262_1: write-var gs#122441 <= s_262_0
        fn_state.gs_122441 = s_262_0;
        // N s_262_2: jump b263
        return block_263(state, tracer, fn_state);
    }
    fn block_263<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_263_0: read-var gs#122441:u8
        let s_263_0: bool = fn_state.gs_122441;
        // N s_263_1: branch s_263_0 b265 b264
        if s_263_0 {
            return block_265(state, tracer, fn_state);
        } else {
            return block_264(state, tracer, fn_state);
        };
    }
    fn block_264<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_264_0: const #4u : u8
        let s_264_0: u8 = 4;
        // C s_264_1: cast zx s_264_0 -> bv
        let s_264_1: Bits = Bits::new(s_264_0 as u128, 8u16);
        // C s_264_2: cast zx s_264_1 -> i
        let s_264_2: i128 = (s_264_1.value() as i128);
        // C s_264_3: cast reint s_264_2 -> i64
        let s_264_3: i64 = (s_264_2 as i64);
        // C s_264_4: cast zx s_264_3 -> i
        let s_264_4: i128 = (i128::try_from(s_264_3).unwrap());
        // C s_264_5: const #440u : u32
        let s_264_5: u32 = 440;
        // D s_264_6: read-reg s_264_5:u8
        let s_264_6: u8 = {
            let value = state.read_register::<u8>(s_264_5 as isize);
            tracer.read_register(s_264_5 as isize, value);
            value
        };
        // D s_264_7: call AArch64_AArch32SystemAccessTrap(s_264_6, s_264_4)
        let s_264_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_264_6,
            s_264_4,
        );
        // N s_264_8: return
        return;
    }
    fn block_265<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_265_0: const #4u : u8
        let s_265_0: u8 = 4;
        // C s_265_1: cast zx s_265_0 -> bv
        let s_265_1: Bits = Bits::new(s_265_0 as u128, 8u16);
        // C s_265_2: cast zx s_265_1 -> i
        let s_265_2: i128 = (s_265_1.value() as i128);
        // C s_265_3: cast reint s_265_2 -> i64
        let s_265_3: i64 = (s_265_2 as i64);
        // C s_265_4: cast zx s_265_3 -> i
        let s_265_4: i128 = (i128::try_from(s_265_3).unwrap());
        // C s_265_5: const #432u : u32
        let s_265_5: u32 = 432;
        // D s_265_6: read-reg s_265_5:u8
        let s_265_6: u8 = {
            let value = state.read_register::<u8>(s_265_5 as isize);
            tracer.read_register(s_265_5 as isize, value);
            value
        };
        // D s_265_7: call AArch64_AArch32SystemAccessTrap(s_265_6, s_265_4)
        let s_265_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_265_6,
            s_265_4,
        );
        // N s_265_8: return
        return;
    }
    fn block_266<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_266_0: read-var __HCR_EL2_TGE:u8
        let s_266_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_266_1: cast zx s_266_0 -> bv
        let s_266_1: Bits = Bits::new(s_266_0 as u128, 1u16);
        // C s_266_2: const #1u : u8
        let s_266_2: bool = true;
        // C s_266_3: cast zx s_266_2 -> bv
        let s_266_3: Bits = Bits::new(s_266_2 as u128, 1u16);
        // D s_266_4: cmp-eq s_266_1 s_266_3
        let s_266_4: bool = ((s_266_1) == (s_266_3));
        // D s_266_5: write-var gs#122441 <= s_266_4
        fn_state.gs_122441 = s_266_4;
        // N s_266_6: jump b263
        return block_263(state, tracer, fn_state);
    }
    fn block_267<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_267_0: const #432u : u32
        let s_267_0: u32 = 432;
        // D s_267_1: read-reg s_267_0:u8
        let s_267_1: u8 = {
            let value = state.read_register::<u8>(s_267_0 as isize);
            tracer.read_register(s_267_0 as isize, value);
            value
        };
        // D s_267_2: call ELUsingAArch32(s_267_1)
        let s_267_2: bool = ELUsingAArch32(state, tracer, s_267_1);
        // D s_267_3: not s_267_2
        let s_267_3: bool = !s_267_2;
        // D s_267_4: write-var gs#122440 <= s_267_3
        fn_state.gs_122440 = s_267_3;
        // N s_267_5: jump b261
        return block_261(state, tracer, fn_state);
    }
    fn block_268<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_268_0: read-var __AMUSERENR_EL0_EN:u8
        let s_268_0: bool = fn_state.u__AMUSERENR_EL0_EN;
        // D s_268_1: cast zx s_268_0 -> bv
        let s_268_1: Bits = Bits::new(s_268_0 as u128, 1u16);
        // C s_268_2: const #0u : u8
        let s_268_2: bool = false;
        // C s_268_3: cast zx s_268_2 -> bv
        let s_268_3: Bits = Bits::new(s_268_2 as u128, 1u16);
        // D s_268_4: cmp-eq s_268_1 s_268_3
        let s_268_4: bool = ((s_268_1) == (s_268_3));
        // D s_268_5: write-var gs#122386 <= s_268_4
        fn_state.gs_122386 = s_268_4;
        // N s_268_6: jump b152
        return block_152(state, tracer, fn_state);
    }
    fn block_269<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_269_0: panic
        panic!("{:?}", ());
        // N s_269_1: return
        return;
    }
    fn block_270<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_270_0: read-var __CPTR_EL3_TAM:u8
        let s_270_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_270_1: cast zx s_270_0 -> bv
        let s_270_1: Bits = Bits::new(s_270_0 as u128, 1u16);
        // C s_270_2: const #1u : u8
        let s_270_2: bool = true;
        // C s_270_3: cast zx s_270_2 -> bv
        let s_270_3: Bits = Bits::new(s_270_2 as u128, 1u16);
        // D s_270_4: cmp-eq s_270_1 s_270_3
        let s_270_4: bool = ((s_270_1) == (s_270_3));
        // D s_270_5: write-var gs#122385 <= s_270_4
        fn_state.gs_122385 = s_270_4;
        // N s_270_6: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_271<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_271_0: const #424u : u32
        let s_271_0: u32 = 424;
        // D s_271_1: read-reg s_271_0:u8
        let s_271_1: u8 = {
            let value = state.read_register::<u8>(s_271_0 as isize);
            tracer.read_register(s_271_0 as isize, value);
            value
        };
        // D s_271_2: call ELUsingAArch32(s_271_1)
        let s_271_2: bool = ELUsingAArch32(state, tracer, s_271_1);
        // D s_271_3: not s_271_2
        let s_271_3: bool = !s_271_2;
        // D s_271_4: write-var gs#122384 <= s_271_3
        fn_state.gs_122384 = s_271_3;
        // N s_271_5: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_272<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_272_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_272_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_272_1: call __IMPDEF_boolean(s_272_0)
        let s_272_1: bool = u__IMPDEF_boolean(state, tracer, s_272_0);
        // D s_272_2: write-var gs#122383 <= s_272_1
        fn_state.gs_122383 = s_272_1;
        // N s_272_3: jump b145
        return block_145(state, tracer, fn_state);
    }
    fn block_273<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_273_0: const #() : ()
        let s_273_0: () = ();
        // S s_273_1: call EDSCR_read(s_273_0)
        let s_273_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_273_0);
        // S s_273_2: call _get_EDSCR_Type_SDD(s_273_1)
        let s_273_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_273_1);
        // S s_273_3: cast zx s_273_2 -> bv
        let s_273_3: Bits = Bits::new(s_273_2 as u128, 1u16);
        // C s_273_4: const #1u : u8
        let s_273_4: bool = true;
        // C s_273_5: cast zx s_273_4 -> bv
        let s_273_5: Bits = Bits::new(s_273_4 as u128, 1u16);
        // S s_273_6: cmp-eq s_273_3 s_273_5
        let s_273_6: bool = ((s_273_3) == (s_273_5));
        // D s_273_7: write-var gs#122382 <= s_273_6
        fn_state.gs_122382 = s_273_6;
        // N s_273_8: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_274<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_274_0: const #424u : u32
        let s_274_0: u32 = 424;
        // D s_274_1: read-reg s_274_0:u8
        let s_274_1: u8 = {
            let value = state.read_register::<u8>(s_274_0 as isize);
            tracer.read_register(s_274_0 as isize, value);
            value
        };
        // C s_274_2: const #2u : u8
        let s_274_2: u8 = 2;
        // D s_274_3: cmp-lt s_274_1 s_274_2
        let s_274_3: bool = ((s_274_1) < (s_274_2));
        // D s_274_4: write-var gs#122381 <= s_274_3
        fn_state.gs_122381 = s_274_3;
        // N s_274_5: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_275<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_275_0: panic
        panic!("{:?}", ());
        // N s_275_1: return
        return;
    }
}
