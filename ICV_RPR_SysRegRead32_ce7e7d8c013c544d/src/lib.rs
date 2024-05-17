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
use u_get_SCR_Type_IRQ::*;
use Halted::*;
use ICV_RPR_read::*;
use u__get_ICV_RPR::*;
use ICC_HSRE_read::*;
use u_get_HCR_Type_FMO::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_HCR_Type_IMO::*;
use EL2Enabled::*;
use u_get_HSTR_Type_T12::*;
use u_get_ICC_MSRE_Type_SRE::*;
use u_get_SCR_Type_FIQ::*;
use HCR_read::*;
use u_get_ICH_HCR_Type_TC::*;
use u_get_ICC_HSRE_Type_SRE::*;
use u_get_SCR_EL3_Type_IRQ::*;
use u__IMPDEF_boolean::*;
use u_get_HSTR_EL2_Type_T12::*;
use HSTR_read::*;
use u__get_ICC_RPR::*;
use u_get_HCR_EL2_Type_FMO::*;
use AArch64_AArch32SystemAccessTrap::*;
use u_get_SCR_EL3_Type_FIQ::*;
use R_set::*;
use ELUsingAArch32::*;
use ICC_RPR_read::*;
use u_get_ICH_HCR_EL2_Type_TC::*;
use u_get_HCR_EL2_Type_IMO::*;
use ICH_HCR_read::*;
use AArch32_TakeMonitorTrapException::*;
use EDSCR_read::*;
use common::*;
pub fn ICV_RPR_SysRegRead32_ce7e7d8c013c544d<T: Tracer>(
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
        gs_112364: bool,
        gs_112362: bool,
        u__HCR_EL2_IMO: bool,
        gs_112365: bool,
        gs_112377: bool,
        gs_112384: bool,
        gs_112372: bool,
        gs_112382: bool,
        gs_112356: bool,
        gs_112378: bool,
        gs_112343: bool,
        ga_183319: ProductType700c18a878c5601b,
        gs_112376: bool,
        gs_112367: bool,
        gs_112355: bool,
        gs_112369: bool,
        gs_112340: bool,
        ga_183218: ProductType700c18a878c5601b,
        u__HSTR_EL2_T12: bool,
        gs_112352: bool,
        u__HCR_EL2_FMO: bool,
        gs_112373: bool,
        gs_112363: bool,
        gs_112357: bool,
        gs_112358: bool,
        u__PSTATE_EL: u8,
        ga_183230: ProductType700c18a878c5601b,
        ga_183211: ProductType700c18a878c5601b,
        gs_112342: bool,
        gs_112385: bool,
        gs_112371: bool,
        ga_183313: ProductType700c18a878c5601b,
        gs_112380: bool,
        u__ICH_HCR_EL2_TC: bool,
        gs_112354: bool,
        u__ICC_HSRE_SRE: bool,
        u__HSTR_T12: bool,
        gs_112374: bool,
        u__ICH_HCR_TC: bool,
        gs_112383: bool,
        gs_112345: bool,
        u__HCR_IMO: bool,
        gs_112389: bool,
        gs_112388: bool,
        gs_112370: bool,
        ga_183258: ProductType700c18a878c5601b,
        u__ICC_MSRE_SRE: bool,
        gs_112353: bool,
        gs_112350: bool,
        gs_112375: bool,
        gs_112344: bool,
        gs_112341: bool,
        ga_183224: ProductType700c18a878c5601b,
        gs_112381: bool,
        u__HCR_FMO: bool,
        gs_112360: bool,
        u__PSTATE_M: u8,
        gs_112346: bool,
        gs_112359: bool,
        gs_112347: bool,
        gs_112368: bool,
        gs_112387: bool,
        gs_112361: bool,
        gs_112366: bool,
        gs_112351: bool,
        gs_112348: bool,
        gs_112386: bool,
        gs_112379: bool,
        gs_112349: bool,
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
        // C s_0_3: const #16983u : u32
        let s_0_3: u32 = 16983;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: u8 = {
            let value = state.read_register::<u8>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: write-var __PSTATE_M <= s_0_4
        fn_state.u__PSTATE_M = s_0_4;
        // C s_0_6: const #104936u : u32
        let s_0_6: u32 = 104936;
        // D s_0_7: read-reg s_0_6:struct
        let s_0_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_6 as isize);
            tracer.read_register(s_0_6 as isize, value);
            value
        };
        // D s_0_8: call _get_HSTR_EL2_Type_T12(s_0_7)
        let s_0_8: bool = u_get_HSTR_EL2_Type_T12(state, tracer, s_0_7);
        // D s_0_9: write-var __HSTR_EL2_T12 <= s_0_8
        fn_state.u__HSTR_EL2_T12 = s_0_8;
        // C s_0_10: const #() : ()
        let s_0_10: () = ();
        // S s_0_11: call HSTR_read(s_0_10)
        let s_0_11: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_10);
        // S s_0_12: call _get_HSTR_Type_T12(s_0_11)
        let s_0_12: bool = u_get_HSTR_Type_T12(state, tracer, s_0_11);
        // D s_0_13: write-var __HSTR_T12 <= s_0_12
        fn_state.u__HSTR_T12 = s_0_12;
        // C s_0_14: const #20992u : u32
        let s_0_14: u32 = 20992;
        // D s_0_15: read-reg s_0_14:struct
        let s_0_15: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_14 as isize);
            tracer.read_register(s_0_14 as isize, value);
            value
        };
        // D s_0_16: call _get_ICH_HCR_EL2_Type_TC(s_0_15)
        let s_0_16: bool = u_get_ICH_HCR_EL2_Type_TC(state, tracer, s_0_15);
        // D s_0_17: write-var __ICH_HCR_EL2_TC <= s_0_16
        fn_state.u__ICH_HCR_EL2_TC = s_0_16;
        // C s_0_18: const #() : ()
        let s_0_18: () = ();
        // S s_0_19: call ICH_HCR_read(s_0_18)
        let s_0_19: ProductType700c18a878c5601b = ICH_HCR_read(state, tracer, s_0_18);
        // S s_0_20: call _get_ICH_HCR_Type_TC(s_0_19)
        let s_0_20: bool = u_get_ICH_HCR_Type_TC(state, tracer, s_0_19);
        // D s_0_21: write-var __ICH_HCR_TC <= s_0_20
        fn_state.u__ICH_HCR_TC = s_0_20;
        // C s_0_22: const #102552u : u32
        let s_0_22: u32 = 102552;
        // D s_0_23: read-reg s_0_22:struct
        let s_0_23: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_22 as isize);
            tracer.read_register(s_0_22 as isize, value);
            value
        };
        // D s_0_24: call _get_HCR_EL2_Type_FMO(s_0_23)
        let s_0_24: bool = u_get_HCR_EL2_Type_FMO(state, tracer, s_0_23);
        // D s_0_25: write-var __HCR_EL2_FMO <= s_0_24
        fn_state.u__HCR_EL2_FMO = s_0_24;
        // C s_0_26: const #102552u : u32
        let s_0_26: u32 = 102552;
        // D s_0_27: read-reg s_0_26:struct
        let s_0_27: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_26 as isize);
            tracer.read_register(s_0_26 as isize, value);
            value
        };
        // D s_0_28: call _get_HCR_EL2_Type_IMO(s_0_27)
        let s_0_28: bool = u_get_HCR_EL2_Type_IMO(state, tracer, s_0_27);
        // D s_0_29: write-var __HCR_EL2_IMO <= s_0_28
        fn_state.u__HCR_EL2_IMO = s_0_28;
        // C s_0_30: const #() : ()
        let s_0_30: () = ();
        // S s_0_31: call HCR_read(s_0_30)
        let s_0_31: ProductType700c18a878c5601b = HCR_read(state, tracer, s_0_30);
        // S s_0_32: call _get_HCR_Type_FMO(s_0_31)
        let s_0_32: bool = u_get_HCR_Type_FMO(state, tracer, s_0_31);
        // D s_0_33: write-var __HCR_FMO <= s_0_32
        fn_state.u__HCR_FMO = s_0_32;
        // C s_0_34: const #() : ()
        let s_0_34: () = ();
        // S s_0_35: call HCR_read(s_0_34)
        let s_0_35: ProductType700c18a878c5601b = HCR_read(state, tracer, s_0_34);
        // S s_0_36: call _get_HCR_Type_IMO(s_0_35)
        let s_0_36: bool = u_get_HCR_Type_IMO(state, tracer, s_0_35);
        // D s_0_37: write-var __HCR_IMO <= s_0_36
        fn_state.u__HCR_IMO = s_0_36;
        // C s_0_38: const #() : ()
        let s_0_38: () = ();
        // S s_0_39: call ICC_HSRE_read(s_0_38)
        let s_0_39: ProductType700c18a878c5601b = ICC_HSRE_read(state, tracer, s_0_38);
        // S s_0_40: call _get_ICC_HSRE_Type_SRE(s_0_39)
        let s_0_40: bool = u_get_ICC_HSRE_Type_SRE(state, tracer, s_0_39);
        // D s_0_41: write-var __ICC_HSRE_SRE <= s_0_40
        fn_state.u__ICC_HSRE_SRE = s_0_40;
        // C s_0_42: const #19992u : u32
        let s_0_42: u32 = 19992;
        // D s_0_43: read-reg s_0_42:struct
        let s_0_43: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_42 as isize);
            tracer.read_register(s_0_42 as isize, value);
            value
        };
        // D s_0_44: call _get_ICC_MSRE_Type_SRE(s_0_43)
        let s_0_44: bool = u_get_ICC_MSRE_Type_SRE(state, tracer, s_0_43);
        // D s_0_45: write-var __ICC_MSRE_SRE <= s_0_44
        fn_state.u__ICC_MSRE_SRE = s_0_44;
        // D s_0_46: read-var __PSTATE_EL:u8
        let s_0_46: u8 = fn_state.u__PSTATE_EL;
        // D s_0_47: cast zx s_0_46 -> bv
        let s_0_47: Bits = Bits::new(s_0_46 as u128, 2u16);
        // C s_0_48: const #448u : u32
        let s_0_48: u32 = 448;
        // D s_0_49: read-reg s_0_48:u8
        let s_0_49: u8 = {
            let value = state.read_register::<u8>(s_0_48 as isize);
            tracer.read_register(s_0_48 as isize, value);
            value
        };
        // D s_0_50: cast zx s_0_49 -> bv
        let s_0_50: Bits = Bits::new(s_0_49 as u128, 2u16);
        // D s_0_51: cmp-eq s_0_47 s_0_50
        let s_0_51: bool = ((s_0_47) == (s_0_50));
        // N s_0_52: branch s_0_51 b202 b1
        if s_0_51 {
            return block_202(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b71 b2
        if s_1_5 {
            return block_71(state, tracer, fn_state);
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
        // D s_5_0: read-var __ICC_MSRE_SRE:u8
        let s_5_0: bool = fn_state.u__ICC_MSRE_SRE;
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
        // S s_6_1: call ICC_RPR_read(s_6_0)
        let s_6_1: ProductType700c18a878c5601b = ICC_RPR_read(state, tracer, s_6_0);
        // S s_6_2: call __get_ICC_RPR(s_6_1)
        let s_6_2: ProductType700c18a878c5601b = u__get_ICC_RPR(state, tracer, s_6_1);
        // D s_6_3: write-var ga#183319 <= s_6_2
        fn_state.ga_183319 = s_6_2;
        // D s_6_4: read-var ga#183319.0:struct
        let s_6_4: u32 = fn_state.ga_183319._0;
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
        // N s_7_0: panic
        panic!("{:?}", ());
        // N s_7_1: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call Halted(s_8_0)
        let s_8_1: bool = Halted(state, tracer, s_8_0);
        // N s_8_2: branch s_8_1 b70 b9
        if s_8_1 {
            return block_70(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#112340 <= s_9_0
        fn_state.gs_112340 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#112340:u8
        let s_10_0: bool = fn_state.gs_112340;
        // N s_10_1: branch s_10_0 b69 b11
        if s_10_0 {
            return block_69(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#112341 <= s_11_0
        fn_state.gs_112341 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#112341:u8
        let s_12_0: bool = fn_state.gs_112341;
        // N s_12_1: branch s_12_0 b68 b13
        if s_12_0 {
            return block_68(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#112342 <= s_13_0
        fn_state.gs_112342 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#112342:u8
        let s_14_0: bool = fn_state.gs_112342;
        // N s_14_1: branch s_14_0 b67 b15
        if s_14_0 {
            return block_67(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#112343 <= s_15_0
        fn_state.gs_112343 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#112343:u8
        let s_16_0: bool = fn_state.gs_112343;
        // N s_16_1: branch s_16_0 b66 b17
        if s_16_0 {
            return block_66(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#112344 <= s_17_0
        fn_state.gs_112344 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#112344:u8
        let s_18_0: bool = fn_state.gs_112344;
        // N s_18_1: branch s_18_0 b65 b19
        if s_18_0 {
            return block_65(state, tracer, fn_state);
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
        // S s_19_1: call Halted(s_19_0)
        let s_19_1: bool = Halted(state, tracer, s_19_0);
        // N s_19_2: branch s_19_1 b64 b20
        if s_19_1 {
            return block_64(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#112345 <= s_20_0
        fn_state.gs_112345 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#112345:u8
        let s_21_0: bool = fn_state.gs_112345;
        // N s_21_1: branch s_21_0 b63 b22
        if s_21_0 {
            return block_63(state, tracer, fn_state);
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
        // D s_22_1: write-var gs#112346 <= s_22_0
        fn_state.gs_112346 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#112346:u8
        let s_23_0: bool = fn_state.gs_112346;
        // N s_23_1: branch s_23_0 b62 b24
        if s_23_0 {
            return block_62(state, tracer, fn_state);
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
        // D s_24_1: write-var gs#112347 <= s_24_0
        fn_state.gs_112347 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#112347:u8
        let s_25_0: bool = fn_state.gs_112347;
        // N s_25_1: branch s_25_0 b61 b26
        if s_25_0 {
            return block_61(state, tracer, fn_state);
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
        // D s_26_1: write-var gs#112348 <= s_26_0
        fn_state.gs_112348 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#112348:u8
        let s_27_0: bool = fn_state.gs_112348;
        // N s_27_1: branch s_27_0 b60 b28
        if s_27_0 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #0u : u8
        let s_28_0: bool = false;
        // D s_28_1: write-var gs#112349 <= s_28_0
        fn_state.gs_112349 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#112349:u8
        let s_29_0: bool = fn_state.gs_112349;
        // N s_29_1: branch s_29_0 b59 b30
        if s_29_0 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var __ICC_HSRE_SRE:u8
        let s_30_0: bool = fn_state.u__ICC_HSRE_SRE;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 1u16);
        // C s_30_2: const #0u : u8
        let s_30_2: bool = false;
        // C s_30_3: cast zx s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 1u16);
        // D s_30_4: cmp-eq s_30_1 s_30_3
        let s_30_4: bool = ((s_30_1) == (s_30_3));
        // N s_30_5: branch s_30_4 b58 b31
        if s_30_4 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
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
        // C s_31_2: const #2u : u8
        let s_31_2: u8 = 2;
        // D s_31_3: cmp-lt s_31_1 s_31_2
        let s_31_3: bool = ((s_31_1) < (s_31_2));
        // N s_31_4: branch s_31_3 b57 b32
        if s_31_3 {
            return block_57(state, tracer, fn_state);
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
        // D s_32_1: write-var gs#112350 <= s_32_0
        fn_state.gs_112350 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#112350:u8
        let s_33_0: bool = fn_state.gs_112350;
        // N s_33_1: branch s_33_0 b56 b34
        if s_33_0 {
            return block_56(state, tracer, fn_state);
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
        // D s_34_1: write-var gs#112351 <= s_34_0
        fn_state.gs_112351 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#112351:u8
        let s_35_0: bool = fn_state.gs_112351;
        // N s_35_1: branch s_35_0 b50 b36
        if s_35_0 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
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
        // N s_36_4: branch s_36_3 b49 b37
        if s_36_3 {
            return block_49(state, tracer, fn_state);
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
        // D s_37_1: write-var gs#112352 <= s_37_0
        fn_state.gs_112352 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#112352:u8
        let s_38_0: bool = fn_state.gs_112352;
        // N s_38_1: branch s_38_0 b48 b39
        if s_38_0 {
            return block_48(state, tracer, fn_state);
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
        // D s_39_1: write-var gs#112353 <= s_39_0
        fn_state.gs_112353 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#112353:u8
        let s_40_0: bool = fn_state.gs_112353;
        // N s_40_1: branch s_40_0 b42 b41
        if s_40_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #() : ()
        let s_41_0: () = ();
        // S s_41_1: call ICC_RPR_read(s_41_0)
        let s_41_1: ProductType700c18a878c5601b = ICC_RPR_read(state, tracer, s_41_0);
        // S s_41_2: call __get_ICC_RPR(s_41_1)
        let s_41_2: ProductType700c18a878c5601b = u__get_ICC_RPR(state, tracer, s_41_1);
        // D s_41_3: write-var ga#183313 <= s_41_2
        fn_state.ga_183313 = s_41_2;
        // D s_41_4: read-var ga#183313.0:struct
        let s_41_4: u32 = fn_state.ga_183313._0;
        // D s_41_5: read-var t:i
        let s_41_5: i128 = fn_state.t;
        // D s_41_6: call R_set(s_41_5, s_41_4)
        let s_41_6: () = R_set(state, tracer, s_41_5, s_41_4);
        // N s_41_7: return
        return;
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #() : ()
        let s_42_0: () = ();
        // S s_42_1: call Halted(s_42_0)
        let s_42_1: bool = Halted(state, tracer, s_42_0);
        // N s_42_2: branch s_42_1 b47 b43
        if s_42_1 {
            return block_47(state, tracer, fn_state);
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
        // D s_43_1: write-var gs#112354 <= s_43_0
        fn_state.gs_112354 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#112354:u8
        let s_44_0: bool = fn_state.gs_112354;
        // N s_44_1: branch s_44_0 b46 b45
        if s_44_0 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #() : ()
        let s_45_0: () = ();
        // S s_45_1: call AArch32_TakeMonitorTrapException(s_45_0)
        let s_45_1: () = AArch32_TakeMonitorTrapException(state, tracer, s_45_0);
        // N s_45_2: return
        return;
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_46_0: panic
        panic!("{:?}", ());
        // N s_46_1: return
        return;
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #() : ()
        let s_47_0: () = ();
        // S s_47_1: call EDSCR_read(s_47_0)
        let s_47_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_47_0);
        // S s_47_2: call _get_EDSCR_Type_SDD(s_47_1)
        let s_47_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_47_1);
        // S s_47_3: cast zx s_47_2 -> bv
        let s_47_3: Bits = Bits::new(s_47_2 as u128, 1u16);
        // C s_47_4: const #1u : u8
        let s_47_4: bool = true;
        // C s_47_5: cast zx s_47_4 -> bv
        let s_47_5: Bits = Bits::new(s_47_4 as u128, 1u16);
        // S s_47_6: cmp-eq s_47_3 s_47_5
        let s_47_6: bool = ((s_47_3) == (s_47_5));
        // D s_47_7: write-var gs#112354 <= s_47_6
        fn_state.gs_112354 = s_47_6;
        // N s_47_8: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #20920u : u32
        let s_48_0: u32 = 20920;
        // D s_48_1: read-reg s_48_0:struct
        let s_48_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_48_0 as isize);
            tracer.read_register(s_48_0 as isize, value);
            value
        };
        // D s_48_2: call _get_SCR_Type_IRQ(s_48_1)
        let s_48_2: bool = u_get_SCR_Type_IRQ(state, tracer, s_48_1);
        // C s_48_3: const #20920u : u32
        let s_48_3: u32 = 20920;
        // D s_48_4: read-reg s_48_3:struct
        let s_48_4: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_48_3 as isize);
            tracer.read_register(s_48_3 as isize, value);
            value
        };
        // D s_48_5: call _get_SCR_Type_FIQ(s_48_4)
        let s_48_5: bool = u_get_SCR_Type_FIQ(state, tracer, s_48_4);
        // D s_48_6: cast zx s_48_2 -> bv
        let s_48_6: Bits = Bits::new(s_48_2 as u128, 1u16);
        // D s_48_7: cast zx s_48_5 -> bv
        let s_48_7: Bits = Bits::new(s_48_5 as u128, 1u16);
        // D s_48_8: cast reint s_48_6 -> u128
        let s_48_8: u128 = (s_48_6.value() as u128);
        // D s_48_9: size-of s_48_6
        let s_48_9: u16 = s_48_6.length();
        // D s_48_10: cast reint s_48_7 -> u128
        let s_48_10: u128 = (s_48_7.value() as u128);
        // D s_48_11: size-of s_48_7
        let s_48_11: u16 = s_48_7.length();
        // D s_48_12: lsl s_48_8 s_48_11
        let s_48_12: u128 = s_48_8 << s_48_11;
        // D s_48_13: or s_48_12 s_48_10
        let s_48_13: u128 = ((s_48_12) | (s_48_10));
        // D s_48_14: add s_48_9 s_48_11
        let s_48_14: u16 = (s_48_9 + s_48_11);
        // D s_48_15: create-bits s_48_13 s_48_14
        let s_48_15: Bits = Bits::new(s_48_13, s_48_14);
        // D s_48_16: cast reint s_48_15 -> u8
        let s_48_16: u8 = (s_48_15.value() as u8);
        // D s_48_17: cast zx s_48_16 -> bv
        let s_48_17: Bits = Bits::new(s_48_16 as u128, 2u16);
        // C s_48_18: const #3u : u8
        let s_48_18: u8 = 3;
        // C s_48_19: cast zx s_48_18 -> bv
        let s_48_19: Bits = Bits::new(s_48_18 as u128, 2u16);
        // D s_48_20: cmp-eq s_48_17 s_48_19
        let s_48_20: bool = ((s_48_17) == (s_48_19));
        // D s_48_21: write-var gs#112353 <= s_48_20
        fn_state.gs_112353 = s_48_20;
        // N s_48_22: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #424u : u32
        let s_49_0: u32 = 424;
        // D s_49_1: read-reg s_49_0:u8
        let s_49_1: u8 = {
            let value = state.read_register::<u8>(s_49_0 as isize);
            tracer.read_register(s_49_0 as isize, value);
            value
        };
        // D s_49_2: call ELUsingAArch32(s_49_1)
        let s_49_2: bool = ELUsingAArch32(state, tracer, s_49_1);
        // D s_49_3: write-var gs#112352 <= s_49_2
        fn_state.gs_112352 = s_49_2;
        // N s_49_4: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #() : ()
        let s_50_0: () = ();
        // S s_50_1: call Halted(s_50_0)
        let s_50_1: bool = Halted(state, tracer, s_50_0);
        // N s_50_2: branch s_50_1 b55 b51
        if s_50_1 {
            return block_55(state, tracer, fn_state);
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
        // D s_51_1: write-var gs#112355 <= s_51_0
        fn_state.gs_112355 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#112355:u8
        let s_52_0: bool = fn_state.gs_112355;
        // N s_52_1: branch s_52_0 b54 b53
        if s_52_0 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #3u : u8
        let s_53_0: u8 = 3;
        // C s_53_1: cast zx s_53_0 -> bv
        let s_53_1: Bits = Bits::new(s_53_0 as u128, 8u16);
        // C s_53_2: cast zx s_53_1 -> i
        let s_53_2: i128 = (s_53_1.value() as i128);
        // C s_53_3: cast reint s_53_2 -> i64
        let s_53_3: i64 = (s_53_2 as i64);
        // C s_53_4: cast zx s_53_3 -> i
        let s_53_4: i128 = (i128::try_from(s_53_3).unwrap());
        // C s_53_5: const #424u : u32
        let s_53_5: u32 = 424;
        // D s_53_6: read-reg s_53_5:u8
        let s_53_6: u8 = {
            let value = state.read_register::<u8>(s_53_5 as isize);
            tracer.read_register(s_53_5 as isize, value);
            value
        };
        // D s_53_7: call AArch64_AArch32SystemAccessTrap(s_53_6, s_53_4)
        let s_53_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_53_6, s_53_4);
        // N s_53_8: return
        return;
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_54_0: panic
        panic!("{:?}", ());
        // N s_54_1: return
        return;
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #() : ()
        let s_55_0: () = ();
        // S s_55_1: call EDSCR_read(s_55_0)
        let s_55_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_55_0);
        // S s_55_2: call _get_EDSCR_Type_SDD(s_55_1)
        let s_55_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_55_1);
        // S s_55_3: cast zx s_55_2 -> bv
        let s_55_3: Bits = Bits::new(s_55_2 as u128, 1u16);
        // C s_55_4: const #1u : u8
        let s_55_4: bool = true;
        // C s_55_5: cast zx s_55_4 -> bv
        let s_55_5: Bits = Bits::new(s_55_4 as u128, 1u16);
        // S s_55_6: cmp-eq s_55_3 s_55_5
        let s_55_6: bool = ((s_55_3) == (s_55_5));
        // D s_55_7: write-var gs#112355 <= s_55_6
        fn_state.gs_112355 = s_55_6;
        // N s_55_8: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #90704u : u32
        let s_56_0: u32 = 90704;
        // D s_56_1: read-reg s_56_0:struct
        let s_56_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_56_0 as isize);
            tracer.read_register(s_56_0 as isize, value);
            value
        };
        // D s_56_2: call _get_SCR_EL3_Type_IRQ(s_56_1)
        let s_56_2: bool = u_get_SCR_EL3_Type_IRQ(state, tracer, s_56_1);
        // C s_56_3: const #90704u : u32
        let s_56_3: u32 = 90704;
        // D s_56_4: read-reg s_56_3:struct
        let s_56_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_56_3 as isize);
            tracer.read_register(s_56_3 as isize, value);
            value
        };
        // D s_56_5: call _get_SCR_EL3_Type_FIQ(s_56_4)
        let s_56_5: bool = u_get_SCR_EL3_Type_FIQ(state, tracer, s_56_4);
        // D s_56_6: cast zx s_56_2 -> bv
        let s_56_6: Bits = Bits::new(s_56_2 as u128, 1u16);
        // D s_56_7: cast zx s_56_5 -> bv
        let s_56_7: Bits = Bits::new(s_56_5 as u128, 1u16);
        // D s_56_8: cast reint s_56_6 -> u128
        let s_56_8: u128 = (s_56_6.value() as u128);
        // D s_56_9: size-of s_56_6
        let s_56_9: u16 = s_56_6.length();
        // D s_56_10: cast reint s_56_7 -> u128
        let s_56_10: u128 = (s_56_7.value() as u128);
        // D s_56_11: size-of s_56_7
        let s_56_11: u16 = s_56_7.length();
        // D s_56_12: lsl s_56_8 s_56_11
        let s_56_12: u128 = s_56_8 << s_56_11;
        // D s_56_13: or s_56_12 s_56_10
        let s_56_13: u128 = ((s_56_12) | (s_56_10));
        // D s_56_14: add s_56_9 s_56_11
        let s_56_14: u16 = (s_56_9 + s_56_11);
        // D s_56_15: create-bits s_56_13 s_56_14
        let s_56_15: Bits = Bits::new(s_56_13, s_56_14);
        // D s_56_16: cast reint s_56_15 -> u8
        let s_56_16: u8 = (s_56_15.value() as u8);
        // D s_56_17: cast zx s_56_16 -> bv
        let s_56_17: Bits = Bits::new(s_56_16 as u128, 2u16);
        // C s_56_18: const #3u : u8
        let s_56_18: u8 = 3;
        // C s_56_19: cast zx s_56_18 -> bv
        let s_56_19: Bits = Bits::new(s_56_18 as u128, 2u16);
        // D s_56_20: cmp-eq s_56_17 s_56_19
        let s_56_20: bool = ((s_56_17) == (s_56_19));
        // D s_56_21: write-var gs#112351 <= s_56_20
        fn_state.gs_112351 = s_56_20;
        // N s_56_22: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #424u : u32
        let s_57_0: u32 = 424;
        // D s_57_1: read-reg s_57_0:u8
        let s_57_1: u8 = {
            let value = state.read_register::<u8>(s_57_0 as isize);
            tracer.read_register(s_57_0 as isize, value);
            value
        };
        // D s_57_2: call ELUsingAArch32(s_57_1)
        let s_57_2: bool = ELUsingAArch32(state, tracer, s_57_1);
        // D s_57_3: not s_57_2
        let s_57_3: bool = !s_57_2;
        // D s_57_4: write-var gs#112350 <= s_57_3
        fn_state.gs_112350 = s_57_3;
        // N s_57_5: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_58_0: panic
        panic!("{:?}", ());
        // N s_58_1: return
        return;
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_59_0: panic
        panic!("{:?}", ());
        // N s_59_1: return
        return;
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #20920u : u32
        let s_60_0: u32 = 20920;
        // D s_60_1: read-reg s_60_0:struct
        let s_60_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_60_0 as isize);
            tracer.read_register(s_60_0 as isize, value);
            value
        };
        // D s_60_2: call _get_SCR_Type_IRQ(s_60_1)
        let s_60_2: bool = u_get_SCR_Type_IRQ(state, tracer, s_60_1);
        // C s_60_3: const #20920u : u32
        let s_60_3: u32 = 20920;
        // D s_60_4: read-reg s_60_3:struct
        let s_60_4: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_60_3 as isize);
            tracer.read_register(s_60_3 as isize, value);
            value
        };
        // D s_60_5: call _get_SCR_Type_FIQ(s_60_4)
        let s_60_5: bool = u_get_SCR_Type_FIQ(state, tracer, s_60_4);
        // D s_60_6: cast zx s_60_2 -> bv
        let s_60_6: Bits = Bits::new(s_60_2 as u128, 1u16);
        // D s_60_7: cast zx s_60_5 -> bv
        let s_60_7: Bits = Bits::new(s_60_5 as u128, 1u16);
        // D s_60_8: cast reint s_60_6 -> u128
        let s_60_8: u128 = (s_60_6.value() as u128);
        // D s_60_9: size-of s_60_6
        let s_60_9: u16 = s_60_6.length();
        // D s_60_10: cast reint s_60_7 -> u128
        let s_60_10: u128 = (s_60_7.value() as u128);
        // D s_60_11: size-of s_60_7
        let s_60_11: u16 = s_60_7.length();
        // D s_60_12: lsl s_60_8 s_60_11
        let s_60_12: u128 = s_60_8 << s_60_11;
        // D s_60_13: or s_60_12 s_60_10
        let s_60_13: u128 = ((s_60_12) | (s_60_10));
        // D s_60_14: add s_60_9 s_60_11
        let s_60_14: u16 = (s_60_9 + s_60_11);
        // D s_60_15: create-bits s_60_13 s_60_14
        let s_60_15: Bits = Bits::new(s_60_13, s_60_14);
        // D s_60_16: cast reint s_60_15 -> u8
        let s_60_16: u8 = (s_60_15.value() as u8);
        // D s_60_17: cast zx s_60_16 -> bv
        let s_60_17: Bits = Bits::new(s_60_16 as u128, 2u16);
        // C s_60_18: const #3u : u8
        let s_60_18: u8 = 3;
        // C s_60_19: cast zx s_60_18 -> bv
        let s_60_19: Bits = Bits::new(s_60_18 as u128, 2u16);
        // D s_60_20: cmp-eq s_60_17 s_60_19
        let s_60_20: bool = ((s_60_17) == (s_60_19));
        // D s_60_21: write-var gs#112349 <= s_60_20
        fn_state.gs_112349 = s_60_20;
        // N s_60_22: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #424u : u32
        let s_61_0: u32 = 424;
        // D s_61_1: read-reg s_61_0:u8
        let s_61_1: u8 = {
            let value = state.read_register::<u8>(s_61_0 as isize);
            tracer.read_register(s_61_0 as isize, value);
            value
        };
        // D s_61_2: call ELUsingAArch32(s_61_1)
        let s_61_2: bool = ELUsingAArch32(state, tracer, s_61_1);
        // D s_61_3: write-var gs#112348 <= s_61_2
        fn_state.gs_112348 = s_61_2;
        // N s_61_4: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_62_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_62_1: call __IMPDEF_boolean(s_62_0)
        let s_62_1: bool = u__IMPDEF_boolean(state, tracer, s_62_0);
        // D s_62_2: write-var gs#112347 <= s_62_1
        fn_state.gs_112347 = s_62_1;
        // N s_62_3: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #() : ()
        let s_63_0: () = ();
        // S s_63_1: call EDSCR_read(s_63_0)
        let s_63_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_63_0);
        // S s_63_2: call _get_EDSCR_Type_SDD(s_63_1)
        let s_63_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_63_1);
        // S s_63_3: cast zx s_63_2 -> bv
        let s_63_3: Bits = Bits::new(s_63_2 as u128, 1u16);
        // C s_63_4: const #1u : u8
        let s_63_4: bool = true;
        // C s_63_5: cast zx s_63_4 -> bv
        let s_63_5: Bits = Bits::new(s_63_4 as u128, 1u16);
        // S s_63_6: cmp-eq s_63_3 s_63_5
        let s_63_6: bool = ((s_63_3) == (s_63_5));
        // D s_63_7: write-var gs#112346 <= s_63_6
        fn_state.gs_112346 = s_63_6;
        // N s_63_8: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #424u : u32
        let s_64_0: u32 = 424;
        // D s_64_1: read-reg s_64_0:u8
        let s_64_1: u8 = {
            let value = state.read_register::<u8>(s_64_0 as isize);
            tracer.read_register(s_64_0 as isize, value);
            value
        };
        // C s_64_2: const #2u : u8
        let s_64_2: u8 = 2;
        // D s_64_3: cmp-lt s_64_1 s_64_2
        let s_64_3: bool = ((s_64_1) < (s_64_2));
        // D s_64_4: write-var gs#112345 <= s_64_3
        fn_state.gs_112345 = s_64_3;
        // N s_64_5: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_65_0: panic
        panic!("{:?}", ());
        // N s_65_1: return
        return;
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #90704u : u32
        let s_66_0: u32 = 90704;
        // D s_66_1: read-reg s_66_0:struct
        let s_66_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_66_0 as isize);
            tracer.read_register(s_66_0 as isize, value);
            value
        };
        // D s_66_2: call _get_SCR_EL3_Type_IRQ(s_66_1)
        let s_66_2: bool = u_get_SCR_EL3_Type_IRQ(state, tracer, s_66_1);
        // C s_66_3: const #90704u : u32
        let s_66_3: u32 = 90704;
        // D s_66_4: read-reg s_66_3:struct
        let s_66_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_66_3 as isize);
            tracer.read_register(s_66_3 as isize, value);
            value
        };
        // D s_66_5: call _get_SCR_EL3_Type_FIQ(s_66_4)
        let s_66_5: bool = u_get_SCR_EL3_Type_FIQ(state, tracer, s_66_4);
        // D s_66_6: cast zx s_66_2 -> bv
        let s_66_6: Bits = Bits::new(s_66_2 as u128, 1u16);
        // D s_66_7: cast zx s_66_5 -> bv
        let s_66_7: Bits = Bits::new(s_66_5 as u128, 1u16);
        // D s_66_8: cast reint s_66_6 -> u128
        let s_66_8: u128 = (s_66_6.value() as u128);
        // D s_66_9: size-of s_66_6
        let s_66_9: u16 = s_66_6.length();
        // D s_66_10: cast reint s_66_7 -> u128
        let s_66_10: u128 = (s_66_7.value() as u128);
        // D s_66_11: size-of s_66_7
        let s_66_11: u16 = s_66_7.length();
        // D s_66_12: lsl s_66_8 s_66_11
        let s_66_12: u128 = s_66_8 << s_66_11;
        // D s_66_13: or s_66_12 s_66_10
        let s_66_13: u128 = ((s_66_12) | (s_66_10));
        // D s_66_14: add s_66_9 s_66_11
        let s_66_14: u16 = (s_66_9 + s_66_11);
        // D s_66_15: create-bits s_66_13 s_66_14
        let s_66_15: Bits = Bits::new(s_66_13, s_66_14);
        // D s_66_16: cast reint s_66_15 -> u8
        let s_66_16: u8 = (s_66_15.value() as u8);
        // D s_66_17: cast zx s_66_16 -> bv
        let s_66_17: Bits = Bits::new(s_66_16 as u128, 2u16);
        // C s_66_18: const #3u : u8
        let s_66_18: u8 = 3;
        // C s_66_19: cast zx s_66_18 -> bv
        let s_66_19: Bits = Bits::new(s_66_18 as u128, 2u16);
        // D s_66_20: cmp-eq s_66_17 s_66_19
        let s_66_20: bool = ((s_66_17) == (s_66_19));
        // D s_66_21: write-var gs#112344 <= s_66_20
        fn_state.gs_112344 = s_66_20;
        // N s_66_22: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #424u : u32
        let s_67_0: u32 = 424;
        // D s_67_1: read-reg s_67_0:u8
        let s_67_1: u8 = {
            let value = state.read_register::<u8>(s_67_0 as isize);
            tracer.read_register(s_67_0 as isize, value);
            value
        };
        // D s_67_2: call ELUsingAArch32(s_67_1)
        let s_67_2: bool = ELUsingAArch32(state, tracer, s_67_1);
        // D s_67_3: not s_67_2
        let s_67_3: bool = !s_67_2;
        // D s_67_4: write-var gs#112343 <= s_67_3
        fn_state.gs_112343 = s_67_3;
        // N s_67_5: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_68_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_68_1: call __IMPDEF_boolean(s_68_0)
        let s_68_1: bool = u__IMPDEF_boolean(state, tracer, s_68_0);
        // D s_68_2: write-var gs#112342 <= s_68_1
        fn_state.gs_112342 = s_68_1;
        // N s_68_3: jump b14
        return block_14(state, tracer, fn_state);
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
        // D s_69_7: write-var gs#112341 <= s_69_6
        fn_state.gs_112341 = s_69_6;
        // N s_69_8: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #424u : u32
        let s_70_0: u32 = 424;
        // D s_70_1: read-reg s_70_0:u8
        let s_70_1: u8 = {
            let value = state.read_register::<u8>(s_70_0 as isize);
            tracer.read_register(s_70_0 as isize, value);
            value
        };
        // C s_70_2: const #2u : u8
        let s_70_2: u8 = 2;
        // D s_70_3: cmp-lt s_70_1 s_70_2
        let s_70_3: bool = ((s_70_1) < (s_70_2));
        // D s_70_4: write-var gs#112340 <= s_70_3
        fn_state.gs_112340 = s_70_3;
        // N s_70_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #() : ()
        let s_71_0: () = ();
        // S s_71_1: call Halted(s_71_0)
        let s_71_1: bool = Halted(state, tracer, s_71_0);
        // N s_71_2: branch s_71_1 b201 b72
        if s_71_1 {
            return block_201(state, tracer, fn_state);
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
        // D s_72_1: write-var gs#112356 <= s_72_0
        fn_state.gs_112356 = s_72_0;
        // N s_72_2: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var gs#112356:u8
        let s_73_0: bool = fn_state.gs_112356;
        // N s_73_1: branch s_73_0 b200 b74
        if s_73_0 {
            return block_200(state, tracer, fn_state);
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
        // D s_74_1: write-var gs#112357 <= s_74_0
        fn_state.gs_112357 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var gs#112357:u8
        let s_75_0: bool = fn_state.gs_112357;
        // N s_75_1: branch s_75_0 b199 b76
        if s_75_0 {
            return block_199(state, tracer, fn_state);
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
        // D s_76_1: write-var gs#112358 <= s_76_0
        fn_state.gs_112358 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var gs#112358:u8
        let s_77_0: bool = fn_state.gs_112358;
        // N s_77_1: branch s_77_0 b198 b78
        if s_77_0 {
            return block_198(state, tracer, fn_state);
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
        // D s_78_1: write-var gs#112359 <= s_78_0
        fn_state.gs_112359 = s_78_0;
        // N s_78_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var gs#112359:u8
        let s_79_0: bool = fn_state.gs_112359;
        // N s_79_1: branch s_79_0 b197 b80
        if s_79_0 {
            return block_197(state, tracer, fn_state);
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
        // D s_80_1: write-var gs#112360 <= s_80_0
        fn_state.gs_112360 = s_80_0;
        // N s_80_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#112360:u8
        let s_81_0: bool = fn_state.gs_112360;
        // N s_81_1: branch s_81_0 b196 b82
        if s_81_0 {
            return block_196(state, tracer, fn_state);
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
        // S s_82_1: call Halted(s_82_0)
        let s_82_1: bool = Halted(state, tracer, s_82_0);
        // N s_82_2: branch s_82_1 b195 b83
        if s_82_1 {
            return block_195(state, tracer, fn_state);
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
        // D s_83_1: write-var gs#112361 <= s_83_0
        fn_state.gs_112361 = s_83_0;
        // N s_83_2: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var gs#112361:u8
        let s_84_0: bool = fn_state.gs_112361;
        // N s_84_1: branch s_84_0 b194 b85
        if s_84_0 {
            return block_194(state, tracer, fn_state);
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
        // D s_85_1: write-var gs#112362 <= s_85_0
        fn_state.gs_112362 = s_85_0;
        // N s_85_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var gs#112362:u8
        let s_86_0: bool = fn_state.gs_112362;
        // N s_86_1: branch s_86_0 b193 b87
        if s_86_0 {
            return block_193(state, tracer, fn_state);
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
        // D s_87_1: write-var gs#112363 <= s_87_0
        fn_state.gs_112363 = s_87_0;
        // N s_87_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var gs#112363:u8
        let s_88_0: bool = fn_state.gs_112363;
        // N s_88_1: branch s_88_0 b192 b89
        if s_88_0 {
            return block_192(state, tracer, fn_state);
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
        // D s_89_1: write-var gs#112364 <= s_89_0
        fn_state.gs_112364 = s_89_0;
        // N s_89_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var gs#112364:u8
        let s_90_0: bool = fn_state.gs_112364;
        // N s_90_1: branch s_90_0 b191 b91
        if s_90_0 {
            return block_191(state, tracer, fn_state);
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
        // D s_91_1: write-var gs#112365 <= s_91_0
        fn_state.gs_112365 = s_91_0;
        // N s_91_2: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var gs#112365:u8
        let s_92_0: bool = fn_state.gs_112365;
        // N s_92_1: branch s_92_0 b190 b93
        if s_92_0 {
            return block_190(state, tracer, fn_state);
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
        // D s_93_1: write-var gs#112366 <= s_93_0
        fn_state.gs_112366 = s_93_0;
        // N s_93_2: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var gs#112366:u8
        let s_94_0: bool = fn_state.gs_112366;
        // N s_94_1: branch s_94_0 b189 b95
        if s_94_0 {
            return block_189(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #() : ()
        let s_95_0: () = ();
        // S s_95_1: call EL2Enabled(s_95_0)
        let s_95_1: bool = EL2Enabled(state, tracer, s_95_0);
        // N s_95_2: branch s_95_1 b188 b96
        if s_95_1 {
            return block_188(state, tracer, fn_state);
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
        // D s_96_1: write-var gs#112367 <= s_96_0
        fn_state.gs_112367 = s_96_0;
        // N s_96_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var gs#112367:u8
        let s_97_0: bool = fn_state.gs_112367;
        // N s_97_1: branch s_97_0 b187 b98
        if s_97_0 {
            return block_187(state, tracer, fn_state);
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
        // D s_98_1: write-var gs#112368 <= s_98_0
        fn_state.gs_112368 = s_98_0;
        // N s_98_2: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_99_0: read-var gs#112368:u8
        let s_99_0: bool = fn_state.gs_112368;
        // N s_99_1: branch s_99_0 b186 b100
        if s_99_0 {
            return block_186(state, tracer, fn_state);
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
        // N s_100_2: branch s_100_1 b185 b101
        if s_100_1 {
            return block_185(state, tracer, fn_state);
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
        // D s_101_1: write-var gs#112369 <= s_101_0
        fn_state.gs_112369 = s_101_0;
        // N s_101_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var gs#112369:u8
        let s_102_0: bool = fn_state.gs_112369;
        // N s_102_1: branch s_102_0 b184 b103
        if s_102_0 {
            return block_184(state, tracer, fn_state);
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
        // D s_103_1: write-var gs#112370 <= s_103_0
        fn_state.gs_112370 = s_103_0;
        // N s_103_2: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_104_0: read-var gs#112370:u8
        let s_104_0: bool = fn_state.gs_112370;
        // N s_104_1: branch s_104_0 b183 b105
        if s_104_0 {
            return block_183(state, tracer, fn_state);
        } else {
            return block_105(state, tracer, fn_state);
        };
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #() : ()
        let s_105_0: () = ();
        // S s_105_1: call EL2Enabled(s_105_0)
        let s_105_1: bool = EL2Enabled(state, tracer, s_105_0);
        // N s_105_2: branch s_105_1 b182 b106
        if s_105_1 {
            return block_182(state, tracer, fn_state);
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
        // D s_106_1: write-var gs#112371 <= s_106_0
        fn_state.gs_112371 = s_106_0;
        // N s_106_2: jump b107
        return block_107(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_107_0: read-var gs#112371:u8
        let s_107_0: bool = fn_state.gs_112371;
        // N s_107_1: branch s_107_0 b181 b108
        if s_107_0 {
            return block_181(state, tracer, fn_state);
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
        // D s_108_1: write-var gs#112372 <= s_108_0
        fn_state.gs_112372 = s_108_0;
        // N s_108_2: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_109_0: read-var gs#112372:u8
        let s_109_0: bool = fn_state.gs_112372;
        // N s_109_1: branch s_109_0 b180 b110
        if s_109_0 {
            return block_180(state, tracer, fn_state);
        } else {
            return block_110(state, tracer, fn_state);
        };
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_110_0: const #() : ()
        let s_110_0: () = ();
        // S s_110_1: call EL2Enabled(s_110_0)
        let s_110_1: bool = EL2Enabled(state, tracer, s_110_0);
        // N s_110_2: branch s_110_1 b179 b111
        if s_110_1 {
            return block_179(state, tracer, fn_state);
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
        // D s_111_1: write-var gs#112373 <= s_111_0
        fn_state.gs_112373 = s_111_0;
        // N s_111_2: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_112_0: read-var gs#112373:u8
        let s_112_0: bool = fn_state.gs_112373;
        // N s_112_1: branch s_112_0 b178 b113
        if s_112_0 {
            return block_178(state, tracer, fn_state);
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
        // D s_113_1: write-var gs#112374 <= s_113_0
        fn_state.gs_112374 = s_113_0;
        // N s_113_2: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_114_0: read-var gs#112374:u8
        let s_114_0: bool = fn_state.gs_112374;
        // N s_114_1: branch s_114_0 b177 b115
        if s_114_0 {
            return block_177(state, tracer, fn_state);
        } else {
            return block_115(state, tracer, fn_state);
        };
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #() : ()
        let s_115_0: () = ();
        // S s_115_1: call EL2Enabled(s_115_0)
        let s_115_1: bool = EL2Enabled(state, tracer, s_115_0);
        // N s_115_2: branch s_115_1 b176 b116
        if s_115_1 {
            return block_176(state, tracer, fn_state);
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
        // D s_116_1: write-var gs#112375 <= s_116_0
        fn_state.gs_112375 = s_116_0;
        // N s_116_2: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_117_0: read-var gs#112375:u8
        let s_117_0: bool = fn_state.gs_112375;
        // N s_117_1: branch s_117_0 b175 b118
        if s_117_0 {
            return block_175(state, tracer, fn_state);
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
        // D s_118_1: write-var gs#112376 <= s_118_0
        fn_state.gs_112376 = s_118_0;
        // N s_118_2: jump b119
        return block_119(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_119_0: read-var gs#112376:u8
        let s_119_0: bool = fn_state.gs_112376;
        // N s_119_1: branch s_119_0 b174 b120
        if s_119_0 {
            return block_174(state, tracer, fn_state);
        } else {
            return block_120(state, tracer, fn_state);
        };
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_120_0: const #() : ()
        let s_120_0: () = ();
        // S s_120_1: call EL2Enabled(s_120_0)
        let s_120_1: bool = EL2Enabled(state, tracer, s_120_0);
        // N s_120_2: branch s_120_1 b173 b121
        if s_120_1 {
            return block_173(state, tracer, fn_state);
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
        // D s_121_1: write-var gs#112377 <= s_121_0
        fn_state.gs_112377 = s_121_0;
        // N s_121_2: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_122_0: read-var gs#112377:u8
        let s_122_0: bool = fn_state.gs_112377;
        // N s_122_1: branch s_122_0 b172 b123
        if s_122_0 {
            return block_172(state, tracer, fn_state);
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
        // D s_123_1: write-var gs#112378 <= s_123_0
        fn_state.gs_112378 = s_123_0;
        // N s_123_2: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_124_0: read-var gs#112378:u8
        let s_124_0: bool = fn_state.gs_112378;
        // N s_124_1: branch s_124_0 b171 b125
        if s_124_0 {
            return block_171(state, tracer, fn_state);
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
        // N s_125_2: branch s_125_1 b170 b126
        if s_125_1 {
            return block_170(state, tracer, fn_state);
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
        // D s_126_1: write-var gs#112379 <= s_126_0
        fn_state.gs_112379 = s_126_0;
        // N s_126_2: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_127_0: read-var gs#112379:u8
        let s_127_0: bool = fn_state.gs_112379;
        // N s_127_1: branch s_127_0 b169 b128
        if s_127_0 {
            return block_169(state, tracer, fn_state);
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
        // D s_128_1: write-var gs#112380 <= s_128_0
        fn_state.gs_112380 = s_128_0;
        // N s_128_2: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_129_0: read-var gs#112380:u8
        let s_129_0: bool = fn_state.gs_112380;
        // N s_129_1: branch s_129_0 b168 b130
        if s_129_0 {
            return block_168(state, tracer, fn_state);
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
        // N s_130_2: branch s_130_1 b167 b131
        if s_130_1 {
            return block_167(state, tracer, fn_state);
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
        // D s_131_1: write-var gs#112381 <= s_131_0
        fn_state.gs_112381 = s_131_0;
        // N s_131_2: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_132_0: read-var gs#112381:u8
        let s_132_0: bool = fn_state.gs_112381;
        // N s_132_1: branch s_132_0 b166 b133
        if s_132_0 {
            return block_166(state, tracer, fn_state);
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
        // D s_133_1: write-var gs#112382 <= s_133_0
        fn_state.gs_112382 = s_133_0;
        // N s_133_2: jump b134
        return block_134(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_134_0: read-var gs#112382:u8
        let s_134_0: bool = fn_state.gs_112382;
        // N s_134_1: branch s_134_0 b165 b135
        if s_134_0 {
            return block_165(state, tracer, fn_state);
        } else {
            return block_135(state, tracer, fn_state);
        };
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
        // C s_135_2: const #2u : u8
        let s_135_2: u8 = 2;
        // D s_135_3: cmp-lt s_135_1 s_135_2
        let s_135_3: bool = ((s_135_1) < (s_135_2));
        // N s_135_4: branch s_135_3 b164 b136
        if s_135_3 {
            return block_164(state, tracer, fn_state);
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
        // D s_136_1: write-var gs#112383 <= s_136_0
        fn_state.gs_112383 = s_136_0;
        // N s_136_2: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_137_0: read-var gs#112383:u8
        let s_137_0: bool = fn_state.gs_112383;
        // N s_137_1: branch s_137_0 b163 b138
        if s_137_0 {
            return block_163(state, tracer, fn_state);
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
        // D s_138_1: write-var gs#112384 <= s_138_0
        fn_state.gs_112384 = s_138_0;
        // N s_138_2: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_139_0: read-var gs#112384:u8
        let s_139_0: bool = fn_state.gs_112384;
        // N s_139_1: branch s_139_0 b157 b140
        if s_139_0 {
            return block_157(state, tracer, fn_state);
        } else {
            return block_140(state, tracer, fn_state);
        };
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_140_0: const #424u : u32
        let s_140_0: u32 = 424;
        // D s_140_1: read-reg s_140_0:u8
        let s_140_1: u8 = {
            let value = state.read_register::<u8>(s_140_0 as isize);
            tracer.read_register(s_140_0 as isize, value);
            value
        };
        // C s_140_2: const #2u : u8
        let s_140_2: u8 = 2;
        // D s_140_3: cmp-lt s_140_1 s_140_2
        let s_140_3: bool = ((s_140_1) < (s_140_2));
        // N s_140_4: branch s_140_3 b156 b141
        if s_140_3 {
            return block_156(state, tracer, fn_state);
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
        // D s_141_1: write-var gs#112385 <= s_141_0
        fn_state.gs_112385 = s_141_0;
        // N s_141_2: jump b142
        return block_142(state, tracer, fn_state);
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_142_0: read-var gs#112385:u8
        let s_142_0: bool = fn_state.gs_112385;
        // N s_142_1: branch s_142_0 b155 b143
        if s_142_0 {
            return block_155(state, tracer, fn_state);
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
        // D s_143_1: write-var gs#112386 <= s_143_0
        fn_state.gs_112386 = s_143_0;
        // N s_143_2: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_144_0: read-var gs#112386:u8
        let s_144_0: bool = fn_state.gs_112386;
        // N s_144_1: branch s_144_0 b154 b145
        if s_144_0 {
            return block_154(state, tracer, fn_state);
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
        // D s_145_1: write-var gs#112387 <= s_145_0
        fn_state.gs_112387 = s_145_0;
        // N s_145_2: jump b146
        return block_146(state, tracer, fn_state);
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_146_0: read-var gs#112387:u8
        let s_146_0: bool = fn_state.gs_112387;
        // N s_146_1: branch s_146_0 b148 b147
        if s_146_0 {
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
        // C s_147_0: const #() : ()
        let s_147_0: () = ();
        // S s_147_1: call ICC_RPR_read(s_147_0)
        let s_147_1: ProductType700c18a878c5601b = ICC_RPR_read(state, tracer, s_147_0);
        // S s_147_2: call __get_ICC_RPR(s_147_1)
        let s_147_2: ProductType700c18a878c5601b = u__get_ICC_RPR(
            state,
            tracer,
            s_147_1,
        );
        // D s_147_3: write-var ga#183258 <= s_147_2
        fn_state.ga_183258 = s_147_2;
        // D s_147_4: read-var ga#183258.0:struct
        let s_147_4: u32 = fn_state.ga_183258._0;
        // D s_147_5: read-var t:i
        let s_147_5: i128 = fn_state.t;
        // D s_147_6: call R_set(s_147_5, s_147_4)
        let s_147_6: () = R_set(state, tracer, s_147_5, s_147_4);
        // N s_147_7: return
        return;
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_148_0: const #() : ()
        let s_148_0: () = ();
        // S s_148_1: call Halted(s_148_0)
        let s_148_1: bool = Halted(state, tracer, s_148_0);
        // N s_148_2: branch s_148_1 b153 b149
        if s_148_1 {
            return block_153(state, tracer, fn_state);
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
        // D s_149_1: write-var gs#112388 <= s_149_0
        fn_state.gs_112388 = s_149_0;
        // N s_149_2: jump b150
        return block_150(state, tracer, fn_state);
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_150_0: read-var gs#112388:u8
        let s_150_0: bool = fn_state.gs_112388;
        // N s_150_1: branch s_150_0 b152 b151
        if s_150_0 {
            return block_152(state, tracer, fn_state);
        } else {
            return block_151(state, tracer, fn_state);
        };
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_151_0: const #() : ()
        let s_151_0: () = ();
        // S s_151_1: call AArch32_TakeMonitorTrapException(s_151_0)
        let s_151_1: () = AArch32_TakeMonitorTrapException(state, tracer, s_151_0);
        // N s_151_2: return
        return;
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_152_0: panic
        panic!("{:?}", ());
        // N s_152_1: return
        return;
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_153_0: const #() : ()
        let s_153_0: () = ();
        // S s_153_1: call EDSCR_read(s_153_0)
        let s_153_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_153_0);
        // S s_153_2: call _get_EDSCR_Type_SDD(s_153_1)
        let s_153_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_153_1);
        // S s_153_3: cast zx s_153_2 -> bv
        let s_153_3: Bits = Bits::new(s_153_2 as u128, 1u16);
        // C s_153_4: const #1u : u8
        let s_153_4: bool = true;
        // C s_153_5: cast zx s_153_4 -> bv
        let s_153_5: Bits = Bits::new(s_153_4 as u128, 1u16);
        // S s_153_6: cmp-eq s_153_3 s_153_5
        let s_153_6: bool = ((s_153_3) == (s_153_5));
        // D s_153_7: write-var gs#112388 <= s_153_6
        fn_state.gs_112388 = s_153_6;
        // N s_153_8: jump b150
        return block_150(state, tracer, fn_state);
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_154_0: const #20920u : u32
        let s_154_0: u32 = 20920;
        // D s_154_1: read-reg s_154_0:struct
        let s_154_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_154_0 as isize);
            tracer.read_register(s_154_0 as isize, value);
            value
        };
        // D s_154_2: call _get_SCR_Type_IRQ(s_154_1)
        let s_154_2: bool = u_get_SCR_Type_IRQ(state, tracer, s_154_1);
        // C s_154_3: const #20920u : u32
        let s_154_3: u32 = 20920;
        // D s_154_4: read-reg s_154_3:struct
        let s_154_4: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_154_3 as isize);
            tracer.read_register(s_154_3 as isize, value);
            value
        };
        // D s_154_5: call _get_SCR_Type_FIQ(s_154_4)
        let s_154_5: bool = u_get_SCR_Type_FIQ(state, tracer, s_154_4);
        // D s_154_6: cast zx s_154_2 -> bv
        let s_154_6: Bits = Bits::new(s_154_2 as u128, 1u16);
        // D s_154_7: cast zx s_154_5 -> bv
        let s_154_7: Bits = Bits::new(s_154_5 as u128, 1u16);
        // D s_154_8: cast reint s_154_6 -> u128
        let s_154_8: u128 = (s_154_6.value() as u128);
        // D s_154_9: size-of s_154_6
        let s_154_9: u16 = s_154_6.length();
        // D s_154_10: cast reint s_154_7 -> u128
        let s_154_10: u128 = (s_154_7.value() as u128);
        // D s_154_11: size-of s_154_7
        let s_154_11: u16 = s_154_7.length();
        // D s_154_12: lsl s_154_8 s_154_11
        let s_154_12: u128 = s_154_8 << s_154_11;
        // D s_154_13: or s_154_12 s_154_10
        let s_154_13: u128 = ((s_154_12) | (s_154_10));
        // D s_154_14: add s_154_9 s_154_11
        let s_154_14: u16 = (s_154_9 + s_154_11);
        // D s_154_15: create-bits s_154_13 s_154_14
        let s_154_15: Bits = Bits::new(s_154_13, s_154_14);
        // D s_154_16: cast reint s_154_15 -> u8
        let s_154_16: u8 = (s_154_15.value() as u8);
        // D s_154_17: cast zx s_154_16 -> bv
        let s_154_17: Bits = Bits::new(s_154_16 as u128, 2u16);
        // C s_154_18: const #3u : u8
        let s_154_18: u8 = 3;
        // C s_154_19: cast zx s_154_18 -> bv
        let s_154_19: Bits = Bits::new(s_154_18 as u128, 2u16);
        // D s_154_20: cmp-eq s_154_17 s_154_19
        let s_154_20: bool = ((s_154_17) == (s_154_19));
        // D s_154_21: write-var gs#112387 <= s_154_20
        fn_state.gs_112387 = s_154_20;
        // N s_154_22: jump b146
        return block_146(state, tracer, fn_state);
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_155_0: read-var __PSTATE_M:u8
        let s_155_0: u8 = fn_state.u__PSTATE_M;
        // D s_155_1: cast zx s_155_0 -> bv
        let s_155_1: Bits = Bits::new(s_155_0 as u128, 5u16);
        // C s_155_2: const #384u : u32
        let s_155_2: u32 = 384;
        // D s_155_3: read-reg s_155_2:u8
        let s_155_3: u8 = {
            let value = state.read_register::<u8>(s_155_2 as isize);
            tracer.read_register(s_155_2 as isize, value);
            value
        };
        // D s_155_4: cast zx s_155_3 -> bv
        let s_155_4: Bits = Bits::new(s_155_3 as u128, 5u16);
        // D s_155_5: cmp-ne s_155_1 s_155_4
        let s_155_5: bool = ((s_155_1) != (s_155_4));
        // D s_155_6: write-var gs#112386 <= s_155_5
        fn_state.gs_112386 = s_155_5;
        // N s_155_7: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_156_0: const #424u : u32
        let s_156_0: u32 = 424;
        // D s_156_1: read-reg s_156_0:u8
        let s_156_1: u8 = {
            let value = state.read_register::<u8>(s_156_0 as isize);
            tracer.read_register(s_156_0 as isize, value);
            value
        };
        // D s_156_2: call ELUsingAArch32(s_156_1)
        let s_156_2: bool = ELUsingAArch32(state, tracer, s_156_1);
        // D s_156_3: write-var gs#112385 <= s_156_2
        fn_state.gs_112385 = s_156_2;
        // N s_156_4: jump b142
        return block_142(state, tracer, fn_state);
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
        // D s_158_1: write-var gs#112389 <= s_158_0
        fn_state.gs_112389 = s_158_0;
        // N s_158_2: jump b159
        return block_159(state, tracer, fn_state);
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_159_0: read-var gs#112389:u8
        let s_159_0: bool = fn_state.gs_112389;
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
        // D s_162_7: write-var gs#112389 <= s_162_6
        fn_state.gs_112389 = s_162_6;
        // N s_162_8: jump b159
        return block_159(state, tracer, fn_state);
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_163_0: const #90704u : u32
        let s_163_0: u32 = 90704;
        // D s_163_1: read-reg s_163_0:struct
        let s_163_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_163_0 as isize);
            tracer.read_register(s_163_0 as isize, value);
            value
        };
        // D s_163_2: call _get_SCR_EL3_Type_IRQ(s_163_1)
        let s_163_2: bool = u_get_SCR_EL3_Type_IRQ(state, tracer, s_163_1);
        // C s_163_3: const #90704u : u32
        let s_163_3: u32 = 90704;
        // D s_163_4: read-reg s_163_3:struct
        let s_163_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_163_3 as isize);
            tracer.read_register(s_163_3 as isize, value);
            value
        };
        // D s_163_5: call _get_SCR_EL3_Type_FIQ(s_163_4)
        let s_163_5: bool = u_get_SCR_EL3_Type_FIQ(state, tracer, s_163_4);
        // D s_163_6: cast zx s_163_2 -> bv
        let s_163_6: Bits = Bits::new(s_163_2 as u128, 1u16);
        // D s_163_7: cast zx s_163_5 -> bv
        let s_163_7: Bits = Bits::new(s_163_5 as u128, 1u16);
        // D s_163_8: cast reint s_163_6 -> u128
        let s_163_8: u128 = (s_163_6.value() as u128);
        // D s_163_9: size-of s_163_6
        let s_163_9: u16 = s_163_6.length();
        // D s_163_10: cast reint s_163_7 -> u128
        let s_163_10: u128 = (s_163_7.value() as u128);
        // D s_163_11: size-of s_163_7
        let s_163_11: u16 = s_163_7.length();
        // D s_163_12: lsl s_163_8 s_163_11
        let s_163_12: u128 = s_163_8 << s_163_11;
        // D s_163_13: or s_163_12 s_163_10
        let s_163_13: u128 = ((s_163_12) | (s_163_10));
        // D s_163_14: add s_163_9 s_163_11
        let s_163_14: u16 = (s_163_9 + s_163_11);
        // D s_163_15: create-bits s_163_13 s_163_14
        let s_163_15: Bits = Bits::new(s_163_13, s_163_14);
        // D s_163_16: cast reint s_163_15 -> u8
        let s_163_16: u8 = (s_163_15.value() as u8);
        // D s_163_17: cast zx s_163_16 -> bv
        let s_163_17: Bits = Bits::new(s_163_16 as u128, 2u16);
        // C s_163_18: const #3u : u8
        let s_163_18: u8 = 3;
        // C s_163_19: cast zx s_163_18 -> bv
        let s_163_19: Bits = Bits::new(s_163_18 as u128, 2u16);
        // D s_163_20: cmp-eq s_163_17 s_163_19
        let s_163_20: bool = ((s_163_17) == (s_163_19));
        // D s_163_21: write-var gs#112384 <= s_163_20
        fn_state.gs_112384 = s_163_20;
        // N s_163_22: jump b139
        return block_139(state, tracer, fn_state);
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
        // D s_164_4: write-var gs#112383 <= s_164_3
        fn_state.gs_112383 = s_164_3;
        // N s_164_5: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_165_0: const #() : ()
        let s_165_0: () = ();
        // S s_165_1: call ICV_RPR_read(s_165_0)
        let s_165_1: ProductType700c18a878c5601b = ICV_RPR_read(state, tracer, s_165_0);
        // S s_165_2: call __get_ICV_RPR(s_165_1)
        let s_165_2: ProductType700c18a878c5601b = u__get_ICV_RPR(
            state,
            tracer,
            s_165_1,
        );
        // D s_165_3: write-var ga#183230 <= s_165_2
        fn_state.ga_183230 = s_165_2;
        // D s_165_4: read-var ga#183230.0:struct
        let s_165_4: u32 = fn_state.ga_183230._0;
        // D s_165_5: read-var t:i
        let s_165_5: i128 = fn_state.t;
        // D s_165_6: call R_set(s_165_5, s_165_4)
        let s_165_6: () = R_set(state, tracer, s_165_5, s_165_4);
        // N s_165_7: return
        return;
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_166_0: read-var __HCR_IMO:u8
        let s_166_0: bool = fn_state.u__HCR_IMO;
        // D s_166_1: cast zx s_166_0 -> bv
        let s_166_1: Bits = Bits::new(s_166_0 as u128, 1u16);
        // C s_166_2: const #1u : u8
        let s_166_2: bool = true;
        // C s_166_3: cast zx s_166_2 -> bv
        let s_166_3: Bits = Bits::new(s_166_2 as u128, 1u16);
        // D s_166_4: cmp-eq s_166_1 s_166_3
        let s_166_4: bool = ((s_166_1) == (s_166_3));
        // D s_166_5: write-var gs#112382 <= s_166_4
        fn_state.gs_112382 = s_166_4;
        // N s_166_6: jump b134
        return block_134(state, tracer, fn_state);
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
        // D s_167_3: write-var gs#112381 <= s_167_2
        fn_state.gs_112381 = s_167_2;
        // N s_167_4: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_168_0: const #() : ()
        let s_168_0: () = ();
        // S s_168_1: call ICV_RPR_read(s_168_0)
        let s_168_1: ProductType700c18a878c5601b = ICV_RPR_read(state, tracer, s_168_0);
        // S s_168_2: call __get_ICV_RPR(s_168_1)
        let s_168_2: ProductType700c18a878c5601b = u__get_ICV_RPR(
            state,
            tracer,
            s_168_1,
        );
        // D s_168_3: write-var ga#183224 <= s_168_2
        fn_state.ga_183224 = s_168_2;
        // D s_168_4: read-var ga#183224.0:struct
        let s_168_4: u32 = fn_state.ga_183224._0;
        // D s_168_5: read-var t:i
        let s_168_5: i128 = fn_state.t;
        // D s_168_6: call R_set(s_168_5, s_168_4)
        let s_168_6: () = R_set(state, tracer, s_168_5, s_168_4);
        // N s_168_7: return
        return;
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_169_0: read-var __HCR_FMO:u8
        let s_169_0: bool = fn_state.u__HCR_FMO;
        // D s_169_1: cast zx s_169_0 -> bv
        let s_169_1: Bits = Bits::new(s_169_0 as u128, 1u16);
        // C s_169_2: const #1u : u8
        let s_169_2: bool = true;
        // C s_169_3: cast zx s_169_2 -> bv
        let s_169_3: Bits = Bits::new(s_169_2 as u128, 1u16);
        // D s_169_4: cmp-eq s_169_1 s_169_3
        let s_169_4: bool = ((s_169_1) == (s_169_3));
        // D s_169_5: write-var gs#112380 <= s_169_4
        fn_state.gs_112380 = s_169_4;
        // N s_169_6: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_170_0: const #432u : u32
        let s_170_0: u32 = 432;
        // D s_170_1: read-reg s_170_0:u8
        let s_170_1: u8 = {
            let value = state.read_register::<u8>(s_170_0 as isize);
            tracer.read_register(s_170_0 as isize, value);
            value
        };
        // D s_170_2: call ELUsingAArch32(s_170_1)
        let s_170_2: bool = ELUsingAArch32(state, tracer, s_170_1);
        // D s_170_3: write-var gs#112379 <= s_170_2
        fn_state.gs_112379 = s_170_2;
        // N s_170_4: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_171_0: const #() : ()
        let s_171_0: () = ();
        // S s_171_1: call ICV_RPR_read(s_171_0)
        let s_171_1: ProductType700c18a878c5601b = ICV_RPR_read(state, tracer, s_171_0);
        // S s_171_2: call __get_ICV_RPR(s_171_1)
        let s_171_2: ProductType700c18a878c5601b = u__get_ICV_RPR(
            state,
            tracer,
            s_171_1,
        );
        // D s_171_3: write-var ga#183218 <= s_171_2
        fn_state.ga_183218 = s_171_2;
        // D s_171_4: read-var ga#183218.0:struct
        let s_171_4: u32 = fn_state.ga_183218._0;
        // D s_171_5: read-var t:i
        let s_171_5: i128 = fn_state.t;
        // D s_171_6: call R_set(s_171_5, s_171_4)
        let s_171_6: () = R_set(state, tracer, s_171_5, s_171_4);
        // N s_171_7: return
        return;
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_172_0: read-var __HCR_EL2_IMO:u8
        let s_172_0: bool = fn_state.u__HCR_EL2_IMO;
        // D s_172_1: cast zx s_172_0 -> bv
        let s_172_1: Bits = Bits::new(s_172_0 as u128, 1u16);
        // C s_172_2: const #1u : u8
        let s_172_2: bool = true;
        // C s_172_3: cast zx s_172_2 -> bv
        let s_172_3: Bits = Bits::new(s_172_2 as u128, 1u16);
        // D s_172_4: cmp-eq s_172_1 s_172_3
        let s_172_4: bool = ((s_172_1) == (s_172_3));
        // D s_172_5: write-var gs#112378 <= s_172_4
        fn_state.gs_112378 = s_172_4;
        // N s_172_6: jump b124
        return block_124(state, tracer, fn_state);
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
        // D s_173_4: write-var gs#112377 <= s_173_3
        fn_state.gs_112377 = s_173_3;
        // N s_173_5: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_174_0: const #() : ()
        let s_174_0: () = ();
        // S s_174_1: call ICV_RPR_read(s_174_0)
        let s_174_1: ProductType700c18a878c5601b = ICV_RPR_read(state, tracer, s_174_0);
        // S s_174_2: call __get_ICV_RPR(s_174_1)
        let s_174_2: ProductType700c18a878c5601b = u__get_ICV_RPR(
            state,
            tracer,
            s_174_1,
        );
        // D s_174_3: write-var ga#183211 <= s_174_2
        fn_state.ga_183211 = s_174_2;
        // D s_174_4: read-var ga#183211.0:struct
        let s_174_4: u32 = fn_state.ga_183211._0;
        // D s_174_5: read-var t:i
        let s_174_5: i128 = fn_state.t;
        // D s_174_6: call R_set(s_174_5, s_174_4)
        let s_174_6: () = R_set(state, tracer, s_174_5, s_174_4);
        // N s_174_7: return
        return;
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_175_0: read-var __HCR_EL2_FMO:u8
        let s_175_0: bool = fn_state.u__HCR_EL2_FMO;
        // D s_175_1: cast zx s_175_0 -> bv
        let s_175_1: Bits = Bits::new(s_175_0 as u128, 1u16);
        // C s_175_2: const #1u : u8
        let s_175_2: bool = true;
        // C s_175_3: cast zx s_175_2 -> bv
        let s_175_3: Bits = Bits::new(s_175_2 as u128, 1u16);
        // D s_175_4: cmp-eq s_175_1 s_175_3
        let s_175_4: bool = ((s_175_1) == (s_175_3));
        // D s_175_5: write-var gs#112376 <= s_175_4
        fn_state.gs_112376 = s_175_4;
        // N s_175_6: jump b119
        return block_119(state, tracer, fn_state);
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
        // D s_176_3: not s_176_2
        let s_176_3: bool = !s_176_2;
        // D s_176_4: write-var gs#112375 <= s_176_3
        fn_state.gs_112375 = s_176_3;
        // N s_176_5: jump b117
        return block_117(state, tracer, fn_state);
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
        // S s_177_5: call AArch32_TakeHypTrapException(s_177_4)
        let s_177_5: () = AArch32_TakeHypTrapException(state, tracer, s_177_4);
        // N s_177_6: return
        return;
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_178_0: read-var __ICH_HCR_TC:u8
        let s_178_0: bool = fn_state.u__ICH_HCR_TC;
        // D s_178_1: cast zx s_178_0 -> bv
        let s_178_1: Bits = Bits::new(s_178_0 as u128, 1u16);
        // C s_178_2: const #1u : u8
        let s_178_2: bool = true;
        // C s_178_3: cast zx s_178_2 -> bv
        let s_178_3: Bits = Bits::new(s_178_2 as u128, 1u16);
        // D s_178_4: cmp-eq s_178_1 s_178_3
        let s_178_4: bool = ((s_178_1) == (s_178_3));
        // D s_178_5: write-var gs#112374 <= s_178_4
        fn_state.gs_112374 = s_178_4;
        // N s_178_6: jump b114
        return block_114(state, tracer, fn_state);
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
        // D s_179_3: write-var gs#112373 <= s_179_2
        fn_state.gs_112373 = s_179_2;
        // N s_179_4: jump b112
        return block_112(state, tracer, fn_state);
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
        // D s_181_0: read-var __ICH_HCR_EL2_TC:u8
        let s_181_0: bool = fn_state.u__ICH_HCR_EL2_TC;
        // D s_181_1: cast zx s_181_0 -> bv
        let s_181_1: Bits = Bits::new(s_181_0 as u128, 1u16);
        // C s_181_2: const #1u : u8
        let s_181_2: bool = true;
        // C s_181_3: cast zx s_181_2 -> bv
        let s_181_3: Bits = Bits::new(s_181_2 as u128, 1u16);
        // D s_181_4: cmp-eq s_181_1 s_181_3
        let s_181_4: bool = ((s_181_1) == (s_181_3));
        // D s_181_5: write-var gs#112372 <= s_181_4
        fn_state.gs_112372 = s_181_4;
        // N s_181_6: jump b109
        return block_109(state, tracer, fn_state);
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
        // D s_182_3: not s_182_2
        let s_182_3: bool = !s_182_2;
        // D s_182_4: write-var gs#112371 <= s_182_3
        fn_state.gs_112371 = s_182_3;
        // N s_182_5: jump b107
        return block_107(state, tracer, fn_state);
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
        // D s_184_0: read-var __HSTR_T12:u8
        let s_184_0: bool = fn_state.u__HSTR_T12;
        // D s_184_1: cast zx s_184_0 -> bv
        let s_184_1: Bits = Bits::new(s_184_0 as u128, 1u16);
        // C s_184_2: const #1u : u8
        let s_184_2: bool = true;
        // C s_184_3: cast zx s_184_2 -> bv
        let s_184_3: Bits = Bits::new(s_184_2 as u128, 1u16);
        // D s_184_4: cmp-eq s_184_1 s_184_3
        let s_184_4: bool = ((s_184_1) == (s_184_3));
        // D s_184_5: write-var gs#112370 <= s_184_4
        fn_state.gs_112370 = s_184_4;
        // N s_184_6: jump b104
        return block_104(state, tracer, fn_state);
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
        // D s_185_3: write-var gs#112369 <= s_185_2
        fn_state.gs_112369 = s_185_2;
        // N s_185_4: jump b102
        return block_102(state, tracer, fn_state);
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
        // D s_187_0: read-var __HSTR_EL2_T12:u8
        let s_187_0: bool = fn_state.u__HSTR_EL2_T12;
        // D s_187_1: cast zx s_187_0 -> bv
        let s_187_1: Bits = Bits::new(s_187_0 as u128, 1u16);
        // C s_187_2: const #1u : u8
        let s_187_2: bool = true;
        // C s_187_3: cast zx s_187_2 -> bv
        let s_187_3: Bits = Bits::new(s_187_2 as u128, 1u16);
        // D s_187_4: cmp-eq s_187_1 s_187_3
        let s_187_4: bool = ((s_187_1) == (s_187_3));
        // D s_187_5: write-var gs#112368 <= s_187_4
        fn_state.gs_112368 = s_187_4;
        // N s_187_6: jump b99
        return block_99(state, tracer, fn_state);
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
        // D s_188_4: write-var gs#112367 <= s_188_3
        fn_state.gs_112367 = s_188_3;
        // N s_188_5: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_189<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_189_0: panic
        panic!("{:?}", ());
        // N s_189_1: return
        return;
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_190_0: const #20920u : u32
        let s_190_0: u32 = 20920;
        // D s_190_1: read-reg s_190_0:struct
        let s_190_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_190_0 as isize);
            tracer.read_register(s_190_0 as isize, value);
            value
        };
        // D s_190_2: call _get_SCR_Type_IRQ(s_190_1)
        let s_190_2: bool = u_get_SCR_Type_IRQ(state, tracer, s_190_1);
        // C s_190_3: const #20920u : u32
        let s_190_3: u32 = 20920;
        // D s_190_4: read-reg s_190_3:struct
        let s_190_4: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_190_3 as isize);
            tracer.read_register(s_190_3 as isize, value);
            value
        };
        // D s_190_5: call _get_SCR_Type_FIQ(s_190_4)
        let s_190_5: bool = u_get_SCR_Type_FIQ(state, tracer, s_190_4);
        // D s_190_6: cast zx s_190_2 -> bv
        let s_190_6: Bits = Bits::new(s_190_2 as u128, 1u16);
        // D s_190_7: cast zx s_190_5 -> bv
        let s_190_7: Bits = Bits::new(s_190_5 as u128, 1u16);
        // D s_190_8: cast reint s_190_6 -> u128
        let s_190_8: u128 = (s_190_6.value() as u128);
        // D s_190_9: size-of s_190_6
        let s_190_9: u16 = s_190_6.length();
        // D s_190_10: cast reint s_190_7 -> u128
        let s_190_10: u128 = (s_190_7.value() as u128);
        // D s_190_11: size-of s_190_7
        let s_190_11: u16 = s_190_7.length();
        // D s_190_12: lsl s_190_8 s_190_11
        let s_190_12: u128 = s_190_8 << s_190_11;
        // D s_190_13: or s_190_12 s_190_10
        let s_190_13: u128 = ((s_190_12) | (s_190_10));
        // D s_190_14: add s_190_9 s_190_11
        let s_190_14: u16 = (s_190_9 + s_190_11);
        // D s_190_15: create-bits s_190_13 s_190_14
        let s_190_15: Bits = Bits::new(s_190_13, s_190_14);
        // D s_190_16: cast reint s_190_15 -> u8
        let s_190_16: u8 = (s_190_15.value() as u8);
        // D s_190_17: cast zx s_190_16 -> bv
        let s_190_17: Bits = Bits::new(s_190_16 as u128, 2u16);
        // C s_190_18: const #3u : u8
        let s_190_18: u8 = 3;
        // C s_190_19: cast zx s_190_18 -> bv
        let s_190_19: Bits = Bits::new(s_190_18 as u128, 2u16);
        // D s_190_20: cmp-eq s_190_17 s_190_19
        let s_190_20: bool = ((s_190_17) == (s_190_19));
        // D s_190_21: write-var gs#112366 <= s_190_20
        fn_state.gs_112366 = s_190_20;
        // N s_190_22: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_191_0: read-var __PSTATE_M:u8
        let s_191_0: u8 = fn_state.u__PSTATE_M;
        // D s_191_1: cast zx s_191_0 -> bv
        let s_191_1: Bits = Bits::new(s_191_0 as u128, 5u16);
        // C s_191_2: const #384u : u32
        let s_191_2: u32 = 384;
        // D s_191_3: read-reg s_191_2:u8
        let s_191_3: u8 = {
            let value = state.read_register::<u8>(s_191_2 as isize);
            tracer.read_register(s_191_2 as isize, value);
            value
        };
        // D s_191_4: cast zx s_191_3 -> bv
        let s_191_4: Bits = Bits::new(s_191_3 as u128, 5u16);
        // D s_191_5: cmp-ne s_191_1 s_191_4
        let s_191_5: bool = ((s_191_1) != (s_191_4));
        // D s_191_6: write-var gs#112365 <= s_191_5
        fn_state.gs_112365 = s_191_5;
        // N s_191_7: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_192_0: const #424u : u32
        let s_192_0: u32 = 424;
        // D s_192_1: read-reg s_192_0:u8
        let s_192_1: u8 = {
            let value = state.read_register::<u8>(s_192_0 as isize);
            tracer.read_register(s_192_0 as isize, value);
            value
        };
        // D s_192_2: call ELUsingAArch32(s_192_1)
        let s_192_2: bool = ELUsingAArch32(state, tracer, s_192_1);
        // D s_192_3: write-var gs#112364 <= s_192_2
        fn_state.gs_112364 = s_192_2;
        // N s_192_4: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_193_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_193_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_193_1: call __IMPDEF_boolean(s_193_0)
        let s_193_1: bool = u__IMPDEF_boolean(state, tracer, s_193_0);
        // D s_193_2: write-var gs#112363 <= s_193_1
        fn_state.gs_112363 = s_193_1;
        // N s_193_3: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_194_0: const #() : ()
        let s_194_0: () = ();
        // S s_194_1: call EDSCR_read(s_194_0)
        let s_194_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_194_0);
        // S s_194_2: call _get_EDSCR_Type_SDD(s_194_1)
        let s_194_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_194_1);
        // S s_194_3: cast zx s_194_2 -> bv
        let s_194_3: Bits = Bits::new(s_194_2 as u128, 1u16);
        // C s_194_4: const #1u : u8
        let s_194_4: bool = true;
        // C s_194_5: cast zx s_194_4 -> bv
        let s_194_5: Bits = Bits::new(s_194_4 as u128, 1u16);
        // S s_194_6: cmp-eq s_194_3 s_194_5
        let s_194_6: bool = ((s_194_3) == (s_194_5));
        // D s_194_7: write-var gs#112362 <= s_194_6
        fn_state.gs_112362 = s_194_6;
        // N s_194_8: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_195<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_195_0: const #424u : u32
        let s_195_0: u32 = 424;
        // D s_195_1: read-reg s_195_0:u8
        let s_195_1: u8 = {
            let value = state.read_register::<u8>(s_195_0 as isize);
            tracer.read_register(s_195_0 as isize, value);
            value
        };
        // C s_195_2: const #2u : u8
        let s_195_2: u8 = 2;
        // D s_195_3: cmp-lt s_195_1 s_195_2
        let s_195_3: bool = ((s_195_1) < (s_195_2));
        // D s_195_4: write-var gs#112361 <= s_195_3
        fn_state.gs_112361 = s_195_3;
        // N s_195_5: jump b84
        return block_84(state, tracer, fn_state);
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
        // C s_197_0: const #90704u : u32
        let s_197_0: u32 = 90704;
        // D s_197_1: read-reg s_197_0:struct
        let s_197_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_197_0 as isize);
            tracer.read_register(s_197_0 as isize, value);
            value
        };
        // D s_197_2: call _get_SCR_EL3_Type_IRQ(s_197_1)
        let s_197_2: bool = u_get_SCR_EL3_Type_IRQ(state, tracer, s_197_1);
        // C s_197_3: const #90704u : u32
        let s_197_3: u32 = 90704;
        // D s_197_4: read-reg s_197_3:struct
        let s_197_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_197_3 as isize);
            tracer.read_register(s_197_3 as isize, value);
            value
        };
        // D s_197_5: call _get_SCR_EL3_Type_FIQ(s_197_4)
        let s_197_5: bool = u_get_SCR_EL3_Type_FIQ(state, tracer, s_197_4);
        // D s_197_6: cast zx s_197_2 -> bv
        let s_197_6: Bits = Bits::new(s_197_2 as u128, 1u16);
        // D s_197_7: cast zx s_197_5 -> bv
        let s_197_7: Bits = Bits::new(s_197_5 as u128, 1u16);
        // D s_197_8: cast reint s_197_6 -> u128
        let s_197_8: u128 = (s_197_6.value() as u128);
        // D s_197_9: size-of s_197_6
        let s_197_9: u16 = s_197_6.length();
        // D s_197_10: cast reint s_197_7 -> u128
        let s_197_10: u128 = (s_197_7.value() as u128);
        // D s_197_11: size-of s_197_7
        let s_197_11: u16 = s_197_7.length();
        // D s_197_12: lsl s_197_8 s_197_11
        let s_197_12: u128 = s_197_8 << s_197_11;
        // D s_197_13: or s_197_12 s_197_10
        let s_197_13: u128 = ((s_197_12) | (s_197_10));
        // D s_197_14: add s_197_9 s_197_11
        let s_197_14: u16 = (s_197_9 + s_197_11);
        // D s_197_15: create-bits s_197_13 s_197_14
        let s_197_15: Bits = Bits::new(s_197_13, s_197_14);
        // D s_197_16: cast reint s_197_15 -> u8
        let s_197_16: u8 = (s_197_15.value() as u8);
        // D s_197_17: cast zx s_197_16 -> bv
        let s_197_17: Bits = Bits::new(s_197_16 as u128, 2u16);
        // C s_197_18: const #3u : u8
        let s_197_18: u8 = 3;
        // C s_197_19: cast zx s_197_18 -> bv
        let s_197_19: Bits = Bits::new(s_197_18 as u128, 2u16);
        // D s_197_20: cmp-eq s_197_17 s_197_19
        let s_197_20: bool = ((s_197_17) == (s_197_19));
        // D s_197_21: write-var gs#112360 <= s_197_20
        fn_state.gs_112360 = s_197_20;
        // N s_197_22: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_198<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_198_0: const #424u : u32
        let s_198_0: u32 = 424;
        // D s_198_1: read-reg s_198_0:u8
        let s_198_1: u8 = {
            let value = state.read_register::<u8>(s_198_0 as isize);
            tracer.read_register(s_198_0 as isize, value);
            value
        };
        // D s_198_2: call ELUsingAArch32(s_198_1)
        let s_198_2: bool = ELUsingAArch32(state, tracer, s_198_1);
        // D s_198_3: not s_198_2
        let s_198_3: bool = !s_198_2;
        // D s_198_4: write-var gs#112359 <= s_198_3
        fn_state.gs_112359 = s_198_3;
        // N s_198_5: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_199<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_199_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_199_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_199_1: call __IMPDEF_boolean(s_199_0)
        let s_199_1: bool = u__IMPDEF_boolean(state, tracer, s_199_0);
        // D s_199_2: write-var gs#112358 <= s_199_1
        fn_state.gs_112358 = s_199_1;
        // N s_199_3: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_200<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_200_0: const #() : ()
        let s_200_0: () = ();
        // S s_200_1: call EDSCR_read(s_200_0)
        let s_200_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_200_0);
        // S s_200_2: call _get_EDSCR_Type_SDD(s_200_1)
        let s_200_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_200_1);
        // S s_200_3: cast zx s_200_2 -> bv
        let s_200_3: Bits = Bits::new(s_200_2 as u128, 1u16);
        // C s_200_4: const #1u : u8
        let s_200_4: bool = true;
        // C s_200_5: cast zx s_200_4 -> bv
        let s_200_5: Bits = Bits::new(s_200_4 as u128, 1u16);
        // S s_200_6: cmp-eq s_200_3 s_200_5
        let s_200_6: bool = ((s_200_3) == (s_200_5));
        // D s_200_7: write-var gs#112357 <= s_200_6
        fn_state.gs_112357 = s_200_6;
        // N s_200_8: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_201<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_201_0: const #424u : u32
        let s_201_0: u32 = 424;
        // D s_201_1: read-reg s_201_0:u8
        let s_201_1: u8 = {
            let value = state.read_register::<u8>(s_201_0 as isize);
            tracer.read_register(s_201_0 as isize, value);
            value
        };
        // C s_201_2: const #2u : u8
        let s_201_2: u8 = 2;
        // D s_201_3: cmp-lt s_201_1 s_201_2
        let s_201_3: bool = ((s_201_1) < (s_201_2));
        // D s_201_4: write-var gs#112356 <= s_201_3
        fn_state.gs_112356 = s_201_3;
        // N s_201_5: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_202<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_202_0: panic
        panic!("{:?}", ());
        // N s_202_1: return
        return;
    }
}
