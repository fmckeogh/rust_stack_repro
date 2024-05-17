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
use u_get_ICC_SRE_Type_SRE::*;
use Halted::*;
use ICC_HSRE_read::*;
use ICC_SRE_read::*;
use ICC_BPR1_NS_write::*;
use Mk_ICC_BPR1_Type::*;
use R_read::*;
use u_get_ICH_HCR_Type_TALL1::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_HCR_Type_IMO::*;
use EL2Enabled::*;
use u_get_HSTR_Type_T12::*;
use u_get_ICC_MSRE_Type_SRE::*;
use Mk_ICV_BPR1_Type::*;
use ICC_BPR1_S_write::*;
use HCR_read::*;
use ICV_BPR1_write::*;
use u_get_SCR_EL3_Type_IRQ::*;
use u_get_ICC_HSRE_Type_SRE::*;
use u__IMPDEF_boolean::*;
use u_get_HSTR_EL2_Type_T12::*;
use HSTR_read::*;
use u_get_SCR_Type_NS::*;
use AArch64_AArch32SystemAccessTrap::*;
use ICC_BPR1_write::*;
use u_get_ICH_HCR_EL2_Type_TALL1::*;
use ELUsingAArch32::*;
use u_get_HCR_EL2_Type_IMO::*;
use ICH_HCR_read::*;
use AArch32_TakeMonitorTrapException::*;
use EDSCR_read::*;
use common::*;
pub fn ICC_BPR1_SysRegWrite32_fa0a3cea06e15e3e<T: Tracer>(
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
        gs_128572: bool,
        u__SCR_EL3_IRQ: bool,
        gs_128582: bool,
        u__HCR_EL2_IMO: bool,
        gs_128604: bool,
        gs_128606: bool,
        gs_128565: bool,
        u__SCR_IRQ: bool,
        gs_128581: bool,
        gs_128601: bool,
        u__ICH_HCR_EL2_TALL1: bool,
        gs_128584: bool,
        gs_128592: bool,
        u__ICH_HCR_TALL1: bool,
        gs_128571: bool,
        gs_128569: bool,
        gs_128587: bool,
        gs_128570: bool,
        gs_128573: bool,
        gs_128579: bool,
        u__HSTR_EL2_T12: bool,
        gs_128605: bool,
        gs_128563: bool,
        gs_128596: bool,
        gs_128597: bool,
        u__PSTATE_EL: u8,
        gs_128564: bool,
        gs_128576: bool,
        gs_128598: bool,
        gs_128590: bool,
        gs_128591: bool,
        gs_128602: bool,
        gs_128585: bool,
        gs_128593: bool,
        gs_128566: bool,
        gs_128586: bool,
        gs_128575: bool,
        gs_128567: bool,
        gs_128600: bool,
        u__ICC_HSRE_SRE: bool,
        u__HSTR_T12: bool,
        gs_128589: bool,
        gs_128568: bool,
        u__HCR_IMO: bool,
        u__SCR_NS: bool,
        gs_128603: bool,
        gs_128595: bool,
        gs_128562: bool,
        u__ICC_MSRE_SRE: bool,
        gs_128574: bool,
        gs_128599: bool,
        gs_128583: bool,
        gs_128577: bool,
        gs_128588: bool,
        gs_128580: bool,
        gs_128594: bool,
        u__ICC_SRE_SRE: bool,
        u__PSTATE_M: u8,
        gs_128607: bool,
        gs_128578: bool,
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
        // D s_0_54: read-var __PSTATE_EL:u8
        let s_0_54: u8 = fn_state.u__PSTATE_EL;
        // D s_0_55: cast zx s_0_54 -> bv
        let s_0_55: Bits = Bits::new(s_0_54 as u128, 2u16);
        // C s_0_56: const #448u : u32
        let s_0_56: u32 = 448;
        // D s_0_57: read-reg s_0_56:u8
        let s_0_57: u8 = {
            let value = state.read_register::<u8>(s_0_56 as isize);
            tracer.read_register(s_0_56 as isize, value);
            value
        };
        // D s_0_58: cast zx s_0_57 -> bv
        let s_0_58: Bits = Bits::new(s_0_57 as u128, 2u16);
        // D s_0_59: cmp-eq s_0_55 s_0_58
        let s_0_59: bool = ((s_0_55) == (s_0_58));
        // N s_0_60: branch s_0_59 b194 b1
        if s_0_59 {
            return block_194(state, tracer, fn_state);
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
        // D s_7_0: read-var t:i
        let s_7_0: i128 = fn_state.t;
        // D s_7_1: call R_read(s_7_0)
        let s_7_1: u32 = R_read(state, tracer, s_7_0);
        // D s_7_2: call Mk_ICC_BPR1_Type(s_7_1)
        let s_7_2: ProductType700c18a878c5601b = Mk_ICC_BPR1_Type(state, tracer, s_7_1);
        // D s_7_3: call ICC_BPR1_NS_write(s_7_2)
        let s_7_3: () = ICC_BPR1_NS_write(state, tracer, s_7_2);
        // N s_7_4: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var t:i
        let s_8_0: i128 = fn_state.t;
        // D s_8_1: call R_read(s_8_0)
        let s_8_1: u32 = R_read(state, tracer, s_8_0);
        // D s_8_2: call Mk_ICC_BPR1_Type(s_8_1)
        let s_8_2: ProductType700c18a878c5601b = Mk_ICC_BPR1_Type(state, tracer, s_8_1);
        // D s_8_3: call ICC_BPR1_S_write(s_8_2)
        let s_8_3: () = ICC_BPR1_S_write(state, tracer, s_8_2);
        // N s_8_4: return
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
        // D s_11_1: write-var gs#128562 <= s_11_0
        fn_state.gs_128562 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#128562:u8
        let s_12_0: bool = fn_state.gs_128562;
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
        // D s_13_1: write-var gs#128563 <= s_13_0
        fn_state.gs_128563 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#128563:u8
        let s_14_0: bool = fn_state.gs_128563;
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
        // D s_15_1: write-var gs#128564 <= s_15_0
        fn_state.gs_128564 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#128564:u8
        let s_16_0: bool = fn_state.gs_128564;
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
        // D s_17_1: write-var gs#128565 <= s_17_0
        fn_state.gs_128565 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#128565:u8
        let s_18_0: bool = fn_state.gs_128565;
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
        // D s_19_1: write-var gs#128566 <= s_19_0
        fn_state.gs_128566 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#128566:u8
        let s_20_0: bool = fn_state.gs_128566;
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
        // D s_22_1: write-var gs#128567 <= s_22_0
        fn_state.gs_128567 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#128567:u8
        let s_23_0: bool = fn_state.gs_128567;
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
        // D s_24_1: write-var gs#128568 <= s_24_0
        fn_state.gs_128568 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#128568:u8
        let s_25_0: bool = fn_state.gs_128568;
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
        // D s_26_1: write-var gs#128569 <= s_26_0
        fn_state.gs_128569 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#128569:u8
        let s_27_0: bool = fn_state.gs_128569;
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
        // D s_28_1: write-var gs#128570 <= s_28_0
        fn_state.gs_128570 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#128570:u8
        let s_29_0: bool = fn_state.gs_128570;
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
        // D s_30_1: write-var gs#128571 <= s_30_0
        fn_state.gs_128571 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#128571:u8
        let s_31_0: bool = fn_state.gs_128571;
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
        // D s_34_1: write-var gs#128572 <= s_34_0
        fn_state.gs_128572 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#128572:u8
        let s_35_0: bool = fn_state.gs_128572;
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
        // D s_36_1: write-var gs#128573 <= s_36_0
        fn_state.gs_128573 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#128573:u8
        let s_37_0: bool = fn_state.gs_128573;
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
        // D s_39_1: write-var gs#128574 <= s_39_0
        fn_state.gs_128574 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#128574:u8
        let s_40_0: bool = fn_state.gs_128574;
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
        // D s_41_1: write-var gs#128575 <= s_41_0
        fn_state.gs_128575 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#128575:u8
        let s_42_0: bool = fn_state.gs_128575;
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
        // D s_44_0: read-var t:i
        let s_44_0: i128 = fn_state.t;
        // D s_44_1: call R_read(s_44_0)
        let s_44_1: u32 = R_read(state, tracer, s_44_0);
        // D s_44_2: call Mk_ICC_BPR1_Type(s_44_1)
        let s_44_2: ProductType700c18a878c5601b = Mk_ICC_BPR1_Type(
            state,
            tracer,
            s_44_1,
        );
        // D s_44_3: call ICC_BPR1_write(s_44_2)
        let s_44_3: () = ICC_BPR1_write(state, tracer, s_44_2);
        // N s_44_4: return
        return;
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var t:i
        let s_45_0: i128 = fn_state.t;
        // D s_45_1: call R_read(s_45_0)
        let s_45_1: u32 = R_read(state, tracer, s_45_0);
        // D s_45_2: call Mk_ICC_BPR1_Type(s_45_1)
        let s_45_2: ProductType700c18a878c5601b = Mk_ICC_BPR1_Type(
            state,
            tracer,
            s_45_1,
        );
        // D s_45_3: call ICC_BPR1_NS_write(s_45_2)
        let s_45_3: () = ICC_BPR1_NS_write(state, tracer, s_45_2);
        // N s_45_4: return
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
        // D s_47_1: write-var gs#128576 <= s_47_0
        fn_state.gs_128576 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#128576:u8
        let s_48_0: bool = fn_state.gs_128576;
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
        // D s_51_7: write-var gs#128576 <= s_51_6
        fn_state.gs_128576 = s_51_6;
        // N s_51_8: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var __SCR_IRQ:u8
        let s_52_0: bool = fn_state.u__SCR_IRQ;
        // D s_52_1: cast zx s_52_0 -> bv
        let s_52_1: Bits = Bits::new(s_52_0 as u128, 1u16);
        // C s_52_2: const #1u : u8
        let s_52_2: bool = true;
        // C s_52_3: cast zx s_52_2 -> bv
        let s_52_3: Bits = Bits::new(s_52_2 as u128, 1u16);
        // D s_52_4: cmp-eq s_52_1 s_52_3
        let s_52_4: bool = ((s_52_1) == (s_52_3));
        // D s_52_5: write-var gs#128575 <= s_52_4
        fn_state.gs_128575 = s_52_4;
        // N s_52_6: jump b42
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
        // D s_53_3: write-var gs#128574 <= s_53_2
        fn_state.gs_128574 = s_53_2;
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
        // D s_55_1: write-var gs#128577 <= s_55_0
        fn_state.gs_128577 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#128577:u8
        let s_56_0: bool = fn_state.gs_128577;
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
        // D s_59_7: write-var gs#128577 <= s_59_6
        fn_state.gs_128577 = s_59_6;
        // N s_59_8: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var __SCR_EL3_IRQ:u8
        let s_60_0: bool = fn_state.u__SCR_EL3_IRQ;
        // D s_60_1: cast zx s_60_0 -> bv
        let s_60_1: Bits = Bits::new(s_60_0 as u128, 1u16);
        // C s_60_2: const #1u : u8
        let s_60_2: bool = true;
        // C s_60_3: cast zx s_60_2 -> bv
        let s_60_3: Bits = Bits::new(s_60_2 as u128, 1u16);
        // D s_60_4: cmp-eq s_60_1 s_60_3
        let s_60_4: bool = ((s_60_1) == (s_60_3));
        // D s_60_5: write-var gs#128573 <= s_60_4
        fn_state.gs_128573 = s_60_4;
        // N s_60_6: jump b37
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
        // D s_61_4: write-var gs#128572 <= s_61_3
        fn_state.gs_128572 = s_61_3;
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
        // D s_64_0: read-var __SCR_IRQ:u8
        let s_64_0: bool = fn_state.u__SCR_IRQ;
        // D s_64_1: cast zx s_64_0 -> bv
        let s_64_1: Bits = Bits::new(s_64_0 as u128, 1u16);
        // C s_64_2: const #1u : u8
        let s_64_2: bool = true;
        // C s_64_3: cast zx s_64_2 -> bv
        let s_64_3: Bits = Bits::new(s_64_2 as u128, 1u16);
        // D s_64_4: cmp-eq s_64_1 s_64_3
        let s_64_4: bool = ((s_64_1) == (s_64_3));
        // D s_64_5: write-var gs#128571 <= s_64_4
        fn_state.gs_128571 = s_64_4;
        // N s_64_6: jump b31
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
        // D s_65_3: write-var gs#128570 <= s_65_2
        fn_state.gs_128570 = s_65_2;
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
        // D s_66_2: write-var gs#128569 <= s_66_1
        fn_state.gs_128569 = s_66_1;
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
        // D s_67_7: write-var gs#128568 <= s_67_6
        fn_state.gs_128568 = s_67_6;
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
        // D s_68_4: write-var gs#128567 <= s_68_3
        fn_state.gs_128567 = s_68_3;
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
        // D s_70_0: read-var __SCR_EL3_IRQ:u8
        let s_70_0: bool = fn_state.u__SCR_EL3_IRQ;
        // D s_70_1: cast zx s_70_0 -> bv
        let s_70_1: Bits = Bits::new(s_70_0 as u128, 1u16);
        // C s_70_2: const #1u : u8
        let s_70_2: bool = true;
        // C s_70_3: cast zx s_70_2 -> bv
        let s_70_3: Bits = Bits::new(s_70_2 as u128, 1u16);
        // D s_70_4: cmp-eq s_70_1 s_70_3
        let s_70_4: bool = ((s_70_1) == (s_70_3));
        // D s_70_5: write-var gs#128566 <= s_70_4
        fn_state.gs_128566 = s_70_4;
        // N s_70_6: jump b20
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
        // D s_71_4: write-var gs#128565 <= s_71_3
        fn_state.gs_128565 = s_71_3;
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
        // D s_72_2: write-var gs#128564 <= s_72_1
        fn_state.gs_128564 = s_72_1;
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
        // D s_73_7: write-var gs#128563 <= s_73_6
        fn_state.gs_128563 = s_73_6;
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
        // D s_74_4: write-var gs#128562 <= s_74_3
        fn_state.gs_128562 = s_74_3;
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
        // N s_75_2: branch s_75_1 b193 b76
        if s_75_1 {
            return block_193(state, tracer, fn_state);
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
        // D s_76_1: write-var gs#128578 <= s_76_0
        fn_state.gs_128578 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var gs#128578:u8
        let s_77_0: bool = fn_state.gs_128578;
        // N s_77_1: branch s_77_0 b192 b78
        if s_77_0 {
            return block_192(state, tracer, fn_state);
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
        // D s_78_1: write-var gs#128579 <= s_78_0
        fn_state.gs_128579 = s_78_0;
        // N s_78_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var gs#128579:u8
        let s_79_0: bool = fn_state.gs_128579;
        // N s_79_1: branch s_79_0 b191 b80
        if s_79_0 {
            return block_191(state, tracer, fn_state);
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
        // D s_80_1: write-var gs#128580 <= s_80_0
        fn_state.gs_128580 = s_80_0;
        // N s_80_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#128580:u8
        let s_81_0: bool = fn_state.gs_128580;
        // N s_81_1: branch s_81_0 b190 b82
        if s_81_0 {
            return block_190(state, tracer, fn_state);
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
        // D s_82_1: write-var gs#128581 <= s_82_0
        fn_state.gs_128581 = s_82_0;
        // N s_82_2: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_83_0: read-var gs#128581:u8
        let s_83_0: bool = fn_state.gs_128581;
        // N s_83_1: branch s_83_0 b189 b84
        if s_83_0 {
            return block_189(state, tracer, fn_state);
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
        // D s_84_1: write-var gs#128582 <= s_84_0
        fn_state.gs_128582 = s_84_0;
        // N s_84_2: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var gs#128582:u8
        let s_85_0: bool = fn_state.gs_128582;
        // N s_85_1: branch s_85_0 b188 b86
        if s_85_0 {
            return block_188(state, tracer, fn_state);
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
        // N s_86_2: branch s_86_1 b187 b87
        if s_86_1 {
            return block_187(state, tracer, fn_state);
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
        // D s_87_1: write-var gs#128583 <= s_87_0
        fn_state.gs_128583 = s_87_0;
        // N s_87_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var gs#128583:u8
        let s_88_0: bool = fn_state.gs_128583;
        // N s_88_1: branch s_88_0 b186 b89
        if s_88_0 {
            return block_186(state, tracer, fn_state);
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
        // D s_89_1: write-var gs#128584 <= s_89_0
        fn_state.gs_128584 = s_89_0;
        // N s_89_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var gs#128584:u8
        let s_90_0: bool = fn_state.gs_128584;
        // N s_90_1: branch s_90_0 b185 b91
        if s_90_0 {
            return block_185(state, tracer, fn_state);
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
        // D s_91_1: write-var gs#128585 <= s_91_0
        fn_state.gs_128585 = s_91_0;
        // N s_91_2: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var gs#128585:u8
        let s_92_0: bool = fn_state.gs_128585;
        // N s_92_1: branch s_92_0 b184 b93
        if s_92_0 {
            return block_184(state, tracer, fn_state);
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
        // D s_93_1: write-var gs#128586 <= s_93_0
        fn_state.gs_128586 = s_93_0;
        // N s_93_2: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var gs#128586:u8
        let s_94_0: bool = fn_state.gs_128586;
        // N s_94_1: branch s_94_0 b183 b95
        if s_94_0 {
            return block_183(state, tracer, fn_state);
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
        // D s_95_1: write-var gs#128587 <= s_95_0
        fn_state.gs_128587 = s_95_0;
        // N s_95_2: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var gs#128587:u8
        let s_96_0: bool = fn_state.gs_128587;
        // N s_96_1: branch s_96_0 b182 b97
        if s_96_0 {
            return block_182(state, tracer, fn_state);
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
        // D s_97_1: write-var gs#128588 <= s_97_0
        fn_state.gs_128588 = s_97_0;
        // N s_97_2: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var gs#128588:u8
        let s_98_0: bool = fn_state.gs_128588;
        // N s_98_1: branch s_98_0 b181 b99
        if s_98_0 {
            return block_181(state, tracer, fn_state);
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
        // N s_99_2: branch s_99_1 b180 b100
        if s_99_1 {
            return block_180(state, tracer, fn_state);
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
        // D s_100_1: write-var gs#128589 <= s_100_0
        fn_state.gs_128589 = s_100_0;
        // N s_100_2: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_101_0: read-var gs#128589:u8
        let s_101_0: bool = fn_state.gs_128589;
        // N s_101_1: branch s_101_0 b179 b102
        if s_101_0 {
            return block_179(state, tracer, fn_state);
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
        // D s_102_1: write-var gs#128590 <= s_102_0
        fn_state.gs_128590 = s_102_0;
        // N s_102_2: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_103_0: read-var gs#128590:u8
        let s_103_0: bool = fn_state.gs_128590;
        // N s_103_1: branch s_103_0 b178 b104
        if s_103_0 {
            return block_178(state, tracer, fn_state);
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
        // N s_104_2: branch s_104_1 b177 b105
        if s_104_1 {
            return block_177(state, tracer, fn_state);
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
        // D s_105_1: write-var gs#128591 <= s_105_0
        fn_state.gs_128591 = s_105_0;
        // N s_105_2: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var gs#128591:u8
        let s_106_0: bool = fn_state.gs_128591;
        // N s_106_1: branch s_106_0 b176 b107
        if s_106_0 {
            return block_176(state, tracer, fn_state);
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
        // D s_107_1: write-var gs#128592 <= s_107_0
        fn_state.gs_128592 = s_107_0;
        // N s_107_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var gs#128592:u8
        let s_108_0: bool = fn_state.gs_128592;
        // N s_108_1: branch s_108_0 b175 b109
        if s_108_0 {
            return block_175(state, tracer, fn_state);
        } else {
            return block_109(state, tracer, fn_state);
        };
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_109_0: read-var __ICC_SRE_SRE:u8
        let s_109_0: bool = fn_state.u__ICC_SRE_SRE;
        // D s_109_1: cast zx s_109_0 -> bv
        let s_109_1: Bits = Bits::new(s_109_0 as u128, 1u16);
        // C s_109_2: const #0u : u8
        let s_109_2: bool = false;
        // C s_109_3: cast zx s_109_2 -> bv
        let s_109_3: Bits = Bits::new(s_109_2 as u128, 1u16);
        // D s_109_4: cmp-eq s_109_1 s_109_3
        let s_109_4: bool = ((s_109_1) == (s_109_3));
        // N s_109_5: branch s_109_4 b174 b110
        if s_109_4 {
            return block_174(state, tracer, fn_state);
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
        // N s_110_2: branch s_110_1 b173 b111
        if s_110_1 {
            return block_173(state, tracer, fn_state);
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
        // D s_111_1: write-var gs#128593 <= s_111_0
        fn_state.gs_128593 = s_111_0;
        // N s_111_2: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_112_0: read-var gs#128593:u8
        let s_112_0: bool = fn_state.gs_128593;
        // N s_112_1: branch s_112_0 b172 b113
        if s_112_0 {
            return block_172(state, tracer, fn_state);
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
        // D s_113_1: write-var gs#128594 <= s_113_0
        fn_state.gs_128594 = s_113_0;
        // N s_113_2: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_114_0: read-var gs#128594:u8
        let s_114_0: bool = fn_state.gs_128594;
        // N s_114_1: branch s_114_0 b171 b115
        if s_114_0 {
            return block_171(state, tracer, fn_state);
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
        // N s_115_2: branch s_115_1 b170 b116
        if s_115_1 {
            return block_170(state, tracer, fn_state);
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
        // D s_116_1: write-var gs#128595 <= s_116_0
        fn_state.gs_128595 = s_116_0;
        // N s_116_2: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_117_0: read-var gs#128595:u8
        let s_117_0: bool = fn_state.gs_128595;
        // N s_117_1: branch s_117_0 b169 b118
        if s_117_0 {
            return block_169(state, tracer, fn_state);
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
        // D s_118_1: write-var gs#128596 <= s_118_0
        fn_state.gs_128596 = s_118_0;
        // N s_118_2: jump b119
        return block_119(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_119_0: read-var gs#128596:u8
        let s_119_0: bool = fn_state.gs_128596;
        // N s_119_1: branch s_119_0 b168 b120
        if s_119_0 {
            return block_168(state, tracer, fn_state);
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
        // N s_120_2: branch s_120_1 b167 b121
        if s_120_1 {
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
        // C s_121_0: const #0u : u8
        let s_121_0: bool = false;
        // D s_121_1: write-var gs#128597 <= s_121_0
        fn_state.gs_128597 = s_121_0;
        // N s_121_2: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_122_0: read-var gs#128597:u8
        let s_122_0: bool = fn_state.gs_128597;
        // N s_122_1: branch s_122_0 b166 b123
        if s_122_0 {
            return block_166(state, tracer, fn_state);
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
        // D s_123_1: write-var gs#128598 <= s_123_0
        fn_state.gs_128598 = s_123_0;
        // N s_123_2: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_124_0: read-var gs#128598:u8
        let s_124_0: bool = fn_state.gs_128598;
        // N s_124_1: branch s_124_0 b165 b125
        if s_124_0 {
            return block_165(state, tracer, fn_state);
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
        // N s_125_2: branch s_125_1 b164 b126
        if s_125_1 {
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
        // D s_126_1: write-var gs#128599 <= s_126_0
        fn_state.gs_128599 = s_126_0;
        // N s_126_2: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_127_0: read-var gs#128599:u8
        let s_127_0: bool = fn_state.gs_128599;
        // N s_127_1: branch s_127_0 b163 b128
        if s_127_0 {
            return block_163(state, tracer, fn_state);
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
        // D s_128_1: write-var gs#128600 <= s_128_0
        fn_state.gs_128600 = s_128_0;
        // N s_128_2: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_129_0: read-var gs#128600:u8
        let s_129_0: bool = fn_state.gs_128600;
        // N s_129_1: branch s_129_0 b162 b130
        if s_129_0 {
            return block_162(state, tracer, fn_state);
        } else {
            return block_130(state, tracer, fn_state);
        };
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_130_0: const #424u : u32
        let s_130_0: u32 = 424;
        // D s_130_1: read-reg s_130_0:u8
        let s_130_1: u8 = {
            let value = state.read_register::<u8>(s_130_0 as isize);
            tracer.read_register(s_130_0 as isize, value);
            value
        };
        // C s_130_2: const #2u : u8
        let s_130_2: u8 = 2;
        // D s_130_3: cmp-lt s_130_1 s_130_2
        let s_130_3: bool = ((s_130_1) < (s_130_2));
        // N s_130_4: branch s_130_3 b161 b131
        if s_130_3 {
            return block_161(state, tracer, fn_state);
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
        // D s_131_1: write-var gs#128601 <= s_131_0
        fn_state.gs_128601 = s_131_0;
        // N s_131_2: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_132_0: read-var gs#128601:u8
        let s_132_0: bool = fn_state.gs_128601;
        // N s_132_1: branch s_132_0 b160 b133
        if s_132_0 {
            return block_160(state, tracer, fn_state);
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
        // D s_133_1: write-var gs#128602 <= s_133_0
        fn_state.gs_128602 = s_133_0;
        // N s_133_2: jump b134
        return block_134(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_134_0: read-var gs#128602:u8
        let s_134_0: bool = fn_state.gs_128602;
        // N s_134_1: branch s_134_0 b154 b135
        if s_134_0 {
            return block_154(state, tracer, fn_state);
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
        // N s_135_4: branch s_135_3 b153 b136
        if s_135_3 {
            return block_153(state, tracer, fn_state);
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
        // D s_136_1: write-var gs#128603 <= s_136_0
        fn_state.gs_128603 = s_136_0;
        // N s_136_2: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_137_0: read-var gs#128603:u8
        let s_137_0: bool = fn_state.gs_128603;
        // N s_137_1: branch s_137_0 b152 b138
        if s_137_0 {
            return block_152(state, tracer, fn_state);
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
        // D s_138_1: write-var gs#128604 <= s_138_0
        fn_state.gs_128604 = s_138_0;
        // N s_138_2: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_139_0: read-var gs#128604:u8
        let s_139_0: bool = fn_state.gs_128604;
        // N s_139_1: branch s_139_0 b151 b140
        if s_139_0 {
            return block_151(state, tracer, fn_state);
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
        // D s_140_1: write-var gs#128605 <= s_140_0
        fn_state.gs_128605 = s_140_0;
        // N s_140_2: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_141_0: read-var gs#128605:u8
        let s_141_0: bool = fn_state.gs_128605;
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
        // C s_142_0: const #424u : u32
        let s_142_0: u32 = 424;
        // D s_142_1: read-reg s_142_0:u8
        let s_142_1: u8 = {
            let value = state.read_register::<u8>(s_142_0 as isize);
            tracer.read_register(s_142_0 as isize, value);
            value
        };
        // C s_142_2: const #2u : u8
        let s_142_2: u8 = 2;
        // D s_142_3: cmp-lt s_142_1 s_142_2
        let s_142_3: bool = ((s_142_1) < (s_142_2));
        // N s_142_4: branch s_142_3 b144 b143
        if s_142_3 {
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
        // D s_143_0: read-var t:i
        let s_143_0: i128 = fn_state.t;
        // D s_143_1: call R_read(s_143_0)
        let s_143_1: u32 = R_read(state, tracer, s_143_0);
        // D s_143_2: call Mk_ICC_BPR1_Type(s_143_1)
        let s_143_2: ProductType700c18a878c5601b = Mk_ICC_BPR1_Type(
            state,
            tracer,
            s_143_1,
        );
        // D s_143_3: call ICC_BPR1_write(s_143_2)
        let s_143_3: () = ICC_BPR1_write(state, tracer, s_143_2);
        // N s_143_4: return
        return;
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_144_0: read-var t:i
        let s_144_0: i128 = fn_state.t;
        // D s_144_1: call R_read(s_144_0)
        let s_144_1: u32 = R_read(state, tracer, s_144_0);
        // D s_144_2: call Mk_ICC_BPR1_Type(s_144_1)
        let s_144_2: ProductType700c18a878c5601b = Mk_ICC_BPR1_Type(
            state,
            tracer,
            s_144_1,
        );
        // D s_144_3: call ICC_BPR1_NS_write(s_144_2)
        let s_144_3: () = ICC_BPR1_NS_write(state, tracer, s_144_2);
        // N s_144_4: return
        return;
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_145_0: const #() : ()
        let s_145_0: () = ();
        // S s_145_1: call Halted(s_145_0)
        let s_145_1: bool = Halted(state, tracer, s_145_0);
        // N s_145_2: branch s_145_1 b150 b146
        if s_145_1 {
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
        // D s_146_1: write-var gs#128606 <= s_146_0
        fn_state.gs_128606 = s_146_0;
        // N s_146_2: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_147_0: read-var gs#128606:u8
        let s_147_0: bool = fn_state.gs_128606;
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
        // C s_148_0: const #() : ()
        let s_148_0: () = ();
        // S s_148_1: call AArch32_TakeMonitorTrapException(s_148_0)
        let s_148_1: () = AArch32_TakeMonitorTrapException(state, tracer, s_148_0);
        // N s_148_2: return
        return;
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_149_0: panic
        panic!("{:?}", ());
        // N s_149_1: return
        return;
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_150_0: const #() : ()
        let s_150_0: () = ();
        // S s_150_1: call EDSCR_read(s_150_0)
        let s_150_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_150_0);
        // S s_150_2: call _get_EDSCR_Type_SDD(s_150_1)
        let s_150_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_150_1);
        // S s_150_3: cast zx s_150_2 -> bv
        let s_150_3: Bits = Bits::new(s_150_2 as u128, 1u16);
        // C s_150_4: const #1u : u8
        let s_150_4: bool = true;
        // C s_150_5: cast zx s_150_4 -> bv
        let s_150_5: Bits = Bits::new(s_150_4 as u128, 1u16);
        // S s_150_6: cmp-eq s_150_3 s_150_5
        let s_150_6: bool = ((s_150_3) == (s_150_5));
        // D s_150_7: write-var gs#128606 <= s_150_6
        fn_state.gs_128606 = s_150_6;
        // N s_150_8: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_151_0: read-var __SCR_IRQ:u8
        let s_151_0: bool = fn_state.u__SCR_IRQ;
        // D s_151_1: cast zx s_151_0 -> bv
        let s_151_1: Bits = Bits::new(s_151_0 as u128, 1u16);
        // C s_151_2: const #1u : u8
        let s_151_2: bool = true;
        // C s_151_3: cast zx s_151_2 -> bv
        let s_151_3: Bits = Bits::new(s_151_2 as u128, 1u16);
        // D s_151_4: cmp-eq s_151_1 s_151_3
        let s_151_4: bool = ((s_151_1) == (s_151_3));
        // D s_151_5: write-var gs#128605 <= s_151_4
        fn_state.gs_128605 = s_151_4;
        // N s_151_6: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_152_0: read-var __PSTATE_M:u8
        let s_152_0: u8 = fn_state.u__PSTATE_M;
        // D s_152_1: cast zx s_152_0 -> bv
        let s_152_1: Bits = Bits::new(s_152_0 as u128, 5u16);
        // C s_152_2: const #384u : u32
        let s_152_2: u32 = 384;
        // D s_152_3: read-reg s_152_2:u8
        let s_152_3: u8 = {
            let value = state.read_register::<u8>(s_152_2 as isize);
            tracer.read_register(s_152_2 as isize, value);
            value
        };
        // D s_152_4: cast zx s_152_3 -> bv
        let s_152_4: Bits = Bits::new(s_152_3 as u128, 5u16);
        // D s_152_5: cmp-ne s_152_1 s_152_4
        let s_152_5: bool = ((s_152_1) != (s_152_4));
        // D s_152_6: write-var gs#128604 <= s_152_5
        fn_state.gs_128604 = s_152_5;
        // N s_152_7: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_153_0: const #424u : u32
        let s_153_0: u32 = 424;
        // D s_153_1: read-reg s_153_0:u8
        let s_153_1: u8 = {
            let value = state.read_register::<u8>(s_153_0 as isize);
            tracer.read_register(s_153_0 as isize, value);
            value
        };
        // D s_153_2: call ELUsingAArch32(s_153_1)
        let s_153_2: bool = ELUsingAArch32(state, tracer, s_153_1);
        // D s_153_3: write-var gs#128603 <= s_153_2
        fn_state.gs_128603 = s_153_2;
        // N s_153_4: jump b137
        return block_137(state, tracer, fn_state);
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
        // D s_155_1: write-var gs#128607 <= s_155_0
        fn_state.gs_128607 = s_155_0;
        // N s_155_2: jump b156
        return block_156(state, tracer, fn_state);
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_156_0: read-var gs#128607:u8
        let s_156_0: bool = fn_state.gs_128607;
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
        // C s_157_0: const #3u : u8
        let s_157_0: u8 = 3;
        // C s_157_1: cast zx s_157_0 -> bv
        let s_157_1: Bits = Bits::new(s_157_0 as u128, 8u16);
        // C s_157_2: cast zx s_157_1 -> i
        let s_157_2: i128 = (s_157_1.value() as i128);
        // C s_157_3: cast reint s_157_2 -> i64
        let s_157_3: i64 = (s_157_2 as i64);
        // C s_157_4: cast zx s_157_3 -> i
        let s_157_4: i128 = (i128::try_from(s_157_3).unwrap());
        // C s_157_5: const #424u : u32
        let s_157_5: u32 = 424;
        // D s_157_6: read-reg s_157_5:u8
        let s_157_6: u8 = {
            let value = state.read_register::<u8>(s_157_5 as isize);
            tracer.read_register(s_157_5 as isize, value);
            value
        };
        // D s_157_7: call AArch64_AArch32SystemAccessTrap(s_157_6, s_157_4)
        let s_157_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_157_6,
            s_157_4,
        );
        // N s_157_8: return
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
        // D s_159_7: write-var gs#128607 <= s_159_6
        fn_state.gs_128607 = s_159_6;
        // N s_159_8: jump b156
        return block_156(state, tracer, fn_state);
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_160_0: read-var __SCR_EL3_IRQ:u8
        let s_160_0: bool = fn_state.u__SCR_EL3_IRQ;
        // D s_160_1: cast zx s_160_0 -> bv
        let s_160_1: Bits = Bits::new(s_160_0 as u128, 1u16);
        // C s_160_2: const #1u : u8
        let s_160_2: bool = true;
        // C s_160_3: cast zx s_160_2 -> bv
        let s_160_3: Bits = Bits::new(s_160_2 as u128, 1u16);
        // D s_160_4: cmp-eq s_160_1 s_160_3
        let s_160_4: bool = ((s_160_1) == (s_160_3));
        // D s_160_5: write-var gs#128602 <= s_160_4
        fn_state.gs_128602 = s_160_4;
        // N s_160_6: jump b134
        return block_134(state, tracer, fn_state);
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_161_0: const #424u : u32
        let s_161_0: u32 = 424;
        // D s_161_1: read-reg s_161_0:u8
        let s_161_1: u8 = {
            let value = state.read_register::<u8>(s_161_0 as isize);
            tracer.read_register(s_161_0 as isize, value);
            value
        };
        // D s_161_2: call ELUsingAArch32(s_161_1)
        let s_161_2: bool = ELUsingAArch32(state, tracer, s_161_1);
        // D s_161_3: not s_161_2
        let s_161_3: bool = !s_161_2;
        // D s_161_4: write-var gs#128601 <= s_161_3
        fn_state.gs_128601 = s_161_3;
        // N s_161_5: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_162_0: read-var t:i
        let s_162_0: i128 = fn_state.t;
        // D s_162_1: call R_read(s_162_0)
        let s_162_1: u32 = R_read(state, tracer, s_162_0);
        // D s_162_2: call Mk_ICV_BPR1_Type(s_162_1)
        let s_162_2: ProductType700c18a878c5601b = Mk_ICV_BPR1_Type(
            state,
            tracer,
            s_162_1,
        );
        // D s_162_3: call ICV_BPR1_write(s_162_2)
        let s_162_3: () = ICV_BPR1_write(state, tracer, s_162_2);
        // N s_162_4: return
        return;
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_163_0: read-var __HCR_IMO:u8
        let s_163_0: bool = fn_state.u__HCR_IMO;
        // D s_163_1: cast zx s_163_0 -> bv
        let s_163_1: Bits = Bits::new(s_163_0 as u128, 1u16);
        // C s_163_2: const #1u : u8
        let s_163_2: bool = true;
        // C s_163_3: cast zx s_163_2 -> bv
        let s_163_3: Bits = Bits::new(s_163_2 as u128, 1u16);
        // D s_163_4: cmp-eq s_163_1 s_163_3
        let s_163_4: bool = ((s_163_1) == (s_163_3));
        // D s_163_5: write-var gs#128600 <= s_163_4
        fn_state.gs_128600 = s_163_4;
        // N s_163_6: jump b129
        return block_129(state, tracer, fn_state);
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
        // D s_164_3: write-var gs#128599 <= s_164_2
        fn_state.gs_128599 = s_164_2;
        // N s_164_4: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_165_0: read-var t:i
        let s_165_0: i128 = fn_state.t;
        // D s_165_1: call R_read(s_165_0)
        let s_165_1: u32 = R_read(state, tracer, s_165_0);
        // D s_165_2: call Mk_ICV_BPR1_Type(s_165_1)
        let s_165_2: ProductType700c18a878c5601b = Mk_ICV_BPR1_Type(
            state,
            tracer,
            s_165_1,
        );
        // D s_165_3: call ICV_BPR1_write(s_165_2)
        let s_165_3: () = ICV_BPR1_write(state, tracer, s_165_2);
        // N s_165_4: return
        return;
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_166_0: read-var __HCR_EL2_IMO:u8
        let s_166_0: bool = fn_state.u__HCR_EL2_IMO;
        // D s_166_1: cast zx s_166_0 -> bv
        let s_166_1: Bits = Bits::new(s_166_0 as u128, 1u16);
        // C s_166_2: const #1u : u8
        let s_166_2: bool = true;
        // C s_166_3: cast zx s_166_2 -> bv
        let s_166_3: Bits = Bits::new(s_166_2 as u128, 1u16);
        // D s_166_4: cmp-eq s_166_1 s_166_3
        let s_166_4: bool = ((s_166_1) == (s_166_3));
        // D s_166_5: write-var gs#128598 <= s_166_4
        fn_state.gs_128598 = s_166_4;
        // N s_166_6: jump b124
        return block_124(state, tracer, fn_state);
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
        // D s_167_4: write-var gs#128597 <= s_167_3
        fn_state.gs_128597 = s_167_3;
        // N s_167_5: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_168_0: const #3u : u8
        let s_168_0: u8 = 3;
        // C s_168_1: cast zx s_168_0 -> bv
        let s_168_1: Bits = Bits::new(s_168_0 as u128, 8u16);
        // C s_168_2: cast zx s_168_1 -> i
        let s_168_2: i128 = (s_168_1.value() as i128);
        // C s_168_3: cast reint s_168_2 -> i64
        let s_168_3: i64 = (s_168_2 as i64);
        // C s_168_4: cast zx s_168_3 -> i
        let s_168_4: i128 = (i128::try_from(s_168_3).unwrap());
        // S s_168_5: call AArch32_TakeHypTrapException(s_168_4)
        let s_168_5: () = AArch32_TakeHypTrapException(state, tracer, s_168_4);
        // N s_168_6: return
        return;
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_169_0: read-var __ICH_HCR_TALL1:u8
        let s_169_0: bool = fn_state.u__ICH_HCR_TALL1;
        // D s_169_1: cast zx s_169_0 -> bv
        let s_169_1: Bits = Bits::new(s_169_0 as u128, 1u16);
        // C s_169_2: const #1u : u8
        let s_169_2: bool = true;
        // C s_169_3: cast zx s_169_2 -> bv
        let s_169_3: Bits = Bits::new(s_169_2 as u128, 1u16);
        // D s_169_4: cmp-eq s_169_1 s_169_3
        let s_169_4: bool = ((s_169_1) == (s_169_3));
        // D s_169_5: write-var gs#128596 <= s_169_4
        fn_state.gs_128596 = s_169_4;
        // N s_169_6: jump b119
        return block_119(state, tracer, fn_state);
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
        // D s_170_3: write-var gs#128595 <= s_170_2
        fn_state.gs_128595 = s_170_2;
        // N s_170_4: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_171_0: const #3u : u8
        let s_171_0: u8 = 3;
        // C s_171_1: cast zx s_171_0 -> bv
        let s_171_1: Bits = Bits::new(s_171_0 as u128, 8u16);
        // C s_171_2: cast zx s_171_1 -> i
        let s_171_2: i128 = (s_171_1.value() as i128);
        // C s_171_3: cast reint s_171_2 -> i64
        let s_171_3: i64 = (s_171_2 as i64);
        // C s_171_4: cast zx s_171_3 -> i
        let s_171_4: i128 = (i128::try_from(s_171_3).unwrap());
        // C s_171_5: const #432u : u32
        let s_171_5: u32 = 432;
        // D s_171_6: read-reg s_171_5:u8
        let s_171_6: u8 = {
            let value = state.read_register::<u8>(s_171_5 as isize);
            tracer.read_register(s_171_5 as isize, value);
            value
        };
        // D s_171_7: call AArch64_AArch32SystemAccessTrap(s_171_6, s_171_4)
        let s_171_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_171_6,
            s_171_4,
        );
        // N s_171_8: return
        return;
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_172_0: read-var __ICH_HCR_EL2_TALL1:u8
        let s_172_0: bool = fn_state.u__ICH_HCR_EL2_TALL1;
        // D s_172_1: cast zx s_172_0 -> bv
        let s_172_1: Bits = Bits::new(s_172_0 as u128, 1u16);
        // C s_172_2: const #1u : u8
        let s_172_2: bool = true;
        // C s_172_3: cast zx s_172_2 -> bv
        let s_172_3: Bits = Bits::new(s_172_2 as u128, 1u16);
        // D s_172_4: cmp-eq s_172_1 s_172_3
        let s_172_4: bool = ((s_172_1) == (s_172_3));
        // D s_172_5: write-var gs#128594 <= s_172_4
        fn_state.gs_128594 = s_172_4;
        // N s_172_6: jump b114
        return block_114(state, tracer, fn_state);
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
        // D s_173_4: write-var gs#128593 <= s_173_3
        fn_state.gs_128593 = s_173_3;
        // N s_173_5: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_174_0: panic
        panic!("{:?}", ());
        // N s_174_1: return
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
        // S s_175_5: call AArch32_TakeHypTrapException(s_175_4)
        let s_175_5: () = AArch32_TakeHypTrapException(state, tracer, s_175_4);
        // N s_175_6: return
        return;
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_176_0: read-var __HSTR_T12:u8
        let s_176_0: bool = fn_state.u__HSTR_T12;
        // D s_176_1: cast zx s_176_0 -> bv
        let s_176_1: Bits = Bits::new(s_176_0 as u128, 1u16);
        // C s_176_2: const #1u : u8
        let s_176_2: bool = true;
        // C s_176_3: cast zx s_176_2 -> bv
        let s_176_3: Bits = Bits::new(s_176_2 as u128, 1u16);
        // D s_176_4: cmp-eq s_176_1 s_176_3
        let s_176_4: bool = ((s_176_1) == (s_176_3));
        // D s_176_5: write-var gs#128592 <= s_176_4
        fn_state.gs_128592 = s_176_4;
        // N s_176_6: jump b108
        return block_108(state, tracer, fn_state);
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
        // D s_177_3: write-var gs#128591 <= s_177_2
        fn_state.gs_128591 = s_177_2;
        // N s_177_4: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_178_0: const #3u : u8
        let s_178_0: u8 = 3;
        // C s_178_1: cast zx s_178_0 -> bv
        let s_178_1: Bits = Bits::new(s_178_0 as u128, 8u16);
        // C s_178_2: cast zx s_178_1 -> i
        let s_178_2: i128 = (s_178_1.value() as i128);
        // C s_178_3: cast reint s_178_2 -> i64
        let s_178_3: i64 = (s_178_2 as i64);
        // C s_178_4: cast zx s_178_3 -> i
        let s_178_4: i128 = (i128::try_from(s_178_3).unwrap());
        // C s_178_5: const #432u : u32
        let s_178_5: u32 = 432;
        // D s_178_6: read-reg s_178_5:u8
        let s_178_6: u8 = {
            let value = state.read_register::<u8>(s_178_5 as isize);
            tracer.read_register(s_178_5 as isize, value);
            value
        };
        // D s_178_7: call AArch64_AArch32SystemAccessTrap(s_178_6, s_178_4)
        let s_178_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_178_6,
            s_178_4,
        );
        // N s_178_8: return
        return;
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_179_0: read-var __HSTR_EL2_T12:u8
        let s_179_0: bool = fn_state.u__HSTR_EL2_T12;
        // D s_179_1: cast zx s_179_0 -> bv
        let s_179_1: Bits = Bits::new(s_179_0 as u128, 1u16);
        // C s_179_2: const #1u : u8
        let s_179_2: bool = true;
        // C s_179_3: cast zx s_179_2 -> bv
        let s_179_3: Bits = Bits::new(s_179_2 as u128, 1u16);
        // D s_179_4: cmp-eq s_179_1 s_179_3
        let s_179_4: bool = ((s_179_1) == (s_179_3));
        // D s_179_5: write-var gs#128590 <= s_179_4
        fn_state.gs_128590 = s_179_4;
        // N s_179_6: jump b103
        return block_103(state, tracer, fn_state);
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
        // D s_180_4: write-var gs#128589 <= s_180_3
        fn_state.gs_128589 = s_180_3;
        // N s_180_5: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_181_0: panic
        panic!("{:?}", ());
        // N s_181_1: return
        return;
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_182_0: read-var __SCR_IRQ:u8
        let s_182_0: bool = fn_state.u__SCR_IRQ;
        // D s_182_1: cast zx s_182_0 -> bv
        let s_182_1: Bits = Bits::new(s_182_0 as u128, 1u16);
        // C s_182_2: const #1u : u8
        let s_182_2: bool = true;
        // C s_182_3: cast zx s_182_2 -> bv
        let s_182_3: Bits = Bits::new(s_182_2 as u128, 1u16);
        // D s_182_4: cmp-eq s_182_1 s_182_3
        let s_182_4: bool = ((s_182_1) == (s_182_3));
        // D s_182_5: write-var gs#128588 <= s_182_4
        fn_state.gs_128588 = s_182_4;
        // N s_182_6: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_183_0: read-var __PSTATE_M:u8
        let s_183_0: u8 = fn_state.u__PSTATE_M;
        // D s_183_1: cast zx s_183_0 -> bv
        let s_183_1: Bits = Bits::new(s_183_0 as u128, 5u16);
        // C s_183_2: const #384u : u32
        let s_183_2: u32 = 384;
        // D s_183_3: read-reg s_183_2:u8
        let s_183_3: u8 = {
            let value = state.read_register::<u8>(s_183_2 as isize);
            tracer.read_register(s_183_2 as isize, value);
            value
        };
        // D s_183_4: cast zx s_183_3 -> bv
        let s_183_4: Bits = Bits::new(s_183_3 as u128, 5u16);
        // D s_183_5: cmp-ne s_183_1 s_183_4
        let s_183_5: bool = ((s_183_1) != (s_183_4));
        // D s_183_6: write-var gs#128587 <= s_183_5
        fn_state.gs_128587 = s_183_5;
        // N s_183_7: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_184_0: const #424u : u32
        let s_184_0: u32 = 424;
        // D s_184_1: read-reg s_184_0:u8
        let s_184_1: u8 = {
            let value = state.read_register::<u8>(s_184_0 as isize);
            tracer.read_register(s_184_0 as isize, value);
            value
        };
        // D s_184_2: call ELUsingAArch32(s_184_1)
        let s_184_2: bool = ELUsingAArch32(state, tracer, s_184_1);
        // D s_184_3: write-var gs#128586 <= s_184_2
        fn_state.gs_128586 = s_184_2;
        // N s_184_4: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_185<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_185_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_185_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_185_1: call __IMPDEF_boolean(s_185_0)
        let s_185_1: bool = u__IMPDEF_boolean(state, tracer, s_185_0);
        // D s_185_2: write-var gs#128585 <= s_185_1
        fn_state.gs_128585 = s_185_1;
        // N s_185_3: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_186_0: const #() : ()
        let s_186_0: () = ();
        // S s_186_1: call EDSCR_read(s_186_0)
        let s_186_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_186_0);
        // S s_186_2: call _get_EDSCR_Type_SDD(s_186_1)
        let s_186_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_186_1);
        // S s_186_3: cast zx s_186_2 -> bv
        let s_186_3: Bits = Bits::new(s_186_2 as u128, 1u16);
        // C s_186_4: const #1u : u8
        let s_186_4: bool = true;
        // C s_186_5: cast zx s_186_4 -> bv
        let s_186_5: Bits = Bits::new(s_186_4 as u128, 1u16);
        // S s_186_6: cmp-eq s_186_3 s_186_5
        let s_186_6: bool = ((s_186_3) == (s_186_5));
        // D s_186_7: write-var gs#128584 <= s_186_6
        fn_state.gs_128584 = s_186_6;
        // N s_186_8: jump b90
        return block_90(state, tracer, fn_state);
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
        // D s_187_4: write-var gs#128583 <= s_187_3
        fn_state.gs_128583 = s_187_3;
        // N s_187_5: jump b88
        return block_88(state, tracer, fn_state);
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
        // D s_189_0: read-var __SCR_EL3_IRQ:u8
        let s_189_0: bool = fn_state.u__SCR_EL3_IRQ;
        // D s_189_1: cast zx s_189_0 -> bv
        let s_189_1: Bits = Bits::new(s_189_0 as u128, 1u16);
        // C s_189_2: const #1u : u8
        let s_189_2: bool = true;
        // C s_189_3: cast zx s_189_2 -> bv
        let s_189_3: Bits = Bits::new(s_189_2 as u128, 1u16);
        // D s_189_4: cmp-eq s_189_1 s_189_3
        let s_189_4: bool = ((s_189_1) == (s_189_3));
        // D s_189_5: write-var gs#128582 <= s_189_4
        fn_state.gs_128582 = s_189_4;
        // N s_189_6: jump b85
        return block_85(state, tracer, fn_state);
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
        // D s_190_4: write-var gs#128581 <= s_190_3
        fn_state.gs_128581 = s_190_3;
        // N s_190_5: jump b83
        return block_83(state, tracer, fn_state);
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
        // D s_191_2: write-var gs#128580 <= s_191_1
        fn_state.gs_128580 = s_191_1;
        // N s_191_3: jump b81
        return block_81(state, tracer, fn_state);
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
        // D s_192_7: write-var gs#128579 <= s_192_6
        fn_state.gs_128579 = s_192_6;
        // N s_192_8: jump b79
        return block_79(state, tracer, fn_state);
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
        // D s_193_4: write-var gs#128578 <= s_193_3
        fn_state.gs_128578 = s_193_3;
        // N s_193_5: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_194_0: panic
        panic!("{:?}", ());
        // N s_194_1: return
        return;
    }
}
