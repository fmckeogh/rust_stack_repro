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
use u_get_HAFGRTR_EL2_Type_AMEVTYPER15_EL0::*;
use u_get_AMUSERENR_Type_EN::*;
use u_get_SCR_EL3_Type_FGTEn::*;
use Halted::*;
use u__get_AMEVTYPER1::*;
use AMUSERENR_read::*;
use u_get_HSTR_Type_T13::*;
use u_get_HCPTR_Type_TAM::*;
use IsG1ActivityMonitorImplemented::*;
use HCPTR_read::*;
use u_get_CPTR_EL2_Type_TAM::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_E2H::*;
use HCR_read::*;
use IsFeatureImplemented::*;
use u__IMPDEF_boolean::*;
use AMEVTYPER1_read::*;
use AArch64_AArch32SystemAccessTrap::*;
use HSTR_read::*;
use u_get_HCR_Type_TGE::*;
use u_get_CPTR_EL3_Type_TAM::*;
use u_get_HSTR_EL2_Type_T13::*;
use R_set::*;
use ELUsingAArch32::*;
use u_get_AMUSERENR_EL0_Type_EN::*;
use EDSCR_read::*;
use AArch32_TakeHypTrapException::*;
use common::*;
pub fn AMEVTYPER1_SysRegRead32_aefd3ad06352acdf<T: Tracer>(
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
        u__HSTR_EL2_T13: bool,
        gs_106513: bool,
        gs_106526: bool,
        gs_106475: bool,
        gs_106479: bool,
        gs_106492: bool,
        gs_106503: bool,
        gs_106512: bool,
        gs_106481: bool,
        u__CPTR_EL3_TAM: bool,
        gs_106510: bool,
        u__CPTR_EL2_TAM: bool,
        gs_106515: bool,
        gs_106486: bool,
        gs_106509: bool,
        gs_106498: bool,
        u__HCPTR_TAM: bool,
        gs_106511: bool,
        gs_106484: bool,
        gs_106527: bool,
        u__HCR_TGE: bool,
        gs_106529: bool,
        gs_106508: bool,
        gs_106488: bool,
        u__HSTR_T13: bool,
        gs_106483: bool,
        u__HAFGRTR_EL2_AMEVTYPER15_EL0: bool,
        gs_106507: bool,
        ga_165289: ProductType700c18a878c5601b,
        gs_106504: bool,
        gs_106499: bool,
        u__PSTATE_EL: u8,
        gs_106506: bool,
        gs_106477: bool,
        gs_106517: bool,
        gs_106524: bool,
        gs_106520: bool,
        gs_106478: bool,
        u__HCR_EL2_TGE: bool,
        gs_106491: bool,
        gs_106487: bool,
        gs_106489: bool,
        gs_106514: bool,
        u__SCR_EL3_FGTEn: bool,
        gs_106530: bool,
        ga_165265: ProductType700c18a878c5601b,
        gs_106496: bool,
        u__AMUSERENR_EL0_EN: bool,
        gs_106516: bool,
        gs_106519: bool,
        gs_106521: bool,
        gs_106494: bool,
        gs_106528: bool,
        gs_106505: bool,
        gs_106485: bool,
        gs_106518: bool,
        gs_106502: bool,
        gs_106522: bool,
        gs_106493: bool,
        ga_165223: ProductType700c18a878c5601b,
        gs_106525: bool,
        gs_106495: bool,
        gs_106490: bool,
        gs_106474: bool,
        gs_106501: bool,
        ga_165293: ProductType700c18a878c5601b,
        gs_106482: bool,
        gs_106473: bool,
        gs_106476: bool,
        u__AMUSERENR_EN: bool,
        gs_106500: bool,
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
        // C s_0_3: const #16840u : u32
        let s_0_3: u32 = 16840;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_CPTR_EL3_Type_TAM(s_0_4)
        let s_0_5: bool = u_get_CPTR_EL3_Type_TAM(state, tracer, s_0_4);
        // D s_0_6: write-var __CPTR_EL3_TAM <= s_0_5
        fn_state.u__CPTR_EL3_TAM = s_0_5;
        // C s_0_7: const #90496u : u32
        let s_0_7: u32 = 90496;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_AMUSERENR_EL0_Type_EN(s_0_8)
        let s_0_9: bool = u_get_AMUSERENR_EL0_Type_EN(state, tracer, s_0_8);
        // D s_0_10: write-var __AMUSERENR_EL0_EN <= s_0_9
        fn_state.u__AMUSERENR_EL0_EN = s_0_9;
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
        // S s_0_16: call AMUSERENR_read(s_0_15)
        let s_0_16: ProductType700c18a878c5601b = AMUSERENR_read(state, tracer, s_0_15);
        // S s_0_17: call _get_AMUSERENR_Type_EN(s_0_16)
        let s_0_17: bool = u_get_AMUSERENR_Type_EN(state, tracer, s_0_16);
        // D s_0_18: write-var __AMUSERENR_EN <= s_0_17
        fn_state.u__AMUSERENR_EN = s_0_17;
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
        // D s_0_25: call _get_HSTR_EL2_Type_T13(s_0_24)
        let s_0_25: bool = u_get_HSTR_EL2_Type_T13(state, tracer, s_0_24);
        // D s_0_26: write-var __HSTR_EL2_T13 <= s_0_25
        fn_state.u__HSTR_EL2_T13 = s_0_25;
        // C s_0_27: const #() : ()
        let s_0_27: () = ();
        // S s_0_28: call HSTR_read(s_0_27)
        let s_0_28: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_27);
        // S s_0_29: call _get_HSTR_Type_T13(s_0_28)
        let s_0_29: bool = u_get_HSTR_Type_T13(state, tracer, s_0_28);
        // D s_0_30: write-var __HSTR_T13 <= s_0_29
        fn_state.u__HSTR_T13 = s_0_29;
        // C s_0_31: const #11088u : u32
        let s_0_31: u32 = 11088;
        // D s_0_32: read-reg s_0_31:struct
        let s_0_32: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_31 as isize);
            tracer.read_register(s_0_31 as isize, value);
            value
        };
        // D s_0_33: call _get_CPTR_EL2_Type_TAM(s_0_32)
        let s_0_33: bool = u_get_CPTR_EL2_Type_TAM(state, tracer, s_0_32);
        // D s_0_34: write-var __CPTR_EL2_TAM <= s_0_33
        fn_state.u__CPTR_EL2_TAM = s_0_33;
        // C s_0_35: const #() : ()
        let s_0_35: () = ();
        // S s_0_36: call HCPTR_read(s_0_35)
        let s_0_36: ProductType700c18a878c5601b = HCPTR_read(state, tracer, s_0_35);
        // S s_0_37: call _get_HCPTR_Type_TAM(s_0_36)
        let s_0_37: bool = u_get_HCPTR_Type_TAM(state, tracer, s_0_36);
        // D s_0_38: write-var __HCPTR_TAM <= s_0_37
        fn_state.u__HCPTR_TAM = s_0_37;
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
        // C s_0_43: const #21760u : u32
        let s_0_43: u32 = 21760;
        // D s_0_44: read-reg s_0_43:struct
        let s_0_44: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_43 as isize);
            tracer.read_register(s_0_43 as isize, value);
            value
        };
        // D s_0_45: call _get_HAFGRTR_EL2_Type_AMEVTYPER15_EL0(s_0_44)
        let s_0_45: bool = u_get_HAFGRTR_EL2_Type_AMEVTYPER15_EL0(state, tracer, s_0_44);
        // D s_0_46: write-var __HAFGRTR_EL2_AMEVTYPER15_EL0 <= s_0_45
        fn_state.u__HAFGRTR_EL2_AMEVTYPER15_EL0 = s_0_45;
        // C s_0_47: const #5s : i
        let s_0_47: i128 = 5;
        // S s_0_48: call IsG1ActivityMonitorImplemented(s_0_47)
        let s_0_48: bool = IsG1ActivityMonitorImplemented(state, tracer, s_0_47);
        // S s_0_49: not s_0_48
        let s_0_49: bool = !s_0_48;
        // N s_0_50: branch s_0_49 b221 b1
        if s_0_49 {
            return block_221(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b101 b2
        if s_1_5 {
            return block_101(state, tracer, fn_state);
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
        // C s_6_0: const #5s : i64
        let s_6_0: i64 = 5;
        // S s_6_1: call AMEVTYPER1_read(s_6_0)
        let s_6_1: ProductType700c18a878c5601b = AMEVTYPER1_read(state, tracer, s_6_0);
        // S s_6_2: call __get_AMEVTYPER1(s_6_1)
        let s_6_2: ProductType700c18a878c5601b = u__get_AMEVTYPER1(state, tracer, s_6_1);
        // D s_6_3: write-var ga#165293 <= s_6_2
        fn_state.ga_165293 = s_6_2;
        // D s_6_4: read-var ga#165293.0:struct
        let s_6_4: u32 = fn_state.ga_165293._0;
        // D s_6_5: read-var t:i
        let s_6_5: i128 = fn_state.t;
        // D s_6_6: call R_set(s_6_5, s_6_4)
        let s_6_6: () = R_set(state, tracer, s_6_5, s_6_4);
        // N s_6_7: return
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
        // D s_8_1: write-var gs#106473 <= s_8_0
        fn_state.gs_106473 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#106473:u8
        let s_9_0: bool = fn_state.gs_106473;
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
        // D s_10_1: write-var gs#106474 <= s_10_0
        fn_state.gs_106474 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#106474:u8
        let s_11_0: bool = fn_state.gs_106474;
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
        // D s_12_1: write-var gs#106475 <= s_12_0
        fn_state.gs_106475 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#106475:u8
        let s_13_0: bool = fn_state.gs_106475;
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
        // D s_14_1: write-var gs#106476 <= s_14_0
        fn_state.gs_106476 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#106476:u8
        let s_15_0: bool = fn_state.gs_106476;
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
        // D s_16_1: write-var gs#106477 <= s_16_0
        fn_state.gs_106477 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#106477:u8
        let s_17_0: bool = fn_state.gs_106477;
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
        // D s_19_1: write-var gs#106478 <= s_19_0
        fn_state.gs_106478 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#106478:u8
        let s_20_0: bool = fn_state.gs_106478;
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
        // D s_21_1: write-var gs#106479 <= s_21_0
        fn_state.gs_106479 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#106479:u8
        let s_22_0: bool = fn_state.gs_106479;
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
        // C s_23_0: const #5s : i64
        let s_23_0: i64 = 5;
        // S s_23_1: call AMEVTYPER1_read(s_23_0)
        let s_23_1: ProductType700c18a878c5601b = AMEVTYPER1_read(state, tracer, s_23_0);
        // S s_23_2: call __get_AMEVTYPER1(s_23_1)
        let s_23_2: ProductType700c18a878c5601b = u__get_AMEVTYPER1(
            state,
            tracer,
            s_23_1,
        );
        // D s_23_3: write-var ga#165289 <= s_23_2
        fn_state.ga_165289 = s_23_2;
        // D s_23_4: read-var ga#165289.0:struct
        let s_23_4: u32 = fn_state.ga_165289._0;
        // D s_23_5: read-var t:i
        let s_23_5: i128 = fn_state.t;
        // D s_23_6: call R_set(s_23_5, s_23_4)
        let s_23_6: () = R_set(state, tracer, s_23_5, s_23_4);
        // N s_23_7: return
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
        // D s_25_1: write-var gs#106481 <= s_25_0
        fn_state.gs_106481 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#106481:u8
        let s_26_0: bool = fn_state.gs_106481;
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
        // D s_29_7: write-var gs#106481 <= s_29_6
        fn_state.gs_106481 = s_29_6;
        // N s_29_8: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var __CPTR_EL3_TAM:u8
        let s_30_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 1u16);
        // C s_30_2: const #1u : u8
        let s_30_2: bool = true;
        // C s_30_3: cast zx s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 1u16);
        // D s_30_4: cmp-eq s_30_1 s_30_3
        let s_30_4: bool = ((s_30_1) == (s_30_3));
        // D s_30_5: write-var gs#106479 <= s_30_4
        fn_state.gs_106479 = s_30_4;
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
        // D s_31_4: write-var gs#106478 <= s_31_3
        fn_state.gs_106478 = s_31_3;
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
        // D s_33_0: read-var __CPTR_EL3_TAM:u8
        let s_33_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 1u16);
        // C s_33_2: const #1u : u8
        let s_33_2: bool = true;
        // C s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 1u16);
        // D s_33_4: cmp-eq s_33_1 s_33_3
        let s_33_4: bool = ((s_33_1) == (s_33_3));
        // D s_33_5: write-var gs#106477 <= s_33_4
        fn_state.gs_106477 = s_33_4;
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
        // D s_34_4: write-var gs#106476 <= s_34_3
        fn_state.gs_106476 = s_34_3;
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
        // D s_35_2: write-var gs#106475 <= s_35_1
        fn_state.gs_106475 = s_35_1;
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
        // D s_36_7: write-var gs#106474 <= s_36_6
        fn_state.gs_106474 = s_36_6;
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
        // D s_37_4: write-var gs#106473 <= s_37_3
        fn_state.gs_106473 = s_37_3;
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
        // N s_38_2: branch s_38_1 b100 b39
        if s_38_1 {
            return block_100(state, tracer, fn_state);
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
        // D s_39_1: write-var gs#106482 <= s_39_0
        fn_state.gs_106482 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#106482:u8
        let s_40_0: bool = fn_state.gs_106482;
        // N s_40_1: branch s_40_0 b99 b41
        if s_40_0 {
            return block_99(state, tracer, fn_state);
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
        // D s_41_1: write-var gs#106483 <= s_41_0
        fn_state.gs_106483 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#106483:u8
        let s_42_0: bool = fn_state.gs_106483;
        // N s_42_1: branch s_42_0 b98 b43
        if s_42_0 {
            return block_98(state, tracer, fn_state);
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
        // D s_43_1: write-var gs#106484 <= s_43_0
        fn_state.gs_106484 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#106484:u8
        let s_44_0: bool = fn_state.gs_106484;
        // N s_44_1: branch s_44_0 b97 b45
        if s_44_0 {
            return block_97(state, tracer, fn_state);
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
        // D s_45_1: write-var gs#106485 <= s_45_0
        fn_state.gs_106485 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#106485:u8
        let s_46_0: bool = fn_state.gs_106485;
        // N s_46_1: branch s_46_0 b96 b47
        if s_46_0 {
            return block_96(state, tracer, fn_state);
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
        // D s_47_1: write-var gs#106486 <= s_47_0
        fn_state.gs_106486 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#106486:u8
        let s_48_0: bool = fn_state.gs_106486;
        // N s_48_1: branch s_48_0 b95 b49
        if s_48_0 {
            return block_95(state, tracer, fn_state);
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
        // N s_49_2: branch s_49_1 b94 b50
        if s_49_1 {
            return block_94(state, tracer, fn_state);
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
        // D s_50_1: write-var gs#106487 <= s_50_0
        fn_state.gs_106487 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#106487:u8
        let s_51_0: bool = fn_state.gs_106487;
        // N s_51_1: branch s_51_0 b93 b52
        if s_51_0 {
            return block_93(state, tracer, fn_state);
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
        // D s_52_1: write-var gs#106488 <= s_52_0
        fn_state.gs_106488 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#106488:u8
        let s_53_0: bool = fn_state.gs_106488;
        // N s_53_1: branch s_53_0 b92 b54
        if s_53_0 {
            return block_92(state, tracer, fn_state);
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
        // N s_54_2: branch s_54_1 b91 b55
        if s_54_1 {
            return block_91(state, tracer, fn_state);
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
        // D s_55_1: write-var gs#106489 <= s_55_0
        fn_state.gs_106489 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#106489:u8
        let s_56_0: bool = fn_state.gs_106489;
        // N s_56_1: branch s_56_0 b90 b57
        if s_56_0 {
            return block_90(state, tracer, fn_state);
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
        // D s_57_1: write-var gs#106490 <= s_57_0
        fn_state.gs_106490 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var gs#106490:u8
        let s_58_0: bool = fn_state.gs_106490;
        // N s_58_1: branch s_58_0 b89 b59
        if s_58_0 {
            return block_89(state, tracer, fn_state);
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
        // N s_59_2: branch s_59_1 b88 b60
        if s_59_1 {
            return block_88(state, tracer, fn_state);
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
        // D s_60_1: write-var gs#106491 <= s_60_0
        fn_state.gs_106491 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#106491:u8
        let s_61_0: bool = fn_state.gs_106491;
        // N s_61_1: branch s_61_0 b87 b62
        if s_61_0 {
            return block_87(state, tracer, fn_state);
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
        // D s_62_1: write-var gs#106492 <= s_62_0
        fn_state.gs_106492 = s_62_0;
        // N s_62_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var gs#106492:u8
        let s_63_0: bool = fn_state.gs_106492;
        // N s_63_1: branch s_63_0 b86 b64
        if s_63_0 {
            return block_86(state, tracer, fn_state);
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
        // N s_64_2: branch s_64_1 b85 b65
        if s_64_1 {
            return block_85(state, tracer, fn_state);
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
        // D s_65_1: write-var gs#106493 <= s_65_0
        fn_state.gs_106493 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#106493:u8
        let s_66_0: bool = fn_state.gs_106493;
        // N s_66_1: branch s_66_0 b84 b67
        if s_66_0 {
            return block_84(state, tracer, fn_state);
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
        // D s_67_1: write-var gs#106494 <= s_67_0
        fn_state.gs_106494 = s_67_0;
        // N s_67_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var gs#106494:u8
        let s_68_0: bool = fn_state.gs_106494;
        // N s_68_1: branch s_68_0 b83 b69
        if s_68_0 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #424u : u32
        let s_69_0: u32 = 424;
        // D s_69_1: read-reg s_69_0:u8
        let s_69_1: u8 = {
            let value = state.read_register::<u8>(s_69_0 as isize);
            tracer.read_register(s_69_0 as isize, value);
            value
        };
        // C s_69_2: const #2u : u8
        let s_69_2: u8 = 2;
        // D s_69_3: cmp-lt s_69_1 s_69_2
        let s_69_3: bool = ((s_69_1) < (s_69_2));
        // N s_69_4: branch s_69_3 b82 b70
        if s_69_3 {
            return block_82(state, tracer, fn_state);
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
        // D s_70_1: write-var gs#106495 <= s_70_0
        fn_state.gs_106495 = s_70_0;
        // N s_70_2: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var gs#106495:u8
        let s_71_0: bool = fn_state.gs_106495;
        // N s_71_1: branch s_71_0 b81 b72
        if s_71_0 {
            return block_81(state, tracer, fn_state);
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
        // D s_72_1: write-var gs#106496 <= s_72_0
        fn_state.gs_106496 = s_72_0;
        // N s_72_2: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var gs#106496:u8
        let s_73_0: bool = fn_state.gs_106496;
        // N s_73_1: branch s_73_0 b75 b74
        if s_73_0 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_74(state, tracer, fn_state);
        };
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #5s : i64
        let s_74_0: i64 = 5;
        // S s_74_1: call AMEVTYPER1_read(s_74_0)
        let s_74_1: ProductType700c18a878c5601b = AMEVTYPER1_read(state, tracer, s_74_0);
        // S s_74_2: call __get_AMEVTYPER1(s_74_1)
        let s_74_2: ProductType700c18a878c5601b = u__get_AMEVTYPER1(
            state,
            tracer,
            s_74_1,
        );
        // D s_74_3: write-var ga#165265 <= s_74_2
        fn_state.ga_165265 = s_74_2;
        // D s_74_4: read-var ga#165265.0:struct
        let s_74_4: u32 = fn_state.ga_165265._0;
        // D s_74_5: read-var t:i
        let s_74_5: i128 = fn_state.t;
        // D s_74_6: call R_set(s_74_5, s_74_4)
        let s_74_6: () = R_set(state, tracer, s_74_5, s_74_4);
        // N s_74_7: return
        return;
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #() : ()
        let s_75_0: () = ();
        // S s_75_1: call Halted(s_75_0)
        let s_75_1: bool = Halted(state, tracer, s_75_0);
        // N s_75_2: branch s_75_1 b80 b76
        if s_75_1 {
            return block_80(state, tracer, fn_state);
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
        // D s_76_1: write-var gs#106498 <= s_76_0
        fn_state.gs_106498 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var gs#106498:u8
        let s_77_0: bool = fn_state.gs_106498;
        // N s_77_1: branch s_77_0 b79 b78
        if s_77_0 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_78(state, tracer, fn_state);
        };
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #3u : u8
        let s_78_0: u8 = 3;
        // C s_78_1: cast zx s_78_0 -> bv
        let s_78_1: Bits = Bits::new(s_78_0 as u128, 8u16);
        // C s_78_2: cast zx s_78_1 -> i
        let s_78_2: i128 = (s_78_1.value() as i128);
        // C s_78_3: cast reint s_78_2 -> i64
        let s_78_3: i64 = (s_78_2 as i64);
        // C s_78_4: cast zx s_78_3 -> i
        let s_78_4: i128 = (i128::try_from(s_78_3).unwrap());
        // C s_78_5: const #424u : u32
        let s_78_5: u32 = 424;
        // D s_78_6: read-reg s_78_5:u8
        let s_78_6: u8 = {
            let value = state.read_register::<u8>(s_78_5 as isize);
            tracer.read_register(s_78_5 as isize, value);
            value
        };
        // D s_78_7: call AArch64_AArch32SystemAccessTrap(s_78_6, s_78_4)
        let s_78_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_78_6, s_78_4);
        // N s_78_8: return
        return;
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_79_0: panic
        panic!("{:?}", ());
        // N s_79_1: return
        return;
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #() : ()
        let s_80_0: () = ();
        // S s_80_1: call EDSCR_read(s_80_0)
        let s_80_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_80_0);
        // S s_80_2: call _get_EDSCR_Type_SDD(s_80_1)
        let s_80_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_80_1);
        // S s_80_3: cast zx s_80_2 -> bv
        let s_80_3: Bits = Bits::new(s_80_2 as u128, 1u16);
        // C s_80_4: const #1u : u8
        let s_80_4: bool = true;
        // C s_80_5: cast zx s_80_4 -> bv
        let s_80_5: Bits = Bits::new(s_80_4 as u128, 1u16);
        // S s_80_6: cmp-eq s_80_3 s_80_5
        let s_80_6: bool = ((s_80_3) == (s_80_5));
        // D s_80_7: write-var gs#106498 <= s_80_6
        fn_state.gs_106498 = s_80_6;
        // N s_80_8: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var __CPTR_EL3_TAM:u8
        let s_81_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_81_1: cast zx s_81_0 -> bv
        let s_81_1: Bits = Bits::new(s_81_0 as u128, 1u16);
        // C s_81_2: const #1u : u8
        let s_81_2: bool = true;
        // C s_81_3: cast zx s_81_2 -> bv
        let s_81_3: Bits = Bits::new(s_81_2 as u128, 1u16);
        // D s_81_4: cmp-eq s_81_1 s_81_3
        let s_81_4: bool = ((s_81_1) == (s_81_3));
        // D s_81_5: write-var gs#106496 <= s_81_4
        fn_state.gs_106496 = s_81_4;
        // N s_81_6: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #424u : u32
        let s_82_0: u32 = 424;
        // D s_82_1: read-reg s_82_0:u8
        let s_82_1: u8 = {
            let value = state.read_register::<u8>(s_82_0 as isize);
            tracer.read_register(s_82_0 as isize, value);
            value
        };
        // D s_82_2: call ELUsingAArch32(s_82_1)
        let s_82_2: bool = ELUsingAArch32(state, tracer, s_82_1);
        // D s_82_3: not s_82_2
        let s_82_3: bool = !s_82_2;
        // D s_82_4: write-var gs#106495 <= s_82_3
        fn_state.gs_106495 = s_82_3;
        // N s_82_5: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #3u : u8
        let s_83_0: u8 = 3;
        // C s_83_1: cast zx s_83_0 -> bv
        let s_83_1: Bits = Bits::new(s_83_0 as u128, 8u16);
        // C s_83_2: cast zx s_83_1 -> i
        let s_83_2: i128 = (s_83_1.value() as i128);
        // C s_83_3: cast reint s_83_2 -> i64
        let s_83_3: i64 = (s_83_2 as i64);
        // C s_83_4: cast zx s_83_3 -> i
        let s_83_4: i128 = (i128::try_from(s_83_3).unwrap());
        // S s_83_5: call AArch32_TakeHypTrapException(s_83_4)
        let s_83_5: () = AArch32_TakeHypTrapException(state, tracer, s_83_4);
        // N s_83_6: return
        return;
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var __HCPTR_TAM:u8
        let s_84_0: bool = fn_state.u__HCPTR_TAM;
        // D s_84_1: cast zx s_84_0 -> bv
        let s_84_1: Bits = Bits::new(s_84_0 as u128, 1u16);
        // C s_84_2: const #1u : u8
        let s_84_2: bool = true;
        // C s_84_3: cast zx s_84_2 -> bv
        let s_84_3: Bits = Bits::new(s_84_2 as u128, 1u16);
        // D s_84_4: cmp-eq s_84_1 s_84_3
        let s_84_4: bool = ((s_84_1) == (s_84_3));
        // D s_84_5: write-var gs#106494 <= s_84_4
        fn_state.gs_106494 = s_84_4;
        // N s_84_6: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #432u : u32
        let s_85_0: u32 = 432;
        // D s_85_1: read-reg s_85_0:u8
        let s_85_1: u8 = {
            let value = state.read_register::<u8>(s_85_0 as isize);
            tracer.read_register(s_85_0 as isize, value);
            value
        };
        // D s_85_2: call ELUsingAArch32(s_85_1)
        let s_85_2: bool = ELUsingAArch32(state, tracer, s_85_1);
        // D s_85_3: write-var gs#106493 <= s_85_2
        fn_state.gs_106493 = s_85_2;
        // N s_85_4: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #3u : u8
        let s_86_0: u8 = 3;
        // C s_86_1: cast zx s_86_0 -> bv
        let s_86_1: Bits = Bits::new(s_86_0 as u128, 8u16);
        // C s_86_2: cast zx s_86_1 -> i
        let s_86_2: i128 = (s_86_1.value() as i128);
        // C s_86_3: cast reint s_86_2 -> i64
        let s_86_3: i64 = (s_86_2 as i64);
        // C s_86_4: cast zx s_86_3 -> i
        let s_86_4: i128 = (i128::try_from(s_86_3).unwrap());
        // C s_86_5: const #432u : u32
        let s_86_5: u32 = 432;
        // D s_86_6: read-reg s_86_5:u8
        let s_86_6: u8 = {
            let value = state.read_register::<u8>(s_86_5 as isize);
            tracer.read_register(s_86_5 as isize, value);
            value
        };
        // D s_86_7: call AArch64_AArch32SystemAccessTrap(s_86_6, s_86_4)
        let s_86_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_86_6, s_86_4);
        // N s_86_8: return
        return;
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var __CPTR_EL2_TAM:u8
        let s_87_0: bool = fn_state.u__CPTR_EL2_TAM;
        // D s_87_1: cast zx s_87_0 -> bv
        let s_87_1: Bits = Bits::new(s_87_0 as u128, 1u16);
        // C s_87_2: const #1u : u8
        let s_87_2: bool = true;
        // C s_87_3: cast zx s_87_2 -> bv
        let s_87_3: Bits = Bits::new(s_87_2 as u128, 1u16);
        // D s_87_4: cmp-eq s_87_1 s_87_3
        let s_87_4: bool = ((s_87_1) == (s_87_3));
        // D s_87_5: write-var gs#106492 <= s_87_4
        fn_state.gs_106492 = s_87_4;
        // N s_87_6: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #432u : u32
        let s_88_0: u32 = 432;
        // D s_88_1: read-reg s_88_0:u8
        let s_88_1: u8 = {
            let value = state.read_register::<u8>(s_88_0 as isize);
            tracer.read_register(s_88_0 as isize, value);
            value
        };
        // D s_88_2: call ELUsingAArch32(s_88_1)
        let s_88_2: bool = ELUsingAArch32(state, tracer, s_88_1);
        // D s_88_3: not s_88_2
        let s_88_3: bool = !s_88_2;
        // D s_88_4: write-var gs#106491 <= s_88_3
        fn_state.gs_106491 = s_88_3;
        // N s_88_5: jump b61
        return block_61(state, tracer, fn_state);
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
        // D s_90_0: read-var __HSTR_T13:u8
        let s_90_0: bool = fn_state.u__HSTR_T13;
        // D s_90_1: cast zx s_90_0 -> bv
        let s_90_1: Bits = Bits::new(s_90_0 as u128, 1u16);
        // C s_90_2: const #1u : u8
        let s_90_2: bool = true;
        // C s_90_3: cast zx s_90_2 -> bv
        let s_90_3: Bits = Bits::new(s_90_2 as u128, 1u16);
        // D s_90_4: cmp-eq s_90_1 s_90_3
        let s_90_4: bool = ((s_90_1) == (s_90_3));
        // D s_90_5: write-var gs#106490 <= s_90_4
        fn_state.gs_106490 = s_90_4;
        // N s_90_6: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #432u : u32
        let s_91_0: u32 = 432;
        // D s_91_1: read-reg s_91_0:u8
        let s_91_1: u8 = {
            let value = state.read_register::<u8>(s_91_0 as isize);
            tracer.read_register(s_91_0 as isize, value);
            value
        };
        // D s_91_2: call ELUsingAArch32(s_91_1)
        let s_91_2: bool = ELUsingAArch32(state, tracer, s_91_1);
        // D s_91_3: write-var gs#106489 <= s_91_2
        fn_state.gs_106489 = s_91_2;
        // N s_91_4: jump b56
        return block_56(state, tracer, fn_state);
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
        // C s_92_5: const #432u : u32
        let s_92_5: u32 = 432;
        // D s_92_6: read-reg s_92_5:u8
        let s_92_6: u8 = {
            let value = state.read_register::<u8>(s_92_5 as isize);
            tracer.read_register(s_92_5 as isize, value);
            value
        };
        // D s_92_7: call AArch64_AArch32SystemAccessTrap(s_92_6, s_92_4)
        let s_92_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_92_6, s_92_4);
        // N s_92_8: return
        return;
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_93_0: read-var __HSTR_EL2_T13:u8
        let s_93_0: bool = fn_state.u__HSTR_EL2_T13;
        // D s_93_1: cast zx s_93_0 -> bv
        let s_93_1: Bits = Bits::new(s_93_0 as u128, 1u16);
        // C s_93_2: const #1u : u8
        let s_93_2: bool = true;
        // C s_93_3: cast zx s_93_2 -> bv
        let s_93_3: Bits = Bits::new(s_93_2 as u128, 1u16);
        // D s_93_4: cmp-eq s_93_1 s_93_3
        let s_93_4: bool = ((s_93_1) == (s_93_3));
        // D s_93_5: write-var gs#106488 <= s_93_4
        fn_state.gs_106488 = s_93_4;
        // N s_93_6: jump b53
        return block_53(state, tracer, fn_state);
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
        // D s_94_3: not s_94_2
        let s_94_3: bool = !s_94_2;
        // D s_94_4: write-var gs#106487 <= s_94_3
        fn_state.gs_106487 = s_94_3;
        // N s_94_5: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_95_0: panic
        panic!("{:?}", ());
        // N s_95_1: return
        return;
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var __CPTR_EL3_TAM:u8
        let s_96_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_96_1: cast zx s_96_0 -> bv
        let s_96_1: Bits = Bits::new(s_96_0 as u128, 1u16);
        // C s_96_2: const #1u : u8
        let s_96_2: bool = true;
        // C s_96_3: cast zx s_96_2 -> bv
        let s_96_3: Bits = Bits::new(s_96_2 as u128, 1u16);
        // D s_96_4: cmp-eq s_96_1 s_96_3
        let s_96_4: bool = ((s_96_1) == (s_96_3));
        // D s_96_5: write-var gs#106486 <= s_96_4
        fn_state.gs_106486 = s_96_4;
        // N s_96_6: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #424u : u32
        let s_97_0: u32 = 424;
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
        // D s_97_4: write-var gs#106485 <= s_97_3
        fn_state.gs_106485 = s_97_3;
        // N s_97_5: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_98_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_98_1: call __IMPDEF_boolean(s_98_0)
        let s_98_1: bool = u__IMPDEF_boolean(state, tracer, s_98_0);
        // D s_98_2: write-var gs#106484 <= s_98_1
        fn_state.gs_106484 = s_98_1;
        // N s_98_3: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #() : ()
        let s_99_0: () = ();
        // S s_99_1: call EDSCR_read(s_99_0)
        let s_99_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_99_0);
        // S s_99_2: call _get_EDSCR_Type_SDD(s_99_1)
        let s_99_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_99_1);
        // S s_99_3: cast zx s_99_2 -> bv
        let s_99_3: Bits = Bits::new(s_99_2 as u128, 1u16);
        // C s_99_4: const #1u : u8
        let s_99_4: bool = true;
        // C s_99_5: cast zx s_99_4 -> bv
        let s_99_5: Bits = Bits::new(s_99_4 as u128, 1u16);
        // S s_99_6: cmp-eq s_99_3 s_99_5
        let s_99_6: bool = ((s_99_3) == (s_99_5));
        // D s_99_7: write-var gs#106483 <= s_99_6
        fn_state.gs_106483 = s_99_6;
        // N s_99_8: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #424u : u32
        let s_100_0: u32 = 424;
        // D s_100_1: read-reg s_100_0:u8
        let s_100_1: u8 = {
            let value = state.read_register::<u8>(s_100_0 as isize);
            tracer.read_register(s_100_0 as isize, value);
            value
        };
        // C s_100_2: const #2u : u8
        let s_100_2: u8 = 2;
        // D s_100_3: cmp-lt s_100_1 s_100_2
        let s_100_3: bool = ((s_100_1) < (s_100_2));
        // D s_100_4: write-var gs#106482 <= s_100_3
        fn_state.gs_106482 = s_100_3;
        // N s_100_5: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #() : ()
        let s_101_0: () = ();
        // S s_101_1: call Halted(s_101_0)
        let s_101_1: bool = Halted(state, tracer, s_101_0);
        // N s_101_2: branch s_101_1 b220 b102
        if s_101_1 {
            return block_220(state, tracer, fn_state);
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
        // D s_102_1: write-var gs#106499 <= s_102_0
        fn_state.gs_106499 = s_102_0;
        // N s_102_2: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_103_0: read-var gs#106499:u8
        let s_103_0: bool = fn_state.gs_106499;
        // N s_103_1: branch s_103_0 b219 b104
        if s_103_0 {
            return block_219(state, tracer, fn_state);
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
        // D s_104_1: write-var gs#106500 <= s_104_0
        fn_state.gs_106500 = s_104_0;
        // N s_104_2: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_105_0: read-var gs#106500:u8
        let s_105_0: bool = fn_state.gs_106500;
        // N s_105_1: branch s_105_0 b218 b106
        if s_105_0 {
            return block_218(state, tracer, fn_state);
        } else {
            return block_106(state, tracer, fn_state);
        };
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #0u : u8
        let s_106_0: bool = false;
        // D s_106_1: write-var gs#106501 <= s_106_0
        fn_state.gs_106501 = s_106_0;
        // N s_106_2: jump b107
        return block_107(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_107_0: read-var gs#106501:u8
        let s_107_0: bool = fn_state.gs_106501;
        // N s_107_1: branch s_107_0 b217 b108
        if s_107_0 {
            return block_217(state, tracer, fn_state);
        } else {
            return block_108(state, tracer, fn_state);
        };
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #0u : u8
        let s_108_0: bool = false;
        // D s_108_1: write-var gs#106502 <= s_108_0
        fn_state.gs_106502 = s_108_0;
        // N s_108_2: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_109_0: read-var gs#106502:u8
        let s_109_0: bool = fn_state.gs_106502;
        // N s_109_1: branch s_109_0 b216 b110
        if s_109_0 {
            return block_216(state, tracer, fn_state);
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
        // D s_110_1: write-var gs#106503 <= s_110_0
        fn_state.gs_106503 = s_110_0;
        // N s_110_2: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var gs#106503:u8
        let s_111_0: bool = fn_state.gs_106503;
        // N s_111_1: branch s_111_0 b215 b112
        if s_111_0 {
            return block_215(state, tracer, fn_state);
        } else {
            return block_112(state, tracer, fn_state);
        };
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_112_0: const #440u : u32
        let s_112_0: u32 = 440;
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
        // N s_112_4: branch s_112_3 b214 b113
        if s_112_3 {
            return block_214(state, tracer, fn_state);
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
        // D s_113_1: write-var gs#106504 <= s_113_0
        fn_state.gs_106504 = s_113_0;
        // N s_113_2: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_114_0: read-var gs#106504:u8
        let s_114_0: bool = fn_state.gs_106504;
        // N s_114_1: branch s_114_0 b205 b115
        if s_114_0 {
            return block_205(state, tracer, fn_state);
        } else {
            return block_115(state, tracer, fn_state);
        };
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #440u : u32
        let s_115_0: u32 = 440;
        // D s_115_1: read-reg s_115_0:u8
        let s_115_1: u8 = {
            let value = state.read_register::<u8>(s_115_0 as isize);
            tracer.read_register(s_115_0 as isize, value);
            value
        };
        // D s_115_2: call ELUsingAArch32(s_115_1)
        let s_115_2: bool = ELUsingAArch32(state, tracer, s_115_1);
        // N s_115_3: branch s_115_2 b204 b116
        if s_115_2 {
            return block_204(state, tracer, fn_state);
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
        // D s_116_1: write-var gs#106505 <= s_116_0
        fn_state.gs_106505 = s_116_0;
        // N s_116_2: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_117_0: read-var gs#106505:u8
        let s_117_0: bool = fn_state.gs_106505;
        // N s_117_1: branch s_117_0 b187 b118
        if s_117_0 {
            return block_187(state, tracer, fn_state);
        } else {
            return block_118(state, tracer, fn_state);
        };
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #() : ()
        let s_118_0: () = ();
        // S s_118_1: call EL2Enabled(s_118_0)
        let s_118_1: bool = EL2Enabled(state, tracer, s_118_0);
        // N s_118_2: branch s_118_1 b186 b119
        if s_118_1 {
            return block_186(state, tracer, fn_state);
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
        // D s_119_1: write-var gs#106506 <= s_119_0
        fn_state.gs_106506 = s_119_0;
        // N s_119_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_120_0: read-var gs#106506:u8
        let s_120_0: bool = fn_state.gs_106506;
        // N s_120_1: branch s_120_0 b185 b121
        if s_120_0 {
            return block_185(state, tracer, fn_state);
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
        // D s_121_1: write-var gs#106507 <= s_121_0
        fn_state.gs_106507 = s_121_0;
        // N s_121_2: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_122_0: read-var gs#106507:u8
        let s_122_0: bool = fn_state.gs_106507;
        // N s_122_1: branch s_122_0 b184 b123
        if s_122_0 {
            return block_184(state, tracer, fn_state);
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
        // D s_123_1: write-var gs#106508 <= s_123_0
        fn_state.gs_106508 = s_123_0;
        // N s_123_2: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_124_0: read-var gs#106508:u8
        let s_124_0: bool = fn_state.gs_106508;
        // N s_124_1: branch s_124_0 b183 b125
        if s_124_0 {
            return block_183(state, tracer, fn_state);
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
        // S s_125_1: call EL2Enabled(s_125_0)
        let s_125_1: bool = EL2Enabled(state, tracer, s_125_0);
        // N s_125_2: branch s_125_1 b182 b126
        if s_125_1 {
            return block_182(state, tracer, fn_state);
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
        // D s_126_1: write-var gs#106509 <= s_126_0
        fn_state.gs_106509 = s_126_0;
        // N s_126_2: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_127_0: read-var gs#106509:u8
        let s_127_0: bool = fn_state.gs_106509;
        // N s_127_1: branch s_127_0 b181 b128
        if s_127_0 {
            return block_181(state, tracer, fn_state);
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
        // D s_128_1: write-var gs#106510 <= s_128_0
        fn_state.gs_106510 = s_128_0;
        // N s_128_2: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_129_0: read-var gs#106510:u8
        let s_129_0: bool = fn_state.gs_106510;
        // N s_129_1: branch s_129_0 b180 b130
        if s_129_0 {
            return block_180(state, tracer, fn_state);
        } else {
            return block_130(state, tracer, fn_state);
        };
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_130_0: const #() : ()
        let s_130_0: () = ();
        // S s_130_1: call EL2Enabled(s_130_0)
        let s_130_1: bool = EL2Enabled(state, tracer, s_130_0);
        // N s_130_2: branch s_130_1 b179 b131
        if s_130_1 {
            return block_179(state, tracer, fn_state);
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
        // D s_131_1: write-var gs#106511 <= s_131_0
        fn_state.gs_106511 = s_131_0;
        // N s_131_2: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_132_0: read-var gs#106511:u8
        let s_132_0: bool = fn_state.gs_106511;
        // N s_132_1: branch s_132_0 b178 b133
        if s_132_0 {
            return block_178(state, tracer, fn_state);
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
        // D s_133_1: write-var gs#106512 <= s_133_0
        fn_state.gs_106512 = s_133_0;
        // N s_133_2: jump b134
        return block_134(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_134_0: read-var gs#106512:u8
        let s_134_0: bool = fn_state.gs_106512;
        // N s_134_1: branch s_134_0 b177 b135
        if s_134_0 {
            return block_177(state, tracer, fn_state);
        } else {
            return block_135(state, tracer, fn_state);
        };
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_135_0: const #() : ()
        let s_135_0: () = ();
        // S s_135_1: call EL2Enabled(s_135_0)
        let s_135_1: bool = EL2Enabled(state, tracer, s_135_0);
        // N s_135_2: branch s_135_1 b176 b136
        if s_135_1 {
            return block_176(state, tracer, fn_state);
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
        // D s_136_1: write-var gs#106513 <= s_136_0
        fn_state.gs_106513 = s_136_0;
        // N s_136_2: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_137_0: read-var gs#106513:u8
        let s_137_0: bool = fn_state.gs_106513;
        // N s_137_1: branch s_137_0 b175 b138
        if s_137_0 {
            return block_175(state, tracer, fn_state);
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
        // D s_138_1: write-var gs#106514 <= s_138_0
        fn_state.gs_106514 = s_138_0;
        // N s_138_2: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_139_0: read-var gs#106514:u8
        let s_139_0: bool = fn_state.gs_106514;
        // N s_139_1: branch s_139_0 b174 b140
        if s_139_0 {
            return block_174(state, tracer, fn_state);
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
        // N s_140_2: branch s_140_1 b173 b141
        if s_140_1 {
            return block_173(state, tracer, fn_state);
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
        // D s_141_1: write-var gs#106515 <= s_141_0
        fn_state.gs_106515 = s_141_0;
        // N s_141_2: jump b142
        return block_142(state, tracer, fn_state);
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_142_0: read-var gs#106515:u8
        let s_142_0: bool = fn_state.gs_106515;
        // N s_142_1: branch s_142_0 b172 b143
        if s_142_0 {
            return block_172(state, tracer, fn_state);
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
        // D s_143_1: write-var gs#106516 <= s_143_0
        fn_state.gs_106516 = s_143_0;
        // N s_143_2: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_144_0: read-var gs#106516:u8
        let s_144_0: bool = fn_state.gs_106516;
        // N s_144_1: branch s_144_0 b171 b145
        if s_144_0 {
            return block_171(state, tracer, fn_state);
        } else {
            return block_145(state, tracer, fn_state);
        };
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_145_0: const #0u : u8
        let s_145_0: bool = false;
        // D s_145_1: write-var gs#106517 <= s_145_0
        fn_state.gs_106517 = s_145_0;
        // N s_145_2: jump b146
        return block_146(state, tracer, fn_state);
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_146_0: read-var gs#106517:u8
        let s_146_0: bool = fn_state.gs_106517;
        // N s_146_1: branch s_146_0 b167 b147
        if s_146_0 {
            return block_167(state, tracer, fn_state);
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
        // D s_147_1: write-var gs#106519 <= s_147_0
        fn_state.gs_106519 = s_147_0;
        // N s_147_2: jump b148
        return block_148(state, tracer, fn_state);
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_148_0: read-var gs#106519:u8
        let s_148_0: bool = fn_state.gs_106519;
        // N s_148_1: branch s_148_0 b166 b149
        if s_148_0 {
            return block_166(state, tracer, fn_state);
        } else {
            return block_149(state, tracer, fn_state);
        };
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_149_0: const #0u : u8
        let s_149_0: bool = false;
        // D s_149_1: write-var gs#106520 <= s_149_0
        fn_state.gs_106520 = s_149_0;
        // N s_149_2: jump b150
        return block_150(state, tracer, fn_state);
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_150_0: read-var gs#106520:u8
        let s_150_0: bool = fn_state.gs_106520;
        // N s_150_1: branch s_150_0 b165 b151
        if s_150_0 {
            return block_165(state, tracer, fn_state);
        } else {
            return block_151(state, tracer, fn_state);
        };
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_151_0: const #424u : u32
        let s_151_0: u32 = 424;
        // D s_151_1: read-reg s_151_0:u8
        let s_151_1: u8 = {
            let value = state.read_register::<u8>(s_151_0 as isize);
            tracer.read_register(s_151_0 as isize, value);
            value
        };
        // C s_151_2: const #2u : u8
        let s_151_2: u8 = 2;
        // D s_151_3: cmp-lt s_151_1 s_151_2
        let s_151_3: bool = ((s_151_1) < (s_151_2));
        // N s_151_4: branch s_151_3 b164 b152
        if s_151_3 {
            return block_164(state, tracer, fn_state);
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
        // D s_152_1: write-var gs#106521 <= s_152_0
        fn_state.gs_106521 = s_152_0;
        // N s_152_2: jump b153
        return block_153(state, tracer, fn_state);
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_153_0: read-var gs#106521:u8
        let s_153_0: bool = fn_state.gs_106521;
        // N s_153_1: branch s_153_0 b163 b154
        if s_153_0 {
            return block_163(state, tracer, fn_state);
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
        // D s_154_1: write-var gs#106522 <= s_154_0
        fn_state.gs_106522 = s_154_0;
        // N s_154_2: jump b155
        return block_155(state, tracer, fn_state);
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_155_0: read-var gs#106522:u8
        let s_155_0: bool = fn_state.gs_106522;
        // N s_155_1: branch s_155_0 b157 b156
        if s_155_0 {
            return block_157(state, tracer, fn_state);
        } else {
            return block_156(state, tracer, fn_state);
        };
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_156_0: const #5s : i64
        let s_156_0: i64 = 5;
        // S s_156_1: call AMEVTYPER1_read(s_156_0)
        let s_156_1: ProductType700c18a878c5601b = AMEVTYPER1_read(
            state,
            tracer,
            s_156_0,
        );
        // S s_156_2: call __get_AMEVTYPER1(s_156_1)
        let s_156_2: ProductType700c18a878c5601b = u__get_AMEVTYPER1(
            state,
            tracer,
            s_156_1,
        );
        // D s_156_3: write-var ga#165223 <= s_156_2
        fn_state.ga_165223 = s_156_2;
        // D s_156_4: read-var ga#165223.0:struct
        let s_156_4: u32 = fn_state.ga_165223._0;
        // D s_156_5: read-var t:i
        let s_156_5: i128 = fn_state.t;
        // D s_156_6: call R_set(s_156_5, s_156_4)
        let s_156_6: () = R_set(state, tracer, s_156_5, s_156_4);
        // N s_156_7: return
        return;
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_157_0: const #() : ()
        let s_157_0: () = ();
        // S s_157_1: call Halted(s_157_0)
        let s_157_1: bool = Halted(state, tracer, s_157_0);
        // N s_157_2: branch s_157_1 b162 b158
        if s_157_1 {
            return block_162(state, tracer, fn_state);
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
        // D s_158_1: write-var gs#106524 <= s_158_0
        fn_state.gs_106524 = s_158_0;
        // N s_158_2: jump b159
        return block_159(state, tracer, fn_state);
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_159_0: read-var gs#106524:u8
        let s_159_0: bool = fn_state.gs_106524;
        // N s_159_1: branch s_159_0 b161 b160
        if s_159_0 {
            return block_161(state, tracer, fn_state);
        } else {
            return block_160(state, tracer, fn_state);
        };
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_160_0: const #3u : u8
        let s_160_0: u8 = 3;
        // C s_160_1: cast zx s_160_0 -> bv
        let s_160_1: Bits = Bits::new(s_160_0 as u128, 8u16);
        // C s_160_2: cast zx s_160_1 -> i
        let s_160_2: i128 = (s_160_1.value() as i128);
        // C s_160_3: cast reint s_160_2 -> i64
        let s_160_3: i64 = (s_160_2 as i64);
        // C s_160_4: cast zx s_160_3 -> i
        let s_160_4: i128 = (i128::try_from(s_160_3).unwrap());
        // C s_160_5: const #424u : u32
        let s_160_5: u32 = 424;
        // D s_160_6: read-reg s_160_5:u8
        let s_160_6: u8 = {
            let value = state.read_register::<u8>(s_160_5 as isize);
            tracer.read_register(s_160_5 as isize, value);
            value
        };
        // D s_160_7: call AArch64_AArch32SystemAccessTrap(s_160_6, s_160_4)
        let s_160_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_160_6,
            s_160_4,
        );
        // N s_160_8: return
        return;
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_161_0: panic
        panic!("{:?}", ());
        // N s_161_1: return
        return;
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_162_0: const #() : ()
        let s_162_0: () = ();
        // S s_162_1: call EDSCR_read(s_162_0)
        let s_162_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_162_0);
        // S s_162_2: call _get_EDSCR_Type_SDD(s_162_1)
        let s_162_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_162_1);
        // S s_162_3: cast zx s_162_2 -> bv
        let s_162_3: Bits = Bits::new(s_162_2 as u128, 1u16);
        // C s_162_4: const #1u : u8
        let s_162_4: bool = true;
        // C s_162_5: cast zx s_162_4 -> bv
        let s_162_5: Bits = Bits::new(s_162_4 as u128, 1u16);
        // S s_162_6: cmp-eq s_162_3 s_162_5
        let s_162_6: bool = ((s_162_3) == (s_162_5));
        // D s_162_7: write-var gs#106524 <= s_162_6
        fn_state.gs_106524 = s_162_6;
        // N s_162_8: jump b159
        return block_159(state, tracer, fn_state);
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_163_0: read-var __CPTR_EL3_TAM:u8
        let s_163_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_163_1: cast zx s_163_0 -> bv
        let s_163_1: Bits = Bits::new(s_163_0 as u128, 1u16);
        // C s_163_2: const #1u : u8
        let s_163_2: bool = true;
        // C s_163_3: cast zx s_163_2 -> bv
        let s_163_3: Bits = Bits::new(s_163_2 as u128, 1u16);
        // D s_163_4: cmp-eq s_163_1 s_163_3
        let s_163_4: bool = ((s_163_1) == (s_163_3));
        // D s_163_5: write-var gs#106522 <= s_163_4
        fn_state.gs_106522 = s_163_4;
        // N s_163_6: jump b155
        return block_155(state, tracer, fn_state);
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_164_0: const #424u : u32
        let s_164_0: u32 = 424;
        // D s_164_1: read-reg s_164_0:u8
        let s_164_1: u8 = {
            let value = state.read_register::<u8>(s_164_0 as isize);
            tracer.read_register(s_164_0 as isize, value);
            value
        };
        // D s_164_2: call ELUsingAArch32(s_164_1)
        let s_164_2: bool = ELUsingAArch32(state, tracer, s_164_1);
        // D s_164_3: not s_164_2
        let s_164_3: bool = !s_164_2;
        // D s_164_4: write-var gs#106521 <= s_164_3
        fn_state.gs_106521 = s_164_3;
        // N s_164_5: jump b153
        return block_153(state, tracer, fn_state);
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_165_0: const #3u : u8
        let s_165_0: u8 = 3;
        // C s_165_1: cast zx s_165_0 -> bv
        let s_165_1: Bits = Bits::new(s_165_0 as u128, 8u16);
        // C s_165_2: cast zx s_165_1 -> i
        let s_165_2: i128 = (s_165_1.value() as i128);
        // C s_165_3: cast reint s_165_2 -> i64
        let s_165_3: i64 = (s_165_2 as i64);
        // C s_165_4: cast zx s_165_3 -> i
        let s_165_4: i128 = (i128::try_from(s_165_3).unwrap());
        // C s_165_5: const #432u : u32
        let s_165_5: u32 = 432;
        // D s_165_6: read-reg s_165_5:u8
        let s_165_6: u8 = {
            let value = state.read_register::<u8>(s_165_5 as isize);
            tracer.read_register(s_165_5 as isize, value);
            value
        };
        // D s_165_7: call AArch64_AArch32SystemAccessTrap(s_165_6, s_165_4)
        let s_165_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_165_6,
            s_165_4,
        );
        // N s_165_8: return
        return;
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_166_0: read-var __HAFGRTR_EL2_AMEVTYPER15_EL0:u8
        let s_166_0: bool = fn_state.u__HAFGRTR_EL2_AMEVTYPER15_EL0;
        // D s_166_1: cast zx s_166_0 -> bv
        let s_166_1: Bits = Bits::new(s_166_0 as u128, 1u16);
        // C s_166_2: const #1u : u8
        let s_166_2: bool = true;
        // C s_166_3: cast zx s_166_2 -> bv
        let s_166_3: Bits = Bits::new(s_166_2 as u128, 1u16);
        // D s_166_4: cmp-eq s_166_1 s_166_3
        let s_166_4: bool = ((s_166_1) == (s_166_3));
        // D s_166_5: write-var gs#106520 <= s_166_4
        fn_state.gs_106520 = s_166_4;
        // N s_166_6: jump b150
        return block_150(state, tracer, fn_state);
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_167_0: const #424u : u32
        let s_167_0: u32 = 424;
        // D s_167_1: read-reg s_167_0:u8
        let s_167_1: u8 = {
            let value = state.read_register::<u8>(s_167_0 as isize);
            tracer.read_register(s_167_0 as isize, value);
            value
        };
        // C s_167_2: const #2u : u8
        let s_167_2: u8 = 2;
        // D s_167_3: cmp-lt s_167_1 s_167_2
        let s_167_3: bool = ((s_167_1) < (s_167_2));
        // D s_167_4: not s_167_3
        let s_167_4: bool = !s_167_3;
        // N s_167_5: branch s_167_4 b170 b168
        if s_167_4 {
            return block_170(state, tracer, fn_state);
        } else {
            return block_168(state, tracer, fn_state);
        };
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_168_0: read-var __SCR_EL3_FGTEn:u8
        let s_168_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_168_1: cast zx s_168_0 -> bv
        let s_168_1: Bits = Bits::new(s_168_0 as u128, 1u16);
        // C s_168_2: const #1u : u8
        let s_168_2: bool = true;
        // C s_168_3: cast zx s_168_2 -> bv
        let s_168_3: Bits = Bits::new(s_168_2 as u128, 1u16);
        // D s_168_4: cmp-eq s_168_1 s_168_3
        let s_168_4: bool = ((s_168_1) == (s_168_3));
        // D s_168_5: write-var gs#106518 <= s_168_4
        fn_state.gs_106518 = s_168_4;
        // N s_168_6: jump b169
        return block_169(state, tracer, fn_state);
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_169_0: read-var gs#106518:u8
        let s_169_0: bool = fn_state.gs_106518;
        // D s_169_1: write-var gs#106519 <= s_169_0
        fn_state.gs_106519 = s_169_0;
        // N s_169_2: jump b148
        return block_148(state, tracer, fn_state);
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_170_0: const #1u : u8
        let s_170_0: bool = true;
        // D s_170_1: write-var gs#106518 <= s_170_0
        fn_state.gs_106518 = s_170_0;
        // N s_170_2: jump b169
        return block_169(state, tracer, fn_state);
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_171_0: const #146u : u32
        let s_171_0: u32 = 146;
        // S s_171_1: call IsFeatureImplemented(s_171_0)
        let s_171_1: bool = IsFeatureImplemented(state, tracer, s_171_0);
        // D s_171_2: write-var gs#106517 <= s_171_1
        fn_state.gs_106517 = s_171_1;
        // N s_171_3: jump b146
        return block_146(state, tracer, fn_state);
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_172_0: const #102552u : u32
        let s_172_0: u32 = 102552;
        // D s_172_1: read-reg s_172_0:struct
        let s_172_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_172_0 as isize);
            tracer.read_register(s_172_0 as isize, value);
            value
        };
        // D s_172_2: call _get_HCR_EL2_Type_E2H(s_172_1)
        let s_172_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_172_1);
        // C s_172_3: const #102552u : u32
        let s_172_3: u32 = 102552;
        // D s_172_4: read-reg s_172_3:struct
        let s_172_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_172_3 as isize);
            tracer.read_register(s_172_3 as isize, value);
            value
        };
        // D s_172_5: call _get_HCR_EL2_Type_TGE(s_172_4)
        let s_172_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_172_4);
        // D s_172_6: cast zx s_172_2 -> bv
        let s_172_6: Bits = Bits::new(s_172_2 as u128, 1u16);
        // D s_172_7: cast zx s_172_5 -> bv
        let s_172_7: Bits = Bits::new(s_172_5 as u128, 1u16);
        // D s_172_8: cast reint s_172_6 -> u128
        let s_172_8: u128 = (s_172_6.value() as u128);
        // D s_172_9: size-of s_172_6
        let s_172_9: u16 = s_172_6.length();
        // D s_172_10: cast reint s_172_7 -> u128
        let s_172_10: u128 = (s_172_7.value() as u128);
        // D s_172_11: size-of s_172_7
        let s_172_11: u16 = s_172_7.length();
        // D s_172_12: lsl s_172_8 s_172_11
        let s_172_12: u128 = s_172_8 << s_172_11;
        // D s_172_13: or s_172_12 s_172_10
        let s_172_13: u128 = ((s_172_12) | (s_172_10));
        // D s_172_14: add s_172_9 s_172_11
        let s_172_14: u16 = (s_172_9 + s_172_11);
        // D s_172_15: create-bits s_172_13 s_172_14
        let s_172_15: Bits = Bits::new(s_172_13, s_172_14);
        // D s_172_16: cast reint s_172_15 -> u8
        let s_172_16: u8 = (s_172_15.value() as u8);
        // D s_172_17: cast zx s_172_16 -> bv
        let s_172_17: Bits = Bits::new(s_172_16 as u128, 2u16);
        // C s_172_18: const #3u : u8
        let s_172_18: u8 = 3;
        // C s_172_19: cast zx s_172_18 -> bv
        let s_172_19: Bits = Bits::new(s_172_18 as u128, 2u16);
        // D s_172_20: cmp-ne s_172_17 s_172_19
        let s_172_20: bool = ((s_172_17) != (s_172_19));
        // D s_172_21: write-var gs#106516 <= s_172_20
        fn_state.gs_106516 = s_172_20;
        // N s_172_22: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_173_0: const #440u : u32
        let s_173_0: u32 = 440;
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
        // D s_173_4: write-var gs#106515 <= s_173_3
        fn_state.gs_106515 = s_173_3;
        // N s_173_5: jump b142
        return block_142(state, tracer, fn_state);
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
        // D s_175_0: read-var __HCPTR_TAM:u8
        let s_175_0: bool = fn_state.u__HCPTR_TAM;
        // D s_175_1: cast zx s_175_0 -> bv
        let s_175_1: Bits = Bits::new(s_175_0 as u128, 1u16);
        // C s_175_2: const #1u : u8
        let s_175_2: bool = true;
        // C s_175_3: cast zx s_175_2 -> bv
        let s_175_3: Bits = Bits::new(s_175_2 as u128, 1u16);
        // D s_175_4: cmp-eq s_175_1 s_175_3
        let s_175_4: bool = ((s_175_1) == (s_175_3));
        // D s_175_5: write-var gs#106514 <= s_175_4
        fn_state.gs_106514 = s_175_4;
        // N s_175_6: jump b139
        return block_139(state, tracer, fn_state);
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
        // D s_176_3: write-var gs#106513 <= s_176_2
        fn_state.gs_106513 = s_176_2;
        // N s_176_4: jump b137
        return block_137(state, tracer, fn_state);
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
        // D s_178_0: read-var __CPTR_EL2_TAM:u8
        let s_178_0: bool = fn_state.u__CPTR_EL2_TAM;
        // D s_178_1: cast zx s_178_0 -> bv
        let s_178_1: Bits = Bits::new(s_178_0 as u128, 1u16);
        // C s_178_2: const #1u : u8
        let s_178_2: bool = true;
        // C s_178_3: cast zx s_178_2 -> bv
        let s_178_3: Bits = Bits::new(s_178_2 as u128, 1u16);
        // D s_178_4: cmp-eq s_178_1 s_178_3
        let s_178_4: bool = ((s_178_1) == (s_178_3));
        // D s_178_5: write-var gs#106512 <= s_178_4
        fn_state.gs_106512 = s_178_4;
        // N s_178_6: jump b134
        return block_134(state, tracer, fn_state);
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
        // D s_179_4: write-var gs#106511 <= s_179_3
        fn_state.gs_106511 = s_179_3;
        // N s_179_5: jump b132
        return block_132(state, tracer, fn_state);
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
        // D s_181_0: read-var __HSTR_T13:u8
        let s_181_0: bool = fn_state.u__HSTR_T13;
        // D s_181_1: cast zx s_181_0 -> bv
        let s_181_1: Bits = Bits::new(s_181_0 as u128, 1u16);
        // C s_181_2: const #1u : u8
        let s_181_2: bool = true;
        // C s_181_3: cast zx s_181_2 -> bv
        let s_181_3: Bits = Bits::new(s_181_2 as u128, 1u16);
        // D s_181_4: cmp-eq s_181_1 s_181_3
        let s_181_4: bool = ((s_181_1) == (s_181_3));
        // D s_181_5: write-var gs#106510 <= s_181_4
        fn_state.gs_106510 = s_181_4;
        // N s_181_6: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_182_0: const #432u : u32
        let s_182_0: u32 = 432;
        // D s_182_1: read-reg s_182_0:u8
        let s_182_1: u8 = {
            let value = state.read_register::<u8>(s_182_0 as isize);
            tracer.read_register(s_182_0 as isize, value);
            value
        };
        // D s_182_2: call ELUsingAArch32(s_182_1)
        let s_182_2: bool = ELUsingAArch32(state, tracer, s_182_1);
        // D s_182_3: write-var gs#106509 <= s_182_2
        fn_state.gs_106509 = s_182_2;
        // N s_182_4: jump b127
        return block_127(state, tracer, fn_state);
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
        // C s_183_5: const #432u : u32
        let s_183_5: u32 = 432;
        // D s_183_6: read-reg s_183_5:u8
        let s_183_6: u8 = {
            let value = state.read_register::<u8>(s_183_5 as isize);
            tracer.read_register(s_183_5 as isize, value);
            value
        };
        // D s_183_7: call AArch64_AArch32SystemAccessTrap(s_183_6, s_183_4)
        let s_183_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_183_6,
            s_183_4,
        );
        // N s_183_8: return
        return;
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_184_0: read-var __HSTR_EL2_T13:u8
        let s_184_0: bool = fn_state.u__HSTR_EL2_T13;
        // D s_184_1: cast zx s_184_0 -> bv
        let s_184_1: Bits = Bits::new(s_184_0 as u128, 1u16);
        // C s_184_2: const #1u : u8
        let s_184_2: bool = true;
        // C s_184_3: cast zx s_184_2 -> bv
        let s_184_3: Bits = Bits::new(s_184_2 as u128, 1u16);
        // D s_184_4: cmp-eq s_184_1 s_184_3
        let s_184_4: bool = ((s_184_1) == (s_184_3));
        // D s_184_5: write-var gs#106508 <= s_184_4
        fn_state.gs_106508 = s_184_4;
        // N s_184_6: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_185<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_185_0: const #102552u : u32
        let s_185_0: u32 = 102552;
        // D s_185_1: read-reg s_185_0:struct
        let s_185_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_185_0 as isize);
            tracer.read_register(s_185_0 as isize, value);
            value
        };
        // D s_185_2: call _get_HCR_EL2_Type_E2H(s_185_1)
        let s_185_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_185_1);
        // C s_185_3: const #102552u : u32
        let s_185_3: u32 = 102552;
        // D s_185_4: read-reg s_185_3:struct
        let s_185_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_185_3 as isize);
            tracer.read_register(s_185_3 as isize, value);
            value
        };
        // D s_185_5: call _get_HCR_EL2_Type_TGE(s_185_4)
        let s_185_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_185_4);
        // D s_185_6: cast zx s_185_2 -> bv
        let s_185_6: Bits = Bits::new(s_185_2 as u128, 1u16);
        // D s_185_7: cast zx s_185_5 -> bv
        let s_185_7: Bits = Bits::new(s_185_5 as u128, 1u16);
        // D s_185_8: cast reint s_185_6 -> u128
        let s_185_8: u128 = (s_185_6.value() as u128);
        // D s_185_9: size-of s_185_6
        let s_185_9: u16 = s_185_6.length();
        // D s_185_10: cast reint s_185_7 -> u128
        let s_185_10: u128 = (s_185_7.value() as u128);
        // D s_185_11: size-of s_185_7
        let s_185_11: u16 = s_185_7.length();
        // D s_185_12: lsl s_185_8 s_185_11
        let s_185_12: u128 = s_185_8 << s_185_11;
        // D s_185_13: or s_185_12 s_185_10
        let s_185_13: u128 = ((s_185_12) | (s_185_10));
        // D s_185_14: add s_185_9 s_185_11
        let s_185_14: u16 = (s_185_9 + s_185_11);
        // D s_185_15: create-bits s_185_13 s_185_14
        let s_185_15: Bits = Bits::new(s_185_13, s_185_14);
        // D s_185_16: cast reint s_185_15 -> u8
        let s_185_16: u8 = (s_185_15.value() as u8);
        // D s_185_17: cast zx s_185_16 -> bv
        let s_185_17: Bits = Bits::new(s_185_16 as u128, 2u16);
        // C s_185_18: const #3u : u8
        let s_185_18: u8 = 3;
        // C s_185_19: cast zx s_185_18 -> bv
        let s_185_19: Bits = Bits::new(s_185_18 as u128, 2u16);
        // D s_185_20: cmp-ne s_185_17 s_185_19
        let s_185_20: bool = ((s_185_17) != (s_185_19));
        // D s_185_21: write-var gs#106507 <= s_185_20
        fn_state.gs_106507 = s_185_20;
        // N s_185_22: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_186_0: const #432u : u32
        let s_186_0: u32 = 432;
        // D s_186_1: read-reg s_186_0:u8
        let s_186_1: u8 = {
            let value = state.read_register::<u8>(s_186_0 as isize);
            tracer.read_register(s_186_0 as isize, value);
            value
        };
        // D s_186_2: call ELUsingAArch32(s_186_1)
        let s_186_2: bool = ELUsingAArch32(state, tracer, s_186_1);
        // D s_186_3: not s_186_2
        let s_186_3: bool = !s_186_2;
        // D s_186_4: write-var gs#106506 <= s_186_3
        fn_state.gs_106506 = s_186_3;
        // N s_186_5: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_187_0: const #() : ()
        let s_187_0: () = ();
        // S s_187_1: call EL2Enabled(s_187_0)
        let s_187_1: bool = EL2Enabled(state, tracer, s_187_0);
        // N s_187_2: branch s_187_1 b203 b188
        if s_187_1 {
            return block_203(state, tracer, fn_state);
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
        // D s_188_1: write-var gs#106525 <= s_188_0
        fn_state.gs_106525 = s_188_0;
        // N s_188_2: jump b189
        return block_189(state, tracer, fn_state);
    }
    fn block_189<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_189_0: read-var gs#106525:u8
        let s_189_0: bool = fn_state.gs_106525;
        // N s_189_1: branch s_189_0 b202 b190
        if s_189_0 {
            return block_202(state, tracer, fn_state);
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
        // D s_190_1: write-var gs#106526 <= s_190_0
        fn_state.gs_106526 = s_190_0;
        // N s_190_2: jump b191
        return block_191(state, tracer, fn_state);
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_191_0: read-var gs#106526:u8
        let s_191_0: bool = fn_state.gs_106526;
        // N s_191_1: branch s_191_0 b201 b192
        if s_191_0 {
            return block_201(state, tracer, fn_state);
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
        // S s_192_1: call EL2Enabled(s_192_0)
        let s_192_1: bool = EL2Enabled(state, tracer, s_192_0);
        // N s_192_2: branch s_192_1 b200 b193
        if s_192_1 {
            return block_200(state, tracer, fn_state);
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
        // D s_193_1: write-var gs#106527 <= s_193_0
        fn_state.gs_106527 = s_193_0;
        // N s_193_2: jump b194
        return block_194(state, tracer, fn_state);
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_194_0: read-var gs#106527:u8
        let s_194_0: bool = fn_state.gs_106527;
        // N s_194_1: branch s_194_0 b199 b195
        if s_194_0 {
            return block_199(state, tracer, fn_state);
        } else {
            return block_195(state, tracer, fn_state);
        };
    }
    fn block_195<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_195_0: const #0u : u8
        let s_195_0: bool = false;
        // D s_195_1: write-var gs#106528 <= s_195_0
        fn_state.gs_106528 = s_195_0;
        // N s_195_2: jump b196
        return block_196(state, tracer, fn_state);
    }
    fn block_196<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_196_0: read-var gs#106528:u8
        let s_196_0: bool = fn_state.gs_106528;
        // N s_196_1: branch s_196_0 b198 b197
        if s_196_0 {
            return block_198(state, tracer, fn_state);
        } else {
            return block_197(state, tracer, fn_state);
        };
    }
    fn block_197<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_197_0: panic
        panic!("{:?}", ());
        // N s_197_1: return
        return;
    }
    fn block_198<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_198_0: const #0u : u8
        let s_198_0: u8 = 0;
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
        // D s_199_0: read-var __HCR_TGE:u8
        let s_199_0: bool = fn_state.u__HCR_TGE;
        // D s_199_1: cast zx s_199_0 -> bv
        let s_199_1: Bits = Bits::new(s_199_0 as u128, 1u16);
        // C s_199_2: const #1u : u8
        let s_199_2: bool = true;
        // C s_199_3: cast zx s_199_2 -> bv
        let s_199_3: Bits = Bits::new(s_199_2 as u128, 1u16);
        // D s_199_4: cmp-eq s_199_1 s_199_3
        let s_199_4: bool = ((s_199_1) == (s_199_3));
        // D s_199_5: write-var gs#106528 <= s_199_4
        fn_state.gs_106528 = s_199_4;
        // N s_199_6: jump b196
        return block_196(state, tracer, fn_state);
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
        // D s_200_3: write-var gs#106527 <= s_200_2
        fn_state.gs_106527 = s_200_2;
        // N s_200_4: jump b194
        return block_194(state, tracer, fn_state);
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
        // D s_202_0: read-var __HCR_EL2_TGE:u8
        let s_202_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_202_1: cast zx s_202_0 -> bv
        let s_202_1: Bits = Bits::new(s_202_0 as u128, 1u16);
        // C s_202_2: const #1u : u8
        let s_202_2: bool = true;
        // C s_202_3: cast zx s_202_2 -> bv
        let s_202_3: Bits = Bits::new(s_202_2 as u128, 1u16);
        // D s_202_4: cmp-eq s_202_1 s_202_3
        let s_202_4: bool = ((s_202_1) == (s_202_3));
        // D s_202_5: write-var gs#106526 <= s_202_4
        fn_state.gs_106526 = s_202_4;
        // N s_202_6: jump b191
        return block_191(state, tracer, fn_state);
    }
    fn block_203<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_203_0: const #432u : u32
        let s_203_0: u32 = 432;
        // D s_203_1: read-reg s_203_0:u8
        let s_203_1: u8 = {
            let value = state.read_register::<u8>(s_203_0 as isize);
            tracer.read_register(s_203_0 as isize, value);
            value
        };
        // D s_203_2: call ELUsingAArch32(s_203_1)
        let s_203_2: bool = ELUsingAArch32(state, tracer, s_203_1);
        // D s_203_3: not s_203_2
        let s_203_3: bool = !s_203_2;
        // D s_203_4: write-var gs#106525 <= s_203_3
        fn_state.gs_106525 = s_203_3;
        // N s_203_5: jump b189
        return block_189(state, tracer, fn_state);
    }
    fn block_204<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_204_0: read-var __AMUSERENR_EN:u8
        let s_204_0: bool = fn_state.u__AMUSERENR_EN;
        // D s_204_1: cast zx s_204_0 -> bv
        let s_204_1: Bits = Bits::new(s_204_0 as u128, 1u16);
        // C s_204_2: const #0u : u8
        let s_204_2: bool = false;
        // C s_204_3: cast zx s_204_2 -> bv
        let s_204_3: Bits = Bits::new(s_204_2 as u128, 1u16);
        // D s_204_4: cmp-eq s_204_1 s_204_3
        let s_204_4: bool = ((s_204_1) == (s_204_3));
        // D s_204_5: write-var gs#106505 <= s_204_4
        fn_state.gs_106505 = s_204_4;
        // N s_204_6: jump b117
        return block_117(state, tracer, fn_state);
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
        // N s_205_2: branch s_205_1 b213 b206
        if s_205_1 {
            return block_213(state, tracer, fn_state);
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
        // D s_206_1: write-var gs#106529 <= s_206_0
        fn_state.gs_106529 = s_206_0;
        // N s_206_2: jump b207
        return block_207(state, tracer, fn_state);
    }
    fn block_207<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_207_0: read-var gs#106529:u8
        let s_207_0: bool = fn_state.gs_106529;
        // N s_207_1: branch s_207_0 b212 b208
        if s_207_0 {
            return block_212(state, tracer, fn_state);
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
        // D s_208_1: write-var gs#106530 <= s_208_0
        fn_state.gs_106530 = s_208_0;
        // N s_208_2: jump b209
        return block_209(state, tracer, fn_state);
    }
    fn block_209<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_209_0: read-var gs#106530:u8
        let s_209_0: bool = fn_state.gs_106530;
        // N s_209_1: branch s_209_0 b211 b210
        if s_209_0 {
            return block_211(state, tracer, fn_state);
        } else {
            return block_210(state, tracer, fn_state);
        };
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
        // C s_210_5: const #440u : u32
        let s_210_5: u32 = 440;
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
        // D s_212_0: read-var __HCR_EL2_TGE:u8
        let s_212_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_212_1: cast zx s_212_0 -> bv
        let s_212_1: Bits = Bits::new(s_212_0 as u128, 1u16);
        // C s_212_2: const #1u : u8
        let s_212_2: bool = true;
        // C s_212_3: cast zx s_212_2 -> bv
        let s_212_3: Bits = Bits::new(s_212_2 as u128, 1u16);
        // D s_212_4: cmp-eq s_212_1 s_212_3
        let s_212_4: bool = ((s_212_1) == (s_212_3));
        // D s_212_5: write-var gs#106530 <= s_212_4
        fn_state.gs_106530 = s_212_4;
        // N s_212_6: jump b209
        return block_209(state, tracer, fn_state);
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
        // D s_213_3: not s_213_2
        let s_213_3: bool = !s_213_2;
        // D s_213_4: write-var gs#106529 <= s_213_3
        fn_state.gs_106529 = s_213_3;
        // N s_213_5: jump b207
        return block_207(state, tracer, fn_state);
    }
    fn block_214<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_214_0: read-var __AMUSERENR_EL0_EN:u8
        let s_214_0: bool = fn_state.u__AMUSERENR_EL0_EN;
        // D s_214_1: cast zx s_214_0 -> bv
        let s_214_1: Bits = Bits::new(s_214_0 as u128, 1u16);
        // C s_214_2: const #0u : u8
        let s_214_2: bool = false;
        // C s_214_3: cast zx s_214_2 -> bv
        let s_214_3: Bits = Bits::new(s_214_2 as u128, 1u16);
        // D s_214_4: cmp-eq s_214_1 s_214_3
        let s_214_4: bool = ((s_214_1) == (s_214_3));
        // D s_214_5: write-var gs#106504 <= s_214_4
        fn_state.gs_106504 = s_214_4;
        // N s_214_6: jump b114
        return block_114(state, tracer, fn_state);
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
        // D s_216_0: read-var __CPTR_EL3_TAM:u8
        let s_216_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_216_1: cast zx s_216_0 -> bv
        let s_216_1: Bits = Bits::new(s_216_0 as u128, 1u16);
        // C s_216_2: const #1u : u8
        let s_216_2: bool = true;
        // C s_216_3: cast zx s_216_2 -> bv
        let s_216_3: Bits = Bits::new(s_216_2 as u128, 1u16);
        // D s_216_4: cmp-eq s_216_1 s_216_3
        let s_216_4: bool = ((s_216_1) == (s_216_3));
        // D s_216_5: write-var gs#106503 <= s_216_4
        fn_state.gs_106503 = s_216_4;
        // N s_216_6: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_217<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_217_0: const #424u : u32
        let s_217_0: u32 = 424;
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
        // D s_217_4: write-var gs#106502 <= s_217_3
        fn_state.gs_106502 = s_217_3;
        // N s_217_5: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_218<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_218_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_218_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_218_1: call __IMPDEF_boolean(s_218_0)
        let s_218_1: bool = u__IMPDEF_boolean(state, tracer, s_218_0);
        // D s_218_2: write-var gs#106501 <= s_218_1
        fn_state.gs_106501 = s_218_1;
        // N s_218_3: jump b107
        return block_107(state, tracer, fn_state);
    }
    fn block_219<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_219_0: const #() : ()
        let s_219_0: () = ();
        // S s_219_1: call EDSCR_read(s_219_0)
        let s_219_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_219_0);
        // S s_219_2: call _get_EDSCR_Type_SDD(s_219_1)
        let s_219_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_219_1);
        // S s_219_3: cast zx s_219_2 -> bv
        let s_219_3: Bits = Bits::new(s_219_2 as u128, 1u16);
        // C s_219_4: const #1u : u8
        let s_219_4: bool = true;
        // C s_219_5: cast zx s_219_4 -> bv
        let s_219_5: Bits = Bits::new(s_219_4 as u128, 1u16);
        // S s_219_6: cmp-eq s_219_3 s_219_5
        let s_219_6: bool = ((s_219_3) == (s_219_5));
        // D s_219_7: write-var gs#106500 <= s_219_6
        fn_state.gs_106500 = s_219_6;
        // N s_219_8: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_220<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_220_0: const #424u : u32
        let s_220_0: u32 = 424;
        // D s_220_1: read-reg s_220_0:u8
        let s_220_1: u8 = {
            let value = state.read_register::<u8>(s_220_0 as isize);
            tracer.read_register(s_220_0 as isize, value);
            value
        };
        // C s_220_2: const #2u : u8
        let s_220_2: u8 = 2;
        // D s_220_3: cmp-lt s_220_1 s_220_2
        let s_220_3: bool = ((s_220_1) < (s_220_2));
        // D s_220_4: write-var gs#106499 <= s_220_3
        fn_state.gs_106499 = s_220_3;
        // N s_220_5: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_221<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_221_0: panic
        panic!("{:?}", ());
        // N s_221_1: return
        return;
    }
}
