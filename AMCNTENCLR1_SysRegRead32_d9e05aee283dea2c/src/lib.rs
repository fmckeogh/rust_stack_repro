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
use Halted::*;
use AMUSERENR_read::*;
use AMCNTENCLR1_read::*;
use u_get_HSTR_Type_T13::*;
use u_get_HCPTR_Type_TAM::*;
use HCPTR_read::*;
use u_get_CPTR_EL2_Type_TAM::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use u_get_HAFGRTR_EL2_Type_AMCNTEN1::*;
use u_get_HCR_EL2_Type_E2H::*;
use HCR_read::*;
use IsFeatureImplemented::*;
use u__IMPDEF_boolean::*;
use AArch64_AArch32SystemAccessTrap::*;
use HSTR_read::*;
use u_get_HCR_Type_TGE::*;
use u__get_AMCNTENCLR1::*;
use u_get_CPTR_EL3_Type_TAM::*;
use u_get_HSTR_EL2_Type_T13::*;
use R_set::*;
use ELUsingAArch32::*;
use u_get_AMUSERENR_EL0_Type_EN::*;
use EDSCR_read::*;
use AArch32_TakeHypTrapException::*;
use common::*;
pub fn AMCNTENCLR1_SysRegRead32_d9e05aee283dea2c<T: Tracer>(
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
        gs_105446: bool,
        gs_105434: bool,
        gs_105455: bool,
        gs_105430: bool,
        gs_105426: bool,
        u__CPTR_EL3_TAM: bool,
        u__CPTR_EL2_TAM: bool,
        gs_105422: bool,
        gs_105459: bool,
        gs_105443: bool,
        u__HCPTR_TAM: bool,
        gs_105450: bool,
        gs_105451: bool,
        gs_105436: bool,
        u__HCR_TGE: bool,
        gs_105423: bool,
        gs_105454: bool,
        u__HSTR_T13: bool,
        gs_105460: bool,
        gs_105464: bool,
        gs_105441: bool,
        gs_105428: bool,
        gs_105427: bool,
        gs_105449: bool,
        u__PSTATE_EL: u8,
        gs_105445: bool,
        ga_162495: ProductType700c18a878c5601b,
        gs_105457: bool,
        gs_105469: bool,
        gs_105452: bool,
        gs_105431: bool,
        u__HCR_EL2_TGE: bool,
        gs_105461: bool,
        gs_105462: bool,
        gs_105463: bool,
        ga_162453: ProductType700c18a878c5601b,
        gs_105453: bool,
        gs_105438: bool,
        gs_105419: bool,
        u__SCR_EL3_FGTEn: bool,
        gs_105420: bool,
        gs_105468: bool,
        ga_162519: ProductType700c18a878c5601b,
        gs_105442: bool,
        u__AMUSERENR_EL0_EN: bool,
        gs_105415: bool,
        gs_105417: bool,
        gs_105433: bool,
        gs_105467: bool,
        gs_105447: bool,
        gs_105439: bool,
        ga_162523: ProductType700c18a878c5601b,
        gs_105437: bool,
        u__HAFGRTR_EL2_AMCNTEN1: bool,
        gs_105418: bool,
        gs_105456: bool,
        gs_105421: bool,
        gs_105424: bool,
        gs_105465: bool,
        gs_105466: bool,
        gs_105448: bool,
        gs_105458: bool,
        gs_105435: bool,
        gs_105416: bool,
        gs_105425: bool,
        gs_105429: bool,
        u__AMUSERENR_EN: bool,
        gs_105440: bool,
        gs_105444: bool,
        gs_105432: bool,
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
        // D s_0_45: call _get_HAFGRTR_EL2_Type_AMCNTEN1(s_0_44)
        let s_0_45: bool = u_get_HAFGRTR_EL2_Type_AMCNTEN1(state, tracer, s_0_44);
        // D s_0_46: write-var __HAFGRTR_EL2_AMCNTEN1 <= s_0_45
        fn_state.u__HAFGRTR_EL2_AMCNTEN1 = s_0_45;
        // D s_0_47: read-var __PSTATE_EL:u8
        let s_0_47: u8 = fn_state.u__PSTATE_EL;
        // D s_0_48: cast zx s_0_47 -> bv
        let s_0_48: Bits = Bits::new(s_0_47 as u128, 2u16);
        // C s_0_49: const #448u : u32
        let s_0_49: u32 = 448;
        // D s_0_50: read-reg s_0_49:u8
        let s_0_50: u8 = {
            let value = state.read_register::<u8>(s_0_49 as isize);
            tracer.read_register(s_0_49 as isize, value);
            value
        };
        // D s_0_51: cast zx s_0_50 -> bv
        let s_0_51: Bits = Bits::new(s_0_50 as u128, 2u16);
        // D s_0_52: cmp-eq s_0_48 s_0_51
        let s_0_52: bool = ((s_0_48) == (s_0_51));
        // N s_0_53: branch s_0_52 b100 b1
        if s_0_52 {
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
        // S s_5_1: call AMCNTENCLR1_read(s_5_0)
        let s_5_1: ProductType700c18a878c5601b = AMCNTENCLR1_read(state, tracer, s_5_0);
        // S s_5_2: call __get_AMCNTENCLR1(s_5_1)
        let s_5_2: ProductType700c18a878c5601b = u__get_AMCNTENCLR1(
            state,
            tracer,
            s_5_1,
        );
        // D s_5_3: write-var ga#162523 <= s_5_2
        fn_state.ga_162523 = s_5_2;
        // D s_5_4: read-var ga#162523.0:struct
        let s_5_4: u32 = fn_state.ga_162523._0;
        // D s_5_5: read-var t:i
        let s_5_5: i128 = fn_state.t;
        // D s_5_6: call R_set(s_5_5, s_5_4)
        let s_5_6: () = R_set(state, tracer, s_5_5, s_5_4);
        // N s_5_7: return
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
        // D s_7_1: write-var gs#105415 <= s_7_0
        fn_state.gs_105415 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#105415:u8
        let s_8_0: bool = fn_state.gs_105415;
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
        // D s_9_1: write-var gs#105416 <= s_9_0
        fn_state.gs_105416 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#105416:u8
        let s_10_0: bool = fn_state.gs_105416;
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
        // D s_11_1: write-var gs#105417 <= s_11_0
        fn_state.gs_105417 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#105417:u8
        let s_12_0: bool = fn_state.gs_105417;
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
        // D s_13_1: write-var gs#105418 <= s_13_0
        fn_state.gs_105418 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#105418:u8
        let s_14_0: bool = fn_state.gs_105418;
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
        // D s_15_1: write-var gs#105419 <= s_15_0
        fn_state.gs_105419 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#105419:u8
        let s_16_0: bool = fn_state.gs_105419;
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
        // D s_18_1: write-var gs#105420 <= s_18_0
        fn_state.gs_105420 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#105420:u8
        let s_19_0: bool = fn_state.gs_105420;
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
        // D s_20_1: write-var gs#105421 <= s_20_0
        fn_state.gs_105421 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#105421:u8
        let s_21_0: bool = fn_state.gs_105421;
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
        // S s_22_1: call AMCNTENCLR1_read(s_22_0)
        let s_22_1: ProductType700c18a878c5601b = AMCNTENCLR1_read(
            state,
            tracer,
            s_22_0,
        );
        // S s_22_2: call __get_AMCNTENCLR1(s_22_1)
        let s_22_2: ProductType700c18a878c5601b = u__get_AMCNTENCLR1(
            state,
            tracer,
            s_22_1,
        );
        // D s_22_3: write-var ga#162519 <= s_22_2
        fn_state.ga_162519 = s_22_2;
        // D s_22_4: read-var ga#162519.0:struct
        let s_22_4: u32 = fn_state.ga_162519._0;
        // D s_22_5: read-var t:i
        let s_22_5: i128 = fn_state.t;
        // D s_22_6: call R_set(s_22_5, s_22_4)
        let s_22_6: () = R_set(state, tracer, s_22_5, s_22_4);
        // N s_22_7: return
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
        // D s_24_1: write-var gs#105422 <= s_24_0
        fn_state.gs_105422 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#105422:u8
        let s_25_0: bool = fn_state.gs_105422;
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
        // D s_28_7: write-var gs#105422 <= s_28_6
        fn_state.gs_105422 = s_28_6;
        // N s_28_8: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var __CPTR_EL3_TAM:u8
        let s_29_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 1u16);
        // C s_29_2: const #1u : u8
        let s_29_2: bool = true;
        // C s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // D s_29_4: cmp-eq s_29_1 s_29_3
        let s_29_4: bool = ((s_29_1) == (s_29_3));
        // D s_29_5: write-var gs#105421 <= s_29_4
        fn_state.gs_105421 = s_29_4;
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
        // D s_30_4: write-var gs#105420 <= s_30_3
        fn_state.gs_105420 = s_30_3;
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
        // D s_32_0: read-var __CPTR_EL3_TAM:u8
        let s_32_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 1u16);
        // C s_32_2: const #1u : u8
        let s_32_2: bool = true;
        // C s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 1u16);
        // D s_32_4: cmp-eq s_32_1 s_32_3
        let s_32_4: bool = ((s_32_1) == (s_32_3));
        // D s_32_5: write-var gs#105419 <= s_32_4
        fn_state.gs_105419 = s_32_4;
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
        // D s_33_4: write-var gs#105418 <= s_33_3
        fn_state.gs_105418 = s_33_3;
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
        // D s_34_2: write-var gs#105417 <= s_34_1
        fn_state.gs_105417 = s_34_1;
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
        // D s_35_7: write-var gs#105416 <= s_35_6
        fn_state.gs_105416 = s_35_6;
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
        // D s_36_4: write-var gs#105415 <= s_36_3
        fn_state.gs_105415 = s_36_3;
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
        // D s_38_1: write-var gs#105423 <= s_38_0
        fn_state.gs_105423 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#105423:u8
        let s_39_0: bool = fn_state.gs_105423;
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
        // D s_40_1: write-var gs#105424 <= s_40_0
        fn_state.gs_105424 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#105424:u8
        let s_41_0: bool = fn_state.gs_105424;
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
        // D s_42_1: write-var gs#105425 <= s_42_0
        fn_state.gs_105425 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#105425:u8
        let s_43_0: bool = fn_state.gs_105425;
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
        // D s_44_1: write-var gs#105426 <= s_44_0
        fn_state.gs_105426 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#105426:u8
        let s_45_0: bool = fn_state.gs_105426;
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
        // D s_46_1: write-var gs#105427 <= s_46_0
        fn_state.gs_105427 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#105427:u8
        let s_47_0: bool = fn_state.gs_105427;
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
        // D s_49_1: write-var gs#105428 <= s_49_0
        fn_state.gs_105428 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#105428:u8
        let s_50_0: bool = fn_state.gs_105428;
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
        // D s_51_1: write-var gs#105429 <= s_51_0
        fn_state.gs_105429 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#105429:u8
        let s_52_0: bool = fn_state.gs_105429;
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
        // D s_54_1: write-var gs#105430 <= s_54_0
        fn_state.gs_105430 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#105430:u8
        let s_55_0: bool = fn_state.gs_105430;
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
        // D s_56_1: write-var gs#105431 <= s_56_0
        fn_state.gs_105431 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#105431:u8
        let s_57_0: bool = fn_state.gs_105431;
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
        // D s_59_1: write-var gs#105432 <= s_59_0
        fn_state.gs_105432 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#105432:u8
        let s_60_0: bool = fn_state.gs_105432;
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
        // D s_61_1: write-var gs#105433 <= s_61_0
        fn_state.gs_105433 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#105433:u8
        let s_62_0: bool = fn_state.gs_105433;
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
        // D s_64_1: write-var gs#105434 <= s_64_0
        fn_state.gs_105434 = s_64_0;
        // N s_64_2: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var gs#105434:u8
        let s_65_0: bool = fn_state.gs_105434;
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
        // D s_66_1: write-var gs#105435 <= s_66_0
        fn_state.gs_105435 = s_66_0;
        // N s_66_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#105435:u8
        let s_67_0: bool = fn_state.gs_105435;
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
        // D s_69_1: write-var gs#105436 <= s_69_0
        fn_state.gs_105436 = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var gs#105436:u8
        let s_70_0: bool = fn_state.gs_105436;
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
        // D s_71_1: write-var gs#105437 <= s_71_0
        fn_state.gs_105437 = s_71_0;
        // N s_71_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var gs#105437:u8
        let s_72_0: bool = fn_state.gs_105437;
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
        // S s_73_1: call AMCNTENCLR1_read(s_73_0)
        let s_73_1: ProductType700c18a878c5601b = AMCNTENCLR1_read(
            state,
            tracer,
            s_73_0,
        );
        // S s_73_2: call __get_AMCNTENCLR1(s_73_1)
        let s_73_2: ProductType700c18a878c5601b = u__get_AMCNTENCLR1(
            state,
            tracer,
            s_73_1,
        );
        // D s_73_3: write-var ga#162495 <= s_73_2
        fn_state.ga_162495 = s_73_2;
        // D s_73_4: read-var ga#162495.0:struct
        let s_73_4: u32 = fn_state.ga_162495._0;
        // D s_73_5: read-var t:i
        let s_73_5: i128 = fn_state.t;
        // D s_73_6: call R_set(s_73_5, s_73_4)
        let s_73_6: () = R_set(state, tracer, s_73_5, s_73_4);
        // N s_73_7: return
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
        // D s_75_1: write-var gs#105438 <= s_75_0
        fn_state.gs_105438 = s_75_0;
        // N s_75_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var gs#105438:u8
        let s_76_0: bool = fn_state.gs_105438;
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
        // D s_79_7: write-var gs#105438 <= s_79_6
        fn_state.gs_105438 = s_79_6;
        // N s_79_8: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var __CPTR_EL3_TAM:u8
        let s_80_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_80_1: cast zx s_80_0 -> bv
        let s_80_1: Bits = Bits::new(s_80_0 as u128, 1u16);
        // C s_80_2: const #1u : u8
        let s_80_2: bool = true;
        // C s_80_3: cast zx s_80_2 -> bv
        let s_80_3: Bits = Bits::new(s_80_2 as u128, 1u16);
        // D s_80_4: cmp-eq s_80_1 s_80_3
        let s_80_4: bool = ((s_80_1) == (s_80_3));
        // D s_80_5: write-var gs#105437 <= s_80_4
        fn_state.gs_105437 = s_80_4;
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
        // D s_81_4: write-var gs#105436 <= s_81_3
        fn_state.gs_105436 = s_81_3;
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
        // D s_83_0: read-var __HCPTR_TAM:u8
        let s_83_0: bool = fn_state.u__HCPTR_TAM;
        // D s_83_1: cast zx s_83_0 -> bv
        let s_83_1: Bits = Bits::new(s_83_0 as u128, 1u16);
        // C s_83_2: const #1u : u8
        let s_83_2: bool = true;
        // C s_83_3: cast zx s_83_2 -> bv
        let s_83_3: Bits = Bits::new(s_83_2 as u128, 1u16);
        // D s_83_4: cmp-eq s_83_1 s_83_3
        let s_83_4: bool = ((s_83_1) == (s_83_3));
        // D s_83_5: write-var gs#105435 <= s_83_4
        fn_state.gs_105435 = s_83_4;
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
        // D s_84_3: write-var gs#105434 <= s_84_2
        fn_state.gs_105434 = s_84_2;
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
        // D s_86_0: read-var __CPTR_EL2_TAM:u8
        let s_86_0: bool = fn_state.u__CPTR_EL2_TAM;
        // D s_86_1: cast zx s_86_0 -> bv
        let s_86_1: Bits = Bits::new(s_86_0 as u128, 1u16);
        // C s_86_2: const #1u : u8
        let s_86_2: bool = true;
        // C s_86_3: cast zx s_86_2 -> bv
        let s_86_3: Bits = Bits::new(s_86_2 as u128, 1u16);
        // D s_86_4: cmp-eq s_86_1 s_86_3
        let s_86_4: bool = ((s_86_1) == (s_86_3));
        // D s_86_5: write-var gs#105433 <= s_86_4
        fn_state.gs_105433 = s_86_4;
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
        // D s_87_4: write-var gs#105432 <= s_87_3
        fn_state.gs_105432 = s_87_3;
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
        // D s_89_0: read-var __HSTR_T13:u8
        let s_89_0: bool = fn_state.u__HSTR_T13;
        // D s_89_1: cast zx s_89_0 -> bv
        let s_89_1: Bits = Bits::new(s_89_0 as u128, 1u16);
        // C s_89_2: const #1u : u8
        let s_89_2: bool = true;
        // C s_89_3: cast zx s_89_2 -> bv
        let s_89_3: Bits = Bits::new(s_89_2 as u128, 1u16);
        // D s_89_4: cmp-eq s_89_1 s_89_3
        let s_89_4: bool = ((s_89_1) == (s_89_3));
        // D s_89_5: write-var gs#105431 <= s_89_4
        fn_state.gs_105431 = s_89_4;
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
        // D s_90_3: write-var gs#105430 <= s_90_2
        fn_state.gs_105430 = s_90_2;
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
        // D s_92_0: read-var __HSTR_EL2_T13:u8
        let s_92_0: bool = fn_state.u__HSTR_EL2_T13;
        // D s_92_1: cast zx s_92_0 -> bv
        let s_92_1: Bits = Bits::new(s_92_0 as u128, 1u16);
        // C s_92_2: const #1u : u8
        let s_92_2: bool = true;
        // C s_92_3: cast zx s_92_2 -> bv
        let s_92_3: Bits = Bits::new(s_92_2 as u128, 1u16);
        // D s_92_4: cmp-eq s_92_1 s_92_3
        let s_92_4: bool = ((s_92_1) == (s_92_3));
        // D s_92_5: write-var gs#105429 <= s_92_4
        fn_state.gs_105429 = s_92_4;
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
        // D s_93_4: write-var gs#105428 <= s_93_3
        fn_state.gs_105428 = s_93_3;
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
        // D s_95_0: read-var __CPTR_EL3_TAM:u8
        let s_95_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_95_1: cast zx s_95_0 -> bv
        let s_95_1: Bits = Bits::new(s_95_0 as u128, 1u16);
        // C s_95_2: const #1u : u8
        let s_95_2: bool = true;
        // C s_95_3: cast zx s_95_2 -> bv
        let s_95_3: Bits = Bits::new(s_95_2 as u128, 1u16);
        // D s_95_4: cmp-eq s_95_1 s_95_3
        let s_95_4: bool = ((s_95_1) == (s_95_3));
        // D s_95_5: write-var gs#105427 <= s_95_4
        fn_state.gs_105427 = s_95_4;
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
        // D s_96_4: write-var gs#105426 <= s_96_3
        fn_state.gs_105426 = s_96_3;
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
        // D s_97_2: write-var gs#105425 <= s_97_1
        fn_state.gs_105425 = s_97_1;
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
        // D s_98_7: write-var gs#105424 <= s_98_6
        fn_state.gs_105424 = s_98_6;
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
        // D s_99_4: write-var gs#105423 <= s_99_3
        fn_state.gs_105423 = s_99_3;
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
        // N s_100_2: branch s_100_1 b219 b101
        if s_100_1 {
            return block_219(state, tracer, fn_state);
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
        // D s_101_1: write-var gs#105439 <= s_101_0
        fn_state.gs_105439 = s_101_0;
        // N s_101_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var gs#105439:u8
        let s_102_0: bool = fn_state.gs_105439;
        // N s_102_1: branch s_102_0 b218 b103
        if s_102_0 {
            return block_218(state, tracer, fn_state);
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
        // D s_103_1: write-var gs#105440 <= s_103_0
        fn_state.gs_105440 = s_103_0;
        // N s_103_2: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_104_0: read-var gs#105440:u8
        let s_104_0: bool = fn_state.gs_105440;
        // N s_104_1: branch s_104_0 b217 b105
        if s_104_0 {
            return block_217(state, tracer, fn_state);
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
        // D s_105_1: write-var gs#105441 <= s_105_0
        fn_state.gs_105441 = s_105_0;
        // N s_105_2: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var gs#105441:u8
        let s_106_0: bool = fn_state.gs_105441;
        // N s_106_1: branch s_106_0 b216 b107
        if s_106_0 {
            return block_216(state, tracer, fn_state);
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
        // D s_107_1: write-var gs#105442 <= s_107_0
        fn_state.gs_105442 = s_107_0;
        // N s_107_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var gs#105442:u8
        let s_108_0: bool = fn_state.gs_105442;
        // N s_108_1: branch s_108_0 b215 b109
        if s_108_0 {
            return block_215(state, tracer, fn_state);
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
        // D s_109_1: write-var gs#105443 <= s_109_0
        fn_state.gs_105443 = s_109_0;
        // N s_109_2: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_110_0: read-var gs#105443:u8
        let s_110_0: bool = fn_state.gs_105443;
        // N s_110_1: branch s_110_0 b214 b111
        if s_110_0 {
            return block_214(state, tracer, fn_state);
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
        // N s_111_4: branch s_111_3 b213 b112
        if s_111_3 {
            return block_213(state, tracer, fn_state);
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
        // D s_112_1: write-var gs#105444 <= s_112_0
        fn_state.gs_105444 = s_112_0;
        // N s_112_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var gs#105444:u8
        let s_113_0: bool = fn_state.gs_105444;
        // N s_113_1: branch s_113_0 b204 b114
        if s_113_0 {
            return block_204(state, tracer, fn_state);
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
        // N s_114_3: branch s_114_2 b203 b115
        if s_114_2 {
            return block_203(state, tracer, fn_state);
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
        // D s_115_1: write-var gs#105445 <= s_115_0
        fn_state.gs_105445 = s_115_0;
        // N s_115_2: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_116_0: read-var gs#105445:u8
        let s_116_0: bool = fn_state.gs_105445;
        // N s_116_1: branch s_116_0 b186 b117
        if s_116_0 {
            return block_186(state, tracer, fn_state);
        } else {
            return block_117(state, tracer, fn_state);
        };
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #() : ()
        let s_117_0: () = ();
        // S s_117_1: call EL2Enabled(s_117_0)
        let s_117_1: bool = EL2Enabled(state, tracer, s_117_0);
        // N s_117_2: branch s_117_1 b185 b118
        if s_117_1 {
            return block_185(state, tracer, fn_state);
        } else {
            return block_118(state, tracer, fn_state);
        };
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #0u : u8
        let s_118_0: bool = false;
        // D s_118_1: write-var gs#105446 <= s_118_0
        fn_state.gs_105446 = s_118_0;
        // N s_118_2: jump b119
        return block_119(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_119_0: read-var gs#105446:u8
        let s_119_0: bool = fn_state.gs_105446;
        // N s_119_1: branch s_119_0 b184 b120
        if s_119_0 {
            return block_184(state, tracer, fn_state);
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
        // D s_120_1: write-var gs#105447 <= s_120_0
        fn_state.gs_105447 = s_120_0;
        // N s_120_2: jump b121
        return block_121(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_121_0: read-var gs#105447:u8
        let s_121_0: bool = fn_state.gs_105447;
        // N s_121_1: branch s_121_0 b183 b122
        if s_121_0 {
            return block_183(state, tracer, fn_state);
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
        // D s_122_1: write-var gs#105448 <= s_122_0
        fn_state.gs_105448 = s_122_0;
        // N s_122_2: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_123_0: read-var gs#105448:u8
        let s_123_0: bool = fn_state.gs_105448;
        // N s_123_1: branch s_123_0 b182 b124
        if s_123_0 {
            return block_182(state, tracer, fn_state);
        } else {
            return block_124(state, tracer, fn_state);
        };
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #() : ()
        let s_124_0: () = ();
        // S s_124_1: call EL2Enabled(s_124_0)
        let s_124_1: bool = EL2Enabled(state, tracer, s_124_0);
        // N s_124_2: branch s_124_1 b181 b125
        if s_124_1 {
            return block_181(state, tracer, fn_state);
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
        // D s_125_1: write-var gs#105449 <= s_125_0
        fn_state.gs_105449 = s_125_0;
        // N s_125_2: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_126_0: read-var gs#105449:u8
        let s_126_0: bool = fn_state.gs_105449;
        // N s_126_1: branch s_126_0 b180 b127
        if s_126_0 {
            return block_180(state, tracer, fn_state);
        } else {
            return block_127(state, tracer, fn_state);
        };
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #0u : u8
        let s_127_0: bool = false;
        // D s_127_1: write-var gs#105450 <= s_127_0
        fn_state.gs_105450 = s_127_0;
        // N s_127_2: jump b128
        return block_128(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_128_0: read-var gs#105450:u8
        let s_128_0: bool = fn_state.gs_105450;
        // N s_128_1: branch s_128_0 b179 b129
        if s_128_0 {
            return block_179(state, tracer, fn_state);
        } else {
            return block_129(state, tracer, fn_state);
        };
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_129_0: const #() : ()
        let s_129_0: () = ();
        // S s_129_1: call EL2Enabled(s_129_0)
        let s_129_1: bool = EL2Enabled(state, tracer, s_129_0);
        // N s_129_2: branch s_129_1 b178 b130
        if s_129_1 {
            return block_178(state, tracer, fn_state);
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
        // D s_130_1: write-var gs#105451 <= s_130_0
        fn_state.gs_105451 = s_130_0;
        // N s_130_2: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_131_0: read-var gs#105451:u8
        let s_131_0: bool = fn_state.gs_105451;
        // N s_131_1: branch s_131_0 b177 b132
        if s_131_0 {
            return block_177(state, tracer, fn_state);
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
        // D s_132_1: write-var gs#105452 <= s_132_0
        fn_state.gs_105452 = s_132_0;
        // N s_132_2: jump b133
        return block_133(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_133_0: read-var gs#105452:u8
        let s_133_0: bool = fn_state.gs_105452;
        // N s_133_1: branch s_133_0 b176 b134
        if s_133_0 {
            return block_176(state, tracer, fn_state);
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
        // N s_134_2: branch s_134_1 b175 b135
        if s_134_1 {
            return block_175(state, tracer, fn_state);
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
        // D s_135_1: write-var gs#105453 <= s_135_0
        fn_state.gs_105453 = s_135_0;
        // N s_135_2: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_136_0: read-var gs#105453:u8
        let s_136_0: bool = fn_state.gs_105453;
        // N s_136_1: branch s_136_0 b174 b137
        if s_136_0 {
            return block_174(state, tracer, fn_state);
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
        // D s_137_1: write-var gs#105454 <= s_137_0
        fn_state.gs_105454 = s_137_0;
        // N s_137_2: jump b138
        return block_138(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_138_0: read-var gs#105454:u8
        let s_138_0: bool = fn_state.gs_105454;
        // N s_138_1: branch s_138_0 b173 b139
        if s_138_0 {
            return block_173(state, tracer, fn_state);
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
        // N s_139_2: branch s_139_1 b172 b140
        if s_139_1 {
            return block_172(state, tracer, fn_state);
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
        // D s_140_1: write-var gs#105455 <= s_140_0
        fn_state.gs_105455 = s_140_0;
        // N s_140_2: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_141_0: read-var gs#105455:u8
        let s_141_0: bool = fn_state.gs_105455;
        // N s_141_1: branch s_141_0 b171 b142
        if s_141_0 {
            return block_171(state, tracer, fn_state);
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
        // D s_142_1: write-var gs#105456 <= s_142_0
        fn_state.gs_105456 = s_142_0;
        // N s_142_2: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_143_0: read-var gs#105456:u8
        let s_143_0: bool = fn_state.gs_105456;
        // N s_143_1: branch s_143_0 b170 b144
        if s_143_0 {
            return block_170(state, tracer, fn_state);
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
        // D s_144_1: write-var gs#105457 <= s_144_0
        fn_state.gs_105457 = s_144_0;
        // N s_144_2: jump b145
        return block_145(state, tracer, fn_state);
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_145_0: read-var gs#105457:u8
        let s_145_0: bool = fn_state.gs_105457;
        // N s_145_1: branch s_145_0 b166 b146
        if s_145_0 {
            return block_166(state, tracer, fn_state);
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
        // D s_146_1: write-var gs#105459 <= s_146_0
        fn_state.gs_105459 = s_146_0;
        // N s_146_2: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_147_0: read-var gs#105459:u8
        let s_147_0: bool = fn_state.gs_105459;
        // N s_147_1: branch s_147_0 b165 b148
        if s_147_0 {
            return block_165(state, tracer, fn_state);
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
        // D s_148_1: write-var gs#105460 <= s_148_0
        fn_state.gs_105460 = s_148_0;
        // N s_148_2: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_149_0: read-var gs#105460:u8
        let s_149_0: bool = fn_state.gs_105460;
        // N s_149_1: branch s_149_0 b164 b150
        if s_149_0 {
            return block_164(state, tracer, fn_state);
        } else {
            return block_150(state, tracer, fn_state);
        };
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_150_0: const #424u : u32
        let s_150_0: u32 = 424;
        // D s_150_1: read-reg s_150_0:u8
        let s_150_1: u8 = {
            let value = state.read_register::<u8>(s_150_0 as isize);
            tracer.read_register(s_150_0 as isize, value);
            value
        };
        // C s_150_2: const #2u : u8
        let s_150_2: u8 = 2;
        // D s_150_3: cmp-lt s_150_1 s_150_2
        let s_150_3: bool = ((s_150_1) < (s_150_2));
        // N s_150_4: branch s_150_3 b163 b151
        if s_150_3 {
            return block_163(state, tracer, fn_state);
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
        // D s_151_1: write-var gs#105461 <= s_151_0
        fn_state.gs_105461 = s_151_0;
        // N s_151_2: jump b152
        return block_152(state, tracer, fn_state);
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_152_0: read-var gs#105461:u8
        let s_152_0: bool = fn_state.gs_105461;
        // N s_152_1: branch s_152_0 b162 b153
        if s_152_0 {
            return block_162(state, tracer, fn_state);
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
        // D s_153_1: write-var gs#105462 <= s_153_0
        fn_state.gs_105462 = s_153_0;
        // N s_153_2: jump b154
        return block_154(state, tracer, fn_state);
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_154_0: read-var gs#105462:u8
        let s_154_0: bool = fn_state.gs_105462;
        // N s_154_1: branch s_154_0 b156 b155
        if s_154_0 {
            return block_156(state, tracer, fn_state);
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
        // S s_155_1: call AMCNTENCLR1_read(s_155_0)
        let s_155_1: ProductType700c18a878c5601b = AMCNTENCLR1_read(
            state,
            tracer,
            s_155_0,
        );
        // S s_155_2: call __get_AMCNTENCLR1(s_155_1)
        let s_155_2: ProductType700c18a878c5601b = u__get_AMCNTENCLR1(
            state,
            tracer,
            s_155_1,
        );
        // D s_155_3: write-var ga#162453 <= s_155_2
        fn_state.ga_162453 = s_155_2;
        // D s_155_4: read-var ga#162453.0:struct
        let s_155_4: u32 = fn_state.ga_162453._0;
        // D s_155_5: read-var t:i
        let s_155_5: i128 = fn_state.t;
        // D s_155_6: call R_set(s_155_5, s_155_4)
        let s_155_6: () = R_set(state, tracer, s_155_5, s_155_4);
        // N s_155_7: return
        return;
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_156_0: const #() : ()
        let s_156_0: () = ();
        // S s_156_1: call Halted(s_156_0)
        let s_156_1: bool = Halted(state, tracer, s_156_0);
        // N s_156_2: branch s_156_1 b161 b157
        if s_156_1 {
            return block_161(state, tracer, fn_state);
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
        // D s_157_1: write-var gs#105463 <= s_157_0
        fn_state.gs_105463 = s_157_0;
        // N s_157_2: jump b158
        return block_158(state, tracer, fn_state);
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_158_0: read-var gs#105463:u8
        let s_158_0: bool = fn_state.gs_105463;
        // N s_158_1: branch s_158_0 b160 b159
        if s_158_0 {
            return block_160(state, tracer, fn_state);
        } else {
            return block_159(state, tracer, fn_state);
        };
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_159_0: const #3u : u8
        let s_159_0: u8 = 3;
        // C s_159_1: cast zx s_159_0 -> bv
        let s_159_1: Bits = Bits::new(s_159_0 as u128, 8u16);
        // C s_159_2: cast zx s_159_1 -> i
        let s_159_2: i128 = (s_159_1.value() as i128);
        // C s_159_3: cast reint s_159_2 -> i64
        let s_159_3: i64 = (s_159_2 as i64);
        // C s_159_4: cast zx s_159_3 -> i
        let s_159_4: i128 = (i128::try_from(s_159_3).unwrap());
        // C s_159_5: const #424u : u32
        let s_159_5: u32 = 424;
        // D s_159_6: read-reg s_159_5:u8
        let s_159_6: u8 = {
            let value = state.read_register::<u8>(s_159_5 as isize);
            tracer.read_register(s_159_5 as isize, value);
            value
        };
        // D s_159_7: call AArch64_AArch32SystemAccessTrap(s_159_6, s_159_4)
        let s_159_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_159_6,
            s_159_4,
        );
        // N s_159_8: return
        return;
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_160_0: panic
        panic!("{:?}", ());
        // N s_160_1: return
        return;
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_161_0: const #() : ()
        let s_161_0: () = ();
        // S s_161_1: call EDSCR_read(s_161_0)
        let s_161_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_161_0);
        // S s_161_2: call _get_EDSCR_Type_SDD(s_161_1)
        let s_161_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_161_1);
        // S s_161_3: cast zx s_161_2 -> bv
        let s_161_3: Bits = Bits::new(s_161_2 as u128, 1u16);
        // C s_161_4: const #1u : u8
        let s_161_4: bool = true;
        // C s_161_5: cast zx s_161_4 -> bv
        let s_161_5: Bits = Bits::new(s_161_4 as u128, 1u16);
        // S s_161_6: cmp-eq s_161_3 s_161_5
        let s_161_6: bool = ((s_161_3) == (s_161_5));
        // D s_161_7: write-var gs#105463 <= s_161_6
        fn_state.gs_105463 = s_161_6;
        // N s_161_8: jump b158
        return block_158(state, tracer, fn_state);
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_162_0: read-var __CPTR_EL3_TAM:u8
        let s_162_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_162_1: cast zx s_162_0 -> bv
        let s_162_1: Bits = Bits::new(s_162_0 as u128, 1u16);
        // C s_162_2: const #1u : u8
        let s_162_2: bool = true;
        // C s_162_3: cast zx s_162_2 -> bv
        let s_162_3: Bits = Bits::new(s_162_2 as u128, 1u16);
        // D s_162_4: cmp-eq s_162_1 s_162_3
        let s_162_4: bool = ((s_162_1) == (s_162_3));
        // D s_162_5: write-var gs#105462 <= s_162_4
        fn_state.gs_105462 = s_162_4;
        // N s_162_6: jump b154
        return block_154(state, tracer, fn_state);
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
        // D s_163_2: call ELUsingAArch32(s_163_1)
        let s_163_2: bool = ELUsingAArch32(state, tracer, s_163_1);
        // D s_163_3: not s_163_2
        let s_163_3: bool = !s_163_2;
        // D s_163_4: write-var gs#105461 <= s_163_3
        fn_state.gs_105461 = s_163_3;
        // N s_163_5: jump b152
        return block_152(state, tracer, fn_state);
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_164_0: const #3u : u8
        let s_164_0: u8 = 3;
        // C s_164_1: cast zx s_164_0 -> bv
        let s_164_1: Bits = Bits::new(s_164_0 as u128, 8u16);
        // C s_164_2: cast zx s_164_1 -> i
        let s_164_2: i128 = (s_164_1.value() as i128);
        // C s_164_3: cast reint s_164_2 -> i64
        let s_164_3: i64 = (s_164_2 as i64);
        // C s_164_4: cast zx s_164_3 -> i
        let s_164_4: i128 = (i128::try_from(s_164_3).unwrap());
        // C s_164_5: const #432u : u32
        let s_164_5: u32 = 432;
        // D s_164_6: read-reg s_164_5:u8
        let s_164_6: u8 = {
            let value = state.read_register::<u8>(s_164_5 as isize);
            tracer.read_register(s_164_5 as isize, value);
            value
        };
        // D s_164_7: call AArch64_AArch32SystemAccessTrap(s_164_6, s_164_4)
        let s_164_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_164_6,
            s_164_4,
        );
        // N s_164_8: return
        return;
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_165_0: read-var __HAFGRTR_EL2_AMCNTEN1:u8
        let s_165_0: bool = fn_state.u__HAFGRTR_EL2_AMCNTEN1;
        // D s_165_1: cast zx s_165_0 -> bv
        let s_165_1: Bits = Bits::new(s_165_0 as u128, 1u16);
        // C s_165_2: const #1u : u8
        let s_165_2: bool = true;
        // C s_165_3: cast zx s_165_2 -> bv
        let s_165_3: Bits = Bits::new(s_165_2 as u128, 1u16);
        // D s_165_4: cmp-eq s_165_1 s_165_3
        let s_165_4: bool = ((s_165_1) == (s_165_3));
        // D s_165_5: write-var gs#105460 <= s_165_4
        fn_state.gs_105460 = s_165_4;
        // N s_165_6: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_166_0: const #424u : u32
        let s_166_0: u32 = 424;
        // D s_166_1: read-reg s_166_0:u8
        let s_166_1: u8 = {
            let value = state.read_register::<u8>(s_166_0 as isize);
            tracer.read_register(s_166_0 as isize, value);
            value
        };
        // C s_166_2: const #2u : u8
        let s_166_2: u8 = 2;
        // D s_166_3: cmp-lt s_166_1 s_166_2
        let s_166_3: bool = ((s_166_1) < (s_166_2));
        // D s_166_4: not s_166_3
        let s_166_4: bool = !s_166_3;
        // N s_166_5: branch s_166_4 b169 b167
        if s_166_4 {
            return block_169(state, tracer, fn_state);
        } else {
            return block_167(state, tracer, fn_state);
        };
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_167_0: read-var __SCR_EL3_FGTEn:u8
        let s_167_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_167_1: cast zx s_167_0 -> bv
        let s_167_1: Bits = Bits::new(s_167_0 as u128, 1u16);
        // C s_167_2: const #1u : u8
        let s_167_2: bool = true;
        // C s_167_3: cast zx s_167_2 -> bv
        let s_167_3: Bits = Bits::new(s_167_2 as u128, 1u16);
        // D s_167_4: cmp-eq s_167_1 s_167_3
        let s_167_4: bool = ((s_167_1) == (s_167_3));
        // D s_167_5: write-var gs#105458 <= s_167_4
        fn_state.gs_105458 = s_167_4;
        // N s_167_6: jump b168
        return block_168(state, tracer, fn_state);
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_168_0: read-var gs#105458:u8
        let s_168_0: bool = fn_state.gs_105458;
        // D s_168_1: write-var gs#105459 <= s_168_0
        fn_state.gs_105459 = s_168_0;
        // N s_168_2: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_169_0: const #1u : u8
        let s_169_0: bool = true;
        // D s_169_1: write-var gs#105458 <= s_169_0
        fn_state.gs_105458 = s_169_0;
        // N s_169_2: jump b168
        return block_168(state, tracer, fn_state);
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_170_0: const #146u : u32
        let s_170_0: u32 = 146;
        // S s_170_1: call IsFeatureImplemented(s_170_0)
        let s_170_1: bool = IsFeatureImplemented(state, tracer, s_170_0);
        // D s_170_2: write-var gs#105457 <= s_170_1
        fn_state.gs_105457 = s_170_1;
        // N s_170_3: jump b145
        return block_145(state, tracer, fn_state);
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_171_0: const #102552u : u32
        let s_171_0: u32 = 102552;
        // D s_171_1: read-reg s_171_0:struct
        let s_171_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_171_0 as isize);
            tracer.read_register(s_171_0 as isize, value);
            value
        };
        // D s_171_2: call _get_HCR_EL2_Type_E2H(s_171_1)
        let s_171_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_171_1);
        // C s_171_3: const #102552u : u32
        let s_171_3: u32 = 102552;
        // D s_171_4: read-reg s_171_3:struct
        let s_171_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_171_3 as isize);
            tracer.read_register(s_171_3 as isize, value);
            value
        };
        // D s_171_5: call _get_HCR_EL2_Type_TGE(s_171_4)
        let s_171_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_171_4);
        // D s_171_6: cast zx s_171_2 -> bv
        let s_171_6: Bits = Bits::new(s_171_2 as u128, 1u16);
        // D s_171_7: cast zx s_171_5 -> bv
        let s_171_7: Bits = Bits::new(s_171_5 as u128, 1u16);
        // D s_171_8: cast reint s_171_6 -> u128
        let s_171_8: u128 = (s_171_6.value() as u128);
        // D s_171_9: size-of s_171_6
        let s_171_9: u16 = s_171_6.length();
        // D s_171_10: cast reint s_171_7 -> u128
        let s_171_10: u128 = (s_171_7.value() as u128);
        // D s_171_11: size-of s_171_7
        let s_171_11: u16 = s_171_7.length();
        // D s_171_12: lsl s_171_8 s_171_11
        let s_171_12: u128 = s_171_8 << s_171_11;
        // D s_171_13: or s_171_12 s_171_10
        let s_171_13: u128 = ((s_171_12) | (s_171_10));
        // D s_171_14: add s_171_9 s_171_11
        let s_171_14: u16 = (s_171_9 + s_171_11);
        // D s_171_15: create-bits s_171_13 s_171_14
        let s_171_15: Bits = Bits::new(s_171_13, s_171_14);
        // D s_171_16: cast reint s_171_15 -> u8
        let s_171_16: u8 = (s_171_15.value() as u8);
        // D s_171_17: cast zx s_171_16 -> bv
        let s_171_17: Bits = Bits::new(s_171_16 as u128, 2u16);
        // C s_171_18: const #3u : u8
        let s_171_18: u8 = 3;
        // C s_171_19: cast zx s_171_18 -> bv
        let s_171_19: Bits = Bits::new(s_171_18 as u128, 2u16);
        // D s_171_20: cmp-ne s_171_17 s_171_19
        let s_171_20: bool = ((s_171_17) != (s_171_19));
        // D s_171_21: write-var gs#105456 <= s_171_20
        fn_state.gs_105456 = s_171_20;
        // N s_171_22: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_172_0: const #440u : u32
        let s_172_0: u32 = 440;
        // D s_172_1: read-reg s_172_0:u8
        let s_172_1: u8 = {
            let value = state.read_register::<u8>(s_172_0 as isize);
            tracer.read_register(s_172_0 as isize, value);
            value
        };
        // D s_172_2: call ELUsingAArch32(s_172_1)
        let s_172_2: bool = ELUsingAArch32(state, tracer, s_172_1);
        // D s_172_3: not s_172_2
        let s_172_3: bool = !s_172_2;
        // D s_172_4: write-var gs#105455 <= s_172_3
        fn_state.gs_105455 = s_172_3;
        // N s_172_5: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_173_0: const #3u : u8
        let s_173_0: u8 = 3;
        // C s_173_1: cast zx s_173_0 -> bv
        let s_173_1: Bits = Bits::new(s_173_0 as u128, 8u16);
        // C s_173_2: cast zx s_173_1 -> i
        let s_173_2: i128 = (s_173_1.value() as i128);
        // C s_173_3: cast reint s_173_2 -> i64
        let s_173_3: i64 = (s_173_2 as i64);
        // C s_173_4: cast zx s_173_3 -> i
        let s_173_4: i128 = (i128::try_from(s_173_3).unwrap());
        // S s_173_5: call AArch32_TakeHypTrapException(s_173_4)
        let s_173_5: () = AArch32_TakeHypTrapException(state, tracer, s_173_4);
        // N s_173_6: return
        return;
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_174_0: read-var __HCPTR_TAM:u8
        let s_174_0: bool = fn_state.u__HCPTR_TAM;
        // D s_174_1: cast zx s_174_0 -> bv
        let s_174_1: Bits = Bits::new(s_174_0 as u128, 1u16);
        // C s_174_2: const #1u : u8
        let s_174_2: bool = true;
        // C s_174_3: cast zx s_174_2 -> bv
        let s_174_3: Bits = Bits::new(s_174_2 as u128, 1u16);
        // D s_174_4: cmp-eq s_174_1 s_174_3
        let s_174_4: bool = ((s_174_1) == (s_174_3));
        // D s_174_5: write-var gs#105454 <= s_174_4
        fn_state.gs_105454 = s_174_4;
        // N s_174_6: jump b138
        return block_138(state, tracer, fn_state);
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_175_0: const #432u : u32
        let s_175_0: u32 = 432;
        // D s_175_1: read-reg s_175_0:u8
        let s_175_1: u8 = {
            let value = state.read_register::<u8>(s_175_0 as isize);
            tracer.read_register(s_175_0 as isize, value);
            value
        };
        // D s_175_2: call ELUsingAArch32(s_175_1)
        let s_175_2: bool = ELUsingAArch32(state, tracer, s_175_1);
        // D s_175_3: write-var gs#105453 <= s_175_2
        fn_state.gs_105453 = s_175_2;
        // N s_175_4: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_176_0: const #3u : u8
        let s_176_0: u8 = 3;
        // C s_176_1: cast zx s_176_0 -> bv
        let s_176_1: Bits = Bits::new(s_176_0 as u128, 8u16);
        // C s_176_2: cast zx s_176_1 -> i
        let s_176_2: i128 = (s_176_1.value() as i128);
        // C s_176_3: cast reint s_176_2 -> i64
        let s_176_3: i64 = (s_176_2 as i64);
        // C s_176_4: cast zx s_176_3 -> i
        let s_176_4: i128 = (i128::try_from(s_176_3).unwrap());
        // C s_176_5: const #432u : u32
        let s_176_5: u32 = 432;
        // D s_176_6: read-reg s_176_5:u8
        let s_176_6: u8 = {
            let value = state.read_register::<u8>(s_176_5 as isize);
            tracer.read_register(s_176_5 as isize, value);
            value
        };
        // D s_176_7: call AArch64_AArch32SystemAccessTrap(s_176_6, s_176_4)
        let s_176_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_176_6,
            s_176_4,
        );
        // N s_176_8: return
        return;
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_177_0: read-var __CPTR_EL2_TAM:u8
        let s_177_0: bool = fn_state.u__CPTR_EL2_TAM;
        // D s_177_1: cast zx s_177_0 -> bv
        let s_177_1: Bits = Bits::new(s_177_0 as u128, 1u16);
        // C s_177_2: const #1u : u8
        let s_177_2: bool = true;
        // C s_177_3: cast zx s_177_2 -> bv
        let s_177_3: Bits = Bits::new(s_177_2 as u128, 1u16);
        // D s_177_4: cmp-eq s_177_1 s_177_3
        let s_177_4: bool = ((s_177_1) == (s_177_3));
        // D s_177_5: write-var gs#105452 <= s_177_4
        fn_state.gs_105452 = s_177_4;
        // N s_177_6: jump b133
        return block_133(state, tracer, fn_state);
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_178_0: const #432u : u32
        let s_178_0: u32 = 432;
        // D s_178_1: read-reg s_178_0:u8
        let s_178_1: u8 = {
            let value = state.read_register::<u8>(s_178_0 as isize);
            tracer.read_register(s_178_0 as isize, value);
            value
        };
        // D s_178_2: call ELUsingAArch32(s_178_1)
        let s_178_2: bool = ELUsingAArch32(state, tracer, s_178_1);
        // D s_178_3: not s_178_2
        let s_178_3: bool = !s_178_2;
        // D s_178_4: write-var gs#105451 <= s_178_3
        fn_state.gs_105451 = s_178_3;
        // N s_178_5: jump b131
        return block_131(state, tracer, fn_state);
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
        // S s_179_5: call AArch32_TakeHypTrapException(s_179_4)
        let s_179_5: () = AArch32_TakeHypTrapException(state, tracer, s_179_4);
        // N s_179_6: return
        return;
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_180_0: read-var __HSTR_T13:u8
        let s_180_0: bool = fn_state.u__HSTR_T13;
        // D s_180_1: cast zx s_180_0 -> bv
        let s_180_1: Bits = Bits::new(s_180_0 as u128, 1u16);
        // C s_180_2: const #1u : u8
        let s_180_2: bool = true;
        // C s_180_3: cast zx s_180_2 -> bv
        let s_180_3: Bits = Bits::new(s_180_2 as u128, 1u16);
        // D s_180_4: cmp-eq s_180_1 s_180_3
        let s_180_4: bool = ((s_180_1) == (s_180_3));
        // D s_180_5: write-var gs#105450 <= s_180_4
        fn_state.gs_105450 = s_180_4;
        // N s_180_6: jump b128
        return block_128(state, tracer, fn_state);
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_181_0: const #432u : u32
        let s_181_0: u32 = 432;
        // D s_181_1: read-reg s_181_0:u8
        let s_181_1: u8 = {
            let value = state.read_register::<u8>(s_181_0 as isize);
            tracer.read_register(s_181_0 as isize, value);
            value
        };
        // D s_181_2: call ELUsingAArch32(s_181_1)
        let s_181_2: bool = ELUsingAArch32(state, tracer, s_181_1);
        // D s_181_3: write-var gs#105449 <= s_181_2
        fn_state.gs_105449 = s_181_2;
        // N s_181_4: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_182_0: const #3u : u8
        let s_182_0: u8 = 3;
        // C s_182_1: cast zx s_182_0 -> bv
        let s_182_1: Bits = Bits::new(s_182_0 as u128, 8u16);
        // C s_182_2: cast zx s_182_1 -> i
        let s_182_2: i128 = (s_182_1.value() as i128);
        // C s_182_3: cast reint s_182_2 -> i64
        let s_182_3: i64 = (s_182_2 as i64);
        // C s_182_4: cast zx s_182_3 -> i
        let s_182_4: i128 = (i128::try_from(s_182_3).unwrap());
        // C s_182_5: const #432u : u32
        let s_182_5: u32 = 432;
        // D s_182_6: read-reg s_182_5:u8
        let s_182_6: u8 = {
            let value = state.read_register::<u8>(s_182_5 as isize);
            tracer.read_register(s_182_5 as isize, value);
            value
        };
        // D s_182_7: call AArch64_AArch32SystemAccessTrap(s_182_6, s_182_4)
        let s_182_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_182_6,
            s_182_4,
        );
        // N s_182_8: return
        return;
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_183_0: read-var __HSTR_EL2_T13:u8
        let s_183_0: bool = fn_state.u__HSTR_EL2_T13;
        // D s_183_1: cast zx s_183_0 -> bv
        let s_183_1: Bits = Bits::new(s_183_0 as u128, 1u16);
        // C s_183_2: const #1u : u8
        let s_183_2: bool = true;
        // C s_183_3: cast zx s_183_2 -> bv
        let s_183_3: Bits = Bits::new(s_183_2 as u128, 1u16);
        // D s_183_4: cmp-eq s_183_1 s_183_3
        let s_183_4: bool = ((s_183_1) == (s_183_3));
        // D s_183_5: write-var gs#105448 <= s_183_4
        fn_state.gs_105448 = s_183_4;
        // N s_183_6: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_184_0: const #102552u : u32
        let s_184_0: u32 = 102552;
        // D s_184_1: read-reg s_184_0:struct
        let s_184_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_184_0 as isize);
            tracer.read_register(s_184_0 as isize, value);
            value
        };
        // D s_184_2: call _get_HCR_EL2_Type_E2H(s_184_1)
        let s_184_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_184_1);
        // C s_184_3: const #102552u : u32
        let s_184_3: u32 = 102552;
        // D s_184_4: read-reg s_184_3:struct
        let s_184_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_184_3 as isize);
            tracer.read_register(s_184_3 as isize, value);
            value
        };
        // D s_184_5: call _get_HCR_EL2_Type_TGE(s_184_4)
        let s_184_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_184_4);
        // D s_184_6: cast zx s_184_2 -> bv
        let s_184_6: Bits = Bits::new(s_184_2 as u128, 1u16);
        // D s_184_7: cast zx s_184_5 -> bv
        let s_184_7: Bits = Bits::new(s_184_5 as u128, 1u16);
        // D s_184_8: cast reint s_184_6 -> u128
        let s_184_8: u128 = (s_184_6.value() as u128);
        // D s_184_9: size-of s_184_6
        let s_184_9: u16 = s_184_6.length();
        // D s_184_10: cast reint s_184_7 -> u128
        let s_184_10: u128 = (s_184_7.value() as u128);
        // D s_184_11: size-of s_184_7
        let s_184_11: u16 = s_184_7.length();
        // D s_184_12: lsl s_184_8 s_184_11
        let s_184_12: u128 = s_184_8 << s_184_11;
        // D s_184_13: or s_184_12 s_184_10
        let s_184_13: u128 = ((s_184_12) | (s_184_10));
        // D s_184_14: add s_184_9 s_184_11
        let s_184_14: u16 = (s_184_9 + s_184_11);
        // D s_184_15: create-bits s_184_13 s_184_14
        let s_184_15: Bits = Bits::new(s_184_13, s_184_14);
        // D s_184_16: cast reint s_184_15 -> u8
        let s_184_16: u8 = (s_184_15.value() as u8);
        // D s_184_17: cast zx s_184_16 -> bv
        let s_184_17: Bits = Bits::new(s_184_16 as u128, 2u16);
        // C s_184_18: const #3u : u8
        let s_184_18: u8 = 3;
        // C s_184_19: cast zx s_184_18 -> bv
        let s_184_19: Bits = Bits::new(s_184_18 as u128, 2u16);
        // D s_184_20: cmp-ne s_184_17 s_184_19
        let s_184_20: bool = ((s_184_17) != (s_184_19));
        // D s_184_21: write-var gs#105447 <= s_184_20
        fn_state.gs_105447 = s_184_20;
        // N s_184_22: jump b121
        return block_121(state, tracer, fn_state);
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
        // D s_185_3: not s_185_2
        let s_185_3: bool = !s_185_2;
        // D s_185_4: write-var gs#105446 <= s_185_3
        fn_state.gs_105446 = s_185_3;
        // N s_185_5: jump b119
        return block_119(state, tracer, fn_state);
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_186_0: const #() : ()
        let s_186_0: () = ();
        // S s_186_1: call EL2Enabled(s_186_0)
        let s_186_1: bool = EL2Enabled(state, tracer, s_186_0);
        // N s_186_2: branch s_186_1 b202 b187
        if s_186_1 {
            return block_202(state, tracer, fn_state);
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
        // D s_187_1: write-var gs#105464 <= s_187_0
        fn_state.gs_105464 = s_187_0;
        // N s_187_2: jump b188
        return block_188(state, tracer, fn_state);
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_188_0: read-var gs#105464:u8
        let s_188_0: bool = fn_state.gs_105464;
        // N s_188_1: branch s_188_0 b201 b189
        if s_188_0 {
            return block_201(state, tracer, fn_state);
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
        // D s_189_1: write-var gs#105465 <= s_189_0
        fn_state.gs_105465 = s_189_0;
        // N s_189_2: jump b190
        return block_190(state, tracer, fn_state);
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_190_0: read-var gs#105465:u8
        let s_190_0: bool = fn_state.gs_105465;
        // N s_190_1: branch s_190_0 b200 b191
        if s_190_0 {
            return block_200(state, tracer, fn_state);
        } else {
            return block_191(state, tracer, fn_state);
        };
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_191_0: const #() : ()
        let s_191_0: () = ();
        // S s_191_1: call EL2Enabled(s_191_0)
        let s_191_1: bool = EL2Enabled(state, tracer, s_191_0);
        // N s_191_2: branch s_191_1 b199 b192
        if s_191_1 {
            return block_199(state, tracer, fn_state);
        } else {
            return block_192(state, tracer, fn_state);
        };
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_192_0: const #0u : u8
        let s_192_0: bool = false;
        // D s_192_1: write-var gs#105466 <= s_192_0
        fn_state.gs_105466 = s_192_0;
        // N s_192_2: jump b193
        return block_193(state, tracer, fn_state);
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_193_0: read-var gs#105466:u8
        let s_193_0: bool = fn_state.gs_105466;
        // N s_193_1: branch s_193_0 b198 b194
        if s_193_0 {
            return block_198(state, tracer, fn_state);
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
        // D s_194_1: write-var gs#105467 <= s_194_0
        fn_state.gs_105467 = s_194_0;
        // N s_194_2: jump b195
        return block_195(state, tracer, fn_state);
    }
    fn block_195<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_195_0: read-var gs#105467:u8
        let s_195_0: bool = fn_state.gs_105467;
        // N s_195_1: branch s_195_0 b197 b196
        if s_195_0 {
            return block_197(state, tracer, fn_state);
        } else {
            return block_196(state, tracer, fn_state);
        };
    }
    fn block_196<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_196_0: panic
        panic!("{:?}", ());
        // N s_196_1: return
        return;
    }
    fn block_197<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_197_0: const #0u : u8
        let s_197_0: u8 = 0;
        // C s_197_1: cast zx s_197_0 -> bv
        let s_197_1: Bits = Bits::new(s_197_0 as u128, 8u16);
        // C s_197_2: cast zx s_197_1 -> i
        let s_197_2: i128 = (s_197_1.value() as i128);
        // C s_197_3: cast reint s_197_2 -> i64
        let s_197_3: i64 = (s_197_2 as i64);
        // C s_197_4: cast zx s_197_3 -> i
        let s_197_4: i128 = (i128::try_from(s_197_3).unwrap());
        // S s_197_5: call AArch32_TakeHypTrapException(s_197_4)
        let s_197_5: () = AArch32_TakeHypTrapException(state, tracer, s_197_4);
        // N s_197_6: return
        return;
    }
    fn block_198<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_198_0: read-var __HCR_TGE:u8
        let s_198_0: bool = fn_state.u__HCR_TGE;
        // D s_198_1: cast zx s_198_0 -> bv
        let s_198_1: Bits = Bits::new(s_198_0 as u128, 1u16);
        // C s_198_2: const #1u : u8
        let s_198_2: bool = true;
        // C s_198_3: cast zx s_198_2 -> bv
        let s_198_3: Bits = Bits::new(s_198_2 as u128, 1u16);
        // D s_198_4: cmp-eq s_198_1 s_198_3
        let s_198_4: bool = ((s_198_1) == (s_198_3));
        // D s_198_5: write-var gs#105467 <= s_198_4
        fn_state.gs_105467 = s_198_4;
        // N s_198_6: jump b195
        return block_195(state, tracer, fn_state);
    }
    fn block_199<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_199_0: const #432u : u32
        let s_199_0: u32 = 432;
        // D s_199_1: read-reg s_199_0:u8
        let s_199_1: u8 = {
            let value = state.read_register::<u8>(s_199_0 as isize);
            tracer.read_register(s_199_0 as isize, value);
            value
        };
        // D s_199_2: call ELUsingAArch32(s_199_1)
        let s_199_2: bool = ELUsingAArch32(state, tracer, s_199_1);
        // D s_199_3: write-var gs#105466 <= s_199_2
        fn_state.gs_105466 = s_199_2;
        // N s_199_4: jump b193
        return block_193(state, tracer, fn_state);
    }
    fn block_200<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_200_0: const #3u : u8
        let s_200_0: u8 = 3;
        // C s_200_1: cast zx s_200_0 -> bv
        let s_200_1: Bits = Bits::new(s_200_0 as u128, 8u16);
        // C s_200_2: cast zx s_200_1 -> i
        let s_200_2: i128 = (s_200_1.value() as i128);
        // C s_200_3: cast reint s_200_2 -> i64
        let s_200_3: i64 = (s_200_2 as i64);
        // C s_200_4: cast zx s_200_3 -> i
        let s_200_4: i128 = (i128::try_from(s_200_3).unwrap());
        // C s_200_5: const #432u : u32
        let s_200_5: u32 = 432;
        // D s_200_6: read-reg s_200_5:u8
        let s_200_6: u8 = {
            let value = state.read_register::<u8>(s_200_5 as isize);
            tracer.read_register(s_200_5 as isize, value);
            value
        };
        // D s_200_7: call AArch64_AArch32SystemAccessTrap(s_200_6, s_200_4)
        let s_200_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_200_6,
            s_200_4,
        );
        // N s_200_8: return
        return;
    }
    fn block_201<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_201_0: read-var __HCR_EL2_TGE:u8
        let s_201_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_201_1: cast zx s_201_0 -> bv
        let s_201_1: Bits = Bits::new(s_201_0 as u128, 1u16);
        // C s_201_2: const #1u : u8
        let s_201_2: bool = true;
        // C s_201_3: cast zx s_201_2 -> bv
        let s_201_3: Bits = Bits::new(s_201_2 as u128, 1u16);
        // D s_201_4: cmp-eq s_201_1 s_201_3
        let s_201_4: bool = ((s_201_1) == (s_201_3));
        // D s_201_5: write-var gs#105465 <= s_201_4
        fn_state.gs_105465 = s_201_4;
        // N s_201_6: jump b190
        return block_190(state, tracer, fn_state);
    }
    fn block_202<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_202_0: const #432u : u32
        let s_202_0: u32 = 432;
        // D s_202_1: read-reg s_202_0:u8
        let s_202_1: u8 = {
            let value = state.read_register::<u8>(s_202_0 as isize);
            tracer.read_register(s_202_0 as isize, value);
            value
        };
        // D s_202_2: call ELUsingAArch32(s_202_1)
        let s_202_2: bool = ELUsingAArch32(state, tracer, s_202_1);
        // D s_202_3: not s_202_2
        let s_202_3: bool = !s_202_2;
        // D s_202_4: write-var gs#105464 <= s_202_3
        fn_state.gs_105464 = s_202_3;
        // N s_202_5: jump b188
        return block_188(state, tracer, fn_state);
    }
    fn block_203<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_203_0: read-var __AMUSERENR_EN:u8
        let s_203_0: bool = fn_state.u__AMUSERENR_EN;
        // D s_203_1: cast zx s_203_0 -> bv
        let s_203_1: Bits = Bits::new(s_203_0 as u128, 1u16);
        // C s_203_2: const #0u : u8
        let s_203_2: bool = false;
        // C s_203_3: cast zx s_203_2 -> bv
        let s_203_3: Bits = Bits::new(s_203_2 as u128, 1u16);
        // D s_203_4: cmp-eq s_203_1 s_203_3
        let s_203_4: bool = ((s_203_1) == (s_203_3));
        // D s_203_5: write-var gs#105445 <= s_203_4
        fn_state.gs_105445 = s_203_4;
        // N s_203_6: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_204<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_204_0: const #() : ()
        let s_204_0: () = ();
        // S s_204_1: call EL2Enabled(s_204_0)
        let s_204_1: bool = EL2Enabled(state, tracer, s_204_0);
        // N s_204_2: branch s_204_1 b212 b205
        if s_204_1 {
            return block_212(state, tracer, fn_state);
        } else {
            return block_205(state, tracer, fn_state);
        };
    }
    fn block_205<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_205_0: const #0u : u8
        let s_205_0: bool = false;
        // D s_205_1: write-var gs#105468 <= s_205_0
        fn_state.gs_105468 = s_205_0;
        // N s_205_2: jump b206
        return block_206(state, tracer, fn_state);
    }
    fn block_206<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_206_0: read-var gs#105468:u8
        let s_206_0: bool = fn_state.gs_105468;
        // N s_206_1: branch s_206_0 b211 b207
        if s_206_0 {
            return block_211(state, tracer, fn_state);
        } else {
            return block_207(state, tracer, fn_state);
        };
    }
    fn block_207<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_207_0: const #0u : u8
        let s_207_0: bool = false;
        // D s_207_1: write-var gs#105469 <= s_207_0
        fn_state.gs_105469 = s_207_0;
        // N s_207_2: jump b208
        return block_208(state, tracer, fn_state);
    }
    fn block_208<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_208_0: read-var gs#105469:u8
        let s_208_0: bool = fn_state.gs_105469;
        // N s_208_1: branch s_208_0 b210 b209
        if s_208_0 {
            return block_210(state, tracer, fn_state);
        } else {
            return block_209(state, tracer, fn_state);
        };
    }
    fn block_209<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_209_0: const #3u : u8
        let s_209_0: u8 = 3;
        // C s_209_1: cast zx s_209_0 -> bv
        let s_209_1: Bits = Bits::new(s_209_0 as u128, 8u16);
        // C s_209_2: cast zx s_209_1 -> i
        let s_209_2: i128 = (s_209_1.value() as i128);
        // C s_209_3: cast reint s_209_2 -> i64
        let s_209_3: i64 = (s_209_2 as i64);
        // C s_209_4: cast zx s_209_3 -> i
        let s_209_4: i128 = (i128::try_from(s_209_3).unwrap());
        // C s_209_5: const #440u : u32
        let s_209_5: u32 = 440;
        // D s_209_6: read-reg s_209_5:u8
        let s_209_6: u8 = {
            let value = state.read_register::<u8>(s_209_5 as isize);
            tracer.read_register(s_209_5 as isize, value);
            value
        };
        // D s_209_7: call AArch64_AArch32SystemAccessTrap(s_209_6, s_209_4)
        let s_209_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_209_6,
            s_209_4,
        );
        // N s_209_8: return
        return;
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
        // D s_211_5: write-var gs#105469 <= s_211_4
        fn_state.gs_105469 = s_211_4;
        // N s_211_6: jump b208
        return block_208(state, tracer, fn_state);
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
        // D s_212_4: write-var gs#105468 <= s_212_3
        fn_state.gs_105468 = s_212_3;
        // N s_212_5: jump b206
        return block_206(state, tracer, fn_state);
    }
    fn block_213<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_213_0: read-var __AMUSERENR_EL0_EN:u8
        let s_213_0: bool = fn_state.u__AMUSERENR_EL0_EN;
        // D s_213_1: cast zx s_213_0 -> bv
        let s_213_1: Bits = Bits::new(s_213_0 as u128, 1u16);
        // C s_213_2: const #0u : u8
        let s_213_2: bool = false;
        // C s_213_3: cast zx s_213_2 -> bv
        let s_213_3: Bits = Bits::new(s_213_2 as u128, 1u16);
        // D s_213_4: cmp-eq s_213_1 s_213_3
        let s_213_4: bool = ((s_213_1) == (s_213_3));
        // D s_213_5: write-var gs#105444 <= s_213_4
        fn_state.gs_105444 = s_213_4;
        // N s_213_6: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_214<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_214_0: panic
        panic!("{:?}", ());
        // N s_214_1: return
        return;
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
        // D s_215_5: write-var gs#105443 <= s_215_4
        fn_state.gs_105443 = s_215_4;
        // N s_215_6: jump b110
        return block_110(state, tracer, fn_state);
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
        // D s_216_4: write-var gs#105442 <= s_216_3
        fn_state.gs_105442 = s_216_3;
        // N s_216_5: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_217<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_217_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_217_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_217_1: call __IMPDEF_boolean(s_217_0)
        let s_217_1: bool = u__IMPDEF_boolean(state, tracer, s_217_0);
        // D s_217_2: write-var gs#105441 <= s_217_1
        fn_state.gs_105441 = s_217_1;
        // N s_217_3: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_218<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_218_0: const #() : ()
        let s_218_0: () = ();
        // S s_218_1: call EDSCR_read(s_218_0)
        let s_218_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_218_0);
        // S s_218_2: call _get_EDSCR_Type_SDD(s_218_1)
        let s_218_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_218_1);
        // S s_218_3: cast zx s_218_2 -> bv
        let s_218_3: Bits = Bits::new(s_218_2 as u128, 1u16);
        // C s_218_4: const #1u : u8
        let s_218_4: bool = true;
        // C s_218_5: cast zx s_218_4 -> bv
        let s_218_5: Bits = Bits::new(s_218_4 as u128, 1u16);
        // S s_218_6: cmp-eq s_218_3 s_218_5
        let s_218_6: bool = ((s_218_3) == (s_218_5));
        // D s_218_7: write-var gs#105440 <= s_218_6
        fn_state.gs_105440 = s_218_6;
        // N s_218_8: jump b104
        return block_104(state, tracer, fn_state);
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
        // D s_219_4: write-var gs#105439 <= s_219_3
        fn_state.gs_105439 = s_219_3;
        // N s_219_5: jump b102
        return block_102(state, tracer, fn_state);
    }
}
