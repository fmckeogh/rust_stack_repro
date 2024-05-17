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
use u_get_HSTR_EL2_Type_T1::*;
use Halted::*;
use u__IMPDEF_boolean::*;
use u_get_SCR_Type_NS::*;
use HSTR_read::*;
use AArch64_AArch32SystemAccessTrap::*;
use u_get_SDCR_Type_TTRF::*;
use R_read::*;
use u_get_MDCR_EL3_Type_TTRF::*;
use Mk_HTRFCR_Type::*;
use u_get_HSTR_Type_T1::*;
use ELUsingAArch32::*;
use u_get_EDSCR_Type_SDD::*;
use EL2Enabled::*;
use HTRFCR_write::*;
use AArch32_TakeMonitorTrapException::*;
use EDSCR_read::*;
use common::*;
pub fn HTRFCR_SysRegWrite32_6d3975a6f673d36e<T: Tracer>(
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
        gs_128510: bool,
        gs_128509: bool,
        gs_128511: bool,
        gs_128502: bool,
        gs_128503: bool,
        gs_128506: bool,
        u__HSTR_EL2_T1: bool,
        gs_128498: bool,
        gs_128501: bool,
        u__HSTR_T1: bool,
        u__SCR_NS: bool,
        gs_128494: bool,
        gs_128495: bool,
        gs_128499: bool,
        gs_128496: bool,
        gs_128504: bool,
        gs_128497: bool,
        u__SDCR_TTRF: bool,
        gs_128493: bool,
        gs_128500: bool,
        gs_128492: bool,
        u__MDCR_EL3_TTRF: bool,
        gs_128507: bool,
        u__PSTATE_EL: u8,
        gs_128505: bool,
        gs_128508: bool,
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
        // C s_0_3: const #104936u : u32
        let s_0_3: u32 = 104936;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_HSTR_EL2_Type_T1(s_0_4)
        let s_0_5: bool = u_get_HSTR_EL2_Type_T1(state, tracer, s_0_4);
        // D s_0_6: write-var __HSTR_EL2_T1 <= s_0_5
        fn_state.u__HSTR_EL2_T1 = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call HSTR_read(s_0_7)
        let s_0_8: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_7);
        // S s_0_9: call _get_HSTR_Type_T1(s_0_8)
        let s_0_9: bool = u_get_HSTR_Type_T1(state, tracer, s_0_8);
        // D s_0_10: write-var __HSTR_T1 <= s_0_9
        fn_state.u__HSTR_T1 = s_0_9;
        // C s_0_11: const #22712u : u32
        let s_0_11: u32 = 22712;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_MDCR_EL3_Type_TTRF(s_0_12)
        let s_0_13: bool = u_get_MDCR_EL3_Type_TTRF(state, tracer, s_0_12);
        // D s_0_14: write-var __MDCR_EL3_TTRF <= s_0_13
        fn_state.u__MDCR_EL3_TTRF = s_0_13;
        // C s_0_15: const #15048u : u32
        let s_0_15: u32 = 15048;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_SDCR_Type_TTRF(s_0_16)
        let s_0_17: bool = u_get_SDCR_Type_TTRF(state, tracer, s_0_16);
        // D s_0_18: write-var __SDCR_TTRF <= s_0_17
        fn_state.u__SDCR_TTRF = s_0_17;
        // C s_0_19: const #20920u : u32
        let s_0_19: u32 = 20920;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_SCR_Type_NS(s_0_20)
        let s_0_21: bool = u_get_SCR_Type_NS(state, tracer, s_0_20);
        // D s_0_22: write-var __SCR_NS <= s_0_21
        fn_state.u__SCR_NS = s_0_21;
        // D s_0_23: read-var __PSTATE_EL:u8
        let s_0_23: u8 = fn_state.u__PSTATE_EL;
        // D s_0_24: cast zx s_0_23 -> bv
        let s_0_24: Bits = Bits::new(s_0_23 as u128, 2u16);
        // C s_0_25: const #448u : u32
        let s_0_25: u32 = 448;
        // D s_0_26: read-reg s_0_25:u8
        let s_0_26: u8 = {
            let value = state.read_register::<u8>(s_0_25 as isize);
            tracer.read_register(s_0_25 as isize, value);
            value
        };
        // D s_0_27: cast zx s_0_26 -> bv
        let s_0_27: Bits = Bits::new(s_0_26 as u128, 2u16);
        // D s_0_28: cmp-eq s_0_24 s_0_27
        let s_0_28: bool = ((s_0_24) == (s_0_27));
        // N s_0_29: branch s_0_28 b86 b1
        if s_0_28 {
            return block_86(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b69 b2
        if s_1_5 {
            return block_69(state, tracer, fn_state);
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
        // D s_6_0: read-var t:i
        let s_6_0: i128 = fn_state.t;
        // D s_6_1: call R_read(s_6_0)
        let s_6_1: u32 = R_read(state, tracer, s_6_0);
        // D s_6_2: call Mk_HTRFCR_Type(s_6_1)
        let s_6_2: ProductType700c18a878c5601b = Mk_HTRFCR_Type(state, tracer, s_6_1);
        // D s_6_3: call HTRFCR_write(s_6_2)
        let s_6_3: () = HTRFCR_write(state, tracer, s_6_2);
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
        // N s_8_2: branch s_8_1 b68 b9
        if s_8_1 {
            return block_68(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#128492 <= s_9_0
        fn_state.gs_128492 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#128492:u8
        let s_10_0: bool = fn_state.gs_128492;
        // N s_10_1: branch s_10_0 b67 b11
        if s_10_0 {
            return block_67(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#128493 <= s_11_0
        fn_state.gs_128493 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#128493:u8
        let s_12_0: bool = fn_state.gs_128493;
        // N s_12_1: branch s_12_0 b66 b13
        if s_12_0 {
            return block_66(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#128494 <= s_13_0
        fn_state.gs_128494 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#128494:u8
        let s_14_0: bool = fn_state.gs_128494;
        // N s_14_1: branch s_14_0 b65 b15
        if s_14_0 {
            return block_65(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#128495 <= s_15_0
        fn_state.gs_128495 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#128495:u8
        let s_16_0: bool = fn_state.gs_128495;
        // N s_16_1: branch s_16_0 b64 b17
        if s_16_0 {
            return block_64(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#128496 <= s_17_0
        fn_state.gs_128496 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#128496:u8
        let s_18_0: bool = fn_state.gs_128496;
        // N s_18_1: branch s_18_0 b63 b19
        if s_18_0 {
            return block_63(state, tracer, fn_state);
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
        // N s_19_2: branch s_19_1 b62 b20
        if s_19_1 {
            return block_62(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#128497 <= s_20_0
        fn_state.gs_128497 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#128497:u8
        let s_21_0: bool = fn_state.gs_128497;
        // N s_21_1: branch s_21_0 b61 b22
        if s_21_0 {
            return block_61(state, tracer, fn_state);
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
        // D s_22_1: write-var gs#128498 <= s_22_0
        fn_state.gs_128498 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#128498:u8
        let s_23_0: bool = fn_state.gs_128498;
        // N s_23_1: branch s_23_0 b60 b24
        if s_23_0 {
            return block_60(state, tracer, fn_state);
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
        // D s_24_1: write-var gs#128499 <= s_24_0
        fn_state.gs_128499 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#128499:u8
        let s_25_0: bool = fn_state.gs_128499;
        // N s_25_1: branch s_25_0 b59 b26
        if s_25_0 {
            return block_59(state, tracer, fn_state);
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
        // D s_26_1: write-var gs#128500 <= s_26_0
        fn_state.gs_128500 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#128500:u8
        let s_27_0: bool = fn_state.gs_128500;
        // N s_27_1: branch s_27_0 b58 b28
        if s_27_0 {
            return block_58(state, tracer, fn_state);
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
        // D s_28_1: write-var gs#128501 <= s_28_0
        fn_state.gs_128501 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#128501:u8
        let s_29_0: bool = fn_state.gs_128501;
        // N s_29_1: branch s_29_0 b57 b30
        if s_29_0 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
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
        // C s_30_2: const #2u : u8
        let s_30_2: u8 = 2;
        // D s_30_3: cmp-lt s_30_1 s_30_2
        let s_30_3: bool = ((s_30_1) < (s_30_2));
        // N s_30_4: branch s_30_3 b56 b31
        if s_30_3 {
            return block_56(state, tracer, fn_state);
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
        // D s_31_1: write-var gs#128502 <= s_31_0
        fn_state.gs_128502 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#128502:u8
        let s_32_0: bool = fn_state.gs_128502;
        // N s_32_1: branch s_32_0 b55 b33
        if s_32_0 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #0u : u8
        let s_33_0: bool = false;
        // D s_33_1: write-var gs#128503 <= s_33_0
        fn_state.gs_128503 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#128503:u8
        let s_34_0: bool = fn_state.gs_128503;
        // N s_34_1: branch s_34_0 b49 b35
        if s_34_0 {
            return block_49(state, tracer, fn_state);
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
        // N s_35_4: branch s_35_3 b48 b36
        if s_35_3 {
            return block_48(state, tracer, fn_state);
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
        // D s_36_1: write-var gs#128504 <= s_36_0
        fn_state.gs_128504 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#128504:u8
        let s_37_0: bool = fn_state.gs_128504;
        // N s_37_1: branch s_37_0 b47 b38
        if s_37_0 {
            return block_47(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#128505 <= s_38_0
        fn_state.gs_128505 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#128505:u8
        let s_39_0: bool = fn_state.gs_128505;
        // N s_39_1: branch s_39_0 b41 b40
        if s_39_0 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var t:i
        let s_40_0: i128 = fn_state.t;
        // D s_40_1: call R_read(s_40_0)
        let s_40_1: u32 = R_read(state, tracer, s_40_0);
        // D s_40_2: call Mk_HTRFCR_Type(s_40_1)
        let s_40_2: ProductType700c18a878c5601b = Mk_HTRFCR_Type(state, tracer, s_40_1);
        // D s_40_3: call HTRFCR_write(s_40_2)
        let s_40_3: () = HTRFCR_write(state, tracer, s_40_2);
        // N s_40_4: return
        return;
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #() : ()
        let s_41_0: () = ();
        // S s_41_1: call Halted(s_41_0)
        let s_41_1: bool = Halted(state, tracer, s_41_0);
        // N s_41_2: branch s_41_1 b46 b42
        if s_41_1 {
            return block_46(state, tracer, fn_state);
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
        // D s_42_1: write-var gs#128506 <= s_42_0
        fn_state.gs_128506 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#128506:u8
        let s_43_0: bool = fn_state.gs_128506;
        // N s_43_1: branch s_43_0 b45 b44
        if s_43_0 {
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
        // S s_44_1: call AArch32_TakeMonitorTrapException(s_44_0)
        let s_44_1: () = AArch32_TakeMonitorTrapException(state, tracer, s_44_0);
        // N s_44_2: return
        return;
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_45_0: panic
        panic!("{:?}", ());
        // N s_45_1: return
        return;
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #() : ()
        let s_46_0: () = ();
        // S s_46_1: call EDSCR_read(s_46_0)
        let s_46_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_46_0);
        // S s_46_2: call _get_EDSCR_Type_SDD(s_46_1)
        let s_46_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_46_1);
        // S s_46_3: cast zx s_46_2 -> bv
        let s_46_3: Bits = Bits::new(s_46_2 as u128, 1u16);
        // C s_46_4: const #1u : u8
        let s_46_4: bool = true;
        // C s_46_5: cast zx s_46_4 -> bv
        let s_46_5: Bits = Bits::new(s_46_4 as u128, 1u16);
        // S s_46_6: cmp-eq s_46_3 s_46_5
        let s_46_6: bool = ((s_46_3) == (s_46_5));
        // D s_46_7: write-var gs#128506 <= s_46_6
        fn_state.gs_128506 = s_46_6;
        // N s_46_8: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var __SDCR_TTRF:u8
        let s_47_0: bool = fn_state.u__SDCR_TTRF;
        // D s_47_1: cast zx s_47_0 -> bv
        let s_47_1: Bits = Bits::new(s_47_0 as u128, 1u16);
        // C s_47_2: const #1u : u8
        let s_47_2: bool = true;
        // C s_47_3: cast zx s_47_2 -> bv
        let s_47_3: Bits = Bits::new(s_47_2 as u128, 1u16);
        // D s_47_4: cmp-eq s_47_1 s_47_3
        let s_47_4: bool = ((s_47_1) == (s_47_3));
        // D s_47_5: write-var gs#128505 <= s_47_4
        fn_state.gs_128505 = s_47_4;
        // N s_47_6: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #424u : u32
        let s_48_0: u32 = 424;
        // D s_48_1: read-reg s_48_0:u8
        let s_48_1: u8 = {
            let value = state.read_register::<u8>(s_48_0 as isize);
            tracer.read_register(s_48_0 as isize, value);
            value
        };
        // D s_48_2: call ELUsingAArch32(s_48_1)
        let s_48_2: bool = ELUsingAArch32(state, tracer, s_48_1);
        // D s_48_3: write-var gs#128504 <= s_48_2
        fn_state.gs_128504 = s_48_2;
        // N s_48_4: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #() : ()
        let s_49_0: () = ();
        // S s_49_1: call Halted(s_49_0)
        let s_49_1: bool = Halted(state, tracer, s_49_0);
        // N s_49_2: branch s_49_1 b54 b50
        if s_49_1 {
            return block_54(state, tracer, fn_state);
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
        // D s_50_1: write-var gs#128507 <= s_50_0
        fn_state.gs_128507 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#128507:u8
        let s_51_0: bool = fn_state.gs_128507;
        // N s_51_1: branch s_51_0 b53 b52
        if s_51_0 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #3u : u8
        let s_52_0: u8 = 3;
        // C s_52_1: cast zx s_52_0 -> bv
        let s_52_1: Bits = Bits::new(s_52_0 as u128, 8u16);
        // C s_52_2: cast zx s_52_1 -> i
        let s_52_2: i128 = (s_52_1.value() as i128);
        // C s_52_3: cast reint s_52_2 -> i64
        let s_52_3: i64 = (s_52_2 as i64);
        // C s_52_4: cast zx s_52_3 -> i
        let s_52_4: i128 = (i128::try_from(s_52_3).unwrap());
        // C s_52_5: const #424u : u32
        let s_52_5: u32 = 424;
        // D s_52_6: read-reg s_52_5:u8
        let s_52_6: u8 = {
            let value = state.read_register::<u8>(s_52_5 as isize);
            tracer.read_register(s_52_5 as isize, value);
            value
        };
        // D s_52_7: call AArch64_AArch32SystemAccessTrap(s_52_6, s_52_4)
        let s_52_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_52_6, s_52_4);
        // N s_52_8: return
        return;
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_53_0: panic
        panic!("{:?}", ());
        // N s_53_1: return
        return;
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #() : ()
        let s_54_0: () = ();
        // S s_54_1: call EDSCR_read(s_54_0)
        let s_54_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_54_0);
        // S s_54_2: call _get_EDSCR_Type_SDD(s_54_1)
        let s_54_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_54_1);
        // S s_54_3: cast zx s_54_2 -> bv
        let s_54_3: Bits = Bits::new(s_54_2 as u128, 1u16);
        // C s_54_4: const #1u : u8
        let s_54_4: bool = true;
        // C s_54_5: cast zx s_54_4 -> bv
        let s_54_5: Bits = Bits::new(s_54_4 as u128, 1u16);
        // S s_54_6: cmp-eq s_54_3 s_54_5
        let s_54_6: bool = ((s_54_3) == (s_54_5));
        // D s_54_7: write-var gs#128507 <= s_54_6
        fn_state.gs_128507 = s_54_6;
        // N s_54_8: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var __MDCR_EL3_TTRF:u8
        let s_55_0: bool = fn_state.u__MDCR_EL3_TTRF;
        // D s_55_1: cast zx s_55_0 -> bv
        let s_55_1: Bits = Bits::new(s_55_0 as u128, 1u16);
        // C s_55_2: const #1u : u8
        let s_55_2: bool = true;
        // C s_55_3: cast zx s_55_2 -> bv
        let s_55_3: Bits = Bits::new(s_55_2 as u128, 1u16);
        // D s_55_4: cmp-eq s_55_1 s_55_3
        let s_55_4: bool = ((s_55_1) == (s_55_3));
        // D s_55_5: write-var gs#128503 <= s_55_4
        fn_state.gs_128503 = s_55_4;
        // N s_55_6: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #424u : u32
        let s_56_0: u32 = 424;
        // D s_56_1: read-reg s_56_0:u8
        let s_56_1: u8 = {
            let value = state.read_register::<u8>(s_56_0 as isize);
            tracer.read_register(s_56_0 as isize, value);
            value
        };
        // D s_56_2: call ELUsingAArch32(s_56_1)
        let s_56_2: bool = ELUsingAArch32(state, tracer, s_56_1);
        // D s_56_3: not s_56_2
        let s_56_3: bool = !s_56_2;
        // D s_56_4: write-var gs#128502 <= s_56_3
        fn_state.gs_128502 = s_56_3;
        // N s_56_5: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_57_0: panic
        panic!("{:?}", ());
        // N s_57_1: return
        return;
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var __SDCR_TTRF:u8
        let s_58_0: bool = fn_state.u__SDCR_TTRF;
        // D s_58_1: cast zx s_58_0 -> bv
        let s_58_1: Bits = Bits::new(s_58_0 as u128, 1u16);
        // C s_58_2: const #1u : u8
        let s_58_2: bool = true;
        // C s_58_3: cast zx s_58_2 -> bv
        let s_58_3: Bits = Bits::new(s_58_2 as u128, 1u16);
        // D s_58_4: cmp-eq s_58_1 s_58_3
        let s_58_4: bool = ((s_58_1) == (s_58_3));
        // D s_58_5: write-var gs#128501 <= s_58_4
        fn_state.gs_128501 = s_58_4;
        // N s_58_6: jump b29
        return block_29(state, tracer, fn_state);
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
        // D s_59_3: write-var gs#128500 <= s_59_2
        fn_state.gs_128500 = s_59_2;
        // N s_59_4: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_60_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_60_1: call __IMPDEF_boolean(s_60_0)
        let s_60_1: bool = u__IMPDEF_boolean(state, tracer, s_60_0);
        // D s_60_2: write-var gs#128499 <= s_60_1
        fn_state.gs_128499 = s_60_1;
        // N s_60_3: jump b25
        return block_25(state, tracer, fn_state);
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
        // D s_61_7: write-var gs#128498 <= s_61_6
        fn_state.gs_128498 = s_61_6;
        // N s_61_8: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #424u : u32
        let s_62_0: u32 = 424;
        // D s_62_1: read-reg s_62_0:u8
        let s_62_1: u8 = {
            let value = state.read_register::<u8>(s_62_0 as isize);
            tracer.read_register(s_62_0 as isize, value);
            value
        };
        // C s_62_2: const #2u : u8
        let s_62_2: u8 = 2;
        // D s_62_3: cmp-lt s_62_1 s_62_2
        let s_62_3: bool = ((s_62_1) < (s_62_2));
        // D s_62_4: write-var gs#128497 <= s_62_3
        fn_state.gs_128497 = s_62_3;
        // N s_62_5: jump b21
        return block_21(state, tracer, fn_state);
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
        // D s_64_0: read-var __MDCR_EL3_TTRF:u8
        let s_64_0: bool = fn_state.u__MDCR_EL3_TTRF;
        // D s_64_1: cast zx s_64_0 -> bv
        let s_64_1: Bits = Bits::new(s_64_0 as u128, 1u16);
        // C s_64_2: const #1u : u8
        let s_64_2: bool = true;
        // C s_64_3: cast zx s_64_2 -> bv
        let s_64_3: Bits = Bits::new(s_64_2 as u128, 1u16);
        // D s_64_4: cmp-eq s_64_1 s_64_3
        let s_64_4: bool = ((s_64_1) == (s_64_3));
        // D s_64_5: write-var gs#128496 <= s_64_4
        fn_state.gs_128496 = s_64_4;
        // N s_64_6: jump b18
        return block_18(state, tracer, fn_state);
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
        // D s_65_3: not s_65_2
        let s_65_3: bool = !s_65_2;
        // D s_65_4: write-var gs#128495 <= s_65_3
        fn_state.gs_128495 = s_65_3;
        // N s_65_5: jump b16
        return block_16(state, tracer, fn_state);
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
        // D s_66_2: write-var gs#128494 <= s_66_1
        fn_state.gs_128494 = s_66_1;
        // N s_66_3: jump b14
        return block_14(state, tracer, fn_state);
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
        // D s_67_7: write-var gs#128493 <= s_67_6
        fn_state.gs_128493 = s_67_6;
        // N s_67_8: jump b12
        return block_12(state, tracer, fn_state);
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
        // D s_68_4: write-var gs#128492 <= s_68_3
        fn_state.gs_128492 = s_68_3;
        // N s_68_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #() : ()
        let s_69_0: () = ();
        // S s_69_1: call EL2Enabled(s_69_0)
        let s_69_1: bool = EL2Enabled(state, tracer, s_69_0);
        // N s_69_2: branch s_69_1 b85 b70
        if s_69_1 {
            return block_85(state, tracer, fn_state);
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
        // D s_70_1: write-var gs#128508 <= s_70_0
        fn_state.gs_128508 = s_70_0;
        // N s_70_2: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var gs#128508:u8
        let s_71_0: bool = fn_state.gs_128508;
        // N s_71_1: branch s_71_0 b84 b72
        if s_71_0 {
            return block_84(state, tracer, fn_state);
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
        // D s_72_1: write-var gs#128509 <= s_72_0
        fn_state.gs_128509 = s_72_0;
        // N s_72_2: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var gs#128509:u8
        let s_73_0: bool = fn_state.gs_128509;
        // N s_73_1: branch s_73_0 b83 b74
        if s_73_0 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_74(state, tracer, fn_state);
        };
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #() : ()
        let s_74_0: () = ();
        // S s_74_1: call EL2Enabled(s_74_0)
        let s_74_1: bool = EL2Enabled(state, tracer, s_74_0);
        // N s_74_2: branch s_74_1 b82 b75
        if s_74_1 {
            return block_82(state, tracer, fn_state);
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
        // D s_75_1: write-var gs#128510 <= s_75_0
        fn_state.gs_128510 = s_75_0;
        // N s_75_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var gs#128510:u8
        let s_76_0: bool = fn_state.gs_128510;
        // N s_76_1: branch s_76_0 b81 b77
        if s_76_0 {
            return block_81(state, tracer, fn_state);
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
        // D s_77_1: write-var gs#128511 <= s_77_0
        fn_state.gs_128511 = s_77_0;
        // N s_77_2: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var gs#128511:u8
        let s_78_0: bool = fn_state.gs_128511;
        // N s_78_1: branch s_78_0 b80 b79
        if s_78_0 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_79_0: panic
        panic!("{:?}", ());
        // N s_79_1: return
        return;
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #3u : u8
        let s_80_0: u8 = 3;
        // C s_80_1: cast zx s_80_0 -> bv
        let s_80_1: Bits = Bits::new(s_80_0 as u128, 8u16);
        // C s_80_2: cast zx s_80_1 -> i
        let s_80_2: i128 = (s_80_1.value() as i128);
        // C s_80_3: cast reint s_80_2 -> i64
        let s_80_3: i64 = (s_80_2 as i64);
        // C s_80_4: cast zx s_80_3 -> i
        let s_80_4: i128 = (i128::try_from(s_80_3).unwrap());
        // S s_80_5: call AArch32_TakeHypTrapException(s_80_4)
        let s_80_5: () = AArch32_TakeHypTrapException(state, tracer, s_80_4);
        // N s_80_6: return
        return;
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var __HSTR_T1:u8
        let s_81_0: bool = fn_state.u__HSTR_T1;
        // D s_81_1: cast zx s_81_0 -> bv
        let s_81_1: Bits = Bits::new(s_81_0 as u128, 1u16);
        // C s_81_2: const #1u : u8
        let s_81_2: bool = true;
        // C s_81_3: cast zx s_81_2 -> bv
        let s_81_3: Bits = Bits::new(s_81_2 as u128, 1u16);
        // D s_81_4: cmp-eq s_81_1 s_81_3
        let s_81_4: bool = ((s_81_1) == (s_81_3));
        // D s_81_5: write-var gs#128511 <= s_81_4
        fn_state.gs_128511 = s_81_4;
        // N s_81_6: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #432u : u32
        let s_82_0: u32 = 432;
        // D s_82_1: read-reg s_82_0:u8
        let s_82_1: u8 = {
            let value = state.read_register::<u8>(s_82_0 as isize);
            tracer.read_register(s_82_0 as isize, value);
            value
        };
        // D s_82_2: call ELUsingAArch32(s_82_1)
        let s_82_2: bool = ELUsingAArch32(state, tracer, s_82_1);
        // D s_82_3: write-var gs#128510 <= s_82_2
        fn_state.gs_128510 = s_82_2;
        // N s_82_4: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #3u : u8
        let s_83_0: u8 = 3;
        // C s_83_1: cast zx s_83_0 -> bv
        let s_83_1: Bits = Bits::new(s_83_0 as u128, 8u16);
        // C s_83_2: cast zx s_83_1 -> i
        let s_83_2: i128 = (s_83_1.value() as i128);
        // C s_83_3: cast reint s_83_2 -> i64
        let s_83_3: i64 = (s_83_2 as i64);
        // C s_83_4: cast zx s_83_3 -> i
        let s_83_4: i128 = (i128::try_from(s_83_3).unwrap());
        // C s_83_5: const #432u : u32
        let s_83_5: u32 = 432;
        // D s_83_6: read-reg s_83_5:u8
        let s_83_6: u8 = {
            let value = state.read_register::<u8>(s_83_5 as isize);
            tracer.read_register(s_83_5 as isize, value);
            value
        };
        // D s_83_7: call AArch64_AArch32SystemAccessTrap(s_83_6, s_83_4)
        let s_83_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_83_6, s_83_4);
        // N s_83_8: return
        return;
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var __HSTR_EL2_T1:u8
        let s_84_0: bool = fn_state.u__HSTR_EL2_T1;
        // D s_84_1: cast zx s_84_0 -> bv
        let s_84_1: Bits = Bits::new(s_84_0 as u128, 1u16);
        // C s_84_2: const #1u : u8
        let s_84_2: bool = true;
        // C s_84_3: cast zx s_84_2 -> bv
        let s_84_3: Bits = Bits::new(s_84_2 as u128, 1u16);
        // D s_84_4: cmp-eq s_84_1 s_84_3
        let s_84_4: bool = ((s_84_1) == (s_84_3));
        // D s_84_5: write-var gs#128509 <= s_84_4
        fn_state.gs_128509 = s_84_4;
        // N s_84_6: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #432u : u32
        let s_85_0: u32 = 432;
        // D s_85_1: read-reg s_85_0:u8
        let s_85_1: u8 = {
            let value = state.read_register::<u8>(s_85_0 as isize);
            tracer.read_register(s_85_0 as isize, value);
            value
        };
        // D s_85_2: call ELUsingAArch32(s_85_1)
        let s_85_2: bool = ELUsingAArch32(state, tracer, s_85_1);
        // D s_85_3: not s_85_2
        let s_85_3: bool = !s_85_2;
        // D s_85_4: write-var gs#128508 <= s_85_3
        fn_state.gs_128508 = s_85_3;
        // N s_85_5: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_86_0: panic
        panic!("{:?}", ());
        // N s_86_1: return
        return;
    }
}
