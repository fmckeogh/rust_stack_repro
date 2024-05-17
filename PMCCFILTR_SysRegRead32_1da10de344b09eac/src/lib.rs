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
use u_get_MDCR_EL2_Type_TPM::*;
use u_get_PMUSERENR_EL0_Type_UEN::*;
use u_get_SCR_EL3_Type_FGTEn::*;
use PMCCFILTR_read::*;
use u_get_HCR_EL2_Type_E2H::*;
use HCR_read::*;
use PMUSERENR_read::*;
use HDCR_read::*;
use Halted::*;
use IsFeatureImplemented::*;
use u__get_PMCCFILTR::*;
use u__IMPDEF_boolean::*;
use AArch64_AArch32SystemAccessTrap::*;
use u_get_HDCR_Type_TPM::*;
use u_get_HCR_Type_TGE::*;
use u_get_MDCR_EL3_Type_TPM::*;
use u_get_PMUSERENR_Type_EN::*;
use R_set::*;
use u_get_HDFGRTR_EL2_Type_PMCCFILTR_EL0::*;
use u_get_PMUSERENR_EL0_Type_EN::*;
use ELUsingAArch32::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use EDSCR_read::*;
use AArch32_TakeHypTrapException::*;
use common::*;
pub fn PMCCFILTR_SysRegRead32_1da10de344b09eac<T: Tracer>(
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
        gs_112702: bool,
        gs_112693: bool,
        ga_184458: ProductType700c18a878c5601b,
        gs_112691: bool,
        u__HDFGRTR_EL2_PMCCFILTR_EL0: bool,
        gs_112666: bool,
        gs_112656: bool,
        gs_112658: bool,
        u__MDCR_EL3_TPM: bool,
        gs_112696: bool,
        gs_112698: bool,
        gs_112700: bool,
        gs_112688: bool,
        gs_112681: bool,
        gs_112683: bool,
        u__HCR_TGE: bool,
        gs_112660: bool,
        gs_112680: bool,
        gs_112684: bool,
        gs_112669: bool,
        ga_184454: ProductType700c18a878c5601b,
        u__PSTATE_EL: u8,
        gs_112692: bool,
        gs_112663: bool,
        u__MDCR_EL2_TPM: bool,
        u__PMUSERENR_EN: bool,
        gs_112670: bool,
        u__HCR_EL2_TGE: bool,
        gs_112671: bool,
        gs_112668: bool,
        gs_112687: bool,
        u__SCR_EL3_FGTEn: bool,
        gs_112699: bool,
        u__PMUSERENR_EL0_EN: bool,
        gs_112673: bool,
        gs_112695: bool,
        gs_112657: bool,
        gs_112676: bool,
        gs_112703: bool,
        u__HDCR_TPM: bool,
        gs_112659: bool,
        gs_112686: bool,
        gs_112697: bool,
        gs_112667: bool,
        ga_184430: ProductType700c18a878c5601b,
        gs_112701: bool,
        gs_112677: bool,
        gs_112672: bool,
        gs_112689: bool,
        gs_112662: bool,
        gs_112661: bool,
        gs_112655: bool,
        ga_184397: ProductType700c18a878c5601b,
        gs_112675: bool,
        gs_112694: bool,
        gs_112682: bool,
        gs_112685: bool,
        gs_112664: bool,
        gs_112679: bool,
        gs_112674: bool,
        gs_112690: bool,
        gs_112665: bool,
        gs_112678: bool,
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
        // C s_0_23: const #90704u : u32
        let s_0_23: u32 = 90704;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_SCR_EL3_Type_FGTEn(s_0_24)
        let s_0_25: bool = u_get_SCR_EL3_Type_FGTEn(state, tracer, s_0_24);
        // D s_0_26: write-var __SCR_EL3_FGTEn <= s_0_25
        fn_state.u__SCR_EL3_FGTEn = s_0_25;
        // C s_0_27: const #19144u : u32
        let s_0_27: u32 = 19144;
        // D s_0_28: read-reg s_0_27:struct
        let s_0_28: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_27 as isize);
            tracer.read_register(s_0_27 as isize, value);
            value
        };
        // D s_0_29: call _get_HDFGRTR_EL2_Type_PMCCFILTR_EL0(s_0_28)
        let s_0_29: bool = u_get_HDFGRTR_EL2_Type_PMCCFILTR_EL0(state, tracer, s_0_28);
        // D s_0_30: write-var __HDFGRTR_EL2_PMCCFILTR_EL0 <= s_0_29
        fn_state.u__HDFGRTR_EL2_PMCCFILTR_EL0 = s_0_29;
        // C s_0_31: const #104880u : u32
        let s_0_31: u32 = 104880;
        // D s_0_32: read-reg s_0_31:struct
        let s_0_32: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_31 as isize);
            tracer.read_register(s_0_31 as isize, value);
            value
        };
        // D s_0_33: call _get_MDCR_EL2_Type_TPM(s_0_32)
        let s_0_33: bool = u_get_MDCR_EL2_Type_TPM(state, tracer, s_0_32);
        // D s_0_34: write-var __MDCR_EL2_TPM <= s_0_33
        fn_state.u__MDCR_EL2_TPM = s_0_33;
        // C s_0_35: const #() : ()
        let s_0_35: () = ();
        // S s_0_36: call HDCR_read(s_0_35)
        let s_0_36: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_0_35);
        // S s_0_37: call _get_HDCR_Type_TPM(s_0_36)
        let s_0_37: bool = u_get_HDCR_Type_TPM(state, tracer, s_0_36);
        // D s_0_38: write-var __HDCR_TPM <= s_0_37
        fn_state.u__HDCR_TPM = s_0_37;
        // D s_0_39: read-var __PSTATE_EL:u8
        let s_0_39: u8 = fn_state.u__PSTATE_EL;
        // D s_0_40: cast zx s_0_39 -> bv
        let s_0_40: Bits = Bits::new(s_0_39 as u128, 2u16);
        // C s_0_41: const #448u : u32
        let s_0_41: u32 = 448;
        // D s_0_42: read-reg s_0_41:u8
        let s_0_42: u8 = {
            let value = state.read_register::<u8>(s_0_41 as isize);
            tracer.read_register(s_0_41 as isize, value);
            value
        };
        // D s_0_43: cast zx s_0_42 -> bv
        let s_0_43: Bits = Bits::new(s_0_42 as u128, 2u16);
        // D s_0_44: cmp-eq s_0_40 s_0_43
        let s_0_44: bool = ((s_0_40) == (s_0_43));
        // N s_0_45: branch s_0_44 b84 b1
        if s_0_44 {
            return block_84(state, tracer, fn_state);
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
        // S s_5_1: call PMCCFILTR_read(s_5_0)
        let s_5_1: ProductType700c18a878c5601b = PMCCFILTR_read(state, tracer, s_5_0);
        // S s_5_2: call __get_PMCCFILTR(s_5_1)
        let s_5_2: ProductType700c18a878c5601b = u__get_PMCCFILTR(state, tracer, s_5_1);
        // D s_5_3: write-var ga#184458 <= s_5_2
        fn_state.ga_184458 = s_5_2;
        // D s_5_4: read-var ga#184458.0:struct
        let s_5_4: u32 = fn_state.ga_184458._0;
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
        // D s_7_1: write-var gs#112655 <= s_7_0
        fn_state.gs_112655 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#112655:u8
        let s_8_0: bool = fn_state.gs_112655;
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
        // D s_9_1: write-var gs#112656 <= s_9_0
        fn_state.gs_112656 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#112656:u8
        let s_10_0: bool = fn_state.gs_112656;
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
        // D s_11_1: write-var gs#112657 <= s_11_0
        fn_state.gs_112657 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#112657:u8
        let s_12_0: bool = fn_state.gs_112657;
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
        // D s_13_1: write-var gs#112658 <= s_13_0
        fn_state.gs_112658 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#112658:u8
        let s_14_0: bool = fn_state.gs_112658;
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
        // D s_15_1: write-var gs#112659 <= s_15_0
        fn_state.gs_112659 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#112659:u8
        let s_16_0: bool = fn_state.gs_112659;
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
        // D s_18_1: write-var gs#112660 <= s_18_0
        fn_state.gs_112660 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#112660:u8
        let s_19_0: bool = fn_state.gs_112660;
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
        // D s_20_1: write-var gs#112661 <= s_20_0
        fn_state.gs_112661 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#112661:u8
        let s_21_0: bool = fn_state.gs_112661;
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
        // S s_22_1: call PMCCFILTR_read(s_22_0)
        let s_22_1: ProductType700c18a878c5601b = PMCCFILTR_read(state, tracer, s_22_0);
        // S s_22_2: call __get_PMCCFILTR(s_22_1)
        let s_22_2: ProductType700c18a878c5601b = u__get_PMCCFILTR(
            state,
            tracer,
            s_22_1,
        );
        // D s_22_3: write-var ga#184454 <= s_22_2
        fn_state.ga_184454 = s_22_2;
        // D s_22_4: read-var ga#184454.0:struct
        let s_22_4: u32 = fn_state.ga_184454._0;
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
        // D s_24_1: write-var gs#112662 <= s_24_0
        fn_state.gs_112662 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#112662:u8
        let s_25_0: bool = fn_state.gs_112662;
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
        // D s_28_7: write-var gs#112662 <= s_28_6
        fn_state.gs_112662 = s_28_6;
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
        // D s_29_5: write-var gs#112661 <= s_29_4
        fn_state.gs_112661 = s_29_4;
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
        // D s_30_4: write-var gs#112660 <= s_30_3
        fn_state.gs_112660 = s_30_3;
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
        // D s_32_5: write-var gs#112659 <= s_32_4
        fn_state.gs_112659 = s_32_4;
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
        // D s_33_4: write-var gs#112658 <= s_33_3
        fn_state.gs_112658 = s_33_3;
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
        // D s_34_2: write-var gs#112657 <= s_34_1
        fn_state.gs_112657 = s_34_1;
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
        // D s_35_7: write-var gs#112656 <= s_35_6
        fn_state.gs_112656 = s_35_6;
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
        // D s_36_4: write-var gs#112655 <= s_36_3
        fn_state.gs_112655 = s_36_3;
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
        // N s_37_2: branch s_37_1 b83 b38
        if s_37_1 {
            return block_83(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#112663 <= s_38_0
        fn_state.gs_112663 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#112663:u8
        let s_39_0: bool = fn_state.gs_112663;
        // N s_39_1: branch s_39_0 b82 b40
        if s_39_0 {
            return block_82(state, tracer, fn_state);
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
        // D s_40_1: write-var gs#112664 <= s_40_0
        fn_state.gs_112664 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#112664:u8
        let s_41_0: bool = fn_state.gs_112664;
        // N s_41_1: branch s_41_0 b81 b42
        if s_41_0 {
            return block_81(state, tracer, fn_state);
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
        // D s_42_1: write-var gs#112665 <= s_42_0
        fn_state.gs_112665 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#112665:u8
        let s_43_0: bool = fn_state.gs_112665;
        // N s_43_1: branch s_43_0 b80 b44
        if s_43_0 {
            return block_80(state, tracer, fn_state);
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
        // D s_44_1: write-var gs#112666 <= s_44_0
        fn_state.gs_112666 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#112666:u8
        let s_45_0: bool = fn_state.gs_112666;
        // N s_45_1: branch s_45_0 b79 b46
        if s_45_0 {
            return block_79(state, tracer, fn_state);
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
        // D s_46_1: write-var gs#112667 <= s_46_0
        fn_state.gs_112667 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#112667:u8
        let s_47_0: bool = fn_state.gs_112667;
        // N s_47_1: branch s_47_0 b78 b48
        if s_47_0 {
            return block_78(state, tracer, fn_state);
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
        // N s_48_2: branch s_48_1 b77 b49
        if s_48_1 {
            return block_77(state, tracer, fn_state);
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
        // D s_49_1: write-var gs#112668 <= s_49_0
        fn_state.gs_112668 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#112668:u8
        let s_50_0: bool = fn_state.gs_112668;
        // N s_50_1: branch s_50_0 b76 b51
        if s_50_0 {
            return block_76(state, tracer, fn_state);
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
        // D s_51_1: write-var gs#112669 <= s_51_0
        fn_state.gs_112669 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#112669:u8
        let s_52_0: bool = fn_state.gs_112669;
        // N s_52_1: branch s_52_0 b75 b53
        if s_52_0 {
            return block_75(state, tracer, fn_state);
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
        // N s_53_2: branch s_53_1 b74 b54
        if s_53_1 {
            return block_74(state, tracer, fn_state);
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
        // D s_54_1: write-var gs#112670 <= s_54_0
        fn_state.gs_112670 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#112670:u8
        let s_55_0: bool = fn_state.gs_112670;
        // N s_55_1: branch s_55_0 b73 b56
        if s_55_0 {
            return block_73(state, tracer, fn_state);
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
        // D s_56_1: write-var gs#112671 <= s_56_0
        fn_state.gs_112671 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#112671:u8
        let s_57_0: bool = fn_state.gs_112671;
        // N s_57_1: branch s_57_0 b72 b58
        if s_57_0 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #424u : u32
        let s_58_0: u32 = 424;
        // D s_58_1: read-reg s_58_0:u8
        let s_58_1: u8 = {
            let value = state.read_register::<u8>(s_58_0 as isize);
            tracer.read_register(s_58_0 as isize, value);
            value
        };
        // C s_58_2: const #2u : u8
        let s_58_2: u8 = 2;
        // D s_58_3: cmp-lt s_58_1 s_58_2
        let s_58_3: bool = ((s_58_1) < (s_58_2));
        // N s_58_4: branch s_58_3 b71 b59
        if s_58_3 {
            return block_71(state, tracer, fn_state);
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
        // D s_59_1: write-var gs#112672 <= s_59_0
        fn_state.gs_112672 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#112672:u8
        let s_60_0: bool = fn_state.gs_112672;
        // N s_60_1: branch s_60_0 b70 b61
        if s_60_0 {
            return block_70(state, tracer, fn_state);
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
        // D s_61_1: write-var gs#112673 <= s_61_0
        fn_state.gs_112673 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#112673:u8
        let s_62_0: bool = fn_state.gs_112673;
        // N s_62_1: branch s_62_0 b64 b63
        if s_62_0 {
            return block_64(state, tracer, fn_state);
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
        // S s_63_1: call PMCCFILTR_read(s_63_0)
        let s_63_1: ProductType700c18a878c5601b = PMCCFILTR_read(state, tracer, s_63_0);
        // S s_63_2: call __get_PMCCFILTR(s_63_1)
        let s_63_2: ProductType700c18a878c5601b = u__get_PMCCFILTR(
            state,
            tracer,
            s_63_1,
        );
        // D s_63_3: write-var ga#184430 <= s_63_2
        fn_state.ga_184430 = s_63_2;
        // D s_63_4: read-var ga#184430.0:struct
        let s_63_4: u32 = fn_state.ga_184430._0;
        // D s_63_5: read-var t:i
        let s_63_5: i128 = fn_state.t;
        // D s_63_6: call R_set(s_63_5, s_63_4)
        let s_63_6: () = R_set(state, tracer, s_63_5, s_63_4);
        // N s_63_7: return
        return;
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #() : ()
        let s_64_0: () = ();
        // S s_64_1: call Halted(s_64_0)
        let s_64_1: bool = Halted(state, tracer, s_64_0);
        // N s_64_2: branch s_64_1 b69 b65
        if s_64_1 {
            return block_69(state, tracer, fn_state);
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
        // D s_65_1: write-var gs#112674 <= s_65_0
        fn_state.gs_112674 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#112674:u8
        let s_66_0: bool = fn_state.gs_112674;
        // N s_66_1: branch s_66_0 b68 b67
        if s_66_0 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
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
        // C s_67_5: const #424u : u32
        let s_67_5: u32 = 424;
        // D s_67_6: read-reg s_67_5:u8
        let s_67_6: u8 = {
            let value = state.read_register::<u8>(s_67_5 as isize);
            tracer.read_register(s_67_5 as isize, value);
            value
        };
        // D s_67_7: call AArch64_AArch32SystemAccessTrap(s_67_6, s_67_4)
        let s_67_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_67_6, s_67_4);
        // N s_67_8: return
        return;
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_68_0: panic
        panic!("{:?}", ());
        // N s_68_1: return
        return;
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #() : ()
        let s_69_0: () = ();
        // S s_69_1: call EDSCR_read(s_69_0)
        let s_69_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_69_0);
        // S s_69_2: call _get_EDSCR_Type_SDD(s_69_1)
        let s_69_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_69_1);
        // S s_69_3: cast zx s_69_2 -> bv
        let s_69_3: Bits = Bits::new(s_69_2 as u128, 1u16);
        // C s_69_4: const #1u : u8
        let s_69_4: bool = true;
        // C s_69_5: cast zx s_69_4 -> bv
        let s_69_5: Bits = Bits::new(s_69_4 as u128, 1u16);
        // S s_69_6: cmp-eq s_69_3 s_69_5
        let s_69_6: bool = ((s_69_3) == (s_69_5));
        // D s_69_7: write-var gs#112674 <= s_69_6
        fn_state.gs_112674 = s_69_6;
        // N s_69_8: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var __MDCR_EL3_TPM:u8
        let s_70_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_70_1: cast zx s_70_0 -> bv
        let s_70_1: Bits = Bits::new(s_70_0 as u128, 1u16);
        // C s_70_2: const #1u : u8
        let s_70_2: bool = true;
        // C s_70_3: cast zx s_70_2 -> bv
        let s_70_3: Bits = Bits::new(s_70_2 as u128, 1u16);
        // D s_70_4: cmp-eq s_70_1 s_70_3
        let s_70_4: bool = ((s_70_1) == (s_70_3));
        // D s_70_5: write-var gs#112673 <= s_70_4
        fn_state.gs_112673 = s_70_4;
        // N s_70_6: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #424u : u32
        let s_71_0: u32 = 424;
        // D s_71_1: read-reg s_71_0:u8
        let s_71_1: u8 = {
            let value = state.read_register::<u8>(s_71_0 as isize);
            tracer.read_register(s_71_0 as isize, value);
            value
        };
        // D s_71_2: call ELUsingAArch32(s_71_1)
        let s_71_2: bool = ELUsingAArch32(state, tracer, s_71_1);
        // D s_71_3: not s_71_2
        let s_71_3: bool = !s_71_2;
        // D s_71_4: write-var gs#112672 <= s_71_3
        fn_state.gs_112672 = s_71_3;
        // N s_71_5: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #3u : u8
        let s_72_0: u8 = 3;
        // C s_72_1: cast zx s_72_0 -> bv
        let s_72_1: Bits = Bits::new(s_72_0 as u128, 8u16);
        // C s_72_2: cast zx s_72_1 -> i
        let s_72_2: i128 = (s_72_1.value() as i128);
        // C s_72_3: cast reint s_72_2 -> i64
        let s_72_3: i64 = (s_72_2 as i64);
        // C s_72_4: cast zx s_72_3 -> i
        let s_72_4: i128 = (i128::try_from(s_72_3).unwrap());
        // S s_72_5: call AArch32_TakeHypTrapException(s_72_4)
        let s_72_5: () = AArch32_TakeHypTrapException(state, tracer, s_72_4);
        // N s_72_6: return
        return;
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var __HDCR_TPM:u8
        let s_73_0: bool = fn_state.u__HDCR_TPM;
        // D s_73_1: cast zx s_73_0 -> bv
        let s_73_1: Bits = Bits::new(s_73_0 as u128, 1u16);
        // C s_73_2: const #1u : u8
        let s_73_2: bool = true;
        // C s_73_3: cast zx s_73_2 -> bv
        let s_73_3: Bits = Bits::new(s_73_2 as u128, 1u16);
        // D s_73_4: cmp-eq s_73_1 s_73_3
        let s_73_4: bool = ((s_73_1) == (s_73_3));
        // D s_73_5: write-var gs#112671 <= s_73_4
        fn_state.gs_112671 = s_73_4;
        // N s_73_6: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #432u : u32
        let s_74_0: u32 = 432;
        // D s_74_1: read-reg s_74_0:u8
        let s_74_1: u8 = {
            let value = state.read_register::<u8>(s_74_0 as isize);
            tracer.read_register(s_74_0 as isize, value);
            value
        };
        // D s_74_2: call ELUsingAArch32(s_74_1)
        let s_74_2: bool = ELUsingAArch32(state, tracer, s_74_1);
        // D s_74_3: write-var gs#112670 <= s_74_2
        fn_state.gs_112670 = s_74_2;
        // N s_74_4: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #3u : u8
        let s_75_0: u8 = 3;
        // C s_75_1: cast zx s_75_0 -> bv
        let s_75_1: Bits = Bits::new(s_75_0 as u128, 8u16);
        // C s_75_2: cast zx s_75_1 -> i
        let s_75_2: i128 = (s_75_1.value() as i128);
        // C s_75_3: cast reint s_75_2 -> i64
        let s_75_3: i64 = (s_75_2 as i64);
        // C s_75_4: cast zx s_75_3 -> i
        let s_75_4: i128 = (i128::try_from(s_75_3).unwrap());
        // C s_75_5: const #432u : u32
        let s_75_5: u32 = 432;
        // D s_75_6: read-reg s_75_5:u8
        let s_75_6: u8 = {
            let value = state.read_register::<u8>(s_75_5 as isize);
            tracer.read_register(s_75_5 as isize, value);
            value
        };
        // D s_75_7: call AArch64_AArch32SystemAccessTrap(s_75_6, s_75_4)
        let s_75_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_75_6, s_75_4);
        // N s_75_8: return
        return;
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var __MDCR_EL2_TPM:u8
        let s_76_0: bool = fn_state.u__MDCR_EL2_TPM;
        // D s_76_1: cast zx s_76_0 -> bv
        let s_76_1: Bits = Bits::new(s_76_0 as u128, 1u16);
        // C s_76_2: const #1u : u8
        let s_76_2: bool = true;
        // C s_76_3: cast zx s_76_2 -> bv
        let s_76_3: Bits = Bits::new(s_76_2 as u128, 1u16);
        // D s_76_4: cmp-eq s_76_1 s_76_3
        let s_76_4: bool = ((s_76_1) == (s_76_3));
        // D s_76_5: write-var gs#112669 <= s_76_4
        fn_state.gs_112669 = s_76_4;
        // N s_76_6: jump b52
        return block_52(state, tracer, fn_state);
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
        // D s_77_4: write-var gs#112668 <= s_77_3
        fn_state.gs_112668 = s_77_3;
        // N s_77_5: jump b50
        return block_50(state, tracer, fn_state);
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
        // D s_79_0: read-var __MDCR_EL3_TPM:u8
        let s_79_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_79_1: cast zx s_79_0 -> bv
        let s_79_1: Bits = Bits::new(s_79_0 as u128, 1u16);
        // C s_79_2: const #1u : u8
        let s_79_2: bool = true;
        // C s_79_3: cast zx s_79_2 -> bv
        let s_79_3: Bits = Bits::new(s_79_2 as u128, 1u16);
        // D s_79_4: cmp-eq s_79_1 s_79_3
        let s_79_4: bool = ((s_79_1) == (s_79_3));
        // D s_79_5: write-var gs#112667 <= s_79_4
        fn_state.gs_112667 = s_79_4;
        // N s_79_6: jump b47
        return block_47(state, tracer, fn_state);
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
        // D s_80_2: call ELUsingAArch32(s_80_1)
        let s_80_2: bool = ELUsingAArch32(state, tracer, s_80_1);
        // D s_80_3: not s_80_2
        let s_80_3: bool = !s_80_2;
        // D s_80_4: write-var gs#112666 <= s_80_3
        fn_state.gs_112666 = s_80_3;
        // N s_80_5: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_81_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_81_1: call __IMPDEF_boolean(s_81_0)
        let s_81_1: bool = u__IMPDEF_boolean(state, tracer, s_81_0);
        // D s_81_2: write-var gs#112665 <= s_81_1
        fn_state.gs_112665 = s_81_1;
        // N s_81_3: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #() : ()
        let s_82_0: () = ();
        // S s_82_1: call EDSCR_read(s_82_0)
        let s_82_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_82_0);
        // S s_82_2: call _get_EDSCR_Type_SDD(s_82_1)
        let s_82_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_82_1);
        // S s_82_3: cast zx s_82_2 -> bv
        let s_82_3: Bits = Bits::new(s_82_2 as u128, 1u16);
        // C s_82_4: const #1u : u8
        let s_82_4: bool = true;
        // C s_82_5: cast zx s_82_4 -> bv
        let s_82_5: Bits = Bits::new(s_82_4 as u128, 1u16);
        // S s_82_6: cmp-eq s_82_3 s_82_5
        let s_82_6: bool = ((s_82_3) == (s_82_5));
        // D s_82_7: write-var gs#112664 <= s_82_6
        fn_state.gs_112664 = s_82_6;
        // N s_82_8: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #424u : u32
        let s_83_0: u32 = 424;
        // D s_83_1: read-reg s_83_0:u8
        let s_83_1: u8 = {
            let value = state.read_register::<u8>(s_83_0 as isize);
            tracer.read_register(s_83_0 as isize, value);
            value
        };
        // C s_83_2: const #2u : u8
        let s_83_2: u8 = 2;
        // D s_83_3: cmp-lt s_83_1 s_83_2
        let s_83_3: bool = ((s_83_1) < (s_83_2));
        // D s_83_4: write-var gs#112663 <= s_83_3
        fn_state.gs_112663 = s_83_3;
        // N s_83_5: jump b39
        return block_39(state, tracer, fn_state);
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
        // N s_84_2: branch s_84_1 b193 b85
        if s_84_1 {
            return block_193(state, tracer, fn_state);
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
        // D s_85_1: write-var gs#112675 <= s_85_0
        fn_state.gs_112675 = s_85_0;
        // N s_85_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var gs#112675:u8
        let s_86_0: bool = fn_state.gs_112675;
        // N s_86_1: branch s_86_0 b192 b87
        if s_86_0 {
            return block_192(state, tracer, fn_state);
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
        // D s_87_1: write-var gs#112676 <= s_87_0
        fn_state.gs_112676 = s_87_0;
        // N s_87_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var gs#112676:u8
        let s_88_0: bool = fn_state.gs_112676;
        // N s_88_1: branch s_88_0 b191 b89
        if s_88_0 {
            return block_191(state, tracer, fn_state);
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
        // D s_89_1: write-var gs#112677 <= s_89_0
        fn_state.gs_112677 = s_89_0;
        // N s_89_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var gs#112677:u8
        let s_90_0: bool = fn_state.gs_112677;
        // N s_90_1: branch s_90_0 b190 b91
        if s_90_0 {
            return block_190(state, tracer, fn_state);
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
        // D s_91_1: write-var gs#112678 <= s_91_0
        fn_state.gs_112678 = s_91_0;
        // N s_91_2: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var gs#112678:u8
        let s_92_0: bool = fn_state.gs_112678;
        // N s_92_1: branch s_92_0 b189 b93
        if s_92_0 {
            return block_189(state, tracer, fn_state);
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
        // D s_93_1: write-var gs#112679 <= s_93_0
        fn_state.gs_112679 = s_93_0;
        // N s_93_2: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var gs#112679:u8
        let s_94_0: bool = fn_state.gs_112679;
        // N s_94_1: branch s_94_0 b188 b95
        if s_94_0 {
            return block_188(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #440u : u32
        let s_95_0: u32 = 440;
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
        // N s_95_4: branch s_95_3 b178 b96
        if s_95_3 {
            return block_178(state, tracer, fn_state);
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
        // D s_96_1: write-var gs#112683 <= s_96_0
        fn_state.gs_112683 = s_96_0;
        // N s_96_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var gs#112683:u8
        let s_97_0: bool = fn_state.gs_112683;
        // N s_97_1: branch s_97_0 b169 b98
        if s_97_0 {
            return block_169(state, tracer, fn_state);
        } else {
            return block_98(state, tracer, fn_state);
        };
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #440u : u32
        let s_98_0: u32 = 440;
        // D s_98_1: read-reg s_98_0:u8
        let s_98_1: u8 = {
            let value = state.read_register::<u8>(s_98_0 as isize);
            tracer.read_register(s_98_0 as isize, value);
            value
        };
        // D s_98_2: call ELUsingAArch32(s_98_1)
        let s_98_2: bool = ELUsingAArch32(state, tracer, s_98_1);
        // N s_98_3: branch s_98_2 b168 b99
        if s_98_2 {
            return block_168(state, tracer, fn_state);
        } else {
            return block_99(state, tracer, fn_state);
        };
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #0u : u8
        let s_99_0: bool = false;
        // D s_99_1: write-var gs#112684 <= s_99_0
        fn_state.gs_112684 = s_99_0;
        // N s_99_2: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_100_0: read-var gs#112684:u8
        let s_100_0: bool = fn_state.gs_112684;
        // N s_100_1: branch s_100_0 b151 b101
        if s_100_0 {
            return block_151(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #() : ()
        let s_101_0: () = ();
        // S s_101_1: call EL2Enabled(s_101_0)
        let s_101_1: bool = EL2Enabled(state, tracer, s_101_0);
        // N s_101_2: branch s_101_1 b150 b102
        if s_101_1 {
            return block_150(state, tracer, fn_state);
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
        // D s_102_1: write-var gs#112685 <= s_102_0
        fn_state.gs_112685 = s_102_0;
        // N s_102_2: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_103_0: read-var gs#112685:u8
        let s_103_0: bool = fn_state.gs_112685;
        // N s_103_1: branch s_103_0 b149 b104
        if s_103_0 {
            return block_149(state, tracer, fn_state);
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
        // D s_104_1: write-var gs#112686 <= s_104_0
        fn_state.gs_112686 = s_104_0;
        // N s_104_2: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_105_0: read-var gs#112686:u8
        let s_105_0: bool = fn_state.gs_112686;
        // N s_105_1: branch s_105_0 b148 b106
        if s_105_0 {
            return block_148(state, tracer, fn_state);
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
        // D s_106_1: write-var gs#112687 <= s_106_0
        fn_state.gs_112687 = s_106_0;
        // N s_106_2: jump b107
        return block_107(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_107_0: read-var gs#112687:u8
        let s_107_0: bool = fn_state.gs_112687;
        // N s_107_1: branch s_107_0 b144 b108
        if s_107_0 {
            return block_144(state, tracer, fn_state);
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
        // D s_108_1: write-var gs#112689 <= s_108_0
        fn_state.gs_112689 = s_108_0;
        // N s_108_2: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_109_0: read-var gs#112689:u8
        let s_109_0: bool = fn_state.gs_112689;
        // N s_109_1: branch s_109_0 b143 b110
        if s_109_0 {
            return block_143(state, tracer, fn_state);
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
        // D s_110_1: write-var gs#112690 <= s_110_0
        fn_state.gs_112690 = s_110_0;
        // N s_110_2: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var gs#112690:u8
        let s_111_0: bool = fn_state.gs_112690;
        // N s_111_1: branch s_111_0 b142 b112
        if s_111_0 {
            return block_142(state, tracer, fn_state);
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
        // N s_112_2: branch s_112_1 b141 b113
        if s_112_1 {
            return block_141(state, tracer, fn_state);
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
        // D s_113_1: write-var gs#112691 <= s_113_0
        fn_state.gs_112691 = s_113_0;
        // N s_113_2: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_114_0: read-var gs#112691:u8
        let s_114_0: bool = fn_state.gs_112691;
        // N s_114_1: branch s_114_0 b140 b115
        if s_114_0 {
            return block_140(state, tracer, fn_state);
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
        // D s_115_1: write-var gs#112692 <= s_115_0
        fn_state.gs_112692 = s_115_0;
        // N s_115_2: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_116_0: read-var gs#112692:u8
        let s_116_0: bool = fn_state.gs_112692;
        // N s_116_1: branch s_116_0 b139 b117
        if s_116_0 {
            return block_139(state, tracer, fn_state);
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
        // N s_117_2: branch s_117_1 b138 b118
        if s_117_1 {
            return block_138(state, tracer, fn_state);
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
        // D s_118_1: write-var gs#112693 <= s_118_0
        fn_state.gs_112693 = s_118_0;
        // N s_118_2: jump b119
        return block_119(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_119_0: read-var gs#112693:u8
        let s_119_0: bool = fn_state.gs_112693;
        // N s_119_1: branch s_119_0 b137 b120
        if s_119_0 {
            return block_137(state, tracer, fn_state);
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
        // D s_120_1: write-var gs#112694 <= s_120_0
        fn_state.gs_112694 = s_120_0;
        // N s_120_2: jump b121
        return block_121(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_121_0: read-var gs#112694:u8
        let s_121_0: bool = fn_state.gs_112694;
        // N s_121_1: branch s_121_0 b136 b122
        if s_121_0 {
            return block_136(state, tracer, fn_state);
        } else {
            return block_122(state, tracer, fn_state);
        };
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_122_0: const #424u : u32
        let s_122_0: u32 = 424;
        // D s_122_1: read-reg s_122_0:u8
        let s_122_1: u8 = {
            let value = state.read_register::<u8>(s_122_0 as isize);
            tracer.read_register(s_122_0 as isize, value);
            value
        };
        // C s_122_2: const #2u : u8
        let s_122_2: u8 = 2;
        // D s_122_3: cmp-lt s_122_1 s_122_2
        let s_122_3: bool = ((s_122_1) < (s_122_2));
        // N s_122_4: branch s_122_3 b135 b123
        if s_122_3 {
            return block_135(state, tracer, fn_state);
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
        // D s_123_1: write-var gs#112695 <= s_123_0
        fn_state.gs_112695 = s_123_0;
        // N s_123_2: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_124_0: read-var gs#112695:u8
        let s_124_0: bool = fn_state.gs_112695;
        // N s_124_1: branch s_124_0 b134 b125
        if s_124_0 {
            return block_134(state, tracer, fn_state);
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
        // D s_125_1: write-var gs#112696 <= s_125_0
        fn_state.gs_112696 = s_125_0;
        // N s_125_2: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_126_0: read-var gs#112696:u8
        let s_126_0: bool = fn_state.gs_112696;
        // N s_126_1: branch s_126_0 b128 b127
        if s_126_0 {
            return block_128(state, tracer, fn_state);
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
        // S s_127_1: call PMCCFILTR_read(s_127_0)
        let s_127_1: ProductType700c18a878c5601b = PMCCFILTR_read(
            state,
            tracer,
            s_127_0,
        );
        // S s_127_2: call __get_PMCCFILTR(s_127_1)
        let s_127_2: ProductType700c18a878c5601b = u__get_PMCCFILTR(
            state,
            tracer,
            s_127_1,
        );
        // D s_127_3: write-var ga#184397 <= s_127_2
        fn_state.ga_184397 = s_127_2;
        // D s_127_4: read-var ga#184397.0:struct
        let s_127_4: u32 = fn_state.ga_184397._0;
        // D s_127_5: read-var t:i
        let s_127_5: i128 = fn_state.t;
        // D s_127_6: call R_set(s_127_5, s_127_4)
        let s_127_6: () = R_set(state, tracer, s_127_5, s_127_4);
        // N s_127_7: return
        return;
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_128_0: const #() : ()
        let s_128_0: () = ();
        // S s_128_1: call Halted(s_128_0)
        let s_128_1: bool = Halted(state, tracer, s_128_0);
        // N s_128_2: branch s_128_1 b133 b129
        if s_128_1 {
            return block_133(state, tracer, fn_state);
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
        // D s_129_1: write-var gs#112697 <= s_129_0
        fn_state.gs_112697 = s_129_0;
        // N s_129_2: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_130_0: read-var gs#112697:u8
        let s_130_0: bool = fn_state.gs_112697;
        // N s_130_1: branch s_130_0 b132 b131
        if s_130_0 {
            return block_132(state, tracer, fn_state);
        } else {
            return block_131(state, tracer, fn_state);
        };
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_131_0: const #3u : u8
        let s_131_0: u8 = 3;
        // C s_131_1: cast zx s_131_0 -> bv
        let s_131_1: Bits = Bits::new(s_131_0 as u128, 8u16);
        // C s_131_2: cast zx s_131_1 -> i
        let s_131_2: i128 = (s_131_1.value() as i128);
        // C s_131_3: cast reint s_131_2 -> i64
        let s_131_3: i64 = (s_131_2 as i64);
        // C s_131_4: cast zx s_131_3 -> i
        let s_131_4: i128 = (i128::try_from(s_131_3).unwrap());
        // C s_131_5: const #424u : u32
        let s_131_5: u32 = 424;
        // D s_131_6: read-reg s_131_5:u8
        let s_131_6: u8 = {
            let value = state.read_register::<u8>(s_131_5 as isize);
            tracer.read_register(s_131_5 as isize, value);
            value
        };
        // D s_131_7: call AArch64_AArch32SystemAccessTrap(s_131_6, s_131_4)
        let s_131_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_131_6,
            s_131_4,
        );
        // N s_131_8: return
        return;
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_132_0: panic
        panic!("{:?}", ());
        // N s_132_1: return
        return;
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
        // D s_133_7: write-var gs#112697 <= s_133_6
        fn_state.gs_112697 = s_133_6;
        // N s_133_8: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_134_0: read-var __MDCR_EL3_TPM:u8
        let s_134_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_134_1: cast zx s_134_0 -> bv
        let s_134_1: Bits = Bits::new(s_134_0 as u128, 1u16);
        // C s_134_2: const #1u : u8
        let s_134_2: bool = true;
        // C s_134_3: cast zx s_134_2 -> bv
        let s_134_3: Bits = Bits::new(s_134_2 as u128, 1u16);
        // D s_134_4: cmp-eq s_134_1 s_134_3
        let s_134_4: bool = ((s_134_1) == (s_134_3));
        // D s_134_5: write-var gs#112696 <= s_134_4
        fn_state.gs_112696 = s_134_4;
        // N s_134_6: jump b126
        return block_126(state, tracer, fn_state);
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
        // D s_135_4: write-var gs#112695 <= s_135_3
        fn_state.gs_112695 = s_135_3;
        // N s_135_5: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_136_0: const #3u : u8
        let s_136_0: u8 = 3;
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
        // D s_137_0: read-var __HDCR_TPM:u8
        let s_137_0: bool = fn_state.u__HDCR_TPM;
        // D s_137_1: cast zx s_137_0 -> bv
        let s_137_1: Bits = Bits::new(s_137_0 as u128, 1u16);
        // C s_137_2: const #1u : u8
        let s_137_2: bool = true;
        // C s_137_3: cast zx s_137_2 -> bv
        let s_137_3: Bits = Bits::new(s_137_2 as u128, 1u16);
        // D s_137_4: cmp-eq s_137_1 s_137_3
        let s_137_4: bool = ((s_137_1) == (s_137_3));
        // D s_137_5: write-var gs#112694 <= s_137_4
        fn_state.gs_112694 = s_137_4;
        // N s_137_6: jump b121
        return block_121(state, tracer, fn_state);
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
        // D s_138_3: write-var gs#112693 <= s_138_2
        fn_state.gs_112693 = s_138_2;
        // N s_138_4: jump b119
        return block_119(state, tracer, fn_state);
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
        // D s_140_0: read-var __MDCR_EL2_TPM:u8
        let s_140_0: bool = fn_state.u__MDCR_EL2_TPM;
        // D s_140_1: cast zx s_140_0 -> bv
        let s_140_1: Bits = Bits::new(s_140_0 as u128, 1u16);
        // C s_140_2: const #1u : u8
        let s_140_2: bool = true;
        // C s_140_3: cast zx s_140_2 -> bv
        let s_140_3: Bits = Bits::new(s_140_2 as u128, 1u16);
        // D s_140_4: cmp-eq s_140_1 s_140_3
        let s_140_4: bool = ((s_140_1) == (s_140_3));
        // D s_140_5: write-var gs#112692 <= s_140_4
        fn_state.gs_112692 = s_140_4;
        // N s_140_6: jump b116
        return block_116(state, tracer, fn_state);
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
        // D s_141_4: write-var gs#112691 <= s_141_3
        fn_state.gs_112691 = s_141_3;
        // N s_141_5: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_142_0: const #3u : u8
        let s_142_0: u8 = 3;
        // C s_142_1: cast zx s_142_0 -> bv
        let s_142_1: Bits = Bits::new(s_142_0 as u128, 8u16);
        // C s_142_2: cast zx s_142_1 -> i
        let s_142_2: i128 = (s_142_1.value() as i128);
        // C s_142_3: cast reint s_142_2 -> i64
        let s_142_3: i64 = (s_142_2 as i64);
        // C s_142_4: cast zx s_142_3 -> i
        let s_142_4: i128 = (i128::try_from(s_142_3).unwrap());
        // C s_142_5: const #432u : u32
        let s_142_5: u32 = 432;
        // D s_142_6: read-reg s_142_5:u8
        let s_142_6: u8 = {
            let value = state.read_register::<u8>(s_142_5 as isize);
            tracer.read_register(s_142_5 as isize, value);
            value
        };
        // D s_142_7: call AArch64_AArch32SystemAccessTrap(s_142_6, s_142_4)
        let s_142_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_142_6,
            s_142_4,
        );
        // N s_142_8: return
        return;
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_143_0: read-var __HDFGRTR_EL2_PMCCFILTR_EL0:u8
        let s_143_0: bool = fn_state.u__HDFGRTR_EL2_PMCCFILTR_EL0;
        // D s_143_1: cast zx s_143_0 -> bv
        let s_143_1: Bits = Bits::new(s_143_0 as u128, 1u16);
        // C s_143_2: const #1u : u8
        let s_143_2: bool = true;
        // C s_143_3: cast zx s_143_2 -> bv
        let s_143_3: Bits = Bits::new(s_143_2 as u128, 1u16);
        // D s_143_4: cmp-eq s_143_1 s_143_3
        let s_143_4: bool = ((s_143_1) == (s_143_3));
        // D s_143_5: write-var gs#112690 <= s_143_4
        fn_state.gs_112690 = s_143_4;
        // N s_143_6: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_144_0: const #424u : u32
        let s_144_0: u32 = 424;
        // D s_144_1: read-reg s_144_0:u8
        let s_144_1: u8 = {
            let value = state.read_register::<u8>(s_144_0 as isize);
            tracer.read_register(s_144_0 as isize, value);
            value
        };
        // C s_144_2: const #2u : u8
        let s_144_2: u8 = 2;
        // D s_144_3: cmp-lt s_144_1 s_144_2
        let s_144_3: bool = ((s_144_1) < (s_144_2));
        // D s_144_4: not s_144_3
        let s_144_4: bool = !s_144_3;
        // N s_144_5: branch s_144_4 b147 b145
        if s_144_4 {
            return block_147(state, tracer, fn_state);
        } else {
            return block_145(state, tracer, fn_state);
        };
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_145_0: read-var __SCR_EL3_FGTEn:u8
        let s_145_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_145_1: cast zx s_145_0 -> bv
        let s_145_1: Bits = Bits::new(s_145_0 as u128, 1u16);
        // C s_145_2: const #1u : u8
        let s_145_2: bool = true;
        // C s_145_3: cast zx s_145_2 -> bv
        let s_145_3: Bits = Bits::new(s_145_2 as u128, 1u16);
        // D s_145_4: cmp-eq s_145_1 s_145_3
        let s_145_4: bool = ((s_145_1) == (s_145_3));
        // D s_145_5: write-var gs#112688 <= s_145_4
        fn_state.gs_112688 = s_145_4;
        // N s_145_6: jump b146
        return block_146(state, tracer, fn_state);
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_146_0: read-var gs#112688:u8
        let s_146_0: bool = fn_state.gs_112688;
        // D s_146_1: write-var gs#112689 <= s_146_0
        fn_state.gs_112689 = s_146_0;
        // N s_146_2: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_147_0: const #1u : u8
        let s_147_0: bool = true;
        // D s_147_1: write-var gs#112688 <= s_147_0
        fn_state.gs_112688 = s_147_0;
        // N s_147_2: jump b146
        return block_146(state, tracer, fn_state);
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_148_0: const #146u : u32
        let s_148_0: u32 = 146;
        // S s_148_1: call IsFeatureImplemented(s_148_0)
        let s_148_1: bool = IsFeatureImplemented(state, tracer, s_148_0);
        // D s_148_2: write-var gs#112687 <= s_148_1
        fn_state.gs_112687 = s_148_1;
        // N s_148_3: jump b107
        return block_107(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_149_0: const #102552u : u32
        let s_149_0: u32 = 102552;
        // D s_149_1: read-reg s_149_0:struct
        let s_149_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_149_0 as isize);
            tracer.read_register(s_149_0 as isize, value);
            value
        };
        // D s_149_2: call _get_HCR_EL2_Type_E2H(s_149_1)
        let s_149_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_149_1);
        // C s_149_3: const #102552u : u32
        let s_149_3: u32 = 102552;
        // D s_149_4: read-reg s_149_3:struct
        let s_149_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_149_3 as isize);
            tracer.read_register(s_149_3 as isize, value);
            value
        };
        // D s_149_5: call _get_HCR_EL2_Type_TGE(s_149_4)
        let s_149_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_149_4);
        // D s_149_6: cast zx s_149_2 -> bv
        let s_149_6: Bits = Bits::new(s_149_2 as u128, 1u16);
        // D s_149_7: cast zx s_149_5 -> bv
        let s_149_7: Bits = Bits::new(s_149_5 as u128, 1u16);
        // D s_149_8: cast reint s_149_6 -> u128
        let s_149_8: u128 = (s_149_6.value() as u128);
        // D s_149_9: size-of s_149_6
        let s_149_9: u16 = s_149_6.length();
        // D s_149_10: cast reint s_149_7 -> u128
        let s_149_10: u128 = (s_149_7.value() as u128);
        // D s_149_11: size-of s_149_7
        let s_149_11: u16 = s_149_7.length();
        // D s_149_12: lsl s_149_8 s_149_11
        let s_149_12: u128 = s_149_8 << s_149_11;
        // D s_149_13: or s_149_12 s_149_10
        let s_149_13: u128 = ((s_149_12) | (s_149_10));
        // D s_149_14: add s_149_9 s_149_11
        let s_149_14: u16 = (s_149_9 + s_149_11);
        // D s_149_15: create-bits s_149_13 s_149_14
        let s_149_15: Bits = Bits::new(s_149_13, s_149_14);
        // D s_149_16: cast reint s_149_15 -> u8
        let s_149_16: u8 = (s_149_15.value() as u8);
        // D s_149_17: cast zx s_149_16 -> bv
        let s_149_17: Bits = Bits::new(s_149_16 as u128, 2u16);
        // C s_149_18: const #3u : u8
        let s_149_18: u8 = 3;
        // C s_149_19: cast zx s_149_18 -> bv
        let s_149_19: Bits = Bits::new(s_149_18 as u128, 2u16);
        // D s_149_20: cmp-ne s_149_17 s_149_19
        let s_149_20: bool = ((s_149_17) != (s_149_19));
        // D s_149_21: write-var gs#112686 <= s_149_20
        fn_state.gs_112686 = s_149_20;
        // N s_149_22: jump b105
        return block_105(state, tracer, fn_state);
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
        // D s_150_4: write-var gs#112685 <= s_150_3
        fn_state.gs_112685 = s_150_3;
        // N s_150_5: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_151_0: const #() : ()
        let s_151_0: () = ();
        // S s_151_1: call EL2Enabled(s_151_0)
        let s_151_1: bool = EL2Enabled(state, tracer, s_151_0);
        // N s_151_2: branch s_151_1 b167 b152
        if s_151_1 {
            return block_167(state, tracer, fn_state);
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
        // D s_152_1: write-var gs#112698 <= s_152_0
        fn_state.gs_112698 = s_152_0;
        // N s_152_2: jump b153
        return block_153(state, tracer, fn_state);
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_153_0: read-var gs#112698:u8
        let s_153_0: bool = fn_state.gs_112698;
        // N s_153_1: branch s_153_0 b166 b154
        if s_153_0 {
            return block_166(state, tracer, fn_state);
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
        // D s_154_1: write-var gs#112699 <= s_154_0
        fn_state.gs_112699 = s_154_0;
        // N s_154_2: jump b155
        return block_155(state, tracer, fn_state);
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_155_0: read-var gs#112699:u8
        let s_155_0: bool = fn_state.gs_112699;
        // N s_155_1: branch s_155_0 b165 b156
        if s_155_0 {
            return block_165(state, tracer, fn_state);
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
        // N s_156_2: branch s_156_1 b164 b157
        if s_156_1 {
            return block_164(state, tracer, fn_state);
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
        // D s_157_1: write-var gs#112700 <= s_157_0
        fn_state.gs_112700 = s_157_0;
        // N s_157_2: jump b158
        return block_158(state, tracer, fn_state);
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_158_0: read-var gs#112700:u8
        let s_158_0: bool = fn_state.gs_112700;
        // N s_158_1: branch s_158_0 b163 b159
        if s_158_0 {
            return block_163(state, tracer, fn_state);
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
        // D s_159_1: write-var gs#112701 <= s_159_0
        fn_state.gs_112701 = s_159_0;
        // N s_159_2: jump b160
        return block_160(state, tracer, fn_state);
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_160_0: read-var gs#112701:u8
        let s_160_0: bool = fn_state.gs_112701;
        // N s_160_1: branch s_160_0 b162 b161
        if s_160_0 {
            return block_162(state, tracer, fn_state);
        } else {
            return block_161(state, tracer, fn_state);
        };
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
        // C s_162_0: const #0u : u8
        let s_162_0: u8 = 0;
        // C s_162_1: cast zx s_162_0 -> bv
        let s_162_1: Bits = Bits::new(s_162_0 as u128, 8u16);
        // C s_162_2: cast zx s_162_1 -> i
        let s_162_2: i128 = (s_162_1.value() as i128);
        // C s_162_3: cast reint s_162_2 -> i64
        let s_162_3: i64 = (s_162_2 as i64);
        // C s_162_4: cast zx s_162_3 -> i
        let s_162_4: i128 = (i128::try_from(s_162_3).unwrap());
        // S s_162_5: call AArch32_TakeHypTrapException(s_162_4)
        let s_162_5: () = AArch32_TakeHypTrapException(state, tracer, s_162_4);
        // N s_162_6: return
        return;
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_163_0: read-var __HCR_TGE:u8
        let s_163_0: bool = fn_state.u__HCR_TGE;
        // D s_163_1: cast zx s_163_0 -> bv
        let s_163_1: Bits = Bits::new(s_163_0 as u128, 1u16);
        // C s_163_2: const #1u : u8
        let s_163_2: bool = true;
        // C s_163_3: cast zx s_163_2 -> bv
        let s_163_3: Bits = Bits::new(s_163_2 as u128, 1u16);
        // D s_163_4: cmp-eq s_163_1 s_163_3
        let s_163_4: bool = ((s_163_1) == (s_163_3));
        // D s_163_5: write-var gs#112701 <= s_163_4
        fn_state.gs_112701 = s_163_4;
        // N s_163_6: jump b160
        return block_160(state, tracer, fn_state);
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_164_0: const #432u : u32
        let s_164_0: u32 = 432;
        // D s_164_1: read-reg s_164_0:u8
        let s_164_1: u8 = {
            let value = state.read_register::<u8>(s_164_0 as isize);
            tracer.read_register(s_164_0 as isize, value);
            value
        };
        // D s_164_2: call ELUsingAArch32(s_164_1)
        let s_164_2: bool = ELUsingAArch32(state, tracer, s_164_1);
        // D s_164_3: write-var gs#112700 <= s_164_2
        fn_state.gs_112700 = s_164_2;
        // N s_164_4: jump b158
        return block_158(state, tracer, fn_state);
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
        // D s_166_0: read-var __HCR_EL2_TGE:u8
        let s_166_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_166_1: cast zx s_166_0 -> bv
        let s_166_1: Bits = Bits::new(s_166_0 as u128, 1u16);
        // C s_166_2: const #1u : u8
        let s_166_2: bool = true;
        // C s_166_3: cast zx s_166_2 -> bv
        let s_166_3: Bits = Bits::new(s_166_2 as u128, 1u16);
        // D s_166_4: cmp-eq s_166_1 s_166_3
        let s_166_4: bool = ((s_166_1) == (s_166_3));
        // D s_166_5: write-var gs#112699 <= s_166_4
        fn_state.gs_112699 = s_166_4;
        // N s_166_6: jump b155
        return block_155(state, tracer, fn_state);
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_167_0: const #432u : u32
        let s_167_0: u32 = 432;
        // D s_167_1: read-reg s_167_0:u8
        let s_167_1: u8 = {
            let value = state.read_register::<u8>(s_167_0 as isize);
            tracer.read_register(s_167_0 as isize, value);
            value
        };
        // D s_167_2: call ELUsingAArch32(s_167_1)
        let s_167_2: bool = ELUsingAArch32(state, tracer, s_167_1);
        // D s_167_3: not s_167_2
        let s_167_3: bool = !s_167_2;
        // D s_167_4: write-var gs#112698 <= s_167_3
        fn_state.gs_112698 = s_167_3;
        // N s_167_5: jump b153
        return block_153(state, tracer, fn_state);
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_168_0: read-var __PMUSERENR_EN:u8
        let s_168_0: bool = fn_state.u__PMUSERENR_EN;
        // D s_168_1: cast zx s_168_0 -> bv
        let s_168_1: Bits = Bits::new(s_168_0 as u128, 1u16);
        // C s_168_2: const #0u : u8
        let s_168_2: bool = false;
        // C s_168_3: cast zx s_168_2 -> bv
        let s_168_3: Bits = Bits::new(s_168_2 as u128, 1u16);
        // D s_168_4: cmp-eq s_168_1 s_168_3
        let s_168_4: bool = ((s_168_1) == (s_168_3));
        // D s_168_5: write-var gs#112684 <= s_168_4
        fn_state.gs_112684 = s_168_4;
        // N s_168_6: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_169_0: const #() : ()
        let s_169_0: () = ();
        // S s_169_1: call EL2Enabled(s_169_0)
        let s_169_1: bool = EL2Enabled(state, tracer, s_169_0);
        // N s_169_2: branch s_169_1 b177 b170
        if s_169_1 {
            return block_177(state, tracer, fn_state);
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
        // D s_170_1: write-var gs#112702 <= s_170_0
        fn_state.gs_112702 = s_170_0;
        // N s_170_2: jump b171
        return block_171(state, tracer, fn_state);
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_171_0: read-var gs#112702:u8
        let s_171_0: bool = fn_state.gs_112702;
        // N s_171_1: branch s_171_0 b176 b172
        if s_171_0 {
            return block_176(state, tracer, fn_state);
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
        // D s_172_1: write-var gs#112703 <= s_172_0
        fn_state.gs_112703 = s_172_0;
        // N s_172_2: jump b173
        return block_173(state, tracer, fn_state);
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_173_0: read-var gs#112703:u8
        let s_173_0: bool = fn_state.gs_112703;
        // N s_173_1: branch s_173_0 b175 b174
        if s_173_0 {
            return block_175(state, tracer, fn_state);
        } else {
            return block_174(state, tracer, fn_state);
        };
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
        // C s_174_5: const #440u : u32
        let s_174_5: u32 = 440;
        // D s_174_6: read-reg s_174_5:u8
        let s_174_6: u8 = {
            let value = state.read_register::<u8>(s_174_5 as isize);
            tracer.read_register(s_174_5 as isize, value);
            value
        };
        // D s_174_7: call AArch64_AArch32SystemAccessTrap(s_174_6, s_174_4)
        let s_174_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_174_6,
            s_174_4,
        );
        // N s_174_8: return
        return;
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_175_0: const #3u : u8
        let s_175_0: u8 = 3;
        // C s_175_1: cast zx s_175_0 -> bv
        let s_175_1: Bits = Bits::new(s_175_0 as u128, 8u16);
        // C s_175_2: cast zx s_175_1 -> i
        let s_175_2: i128 = (s_175_1.value() as i128);
        // C s_175_3: cast reint s_175_2 -> i64
        let s_175_3: i64 = (s_175_2 as i64);
        // C s_175_4: cast zx s_175_3 -> i
        let s_175_4: i128 = (i128::try_from(s_175_3).unwrap());
        // C s_175_5: const #432u : u32
        let s_175_5: u32 = 432;
        // D s_175_6: read-reg s_175_5:u8
        let s_175_6: u8 = {
            let value = state.read_register::<u8>(s_175_5 as isize);
            tracer.read_register(s_175_5 as isize, value);
            value
        };
        // D s_175_7: call AArch64_AArch32SystemAccessTrap(s_175_6, s_175_4)
        let s_175_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_175_6,
            s_175_4,
        );
        // N s_175_8: return
        return;
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_176_0: read-var __HCR_EL2_TGE:u8
        let s_176_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_176_1: cast zx s_176_0 -> bv
        let s_176_1: Bits = Bits::new(s_176_0 as u128, 1u16);
        // C s_176_2: const #1u : u8
        let s_176_2: bool = true;
        // C s_176_3: cast zx s_176_2 -> bv
        let s_176_3: Bits = Bits::new(s_176_2 as u128, 1u16);
        // D s_176_4: cmp-eq s_176_1 s_176_3
        let s_176_4: bool = ((s_176_1) == (s_176_3));
        // D s_176_5: write-var gs#112703 <= s_176_4
        fn_state.gs_112703 = s_176_4;
        // N s_176_6: jump b173
        return block_173(state, tracer, fn_state);
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_177_0: const #432u : u32
        let s_177_0: u32 = 432;
        // D s_177_1: read-reg s_177_0:u8
        let s_177_1: u8 = {
            let value = state.read_register::<u8>(s_177_0 as isize);
            tracer.read_register(s_177_0 as isize, value);
            value
        };
        // D s_177_2: call ELUsingAArch32(s_177_1)
        let s_177_2: bool = ELUsingAArch32(state, tracer, s_177_1);
        // D s_177_3: not s_177_2
        let s_177_3: bool = !s_177_2;
        // D s_177_4: write-var gs#112702 <= s_177_3
        fn_state.gs_112702 = s_177_3;
        // N s_177_5: jump b171
        return block_171(state, tracer, fn_state);
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_178_0: const #204u : u32
        let s_178_0: u32 = 204;
        // S s_178_1: call IsFeatureImplemented(s_178_0)
        let s_178_1: bool = IsFeatureImplemented(state, tracer, s_178_0);
        // N s_178_2: branch s_178_1 b187 b179
        if s_178_1 {
            return block_187(state, tracer, fn_state);
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
        // D s_179_1: write-var gs#112680 <= s_179_0
        fn_state.gs_112680 = s_179_0;
        // N s_179_2: jump b180
        return block_180(state, tracer, fn_state);
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_180_0: read-var gs#112680:u8
        let s_180_0: bool = fn_state.gs_112680;
        // N s_180_1: branch s_180_0 b186 b181
        if s_180_0 {
            return block_186(state, tracer, fn_state);
        } else {
            return block_181(state, tracer, fn_state);
        };
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_181_0: const #204u : u32
        let s_181_0: u32 = 204;
        // S s_181_1: call IsFeatureImplemented(s_181_0)
        let s_181_1: bool = IsFeatureImplemented(state, tracer, s_181_0);
        // S s_181_2: not s_181_1
        let s_181_2: bool = !s_181_1;
        // N s_181_3: branch s_181_2 b185 b182
        if s_181_2 {
            return block_185(state, tracer, fn_state);
        } else {
            return block_182(state, tracer, fn_state);
        };
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_182_0: const #0u : u8
        let s_182_0: bool = false;
        // D s_182_1: write-var gs#112681 <= s_182_0
        fn_state.gs_112681 = s_182_0;
        // N s_182_2: jump b183
        return block_183(state, tracer, fn_state);
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_183_0: read-var gs#112681:u8
        let s_183_0: bool = fn_state.gs_112681;
        // D s_183_1: write-var gs#112682 <= s_183_0
        fn_state.gs_112682 = s_183_0;
        // N s_183_2: jump b184
        return block_184(state, tracer, fn_state);
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_184_0: read-var gs#112682:u8
        let s_184_0: bool = fn_state.gs_112682;
        // D s_184_1: write-var gs#112683 <= s_184_0
        fn_state.gs_112683 = s_184_0;
        // N s_184_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_185<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_185_0: read-var __PMUSERENR_EL0_EN:u8
        let s_185_0: bool = fn_state.u__PMUSERENR_EL0_EN;
        // D s_185_1: cast zx s_185_0 -> bv
        let s_185_1: Bits = Bits::new(s_185_0 as u128, 1u16);
        // C s_185_2: const #0u : u8
        let s_185_2: bool = false;
        // C s_185_3: cast zx s_185_2 -> bv
        let s_185_3: Bits = Bits::new(s_185_2 as u128, 1u16);
        // D s_185_4: cmp-eq s_185_1 s_185_3
        let s_185_4: bool = ((s_185_1) == (s_185_3));
        // D s_185_5: write-var gs#112681 <= s_185_4
        fn_state.gs_112681 = s_185_4;
        // N s_185_6: jump b183
        return block_183(state, tracer, fn_state);
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_186_0: const #1u : u8
        let s_186_0: bool = true;
        // D s_186_1: write-var gs#112682 <= s_186_0
        fn_state.gs_112682 = s_186_0;
        // N s_186_2: jump b184
        return block_184(state, tracer, fn_state);
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_187_0: const #102624u : u32
        let s_187_0: u32 = 102624;
        // D s_187_1: read-reg s_187_0:struct
        let s_187_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_187_0 as isize);
            tracer.read_register(s_187_0 as isize, value);
            value
        };
        // D s_187_2: call _get_PMUSERENR_EL0_Type_UEN(s_187_1)
        let s_187_2: bool = u_get_PMUSERENR_EL0_Type_UEN(state, tracer, s_187_1);
        // C s_187_3: const #102624u : u32
        let s_187_3: u32 = 102624;
        // D s_187_4: read-reg s_187_3:struct
        let s_187_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_187_3 as isize);
            tracer.read_register(s_187_3 as isize, value);
            value
        };
        // D s_187_5: call _get_PMUSERENR_EL0_Type_EN(s_187_4)
        let s_187_5: bool = u_get_PMUSERENR_EL0_Type_EN(state, tracer, s_187_4);
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
        // C s_187_18: const #0u : u8
        let s_187_18: u8 = 0;
        // C s_187_19: cast zx s_187_18 -> bv
        let s_187_19: Bits = Bits::new(s_187_18 as u128, 2u16);
        // D s_187_20: cmp-eq s_187_17 s_187_19
        let s_187_20: bool = ((s_187_17) == (s_187_19));
        // D s_187_21: write-var gs#112680 <= s_187_20
        fn_state.gs_112680 = s_187_20;
        // N s_187_22: jump b180
        return block_180(state, tracer, fn_state);
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
        // D s_189_0: read-var __MDCR_EL3_TPM:u8
        let s_189_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_189_1: cast zx s_189_0 -> bv
        let s_189_1: Bits = Bits::new(s_189_0 as u128, 1u16);
        // C s_189_2: const #1u : u8
        let s_189_2: bool = true;
        // C s_189_3: cast zx s_189_2 -> bv
        let s_189_3: Bits = Bits::new(s_189_2 as u128, 1u16);
        // D s_189_4: cmp-eq s_189_1 s_189_3
        let s_189_4: bool = ((s_189_1) == (s_189_3));
        // D s_189_5: write-var gs#112679 <= s_189_4
        fn_state.gs_112679 = s_189_4;
        // N s_189_6: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_190_0: const #424u : u32
        let s_190_0: u32 = 424;
        // D s_190_1: read-reg s_190_0:u8
        let s_190_1: u8 = {
            let value = state.read_register::<u8>(s_190_0 as isize);
            tracer.read_register(s_190_0 as isize, value);
            value
        };
        // D s_190_2: call ELUsingAArch32(s_190_1)
        let s_190_2: bool = ELUsingAArch32(state, tracer, s_190_1);
        // D s_190_3: not s_190_2
        let s_190_3: bool = !s_190_2;
        // D s_190_4: write-var gs#112678 <= s_190_3
        fn_state.gs_112678 = s_190_3;
        // N s_190_5: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_191_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_191_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_191_1: call __IMPDEF_boolean(s_191_0)
        let s_191_1: bool = u__IMPDEF_boolean(state, tracer, s_191_0);
        // D s_191_2: write-var gs#112677 <= s_191_1
        fn_state.gs_112677 = s_191_1;
        // N s_191_3: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_192_0: const #() : ()
        let s_192_0: () = ();
        // S s_192_1: call EDSCR_read(s_192_0)
        let s_192_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_192_0);
        // S s_192_2: call _get_EDSCR_Type_SDD(s_192_1)
        let s_192_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_192_1);
        // S s_192_3: cast zx s_192_2 -> bv
        let s_192_3: Bits = Bits::new(s_192_2 as u128, 1u16);
        // C s_192_4: const #1u : u8
        let s_192_4: bool = true;
        // C s_192_5: cast zx s_192_4 -> bv
        let s_192_5: Bits = Bits::new(s_192_4 as u128, 1u16);
        // S s_192_6: cmp-eq s_192_3 s_192_5
        let s_192_6: bool = ((s_192_3) == (s_192_5));
        // D s_192_7: write-var gs#112676 <= s_192_6
        fn_state.gs_112676 = s_192_6;
        // N s_192_8: jump b88
        return block_88(state, tracer, fn_state);
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
        // D s_193_4: write-var gs#112675 <= s_193_3
        fn_state.gs_112675 = s_193_3;
        // N s_193_5: jump b86
        return block_86(state, tracer, fn_state);
    }
}
