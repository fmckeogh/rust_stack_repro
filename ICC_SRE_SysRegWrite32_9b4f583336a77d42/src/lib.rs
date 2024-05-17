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
use u_get_ICC_SRE_EL2_Type_Enable::*;
use Halted::*;
use u_get_ICC_SRE_EL3_Type_Enable::*;
use Mk_ICC_SRE_Type::*;
use ICC_HSRE_read::*;
use u_get_ICC_HSRE_Type_Enable::*;
use u_get_HSTR_EL2_Type_T12::*;
use HSTR_read::*;
use u_get_SCR_EL3_Type_NS::*;
use ICC_SRE_NS_write::*;
use AArch64_AArch32SystemAccessTrap::*;
use u__IMPDEF_boolean::*;
use R_read::*;
use ICC_SRE_S_write::*;
use u_get_ICC_MSRE_Type_Enable::*;
use ICC_SRE_write::*;
use ELUsingAArch32::*;
use u_get_EDSCR_Type_SDD::*;
use EL2Enabled::*;
use u_get_HSTR_Type_T12::*;
use EDSCR_read::*;
use common::*;
pub fn ICC_SRE_SysRegWrite32_9b4f583336a77d42<T: Tracer>(
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
        gs_128798: bool,
        gs_128793: bool,
        u__SCR_EL3_NS: bool,
        gs_128785: bool,
        gs_128804: bool,
        u__HSTR_T12: bool,
        u__ICC_MSRE_Enable: bool,
        gs_128796: bool,
        gs_128797: bool,
        gs_128807: bool,
        gs_128803: bool,
        gs_128784: bool,
        u__ICC_HSRE_Enable: bool,
        gs_128786: bool,
        gs_128799: bool,
        u__HSTR_EL2_T12: bool,
        gs_128790: bool,
        gs_128788: bool,
        u__ICC_SRE_EL2_Enable: bool,
        gs_128802: bool,
        gs_128805: bool,
        gs_128783: bool,
        gs_128787: bool,
        gs_128782: bool,
        u__PSTATE_EL: u8,
        gs_128806: bool,
        u__ICC_SRE_EL3_Enable: bool,
        gs_128794: bool,
        gs_128795: bool,
        gs_128792: bool,
        gs_128791: bool,
        gs_128789: bool,
        gs_128800: bool,
        gs_128801: bool,
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
        // C s_0_3: const #10200u : u32
        let s_0_3: u32 = 10200;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_ICC_SRE_EL3_Type_Enable(s_0_4)
        let s_0_5: bool = u_get_ICC_SRE_EL3_Type_Enable(state, tracer, s_0_4);
        // D s_0_6: write-var __ICC_SRE_EL3_Enable <= s_0_5
        fn_state.u__ICC_SRE_EL3_Enable = s_0_5;
        // C s_0_7: const #104936u : u32
        let s_0_7: u32 = 104936;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_HSTR_EL2_Type_T12(s_0_8)
        let s_0_9: bool = u_get_HSTR_EL2_Type_T12(state, tracer, s_0_8);
        // D s_0_10: write-var __HSTR_EL2_T12 <= s_0_9
        fn_state.u__HSTR_EL2_T12 = s_0_9;
        // C s_0_11: const #() : ()
        let s_0_11: () = ();
        // S s_0_12: call HSTR_read(s_0_11)
        let s_0_12: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_11);
        // S s_0_13: call _get_HSTR_Type_T12(s_0_12)
        let s_0_13: bool = u_get_HSTR_Type_T12(state, tracer, s_0_12);
        // D s_0_14: write-var __HSTR_T12 <= s_0_13
        fn_state.u__HSTR_T12 = s_0_13;
        // C s_0_15: const #16368u : u32
        let s_0_15: u32 = 16368;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_ICC_SRE_EL2_Type_Enable(s_0_16)
        let s_0_17: bool = u_get_ICC_SRE_EL2_Type_Enable(state, tracer, s_0_16);
        // D s_0_18: write-var __ICC_SRE_EL2_Enable <= s_0_17
        fn_state.u__ICC_SRE_EL2_Enable = s_0_17;
        // C s_0_19: const #() : ()
        let s_0_19: () = ();
        // S s_0_20: call ICC_HSRE_read(s_0_19)
        let s_0_20: ProductType700c18a878c5601b = ICC_HSRE_read(state, tracer, s_0_19);
        // S s_0_21: call _get_ICC_HSRE_Type_Enable(s_0_20)
        let s_0_21: bool = u_get_ICC_HSRE_Type_Enable(state, tracer, s_0_20);
        // D s_0_22: write-var __ICC_HSRE_Enable <= s_0_21
        fn_state.u__ICC_HSRE_Enable = s_0_21;
        // C s_0_23: const #19992u : u32
        let s_0_23: u32 = 19992;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_ICC_MSRE_Type_Enable(s_0_24)
        let s_0_25: bool = u_get_ICC_MSRE_Type_Enable(state, tracer, s_0_24);
        // D s_0_26: write-var __ICC_MSRE_Enable <= s_0_25
        fn_state.u__ICC_MSRE_Enable = s_0_25;
        // C s_0_27: const #90704u : u32
        let s_0_27: u32 = 90704;
        // D s_0_28: read-reg s_0_27:struct
        let s_0_28: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_27 as isize);
            tracer.read_register(s_0_27 as isize, value);
            value
        };
        // D s_0_29: call _get_SCR_EL3_Type_NS(s_0_28)
        let s_0_29: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_0_28);
        // D s_0_30: write-var __SCR_EL3_NS <= s_0_29
        fn_state.u__SCR_EL3_NS = s_0_29;
        // D s_0_31: read-var __PSTATE_EL:u8
        let s_0_31: u8 = fn_state.u__PSTATE_EL;
        // D s_0_32: cast zx s_0_31 -> bv
        let s_0_32: Bits = Bits::new(s_0_31 as u128, 2u16);
        // C s_0_33: const #448u : u32
        let s_0_33: u32 = 448;
        // D s_0_34: read-reg s_0_33:u8
        let s_0_34: u8 = {
            let value = state.read_register::<u8>(s_0_33 as isize);
            tracer.read_register(s_0_33 as isize, value);
            value
        };
        // D s_0_35: cast zx s_0_34 -> bv
        let s_0_35: Bits = Bits::new(s_0_34 as u128, 2u16);
        // D s_0_36: cmp-eq s_0_32 s_0_35
        let s_0_36: bool = ((s_0_32) == (s_0_35));
        // N s_0_37: branch s_0_36 b120 b1
        if s_0_36 {
            return block_120(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b45 b2
        if s_1_5 {
            return block_45(state, tracer, fn_state);
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
        // D s_5_0: read-var __SCR_EL3_NS:u8
        let s_5_0: bool = fn_state.u__SCR_EL3_NS;
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
        // D s_6_2: call Mk_ICC_SRE_Type(s_6_1)
        let s_6_2: ProductType700c18a878c5601b = Mk_ICC_SRE_Type(state, tracer, s_6_1);
        // D s_6_3: call ICC_SRE_NS_write(s_6_2)
        let s_6_3: () = ICC_SRE_NS_write(state, tracer, s_6_2);
        // N s_6_4: return
        return;
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
        // D s_7_2: call Mk_ICC_SRE_Type(s_7_1)
        let s_7_2: ProductType700c18a878c5601b = Mk_ICC_SRE_Type(state, tracer, s_7_1);
        // D s_7_3: call ICC_SRE_S_write(s_7_2)
        let s_7_3: () = ICC_SRE_S_write(state, tracer, s_7_2);
        // N s_7_4: return
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
        // N s_8_2: branch s_8_1 b44 b9
        if s_8_1 {
            return block_44(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#128782 <= s_9_0
        fn_state.gs_128782 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#128782:u8
        let s_10_0: bool = fn_state.gs_128782;
        // N s_10_1: branch s_10_0 b43 b11
        if s_10_0 {
            return block_43(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#128783 <= s_11_0
        fn_state.gs_128783 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#128783:u8
        let s_12_0: bool = fn_state.gs_128783;
        // N s_12_1: branch s_12_0 b42 b13
        if s_12_0 {
            return block_42(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#128784 <= s_13_0
        fn_state.gs_128784 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#128784:u8
        let s_14_0: bool = fn_state.gs_128784;
        // N s_14_1: branch s_14_0 b41 b15
        if s_14_0 {
            return block_41(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#128785 <= s_15_0
        fn_state.gs_128785 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#128785:u8
        let s_16_0: bool = fn_state.gs_128785;
        // N s_16_1: branch s_16_0 b40 b17
        if s_16_0 {
            return block_40(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#128786 <= s_17_0
        fn_state.gs_128786 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#128786:u8
        let s_18_0: bool = fn_state.gs_128786;
        // N s_18_1: branch s_18_0 b39 b19
        if s_18_0 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #424u : u32
        let s_19_0: u32 = 424;
        // D s_19_1: read-reg s_19_0:u8
        let s_19_1: u8 = {
            let value = state.read_register::<u8>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // C s_19_2: const #2u : u8
        let s_19_2: u8 = 2;
        // D s_19_3: cmp-lt s_19_1 s_19_2
        let s_19_3: bool = ((s_19_1) < (s_19_2));
        // N s_19_4: branch s_19_3 b38 b20
        if s_19_3 {
            return block_38(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#128787 <= s_20_0
        fn_state.gs_128787 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#128787:u8
        let s_21_0: bool = fn_state.gs_128787;
        // N s_21_1: branch s_21_0 b37 b22
        if s_21_0 {
            return block_37(state, tracer, fn_state);
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
        // D s_22_1: write-var gs#128788 <= s_22_0
        fn_state.gs_128788 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#128788:u8
        let s_23_0: bool = fn_state.gs_128788;
        // N s_23_1: branch s_23_0 b31 b24
        if s_23_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var __ICC_MSRE_Enable:u8
        let s_24_0: bool = fn_state.u__ICC_MSRE_Enable;
        // D s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 1u16);
        // C s_24_2: const #0u : u8
        let s_24_2: bool = false;
        // C s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 1u16);
        // D s_24_4: cmp-eq s_24_1 s_24_3
        let s_24_4: bool = ((s_24_1) == (s_24_3));
        // N s_24_5: branch s_24_4 b30 b25
        if s_24_4 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #424u : u32
        let s_25_0: u32 = 424;
        // D s_25_1: read-reg s_25_0:u8
        let s_25_1: u8 = {
            let value = state.read_register::<u8>(s_25_0 as isize);
            tracer.read_register(s_25_0 as isize, value);
            value
        };
        // C s_25_2: const #2u : u8
        let s_25_2: u8 = 2;
        // D s_25_3: cmp-lt s_25_1 s_25_2
        let s_25_3: bool = ((s_25_1) < (s_25_2));
        // N s_25_4: branch s_25_3 b27 b26
        if s_25_3 {
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
        // D s_26_0: read-var t:i
        let s_26_0: i128 = fn_state.t;
        // D s_26_1: call R_read(s_26_0)
        let s_26_1: u32 = R_read(state, tracer, s_26_0);
        // D s_26_2: call Mk_ICC_SRE_Type(s_26_1)
        let s_26_2: ProductType700c18a878c5601b = Mk_ICC_SRE_Type(state, tracer, s_26_1);
        // D s_26_3: call ICC_SRE_write(s_26_2)
        let s_26_3: () = ICC_SRE_write(state, tracer, s_26_2);
        // N s_26_4: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var __SCR_EL3_NS:u8
        let s_27_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 1u16);
        // C s_27_2: const #0u : u8
        let s_27_2: bool = false;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // N s_27_5: branch s_27_4 b29 b28
        if s_27_4 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var t:i
        let s_28_0: i128 = fn_state.t;
        // D s_28_1: call R_read(s_28_0)
        let s_28_1: u32 = R_read(state, tracer, s_28_0);
        // D s_28_2: call Mk_ICC_SRE_Type(s_28_1)
        let s_28_2: ProductType700c18a878c5601b = Mk_ICC_SRE_Type(state, tracer, s_28_1);
        // D s_28_3: call ICC_SRE_NS_write(s_28_2)
        let s_28_3: () = ICC_SRE_NS_write(state, tracer, s_28_2);
        // N s_28_4: return
        return;
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var t:i
        let s_29_0: i128 = fn_state.t;
        // D s_29_1: call R_read(s_29_0)
        let s_29_1: u32 = R_read(state, tracer, s_29_0);
        // D s_29_2: call Mk_ICC_SRE_Type(s_29_1)
        let s_29_2: ProductType700c18a878c5601b = Mk_ICC_SRE_Type(state, tracer, s_29_1);
        // D s_29_3: call ICC_SRE_S_write(s_29_2)
        let s_29_3: () = ICC_SRE_S_write(state, tracer, s_29_2);
        // N s_29_4: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_30_0: panic
        panic!("{:?}", ());
        // N s_30_1: return
        return;
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #() : ()
        let s_31_0: () = ();
        // S s_31_1: call Halted(s_31_0)
        let s_31_1: bool = Halted(state, tracer, s_31_0);
        // N s_31_2: branch s_31_1 b36 b32
        if s_31_1 {
            return block_36(state, tracer, fn_state);
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
        // D s_32_1: write-var gs#128789 <= s_32_0
        fn_state.gs_128789 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#128789:u8
        let s_33_0: bool = fn_state.gs_128789;
        // N s_33_1: branch s_33_0 b35 b34
        if s_33_0 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #3u : u8
        let s_34_0: u8 = 3;
        // C s_34_1: cast zx s_34_0 -> bv
        let s_34_1: Bits = Bits::new(s_34_0 as u128, 8u16);
        // C s_34_2: cast zx s_34_1 -> i
        let s_34_2: i128 = (s_34_1.value() as i128);
        // C s_34_3: cast reint s_34_2 -> i64
        let s_34_3: i64 = (s_34_2 as i64);
        // C s_34_4: cast zx s_34_3 -> i
        let s_34_4: i128 = (i128::try_from(s_34_3).unwrap());
        // C s_34_5: const #424u : u32
        let s_34_5: u32 = 424;
        // D s_34_6: read-reg s_34_5:u8
        let s_34_6: u8 = {
            let value = state.read_register::<u8>(s_34_5 as isize);
            tracer.read_register(s_34_5 as isize, value);
            value
        };
        // D s_34_7: call AArch64_AArch32SystemAccessTrap(s_34_6, s_34_4)
        let s_34_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_34_6, s_34_4);
        // N s_34_8: return
        return;
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_35_0: panic
        panic!("{:?}", ());
        // N s_35_1: return
        return;
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
        // D s_36_7: write-var gs#128789 <= s_36_6
        fn_state.gs_128789 = s_36_6;
        // N s_36_8: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var __ICC_SRE_EL3_Enable:u8
        let s_37_0: bool = fn_state.u__ICC_SRE_EL3_Enable;
        // D s_37_1: cast zx s_37_0 -> bv
        let s_37_1: Bits = Bits::new(s_37_0 as u128, 1u16);
        // C s_37_2: const #0u : u8
        let s_37_2: bool = false;
        // C s_37_3: cast zx s_37_2 -> bv
        let s_37_3: Bits = Bits::new(s_37_2 as u128, 1u16);
        // D s_37_4: cmp-eq s_37_1 s_37_3
        let s_37_4: bool = ((s_37_1) == (s_37_3));
        // D s_37_5: write-var gs#128788 <= s_37_4
        fn_state.gs_128788 = s_37_4;
        // N s_37_6: jump b23
        return block_23(state, tracer, fn_state);
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
        // D s_38_3: not s_38_2
        let s_38_3: bool = !s_38_2;
        // D s_38_4: write-var gs#128787 <= s_38_3
        fn_state.gs_128787 = s_38_3;
        // N s_38_5: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_39_0: panic
        panic!("{:?}", ());
        // N s_39_1: return
        return;
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var __ICC_SRE_EL3_Enable:u8
        let s_40_0: bool = fn_state.u__ICC_SRE_EL3_Enable;
        // D s_40_1: cast zx s_40_0 -> bv
        let s_40_1: Bits = Bits::new(s_40_0 as u128, 1u16);
        // C s_40_2: const #0u : u8
        let s_40_2: bool = false;
        // C s_40_3: cast zx s_40_2 -> bv
        let s_40_3: Bits = Bits::new(s_40_2 as u128, 1u16);
        // D s_40_4: cmp-eq s_40_1 s_40_3
        let s_40_4: bool = ((s_40_1) == (s_40_3));
        // D s_40_5: write-var gs#128786 <= s_40_4
        fn_state.gs_128786 = s_40_4;
        // N s_40_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #424u : u32
        let s_41_0: u32 = 424;
        // D s_41_1: read-reg s_41_0:u8
        let s_41_1: u8 = {
            let value = state.read_register::<u8>(s_41_0 as isize);
            tracer.read_register(s_41_0 as isize, value);
            value
        };
        // D s_41_2: call ELUsingAArch32(s_41_1)
        let s_41_2: bool = ELUsingAArch32(state, tracer, s_41_1);
        // D s_41_3: not s_41_2
        let s_41_3: bool = !s_41_2;
        // D s_41_4: write-var gs#128785 <= s_41_3
        fn_state.gs_128785 = s_41_3;
        // N s_41_5: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_42_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_42_1: call __IMPDEF_boolean(s_42_0)
        let s_42_1: bool = u__IMPDEF_boolean(state, tracer, s_42_0);
        // D s_42_2: write-var gs#128784 <= s_42_1
        fn_state.gs_128784 = s_42_1;
        // N s_42_3: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #() : ()
        let s_43_0: () = ();
        // S s_43_1: call EDSCR_read(s_43_0)
        let s_43_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_43_0);
        // S s_43_2: call _get_EDSCR_Type_SDD(s_43_1)
        let s_43_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_43_1);
        // S s_43_3: cast zx s_43_2 -> bv
        let s_43_3: Bits = Bits::new(s_43_2 as u128, 1u16);
        // C s_43_4: const #1u : u8
        let s_43_4: bool = true;
        // C s_43_5: cast zx s_43_4 -> bv
        let s_43_5: Bits = Bits::new(s_43_4 as u128, 1u16);
        // S s_43_6: cmp-eq s_43_3 s_43_5
        let s_43_6: bool = ((s_43_3) == (s_43_5));
        // D s_43_7: write-var gs#128783 <= s_43_6
        fn_state.gs_128783 = s_43_6;
        // N s_43_8: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #424u : u32
        let s_44_0: u32 = 424;
        // D s_44_1: read-reg s_44_0:u8
        let s_44_1: u8 = {
            let value = state.read_register::<u8>(s_44_0 as isize);
            tracer.read_register(s_44_0 as isize, value);
            value
        };
        // C s_44_2: const #2u : u8
        let s_44_2: u8 = 2;
        // D s_44_3: cmp-lt s_44_1 s_44_2
        let s_44_3: bool = ((s_44_1) < (s_44_2));
        // D s_44_4: write-var gs#128782 <= s_44_3
        fn_state.gs_128782 = s_44_3;
        // N s_44_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #() : ()
        let s_45_0: () = ();
        // S s_45_1: call Halted(s_45_0)
        let s_45_1: bool = Halted(state, tracer, s_45_0);
        // N s_45_2: branch s_45_1 b119 b46
        if s_45_1 {
            return block_119(state, tracer, fn_state);
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
        // D s_46_1: write-var gs#128790 <= s_46_0
        fn_state.gs_128790 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#128790:u8
        let s_47_0: bool = fn_state.gs_128790;
        // N s_47_1: branch s_47_0 b118 b48
        if s_47_0 {
            return block_118(state, tracer, fn_state);
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
        // D s_48_1: write-var gs#128791 <= s_48_0
        fn_state.gs_128791 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#128791:u8
        let s_49_0: bool = fn_state.gs_128791;
        // N s_49_1: branch s_49_0 b117 b50
        if s_49_0 {
            return block_117(state, tracer, fn_state);
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
        // D s_50_1: write-var gs#128792 <= s_50_0
        fn_state.gs_128792 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#128792:u8
        let s_51_0: bool = fn_state.gs_128792;
        // N s_51_1: branch s_51_0 b116 b52
        if s_51_0 {
            return block_116(state, tracer, fn_state);
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
        // D s_52_1: write-var gs#128793 <= s_52_0
        fn_state.gs_128793 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#128793:u8
        let s_53_0: bool = fn_state.gs_128793;
        // N s_53_1: branch s_53_0 b115 b54
        if s_53_0 {
            return block_115(state, tracer, fn_state);
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
        // D s_54_1: write-var gs#128794 <= s_54_0
        fn_state.gs_128794 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#128794:u8
        let s_55_0: bool = fn_state.gs_128794;
        // N s_55_1: branch s_55_0 b114 b56
        if s_55_0 {
            return block_114(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #() : ()
        let s_56_0: () = ();
        // S s_56_1: call EL2Enabled(s_56_0)
        let s_56_1: bool = EL2Enabled(state, tracer, s_56_0);
        // N s_56_2: branch s_56_1 b113 b57
        if s_56_1 {
            return block_113(state, tracer, fn_state);
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
        // D s_57_1: write-var gs#128795 <= s_57_0
        fn_state.gs_128795 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var gs#128795:u8
        let s_58_0: bool = fn_state.gs_128795;
        // N s_58_1: branch s_58_0 b112 b59
        if s_58_0 {
            return block_112(state, tracer, fn_state);
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
        // D s_59_1: write-var gs#128796 <= s_59_0
        fn_state.gs_128796 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#128796:u8
        let s_60_0: bool = fn_state.gs_128796;
        // N s_60_1: branch s_60_0 b111 b61
        if s_60_0 {
            return block_111(state, tracer, fn_state);
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
        // S s_61_1: call EL2Enabled(s_61_0)
        let s_61_1: bool = EL2Enabled(state, tracer, s_61_0);
        // N s_61_2: branch s_61_1 b110 b62
        if s_61_1 {
            return block_110(state, tracer, fn_state);
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
        // D s_62_1: write-var gs#128797 <= s_62_0
        fn_state.gs_128797 = s_62_0;
        // N s_62_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var gs#128797:u8
        let s_63_0: bool = fn_state.gs_128797;
        // N s_63_1: branch s_63_0 b109 b64
        if s_63_0 {
            return block_109(state, tracer, fn_state);
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
        // D s_64_1: write-var gs#128798 <= s_64_0
        fn_state.gs_128798 = s_64_0;
        // N s_64_2: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var gs#128798:u8
        let s_65_0: bool = fn_state.gs_128798;
        // N s_65_1: branch s_65_0 b108 b66
        if s_65_0 {
            return block_108(state, tracer, fn_state);
        } else {
            return block_66(state, tracer, fn_state);
        };
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
        // N s_66_2: branch s_66_1 b107 b67
        if s_66_1 {
            return block_107(state, tracer, fn_state);
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
        // D s_67_1: write-var gs#128799 <= s_67_0
        fn_state.gs_128799 = s_67_0;
        // N s_67_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var gs#128799:u8
        let s_68_0: bool = fn_state.gs_128799;
        // N s_68_1: branch s_68_0 b106 b69
        if s_68_0 {
            return block_106(state, tracer, fn_state);
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
        // D s_69_1: write-var gs#128800 <= s_69_0
        fn_state.gs_128800 = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var gs#128800:u8
        let s_70_0: bool = fn_state.gs_128800;
        // N s_70_1: branch s_70_0 b105 b71
        if s_70_0 {
            return block_105(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #() : ()
        let s_71_0: () = ();
        // S s_71_1: call EL2Enabled(s_71_0)
        let s_71_1: bool = EL2Enabled(state, tracer, s_71_0);
        // N s_71_2: branch s_71_1 b104 b72
        if s_71_1 {
            return block_104(state, tracer, fn_state);
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
        // D s_72_1: write-var gs#128801 <= s_72_0
        fn_state.gs_128801 = s_72_0;
        // N s_72_2: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var gs#128801:u8
        let s_73_0: bool = fn_state.gs_128801;
        // N s_73_1: branch s_73_0 b103 b74
        if s_73_0 {
            return block_103(state, tracer, fn_state);
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
        // D s_74_1: write-var gs#128802 <= s_74_0
        fn_state.gs_128802 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var gs#128802:u8
        let s_75_0: bool = fn_state.gs_128802;
        // N s_75_1: branch s_75_0 b102 b76
        if s_75_0 {
            return block_102(state, tracer, fn_state);
        } else {
            return block_76(state, tracer, fn_state);
        };
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
        // N s_76_4: branch s_76_3 b101 b77
        if s_76_3 {
            return block_101(state, tracer, fn_state);
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
        // D s_77_1: write-var gs#128803 <= s_77_0
        fn_state.gs_128803 = s_77_0;
        // N s_77_2: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var gs#128803:u8
        let s_78_0: bool = fn_state.gs_128803;
        // N s_78_1: branch s_78_0 b100 b79
        if s_78_0 {
            return block_100(state, tracer, fn_state);
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
        // D s_79_1: write-var gs#128804 <= s_79_0
        fn_state.gs_128804 = s_79_0;
        // N s_79_2: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var gs#128804:u8
        let s_80_0: bool = fn_state.gs_128804;
        // N s_80_1: branch s_80_0 b99 b81
        if s_80_0 {
            return block_99(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
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
        // C s_81_2: const #2u : u8
        let s_81_2: u8 = 2;
        // D s_81_3: cmp-lt s_81_1 s_81_2
        let s_81_3: bool = ((s_81_1) < (s_81_2));
        // N s_81_4: branch s_81_3 b98 b82
        if s_81_3 {
            return block_98(state, tracer, fn_state);
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
        // D s_82_1: write-var gs#128805 <= s_82_0
        fn_state.gs_128805 = s_82_0;
        // N s_82_2: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_83_0: read-var gs#128805:u8
        let s_83_0: bool = fn_state.gs_128805;
        // N s_83_1: branch s_83_0 b97 b84
        if s_83_0 {
            return block_97(state, tracer, fn_state);
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
        // D s_84_1: write-var gs#128806 <= s_84_0
        fn_state.gs_128806 = s_84_0;
        // N s_84_2: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var gs#128806:u8
        let s_85_0: bool = fn_state.gs_128806;
        // N s_85_1: branch s_85_0 b91 b86
        if s_85_0 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_86(state, tracer, fn_state);
        };
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #424u : u32
        let s_86_0: u32 = 424;
        // D s_86_1: read-reg s_86_0:u8
        let s_86_1: u8 = {
            let value = state.read_register::<u8>(s_86_0 as isize);
            tracer.read_register(s_86_0 as isize, value);
            value
        };
        // C s_86_2: const #2u : u8
        let s_86_2: u8 = 2;
        // D s_86_3: cmp-lt s_86_1 s_86_2
        let s_86_3: bool = ((s_86_1) < (s_86_2));
        // N s_86_4: branch s_86_3 b88 b87
        if s_86_3 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var t:i
        let s_87_0: i128 = fn_state.t;
        // D s_87_1: call R_read(s_87_0)
        let s_87_1: u32 = R_read(state, tracer, s_87_0);
        // D s_87_2: call Mk_ICC_SRE_Type(s_87_1)
        let s_87_2: ProductType700c18a878c5601b = Mk_ICC_SRE_Type(state, tracer, s_87_1);
        // D s_87_3: call ICC_SRE_write(s_87_2)
        let s_87_3: () = ICC_SRE_write(state, tracer, s_87_2);
        // N s_87_4: return
        return;
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var __SCR_EL3_NS:u8
        let s_88_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_88_1: cast zx s_88_0 -> bv
        let s_88_1: Bits = Bits::new(s_88_0 as u128, 1u16);
        // C s_88_2: const #0u : u8
        let s_88_2: bool = false;
        // C s_88_3: cast zx s_88_2 -> bv
        let s_88_3: Bits = Bits::new(s_88_2 as u128, 1u16);
        // D s_88_4: cmp-eq s_88_1 s_88_3
        let s_88_4: bool = ((s_88_1) == (s_88_3));
        // N s_88_5: branch s_88_4 b90 b89
        if s_88_4 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_89(state, tracer, fn_state);
        };
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_89_0: read-var t:i
        let s_89_0: i128 = fn_state.t;
        // D s_89_1: call R_read(s_89_0)
        let s_89_1: u32 = R_read(state, tracer, s_89_0);
        // D s_89_2: call Mk_ICC_SRE_Type(s_89_1)
        let s_89_2: ProductType700c18a878c5601b = Mk_ICC_SRE_Type(state, tracer, s_89_1);
        // D s_89_3: call ICC_SRE_NS_write(s_89_2)
        let s_89_3: () = ICC_SRE_NS_write(state, tracer, s_89_2);
        // N s_89_4: return
        return;
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var t:i
        let s_90_0: i128 = fn_state.t;
        // D s_90_1: call R_read(s_90_0)
        let s_90_1: u32 = R_read(state, tracer, s_90_0);
        // D s_90_2: call Mk_ICC_SRE_Type(s_90_1)
        let s_90_2: ProductType700c18a878c5601b = Mk_ICC_SRE_Type(state, tracer, s_90_1);
        // D s_90_3: call ICC_SRE_S_write(s_90_2)
        let s_90_3: () = ICC_SRE_S_write(state, tracer, s_90_2);
        // N s_90_4: return
        return;
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #() : ()
        let s_91_0: () = ();
        // S s_91_1: call Halted(s_91_0)
        let s_91_1: bool = Halted(state, tracer, s_91_0);
        // N s_91_2: branch s_91_1 b96 b92
        if s_91_1 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_92(state, tracer, fn_state);
        };
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #0u : u8
        let s_92_0: bool = false;
        // D s_92_1: write-var gs#128807 <= s_92_0
        fn_state.gs_128807 = s_92_0;
        // N s_92_2: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_93_0: read-var gs#128807:u8
        let s_93_0: bool = fn_state.gs_128807;
        // N s_93_1: branch s_93_0 b95 b94
        if s_93_0 {
            return block_95(state, tracer, fn_state);
        } else {
            return block_94(state, tracer, fn_state);
        };
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #3u : u8
        let s_94_0: u8 = 3;
        // C s_94_1: cast zx s_94_0 -> bv
        let s_94_1: Bits = Bits::new(s_94_0 as u128, 8u16);
        // C s_94_2: cast zx s_94_1 -> i
        let s_94_2: i128 = (s_94_1.value() as i128);
        // C s_94_3: cast reint s_94_2 -> i64
        let s_94_3: i64 = (s_94_2 as i64);
        // C s_94_4: cast zx s_94_3 -> i
        let s_94_4: i128 = (i128::try_from(s_94_3).unwrap());
        // C s_94_5: const #424u : u32
        let s_94_5: u32 = 424;
        // D s_94_6: read-reg s_94_5:u8
        let s_94_6: u8 = {
            let value = state.read_register::<u8>(s_94_5 as isize);
            tracer.read_register(s_94_5 as isize, value);
            value
        };
        // D s_94_7: call AArch64_AArch32SystemAccessTrap(s_94_6, s_94_4)
        let s_94_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_94_6, s_94_4);
        // N s_94_8: return
        return;
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
        // C s_96_0: const #() : ()
        let s_96_0: () = ();
        // S s_96_1: call EDSCR_read(s_96_0)
        let s_96_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_96_0);
        // S s_96_2: call _get_EDSCR_Type_SDD(s_96_1)
        let s_96_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_96_1);
        // S s_96_3: cast zx s_96_2 -> bv
        let s_96_3: Bits = Bits::new(s_96_2 as u128, 1u16);
        // C s_96_4: const #1u : u8
        let s_96_4: bool = true;
        // C s_96_5: cast zx s_96_4 -> bv
        let s_96_5: Bits = Bits::new(s_96_4 as u128, 1u16);
        // S s_96_6: cmp-eq s_96_3 s_96_5
        let s_96_6: bool = ((s_96_3) == (s_96_5));
        // D s_96_7: write-var gs#128807 <= s_96_6
        fn_state.gs_128807 = s_96_6;
        // N s_96_8: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var __ICC_SRE_EL3_Enable:u8
        let s_97_0: bool = fn_state.u__ICC_SRE_EL3_Enable;
        // D s_97_1: cast zx s_97_0 -> bv
        let s_97_1: Bits = Bits::new(s_97_0 as u128, 1u16);
        // C s_97_2: const #0u : u8
        let s_97_2: bool = false;
        // C s_97_3: cast zx s_97_2 -> bv
        let s_97_3: Bits = Bits::new(s_97_2 as u128, 1u16);
        // D s_97_4: cmp-eq s_97_1 s_97_3
        let s_97_4: bool = ((s_97_1) == (s_97_3));
        // D s_97_5: write-var gs#128806 <= s_97_4
        fn_state.gs_128806 = s_97_4;
        // N s_97_6: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #424u : u32
        let s_98_0: u32 = 424;
        // D s_98_1: read-reg s_98_0:u8
        let s_98_1: u8 = {
            let value = state.read_register::<u8>(s_98_0 as isize);
            tracer.read_register(s_98_0 as isize, value);
            value
        };
        // D s_98_2: call ELUsingAArch32(s_98_1)
        let s_98_2: bool = ELUsingAArch32(state, tracer, s_98_1);
        // D s_98_3: not s_98_2
        let s_98_3: bool = !s_98_2;
        // D s_98_4: write-var gs#128805 <= s_98_3
        fn_state.gs_128805 = s_98_3;
        // N s_98_5: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_99_0: panic
        panic!("{:?}", ());
        // N s_99_1: return
        return;
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_100_0: read-var __ICC_MSRE_Enable:u8
        let s_100_0: bool = fn_state.u__ICC_MSRE_Enable;
        // D s_100_1: cast zx s_100_0 -> bv
        let s_100_1: Bits = Bits::new(s_100_0 as u128, 1u16);
        // C s_100_2: const #0u : u8
        let s_100_2: bool = false;
        // C s_100_3: cast zx s_100_2 -> bv
        let s_100_3: Bits = Bits::new(s_100_2 as u128, 1u16);
        // D s_100_4: cmp-eq s_100_1 s_100_3
        let s_100_4: bool = ((s_100_1) == (s_100_3));
        // D s_100_5: write-var gs#128804 <= s_100_4
        fn_state.gs_128804 = s_100_4;
        // N s_100_6: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #424u : u32
        let s_101_0: u32 = 424;
        // D s_101_1: read-reg s_101_0:u8
        let s_101_1: u8 = {
            let value = state.read_register::<u8>(s_101_0 as isize);
            tracer.read_register(s_101_0 as isize, value);
            value
        };
        // D s_101_2: call ELUsingAArch32(s_101_1)
        let s_101_2: bool = ELUsingAArch32(state, tracer, s_101_1);
        // D s_101_3: write-var gs#128803 <= s_101_2
        fn_state.gs_128803 = s_101_2;
        // N s_101_4: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #3u : u8
        let s_102_0: u8 = 3;
        // C s_102_1: cast zx s_102_0 -> bv
        let s_102_1: Bits = Bits::new(s_102_0 as u128, 8u16);
        // C s_102_2: cast zx s_102_1 -> i
        let s_102_2: i128 = (s_102_1.value() as i128);
        // C s_102_3: cast reint s_102_2 -> i64
        let s_102_3: i64 = (s_102_2 as i64);
        // C s_102_4: cast zx s_102_3 -> i
        let s_102_4: i128 = (i128::try_from(s_102_3).unwrap());
        // S s_102_5: call AArch32_TakeHypTrapException(s_102_4)
        let s_102_5: () = AArch32_TakeHypTrapException(state, tracer, s_102_4);
        // N s_102_6: return
        return;
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_103_0: read-var __ICC_HSRE_Enable:u8
        let s_103_0: bool = fn_state.u__ICC_HSRE_Enable;
        // D s_103_1: cast zx s_103_0 -> bv
        let s_103_1: Bits = Bits::new(s_103_0 as u128, 1u16);
        // C s_103_2: const #0u : u8
        let s_103_2: bool = false;
        // C s_103_3: cast zx s_103_2 -> bv
        let s_103_3: Bits = Bits::new(s_103_2 as u128, 1u16);
        // D s_103_4: cmp-eq s_103_1 s_103_3
        let s_103_4: bool = ((s_103_1) == (s_103_3));
        // D s_103_5: write-var gs#128802 <= s_103_4
        fn_state.gs_128802 = s_103_4;
        // N s_103_6: jump b75
        return block_75(state, tracer, fn_state);
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
        // D s_104_3: write-var gs#128801 <= s_104_2
        fn_state.gs_128801 = s_104_2;
        // N s_104_4: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #3u : u8
        let s_105_0: u8 = 3;
        // C s_105_1: cast zx s_105_0 -> bv
        let s_105_1: Bits = Bits::new(s_105_0 as u128, 8u16);
        // C s_105_2: cast zx s_105_1 -> i
        let s_105_2: i128 = (s_105_1.value() as i128);
        // C s_105_3: cast reint s_105_2 -> i64
        let s_105_3: i64 = (s_105_2 as i64);
        // C s_105_4: cast zx s_105_3 -> i
        let s_105_4: i128 = (i128::try_from(s_105_3).unwrap());
        // C s_105_5: const #432u : u32
        let s_105_5: u32 = 432;
        // D s_105_6: read-reg s_105_5:u8
        let s_105_6: u8 = {
            let value = state.read_register::<u8>(s_105_5 as isize);
            tracer.read_register(s_105_5 as isize, value);
            value
        };
        // D s_105_7: call AArch64_AArch32SystemAccessTrap(s_105_6, s_105_4)
        let s_105_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_105_6,
            s_105_4,
        );
        // N s_105_8: return
        return;
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var __ICC_SRE_EL2_Enable:u8
        let s_106_0: bool = fn_state.u__ICC_SRE_EL2_Enable;
        // D s_106_1: cast zx s_106_0 -> bv
        let s_106_1: Bits = Bits::new(s_106_0 as u128, 1u16);
        // C s_106_2: const #0u : u8
        let s_106_2: bool = false;
        // C s_106_3: cast zx s_106_2 -> bv
        let s_106_3: Bits = Bits::new(s_106_2 as u128, 1u16);
        // D s_106_4: cmp-eq s_106_1 s_106_3
        let s_106_4: bool = ((s_106_1) == (s_106_3));
        // D s_106_5: write-var gs#128800 <= s_106_4
        fn_state.gs_128800 = s_106_4;
        // N s_106_6: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #432u : u32
        let s_107_0: u32 = 432;
        // D s_107_1: read-reg s_107_0:u8
        let s_107_1: u8 = {
            let value = state.read_register::<u8>(s_107_0 as isize);
            tracer.read_register(s_107_0 as isize, value);
            value
        };
        // D s_107_2: call ELUsingAArch32(s_107_1)
        let s_107_2: bool = ELUsingAArch32(state, tracer, s_107_1);
        // D s_107_3: not s_107_2
        let s_107_3: bool = !s_107_2;
        // D s_107_4: write-var gs#128799 <= s_107_3
        fn_state.gs_128799 = s_107_3;
        // N s_107_5: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #3u : u8
        let s_108_0: u8 = 3;
        // C s_108_1: cast zx s_108_0 -> bv
        let s_108_1: Bits = Bits::new(s_108_0 as u128, 8u16);
        // C s_108_2: cast zx s_108_1 -> i
        let s_108_2: i128 = (s_108_1.value() as i128);
        // C s_108_3: cast reint s_108_2 -> i64
        let s_108_3: i64 = (s_108_2 as i64);
        // C s_108_4: cast zx s_108_3 -> i
        let s_108_4: i128 = (i128::try_from(s_108_3).unwrap());
        // S s_108_5: call AArch32_TakeHypTrapException(s_108_4)
        let s_108_5: () = AArch32_TakeHypTrapException(state, tracer, s_108_4);
        // N s_108_6: return
        return;
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_109_0: read-var __HSTR_T12:u8
        let s_109_0: bool = fn_state.u__HSTR_T12;
        // D s_109_1: cast zx s_109_0 -> bv
        let s_109_1: Bits = Bits::new(s_109_0 as u128, 1u16);
        // C s_109_2: const #1u : u8
        let s_109_2: bool = true;
        // C s_109_3: cast zx s_109_2 -> bv
        let s_109_3: Bits = Bits::new(s_109_2 as u128, 1u16);
        // D s_109_4: cmp-eq s_109_1 s_109_3
        let s_109_4: bool = ((s_109_1) == (s_109_3));
        // D s_109_5: write-var gs#128798 <= s_109_4
        fn_state.gs_128798 = s_109_4;
        // N s_109_6: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_110_0: const #432u : u32
        let s_110_0: u32 = 432;
        // D s_110_1: read-reg s_110_0:u8
        let s_110_1: u8 = {
            let value = state.read_register::<u8>(s_110_0 as isize);
            tracer.read_register(s_110_0 as isize, value);
            value
        };
        // D s_110_2: call ELUsingAArch32(s_110_1)
        let s_110_2: bool = ELUsingAArch32(state, tracer, s_110_1);
        // D s_110_3: write-var gs#128797 <= s_110_2
        fn_state.gs_128797 = s_110_2;
        // N s_110_4: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_111_0: const #3u : u8
        let s_111_0: u8 = 3;
        // C s_111_1: cast zx s_111_0 -> bv
        let s_111_1: Bits = Bits::new(s_111_0 as u128, 8u16);
        // C s_111_2: cast zx s_111_1 -> i
        let s_111_2: i128 = (s_111_1.value() as i128);
        // C s_111_3: cast reint s_111_2 -> i64
        let s_111_3: i64 = (s_111_2 as i64);
        // C s_111_4: cast zx s_111_3 -> i
        let s_111_4: i128 = (i128::try_from(s_111_3).unwrap());
        // C s_111_5: const #432u : u32
        let s_111_5: u32 = 432;
        // D s_111_6: read-reg s_111_5:u8
        let s_111_6: u8 = {
            let value = state.read_register::<u8>(s_111_5 as isize);
            tracer.read_register(s_111_5 as isize, value);
            value
        };
        // D s_111_7: call AArch64_AArch32SystemAccessTrap(s_111_6, s_111_4)
        let s_111_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_111_6,
            s_111_4,
        );
        // N s_111_8: return
        return;
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_112_0: read-var __HSTR_EL2_T12:u8
        let s_112_0: bool = fn_state.u__HSTR_EL2_T12;
        // D s_112_1: cast zx s_112_0 -> bv
        let s_112_1: Bits = Bits::new(s_112_0 as u128, 1u16);
        // C s_112_2: const #1u : u8
        let s_112_2: bool = true;
        // C s_112_3: cast zx s_112_2 -> bv
        let s_112_3: Bits = Bits::new(s_112_2 as u128, 1u16);
        // D s_112_4: cmp-eq s_112_1 s_112_3
        let s_112_4: bool = ((s_112_1) == (s_112_3));
        // D s_112_5: write-var gs#128796 <= s_112_4
        fn_state.gs_128796 = s_112_4;
        // N s_112_6: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_113_0: const #432u : u32
        let s_113_0: u32 = 432;
        // D s_113_1: read-reg s_113_0:u8
        let s_113_1: u8 = {
            let value = state.read_register::<u8>(s_113_0 as isize);
            tracer.read_register(s_113_0 as isize, value);
            value
        };
        // D s_113_2: call ELUsingAArch32(s_113_1)
        let s_113_2: bool = ELUsingAArch32(state, tracer, s_113_1);
        // D s_113_3: not s_113_2
        let s_113_3: bool = !s_113_2;
        // D s_113_4: write-var gs#128795 <= s_113_3
        fn_state.gs_128795 = s_113_3;
        // N s_113_5: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_114_0: panic
        panic!("{:?}", ());
        // N s_114_1: return
        return;
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_115_0: read-var __ICC_SRE_EL3_Enable:u8
        let s_115_0: bool = fn_state.u__ICC_SRE_EL3_Enable;
        // D s_115_1: cast zx s_115_0 -> bv
        let s_115_1: Bits = Bits::new(s_115_0 as u128, 1u16);
        // C s_115_2: const #0u : u8
        let s_115_2: bool = false;
        // C s_115_3: cast zx s_115_2 -> bv
        let s_115_3: Bits = Bits::new(s_115_2 as u128, 1u16);
        // D s_115_4: cmp-eq s_115_1 s_115_3
        let s_115_4: bool = ((s_115_1) == (s_115_3));
        // D s_115_5: write-var gs#128794 <= s_115_4
        fn_state.gs_128794 = s_115_4;
        // N s_115_6: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #424u : u32
        let s_116_0: u32 = 424;
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
        // D s_116_4: write-var gs#128793 <= s_116_3
        fn_state.gs_128793 = s_116_3;
        // N s_116_5: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_117_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_117_1: call __IMPDEF_boolean(s_117_0)
        let s_117_1: bool = u__IMPDEF_boolean(state, tracer, s_117_0);
        // D s_117_2: write-var gs#128792 <= s_117_1
        fn_state.gs_128792 = s_117_1;
        // N s_117_3: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #() : ()
        let s_118_0: () = ();
        // S s_118_1: call EDSCR_read(s_118_0)
        let s_118_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_118_0);
        // S s_118_2: call _get_EDSCR_Type_SDD(s_118_1)
        let s_118_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_118_1);
        // S s_118_3: cast zx s_118_2 -> bv
        let s_118_3: Bits = Bits::new(s_118_2 as u128, 1u16);
        // C s_118_4: const #1u : u8
        let s_118_4: bool = true;
        // C s_118_5: cast zx s_118_4 -> bv
        let s_118_5: Bits = Bits::new(s_118_4 as u128, 1u16);
        // S s_118_6: cmp-eq s_118_3 s_118_5
        let s_118_6: bool = ((s_118_3) == (s_118_5));
        // D s_118_7: write-var gs#128791 <= s_118_6
        fn_state.gs_128791 = s_118_6;
        // N s_118_8: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #424u : u32
        let s_119_0: u32 = 424;
        // D s_119_1: read-reg s_119_0:u8
        let s_119_1: u8 = {
            let value = state.read_register::<u8>(s_119_0 as isize);
            tracer.read_register(s_119_0 as isize, value);
            value
        };
        // C s_119_2: const #2u : u8
        let s_119_2: u8 = 2;
        // D s_119_3: cmp-lt s_119_1 s_119_2
        let s_119_3: bool = ((s_119_1) < (s_119_2));
        // D s_119_4: write-var gs#128790 <= s_119_3
        fn_state.gs_128790 = s_119_3;
        // N s_119_5: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_120_0: panic
        panic!("{:?}", ());
        // N s_120_1: return
        return;
    }
}
