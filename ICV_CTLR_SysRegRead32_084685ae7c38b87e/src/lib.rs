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
use u__get_ICC_CTLR_S::*;
use u__get_ICV_CTLR::*;
use u_get_SCR_Type_IRQ::*;
use AArch32_TakeHypTrapException::*;
use Halted::*;
use ICC_CTLR_read::*;
use u__get_ICC_CTLR::*;
use ICC_HSRE_read::*;
use ICC_CTLR_NS_read::*;
use ICC_CTLR_S_read::*;
use u_get_HCR_Type_FMO::*;
use u_get_EDSCR_Type_SDD::*;
use ICV_CTLR_read::*;
use u__get_ICC_CTLR_NS::*;
use u_get_HCR_Type_IMO::*;
use u_get_HSTR_Type_T12::*;
use u_get_ICC_MSRE_Type_SRE::*;
use EL2Enabled::*;
use u_get_SCR_Type_FIQ::*;
use HCR_read::*;
use u_get_ICH_HCR_Type_TC::*;
use u_get_ICC_HSRE_Type_SRE::*;
use u_get_SCR_EL3_Type_IRQ::*;
use u__IMPDEF_boolean::*;
use u_get_HSTR_EL2_Type_T12::*;
use HSTR_read::*;
use u_get_SCR_Type_NS::*;
use u_get_HCR_EL2_Type_FMO::*;
use AArch64_AArch32SystemAccessTrap::*;
use u_get_SCR_EL3_Type_FIQ::*;
use R_set::*;
use ELUsingAArch32::*;
use u_get_ICH_HCR_EL2_Type_TC::*;
use u_get_HCR_EL2_Type_IMO::*;
use ICH_HCR_read::*;
use AArch32_TakeMonitorTrapException::*;
use EDSCR_read::*;
use common::*;
pub fn ICV_CTLR_SysRegRead32_084685ae7c38b87e<T: Tracer>(
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
        gs_112198: bool,
        gs_112168: bool,
        gs_112158: bool,
        gs_112176: bool,
        gs_112179: bool,
        gs_112157: bool,
        gs_112184: bool,
        gs_112155: bool,
        u__HCR_EL2_IMO: bool,
        gs_112200: bool,
        ga_182741: ProductType700c18a878c5601b,
        gs_112171: bool,
        ga_182637: ProductType700c18a878c5601b,
        gs_112159: bool,
        gs_112201: bool,
        ga_182744: ProductType700c18a878c5601b,
        gs_112164: bool,
        gs_112188: bool,
        ga_182672: ProductType700c18a878c5601b,
        gs_112181: bool,
        gs_112191: bool,
        gs_112194: bool,
        u__HSTR_EL2_T12: bool,
        gs_112196: bool,
        ga_182675: ProductType700c18a878c5601b,
        ga_182643: ProductType700c18a878c5601b,
        u__HCR_EL2_FMO: bool,
        gs_112193: bool,
        gs_112170: bool,
        gs_112182: bool,
        gs_112165: bool,
        u__PSTATE_EL: u8,
        gs_112180: bool,
        ga_182731: ProductType700c18a878c5601b,
        gs_112178: bool,
        gs_112187: bool,
        gs_112154: bool,
        u__ICH_HCR_EL2_TC: bool,
        gs_112160: bool,
        gs_112197: bool,
        ga_182631: ProductType700c18a878c5601b,
        u__ICC_HSRE_SRE: bool,
        u__HSTR_T12: bool,
        gs_112175: bool,
        u__ICH_HCR_TC: bool,
        gs_112163: bool,
        gs_112189: bool,
        u__HCR_IMO: bool,
        gs_112162: bool,
        gs_112161: bool,
        gs_112183: bool,
        u__SCR_NS: bool,
        gs_112192: bool,
        gs_112199: bool,
        u__ICC_MSRE_SRE: bool,
        gs_112173: bool,
        gs_112166: bool,
        gs_112195: bool,
        ga_182734: ProductType700c18a878c5601b,
        gs_112174: bool,
        gs_112185: bool,
        ga_182624: ProductType700c18a878c5601b,
        u__HCR_FMO: bool,
        u__PSTATE_M: u8,
        gs_112167: bool,
        gs_112190: bool,
        gs_112172: bool,
        gs_112177: bool,
        gs_112153: bool,
        gs_112169: bool,
        gs_112186: bool,
        gs_112156: bool,
        gs_112152: bool,
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
        // C s_0_46: const #20920u : u32
        let s_0_46: u32 = 20920;
        // D s_0_47: read-reg s_0_46:struct
        let s_0_47: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_46 as isize);
            tracer.read_register(s_0_46 as isize, value);
            value
        };
        // D s_0_48: call _get_SCR_Type_NS(s_0_47)
        let s_0_48: bool = u_get_SCR_Type_NS(state, tracer, s_0_47);
        // D s_0_49: write-var __SCR_NS <= s_0_48
        fn_state.u__SCR_NS = s_0_48;
        // D s_0_50: read-var __PSTATE_EL:u8
        let s_0_50: u8 = fn_state.u__PSTATE_EL;
        // D s_0_51: cast zx s_0_50 -> bv
        let s_0_51: Bits = Bits::new(s_0_50 as u128, 2u16);
        // C s_0_52: const #448u : u32
        let s_0_52: u32 = 448;
        // D s_0_53: read-reg s_0_52:u8
        let s_0_53: u8 = {
            let value = state.read_register::<u8>(s_0_52 as isize);
            tracer.read_register(s_0_52 as isize, value);
            value
        };
        // D s_0_54: cast zx s_0_53 -> bv
        let s_0_54: Bits = Bits::new(s_0_53 as u128, 2u16);
        // D s_0_55: cmp-eq s_0_51 s_0_54
        let s_0_55: bool = ((s_0_51) == (s_0_54));
        // N s_0_56: branch s_0_55 b208 b1
        if s_0_55 {
            return block_208(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b75 b2
        if s_1_5 {
            return block_75(state, tracer, fn_state);
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
        // N s_2_6: branch s_2_5 b10 b3
        if s_2_5 {
            return block_10(state, tracer, fn_state);
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
        // D s_6_0: read-var __SCR_NS:u8
        let s_6_0: bool = fn_state.u__SCR_NS;
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
        // S s_7_1: call ICC_CTLR_NS_read(s_7_0)
        let s_7_1: ProductType700c18a878c5601b = ICC_CTLR_NS_read(state, tracer, s_7_0);
        // S s_7_2: call __get_ICC_CTLR_NS(s_7_1)
        let s_7_2: ProductType700c18a878c5601b = u__get_ICC_CTLR_NS(
            state,
            tracer,
            s_7_1,
        );
        // D s_7_3: write-var ga#182744 <= s_7_2
        fn_state.ga_182744 = s_7_2;
        // D s_7_4: read-var ga#182744.0:struct
        let s_7_4: u32 = fn_state.ga_182744._0;
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
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call ICC_CTLR_S_read(s_8_0)
        let s_8_1: ProductType700c18a878c5601b = ICC_CTLR_S_read(state, tracer, s_8_0);
        // S s_8_2: call __get_ICC_CTLR_S(s_8_1)
        let s_8_2: ProductType700c18a878c5601b = u__get_ICC_CTLR_S(state, tracer, s_8_1);
        // D s_8_3: write-var ga#182741 <= s_8_2
        fn_state.ga_182741 = s_8_2;
        // D s_8_4: read-var ga#182741.0:struct
        let s_8_4: u32 = fn_state.ga_182741._0;
        // D s_8_5: read-var t:i
        let s_8_5: i128 = fn_state.t;
        // D s_8_6: call R_set(s_8_5, s_8_4)
        let s_8_6: () = R_set(state, tracer, s_8_5, s_8_4);
        // N s_8_7: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: panic
        panic!("{:?}", ());
        // N s_9_1: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call Halted(s_10_0)
        let s_10_1: bool = Halted(state, tracer, s_10_0);
        // N s_10_2: branch s_10_1 b74 b11
        if s_10_1 {
            return block_74(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#112152 <= s_11_0
        fn_state.gs_112152 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#112152:u8
        let s_12_0: bool = fn_state.gs_112152;
        // N s_12_1: branch s_12_0 b73 b13
        if s_12_0 {
            return block_73(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#112153 <= s_13_0
        fn_state.gs_112153 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#112153:u8
        let s_14_0: bool = fn_state.gs_112153;
        // N s_14_1: branch s_14_0 b72 b15
        if s_14_0 {
            return block_72(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#112154 <= s_15_0
        fn_state.gs_112154 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#112154:u8
        let s_16_0: bool = fn_state.gs_112154;
        // N s_16_1: branch s_16_0 b71 b17
        if s_16_0 {
            return block_71(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#112155 <= s_17_0
        fn_state.gs_112155 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#112155:u8
        let s_18_0: bool = fn_state.gs_112155;
        // N s_18_1: branch s_18_0 b70 b19
        if s_18_0 {
            return block_70(state, tracer, fn_state);
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
        // D s_19_1: write-var gs#112156 <= s_19_0
        fn_state.gs_112156 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#112156:u8
        let s_20_0: bool = fn_state.gs_112156;
        // N s_20_1: branch s_20_0 b69 b21
        if s_20_0 {
            return block_69(state, tracer, fn_state);
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
        // S s_21_1: call Halted(s_21_0)
        let s_21_1: bool = Halted(state, tracer, s_21_0);
        // N s_21_2: branch s_21_1 b68 b22
        if s_21_1 {
            return block_68(state, tracer, fn_state);
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
        // D s_22_1: write-var gs#112157 <= s_22_0
        fn_state.gs_112157 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#112157:u8
        let s_23_0: bool = fn_state.gs_112157;
        // N s_23_1: branch s_23_0 b67 b24
        if s_23_0 {
            return block_67(state, tracer, fn_state);
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
        // D s_24_1: write-var gs#112158 <= s_24_0
        fn_state.gs_112158 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#112158:u8
        let s_25_0: bool = fn_state.gs_112158;
        // N s_25_1: branch s_25_0 b66 b26
        if s_25_0 {
            return block_66(state, tracer, fn_state);
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
        // D s_26_1: write-var gs#112159 <= s_26_0
        fn_state.gs_112159 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#112159:u8
        let s_27_0: bool = fn_state.gs_112159;
        // N s_27_1: branch s_27_0 b65 b28
        if s_27_0 {
            return block_65(state, tracer, fn_state);
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
        // D s_28_1: write-var gs#112160 <= s_28_0
        fn_state.gs_112160 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#112160:u8
        let s_29_0: bool = fn_state.gs_112160;
        // N s_29_1: branch s_29_0 b64 b30
        if s_29_0 {
            return block_64(state, tracer, fn_state);
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
        // D s_30_1: write-var gs#112161 <= s_30_0
        fn_state.gs_112161 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#112161:u8
        let s_31_0: bool = fn_state.gs_112161;
        // N s_31_1: branch s_31_0 b63 b32
        if s_31_0 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var __ICC_HSRE_SRE:u8
        let s_32_0: bool = fn_state.u__ICC_HSRE_SRE;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 1u16);
        // C s_32_2: const #0u : u8
        let s_32_2: bool = false;
        // C s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 1u16);
        // D s_32_4: cmp-eq s_32_1 s_32_3
        let s_32_4: bool = ((s_32_1) == (s_32_3));
        // N s_32_5: branch s_32_4 b62 b33
        if s_32_4 {
            return block_62(state, tracer, fn_state);
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
        // N s_33_4: branch s_33_3 b61 b34
        if s_33_3 {
            return block_61(state, tracer, fn_state);
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
        // D s_34_1: write-var gs#112162 <= s_34_0
        fn_state.gs_112162 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#112162:u8
        let s_35_0: bool = fn_state.gs_112162;
        // N s_35_1: branch s_35_0 b60 b36
        if s_35_0 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #0u : u8
        let s_36_0: bool = false;
        // D s_36_1: write-var gs#112163 <= s_36_0
        fn_state.gs_112163 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#112163:u8
        let s_37_0: bool = fn_state.gs_112163;
        // N s_37_1: branch s_37_0 b54 b38
        if s_37_0 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
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
        // C s_38_2: const #2u : u8
        let s_38_2: u8 = 2;
        // D s_38_3: cmp-lt s_38_1 s_38_2
        let s_38_3: bool = ((s_38_1) < (s_38_2));
        // N s_38_4: branch s_38_3 b53 b39
        if s_38_3 {
            return block_53(state, tracer, fn_state);
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
        // D s_39_1: write-var gs#112164 <= s_39_0
        fn_state.gs_112164 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#112164:u8
        let s_40_0: bool = fn_state.gs_112164;
        // N s_40_1: branch s_40_0 b52 b41
        if s_40_0 {
            return block_52(state, tracer, fn_state);
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
        // D s_41_1: write-var gs#112165 <= s_41_0
        fn_state.gs_112165 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#112165:u8
        let s_42_0: bool = fn_state.gs_112165;
        // N s_42_1: branch s_42_0 b46 b43
        if s_42_0 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
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
        // N s_43_4: branch s_43_3 b45 b44
        if s_43_3 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #() : ()
        let s_44_0: () = ();
        // S s_44_1: call ICC_CTLR_read(s_44_0)
        let s_44_1: ProductType700c18a878c5601b = ICC_CTLR_read(state, tracer, s_44_0);
        // S s_44_2: call __get_ICC_CTLR(s_44_1)
        let s_44_2: ProductType700c18a878c5601b = u__get_ICC_CTLR(state, tracer, s_44_1);
        // D s_44_3: write-var ga#182734 <= s_44_2
        fn_state.ga_182734 = s_44_2;
        // D s_44_4: read-var ga#182734.0:struct
        let s_44_4: u32 = fn_state.ga_182734._0;
        // D s_44_5: read-var t:i
        let s_44_5: i128 = fn_state.t;
        // D s_44_6: call R_set(s_44_5, s_44_4)
        let s_44_6: () = R_set(state, tracer, s_44_5, s_44_4);
        // N s_44_7: return
        return;
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #() : ()
        let s_45_0: () = ();
        // S s_45_1: call ICC_CTLR_NS_read(s_45_0)
        let s_45_1: ProductType700c18a878c5601b = ICC_CTLR_NS_read(
            state,
            tracer,
            s_45_0,
        );
        // S s_45_2: call __get_ICC_CTLR_NS(s_45_1)
        let s_45_2: ProductType700c18a878c5601b = u__get_ICC_CTLR_NS(
            state,
            tracer,
            s_45_1,
        );
        // D s_45_3: write-var ga#182731 <= s_45_2
        fn_state.ga_182731 = s_45_2;
        // D s_45_4: read-var ga#182731.0:struct
        let s_45_4: u32 = fn_state.ga_182731._0;
        // D s_45_5: read-var t:i
        let s_45_5: i128 = fn_state.t;
        // D s_45_6: call R_set(s_45_5, s_45_4)
        let s_45_6: () = R_set(state, tracer, s_45_5, s_45_4);
        // N s_45_7: return
        return;
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #() : ()
        let s_46_0: () = ();
        // S s_46_1: call Halted(s_46_0)
        let s_46_1: bool = Halted(state, tracer, s_46_0);
        // N s_46_2: branch s_46_1 b51 b47
        if s_46_1 {
            return block_51(state, tracer, fn_state);
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
        // D s_47_1: write-var gs#112166 <= s_47_0
        fn_state.gs_112166 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#112166:u8
        let s_48_0: bool = fn_state.gs_112166;
        // N s_48_1: branch s_48_0 b50 b49
        if s_48_0 {
            return block_50(state, tracer, fn_state);
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
        // S s_49_1: call AArch32_TakeMonitorTrapException(s_49_0)
        let s_49_1: () = AArch32_TakeMonitorTrapException(state, tracer, s_49_0);
        // N s_49_2: return
        return;
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_50_0: panic
        panic!("{:?}", ());
        // N s_50_1: return
        return;
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #() : ()
        let s_51_0: () = ();
        // S s_51_1: call EDSCR_read(s_51_0)
        let s_51_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_51_0);
        // S s_51_2: call _get_EDSCR_Type_SDD(s_51_1)
        let s_51_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_51_1);
        // S s_51_3: cast zx s_51_2 -> bv
        let s_51_3: Bits = Bits::new(s_51_2 as u128, 1u16);
        // C s_51_4: const #1u : u8
        let s_51_4: bool = true;
        // C s_51_5: cast zx s_51_4 -> bv
        let s_51_5: Bits = Bits::new(s_51_4 as u128, 1u16);
        // S s_51_6: cmp-eq s_51_3 s_51_5
        let s_51_6: bool = ((s_51_3) == (s_51_5));
        // D s_51_7: write-var gs#112166 <= s_51_6
        fn_state.gs_112166 = s_51_6;
        // N s_51_8: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #20920u : u32
        let s_52_0: u32 = 20920;
        // D s_52_1: read-reg s_52_0:struct
        let s_52_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_52_0 as isize);
            tracer.read_register(s_52_0 as isize, value);
            value
        };
        // D s_52_2: call _get_SCR_Type_IRQ(s_52_1)
        let s_52_2: bool = u_get_SCR_Type_IRQ(state, tracer, s_52_1);
        // C s_52_3: const #20920u : u32
        let s_52_3: u32 = 20920;
        // D s_52_4: read-reg s_52_3:struct
        let s_52_4: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_52_3 as isize);
            tracer.read_register(s_52_3 as isize, value);
            value
        };
        // D s_52_5: call _get_SCR_Type_FIQ(s_52_4)
        let s_52_5: bool = u_get_SCR_Type_FIQ(state, tracer, s_52_4);
        // D s_52_6: cast zx s_52_2 -> bv
        let s_52_6: Bits = Bits::new(s_52_2 as u128, 1u16);
        // D s_52_7: cast zx s_52_5 -> bv
        let s_52_7: Bits = Bits::new(s_52_5 as u128, 1u16);
        // D s_52_8: cast reint s_52_6 -> u128
        let s_52_8: u128 = (s_52_6.value() as u128);
        // D s_52_9: size-of s_52_6
        let s_52_9: u16 = s_52_6.length();
        // D s_52_10: cast reint s_52_7 -> u128
        let s_52_10: u128 = (s_52_7.value() as u128);
        // D s_52_11: size-of s_52_7
        let s_52_11: u16 = s_52_7.length();
        // D s_52_12: lsl s_52_8 s_52_11
        let s_52_12: u128 = s_52_8 << s_52_11;
        // D s_52_13: or s_52_12 s_52_10
        let s_52_13: u128 = ((s_52_12) | (s_52_10));
        // D s_52_14: add s_52_9 s_52_11
        let s_52_14: u16 = (s_52_9 + s_52_11);
        // D s_52_15: create-bits s_52_13 s_52_14
        let s_52_15: Bits = Bits::new(s_52_13, s_52_14);
        // D s_52_16: cast reint s_52_15 -> u8
        let s_52_16: u8 = (s_52_15.value() as u8);
        // D s_52_17: cast zx s_52_16 -> bv
        let s_52_17: Bits = Bits::new(s_52_16 as u128, 2u16);
        // C s_52_18: const #3u : u8
        let s_52_18: u8 = 3;
        // C s_52_19: cast zx s_52_18 -> bv
        let s_52_19: Bits = Bits::new(s_52_18 as u128, 2u16);
        // D s_52_20: cmp-eq s_52_17 s_52_19
        let s_52_20: bool = ((s_52_17) == (s_52_19));
        // D s_52_21: write-var gs#112165 <= s_52_20
        fn_state.gs_112165 = s_52_20;
        // N s_52_22: jump b42
        return block_42(state, tracer, fn_state);
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
        // D s_53_2: call ELUsingAArch32(s_53_1)
        let s_53_2: bool = ELUsingAArch32(state, tracer, s_53_1);
        // D s_53_3: write-var gs#112164 <= s_53_2
        fn_state.gs_112164 = s_53_2;
        // N s_53_4: jump b40
        return block_40(state, tracer, fn_state);
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
        // N s_54_2: branch s_54_1 b59 b55
        if s_54_1 {
            return block_59(state, tracer, fn_state);
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
        // D s_55_1: write-var gs#112167 <= s_55_0
        fn_state.gs_112167 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#112167:u8
        let s_56_0: bool = fn_state.gs_112167;
        // N s_56_1: branch s_56_0 b58 b57
        if s_56_0 {
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
        // C s_57_0: const #3u : u8
        let s_57_0: u8 = 3;
        // C s_57_1: cast zx s_57_0 -> bv
        let s_57_1: Bits = Bits::new(s_57_0 as u128, 8u16);
        // C s_57_2: cast zx s_57_1 -> i
        let s_57_2: i128 = (s_57_1.value() as i128);
        // C s_57_3: cast reint s_57_2 -> i64
        let s_57_3: i64 = (s_57_2 as i64);
        // C s_57_4: cast zx s_57_3 -> i
        let s_57_4: i128 = (i128::try_from(s_57_3).unwrap());
        // C s_57_5: const #424u : u32
        let s_57_5: u32 = 424;
        // D s_57_6: read-reg s_57_5:u8
        let s_57_6: u8 = {
            let value = state.read_register::<u8>(s_57_5 as isize);
            tracer.read_register(s_57_5 as isize, value);
            value
        };
        // D s_57_7: call AArch64_AArch32SystemAccessTrap(s_57_6, s_57_4)
        let s_57_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_57_6, s_57_4);
        // N s_57_8: return
        return;
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
        // C s_59_0: const #() : ()
        let s_59_0: () = ();
        // S s_59_1: call EDSCR_read(s_59_0)
        let s_59_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_59_0);
        // S s_59_2: call _get_EDSCR_Type_SDD(s_59_1)
        let s_59_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_59_1);
        // S s_59_3: cast zx s_59_2 -> bv
        let s_59_3: Bits = Bits::new(s_59_2 as u128, 1u16);
        // C s_59_4: const #1u : u8
        let s_59_4: bool = true;
        // C s_59_5: cast zx s_59_4 -> bv
        let s_59_5: Bits = Bits::new(s_59_4 as u128, 1u16);
        // S s_59_6: cmp-eq s_59_3 s_59_5
        let s_59_6: bool = ((s_59_3) == (s_59_5));
        // D s_59_7: write-var gs#112167 <= s_59_6
        fn_state.gs_112167 = s_59_6;
        // N s_59_8: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #90704u : u32
        let s_60_0: u32 = 90704;
        // D s_60_1: read-reg s_60_0:struct
        let s_60_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_60_0 as isize);
            tracer.read_register(s_60_0 as isize, value);
            value
        };
        // D s_60_2: call _get_SCR_EL3_Type_IRQ(s_60_1)
        let s_60_2: bool = u_get_SCR_EL3_Type_IRQ(state, tracer, s_60_1);
        // C s_60_3: const #90704u : u32
        let s_60_3: u32 = 90704;
        // D s_60_4: read-reg s_60_3:struct
        let s_60_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_60_3 as isize);
            tracer.read_register(s_60_3 as isize, value);
            value
        };
        // D s_60_5: call _get_SCR_EL3_Type_FIQ(s_60_4)
        let s_60_5: bool = u_get_SCR_EL3_Type_FIQ(state, tracer, s_60_4);
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
        // D s_60_21: write-var gs#112163 <= s_60_20
        fn_state.gs_112163 = s_60_20;
        // N s_60_22: jump b37
        return block_37(state, tracer, fn_state);
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
        // D s_61_3: not s_61_2
        let s_61_3: bool = !s_61_2;
        // D s_61_4: write-var gs#112162 <= s_61_3
        fn_state.gs_112162 = s_61_3;
        // N s_61_5: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_62_0: panic
        panic!("{:?}", ());
        // N s_62_1: return
        return;
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_63_0: panic
        panic!("{:?}", ());
        // N s_63_1: return
        return;
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #20920u : u32
        let s_64_0: u32 = 20920;
        // D s_64_1: read-reg s_64_0:struct
        let s_64_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_64_0 as isize);
            tracer.read_register(s_64_0 as isize, value);
            value
        };
        // D s_64_2: call _get_SCR_Type_IRQ(s_64_1)
        let s_64_2: bool = u_get_SCR_Type_IRQ(state, tracer, s_64_1);
        // C s_64_3: const #20920u : u32
        let s_64_3: u32 = 20920;
        // D s_64_4: read-reg s_64_3:struct
        let s_64_4: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_64_3 as isize);
            tracer.read_register(s_64_3 as isize, value);
            value
        };
        // D s_64_5: call _get_SCR_Type_FIQ(s_64_4)
        let s_64_5: bool = u_get_SCR_Type_FIQ(state, tracer, s_64_4);
        // D s_64_6: cast zx s_64_2 -> bv
        let s_64_6: Bits = Bits::new(s_64_2 as u128, 1u16);
        // D s_64_7: cast zx s_64_5 -> bv
        let s_64_7: Bits = Bits::new(s_64_5 as u128, 1u16);
        // D s_64_8: cast reint s_64_6 -> u128
        let s_64_8: u128 = (s_64_6.value() as u128);
        // D s_64_9: size-of s_64_6
        let s_64_9: u16 = s_64_6.length();
        // D s_64_10: cast reint s_64_7 -> u128
        let s_64_10: u128 = (s_64_7.value() as u128);
        // D s_64_11: size-of s_64_7
        let s_64_11: u16 = s_64_7.length();
        // D s_64_12: lsl s_64_8 s_64_11
        let s_64_12: u128 = s_64_8 << s_64_11;
        // D s_64_13: or s_64_12 s_64_10
        let s_64_13: u128 = ((s_64_12) | (s_64_10));
        // D s_64_14: add s_64_9 s_64_11
        let s_64_14: u16 = (s_64_9 + s_64_11);
        // D s_64_15: create-bits s_64_13 s_64_14
        let s_64_15: Bits = Bits::new(s_64_13, s_64_14);
        // D s_64_16: cast reint s_64_15 -> u8
        let s_64_16: u8 = (s_64_15.value() as u8);
        // D s_64_17: cast zx s_64_16 -> bv
        let s_64_17: Bits = Bits::new(s_64_16 as u128, 2u16);
        // C s_64_18: const #3u : u8
        let s_64_18: u8 = 3;
        // C s_64_19: cast zx s_64_18 -> bv
        let s_64_19: Bits = Bits::new(s_64_18 as u128, 2u16);
        // D s_64_20: cmp-eq s_64_17 s_64_19
        let s_64_20: bool = ((s_64_17) == (s_64_19));
        // D s_64_21: write-var gs#112161 <= s_64_20
        fn_state.gs_112161 = s_64_20;
        // N s_64_22: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #424u : u32
        let s_65_0: u32 = 424;
        // D s_65_1: read-reg s_65_0:u8
        let s_65_1: u8 = {
            let value = state.read_register::<u8>(s_65_0 as isize);
            tracer.read_register(s_65_0 as isize, value);
            value
        };
        // D s_65_2: call ELUsingAArch32(s_65_1)
        let s_65_2: bool = ELUsingAArch32(state, tracer, s_65_1);
        // D s_65_3: write-var gs#112160 <= s_65_2
        fn_state.gs_112160 = s_65_2;
        // N s_65_4: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_66_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_66_1: call __IMPDEF_boolean(s_66_0)
        let s_66_1: bool = u__IMPDEF_boolean(state, tracer, s_66_0);
        // D s_66_2: write-var gs#112159 <= s_66_1
        fn_state.gs_112159 = s_66_1;
        // N s_66_3: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #() : ()
        let s_67_0: () = ();
        // S s_67_1: call EDSCR_read(s_67_0)
        let s_67_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_67_0);
        // S s_67_2: call _get_EDSCR_Type_SDD(s_67_1)
        let s_67_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_67_1);
        // S s_67_3: cast zx s_67_2 -> bv
        let s_67_3: Bits = Bits::new(s_67_2 as u128, 1u16);
        // C s_67_4: const #1u : u8
        let s_67_4: bool = true;
        // C s_67_5: cast zx s_67_4 -> bv
        let s_67_5: Bits = Bits::new(s_67_4 as u128, 1u16);
        // S s_67_6: cmp-eq s_67_3 s_67_5
        let s_67_6: bool = ((s_67_3) == (s_67_5));
        // D s_67_7: write-var gs#112158 <= s_67_6
        fn_state.gs_112158 = s_67_6;
        // N s_67_8: jump b25
        return block_25(state, tracer, fn_state);
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
        // D s_68_4: write-var gs#112157 <= s_68_3
        fn_state.gs_112157 = s_68_3;
        // N s_68_5: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_69_0: panic
        panic!("{:?}", ());
        // N s_69_1: return
        return;
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #90704u : u32
        let s_70_0: u32 = 90704;
        // D s_70_1: read-reg s_70_0:struct
        let s_70_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_70_0 as isize);
            tracer.read_register(s_70_0 as isize, value);
            value
        };
        // D s_70_2: call _get_SCR_EL3_Type_IRQ(s_70_1)
        let s_70_2: bool = u_get_SCR_EL3_Type_IRQ(state, tracer, s_70_1);
        // C s_70_3: const #90704u : u32
        let s_70_3: u32 = 90704;
        // D s_70_4: read-reg s_70_3:struct
        let s_70_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_70_3 as isize);
            tracer.read_register(s_70_3 as isize, value);
            value
        };
        // D s_70_5: call _get_SCR_EL3_Type_FIQ(s_70_4)
        let s_70_5: bool = u_get_SCR_EL3_Type_FIQ(state, tracer, s_70_4);
        // D s_70_6: cast zx s_70_2 -> bv
        let s_70_6: Bits = Bits::new(s_70_2 as u128, 1u16);
        // D s_70_7: cast zx s_70_5 -> bv
        let s_70_7: Bits = Bits::new(s_70_5 as u128, 1u16);
        // D s_70_8: cast reint s_70_6 -> u128
        let s_70_8: u128 = (s_70_6.value() as u128);
        // D s_70_9: size-of s_70_6
        let s_70_9: u16 = s_70_6.length();
        // D s_70_10: cast reint s_70_7 -> u128
        let s_70_10: u128 = (s_70_7.value() as u128);
        // D s_70_11: size-of s_70_7
        let s_70_11: u16 = s_70_7.length();
        // D s_70_12: lsl s_70_8 s_70_11
        let s_70_12: u128 = s_70_8 << s_70_11;
        // D s_70_13: or s_70_12 s_70_10
        let s_70_13: u128 = ((s_70_12) | (s_70_10));
        // D s_70_14: add s_70_9 s_70_11
        let s_70_14: u16 = (s_70_9 + s_70_11);
        // D s_70_15: create-bits s_70_13 s_70_14
        let s_70_15: Bits = Bits::new(s_70_13, s_70_14);
        // D s_70_16: cast reint s_70_15 -> u8
        let s_70_16: u8 = (s_70_15.value() as u8);
        // D s_70_17: cast zx s_70_16 -> bv
        let s_70_17: Bits = Bits::new(s_70_16 as u128, 2u16);
        // C s_70_18: const #3u : u8
        let s_70_18: u8 = 3;
        // C s_70_19: cast zx s_70_18 -> bv
        let s_70_19: Bits = Bits::new(s_70_18 as u128, 2u16);
        // D s_70_20: cmp-eq s_70_17 s_70_19
        let s_70_20: bool = ((s_70_17) == (s_70_19));
        // D s_70_21: write-var gs#112156 <= s_70_20
        fn_state.gs_112156 = s_70_20;
        // N s_70_22: jump b20
        return block_20(state, tracer, fn_state);
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
        // D s_71_4: write-var gs#112155 <= s_71_3
        fn_state.gs_112155 = s_71_3;
        // N s_71_5: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_72_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_72_1: call __IMPDEF_boolean(s_72_0)
        let s_72_1: bool = u__IMPDEF_boolean(state, tracer, s_72_0);
        // D s_72_2: write-var gs#112154 <= s_72_1
        fn_state.gs_112154 = s_72_1;
        // N s_72_3: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #() : ()
        let s_73_0: () = ();
        // S s_73_1: call EDSCR_read(s_73_0)
        let s_73_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_73_0);
        // S s_73_2: call _get_EDSCR_Type_SDD(s_73_1)
        let s_73_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_73_1);
        // S s_73_3: cast zx s_73_2 -> bv
        let s_73_3: Bits = Bits::new(s_73_2 as u128, 1u16);
        // C s_73_4: const #1u : u8
        let s_73_4: bool = true;
        // C s_73_5: cast zx s_73_4 -> bv
        let s_73_5: Bits = Bits::new(s_73_4 as u128, 1u16);
        // S s_73_6: cmp-eq s_73_3 s_73_5
        let s_73_6: bool = ((s_73_3) == (s_73_5));
        // D s_73_7: write-var gs#112153 <= s_73_6
        fn_state.gs_112153 = s_73_6;
        // N s_73_8: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #424u : u32
        let s_74_0: u32 = 424;
        // D s_74_1: read-reg s_74_0:u8
        let s_74_1: u8 = {
            let value = state.read_register::<u8>(s_74_0 as isize);
            tracer.read_register(s_74_0 as isize, value);
            value
        };
        // C s_74_2: const #2u : u8
        let s_74_2: u8 = 2;
        // D s_74_3: cmp-lt s_74_1 s_74_2
        let s_74_3: bool = ((s_74_1) < (s_74_2));
        // D s_74_4: write-var gs#112152 <= s_74_3
        fn_state.gs_112152 = s_74_3;
        // N s_74_5: jump b12
        return block_12(state, tracer, fn_state);
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
        // N s_75_2: branch s_75_1 b207 b76
        if s_75_1 {
            return block_207(state, tracer, fn_state);
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
        // D s_76_1: write-var gs#112168 <= s_76_0
        fn_state.gs_112168 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var gs#112168:u8
        let s_77_0: bool = fn_state.gs_112168;
        // N s_77_1: branch s_77_0 b206 b78
        if s_77_0 {
            return block_206(state, tracer, fn_state);
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
        // D s_78_1: write-var gs#112169 <= s_78_0
        fn_state.gs_112169 = s_78_0;
        // N s_78_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var gs#112169:u8
        let s_79_0: bool = fn_state.gs_112169;
        // N s_79_1: branch s_79_0 b205 b80
        if s_79_0 {
            return block_205(state, tracer, fn_state);
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
        // D s_80_1: write-var gs#112170 <= s_80_0
        fn_state.gs_112170 = s_80_0;
        // N s_80_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#112170:u8
        let s_81_0: bool = fn_state.gs_112170;
        // N s_81_1: branch s_81_0 b204 b82
        if s_81_0 {
            return block_204(state, tracer, fn_state);
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
        // D s_82_1: write-var gs#112171 <= s_82_0
        fn_state.gs_112171 = s_82_0;
        // N s_82_2: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_83_0: read-var gs#112171:u8
        let s_83_0: bool = fn_state.gs_112171;
        // N s_83_1: branch s_83_0 b203 b84
        if s_83_0 {
            return block_203(state, tracer, fn_state);
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
        // D s_84_1: write-var gs#112172 <= s_84_0
        fn_state.gs_112172 = s_84_0;
        // N s_84_2: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var gs#112172:u8
        let s_85_0: bool = fn_state.gs_112172;
        // N s_85_1: branch s_85_0 b202 b86
        if s_85_0 {
            return block_202(state, tracer, fn_state);
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
        // S s_86_1: call Halted(s_86_0)
        let s_86_1: bool = Halted(state, tracer, s_86_0);
        // N s_86_2: branch s_86_1 b201 b87
        if s_86_1 {
            return block_201(state, tracer, fn_state);
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
        // D s_87_1: write-var gs#112173 <= s_87_0
        fn_state.gs_112173 = s_87_0;
        // N s_87_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var gs#112173:u8
        let s_88_0: bool = fn_state.gs_112173;
        // N s_88_1: branch s_88_0 b200 b89
        if s_88_0 {
            return block_200(state, tracer, fn_state);
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
        // D s_89_1: write-var gs#112174 <= s_89_0
        fn_state.gs_112174 = s_89_0;
        // N s_89_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var gs#112174:u8
        let s_90_0: bool = fn_state.gs_112174;
        // N s_90_1: branch s_90_0 b199 b91
        if s_90_0 {
            return block_199(state, tracer, fn_state);
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
        // D s_91_1: write-var gs#112175 <= s_91_0
        fn_state.gs_112175 = s_91_0;
        // N s_91_2: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var gs#112175:u8
        let s_92_0: bool = fn_state.gs_112175;
        // N s_92_1: branch s_92_0 b198 b93
        if s_92_0 {
            return block_198(state, tracer, fn_state);
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
        // D s_93_1: write-var gs#112176 <= s_93_0
        fn_state.gs_112176 = s_93_0;
        // N s_93_2: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var gs#112176:u8
        let s_94_0: bool = fn_state.gs_112176;
        // N s_94_1: branch s_94_0 b197 b95
        if s_94_0 {
            return block_197(state, tracer, fn_state);
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
        // D s_95_1: write-var gs#112177 <= s_95_0
        fn_state.gs_112177 = s_95_0;
        // N s_95_2: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var gs#112177:u8
        let s_96_0: bool = fn_state.gs_112177;
        // N s_96_1: branch s_96_0 b196 b97
        if s_96_0 {
            return block_196(state, tracer, fn_state);
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
        // D s_97_1: write-var gs#112178 <= s_97_0
        fn_state.gs_112178 = s_97_0;
        // N s_97_2: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var gs#112178:u8
        let s_98_0: bool = fn_state.gs_112178;
        // N s_98_1: branch s_98_0 b195 b99
        if s_98_0 {
            return block_195(state, tracer, fn_state);
        } else {
            return block_99(state, tracer, fn_state);
        };
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #() : ()
        let s_99_0: () = ();
        // S s_99_1: call EL2Enabled(s_99_0)
        let s_99_1: bool = EL2Enabled(state, tracer, s_99_0);
        // N s_99_2: branch s_99_1 b194 b100
        if s_99_1 {
            return block_194(state, tracer, fn_state);
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
        // D s_100_1: write-var gs#112179 <= s_100_0
        fn_state.gs_112179 = s_100_0;
        // N s_100_2: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_101_0: read-var gs#112179:u8
        let s_101_0: bool = fn_state.gs_112179;
        // N s_101_1: branch s_101_0 b193 b102
        if s_101_0 {
            return block_193(state, tracer, fn_state);
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
        // D s_102_1: write-var gs#112180 <= s_102_0
        fn_state.gs_112180 = s_102_0;
        // N s_102_2: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_103_0: read-var gs#112180:u8
        let s_103_0: bool = fn_state.gs_112180;
        // N s_103_1: branch s_103_0 b192 b104
        if s_103_0 {
            return block_192(state, tracer, fn_state);
        } else {
            return block_104(state, tracer, fn_state);
        };
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #() : ()
        let s_104_0: () = ();
        // S s_104_1: call EL2Enabled(s_104_0)
        let s_104_1: bool = EL2Enabled(state, tracer, s_104_0);
        // N s_104_2: branch s_104_1 b191 b105
        if s_104_1 {
            return block_191(state, tracer, fn_state);
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
        // D s_105_1: write-var gs#112181 <= s_105_0
        fn_state.gs_112181 = s_105_0;
        // N s_105_2: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var gs#112181:u8
        let s_106_0: bool = fn_state.gs_112181;
        // N s_106_1: branch s_106_0 b190 b107
        if s_106_0 {
            return block_190(state, tracer, fn_state);
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
        // D s_107_1: write-var gs#112182 <= s_107_0
        fn_state.gs_112182 = s_107_0;
        // N s_107_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var gs#112182:u8
        let s_108_0: bool = fn_state.gs_112182;
        // N s_108_1: branch s_108_0 b189 b109
        if s_108_0 {
            return block_189(state, tracer, fn_state);
        } else {
            return block_109(state, tracer, fn_state);
        };
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #() : ()
        let s_109_0: () = ();
        // S s_109_1: call EL2Enabled(s_109_0)
        let s_109_1: bool = EL2Enabled(state, tracer, s_109_0);
        // N s_109_2: branch s_109_1 b188 b110
        if s_109_1 {
            return block_188(state, tracer, fn_state);
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
        // D s_110_1: write-var gs#112183 <= s_110_0
        fn_state.gs_112183 = s_110_0;
        // N s_110_2: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var gs#112183:u8
        let s_111_0: bool = fn_state.gs_112183;
        // N s_111_1: branch s_111_0 b187 b112
        if s_111_0 {
            return block_187(state, tracer, fn_state);
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
        // D s_112_1: write-var gs#112184 <= s_112_0
        fn_state.gs_112184 = s_112_0;
        // N s_112_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var gs#112184:u8
        let s_113_0: bool = fn_state.gs_112184;
        // N s_113_1: branch s_113_0 b186 b114
        if s_113_0 {
            return block_186(state, tracer, fn_state);
        } else {
            return block_114(state, tracer, fn_state);
        };
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #() : ()
        let s_114_0: () = ();
        // S s_114_1: call EL2Enabled(s_114_0)
        let s_114_1: bool = EL2Enabled(state, tracer, s_114_0);
        // N s_114_2: branch s_114_1 b185 b115
        if s_114_1 {
            return block_185(state, tracer, fn_state);
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
        // D s_115_1: write-var gs#112185 <= s_115_0
        fn_state.gs_112185 = s_115_0;
        // N s_115_2: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_116_0: read-var gs#112185:u8
        let s_116_0: bool = fn_state.gs_112185;
        // N s_116_1: branch s_116_0 b184 b117
        if s_116_0 {
            return block_184(state, tracer, fn_state);
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
        // D s_117_1: write-var gs#112186 <= s_117_0
        fn_state.gs_112186 = s_117_0;
        // N s_117_2: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_118_0: read-var gs#112186:u8
        let s_118_0: bool = fn_state.gs_112186;
        // N s_118_1: branch s_118_0 b183 b119
        if s_118_0 {
            return block_183(state, tracer, fn_state);
        } else {
            return block_119(state, tracer, fn_state);
        };
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #() : ()
        let s_119_0: () = ();
        // S s_119_1: call EL2Enabled(s_119_0)
        let s_119_1: bool = EL2Enabled(state, tracer, s_119_0);
        // N s_119_2: branch s_119_1 b182 b120
        if s_119_1 {
            return block_182(state, tracer, fn_state);
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
        // D s_120_1: write-var gs#112187 <= s_120_0
        fn_state.gs_112187 = s_120_0;
        // N s_120_2: jump b121
        return block_121(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_121_0: read-var gs#112187:u8
        let s_121_0: bool = fn_state.gs_112187;
        // N s_121_1: branch s_121_0 b181 b122
        if s_121_0 {
            return block_181(state, tracer, fn_state);
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
        // D s_122_1: write-var gs#112188 <= s_122_0
        fn_state.gs_112188 = s_122_0;
        // N s_122_2: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_123_0: read-var gs#112188:u8
        let s_123_0: bool = fn_state.gs_112188;
        // N s_123_1: branch s_123_0 b180 b124
        if s_123_0 {
            return block_180(state, tracer, fn_state);
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
        // N s_124_2: branch s_124_1 b179 b125
        if s_124_1 {
            return block_179(state, tracer, fn_state);
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
        // D s_125_1: write-var gs#112189 <= s_125_0
        fn_state.gs_112189 = s_125_0;
        // N s_125_2: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_126_0: read-var gs#112189:u8
        let s_126_0: bool = fn_state.gs_112189;
        // N s_126_1: branch s_126_0 b178 b127
        if s_126_0 {
            return block_178(state, tracer, fn_state);
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
        // D s_127_1: write-var gs#112190 <= s_127_0
        fn_state.gs_112190 = s_127_0;
        // N s_127_2: jump b128
        return block_128(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_128_0: read-var gs#112190:u8
        let s_128_0: bool = fn_state.gs_112190;
        // N s_128_1: branch s_128_0 b177 b129
        if s_128_0 {
            return block_177(state, tracer, fn_state);
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
        // N s_129_2: branch s_129_1 b176 b130
        if s_129_1 {
            return block_176(state, tracer, fn_state);
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
        // D s_130_1: write-var gs#112191 <= s_130_0
        fn_state.gs_112191 = s_130_0;
        // N s_130_2: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_131_0: read-var gs#112191:u8
        let s_131_0: bool = fn_state.gs_112191;
        // N s_131_1: branch s_131_0 b175 b132
        if s_131_0 {
            return block_175(state, tracer, fn_state);
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
        // D s_132_1: write-var gs#112192 <= s_132_0
        fn_state.gs_112192 = s_132_0;
        // N s_132_2: jump b133
        return block_133(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_133_0: read-var gs#112192:u8
        let s_133_0: bool = fn_state.gs_112192;
        // N s_133_1: branch s_133_0 b174 b134
        if s_133_0 {
            return block_174(state, tracer, fn_state);
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
        // N s_134_2: branch s_134_1 b173 b135
        if s_134_1 {
            return block_173(state, tracer, fn_state);
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
        // D s_135_1: write-var gs#112193 <= s_135_0
        fn_state.gs_112193 = s_135_0;
        // N s_135_2: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_136_0: read-var gs#112193:u8
        let s_136_0: bool = fn_state.gs_112193;
        // N s_136_1: branch s_136_0 b172 b137
        if s_136_0 {
            return block_172(state, tracer, fn_state);
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
        // D s_137_1: write-var gs#112194 <= s_137_0
        fn_state.gs_112194 = s_137_0;
        // N s_137_2: jump b138
        return block_138(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_138_0: read-var gs#112194:u8
        let s_138_0: bool = fn_state.gs_112194;
        // N s_138_1: branch s_138_0 b171 b139
        if s_138_0 {
            return block_171(state, tracer, fn_state);
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
        // N s_139_4: branch s_139_3 b170 b140
        if s_139_3 {
            return block_170(state, tracer, fn_state);
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
        // D s_140_1: write-var gs#112195 <= s_140_0
        fn_state.gs_112195 = s_140_0;
        // N s_140_2: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_141_0: read-var gs#112195:u8
        let s_141_0: bool = fn_state.gs_112195;
        // N s_141_1: branch s_141_0 b169 b142
        if s_141_0 {
            return block_169(state, tracer, fn_state);
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
        // D s_142_1: write-var gs#112196 <= s_142_0
        fn_state.gs_112196 = s_142_0;
        // N s_142_2: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_143_0: read-var gs#112196:u8
        let s_143_0: bool = fn_state.gs_112196;
        // N s_143_1: branch s_143_0 b163 b144
        if s_143_0 {
            return block_163(state, tracer, fn_state);
        } else {
            return block_144(state, tracer, fn_state);
        };
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
        // N s_144_4: branch s_144_3 b162 b145
        if s_144_3 {
            return block_162(state, tracer, fn_state);
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
        // D s_145_1: write-var gs#112197 <= s_145_0
        fn_state.gs_112197 = s_145_0;
        // N s_145_2: jump b146
        return block_146(state, tracer, fn_state);
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_146_0: read-var gs#112197:u8
        let s_146_0: bool = fn_state.gs_112197;
        // N s_146_1: branch s_146_0 b161 b147
        if s_146_0 {
            return block_161(state, tracer, fn_state);
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
        // D s_147_1: write-var gs#112198 <= s_147_0
        fn_state.gs_112198 = s_147_0;
        // N s_147_2: jump b148
        return block_148(state, tracer, fn_state);
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_148_0: read-var gs#112198:u8
        let s_148_0: bool = fn_state.gs_112198;
        // N s_148_1: branch s_148_0 b160 b149
        if s_148_0 {
            return block_160(state, tracer, fn_state);
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
        // D s_149_1: write-var gs#112199 <= s_149_0
        fn_state.gs_112199 = s_149_0;
        // N s_149_2: jump b150
        return block_150(state, tracer, fn_state);
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_150_0: read-var gs#112199:u8
        let s_150_0: bool = fn_state.gs_112199;
        // N s_150_1: branch s_150_0 b154 b151
        if s_150_0 {
            return block_154(state, tracer, fn_state);
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
        // N s_151_4: branch s_151_3 b153 b152
        if s_151_3 {
            return block_153(state, tracer, fn_state);
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
        // S s_152_1: call ICC_CTLR_read(s_152_0)
        let s_152_1: ProductType700c18a878c5601b = ICC_CTLR_read(state, tracer, s_152_0);
        // S s_152_2: call __get_ICC_CTLR(s_152_1)
        let s_152_2: ProductType700c18a878c5601b = u__get_ICC_CTLR(
            state,
            tracer,
            s_152_1,
        );
        // D s_152_3: write-var ga#182675 <= s_152_2
        fn_state.ga_182675 = s_152_2;
        // D s_152_4: read-var ga#182675.0:struct
        let s_152_4: u32 = fn_state.ga_182675._0;
        // D s_152_5: read-var t:i
        let s_152_5: i128 = fn_state.t;
        // D s_152_6: call R_set(s_152_5, s_152_4)
        let s_152_6: () = R_set(state, tracer, s_152_5, s_152_4);
        // N s_152_7: return
        return;
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_153_0: const #() : ()
        let s_153_0: () = ();
        // S s_153_1: call ICC_CTLR_NS_read(s_153_0)
        let s_153_1: ProductType700c18a878c5601b = ICC_CTLR_NS_read(
            state,
            tracer,
            s_153_0,
        );
        // S s_153_2: call __get_ICC_CTLR_NS(s_153_1)
        let s_153_2: ProductType700c18a878c5601b = u__get_ICC_CTLR_NS(
            state,
            tracer,
            s_153_1,
        );
        // D s_153_3: write-var ga#182672 <= s_153_2
        fn_state.ga_182672 = s_153_2;
        // D s_153_4: read-var ga#182672.0:struct
        let s_153_4: u32 = fn_state.ga_182672._0;
        // D s_153_5: read-var t:i
        let s_153_5: i128 = fn_state.t;
        // D s_153_6: call R_set(s_153_5, s_153_4)
        let s_153_6: () = R_set(state, tracer, s_153_5, s_153_4);
        // N s_153_7: return
        return;
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_154_0: const #() : ()
        let s_154_0: () = ();
        // S s_154_1: call Halted(s_154_0)
        let s_154_1: bool = Halted(state, tracer, s_154_0);
        // N s_154_2: branch s_154_1 b159 b155
        if s_154_1 {
            return block_159(state, tracer, fn_state);
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
        // D s_155_1: write-var gs#112200 <= s_155_0
        fn_state.gs_112200 = s_155_0;
        // N s_155_2: jump b156
        return block_156(state, tracer, fn_state);
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_156_0: read-var gs#112200:u8
        let s_156_0: bool = fn_state.gs_112200;
        // N s_156_1: branch s_156_0 b158 b157
        if s_156_0 {
            return block_158(state, tracer, fn_state);
        } else {
            return block_157(state, tracer, fn_state);
        };
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_157_0: const #() : ()
        let s_157_0: () = ();
        // S s_157_1: call AArch32_TakeMonitorTrapException(s_157_0)
        let s_157_1: () = AArch32_TakeMonitorTrapException(state, tracer, s_157_0);
        // N s_157_2: return
        return;
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_158_0: panic
        panic!("{:?}", ());
        // N s_158_1: return
        return;
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_159_0: const #() : ()
        let s_159_0: () = ();
        // S s_159_1: call EDSCR_read(s_159_0)
        let s_159_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_159_0);
        // S s_159_2: call _get_EDSCR_Type_SDD(s_159_1)
        let s_159_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_159_1);
        // S s_159_3: cast zx s_159_2 -> bv
        let s_159_3: Bits = Bits::new(s_159_2 as u128, 1u16);
        // C s_159_4: const #1u : u8
        let s_159_4: bool = true;
        // C s_159_5: cast zx s_159_4 -> bv
        let s_159_5: Bits = Bits::new(s_159_4 as u128, 1u16);
        // S s_159_6: cmp-eq s_159_3 s_159_5
        let s_159_6: bool = ((s_159_3) == (s_159_5));
        // D s_159_7: write-var gs#112200 <= s_159_6
        fn_state.gs_112200 = s_159_6;
        // N s_159_8: jump b156
        return block_156(state, tracer, fn_state);
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_160_0: const #20920u : u32
        let s_160_0: u32 = 20920;
        // D s_160_1: read-reg s_160_0:struct
        let s_160_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_160_0 as isize);
            tracer.read_register(s_160_0 as isize, value);
            value
        };
        // D s_160_2: call _get_SCR_Type_IRQ(s_160_1)
        let s_160_2: bool = u_get_SCR_Type_IRQ(state, tracer, s_160_1);
        // C s_160_3: const #20920u : u32
        let s_160_3: u32 = 20920;
        // D s_160_4: read-reg s_160_3:struct
        let s_160_4: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_160_3 as isize);
            tracer.read_register(s_160_3 as isize, value);
            value
        };
        // D s_160_5: call _get_SCR_Type_FIQ(s_160_4)
        let s_160_5: bool = u_get_SCR_Type_FIQ(state, tracer, s_160_4);
        // D s_160_6: cast zx s_160_2 -> bv
        let s_160_6: Bits = Bits::new(s_160_2 as u128, 1u16);
        // D s_160_7: cast zx s_160_5 -> bv
        let s_160_7: Bits = Bits::new(s_160_5 as u128, 1u16);
        // D s_160_8: cast reint s_160_6 -> u128
        let s_160_8: u128 = (s_160_6.value() as u128);
        // D s_160_9: size-of s_160_6
        let s_160_9: u16 = s_160_6.length();
        // D s_160_10: cast reint s_160_7 -> u128
        let s_160_10: u128 = (s_160_7.value() as u128);
        // D s_160_11: size-of s_160_7
        let s_160_11: u16 = s_160_7.length();
        // D s_160_12: lsl s_160_8 s_160_11
        let s_160_12: u128 = s_160_8 << s_160_11;
        // D s_160_13: or s_160_12 s_160_10
        let s_160_13: u128 = ((s_160_12) | (s_160_10));
        // D s_160_14: add s_160_9 s_160_11
        let s_160_14: u16 = (s_160_9 + s_160_11);
        // D s_160_15: create-bits s_160_13 s_160_14
        let s_160_15: Bits = Bits::new(s_160_13, s_160_14);
        // D s_160_16: cast reint s_160_15 -> u8
        let s_160_16: u8 = (s_160_15.value() as u8);
        // D s_160_17: cast zx s_160_16 -> bv
        let s_160_17: Bits = Bits::new(s_160_16 as u128, 2u16);
        // C s_160_18: const #3u : u8
        let s_160_18: u8 = 3;
        // C s_160_19: cast zx s_160_18 -> bv
        let s_160_19: Bits = Bits::new(s_160_18 as u128, 2u16);
        // D s_160_20: cmp-eq s_160_17 s_160_19
        let s_160_20: bool = ((s_160_17) == (s_160_19));
        // D s_160_21: write-var gs#112199 <= s_160_20
        fn_state.gs_112199 = s_160_20;
        // N s_160_22: jump b150
        return block_150(state, tracer, fn_state);
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_161_0: read-var __PSTATE_M:u8
        let s_161_0: u8 = fn_state.u__PSTATE_M;
        // D s_161_1: cast zx s_161_0 -> bv
        let s_161_1: Bits = Bits::new(s_161_0 as u128, 5u16);
        // C s_161_2: const #384u : u32
        let s_161_2: u32 = 384;
        // D s_161_3: read-reg s_161_2:u8
        let s_161_3: u8 = {
            let value = state.read_register::<u8>(s_161_2 as isize);
            tracer.read_register(s_161_2 as isize, value);
            value
        };
        // D s_161_4: cast zx s_161_3 -> bv
        let s_161_4: Bits = Bits::new(s_161_3 as u128, 5u16);
        // D s_161_5: cmp-ne s_161_1 s_161_4
        let s_161_5: bool = ((s_161_1) != (s_161_4));
        // D s_161_6: write-var gs#112198 <= s_161_5
        fn_state.gs_112198 = s_161_5;
        // N s_161_7: jump b148
        return block_148(state, tracer, fn_state);
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_162_0: const #424u : u32
        let s_162_0: u32 = 424;
        // D s_162_1: read-reg s_162_0:u8
        let s_162_1: u8 = {
            let value = state.read_register::<u8>(s_162_0 as isize);
            tracer.read_register(s_162_0 as isize, value);
            value
        };
        // D s_162_2: call ELUsingAArch32(s_162_1)
        let s_162_2: bool = ELUsingAArch32(state, tracer, s_162_1);
        // D s_162_3: write-var gs#112197 <= s_162_2
        fn_state.gs_112197 = s_162_2;
        // N s_162_4: jump b146
        return block_146(state, tracer, fn_state);
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_163_0: const #() : ()
        let s_163_0: () = ();
        // S s_163_1: call Halted(s_163_0)
        let s_163_1: bool = Halted(state, tracer, s_163_0);
        // N s_163_2: branch s_163_1 b168 b164
        if s_163_1 {
            return block_168(state, tracer, fn_state);
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
        // D s_164_1: write-var gs#112201 <= s_164_0
        fn_state.gs_112201 = s_164_0;
        // N s_164_2: jump b165
        return block_165(state, tracer, fn_state);
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_165_0: read-var gs#112201:u8
        let s_165_0: bool = fn_state.gs_112201;
        // N s_165_1: branch s_165_0 b167 b166
        if s_165_0 {
            return block_167(state, tracer, fn_state);
        } else {
            return block_166(state, tracer, fn_state);
        };
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_166_0: const #3u : u8
        let s_166_0: u8 = 3;
        // C s_166_1: cast zx s_166_0 -> bv
        let s_166_1: Bits = Bits::new(s_166_0 as u128, 8u16);
        // C s_166_2: cast zx s_166_1 -> i
        let s_166_2: i128 = (s_166_1.value() as i128);
        // C s_166_3: cast reint s_166_2 -> i64
        let s_166_3: i64 = (s_166_2 as i64);
        // C s_166_4: cast zx s_166_3 -> i
        let s_166_4: i128 = (i128::try_from(s_166_3).unwrap());
        // C s_166_5: const #424u : u32
        let s_166_5: u32 = 424;
        // D s_166_6: read-reg s_166_5:u8
        let s_166_6: u8 = {
            let value = state.read_register::<u8>(s_166_5 as isize);
            tracer.read_register(s_166_5 as isize, value);
            value
        };
        // D s_166_7: call AArch64_AArch32SystemAccessTrap(s_166_6, s_166_4)
        let s_166_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_166_6,
            s_166_4,
        );
        // N s_166_8: return
        return;
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_167_0: panic
        panic!("{:?}", ());
        // N s_167_1: return
        return;
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_168_0: const #() : ()
        let s_168_0: () = ();
        // S s_168_1: call EDSCR_read(s_168_0)
        let s_168_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_168_0);
        // S s_168_2: call _get_EDSCR_Type_SDD(s_168_1)
        let s_168_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_168_1);
        // S s_168_3: cast zx s_168_2 -> bv
        let s_168_3: Bits = Bits::new(s_168_2 as u128, 1u16);
        // C s_168_4: const #1u : u8
        let s_168_4: bool = true;
        // C s_168_5: cast zx s_168_4 -> bv
        let s_168_5: Bits = Bits::new(s_168_4 as u128, 1u16);
        // S s_168_6: cmp-eq s_168_3 s_168_5
        let s_168_6: bool = ((s_168_3) == (s_168_5));
        // D s_168_7: write-var gs#112201 <= s_168_6
        fn_state.gs_112201 = s_168_6;
        // N s_168_8: jump b165
        return block_165(state, tracer, fn_state);
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_169_0: const #90704u : u32
        let s_169_0: u32 = 90704;
        // D s_169_1: read-reg s_169_0:struct
        let s_169_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_169_0 as isize);
            tracer.read_register(s_169_0 as isize, value);
            value
        };
        // D s_169_2: call _get_SCR_EL3_Type_IRQ(s_169_1)
        let s_169_2: bool = u_get_SCR_EL3_Type_IRQ(state, tracer, s_169_1);
        // C s_169_3: const #90704u : u32
        let s_169_3: u32 = 90704;
        // D s_169_4: read-reg s_169_3:struct
        let s_169_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_169_3 as isize);
            tracer.read_register(s_169_3 as isize, value);
            value
        };
        // D s_169_5: call _get_SCR_EL3_Type_FIQ(s_169_4)
        let s_169_5: bool = u_get_SCR_EL3_Type_FIQ(state, tracer, s_169_4);
        // D s_169_6: cast zx s_169_2 -> bv
        let s_169_6: Bits = Bits::new(s_169_2 as u128, 1u16);
        // D s_169_7: cast zx s_169_5 -> bv
        let s_169_7: Bits = Bits::new(s_169_5 as u128, 1u16);
        // D s_169_8: cast reint s_169_6 -> u128
        let s_169_8: u128 = (s_169_6.value() as u128);
        // D s_169_9: size-of s_169_6
        let s_169_9: u16 = s_169_6.length();
        // D s_169_10: cast reint s_169_7 -> u128
        let s_169_10: u128 = (s_169_7.value() as u128);
        // D s_169_11: size-of s_169_7
        let s_169_11: u16 = s_169_7.length();
        // D s_169_12: lsl s_169_8 s_169_11
        let s_169_12: u128 = s_169_8 << s_169_11;
        // D s_169_13: or s_169_12 s_169_10
        let s_169_13: u128 = ((s_169_12) | (s_169_10));
        // D s_169_14: add s_169_9 s_169_11
        let s_169_14: u16 = (s_169_9 + s_169_11);
        // D s_169_15: create-bits s_169_13 s_169_14
        let s_169_15: Bits = Bits::new(s_169_13, s_169_14);
        // D s_169_16: cast reint s_169_15 -> u8
        let s_169_16: u8 = (s_169_15.value() as u8);
        // D s_169_17: cast zx s_169_16 -> bv
        let s_169_17: Bits = Bits::new(s_169_16 as u128, 2u16);
        // C s_169_18: const #3u : u8
        let s_169_18: u8 = 3;
        // C s_169_19: cast zx s_169_18 -> bv
        let s_169_19: Bits = Bits::new(s_169_18 as u128, 2u16);
        // D s_169_20: cmp-eq s_169_17 s_169_19
        let s_169_20: bool = ((s_169_17) == (s_169_19));
        // D s_169_21: write-var gs#112196 <= s_169_20
        fn_state.gs_112196 = s_169_20;
        // N s_169_22: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_170_0: const #424u : u32
        let s_170_0: u32 = 424;
        // D s_170_1: read-reg s_170_0:u8
        let s_170_1: u8 = {
            let value = state.read_register::<u8>(s_170_0 as isize);
            tracer.read_register(s_170_0 as isize, value);
            value
        };
        // D s_170_2: call ELUsingAArch32(s_170_1)
        let s_170_2: bool = ELUsingAArch32(state, tracer, s_170_1);
        // D s_170_3: not s_170_2
        let s_170_3: bool = !s_170_2;
        // D s_170_4: write-var gs#112195 <= s_170_3
        fn_state.gs_112195 = s_170_3;
        // N s_170_5: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_171_0: const #() : ()
        let s_171_0: () = ();
        // S s_171_1: call ICV_CTLR_read(s_171_0)
        let s_171_1: ProductType700c18a878c5601b = ICV_CTLR_read(state, tracer, s_171_0);
        // S s_171_2: call __get_ICV_CTLR(s_171_1)
        let s_171_2: ProductType700c18a878c5601b = u__get_ICV_CTLR(
            state,
            tracer,
            s_171_1,
        );
        // D s_171_3: write-var ga#182643 <= s_171_2
        fn_state.ga_182643 = s_171_2;
        // D s_171_4: read-var ga#182643.0:struct
        let s_171_4: u32 = fn_state.ga_182643._0;
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
        // D s_172_0: read-var __HCR_IMO:u8
        let s_172_0: bool = fn_state.u__HCR_IMO;
        // D s_172_1: cast zx s_172_0 -> bv
        let s_172_1: Bits = Bits::new(s_172_0 as u128, 1u16);
        // C s_172_2: const #1u : u8
        let s_172_2: bool = true;
        // C s_172_3: cast zx s_172_2 -> bv
        let s_172_3: Bits = Bits::new(s_172_2 as u128, 1u16);
        // D s_172_4: cmp-eq s_172_1 s_172_3
        let s_172_4: bool = ((s_172_1) == (s_172_3));
        // D s_172_5: write-var gs#112194 <= s_172_4
        fn_state.gs_112194 = s_172_4;
        // N s_172_6: jump b138
        return block_138(state, tracer, fn_state);
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
        // D s_173_3: write-var gs#112193 <= s_173_2
        fn_state.gs_112193 = s_173_2;
        // N s_173_4: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_174_0: const #() : ()
        let s_174_0: () = ();
        // S s_174_1: call ICV_CTLR_read(s_174_0)
        let s_174_1: ProductType700c18a878c5601b = ICV_CTLR_read(state, tracer, s_174_0);
        // S s_174_2: call __get_ICV_CTLR(s_174_1)
        let s_174_2: ProductType700c18a878c5601b = u__get_ICV_CTLR(
            state,
            tracer,
            s_174_1,
        );
        // D s_174_3: write-var ga#182637 <= s_174_2
        fn_state.ga_182637 = s_174_2;
        // D s_174_4: read-var ga#182637.0:struct
        let s_174_4: u32 = fn_state.ga_182637._0;
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
        // D s_175_0: read-var __HCR_FMO:u8
        let s_175_0: bool = fn_state.u__HCR_FMO;
        // D s_175_1: cast zx s_175_0 -> bv
        let s_175_1: Bits = Bits::new(s_175_0 as u128, 1u16);
        // C s_175_2: const #1u : u8
        let s_175_2: bool = true;
        // C s_175_3: cast zx s_175_2 -> bv
        let s_175_3: Bits = Bits::new(s_175_2 as u128, 1u16);
        // D s_175_4: cmp-eq s_175_1 s_175_3
        let s_175_4: bool = ((s_175_1) == (s_175_3));
        // D s_175_5: write-var gs#112192 <= s_175_4
        fn_state.gs_112192 = s_175_4;
        // N s_175_6: jump b133
        return block_133(state, tracer, fn_state);
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
        // D s_176_3: write-var gs#112191 <= s_176_2
        fn_state.gs_112191 = s_176_2;
        // N s_176_4: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_177_0: const #() : ()
        let s_177_0: () = ();
        // S s_177_1: call ICV_CTLR_read(s_177_0)
        let s_177_1: ProductType700c18a878c5601b = ICV_CTLR_read(state, tracer, s_177_0);
        // S s_177_2: call __get_ICV_CTLR(s_177_1)
        let s_177_2: ProductType700c18a878c5601b = u__get_ICV_CTLR(
            state,
            tracer,
            s_177_1,
        );
        // D s_177_3: write-var ga#182631 <= s_177_2
        fn_state.ga_182631 = s_177_2;
        // D s_177_4: read-var ga#182631.0:struct
        let s_177_4: u32 = fn_state.ga_182631._0;
        // D s_177_5: read-var t:i
        let s_177_5: i128 = fn_state.t;
        // D s_177_6: call R_set(s_177_5, s_177_4)
        let s_177_6: () = R_set(state, tracer, s_177_5, s_177_4);
        // N s_177_7: return
        return;
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_178_0: read-var __HCR_EL2_IMO:u8
        let s_178_0: bool = fn_state.u__HCR_EL2_IMO;
        // D s_178_1: cast zx s_178_0 -> bv
        let s_178_1: Bits = Bits::new(s_178_0 as u128, 1u16);
        // C s_178_2: const #1u : u8
        let s_178_2: bool = true;
        // C s_178_3: cast zx s_178_2 -> bv
        let s_178_3: Bits = Bits::new(s_178_2 as u128, 1u16);
        // D s_178_4: cmp-eq s_178_1 s_178_3
        let s_178_4: bool = ((s_178_1) == (s_178_3));
        // D s_178_5: write-var gs#112190 <= s_178_4
        fn_state.gs_112190 = s_178_4;
        // N s_178_6: jump b128
        return block_128(state, tracer, fn_state);
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
        // D s_179_4: write-var gs#112189 <= s_179_3
        fn_state.gs_112189 = s_179_3;
        // N s_179_5: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_180_0: const #() : ()
        let s_180_0: () = ();
        // S s_180_1: call ICV_CTLR_read(s_180_0)
        let s_180_1: ProductType700c18a878c5601b = ICV_CTLR_read(state, tracer, s_180_0);
        // S s_180_2: call __get_ICV_CTLR(s_180_1)
        let s_180_2: ProductType700c18a878c5601b = u__get_ICV_CTLR(
            state,
            tracer,
            s_180_1,
        );
        // D s_180_3: write-var ga#182624 <= s_180_2
        fn_state.ga_182624 = s_180_2;
        // D s_180_4: read-var ga#182624.0:struct
        let s_180_4: u32 = fn_state.ga_182624._0;
        // D s_180_5: read-var t:i
        let s_180_5: i128 = fn_state.t;
        // D s_180_6: call R_set(s_180_5, s_180_4)
        let s_180_6: () = R_set(state, tracer, s_180_5, s_180_4);
        // N s_180_7: return
        return;
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_181_0: read-var __HCR_EL2_FMO:u8
        let s_181_0: bool = fn_state.u__HCR_EL2_FMO;
        // D s_181_1: cast zx s_181_0 -> bv
        let s_181_1: Bits = Bits::new(s_181_0 as u128, 1u16);
        // C s_181_2: const #1u : u8
        let s_181_2: bool = true;
        // C s_181_3: cast zx s_181_2 -> bv
        let s_181_3: Bits = Bits::new(s_181_2 as u128, 1u16);
        // D s_181_4: cmp-eq s_181_1 s_181_3
        let s_181_4: bool = ((s_181_1) == (s_181_3));
        // D s_181_5: write-var gs#112188 <= s_181_4
        fn_state.gs_112188 = s_181_4;
        // N s_181_6: jump b123
        return block_123(state, tracer, fn_state);
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
        // D s_182_4: write-var gs#112187 <= s_182_3
        fn_state.gs_112187 = s_182_3;
        // N s_182_5: jump b121
        return block_121(state, tracer, fn_state);
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
        // D s_184_0: read-var __ICH_HCR_TC:u8
        let s_184_0: bool = fn_state.u__ICH_HCR_TC;
        // D s_184_1: cast zx s_184_0 -> bv
        let s_184_1: Bits = Bits::new(s_184_0 as u128, 1u16);
        // C s_184_2: const #1u : u8
        let s_184_2: bool = true;
        // C s_184_3: cast zx s_184_2 -> bv
        let s_184_3: Bits = Bits::new(s_184_2 as u128, 1u16);
        // D s_184_4: cmp-eq s_184_1 s_184_3
        let s_184_4: bool = ((s_184_1) == (s_184_3));
        // D s_184_5: write-var gs#112186 <= s_184_4
        fn_state.gs_112186 = s_184_4;
        // N s_184_6: jump b118
        return block_118(state, tracer, fn_state);
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
        // D s_185_3: write-var gs#112185 <= s_185_2
        fn_state.gs_112185 = s_185_2;
        // N s_185_4: jump b116
        return block_116(state, tracer, fn_state);
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
        // D s_187_0: read-var __ICH_HCR_EL2_TC:u8
        let s_187_0: bool = fn_state.u__ICH_HCR_EL2_TC;
        // D s_187_1: cast zx s_187_0 -> bv
        let s_187_1: Bits = Bits::new(s_187_0 as u128, 1u16);
        // C s_187_2: const #1u : u8
        let s_187_2: bool = true;
        // C s_187_3: cast zx s_187_2 -> bv
        let s_187_3: Bits = Bits::new(s_187_2 as u128, 1u16);
        // D s_187_4: cmp-eq s_187_1 s_187_3
        let s_187_4: bool = ((s_187_1) == (s_187_3));
        // D s_187_5: write-var gs#112184 <= s_187_4
        fn_state.gs_112184 = s_187_4;
        // N s_187_6: jump b113
        return block_113(state, tracer, fn_state);
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
        // D s_188_4: write-var gs#112183 <= s_188_3
        fn_state.gs_112183 = s_188_3;
        // N s_188_5: jump b111
        return block_111(state, tracer, fn_state);
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
        // S s_189_5: call AArch32_TakeHypTrapException(s_189_4)
        let s_189_5: () = AArch32_TakeHypTrapException(state, tracer, s_189_4);
        // N s_189_6: return
        return;
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_190_0: read-var __HSTR_T12:u8
        let s_190_0: bool = fn_state.u__HSTR_T12;
        // D s_190_1: cast zx s_190_0 -> bv
        let s_190_1: Bits = Bits::new(s_190_0 as u128, 1u16);
        // C s_190_2: const #1u : u8
        let s_190_2: bool = true;
        // C s_190_3: cast zx s_190_2 -> bv
        let s_190_3: Bits = Bits::new(s_190_2 as u128, 1u16);
        // D s_190_4: cmp-eq s_190_1 s_190_3
        let s_190_4: bool = ((s_190_1) == (s_190_3));
        // D s_190_5: write-var gs#112182 <= s_190_4
        fn_state.gs_112182 = s_190_4;
        // N s_190_6: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_191_0: const #432u : u32
        let s_191_0: u32 = 432;
        // D s_191_1: read-reg s_191_0:u8
        let s_191_1: u8 = {
            let value = state.read_register::<u8>(s_191_0 as isize);
            tracer.read_register(s_191_0 as isize, value);
            value
        };
        // D s_191_2: call ELUsingAArch32(s_191_1)
        let s_191_2: bool = ELUsingAArch32(state, tracer, s_191_1);
        // D s_191_3: write-var gs#112181 <= s_191_2
        fn_state.gs_112181 = s_191_2;
        // N s_191_4: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_192_0: const #3u : u8
        let s_192_0: u8 = 3;
        // C s_192_1: cast zx s_192_0 -> bv
        let s_192_1: Bits = Bits::new(s_192_0 as u128, 8u16);
        // C s_192_2: cast zx s_192_1 -> i
        let s_192_2: i128 = (s_192_1.value() as i128);
        // C s_192_3: cast reint s_192_2 -> i64
        let s_192_3: i64 = (s_192_2 as i64);
        // C s_192_4: cast zx s_192_3 -> i
        let s_192_4: i128 = (i128::try_from(s_192_3).unwrap());
        // C s_192_5: const #432u : u32
        let s_192_5: u32 = 432;
        // D s_192_6: read-reg s_192_5:u8
        let s_192_6: u8 = {
            let value = state.read_register::<u8>(s_192_5 as isize);
            tracer.read_register(s_192_5 as isize, value);
            value
        };
        // D s_192_7: call AArch64_AArch32SystemAccessTrap(s_192_6, s_192_4)
        let s_192_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_192_6,
            s_192_4,
        );
        // N s_192_8: return
        return;
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_193_0: read-var __HSTR_EL2_T12:u8
        let s_193_0: bool = fn_state.u__HSTR_EL2_T12;
        // D s_193_1: cast zx s_193_0 -> bv
        let s_193_1: Bits = Bits::new(s_193_0 as u128, 1u16);
        // C s_193_2: const #1u : u8
        let s_193_2: bool = true;
        // C s_193_3: cast zx s_193_2 -> bv
        let s_193_3: Bits = Bits::new(s_193_2 as u128, 1u16);
        // D s_193_4: cmp-eq s_193_1 s_193_3
        let s_193_4: bool = ((s_193_1) == (s_193_3));
        // D s_193_5: write-var gs#112180 <= s_193_4
        fn_state.gs_112180 = s_193_4;
        // N s_193_6: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_194_0: const #432u : u32
        let s_194_0: u32 = 432;
        // D s_194_1: read-reg s_194_0:u8
        let s_194_1: u8 = {
            let value = state.read_register::<u8>(s_194_0 as isize);
            tracer.read_register(s_194_0 as isize, value);
            value
        };
        // D s_194_2: call ELUsingAArch32(s_194_1)
        let s_194_2: bool = ELUsingAArch32(state, tracer, s_194_1);
        // D s_194_3: not s_194_2
        let s_194_3: bool = !s_194_2;
        // D s_194_4: write-var gs#112179 <= s_194_3
        fn_state.gs_112179 = s_194_3;
        // N s_194_5: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_195<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_195_0: panic
        panic!("{:?}", ());
        // N s_195_1: return
        return;
    }
    fn block_196<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_196_0: const #20920u : u32
        let s_196_0: u32 = 20920;
        // D s_196_1: read-reg s_196_0:struct
        let s_196_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_196_0 as isize);
            tracer.read_register(s_196_0 as isize, value);
            value
        };
        // D s_196_2: call _get_SCR_Type_IRQ(s_196_1)
        let s_196_2: bool = u_get_SCR_Type_IRQ(state, tracer, s_196_1);
        // C s_196_3: const #20920u : u32
        let s_196_3: u32 = 20920;
        // D s_196_4: read-reg s_196_3:struct
        let s_196_4: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_196_3 as isize);
            tracer.read_register(s_196_3 as isize, value);
            value
        };
        // D s_196_5: call _get_SCR_Type_FIQ(s_196_4)
        let s_196_5: bool = u_get_SCR_Type_FIQ(state, tracer, s_196_4);
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
        // D s_196_20: cmp-eq s_196_17 s_196_19
        let s_196_20: bool = ((s_196_17) == (s_196_19));
        // D s_196_21: write-var gs#112178 <= s_196_20
        fn_state.gs_112178 = s_196_20;
        // N s_196_22: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_197<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_197_0: read-var __PSTATE_M:u8
        let s_197_0: u8 = fn_state.u__PSTATE_M;
        // D s_197_1: cast zx s_197_0 -> bv
        let s_197_1: Bits = Bits::new(s_197_0 as u128, 5u16);
        // C s_197_2: const #384u : u32
        let s_197_2: u32 = 384;
        // D s_197_3: read-reg s_197_2:u8
        let s_197_3: u8 = {
            let value = state.read_register::<u8>(s_197_2 as isize);
            tracer.read_register(s_197_2 as isize, value);
            value
        };
        // D s_197_4: cast zx s_197_3 -> bv
        let s_197_4: Bits = Bits::new(s_197_3 as u128, 5u16);
        // D s_197_5: cmp-ne s_197_1 s_197_4
        let s_197_5: bool = ((s_197_1) != (s_197_4));
        // D s_197_6: write-var gs#112177 <= s_197_5
        fn_state.gs_112177 = s_197_5;
        // N s_197_7: jump b96
        return block_96(state, tracer, fn_state);
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
        // D s_198_3: write-var gs#112176 <= s_198_2
        fn_state.gs_112176 = s_198_2;
        // N s_198_4: jump b94
        return block_94(state, tracer, fn_state);
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
        // D s_199_2: write-var gs#112175 <= s_199_1
        fn_state.gs_112175 = s_199_1;
        // N s_199_3: jump b92
        return block_92(state, tracer, fn_state);
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
        // D s_200_7: write-var gs#112174 <= s_200_6
        fn_state.gs_112174 = s_200_6;
        // N s_200_8: jump b90
        return block_90(state, tracer, fn_state);
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
        // D s_201_4: write-var gs#112173 <= s_201_3
        fn_state.gs_112173 = s_201_3;
        // N s_201_5: jump b88
        return block_88(state, tracer, fn_state);
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
    fn block_203<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_203_0: const #90704u : u32
        let s_203_0: u32 = 90704;
        // D s_203_1: read-reg s_203_0:struct
        let s_203_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_203_0 as isize);
            tracer.read_register(s_203_0 as isize, value);
            value
        };
        // D s_203_2: call _get_SCR_EL3_Type_IRQ(s_203_1)
        let s_203_2: bool = u_get_SCR_EL3_Type_IRQ(state, tracer, s_203_1);
        // C s_203_3: const #90704u : u32
        let s_203_3: u32 = 90704;
        // D s_203_4: read-reg s_203_3:struct
        let s_203_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_203_3 as isize);
            tracer.read_register(s_203_3 as isize, value);
            value
        };
        // D s_203_5: call _get_SCR_EL3_Type_FIQ(s_203_4)
        let s_203_5: bool = u_get_SCR_EL3_Type_FIQ(state, tracer, s_203_4);
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
        // D s_203_20: cmp-eq s_203_17 s_203_19
        let s_203_20: bool = ((s_203_17) == (s_203_19));
        // D s_203_21: write-var gs#112172 <= s_203_20
        fn_state.gs_112172 = s_203_20;
        // N s_203_22: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_204<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_204_0: const #424u : u32
        let s_204_0: u32 = 424;
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
        // D s_204_4: write-var gs#112171 <= s_204_3
        fn_state.gs_112171 = s_204_3;
        // N s_204_5: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_205<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_205_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_205_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_205_1: call __IMPDEF_boolean(s_205_0)
        let s_205_1: bool = u__IMPDEF_boolean(state, tracer, s_205_0);
        // D s_205_2: write-var gs#112170 <= s_205_1
        fn_state.gs_112170 = s_205_1;
        // N s_205_3: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_206<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_206_0: const #() : ()
        let s_206_0: () = ();
        // S s_206_1: call EDSCR_read(s_206_0)
        let s_206_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_206_0);
        // S s_206_2: call _get_EDSCR_Type_SDD(s_206_1)
        let s_206_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_206_1);
        // S s_206_3: cast zx s_206_2 -> bv
        let s_206_3: Bits = Bits::new(s_206_2 as u128, 1u16);
        // C s_206_4: const #1u : u8
        let s_206_4: bool = true;
        // C s_206_5: cast zx s_206_4 -> bv
        let s_206_5: Bits = Bits::new(s_206_4 as u128, 1u16);
        // S s_206_6: cmp-eq s_206_3 s_206_5
        let s_206_6: bool = ((s_206_3) == (s_206_5));
        // D s_206_7: write-var gs#112169 <= s_206_6
        fn_state.gs_112169 = s_206_6;
        // N s_206_8: jump b79
        return block_79(state, tracer, fn_state);
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
        // D s_207_4: write-var gs#112168 <= s_207_3
        fn_state.gs_112168 = s_207_3;
        // N s_207_5: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_208<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_208_0: panic
        panic!("{:?}", ());
        // N s_208_1: return
        return;
    }
}
