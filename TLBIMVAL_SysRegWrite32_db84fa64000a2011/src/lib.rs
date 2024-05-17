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
use u_get_HCR_Type_FB::*;
use HCR_read::*;
use u_get_HSTR_EL2_Type_T8::*;
use IsFeatureImplemented::*;
use VMID_read::*;
use u_get_HCR_Type_TTLB::*;
use HSTR_read::*;
use AArch64_AArch32SystemAccessTrap::*;
use u_get_HCR_EL2_Type_TTLB::*;
use R_read::*;
use IsHCRXEL2Enabled::*;
use ELUsingAArch32::*;
use SecurityStateAtEL::*;
use AArch32_TLBI_VA::*;
use u_get_HSTR_Type_T8::*;
use EL2Enabled::*;
use u_get_HCRX_EL2_Type_FnXS::*;
use u_get_HCR_EL2_Type_FB::*;
use AArch32_TakeHypTrapException::*;
use common::*;
pub fn TLBIMVAL_SysRegWrite32_db84fa64000a2011<T: Tracer>(
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
        gs_134266: bool,
        u__HCR_EL2_FB: bool,
        u__HSTR_EL2_T8: bool,
        gs_134269: bool,
        gs_134270: bool,
        gs_134277: bool,
        gs_134271: bool,
        gs_134280: bool,
        gs_134278: bool,
        gs_134273: bool,
        gs_134276: bool,
        gs_134272: bool,
        u__HCR_TTLB: bool,
        gs_134265: bool,
        gs_134281: bool,
        u__HCRX_EL2_FnXS: bool,
        gs_134274: bool,
        gs_134267: bool,
        u__PSTATE_EL: u8,
        gs_134279: bool,
        gs_134268: bool,
        u__HSTR_T8: bool,
        gs_134275: bool,
        u__HCR_EL2_TTLB: bool,
        gs_134282: bool,
        u__HCR_FB: bool,
        gs_134264: bool,
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
        // C s_0_19: const #102552u : u32
        let s_0_19: u32 = 102552;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_HCR_EL2_Type_FB(s_0_20)
        let s_0_21: bool = u_get_HCR_EL2_Type_FB(state, tracer, s_0_20);
        // D s_0_22: write-var __HCR_EL2_FB <= s_0_21
        fn_state.u__HCR_EL2_FB = s_0_21;
        // C s_0_23: const #22528u : u32
        let s_0_23: u32 = 22528;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_HCRX_EL2_Type_FnXS(s_0_24)
        let s_0_25: bool = u_get_HCRX_EL2_Type_FnXS(state, tracer, s_0_24);
        // D s_0_26: write-var __HCRX_EL2_FnXS <= s_0_25
        fn_state.u__HCRX_EL2_FnXS = s_0_25;
        // C s_0_27: const #() : ()
        let s_0_27: () = ();
        // S s_0_28: call HCR_read(s_0_27)
        let s_0_28: ProductType700c18a878c5601b = HCR_read(state, tracer, s_0_27);
        // S s_0_29: call _get_HCR_Type_FB(s_0_28)
        let s_0_29: bool = u_get_HCR_Type_FB(state, tracer, s_0_28);
        // D s_0_30: write-var __HCR_FB <= s_0_29
        fn_state.u__HCR_FB = s_0_29;
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
        // N s_0_37: branch s_0_36 b81 b1
        if s_0_36 {
            return block_81(state, tracer, fn_state);
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
        // C s_5_9: const #1u : u32
        let s_5_9: u32 = 1;
        // C s_5_10: const #0u : u32
        let s_5_10: u32 = 0;
        // D s_5_11: call AArch32_TLBI_VA(s_5_2, s_5_5, s_5_7, s_5_8, s_5_9, s_5_10, s_5_4)
        let s_5_11: () = AArch32_TLBI_VA(
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
        // C s_6_9: const #1u : u32
        let s_6_9: u32 = 1;
        // C s_6_10: const #0u : u32
        let s_6_10: u32 = 0;
        // D s_6_11: call AArch32_TLBI_VA(s_6_2, s_6_7, s_6_4, s_6_8, s_6_9, s_6_10, s_6_6)
        let s_6_11: () = AArch32_TLBI_VA(
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
        // N s_7_2: branch s_7_1 b80 b8
        if s_7_1 {
            return block_80(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#134264 <= s_8_0
        fn_state.gs_134264 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#134264:u8
        let s_9_0: bool = fn_state.gs_134264;
        // N s_9_1: branch s_9_0 b79 b10
        if s_9_0 {
            return block_79(state, tracer, fn_state);
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
        // D s_10_1: write-var gs#134265 <= s_10_0
        fn_state.gs_134265 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#134265:u8
        let s_11_0: bool = fn_state.gs_134265;
        // N s_11_1: branch s_11_0 b78 b12
        if s_11_0 {
            return block_78(state, tracer, fn_state);
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
        // N s_12_2: branch s_12_1 b77 b13
        if s_12_1 {
            return block_77(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#134266 <= s_13_0
        fn_state.gs_134266 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#134266:u8
        let s_14_0: bool = fn_state.gs_134266;
        // N s_14_1: branch s_14_0 b76 b15
        if s_14_0 {
            return block_76(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#134267 <= s_15_0
        fn_state.gs_134267 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#134267:u8
        let s_16_0: bool = fn_state.gs_134267;
        // N s_16_1: branch s_16_0 b75 b17
        if s_16_0 {
            return block_75(state, tracer, fn_state);
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
        // N s_17_2: branch s_17_1 b74 b18
        if s_17_1 {
            return block_74(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#134268 <= s_18_0
        fn_state.gs_134268 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#134268:u8
        let s_19_0: bool = fn_state.gs_134268;
        // N s_19_1: branch s_19_0 b73 b20
        if s_19_0 {
            return block_73(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#134269 <= s_20_0
        fn_state.gs_134269 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#134269:u8
        let s_21_0: bool = fn_state.gs_134269;
        // N s_21_1: branch s_21_0 b72 b22
        if s_21_0 {
            return block_72(state, tracer, fn_state);
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
        // N s_22_2: branch s_22_1 b71 b23
        if s_22_1 {
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
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#134270 <= s_23_0
        fn_state.gs_134270 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#134270:u8
        let s_24_0: bool = fn_state.gs_134270;
        // N s_24_1: branch s_24_0 b70 b25
        if s_24_0 {
            return block_70(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#134271 <= s_25_0
        fn_state.gs_134271 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#134271:u8
        let s_26_0: bool = fn_state.gs_134271;
        // N s_26_1: branch s_26_0 b69 b27
        if s_26_0 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #() : ()
        let s_27_0: () = ();
        // S s_27_1: call EL2Enabled(s_27_0)
        let s_27_1: bool = EL2Enabled(state, tracer, s_27_0);
        // N s_27_2: branch s_27_1 b68 b28
        if s_27_1 {
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
        // D s_28_1: write-var gs#134272 <= s_28_0
        fn_state.gs_134272 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#134272:u8
        let s_29_0: bool = fn_state.gs_134272;
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
        // D s_30_1: write-var gs#134273 <= s_30_0
        fn_state.gs_134273 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#134273:u8
        let s_31_0: bool = fn_state.gs_134273;
        // N s_31_1: branch s_31_0 b55 b32
        if s_31_0 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #() : ()
        let s_32_0: () = ();
        // S s_32_1: call EL2Enabled(s_32_0)
        let s_32_1: bool = EL2Enabled(state, tracer, s_32_0);
        // N s_32_2: branch s_32_1 b54 b33
        if s_32_1 {
            return block_54(state, tracer, fn_state);
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
        // D s_33_1: write-var gs#134274 <= s_33_0
        fn_state.gs_134274 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#134274:u8
        let s_34_0: bool = fn_state.gs_134274;
        // N s_34_1: branch s_34_0 b53 b35
        if s_34_0 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #0u : u8
        let s_35_0: bool = false;
        // D s_35_1: write-var gs#134275 <= s_35_0
        fn_state.gs_134275 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#134275:u8
        let s_36_0: bool = fn_state.gs_134275;
        // N s_36_1: branch s_36_0 b52 b37
        if s_36_0 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #166u : u32
        let s_37_0: u32 = 166;
        // S s_37_1: call IsFeatureImplemented(s_37_0)
        let s_37_1: bool = IsFeatureImplemented(state, tracer, s_37_0);
        // N s_37_2: branch s_37_1 b51 b38
        if s_37_1 {
            return block_51(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#134276 <= s_38_0
        fn_state.gs_134276 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#134276:u8
        let s_39_0: bool = fn_state.gs_134276;
        // N s_39_1: branch s_39_0 b50 b40
        if s_39_0 {
            return block_50(state, tracer, fn_state);
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
        // D s_40_1: write-var gs#134277 <= s_40_0
        fn_state.gs_134277 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#134277:u8
        let s_41_0: bool = fn_state.gs_134277;
        // N s_41_1: branch s_41_0 b49 b42
        if s_41_0 {
            return block_49(state, tracer, fn_state);
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
        // D s_42_1: write-var gs#134278 <= s_42_0
        fn_state.gs_134278 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#134278:u8
        let s_43_0: bool = fn_state.gs_134278;
        // N s_43_1: branch s_43_0 b48 b44
        if s_43_0 {
            return block_48(state, tracer, fn_state);
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
        // D s_44_1: write-var gs#134279 <= s_44_0
        fn_state.gs_134279 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#134279:u8
        let s_45_0: bool = fn_state.gs_134279;
        // N s_45_1: branch s_45_0 b47 b46
        if s_45_0 {
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
        // C s_46_0: const #440u : u32
        let s_46_0: u32 = 440;
        // D s_46_1: read-reg s_46_0:u8
        let s_46_1: u8 = {
            let value = state.read_register::<u8>(s_46_0 as isize);
            tracer.read_register(s_46_0 as isize, value);
            value
        };
        // D s_46_2: call SecurityStateAtEL(s_46_1)
        let s_46_2: u32 = SecurityStateAtEL(state, tracer, s_46_1);
        // C s_46_3: const #() : ()
        let s_46_3: () = ();
        // S s_46_4: call VMID_read(s_46_3)
        let s_46_4: u16 = VMID_read(state, tracer, s_46_3);
        // D s_46_5: read-var t:i
        let s_46_5: i128 = fn_state.t;
        // D s_46_6: call R_read(s_46_5)
        let s_46_6: u32 = R_read(state, tracer, s_46_5);
        // C s_46_7: const #4u : u32
        let s_46_7: u32 = 4;
        // C s_46_8: const #0u : u32
        let s_46_8: u32 = 0;
        // C s_46_9: const #1u : u32
        let s_46_9: u32 = 1;
        // C s_46_10: const #0u : u32
        let s_46_10: u32 = 0;
        // D s_46_11: call AArch32_TLBI_VA(s_46_2, s_46_7, s_46_4, s_46_8, s_46_9, s_46_10, s_46_6)
        let s_46_11: () = AArch32_TLBI_VA(
            state,
            tracer,
            s_46_2,
            s_46_7,
            s_46_4,
            s_46_8,
            s_46_9,
            s_46_10,
            s_46_6,
        );
        // N s_46_12: return
        return;
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #440u : u32
        let s_47_0: u32 = 440;
        // D s_47_1: read-reg s_47_0:u8
        let s_47_1: u8 = {
            let value = state.read_register::<u8>(s_47_0 as isize);
            tracer.read_register(s_47_0 as isize, value);
            value
        };
        // D s_47_2: call SecurityStateAtEL(s_47_1)
        let s_47_2: u32 = SecurityStateAtEL(state, tracer, s_47_1);
        // C s_47_3: const #() : ()
        let s_47_3: () = ();
        // S s_47_4: call VMID_read(s_47_3)
        let s_47_4: u16 = VMID_read(state, tracer, s_47_3);
        // D s_47_5: read-var t:i
        let s_47_5: i128 = fn_state.t;
        // D s_47_6: call R_read(s_47_5)
        let s_47_6: u32 = R_read(state, tracer, s_47_5);
        // C s_47_7: const #4u : u32
        let s_47_7: u32 = 4;
        // C s_47_8: const #0u : u32
        let s_47_8: u32 = 0;
        // C s_47_9: const #1u : u32
        let s_47_9: u32 = 1;
        // C s_47_10: const #1u : u32
        let s_47_10: u32 = 1;
        // D s_47_11: call AArch32_TLBI_VA(s_47_2, s_47_7, s_47_4, s_47_8, s_47_9, s_47_10, s_47_6)
        let s_47_11: () = AArch32_TLBI_VA(
            state,
            tracer,
            s_47_2,
            s_47_7,
            s_47_4,
            s_47_8,
            s_47_9,
            s_47_10,
            s_47_6,
        );
        // N s_47_12: return
        return;
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var __HCRX_EL2_FnXS:u8
        let s_48_0: bool = fn_state.u__HCRX_EL2_FnXS;
        // D s_48_1: cast zx s_48_0 -> bv
        let s_48_1: Bits = Bits::new(s_48_0 as u128, 1u16);
        // C s_48_2: const #1u : u8
        let s_48_2: bool = true;
        // C s_48_3: cast zx s_48_2 -> bv
        let s_48_3: Bits = Bits::new(s_48_2 as u128, 1u16);
        // D s_48_4: cmp-eq s_48_1 s_48_3
        let s_48_4: bool = ((s_48_1) == (s_48_3));
        // D s_48_5: write-var gs#134279 <= s_48_4
        fn_state.gs_134279 = s_48_4;
        // N s_48_6: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #() : ()
        let s_49_0: () = ();
        // S s_49_1: call IsHCRXEL2Enabled(s_49_0)
        let s_49_1: bool = IsHCRXEL2Enabled(state, tracer, s_49_0);
        // D s_49_2: write-var gs#134278 <= s_49_1
        fn_state.gs_134278 = s_49_1;
        // N s_49_3: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #155u : u32
        let s_50_0: u32 = 155;
        // S s_50_1: call IsFeatureImplemented(s_50_0)
        let s_50_1: bool = IsFeatureImplemented(state, tracer, s_50_0);
        // D s_50_2: write-var gs#134277 <= s_50_1
        fn_state.gs_134277 = s_50_1;
        // N s_50_3: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #432u : u32
        let s_51_0: u32 = 432;
        // D s_51_1: read-reg s_51_0:u8
        let s_51_1: u8 = {
            let value = state.read_register::<u8>(s_51_0 as isize);
            tracer.read_register(s_51_0 as isize, value);
            value
        };
        // D s_51_2: call ELUsingAArch32(s_51_1)
        let s_51_2: bool = ELUsingAArch32(state, tracer, s_51_1);
        // D s_51_3: not s_51_2
        let s_51_3: bool = !s_51_2;
        // D s_51_4: write-var gs#134276 <= s_51_3
        fn_state.gs_134276 = s_51_3;
        // N s_51_5: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #440u : u32
        let s_52_0: u32 = 440;
        // D s_52_1: read-reg s_52_0:u8
        let s_52_1: u8 = {
            let value = state.read_register::<u8>(s_52_0 as isize);
            tracer.read_register(s_52_0 as isize, value);
            value
        };
        // D s_52_2: call SecurityStateAtEL(s_52_1)
        let s_52_2: u32 = SecurityStateAtEL(state, tracer, s_52_1);
        // C s_52_3: const #() : ()
        let s_52_3: () = ();
        // S s_52_4: call VMID_read(s_52_3)
        let s_52_4: u16 = VMID_read(state, tracer, s_52_3);
        // D s_52_5: read-var t:i
        let s_52_5: i128 = fn_state.t;
        // D s_52_6: call R_read(s_52_5)
        let s_52_6: u32 = R_read(state, tracer, s_52_5);
        // C s_52_7: const #4u : u32
        let s_52_7: u32 = 4;
        // C s_52_8: const #1u : u32
        let s_52_8: u32 = 1;
        // C s_52_9: const #1u : u32
        let s_52_9: u32 = 1;
        // C s_52_10: const #0u : u32
        let s_52_10: u32 = 0;
        // D s_52_11: call AArch32_TLBI_VA(s_52_2, s_52_7, s_52_4, s_52_8, s_52_9, s_52_10, s_52_6)
        let s_52_11: () = AArch32_TLBI_VA(
            state,
            tracer,
            s_52_2,
            s_52_7,
            s_52_4,
            s_52_8,
            s_52_9,
            s_52_10,
            s_52_6,
        );
        // N s_52_12: return
        return;
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var __HCR_FB:u8
        let s_53_0: bool = fn_state.u__HCR_FB;
        // D s_53_1: cast zx s_53_0 -> bv
        let s_53_1: Bits = Bits::new(s_53_0 as u128, 1u16);
        // C s_53_2: const #1u : u8
        let s_53_2: bool = true;
        // C s_53_3: cast zx s_53_2 -> bv
        let s_53_3: Bits = Bits::new(s_53_2 as u128, 1u16);
        // D s_53_4: cmp-eq s_53_1 s_53_3
        let s_53_4: bool = ((s_53_1) == (s_53_3));
        // D s_53_5: write-var gs#134275 <= s_53_4
        fn_state.gs_134275 = s_53_4;
        // N s_53_6: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #432u : u32
        let s_54_0: u32 = 432;
        // D s_54_1: read-reg s_54_0:u8
        let s_54_1: u8 = {
            let value = state.read_register::<u8>(s_54_0 as isize);
            tracer.read_register(s_54_0 as isize, value);
            value
        };
        // D s_54_2: call ELUsingAArch32(s_54_1)
        let s_54_2: bool = ELUsingAArch32(state, tracer, s_54_1);
        // D s_54_3: write-var gs#134274 <= s_54_2
        fn_state.gs_134274 = s_54_2;
        // N s_54_4: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #166u : u32
        let s_55_0: u32 = 166;
        // S s_55_1: call IsFeatureImplemented(s_55_0)
        let s_55_1: bool = IsFeatureImplemented(state, tracer, s_55_0);
        // N s_55_2: branch s_55_1 b66 b56
        if s_55_1 {
            return block_66(state, tracer, fn_state);
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
        // D s_56_1: write-var gs#134280 <= s_56_0
        fn_state.gs_134280 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#134280:u8
        let s_57_0: bool = fn_state.gs_134280;
        // N s_57_1: branch s_57_0 b65 b58
        if s_57_0 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #0u : u8
        let s_58_0: bool = false;
        // D s_58_1: write-var gs#134281 <= s_58_0
        fn_state.gs_134281 = s_58_0;
        // N s_58_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var gs#134281:u8
        let s_59_0: bool = fn_state.gs_134281;
        // N s_59_1: branch s_59_0 b64 b60
        if s_59_0 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #0u : u8
        let s_60_0: bool = false;
        // D s_60_1: write-var gs#134282 <= s_60_0
        fn_state.gs_134282 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#134282:u8
        let s_61_0: bool = fn_state.gs_134282;
        // N s_61_1: branch s_61_0 b63 b62
        if s_61_0 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #440u : u32
        let s_62_0: u32 = 440;
        // D s_62_1: read-reg s_62_0:u8
        let s_62_1: u8 = {
            let value = state.read_register::<u8>(s_62_0 as isize);
            tracer.read_register(s_62_0 as isize, value);
            value
        };
        // D s_62_2: call SecurityStateAtEL(s_62_1)
        let s_62_2: u32 = SecurityStateAtEL(state, tracer, s_62_1);
        // C s_62_3: const #() : ()
        let s_62_3: () = ();
        // S s_62_4: call VMID_read(s_62_3)
        let s_62_4: u16 = VMID_read(state, tracer, s_62_3);
        // D s_62_5: read-var t:i
        let s_62_5: i128 = fn_state.t;
        // D s_62_6: call R_read(s_62_5)
        let s_62_6: u32 = R_read(state, tracer, s_62_5);
        // C s_62_7: const #4u : u32
        let s_62_7: u32 = 4;
        // C s_62_8: const #1u : u32
        let s_62_8: u32 = 1;
        // C s_62_9: const #1u : u32
        let s_62_9: u32 = 1;
        // C s_62_10: const #0u : u32
        let s_62_10: u32 = 0;
        // D s_62_11: call AArch32_TLBI_VA(s_62_2, s_62_7, s_62_4, s_62_8, s_62_9, s_62_10, s_62_6)
        let s_62_11: () = AArch32_TLBI_VA(
            state,
            tracer,
            s_62_2,
            s_62_7,
            s_62_4,
            s_62_8,
            s_62_9,
            s_62_10,
            s_62_6,
        );
        // N s_62_12: return
        return;
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #440u : u32
        let s_63_0: u32 = 440;
        // D s_63_1: read-reg s_63_0:u8
        let s_63_1: u8 = {
            let value = state.read_register::<u8>(s_63_0 as isize);
            tracer.read_register(s_63_0 as isize, value);
            value
        };
        // D s_63_2: call SecurityStateAtEL(s_63_1)
        let s_63_2: u32 = SecurityStateAtEL(state, tracer, s_63_1);
        // C s_63_3: const #() : ()
        let s_63_3: () = ();
        // S s_63_4: call VMID_read(s_63_3)
        let s_63_4: u16 = VMID_read(state, tracer, s_63_3);
        // D s_63_5: read-var t:i
        let s_63_5: i128 = fn_state.t;
        // D s_63_6: call R_read(s_63_5)
        let s_63_6: u32 = R_read(state, tracer, s_63_5);
        // C s_63_7: const #4u : u32
        let s_63_7: u32 = 4;
        // C s_63_8: const #1u : u32
        let s_63_8: u32 = 1;
        // C s_63_9: const #1u : u32
        let s_63_9: u32 = 1;
        // C s_63_10: const #1u : u32
        let s_63_10: u32 = 1;
        // D s_63_11: call AArch32_TLBI_VA(s_63_2, s_63_7, s_63_4, s_63_8, s_63_9, s_63_10, s_63_6)
        let s_63_11: () = AArch32_TLBI_VA(
            state,
            tracer,
            s_63_2,
            s_63_7,
            s_63_4,
            s_63_8,
            s_63_9,
            s_63_10,
            s_63_6,
        );
        // N s_63_12: return
        return;
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var __HCRX_EL2_FnXS:u8
        let s_64_0: bool = fn_state.u__HCRX_EL2_FnXS;
        // D s_64_1: cast zx s_64_0 -> bv
        let s_64_1: Bits = Bits::new(s_64_0 as u128, 1u16);
        // C s_64_2: const #1u : u8
        let s_64_2: bool = true;
        // C s_64_3: cast zx s_64_2 -> bv
        let s_64_3: Bits = Bits::new(s_64_2 as u128, 1u16);
        // D s_64_4: cmp-eq s_64_1 s_64_3
        let s_64_4: bool = ((s_64_1) == (s_64_3));
        // D s_64_5: write-var gs#134282 <= s_64_4
        fn_state.gs_134282 = s_64_4;
        // N s_64_6: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #() : ()
        let s_65_0: () = ();
        // S s_65_1: call IsHCRXEL2Enabled(s_65_0)
        let s_65_1: bool = IsHCRXEL2Enabled(state, tracer, s_65_0);
        // D s_65_2: write-var gs#134281 <= s_65_1
        fn_state.gs_134281 = s_65_1;
        // N s_65_3: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #155u : u32
        let s_66_0: u32 = 155;
        // S s_66_1: call IsFeatureImplemented(s_66_0)
        let s_66_1: bool = IsFeatureImplemented(state, tracer, s_66_0);
        // D s_66_2: write-var gs#134280 <= s_66_1
        fn_state.gs_134280 = s_66_1;
        // N s_66_3: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var __HCR_EL2_FB:u8
        let s_67_0: bool = fn_state.u__HCR_EL2_FB;
        // D s_67_1: cast zx s_67_0 -> bv
        let s_67_1: Bits = Bits::new(s_67_0 as u128, 1u16);
        // C s_67_2: const #1u : u8
        let s_67_2: bool = true;
        // C s_67_3: cast zx s_67_2 -> bv
        let s_67_3: Bits = Bits::new(s_67_2 as u128, 1u16);
        // D s_67_4: cmp-eq s_67_1 s_67_3
        let s_67_4: bool = ((s_67_1) == (s_67_3));
        // D s_67_5: write-var gs#134273 <= s_67_4
        fn_state.gs_134273 = s_67_4;
        // N s_67_6: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #432u : u32
        let s_68_0: u32 = 432;
        // D s_68_1: read-reg s_68_0:u8
        let s_68_1: u8 = {
            let value = state.read_register::<u8>(s_68_0 as isize);
            tracer.read_register(s_68_0 as isize, value);
            value
        };
        // D s_68_2: call ELUsingAArch32(s_68_1)
        let s_68_2: bool = ELUsingAArch32(state, tracer, s_68_1);
        // D s_68_3: not s_68_2
        let s_68_3: bool = !s_68_2;
        // D s_68_4: write-var gs#134272 <= s_68_3
        fn_state.gs_134272 = s_68_3;
        // N s_68_5: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #3u : u8
        let s_69_0: u8 = 3;
        // C s_69_1: cast zx s_69_0 -> bv
        let s_69_1: Bits = Bits::new(s_69_0 as u128, 8u16);
        // C s_69_2: cast zx s_69_1 -> i
        let s_69_2: i128 = (s_69_1.value() as i128);
        // C s_69_3: cast reint s_69_2 -> i64
        let s_69_3: i64 = (s_69_2 as i64);
        // C s_69_4: cast zx s_69_3 -> i
        let s_69_4: i128 = (i128::try_from(s_69_3).unwrap());
        // S s_69_5: call AArch32_TakeHypTrapException(s_69_4)
        let s_69_5: () = AArch32_TakeHypTrapException(state, tracer, s_69_4);
        // N s_69_6: return
        return;
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var __HCR_TTLB:u8
        let s_70_0: bool = fn_state.u__HCR_TTLB;
        // D s_70_1: cast zx s_70_0 -> bv
        let s_70_1: Bits = Bits::new(s_70_0 as u128, 1u16);
        // C s_70_2: const #1u : u8
        let s_70_2: bool = true;
        // C s_70_3: cast zx s_70_2 -> bv
        let s_70_3: Bits = Bits::new(s_70_2 as u128, 1u16);
        // D s_70_4: cmp-eq s_70_1 s_70_3
        let s_70_4: bool = ((s_70_1) == (s_70_3));
        // D s_70_5: write-var gs#134271 <= s_70_4
        fn_state.gs_134271 = s_70_4;
        // N s_70_6: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #432u : u32
        let s_71_0: u32 = 432;
        // D s_71_1: read-reg s_71_0:u8
        let s_71_1: u8 = {
            let value = state.read_register::<u8>(s_71_0 as isize);
            tracer.read_register(s_71_0 as isize, value);
            value
        };
        // D s_71_2: call ELUsingAArch32(s_71_1)
        let s_71_2: bool = ELUsingAArch32(state, tracer, s_71_1);
        // D s_71_3: write-var gs#134270 <= s_71_2
        fn_state.gs_134270 = s_71_2;
        // N s_71_4: jump b24
        return block_24(state, tracer, fn_state);
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
        // C s_72_5: const #432u : u32
        let s_72_5: u32 = 432;
        // D s_72_6: read-reg s_72_5:u8
        let s_72_6: u8 = {
            let value = state.read_register::<u8>(s_72_5 as isize);
            tracer.read_register(s_72_5 as isize, value);
            value
        };
        // D s_72_7: call AArch64_AArch32SystemAccessTrap(s_72_6, s_72_4)
        let s_72_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_72_6, s_72_4);
        // N s_72_8: return
        return;
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var __HCR_EL2_TTLB:u8
        let s_73_0: bool = fn_state.u__HCR_EL2_TTLB;
        // D s_73_1: cast zx s_73_0 -> bv
        let s_73_1: Bits = Bits::new(s_73_0 as u128, 1u16);
        // C s_73_2: const #1u : u8
        let s_73_2: bool = true;
        // C s_73_3: cast zx s_73_2 -> bv
        let s_73_3: Bits = Bits::new(s_73_2 as u128, 1u16);
        // D s_73_4: cmp-eq s_73_1 s_73_3
        let s_73_4: bool = ((s_73_1) == (s_73_3));
        // D s_73_5: write-var gs#134269 <= s_73_4
        fn_state.gs_134269 = s_73_4;
        // N s_73_6: jump b21
        return block_21(state, tracer, fn_state);
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
        // D s_74_3: not s_74_2
        let s_74_3: bool = !s_74_2;
        // D s_74_4: write-var gs#134268 <= s_74_3
        fn_state.gs_134268 = s_74_3;
        // N s_74_5: jump b19
        return block_19(state, tracer, fn_state);
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
        // S s_75_5: call AArch32_TakeHypTrapException(s_75_4)
        let s_75_5: () = AArch32_TakeHypTrapException(state, tracer, s_75_4);
        // N s_75_6: return
        return;
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var __HSTR_T8:u8
        let s_76_0: bool = fn_state.u__HSTR_T8;
        // D s_76_1: cast zx s_76_0 -> bv
        let s_76_1: Bits = Bits::new(s_76_0 as u128, 1u16);
        // C s_76_2: const #1u : u8
        let s_76_2: bool = true;
        // C s_76_3: cast zx s_76_2 -> bv
        let s_76_3: Bits = Bits::new(s_76_2 as u128, 1u16);
        // D s_76_4: cmp-eq s_76_1 s_76_3
        let s_76_4: bool = ((s_76_1) == (s_76_3));
        // D s_76_5: write-var gs#134267 <= s_76_4
        fn_state.gs_134267 = s_76_4;
        // N s_76_6: jump b16
        return block_16(state, tracer, fn_state);
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
        // D s_77_3: write-var gs#134266 <= s_77_2
        fn_state.gs_134266 = s_77_2;
        // N s_77_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #3u : u8
        let s_78_0: u8 = 3;
        // C s_78_1: cast zx s_78_0 -> bv
        let s_78_1: Bits = Bits::new(s_78_0 as u128, 8u16);
        // C s_78_2: cast zx s_78_1 -> i
        let s_78_2: i128 = (s_78_1.value() as i128);
        // C s_78_3: cast reint s_78_2 -> i64
        let s_78_3: i64 = (s_78_2 as i64);
        // C s_78_4: cast zx s_78_3 -> i
        let s_78_4: i128 = (i128::try_from(s_78_3).unwrap());
        // C s_78_5: const #432u : u32
        let s_78_5: u32 = 432;
        // D s_78_6: read-reg s_78_5:u8
        let s_78_6: u8 = {
            let value = state.read_register::<u8>(s_78_5 as isize);
            tracer.read_register(s_78_5 as isize, value);
            value
        };
        // D s_78_7: call AArch64_AArch32SystemAccessTrap(s_78_6, s_78_4)
        let s_78_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_78_6, s_78_4);
        // N s_78_8: return
        return;
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var __HSTR_EL2_T8:u8
        let s_79_0: bool = fn_state.u__HSTR_EL2_T8;
        // D s_79_1: cast zx s_79_0 -> bv
        let s_79_1: Bits = Bits::new(s_79_0 as u128, 1u16);
        // C s_79_2: const #1u : u8
        let s_79_2: bool = true;
        // C s_79_3: cast zx s_79_2 -> bv
        let s_79_3: Bits = Bits::new(s_79_2 as u128, 1u16);
        // D s_79_4: cmp-eq s_79_1 s_79_3
        let s_79_4: bool = ((s_79_1) == (s_79_3));
        // D s_79_5: write-var gs#134265 <= s_79_4
        fn_state.gs_134265 = s_79_4;
        // N s_79_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #432u : u32
        let s_80_0: u32 = 432;
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
        // D s_80_4: write-var gs#134264 <= s_80_3
        fn_state.gs_134264 = s_80_3;
        // N s_80_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_81_0: panic
        panic!("{:?}", ());
        // N s_81_1: return
        return;
    }
}
