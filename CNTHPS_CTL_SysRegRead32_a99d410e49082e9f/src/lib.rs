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
use u__get_CNTHP_CTL_EL2::*;
use u_get_HCR_EL2_Type_E2H::*;
use u_get_CNTKCTL_Type_PL0PTEN::*;
use HCR_read::*;
use u_get_CNTHCTL_EL2_Type_EL1PTEN::*;
use u__get_CNTP_CTL_NS::*;
use CNTHCTL_read::*;
use CNTP_CTL_NS_read::*;
use CNTP_CTL_read::*;
use u_get_SCR_Type_NS::*;
use AArch64_AArch32SystemAccessTrap::*;
use u_get_HCR_Type_TGE::*;
use u_get_SCR_EL3_Type_NS::*;
use u__get_CNTHPS_CTL_EL2::*;
use IsFeatureImplemented::*;
use u_get_CNTHCTL_EL2_Type_EL1PCEN::*;
use u_get_CNTHCTL_EL2_Type_EL0PTEN::*;
use R_set::*;
use u__get_CNTP_CTL_S::*;
use ELUsingAArch32::*;
use u__get_CNTP_CTL::*;
use CNTKCTL_read__1::*;
use u_get_CNTKCTL_EL1_Type_EL0PTEN::*;
use u_get_HCR_EL2_Type_TGE::*;
use u_get_CNTHCTL_Type_PL1PCEN::*;
use EL2Enabled::*;
use common::*;
pub fn CNTHPS_CTL_SysRegRead32_a99d410e49082e9f<T: Tracer>(
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
        gs_106953: bool,
        u__HCR_EL2_E2H: bool,
        gs_106955: bool,
        gs_106958: bool,
        gs_106951: bool,
        gs_106960: bool,
        gs_106934: bool,
        gs_106952: bool,
        ga_166472: ProductType5c790c8ef59cc8b2,
        ga_166510: ProductType700c18a878c5601b,
        ga_166484: ProductType5c790c8ef59cc8b2,
        gs_106961: bool,
        gs_106954: bool,
        gs_106942: bool,
        gs_106959: bool,
        u__HCR_TGE: bool,
        gs_106937: bool,
        gs_106938: bool,
        gs_106968: bool,
        gs_106943: bool,
        gs_106941: bool,
        ga_166526: ProductType700c18a878c5601b,
        u__PSTATE_EL: u8,
        gs_106971: bool,
        u__CNTHCTL_EL2_EL1PCEN: bool,
        ga_166513: ProductType700c18a878c5601b,
        gs_106935: bool,
        gs_106969: bool,
        gs_106957: bool,
        gs_106933: bool,
        u__HCR_EL2_TGE: bool,
        gs_106939: bool,
        ga_166519: ProductType700c18a878c5601b,
        ga_166529: ProductType700c18a878c5601b,
        u__CNTKCTL_PL0PTEN: bool,
        gs_106945: bool,
        u__CNTHCTL_EL2_EL0PTEN: bool,
        u__SCR_NS: bool,
        gs_106972: bool,
        gs_106947: bool,
        gs_106956: bool,
        u__CNTHCTL_PL1PCEN: bool,
        gs_106948: bool,
        gs_106962: bool,
        gs_106970: bool,
        gs_106944: bool,
        gs_106963: bool,
        gs_106973: bool,
        gs_106936: bool,
        gs_106949: bool,
        u__CNTKCTL_EL1_EL0PTEN: bool,
        gs_106946: bool,
        gs_106950: bool,
        ga_166522: ProductType700c18a878c5601b,
        ga_166488: ProductType700c18a878c5601b,
        gs_106940: bool,
        u__CNTHCTL_EL2_EL1PTEN: bool,
        gs_106932: bool,
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
        // C s_0_39: const #20920u : u32
        let s_0_39: u32 = 20920;
        // D s_0_40: read-reg s_0_39:struct
        let s_0_40: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_39 as isize);
            tracer.read_register(s_0_39 as isize, value);
            value
        };
        // D s_0_41: call _get_SCR_Type_NS(s_0_40)
        let s_0_41: bool = u_get_SCR_Type_NS(state, tracer, s_0_40);
        // D s_0_42: write-var __SCR_NS <= s_0_41
        fn_state.u__SCR_NS = s_0_41;
        // D s_0_43: read-var __PSTATE_EL:u8
        let s_0_43: u8 = fn_state.u__PSTATE_EL;
        // D s_0_44: cast zx s_0_43 -> bv
        let s_0_44: Bits = Bits::new(s_0_43 as u128, 2u16);
        // C s_0_45: const #448u : u32
        let s_0_45: u32 = 448;
        // D s_0_46: read-reg s_0_45:u8
        let s_0_46: u8 = {
            let value = state.read_register::<u8>(s_0_45 as isize);
            tracer.read_register(s_0_45 as isize, value);
            value
        };
        // D s_0_47: cast zx s_0_46 -> bv
        let s_0_47: Bits = Bits::new(s_0_46 as u128, 2u16);
        // D s_0_48: cmp-eq s_0_44 s_0_47
        let s_0_48: bool = ((s_0_44) == (s_0_47));
        // N s_0_49: branch s_0_48 b50 b1
        if s_0_48 {
            return block_50(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b14 b2
        if s_1_5 {
            return block_14(state, tracer, fn_state);
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
        // N s_2_6: branch s_2_5 b8 b3
        if s_2_5 {
            return block_8(state, tracer, fn_state);
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
        // N s_5_5: branch s_5_4 b7 b6
        if s_5_4 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call CNTP_CTL_NS_read(s_6_0)
        let s_6_1: ProductType700c18a878c5601b = CNTP_CTL_NS_read(state, tracer, s_6_0);
        // S s_6_2: call __get_CNTP_CTL_NS(s_6_1)
        let s_6_2: ProductType700c18a878c5601b = u__get_CNTP_CTL_NS(
            state,
            tracer,
            s_6_1,
        );
        // D s_6_3: write-var ga#166529 <= s_6_2
        fn_state.ga_166529 = s_6_2;
        // D s_6_4: read-var ga#166529.0:struct
        let s_6_4: u32 = fn_state.ga_166529._0;
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
        // C s_7_0: const #21888u : u32
        let s_7_0: u32 = 21888;
        // D s_7_1: read-reg s_7_0:struct
        let s_7_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: call __get_CNTP_CTL_S(s_7_1)
        let s_7_2: ProductType700c18a878c5601b = u__get_CNTP_CTL_S(state, tracer, s_7_1);
        // D s_7_3: write-var ga#166526 <= s_7_2
        fn_state.ga_166526 = s_7_2;
        // D s_7_4: read-var ga#166526.0:struct
        let s_7_4: u32 = fn_state.ga_166526._0;
        // D s_7_5: read-var t:i
        let s_7_5: i128 = fn_state.t;
        // D s_7_6: call R_set(s_7_5, s_7_4)
        let s_7_6: () = R_set(state, tracer, s_7_5, s_7_4);
        // N s_7_7: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #424u : u32
        let s_8_0: u32 = 424;
        // D s_8_1: read-reg s_8_0:u8
        let s_8_1: u8 = {
            let value = state.read_register::<u8>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // C s_8_2: const #2u : u8
        let s_8_2: u8 = 2;
        // D s_8_3: cmp-lt s_8_1 s_8_2
        let s_8_3: bool = ((s_8_1) < (s_8_2));
        // N s_8_4: branch s_8_3 b13 b9
        if s_8_3 {
            return block_13(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#106932 <= s_9_0
        fn_state.gs_106932 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#106932:u8
        let s_10_0: bool = fn_state.gs_106932;
        // N s_10_1: branch s_10_0 b12 b11
        if s_10_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call CNTP_CTL_read(s_11_0)
        let s_11_1: ProductType700c18a878c5601b = CNTP_CTL_read(state, tracer, s_11_0);
        // S s_11_2: call __get_CNTP_CTL(s_11_1)
        let s_11_2: ProductType700c18a878c5601b = u__get_CNTP_CTL(state, tracer, s_11_1);
        // D s_11_3: write-var ga#166522 <= s_11_2
        fn_state.ga_166522 = s_11_2;
        // D s_11_4: read-var ga#166522.0:struct
        let s_11_4: u32 = fn_state.ga_166522._0;
        // D s_11_5: read-var t:i
        let s_11_5: i128 = fn_state.t;
        // D s_11_6: call R_set(s_11_5, s_11_4)
        let s_11_6: () = R_set(state, tracer, s_11_5, s_11_4);
        // N s_11_7: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call CNTP_CTL_NS_read(s_12_0)
        let s_12_1: ProductType700c18a878c5601b = CNTP_CTL_NS_read(
            state,
            tracer,
            s_12_0,
        );
        // S s_12_2: call __get_CNTP_CTL_NS(s_12_1)
        let s_12_2: ProductType700c18a878c5601b = u__get_CNTP_CTL_NS(
            state,
            tracer,
            s_12_1,
        );
        // D s_12_3: write-var ga#166519 <= s_12_2
        fn_state.ga_166519 = s_12_2;
        // D s_12_4: read-var ga#166519.0:struct
        let s_12_4: u32 = fn_state.ga_166519._0;
        // D s_12_5: read-var t:i
        let s_12_5: i128 = fn_state.t;
        // D s_12_6: call R_set(s_12_5, s_12_4)
        let s_12_6: () = R_set(state, tracer, s_12_5, s_12_4);
        // N s_12_7: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #424u : u32
        let s_13_0: u32 = 424;
        // D s_13_1: read-reg s_13_0:u8
        let s_13_1: u8 = {
            let value = state.read_register::<u8>(s_13_0 as isize);
            tracer.read_register(s_13_0 as isize, value);
            value
        };
        // D s_13_2: call ELUsingAArch32(s_13_1)
        let s_13_2: bool = ELUsingAArch32(state, tracer, s_13_1);
        // D s_13_3: write-var gs#106932 <= s_13_2
        fn_state.gs_106932 = s_13_2;
        // N s_13_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call EL2Enabled(s_14_0)
        let s_14_1: bool = EL2Enabled(state, tracer, s_14_0);
        // N s_14_2: branch s_14_1 b49 b15
        if s_14_1 {
            return block_49(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#106933 <= s_15_0
        fn_state.gs_106933 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#106933:u8
        let s_16_0: bool = fn_state.gs_106933;
        // N s_16_1: branch s_16_0 b48 b17
        if s_16_0 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#106934 <= s_17_0
        fn_state.gs_106934 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#106934:u8
        let s_18_0: bool = fn_state.gs_106934;
        // N s_18_1: branch s_18_0 b47 b19
        if s_18_0 {
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
        // D s_19_1: write-var gs#106935 <= s_19_0
        fn_state.gs_106935 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#106935:u8
        let s_20_0: bool = fn_state.gs_106935;
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
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call EL2Enabled(s_21_0)
        let s_21_1: bool = EL2Enabled(state, tracer, s_21_0);
        // N s_21_2: branch s_21_1 b45 b22
        if s_21_1 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var gs#106936 <= s_22_0
        fn_state.gs_106936 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#106936:u8
        let s_23_0: bool = fn_state.gs_106936;
        // N s_23_1: branch s_23_0 b44 b24
        if s_23_0 {
            return block_44(state, tracer, fn_state);
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
        // D s_24_1: write-var gs#106937 <= s_24_0
        fn_state.gs_106937 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#106937:u8
        let s_25_0: bool = fn_state.gs_106937;
        // N s_25_1: branch s_25_0 b43 b26
        if s_25_0 {
            return block_43(state, tracer, fn_state);
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
        // D s_26_1: write-var gs#106938 <= s_26_0
        fn_state.gs_106938 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#106938:u8
        let s_27_0: bool = fn_state.gs_106938;
        // N s_27_1: branch s_27_0 b42 b28
        if s_27_0 {
            return block_42(state, tracer, fn_state);
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
        // S s_28_1: call EL2Enabled(s_28_0)
        let s_28_1: bool = EL2Enabled(state, tracer, s_28_0);
        // N s_28_2: branch s_28_1 b41 b29
        if s_28_1 {
            return block_41(state, tracer, fn_state);
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
        // D s_29_1: write-var gs#106939 <= s_29_0
        fn_state.gs_106939 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#106939:u8
        let s_30_0: bool = fn_state.gs_106939;
        // N s_30_1: branch s_30_0 b40 b31
        if s_30_0 {
            return block_40(state, tracer, fn_state);
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
        // D s_31_1: write-var gs#106940 <= s_31_0
        fn_state.gs_106940 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#106940:u8
        let s_32_0: bool = fn_state.gs_106940;
        // N s_32_1: branch s_32_0 b39 b33
        if s_32_0 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
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
        // C s_33_2: const #2u : u8
        let s_33_2: u8 = 2;
        // D s_33_3: cmp-lt s_33_1 s_33_2
        let s_33_3: bool = ((s_33_1) < (s_33_2));
        // N s_33_4: branch s_33_3 b38 b34
        if s_33_3 {
            return block_38(state, tracer, fn_state);
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
        // D s_34_1: write-var gs#106941 <= s_34_0
        fn_state.gs_106941 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#106941:u8
        let s_35_0: bool = fn_state.gs_106941;
        // N s_35_1: branch s_35_0 b37 b36
        if s_35_0 {
            return block_37(state, tracer, fn_state);
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
        // S s_36_1: call CNTP_CTL_read(s_36_0)
        let s_36_1: ProductType700c18a878c5601b = CNTP_CTL_read(state, tracer, s_36_0);
        // S s_36_2: call __get_CNTP_CTL(s_36_1)
        let s_36_2: ProductType700c18a878c5601b = u__get_CNTP_CTL(state, tracer, s_36_1);
        // D s_36_3: write-var ga#166513 <= s_36_2
        fn_state.ga_166513 = s_36_2;
        // D s_36_4: read-var ga#166513.0:struct
        let s_36_4: u32 = fn_state.ga_166513._0;
        // D s_36_5: read-var t:i
        let s_36_5: i128 = fn_state.t;
        // D s_36_6: call R_set(s_36_5, s_36_4)
        let s_36_6: () = R_set(state, tracer, s_36_5, s_36_4);
        // N s_36_7: return
        return;
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #() : ()
        let s_37_0: () = ();
        // S s_37_1: call CNTP_CTL_NS_read(s_37_0)
        let s_37_1: ProductType700c18a878c5601b = CNTP_CTL_NS_read(
            state,
            tracer,
            s_37_0,
        );
        // S s_37_2: call __get_CNTP_CTL_NS(s_37_1)
        let s_37_2: ProductType700c18a878c5601b = u__get_CNTP_CTL_NS(
            state,
            tracer,
            s_37_1,
        );
        // D s_37_3: write-var ga#166510 <= s_37_2
        fn_state.ga_166510 = s_37_2;
        // D s_37_4: read-var ga#166510.0:struct
        let s_37_4: u32 = fn_state.ga_166510._0;
        // D s_37_5: read-var t:i
        let s_37_5: i128 = fn_state.t;
        // D s_37_6: call R_set(s_37_5, s_37_4)
        let s_37_6: () = R_set(state, tracer, s_37_5, s_37_4);
        // N s_37_7: return
        return;
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #424u : u32
        let s_38_0: u32 = 424;
        // D s_38_1: read-reg s_38_0:u8
        let s_38_1: u8 = {
            let value = state.read_register::<u8>(s_38_0 as isize);
            tracer.read_register(s_38_0 as isize, value);
            value
        };
        // D s_38_2: call ELUsingAArch32(s_38_1)
        let s_38_2: bool = ELUsingAArch32(state, tracer, s_38_1);
        // D s_38_3: write-var gs#106941 <= s_38_2
        fn_state.gs_106941 = s_38_2;
        // N s_38_4: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #3u : u8
        let s_39_0: u8 = 3;
        // C s_39_1: cast zx s_39_0 -> bv
        let s_39_1: Bits = Bits::new(s_39_0 as u128, 8u16);
        // C s_39_2: cast zx s_39_1 -> i
        let s_39_2: i128 = (s_39_1.value() as i128);
        // C s_39_3: cast reint s_39_2 -> i64
        let s_39_3: i64 = (s_39_2 as i64);
        // C s_39_4: cast zx s_39_3 -> i
        let s_39_4: i128 = (i128::try_from(s_39_3).unwrap());
        // S s_39_5: call AArch32_TakeHypTrapException(s_39_4)
        let s_39_5: () = AArch32_TakeHypTrapException(state, tracer, s_39_4);
        // N s_39_6: return
        return;
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var __CNTHCTL_PL1PCEN:u8
        let s_40_0: bool = fn_state.u__CNTHCTL_PL1PCEN;
        // D s_40_1: cast zx s_40_0 -> bv
        let s_40_1: Bits = Bits::new(s_40_0 as u128, 1u16);
        // C s_40_2: const #0u : u8
        let s_40_2: bool = false;
        // C s_40_3: cast zx s_40_2 -> bv
        let s_40_3: Bits = Bits::new(s_40_2 as u128, 1u16);
        // D s_40_4: cmp-eq s_40_1 s_40_3
        let s_40_4: bool = ((s_40_1) == (s_40_3));
        // D s_40_5: write-var gs#106940 <= s_40_4
        fn_state.gs_106940 = s_40_4;
        // N s_40_6: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #432u : u32
        let s_41_0: u32 = 432;
        // D s_41_1: read-reg s_41_0:u8
        let s_41_1: u8 = {
            let value = state.read_register::<u8>(s_41_0 as isize);
            tracer.read_register(s_41_0 as isize, value);
            value
        };
        // D s_41_2: call ELUsingAArch32(s_41_1)
        let s_41_2: bool = ELUsingAArch32(state, tracer, s_41_1);
        // D s_41_3: write-var gs#106939 <= s_41_2
        fn_state.gs_106939 = s_41_2;
        // N s_41_4: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #3u : u8
        let s_42_0: u8 = 3;
        // C s_42_1: cast zx s_42_0 -> bv
        let s_42_1: Bits = Bits::new(s_42_0 as u128, 8u16);
        // C s_42_2: cast zx s_42_1 -> i
        let s_42_2: i128 = (s_42_1.value() as i128);
        // C s_42_3: cast reint s_42_2 -> i64
        let s_42_3: i64 = (s_42_2 as i64);
        // C s_42_4: cast zx s_42_3 -> i
        let s_42_4: i128 = (i128::try_from(s_42_3).unwrap());
        // C s_42_5: const #432u : u32
        let s_42_5: u32 = 432;
        // D s_42_6: read-reg s_42_5:u8
        let s_42_6: u8 = {
            let value = state.read_register::<u8>(s_42_5 as isize);
            tracer.read_register(s_42_5 as isize, value);
            value
        };
        // D s_42_7: call AArch64_AArch32SystemAccessTrap(s_42_6, s_42_4)
        let s_42_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_42_6, s_42_4);
        // N s_42_8: return
        return;
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var __CNTHCTL_EL2_EL1PTEN:u8
        let s_43_0: bool = fn_state.u__CNTHCTL_EL2_EL1PTEN;
        // D s_43_1: cast zx s_43_0 -> bv
        let s_43_1: Bits = Bits::new(s_43_0 as u128, 1u16);
        // C s_43_2: const #0u : u8
        let s_43_2: bool = false;
        // C s_43_3: cast zx s_43_2 -> bv
        let s_43_3: Bits = Bits::new(s_43_2 as u128, 1u16);
        // D s_43_4: cmp-eq s_43_1 s_43_3
        let s_43_4: bool = ((s_43_1) == (s_43_3));
        // D s_43_5: write-var gs#106938 <= s_43_4
        fn_state.gs_106938 = s_43_4;
        // N s_43_6: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var __HCR_EL2_E2H:u8
        let s_44_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_44_1: cast zx s_44_0 -> bv
        let s_44_1: Bits = Bits::new(s_44_0 as u128, 1u16);
        // C s_44_2: const #1u : u8
        let s_44_2: bool = true;
        // C s_44_3: cast zx s_44_2 -> bv
        let s_44_3: Bits = Bits::new(s_44_2 as u128, 1u16);
        // D s_44_4: cmp-eq s_44_1 s_44_3
        let s_44_4: bool = ((s_44_1) == (s_44_3));
        // D s_44_5: write-var gs#106937 <= s_44_4
        fn_state.gs_106937 = s_44_4;
        // N s_44_6: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #432u : u32
        let s_45_0: u32 = 432;
        // D s_45_1: read-reg s_45_0:u8
        let s_45_1: u8 = {
            let value = state.read_register::<u8>(s_45_0 as isize);
            tracer.read_register(s_45_0 as isize, value);
            value
        };
        // D s_45_2: call ELUsingAArch32(s_45_1)
        let s_45_2: bool = ELUsingAArch32(state, tracer, s_45_1);
        // D s_45_3: not s_45_2
        let s_45_3: bool = !s_45_2;
        // D s_45_4: write-var gs#106936 <= s_45_3
        fn_state.gs_106936 = s_45_3;
        // N s_45_5: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #3u : u8
        let s_46_0: u8 = 3;
        // C s_46_1: cast zx s_46_0 -> bv
        let s_46_1: Bits = Bits::new(s_46_0 as u128, 8u16);
        // C s_46_2: cast zx s_46_1 -> i
        let s_46_2: i128 = (s_46_1.value() as i128);
        // C s_46_3: cast reint s_46_2 -> i64
        let s_46_3: i64 = (s_46_2 as i64);
        // C s_46_4: cast zx s_46_3 -> i
        let s_46_4: i128 = (i128::try_from(s_46_3).unwrap());
        // C s_46_5: const #432u : u32
        let s_46_5: u32 = 432;
        // D s_46_6: read-reg s_46_5:u8
        let s_46_6: u8 = {
            let value = state.read_register::<u8>(s_46_5 as isize);
            tracer.read_register(s_46_5 as isize, value);
            value
        };
        // D s_46_7: call AArch64_AArch32SystemAccessTrap(s_46_6, s_46_4)
        let s_46_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_46_6, s_46_4);
        // N s_46_8: return
        return;
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var __CNTHCTL_EL2_EL1PCEN:u8
        let s_47_0: bool = fn_state.u__CNTHCTL_EL2_EL1PCEN;
        // D s_47_1: cast zx s_47_0 -> bv
        let s_47_1: Bits = Bits::new(s_47_0 as u128, 1u16);
        // C s_47_2: const #0u : u8
        let s_47_2: bool = false;
        // C s_47_3: cast zx s_47_2 -> bv
        let s_47_3: Bits = Bits::new(s_47_2 as u128, 1u16);
        // D s_47_4: cmp-eq s_47_1 s_47_3
        let s_47_4: bool = ((s_47_1) == (s_47_3));
        // D s_47_5: write-var gs#106935 <= s_47_4
        fn_state.gs_106935 = s_47_4;
        // N s_47_6: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var __HCR_EL2_E2H:u8
        let s_48_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_48_1: cast zx s_48_0 -> bv
        let s_48_1: Bits = Bits::new(s_48_0 as u128, 1u16);
        // C s_48_2: const #0u : u8
        let s_48_2: bool = false;
        // C s_48_3: cast zx s_48_2 -> bv
        let s_48_3: Bits = Bits::new(s_48_2 as u128, 1u16);
        // D s_48_4: cmp-eq s_48_1 s_48_3
        let s_48_4: bool = ((s_48_1) == (s_48_3));
        // D s_48_5: write-var gs#106934 <= s_48_4
        fn_state.gs_106934 = s_48_4;
        // N s_48_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #432u : u32
        let s_49_0: u32 = 432;
        // D s_49_1: read-reg s_49_0:u8
        let s_49_1: u8 = {
            let value = state.read_register::<u8>(s_49_0 as isize);
            tracer.read_register(s_49_0 as isize, value);
            value
        };
        // D s_49_2: call ELUsingAArch32(s_49_1)
        let s_49_2: bool = ELUsingAArch32(state, tracer, s_49_1);
        // D s_49_3: not s_49_2
        let s_49_3: bool = !s_49_2;
        // D s_49_4: write-var gs#106933 <= s_49_3
        fn_state.gs_106933 = s_49_3;
        // N s_49_5: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #440u : u32
        let s_50_0: u32 = 440;
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
        // N s_50_4: branch s_50_3 b153 b51
        if s_50_3 {
            return block_153(state, tracer, fn_state);
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
        // D s_51_1: write-var gs#106943 <= s_51_0
        fn_state.gs_106943 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#106943:u8
        let s_52_0: bool = fn_state.gs_106943;
        // N s_52_1: branch s_52_0 b152 b53
        if s_52_0 {
            return block_152(state, tracer, fn_state);
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
        // D s_53_1: write-var gs#106944 <= s_53_0
        fn_state.gs_106944 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#106944:u8
        let s_54_0: bool = fn_state.gs_106944;
        // N s_54_1: branch s_54_0 b143 b55
        if s_54_0 {
            return block_143(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #440u : u32
        let s_55_0: u32 = 440;
        // D s_55_1: read-reg s_55_0:u8
        let s_55_1: u8 = {
            let value = state.read_register::<u8>(s_55_0 as isize);
            tracer.read_register(s_55_0 as isize, value);
            value
        };
        // D s_55_2: call ELUsingAArch32(s_55_1)
        let s_55_2: bool = ELUsingAArch32(state, tracer, s_55_1);
        // N s_55_3: branch s_55_2 b142 b56
        if s_55_2 {
            return block_142(state, tracer, fn_state);
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
        // D s_56_1: write-var gs#106945 <= s_56_0
        fn_state.gs_106945 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#106945:u8
        let s_57_0: bool = fn_state.gs_106945;
        // N s_57_1: branch s_57_0 b125 b58
        if s_57_0 {
            return block_125(state, tracer, fn_state);
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
        // N s_58_2: branch s_58_1 b124 b59
        if s_58_1 {
            return block_124(state, tracer, fn_state);
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
        // D s_59_1: write-var gs#106946 <= s_59_0
        fn_state.gs_106946 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#106946:u8
        let s_60_0: bool = fn_state.gs_106946;
        // N s_60_1: branch s_60_0 b123 b61
        if s_60_0 {
            return block_123(state, tracer, fn_state);
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
        // D s_61_1: write-var gs#106947 <= s_61_0
        fn_state.gs_106947 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#106947:u8
        let s_62_0: bool = fn_state.gs_106947;
        // N s_62_1: branch s_62_0 b122 b63
        if s_62_0 {
            return block_122(state, tracer, fn_state);
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
        // D s_63_1: write-var gs#106948 <= s_63_0
        fn_state.gs_106948 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#106948:u8
        let s_64_0: bool = fn_state.gs_106948;
        // N s_64_1: branch s_64_0 b121 b65
        if s_64_0 {
            return block_121(state, tracer, fn_state);
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
        // N s_65_2: branch s_65_1 b120 b66
        if s_65_1 {
            return block_120(state, tracer, fn_state);
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
        // D s_66_1: write-var gs#106949 <= s_66_0
        fn_state.gs_106949 = s_66_0;
        // N s_66_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#106949:u8
        let s_67_0: bool = fn_state.gs_106949;
        // N s_67_1: branch s_67_0 b119 b68
        if s_67_0 {
            return block_119(state, tracer, fn_state);
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
        // D s_68_1: write-var gs#106950 <= s_68_0
        fn_state.gs_106950 = s_68_0;
        // N s_68_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var gs#106950:u8
        let s_69_0: bool = fn_state.gs_106950;
        // N s_69_1: branch s_69_0 b118 b70
        if s_69_0 {
            return block_118(state, tracer, fn_state);
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
        // D s_70_1: write-var gs#106951 <= s_70_0
        fn_state.gs_106951 = s_70_0;
        // N s_70_2: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var gs#106951:u8
        let s_71_0: bool = fn_state.gs_106951;
        // N s_71_1: branch s_71_0 b117 b72
        if s_71_0 {
            return block_117(state, tracer, fn_state);
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
        // N s_72_2: branch s_72_1 b116 b73
        if s_72_1 {
            return block_116(state, tracer, fn_state);
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
        // D s_73_1: write-var gs#106952 <= s_73_0
        fn_state.gs_106952 = s_73_0;
        // N s_73_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var gs#106952:u8
        let s_74_0: bool = fn_state.gs_106952;
        // N s_74_1: branch s_74_0 b115 b75
        if s_74_0 {
            return block_115(state, tracer, fn_state);
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
        // D s_75_1: write-var gs#106953 <= s_75_0
        fn_state.gs_106953 = s_75_0;
        // N s_75_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var gs#106953:u8
        let s_76_0: bool = fn_state.gs_106953;
        // N s_76_1: branch s_76_0 b114 b77
        if s_76_0 {
            return block_114(state, tracer, fn_state);
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
        // D s_77_1: write-var gs#106954 <= s_77_0
        fn_state.gs_106954 = s_77_0;
        // N s_77_2: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var gs#106954:u8
        let s_78_0: bool = fn_state.gs_106954;
        // N s_78_1: branch s_78_0 b113 b79
        if s_78_0 {
            return block_113(state, tracer, fn_state);
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
        // N s_79_2: branch s_79_1 b112 b80
        if s_79_1 {
            return block_112(state, tracer, fn_state);
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
        // D s_80_1: write-var gs#106955 <= s_80_0
        fn_state.gs_106955 = s_80_0;
        // N s_80_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#106955:u8
        let s_81_0: bool = fn_state.gs_106955;
        // N s_81_1: branch s_81_0 b111 b82
        if s_81_0 {
            return block_111(state, tracer, fn_state);
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
        // D s_82_1: write-var gs#106956 <= s_82_0
        fn_state.gs_106956 = s_82_0;
        // N s_82_2: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_83_0: read-var gs#106956:u8
        let s_83_0: bool = fn_state.gs_106956;
        // N s_83_1: branch s_83_0 b110 b84
        if s_83_0 {
            return block_110(state, tracer, fn_state);
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
        // N s_84_2: branch s_84_1 b109 b85
        if s_84_1 {
            return block_109(state, tracer, fn_state);
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
        // D s_85_1: write-var gs#106957 <= s_85_0
        fn_state.gs_106957 = s_85_0;
        // N s_85_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var gs#106957:u8
        let s_86_0: bool = fn_state.gs_106957;
        // N s_86_1: branch s_86_0 b108 b87
        if s_86_0 {
            return block_108(state, tracer, fn_state);
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
        // D s_87_1: write-var gs#106958 <= s_87_0
        fn_state.gs_106958 = s_87_0;
        // N s_87_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var gs#106958:u8
        let s_88_0: bool = fn_state.gs_106958;
        // N s_88_1: branch s_88_0 b107 b89
        if s_88_0 {
            return block_107(state, tracer, fn_state);
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
        // D s_89_1: write-var gs#106959 <= s_89_0
        fn_state.gs_106959 = s_89_0;
        // N s_89_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var gs#106959:u8
        let s_90_0: bool = fn_state.gs_106959;
        // N s_90_1: branch s_90_0 b106 b91
        if s_90_0 {
            return block_106(state, tracer, fn_state);
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
        // D s_91_1: write-var gs#106960 <= s_91_0
        fn_state.gs_106960 = s_91_0;
        // N s_91_2: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var gs#106960:u8
        let s_92_0: bool = fn_state.gs_106960;
        // N s_92_1: branch s_92_0 b105 b93
        if s_92_0 {
            return block_105(state, tracer, fn_state);
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
        // N s_93_2: branch s_93_1 b104 b94
        if s_93_1 {
            return block_104(state, tracer, fn_state);
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
        // D s_94_1: write-var gs#106961 <= s_94_0
        fn_state.gs_106961 = s_94_0;
        // N s_94_2: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_95_0: read-var gs#106961:u8
        let s_95_0: bool = fn_state.gs_106961;
        // N s_95_1: branch s_95_0 b103 b96
        if s_95_0 {
            return block_103(state, tracer, fn_state);
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
        // D s_96_1: write-var gs#106962 <= s_96_0
        fn_state.gs_106962 = s_96_0;
        // N s_96_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var gs#106962:u8
        let s_97_0: bool = fn_state.gs_106962;
        // N s_97_1: branch s_97_0 b102 b98
        if s_97_0 {
            return block_102(state, tracer, fn_state);
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
        // D s_98_1: write-var gs#106963 <= s_98_0
        fn_state.gs_106963 = s_98_0;
        // N s_98_2: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_99_0: read-var gs#106963:u8
        let s_99_0: bool = fn_state.gs_106963;
        // N s_99_1: branch s_99_0 b101 b100
        if s_99_0 {
            return block_101(state, tracer, fn_state);
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
        // S s_100_1: call CNTP_CTL_read(s_100_0)
        let s_100_1: ProductType700c18a878c5601b = CNTP_CTL_read(state, tracer, s_100_0);
        // S s_100_2: call __get_CNTP_CTL(s_100_1)
        let s_100_2: ProductType700c18a878c5601b = u__get_CNTP_CTL(
            state,
            tracer,
            s_100_1,
        );
        // D s_100_3: write-var ga#166488 <= s_100_2
        fn_state.ga_166488 = s_100_2;
        // D s_100_4: read-var ga#166488.0:struct
        let s_100_4: u32 = fn_state.ga_166488._0;
        // D s_100_5: read-var t:i
        let s_100_5: i128 = fn_state.t;
        // D s_100_6: call R_set(s_100_5, s_100_4)
        let s_100_6: () = R_set(state, tracer, s_100_5, s_100_4);
        // N s_100_7: return
        return;
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #100912u : u32
        let s_101_0: u32 = 100912;
        // D s_101_1: read-reg s_101_0:struct
        let s_101_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_101_0 as isize);
            tracer.read_register(s_101_0 as isize, value);
            value
        };
        // D s_101_2: call __get_CNTHP_CTL_EL2(s_101_1)
        let s_101_2: ProductType5c790c8ef59cc8b2 = u__get_CNTHP_CTL_EL2(
            state,
            tracer,
            s_101_1,
        );
        // D s_101_3: write-var ga#166484 <= s_101_2
        fn_state.ga_166484 = s_101_2;
        // D s_101_4: read-var ga#166484.0:struct
        let s_101_4: u64 = fn_state.ga_166484._0;
        // C s_101_5: const #0s : i
        let s_101_5: i128 = 0;
        // D s_101_6: cast zx s_101_4 -> bv
        let s_101_6: Bits = Bits::new(s_101_4 as u128, 64u16);
        // C s_101_7: const #1s : i64
        let s_101_7: i64 = 1;
        // C s_101_8: cast zx s_101_7 -> i
        let s_101_8: i128 = (i128::try_from(s_101_7).unwrap());
        // C s_101_9: const #31s : i
        let s_101_9: i128 = 31;
        // C s_101_10: add s_101_9 s_101_8
        let s_101_10: i128 = (s_101_9 + s_101_8);
        // D s_101_11: bit-extract s_101_6 s_101_5 s_101_10
        let s_101_11: Bits = (Bits::new(
            ((s_101_6) >> (s_101_5)).value(),
            u16::try_from(s_101_10).unwrap(),
        ));
        // D s_101_12: cast reint s_101_11 -> u32
        let s_101_12: u32 = (s_101_11.value() as u32);
        // D s_101_13: read-var t:i
        let s_101_13: i128 = fn_state.t;
        // D s_101_14: call R_set(s_101_13, s_101_12)
        let s_101_14: () = R_set(state, tracer, s_101_13, s_101_12);
        // N s_101_15: return
        return;
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #90704u : u32
        let s_102_0: u32 = 90704;
        // D s_102_1: read-reg s_102_0:struct
        let s_102_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_102_0 as isize);
            tracer.read_register(s_102_0 as isize, value);
            value
        };
        // D s_102_2: call _get_SCR_EL3_Type_NS(s_102_1)
        let s_102_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_102_1);
        // D s_102_3: cast zx s_102_2 -> bv
        let s_102_3: Bits = Bits::new(s_102_2 as u128, 1u16);
        // C s_102_4: const #1u : u8
        let s_102_4: bool = true;
        // C s_102_5: cast zx s_102_4 -> bv
        let s_102_5: Bits = Bits::new(s_102_4 as u128, 1u16);
        // D s_102_6: cmp-eq s_102_3 s_102_5
        let s_102_6: bool = ((s_102_3) == (s_102_5));
        // D s_102_7: write-var gs#106963 <= s_102_6
        fn_state.gs_106963 = s_102_6;
        // N s_102_8: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #102552u : u32
        let s_103_0: u32 = 102552;
        // D s_103_1: read-reg s_103_0:struct
        let s_103_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_103_0 as isize);
            tracer.read_register(s_103_0 as isize, value);
            value
        };
        // D s_103_2: call _get_HCR_EL2_Type_E2H(s_103_1)
        let s_103_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_103_1);
        // C s_103_3: const #102552u : u32
        let s_103_3: u32 = 102552;
        // D s_103_4: read-reg s_103_3:struct
        let s_103_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_103_3 as isize);
            tracer.read_register(s_103_3 as isize, value);
            value
        };
        // D s_103_5: call _get_HCR_EL2_Type_TGE(s_103_4)
        let s_103_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_103_4);
        // D s_103_6: cast zx s_103_2 -> bv
        let s_103_6: Bits = Bits::new(s_103_2 as u128, 1u16);
        // D s_103_7: cast zx s_103_5 -> bv
        let s_103_7: Bits = Bits::new(s_103_5 as u128, 1u16);
        // D s_103_8: cast reint s_103_6 -> u128
        let s_103_8: u128 = (s_103_6.value() as u128);
        // D s_103_9: size-of s_103_6
        let s_103_9: u16 = s_103_6.length();
        // D s_103_10: cast reint s_103_7 -> u128
        let s_103_10: u128 = (s_103_7.value() as u128);
        // D s_103_11: size-of s_103_7
        let s_103_11: u16 = s_103_7.length();
        // D s_103_12: lsl s_103_8 s_103_11
        let s_103_12: u128 = s_103_8 << s_103_11;
        // D s_103_13: or s_103_12 s_103_10
        let s_103_13: u128 = ((s_103_12) | (s_103_10));
        // D s_103_14: add s_103_9 s_103_11
        let s_103_14: u16 = (s_103_9 + s_103_11);
        // D s_103_15: create-bits s_103_13 s_103_14
        let s_103_15: Bits = Bits::new(s_103_13, s_103_14);
        // D s_103_16: cast reint s_103_15 -> u8
        let s_103_16: u8 = (s_103_15.value() as u8);
        // D s_103_17: cast zx s_103_16 -> bv
        let s_103_17: Bits = Bits::new(s_103_16 as u128, 2u16);
        // C s_103_18: const #3u : u8
        let s_103_18: u8 = 3;
        // C s_103_19: cast zx s_103_18 -> bv
        let s_103_19: Bits = Bits::new(s_103_18 as u128, 2u16);
        // D s_103_20: cmp-eq s_103_17 s_103_19
        let s_103_20: bool = ((s_103_17) == (s_103_19));
        // D s_103_21: write-var gs#106962 <= s_103_20
        fn_state.gs_106962 = s_103_20;
        // N s_103_22: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #432u : u32
        let s_104_0: u32 = 432;
        // D s_104_1: read-reg s_104_0:u8
        let s_104_1: u8 = {
            let value = state.read_register::<u8>(s_104_0 as isize);
            tracer.read_register(s_104_0 as isize, value);
            value
        };
        // D s_104_2: call ELUsingAArch32(s_104_1)
        let s_104_2: bool = ELUsingAArch32(state, tracer, s_104_1);
        // D s_104_3: not s_104_2
        let s_104_3: bool = !s_104_2;
        // D s_104_4: write-var gs#106961 <= s_104_3
        fn_state.gs_106961 = s_104_3;
        // N s_104_5: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #10504u : u32
        let s_105_0: u32 = 10504;
        // D s_105_1: read-reg s_105_0:struct
        let s_105_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_105_0 as isize);
            tracer.read_register(s_105_0 as isize, value);
            value
        };
        // D s_105_2: call __get_CNTHPS_CTL_EL2(s_105_1)
        let s_105_2: ProductType5c790c8ef59cc8b2 = u__get_CNTHPS_CTL_EL2(
            state,
            tracer,
            s_105_1,
        );
        // D s_105_3: write-var ga#166472 <= s_105_2
        fn_state.ga_166472 = s_105_2;
        // D s_105_4: read-var ga#166472.0:struct
        let s_105_4: u64 = fn_state.ga_166472._0;
        // C s_105_5: const #0s : i
        let s_105_5: i128 = 0;
        // D s_105_6: cast zx s_105_4 -> bv
        let s_105_6: Bits = Bits::new(s_105_4 as u128, 64u16);
        // C s_105_7: const #1s : i64
        let s_105_7: i64 = 1;
        // C s_105_8: cast zx s_105_7 -> i
        let s_105_8: i128 = (i128::try_from(s_105_7).unwrap());
        // C s_105_9: const #31s : i
        let s_105_9: i128 = 31;
        // C s_105_10: add s_105_9 s_105_8
        let s_105_10: i128 = (s_105_9 + s_105_8);
        // D s_105_11: bit-extract s_105_6 s_105_5 s_105_10
        let s_105_11: Bits = (Bits::new(
            ((s_105_6) >> (s_105_5)).value(),
            u16::try_from(s_105_10).unwrap(),
        ));
        // D s_105_12: cast reint s_105_11 -> u32
        let s_105_12: u32 = (s_105_11.value() as u32);
        // D s_105_13: read-var t:i
        let s_105_13: i128 = fn_state.t;
        // D s_105_14: call R_set(s_105_13, s_105_12)
        let s_105_14: () = R_set(state, tracer, s_105_13, s_105_12);
        // N s_105_15: return
        return;
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #117u : u32
        let s_106_0: u32 = 117;
        // S s_106_1: call IsFeatureImplemented(s_106_0)
        let s_106_1: bool = IsFeatureImplemented(state, tracer, s_106_0);
        // D s_106_2: write-var gs#106960 <= s_106_1
        fn_state.gs_106960 = s_106_1;
        // N s_106_3: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #90704u : u32
        let s_107_0: u32 = 90704;
        // D s_107_1: read-reg s_107_0:struct
        let s_107_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_107_0 as isize);
            tracer.read_register(s_107_0 as isize, value);
            value
        };
        // D s_107_2: call _get_SCR_EL3_Type_NS(s_107_1)
        let s_107_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_107_1);
        // D s_107_3: cast zx s_107_2 -> bv
        let s_107_3: Bits = Bits::new(s_107_2 as u128, 1u16);
        // C s_107_4: const #0u : u8
        let s_107_4: bool = false;
        // C s_107_5: cast zx s_107_4 -> bv
        let s_107_5: Bits = Bits::new(s_107_4 as u128, 1u16);
        // D s_107_6: cmp-eq s_107_3 s_107_5
        let s_107_6: bool = ((s_107_3) == (s_107_5));
        // D s_107_7: write-var gs#106959 <= s_107_6
        fn_state.gs_106959 = s_107_6;
        // N s_107_8: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #102552u : u32
        let s_108_0: u32 = 102552;
        // D s_108_1: read-reg s_108_0:struct
        let s_108_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_108_0 as isize);
            tracer.read_register(s_108_0 as isize, value);
            value
        };
        // D s_108_2: call _get_HCR_EL2_Type_E2H(s_108_1)
        let s_108_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_108_1);
        // C s_108_3: const #102552u : u32
        let s_108_3: u32 = 102552;
        // D s_108_4: read-reg s_108_3:struct
        let s_108_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_108_3 as isize);
            tracer.read_register(s_108_3 as isize, value);
            value
        };
        // D s_108_5: call _get_HCR_EL2_Type_TGE(s_108_4)
        let s_108_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_108_4);
        // D s_108_6: cast zx s_108_2 -> bv
        let s_108_6: Bits = Bits::new(s_108_2 as u128, 1u16);
        // D s_108_7: cast zx s_108_5 -> bv
        let s_108_7: Bits = Bits::new(s_108_5 as u128, 1u16);
        // D s_108_8: cast reint s_108_6 -> u128
        let s_108_8: u128 = (s_108_6.value() as u128);
        // D s_108_9: size-of s_108_6
        let s_108_9: u16 = s_108_6.length();
        // D s_108_10: cast reint s_108_7 -> u128
        let s_108_10: u128 = (s_108_7.value() as u128);
        // D s_108_11: size-of s_108_7
        let s_108_11: u16 = s_108_7.length();
        // D s_108_12: lsl s_108_8 s_108_11
        let s_108_12: u128 = s_108_8 << s_108_11;
        // D s_108_13: or s_108_12 s_108_10
        let s_108_13: u128 = ((s_108_12) | (s_108_10));
        // D s_108_14: add s_108_9 s_108_11
        let s_108_14: u16 = (s_108_9 + s_108_11);
        // D s_108_15: create-bits s_108_13 s_108_14
        let s_108_15: Bits = Bits::new(s_108_13, s_108_14);
        // D s_108_16: cast reint s_108_15 -> u8
        let s_108_16: u8 = (s_108_15.value() as u8);
        // D s_108_17: cast zx s_108_16 -> bv
        let s_108_17: Bits = Bits::new(s_108_16 as u128, 2u16);
        // C s_108_18: const #3u : u8
        let s_108_18: u8 = 3;
        // C s_108_19: cast zx s_108_18 -> bv
        let s_108_19: Bits = Bits::new(s_108_18 as u128, 2u16);
        // D s_108_20: cmp-eq s_108_17 s_108_19
        let s_108_20: bool = ((s_108_17) == (s_108_19));
        // D s_108_21: write-var gs#106958 <= s_108_20
        fn_state.gs_106958 = s_108_20;
        // N s_108_22: jump b88
        return block_88(state, tracer, fn_state);
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
        // D s_109_4: write-var gs#106957 <= s_109_3
        fn_state.gs_106957 = s_109_3;
        // N s_109_5: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_110_0: const #3u : u8
        let s_110_0: u8 = 3;
        // C s_110_1: cast zx s_110_0 -> bv
        let s_110_1: Bits = Bits::new(s_110_0 as u128, 8u16);
        // C s_110_2: cast zx s_110_1 -> i
        let s_110_2: i128 = (s_110_1.value() as i128);
        // C s_110_3: cast reint s_110_2 -> i64
        let s_110_3: i64 = (s_110_2 as i64);
        // C s_110_4: cast zx s_110_3 -> i
        let s_110_4: i128 = (i128::try_from(s_110_3).unwrap());
        // S s_110_5: call AArch32_TakeHypTrapException(s_110_4)
        let s_110_5: () = AArch32_TakeHypTrapException(state, tracer, s_110_4);
        // N s_110_6: return
        return;
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var __CNTHCTL_PL1PCEN:u8
        let s_111_0: bool = fn_state.u__CNTHCTL_PL1PCEN;
        // D s_111_1: cast zx s_111_0 -> bv
        let s_111_1: Bits = Bits::new(s_111_0 as u128, 1u16);
        // C s_111_2: const #0u : u8
        let s_111_2: bool = false;
        // C s_111_3: cast zx s_111_2 -> bv
        let s_111_3: Bits = Bits::new(s_111_2 as u128, 1u16);
        // D s_111_4: cmp-eq s_111_1 s_111_3
        let s_111_4: bool = ((s_111_1) == (s_111_3));
        // D s_111_5: write-var gs#106956 <= s_111_4
        fn_state.gs_106956 = s_111_4;
        // N s_111_6: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_112_0: const #432u : u32
        let s_112_0: u32 = 432;
        // D s_112_1: read-reg s_112_0:u8
        let s_112_1: u8 = {
            let value = state.read_register::<u8>(s_112_0 as isize);
            tracer.read_register(s_112_0 as isize, value);
            value
        };
        // D s_112_2: call ELUsingAArch32(s_112_1)
        let s_112_2: bool = ELUsingAArch32(state, tracer, s_112_1);
        // D s_112_3: write-var gs#106955 <= s_112_2
        fn_state.gs_106955 = s_112_2;
        // N s_112_4: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_113_0: const #3u : u8
        let s_113_0: u8 = 3;
        // C s_113_1: cast zx s_113_0 -> bv
        let s_113_1: Bits = Bits::new(s_113_0 as u128, 8u16);
        // C s_113_2: cast zx s_113_1 -> i
        let s_113_2: i128 = (s_113_1.value() as i128);
        // C s_113_3: cast reint s_113_2 -> i64
        let s_113_3: i64 = (s_113_2 as i64);
        // C s_113_4: cast zx s_113_3 -> i
        let s_113_4: i128 = (i128::try_from(s_113_3).unwrap());
        // C s_113_5: const #432u : u32
        let s_113_5: u32 = 432;
        // D s_113_6: read-reg s_113_5:u8
        let s_113_6: u8 = {
            let value = state.read_register::<u8>(s_113_5 as isize);
            tracer.read_register(s_113_5 as isize, value);
            value
        };
        // D s_113_7: call AArch64_AArch32SystemAccessTrap(s_113_6, s_113_4)
        let s_113_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_113_6,
            s_113_4,
        );
        // N s_113_8: return
        return;
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_114_0: read-var __CNTHCTL_EL2_EL0PTEN:u8
        let s_114_0: bool = fn_state.u__CNTHCTL_EL2_EL0PTEN;
        // D s_114_1: cast zx s_114_0 -> bv
        let s_114_1: Bits = Bits::new(s_114_0 as u128, 1u16);
        // C s_114_2: const #0u : u8
        let s_114_2: bool = false;
        // C s_114_3: cast zx s_114_2 -> bv
        let s_114_3: Bits = Bits::new(s_114_2 as u128, 1u16);
        // D s_114_4: cmp-eq s_114_1 s_114_3
        let s_114_4: bool = ((s_114_1) == (s_114_3));
        // D s_114_5: write-var gs#106954 <= s_114_4
        fn_state.gs_106954 = s_114_4;
        // N s_114_6: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #102552u : u32
        let s_115_0: u32 = 102552;
        // D s_115_1: read-reg s_115_0:struct
        let s_115_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_115_0 as isize);
            tracer.read_register(s_115_0 as isize, value);
            value
        };
        // D s_115_2: call _get_HCR_EL2_Type_E2H(s_115_1)
        let s_115_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_115_1);
        // C s_115_3: const #102552u : u32
        let s_115_3: u32 = 102552;
        // D s_115_4: read-reg s_115_3:struct
        let s_115_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_115_3 as isize);
            tracer.read_register(s_115_3 as isize, value);
            value
        };
        // D s_115_5: call _get_HCR_EL2_Type_TGE(s_115_4)
        let s_115_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_115_4);
        // D s_115_6: cast zx s_115_2 -> bv
        let s_115_6: Bits = Bits::new(s_115_2 as u128, 1u16);
        // D s_115_7: cast zx s_115_5 -> bv
        let s_115_7: Bits = Bits::new(s_115_5 as u128, 1u16);
        // D s_115_8: cast reint s_115_6 -> u128
        let s_115_8: u128 = (s_115_6.value() as u128);
        // D s_115_9: size-of s_115_6
        let s_115_9: u16 = s_115_6.length();
        // D s_115_10: cast reint s_115_7 -> u128
        let s_115_10: u128 = (s_115_7.value() as u128);
        // D s_115_11: size-of s_115_7
        let s_115_11: u16 = s_115_7.length();
        // D s_115_12: lsl s_115_8 s_115_11
        let s_115_12: u128 = s_115_8 << s_115_11;
        // D s_115_13: or s_115_12 s_115_10
        let s_115_13: u128 = ((s_115_12) | (s_115_10));
        // D s_115_14: add s_115_9 s_115_11
        let s_115_14: u16 = (s_115_9 + s_115_11);
        // D s_115_15: create-bits s_115_13 s_115_14
        let s_115_15: Bits = Bits::new(s_115_13, s_115_14);
        // D s_115_16: cast reint s_115_15 -> u8
        let s_115_16: u8 = (s_115_15.value() as u8);
        // D s_115_17: cast zx s_115_16 -> bv
        let s_115_17: Bits = Bits::new(s_115_16 as u128, 2u16);
        // C s_115_18: const #3u : u8
        let s_115_18: u8 = 3;
        // C s_115_19: cast zx s_115_18 -> bv
        let s_115_19: Bits = Bits::new(s_115_18 as u128, 2u16);
        // D s_115_20: cmp-eq s_115_17 s_115_19
        let s_115_20: bool = ((s_115_17) == (s_115_19));
        // D s_115_21: write-var gs#106953 <= s_115_20
        fn_state.gs_106953 = s_115_20;
        // N s_115_22: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #432u : u32
        let s_116_0: u32 = 432;
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
        // D s_116_4: write-var gs#106952 <= s_116_3
        fn_state.gs_106952 = s_116_3;
        // N s_116_5: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #3u : u8
        let s_117_0: u8 = 3;
        // C s_117_1: cast zx s_117_0 -> bv
        let s_117_1: Bits = Bits::new(s_117_0 as u128, 8u16);
        // C s_117_2: cast zx s_117_1 -> i
        let s_117_2: i128 = (s_117_1.value() as i128);
        // C s_117_3: cast reint s_117_2 -> i64
        let s_117_3: i64 = (s_117_2 as i64);
        // C s_117_4: cast zx s_117_3 -> i
        let s_117_4: i128 = (i128::try_from(s_117_3).unwrap());
        // C s_117_5: const #432u : u32
        let s_117_5: u32 = 432;
        // D s_117_6: read-reg s_117_5:u8
        let s_117_6: u8 = {
            let value = state.read_register::<u8>(s_117_5 as isize);
            tracer.read_register(s_117_5 as isize, value);
            value
        };
        // D s_117_7: call AArch64_AArch32SystemAccessTrap(s_117_6, s_117_4)
        let s_117_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_117_6,
            s_117_4,
        );
        // N s_117_8: return
        return;
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_118_0: read-var __CNTHCTL_EL2_EL1PTEN:u8
        let s_118_0: bool = fn_state.u__CNTHCTL_EL2_EL1PTEN;
        // D s_118_1: cast zx s_118_0 -> bv
        let s_118_1: Bits = Bits::new(s_118_0 as u128, 1u16);
        // C s_118_2: const #0u : u8
        let s_118_2: bool = false;
        // C s_118_3: cast zx s_118_2 -> bv
        let s_118_3: Bits = Bits::new(s_118_2 as u128, 1u16);
        // D s_118_4: cmp-eq s_118_1 s_118_3
        let s_118_4: bool = ((s_118_1) == (s_118_3));
        // D s_118_5: write-var gs#106951 <= s_118_4
        fn_state.gs_106951 = s_118_4;
        // N s_118_6: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #102552u : u32
        let s_119_0: u32 = 102552;
        // D s_119_1: read-reg s_119_0:struct
        let s_119_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_119_0 as isize);
            tracer.read_register(s_119_0 as isize, value);
            value
        };
        // D s_119_2: call _get_HCR_EL2_Type_E2H(s_119_1)
        let s_119_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_119_1);
        // C s_119_3: const #102552u : u32
        let s_119_3: u32 = 102552;
        // D s_119_4: read-reg s_119_3:struct
        let s_119_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_119_3 as isize);
            tracer.read_register(s_119_3 as isize, value);
            value
        };
        // D s_119_5: call _get_HCR_EL2_Type_TGE(s_119_4)
        let s_119_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_119_4);
        // D s_119_6: cast zx s_119_2 -> bv
        let s_119_6: Bits = Bits::new(s_119_2 as u128, 1u16);
        // D s_119_7: cast zx s_119_5 -> bv
        let s_119_7: Bits = Bits::new(s_119_5 as u128, 1u16);
        // D s_119_8: cast reint s_119_6 -> u128
        let s_119_8: u128 = (s_119_6.value() as u128);
        // D s_119_9: size-of s_119_6
        let s_119_9: u16 = s_119_6.length();
        // D s_119_10: cast reint s_119_7 -> u128
        let s_119_10: u128 = (s_119_7.value() as u128);
        // D s_119_11: size-of s_119_7
        let s_119_11: u16 = s_119_7.length();
        // D s_119_12: lsl s_119_8 s_119_11
        let s_119_12: u128 = s_119_8 << s_119_11;
        // D s_119_13: or s_119_12 s_119_10
        let s_119_13: u128 = ((s_119_12) | (s_119_10));
        // D s_119_14: add s_119_9 s_119_11
        let s_119_14: u16 = (s_119_9 + s_119_11);
        // D s_119_15: create-bits s_119_13 s_119_14
        let s_119_15: Bits = Bits::new(s_119_13, s_119_14);
        // D s_119_16: cast reint s_119_15 -> u8
        let s_119_16: u8 = (s_119_15.value() as u8);
        // D s_119_17: cast zx s_119_16 -> bv
        let s_119_17: Bits = Bits::new(s_119_16 as u128, 2u16);
        // C s_119_18: const #2u : u8
        let s_119_18: u8 = 2;
        // C s_119_19: cast zx s_119_18 -> bv
        let s_119_19: Bits = Bits::new(s_119_18 as u128, 2u16);
        // D s_119_20: cmp-eq s_119_17 s_119_19
        let s_119_20: bool = ((s_119_17) == (s_119_19));
        // D s_119_21: write-var gs#106950 <= s_119_20
        fn_state.gs_106950 = s_119_20;
        // N s_119_22: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_120_0: const #432u : u32
        let s_120_0: u32 = 432;
        // D s_120_1: read-reg s_120_0:u8
        let s_120_1: u8 = {
            let value = state.read_register::<u8>(s_120_0 as isize);
            tracer.read_register(s_120_0 as isize, value);
            value
        };
        // D s_120_2: call ELUsingAArch32(s_120_1)
        let s_120_2: bool = ELUsingAArch32(state, tracer, s_120_1);
        // D s_120_3: not s_120_2
        let s_120_3: bool = !s_120_2;
        // D s_120_4: write-var gs#106949 <= s_120_3
        fn_state.gs_106949 = s_120_3;
        // N s_120_5: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #3u : u8
        let s_121_0: u8 = 3;
        // C s_121_1: cast zx s_121_0 -> bv
        let s_121_1: Bits = Bits::new(s_121_0 as u128, 8u16);
        // C s_121_2: cast zx s_121_1 -> i
        let s_121_2: i128 = (s_121_1.value() as i128);
        // C s_121_3: cast reint s_121_2 -> i64
        let s_121_3: i64 = (s_121_2 as i64);
        // C s_121_4: cast zx s_121_3 -> i
        let s_121_4: i128 = (i128::try_from(s_121_3).unwrap());
        // C s_121_5: const #432u : u32
        let s_121_5: u32 = 432;
        // D s_121_6: read-reg s_121_5:u8
        let s_121_6: u8 = {
            let value = state.read_register::<u8>(s_121_5 as isize);
            tracer.read_register(s_121_5 as isize, value);
            value
        };
        // D s_121_7: call AArch64_AArch32SystemAccessTrap(s_121_6, s_121_4)
        let s_121_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_121_6,
            s_121_4,
        );
        // N s_121_8: return
        return;
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_122_0: read-var __CNTHCTL_EL2_EL1PCEN:u8
        let s_122_0: bool = fn_state.u__CNTHCTL_EL2_EL1PCEN;
        // D s_122_1: cast zx s_122_0 -> bv
        let s_122_1: Bits = Bits::new(s_122_0 as u128, 1u16);
        // C s_122_2: const #0u : u8
        let s_122_2: bool = false;
        // C s_122_3: cast zx s_122_2 -> bv
        let s_122_3: Bits = Bits::new(s_122_2 as u128, 1u16);
        // D s_122_4: cmp-eq s_122_1 s_122_3
        let s_122_4: bool = ((s_122_1) == (s_122_3));
        // D s_122_5: write-var gs#106948 <= s_122_4
        fn_state.gs_106948 = s_122_4;
        // N s_122_6: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_123_0: read-var __HCR_EL2_E2H:u8
        let s_123_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_123_1: cast zx s_123_0 -> bv
        let s_123_1: Bits = Bits::new(s_123_0 as u128, 1u16);
        // C s_123_2: const #0u : u8
        let s_123_2: bool = false;
        // C s_123_3: cast zx s_123_2 -> bv
        let s_123_3: Bits = Bits::new(s_123_2 as u128, 1u16);
        // D s_123_4: cmp-eq s_123_1 s_123_3
        let s_123_4: bool = ((s_123_1) == (s_123_3));
        // D s_123_5: write-var gs#106947 <= s_123_4
        fn_state.gs_106947 = s_123_4;
        // N s_123_6: jump b62
        return block_62(state, tracer, fn_state);
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
        // D s_124_4: write-var gs#106946 <= s_124_3
        fn_state.gs_106946 = s_124_3;
        // N s_124_5: jump b60
        return block_60(state, tracer, fn_state);
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
        // N s_125_2: branch s_125_1 b141 b126
        if s_125_1 {
            return block_141(state, tracer, fn_state);
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
        // D s_126_1: write-var gs#106968 <= s_126_0
        fn_state.gs_106968 = s_126_0;
        // N s_126_2: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_127_0: read-var gs#106968:u8
        let s_127_0: bool = fn_state.gs_106968;
        // N s_127_1: branch s_127_0 b140 b128
        if s_127_0 {
            return block_140(state, tracer, fn_state);
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
        // D s_128_1: write-var gs#106969 <= s_128_0
        fn_state.gs_106969 = s_128_0;
        // N s_128_2: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_129_0: read-var gs#106969:u8
        let s_129_0: bool = fn_state.gs_106969;
        // N s_129_1: branch s_129_0 b139 b130
        if s_129_0 {
            return block_139(state, tracer, fn_state);
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
        // N s_130_2: branch s_130_1 b138 b131
        if s_130_1 {
            return block_138(state, tracer, fn_state);
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
        // D s_131_1: write-var gs#106970 <= s_131_0
        fn_state.gs_106970 = s_131_0;
        // N s_131_2: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_132_0: read-var gs#106970:u8
        let s_132_0: bool = fn_state.gs_106970;
        // N s_132_1: branch s_132_0 b137 b133
        if s_132_0 {
            return block_137(state, tracer, fn_state);
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
        // D s_133_1: write-var gs#106971 <= s_133_0
        fn_state.gs_106971 = s_133_0;
        // N s_133_2: jump b134
        return block_134(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_134_0: read-var gs#106971:u8
        let s_134_0: bool = fn_state.gs_106971;
        // N s_134_1: branch s_134_0 b136 b135
        if s_134_0 {
            return block_136(state, tracer, fn_state);
        } else {
            return block_135(state, tracer, fn_state);
        };
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_135_0: panic
        panic!("{:?}", ());
        // N s_135_1: return
        return;
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_136_0: const #0u : u8
        let s_136_0: u8 = 0;
        // C s_136_1: cast zx s_136_0 -> bv
        let s_136_1: Bits = Bits::new(s_136_0 as u128, 8u16);
        // C s_136_2: cast zx s_136_1 -> i
        let s_136_2: i128 = (s_136_1.value() as i128);
        // C s_136_3: cast reint s_136_2 -> i64
        let s_136_3: i64 = (s_136_2 as i64);
        // C s_136_4: cast zx s_136_3 -> i
        let s_136_4: i128 = (i128::try_from(s_136_3).unwrap());
        // S s_136_5: call AArch32_TakeHypTrapException(s_136_4)
        let s_136_5: () = AArch32_TakeHypTrapException(state, tracer, s_136_4);
        // N s_136_6: return
        return;
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_137_0: read-var __HCR_TGE:u8
        let s_137_0: bool = fn_state.u__HCR_TGE;
        // D s_137_1: cast zx s_137_0 -> bv
        let s_137_1: Bits = Bits::new(s_137_0 as u128, 1u16);
        // C s_137_2: const #1u : u8
        let s_137_2: bool = true;
        // C s_137_3: cast zx s_137_2 -> bv
        let s_137_3: Bits = Bits::new(s_137_2 as u128, 1u16);
        // D s_137_4: cmp-eq s_137_1 s_137_3
        let s_137_4: bool = ((s_137_1) == (s_137_3));
        // D s_137_5: write-var gs#106971 <= s_137_4
        fn_state.gs_106971 = s_137_4;
        // N s_137_6: jump b134
        return block_134(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_138_0: const #432u : u32
        let s_138_0: u32 = 432;
        // D s_138_1: read-reg s_138_0:u8
        let s_138_1: u8 = {
            let value = state.read_register::<u8>(s_138_0 as isize);
            tracer.read_register(s_138_0 as isize, value);
            value
        };
        // D s_138_2: call ELUsingAArch32(s_138_1)
        let s_138_2: bool = ELUsingAArch32(state, tracer, s_138_1);
        // D s_138_3: write-var gs#106970 <= s_138_2
        fn_state.gs_106970 = s_138_2;
        // N s_138_4: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_139_0: const #3u : u8
        let s_139_0: u8 = 3;
        // C s_139_1: cast zx s_139_0 -> bv
        let s_139_1: Bits = Bits::new(s_139_0 as u128, 8u16);
        // C s_139_2: cast zx s_139_1 -> i
        let s_139_2: i128 = (s_139_1.value() as i128);
        // C s_139_3: cast reint s_139_2 -> i64
        let s_139_3: i64 = (s_139_2 as i64);
        // C s_139_4: cast zx s_139_3 -> i
        let s_139_4: i128 = (i128::try_from(s_139_3).unwrap());
        // C s_139_5: const #432u : u32
        let s_139_5: u32 = 432;
        // D s_139_6: read-reg s_139_5:u8
        let s_139_6: u8 = {
            let value = state.read_register::<u8>(s_139_5 as isize);
            tracer.read_register(s_139_5 as isize, value);
            value
        };
        // D s_139_7: call AArch64_AArch32SystemAccessTrap(s_139_6, s_139_4)
        let s_139_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_139_6,
            s_139_4,
        );
        // N s_139_8: return
        return;
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_140_0: read-var __HCR_EL2_TGE:u8
        let s_140_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_140_1: cast zx s_140_0 -> bv
        let s_140_1: Bits = Bits::new(s_140_0 as u128, 1u16);
        // C s_140_2: const #1u : u8
        let s_140_2: bool = true;
        // C s_140_3: cast zx s_140_2 -> bv
        let s_140_3: Bits = Bits::new(s_140_2 as u128, 1u16);
        // D s_140_4: cmp-eq s_140_1 s_140_3
        let s_140_4: bool = ((s_140_1) == (s_140_3));
        // D s_140_5: write-var gs#106969 <= s_140_4
        fn_state.gs_106969 = s_140_4;
        // N s_140_6: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_141_0: const #432u : u32
        let s_141_0: u32 = 432;
        // D s_141_1: read-reg s_141_0:u8
        let s_141_1: u8 = {
            let value = state.read_register::<u8>(s_141_0 as isize);
            tracer.read_register(s_141_0 as isize, value);
            value
        };
        // D s_141_2: call ELUsingAArch32(s_141_1)
        let s_141_2: bool = ELUsingAArch32(state, tracer, s_141_1);
        // D s_141_3: not s_141_2
        let s_141_3: bool = !s_141_2;
        // D s_141_4: write-var gs#106968 <= s_141_3
        fn_state.gs_106968 = s_141_3;
        // N s_141_5: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_142_0: read-var __CNTKCTL_PL0PTEN:u8
        let s_142_0: bool = fn_state.u__CNTKCTL_PL0PTEN;
        // D s_142_1: cast zx s_142_0 -> bv
        let s_142_1: Bits = Bits::new(s_142_0 as u128, 1u16);
        // C s_142_2: const #0u : u8
        let s_142_2: bool = false;
        // C s_142_3: cast zx s_142_2 -> bv
        let s_142_3: Bits = Bits::new(s_142_2 as u128, 1u16);
        // D s_142_4: cmp-eq s_142_1 s_142_3
        let s_142_4: bool = ((s_142_1) == (s_142_3));
        // D s_142_5: write-var gs#106945 <= s_142_4
        fn_state.gs_106945 = s_142_4;
        // N s_142_6: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_143_0: const #() : ()
        let s_143_0: () = ();
        // S s_143_1: call EL2Enabled(s_143_0)
        let s_143_1: bool = EL2Enabled(state, tracer, s_143_0);
        // N s_143_2: branch s_143_1 b151 b144
        if s_143_1 {
            return block_151(state, tracer, fn_state);
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
        // D s_144_1: write-var gs#106972 <= s_144_0
        fn_state.gs_106972 = s_144_0;
        // N s_144_2: jump b145
        return block_145(state, tracer, fn_state);
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_145_0: read-var gs#106972:u8
        let s_145_0: bool = fn_state.gs_106972;
        // N s_145_1: branch s_145_0 b150 b146
        if s_145_0 {
            return block_150(state, tracer, fn_state);
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
        // D s_146_1: write-var gs#106973 <= s_146_0
        fn_state.gs_106973 = s_146_0;
        // N s_146_2: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_147_0: read-var gs#106973:u8
        let s_147_0: bool = fn_state.gs_106973;
        // N s_147_1: branch s_147_0 b149 b148
        if s_147_0 {
            return block_149(state, tracer, fn_state);
        } else {
            return block_148(state, tracer, fn_state);
        };
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_148_0: const #3u : u8
        let s_148_0: u8 = 3;
        // C s_148_1: cast zx s_148_0 -> bv
        let s_148_1: Bits = Bits::new(s_148_0 as u128, 8u16);
        // C s_148_2: cast zx s_148_1 -> i
        let s_148_2: i128 = (s_148_1.value() as i128);
        // C s_148_3: cast reint s_148_2 -> i64
        let s_148_3: i64 = (s_148_2 as i64);
        // C s_148_4: cast zx s_148_3 -> i
        let s_148_4: i128 = (i128::try_from(s_148_3).unwrap());
        // C s_148_5: const #440u : u32
        let s_148_5: u32 = 440;
        // D s_148_6: read-reg s_148_5:u8
        let s_148_6: u8 = {
            let value = state.read_register::<u8>(s_148_5 as isize);
            tracer.read_register(s_148_5 as isize, value);
            value
        };
        // D s_148_7: call AArch64_AArch32SystemAccessTrap(s_148_6, s_148_4)
        let s_148_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_148_6,
            s_148_4,
        );
        // N s_148_8: return
        return;
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_149_0: const #3u : u8
        let s_149_0: u8 = 3;
        // C s_149_1: cast zx s_149_0 -> bv
        let s_149_1: Bits = Bits::new(s_149_0 as u128, 8u16);
        // C s_149_2: cast zx s_149_1 -> i
        let s_149_2: i128 = (s_149_1.value() as i128);
        // C s_149_3: cast reint s_149_2 -> i64
        let s_149_3: i64 = (s_149_2 as i64);
        // C s_149_4: cast zx s_149_3 -> i
        let s_149_4: i128 = (i128::try_from(s_149_3).unwrap());
        // C s_149_5: const #432u : u32
        let s_149_5: u32 = 432;
        // D s_149_6: read-reg s_149_5:u8
        let s_149_6: u8 = {
            let value = state.read_register::<u8>(s_149_5 as isize);
            tracer.read_register(s_149_5 as isize, value);
            value
        };
        // D s_149_7: call AArch64_AArch32SystemAccessTrap(s_149_6, s_149_4)
        let s_149_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_149_6,
            s_149_4,
        );
        // N s_149_8: return
        return;
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_150_0: read-var __HCR_EL2_TGE:u8
        let s_150_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_150_1: cast zx s_150_0 -> bv
        let s_150_1: Bits = Bits::new(s_150_0 as u128, 1u16);
        // C s_150_2: const #1u : u8
        let s_150_2: bool = true;
        // C s_150_3: cast zx s_150_2 -> bv
        let s_150_3: Bits = Bits::new(s_150_2 as u128, 1u16);
        // D s_150_4: cmp-eq s_150_1 s_150_3
        let s_150_4: bool = ((s_150_1) == (s_150_3));
        // D s_150_5: write-var gs#106973 <= s_150_4
        fn_state.gs_106973 = s_150_4;
        // N s_150_6: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_151_0: const #432u : u32
        let s_151_0: u32 = 432;
        // D s_151_1: read-reg s_151_0:u8
        let s_151_1: u8 = {
            let value = state.read_register::<u8>(s_151_0 as isize);
            tracer.read_register(s_151_0 as isize, value);
            value
        };
        // D s_151_2: call ELUsingAArch32(s_151_1)
        let s_151_2: bool = ELUsingAArch32(state, tracer, s_151_1);
        // D s_151_3: not s_151_2
        let s_151_3: bool = !s_151_2;
        // D s_151_4: write-var gs#106972 <= s_151_3
        fn_state.gs_106972 = s_151_3;
        // N s_151_5: jump b145
        return block_145(state, tracer, fn_state);
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_152_0: read-var __CNTKCTL_EL1_EL0PTEN:u8
        let s_152_0: bool = fn_state.u__CNTKCTL_EL1_EL0PTEN;
        // D s_152_1: cast zx s_152_0 -> bv
        let s_152_1: Bits = Bits::new(s_152_0 as u128, 1u16);
        // C s_152_2: const #0u : u8
        let s_152_2: bool = false;
        // C s_152_3: cast zx s_152_2 -> bv
        let s_152_3: Bits = Bits::new(s_152_2 as u128, 1u16);
        // D s_152_4: cmp-eq s_152_1 s_152_3
        let s_152_4: bool = ((s_152_1) == (s_152_3));
        // D s_152_5: write-var gs#106944 <= s_152_4
        fn_state.gs_106944 = s_152_4;
        // N s_152_6: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_153_0: const #() : ()
        let s_153_0: () = ();
        // S s_153_1: call EL2Enabled(s_153_0)
        let s_153_1: bool = EL2Enabled(state, tracer, s_153_0);
        // N s_153_2: branch s_153_1 b156 b154
        if s_153_1 {
            return block_156(state, tracer, fn_state);
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
        // D s_154_1: write-var gs#106942 <= s_154_0
        fn_state.gs_106942 = s_154_0;
        // N s_154_2: jump b155
        return block_155(state, tracer, fn_state);
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_155_0: read-var gs#106942:u8
        let s_155_0: bool = fn_state.gs_106942;
        // D s_155_1: not s_155_0
        let s_155_1: bool = !s_155_0;
        // D s_155_2: write-var gs#106943 <= s_155_1
        fn_state.gs_106943 = s_155_1;
        // N s_155_3: jump b52
        return block_52(state, tracer, fn_state);
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
        // D s_156_20: cmp-eq s_156_17 s_156_19
        let s_156_20: bool = ((s_156_17) == (s_156_19));
        // D s_156_21: write-var gs#106942 <= s_156_20
        fn_state.gs_106942 = s_156_20;
        // N s_156_22: jump b155
        return block_155(state, tracer, fn_state);
    }
}
