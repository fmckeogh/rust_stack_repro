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
use AArch32_ITLBI_VA::*;
use HCR_read::*;
use IsFeatureImplemented::*;
use u_get_HSTR_EL2_Type_T8::*;
use VMID_read::*;
use u_get_HCR_Type_TTLB::*;
use HSTR_read::*;
use AArch64_AArch32SystemAccessTrap::*;
use u_get_HCR_EL2_Type_TTLB::*;
use R_read::*;
use IsHCRXEL2Enabled::*;
use ELUsingAArch32::*;
use SecurityStateAtEL::*;
use u_get_HSTR_Type_T8::*;
use EL2Enabled::*;
use u_get_HCRX_EL2_Type_FnXS::*;
use AArch32_TakeHypTrapException::*;
use common::*;
pub fn ITLBIMVA_SysRegWrite32_ff4131cdf091f782<T: Tracer>(
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
        gs_129558: bool,
        gs_129555: bool,
        gs_129556: bool,
        u__HCR_TTLB: bool,
        u__HSTR_EL2_T8: bool,
        gs_129561: bool,
        gs_129552: bool,
        gs_129554: bool,
        gs_129560: bool,
        u__HCRX_EL2_FnXS: bool,
        gs_129562: bool,
        u__PSTATE_EL: u8,
        gs_129559: bool,
        gs_129551: bool,
        gs_129553: bool,
        u__HSTR_T8: bool,
        gs_129557: bool,
        u__HCR_EL2_TTLB: bool,
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
        // D s_0_5: call _get_HSTR_EL2_Type_T8(s_0_4)
        let s_0_5: bool = u_get_HSTR_EL2_Type_T8(state, tracer, s_0_4);
        // D s_0_6: write-var __HSTR_EL2_T8 <= s_0_5
        fn_state.u__HSTR_EL2_T8 = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call HSTR_read(s_0_7)
        let s_0_8: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_7);
        // S s_0_9: call _get_HSTR_Type_T8(s_0_8)
        let s_0_9: bool = u_get_HSTR_Type_T8(state, tracer, s_0_8);
        // D s_0_10: write-var __HSTR_T8 <= s_0_9
        fn_state.u__HSTR_T8 = s_0_9;
        // C s_0_11: const #102552u : u32
        let s_0_11: u32 = 102552;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_HCR_EL2_Type_TTLB(s_0_12)
        let s_0_13: bool = u_get_HCR_EL2_Type_TTLB(state, tracer, s_0_12);
        // D s_0_14: write-var __HCR_EL2_TTLB <= s_0_13
        fn_state.u__HCR_EL2_TTLB = s_0_13;
        // C s_0_15: const #() : ()
        let s_0_15: () = ();
        // S s_0_16: call HCR_read(s_0_15)
        let s_0_16: ProductType700c18a878c5601b = HCR_read(state, tracer, s_0_15);
        // S s_0_17: call _get_HCR_Type_TTLB(s_0_16)
        let s_0_17: bool = u_get_HCR_Type_TTLB(state, tracer, s_0_16);
        // D s_0_18: write-var __HCR_TTLB <= s_0_17
        fn_state.u__HCR_TTLB = s_0_17;
        // C s_0_19: const #22528u : u32
        let s_0_19: u32 = 22528;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_HCRX_EL2_Type_FnXS(s_0_20)
        let s_0_21: bool = u_get_HCRX_EL2_Type_FnXS(state, tracer, s_0_20);
        // D s_0_22: write-var __HCRX_EL2_FnXS <= s_0_21
        fn_state.u__HCRX_EL2_FnXS = s_0_21;
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
        // N s_0_29: branch s_0_28 b54 b1
        if s_0_28 {
            return block_54(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b7 b2
        if s_1_5 {
            return block_7(state, tracer, fn_state);
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
        // C s_5_0: const #424u : u32
        let s_5_0: u32 = 424;
        // D s_5_1: read-reg s_5_0:u8
        let s_5_1: u8 = {
            let value = state.read_register::<u8>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: call SecurityStateAtEL(s_5_1)
        let s_5_2: u32 = SecurityStateAtEL(state, tracer, s_5_1);
        // D s_5_3: read-var t:i
        let s_5_3: i128 = fn_state.t;
        // D s_5_4: call R_read(s_5_3)
        let s_5_4: u32 = R_read(state, tracer, s_5_3);
        // C s_5_5: const #1u : u32
        let s_5_5: u32 = 1;
        // C s_5_6: const #1080u : u32
        let s_5_6: u32 = 1080;
        // D s_5_7: read-reg s_5_6:u16
        let s_5_7: u16 = {
            let value = state.read_register::<u16>(s_5_6 as isize);
            tracer.read_register(s_5_6 as isize, value);
            value
        };
        // C s_5_8: const #0u : u32
        let s_5_8: u32 = 0;
        // C s_5_9: const #0u : u32
        let s_5_9: u32 = 0;
        // C s_5_10: const #0u : u32
        let s_5_10: u32 = 0;
        // D s_5_11: call AArch32_ITLBI_VA(s_5_2, s_5_5, s_5_7, s_5_8, s_5_9, s_5_10, s_5_4)
        let s_5_11: () = AArch32_ITLBI_VA(
            state,
            tracer,
            s_5_2,
            s_5_5,
            s_5_7,
            s_5_8,
            s_5_9,
            s_5_10,
            s_5_4,
        );
        // N s_5_12: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #440u : u32
        let s_6_0: u32 = 440;
        // D s_6_1: read-reg s_6_0:u8
        let s_6_1: u8 = {
            let value = state.read_register::<u8>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: call SecurityStateAtEL(s_6_1)
        let s_6_2: u32 = SecurityStateAtEL(state, tracer, s_6_1);
        // C s_6_3: const #() : ()
        let s_6_3: () = ();
        // S s_6_4: call VMID_read(s_6_3)
        let s_6_4: u16 = VMID_read(state, tracer, s_6_3);
        // D s_6_5: read-var t:i
        let s_6_5: i128 = fn_state.t;
        // D s_6_6: call R_read(s_6_5)
        let s_6_6: u32 = R_read(state, tracer, s_6_5);
        // C s_6_7: const #4u : u32
        let s_6_7: u32 = 4;
        // C s_6_8: const #0u : u32
        let s_6_8: u32 = 0;
        // C s_6_9: const #0u : u32
        let s_6_9: u32 = 0;
        // C s_6_10: const #0u : u32
        let s_6_10: u32 = 0;
        // D s_6_11: call AArch32_ITLBI_VA(s_6_2, s_6_7, s_6_4, s_6_8, s_6_9, s_6_10, s_6_6)
        let s_6_11: () = AArch32_ITLBI_VA(
            state,
            tracer,
            s_6_2,
            s_6_7,
            s_6_4,
            s_6_8,
            s_6_9,
            s_6_10,
            s_6_6,
        );
        // N s_6_12: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call EL2Enabled(s_7_0)
        let s_7_1: bool = EL2Enabled(state, tracer, s_7_0);
        // N s_7_2: branch s_7_1 b53 b8
        if s_7_1 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#129551 <= s_8_0
        fn_state.gs_129551 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#129551:u8
        let s_9_0: bool = fn_state.gs_129551;
        // N s_9_1: branch s_9_0 b52 b10
        if s_9_0 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#129552 <= s_10_0
        fn_state.gs_129552 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#129552:u8
        let s_11_0: bool = fn_state.gs_129552;
        // N s_11_1: branch s_11_0 b51 b12
        if s_11_0 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call EL2Enabled(s_12_0)
        let s_12_1: bool = EL2Enabled(state, tracer, s_12_0);
        // N s_12_2: branch s_12_1 b50 b13
        if s_12_1 {
            return block_50(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#129553 <= s_13_0
        fn_state.gs_129553 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#129553:u8
        let s_14_0: bool = fn_state.gs_129553;
        // N s_14_1: branch s_14_0 b49 b15
        if s_14_0 {
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
        // D s_15_1: write-var gs#129554 <= s_15_0
        fn_state.gs_129554 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#129554:u8
        let s_16_0: bool = fn_state.gs_129554;
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
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call EL2Enabled(s_17_0)
        let s_17_1: bool = EL2Enabled(state, tracer, s_17_0);
        // N s_17_2: branch s_17_1 b47 b18
        if s_17_1 {
            return block_47(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#129555 <= s_18_0
        fn_state.gs_129555 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#129555:u8
        let s_19_0: bool = fn_state.gs_129555;
        // N s_19_1: branch s_19_0 b46 b20
        if s_19_0 {
            return block_46(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#129556 <= s_20_0
        fn_state.gs_129556 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#129556:u8
        let s_21_0: bool = fn_state.gs_129556;
        // N s_21_1: branch s_21_0 b45 b22
        if s_21_0 {
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
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call EL2Enabled(s_22_0)
        let s_22_1: bool = EL2Enabled(state, tracer, s_22_0);
        // N s_22_2: branch s_22_1 b44 b23
        if s_22_1 {
            return block_44(state, tracer, fn_state);
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
        // D s_23_1: write-var gs#129557 <= s_23_0
        fn_state.gs_129557 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#129557:u8
        let s_24_0: bool = fn_state.gs_129557;
        // N s_24_1: branch s_24_0 b43 b25
        if s_24_0 {
            return block_43(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#129558 <= s_25_0
        fn_state.gs_129558 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#129558:u8
        let s_26_0: bool = fn_state.gs_129558;
        // N s_26_1: branch s_26_0 b42 b27
        if s_26_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #166u : u32
        let s_27_0: u32 = 166;
        // S s_27_1: call IsFeatureImplemented(s_27_0)
        let s_27_1: bool = IsFeatureImplemented(state, tracer, s_27_0);
        // N s_27_2: branch s_27_1 b41 b28
        if s_27_1 {
            return block_41(state, tracer, fn_state);
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
        // D s_28_1: write-var gs#129559 <= s_28_0
        fn_state.gs_129559 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#129559:u8
        let s_29_0: bool = fn_state.gs_129559;
        // N s_29_1: branch s_29_0 b40 b30
        if s_29_0 {
            return block_40(state, tracer, fn_state);
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
        // D s_30_1: write-var gs#129560 <= s_30_0
        fn_state.gs_129560 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#129560:u8
        let s_31_0: bool = fn_state.gs_129560;
        // N s_31_1: branch s_31_0 b39 b32
        if s_31_0 {
            return block_39(state, tracer, fn_state);
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
        // D s_32_1: write-var gs#129561 <= s_32_0
        fn_state.gs_129561 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#129561:u8
        let s_33_0: bool = fn_state.gs_129561;
        // N s_33_1: branch s_33_0 b38 b34
        if s_33_0 {
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
        // D s_34_1: write-var gs#129562 <= s_34_0
        fn_state.gs_129562 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#129562:u8
        let s_35_0: bool = fn_state.gs_129562;
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
        // C s_36_0: const #440u : u32
        let s_36_0: u32 = 440;
        // D s_36_1: read-reg s_36_0:u8
        let s_36_1: u8 = {
            let value = state.read_register::<u8>(s_36_0 as isize);
            tracer.read_register(s_36_0 as isize, value);
            value
        };
        // D s_36_2: call SecurityStateAtEL(s_36_1)
        let s_36_2: u32 = SecurityStateAtEL(state, tracer, s_36_1);
        // C s_36_3: const #() : ()
        let s_36_3: () = ();
        // S s_36_4: call VMID_read(s_36_3)
        let s_36_4: u16 = VMID_read(state, tracer, s_36_3);
        // D s_36_5: read-var t:i
        let s_36_5: i128 = fn_state.t;
        // D s_36_6: call R_read(s_36_5)
        let s_36_6: u32 = R_read(state, tracer, s_36_5);
        // C s_36_7: const #4u : u32
        let s_36_7: u32 = 4;
        // C s_36_8: const #0u : u32
        let s_36_8: u32 = 0;
        // C s_36_9: const #0u : u32
        let s_36_9: u32 = 0;
        // C s_36_10: const #0u : u32
        let s_36_10: u32 = 0;
        // D s_36_11: call AArch32_ITLBI_VA(s_36_2, s_36_7, s_36_4, s_36_8, s_36_9, s_36_10, s_36_6)
        let s_36_11: () = AArch32_ITLBI_VA(
            state,
            tracer,
            s_36_2,
            s_36_7,
            s_36_4,
            s_36_8,
            s_36_9,
            s_36_10,
            s_36_6,
        );
        // N s_36_12: return
        return;
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #440u : u32
        let s_37_0: u32 = 440;
        // D s_37_1: read-reg s_37_0:u8
        let s_37_1: u8 = {
            let value = state.read_register::<u8>(s_37_0 as isize);
            tracer.read_register(s_37_0 as isize, value);
            value
        };
        // D s_37_2: call SecurityStateAtEL(s_37_1)
        let s_37_2: u32 = SecurityStateAtEL(state, tracer, s_37_1);
        // C s_37_3: const #() : ()
        let s_37_3: () = ();
        // S s_37_4: call VMID_read(s_37_3)
        let s_37_4: u16 = VMID_read(state, tracer, s_37_3);
        // D s_37_5: read-var t:i
        let s_37_5: i128 = fn_state.t;
        // D s_37_6: call R_read(s_37_5)
        let s_37_6: u32 = R_read(state, tracer, s_37_5);
        // C s_37_7: const #4u : u32
        let s_37_7: u32 = 4;
        // C s_37_8: const #0u : u32
        let s_37_8: u32 = 0;
        // C s_37_9: const #0u : u32
        let s_37_9: u32 = 0;
        // C s_37_10: const #1u : u32
        let s_37_10: u32 = 1;
        // D s_37_11: call AArch32_ITLBI_VA(s_37_2, s_37_7, s_37_4, s_37_8, s_37_9, s_37_10, s_37_6)
        let s_37_11: () = AArch32_ITLBI_VA(
            state,
            tracer,
            s_37_2,
            s_37_7,
            s_37_4,
            s_37_8,
            s_37_9,
            s_37_10,
            s_37_6,
        );
        // N s_37_12: return
        return;
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var __HCRX_EL2_FnXS:u8
        let s_38_0: bool = fn_state.u__HCRX_EL2_FnXS;
        // D s_38_1: cast zx s_38_0 -> bv
        let s_38_1: Bits = Bits::new(s_38_0 as u128, 1u16);
        // C s_38_2: const #1u : u8
        let s_38_2: bool = true;
        // C s_38_3: cast zx s_38_2 -> bv
        let s_38_3: Bits = Bits::new(s_38_2 as u128, 1u16);
        // D s_38_4: cmp-eq s_38_1 s_38_3
        let s_38_4: bool = ((s_38_1) == (s_38_3));
        // D s_38_5: write-var gs#129562 <= s_38_4
        fn_state.gs_129562 = s_38_4;
        // N s_38_6: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #() : ()
        let s_39_0: () = ();
        // S s_39_1: call IsHCRXEL2Enabled(s_39_0)
        let s_39_1: bool = IsHCRXEL2Enabled(state, tracer, s_39_0);
        // D s_39_2: write-var gs#129561 <= s_39_1
        fn_state.gs_129561 = s_39_1;
        // N s_39_3: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #155u : u32
        let s_40_0: u32 = 155;
        // S s_40_1: call IsFeatureImplemented(s_40_0)
        let s_40_1: bool = IsFeatureImplemented(state, tracer, s_40_0);
        // D s_40_2: write-var gs#129560 <= s_40_1
        fn_state.gs_129560 = s_40_1;
        // N s_40_3: jump b31
        return block_31(state, tracer, fn_state);
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
        // D s_41_3: not s_41_2
        let s_41_3: bool = !s_41_2;
        // D s_41_4: write-var gs#129559 <= s_41_3
        fn_state.gs_129559 = s_41_3;
        // N s_41_5: jump b29
        return block_29(state, tracer, fn_state);
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
        // S s_42_5: call AArch32_TakeHypTrapException(s_42_4)
        let s_42_5: () = AArch32_TakeHypTrapException(state, tracer, s_42_4);
        // N s_42_6: return
        return;
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var __HCR_TTLB:u8
        let s_43_0: bool = fn_state.u__HCR_TTLB;
        // D s_43_1: cast zx s_43_0 -> bv
        let s_43_1: Bits = Bits::new(s_43_0 as u128, 1u16);
        // C s_43_2: const #1u : u8
        let s_43_2: bool = true;
        // C s_43_3: cast zx s_43_2 -> bv
        let s_43_3: Bits = Bits::new(s_43_2 as u128, 1u16);
        // D s_43_4: cmp-eq s_43_1 s_43_3
        let s_43_4: bool = ((s_43_1) == (s_43_3));
        // D s_43_5: write-var gs#129558 <= s_43_4
        fn_state.gs_129558 = s_43_4;
        // N s_43_6: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #432u : u32
        let s_44_0: u32 = 432;
        // D s_44_1: read-reg s_44_0:u8
        let s_44_1: u8 = {
            let value = state.read_register::<u8>(s_44_0 as isize);
            tracer.read_register(s_44_0 as isize, value);
            value
        };
        // D s_44_2: call ELUsingAArch32(s_44_1)
        let s_44_2: bool = ELUsingAArch32(state, tracer, s_44_1);
        // D s_44_3: write-var gs#129557 <= s_44_2
        fn_state.gs_129557 = s_44_2;
        // N s_44_4: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #3u : u8
        let s_45_0: u8 = 3;
        // C s_45_1: cast zx s_45_0 -> bv
        let s_45_1: Bits = Bits::new(s_45_0 as u128, 8u16);
        // C s_45_2: cast zx s_45_1 -> i
        let s_45_2: i128 = (s_45_1.value() as i128);
        // C s_45_3: cast reint s_45_2 -> i64
        let s_45_3: i64 = (s_45_2 as i64);
        // C s_45_4: cast zx s_45_3 -> i
        let s_45_4: i128 = (i128::try_from(s_45_3).unwrap());
        // C s_45_5: const #432u : u32
        let s_45_5: u32 = 432;
        // D s_45_6: read-reg s_45_5:u8
        let s_45_6: u8 = {
            let value = state.read_register::<u8>(s_45_5 as isize);
            tracer.read_register(s_45_5 as isize, value);
            value
        };
        // D s_45_7: call AArch64_AArch32SystemAccessTrap(s_45_6, s_45_4)
        let s_45_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_45_6, s_45_4);
        // N s_45_8: return
        return;
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var __HCR_EL2_TTLB:u8
        let s_46_0: bool = fn_state.u__HCR_EL2_TTLB;
        // D s_46_1: cast zx s_46_0 -> bv
        let s_46_1: Bits = Bits::new(s_46_0 as u128, 1u16);
        // C s_46_2: const #1u : u8
        let s_46_2: bool = true;
        // C s_46_3: cast zx s_46_2 -> bv
        let s_46_3: Bits = Bits::new(s_46_2 as u128, 1u16);
        // D s_46_4: cmp-eq s_46_1 s_46_3
        let s_46_4: bool = ((s_46_1) == (s_46_3));
        // D s_46_5: write-var gs#129556 <= s_46_4
        fn_state.gs_129556 = s_46_4;
        // N s_46_6: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #432u : u32
        let s_47_0: u32 = 432;
        // D s_47_1: read-reg s_47_0:u8
        let s_47_1: u8 = {
            let value = state.read_register::<u8>(s_47_0 as isize);
            tracer.read_register(s_47_0 as isize, value);
            value
        };
        // D s_47_2: call ELUsingAArch32(s_47_1)
        let s_47_2: bool = ELUsingAArch32(state, tracer, s_47_1);
        // D s_47_3: not s_47_2
        let s_47_3: bool = !s_47_2;
        // D s_47_4: write-var gs#129555 <= s_47_3
        fn_state.gs_129555 = s_47_3;
        // N s_47_5: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #3u : u8
        let s_48_0: u8 = 3;
        // C s_48_1: cast zx s_48_0 -> bv
        let s_48_1: Bits = Bits::new(s_48_0 as u128, 8u16);
        // C s_48_2: cast zx s_48_1 -> i
        let s_48_2: i128 = (s_48_1.value() as i128);
        // C s_48_3: cast reint s_48_2 -> i64
        let s_48_3: i64 = (s_48_2 as i64);
        // C s_48_4: cast zx s_48_3 -> i
        let s_48_4: i128 = (i128::try_from(s_48_3).unwrap());
        // S s_48_5: call AArch32_TakeHypTrapException(s_48_4)
        let s_48_5: () = AArch32_TakeHypTrapException(state, tracer, s_48_4);
        // N s_48_6: return
        return;
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var __HSTR_T8:u8
        let s_49_0: bool = fn_state.u__HSTR_T8;
        // D s_49_1: cast zx s_49_0 -> bv
        let s_49_1: Bits = Bits::new(s_49_0 as u128, 1u16);
        // C s_49_2: const #1u : u8
        let s_49_2: bool = true;
        // C s_49_3: cast zx s_49_2 -> bv
        let s_49_3: Bits = Bits::new(s_49_2 as u128, 1u16);
        // D s_49_4: cmp-eq s_49_1 s_49_3
        let s_49_4: bool = ((s_49_1) == (s_49_3));
        // D s_49_5: write-var gs#129554 <= s_49_4
        fn_state.gs_129554 = s_49_4;
        // N s_49_6: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #432u : u32
        let s_50_0: u32 = 432;
        // D s_50_1: read-reg s_50_0:u8
        let s_50_1: u8 = {
            let value = state.read_register::<u8>(s_50_0 as isize);
            tracer.read_register(s_50_0 as isize, value);
            value
        };
        // D s_50_2: call ELUsingAArch32(s_50_1)
        let s_50_2: bool = ELUsingAArch32(state, tracer, s_50_1);
        // D s_50_3: write-var gs#129553 <= s_50_2
        fn_state.gs_129553 = s_50_2;
        // N s_50_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #3u : u8
        let s_51_0: u8 = 3;
        // C s_51_1: cast zx s_51_0 -> bv
        let s_51_1: Bits = Bits::new(s_51_0 as u128, 8u16);
        // C s_51_2: cast zx s_51_1 -> i
        let s_51_2: i128 = (s_51_1.value() as i128);
        // C s_51_3: cast reint s_51_2 -> i64
        let s_51_3: i64 = (s_51_2 as i64);
        // C s_51_4: cast zx s_51_3 -> i
        let s_51_4: i128 = (i128::try_from(s_51_3).unwrap());
        // C s_51_5: const #432u : u32
        let s_51_5: u32 = 432;
        // D s_51_6: read-reg s_51_5:u8
        let s_51_6: u8 = {
            let value = state.read_register::<u8>(s_51_5 as isize);
            tracer.read_register(s_51_5 as isize, value);
            value
        };
        // D s_51_7: call AArch64_AArch32SystemAccessTrap(s_51_6, s_51_4)
        let s_51_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_51_6, s_51_4);
        // N s_51_8: return
        return;
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var __HSTR_EL2_T8:u8
        let s_52_0: bool = fn_state.u__HSTR_EL2_T8;
        // D s_52_1: cast zx s_52_0 -> bv
        let s_52_1: Bits = Bits::new(s_52_0 as u128, 1u16);
        // C s_52_2: const #1u : u8
        let s_52_2: bool = true;
        // C s_52_3: cast zx s_52_2 -> bv
        let s_52_3: Bits = Bits::new(s_52_2 as u128, 1u16);
        // D s_52_4: cmp-eq s_52_1 s_52_3
        let s_52_4: bool = ((s_52_1) == (s_52_3));
        // D s_52_5: write-var gs#129552 <= s_52_4
        fn_state.gs_129552 = s_52_4;
        // N s_52_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #432u : u32
        let s_53_0: u32 = 432;
        // D s_53_1: read-reg s_53_0:u8
        let s_53_1: u8 = {
            let value = state.read_register::<u8>(s_53_0 as isize);
            tracer.read_register(s_53_0 as isize, value);
            value
        };
        // D s_53_2: call ELUsingAArch32(s_53_1)
        let s_53_2: bool = ELUsingAArch32(state, tracer, s_53_1);
        // D s_53_3: not s_53_2
        let s_53_3: bool = !s_53_2;
        // D s_53_4: write-var gs#129551 <= s_53_3
        fn_state.gs_129551 = s_53_3;
        // N s_53_5: jump b9
        return block_9(state, tracer, fn_state);
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
}
