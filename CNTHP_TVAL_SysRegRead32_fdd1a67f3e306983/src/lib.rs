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
use CNTP_CVAL_NS_read::*;
use u_get_CNTKCTL_Type_PL0PTEN::*;
use u_get_CNTHCTL_EL2_Type_EL1PTEN::*;
use CNTP_CTL_read::*;
use CNTHCTL_read::*;
use CNTP_CTL_NS_read::*;
use u_get_CNTP_CTL_Type_ENABLE::*;
use u_get_SCR_EL3_Type_NS::*;
use u__UNKNOWN_bits::*;
use u_get_CNTHCTL_EL2_Type_EL1PCEN::*;
use u_get_CNTHCTL_EL2_Type_EL0PTEN::*;
use u_get_SCR_EL3_Type_ECVEn::*;
use u_get_CNTHP_CTL_EL2_Type_ENABLE::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_TGE::*;
use u_get_CNTHCTL_Type_PL1PCEN::*;
use u_get_HCR_EL2_Type_E2H::*;
use HCR_read::*;
use IsFeatureImplemented::*;
use u_get_SCR_Type_NS::*;
use AArch64_AArch32SystemAccessTrap::*;
use u_get_HCR_Type_TGE::*;
use CNTP_CVAL_read::*;
use R_set::*;
use ELUsingAArch32::*;
use CNTKCTL_read__1::*;
use PhysicalCountInt::*;
use u_get_CNTKCTL_EL1_Type_EL0PTEN::*;
use u_get_CNTHPS_CTL_EL2_Type_ENABLE::*;
use u_get_CNTHCTL_EL2_Type_ECV::*;
use common::*;
pub fn CNTHP_TVAL_SysRegRead32_fdd1a67f3e306983<T: Tracer>(
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
        u__HCR_EL2_E2H: bool,
        u__CNTP_CTL_NS_ENABLE: bool,
        gs_107000: bool,
        u__CNTHP_CTL_EL2_ENABLE: bool,
        gs_107066: bool,
        u__HCR_TGE: bool,
        ga_166687: ProductType5c790c8ef59cc8b2,
        gs_107030: bool,
        gs_107029: bool,
        ga_166702: ProductType5c790c8ef59cc8b2,
        gs_107005: bool,
        gs_107020: bool,
        u__CNTHCTL_EL2_EL1PCEN: bool,
        gs_107034: bool,
        gs_106986: bool,
        gs_107033: bool,
        gs_107038: bool,
        u__HCR_EL2_TGE: bool,
        gs_107040: bool,
        u__CNTHCTL_EL2_EL0PTEN: bool,
        gs_107039: bool,
        ga_166733: ProductType5c790c8ef59cc8b2,
        ga_166771: ProductType5c790c8ef59cc8b2,
        gs_106995: bool,
        u__CNTKCTL_EL1_EL0PTEN: bool,
        gs_107065: bool,
        gs_107027: bool,
        gs_107063: bool,
        gs_106993: bool,
        gs_107028: bool,
        gs_106998: bool,
        ga_166752: ProductType5c790c8ef59cc8b2,
        gs_107015: bool,
        gs_107018: bool,
        gs_107001: bool,
        gs_107036: bool,
        ga_166675: ProductType5c790c8ef59cc8b2,
        gs_107019: bool,
        gs_106999: bool,
        u__SCR_EL3_ECVEn: bool,
        u__CNTHPS_CTL_EL2_ENABLE: bool,
        gs_107032: bool,
        gs_107022: bool,
        gs_107023: bool,
        gs_106994: bool,
        gs_106996: bool,
        u__CNTHCTL_EL2_ECV: bool,
        u__PSTATE_EL: u8,
        gs_107016: bool,
        u__CNTP_CTL_ENABLE: bool,
        gs_107002: bool,
        gs_107024: bool,
        gs_107003: bool,
        gs_107061: bool,
        ga_166744: ProductType5c790c8ef59cc8b2,
        ga_166788: ProductType5c790c8ef59cc8b2,
        gs_107064: bool,
        gs_106997: bool,
        u__CNTKCTL_PL0PTEN: bool,
        u__SCR_NS: bool,
        u__CNTHCTL_PL1PCEN: bool,
        gs_107037: bool,
        gs_107017: bool,
        gs_107025: bool,
        gs_107004: bool,
        gs_107031: bool,
        gs_107042: bool,
        gs_107041: bool,
        gs_107062: bool,
        gs_107035: bool,
        gs_107021: bool,
        ga_166763: ProductType5c790c8ef59cc8b2,
        u__CNTP_CTL_S_ENABLE: bool,
        u__CNTHCTL_EL2_EL1PTEN: bool,
        gs_107026: bool,
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
        // C s_0_3: const #22056u : u32
        let s_0_3: u32 = 22056;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_CNTKCTL_EL1_Type_EL0PTEN(s_0_4)
        let s_0_5: bool = u_get_CNTKCTL_EL1_Type_EL0PTEN(state, tracer, s_0_4);
        // D s_0_6: write-var __CNTKCTL_EL1_EL0PTEN <= s_0_5
        fn_state.u__CNTKCTL_EL1_EL0PTEN = s_0_5;
        // C s_0_7: const #102552u : u32
        let s_0_7: u32 = 102552;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_HCR_EL2_Type_TGE(s_0_8)
        let s_0_9: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_0_8);
        // D s_0_10: write-var __HCR_EL2_TGE <= s_0_9
        fn_state.u__HCR_EL2_TGE = s_0_9;
        // C s_0_11: const #() : ()
        let s_0_11: () = ();
        // S s_0_12: call CNTKCTL_read__1(s_0_11)
        let s_0_12: ProductType700c18a878c5601b = CNTKCTL_read__1(state, tracer, s_0_11);
        // S s_0_13: call _get_CNTKCTL_Type_PL0PTEN(s_0_12)
        let s_0_13: bool = u_get_CNTKCTL_Type_PL0PTEN(state, tracer, s_0_12);
        // D s_0_14: write-var __CNTKCTL_PL0PTEN <= s_0_13
        fn_state.u__CNTKCTL_PL0PTEN = s_0_13;
        // C s_0_15: const #() : ()
        let s_0_15: () = ();
        // S s_0_16: call HCR_read(s_0_15)
        let s_0_16: ProductType700c18a878c5601b = HCR_read(state, tracer, s_0_15);
        // S s_0_17: call _get_HCR_Type_TGE(s_0_16)
        let s_0_17: bool = u_get_HCR_Type_TGE(state, tracer, s_0_16);
        // D s_0_18: write-var __HCR_TGE <= s_0_17
        fn_state.u__HCR_TGE = s_0_17;
        // C s_0_19: const #102552u : u32
        let s_0_19: u32 = 102552;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_HCR_EL2_Type_E2H(s_0_20)
        let s_0_21: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_0_20);
        // D s_0_22: write-var __HCR_EL2_E2H <= s_0_21
        fn_state.u__HCR_EL2_E2H = s_0_21;
        // C s_0_23: const #12808u : u32
        let s_0_23: u32 = 12808;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_CNTHCTL_EL2_Type_EL1PCEN(s_0_24)
        let s_0_25: bool = u_get_CNTHCTL_EL2_Type_EL1PCEN(state, tracer, s_0_24);
        // D s_0_26: write-var __CNTHCTL_EL2_EL1PCEN <= s_0_25
        fn_state.u__CNTHCTL_EL2_EL1PCEN = s_0_25;
        // C s_0_27: const #12808u : u32
        let s_0_27: u32 = 12808;
        // D s_0_28: read-reg s_0_27:struct
        let s_0_28: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_27 as isize);
            tracer.read_register(s_0_27 as isize, value);
            value
        };
        // D s_0_29: call _get_CNTHCTL_EL2_Type_EL1PTEN(s_0_28)
        let s_0_29: bool = u_get_CNTHCTL_EL2_Type_EL1PTEN(state, tracer, s_0_28);
        // D s_0_30: write-var __CNTHCTL_EL2_EL1PTEN <= s_0_29
        fn_state.u__CNTHCTL_EL2_EL1PTEN = s_0_29;
        // C s_0_31: const #12808u : u32
        let s_0_31: u32 = 12808;
        // D s_0_32: read-reg s_0_31:struct
        let s_0_32: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_31 as isize);
            tracer.read_register(s_0_31 as isize, value);
            value
        };
        // D s_0_33: call _get_CNTHCTL_EL2_Type_EL0PTEN(s_0_32)
        let s_0_33: bool = u_get_CNTHCTL_EL2_Type_EL0PTEN(state, tracer, s_0_32);
        // D s_0_34: write-var __CNTHCTL_EL2_EL0PTEN <= s_0_33
        fn_state.u__CNTHCTL_EL2_EL0PTEN = s_0_33;
        // C s_0_35: const #() : ()
        let s_0_35: () = ();
        // S s_0_36: call CNTHCTL_read(s_0_35)
        let s_0_36: ProductType700c18a878c5601b = CNTHCTL_read(state, tracer, s_0_35);
        // S s_0_37: call _get_CNTHCTL_Type_PL1PCEN(s_0_36)
        let s_0_37: bool = u_get_CNTHCTL_Type_PL1PCEN(state, tracer, s_0_36);
        // D s_0_38: write-var __CNTHCTL_PL1PCEN <= s_0_37
        fn_state.u__CNTHCTL_PL1PCEN = s_0_37;
        // C s_0_39: const #10504u : u32
        let s_0_39: u32 = 10504;
        // D s_0_40: read-reg s_0_39:struct
        let s_0_40: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_39 as isize);
            tracer.read_register(s_0_39 as isize, value);
            value
        };
        // D s_0_41: call _get_CNTHPS_CTL_EL2_Type_ENABLE(s_0_40)
        let s_0_41: bool = u_get_CNTHPS_CTL_EL2_Type_ENABLE(state, tracer, s_0_40);
        // D s_0_42: write-var __CNTHPS_CTL_EL2_ENABLE <= s_0_41
        fn_state.u__CNTHPS_CTL_EL2_ENABLE = s_0_41;
        // C s_0_43: const #100912u : u32
        let s_0_43: u32 = 100912;
        // D s_0_44: read-reg s_0_43:struct
        let s_0_44: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_43 as isize);
            tracer.read_register(s_0_43 as isize, value);
            value
        };
        // D s_0_45: call _get_CNTHP_CTL_EL2_Type_ENABLE(s_0_44)
        let s_0_45: bool = u_get_CNTHP_CTL_EL2_Type_ENABLE(state, tracer, s_0_44);
        // D s_0_46: write-var __CNTHP_CTL_EL2_ENABLE <= s_0_45
        fn_state.u__CNTHP_CTL_EL2_ENABLE = s_0_45;
        // C s_0_47: const #90704u : u32
        let s_0_47: u32 = 90704;
        // D s_0_48: read-reg s_0_47:struct
        let s_0_48: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_47 as isize);
            tracer.read_register(s_0_47 as isize, value);
            value
        };
        // D s_0_49: call _get_SCR_EL3_Type_ECVEn(s_0_48)
        let s_0_49: bool = u_get_SCR_EL3_Type_ECVEn(state, tracer, s_0_48);
        // D s_0_50: write-var __SCR_EL3_ECVEn <= s_0_49
        fn_state.u__SCR_EL3_ECVEn = s_0_49;
        // C s_0_51: const #12808u : u32
        let s_0_51: u32 = 12808;
        // D s_0_52: read-reg s_0_51:struct
        let s_0_52: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_51 as isize);
            tracer.read_register(s_0_51 as isize, value);
            value
        };
        // D s_0_53: call _get_CNTHCTL_EL2_Type_ECV(s_0_52)
        let s_0_53: bool = u_get_CNTHCTL_EL2_Type_ECV(state, tracer, s_0_52);
        // D s_0_54: write-var __CNTHCTL_EL2_ECV <= s_0_53
        fn_state.u__CNTHCTL_EL2_ECV = s_0_53;
        // C s_0_55: const #() : ()
        let s_0_55: () = ();
        // S s_0_56: call CNTP_CTL_read(s_0_55)
        let s_0_56: ProductType700c18a878c5601b = CNTP_CTL_read(state, tracer, s_0_55);
        // S s_0_57: call _get_CNTP_CTL_Type_ENABLE(s_0_56)
        let s_0_57: bool = u_get_CNTP_CTL_Type_ENABLE(state, tracer, s_0_56);
        // D s_0_58: write-var __CNTP_CTL_ENABLE <= s_0_57
        fn_state.u__CNTP_CTL_ENABLE = s_0_57;
        // C s_0_59: const #20920u : u32
        let s_0_59: u32 = 20920;
        // D s_0_60: read-reg s_0_59:struct
        let s_0_60: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_59 as isize);
            tracer.read_register(s_0_59 as isize, value);
            value
        };
        // D s_0_61: call _get_SCR_Type_NS(s_0_60)
        let s_0_61: bool = u_get_SCR_Type_NS(state, tracer, s_0_60);
        // D s_0_62: write-var __SCR_NS <= s_0_61
        fn_state.u__SCR_NS = s_0_61;
        // C s_0_63: const #() : ()
        let s_0_63: () = ();
        // S s_0_64: call CNTP_CTL_NS_read(s_0_63)
        let s_0_64: ProductType700c18a878c5601b = CNTP_CTL_NS_read(
            state,
            tracer,
            s_0_63,
        );
        // S s_0_65: call _get_CNTP_CTL_Type_ENABLE(s_0_64)
        let s_0_65: bool = u_get_CNTP_CTL_Type_ENABLE(state, tracer, s_0_64);
        // D s_0_66: write-var __CNTP_CTL_NS_ENABLE <= s_0_65
        fn_state.u__CNTP_CTL_NS_ENABLE = s_0_65;
        // C s_0_67: const #21888u : u32
        let s_0_67: u32 = 21888;
        // D s_0_68: read-reg s_0_67:struct
        let s_0_68: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_67 as isize);
            tracer.read_register(s_0_67 as isize, value);
            value
        };
        // D s_0_69: call _get_CNTP_CTL_Type_ENABLE(s_0_68)
        let s_0_69: bool = u_get_CNTP_CTL_Type_ENABLE(state, tracer, s_0_68);
        // D s_0_70: write-var __CNTP_CTL_S_ENABLE <= s_0_69
        fn_state.u__CNTP_CTL_S_ENABLE = s_0_69;
        // D s_0_71: read-var __PSTATE_EL:u8
        let s_0_71: u8 = fn_state.u__PSTATE_EL;
        // D s_0_72: cast zx s_0_71 -> bv
        let s_0_72: Bits = Bits::new(s_0_71 as u128, 2u16);
        // C s_0_73: const #448u : u32
        let s_0_73: u32 = 448;
        // D s_0_74: read-reg s_0_73:u8
        let s_0_74: u8 = {
            let value = state.read_register::<u8>(s_0_73 as isize);
            tracer.read_register(s_0_73 as isize, value);
            value
        };
        // D s_0_75: cast zx s_0_74 -> bv
        let s_0_75: Bits = Bits::new(s_0_74 as u128, 2u16);
        // D s_0_76: cmp-eq s_0_72 s_0_75
        let s_0_76: bool = ((s_0_72) == (s_0_75));
        // N s_0_77: branch s_0_76 b78 b1
        if s_0_76 {
            return block_78(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b22 b2
        if s_1_5 {
            return block_22(state, tracer, fn_state);
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
        // N s_2_6: branch s_2_5 b12 b3
        if s_2_5 {
            return block_12(state, tracer, fn_state);
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
        // D s_5_0: read-var __SCR_NS:u8
        let s_5_0: bool = fn_state.u__SCR_NS;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 1u16);
        // C s_5_2: const #0u : u8
        let s_5_2: bool = false;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // N s_5_5: branch s_5_4 b9 b6
        if s_5_4 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var __CNTP_CTL_NS_ENABLE:u8
        let s_6_0: bool = fn_state.u__CNTP_CTL_NS_ENABLE;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 1u16);
        // C s_6_2: const #0u : u8
        let s_6_2: bool = false;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // N s_6_5: branch s_6_4 b8 b7
        if s_6_4 {
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
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call CNTP_CVAL_NS_read(s_7_0)
        let s_7_1: ProductType5c790c8ef59cc8b2 = CNTP_CVAL_NS_read(state, tracer, s_7_0);
        // D s_7_2: write-var ga#166788 <= s_7_1
        fn_state.ga_166788 = s_7_1;
        // D s_7_3: read-var ga#166788.0:struct
        let s_7_3: u64 = fn_state.ga_166788._0;
        // C s_7_4: const #() : ()
        let s_7_4: () = ();
        // S s_7_5: call PhysicalCountInt(s_7_4)
        let s_7_5: u64 = PhysicalCountInt(state, tracer, s_7_4);
        // D s_7_6: cast zx s_7_3 -> bv
        let s_7_6: Bits = Bits::new(s_7_3 as u128, 64u16);
        // S s_7_7: cast zx s_7_5 -> bv
        let s_7_7: Bits = Bits::new(s_7_5 as u128, 64u16);
        // D s_7_8: sub s_7_6 s_7_7
        let s_7_8: Bits = ((s_7_6) - (s_7_7));
        // D s_7_9: cast reint s_7_8 -> u64
        let s_7_9: u64 = (s_7_8.value() as u64);
        // C s_7_10: const #0s : i
        let s_7_10: i128 = 0;
        // D s_7_11: cast zx s_7_9 -> bv
        let s_7_11: Bits = Bits::new(s_7_9 as u128, 64u16);
        // C s_7_12: const #1s : i64
        let s_7_12: i64 = 1;
        // C s_7_13: cast zx s_7_12 -> i
        let s_7_13: i128 = (i128::try_from(s_7_12).unwrap());
        // C s_7_14: const #31s : i
        let s_7_14: i128 = 31;
        // C s_7_15: add s_7_14 s_7_13
        let s_7_15: i128 = (s_7_14 + s_7_13);
        // D s_7_16: bit-extract s_7_11 s_7_10 s_7_15
        let s_7_16: Bits = (Bits::new(
            ((s_7_11) >> (s_7_10)).value(),
            u16::try_from(s_7_15).unwrap(),
        ));
        // D s_7_17: cast reint s_7_16 -> u32
        let s_7_17: u32 = (s_7_16.value() as u32);
        // D s_7_18: read-var t:i
        let s_7_18: i128 = fn_state.t;
        // D s_7_19: call R_set(s_7_18, s_7_17)
        let s_7_19: () = R_set(state, tracer, s_7_18, s_7_17);
        // N s_7_20: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #32s : i64
        let s_8_0: i64 = 32;
        // C s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // S s_8_2: call __UNKNOWN_bits(s_8_1)
        let s_8_2: Bits = u__UNKNOWN_bits(state, tracer, s_8_1);
        // S s_8_3: cast reint s_8_2 -> u32
        let s_8_3: u32 = (s_8_2.value() as u32);
        // D s_8_4: read-var t:i
        let s_8_4: i128 = fn_state.t;
        // D s_8_5: call R_set(s_8_4, s_8_3)
        let s_8_5: () = R_set(state, tracer, s_8_4, s_8_3);
        // N s_8_6: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var __CNTP_CTL_S_ENABLE:u8
        let s_9_0: bool = fn_state.u__CNTP_CTL_S_ENABLE;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 1u16);
        // C s_9_2: const #0u : u8
        let s_9_2: bool = false;
        // C s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 1u16);
        // D s_9_4: cmp-eq s_9_1 s_9_3
        let s_9_4: bool = ((s_9_1) == (s_9_3));
        // N s_9_5: branch s_9_4 b11 b10
        if s_9_4 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #16848u : u32
        let s_10_0: u32 = 16848;
        // D s_10_1: read-reg s_10_0:u64
        let s_10_1: u64 = {
            let value = state.read_register::<u64>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // C s_10_2: const #() : ()
        let s_10_2: () = ();
        // S s_10_3: call PhysicalCountInt(s_10_2)
        let s_10_3: u64 = PhysicalCountInt(state, tracer, s_10_2);
        // D s_10_4: cast zx s_10_1 -> bv
        let s_10_4: Bits = Bits::new(s_10_1 as u128, 64u16);
        // S s_10_5: cast zx s_10_3 -> bv
        let s_10_5: Bits = Bits::new(s_10_3 as u128, 64u16);
        // D s_10_6: sub s_10_4 s_10_5
        let s_10_6: Bits = ((s_10_4) - (s_10_5));
        // D s_10_7: cast reint s_10_6 -> u64
        let s_10_7: u64 = (s_10_6.value() as u64);
        // C s_10_8: const #0s : i
        let s_10_8: i128 = 0;
        // D s_10_9: cast zx s_10_7 -> bv
        let s_10_9: Bits = Bits::new(s_10_7 as u128, 64u16);
        // C s_10_10: const #1s : i64
        let s_10_10: i64 = 1;
        // C s_10_11: cast zx s_10_10 -> i
        let s_10_11: i128 = (i128::try_from(s_10_10).unwrap());
        // C s_10_12: const #31s : i
        let s_10_12: i128 = 31;
        // C s_10_13: add s_10_12 s_10_11
        let s_10_13: i128 = (s_10_12 + s_10_11);
        // D s_10_14: bit-extract s_10_9 s_10_8 s_10_13
        let s_10_14: Bits = (Bits::new(
            ((s_10_9) >> (s_10_8)).value(),
            u16::try_from(s_10_13).unwrap(),
        ));
        // D s_10_15: cast reint s_10_14 -> u32
        let s_10_15: u32 = (s_10_14.value() as u32);
        // D s_10_16: read-var t:i
        let s_10_16: i128 = fn_state.t;
        // D s_10_17: call R_set(s_10_16, s_10_15)
        let s_10_17: () = R_set(state, tracer, s_10_16, s_10_15);
        // N s_10_18: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #32s : i64
        let s_11_0: i64 = 32;
        // C s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // S s_11_2: call __UNKNOWN_bits(s_11_1)
        let s_11_2: Bits = u__UNKNOWN_bits(state, tracer, s_11_1);
        // S s_11_3: cast reint s_11_2 -> u32
        let s_11_3: u32 = (s_11_2.value() as u32);
        // D s_11_4: read-var t:i
        let s_11_4: i128 = fn_state.t;
        // D s_11_5: call R_set(s_11_4, s_11_3)
        let s_11_5: () = R_set(state, tracer, s_11_4, s_11_3);
        // N s_11_6: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #424u : u32
        let s_12_0: u32 = 424;
        // D s_12_1: read-reg s_12_0:u8
        let s_12_1: u8 = {
            let value = state.read_register::<u8>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // C s_12_2: const #2u : u8
        let s_12_2: u8 = 2;
        // D s_12_3: cmp-lt s_12_1 s_12_2
        let s_12_3: bool = ((s_12_1) < (s_12_2));
        // N s_12_4: branch s_12_3 b21 b13
        if s_12_3 {
            return block_21(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#106986 <= s_13_0
        fn_state.gs_106986 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#106986:u8
        let s_14_0: bool = fn_state.gs_106986;
        // N s_14_1: branch s_14_0 b18 b15
        if s_14_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var __CNTP_CTL_ENABLE:u8
        let s_15_0: bool = fn_state.u__CNTP_CTL_ENABLE;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 1u16);
        // C s_15_2: const #0u : u8
        let s_15_2: bool = false;
        // C s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 1u16);
        // D s_15_4: cmp-eq s_15_1 s_15_3
        let s_15_4: bool = ((s_15_1) == (s_15_3));
        // N s_15_5: branch s_15_4 b17 b16
        if s_15_4 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call CNTP_CVAL_read(s_16_0)
        let s_16_1: ProductType5c790c8ef59cc8b2 = CNTP_CVAL_read(state, tracer, s_16_0);
        // D s_16_2: write-var ga#166771 <= s_16_1
        fn_state.ga_166771 = s_16_1;
        // D s_16_3: read-var ga#166771.0:struct
        let s_16_3: u64 = fn_state.ga_166771._0;
        // C s_16_4: const #() : ()
        let s_16_4: () = ();
        // S s_16_5: call PhysicalCountInt(s_16_4)
        let s_16_5: u64 = PhysicalCountInt(state, tracer, s_16_4);
        // D s_16_6: cast zx s_16_3 -> bv
        let s_16_6: Bits = Bits::new(s_16_3 as u128, 64u16);
        // S s_16_7: cast zx s_16_5 -> bv
        let s_16_7: Bits = Bits::new(s_16_5 as u128, 64u16);
        // D s_16_8: sub s_16_6 s_16_7
        let s_16_8: Bits = ((s_16_6) - (s_16_7));
        // D s_16_9: cast reint s_16_8 -> u64
        let s_16_9: u64 = (s_16_8.value() as u64);
        // C s_16_10: const #0s : i
        let s_16_10: i128 = 0;
        // D s_16_11: cast zx s_16_9 -> bv
        let s_16_11: Bits = Bits::new(s_16_9 as u128, 64u16);
        // C s_16_12: const #1s : i64
        let s_16_12: i64 = 1;
        // C s_16_13: cast zx s_16_12 -> i
        let s_16_13: i128 = (i128::try_from(s_16_12).unwrap());
        // C s_16_14: const #31s : i
        let s_16_14: i128 = 31;
        // C s_16_15: add s_16_14 s_16_13
        let s_16_15: i128 = (s_16_14 + s_16_13);
        // D s_16_16: bit-extract s_16_11 s_16_10 s_16_15
        let s_16_16: Bits = (Bits::new(
            ((s_16_11) >> (s_16_10)).value(),
            u16::try_from(s_16_15).unwrap(),
        ));
        // D s_16_17: cast reint s_16_16 -> u32
        let s_16_17: u32 = (s_16_16.value() as u32);
        // D s_16_18: read-var t:i
        let s_16_18: i128 = fn_state.t;
        // D s_16_19: call R_set(s_16_18, s_16_17)
        let s_16_19: () = R_set(state, tracer, s_16_18, s_16_17);
        // N s_16_20: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #32s : i64
        let s_17_0: i64 = 32;
        // C s_17_1: cast zx s_17_0 -> i
        let s_17_1: i128 = (i128::try_from(s_17_0).unwrap());
        // S s_17_2: call __UNKNOWN_bits(s_17_1)
        let s_17_2: Bits = u__UNKNOWN_bits(state, tracer, s_17_1);
        // S s_17_3: cast reint s_17_2 -> u32
        let s_17_3: u32 = (s_17_2.value() as u32);
        // D s_17_4: read-var t:i
        let s_17_4: i128 = fn_state.t;
        // D s_17_5: call R_set(s_17_4, s_17_3)
        let s_17_5: () = R_set(state, tracer, s_17_4, s_17_3);
        // N s_17_6: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var __CNTP_CTL_NS_ENABLE:u8
        let s_18_0: bool = fn_state.u__CNTP_CTL_NS_ENABLE;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 1u16);
        // C s_18_2: const #0u : u8
        let s_18_2: bool = false;
        // C s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 1u16);
        // D s_18_4: cmp-eq s_18_1 s_18_3
        let s_18_4: bool = ((s_18_1) == (s_18_3));
        // N s_18_5: branch s_18_4 b20 b19
        if s_18_4 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call CNTP_CVAL_NS_read(s_19_0)
        let s_19_1: ProductType5c790c8ef59cc8b2 = CNTP_CVAL_NS_read(
            state,
            tracer,
            s_19_0,
        );
        // D s_19_2: write-var ga#166763 <= s_19_1
        fn_state.ga_166763 = s_19_1;
        // D s_19_3: read-var ga#166763.0:struct
        let s_19_3: u64 = fn_state.ga_166763._0;
        // C s_19_4: const #() : ()
        let s_19_4: () = ();
        // S s_19_5: call PhysicalCountInt(s_19_4)
        let s_19_5: u64 = PhysicalCountInt(state, tracer, s_19_4);
        // D s_19_6: cast zx s_19_3 -> bv
        let s_19_6: Bits = Bits::new(s_19_3 as u128, 64u16);
        // S s_19_7: cast zx s_19_5 -> bv
        let s_19_7: Bits = Bits::new(s_19_5 as u128, 64u16);
        // D s_19_8: sub s_19_6 s_19_7
        let s_19_8: Bits = ((s_19_6) - (s_19_7));
        // D s_19_9: cast reint s_19_8 -> u64
        let s_19_9: u64 = (s_19_8.value() as u64);
        // C s_19_10: const #0s : i
        let s_19_10: i128 = 0;
        // D s_19_11: cast zx s_19_9 -> bv
        let s_19_11: Bits = Bits::new(s_19_9 as u128, 64u16);
        // C s_19_12: const #1s : i64
        let s_19_12: i64 = 1;
        // C s_19_13: cast zx s_19_12 -> i
        let s_19_13: i128 = (i128::try_from(s_19_12).unwrap());
        // C s_19_14: const #31s : i
        let s_19_14: i128 = 31;
        // C s_19_15: add s_19_14 s_19_13
        let s_19_15: i128 = (s_19_14 + s_19_13);
        // D s_19_16: bit-extract s_19_11 s_19_10 s_19_15
        let s_19_16: Bits = (Bits::new(
            ((s_19_11) >> (s_19_10)).value(),
            u16::try_from(s_19_15).unwrap(),
        ));
        // D s_19_17: cast reint s_19_16 -> u32
        let s_19_17: u32 = (s_19_16.value() as u32);
        // D s_19_18: read-var t:i
        let s_19_18: i128 = fn_state.t;
        // D s_19_19: call R_set(s_19_18, s_19_17)
        let s_19_19: () = R_set(state, tracer, s_19_18, s_19_17);
        // N s_19_20: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #32s : i64
        let s_20_0: i64 = 32;
        // C s_20_1: cast zx s_20_0 -> i
        let s_20_1: i128 = (i128::try_from(s_20_0).unwrap());
        // S s_20_2: call __UNKNOWN_bits(s_20_1)
        let s_20_2: Bits = u__UNKNOWN_bits(state, tracer, s_20_1);
        // S s_20_3: cast reint s_20_2 -> u32
        let s_20_3: u32 = (s_20_2.value() as u32);
        // D s_20_4: read-var t:i
        let s_20_4: i128 = fn_state.t;
        // D s_20_5: call R_set(s_20_4, s_20_3)
        let s_20_5: () = R_set(state, tracer, s_20_4, s_20_3);
        // N s_20_6: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #424u : u32
        let s_21_0: u32 = 424;
        // D s_21_1: read-reg s_21_0:u8
        let s_21_1: u8 = {
            let value = state.read_register::<u8>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // D s_21_2: call ELUsingAArch32(s_21_1)
        let s_21_2: bool = ELUsingAArch32(state, tracer, s_21_1);
        // D s_21_3: write-var gs#106986 <= s_21_2
        fn_state.gs_106986 = s_21_2;
        // N s_21_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call EL2Enabled(s_22_0)
        let s_22_1: bool = EL2Enabled(state, tracer, s_22_0);
        // N s_22_2: branch s_22_1 b77 b23
        if s_22_1 {
            return block_77(state, tracer, fn_state);
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
        // D s_23_1: write-var gs#106993 <= s_23_0
        fn_state.gs_106993 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#106993:u8
        let s_24_0: bool = fn_state.gs_106993;
        // N s_24_1: branch s_24_0 b76 b25
        if s_24_0 {
            return block_76(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#106994 <= s_25_0
        fn_state.gs_106994 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#106994:u8
        let s_26_0: bool = fn_state.gs_106994;
        // N s_26_1: branch s_26_0 b75 b27
        if s_26_0 {
            return block_75(state, tracer, fn_state);
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
        // D s_27_1: write-var gs#106995 <= s_27_0
        fn_state.gs_106995 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#106995:u8
        let s_28_0: bool = fn_state.gs_106995;
        // N s_28_1: branch s_28_0 b74 b29
        if s_28_0 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #() : ()
        let s_29_0: () = ();
        // S s_29_1: call EL2Enabled(s_29_0)
        let s_29_1: bool = EL2Enabled(state, tracer, s_29_0);
        // N s_29_2: branch s_29_1 b73 b30
        if s_29_1 {
            return block_73(state, tracer, fn_state);
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
        // D s_30_1: write-var gs#106996 <= s_30_0
        fn_state.gs_106996 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#106996:u8
        let s_31_0: bool = fn_state.gs_106996;
        // N s_31_1: branch s_31_0 b72 b32
        if s_31_0 {
            return block_72(state, tracer, fn_state);
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
        // D s_32_1: write-var gs#106997 <= s_32_0
        fn_state.gs_106997 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#106997:u8
        let s_33_0: bool = fn_state.gs_106997;
        // N s_33_1: branch s_33_0 b71 b34
        if s_33_0 {
            return block_71(state, tracer, fn_state);
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
        // D s_34_1: write-var gs#106998 <= s_34_0
        fn_state.gs_106998 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#106998:u8
        let s_35_0: bool = fn_state.gs_106998;
        // N s_35_1: branch s_35_0 b70 b36
        if s_35_0 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #() : ()
        let s_36_0: () = ();
        // S s_36_1: call EL2Enabled(s_36_0)
        let s_36_1: bool = EL2Enabled(state, tracer, s_36_0);
        // N s_36_2: branch s_36_1 b69 b37
        if s_36_1 {
            return block_69(state, tracer, fn_state);
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
        // D s_37_1: write-var gs#106999 <= s_37_0
        fn_state.gs_106999 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#106999:u8
        let s_38_0: bool = fn_state.gs_106999;
        // N s_38_1: branch s_38_0 b68 b39
        if s_38_0 {
            return block_68(state, tracer, fn_state);
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
        // D s_39_1: write-var gs#107000 <= s_39_0
        fn_state.gs_107000 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#107000:u8
        let s_40_0: bool = fn_state.gs_107000;
        // N s_40_1: branch s_40_0 b67 b41
        if s_40_0 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #145u : u32
        let s_41_0: u32 = 145;
        // S s_41_1: call IsFeatureImplemented(s_41_0)
        let s_41_1: bool = IsFeatureImplemented(state, tracer, s_41_0);
        // N s_41_2: branch s_41_1 b66 b42
        if s_41_1 {
            return block_66(state, tracer, fn_state);
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
        // D s_42_1: write-var gs#107001 <= s_42_0
        fn_state.gs_107001 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#107001:u8
        let s_43_0: bool = fn_state.gs_107001;
        // N s_43_1: branch s_43_0 b65 b44
        if s_43_0 {
            return block_65(state, tracer, fn_state);
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
        // D s_44_1: write-var gs#107002 <= s_44_0
        fn_state.gs_107002 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#107002:u8
        let s_45_0: bool = fn_state.gs_107002;
        // N s_45_1: branch s_45_0 b64 b46
        if s_45_0 {
            return block_64(state, tracer, fn_state);
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
        // D s_46_1: write-var gs#107003 <= s_46_0
        fn_state.gs_107003 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#107003:u8
        let s_47_0: bool = fn_state.gs_107003;
        // N s_47_1: branch s_47_0 b63 b48
        if s_47_0 {
            return block_63(state, tracer, fn_state);
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
        // D s_48_1: write-var gs#107004 <= s_48_0
        fn_state.gs_107004 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#107004:u8
        let s_49_0: bool = fn_state.gs_107004;
        // N s_49_1: branch s_49_0 b60 b50
        if s_49_0 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
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
        // C s_50_2: const #2u : u8
        let s_50_2: u8 = 2;
        // D s_50_3: cmp-lt s_50_1 s_50_2
        let s_50_3: bool = ((s_50_1) < (s_50_2));
        // N s_50_4: branch s_50_3 b59 b51
        if s_50_3 {
            return block_59(state, tracer, fn_state);
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
        // D s_51_1: write-var gs#107005 <= s_51_0
        fn_state.gs_107005 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#107005:u8
        let s_52_0: bool = fn_state.gs_107005;
        // N s_52_1: branch s_52_0 b56 b53
        if s_52_0 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var __CNTP_CTL_ENABLE:u8
        let s_53_0: bool = fn_state.u__CNTP_CTL_ENABLE;
        // D s_53_1: cast zx s_53_0 -> bv
        let s_53_1: Bits = Bits::new(s_53_0 as u128, 1u16);
        // C s_53_2: const #0u : u8
        let s_53_2: bool = false;
        // C s_53_3: cast zx s_53_2 -> bv
        let s_53_3: Bits = Bits::new(s_53_2 as u128, 1u16);
        // D s_53_4: cmp-eq s_53_1 s_53_3
        let s_53_4: bool = ((s_53_1) == (s_53_3));
        // N s_53_5: branch s_53_4 b55 b54
        if s_53_4 {
            return block_55(state, tracer, fn_state);
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
        // S s_54_1: call CNTP_CVAL_read(s_54_0)
        let s_54_1: ProductType5c790c8ef59cc8b2 = CNTP_CVAL_read(state, tracer, s_54_0);
        // D s_54_2: write-var ga#166752 <= s_54_1
        fn_state.ga_166752 = s_54_1;
        // D s_54_3: read-var ga#166752.0:struct
        let s_54_3: u64 = fn_state.ga_166752._0;
        // C s_54_4: const #() : ()
        let s_54_4: () = ();
        // S s_54_5: call PhysicalCountInt(s_54_4)
        let s_54_5: u64 = PhysicalCountInt(state, tracer, s_54_4);
        // D s_54_6: cast zx s_54_3 -> bv
        let s_54_6: Bits = Bits::new(s_54_3 as u128, 64u16);
        // S s_54_7: cast zx s_54_5 -> bv
        let s_54_7: Bits = Bits::new(s_54_5 as u128, 64u16);
        // D s_54_8: sub s_54_6 s_54_7
        let s_54_8: Bits = ((s_54_6) - (s_54_7));
        // D s_54_9: cast reint s_54_8 -> u64
        let s_54_9: u64 = (s_54_8.value() as u64);
        // C s_54_10: const #0s : i
        let s_54_10: i128 = 0;
        // D s_54_11: cast zx s_54_9 -> bv
        let s_54_11: Bits = Bits::new(s_54_9 as u128, 64u16);
        // C s_54_12: const #1s : i64
        let s_54_12: i64 = 1;
        // C s_54_13: cast zx s_54_12 -> i
        let s_54_13: i128 = (i128::try_from(s_54_12).unwrap());
        // C s_54_14: const #31s : i
        let s_54_14: i128 = 31;
        // C s_54_15: add s_54_14 s_54_13
        let s_54_15: i128 = (s_54_14 + s_54_13);
        // D s_54_16: bit-extract s_54_11 s_54_10 s_54_15
        let s_54_16: Bits = (Bits::new(
            ((s_54_11) >> (s_54_10)).value(),
            u16::try_from(s_54_15).unwrap(),
        ));
        // D s_54_17: cast reint s_54_16 -> u32
        let s_54_17: u32 = (s_54_16.value() as u32);
        // D s_54_18: read-var t:i
        let s_54_18: i128 = fn_state.t;
        // D s_54_19: call R_set(s_54_18, s_54_17)
        let s_54_19: () = R_set(state, tracer, s_54_18, s_54_17);
        // N s_54_20: return
        return;
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #32s : i64
        let s_55_0: i64 = 32;
        // C s_55_1: cast zx s_55_0 -> i
        let s_55_1: i128 = (i128::try_from(s_55_0).unwrap());
        // S s_55_2: call __UNKNOWN_bits(s_55_1)
        let s_55_2: Bits = u__UNKNOWN_bits(state, tracer, s_55_1);
        // S s_55_3: cast reint s_55_2 -> u32
        let s_55_3: u32 = (s_55_2.value() as u32);
        // D s_55_4: read-var t:i
        let s_55_4: i128 = fn_state.t;
        // D s_55_5: call R_set(s_55_4, s_55_3)
        let s_55_5: () = R_set(state, tracer, s_55_4, s_55_3);
        // N s_55_6: return
        return;
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var __CNTP_CTL_NS_ENABLE:u8
        let s_56_0: bool = fn_state.u__CNTP_CTL_NS_ENABLE;
        // D s_56_1: cast zx s_56_0 -> bv
        let s_56_1: Bits = Bits::new(s_56_0 as u128, 1u16);
        // C s_56_2: const #0u : u8
        let s_56_2: bool = false;
        // C s_56_3: cast zx s_56_2 -> bv
        let s_56_3: Bits = Bits::new(s_56_2 as u128, 1u16);
        // D s_56_4: cmp-eq s_56_1 s_56_3
        let s_56_4: bool = ((s_56_1) == (s_56_3));
        // N s_56_5: branch s_56_4 b58 b57
        if s_56_4 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #() : ()
        let s_57_0: () = ();
        // S s_57_1: call CNTP_CVAL_NS_read(s_57_0)
        let s_57_1: ProductType5c790c8ef59cc8b2 = CNTP_CVAL_NS_read(
            state,
            tracer,
            s_57_0,
        );
        // D s_57_2: write-var ga#166744 <= s_57_1
        fn_state.ga_166744 = s_57_1;
        // D s_57_3: read-var ga#166744.0:struct
        let s_57_3: u64 = fn_state.ga_166744._0;
        // C s_57_4: const #() : ()
        let s_57_4: () = ();
        // S s_57_5: call PhysicalCountInt(s_57_4)
        let s_57_5: u64 = PhysicalCountInt(state, tracer, s_57_4);
        // D s_57_6: cast zx s_57_3 -> bv
        let s_57_6: Bits = Bits::new(s_57_3 as u128, 64u16);
        // S s_57_7: cast zx s_57_5 -> bv
        let s_57_7: Bits = Bits::new(s_57_5 as u128, 64u16);
        // D s_57_8: sub s_57_6 s_57_7
        let s_57_8: Bits = ((s_57_6) - (s_57_7));
        // D s_57_9: cast reint s_57_8 -> u64
        let s_57_9: u64 = (s_57_8.value() as u64);
        // C s_57_10: const #0s : i
        let s_57_10: i128 = 0;
        // D s_57_11: cast zx s_57_9 -> bv
        let s_57_11: Bits = Bits::new(s_57_9 as u128, 64u16);
        // C s_57_12: const #1s : i64
        let s_57_12: i64 = 1;
        // C s_57_13: cast zx s_57_12 -> i
        let s_57_13: i128 = (i128::try_from(s_57_12).unwrap());
        // C s_57_14: const #31s : i
        let s_57_14: i128 = 31;
        // C s_57_15: add s_57_14 s_57_13
        let s_57_15: i128 = (s_57_14 + s_57_13);
        // D s_57_16: bit-extract s_57_11 s_57_10 s_57_15
        let s_57_16: Bits = (Bits::new(
            ((s_57_11) >> (s_57_10)).value(),
            u16::try_from(s_57_15).unwrap(),
        ));
        // D s_57_17: cast reint s_57_16 -> u32
        let s_57_17: u32 = (s_57_16.value() as u32);
        // D s_57_18: read-var t:i
        let s_57_18: i128 = fn_state.t;
        // D s_57_19: call R_set(s_57_18, s_57_17)
        let s_57_19: () = R_set(state, tracer, s_57_18, s_57_17);
        // N s_57_20: return
        return;
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #32s : i64
        let s_58_0: i64 = 32;
        // C s_58_1: cast zx s_58_0 -> i
        let s_58_1: i128 = (i128::try_from(s_58_0).unwrap());
        // S s_58_2: call __UNKNOWN_bits(s_58_1)
        let s_58_2: Bits = u__UNKNOWN_bits(state, tracer, s_58_1);
        // S s_58_3: cast reint s_58_2 -> u32
        let s_58_3: u32 = (s_58_2.value() as u32);
        // D s_58_4: read-var t:i
        let s_58_4: i128 = fn_state.t;
        // D s_58_5: call R_set(s_58_4, s_58_3)
        let s_58_5: () = R_set(state, tracer, s_58_4, s_58_3);
        // N s_58_6: return
        return;
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
        // D s_59_2: call ELUsingAArch32(s_59_1)
        let s_59_2: bool = ELUsingAArch32(state, tracer, s_59_1);
        // D s_59_3: write-var gs#107005 <= s_59_2
        fn_state.gs_107005 = s_59_2;
        // N s_59_4: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var __CNTP_CTL_ENABLE:u8
        let s_60_0: bool = fn_state.u__CNTP_CTL_ENABLE;
        // D s_60_1: cast zx s_60_0 -> bv
        let s_60_1: Bits = Bits::new(s_60_0 as u128, 1u16);
        // C s_60_2: const #0u : u8
        let s_60_2: bool = false;
        // C s_60_3: cast zx s_60_2 -> bv
        let s_60_3: Bits = Bits::new(s_60_2 as u128, 1u16);
        // D s_60_4: cmp-eq s_60_1 s_60_3
        let s_60_4: bool = ((s_60_1) == (s_60_3));
        // N s_60_5: branch s_60_4 b62 b61
        if s_60_4 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #() : ()
        let s_61_0: () = ();
        // S s_61_1: call CNTP_CVAL_read(s_61_0)
        let s_61_1: ProductType5c790c8ef59cc8b2 = CNTP_CVAL_read(state, tracer, s_61_0);
        // D s_61_2: write-var ga#166733 <= s_61_1
        fn_state.ga_166733 = s_61_1;
        // D s_61_3: read-var ga#166733.0:struct
        let s_61_3: u64 = fn_state.ga_166733._0;
        // C s_61_4: const #() : ()
        let s_61_4: () = ();
        // S s_61_5: call PhysicalCountInt(s_61_4)
        let s_61_5: u64 = PhysicalCountInt(state, tracer, s_61_4);
        // S s_61_6: cast zx s_61_5 -> bv
        let s_61_6: Bits = Bits::new(s_61_5 as u128, 64u16);
        // C s_61_7: const #14584u : u32
        let s_61_7: u32 = 14584;
        // D s_61_8: read-reg s_61_7:u64
        let s_61_8: u64 = {
            let value = state.read_register::<u64>(s_61_7 as isize);
            tracer.read_register(s_61_7 as isize, value);
            value
        };
        // D s_61_9: cast zx s_61_8 -> bv
        let s_61_9: Bits = Bits::new(s_61_8 as u128, 64u16);
        // D s_61_10: sub s_61_6 s_61_9
        let s_61_10: Bits = ((s_61_6) - (s_61_9));
        // D s_61_11: cast reint s_61_10 -> u64
        let s_61_11: u64 = (s_61_10.value() as u64);
        // D s_61_12: cast zx s_61_3 -> bv
        let s_61_12: Bits = Bits::new(s_61_3 as u128, 64u16);
        // D s_61_13: cast zx s_61_11 -> bv
        let s_61_13: Bits = Bits::new(s_61_11 as u128, 64u16);
        // D s_61_14: sub s_61_12 s_61_13
        let s_61_14: Bits = ((s_61_12) - (s_61_13));
        // D s_61_15: cast reint s_61_14 -> u64
        let s_61_15: u64 = (s_61_14.value() as u64);
        // C s_61_16: const #0s : i
        let s_61_16: i128 = 0;
        // D s_61_17: cast zx s_61_15 -> bv
        let s_61_17: Bits = Bits::new(s_61_15 as u128, 64u16);
        // C s_61_18: const #1s : i64
        let s_61_18: i64 = 1;
        // C s_61_19: cast zx s_61_18 -> i
        let s_61_19: i128 = (i128::try_from(s_61_18).unwrap());
        // C s_61_20: const #31s : i
        let s_61_20: i128 = 31;
        // C s_61_21: add s_61_20 s_61_19
        let s_61_21: i128 = (s_61_20 + s_61_19);
        // D s_61_22: bit-extract s_61_17 s_61_16 s_61_21
        let s_61_22: Bits = (Bits::new(
            ((s_61_17) >> (s_61_16)).value(),
            u16::try_from(s_61_21).unwrap(),
        ));
        // D s_61_23: cast reint s_61_22 -> u32
        let s_61_23: u32 = (s_61_22.value() as u32);
        // D s_61_24: read-var t:i
        let s_61_24: i128 = fn_state.t;
        // D s_61_25: call R_set(s_61_24, s_61_23)
        let s_61_25: () = R_set(state, tracer, s_61_24, s_61_23);
        // N s_61_26: return
        return;
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #32s : i64
        let s_62_0: i64 = 32;
        // C s_62_1: cast zx s_62_0 -> i
        let s_62_1: i128 = (i128::try_from(s_62_0).unwrap());
        // S s_62_2: call __UNKNOWN_bits(s_62_1)
        let s_62_2: Bits = u__UNKNOWN_bits(state, tracer, s_62_1);
        // S s_62_3: cast reint s_62_2 -> u32
        let s_62_3: u32 = (s_62_2.value() as u32);
        // D s_62_4: read-var t:i
        let s_62_4: i128 = fn_state.t;
        // D s_62_5: call R_set(s_62_4, s_62_3)
        let s_62_5: () = R_set(state, tracer, s_62_4, s_62_3);
        // N s_62_6: return
        return;
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var __CNTHCTL_EL2_ECV:u8
        let s_63_0: bool = fn_state.u__CNTHCTL_EL2_ECV;
        // D s_63_1: cast zx s_63_0 -> bv
        let s_63_1: Bits = Bits::new(s_63_0 as u128, 1u16);
        // C s_63_2: const #1u : u8
        let s_63_2: bool = true;
        // C s_63_3: cast zx s_63_2 -> bv
        let s_63_3: Bits = Bits::new(s_63_2 as u128, 1u16);
        // D s_63_4: cmp-eq s_63_1 s_63_3
        let s_63_4: bool = ((s_63_1) == (s_63_3));
        // D s_63_5: write-var gs#107004 <= s_63_4
        fn_state.gs_107004 = s_63_4;
        // N s_63_6: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var __SCR_EL3_ECVEn:u8
        let s_64_0: bool = fn_state.u__SCR_EL3_ECVEn;
        // D s_64_1: cast zx s_64_0 -> bv
        let s_64_1: Bits = Bits::new(s_64_0 as u128, 1u16);
        // C s_64_2: const #1u : u8
        let s_64_2: bool = true;
        // C s_64_3: cast zx s_64_2 -> bv
        let s_64_3: Bits = Bits::new(s_64_2 as u128, 1u16);
        // D s_64_4: cmp-eq s_64_1 s_64_3
        let s_64_4: bool = ((s_64_1) == (s_64_3));
        // D s_64_5: write-var gs#107003 <= s_64_4
        fn_state.gs_107003 = s_64_4;
        // N s_64_6: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #432u : u32
        let s_65_0: u32 = 432;
        // D s_65_1: read-reg s_65_0:u8
        let s_65_1: u8 = {
            let value = state.read_register::<u8>(s_65_0 as isize);
            tracer.read_register(s_65_0 as isize, value);
            value
        };
        // D s_65_2: call ELUsingAArch32(s_65_1)
        let s_65_2: bool = ELUsingAArch32(state, tracer, s_65_1);
        // D s_65_3: not s_65_2
        let s_65_3: bool = !s_65_2;
        // D s_65_4: write-var gs#107002 <= s_65_3
        fn_state.gs_107002 = s_65_3;
        // N s_65_5: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #() : ()
        let s_66_0: () = ();
        // S s_66_1: call EL2Enabled(s_66_0)
        let s_66_1: bool = EL2Enabled(state, tracer, s_66_0);
        // D s_66_2: write-var gs#107001 <= s_66_1
        fn_state.gs_107001 = s_66_1;
        // N s_66_3: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #3u : u8
        let s_67_0: u8 = 3;
        // C s_67_1: cast zx s_67_0 -> bv
        let s_67_1: Bits = Bits::new(s_67_0 as u128, 8u16);
        // C s_67_2: cast zx s_67_1 -> i
        let s_67_2: i128 = (s_67_1.value() as i128);
        // C s_67_3: cast reint s_67_2 -> i64
        let s_67_3: i64 = (s_67_2 as i64);
        // C s_67_4: cast zx s_67_3 -> i
        let s_67_4: i128 = (i128::try_from(s_67_3).unwrap());
        // S s_67_5: call AArch32_TakeHypTrapException(s_67_4)
        let s_67_5: () = AArch32_TakeHypTrapException(state, tracer, s_67_4);
        // N s_67_6: return
        return;
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var __CNTHCTL_PL1PCEN:u8
        let s_68_0: bool = fn_state.u__CNTHCTL_PL1PCEN;
        // D s_68_1: cast zx s_68_0 -> bv
        let s_68_1: Bits = Bits::new(s_68_0 as u128, 1u16);
        // C s_68_2: const #0u : u8
        let s_68_2: bool = false;
        // C s_68_3: cast zx s_68_2 -> bv
        let s_68_3: Bits = Bits::new(s_68_2 as u128, 1u16);
        // D s_68_4: cmp-eq s_68_1 s_68_3
        let s_68_4: bool = ((s_68_1) == (s_68_3));
        // D s_68_5: write-var gs#107000 <= s_68_4
        fn_state.gs_107000 = s_68_4;
        // N s_68_6: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #432u : u32
        let s_69_0: u32 = 432;
        // D s_69_1: read-reg s_69_0:u8
        let s_69_1: u8 = {
            let value = state.read_register::<u8>(s_69_0 as isize);
            tracer.read_register(s_69_0 as isize, value);
            value
        };
        // D s_69_2: call ELUsingAArch32(s_69_1)
        let s_69_2: bool = ELUsingAArch32(state, tracer, s_69_1);
        // D s_69_3: write-var gs#106999 <= s_69_2
        fn_state.gs_106999 = s_69_2;
        // N s_69_4: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #3u : u8
        let s_70_0: u8 = 3;
        // C s_70_1: cast zx s_70_0 -> bv
        let s_70_1: Bits = Bits::new(s_70_0 as u128, 8u16);
        // C s_70_2: cast zx s_70_1 -> i
        let s_70_2: i128 = (s_70_1.value() as i128);
        // C s_70_3: cast reint s_70_2 -> i64
        let s_70_3: i64 = (s_70_2 as i64);
        // C s_70_4: cast zx s_70_3 -> i
        let s_70_4: i128 = (i128::try_from(s_70_3).unwrap());
        // C s_70_5: const #432u : u32
        let s_70_5: u32 = 432;
        // D s_70_6: read-reg s_70_5:u8
        let s_70_6: u8 = {
            let value = state.read_register::<u8>(s_70_5 as isize);
            tracer.read_register(s_70_5 as isize, value);
            value
        };
        // D s_70_7: call AArch64_AArch32SystemAccessTrap(s_70_6, s_70_4)
        let s_70_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_70_6, s_70_4);
        // N s_70_8: return
        return;
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var __CNTHCTL_EL2_EL1PTEN:u8
        let s_71_0: bool = fn_state.u__CNTHCTL_EL2_EL1PTEN;
        // D s_71_1: cast zx s_71_0 -> bv
        let s_71_1: Bits = Bits::new(s_71_0 as u128, 1u16);
        // C s_71_2: const #0u : u8
        let s_71_2: bool = false;
        // C s_71_3: cast zx s_71_2 -> bv
        let s_71_3: Bits = Bits::new(s_71_2 as u128, 1u16);
        // D s_71_4: cmp-eq s_71_1 s_71_3
        let s_71_4: bool = ((s_71_1) == (s_71_3));
        // D s_71_5: write-var gs#106998 <= s_71_4
        fn_state.gs_106998 = s_71_4;
        // N s_71_6: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var __HCR_EL2_E2H:u8
        let s_72_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_72_1: cast zx s_72_0 -> bv
        let s_72_1: Bits = Bits::new(s_72_0 as u128, 1u16);
        // C s_72_2: const #1u : u8
        let s_72_2: bool = true;
        // C s_72_3: cast zx s_72_2 -> bv
        let s_72_3: Bits = Bits::new(s_72_2 as u128, 1u16);
        // D s_72_4: cmp-eq s_72_1 s_72_3
        let s_72_4: bool = ((s_72_1) == (s_72_3));
        // D s_72_5: write-var gs#106997 <= s_72_4
        fn_state.gs_106997 = s_72_4;
        // N s_72_6: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #432u : u32
        let s_73_0: u32 = 432;
        // D s_73_1: read-reg s_73_0:u8
        let s_73_1: u8 = {
            let value = state.read_register::<u8>(s_73_0 as isize);
            tracer.read_register(s_73_0 as isize, value);
            value
        };
        // D s_73_2: call ELUsingAArch32(s_73_1)
        let s_73_2: bool = ELUsingAArch32(state, tracer, s_73_1);
        // D s_73_3: not s_73_2
        let s_73_3: bool = !s_73_2;
        // D s_73_4: write-var gs#106996 <= s_73_3
        fn_state.gs_106996 = s_73_3;
        // N s_73_5: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #3u : u8
        let s_74_0: u8 = 3;
        // C s_74_1: cast zx s_74_0 -> bv
        let s_74_1: Bits = Bits::new(s_74_0 as u128, 8u16);
        // C s_74_2: cast zx s_74_1 -> i
        let s_74_2: i128 = (s_74_1.value() as i128);
        // C s_74_3: cast reint s_74_2 -> i64
        let s_74_3: i64 = (s_74_2 as i64);
        // C s_74_4: cast zx s_74_3 -> i
        let s_74_4: i128 = (i128::try_from(s_74_3).unwrap());
        // C s_74_5: const #432u : u32
        let s_74_5: u32 = 432;
        // D s_74_6: read-reg s_74_5:u8
        let s_74_6: u8 = {
            let value = state.read_register::<u8>(s_74_5 as isize);
            tracer.read_register(s_74_5 as isize, value);
            value
        };
        // D s_74_7: call AArch64_AArch32SystemAccessTrap(s_74_6, s_74_4)
        let s_74_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_74_6, s_74_4);
        // N s_74_8: return
        return;
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var __CNTHCTL_EL2_EL1PCEN:u8
        let s_75_0: bool = fn_state.u__CNTHCTL_EL2_EL1PCEN;
        // D s_75_1: cast zx s_75_0 -> bv
        let s_75_1: Bits = Bits::new(s_75_0 as u128, 1u16);
        // C s_75_2: const #0u : u8
        let s_75_2: bool = false;
        // C s_75_3: cast zx s_75_2 -> bv
        let s_75_3: Bits = Bits::new(s_75_2 as u128, 1u16);
        // D s_75_4: cmp-eq s_75_1 s_75_3
        let s_75_4: bool = ((s_75_1) == (s_75_3));
        // D s_75_5: write-var gs#106995 <= s_75_4
        fn_state.gs_106995 = s_75_4;
        // N s_75_6: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var __HCR_EL2_E2H:u8
        let s_76_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_76_1: cast zx s_76_0 -> bv
        let s_76_1: Bits = Bits::new(s_76_0 as u128, 1u16);
        // C s_76_2: const #0u : u8
        let s_76_2: bool = false;
        // C s_76_3: cast zx s_76_2 -> bv
        let s_76_3: Bits = Bits::new(s_76_2 as u128, 1u16);
        // D s_76_4: cmp-eq s_76_1 s_76_3
        let s_76_4: bool = ((s_76_1) == (s_76_3));
        // D s_76_5: write-var gs#106994 <= s_76_4
        fn_state.gs_106994 = s_76_4;
        // N s_76_6: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #432u : u32
        let s_77_0: u32 = 432;
        // D s_77_1: read-reg s_77_0:u8
        let s_77_1: u8 = {
            let value = state.read_register::<u8>(s_77_0 as isize);
            tracer.read_register(s_77_0 as isize, value);
            value
        };
        // D s_77_2: call ELUsingAArch32(s_77_1)
        let s_77_2: bool = ELUsingAArch32(state, tracer, s_77_1);
        // D s_77_3: not s_77_2
        let s_77_3: bool = !s_77_2;
        // D s_77_4: write-var gs#106993 <= s_77_3
        fn_state.gs_106993 = s_77_3;
        // N s_77_5: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #440u : u32
        let s_78_0: u32 = 440;
        // D s_78_1: read-reg s_78_0:u8
        let s_78_1: u8 = {
            let value = state.read_register::<u8>(s_78_0 as isize);
            tracer.read_register(s_78_0 as isize, value);
            value
        };
        // D s_78_2: call ELUsingAArch32(s_78_1)
        let s_78_2: bool = ELUsingAArch32(state, tracer, s_78_1);
        // D s_78_3: not s_78_2
        let s_78_3: bool = !s_78_2;
        // N s_78_4: branch s_78_3 b217 b79
        if s_78_3 {
            return block_217(state, tracer, fn_state);
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
        // D s_79_1: write-var gs#107016 <= s_79_0
        fn_state.gs_107016 = s_79_0;
        // N s_79_2: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var gs#107016:u8
        let s_80_0: bool = fn_state.gs_107016;
        // N s_80_1: branch s_80_0 b216 b81
        if s_80_0 {
            return block_216(state, tracer, fn_state);
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
        // D s_81_1: write-var gs#107017 <= s_81_0
        fn_state.gs_107017 = s_81_0;
        // N s_81_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var gs#107017:u8
        let s_82_0: bool = fn_state.gs_107017;
        // N s_82_1: branch s_82_0 b207 b83
        if s_82_0 {
            return block_207(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #440u : u32
        let s_83_0: u32 = 440;
        // D s_83_1: read-reg s_83_0:u8
        let s_83_1: u8 = {
            let value = state.read_register::<u8>(s_83_0 as isize);
            tracer.read_register(s_83_0 as isize, value);
            value
        };
        // D s_83_2: call ELUsingAArch32(s_83_1)
        let s_83_2: bool = ELUsingAArch32(state, tracer, s_83_1);
        // N s_83_3: branch s_83_2 b206 b84
        if s_83_2 {
            return block_206(state, tracer, fn_state);
        } else {
            return block_84(state, tracer, fn_state);
        };
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #0u : u8
        let s_84_0: bool = false;
        // D s_84_1: write-var gs#107018 <= s_84_0
        fn_state.gs_107018 = s_84_0;
        // N s_84_2: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var gs#107018:u8
        let s_85_0: bool = fn_state.gs_107018;
        // N s_85_1: branch s_85_0 b189 b86
        if s_85_0 {
            return block_189(state, tracer, fn_state);
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
        // S s_86_1: call EL2Enabled(s_86_0)
        let s_86_1: bool = EL2Enabled(state, tracer, s_86_0);
        // N s_86_2: branch s_86_1 b188 b87
        if s_86_1 {
            return block_188(state, tracer, fn_state);
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
        // D s_87_1: write-var gs#107019 <= s_87_0
        fn_state.gs_107019 = s_87_0;
        // N s_87_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var gs#107019:u8
        let s_88_0: bool = fn_state.gs_107019;
        // N s_88_1: branch s_88_0 b187 b89
        if s_88_0 {
            return block_187(state, tracer, fn_state);
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
        // D s_89_1: write-var gs#107020 <= s_89_0
        fn_state.gs_107020 = s_89_0;
        // N s_89_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var gs#107020:u8
        let s_90_0: bool = fn_state.gs_107020;
        // N s_90_1: branch s_90_0 b186 b91
        if s_90_0 {
            return block_186(state, tracer, fn_state);
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
        // D s_91_1: write-var gs#107021 <= s_91_0
        fn_state.gs_107021 = s_91_0;
        // N s_91_2: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var gs#107021:u8
        let s_92_0: bool = fn_state.gs_107021;
        // N s_92_1: branch s_92_0 b185 b93
        if s_92_0 {
            return block_185(state, tracer, fn_state);
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
        // N s_93_2: branch s_93_1 b184 b94
        if s_93_1 {
            return block_184(state, tracer, fn_state);
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
        // D s_94_1: write-var gs#107022 <= s_94_0
        fn_state.gs_107022 = s_94_0;
        // N s_94_2: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_95_0: read-var gs#107022:u8
        let s_95_0: bool = fn_state.gs_107022;
        // N s_95_1: branch s_95_0 b183 b96
        if s_95_0 {
            return block_183(state, tracer, fn_state);
        } else {
            return block_96(state, tracer, fn_state);
        };
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #0u : u8
        let s_96_0: bool = false;
        // D s_96_1: write-var gs#107023 <= s_96_0
        fn_state.gs_107023 = s_96_0;
        // N s_96_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var gs#107023:u8
        let s_97_0: bool = fn_state.gs_107023;
        // N s_97_1: branch s_97_0 b182 b98
        if s_97_0 {
            return block_182(state, tracer, fn_state);
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
        // D s_98_1: write-var gs#107024 <= s_98_0
        fn_state.gs_107024 = s_98_0;
        // N s_98_2: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_99_0: read-var gs#107024:u8
        let s_99_0: bool = fn_state.gs_107024;
        // N s_99_1: branch s_99_0 b181 b100
        if s_99_0 {
            return block_181(state, tracer, fn_state);
        } else {
            return block_100(state, tracer, fn_state);
        };
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #() : ()
        let s_100_0: () = ();
        // S s_100_1: call EL2Enabled(s_100_0)
        let s_100_1: bool = EL2Enabled(state, tracer, s_100_0);
        // N s_100_2: branch s_100_1 b180 b101
        if s_100_1 {
            return block_180(state, tracer, fn_state);
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
        // D s_101_1: write-var gs#107025 <= s_101_0
        fn_state.gs_107025 = s_101_0;
        // N s_101_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var gs#107025:u8
        let s_102_0: bool = fn_state.gs_107025;
        // N s_102_1: branch s_102_0 b179 b103
        if s_102_0 {
            return block_179(state, tracer, fn_state);
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
        // D s_103_1: write-var gs#107026 <= s_103_0
        fn_state.gs_107026 = s_103_0;
        // N s_103_2: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_104_0: read-var gs#107026:u8
        let s_104_0: bool = fn_state.gs_107026;
        // N s_104_1: branch s_104_0 b178 b105
        if s_104_0 {
            return block_178(state, tracer, fn_state);
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
        // D s_105_1: write-var gs#107027 <= s_105_0
        fn_state.gs_107027 = s_105_0;
        // N s_105_2: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var gs#107027:u8
        let s_106_0: bool = fn_state.gs_107027;
        // N s_106_1: branch s_106_0 b177 b107
        if s_106_0 {
            return block_177(state, tracer, fn_state);
        } else {
            return block_107(state, tracer, fn_state);
        };
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #() : ()
        let s_107_0: () = ();
        // S s_107_1: call EL2Enabled(s_107_0)
        let s_107_1: bool = EL2Enabled(state, tracer, s_107_0);
        // N s_107_2: branch s_107_1 b176 b108
        if s_107_1 {
            return block_176(state, tracer, fn_state);
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
        // D s_108_1: write-var gs#107028 <= s_108_0
        fn_state.gs_107028 = s_108_0;
        // N s_108_2: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_109_0: read-var gs#107028:u8
        let s_109_0: bool = fn_state.gs_107028;
        // N s_109_1: branch s_109_0 b175 b110
        if s_109_0 {
            return block_175(state, tracer, fn_state);
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
        // D s_110_1: write-var gs#107029 <= s_110_0
        fn_state.gs_107029 = s_110_0;
        // N s_110_2: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var gs#107029:u8
        let s_111_0: bool = fn_state.gs_107029;
        // N s_111_1: branch s_111_0 b174 b112
        if s_111_0 {
            return block_174(state, tracer, fn_state);
        } else {
            return block_112(state, tracer, fn_state);
        };
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_112_0: const #() : ()
        let s_112_0: () = ();
        // S s_112_1: call EL2Enabled(s_112_0)
        let s_112_1: bool = EL2Enabled(state, tracer, s_112_0);
        // N s_112_2: branch s_112_1 b173 b113
        if s_112_1 {
            return block_173(state, tracer, fn_state);
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
        // D s_113_1: write-var gs#107030 <= s_113_0
        fn_state.gs_107030 = s_113_0;
        // N s_113_2: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_114_0: read-var gs#107030:u8
        let s_114_0: bool = fn_state.gs_107030;
        // N s_114_1: branch s_114_0 b172 b115
        if s_114_0 {
            return block_172(state, tracer, fn_state);
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
        // D s_115_1: write-var gs#107031 <= s_115_0
        fn_state.gs_107031 = s_115_0;
        // N s_115_2: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_116_0: read-var gs#107031:u8
        let s_116_0: bool = fn_state.gs_107031;
        // N s_116_1: branch s_116_0 b171 b117
        if s_116_0 {
            return block_171(state, tracer, fn_state);
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
        // D s_117_1: write-var gs#107032 <= s_117_0
        fn_state.gs_107032 = s_117_0;
        // N s_117_2: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_118_0: read-var gs#107032:u8
        let s_118_0: bool = fn_state.gs_107032;
        // N s_118_1: branch s_118_0 b170 b119
        if s_118_0 {
            return block_170(state, tracer, fn_state);
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
        // D s_119_1: write-var gs#107033 <= s_119_0
        fn_state.gs_107033 = s_119_0;
        // N s_119_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_120_0: read-var gs#107033:u8
        let s_120_0: bool = fn_state.gs_107033;
        // N s_120_1: branch s_120_0 b167 b121
        if s_120_0 {
            return block_167(state, tracer, fn_state);
        } else {
            return block_121(state, tracer, fn_state);
        };
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #() : ()
        let s_121_0: () = ();
        // S s_121_1: call EL2Enabled(s_121_0)
        let s_121_1: bool = EL2Enabled(state, tracer, s_121_0);
        // N s_121_2: branch s_121_1 b166 b122
        if s_121_1 {
            return block_166(state, tracer, fn_state);
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
        // D s_122_1: write-var gs#107034 <= s_122_0
        fn_state.gs_107034 = s_122_0;
        // N s_122_2: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_123_0: read-var gs#107034:u8
        let s_123_0: bool = fn_state.gs_107034;
        // N s_123_1: branch s_123_0 b165 b124
        if s_123_0 {
            return block_165(state, tracer, fn_state);
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
        // D s_124_1: write-var gs#107035 <= s_124_0
        fn_state.gs_107035 = s_124_0;
        // N s_124_2: jump b125
        return block_125(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_125_0: read-var gs#107035:u8
        let s_125_0: bool = fn_state.gs_107035;
        // N s_125_1: branch s_125_0 b164 b126
        if s_125_0 {
            return block_164(state, tracer, fn_state);
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
        // D s_126_1: write-var gs#107036 <= s_126_0
        fn_state.gs_107036 = s_126_0;
        // N s_126_2: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_127_0: read-var gs#107036:u8
        let s_127_0: bool = fn_state.gs_107036;
        // N s_127_1: branch s_127_0 b161 b128
        if s_127_0 {
            return block_161(state, tracer, fn_state);
        } else {
            return block_128(state, tracer, fn_state);
        };
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_128_0: const #145u : u32
        let s_128_0: u32 = 145;
        // S s_128_1: call IsFeatureImplemented(s_128_0)
        let s_128_1: bool = IsFeatureImplemented(state, tracer, s_128_0);
        // N s_128_2: branch s_128_1 b160 b129
        if s_128_1 {
            return block_160(state, tracer, fn_state);
        } else {
            return block_129(state, tracer, fn_state);
        };
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_129_0: const #0u : u8
        let s_129_0: bool = false;
        // D s_129_1: write-var gs#107037 <= s_129_0
        fn_state.gs_107037 = s_129_0;
        // N s_129_2: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_130_0: read-var gs#107037:u8
        let s_130_0: bool = fn_state.gs_107037;
        // N s_130_1: branch s_130_0 b159 b131
        if s_130_0 {
            return block_159(state, tracer, fn_state);
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
        // D s_131_1: write-var gs#107038 <= s_131_0
        fn_state.gs_107038 = s_131_0;
        // N s_131_2: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_132_0: read-var gs#107038:u8
        let s_132_0: bool = fn_state.gs_107038;
        // N s_132_1: branch s_132_0 b158 b133
        if s_132_0 {
            return block_158(state, tracer, fn_state);
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
        // D s_133_1: write-var gs#107039 <= s_133_0
        fn_state.gs_107039 = s_133_0;
        // N s_133_2: jump b134
        return block_134(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_134_0: read-var gs#107039:u8
        let s_134_0: bool = fn_state.gs_107039;
        // N s_134_1: branch s_134_0 b157 b135
        if s_134_0 {
            return block_157(state, tracer, fn_state);
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
        // D s_135_1: write-var gs#107040 <= s_135_0
        fn_state.gs_107040 = s_135_0;
        // N s_135_2: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_136_0: read-var gs#107040:u8
        let s_136_0: bool = fn_state.gs_107040;
        // N s_136_1: branch s_136_0 b156 b137
        if s_136_0 {
            return block_156(state, tracer, fn_state);
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
        // D s_137_1: write-var gs#107041 <= s_137_0
        fn_state.gs_107041 = s_137_0;
        // N s_137_2: jump b138
        return block_138(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_138_0: read-var gs#107041:u8
        let s_138_0: bool = fn_state.gs_107041;
        // N s_138_1: branch s_138_0 b153 b139
        if s_138_0 {
            return block_153(state, tracer, fn_state);
        } else {
            return block_139(state, tracer, fn_state);
        };
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_139_0: const #424u : u32
        let s_139_0: u32 = 424;
        // D s_139_1: read-reg s_139_0:u8
        let s_139_1: u8 = {
            let value = state.read_register::<u8>(s_139_0 as isize);
            tracer.read_register(s_139_0 as isize, value);
            value
        };
        // C s_139_2: const #2u : u8
        let s_139_2: u8 = 2;
        // D s_139_3: cmp-lt s_139_1 s_139_2
        let s_139_3: bool = ((s_139_1) < (s_139_2));
        // N s_139_4: branch s_139_3 b152 b140
        if s_139_3 {
            return block_152(state, tracer, fn_state);
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
        // D s_140_1: write-var gs#107042 <= s_140_0
        fn_state.gs_107042 = s_140_0;
        // N s_140_2: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_141_0: read-var gs#107042:u8
        let s_141_0: bool = fn_state.gs_107042;
        // N s_141_1: branch s_141_0 b145 b142
        if s_141_0 {
            return block_145(state, tracer, fn_state);
        } else {
            return block_142(state, tracer, fn_state);
        };
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_142_0: read-var __CNTP_CTL_ENABLE:u8
        let s_142_0: bool = fn_state.u__CNTP_CTL_ENABLE;
        // D s_142_1: cast zx s_142_0 -> bv
        let s_142_1: Bits = Bits::new(s_142_0 as u128, 1u16);
        // C s_142_2: const #0u : u8
        let s_142_2: bool = false;
        // C s_142_3: cast zx s_142_2 -> bv
        let s_142_3: Bits = Bits::new(s_142_2 as u128, 1u16);
        // D s_142_4: cmp-eq s_142_1 s_142_3
        let s_142_4: bool = ((s_142_1) == (s_142_3));
        // N s_142_5: branch s_142_4 b144 b143
        if s_142_4 {
            return block_144(state, tracer, fn_state);
        } else {
            return block_143(state, tracer, fn_state);
        };
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_143_0: const #() : ()
        let s_143_0: () = ();
        // S s_143_1: call CNTP_CVAL_read(s_143_0)
        let s_143_1: ProductType5c790c8ef59cc8b2 = CNTP_CVAL_read(
            state,
            tracer,
            s_143_0,
        );
        // D s_143_2: write-var ga#166702 <= s_143_1
        fn_state.ga_166702 = s_143_1;
        // D s_143_3: read-var ga#166702.0:struct
        let s_143_3: u64 = fn_state.ga_166702._0;
        // C s_143_4: const #() : ()
        let s_143_4: () = ();
        // S s_143_5: call PhysicalCountInt(s_143_4)
        let s_143_5: u64 = PhysicalCountInt(state, tracer, s_143_4);
        // D s_143_6: cast zx s_143_3 -> bv
        let s_143_6: Bits = Bits::new(s_143_3 as u128, 64u16);
        // S s_143_7: cast zx s_143_5 -> bv
        let s_143_7: Bits = Bits::new(s_143_5 as u128, 64u16);
        // D s_143_8: sub s_143_6 s_143_7
        let s_143_8: Bits = ((s_143_6) - (s_143_7));
        // D s_143_9: cast reint s_143_8 -> u64
        let s_143_9: u64 = (s_143_8.value() as u64);
        // C s_143_10: const #0s : i
        let s_143_10: i128 = 0;
        // D s_143_11: cast zx s_143_9 -> bv
        let s_143_11: Bits = Bits::new(s_143_9 as u128, 64u16);
        // C s_143_12: const #1s : i64
        let s_143_12: i64 = 1;
        // C s_143_13: cast zx s_143_12 -> i
        let s_143_13: i128 = (i128::try_from(s_143_12).unwrap());
        // C s_143_14: const #31s : i
        let s_143_14: i128 = 31;
        // C s_143_15: add s_143_14 s_143_13
        let s_143_15: i128 = (s_143_14 + s_143_13);
        // D s_143_16: bit-extract s_143_11 s_143_10 s_143_15
        let s_143_16: Bits = (Bits::new(
            ((s_143_11) >> (s_143_10)).value(),
            u16::try_from(s_143_15).unwrap(),
        ));
        // D s_143_17: cast reint s_143_16 -> u32
        let s_143_17: u32 = (s_143_16.value() as u32);
        // D s_143_18: read-var t:i
        let s_143_18: i128 = fn_state.t;
        // D s_143_19: call R_set(s_143_18, s_143_17)
        let s_143_19: () = R_set(state, tracer, s_143_18, s_143_17);
        // N s_143_20: return
        return;
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_144_0: const #32s : i64
        let s_144_0: i64 = 32;
        // C s_144_1: cast zx s_144_0 -> i
        let s_144_1: i128 = (i128::try_from(s_144_0).unwrap());
        // S s_144_2: call __UNKNOWN_bits(s_144_1)
        let s_144_2: Bits = u__UNKNOWN_bits(state, tracer, s_144_1);
        // S s_144_3: cast reint s_144_2 -> u32
        let s_144_3: u32 = (s_144_2.value() as u32);
        // D s_144_4: read-var t:i
        let s_144_4: i128 = fn_state.t;
        // D s_144_5: call R_set(s_144_4, s_144_3)
        let s_144_5: () = R_set(state, tracer, s_144_4, s_144_3);
        // N s_144_6: return
        return;
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_145_0: read-var __SCR_NS:u8
        let s_145_0: bool = fn_state.u__SCR_NS;
        // D s_145_1: cast zx s_145_0 -> bv
        let s_145_1: Bits = Bits::new(s_145_0 as u128, 1u16);
        // C s_145_2: const #1u : u8
        let s_145_2: bool = true;
        // C s_145_3: cast zx s_145_2 -> bv
        let s_145_3: Bits = Bits::new(s_145_2 as u128, 1u16);
        // D s_145_4: cmp-eq s_145_1 s_145_3
        let s_145_4: bool = ((s_145_1) == (s_145_3));
        // N s_145_5: branch s_145_4 b149 b146
        if s_145_4 {
            return block_149(state, tracer, fn_state);
        } else {
            return block_146(state, tracer, fn_state);
        };
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_146_0: read-var __CNTP_CTL_S_ENABLE:u8
        let s_146_0: bool = fn_state.u__CNTP_CTL_S_ENABLE;
        // D s_146_1: cast zx s_146_0 -> bv
        let s_146_1: Bits = Bits::new(s_146_0 as u128, 1u16);
        // C s_146_2: const #0u : u8
        let s_146_2: bool = false;
        // C s_146_3: cast zx s_146_2 -> bv
        let s_146_3: Bits = Bits::new(s_146_2 as u128, 1u16);
        // D s_146_4: cmp-eq s_146_1 s_146_3
        let s_146_4: bool = ((s_146_1) == (s_146_3));
        // N s_146_5: branch s_146_4 b148 b147
        if s_146_4 {
            return block_148(state, tracer, fn_state);
        } else {
            return block_147(state, tracer, fn_state);
        };
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_147_0: const #16848u : u32
        let s_147_0: u32 = 16848;
        // D s_147_1: read-reg s_147_0:u64
        let s_147_1: u64 = {
            let value = state.read_register::<u64>(s_147_0 as isize);
            tracer.read_register(s_147_0 as isize, value);
            value
        };
        // C s_147_2: const #() : ()
        let s_147_2: () = ();
        // S s_147_3: call PhysicalCountInt(s_147_2)
        let s_147_3: u64 = PhysicalCountInt(state, tracer, s_147_2);
        // D s_147_4: cast zx s_147_1 -> bv
        let s_147_4: Bits = Bits::new(s_147_1 as u128, 64u16);
        // S s_147_5: cast zx s_147_3 -> bv
        let s_147_5: Bits = Bits::new(s_147_3 as u128, 64u16);
        // D s_147_6: sub s_147_4 s_147_5
        let s_147_6: Bits = ((s_147_4) - (s_147_5));
        // D s_147_7: cast reint s_147_6 -> u64
        let s_147_7: u64 = (s_147_6.value() as u64);
        // C s_147_8: const #0s : i
        let s_147_8: i128 = 0;
        // D s_147_9: cast zx s_147_7 -> bv
        let s_147_9: Bits = Bits::new(s_147_7 as u128, 64u16);
        // C s_147_10: const #1s : i64
        let s_147_10: i64 = 1;
        // C s_147_11: cast zx s_147_10 -> i
        let s_147_11: i128 = (i128::try_from(s_147_10).unwrap());
        // C s_147_12: const #31s : i
        let s_147_12: i128 = 31;
        // C s_147_13: add s_147_12 s_147_11
        let s_147_13: i128 = (s_147_12 + s_147_11);
        // D s_147_14: bit-extract s_147_9 s_147_8 s_147_13
        let s_147_14: Bits = (Bits::new(
            ((s_147_9) >> (s_147_8)).value(),
            u16::try_from(s_147_13).unwrap(),
        ));
        // D s_147_15: cast reint s_147_14 -> u32
        let s_147_15: u32 = (s_147_14.value() as u32);
        // D s_147_16: read-var t:i
        let s_147_16: i128 = fn_state.t;
        // D s_147_17: call R_set(s_147_16, s_147_15)
        let s_147_17: () = R_set(state, tracer, s_147_16, s_147_15);
        // N s_147_18: return
        return;
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_148_0: const #32s : i64
        let s_148_0: i64 = 32;
        // C s_148_1: cast zx s_148_0 -> i
        let s_148_1: i128 = (i128::try_from(s_148_0).unwrap());
        // S s_148_2: call __UNKNOWN_bits(s_148_1)
        let s_148_2: Bits = u__UNKNOWN_bits(state, tracer, s_148_1);
        // S s_148_3: cast reint s_148_2 -> u32
        let s_148_3: u32 = (s_148_2.value() as u32);
        // D s_148_4: read-var t:i
        let s_148_4: i128 = fn_state.t;
        // D s_148_5: call R_set(s_148_4, s_148_3)
        let s_148_5: () = R_set(state, tracer, s_148_4, s_148_3);
        // N s_148_6: return
        return;
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_149_0: read-var __CNTP_CTL_NS_ENABLE:u8
        let s_149_0: bool = fn_state.u__CNTP_CTL_NS_ENABLE;
        // D s_149_1: cast zx s_149_0 -> bv
        let s_149_1: Bits = Bits::new(s_149_0 as u128, 1u16);
        // C s_149_2: const #0u : u8
        let s_149_2: bool = false;
        // C s_149_3: cast zx s_149_2 -> bv
        let s_149_3: Bits = Bits::new(s_149_2 as u128, 1u16);
        // D s_149_4: cmp-eq s_149_1 s_149_3
        let s_149_4: bool = ((s_149_1) == (s_149_3));
        // N s_149_5: branch s_149_4 b151 b150
        if s_149_4 {
            return block_151(state, tracer, fn_state);
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
        // S s_150_1: call CNTP_CVAL_NS_read(s_150_0)
        let s_150_1: ProductType5c790c8ef59cc8b2 = CNTP_CVAL_NS_read(
            state,
            tracer,
            s_150_0,
        );
        // D s_150_2: write-var ga#166687 <= s_150_1
        fn_state.ga_166687 = s_150_1;
        // D s_150_3: read-var ga#166687.0:struct
        let s_150_3: u64 = fn_state.ga_166687._0;
        // C s_150_4: const #() : ()
        let s_150_4: () = ();
        // S s_150_5: call PhysicalCountInt(s_150_4)
        let s_150_5: u64 = PhysicalCountInt(state, tracer, s_150_4);
        // D s_150_6: cast zx s_150_3 -> bv
        let s_150_6: Bits = Bits::new(s_150_3 as u128, 64u16);
        // S s_150_7: cast zx s_150_5 -> bv
        let s_150_7: Bits = Bits::new(s_150_5 as u128, 64u16);
        // D s_150_8: sub s_150_6 s_150_7
        let s_150_8: Bits = ((s_150_6) - (s_150_7));
        // D s_150_9: cast reint s_150_8 -> u64
        let s_150_9: u64 = (s_150_8.value() as u64);
        // C s_150_10: const #0s : i
        let s_150_10: i128 = 0;
        // D s_150_11: cast zx s_150_9 -> bv
        let s_150_11: Bits = Bits::new(s_150_9 as u128, 64u16);
        // C s_150_12: const #1s : i64
        let s_150_12: i64 = 1;
        // C s_150_13: cast zx s_150_12 -> i
        let s_150_13: i128 = (i128::try_from(s_150_12).unwrap());
        // C s_150_14: const #31s : i
        let s_150_14: i128 = 31;
        // C s_150_15: add s_150_14 s_150_13
        let s_150_15: i128 = (s_150_14 + s_150_13);
        // D s_150_16: bit-extract s_150_11 s_150_10 s_150_15
        let s_150_16: Bits = (Bits::new(
            ((s_150_11) >> (s_150_10)).value(),
            u16::try_from(s_150_15).unwrap(),
        ));
        // D s_150_17: cast reint s_150_16 -> u32
        let s_150_17: u32 = (s_150_16.value() as u32);
        // D s_150_18: read-var t:i
        let s_150_18: i128 = fn_state.t;
        // D s_150_19: call R_set(s_150_18, s_150_17)
        let s_150_19: () = R_set(state, tracer, s_150_18, s_150_17);
        // N s_150_20: return
        return;
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_151_0: const #32s : i64
        let s_151_0: i64 = 32;
        // C s_151_1: cast zx s_151_0 -> i
        let s_151_1: i128 = (i128::try_from(s_151_0).unwrap());
        // S s_151_2: call __UNKNOWN_bits(s_151_1)
        let s_151_2: Bits = u__UNKNOWN_bits(state, tracer, s_151_1);
        // S s_151_3: cast reint s_151_2 -> u32
        let s_151_3: u32 = (s_151_2.value() as u32);
        // D s_151_4: read-var t:i
        let s_151_4: i128 = fn_state.t;
        // D s_151_5: call R_set(s_151_4, s_151_3)
        let s_151_5: () = R_set(state, tracer, s_151_4, s_151_3);
        // N s_151_6: return
        return;
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_152_0: const #424u : u32
        let s_152_0: u32 = 424;
        // D s_152_1: read-reg s_152_0:u8
        let s_152_1: u8 = {
            let value = state.read_register::<u8>(s_152_0 as isize);
            tracer.read_register(s_152_0 as isize, value);
            value
        };
        // D s_152_2: call ELUsingAArch32(s_152_1)
        let s_152_2: bool = ELUsingAArch32(state, tracer, s_152_1);
        // D s_152_3: write-var gs#107042 <= s_152_2
        fn_state.gs_107042 = s_152_2;
        // N s_152_4: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_153_0: read-var __CNTP_CTL_ENABLE:u8
        let s_153_0: bool = fn_state.u__CNTP_CTL_ENABLE;
        // D s_153_1: cast zx s_153_0 -> bv
        let s_153_1: Bits = Bits::new(s_153_0 as u128, 1u16);
        // C s_153_2: const #0u : u8
        let s_153_2: bool = false;
        // C s_153_3: cast zx s_153_2 -> bv
        let s_153_3: Bits = Bits::new(s_153_2 as u128, 1u16);
        // D s_153_4: cmp-eq s_153_1 s_153_3
        let s_153_4: bool = ((s_153_1) == (s_153_3));
        // N s_153_5: branch s_153_4 b155 b154
        if s_153_4 {
            return block_155(state, tracer, fn_state);
        } else {
            return block_154(state, tracer, fn_state);
        };
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_154_0: const #() : ()
        let s_154_0: () = ();
        // S s_154_1: call CNTP_CVAL_read(s_154_0)
        let s_154_1: ProductType5c790c8ef59cc8b2 = CNTP_CVAL_read(
            state,
            tracer,
            s_154_0,
        );
        // D s_154_2: write-var ga#166675 <= s_154_1
        fn_state.ga_166675 = s_154_1;
        // D s_154_3: read-var ga#166675.0:struct
        let s_154_3: u64 = fn_state.ga_166675._0;
        // C s_154_4: const #() : ()
        let s_154_4: () = ();
        // S s_154_5: call PhysicalCountInt(s_154_4)
        let s_154_5: u64 = PhysicalCountInt(state, tracer, s_154_4);
        // S s_154_6: cast zx s_154_5 -> bv
        let s_154_6: Bits = Bits::new(s_154_5 as u128, 64u16);
        // C s_154_7: const #14584u : u32
        let s_154_7: u32 = 14584;
        // D s_154_8: read-reg s_154_7:u64
        let s_154_8: u64 = {
            let value = state.read_register::<u64>(s_154_7 as isize);
            tracer.read_register(s_154_7 as isize, value);
            value
        };
        // D s_154_9: cast zx s_154_8 -> bv
        let s_154_9: Bits = Bits::new(s_154_8 as u128, 64u16);
        // D s_154_10: sub s_154_6 s_154_9
        let s_154_10: Bits = ((s_154_6) - (s_154_9));
        // D s_154_11: cast reint s_154_10 -> u64
        let s_154_11: u64 = (s_154_10.value() as u64);
        // D s_154_12: cast zx s_154_3 -> bv
        let s_154_12: Bits = Bits::new(s_154_3 as u128, 64u16);
        // D s_154_13: cast zx s_154_11 -> bv
        let s_154_13: Bits = Bits::new(s_154_11 as u128, 64u16);
        // D s_154_14: sub s_154_12 s_154_13
        let s_154_14: Bits = ((s_154_12) - (s_154_13));
        // D s_154_15: cast reint s_154_14 -> u64
        let s_154_15: u64 = (s_154_14.value() as u64);
        // C s_154_16: const #0s : i
        let s_154_16: i128 = 0;
        // D s_154_17: cast zx s_154_15 -> bv
        let s_154_17: Bits = Bits::new(s_154_15 as u128, 64u16);
        // C s_154_18: const #1s : i64
        let s_154_18: i64 = 1;
        // C s_154_19: cast zx s_154_18 -> i
        let s_154_19: i128 = (i128::try_from(s_154_18).unwrap());
        // C s_154_20: const #31s : i
        let s_154_20: i128 = 31;
        // C s_154_21: add s_154_20 s_154_19
        let s_154_21: i128 = (s_154_20 + s_154_19);
        // D s_154_22: bit-extract s_154_17 s_154_16 s_154_21
        let s_154_22: Bits = (Bits::new(
            ((s_154_17) >> (s_154_16)).value(),
            u16::try_from(s_154_21).unwrap(),
        ));
        // D s_154_23: cast reint s_154_22 -> u32
        let s_154_23: u32 = (s_154_22.value() as u32);
        // D s_154_24: read-var t:i
        let s_154_24: i128 = fn_state.t;
        // D s_154_25: call R_set(s_154_24, s_154_23)
        let s_154_25: () = R_set(state, tracer, s_154_24, s_154_23);
        // N s_154_26: return
        return;
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_155_0: const #32s : i64
        let s_155_0: i64 = 32;
        // C s_155_1: cast zx s_155_0 -> i
        let s_155_1: i128 = (i128::try_from(s_155_0).unwrap());
        // S s_155_2: call __UNKNOWN_bits(s_155_1)
        let s_155_2: Bits = u__UNKNOWN_bits(state, tracer, s_155_1);
        // S s_155_3: cast reint s_155_2 -> u32
        let s_155_3: u32 = (s_155_2.value() as u32);
        // D s_155_4: read-var t:i
        let s_155_4: i128 = fn_state.t;
        // D s_155_5: call R_set(s_155_4, s_155_3)
        let s_155_5: () = R_set(state, tracer, s_155_4, s_155_3);
        // N s_155_6: return
        return;
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_156_0: const #102552u : u32
        let s_156_0: u32 = 102552;
        // D s_156_1: read-reg s_156_0:struct
        let s_156_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_156_0 as isize);
            tracer.read_register(s_156_0 as isize, value);
            value
        };
        // D s_156_2: call _get_HCR_EL2_Type_E2H(s_156_1)
        let s_156_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_156_1);
        // C s_156_3: const #102552u : u32
        let s_156_3: u32 = 102552;
        // D s_156_4: read-reg s_156_3:struct
        let s_156_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_156_3 as isize);
            tracer.read_register(s_156_3 as isize, value);
            value
        };
        // D s_156_5: call _get_HCR_EL2_Type_TGE(s_156_4)
        let s_156_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_156_4);
        // D s_156_6: cast zx s_156_2 -> bv
        let s_156_6: Bits = Bits::new(s_156_2 as u128, 1u16);
        // D s_156_7: cast zx s_156_5 -> bv
        let s_156_7: Bits = Bits::new(s_156_5 as u128, 1u16);
        // D s_156_8: cast reint s_156_6 -> u128
        let s_156_8: u128 = (s_156_6.value() as u128);
        // D s_156_9: size-of s_156_6
        let s_156_9: u16 = s_156_6.length();
        // D s_156_10: cast reint s_156_7 -> u128
        let s_156_10: u128 = (s_156_7.value() as u128);
        // D s_156_11: size-of s_156_7
        let s_156_11: u16 = s_156_7.length();
        // D s_156_12: lsl s_156_8 s_156_11
        let s_156_12: u128 = s_156_8 << s_156_11;
        // D s_156_13: or s_156_12 s_156_10
        let s_156_13: u128 = ((s_156_12) | (s_156_10));
        // D s_156_14: add s_156_9 s_156_11
        let s_156_14: u16 = (s_156_9 + s_156_11);
        // D s_156_15: create-bits s_156_13 s_156_14
        let s_156_15: Bits = Bits::new(s_156_13, s_156_14);
        // D s_156_16: cast reint s_156_15 -> u8
        let s_156_16: u8 = (s_156_15.value() as u8);
        // D s_156_17: cast zx s_156_16 -> bv
        let s_156_17: Bits = Bits::new(s_156_16 as u128, 2u16);
        // C s_156_18: const #3u : u8
        let s_156_18: u8 = 3;
        // C s_156_19: cast zx s_156_18 -> bv
        let s_156_19: Bits = Bits::new(s_156_18 as u128, 2u16);
        // D s_156_20: cmp-ne s_156_17 s_156_19
        let s_156_20: bool = ((s_156_17) != (s_156_19));
        // D s_156_21: write-var gs#107041 <= s_156_20
        fn_state.gs_107041 = s_156_20;
        // N s_156_22: jump b138
        return block_138(state, tracer, fn_state);
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_157_0: read-var __CNTHCTL_EL2_ECV:u8
        let s_157_0: bool = fn_state.u__CNTHCTL_EL2_ECV;
        // D s_157_1: cast zx s_157_0 -> bv
        let s_157_1: Bits = Bits::new(s_157_0 as u128, 1u16);
        // C s_157_2: const #1u : u8
        let s_157_2: bool = true;
        // C s_157_3: cast zx s_157_2 -> bv
        let s_157_3: Bits = Bits::new(s_157_2 as u128, 1u16);
        // D s_157_4: cmp-eq s_157_1 s_157_3
        let s_157_4: bool = ((s_157_1) == (s_157_3));
        // D s_157_5: write-var gs#107040 <= s_157_4
        fn_state.gs_107040 = s_157_4;
        // N s_157_6: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_158_0: read-var __SCR_EL3_ECVEn:u8
        let s_158_0: bool = fn_state.u__SCR_EL3_ECVEn;
        // D s_158_1: cast zx s_158_0 -> bv
        let s_158_1: Bits = Bits::new(s_158_0 as u128, 1u16);
        // C s_158_2: const #1u : u8
        let s_158_2: bool = true;
        // C s_158_3: cast zx s_158_2 -> bv
        let s_158_3: Bits = Bits::new(s_158_2 as u128, 1u16);
        // D s_158_4: cmp-eq s_158_1 s_158_3
        let s_158_4: bool = ((s_158_1) == (s_158_3));
        // D s_158_5: write-var gs#107039 <= s_158_4
        fn_state.gs_107039 = s_158_4;
        // N s_158_6: jump b134
        return block_134(state, tracer, fn_state);
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_159_0: const #432u : u32
        let s_159_0: u32 = 432;
        // D s_159_1: read-reg s_159_0:u8
        let s_159_1: u8 = {
            let value = state.read_register::<u8>(s_159_0 as isize);
            tracer.read_register(s_159_0 as isize, value);
            value
        };
        // D s_159_2: call ELUsingAArch32(s_159_1)
        let s_159_2: bool = ELUsingAArch32(state, tracer, s_159_1);
        // D s_159_3: not s_159_2
        let s_159_3: bool = !s_159_2;
        // D s_159_4: write-var gs#107038 <= s_159_3
        fn_state.gs_107038 = s_159_3;
        // N s_159_5: jump b132
        return block_132(state, tracer, fn_state);
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
        // D s_160_2: write-var gs#107037 <= s_160_1
        fn_state.gs_107037 = s_160_1;
        // N s_160_3: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_161_0: read-var __CNTHP_CTL_EL2_ENABLE:u8
        let s_161_0: bool = fn_state.u__CNTHP_CTL_EL2_ENABLE;
        // D s_161_1: cast zx s_161_0 -> bv
        let s_161_1: Bits = Bits::new(s_161_0 as u128, 1u16);
        // C s_161_2: const #0u : u8
        let s_161_2: bool = false;
        // C s_161_3: cast zx s_161_2 -> bv
        let s_161_3: Bits = Bits::new(s_161_2 as u128, 1u16);
        // D s_161_4: cmp-eq s_161_1 s_161_3
        let s_161_4: bool = ((s_161_1) == (s_161_3));
        // N s_161_5: branch s_161_4 b163 b162
        if s_161_4 {
            return block_163(state, tracer, fn_state);
        } else {
            return block_162(state, tracer, fn_state);
        };
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_162_0: const #16640u : u32
        let s_162_0: u32 = 16640;
        // D s_162_1: read-reg s_162_0:u64
        let s_162_1: u64 = {
            let value = state.read_register::<u64>(s_162_0 as isize);
            tracer.read_register(s_162_0 as isize, value);
            value
        };
        // C s_162_2: const #() : ()
        let s_162_2: () = ();
        // S s_162_3: call PhysicalCountInt(s_162_2)
        let s_162_3: u64 = PhysicalCountInt(state, tracer, s_162_2);
        // D s_162_4: cast zx s_162_1 -> bv
        let s_162_4: Bits = Bits::new(s_162_1 as u128, 64u16);
        // S s_162_5: cast zx s_162_3 -> bv
        let s_162_5: Bits = Bits::new(s_162_3 as u128, 64u16);
        // D s_162_6: sub s_162_4 s_162_5
        let s_162_6: Bits = ((s_162_4) - (s_162_5));
        // D s_162_7: cast reint s_162_6 -> u64
        let s_162_7: u64 = (s_162_6.value() as u64);
        // C s_162_8: const #0s : i
        let s_162_8: i128 = 0;
        // D s_162_9: cast zx s_162_7 -> bv
        let s_162_9: Bits = Bits::new(s_162_7 as u128, 64u16);
        // C s_162_10: const #1s : i64
        let s_162_10: i64 = 1;
        // C s_162_11: cast zx s_162_10 -> i
        let s_162_11: i128 = (i128::try_from(s_162_10).unwrap());
        // C s_162_12: const #31s : i
        let s_162_12: i128 = 31;
        // C s_162_13: add s_162_12 s_162_11
        let s_162_13: i128 = (s_162_12 + s_162_11);
        // D s_162_14: bit-extract s_162_9 s_162_8 s_162_13
        let s_162_14: Bits = (Bits::new(
            ((s_162_9) >> (s_162_8)).value(),
            u16::try_from(s_162_13).unwrap(),
        ));
        // D s_162_15: cast reint s_162_14 -> u32
        let s_162_15: u32 = (s_162_14.value() as u32);
        // D s_162_16: read-var t:i
        let s_162_16: i128 = fn_state.t;
        // D s_162_17: call R_set(s_162_16, s_162_15)
        let s_162_17: () = R_set(state, tracer, s_162_16, s_162_15);
        // N s_162_18: return
        return;
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_163_0: const #32s : i64
        let s_163_0: i64 = 32;
        // C s_163_1: cast zx s_163_0 -> i
        let s_163_1: i128 = (i128::try_from(s_163_0).unwrap());
        // S s_163_2: call __UNKNOWN_bits(s_163_1)
        let s_163_2: Bits = u__UNKNOWN_bits(state, tracer, s_163_1);
        // S s_163_3: cast reint s_163_2 -> u32
        let s_163_3: u32 = (s_163_2.value() as u32);
        // D s_163_4: read-var t:i
        let s_163_4: i128 = fn_state.t;
        // D s_163_5: call R_set(s_163_4, s_163_3)
        let s_163_5: () = R_set(state, tracer, s_163_4, s_163_3);
        // N s_163_6: return
        return;
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_164_0: const #90704u : u32
        let s_164_0: u32 = 90704;
        // D s_164_1: read-reg s_164_0:struct
        let s_164_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_164_0 as isize);
            tracer.read_register(s_164_0 as isize, value);
            value
        };
        // D s_164_2: call _get_SCR_EL3_Type_NS(s_164_1)
        let s_164_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_164_1);
        // D s_164_3: cast zx s_164_2 -> bv
        let s_164_3: Bits = Bits::new(s_164_2 as u128, 1u16);
        // C s_164_4: const #1u : u8
        let s_164_4: bool = true;
        // C s_164_5: cast zx s_164_4 -> bv
        let s_164_5: Bits = Bits::new(s_164_4 as u128, 1u16);
        // D s_164_6: cmp-eq s_164_3 s_164_5
        let s_164_6: bool = ((s_164_3) == (s_164_5));
        // D s_164_7: write-var gs#107036 <= s_164_6
        fn_state.gs_107036 = s_164_6;
        // N s_164_8: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_165_0: const #102552u : u32
        let s_165_0: u32 = 102552;
        // D s_165_1: read-reg s_165_0:struct
        let s_165_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_165_0 as isize);
            tracer.read_register(s_165_0 as isize, value);
            value
        };
        // D s_165_2: call _get_HCR_EL2_Type_E2H(s_165_1)
        let s_165_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_165_1);
        // C s_165_3: const #102552u : u32
        let s_165_3: u32 = 102552;
        // D s_165_4: read-reg s_165_3:struct
        let s_165_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_165_3 as isize);
            tracer.read_register(s_165_3 as isize, value);
            value
        };
        // D s_165_5: call _get_HCR_EL2_Type_TGE(s_165_4)
        let s_165_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_165_4);
        // D s_165_6: cast zx s_165_2 -> bv
        let s_165_6: Bits = Bits::new(s_165_2 as u128, 1u16);
        // D s_165_7: cast zx s_165_5 -> bv
        let s_165_7: Bits = Bits::new(s_165_5 as u128, 1u16);
        // D s_165_8: cast reint s_165_6 -> u128
        let s_165_8: u128 = (s_165_6.value() as u128);
        // D s_165_9: size-of s_165_6
        let s_165_9: u16 = s_165_6.length();
        // D s_165_10: cast reint s_165_7 -> u128
        let s_165_10: u128 = (s_165_7.value() as u128);
        // D s_165_11: size-of s_165_7
        let s_165_11: u16 = s_165_7.length();
        // D s_165_12: lsl s_165_8 s_165_11
        let s_165_12: u128 = s_165_8 << s_165_11;
        // D s_165_13: or s_165_12 s_165_10
        let s_165_13: u128 = ((s_165_12) | (s_165_10));
        // D s_165_14: add s_165_9 s_165_11
        let s_165_14: u16 = (s_165_9 + s_165_11);
        // D s_165_15: create-bits s_165_13 s_165_14
        let s_165_15: Bits = Bits::new(s_165_13, s_165_14);
        // D s_165_16: cast reint s_165_15 -> u8
        let s_165_16: u8 = (s_165_15.value() as u8);
        // D s_165_17: cast zx s_165_16 -> bv
        let s_165_17: Bits = Bits::new(s_165_16 as u128, 2u16);
        // C s_165_18: const #3u : u8
        let s_165_18: u8 = 3;
        // C s_165_19: cast zx s_165_18 -> bv
        let s_165_19: Bits = Bits::new(s_165_18 as u128, 2u16);
        // D s_165_20: cmp-eq s_165_17 s_165_19
        let s_165_20: bool = ((s_165_17) == (s_165_19));
        // D s_165_21: write-var gs#107035 <= s_165_20
        fn_state.gs_107035 = s_165_20;
        // N s_165_22: jump b125
        return block_125(state, tracer, fn_state);
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_166_0: const #432u : u32
        let s_166_0: u32 = 432;
        // D s_166_1: read-reg s_166_0:u8
        let s_166_1: u8 = {
            let value = state.read_register::<u8>(s_166_0 as isize);
            tracer.read_register(s_166_0 as isize, value);
            value
        };
        // D s_166_2: call ELUsingAArch32(s_166_1)
        let s_166_2: bool = ELUsingAArch32(state, tracer, s_166_1);
        // D s_166_3: not s_166_2
        let s_166_3: bool = !s_166_2;
        // D s_166_4: write-var gs#107034 <= s_166_3
        fn_state.gs_107034 = s_166_3;
        // N s_166_5: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_167_0: read-var __CNTHPS_CTL_EL2_ENABLE:u8
        let s_167_0: bool = fn_state.u__CNTHPS_CTL_EL2_ENABLE;
        // D s_167_1: cast zx s_167_0 -> bv
        let s_167_1: Bits = Bits::new(s_167_0 as u128, 1u16);
        // C s_167_2: const #0u : u8
        let s_167_2: bool = false;
        // C s_167_3: cast zx s_167_2 -> bv
        let s_167_3: Bits = Bits::new(s_167_2 as u128, 1u16);
        // D s_167_4: cmp-eq s_167_1 s_167_3
        let s_167_4: bool = ((s_167_1) == (s_167_3));
        // N s_167_5: branch s_167_4 b169 b168
        if s_167_4 {
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
        // C s_168_0: const #22672u : u32
        let s_168_0: u32 = 22672;
        // D s_168_1: read-reg s_168_0:u64
        let s_168_1: u64 = {
            let value = state.read_register::<u64>(s_168_0 as isize);
            tracer.read_register(s_168_0 as isize, value);
            value
        };
        // C s_168_2: const #() : ()
        let s_168_2: () = ();
        // S s_168_3: call PhysicalCountInt(s_168_2)
        let s_168_3: u64 = PhysicalCountInt(state, tracer, s_168_2);
        // D s_168_4: cast zx s_168_1 -> bv
        let s_168_4: Bits = Bits::new(s_168_1 as u128, 64u16);
        // S s_168_5: cast zx s_168_3 -> bv
        let s_168_5: Bits = Bits::new(s_168_3 as u128, 64u16);
        // D s_168_6: sub s_168_4 s_168_5
        let s_168_6: Bits = ((s_168_4) - (s_168_5));
        // D s_168_7: cast reint s_168_6 -> u64
        let s_168_7: u64 = (s_168_6.value() as u64);
        // C s_168_8: const #0s : i
        let s_168_8: i128 = 0;
        // D s_168_9: cast zx s_168_7 -> bv
        let s_168_9: Bits = Bits::new(s_168_7 as u128, 64u16);
        // C s_168_10: const #1s : i64
        let s_168_10: i64 = 1;
        // C s_168_11: cast zx s_168_10 -> i
        let s_168_11: i128 = (i128::try_from(s_168_10).unwrap());
        // C s_168_12: const #31s : i
        let s_168_12: i128 = 31;
        // C s_168_13: add s_168_12 s_168_11
        let s_168_13: i128 = (s_168_12 + s_168_11);
        // D s_168_14: bit-extract s_168_9 s_168_8 s_168_13
        let s_168_14: Bits = (Bits::new(
            ((s_168_9) >> (s_168_8)).value(),
            u16::try_from(s_168_13).unwrap(),
        ));
        // D s_168_15: cast reint s_168_14 -> u32
        let s_168_15: u32 = (s_168_14.value() as u32);
        // D s_168_16: read-var t:i
        let s_168_16: i128 = fn_state.t;
        // D s_168_17: call R_set(s_168_16, s_168_15)
        let s_168_17: () = R_set(state, tracer, s_168_16, s_168_15);
        // N s_168_18: return
        return;
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_169_0: const #32s : i64
        let s_169_0: i64 = 32;
        // C s_169_1: cast zx s_169_0 -> i
        let s_169_1: i128 = (i128::try_from(s_169_0).unwrap());
        // S s_169_2: call __UNKNOWN_bits(s_169_1)
        let s_169_2: Bits = u__UNKNOWN_bits(state, tracer, s_169_1);
        // S s_169_3: cast reint s_169_2 -> u32
        let s_169_3: u32 = (s_169_2.value() as u32);
        // D s_169_4: read-var t:i
        let s_169_4: i128 = fn_state.t;
        // D s_169_5: call R_set(s_169_4, s_169_3)
        let s_169_5: () = R_set(state, tracer, s_169_4, s_169_3);
        // N s_169_6: return
        return;
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_170_0: const #117u : u32
        let s_170_0: u32 = 117;
        // S s_170_1: call IsFeatureImplemented(s_170_0)
        let s_170_1: bool = IsFeatureImplemented(state, tracer, s_170_0);
        // D s_170_2: write-var gs#107033 <= s_170_1
        fn_state.gs_107033 = s_170_1;
        // N s_170_3: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_171_0: const #90704u : u32
        let s_171_0: u32 = 90704;
        // D s_171_1: read-reg s_171_0:struct
        let s_171_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_171_0 as isize);
            tracer.read_register(s_171_0 as isize, value);
            value
        };
        // D s_171_2: call _get_SCR_EL3_Type_NS(s_171_1)
        let s_171_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_171_1);
        // D s_171_3: cast zx s_171_2 -> bv
        let s_171_3: Bits = Bits::new(s_171_2 as u128, 1u16);
        // C s_171_4: const #0u : u8
        let s_171_4: bool = false;
        // C s_171_5: cast zx s_171_4 -> bv
        let s_171_5: Bits = Bits::new(s_171_4 as u128, 1u16);
        // D s_171_6: cmp-eq s_171_3 s_171_5
        let s_171_6: bool = ((s_171_3) == (s_171_5));
        // D s_171_7: write-var gs#107032 <= s_171_6
        fn_state.gs_107032 = s_171_6;
        // N s_171_8: jump b118
        return block_118(state, tracer, fn_state);
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
        // D s_172_20: cmp-eq s_172_17 s_172_19
        let s_172_20: bool = ((s_172_17) == (s_172_19));
        // D s_172_21: write-var gs#107031 <= s_172_20
        fn_state.gs_107031 = s_172_20;
        // N s_172_22: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_173_0: const #432u : u32
        let s_173_0: u32 = 432;
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
        // D s_173_4: write-var gs#107030 <= s_173_3
        fn_state.gs_107030 = s_173_3;
        // N s_173_5: jump b114
        return block_114(state, tracer, fn_state);
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
        // D s_175_0: read-var __CNTHCTL_PL1PCEN:u8
        let s_175_0: bool = fn_state.u__CNTHCTL_PL1PCEN;
        // D s_175_1: cast zx s_175_0 -> bv
        let s_175_1: Bits = Bits::new(s_175_0 as u128, 1u16);
        // C s_175_2: const #0u : u8
        let s_175_2: bool = false;
        // C s_175_3: cast zx s_175_2 -> bv
        let s_175_3: Bits = Bits::new(s_175_2 as u128, 1u16);
        // D s_175_4: cmp-eq s_175_1 s_175_3
        let s_175_4: bool = ((s_175_1) == (s_175_3));
        // D s_175_5: write-var gs#107029 <= s_175_4
        fn_state.gs_107029 = s_175_4;
        // N s_175_6: jump b111
        return block_111(state, tracer, fn_state);
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
        // D s_176_3: write-var gs#107028 <= s_176_2
        fn_state.gs_107028 = s_176_2;
        // N s_176_4: jump b109
        return block_109(state, tracer, fn_state);
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
        // D s_178_0: read-var __CNTHCTL_EL2_EL0PTEN:u8
        let s_178_0: bool = fn_state.u__CNTHCTL_EL2_EL0PTEN;
        // D s_178_1: cast zx s_178_0 -> bv
        let s_178_1: Bits = Bits::new(s_178_0 as u128, 1u16);
        // C s_178_2: const #0u : u8
        let s_178_2: bool = false;
        // C s_178_3: cast zx s_178_2 -> bv
        let s_178_3: Bits = Bits::new(s_178_2 as u128, 1u16);
        // D s_178_4: cmp-eq s_178_1 s_178_3
        let s_178_4: bool = ((s_178_1) == (s_178_3));
        // D s_178_5: write-var gs#107027 <= s_178_4
        fn_state.gs_107027 = s_178_4;
        // N s_178_6: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_179_0: const #102552u : u32
        let s_179_0: u32 = 102552;
        // D s_179_1: read-reg s_179_0:struct
        let s_179_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_179_0 as isize);
            tracer.read_register(s_179_0 as isize, value);
            value
        };
        // D s_179_2: call _get_HCR_EL2_Type_E2H(s_179_1)
        let s_179_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_179_1);
        // C s_179_3: const #102552u : u32
        let s_179_3: u32 = 102552;
        // D s_179_4: read-reg s_179_3:struct
        let s_179_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_179_3 as isize);
            tracer.read_register(s_179_3 as isize, value);
            value
        };
        // D s_179_5: call _get_HCR_EL2_Type_TGE(s_179_4)
        let s_179_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_179_4);
        // D s_179_6: cast zx s_179_2 -> bv
        let s_179_6: Bits = Bits::new(s_179_2 as u128, 1u16);
        // D s_179_7: cast zx s_179_5 -> bv
        let s_179_7: Bits = Bits::new(s_179_5 as u128, 1u16);
        // D s_179_8: cast reint s_179_6 -> u128
        let s_179_8: u128 = (s_179_6.value() as u128);
        // D s_179_9: size-of s_179_6
        let s_179_9: u16 = s_179_6.length();
        // D s_179_10: cast reint s_179_7 -> u128
        let s_179_10: u128 = (s_179_7.value() as u128);
        // D s_179_11: size-of s_179_7
        let s_179_11: u16 = s_179_7.length();
        // D s_179_12: lsl s_179_8 s_179_11
        let s_179_12: u128 = s_179_8 << s_179_11;
        // D s_179_13: or s_179_12 s_179_10
        let s_179_13: u128 = ((s_179_12) | (s_179_10));
        // D s_179_14: add s_179_9 s_179_11
        let s_179_14: u16 = (s_179_9 + s_179_11);
        // D s_179_15: create-bits s_179_13 s_179_14
        let s_179_15: Bits = Bits::new(s_179_13, s_179_14);
        // D s_179_16: cast reint s_179_15 -> u8
        let s_179_16: u8 = (s_179_15.value() as u8);
        // D s_179_17: cast zx s_179_16 -> bv
        let s_179_17: Bits = Bits::new(s_179_16 as u128, 2u16);
        // C s_179_18: const #3u : u8
        let s_179_18: u8 = 3;
        // C s_179_19: cast zx s_179_18 -> bv
        let s_179_19: Bits = Bits::new(s_179_18 as u128, 2u16);
        // D s_179_20: cmp-eq s_179_17 s_179_19
        let s_179_20: bool = ((s_179_17) == (s_179_19));
        // D s_179_21: write-var gs#107026 <= s_179_20
        fn_state.gs_107026 = s_179_20;
        // N s_179_22: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_180_0: const #432u : u32
        let s_180_0: u32 = 432;
        // D s_180_1: read-reg s_180_0:u8
        let s_180_1: u8 = {
            let value = state.read_register::<u8>(s_180_0 as isize);
            tracer.read_register(s_180_0 as isize, value);
            value
        };
        // D s_180_2: call ELUsingAArch32(s_180_1)
        let s_180_2: bool = ELUsingAArch32(state, tracer, s_180_1);
        // D s_180_3: not s_180_2
        let s_180_3: bool = !s_180_2;
        // D s_180_4: write-var gs#107025 <= s_180_3
        fn_state.gs_107025 = s_180_3;
        // N s_180_5: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_181_0: const #3u : u8
        let s_181_0: u8 = 3;
        // C s_181_1: cast zx s_181_0 -> bv
        let s_181_1: Bits = Bits::new(s_181_0 as u128, 8u16);
        // C s_181_2: cast zx s_181_1 -> i
        let s_181_2: i128 = (s_181_1.value() as i128);
        // C s_181_3: cast reint s_181_2 -> i64
        let s_181_3: i64 = (s_181_2 as i64);
        // C s_181_4: cast zx s_181_3 -> i
        let s_181_4: i128 = (i128::try_from(s_181_3).unwrap());
        // C s_181_5: const #432u : u32
        let s_181_5: u32 = 432;
        // D s_181_6: read-reg s_181_5:u8
        let s_181_6: u8 = {
            let value = state.read_register::<u8>(s_181_5 as isize);
            tracer.read_register(s_181_5 as isize, value);
            value
        };
        // D s_181_7: call AArch64_AArch32SystemAccessTrap(s_181_6, s_181_4)
        let s_181_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_181_6,
            s_181_4,
        );
        // N s_181_8: return
        return;
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_182_0: read-var __CNTHCTL_EL2_EL1PTEN:u8
        let s_182_0: bool = fn_state.u__CNTHCTL_EL2_EL1PTEN;
        // D s_182_1: cast zx s_182_0 -> bv
        let s_182_1: Bits = Bits::new(s_182_0 as u128, 1u16);
        // C s_182_2: const #0u : u8
        let s_182_2: bool = false;
        // C s_182_3: cast zx s_182_2 -> bv
        let s_182_3: Bits = Bits::new(s_182_2 as u128, 1u16);
        // D s_182_4: cmp-eq s_182_1 s_182_3
        let s_182_4: bool = ((s_182_1) == (s_182_3));
        // D s_182_5: write-var gs#107024 <= s_182_4
        fn_state.gs_107024 = s_182_4;
        // N s_182_6: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_183_0: const #102552u : u32
        let s_183_0: u32 = 102552;
        // D s_183_1: read-reg s_183_0:struct
        let s_183_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_183_0 as isize);
            tracer.read_register(s_183_0 as isize, value);
            value
        };
        // D s_183_2: call _get_HCR_EL2_Type_E2H(s_183_1)
        let s_183_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_183_1);
        // C s_183_3: const #102552u : u32
        let s_183_3: u32 = 102552;
        // D s_183_4: read-reg s_183_3:struct
        let s_183_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_183_3 as isize);
            tracer.read_register(s_183_3 as isize, value);
            value
        };
        // D s_183_5: call _get_HCR_EL2_Type_TGE(s_183_4)
        let s_183_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_183_4);
        // D s_183_6: cast zx s_183_2 -> bv
        let s_183_6: Bits = Bits::new(s_183_2 as u128, 1u16);
        // D s_183_7: cast zx s_183_5 -> bv
        let s_183_7: Bits = Bits::new(s_183_5 as u128, 1u16);
        // D s_183_8: cast reint s_183_6 -> u128
        let s_183_8: u128 = (s_183_6.value() as u128);
        // D s_183_9: size-of s_183_6
        let s_183_9: u16 = s_183_6.length();
        // D s_183_10: cast reint s_183_7 -> u128
        let s_183_10: u128 = (s_183_7.value() as u128);
        // D s_183_11: size-of s_183_7
        let s_183_11: u16 = s_183_7.length();
        // D s_183_12: lsl s_183_8 s_183_11
        let s_183_12: u128 = s_183_8 << s_183_11;
        // D s_183_13: or s_183_12 s_183_10
        let s_183_13: u128 = ((s_183_12) | (s_183_10));
        // D s_183_14: add s_183_9 s_183_11
        let s_183_14: u16 = (s_183_9 + s_183_11);
        // D s_183_15: create-bits s_183_13 s_183_14
        let s_183_15: Bits = Bits::new(s_183_13, s_183_14);
        // D s_183_16: cast reint s_183_15 -> u8
        let s_183_16: u8 = (s_183_15.value() as u8);
        // D s_183_17: cast zx s_183_16 -> bv
        let s_183_17: Bits = Bits::new(s_183_16 as u128, 2u16);
        // C s_183_18: const #2u : u8
        let s_183_18: u8 = 2;
        // C s_183_19: cast zx s_183_18 -> bv
        let s_183_19: Bits = Bits::new(s_183_18 as u128, 2u16);
        // D s_183_20: cmp-eq s_183_17 s_183_19
        let s_183_20: bool = ((s_183_17) == (s_183_19));
        // D s_183_21: write-var gs#107023 <= s_183_20
        fn_state.gs_107023 = s_183_20;
        // N s_183_22: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_184_0: const #432u : u32
        let s_184_0: u32 = 432;
        // D s_184_1: read-reg s_184_0:u8
        let s_184_1: u8 = {
            let value = state.read_register::<u8>(s_184_0 as isize);
            tracer.read_register(s_184_0 as isize, value);
            value
        };
        // D s_184_2: call ELUsingAArch32(s_184_1)
        let s_184_2: bool = ELUsingAArch32(state, tracer, s_184_1);
        // D s_184_3: not s_184_2
        let s_184_3: bool = !s_184_2;
        // D s_184_4: write-var gs#107022 <= s_184_3
        fn_state.gs_107022 = s_184_3;
        // N s_184_5: jump b95
        return block_95(state, tracer, fn_state);
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
        // C s_185_5: const #432u : u32
        let s_185_5: u32 = 432;
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
        // D s_186_0: read-var __CNTHCTL_EL2_EL1PCEN:u8
        let s_186_0: bool = fn_state.u__CNTHCTL_EL2_EL1PCEN;
        // D s_186_1: cast zx s_186_0 -> bv
        let s_186_1: Bits = Bits::new(s_186_0 as u128, 1u16);
        // C s_186_2: const #0u : u8
        let s_186_2: bool = false;
        // C s_186_3: cast zx s_186_2 -> bv
        let s_186_3: Bits = Bits::new(s_186_2 as u128, 1u16);
        // D s_186_4: cmp-eq s_186_1 s_186_3
        let s_186_4: bool = ((s_186_1) == (s_186_3));
        // D s_186_5: write-var gs#107021 <= s_186_4
        fn_state.gs_107021 = s_186_4;
        // N s_186_6: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_187_0: read-var __HCR_EL2_E2H:u8
        let s_187_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_187_1: cast zx s_187_0 -> bv
        let s_187_1: Bits = Bits::new(s_187_0 as u128, 1u16);
        // C s_187_2: const #0u : u8
        let s_187_2: bool = false;
        // C s_187_3: cast zx s_187_2 -> bv
        let s_187_3: Bits = Bits::new(s_187_2 as u128, 1u16);
        // D s_187_4: cmp-eq s_187_1 s_187_3
        let s_187_4: bool = ((s_187_1) == (s_187_3));
        // D s_187_5: write-var gs#107020 <= s_187_4
        fn_state.gs_107020 = s_187_4;
        // N s_187_6: jump b90
        return block_90(state, tracer, fn_state);
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
        // D s_188_4: write-var gs#107019 <= s_188_3
        fn_state.gs_107019 = s_188_3;
        // N s_188_5: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_189<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_189_0: const #() : ()
        let s_189_0: () = ();
        // S s_189_1: call EL2Enabled(s_189_0)
        let s_189_1: bool = EL2Enabled(state, tracer, s_189_0);
        // N s_189_2: branch s_189_1 b205 b190
        if s_189_1 {
            return block_205(state, tracer, fn_state);
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
        // D s_190_1: write-var gs#107061 <= s_190_0
        fn_state.gs_107061 = s_190_0;
        // N s_190_2: jump b191
        return block_191(state, tracer, fn_state);
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_191_0: read-var gs#107061:u8
        let s_191_0: bool = fn_state.gs_107061;
        // N s_191_1: branch s_191_0 b204 b192
        if s_191_0 {
            return block_204(state, tracer, fn_state);
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
        // D s_192_1: write-var gs#107062 <= s_192_0
        fn_state.gs_107062 = s_192_0;
        // N s_192_2: jump b193
        return block_193(state, tracer, fn_state);
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_193_0: read-var gs#107062:u8
        let s_193_0: bool = fn_state.gs_107062;
        // N s_193_1: branch s_193_0 b203 b194
        if s_193_0 {
            return block_203(state, tracer, fn_state);
        } else {
            return block_194(state, tracer, fn_state);
        };
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_194_0: const #() : ()
        let s_194_0: () = ();
        // S s_194_1: call EL2Enabled(s_194_0)
        let s_194_1: bool = EL2Enabled(state, tracer, s_194_0);
        // N s_194_2: branch s_194_1 b202 b195
        if s_194_1 {
            return block_202(state, tracer, fn_state);
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
        // D s_195_1: write-var gs#107063 <= s_195_0
        fn_state.gs_107063 = s_195_0;
        // N s_195_2: jump b196
        return block_196(state, tracer, fn_state);
    }
    fn block_196<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_196_0: read-var gs#107063:u8
        let s_196_0: bool = fn_state.gs_107063;
        // N s_196_1: branch s_196_0 b201 b197
        if s_196_0 {
            return block_201(state, tracer, fn_state);
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
        // D s_197_1: write-var gs#107064 <= s_197_0
        fn_state.gs_107064 = s_197_0;
        // N s_197_2: jump b198
        return block_198(state, tracer, fn_state);
    }
    fn block_198<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_198_0: read-var gs#107064:u8
        let s_198_0: bool = fn_state.gs_107064;
        // N s_198_1: branch s_198_0 b200 b199
        if s_198_0 {
            return block_200(state, tracer, fn_state);
        } else {
            return block_199(state, tracer, fn_state);
        };
    }
    fn block_199<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_199_0: panic
        panic!("{:?}", ());
        // N s_199_1: return
        return;
    }
    fn block_200<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_200_0: const #0u : u8
        let s_200_0: u8 = 0;
        // C s_200_1: cast zx s_200_0 -> bv
        let s_200_1: Bits = Bits::new(s_200_0 as u128, 8u16);
        // C s_200_2: cast zx s_200_1 -> i
        let s_200_2: i128 = (s_200_1.value() as i128);
        // C s_200_3: cast reint s_200_2 -> i64
        let s_200_3: i64 = (s_200_2 as i64);
        // C s_200_4: cast zx s_200_3 -> i
        let s_200_4: i128 = (i128::try_from(s_200_3).unwrap());
        // S s_200_5: call AArch32_TakeHypTrapException(s_200_4)
        let s_200_5: () = AArch32_TakeHypTrapException(state, tracer, s_200_4);
        // N s_200_6: return
        return;
    }
    fn block_201<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_201_0: read-var __HCR_TGE:u8
        let s_201_0: bool = fn_state.u__HCR_TGE;
        // D s_201_1: cast zx s_201_0 -> bv
        let s_201_1: Bits = Bits::new(s_201_0 as u128, 1u16);
        // C s_201_2: const #1u : u8
        let s_201_2: bool = true;
        // C s_201_3: cast zx s_201_2 -> bv
        let s_201_3: Bits = Bits::new(s_201_2 as u128, 1u16);
        // D s_201_4: cmp-eq s_201_1 s_201_3
        let s_201_4: bool = ((s_201_1) == (s_201_3));
        // D s_201_5: write-var gs#107064 <= s_201_4
        fn_state.gs_107064 = s_201_4;
        // N s_201_6: jump b198
        return block_198(state, tracer, fn_state);
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
        // D s_202_3: write-var gs#107063 <= s_202_2
        fn_state.gs_107063 = s_202_2;
        // N s_202_4: jump b196
        return block_196(state, tracer, fn_state);
    }
    fn block_203<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_203_0: const #3u : u8
        let s_203_0: u8 = 3;
        // C s_203_1: cast zx s_203_0 -> bv
        let s_203_1: Bits = Bits::new(s_203_0 as u128, 8u16);
        // C s_203_2: cast zx s_203_1 -> i
        let s_203_2: i128 = (s_203_1.value() as i128);
        // C s_203_3: cast reint s_203_2 -> i64
        let s_203_3: i64 = (s_203_2 as i64);
        // C s_203_4: cast zx s_203_3 -> i
        let s_203_4: i128 = (i128::try_from(s_203_3).unwrap());
        // C s_203_5: const #432u : u32
        let s_203_5: u32 = 432;
        // D s_203_6: read-reg s_203_5:u8
        let s_203_6: u8 = {
            let value = state.read_register::<u8>(s_203_5 as isize);
            tracer.read_register(s_203_5 as isize, value);
            value
        };
        // D s_203_7: call AArch64_AArch32SystemAccessTrap(s_203_6, s_203_4)
        let s_203_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_203_6,
            s_203_4,
        );
        // N s_203_8: return
        return;
    }
    fn block_204<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_204_0: read-var __HCR_EL2_TGE:u8
        let s_204_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_204_1: cast zx s_204_0 -> bv
        let s_204_1: Bits = Bits::new(s_204_0 as u128, 1u16);
        // C s_204_2: const #1u : u8
        let s_204_2: bool = true;
        // C s_204_3: cast zx s_204_2 -> bv
        let s_204_3: Bits = Bits::new(s_204_2 as u128, 1u16);
        // D s_204_4: cmp-eq s_204_1 s_204_3
        let s_204_4: bool = ((s_204_1) == (s_204_3));
        // D s_204_5: write-var gs#107062 <= s_204_4
        fn_state.gs_107062 = s_204_4;
        // N s_204_6: jump b193
        return block_193(state, tracer, fn_state);
    }
    fn block_205<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_205_0: const #432u : u32
        let s_205_0: u32 = 432;
        // D s_205_1: read-reg s_205_0:u8
        let s_205_1: u8 = {
            let value = state.read_register::<u8>(s_205_0 as isize);
            tracer.read_register(s_205_0 as isize, value);
            value
        };
        // D s_205_2: call ELUsingAArch32(s_205_1)
        let s_205_2: bool = ELUsingAArch32(state, tracer, s_205_1);
        // D s_205_3: not s_205_2
        let s_205_3: bool = !s_205_2;
        // D s_205_4: write-var gs#107061 <= s_205_3
        fn_state.gs_107061 = s_205_3;
        // N s_205_5: jump b191
        return block_191(state, tracer, fn_state);
    }
    fn block_206<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_206_0: read-var __CNTKCTL_PL0PTEN:u8
        let s_206_0: bool = fn_state.u__CNTKCTL_PL0PTEN;
        // D s_206_1: cast zx s_206_0 -> bv
        let s_206_1: Bits = Bits::new(s_206_0 as u128, 1u16);
        // C s_206_2: const #0u : u8
        let s_206_2: bool = false;
        // C s_206_3: cast zx s_206_2 -> bv
        let s_206_3: Bits = Bits::new(s_206_2 as u128, 1u16);
        // D s_206_4: cmp-eq s_206_1 s_206_3
        let s_206_4: bool = ((s_206_1) == (s_206_3));
        // D s_206_5: write-var gs#107018 <= s_206_4
        fn_state.gs_107018 = s_206_4;
        // N s_206_6: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_207<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_207_0: const #() : ()
        let s_207_0: () = ();
        // S s_207_1: call EL2Enabled(s_207_0)
        let s_207_1: bool = EL2Enabled(state, tracer, s_207_0);
        // N s_207_2: branch s_207_1 b215 b208
        if s_207_1 {
            return block_215(state, tracer, fn_state);
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
        // D s_208_1: write-var gs#107065 <= s_208_0
        fn_state.gs_107065 = s_208_0;
        // N s_208_2: jump b209
        return block_209(state, tracer, fn_state);
    }
    fn block_209<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_209_0: read-var gs#107065:u8
        let s_209_0: bool = fn_state.gs_107065;
        // N s_209_1: branch s_209_0 b214 b210
        if s_209_0 {
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
        // D s_210_1: write-var gs#107066 <= s_210_0
        fn_state.gs_107066 = s_210_0;
        // N s_210_2: jump b211
        return block_211(state, tracer, fn_state);
    }
    fn block_211<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_211_0: read-var gs#107066:u8
        let s_211_0: bool = fn_state.gs_107066;
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
        // C s_212_0: const #3u : u8
        let s_212_0: u8 = 3;
        // C s_212_1: cast zx s_212_0 -> bv
        let s_212_1: Bits = Bits::new(s_212_0 as u128, 8u16);
        // C s_212_2: cast zx s_212_1 -> i
        let s_212_2: i128 = (s_212_1.value() as i128);
        // C s_212_3: cast reint s_212_2 -> i64
        let s_212_3: i64 = (s_212_2 as i64);
        // C s_212_4: cast zx s_212_3 -> i
        let s_212_4: i128 = (i128::try_from(s_212_3).unwrap());
        // C s_212_5: const #440u : u32
        let s_212_5: u32 = 440;
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
        // C s_213_0: const #3u : u8
        let s_213_0: u8 = 3;
        // C s_213_1: cast zx s_213_0 -> bv
        let s_213_1: Bits = Bits::new(s_213_0 as u128, 8u16);
        // C s_213_2: cast zx s_213_1 -> i
        let s_213_2: i128 = (s_213_1.value() as i128);
        // C s_213_3: cast reint s_213_2 -> i64
        let s_213_3: i64 = (s_213_2 as i64);
        // C s_213_4: cast zx s_213_3 -> i
        let s_213_4: i128 = (i128::try_from(s_213_3).unwrap());
        // C s_213_5: const #432u : u32
        let s_213_5: u32 = 432;
        // D s_213_6: read-reg s_213_5:u8
        let s_213_6: u8 = {
            let value = state.read_register::<u8>(s_213_5 as isize);
            tracer.read_register(s_213_5 as isize, value);
            value
        };
        // D s_213_7: call AArch64_AArch32SystemAccessTrap(s_213_6, s_213_4)
        let s_213_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_213_6,
            s_213_4,
        );
        // N s_213_8: return
        return;
    }
    fn block_214<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_214_0: read-var __HCR_EL2_TGE:u8
        let s_214_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_214_1: cast zx s_214_0 -> bv
        let s_214_1: Bits = Bits::new(s_214_0 as u128, 1u16);
        // C s_214_2: const #1u : u8
        let s_214_2: bool = true;
        // C s_214_3: cast zx s_214_2 -> bv
        let s_214_3: Bits = Bits::new(s_214_2 as u128, 1u16);
        // D s_214_4: cmp-eq s_214_1 s_214_3
        let s_214_4: bool = ((s_214_1) == (s_214_3));
        // D s_214_5: write-var gs#107066 <= s_214_4
        fn_state.gs_107066 = s_214_4;
        // N s_214_6: jump b211
        return block_211(state, tracer, fn_state);
    }
    fn block_215<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_215_0: const #432u : u32
        let s_215_0: u32 = 432;
        // D s_215_1: read-reg s_215_0:u8
        let s_215_1: u8 = {
            let value = state.read_register::<u8>(s_215_0 as isize);
            tracer.read_register(s_215_0 as isize, value);
            value
        };
        // D s_215_2: call ELUsingAArch32(s_215_1)
        let s_215_2: bool = ELUsingAArch32(state, tracer, s_215_1);
        // D s_215_3: not s_215_2
        let s_215_3: bool = !s_215_2;
        // D s_215_4: write-var gs#107065 <= s_215_3
        fn_state.gs_107065 = s_215_3;
        // N s_215_5: jump b209
        return block_209(state, tracer, fn_state);
    }
    fn block_216<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_216_0: read-var __CNTKCTL_EL1_EL0PTEN:u8
        let s_216_0: bool = fn_state.u__CNTKCTL_EL1_EL0PTEN;
        // D s_216_1: cast zx s_216_0 -> bv
        let s_216_1: Bits = Bits::new(s_216_0 as u128, 1u16);
        // C s_216_2: const #0u : u8
        let s_216_2: bool = false;
        // C s_216_3: cast zx s_216_2 -> bv
        let s_216_3: Bits = Bits::new(s_216_2 as u128, 1u16);
        // D s_216_4: cmp-eq s_216_1 s_216_3
        let s_216_4: bool = ((s_216_1) == (s_216_3));
        // D s_216_5: write-var gs#107017 <= s_216_4
        fn_state.gs_107017 = s_216_4;
        // N s_216_6: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_217<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_217_0: const #() : ()
        let s_217_0: () = ();
        // S s_217_1: call EL2Enabled(s_217_0)
        let s_217_1: bool = EL2Enabled(state, tracer, s_217_0);
        // N s_217_2: branch s_217_1 b220 b218
        if s_217_1 {
            return block_220(state, tracer, fn_state);
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
        // D s_218_1: write-var gs#107015 <= s_218_0
        fn_state.gs_107015 = s_218_0;
        // N s_218_2: jump b219
        return block_219(state, tracer, fn_state);
    }
    fn block_219<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_219_0: read-var gs#107015:u8
        let s_219_0: bool = fn_state.gs_107015;
        // D s_219_1: not s_219_0
        let s_219_1: bool = !s_219_0;
        // D s_219_2: write-var gs#107016 <= s_219_1
        fn_state.gs_107016 = s_219_1;
        // N s_219_3: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_220<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_220_0: const #102552u : u32
        let s_220_0: u32 = 102552;
        // D s_220_1: read-reg s_220_0:struct
        let s_220_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_220_0 as isize);
            tracer.read_register(s_220_0 as isize, value);
            value
        };
        // D s_220_2: call _get_HCR_EL2_Type_E2H(s_220_1)
        let s_220_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_220_1);
        // C s_220_3: const #102552u : u32
        let s_220_3: u32 = 102552;
        // D s_220_4: read-reg s_220_3:struct
        let s_220_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_220_3 as isize);
            tracer.read_register(s_220_3 as isize, value);
            value
        };
        // D s_220_5: call _get_HCR_EL2_Type_TGE(s_220_4)
        let s_220_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_220_4);
        // D s_220_6: cast zx s_220_2 -> bv
        let s_220_6: Bits = Bits::new(s_220_2 as u128, 1u16);
        // D s_220_7: cast zx s_220_5 -> bv
        let s_220_7: Bits = Bits::new(s_220_5 as u128, 1u16);
        // D s_220_8: cast reint s_220_6 -> u128
        let s_220_8: u128 = (s_220_6.value() as u128);
        // D s_220_9: size-of s_220_6
        let s_220_9: u16 = s_220_6.length();
        // D s_220_10: cast reint s_220_7 -> u128
        let s_220_10: u128 = (s_220_7.value() as u128);
        // D s_220_11: size-of s_220_7
        let s_220_11: u16 = s_220_7.length();
        // D s_220_12: lsl s_220_8 s_220_11
        let s_220_12: u128 = s_220_8 << s_220_11;
        // D s_220_13: or s_220_12 s_220_10
        let s_220_13: u128 = ((s_220_12) | (s_220_10));
        // D s_220_14: add s_220_9 s_220_11
        let s_220_14: u16 = (s_220_9 + s_220_11);
        // D s_220_15: create-bits s_220_13 s_220_14
        let s_220_15: Bits = Bits::new(s_220_13, s_220_14);
        // D s_220_16: cast reint s_220_15 -> u8
        let s_220_16: u8 = (s_220_15.value() as u8);
        // D s_220_17: cast zx s_220_16 -> bv
        let s_220_17: Bits = Bits::new(s_220_16 as u128, 2u16);
        // C s_220_18: const #3u : u8
        let s_220_18: u8 = 3;
        // C s_220_19: cast zx s_220_18 -> bv
        let s_220_19: Bits = Bits::new(s_220_18 as u128, 2u16);
        // D s_220_20: cmp-eq s_220_17 s_220_19
        let s_220_20: bool = ((s_220_17) == (s_220_19));
        // D s_220_21: write-var gs#107015 <= s_220_20
        fn_state.gs_107015 = s_220_20;
        // N s_220_22: jump b219
        return block_219(state, tracer, fn_state);
    }
}
