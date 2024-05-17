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
use ICC_EOIR1_write::*;
use ICC_HSRE_read::*;
use ICC_SRE_read::*;
use R_read::*;
use ICV_EOIR1_write::*;
use Mk_ICV_EOIR1_Type::*;
use Mk_ICC_EOIR1_Type::*;
use u_get_ICH_HCR_Type_TALL1::*;
use u_get_EDSCR_Type_SDD::*;
use EL2Enabled::*;
use u_get_HCR_Type_IMO::*;
use u_get_HSTR_Type_T12::*;
use u_get_ICC_MSRE_Type_SRE::*;
use HCR_read::*;
use u_get_SCR_EL3_Type_IRQ::*;
use u_get_ICC_HSRE_Type_SRE::*;
use u__IMPDEF_boolean::*;
use u_get_HSTR_EL2_Type_T12::*;
use HSTR_read::*;
use AArch64_AArch32SystemAccessTrap::*;
use u_get_ICH_HCR_EL2_Type_TALL1::*;
use ELUsingAArch32::*;
use u_get_HCR_EL2_Type_IMO::*;
use ICH_HCR_read::*;
use AArch32_TakeMonitorTrapException::*;
use EDSCR_read::*;
use common::*;
pub fn ICC_EOIR1_SysRegWrite32_b67c881f4f810bd8<T: Tracer>(
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
        u__HCR_EL2_IMO: bool,
        gs_128687: bool,
        gs_128700: bool,
        gs_128695: bool,
        u__SCR_IRQ: bool,
        gs_128693: bool,
        gs_128698: bool,
        u__ICH_HCR_EL2_TALL1: bool,
        gs_128683: bool,
        gs_128667: bool,
        gs_128678: bool,
        u__ICH_HCR_TALL1: bool,
        gs_128664: bool,
        gs_128704: bool,
        gs_128679: bool,
        gs_128694: bool,
        gs_128672: bool,
        gs_128696: bool,
        u__HSTR_EL2_T12: bool,
        gs_128669: bool,
        gs_128705: bool,
        gs_128684: bool,
        gs_128668: bool,
        u__PSTATE_EL: u8,
        gs_128675: bool,
        gs_128703: bool,
        gs_128699: bool,
        gs_128701: bool,
        gs_128691: bool,
        u__ICC_HSRE_SRE: bool,
        u__HSTR_T12: bool,
        gs_128686: bool,
        gs_128688: bool,
        gs_128692: bool,
        u__HCR_IMO: bool,
        gs_128689: bool,
        gs_128671: bool,
        gs_128673: bool,
        gs_128682: bool,
        gs_128663: bool,
        gs_128665: bool,
        u__ICC_MSRE_SRE: bool,
        gs_128697: bool,
        gs_128685: bool,
        gs_128690: bool,
        gs_128681: bool,
        gs_128680: bool,
        gs_128677: bool,
        gs_128670: bool,
        gs_128666: bool,
        u__ICC_SRE_SRE: bool,
        u__PSTATE_M: u8,
        gs_128676: bool,
        gs_128674: bool,
        gs_128706: bool,
        gs_128662: bool,
        gs_128707: bool,
        gs_128702: bool,
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
        // N s_0_56: branch s_0_55 b188 b1
        if s_0_55 {
            return block_188(state, tracer, fn_state);
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
        // D s_6_0: read-var t:i
        let s_6_0: i128 = fn_state.t;
        // D s_6_1: call R_read(s_6_0)
        let s_6_1: u32 = R_read(state, tracer, s_6_0);
        // D s_6_2: call Mk_ICC_EOIR1_Type(s_6_1)
        let s_6_2: ProductType700c18a878c5601b = Mk_ICC_EOIR1_Type(state, tracer, s_6_1);
        // D s_6_3: call ICC_EOIR1_write(s_6_2)
        let s_6_3: () = ICC_EOIR1_write(state, tracer, s_6_2);
        // N s_6_4: return
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
        // D s_9_1: write-var gs#128662 <= s_9_0
        fn_state.gs_128662 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#128662:u8
        let s_10_0: bool = fn_state.gs_128662;
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
        // D s_11_1: write-var gs#128663 <= s_11_0
        fn_state.gs_128663 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#128663:u8
        let s_12_0: bool = fn_state.gs_128663;
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
        // D s_13_1: write-var gs#128664 <= s_13_0
        fn_state.gs_128664 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#128664:u8
        let s_14_0: bool = fn_state.gs_128664;
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
        // D s_15_1: write-var gs#128665 <= s_15_0
        fn_state.gs_128665 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#128665:u8
        let s_16_0: bool = fn_state.gs_128665;
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
        // D s_17_1: write-var gs#128666 <= s_17_0
        fn_state.gs_128666 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#128666:u8
        let s_18_0: bool = fn_state.gs_128666;
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
        // D s_20_1: write-var gs#128667 <= s_20_0
        fn_state.gs_128667 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#128667:u8
        let s_21_0: bool = fn_state.gs_128667;
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
        // D s_22_1: write-var gs#128668 <= s_22_0
        fn_state.gs_128668 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#128668:u8
        let s_23_0: bool = fn_state.gs_128668;
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
        // D s_24_1: write-var gs#128669 <= s_24_0
        fn_state.gs_128669 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#128669:u8
        let s_25_0: bool = fn_state.gs_128669;
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
        // D s_26_1: write-var gs#128670 <= s_26_0
        fn_state.gs_128670 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#128670:u8
        let s_27_0: bool = fn_state.gs_128670;
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
        // D s_28_1: write-var gs#128671 <= s_28_0
        fn_state.gs_128671 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#128671:u8
        let s_29_0: bool = fn_state.gs_128671;
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
        // D s_32_1: write-var gs#128672 <= s_32_0
        fn_state.gs_128672 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#128672:u8
        let s_33_0: bool = fn_state.gs_128672;
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
        // D s_34_1: write-var gs#128673 <= s_34_0
        fn_state.gs_128673 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#128673:u8
        let s_35_0: bool = fn_state.gs_128673;
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
        // D s_37_1: write-var gs#128674 <= s_37_0
        fn_state.gs_128674 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#128674:u8
        let s_38_0: bool = fn_state.gs_128674;
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
        // D s_39_1: write-var gs#128675 <= s_39_0
        fn_state.gs_128675 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#128675:u8
        let s_40_0: bool = fn_state.gs_128675;
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
        // D s_41_0: read-var t:i
        let s_41_0: i128 = fn_state.t;
        // D s_41_1: call R_read(s_41_0)
        let s_41_1: u32 = R_read(state, tracer, s_41_0);
        // D s_41_2: call Mk_ICC_EOIR1_Type(s_41_1)
        let s_41_2: ProductType700c18a878c5601b = Mk_ICC_EOIR1_Type(
            state,
            tracer,
            s_41_1,
        );
        // D s_41_3: call ICC_EOIR1_write(s_41_2)
        let s_41_3: () = ICC_EOIR1_write(state, tracer, s_41_2);
        // N s_41_4: return
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
        // D s_43_1: write-var gs#128676 <= s_43_0
        fn_state.gs_128676 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#128676:u8
        let s_44_0: bool = fn_state.gs_128676;
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
        // D s_47_7: write-var gs#128676 <= s_47_6
        fn_state.gs_128676 = s_47_6;
        // N s_47_8: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var __SCR_IRQ:u8
        let s_48_0: bool = fn_state.u__SCR_IRQ;
        // D s_48_1: cast zx s_48_0 -> bv
        let s_48_1: Bits = Bits::new(s_48_0 as u128, 1u16);
        // C s_48_2: const #1u : u8
        let s_48_2: bool = true;
        // C s_48_3: cast zx s_48_2 -> bv
        let s_48_3: Bits = Bits::new(s_48_2 as u128, 1u16);
        // D s_48_4: cmp-eq s_48_1 s_48_3
        let s_48_4: bool = ((s_48_1) == (s_48_3));
        // D s_48_5: write-var gs#128675 <= s_48_4
        fn_state.gs_128675 = s_48_4;
        // N s_48_6: jump b40
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
        // D s_49_3: write-var gs#128674 <= s_49_2
        fn_state.gs_128674 = s_49_2;
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
        // D s_51_1: write-var gs#128677 <= s_51_0
        fn_state.gs_128677 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#128677:u8
        let s_52_0: bool = fn_state.gs_128677;
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
        // D s_55_7: write-var gs#128677 <= s_55_6
        fn_state.gs_128677 = s_55_6;
        // N s_55_8: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var __SCR_EL3_IRQ:u8
        let s_56_0: bool = fn_state.u__SCR_EL3_IRQ;
        // D s_56_1: cast zx s_56_0 -> bv
        let s_56_1: Bits = Bits::new(s_56_0 as u128, 1u16);
        // C s_56_2: const #1u : u8
        let s_56_2: bool = true;
        // C s_56_3: cast zx s_56_2 -> bv
        let s_56_3: Bits = Bits::new(s_56_2 as u128, 1u16);
        // D s_56_4: cmp-eq s_56_1 s_56_3
        let s_56_4: bool = ((s_56_1) == (s_56_3));
        // D s_56_5: write-var gs#128673 <= s_56_4
        fn_state.gs_128673 = s_56_4;
        // N s_56_6: jump b35
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
        // D s_57_4: write-var gs#128672 <= s_57_3
        fn_state.gs_128672 = s_57_3;
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
        // D s_60_0: read-var __SCR_IRQ:u8
        let s_60_0: bool = fn_state.u__SCR_IRQ;
        // D s_60_1: cast zx s_60_0 -> bv
        let s_60_1: Bits = Bits::new(s_60_0 as u128, 1u16);
        // C s_60_2: const #1u : u8
        let s_60_2: bool = true;
        // C s_60_3: cast zx s_60_2 -> bv
        let s_60_3: Bits = Bits::new(s_60_2 as u128, 1u16);
        // D s_60_4: cmp-eq s_60_1 s_60_3
        let s_60_4: bool = ((s_60_1) == (s_60_3));
        // D s_60_5: write-var gs#128671 <= s_60_4
        fn_state.gs_128671 = s_60_4;
        // N s_60_6: jump b29
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
        // D s_61_3: write-var gs#128670 <= s_61_2
        fn_state.gs_128670 = s_61_2;
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
        // D s_62_2: write-var gs#128669 <= s_62_1
        fn_state.gs_128669 = s_62_1;
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
        // D s_63_7: write-var gs#128668 <= s_63_6
        fn_state.gs_128668 = s_63_6;
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
        // D s_64_4: write-var gs#128667 <= s_64_3
        fn_state.gs_128667 = s_64_3;
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
        // D s_66_0: read-var __SCR_EL3_IRQ:u8
        let s_66_0: bool = fn_state.u__SCR_EL3_IRQ;
        // D s_66_1: cast zx s_66_0 -> bv
        let s_66_1: Bits = Bits::new(s_66_0 as u128, 1u16);
        // C s_66_2: const #1u : u8
        let s_66_2: bool = true;
        // C s_66_3: cast zx s_66_2 -> bv
        let s_66_3: Bits = Bits::new(s_66_2 as u128, 1u16);
        // D s_66_4: cmp-eq s_66_1 s_66_3
        let s_66_4: bool = ((s_66_1) == (s_66_3));
        // D s_66_5: write-var gs#128666 <= s_66_4
        fn_state.gs_128666 = s_66_4;
        // N s_66_6: jump b18
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
        // D s_67_4: write-var gs#128665 <= s_67_3
        fn_state.gs_128665 = s_67_3;
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
        // D s_68_2: write-var gs#128664 <= s_68_1
        fn_state.gs_128664 = s_68_1;
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
        // D s_69_7: write-var gs#128663 <= s_69_6
        fn_state.gs_128663 = s_69_6;
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
        // D s_70_4: write-var gs#128662 <= s_70_3
        fn_state.gs_128662 = s_70_3;
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
        // N s_71_2: branch s_71_1 b187 b72
        if s_71_1 {
            return block_187(state, tracer, fn_state);
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
        // D s_72_1: write-var gs#128678 <= s_72_0
        fn_state.gs_128678 = s_72_0;
        // N s_72_2: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var gs#128678:u8
        let s_73_0: bool = fn_state.gs_128678;
        // N s_73_1: branch s_73_0 b186 b74
        if s_73_0 {
            return block_186(state, tracer, fn_state);
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
        // D s_74_1: write-var gs#128679 <= s_74_0
        fn_state.gs_128679 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var gs#128679:u8
        let s_75_0: bool = fn_state.gs_128679;
        // N s_75_1: branch s_75_0 b185 b76
        if s_75_0 {
            return block_185(state, tracer, fn_state);
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
        // D s_76_1: write-var gs#128680 <= s_76_0
        fn_state.gs_128680 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var gs#128680:u8
        let s_77_0: bool = fn_state.gs_128680;
        // N s_77_1: branch s_77_0 b184 b78
        if s_77_0 {
            return block_184(state, tracer, fn_state);
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
        // D s_78_1: write-var gs#128681 <= s_78_0
        fn_state.gs_128681 = s_78_0;
        // N s_78_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var gs#128681:u8
        let s_79_0: bool = fn_state.gs_128681;
        // N s_79_1: branch s_79_0 b183 b80
        if s_79_0 {
            return block_183(state, tracer, fn_state);
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
        // D s_80_1: write-var gs#128682 <= s_80_0
        fn_state.gs_128682 = s_80_0;
        // N s_80_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#128682:u8
        let s_81_0: bool = fn_state.gs_128682;
        // N s_81_1: branch s_81_0 b182 b82
        if s_81_0 {
            return block_182(state, tracer, fn_state);
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
        // N s_82_2: branch s_82_1 b181 b83
        if s_82_1 {
            return block_181(state, tracer, fn_state);
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
        // D s_83_1: write-var gs#128683 <= s_83_0
        fn_state.gs_128683 = s_83_0;
        // N s_83_2: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var gs#128683:u8
        let s_84_0: bool = fn_state.gs_128683;
        // N s_84_1: branch s_84_0 b180 b85
        if s_84_0 {
            return block_180(state, tracer, fn_state);
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
        // D s_85_1: write-var gs#128684 <= s_85_0
        fn_state.gs_128684 = s_85_0;
        // N s_85_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var gs#128684:u8
        let s_86_0: bool = fn_state.gs_128684;
        // N s_86_1: branch s_86_0 b179 b87
        if s_86_0 {
            return block_179(state, tracer, fn_state);
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
        // D s_87_1: write-var gs#128685 <= s_87_0
        fn_state.gs_128685 = s_87_0;
        // N s_87_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var gs#128685:u8
        let s_88_0: bool = fn_state.gs_128685;
        // N s_88_1: branch s_88_0 b178 b89
        if s_88_0 {
            return block_178(state, tracer, fn_state);
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
        // D s_89_1: write-var gs#128686 <= s_89_0
        fn_state.gs_128686 = s_89_0;
        // N s_89_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var gs#128686:u8
        let s_90_0: bool = fn_state.gs_128686;
        // N s_90_1: branch s_90_0 b177 b91
        if s_90_0 {
            return block_177(state, tracer, fn_state);
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
        // D s_91_1: write-var gs#128687 <= s_91_0
        fn_state.gs_128687 = s_91_0;
        // N s_91_2: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var gs#128687:u8
        let s_92_0: bool = fn_state.gs_128687;
        // N s_92_1: branch s_92_0 b176 b93
        if s_92_0 {
            return block_176(state, tracer, fn_state);
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
        // D s_93_1: write-var gs#128688 <= s_93_0
        fn_state.gs_128688 = s_93_0;
        // N s_93_2: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var gs#128688:u8
        let s_94_0: bool = fn_state.gs_128688;
        // N s_94_1: branch s_94_0 b175 b95
        if s_94_0 {
            return block_175(state, tracer, fn_state);
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
        // N s_95_2: branch s_95_1 b174 b96
        if s_95_1 {
            return block_174(state, tracer, fn_state);
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
        // D s_96_1: write-var gs#128689 <= s_96_0
        fn_state.gs_128689 = s_96_0;
        // N s_96_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var gs#128689:u8
        let s_97_0: bool = fn_state.gs_128689;
        // N s_97_1: branch s_97_0 b173 b98
        if s_97_0 {
            return block_173(state, tracer, fn_state);
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
        // D s_98_1: write-var gs#128690 <= s_98_0
        fn_state.gs_128690 = s_98_0;
        // N s_98_2: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_99_0: read-var gs#128690:u8
        let s_99_0: bool = fn_state.gs_128690;
        // N s_99_1: branch s_99_0 b172 b100
        if s_99_0 {
            return block_172(state, tracer, fn_state);
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
        // N s_100_2: branch s_100_1 b171 b101
        if s_100_1 {
            return block_171(state, tracer, fn_state);
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
        // D s_101_1: write-var gs#128691 <= s_101_0
        fn_state.gs_128691 = s_101_0;
        // N s_101_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var gs#128691:u8
        let s_102_0: bool = fn_state.gs_128691;
        // N s_102_1: branch s_102_0 b170 b103
        if s_102_0 {
            return block_170(state, tracer, fn_state);
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
        // D s_103_1: write-var gs#128692 <= s_103_0
        fn_state.gs_128692 = s_103_0;
        // N s_103_2: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_104_0: read-var gs#128692:u8
        let s_104_0: bool = fn_state.gs_128692;
        // N s_104_1: branch s_104_0 b169 b105
        if s_104_0 {
            return block_169(state, tracer, fn_state);
        } else {
            return block_105(state, tracer, fn_state);
        };
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_105_0: read-var __ICC_SRE_SRE:u8
        let s_105_0: bool = fn_state.u__ICC_SRE_SRE;
        // D s_105_1: cast zx s_105_0 -> bv
        let s_105_1: Bits = Bits::new(s_105_0 as u128, 1u16);
        // C s_105_2: const #0u : u8
        let s_105_2: bool = false;
        // C s_105_3: cast zx s_105_2 -> bv
        let s_105_3: Bits = Bits::new(s_105_2 as u128, 1u16);
        // D s_105_4: cmp-eq s_105_1 s_105_3
        let s_105_4: bool = ((s_105_1) == (s_105_3));
        // N s_105_5: branch s_105_4 b168 b106
        if s_105_4 {
            return block_168(state, tracer, fn_state);
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
        // N s_106_2: branch s_106_1 b167 b107
        if s_106_1 {
            return block_167(state, tracer, fn_state);
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
        // D s_107_1: write-var gs#128693 <= s_107_0
        fn_state.gs_128693 = s_107_0;
        // N s_107_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var gs#128693:u8
        let s_108_0: bool = fn_state.gs_128693;
        // N s_108_1: branch s_108_0 b166 b109
        if s_108_0 {
            return block_166(state, tracer, fn_state);
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
        // D s_109_1: write-var gs#128694 <= s_109_0
        fn_state.gs_128694 = s_109_0;
        // N s_109_2: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_110_0: read-var gs#128694:u8
        let s_110_0: bool = fn_state.gs_128694;
        // N s_110_1: branch s_110_0 b165 b111
        if s_110_0 {
            return block_165(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_111_0: const #() : ()
        let s_111_0: () = ();
        // S s_111_1: call EL2Enabled(s_111_0)
        let s_111_1: bool = EL2Enabled(state, tracer, s_111_0);
        // N s_111_2: branch s_111_1 b164 b112
        if s_111_1 {
            return block_164(state, tracer, fn_state);
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
        // D s_112_1: write-var gs#128695 <= s_112_0
        fn_state.gs_128695 = s_112_0;
        // N s_112_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var gs#128695:u8
        let s_113_0: bool = fn_state.gs_128695;
        // N s_113_1: branch s_113_0 b163 b114
        if s_113_0 {
            return block_163(state, tracer, fn_state);
        } else {
            return block_114(state, tracer, fn_state);
        };
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #0u : u8
        let s_114_0: bool = false;
        // D s_114_1: write-var gs#128696 <= s_114_0
        fn_state.gs_128696 = s_114_0;
        // N s_114_2: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_115_0: read-var gs#128696:u8
        let s_115_0: bool = fn_state.gs_128696;
        // N s_115_1: branch s_115_0 b162 b116
        if s_115_0 {
            return block_162(state, tracer, fn_state);
        } else {
            return block_116(state, tracer, fn_state);
        };
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #() : ()
        let s_116_0: () = ();
        // S s_116_1: call EL2Enabled(s_116_0)
        let s_116_1: bool = EL2Enabled(state, tracer, s_116_0);
        // N s_116_2: branch s_116_1 b161 b117
        if s_116_1 {
            return block_161(state, tracer, fn_state);
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
        // D s_117_1: write-var gs#128697 <= s_117_0
        fn_state.gs_128697 = s_117_0;
        // N s_117_2: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_118_0: read-var gs#128697:u8
        let s_118_0: bool = fn_state.gs_128697;
        // N s_118_1: branch s_118_0 b160 b119
        if s_118_0 {
            return block_160(state, tracer, fn_state);
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
        // D s_119_1: write-var gs#128698 <= s_119_0
        fn_state.gs_128698 = s_119_0;
        // N s_119_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_120_0: read-var gs#128698:u8
        let s_120_0: bool = fn_state.gs_128698;
        // N s_120_1: branch s_120_0 b159 b121
        if s_120_0 {
            return block_159(state, tracer, fn_state);
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
        // N s_121_2: branch s_121_1 b158 b122
        if s_121_1 {
            return block_158(state, tracer, fn_state);
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
        // D s_122_1: write-var gs#128699 <= s_122_0
        fn_state.gs_128699 = s_122_0;
        // N s_122_2: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_123_0: read-var gs#128699:u8
        let s_123_0: bool = fn_state.gs_128699;
        // N s_123_1: branch s_123_0 b157 b124
        if s_123_0 {
            return block_157(state, tracer, fn_state);
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
        // D s_124_1: write-var gs#128700 <= s_124_0
        fn_state.gs_128700 = s_124_0;
        // N s_124_2: jump b125
        return block_125(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_125_0: read-var gs#128700:u8
        let s_125_0: bool = fn_state.gs_128700;
        // N s_125_1: branch s_125_0 b156 b126
        if s_125_0 {
            return block_156(state, tracer, fn_state);
        } else {
            return block_126(state, tracer, fn_state);
        };
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_126_0: const #424u : u32
        let s_126_0: u32 = 424;
        // D s_126_1: read-reg s_126_0:u8
        let s_126_1: u8 = {
            let value = state.read_register::<u8>(s_126_0 as isize);
            tracer.read_register(s_126_0 as isize, value);
            value
        };
        // C s_126_2: const #2u : u8
        let s_126_2: u8 = 2;
        // D s_126_3: cmp-lt s_126_1 s_126_2
        let s_126_3: bool = ((s_126_1) < (s_126_2));
        // N s_126_4: branch s_126_3 b155 b127
        if s_126_3 {
            return block_155(state, tracer, fn_state);
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
        // D s_127_1: write-var gs#128701 <= s_127_0
        fn_state.gs_128701 = s_127_0;
        // N s_127_2: jump b128
        return block_128(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_128_0: read-var gs#128701:u8
        let s_128_0: bool = fn_state.gs_128701;
        // N s_128_1: branch s_128_0 b154 b129
        if s_128_0 {
            return block_154(state, tracer, fn_state);
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
        // D s_129_1: write-var gs#128702 <= s_129_0
        fn_state.gs_128702 = s_129_0;
        // N s_129_2: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_130_0: read-var gs#128702:u8
        let s_130_0: bool = fn_state.gs_128702;
        // N s_130_1: branch s_130_0 b148 b131
        if s_130_0 {
            return block_148(state, tracer, fn_state);
        } else {
            return block_131(state, tracer, fn_state);
        };
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_131_0: const #424u : u32
        let s_131_0: u32 = 424;
        // D s_131_1: read-reg s_131_0:u8
        let s_131_1: u8 = {
            let value = state.read_register::<u8>(s_131_0 as isize);
            tracer.read_register(s_131_0 as isize, value);
            value
        };
        // C s_131_2: const #2u : u8
        let s_131_2: u8 = 2;
        // D s_131_3: cmp-lt s_131_1 s_131_2
        let s_131_3: bool = ((s_131_1) < (s_131_2));
        // N s_131_4: branch s_131_3 b147 b132
        if s_131_3 {
            return block_147(state, tracer, fn_state);
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
        // D s_132_1: write-var gs#128703 <= s_132_0
        fn_state.gs_128703 = s_132_0;
        // N s_132_2: jump b133
        return block_133(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_133_0: read-var gs#128703:u8
        let s_133_0: bool = fn_state.gs_128703;
        // N s_133_1: branch s_133_0 b146 b134
        if s_133_0 {
            return block_146(state, tracer, fn_state);
        } else {
            return block_134(state, tracer, fn_state);
        };
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_134_0: const #0u : u8
        let s_134_0: bool = false;
        // D s_134_1: write-var gs#128704 <= s_134_0
        fn_state.gs_128704 = s_134_0;
        // N s_134_2: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_135_0: read-var gs#128704:u8
        let s_135_0: bool = fn_state.gs_128704;
        // N s_135_1: branch s_135_0 b145 b136
        if s_135_0 {
            return block_145(state, tracer, fn_state);
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
        // D s_136_1: write-var gs#128705 <= s_136_0
        fn_state.gs_128705 = s_136_0;
        // N s_136_2: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_137_0: read-var gs#128705:u8
        let s_137_0: bool = fn_state.gs_128705;
        // N s_137_1: branch s_137_0 b139 b138
        if s_137_0 {
            return block_139(state, tracer, fn_state);
        } else {
            return block_138(state, tracer, fn_state);
        };
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_138_0: read-var t:i
        let s_138_0: i128 = fn_state.t;
        // D s_138_1: call R_read(s_138_0)
        let s_138_1: u32 = R_read(state, tracer, s_138_0);
        // D s_138_2: call Mk_ICC_EOIR1_Type(s_138_1)
        let s_138_2: ProductType700c18a878c5601b = Mk_ICC_EOIR1_Type(
            state,
            tracer,
            s_138_1,
        );
        // D s_138_3: call ICC_EOIR1_write(s_138_2)
        let s_138_3: () = ICC_EOIR1_write(state, tracer, s_138_2);
        // N s_138_4: return
        return;
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
        // N s_139_2: branch s_139_1 b144 b140
        if s_139_1 {
            return block_144(state, tracer, fn_state);
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
        // D s_140_1: write-var gs#128706 <= s_140_0
        fn_state.gs_128706 = s_140_0;
        // N s_140_2: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_141_0: read-var gs#128706:u8
        let s_141_0: bool = fn_state.gs_128706;
        // N s_141_1: branch s_141_0 b143 b142
        if s_141_0 {
            return block_143(state, tracer, fn_state);
        } else {
            return block_142(state, tracer, fn_state);
        };
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_142_0: const #() : ()
        let s_142_0: () = ();
        // S s_142_1: call AArch32_TakeMonitorTrapException(s_142_0)
        let s_142_1: () = AArch32_TakeMonitorTrapException(state, tracer, s_142_0);
        // N s_142_2: return
        return;
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_143_0: panic
        panic!("{:?}", ());
        // N s_143_1: return
        return;
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_144_0: const #() : ()
        let s_144_0: () = ();
        // S s_144_1: call EDSCR_read(s_144_0)
        let s_144_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_144_0);
        // S s_144_2: call _get_EDSCR_Type_SDD(s_144_1)
        let s_144_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_144_1);
        // S s_144_3: cast zx s_144_2 -> bv
        let s_144_3: Bits = Bits::new(s_144_2 as u128, 1u16);
        // C s_144_4: const #1u : u8
        let s_144_4: bool = true;
        // C s_144_5: cast zx s_144_4 -> bv
        let s_144_5: Bits = Bits::new(s_144_4 as u128, 1u16);
        // S s_144_6: cmp-eq s_144_3 s_144_5
        let s_144_6: bool = ((s_144_3) == (s_144_5));
        // D s_144_7: write-var gs#128706 <= s_144_6
        fn_state.gs_128706 = s_144_6;
        // N s_144_8: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_145_0: read-var __SCR_IRQ:u8
        let s_145_0: bool = fn_state.u__SCR_IRQ;
        // D s_145_1: cast zx s_145_0 -> bv
        let s_145_1: Bits = Bits::new(s_145_0 as u128, 1u16);
        // C s_145_2: const #1u : u8
        let s_145_2: bool = true;
        // C s_145_3: cast zx s_145_2 -> bv
        let s_145_3: Bits = Bits::new(s_145_2 as u128, 1u16);
        // D s_145_4: cmp-eq s_145_1 s_145_3
        let s_145_4: bool = ((s_145_1) == (s_145_3));
        // D s_145_5: write-var gs#128705 <= s_145_4
        fn_state.gs_128705 = s_145_4;
        // N s_145_6: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_146_0: read-var __PSTATE_M:u8
        let s_146_0: u8 = fn_state.u__PSTATE_M;
        // D s_146_1: cast zx s_146_0 -> bv
        let s_146_1: Bits = Bits::new(s_146_0 as u128, 5u16);
        // C s_146_2: const #384u : u32
        let s_146_2: u32 = 384;
        // D s_146_3: read-reg s_146_2:u8
        let s_146_3: u8 = {
            let value = state.read_register::<u8>(s_146_2 as isize);
            tracer.read_register(s_146_2 as isize, value);
            value
        };
        // D s_146_4: cast zx s_146_3 -> bv
        let s_146_4: Bits = Bits::new(s_146_3 as u128, 5u16);
        // D s_146_5: cmp-ne s_146_1 s_146_4
        let s_146_5: bool = ((s_146_1) != (s_146_4));
        // D s_146_6: write-var gs#128704 <= s_146_5
        fn_state.gs_128704 = s_146_5;
        // N s_146_7: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_147_0: const #424u : u32
        let s_147_0: u32 = 424;
        // D s_147_1: read-reg s_147_0:u8
        let s_147_1: u8 = {
            let value = state.read_register::<u8>(s_147_0 as isize);
            tracer.read_register(s_147_0 as isize, value);
            value
        };
        // D s_147_2: call ELUsingAArch32(s_147_1)
        let s_147_2: bool = ELUsingAArch32(state, tracer, s_147_1);
        // D s_147_3: write-var gs#128703 <= s_147_2
        fn_state.gs_128703 = s_147_2;
        // N s_147_4: jump b133
        return block_133(state, tracer, fn_state);
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
        // D s_149_1: write-var gs#128707 <= s_149_0
        fn_state.gs_128707 = s_149_0;
        // N s_149_2: jump b150
        return block_150(state, tracer, fn_state);
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_150_0: read-var gs#128707:u8
        let s_150_0: bool = fn_state.gs_128707;
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
        // C s_151_0: const #3u : u8
        let s_151_0: u8 = 3;
        // C s_151_1: cast zx s_151_0 -> bv
        let s_151_1: Bits = Bits::new(s_151_0 as u128, 8u16);
        // C s_151_2: cast zx s_151_1 -> i
        let s_151_2: i128 = (s_151_1.value() as i128);
        // C s_151_3: cast reint s_151_2 -> i64
        let s_151_3: i64 = (s_151_2 as i64);
        // C s_151_4: cast zx s_151_3 -> i
        let s_151_4: i128 = (i128::try_from(s_151_3).unwrap());
        // C s_151_5: const #424u : u32
        let s_151_5: u32 = 424;
        // D s_151_6: read-reg s_151_5:u8
        let s_151_6: u8 = {
            let value = state.read_register::<u8>(s_151_5 as isize);
            tracer.read_register(s_151_5 as isize, value);
            value
        };
        // D s_151_7: call AArch64_AArch32SystemAccessTrap(s_151_6, s_151_4)
        let s_151_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_151_6,
            s_151_4,
        );
        // N s_151_8: return
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
        // D s_153_7: write-var gs#128707 <= s_153_6
        fn_state.gs_128707 = s_153_6;
        // N s_153_8: jump b150
        return block_150(state, tracer, fn_state);
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_154_0: read-var __SCR_EL3_IRQ:u8
        let s_154_0: bool = fn_state.u__SCR_EL3_IRQ;
        // D s_154_1: cast zx s_154_0 -> bv
        let s_154_1: Bits = Bits::new(s_154_0 as u128, 1u16);
        // C s_154_2: const #1u : u8
        let s_154_2: bool = true;
        // C s_154_3: cast zx s_154_2 -> bv
        let s_154_3: Bits = Bits::new(s_154_2 as u128, 1u16);
        // D s_154_4: cmp-eq s_154_1 s_154_3
        let s_154_4: bool = ((s_154_1) == (s_154_3));
        // D s_154_5: write-var gs#128702 <= s_154_4
        fn_state.gs_128702 = s_154_4;
        // N s_154_6: jump b130
        return block_130(state, tracer, fn_state);
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
        // D s_155_3: not s_155_2
        let s_155_3: bool = !s_155_2;
        // D s_155_4: write-var gs#128701 <= s_155_3
        fn_state.gs_128701 = s_155_3;
        // N s_155_5: jump b128
        return block_128(state, tracer, fn_state);
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_156_0: read-var t:i
        let s_156_0: i128 = fn_state.t;
        // D s_156_1: call R_read(s_156_0)
        let s_156_1: u32 = R_read(state, tracer, s_156_0);
        // D s_156_2: call Mk_ICV_EOIR1_Type(s_156_1)
        let s_156_2: ProductType700c18a878c5601b = Mk_ICV_EOIR1_Type(
            state,
            tracer,
            s_156_1,
        );
        // D s_156_3: call ICV_EOIR1_write(s_156_2)
        let s_156_3: () = ICV_EOIR1_write(state, tracer, s_156_2);
        // N s_156_4: return
        return;
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_157_0: read-var __HCR_IMO:u8
        let s_157_0: bool = fn_state.u__HCR_IMO;
        // D s_157_1: cast zx s_157_0 -> bv
        let s_157_1: Bits = Bits::new(s_157_0 as u128, 1u16);
        // C s_157_2: const #1u : u8
        let s_157_2: bool = true;
        // C s_157_3: cast zx s_157_2 -> bv
        let s_157_3: Bits = Bits::new(s_157_2 as u128, 1u16);
        // D s_157_4: cmp-eq s_157_1 s_157_3
        let s_157_4: bool = ((s_157_1) == (s_157_3));
        // D s_157_5: write-var gs#128700 <= s_157_4
        fn_state.gs_128700 = s_157_4;
        // N s_157_6: jump b125
        return block_125(state, tracer, fn_state);
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_158_0: const #432u : u32
        let s_158_0: u32 = 432;
        // D s_158_1: read-reg s_158_0:u8
        let s_158_1: u8 = {
            let value = state.read_register::<u8>(s_158_0 as isize);
            tracer.read_register(s_158_0 as isize, value);
            value
        };
        // D s_158_2: call ELUsingAArch32(s_158_1)
        let s_158_2: bool = ELUsingAArch32(state, tracer, s_158_1);
        // D s_158_3: write-var gs#128699 <= s_158_2
        fn_state.gs_128699 = s_158_2;
        // N s_158_4: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_159_0: read-var t:i
        let s_159_0: i128 = fn_state.t;
        // D s_159_1: call R_read(s_159_0)
        let s_159_1: u32 = R_read(state, tracer, s_159_0);
        // D s_159_2: call Mk_ICV_EOIR1_Type(s_159_1)
        let s_159_2: ProductType700c18a878c5601b = Mk_ICV_EOIR1_Type(
            state,
            tracer,
            s_159_1,
        );
        // D s_159_3: call ICV_EOIR1_write(s_159_2)
        let s_159_3: () = ICV_EOIR1_write(state, tracer, s_159_2);
        // N s_159_4: return
        return;
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_160_0: read-var __HCR_EL2_IMO:u8
        let s_160_0: bool = fn_state.u__HCR_EL2_IMO;
        // D s_160_1: cast zx s_160_0 -> bv
        let s_160_1: Bits = Bits::new(s_160_0 as u128, 1u16);
        // C s_160_2: const #1u : u8
        let s_160_2: bool = true;
        // C s_160_3: cast zx s_160_2 -> bv
        let s_160_3: Bits = Bits::new(s_160_2 as u128, 1u16);
        // D s_160_4: cmp-eq s_160_1 s_160_3
        let s_160_4: bool = ((s_160_1) == (s_160_3));
        // D s_160_5: write-var gs#128698 <= s_160_4
        fn_state.gs_128698 = s_160_4;
        // N s_160_6: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_161_0: const #432u : u32
        let s_161_0: u32 = 432;
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
        // D s_161_4: write-var gs#128697 <= s_161_3
        fn_state.gs_128697 = s_161_3;
        // N s_161_5: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_162_0: const #3u : u8
        let s_162_0: u8 = 3;
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
        // D s_163_0: read-var __ICH_HCR_TALL1:u8
        let s_163_0: bool = fn_state.u__ICH_HCR_TALL1;
        // D s_163_1: cast zx s_163_0 -> bv
        let s_163_1: Bits = Bits::new(s_163_0 as u128, 1u16);
        // C s_163_2: const #1u : u8
        let s_163_2: bool = true;
        // C s_163_3: cast zx s_163_2 -> bv
        let s_163_3: Bits = Bits::new(s_163_2 as u128, 1u16);
        // D s_163_4: cmp-eq s_163_1 s_163_3
        let s_163_4: bool = ((s_163_1) == (s_163_3));
        // D s_163_5: write-var gs#128696 <= s_163_4
        fn_state.gs_128696 = s_163_4;
        // N s_163_6: jump b115
        return block_115(state, tracer, fn_state);
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
        // D s_164_3: write-var gs#128695 <= s_164_2
        fn_state.gs_128695 = s_164_2;
        // N s_164_4: jump b113
        return block_113(state, tracer, fn_state);
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
        // D s_166_0: read-var __ICH_HCR_EL2_TALL1:u8
        let s_166_0: bool = fn_state.u__ICH_HCR_EL2_TALL1;
        // D s_166_1: cast zx s_166_0 -> bv
        let s_166_1: Bits = Bits::new(s_166_0 as u128, 1u16);
        // C s_166_2: const #1u : u8
        let s_166_2: bool = true;
        // C s_166_3: cast zx s_166_2 -> bv
        let s_166_3: Bits = Bits::new(s_166_2 as u128, 1u16);
        // D s_166_4: cmp-eq s_166_1 s_166_3
        let s_166_4: bool = ((s_166_1) == (s_166_3));
        // D s_166_5: write-var gs#128694 <= s_166_4
        fn_state.gs_128694 = s_166_4;
        // N s_166_6: jump b110
        return block_110(state, tracer, fn_state);
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
        // D s_167_4: write-var gs#128693 <= s_167_3
        fn_state.gs_128693 = s_167_3;
        // N s_167_5: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_168_0: panic
        panic!("{:?}", ());
        // N s_168_1: return
        return;
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_169_0: const #3u : u8
        let s_169_0: u8 = 3;
        // C s_169_1: cast zx s_169_0 -> bv
        let s_169_1: Bits = Bits::new(s_169_0 as u128, 8u16);
        // C s_169_2: cast zx s_169_1 -> i
        let s_169_2: i128 = (s_169_1.value() as i128);
        // C s_169_3: cast reint s_169_2 -> i64
        let s_169_3: i64 = (s_169_2 as i64);
        // C s_169_4: cast zx s_169_3 -> i
        let s_169_4: i128 = (i128::try_from(s_169_3).unwrap());
        // S s_169_5: call AArch32_TakeHypTrapException(s_169_4)
        let s_169_5: () = AArch32_TakeHypTrapException(state, tracer, s_169_4);
        // N s_169_6: return
        return;
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_170_0: read-var __HSTR_T12:u8
        let s_170_0: bool = fn_state.u__HSTR_T12;
        // D s_170_1: cast zx s_170_0 -> bv
        let s_170_1: Bits = Bits::new(s_170_0 as u128, 1u16);
        // C s_170_2: const #1u : u8
        let s_170_2: bool = true;
        // C s_170_3: cast zx s_170_2 -> bv
        let s_170_3: Bits = Bits::new(s_170_2 as u128, 1u16);
        // D s_170_4: cmp-eq s_170_1 s_170_3
        let s_170_4: bool = ((s_170_1) == (s_170_3));
        // D s_170_5: write-var gs#128692 <= s_170_4
        fn_state.gs_128692 = s_170_4;
        // N s_170_6: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_171_0: const #432u : u32
        let s_171_0: u32 = 432;
        // D s_171_1: read-reg s_171_0:u8
        let s_171_1: u8 = {
            let value = state.read_register::<u8>(s_171_0 as isize);
            tracer.read_register(s_171_0 as isize, value);
            value
        };
        // D s_171_2: call ELUsingAArch32(s_171_1)
        let s_171_2: bool = ELUsingAArch32(state, tracer, s_171_1);
        // D s_171_3: write-var gs#128691 <= s_171_2
        fn_state.gs_128691 = s_171_2;
        // N s_171_4: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_172_0: const #3u : u8
        let s_172_0: u8 = 3;
        // C s_172_1: cast zx s_172_0 -> bv
        let s_172_1: Bits = Bits::new(s_172_0 as u128, 8u16);
        // C s_172_2: cast zx s_172_1 -> i
        let s_172_2: i128 = (s_172_1.value() as i128);
        // C s_172_3: cast reint s_172_2 -> i64
        let s_172_3: i64 = (s_172_2 as i64);
        // C s_172_4: cast zx s_172_3 -> i
        let s_172_4: i128 = (i128::try_from(s_172_3).unwrap());
        // C s_172_5: const #432u : u32
        let s_172_5: u32 = 432;
        // D s_172_6: read-reg s_172_5:u8
        let s_172_6: u8 = {
            let value = state.read_register::<u8>(s_172_5 as isize);
            tracer.read_register(s_172_5 as isize, value);
            value
        };
        // D s_172_7: call AArch64_AArch32SystemAccessTrap(s_172_6, s_172_4)
        let s_172_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_172_6,
            s_172_4,
        );
        // N s_172_8: return
        return;
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_173_0: read-var __HSTR_EL2_T12:u8
        let s_173_0: bool = fn_state.u__HSTR_EL2_T12;
        // D s_173_1: cast zx s_173_0 -> bv
        let s_173_1: Bits = Bits::new(s_173_0 as u128, 1u16);
        // C s_173_2: const #1u : u8
        let s_173_2: bool = true;
        // C s_173_3: cast zx s_173_2 -> bv
        let s_173_3: Bits = Bits::new(s_173_2 as u128, 1u16);
        // D s_173_4: cmp-eq s_173_1 s_173_3
        let s_173_4: bool = ((s_173_1) == (s_173_3));
        // D s_173_5: write-var gs#128690 <= s_173_4
        fn_state.gs_128690 = s_173_4;
        // N s_173_6: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_174_0: const #432u : u32
        let s_174_0: u32 = 432;
        // D s_174_1: read-reg s_174_0:u8
        let s_174_1: u8 = {
            let value = state.read_register::<u8>(s_174_0 as isize);
            tracer.read_register(s_174_0 as isize, value);
            value
        };
        // D s_174_2: call ELUsingAArch32(s_174_1)
        let s_174_2: bool = ELUsingAArch32(state, tracer, s_174_1);
        // D s_174_3: not s_174_2
        let s_174_3: bool = !s_174_2;
        // D s_174_4: write-var gs#128689 <= s_174_3
        fn_state.gs_128689 = s_174_3;
        // N s_174_5: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_175_0: panic
        panic!("{:?}", ());
        // N s_175_1: return
        return;
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_176_0: read-var __SCR_IRQ:u8
        let s_176_0: bool = fn_state.u__SCR_IRQ;
        // D s_176_1: cast zx s_176_0 -> bv
        let s_176_1: Bits = Bits::new(s_176_0 as u128, 1u16);
        // C s_176_2: const #1u : u8
        let s_176_2: bool = true;
        // C s_176_3: cast zx s_176_2 -> bv
        let s_176_3: Bits = Bits::new(s_176_2 as u128, 1u16);
        // D s_176_4: cmp-eq s_176_1 s_176_3
        let s_176_4: bool = ((s_176_1) == (s_176_3));
        // D s_176_5: write-var gs#128688 <= s_176_4
        fn_state.gs_128688 = s_176_4;
        // N s_176_6: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_177_0: read-var __PSTATE_M:u8
        let s_177_0: u8 = fn_state.u__PSTATE_M;
        // D s_177_1: cast zx s_177_0 -> bv
        let s_177_1: Bits = Bits::new(s_177_0 as u128, 5u16);
        // C s_177_2: const #384u : u32
        let s_177_2: u32 = 384;
        // D s_177_3: read-reg s_177_2:u8
        let s_177_3: u8 = {
            let value = state.read_register::<u8>(s_177_2 as isize);
            tracer.read_register(s_177_2 as isize, value);
            value
        };
        // D s_177_4: cast zx s_177_3 -> bv
        let s_177_4: Bits = Bits::new(s_177_3 as u128, 5u16);
        // D s_177_5: cmp-ne s_177_1 s_177_4
        let s_177_5: bool = ((s_177_1) != (s_177_4));
        // D s_177_6: write-var gs#128687 <= s_177_5
        fn_state.gs_128687 = s_177_5;
        // N s_177_7: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_178_0: const #424u : u32
        let s_178_0: u32 = 424;
        // D s_178_1: read-reg s_178_0:u8
        let s_178_1: u8 = {
            let value = state.read_register::<u8>(s_178_0 as isize);
            tracer.read_register(s_178_0 as isize, value);
            value
        };
        // D s_178_2: call ELUsingAArch32(s_178_1)
        let s_178_2: bool = ELUsingAArch32(state, tracer, s_178_1);
        // D s_178_3: write-var gs#128686 <= s_178_2
        fn_state.gs_128686 = s_178_2;
        // N s_178_4: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_179_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_179_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_179_1: call __IMPDEF_boolean(s_179_0)
        let s_179_1: bool = u__IMPDEF_boolean(state, tracer, s_179_0);
        // D s_179_2: write-var gs#128685 <= s_179_1
        fn_state.gs_128685 = s_179_1;
        // N s_179_3: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_180_0: const #() : ()
        let s_180_0: () = ();
        // S s_180_1: call EDSCR_read(s_180_0)
        let s_180_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_180_0);
        // S s_180_2: call _get_EDSCR_Type_SDD(s_180_1)
        let s_180_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_180_1);
        // S s_180_3: cast zx s_180_2 -> bv
        let s_180_3: Bits = Bits::new(s_180_2 as u128, 1u16);
        // C s_180_4: const #1u : u8
        let s_180_4: bool = true;
        // C s_180_5: cast zx s_180_4 -> bv
        let s_180_5: Bits = Bits::new(s_180_4 as u128, 1u16);
        // S s_180_6: cmp-eq s_180_3 s_180_5
        let s_180_6: bool = ((s_180_3) == (s_180_5));
        // D s_180_7: write-var gs#128684 <= s_180_6
        fn_state.gs_128684 = s_180_6;
        // N s_180_8: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_181_0: const #424u : u32
        let s_181_0: u32 = 424;
        // D s_181_1: read-reg s_181_0:u8
        let s_181_1: u8 = {
            let value = state.read_register::<u8>(s_181_0 as isize);
            tracer.read_register(s_181_0 as isize, value);
            value
        };
        // C s_181_2: const #2u : u8
        let s_181_2: u8 = 2;
        // D s_181_3: cmp-lt s_181_1 s_181_2
        let s_181_3: bool = ((s_181_1) < (s_181_2));
        // D s_181_4: write-var gs#128683 <= s_181_3
        fn_state.gs_128683 = s_181_3;
        // N s_181_5: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_182_0: panic
        panic!("{:?}", ());
        // N s_182_1: return
        return;
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_183_0: read-var __SCR_EL3_IRQ:u8
        let s_183_0: bool = fn_state.u__SCR_EL3_IRQ;
        // D s_183_1: cast zx s_183_0 -> bv
        let s_183_1: Bits = Bits::new(s_183_0 as u128, 1u16);
        // C s_183_2: const #1u : u8
        let s_183_2: bool = true;
        // C s_183_3: cast zx s_183_2 -> bv
        let s_183_3: Bits = Bits::new(s_183_2 as u128, 1u16);
        // D s_183_4: cmp-eq s_183_1 s_183_3
        let s_183_4: bool = ((s_183_1) == (s_183_3));
        // D s_183_5: write-var gs#128682 <= s_183_4
        fn_state.gs_128682 = s_183_4;
        // N s_183_6: jump b81
        return block_81(state, tracer, fn_state);
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
        // D s_184_3: not s_184_2
        let s_184_3: bool = !s_184_2;
        // D s_184_4: write-var gs#128681 <= s_184_3
        fn_state.gs_128681 = s_184_3;
        // N s_184_5: jump b79
        return block_79(state, tracer, fn_state);
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
        // D s_185_2: write-var gs#128680 <= s_185_1
        fn_state.gs_128680 = s_185_1;
        // N s_185_3: jump b77
        return block_77(state, tracer, fn_state);
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
        // D s_186_7: write-var gs#128679 <= s_186_6
        fn_state.gs_128679 = s_186_6;
        // N s_186_8: jump b75
        return block_75(state, tracer, fn_state);
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
        // D s_187_4: write-var gs#128678 <= s_187_3
        fn_state.gs_128678 = s_187_3;
        // N s_187_5: jump b73
        return block_73(state, tracer, fn_state);
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
}
