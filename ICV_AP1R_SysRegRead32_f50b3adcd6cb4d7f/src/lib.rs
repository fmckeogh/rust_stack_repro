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
use ICH_HCR_read::*;
use AArch32_TakeHypTrapException::*;
use u_get_SCR_Type_IRQ::*;
use u_get_ICC_SRE_Type_SRE::*;
use Halted::*;
use ICC_HSRE_read::*;
use ICC_SRE_read::*;
use u_get_ICH_HCR_Type_TALL1::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_HCR_Type_IMO::*;
use EL2Enabled::*;
use u_get_HSTR_Type_T12::*;
use u_get_ICC_MSRE_Type_SRE::*;
use HCR_read::*;
use u_get_SCR_EL3_Type_IRQ::*;
use u_get_ICC_HSRE_Type_SRE::*;
use u__IMPDEF_boolean::*;
use u_get_HSTR_EL2_Type_T12::*;
use HSTR_read::*;
use u_get_SCR_Type_NS::*;
use AArch64_AArch32SystemAccessTrap::*;
use ICC_AP1R_read::*;
use ICC_AP1R_NS_read::*;
use u_get_ICH_HCR_EL2_Type_TALL1::*;
use R_set::*;
use ELUsingAArch32::*;
use ICV_AP1R_read::*;
use ICC_AP1R_S_read::*;
use u_get_HCR_EL2_Type_IMO::*;
use AArch32_TakeMonitorTrapException::*;
use EDSCR_read::*;
use common::*;
pub fn ICV_AP1R_SysRegRead32_f50b3adcd6cb4d7f<T: Tracer>(
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
        u__SCR_EL3_IRQ: bool,
        gs_112128: bool,
        gs_112123: bool,
        gs_112137: bool,
        u__HCR_EL2_IMO: bool,
        gs_112126: bool,
        gs_112130: bool,
        gs_112100: bool,
        gs_112119: bool,
        gs_112149: bool,
        u__SCR_IRQ: bool,
        gs_112133: bool,
        gs_112102: bool,
        gs_112131: bool,
        u__ICH_HCR_EL2_TALL1: bool,
        gs_112106: bool,
        gs_112134: bool,
        u__ICH_HCR_TALL1: bool,
        gs_112110: bool,
        gs_112113: bool,
        gs_112109: bool,
        gs_112143: bool,
        gs_112140: bool,
        u__HSTR_EL2_T12: bool,
        gs_112117: bool,
        gs_112145: bool,
        gs_112125: bool,
        gs_112112: bool,
        u__PSTATE_EL: u8,
        gs_112124: bool,
        gs_112136: bool,
        gs_112135: bool,
        gs_112148: bool,
        gs_112142: bool,
        gs_112138: bool,
        gs_112144: bool,
        u__ICC_HSRE_SRE: bool,
        u__HSTR_T12: bool,
        gs_112111: bool,
        u__HCR_IMO: bool,
        gs_112104: bool,
        gs_112127: bool,
        u__SCR_NS: bool,
        gs_112101: bool,
        u__ICC_MSRE_SRE: bool,
        gs_112129: bool,
        gs_112103: bool,
        gs_112121: bool,
        u__ICC_SRE_SRE: bool,
        u__PSTATE_M: u8,
        gs_112107: bool,
        gs_112105: bool,
        gs_112139: bool,
        gs_112116: bool,
        gs_112132: bool,
        gs_112120: bool,
        gs_112108: bool,
        gs_112122: bool,
        gs_112118: bool,
        gs_112141: bool,
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
        // C s_0_3: const #90704u : u32
        let s_0_3: u32 = 90704;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_SCR_EL3_Type_IRQ(s_0_4)
        let s_0_5: bool = u_get_SCR_EL3_Type_IRQ(state, tracer, s_0_4);
        // D s_0_6: write-var __SCR_EL3_IRQ <= s_0_5
        fn_state.u__SCR_EL3_IRQ = s_0_5;
        // C s_0_7: const #16983u : u32
        let s_0_7: u32 = 16983;
        // D s_0_8: read-reg s_0_7:u8
        let s_0_8: u8 = {
            let value = state.read_register::<u8>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: write-var __PSTATE_M <= s_0_8
        fn_state.u__PSTATE_M = s_0_8;
        // C s_0_10: const #20920u : u32
        let s_0_10: u32 = 20920;
        // D s_0_11: read-reg s_0_10:struct
        let s_0_11: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_10 as isize);
            tracer.read_register(s_0_10 as isize, value);
            value
        };
        // D s_0_12: call _get_SCR_Type_IRQ(s_0_11)
        let s_0_12: bool = u_get_SCR_Type_IRQ(state, tracer, s_0_11);
        // D s_0_13: write-var __SCR_IRQ <= s_0_12
        fn_state.u__SCR_IRQ = s_0_12;
        // C s_0_14: const #104936u : u32
        let s_0_14: u32 = 104936;
        // D s_0_15: read-reg s_0_14:struct
        let s_0_15: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_14 as isize);
            tracer.read_register(s_0_14 as isize, value);
            value
        };
        // D s_0_16: call _get_HSTR_EL2_Type_T12(s_0_15)
        let s_0_16: bool = u_get_HSTR_EL2_Type_T12(state, tracer, s_0_15);
        // D s_0_17: write-var __HSTR_EL2_T12 <= s_0_16
        fn_state.u__HSTR_EL2_T12 = s_0_16;
        // C s_0_18: const #() : ()
        let s_0_18: () = ();
        // S s_0_19: call HSTR_read(s_0_18)
        let s_0_19: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_18);
        // S s_0_20: call _get_HSTR_Type_T12(s_0_19)
        let s_0_20: bool = u_get_HSTR_Type_T12(state, tracer, s_0_19);
        // D s_0_21: write-var __HSTR_T12 <= s_0_20
        fn_state.u__HSTR_T12 = s_0_20;
        // C s_0_22: const #() : ()
        let s_0_22: () = ();
        // S s_0_23: call ICC_SRE_read(s_0_22)
        let s_0_23: ProductType700c18a878c5601b = ICC_SRE_read(state, tracer, s_0_22);
        // S s_0_24: call _get_ICC_SRE_Type_SRE(s_0_23)
        let s_0_24: bool = u_get_ICC_SRE_Type_SRE(state, tracer, s_0_23);
        // D s_0_25: write-var __ICC_SRE_SRE <= s_0_24
        fn_state.u__ICC_SRE_SRE = s_0_24;
        // C s_0_26: const #20992u : u32
        let s_0_26: u32 = 20992;
        // D s_0_27: read-reg s_0_26:struct
        let s_0_27: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_26 as isize);
            tracer.read_register(s_0_26 as isize, value);
            value
        };
        // D s_0_28: call _get_ICH_HCR_EL2_Type_TALL1(s_0_27)
        let s_0_28: bool = u_get_ICH_HCR_EL2_Type_TALL1(state, tracer, s_0_27);
        // D s_0_29: write-var __ICH_HCR_EL2_TALL1 <= s_0_28
        fn_state.u__ICH_HCR_EL2_TALL1 = s_0_28;
        // C s_0_30: const #() : ()
        let s_0_30: () = ();
        // S s_0_31: call ICH_HCR_read(s_0_30)
        let s_0_31: ProductType700c18a878c5601b = ICH_HCR_read(state, tracer, s_0_30);
        // S s_0_32: call _get_ICH_HCR_Type_TALL1(s_0_31)
        let s_0_32: bool = u_get_ICH_HCR_Type_TALL1(state, tracer, s_0_31);
        // D s_0_33: write-var __ICH_HCR_TALL1 <= s_0_32
        fn_state.u__ICH_HCR_TALL1 = s_0_32;
        // C s_0_34: const #102552u : u32
        let s_0_34: u32 = 102552;
        // D s_0_35: read-reg s_0_34:struct
        let s_0_35: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_34 as isize);
            tracer.read_register(s_0_34 as isize, value);
            value
        };
        // D s_0_36: call _get_HCR_EL2_Type_IMO(s_0_35)
        let s_0_36: bool = u_get_HCR_EL2_Type_IMO(state, tracer, s_0_35);
        // D s_0_37: write-var __HCR_EL2_IMO <= s_0_36
        fn_state.u__HCR_EL2_IMO = s_0_36;
        // C s_0_38: const #() : ()
        let s_0_38: () = ();
        // S s_0_39: call HCR_read(s_0_38)
        let s_0_39: ProductType700c18a878c5601b = HCR_read(state, tracer, s_0_38);
        // S s_0_40: call _get_HCR_Type_IMO(s_0_39)
        let s_0_40: bool = u_get_HCR_Type_IMO(state, tracer, s_0_39);
        // D s_0_41: write-var __HCR_IMO <= s_0_40
        fn_state.u__HCR_IMO = s_0_40;
        // C s_0_42: const #() : ()
        let s_0_42: () = ();
        // S s_0_43: call ICC_HSRE_read(s_0_42)
        let s_0_43: ProductType700c18a878c5601b = ICC_HSRE_read(state, tracer, s_0_42);
        // S s_0_44: call _get_ICC_HSRE_Type_SRE(s_0_43)
        let s_0_44: bool = u_get_ICC_HSRE_Type_SRE(state, tracer, s_0_43);
        // D s_0_45: write-var __ICC_HSRE_SRE <= s_0_44
        fn_state.u__ICC_HSRE_SRE = s_0_44;
        // C s_0_46: const #19992u : u32
        let s_0_46: u32 = 19992;
        // D s_0_47: read-reg s_0_46:struct
        let s_0_47: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_46 as isize);
            tracer.read_register(s_0_46 as isize, value);
            value
        };
        // D s_0_48: call _get_ICC_MSRE_Type_SRE(s_0_47)
        let s_0_48: bool = u_get_ICC_MSRE_Type_SRE(state, tracer, s_0_47);
        // D s_0_49: write-var __ICC_MSRE_SRE <= s_0_48
        fn_state.u__ICC_MSRE_SRE = s_0_48;
        // C s_0_50: const #20920u : u32
        let s_0_50: u32 = 20920;
        // D s_0_51: read-reg s_0_50:struct
        let s_0_51: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_50 as isize);
            tracer.read_register(s_0_50 as isize, value);
            value
        };
        // D s_0_52: call _get_SCR_Type_NS(s_0_51)
        let s_0_52: bool = u_get_SCR_Type_NS(state, tracer, s_0_51);
        // D s_0_53: write-var __SCR_NS <= s_0_52
        fn_state.u__SCR_NS = s_0_52;
        // N s_0_54: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
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
        // C s_2_2: const #448u : u32
        let s_2_2: u32 = 448;
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
        // N s_2_6: branch s_2_5 b196 b3
        if s_2_5 {
            return block_196(state, tracer, fn_state);
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
        // C s_3_2: const #440u : u32
        let s_3_2: u32 = 440;
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
        // N s_3_6: branch s_3_5 b77 b4
        if s_3_5 {
            return block_77(state, tracer, fn_state);
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
        // C s_4_2: const #432u : u32
        let s_4_2: u32 = 432;
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
        // N s_4_6: branch s_4_5 b12 b5
        if s_4_5 {
            return block_12(state, tracer, fn_state);
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
        // C s_5_2: const #424u : u32
        let s_5_2: u32 = 424;
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
        // N s_5_6: branch s_5_5 b7 b6
        if s_5_5 {
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
        // N s_6_0: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var __ICC_MSRE_SRE:u8
        let s_7_0: bool = fn_state.u__ICC_MSRE_SRE;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 1u16);
        // C s_7_2: const #0u : u8
        let s_7_2: bool = false;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 1u16);
        // D s_7_4: cmp-eq s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) == (s_7_3));
        // N s_7_5: branch s_7_4 b11 b8
        if s_7_4 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var __SCR_NS:u8
        let s_8_0: bool = fn_state.u__SCR_NS;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 1u16);
        // C s_8_2: const #0u : u8
        let s_8_2: bool = false;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 1u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // N s_8_5: branch s_8_4 b10 b9
        if s_8_4 {
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
        // C s_9_0: const #0s : i64
        let s_9_0: i64 = 0;
        // S s_9_1: call ICC_AP1R_NS_read(s_9_0)
        let s_9_1: u32 = ICC_AP1R_NS_read(state, tracer, s_9_0);
        // D s_9_2: read-var t:i
        let s_9_2: i128 = fn_state.t;
        // D s_9_3: call R_set(s_9_2, s_9_1)
        let s_9_3: () = R_set(state, tracer, s_9_2, s_9_1);
        // N s_9_4: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0s : i64
        let s_10_0: i64 = 0;
        // S s_10_1: call ICC_AP1R_S_read(s_10_0)
        let s_10_1: u32 = ICC_AP1R_S_read(state, tracer, s_10_0);
        // D s_10_2: read-var t:i
        let s_10_2: i128 = fn_state.t;
        // D s_10_3: call R_set(s_10_2, s_10_1)
        let s_10_3: () = R_set(state, tracer, s_10_2, s_10_1);
        // N s_10_4: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: panic
        panic!("{:?}", ());
        // N s_11_1: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call Halted(s_12_0)
        let s_12_1: bool = Halted(state, tracer, s_12_0);
        // N s_12_2: branch s_12_1 b76 b13
        if s_12_1 {
            return block_76(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#112100 <= s_13_0
        fn_state.gs_112100 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#112100:u8
        let s_14_0: bool = fn_state.gs_112100;
        // N s_14_1: branch s_14_0 b75 b15
        if s_14_0 {
            return block_75(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#112101 <= s_15_0
        fn_state.gs_112101 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#112101:u8
        let s_16_0: bool = fn_state.gs_112101;
        // N s_16_1: branch s_16_0 b74 b17
        if s_16_0 {
            return block_74(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#112102 <= s_17_0
        fn_state.gs_112102 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#112102:u8
        let s_18_0: bool = fn_state.gs_112102;
        // N s_18_1: branch s_18_0 b73 b19
        if s_18_0 {
            return block_73(state, tracer, fn_state);
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
        // D s_19_1: write-var gs#112103 <= s_19_0
        fn_state.gs_112103 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#112103:u8
        let s_20_0: bool = fn_state.gs_112103;
        // N s_20_1: branch s_20_0 b72 b21
        if s_20_0 {
            return block_72(state, tracer, fn_state);
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
        // D s_21_1: write-var gs#112104 <= s_21_0
        fn_state.gs_112104 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#112104:u8
        let s_22_0: bool = fn_state.gs_112104;
        // N s_22_1: branch s_22_0 b71 b23
        if s_22_0 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
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
        // N s_23_2: branch s_23_1 b70 b24
        if s_23_1 {
            return block_70(state, tracer, fn_state);
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
        // D s_24_1: write-var gs#112105 <= s_24_0
        fn_state.gs_112105 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#112105:u8
        let s_25_0: bool = fn_state.gs_112105;
        // N s_25_1: branch s_25_0 b69 b26
        if s_25_0 {
            return block_69(state, tracer, fn_state);
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
        // D s_26_1: write-var gs#112106 <= s_26_0
        fn_state.gs_112106 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#112106:u8
        let s_27_0: bool = fn_state.gs_112106;
        // N s_27_1: branch s_27_0 b68 b28
        if s_27_0 {
            return block_68(state, tracer, fn_state);
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
        // D s_28_1: write-var gs#112107 <= s_28_0
        fn_state.gs_112107 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#112107:u8
        let s_29_0: bool = fn_state.gs_112107;
        // N s_29_1: branch s_29_0 b67 b30
        if s_29_0 {
            return block_67(state, tracer, fn_state);
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
        // D s_30_1: write-var gs#112108 <= s_30_0
        fn_state.gs_112108 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#112108:u8
        let s_31_0: bool = fn_state.gs_112108;
        // N s_31_1: branch s_31_0 b66 b32
        if s_31_0 {
            return block_66(state, tracer, fn_state);
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
        // D s_32_1: write-var gs#112109 <= s_32_0
        fn_state.gs_112109 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#112109:u8
        let s_33_0: bool = fn_state.gs_112109;
        // N s_33_1: branch s_33_0 b65 b34
        if s_33_0 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var __ICC_HSRE_SRE:u8
        let s_34_0: bool = fn_state.u__ICC_HSRE_SRE;
        // D s_34_1: cast zx s_34_0 -> bv
        let s_34_1: Bits = Bits::new(s_34_0 as u128, 1u16);
        // C s_34_2: const #0u : u8
        let s_34_2: bool = false;
        // C s_34_3: cast zx s_34_2 -> bv
        let s_34_3: Bits = Bits::new(s_34_2 as u128, 1u16);
        // D s_34_4: cmp-eq s_34_1 s_34_3
        let s_34_4: bool = ((s_34_1) == (s_34_3));
        // N s_34_5: branch s_34_4 b64 b35
        if s_34_4 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #424u : u32
        let s_35_0: u32 = 424;
        // D s_35_1: read-reg s_35_0:u8
        let s_35_1: u8 = {
            let value = state.read_register::<u8>(s_35_0 as isize);
            tracer.read_register(s_35_0 as isize, value);
            value
        };
        // C s_35_2: const #2u : u8
        let s_35_2: u8 = 2;
        // D s_35_3: cmp-lt s_35_1 s_35_2
        let s_35_3: bool = ((s_35_1) < (s_35_2));
        // N s_35_4: branch s_35_3 b63 b36
        if s_35_3 {
            return block_63(state, tracer, fn_state);
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
        // D s_36_1: write-var gs#112110 <= s_36_0
        fn_state.gs_112110 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#112110:u8
        let s_37_0: bool = fn_state.gs_112110;
        // N s_37_1: branch s_37_0 b62 b38
        if s_37_0 {
            return block_62(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#112111 <= s_38_0
        fn_state.gs_112111 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#112111:u8
        let s_39_0: bool = fn_state.gs_112111;
        // N s_39_1: branch s_39_0 b56 b40
        if s_39_0 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
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
        // C s_40_2: const #2u : u8
        let s_40_2: u8 = 2;
        // D s_40_3: cmp-lt s_40_1 s_40_2
        let s_40_3: bool = ((s_40_1) < (s_40_2));
        // N s_40_4: branch s_40_3 b55 b41
        if s_40_3 {
            return block_55(state, tracer, fn_state);
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
        // D s_41_1: write-var gs#112112 <= s_41_0
        fn_state.gs_112112 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#112112:u8
        let s_42_0: bool = fn_state.gs_112112;
        // N s_42_1: branch s_42_0 b54 b43
        if s_42_0 {
            return block_54(state, tracer, fn_state);
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
        // D s_43_1: write-var gs#112113 <= s_43_0
        fn_state.gs_112113 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#112113:u8
        let s_44_0: bool = fn_state.gs_112113;
        // N s_44_1: branch s_44_0 b48 b45
        if s_44_0 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #424u : u32
        let s_45_0: u32 = 424;
        // D s_45_1: read-reg s_45_0:u8
        let s_45_1: u8 = {
            let value = state.read_register::<u8>(s_45_0 as isize);
            tracer.read_register(s_45_0 as isize, value);
            value
        };
        // C s_45_2: const #2u : u8
        let s_45_2: u8 = 2;
        // D s_45_3: cmp-lt s_45_1 s_45_2
        let s_45_3: bool = ((s_45_1) < (s_45_2));
        // N s_45_4: branch s_45_3 b47 b46
        if s_45_3 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #0s : i
        let s_46_0: i128 = 0;
        // S s_46_1: call ICC_AP1R_read(s_46_0)
        let s_46_1: u32 = ICC_AP1R_read(state, tracer, s_46_0);
        // D s_46_2: read-var t:i
        let s_46_2: i128 = fn_state.t;
        // D s_46_3: call R_set(s_46_2, s_46_1)
        let s_46_3: () = R_set(state, tracer, s_46_2, s_46_1);
        // N s_46_4: return
        return;
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #0s : i64
        let s_47_0: i64 = 0;
        // S s_47_1: call ICC_AP1R_NS_read(s_47_0)
        let s_47_1: u32 = ICC_AP1R_NS_read(state, tracer, s_47_0);
        // D s_47_2: read-var t:i
        let s_47_2: i128 = fn_state.t;
        // D s_47_3: call R_set(s_47_2, s_47_1)
        let s_47_3: () = R_set(state, tracer, s_47_2, s_47_1);
        // N s_47_4: return
        return;
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #() : ()
        let s_48_0: () = ();
        // S s_48_1: call Halted(s_48_0)
        let s_48_1: bool = Halted(state, tracer, s_48_0);
        // N s_48_2: branch s_48_1 b53 b49
        if s_48_1 {
            return block_53(state, tracer, fn_state);
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
        // D s_49_1: write-var gs#112116 <= s_49_0
        fn_state.gs_112116 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#112116:u8
        let s_50_0: bool = fn_state.gs_112116;
        // N s_50_1: branch s_50_0 b52 b51
        if s_50_0 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #() : ()
        let s_51_0: () = ();
        // S s_51_1: call AArch32_TakeMonitorTrapException(s_51_0)
        let s_51_1: () = AArch32_TakeMonitorTrapException(state, tracer, s_51_0);
        // N s_51_2: return
        return;
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_52_0: panic
        panic!("{:?}", ());
        // N s_52_1: return
        return;
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #() : ()
        let s_53_0: () = ();
        // S s_53_1: call EDSCR_read(s_53_0)
        let s_53_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_53_0);
        // S s_53_2: call _get_EDSCR_Type_SDD(s_53_1)
        let s_53_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_53_1);
        // S s_53_3: cast zx s_53_2 -> bv
        let s_53_3: Bits = Bits::new(s_53_2 as u128, 1u16);
        // C s_53_4: const #1u : u8
        let s_53_4: bool = true;
        // C s_53_5: cast zx s_53_4 -> bv
        let s_53_5: Bits = Bits::new(s_53_4 as u128, 1u16);
        // S s_53_6: cmp-eq s_53_3 s_53_5
        let s_53_6: bool = ((s_53_3) == (s_53_5));
        // D s_53_7: write-var gs#112116 <= s_53_6
        fn_state.gs_112116 = s_53_6;
        // N s_53_8: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var __SCR_IRQ:u8
        let s_54_0: bool = fn_state.u__SCR_IRQ;
        // D s_54_1: cast zx s_54_0 -> bv
        let s_54_1: Bits = Bits::new(s_54_0 as u128, 1u16);
        // C s_54_2: const #1u : u8
        let s_54_2: bool = true;
        // C s_54_3: cast zx s_54_2 -> bv
        let s_54_3: Bits = Bits::new(s_54_2 as u128, 1u16);
        // D s_54_4: cmp-eq s_54_1 s_54_3
        let s_54_4: bool = ((s_54_1) == (s_54_3));
        // D s_54_5: write-var gs#112113 <= s_54_4
        fn_state.gs_112113 = s_54_4;
        // N s_54_6: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #424u : u32
        let s_55_0: u32 = 424;
        // D s_55_1: read-reg s_55_0:u8
        let s_55_1: u8 = {
            let value = state.read_register::<u8>(s_55_0 as isize);
            tracer.read_register(s_55_0 as isize, value);
            value
        };
        // D s_55_2: call ELUsingAArch32(s_55_1)
        let s_55_2: bool = ELUsingAArch32(state, tracer, s_55_1);
        // D s_55_3: write-var gs#112112 <= s_55_2
        fn_state.gs_112112 = s_55_2;
        // N s_55_4: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #() : ()
        let s_56_0: () = ();
        // S s_56_1: call Halted(s_56_0)
        let s_56_1: bool = Halted(state, tracer, s_56_0);
        // N s_56_2: branch s_56_1 b61 b57
        if s_56_1 {
            return block_61(state, tracer, fn_state);
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
        // D s_57_1: write-var gs#112117 <= s_57_0
        fn_state.gs_112117 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var gs#112117:u8
        let s_58_0: bool = fn_state.gs_112117;
        // N s_58_1: branch s_58_0 b60 b59
        if s_58_0 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #3u : u8
        let s_59_0: u8 = 3;
        // C s_59_1: cast zx s_59_0 -> bv
        let s_59_1: Bits = Bits::new(s_59_0 as u128, 8u16);
        // C s_59_2: cast zx s_59_1 -> i
        let s_59_2: i128 = (s_59_1.value() as i128);
        // C s_59_3: cast reint s_59_2 -> i64
        let s_59_3: i64 = (s_59_2 as i64);
        // C s_59_4: cast zx s_59_3 -> i
        let s_59_4: i128 = (i128::try_from(s_59_3).unwrap());
        // C s_59_5: const #424u : u32
        let s_59_5: u32 = 424;
        // D s_59_6: read-reg s_59_5:u8
        let s_59_6: u8 = {
            let value = state.read_register::<u8>(s_59_5 as isize);
            tracer.read_register(s_59_5 as isize, value);
            value
        };
        // D s_59_7: call AArch64_AArch32SystemAccessTrap(s_59_6, s_59_4)
        let s_59_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_59_6, s_59_4);
        // N s_59_8: return
        return;
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_60_0: panic
        panic!("{:?}", ());
        // N s_60_1: return
        return;
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #() : ()
        let s_61_0: () = ();
        // S s_61_1: call EDSCR_read(s_61_0)
        let s_61_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_61_0);
        // S s_61_2: call _get_EDSCR_Type_SDD(s_61_1)
        let s_61_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_61_1);
        // S s_61_3: cast zx s_61_2 -> bv
        let s_61_3: Bits = Bits::new(s_61_2 as u128, 1u16);
        // C s_61_4: const #1u : u8
        let s_61_4: bool = true;
        // C s_61_5: cast zx s_61_4 -> bv
        let s_61_5: Bits = Bits::new(s_61_4 as u128, 1u16);
        // S s_61_6: cmp-eq s_61_3 s_61_5
        let s_61_6: bool = ((s_61_3) == (s_61_5));
        // D s_61_7: write-var gs#112117 <= s_61_6
        fn_state.gs_112117 = s_61_6;
        // N s_61_8: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var __SCR_EL3_IRQ:u8
        let s_62_0: bool = fn_state.u__SCR_EL3_IRQ;
        // D s_62_1: cast zx s_62_0 -> bv
        let s_62_1: Bits = Bits::new(s_62_0 as u128, 1u16);
        // C s_62_2: const #1u : u8
        let s_62_2: bool = true;
        // C s_62_3: cast zx s_62_2 -> bv
        let s_62_3: Bits = Bits::new(s_62_2 as u128, 1u16);
        // D s_62_4: cmp-eq s_62_1 s_62_3
        let s_62_4: bool = ((s_62_1) == (s_62_3));
        // D s_62_5: write-var gs#112111 <= s_62_4
        fn_state.gs_112111 = s_62_4;
        // N s_62_6: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #424u : u32
        let s_63_0: u32 = 424;
        // D s_63_1: read-reg s_63_0:u8
        let s_63_1: u8 = {
            let value = state.read_register::<u8>(s_63_0 as isize);
            tracer.read_register(s_63_0 as isize, value);
            value
        };
        // D s_63_2: call ELUsingAArch32(s_63_1)
        let s_63_2: bool = ELUsingAArch32(state, tracer, s_63_1);
        // D s_63_3: not s_63_2
        let s_63_3: bool = !s_63_2;
        // D s_63_4: write-var gs#112110 <= s_63_3
        fn_state.gs_112110 = s_63_3;
        // N s_63_5: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_64_0: panic
        panic!("{:?}", ());
        // N s_64_1: return
        return;
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
        // D s_66_0: read-var __SCR_IRQ:u8
        let s_66_0: bool = fn_state.u__SCR_IRQ;
        // D s_66_1: cast zx s_66_0 -> bv
        let s_66_1: Bits = Bits::new(s_66_0 as u128, 1u16);
        // C s_66_2: const #1u : u8
        let s_66_2: bool = true;
        // C s_66_3: cast zx s_66_2 -> bv
        let s_66_3: Bits = Bits::new(s_66_2 as u128, 1u16);
        // D s_66_4: cmp-eq s_66_1 s_66_3
        let s_66_4: bool = ((s_66_1) == (s_66_3));
        // D s_66_5: write-var gs#112109 <= s_66_4
        fn_state.gs_112109 = s_66_4;
        // N s_66_6: jump b33
        return block_33(state, tracer, fn_state);
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
        // D s_67_3: write-var gs#112108 <= s_67_2
        fn_state.gs_112108 = s_67_2;
        // N s_67_4: jump b31
        return block_31(state, tracer, fn_state);
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
        // D s_68_2: write-var gs#112107 <= s_68_1
        fn_state.gs_112107 = s_68_1;
        // N s_68_3: jump b29
        return block_29(state, tracer, fn_state);
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
        // D s_69_7: write-var gs#112106 <= s_69_6
        fn_state.gs_112106 = s_69_6;
        // N s_69_8: jump b27
        return block_27(state, tracer, fn_state);
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
        // D s_70_4: write-var gs#112105 <= s_70_3
        fn_state.gs_112105 = s_70_3;
        // N s_70_5: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_71_0: panic
        panic!("{:?}", ());
        // N s_71_1: return
        return;
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var __SCR_EL3_IRQ:u8
        let s_72_0: bool = fn_state.u__SCR_EL3_IRQ;
        // D s_72_1: cast zx s_72_0 -> bv
        let s_72_1: Bits = Bits::new(s_72_0 as u128, 1u16);
        // C s_72_2: const #1u : u8
        let s_72_2: bool = true;
        // C s_72_3: cast zx s_72_2 -> bv
        let s_72_3: Bits = Bits::new(s_72_2 as u128, 1u16);
        // D s_72_4: cmp-eq s_72_1 s_72_3
        let s_72_4: bool = ((s_72_1) == (s_72_3));
        // D s_72_5: write-var gs#112104 <= s_72_4
        fn_state.gs_112104 = s_72_4;
        // N s_72_6: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #424u : u32
        let s_73_0: u32 = 424;
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
        // D s_73_4: write-var gs#112103 <= s_73_3
        fn_state.gs_112103 = s_73_3;
        // N s_73_5: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_74_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_74_1: call __IMPDEF_boolean(s_74_0)
        let s_74_1: bool = u__IMPDEF_boolean(state, tracer, s_74_0);
        // D s_74_2: write-var gs#112102 <= s_74_1
        fn_state.gs_112102 = s_74_1;
        // N s_74_3: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #() : ()
        let s_75_0: () = ();
        // S s_75_1: call EDSCR_read(s_75_0)
        let s_75_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_75_0);
        // S s_75_2: call _get_EDSCR_Type_SDD(s_75_1)
        let s_75_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_75_1);
        // S s_75_3: cast zx s_75_2 -> bv
        let s_75_3: Bits = Bits::new(s_75_2 as u128, 1u16);
        // C s_75_4: const #1u : u8
        let s_75_4: bool = true;
        // C s_75_5: cast zx s_75_4 -> bv
        let s_75_5: Bits = Bits::new(s_75_4 as u128, 1u16);
        // S s_75_6: cmp-eq s_75_3 s_75_5
        let s_75_6: bool = ((s_75_3) == (s_75_5));
        // D s_75_7: write-var gs#112101 <= s_75_6
        fn_state.gs_112101 = s_75_6;
        // N s_75_8: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #424u : u32
        let s_76_0: u32 = 424;
        // D s_76_1: read-reg s_76_0:u8
        let s_76_1: u8 = {
            let value = state.read_register::<u8>(s_76_0 as isize);
            tracer.read_register(s_76_0 as isize, value);
            value
        };
        // C s_76_2: const #2u : u8
        let s_76_2: u8 = 2;
        // D s_76_3: cmp-lt s_76_1 s_76_2
        let s_76_3: bool = ((s_76_1) < (s_76_2));
        // D s_76_4: write-var gs#112100 <= s_76_3
        fn_state.gs_112100 = s_76_3;
        // N s_76_5: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #() : ()
        let s_77_0: () = ();
        // S s_77_1: call Halted(s_77_0)
        let s_77_1: bool = Halted(state, tracer, s_77_0);
        // N s_77_2: branch s_77_1 b195 b78
        if s_77_1 {
            return block_195(state, tracer, fn_state);
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
        // D s_78_1: write-var gs#112118 <= s_78_0
        fn_state.gs_112118 = s_78_0;
        // N s_78_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var gs#112118:u8
        let s_79_0: bool = fn_state.gs_112118;
        // N s_79_1: branch s_79_0 b194 b80
        if s_79_0 {
            return block_194(state, tracer, fn_state);
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
        // D s_80_1: write-var gs#112119 <= s_80_0
        fn_state.gs_112119 = s_80_0;
        // N s_80_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#112119:u8
        let s_81_0: bool = fn_state.gs_112119;
        // N s_81_1: branch s_81_0 b193 b82
        if s_81_0 {
            return block_193(state, tracer, fn_state);
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
        // D s_82_1: write-var gs#112120 <= s_82_0
        fn_state.gs_112120 = s_82_0;
        // N s_82_2: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_83_0: read-var gs#112120:u8
        let s_83_0: bool = fn_state.gs_112120;
        // N s_83_1: branch s_83_0 b192 b84
        if s_83_0 {
            return block_192(state, tracer, fn_state);
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
        // D s_84_1: write-var gs#112121 <= s_84_0
        fn_state.gs_112121 = s_84_0;
        // N s_84_2: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var gs#112121:u8
        let s_85_0: bool = fn_state.gs_112121;
        // N s_85_1: branch s_85_0 b191 b86
        if s_85_0 {
            return block_191(state, tracer, fn_state);
        } else {
            return block_86(state, tracer, fn_state);
        };
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #0u : u8
        let s_86_0: bool = false;
        // D s_86_1: write-var gs#112122 <= s_86_0
        fn_state.gs_112122 = s_86_0;
        // N s_86_2: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var gs#112122:u8
        let s_87_0: bool = fn_state.gs_112122;
        // N s_87_1: branch s_87_0 b190 b88
        if s_87_0 {
            return block_190(state, tracer, fn_state);
        } else {
            return block_88(state, tracer, fn_state);
        };
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
        // N s_88_2: branch s_88_1 b189 b89
        if s_88_1 {
            return block_189(state, tracer, fn_state);
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
        // D s_89_1: write-var gs#112123 <= s_89_0
        fn_state.gs_112123 = s_89_0;
        // N s_89_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var gs#112123:u8
        let s_90_0: bool = fn_state.gs_112123;
        // N s_90_1: branch s_90_0 b188 b91
        if s_90_0 {
            return block_188(state, tracer, fn_state);
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
        // D s_91_1: write-var gs#112124 <= s_91_0
        fn_state.gs_112124 = s_91_0;
        // N s_91_2: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var gs#112124:u8
        let s_92_0: bool = fn_state.gs_112124;
        // N s_92_1: branch s_92_0 b187 b93
        if s_92_0 {
            return block_187(state, tracer, fn_state);
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
        // D s_93_1: write-var gs#112125 <= s_93_0
        fn_state.gs_112125 = s_93_0;
        // N s_93_2: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var gs#112125:u8
        let s_94_0: bool = fn_state.gs_112125;
        // N s_94_1: branch s_94_0 b186 b95
        if s_94_0 {
            return block_186(state, tracer, fn_state);
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
        // D s_95_1: write-var gs#112126 <= s_95_0
        fn_state.gs_112126 = s_95_0;
        // N s_95_2: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var gs#112126:u8
        let s_96_0: bool = fn_state.gs_112126;
        // N s_96_1: branch s_96_0 b185 b97
        if s_96_0 {
            return block_185(state, tracer, fn_state);
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
        // D s_97_1: write-var gs#112127 <= s_97_0
        fn_state.gs_112127 = s_97_0;
        // N s_97_2: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var gs#112127:u8
        let s_98_0: bool = fn_state.gs_112127;
        // N s_98_1: branch s_98_0 b184 b99
        if s_98_0 {
            return block_184(state, tracer, fn_state);
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
        // D s_99_1: write-var gs#112128 <= s_99_0
        fn_state.gs_112128 = s_99_0;
        // N s_99_2: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_100_0: read-var gs#112128:u8
        let s_100_0: bool = fn_state.gs_112128;
        // N s_100_1: branch s_100_0 b183 b101
        if s_100_0 {
            return block_183(state, tracer, fn_state);
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
        // N s_101_2: branch s_101_1 b182 b102
        if s_101_1 {
            return block_182(state, tracer, fn_state);
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
        // D s_102_1: write-var gs#112129 <= s_102_0
        fn_state.gs_112129 = s_102_0;
        // N s_102_2: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_103_0: read-var gs#112129:u8
        let s_103_0: bool = fn_state.gs_112129;
        // N s_103_1: branch s_103_0 b181 b104
        if s_103_0 {
            return block_181(state, tracer, fn_state);
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
        // D s_104_1: write-var gs#112130 <= s_104_0
        fn_state.gs_112130 = s_104_0;
        // N s_104_2: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_105_0: read-var gs#112130:u8
        let s_105_0: bool = fn_state.gs_112130;
        // N s_105_1: branch s_105_0 b180 b106
        if s_105_0 {
            return block_180(state, tracer, fn_state);
        } else {
            return block_106(state, tracer, fn_state);
        };
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #() : ()
        let s_106_0: () = ();
        // S s_106_1: call EL2Enabled(s_106_0)
        let s_106_1: bool = EL2Enabled(state, tracer, s_106_0);
        // N s_106_2: branch s_106_1 b179 b107
        if s_106_1 {
            return block_179(state, tracer, fn_state);
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
        // D s_107_1: write-var gs#112131 <= s_107_0
        fn_state.gs_112131 = s_107_0;
        // N s_107_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var gs#112131:u8
        let s_108_0: bool = fn_state.gs_112131;
        // N s_108_1: branch s_108_0 b178 b109
        if s_108_0 {
            return block_178(state, tracer, fn_state);
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
        // D s_109_1: write-var gs#112132 <= s_109_0
        fn_state.gs_112132 = s_109_0;
        // N s_109_2: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_110_0: read-var gs#112132:u8
        let s_110_0: bool = fn_state.gs_112132;
        // N s_110_1: branch s_110_0 b177 b111
        if s_110_0 {
            return block_177(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var __ICC_SRE_SRE:u8
        let s_111_0: bool = fn_state.u__ICC_SRE_SRE;
        // D s_111_1: cast zx s_111_0 -> bv
        let s_111_1: Bits = Bits::new(s_111_0 as u128, 1u16);
        // C s_111_2: const #0u : u8
        let s_111_2: bool = false;
        // C s_111_3: cast zx s_111_2 -> bv
        let s_111_3: Bits = Bits::new(s_111_2 as u128, 1u16);
        // D s_111_4: cmp-eq s_111_1 s_111_3
        let s_111_4: bool = ((s_111_1) == (s_111_3));
        // N s_111_5: branch s_111_4 b176 b112
        if s_111_4 {
            return block_176(state, tracer, fn_state);
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
        // N s_112_2: branch s_112_1 b175 b113
        if s_112_1 {
            return block_175(state, tracer, fn_state);
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
        // D s_113_1: write-var gs#112133 <= s_113_0
        fn_state.gs_112133 = s_113_0;
        // N s_113_2: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_114_0: read-var gs#112133:u8
        let s_114_0: bool = fn_state.gs_112133;
        // N s_114_1: branch s_114_0 b174 b115
        if s_114_0 {
            return block_174(state, tracer, fn_state);
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
        // D s_115_1: write-var gs#112134 <= s_115_0
        fn_state.gs_112134 = s_115_0;
        // N s_115_2: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_116_0: read-var gs#112134:u8
        let s_116_0: bool = fn_state.gs_112134;
        // N s_116_1: branch s_116_0 b173 b117
        if s_116_0 {
            return block_173(state, tracer, fn_state);
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
        // N s_117_2: branch s_117_1 b172 b118
        if s_117_1 {
            return block_172(state, tracer, fn_state);
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
        // D s_118_1: write-var gs#112135 <= s_118_0
        fn_state.gs_112135 = s_118_0;
        // N s_118_2: jump b119
        return block_119(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_119_0: read-var gs#112135:u8
        let s_119_0: bool = fn_state.gs_112135;
        // N s_119_1: branch s_119_0 b171 b120
        if s_119_0 {
            return block_171(state, tracer, fn_state);
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
        // D s_120_1: write-var gs#112136 <= s_120_0
        fn_state.gs_112136 = s_120_0;
        // N s_120_2: jump b121
        return block_121(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_121_0: read-var gs#112136:u8
        let s_121_0: bool = fn_state.gs_112136;
        // N s_121_1: branch s_121_0 b170 b122
        if s_121_0 {
            return block_170(state, tracer, fn_state);
        } else {
            return block_122(state, tracer, fn_state);
        };
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_122_0: const #() : ()
        let s_122_0: () = ();
        // S s_122_1: call EL2Enabled(s_122_0)
        let s_122_1: bool = EL2Enabled(state, tracer, s_122_0);
        // N s_122_2: branch s_122_1 b169 b123
        if s_122_1 {
            return block_169(state, tracer, fn_state);
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
        // D s_123_1: write-var gs#112137 <= s_123_0
        fn_state.gs_112137 = s_123_0;
        // N s_123_2: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_124_0: read-var gs#112137:u8
        let s_124_0: bool = fn_state.gs_112137;
        // N s_124_1: branch s_124_0 b168 b125
        if s_124_0 {
            return block_168(state, tracer, fn_state);
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
        // D s_125_1: write-var gs#112138 <= s_125_0
        fn_state.gs_112138 = s_125_0;
        // N s_125_2: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_126_0: read-var gs#112138:u8
        let s_126_0: bool = fn_state.gs_112138;
        // N s_126_1: branch s_126_0 b167 b127
        if s_126_0 {
            return block_167(state, tracer, fn_state);
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
        // S s_127_1: call EL2Enabled(s_127_0)
        let s_127_1: bool = EL2Enabled(state, tracer, s_127_0);
        // N s_127_2: branch s_127_1 b166 b128
        if s_127_1 {
            return block_166(state, tracer, fn_state);
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
        // D s_128_1: write-var gs#112139 <= s_128_0
        fn_state.gs_112139 = s_128_0;
        // N s_128_2: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_129_0: read-var gs#112139:u8
        let s_129_0: bool = fn_state.gs_112139;
        // N s_129_1: branch s_129_0 b165 b130
        if s_129_0 {
            return block_165(state, tracer, fn_state);
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
        // D s_130_1: write-var gs#112140 <= s_130_0
        fn_state.gs_112140 = s_130_0;
        // N s_130_2: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_131_0: read-var gs#112140:u8
        let s_131_0: bool = fn_state.gs_112140;
        // N s_131_1: branch s_131_0 b164 b132
        if s_131_0 {
            return block_164(state, tracer, fn_state);
        } else {
            return block_132(state, tracer, fn_state);
        };
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_132_0: const #424u : u32
        let s_132_0: u32 = 424;
        // D s_132_1: read-reg s_132_0:u8
        let s_132_1: u8 = {
            let value = state.read_register::<u8>(s_132_0 as isize);
            tracer.read_register(s_132_0 as isize, value);
            value
        };
        // C s_132_2: const #2u : u8
        let s_132_2: u8 = 2;
        // D s_132_3: cmp-lt s_132_1 s_132_2
        let s_132_3: bool = ((s_132_1) < (s_132_2));
        // N s_132_4: branch s_132_3 b163 b133
        if s_132_3 {
            return block_163(state, tracer, fn_state);
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
        // D s_133_1: write-var gs#112141 <= s_133_0
        fn_state.gs_112141 = s_133_0;
        // N s_133_2: jump b134
        return block_134(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_134_0: read-var gs#112141:u8
        let s_134_0: bool = fn_state.gs_112141;
        // N s_134_1: branch s_134_0 b162 b135
        if s_134_0 {
            return block_162(state, tracer, fn_state);
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
        // D s_135_1: write-var gs#112142 <= s_135_0
        fn_state.gs_112142 = s_135_0;
        // N s_135_2: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_136_0: read-var gs#112142:u8
        let s_136_0: bool = fn_state.gs_112142;
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
        // C s_137_0: const #424u : u32
        let s_137_0: u32 = 424;
        // D s_137_1: read-reg s_137_0:u8
        let s_137_1: u8 = {
            let value = state.read_register::<u8>(s_137_0 as isize);
            tracer.read_register(s_137_0 as isize, value);
            value
        };
        // C s_137_2: const #2u : u8
        let s_137_2: u8 = 2;
        // D s_137_3: cmp-lt s_137_1 s_137_2
        let s_137_3: bool = ((s_137_1) < (s_137_2));
        // N s_137_4: branch s_137_3 b155 b138
        if s_137_3 {
            return block_155(state, tracer, fn_state);
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
        // D s_138_1: write-var gs#112143 <= s_138_0
        fn_state.gs_112143 = s_138_0;
        // N s_138_2: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_139_0: read-var gs#112143:u8
        let s_139_0: bool = fn_state.gs_112143;
        // N s_139_1: branch s_139_0 b154 b140
        if s_139_0 {
            return block_154(state, tracer, fn_state);
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
        // D s_140_1: write-var gs#112144 <= s_140_0
        fn_state.gs_112144 = s_140_0;
        // N s_140_2: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_141_0: read-var gs#112144:u8
        let s_141_0: bool = fn_state.gs_112144;
        // N s_141_1: branch s_141_0 b153 b142
        if s_141_0 {
            return block_153(state, tracer, fn_state);
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
        // D s_142_1: write-var gs#112145 <= s_142_0
        fn_state.gs_112145 = s_142_0;
        // N s_142_2: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_143_0: read-var gs#112145:u8
        let s_143_0: bool = fn_state.gs_112145;
        // N s_143_1: branch s_143_0 b147 b144
        if s_143_0 {
            return block_147(state, tracer, fn_state);
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
        // N s_144_4: branch s_144_3 b146 b145
        if s_144_3 {
            return block_146(state, tracer, fn_state);
        } else {
            return block_145(state, tracer, fn_state);
        };
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_145_0: const #0s : i
        let s_145_0: i128 = 0;
        // S s_145_1: call ICC_AP1R_read(s_145_0)
        let s_145_1: u32 = ICC_AP1R_read(state, tracer, s_145_0);
        // D s_145_2: read-var t:i
        let s_145_2: i128 = fn_state.t;
        // D s_145_3: call R_set(s_145_2, s_145_1)
        let s_145_3: () = R_set(state, tracer, s_145_2, s_145_1);
        // N s_145_4: return
        return;
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_146_0: const #0s : i64
        let s_146_0: i64 = 0;
        // S s_146_1: call ICC_AP1R_NS_read(s_146_0)
        let s_146_1: u32 = ICC_AP1R_NS_read(state, tracer, s_146_0);
        // D s_146_2: read-var t:i
        let s_146_2: i128 = fn_state.t;
        // D s_146_3: call R_set(s_146_2, s_146_1)
        let s_146_3: () = R_set(state, tracer, s_146_2, s_146_1);
        // N s_146_4: return
        return;
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_147_0: const #() : ()
        let s_147_0: () = ();
        // S s_147_1: call Halted(s_147_0)
        let s_147_1: bool = Halted(state, tracer, s_147_0);
        // N s_147_2: branch s_147_1 b152 b148
        if s_147_1 {
            return block_152(state, tracer, fn_state);
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
        // D s_148_1: write-var gs#112148 <= s_148_0
        fn_state.gs_112148 = s_148_0;
        // N s_148_2: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_149_0: read-var gs#112148:u8
        let s_149_0: bool = fn_state.gs_112148;
        // N s_149_1: branch s_149_0 b151 b150
        if s_149_0 {
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
        // S s_150_1: call AArch32_TakeMonitorTrapException(s_150_0)
        let s_150_1: () = AArch32_TakeMonitorTrapException(state, tracer, s_150_0);
        // N s_150_2: return
        return;
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_151_0: panic
        panic!("{:?}", ());
        // N s_151_1: return
        return;
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_152_0: const #() : ()
        let s_152_0: () = ();
        // S s_152_1: call EDSCR_read(s_152_0)
        let s_152_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_152_0);
        // S s_152_2: call _get_EDSCR_Type_SDD(s_152_1)
        let s_152_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_152_1);
        // S s_152_3: cast zx s_152_2 -> bv
        let s_152_3: Bits = Bits::new(s_152_2 as u128, 1u16);
        // C s_152_4: const #1u : u8
        let s_152_4: bool = true;
        // C s_152_5: cast zx s_152_4 -> bv
        let s_152_5: Bits = Bits::new(s_152_4 as u128, 1u16);
        // S s_152_6: cmp-eq s_152_3 s_152_5
        let s_152_6: bool = ((s_152_3) == (s_152_5));
        // D s_152_7: write-var gs#112148 <= s_152_6
        fn_state.gs_112148 = s_152_6;
        // N s_152_8: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_153_0: read-var __SCR_IRQ:u8
        let s_153_0: bool = fn_state.u__SCR_IRQ;
        // D s_153_1: cast zx s_153_0 -> bv
        let s_153_1: Bits = Bits::new(s_153_0 as u128, 1u16);
        // C s_153_2: const #1u : u8
        let s_153_2: bool = true;
        // C s_153_3: cast zx s_153_2 -> bv
        let s_153_3: Bits = Bits::new(s_153_2 as u128, 1u16);
        // D s_153_4: cmp-eq s_153_1 s_153_3
        let s_153_4: bool = ((s_153_1) == (s_153_3));
        // D s_153_5: write-var gs#112145 <= s_153_4
        fn_state.gs_112145 = s_153_4;
        // N s_153_6: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_154_0: read-var __PSTATE_M:u8
        let s_154_0: u8 = fn_state.u__PSTATE_M;
        // D s_154_1: cast zx s_154_0 -> bv
        let s_154_1: Bits = Bits::new(s_154_0 as u128, 5u16);
        // C s_154_2: const #384u : u32
        let s_154_2: u32 = 384;
        // D s_154_3: read-reg s_154_2:u8
        let s_154_3: u8 = {
            let value = state.read_register::<u8>(s_154_2 as isize);
            tracer.read_register(s_154_2 as isize, value);
            value
        };
        // D s_154_4: cast zx s_154_3 -> bv
        let s_154_4: Bits = Bits::new(s_154_3 as u128, 5u16);
        // D s_154_5: cmp-ne s_154_1 s_154_4
        let s_154_5: bool = ((s_154_1) != (s_154_4));
        // D s_154_6: write-var gs#112144 <= s_154_5
        fn_state.gs_112144 = s_154_5;
        // N s_154_7: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_155_0: const #424u : u32
        let s_155_0: u32 = 424;
        // D s_155_1: read-reg s_155_0:u8
        let s_155_1: u8 = {
            let value = state.read_register::<u8>(s_155_0 as isize);
            tracer.read_register(s_155_0 as isize, value);
            value
        };
        // D s_155_2: call ELUsingAArch32(s_155_1)
        let s_155_2: bool = ELUsingAArch32(state, tracer, s_155_1);
        // D s_155_3: write-var gs#112143 <= s_155_2
        fn_state.gs_112143 = s_155_2;
        // N s_155_4: jump b139
        return block_139(state, tracer, fn_state);
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
        // D s_157_1: write-var gs#112149 <= s_157_0
        fn_state.gs_112149 = s_157_0;
        // N s_157_2: jump b158
        return block_158(state, tracer, fn_state);
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_158_0: read-var gs#112149:u8
        let s_158_0: bool = fn_state.gs_112149;
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
        // D s_161_7: write-var gs#112149 <= s_161_6
        fn_state.gs_112149 = s_161_6;
        // N s_161_8: jump b158
        return block_158(state, tracer, fn_state);
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_162_0: read-var __SCR_EL3_IRQ:u8
        let s_162_0: bool = fn_state.u__SCR_EL3_IRQ;
        // D s_162_1: cast zx s_162_0 -> bv
        let s_162_1: Bits = Bits::new(s_162_0 as u128, 1u16);
        // C s_162_2: const #1u : u8
        let s_162_2: bool = true;
        // C s_162_3: cast zx s_162_2 -> bv
        let s_162_3: Bits = Bits::new(s_162_2 as u128, 1u16);
        // D s_162_4: cmp-eq s_162_1 s_162_3
        let s_162_4: bool = ((s_162_1) == (s_162_3));
        // D s_162_5: write-var gs#112142 <= s_162_4
        fn_state.gs_112142 = s_162_4;
        // N s_162_6: jump b136
        return block_136(state, tracer, fn_state);
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
        // D s_163_4: write-var gs#112141 <= s_163_3
        fn_state.gs_112141 = s_163_3;
        // N s_163_5: jump b134
        return block_134(state, tracer, fn_state);
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_164_0: const #0s : i64
        let s_164_0: i64 = 0;
        // S s_164_1: call ICV_AP1R_read(s_164_0)
        let s_164_1: u32 = ICV_AP1R_read(state, tracer, s_164_0);
        // D s_164_2: read-var t:i
        let s_164_2: i128 = fn_state.t;
        // D s_164_3: call R_set(s_164_2, s_164_1)
        let s_164_3: () = R_set(state, tracer, s_164_2, s_164_1);
        // N s_164_4: return
        return;
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_165_0: read-var __HCR_IMO:u8
        let s_165_0: bool = fn_state.u__HCR_IMO;
        // D s_165_1: cast zx s_165_0 -> bv
        let s_165_1: Bits = Bits::new(s_165_0 as u128, 1u16);
        // C s_165_2: const #1u : u8
        let s_165_2: bool = true;
        // C s_165_3: cast zx s_165_2 -> bv
        let s_165_3: Bits = Bits::new(s_165_2 as u128, 1u16);
        // D s_165_4: cmp-eq s_165_1 s_165_3
        let s_165_4: bool = ((s_165_1) == (s_165_3));
        // D s_165_5: write-var gs#112140 <= s_165_4
        fn_state.gs_112140 = s_165_4;
        // N s_165_6: jump b131
        return block_131(state, tracer, fn_state);
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
        // D s_166_3: write-var gs#112139 <= s_166_2
        fn_state.gs_112139 = s_166_2;
        // N s_166_4: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_167_0: const #0s : i64
        let s_167_0: i64 = 0;
        // S s_167_1: call ICV_AP1R_read(s_167_0)
        let s_167_1: u32 = ICV_AP1R_read(state, tracer, s_167_0);
        // D s_167_2: read-var t:i
        let s_167_2: i128 = fn_state.t;
        // D s_167_3: call R_set(s_167_2, s_167_1)
        let s_167_3: () = R_set(state, tracer, s_167_2, s_167_1);
        // N s_167_4: return
        return;
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_168_0: read-var __HCR_EL2_IMO:u8
        let s_168_0: bool = fn_state.u__HCR_EL2_IMO;
        // D s_168_1: cast zx s_168_0 -> bv
        let s_168_1: Bits = Bits::new(s_168_0 as u128, 1u16);
        // C s_168_2: const #1u : u8
        let s_168_2: bool = true;
        // C s_168_3: cast zx s_168_2 -> bv
        let s_168_3: Bits = Bits::new(s_168_2 as u128, 1u16);
        // D s_168_4: cmp-eq s_168_1 s_168_3
        let s_168_4: bool = ((s_168_1) == (s_168_3));
        // D s_168_5: write-var gs#112138 <= s_168_4
        fn_state.gs_112138 = s_168_4;
        // N s_168_6: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_169_0: const #432u : u32
        let s_169_0: u32 = 432;
        // D s_169_1: read-reg s_169_0:u8
        let s_169_1: u8 = {
            let value = state.read_register::<u8>(s_169_0 as isize);
            tracer.read_register(s_169_0 as isize, value);
            value
        };
        // D s_169_2: call ELUsingAArch32(s_169_1)
        let s_169_2: bool = ELUsingAArch32(state, tracer, s_169_1);
        // D s_169_3: not s_169_2
        let s_169_3: bool = !s_169_2;
        // D s_169_4: write-var gs#112137 <= s_169_3
        fn_state.gs_112137 = s_169_3;
        // N s_169_5: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_170_0: const #3u : u8
        let s_170_0: u8 = 3;
        // C s_170_1: cast zx s_170_0 -> bv
        let s_170_1: Bits = Bits::new(s_170_0 as u128, 8u16);
        // C s_170_2: cast zx s_170_1 -> i
        let s_170_2: i128 = (s_170_1.value() as i128);
        // C s_170_3: cast reint s_170_2 -> i64
        let s_170_3: i64 = (s_170_2 as i64);
        // C s_170_4: cast zx s_170_3 -> i
        let s_170_4: i128 = (i128::try_from(s_170_3).unwrap());
        // S s_170_5: call AArch32_TakeHypTrapException(s_170_4)
        let s_170_5: () = AArch32_TakeHypTrapException(state, tracer, s_170_4);
        // N s_170_6: return
        return;
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_171_0: read-var __ICH_HCR_TALL1:u8
        let s_171_0: bool = fn_state.u__ICH_HCR_TALL1;
        // D s_171_1: cast zx s_171_0 -> bv
        let s_171_1: Bits = Bits::new(s_171_0 as u128, 1u16);
        // C s_171_2: const #1u : u8
        let s_171_2: bool = true;
        // C s_171_3: cast zx s_171_2 -> bv
        let s_171_3: Bits = Bits::new(s_171_2 as u128, 1u16);
        // D s_171_4: cmp-eq s_171_1 s_171_3
        let s_171_4: bool = ((s_171_1) == (s_171_3));
        // D s_171_5: write-var gs#112136 <= s_171_4
        fn_state.gs_112136 = s_171_4;
        // N s_171_6: jump b121
        return block_121(state, tracer, fn_state);
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_172_0: const #432u : u32
        let s_172_0: u32 = 432;
        // D s_172_1: read-reg s_172_0:u8
        let s_172_1: u8 = {
            let value = state.read_register::<u8>(s_172_0 as isize);
            tracer.read_register(s_172_0 as isize, value);
            value
        };
        // D s_172_2: call ELUsingAArch32(s_172_1)
        let s_172_2: bool = ELUsingAArch32(state, tracer, s_172_1);
        // D s_172_3: write-var gs#112135 <= s_172_2
        fn_state.gs_112135 = s_172_2;
        // N s_172_4: jump b119
        return block_119(state, tracer, fn_state);
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
        // C s_173_5: const #432u : u32
        let s_173_5: u32 = 432;
        // D s_173_6: read-reg s_173_5:u8
        let s_173_6: u8 = {
            let value = state.read_register::<u8>(s_173_5 as isize);
            tracer.read_register(s_173_5 as isize, value);
            value
        };
        // D s_173_7: call AArch64_AArch32SystemAccessTrap(s_173_6, s_173_4)
        let s_173_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_173_6,
            s_173_4,
        );
        // N s_173_8: return
        return;
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_174_0: read-var __ICH_HCR_EL2_TALL1:u8
        let s_174_0: bool = fn_state.u__ICH_HCR_EL2_TALL1;
        // D s_174_1: cast zx s_174_0 -> bv
        let s_174_1: Bits = Bits::new(s_174_0 as u128, 1u16);
        // C s_174_2: const #1u : u8
        let s_174_2: bool = true;
        // C s_174_3: cast zx s_174_2 -> bv
        let s_174_3: Bits = Bits::new(s_174_2 as u128, 1u16);
        // D s_174_4: cmp-eq s_174_1 s_174_3
        let s_174_4: bool = ((s_174_1) == (s_174_3));
        // D s_174_5: write-var gs#112134 <= s_174_4
        fn_state.gs_112134 = s_174_4;
        // N s_174_6: jump b116
        return block_116(state, tracer, fn_state);
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
        // D s_175_3: not s_175_2
        let s_175_3: bool = !s_175_2;
        // D s_175_4: write-var gs#112133 <= s_175_3
        fn_state.gs_112133 = s_175_3;
        // N s_175_5: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_176_0: panic
        panic!("{:?}", ());
        // N s_176_1: return
        return;
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
        // D s_178_0: read-var __HSTR_T12:u8
        let s_178_0: bool = fn_state.u__HSTR_T12;
        // D s_178_1: cast zx s_178_0 -> bv
        let s_178_1: Bits = Bits::new(s_178_0 as u128, 1u16);
        // C s_178_2: const #1u : u8
        let s_178_2: bool = true;
        // C s_178_3: cast zx s_178_2 -> bv
        let s_178_3: Bits = Bits::new(s_178_2 as u128, 1u16);
        // D s_178_4: cmp-eq s_178_1 s_178_3
        let s_178_4: bool = ((s_178_1) == (s_178_3));
        // D s_178_5: write-var gs#112132 <= s_178_4
        fn_state.gs_112132 = s_178_4;
        // N s_178_6: jump b110
        return block_110(state, tracer, fn_state);
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
        // D s_179_3: write-var gs#112131 <= s_179_2
        fn_state.gs_112131 = s_179_2;
        // N s_179_4: jump b108
        return block_108(state, tracer, fn_state);
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
        // D s_181_0: read-var __HSTR_EL2_T12:u8
        let s_181_0: bool = fn_state.u__HSTR_EL2_T12;
        // D s_181_1: cast zx s_181_0 -> bv
        let s_181_1: Bits = Bits::new(s_181_0 as u128, 1u16);
        // C s_181_2: const #1u : u8
        let s_181_2: bool = true;
        // C s_181_3: cast zx s_181_2 -> bv
        let s_181_3: Bits = Bits::new(s_181_2 as u128, 1u16);
        // D s_181_4: cmp-eq s_181_1 s_181_3
        let s_181_4: bool = ((s_181_1) == (s_181_3));
        // D s_181_5: write-var gs#112130 <= s_181_4
        fn_state.gs_112130 = s_181_4;
        // N s_181_6: jump b105
        return block_105(state, tracer, fn_state);
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
        // D s_182_4: write-var gs#112129 <= s_182_3
        fn_state.gs_112129 = s_182_3;
        // N s_182_5: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_183_0: panic
        panic!("{:?}", ());
        // N s_183_1: return
        return;
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_184_0: read-var __SCR_IRQ:u8
        let s_184_0: bool = fn_state.u__SCR_IRQ;
        // D s_184_1: cast zx s_184_0 -> bv
        let s_184_1: Bits = Bits::new(s_184_0 as u128, 1u16);
        // C s_184_2: const #1u : u8
        let s_184_2: bool = true;
        // C s_184_3: cast zx s_184_2 -> bv
        let s_184_3: Bits = Bits::new(s_184_2 as u128, 1u16);
        // D s_184_4: cmp-eq s_184_1 s_184_3
        let s_184_4: bool = ((s_184_1) == (s_184_3));
        // D s_184_5: write-var gs#112128 <= s_184_4
        fn_state.gs_112128 = s_184_4;
        // N s_184_6: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_185<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_185_0: read-var __PSTATE_M:u8
        let s_185_0: u8 = fn_state.u__PSTATE_M;
        // D s_185_1: cast zx s_185_0 -> bv
        let s_185_1: Bits = Bits::new(s_185_0 as u128, 5u16);
        // C s_185_2: const #384u : u32
        let s_185_2: u32 = 384;
        // D s_185_3: read-reg s_185_2:u8
        let s_185_3: u8 = {
            let value = state.read_register::<u8>(s_185_2 as isize);
            tracer.read_register(s_185_2 as isize, value);
            value
        };
        // D s_185_4: cast zx s_185_3 -> bv
        let s_185_4: Bits = Bits::new(s_185_3 as u128, 5u16);
        // D s_185_5: cmp-ne s_185_1 s_185_4
        let s_185_5: bool = ((s_185_1) != (s_185_4));
        // D s_185_6: write-var gs#112127 <= s_185_5
        fn_state.gs_112127 = s_185_5;
        // N s_185_7: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_186_0: const #424u : u32
        let s_186_0: u32 = 424;
        // D s_186_1: read-reg s_186_0:u8
        let s_186_1: u8 = {
            let value = state.read_register::<u8>(s_186_0 as isize);
            tracer.read_register(s_186_0 as isize, value);
            value
        };
        // D s_186_2: call ELUsingAArch32(s_186_1)
        let s_186_2: bool = ELUsingAArch32(state, tracer, s_186_1);
        // D s_186_3: write-var gs#112126 <= s_186_2
        fn_state.gs_112126 = s_186_2;
        // N s_186_4: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_187_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_187_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_187_1: call __IMPDEF_boolean(s_187_0)
        let s_187_1: bool = u__IMPDEF_boolean(state, tracer, s_187_0);
        // D s_187_2: write-var gs#112125 <= s_187_1
        fn_state.gs_112125 = s_187_1;
        // N s_187_3: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_188_0: const #() : ()
        let s_188_0: () = ();
        // S s_188_1: call EDSCR_read(s_188_0)
        let s_188_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_188_0);
        // S s_188_2: call _get_EDSCR_Type_SDD(s_188_1)
        let s_188_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_188_1);
        // S s_188_3: cast zx s_188_2 -> bv
        let s_188_3: Bits = Bits::new(s_188_2 as u128, 1u16);
        // C s_188_4: const #1u : u8
        let s_188_4: bool = true;
        // C s_188_5: cast zx s_188_4 -> bv
        let s_188_5: Bits = Bits::new(s_188_4 as u128, 1u16);
        // S s_188_6: cmp-eq s_188_3 s_188_5
        let s_188_6: bool = ((s_188_3) == (s_188_5));
        // D s_188_7: write-var gs#112124 <= s_188_6
        fn_state.gs_112124 = s_188_6;
        // N s_188_8: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_189<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_189_0: const #424u : u32
        let s_189_0: u32 = 424;
        // D s_189_1: read-reg s_189_0:u8
        let s_189_1: u8 = {
            let value = state.read_register::<u8>(s_189_0 as isize);
            tracer.read_register(s_189_0 as isize, value);
            value
        };
        // C s_189_2: const #2u : u8
        let s_189_2: u8 = 2;
        // D s_189_3: cmp-lt s_189_1 s_189_2
        let s_189_3: bool = ((s_189_1) < (s_189_2));
        // D s_189_4: write-var gs#112123 <= s_189_3
        fn_state.gs_112123 = s_189_3;
        // N s_189_5: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_190_0: panic
        panic!("{:?}", ());
        // N s_190_1: return
        return;
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_191_0: read-var __SCR_EL3_IRQ:u8
        let s_191_0: bool = fn_state.u__SCR_EL3_IRQ;
        // D s_191_1: cast zx s_191_0 -> bv
        let s_191_1: Bits = Bits::new(s_191_0 as u128, 1u16);
        // C s_191_2: const #1u : u8
        let s_191_2: bool = true;
        // C s_191_3: cast zx s_191_2 -> bv
        let s_191_3: Bits = Bits::new(s_191_2 as u128, 1u16);
        // D s_191_4: cmp-eq s_191_1 s_191_3
        let s_191_4: bool = ((s_191_1) == (s_191_3));
        // D s_191_5: write-var gs#112122 <= s_191_4
        fn_state.gs_112122 = s_191_4;
        // N s_191_6: jump b87
        return block_87(state, tracer, fn_state);
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
        // D s_192_3: not s_192_2
        let s_192_3: bool = !s_192_2;
        // D s_192_4: write-var gs#112121 <= s_192_3
        fn_state.gs_112121 = s_192_3;
        // N s_192_5: jump b85
        return block_85(state, tracer, fn_state);
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
        // D s_193_2: write-var gs#112120 <= s_193_1
        fn_state.gs_112120 = s_193_1;
        // N s_193_3: jump b83
        return block_83(state, tracer, fn_state);
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
        // D s_194_7: write-var gs#112119 <= s_194_6
        fn_state.gs_112119 = s_194_6;
        // N s_194_8: jump b81
        return block_81(state, tracer, fn_state);
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
        // D s_195_4: write-var gs#112118 <= s_195_3
        fn_state.gs_112118 = s_195_3;
        // N s_195_5: jump b79
        return block_79(state, tracer, fn_state);
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
}
