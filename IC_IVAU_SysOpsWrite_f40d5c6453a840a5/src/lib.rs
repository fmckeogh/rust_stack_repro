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
use AArch64_SystemAccessTrap::*;
use u_get_HCR_EL2_Type_TPU::*;
use u_get_SCR_EL3_Type_FGTEn::*;
use u_get_HCR_EL2_Type_E2H::*;
use u_get_HFGITR_EL2_Type_ICIVAU::*;
use u_get_SCTLR_EL2_Type_UCI::*;
use IsFeatureImplemented::*;
use X_read::*;
use AArch64_IC__1::*;
use u_get_HCR_EL2_Type_TGE::*;
use u_get_SCTLR_EL1_Type_UCI::*;
use u_get_HCR_EL2_Type_TOCU::*;
use EL2Enabled::*;
use common::*;
pub fn IC_IVAU_SysOpsWrite_f40d5c6453a840a5<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    op0: u8,
    op1: u8,
    CRn: u8,
    op2: u8,
    CRm: u8,
    t: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_101960: bool,
        u__HCR_EL2_TGE: bool,
        gs_101955: bool,
        gs_101957: bool,
        gs_101956: bool,
        gs_101954: bool,
        u__SCR_EL3_FGTEn: bool,
        gs_101948: bool,
        gs_101965: bool,
        u__HCR_EL2_TPU: bool,
        gs_101961: bool,
        gs_101966: bool,
        gs_101959: bool,
        gs_101962: bool,
        u__SCTLR_EL2_UCI: bool,
        gs_101947: bool,
        gs_101952: bool,
        gs_101964: bool,
        gs_101968: bool,
        u__HCR_EL2_TOCU: bool,
        u__PSTATE_EL: u8,
        gs_101963: bool,
        gs_101949: bool,
        gs_101950: bool,
        gs_101951: bool,
        u__SCTLR_EL1_UCI: bool,
        gs_101958: bool,
        u__HFGITR_EL2_ICIVAU: bool,
        el: u8,
        op0: u8,
        op1: u8,
        CRn: u8,
        op2: u8,
        CRm: u8,
        t: i128,
    }
    let fn_state = FunctionState {
        el,
        op0,
        op1,
        CRn,
        op2,
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
        // C s_0_3: const #90272u : u32
        let s_0_3: u32 = 90272;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_SCTLR_EL1_Type_UCI(s_0_4)
        let s_0_5: bool = u_get_SCTLR_EL1_Type_UCI(state, tracer, s_0_4);
        // D s_0_6: write-var __SCTLR_EL1_UCI <= s_0_5
        fn_state.u__SCTLR_EL1_UCI = s_0_5;
        // C s_0_7: const #102552u : u32
        let s_0_7: u32 = 102552;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_HCR_EL2_Type_TGE(s_0_8)
        let s_0_9: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_0_8);
        // D s_0_10: write-var __HCR_EL2_TGE <= s_0_9
        fn_state.u__HCR_EL2_TGE = s_0_9;
        // C s_0_11: const #102552u : u32
        let s_0_11: u32 = 102552;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_HCR_EL2_Type_TPU(s_0_12)
        let s_0_13: bool = u_get_HCR_EL2_Type_TPU(state, tracer, s_0_12);
        // D s_0_14: write-var __HCR_EL2_TPU <= s_0_13
        fn_state.u__HCR_EL2_TPU = s_0_13;
        // C s_0_15: const #102552u : u32
        let s_0_15: u32 = 102552;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_HCR_EL2_Type_TOCU(s_0_16)
        let s_0_17: bool = u_get_HCR_EL2_Type_TOCU(state, tracer, s_0_16);
        // D s_0_18: write-var __HCR_EL2_TOCU <= s_0_17
        fn_state.u__HCR_EL2_TOCU = s_0_17;
        // C s_0_19: const #90704u : u32
        let s_0_19: u32 = 90704;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_SCR_EL3_Type_FGTEn(s_0_20)
        let s_0_21: bool = u_get_SCR_EL3_Type_FGTEn(state, tracer, s_0_20);
        // D s_0_22: write-var __SCR_EL3_FGTEn <= s_0_21
        fn_state.u__SCR_EL3_FGTEn = s_0_21;
        // C s_0_23: const #13608u : u32
        let s_0_23: u32 = 13608;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_HFGITR_EL2_Type_ICIVAU(s_0_24)
        let s_0_25: bool = u_get_HFGITR_EL2_Type_ICIVAU(state, tracer, s_0_24);
        // D s_0_26: write-var __HFGITR_EL2_ICIVAU <= s_0_25
        fn_state.u__HFGITR_EL2_ICIVAU = s_0_25;
        // C s_0_27: const #20784u : u32
        let s_0_27: u32 = 20784;
        // D s_0_28: read-reg s_0_27:struct
        let s_0_28: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_27 as isize);
            tracer.read_register(s_0_27 as isize, value);
            value
        };
        // D s_0_29: call _get_SCTLR_EL2_Type_UCI(s_0_28)
        let s_0_29: bool = u_get_SCTLR_EL2_Type_UCI(state, tracer, s_0_28);
        // D s_0_30: write-var __SCTLR_EL2_UCI <= s_0_29
        fn_state.u__SCTLR_EL2_UCI = s_0_29;
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
        // N s_0_37: branch s_0_36 b32 b1
        if s_0_36 {
            return block_32(state, tracer, fn_state);
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
        // C s_5_0: const #64s : i64
        let s_5_0: i64 = 64;
        // D s_5_1: read-var t:i
        let s_5_1: i128 = fn_state.t;
        // D s_5_2: call X_read(s_5_1, s_5_0)
        let s_5_2: Bits = X_read(state, tracer, s_5_1, s_5_0);
        // D s_5_3: cast reint s_5_2 -> u64
        let s_5_3: u64 = (s_5_2.value() as u64);
        // C s_5_4: const #1u : u32
        let s_5_4: u32 = 1;
        // D s_5_5: call AArch64_IC__1(s_5_3, s_5_4)
        let s_5_5: () = AArch64_IC__1(state, tracer, s_5_3, s_5_4);
        // N s_5_6: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #64s : i64
        let s_6_0: i64 = 64;
        // D s_6_1: read-var t:i
        let s_6_1: i128 = fn_state.t;
        // D s_6_2: call X_read(s_6_1, s_6_0)
        let s_6_2: Bits = X_read(state, tracer, s_6_1, s_6_0);
        // D s_6_3: cast reint s_6_2 -> u64
        let s_6_3: u64 = (s_6_2.value() as u64);
        // C s_6_4: const #1u : u32
        let s_6_4: u32 = 1;
        // D s_6_5: call AArch64_IC__1(s_6_3, s_6_4)
        let s_6_5: () = AArch64_IC__1(state, tracer, s_6_3, s_6_4);
        // N s_6_6: return
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
        // N s_7_2: branch s_7_1 b31 b8
        if s_7_1 {
            return block_31(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#101947 <= s_8_0
        fn_state.gs_101947 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#101947:u8
        let s_9_0: bool = fn_state.gs_101947;
        // N s_9_1: branch s_9_0 b30 b10
        if s_9_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call EL2Enabled(s_10_0)
        let s_10_1: bool = EL2Enabled(state, tracer, s_10_0);
        // N s_10_2: branch s_10_1 b29 b11
        if s_10_1 {
            return block_29(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#101948 <= s_11_0
        fn_state.gs_101948 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#101948:u8
        let s_12_0: bool = fn_state.gs_101948;
        // N s_12_1: branch s_12_0 b28 b13
        if s_12_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call EL2Enabled(s_13_0)
        let s_13_1: bool = EL2Enabled(state, tracer, s_13_0);
        // N s_13_2: branch s_13_1 b27 b14
        if s_13_1 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#101949 <= s_14_0
        fn_state.gs_101949 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#101949:u8
        let s_15_0: bool = fn_state.gs_101949;
        // N s_15_1: branch s_15_0 b23 b16
        if s_15_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#101951 <= s_16_0
        fn_state.gs_101951 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#101951:u8
        let s_17_0: bool = fn_state.gs_101951;
        // N s_17_1: branch s_17_0 b22 b18
        if s_17_0 {
            return block_22(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#101952 <= s_18_0
        fn_state.gs_101952 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#101952:u8
        let s_19_0: bool = fn_state.gs_101952;
        // N s_19_1: branch s_19_0 b21 b20
        if s_19_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #64s : i64
        let s_20_0: i64 = 64;
        // D s_20_1: read-var t:i
        let s_20_1: i128 = fn_state.t;
        // D s_20_2: call X_read(s_20_1, s_20_0)
        let s_20_2: Bits = X_read(state, tracer, s_20_1, s_20_0);
        // D s_20_3: cast reint s_20_2 -> u64
        let s_20_3: u64 = (s_20_2.value() as u64);
        // C s_20_4: const #1u : u32
        let s_20_4: u32 = 1;
        // D s_20_5: call AArch64_IC__1(s_20_3, s_20_4)
        let s_20_5: () = AArch64_IC__1(state, tracer, s_20_3, s_20_4);
        // N s_20_6: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #24u : u8
        let s_21_0: u8 = 24;
        // C s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 8u16);
        // C s_21_2: cast zx s_21_1 -> i
        let s_21_2: i128 = (s_21_1.value() as i128);
        // C s_21_3: cast reint s_21_2 -> i64
        let s_21_3: i64 = (s_21_2 as i64);
        // C s_21_4: cast zx s_21_3 -> i
        let s_21_4: i128 = (i128::try_from(s_21_3).unwrap());
        // C s_21_5: const #432u : u32
        let s_21_5: u32 = 432;
        // D s_21_6: read-reg s_21_5:u8
        let s_21_6: u8 = {
            let value = state.read_register::<u8>(s_21_5 as isize);
            tracer.read_register(s_21_5 as isize, value);
            value
        };
        // D s_21_7: call AArch64_SystemAccessTrap(s_21_6, s_21_4)
        let s_21_7: () = AArch64_SystemAccessTrap(state, tracer, s_21_6, s_21_4);
        // N s_21_8: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var __HFGITR_EL2_ICIVAU:u8
        let s_22_0: bool = fn_state.u__HFGITR_EL2_ICIVAU;
        // D s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 1u16);
        // C s_22_2: const #1u : u8
        let s_22_2: bool = true;
        // C s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 1u16);
        // D s_22_4: cmp-eq s_22_1 s_22_3
        let s_22_4: bool = ((s_22_1) == (s_22_3));
        // D s_22_5: write-var gs#101952 <= s_22_4
        fn_state.gs_101952 = s_22_4;
        // N s_22_6: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #424u : u32
        let s_23_0: u32 = 424;
        // D s_23_1: read-reg s_23_0:u8
        let s_23_1: u8 = {
            let value = state.read_register::<u8>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // C s_23_2: const #2u : u8
        let s_23_2: u8 = 2;
        // D s_23_3: cmp-lt s_23_1 s_23_2
        let s_23_3: bool = ((s_23_1) < (s_23_2));
        // D s_23_4: not s_23_3
        let s_23_4: bool = !s_23_3;
        // N s_23_5: branch s_23_4 b26 b24
        if s_23_4 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var __SCR_EL3_FGTEn:u8
        let s_24_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 1u16);
        // C s_24_2: const #1u : u8
        let s_24_2: bool = true;
        // C s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 1u16);
        // D s_24_4: cmp-eq s_24_1 s_24_3
        let s_24_4: bool = ((s_24_1) == (s_24_3));
        // D s_24_5: write-var gs#101950 <= s_24_4
        fn_state.gs_101950 = s_24_4;
        // N s_24_6: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#101950:u8
        let s_25_0: bool = fn_state.gs_101950;
        // D s_25_1: write-var gs#101951 <= s_25_0
        fn_state.gs_101951 = s_25_0;
        // N s_25_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // D s_26_1: write-var gs#101950 <= s_26_0
        fn_state.gs_101950 = s_26_0;
        // N s_26_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #146u : u32
        let s_27_0: u32 = 146;
        // S s_27_1: call IsFeatureImplemented(s_27_0)
        let s_27_1: bool = IsFeatureImplemented(state, tracer, s_27_0);
        // D s_27_2: write-var gs#101949 <= s_27_1
        fn_state.gs_101949 = s_27_1;
        // N s_27_3: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #24u : u8
        let s_28_0: u8 = 24;
        // C s_28_1: cast zx s_28_0 -> bv
        let s_28_1: Bits = Bits::new(s_28_0 as u128, 8u16);
        // C s_28_2: cast zx s_28_1 -> i
        let s_28_2: i128 = (s_28_1.value() as i128);
        // C s_28_3: cast reint s_28_2 -> i64
        let s_28_3: i64 = (s_28_2 as i64);
        // C s_28_4: cast zx s_28_3 -> i
        let s_28_4: i128 = (i128::try_from(s_28_3).unwrap());
        // C s_28_5: const #432u : u32
        let s_28_5: u32 = 432;
        // D s_28_6: read-reg s_28_5:u8
        let s_28_6: u8 = {
            let value = state.read_register::<u8>(s_28_5 as isize);
            tracer.read_register(s_28_5 as isize, value);
            value
        };
        // D s_28_7: call AArch64_SystemAccessTrap(s_28_6, s_28_4)
        let s_28_7: () = AArch64_SystemAccessTrap(state, tracer, s_28_6, s_28_4);
        // N s_28_8: return
        return;
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var __HCR_EL2_TOCU:u8
        let s_29_0: bool = fn_state.u__HCR_EL2_TOCU;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 1u16);
        // C s_29_2: const #1u : u8
        let s_29_2: bool = true;
        // C s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // D s_29_4: cmp-eq s_29_1 s_29_3
        let s_29_4: bool = ((s_29_1) == (s_29_3));
        // D s_29_5: write-var gs#101948 <= s_29_4
        fn_state.gs_101948 = s_29_4;
        // N s_29_6: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #24u : u8
        let s_30_0: u8 = 24;
        // C s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 8u16);
        // C s_30_2: cast zx s_30_1 -> i
        let s_30_2: i128 = (s_30_1.value() as i128);
        // C s_30_3: cast reint s_30_2 -> i64
        let s_30_3: i64 = (s_30_2 as i64);
        // C s_30_4: cast zx s_30_3 -> i
        let s_30_4: i128 = (i128::try_from(s_30_3).unwrap());
        // C s_30_5: const #432u : u32
        let s_30_5: u32 = 432;
        // D s_30_6: read-reg s_30_5:u8
        let s_30_6: u8 = {
            let value = state.read_register::<u8>(s_30_5 as isize);
            tracer.read_register(s_30_5 as isize, value);
            value
        };
        // D s_30_7: call AArch64_SystemAccessTrap(s_30_6, s_30_4)
        let s_30_7: () = AArch64_SystemAccessTrap(state, tracer, s_30_6, s_30_4);
        // N s_30_8: return
        return;
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var __HCR_EL2_TPU:u8
        let s_31_0: bool = fn_state.u__HCR_EL2_TPU;
        // D s_31_1: cast zx s_31_0 -> bv
        let s_31_1: Bits = Bits::new(s_31_0 as u128, 1u16);
        // C s_31_2: const #1u : u8
        let s_31_2: bool = true;
        // C s_31_3: cast zx s_31_2 -> bv
        let s_31_3: Bits = Bits::new(s_31_2 as u128, 1u16);
        // D s_31_4: cmp-eq s_31_1 s_31_3
        let s_31_4: bool = ((s_31_1) == (s_31_3));
        // D s_31_5: write-var gs#101947 <= s_31_4
        fn_state.gs_101947 = s_31_4;
        // N s_31_6: jump b9
        return block_9(state, tracer, fn_state);
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
        // N s_32_2: branch s_32_1 b86 b33
        if s_32_1 {
            return block_86(state, tracer, fn_state);
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
        // D s_33_1: write-var gs#101954 <= s_33_0
        fn_state.gs_101954 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#101954:u8
        let s_34_0: bool = fn_state.gs_101954;
        // D s_34_1: not s_34_0
        let s_34_1: bool = !s_34_0;
        // N s_34_2: branch s_34_1 b85 b35
        if s_34_1 {
            return block_85(state, tracer, fn_state);
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
        // D s_35_1: write-var gs#101955 <= s_35_0
        fn_state.gs_101955 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#101955:u8
        let s_36_0: bool = fn_state.gs_101955;
        // N s_36_1: branch s_36_0 b79 b37
        if s_36_0 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #() : ()
        let s_37_0: () = ();
        // S s_37_1: call EL2Enabled(s_37_0)
        let s_37_1: bool = EL2Enabled(state, tracer, s_37_0);
        // N s_37_2: branch s_37_1 b78 b38
        if s_37_1 {
            return block_78(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#101956 <= s_38_0
        fn_state.gs_101956 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#101956:u8
        let s_39_0: bool = fn_state.gs_101956;
        // N s_39_1: branch s_39_0 b77 b40
        if s_39_0 {
            return block_77(state, tracer, fn_state);
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
        // D s_40_1: write-var gs#101957 <= s_40_0
        fn_state.gs_101957 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#101957:u8
        let s_41_0: bool = fn_state.gs_101957;
        // N s_41_1: branch s_41_0 b76 b42
        if s_41_0 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #() : ()
        let s_42_0: () = ();
        // S s_42_1: call EL2Enabled(s_42_0)
        let s_42_1: bool = EL2Enabled(state, tracer, s_42_0);
        // N s_42_2: branch s_42_1 b75 b43
        if s_42_1 {
            return block_75(state, tracer, fn_state);
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
        // D s_43_1: write-var gs#101958 <= s_43_0
        fn_state.gs_101958 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#101958:u8
        let s_44_0: bool = fn_state.gs_101958;
        // N s_44_1: branch s_44_0 b74 b45
        if s_44_0 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #0u : u8
        let s_45_0: bool = false;
        // D s_45_1: write-var gs#101959 <= s_45_0
        fn_state.gs_101959 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#101959:u8
        let s_46_0: bool = fn_state.gs_101959;
        // N s_46_1: branch s_46_0 b73 b47
        if s_46_0 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #() : ()
        let s_47_0: () = ();
        // S s_47_1: call EL2Enabled(s_47_0)
        let s_47_1: bool = EL2Enabled(state, tracer, s_47_0);
        // N s_47_2: branch s_47_1 b72 b48
        if s_47_1 {
            return block_72(state, tracer, fn_state);
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
        // D s_48_1: write-var gs#101960 <= s_48_0
        fn_state.gs_101960 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#101960:u8
        let s_49_0: bool = fn_state.gs_101960;
        // N s_49_1: branch s_49_0 b71 b50
        if s_49_0 {
            return block_71(state, tracer, fn_state);
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
        // D s_50_1: write-var gs#101961 <= s_50_0
        fn_state.gs_101961 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#101961:u8
        let s_51_0: bool = fn_state.gs_101961;
        // N s_51_1: branch s_51_0 b67 b52
        if s_51_0 {
            return block_67(state, tracer, fn_state);
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
        // D s_52_1: write-var gs#101963 <= s_52_0
        fn_state.gs_101963 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#101963:u8
        let s_53_0: bool = fn_state.gs_101963;
        // N s_53_1: branch s_53_0 b66 b54
        if s_53_0 {
            return block_66(state, tracer, fn_state);
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
        // D s_54_1: write-var gs#101964 <= s_54_0
        fn_state.gs_101964 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#101964:u8
        let s_55_0: bool = fn_state.gs_101964;
        // N s_55_1: branch s_55_0 b65 b56
        if s_55_0 {
            return block_65(state, tracer, fn_state);
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
        // N s_56_2: branch s_56_1 b64 b57
        if s_56_1 {
            return block_64(state, tracer, fn_state);
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
        // D s_57_1: write-var gs#101965 <= s_57_0
        fn_state.gs_101965 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var gs#101965:u8
        let s_58_0: bool = fn_state.gs_101965;
        // N s_58_1: branch s_58_0 b63 b59
        if s_58_0 {
            return block_63(state, tracer, fn_state);
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
        // D s_59_1: write-var gs#101966 <= s_59_0
        fn_state.gs_101966 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#101966:u8
        let s_60_0: bool = fn_state.gs_101966;
        // N s_60_1: branch s_60_0 b62 b61
        if s_60_0 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #64s : i64
        let s_61_0: i64 = 64;
        // D s_61_1: read-var t:i
        let s_61_1: i128 = fn_state.t;
        // D s_61_2: call X_read(s_61_1, s_61_0)
        let s_61_2: Bits = X_read(state, tracer, s_61_1, s_61_0);
        // D s_61_3: cast reint s_61_2 -> u64
        let s_61_3: u64 = (s_61_2.value() as u64);
        // C s_61_4: const #1u : u32
        let s_61_4: u32 = 1;
        // D s_61_5: call AArch64_IC__1(s_61_3, s_61_4)
        let s_61_5: () = AArch64_IC__1(state, tracer, s_61_3, s_61_4);
        // N s_61_6: return
        return;
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #24u : u8
        let s_62_0: u8 = 24;
        // C s_62_1: cast zx s_62_0 -> bv
        let s_62_1: Bits = Bits::new(s_62_0 as u128, 8u16);
        // C s_62_2: cast zx s_62_1 -> i
        let s_62_2: i128 = (s_62_1.value() as i128);
        // C s_62_3: cast reint s_62_2 -> i64
        let s_62_3: i64 = (s_62_2 as i64);
        // C s_62_4: cast zx s_62_3 -> i
        let s_62_4: i128 = (i128::try_from(s_62_3).unwrap());
        // C s_62_5: const #432u : u32
        let s_62_5: u32 = 432;
        // D s_62_6: read-reg s_62_5:u8
        let s_62_6: u8 = {
            let value = state.read_register::<u8>(s_62_5 as isize);
            tracer.read_register(s_62_5 as isize, value);
            value
        };
        // D s_62_7: call AArch64_SystemAccessTrap(s_62_6, s_62_4)
        let s_62_7: () = AArch64_SystemAccessTrap(state, tracer, s_62_6, s_62_4);
        // N s_62_8: return
        return;
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var __SCTLR_EL2_UCI:u8
        let s_63_0: bool = fn_state.u__SCTLR_EL2_UCI;
        // D s_63_1: cast zx s_63_0 -> bv
        let s_63_1: Bits = Bits::new(s_63_0 as u128, 1u16);
        // C s_63_2: const #0u : u8
        let s_63_2: bool = false;
        // C s_63_3: cast zx s_63_2 -> bv
        let s_63_3: Bits = Bits::new(s_63_2 as u128, 1u16);
        // D s_63_4: cmp-eq s_63_1 s_63_3
        let s_63_4: bool = ((s_63_1) == (s_63_3));
        // D s_63_5: write-var gs#101966 <= s_63_4
        fn_state.gs_101966 = s_63_4;
        // N s_63_6: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #102552u : u32
        let s_64_0: u32 = 102552;
        // D s_64_1: read-reg s_64_0:struct
        let s_64_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_64_0 as isize);
            tracer.read_register(s_64_0 as isize, value);
            value
        };
        // D s_64_2: call _get_HCR_EL2_Type_E2H(s_64_1)
        let s_64_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_64_1);
        // C s_64_3: const #102552u : u32
        let s_64_3: u32 = 102552;
        // D s_64_4: read-reg s_64_3:struct
        let s_64_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_64_3 as isize);
            tracer.read_register(s_64_3 as isize, value);
            value
        };
        // D s_64_5: call _get_HCR_EL2_Type_TGE(s_64_4)
        let s_64_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_64_4);
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
        // D s_64_21: write-var gs#101965 <= s_64_20
        fn_state.gs_101965 = s_64_20;
        // N s_64_22: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #24u : u8
        let s_65_0: u8 = 24;
        // C s_65_1: cast zx s_65_0 -> bv
        let s_65_1: Bits = Bits::new(s_65_0 as u128, 8u16);
        // C s_65_2: cast zx s_65_1 -> i
        let s_65_2: i128 = (s_65_1.value() as i128);
        // C s_65_3: cast reint s_65_2 -> i64
        let s_65_3: i64 = (s_65_2 as i64);
        // C s_65_4: cast zx s_65_3 -> i
        let s_65_4: i128 = (i128::try_from(s_65_3).unwrap());
        // C s_65_5: const #432u : u32
        let s_65_5: u32 = 432;
        // D s_65_6: read-reg s_65_5:u8
        let s_65_6: u8 = {
            let value = state.read_register::<u8>(s_65_5 as isize);
            tracer.read_register(s_65_5 as isize, value);
            value
        };
        // D s_65_7: call AArch64_SystemAccessTrap(s_65_6, s_65_4)
        let s_65_7: () = AArch64_SystemAccessTrap(state, tracer, s_65_6, s_65_4);
        // N s_65_8: return
        return;
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var __HFGITR_EL2_ICIVAU:u8
        let s_66_0: bool = fn_state.u__HFGITR_EL2_ICIVAU;
        // D s_66_1: cast zx s_66_0 -> bv
        let s_66_1: Bits = Bits::new(s_66_0 as u128, 1u16);
        // C s_66_2: const #1u : u8
        let s_66_2: bool = true;
        // C s_66_3: cast zx s_66_2 -> bv
        let s_66_3: Bits = Bits::new(s_66_2 as u128, 1u16);
        // D s_66_4: cmp-eq s_66_1 s_66_3
        let s_66_4: bool = ((s_66_1) == (s_66_3));
        // D s_66_5: write-var gs#101964 <= s_66_4
        fn_state.gs_101964 = s_66_4;
        // N s_66_6: jump b55
        return block_55(state, tracer, fn_state);
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
        // C s_67_2: const #2u : u8
        let s_67_2: u8 = 2;
        // D s_67_3: cmp-lt s_67_1 s_67_2
        let s_67_3: bool = ((s_67_1) < (s_67_2));
        // D s_67_4: not s_67_3
        let s_67_4: bool = !s_67_3;
        // N s_67_5: branch s_67_4 b70 b68
        if s_67_4 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var __SCR_EL3_FGTEn:u8
        let s_68_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_68_1: cast zx s_68_0 -> bv
        let s_68_1: Bits = Bits::new(s_68_0 as u128, 1u16);
        // C s_68_2: const #1u : u8
        let s_68_2: bool = true;
        // C s_68_3: cast zx s_68_2 -> bv
        let s_68_3: Bits = Bits::new(s_68_2 as u128, 1u16);
        // D s_68_4: cmp-eq s_68_1 s_68_3
        let s_68_4: bool = ((s_68_1) == (s_68_3));
        // D s_68_5: write-var gs#101962 <= s_68_4
        fn_state.gs_101962 = s_68_4;
        // N s_68_6: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var gs#101962:u8
        let s_69_0: bool = fn_state.gs_101962;
        // D s_69_1: write-var gs#101963 <= s_69_0
        fn_state.gs_101963 = s_69_0;
        // N s_69_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #1u : u8
        let s_70_0: bool = true;
        // D s_70_1: write-var gs#101962 <= s_70_0
        fn_state.gs_101962 = s_70_0;
        // N s_70_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #146u : u32
        let s_71_0: u32 = 146;
        // S s_71_1: call IsFeatureImplemented(s_71_0)
        let s_71_1: bool = IsFeatureImplemented(state, tracer, s_71_0);
        // D s_71_2: write-var gs#101961 <= s_71_1
        fn_state.gs_101961 = s_71_1;
        // N s_71_3: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #102552u : u32
        let s_72_0: u32 = 102552;
        // D s_72_1: read-reg s_72_0:struct
        let s_72_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_0 as isize);
            tracer.read_register(s_72_0 as isize, value);
            value
        };
        // D s_72_2: call _get_HCR_EL2_Type_E2H(s_72_1)
        let s_72_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_72_1);
        // C s_72_3: const #102552u : u32
        let s_72_3: u32 = 102552;
        // D s_72_4: read-reg s_72_3:struct
        let s_72_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_3 as isize);
            tracer.read_register(s_72_3 as isize, value);
            value
        };
        // D s_72_5: call _get_HCR_EL2_Type_TGE(s_72_4)
        let s_72_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_72_4);
        // D s_72_6: cast zx s_72_2 -> bv
        let s_72_6: Bits = Bits::new(s_72_2 as u128, 1u16);
        // D s_72_7: cast zx s_72_5 -> bv
        let s_72_7: Bits = Bits::new(s_72_5 as u128, 1u16);
        // D s_72_8: cast reint s_72_6 -> u128
        let s_72_8: u128 = (s_72_6.value() as u128);
        // D s_72_9: size-of s_72_6
        let s_72_9: u16 = s_72_6.length();
        // D s_72_10: cast reint s_72_7 -> u128
        let s_72_10: u128 = (s_72_7.value() as u128);
        // D s_72_11: size-of s_72_7
        let s_72_11: u16 = s_72_7.length();
        // D s_72_12: lsl s_72_8 s_72_11
        let s_72_12: u128 = s_72_8 << s_72_11;
        // D s_72_13: or s_72_12 s_72_10
        let s_72_13: u128 = ((s_72_12) | (s_72_10));
        // D s_72_14: add s_72_9 s_72_11
        let s_72_14: u16 = (s_72_9 + s_72_11);
        // D s_72_15: create-bits s_72_13 s_72_14
        let s_72_15: Bits = Bits::new(s_72_13, s_72_14);
        // D s_72_16: cast reint s_72_15 -> u8
        let s_72_16: u8 = (s_72_15.value() as u8);
        // D s_72_17: cast zx s_72_16 -> bv
        let s_72_17: Bits = Bits::new(s_72_16 as u128, 2u16);
        // C s_72_18: const #3u : u8
        let s_72_18: u8 = 3;
        // C s_72_19: cast zx s_72_18 -> bv
        let s_72_19: Bits = Bits::new(s_72_18 as u128, 2u16);
        // D s_72_20: cmp-ne s_72_17 s_72_19
        let s_72_20: bool = ((s_72_17) != (s_72_19));
        // D s_72_21: write-var gs#101960 <= s_72_20
        fn_state.gs_101960 = s_72_20;
        // N s_72_22: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #24u : u8
        let s_73_0: u8 = 24;
        // C s_73_1: cast zx s_73_0 -> bv
        let s_73_1: Bits = Bits::new(s_73_0 as u128, 8u16);
        // C s_73_2: cast zx s_73_1 -> i
        let s_73_2: i128 = (s_73_1.value() as i128);
        // C s_73_3: cast reint s_73_2 -> i64
        let s_73_3: i64 = (s_73_2 as i64);
        // C s_73_4: cast zx s_73_3 -> i
        let s_73_4: i128 = (i128::try_from(s_73_3).unwrap());
        // C s_73_5: const #432u : u32
        let s_73_5: u32 = 432;
        // D s_73_6: read-reg s_73_5:u8
        let s_73_6: u8 = {
            let value = state.read_register::<u8>(s_73_5 as isize);
            tracer.read_register(s_73_5 as isize, value);
            value
        };
        // D s_73_7: call AArch64_SystemAccessTrap(s_73_6, s_73_4)
        let s_73_7: () = AArch64_SystemAccessTrap(state, tracer, s_73_6, s_73_4);
        // N s_73_8: return
        return;
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var __HCR_EL2_TOCU:u8
        let s_74_0: bool = fn_state.u__HCR_EL2_TOCU;
        // D s_74_1: cast zx s_74_0 -> bv
        let s_74_1: Bits = Bits::new(s_74_0 as u128, 1u16);
        // C s_74_2: const #1u : u8
        let s_74_2: bool = true;
        // C s_74_3: cast zx s_74_2 -> bv
        let s_74_3: Bits = Bits::new(s_74_2 as u128, 1u16);
        // D s_74_4: cmp-eq s_74_1 s_74_3
        let s_74_4: bool = ((s_74_1) == (s_74_3));
        // D s_74_5: write-var gs#101959 <= s_74_4
        fn_state.gs_101959 = s_74_4;
        // N s_74_6: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #102552u : u32
        let s_75_0: u32 = 102552;
        // D s_75_1: read-reg s_75_0:struct
        let s_75_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_75_0 as isize);
            tracer.read_register(s_75_0 as isize, value);
            value
        };
        // D s_75_2: call _get_HCR_EL2_Type_E2H(s_75_1)
        let s_75_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_75_1);
        // C s_75_3: const #102552u : u32
        let s_75_3: u32 = 102552;
        // D s_75_4: read-reg s_75_3:struct
        let s_75_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_75_3 as isize);
            tracer.read_register(s_75_3 as isize, value);
            value
        };
        // D s_75_5: call _get_HCR_EL2_Type_TGE(s_75_4)
        let s_75_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_75_4);
        // D s_75_6: cast zx s_75_2 -> bv
        let s_75_6: Bits = Bits::new(s_75_2 as u128, 1u16);
        // D s_75_7: cast zx s_75_5 -> bv
        let s_75_7: Bits = Bits::new(s_75_5 as u128, 1u16);
        // D s_75_8: cast reint s_75_6 -> u128
        let s_75_8: u128 = (s_75_6.value() as u128);
        // D s_75_9: size-of s_75_6
        let s_75_9: u16 = s_75_6.length();
        // D s_75_10: cast reint s_75_7 -> u128
        let s_75_10: u128 = (s_75_7.value() as u128);
        // D s_75_11: size-of s_75_7
        let s_75_11: u16 = s_75_7.length();
        // D s_75_12: lsl s_75_8 s_75_11
        let s_75_12: u128 = s_75_8 << s_75_11;
        // D s_75_13: or s_75_12 s_75_10
        let s_75_13: u128 = ((s_75_12) | (s_75_10));
        // D s_75_14: add s_75_9 s_75_11
        let s_75_14: u16 = (s_75_9 + s_75_11);
        // D s_75_15: create-bits s_75_13 s_75_14
        let s_75_15: Bits = Bits::new(s_75_13, s_75_14);
        // D s_75_16: cast reint s_75_15 -> u8
        let s_75_16: u8 = (s_75_15.value() as u8);
        // D s_75_17: cast zx s_75_16 -> bv
        let s_75_17: Bits = Bits::new(s_75_16 as u128, 2u16);
        // C s_75_18: const #3u : u8
        let s_75_18: u8 = 3;
        // C s_75_19: cast zx s_75_18 -> bv
        let s_75_19: Bits = Bits::new(s_75_18 as u128, 2u16);
        // D s_75_20: cmp-ne s_75_17 s_75_19
        let s_75_20: bool = ((s_75_17) != (s_75_19));
        // D s_75_21: write-var gs#101958 <= s_75_20
        fn_state.gs_101958 = s_75_20;
        // N s_75_22: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #24u : u8
        let s_76_0: u8 = 24;
        // C s_76_1: cast zx s_76_0 -> bv
        let s_76_1: Bits = Bits::new(s_76_0 as u128, 8u16);
        // C s_76_2: cast zx s_76_1 -> i
        let s_76_2: i128 = (s_76_1.value() as i128);
        // C s_76_3: cast reint s_76_2 -> i64
        let s_76_3: i64 = (s_76_2 as i64);
        // C s_76_4: cast zx s_76_3 -> i
        let s_76_4: i128 = (i128::try_from(s_76_3).unwrap());
        // C s_76_5: const #432u : u32
        let s_76_5: u32 = 432;
        // D s_76_6: read-reg s_76_5:u8
        let s_76_6: u8 = {
            let value = state.read_register::<u8>(s_76_5 as isize);
            tracer.read_register(s_76_5 as isize, value);
            value
        };
        // D s_76_7: call AArch64_SystemAccessTrap(s_76_6, s_76_4)
        let s_76_7: () = AArch64_SystemAccessTrap(state, tracer, s_76_6, s_76_4);
        // N s_76_8: return
        return;
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var __HCR_EL2_TPU:u8
        let s_77_0: bool = fn_state.u__HCR_EL2_TPU;
        // D s_77_1: cast zx s_77_0 -> bv
        let s_77_1: Bits = Bits::new(s_77_0 as u128, 1u16);
        // C s_77_2: const #1u : u8
        let s_77_2: bool = true;
        // C s_77_3: cast zx s_77_2 -> bv
        let s_77_3: Bits = Bits::new(s_77_2 as u128, 1u16);
        // D s_77_4: cmp-eq s_77_1 s_77_3
        let s_77_4: bool = ((s_77_1) == (s_77_3));
        // D s_77_5: write-var gs#101957 <= s_77_4
        fn_state.gs_101957 = s_77_4;
        // N s_77_6: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #102552u : u32
        let s_78_0: u32 = 102552;
        // D s_78_1: read-reg s_78_0:struct
        let s_78_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_78_0 as isize);
            tracer.read_register(s_78_0 as isize, value);
            value
        };
        // D s_78_2: call _get_HCR_EL2_Type_E2H(s_78_1)
        let s_78_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_78_1);
        // C s_78_3: const #102552u : u32
        let s_78_3: u32 = 102552;
        // D s_78_4: read-reg s_78_3:struct
        let s_78_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_78_3 as isize);
            tracer.read_register(s_78_3 as isize, value);
            value
        };
        // D s_78_5: call _get_HCR_EL2_Type_TGE(s_78_4)
        let s_78_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_78_4);
        // D s_78_6: cast zx s_78_2 -> bv
        let s_78_6: Bits = Bits::new(s_78_2 as u128, 1u16);
        // D s_78_7: cast zx s_78_5 -> bv
        let s_78_7: Bits = Bits::new(s_78_5 as u128, 1u16);
        // D s_78_8: cast reint s_78_6 -> u128
        let s_78_8: u128 = (s_78_6.value() as u128);
        // D s_78_9: size-of s_78_6
        let s_78_9: u16 = s_78_6.length();
        // D s_78_10: cast reint s_78_7 -> u128
        let s_78_10: u128 = (s_78_7.value() as u128);
        // D s_78_11: size-of s_78_7
        let s_78_11: u16 = s_78_7.length();
        // D s_78_12: lsl s_78_8 s_78_11
        let s_78_12: u128 = s_78_8 << s_78_11;
        // D s_78_13: or s_78_12 s_78_10
        let s_78_13: u128 = ((s_78_12) | (s_78_10));
        // D s_78_14: add s_78_9 s_78_11
        let s_78_14: u16 = (s_78_9 + s_78_11);
        // D s_78_15: create-bits s_78_13 s_78_14
        let s_78_15: Bits = Bits::new(s_78_13, s_78_14);
        // D s_78_16: cast reint s_78_15 -> u8
        let s_78_16: u8 = (s_78_15.value() as u8);
        // D s_78_17: cast zx s_78_16 -> bv
        let s_78_17: Bits = Bits::new(s_78_16 as u128, 2u16);
        // C s_78_18: const #3u : u8
        let s_78_18: u8 = 3;
        // C s_78_19: cast zx s_78_18 -> bv
        let s_78_19: Bits = Bits::new(s_78_18 as u128, 2u16);
        // D s_78_20: cmp-ne s_78_17 s_78_19
        let s_78_20: bool = ((s_78_17) != (s_78_19));
        // D s_78_21: write-var gs#101956 <= s_78_20
        fn_state.gs_101956 = s_78_20;
        // N s_78_22: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #() : ()
        let s_79_0: () = ();
        // S s_79_1: call EL2Enabled(s_79_0)
        let s_79_1: bool = EL2Enabled(state, tracer, s_79_0);
        // N s_79_2: branch s_79_1 b84 b80
        if s_79_1 {
            return block_84(state, tracer, fn_state);
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
        // D s_80_1: write-var gs#101968 <= s_80_0
        fn_state.gs_101968 = s_80_0;
        // N s_80_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#101968:u8
        let s_81_0: bool = fn_state.gs_101968;
        // N s_81_1: branch s_81_0 b83 b82
        if s_81_0 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_82(state, tracer, fn_state);
        };
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #24u : u8
        let s_82_0: u8 = 24;
        // C s_82_1: cast zx s_82_0 -> bv
        let s_82_1: Bits = Bits::new(s_82_0 as u128, 8u16);
        // C s_82_2: cast zx s_82_1 -> i
        let s_82_2: i128 = (s_82_1.value() as i128);
        // C s_82_3: cast reint s_82_2 -> i64
        let s_82_3: i64 = (s_82_2 as i64);
        // C s_82_4: cast zx s_82_3 -> i
        let s_82_4: i128 = (i128::try_from(s_82_3).unwrap());
        // C s_82_5: const #440u : u32
        let s_82_5: u32 = 440;
        // D s_82_6: read-reg s_82_5:u8
        let s_82_6: u8 = {
            let value = state.read_register::<u8>(s_82_5 as isize);
            tracer.read_register(s_82_5 as isize, value);
            value
        };
        // D s_82_7: call AArch64_SystemAccessTrap(s_82_6, s_82_4)
        let s_82_7: () = AArch64_SystemAccessTrap(state, tracer, s_82_6, s_82_4);
        // N s_82_8: return
        return;
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #24u : u8
        let s_83_0: u8 = 24;
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
        // D s_83_7: call AArch64_SystemAccessTrap(s_83_6, s_83_4)
        let s_83_7: () = AArch64_SystemAccessTrap(state, tracer, s_83_6, s_83_4);
        // N s_83_8: return
        return;
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var __HCR_EL2_TGE:u8
        let s_84_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_84_1: cast zx s_84_0 -> bv
        let s_84_1: Bits = Bits::new(s_84_0 as u128, 1u16);
        // C s_84_2: const #1u : u8
        let s_84_2: bool = true;
        // C s_84_3: cast zx s_84_2 -> bv
        let s_84_3: Bits = Bits::new(s_84_2 as u128, 1u16);
        // D s_84_4: cmp-eq s_84_1 s_84_3
        let s_84_4: bool = ((s_84_1) == (s_84_3));
        // D s_84_5: write-var gs#101968 <= s_84_4
        fn_state.gs_101968 = s_84_4;
        // N s_84_6: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var __SCTLR_EL1_UCI:u8
        let s_85_0: bool = fn_state.u__SCTLR_EL1_UCI;
        // D s_85_1: cast zx s_85_0 -> bv
        let s_85_1: Bits = Bits::new(s_85_0 as u128, 1u16);
        // C s_85_2: const #0u : u8
        let s_85_2: bool = false;
        // C s_85_3: cast zx s_85_2 -> bv
        let s_85_3: Bits = Bits::new(s_85_2 as u128, 1u16);
        // D s_85_4: cmp-eq s_85_1 s_85_3
        let s_85_4: bool = ((s_85_1) == (s_85_3));
        // D s_85_5: write-var gs#101955 <= s_85_4
        fn_state.gs_101955 = s_85_4;
        // N s_85_6: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #102552u : u32
        let s_86_0: u32 = 102552;
        // D s_86_1: read-reg s_86_0:struct
        let s_86_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_86_0 as isize);
            tracer.read_register(s_86_0 as isize, value);
            value
        };
        // D s_86_2: call _get_HCR_EL2_Type_E2H(s_86_1)
        let s_86_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_86_1);
        // C s_86_3: const #102552u : u32
        let s_86_3: u32 = 102552;
        // D s_86_4: read-reg s_86_3:struct
        let s_86_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_86_3 as isize);
            tracer.read_register(s_86_3 as isize, value);
            value
        };
        // D s_86_5: call _get_HCR_EL2_Type_TGE(s_86_4)
        let s_86_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_86_4);
        // D s_86_6: cast zx s_86_2 -> bv
        let s_86_6: Bits = Bits::new(s_86_2 as u128, 1u16);
        // D s_86_7: cast zx s_86_5 -> bv
        let s_86_7: Bits = Bits::new(s_86_5 as u128, 1u16);
        // D s_86_8: cast reint s_86_6 -> u128
        let s_86_8: u128 = (s_86_6.value() as u128);
        // D s_86_9: size-of s_86_6
        let s_86_9: u16 = s_86_6.length();
        // D s_86_10: cast reint s_86_7 -> u128
        let s_86_10: u128 = (s_86_7.value() as u128);
        // D s_86_11: size-of s_86_7
        let s_86_11: u16 = s_86_7.length();
        // D s_86_12: lsl s_86_8 s_86_11
        let s_86_12: u128 = s_86_8 << s_86_11;
        // D s_86_13: or s_86_12 s_86_10
        let s_86_13: u128 = ((s_86_12) | (s_86_10));
        // D s_86_14: add s_86_9 s_86_11
        let s_86_14: u16 = (s_86_9 + s_86_11);
        // D s_86_15: create-bits s_86_13 s_86_14
        let s_86_15: Bits = Bits::new(s_86_13, s_86_14);
        // D s_86_16: cast reint s_86_15 -> u8
        let s_86_16: u8 = (s_86_15.value() as u8);
        // D s_86_17: cast zx s_86_16 -> bv
        let s_86_17: Bits = Bits::new(s_86_16 as u128, 2u16);
        // C s_86_18: const #3u : u8
        let s_86_18: u8 = 3;
        // C s_86_19: cast zx s_86_18 -> bv
        let s_86_19: Bits = Bits::new(s_86_18 as u128, 2u16);
        // D s_86_20: cmp-eq s_86_17 s_86_19
        let s_86_20: bool = ((s_86_17) == (s_86_19));
        // D s_86_21: write-var gs#101954 <= s_86_20
        fn_state.gs_101954 = s_86_20;
        // N s_86_22: jump b34
        return block_34(state, tracer, fn_state);
    }
}
