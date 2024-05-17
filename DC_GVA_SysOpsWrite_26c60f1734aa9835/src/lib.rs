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
use u_get_HFGITR_EL2_Type_DCZVA::*;
use u_get_SCR_EL3_Type_FGTEn::*;
use IsFeatureImplemented::*;
use u_get_HCR_EL2_Type_E2H::*;
use u_get_SCTLR_EL2_Type_DZE::*;
use u_get_HCR_EL2_Type_TDZ::*;
use X_read::*;
use u_get_SCTLR_EL1_Type_DZE::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use AArch64_MemZero::*;
use common::*;
pub fn DC_GVA_SysOpsWrite_26c60f1734aa9835<T: Tracer>(
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
        gs_101770: bool,
        gs_101762: bool,
        gs_101765: bool,
        u__HCR_EL2_TGE: bool,
        gs_101780: bool,
        gs_101777: bool,
        gs_101764: bool,
        u__SCR_EL3_FGTEn: bool,
        u__SCTLR_EL2_DZE: bool,
        u__HFGITR_EL2_DCZVA: bool,
        gs_101766: bool,
        gs_101775: bool,
        u__SCTLR_EL1_DZE: bool,
        gs_101769: bool,
        gs_101774: bool,
        gs_101773: bool,
        gs_101763: bool,
        gs_101771: bool,
        gs_101768: bool,
        gs_101776: bool,
        u__PSTATE_EL: u8,
        gs_101772: bool,
        gs_101778: bool,
        u__HCR_EL2_TDZ: bool,
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
        // D s_0_5: call _get_SCTLR_EL1_Type_DZE(s_0_4)
        let s_0_5: bool = u_get_SCTLR_EL1_Type_DZE(state, tracer, s_0_4);
        // D s_0_6: write-var __SCTLR_EL1_DZE <= s_0_5
        fn_state.u__SCTLR_EL1_DZE = s_0_5;
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
        // D s_0_13: call _get_HCR_EL2_Type_TDZ(s_0_12)
        let s_0_13: bool = u_get_HCR_EL2_Type_TDZ(state, tracer, s_0_12);
        // D s_0_14: write-var __HCR_EL2_TDZ <= s_0_13
        fn_state.u__HCR_EL2_TDZ = s_0_13;
        // C s_0_15: const #90704u : u32
        let s_0_15: u32 = 90704;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_SCR_EL3_Type_FGTEn(s_0_16)
        let s_0_17: bool = u_get_SCR_EL3_Type_FGTEn(state, tracer, s_0_16);
        // D s_0_18: write-var __SCR_EL3_FGTEn <= s_0_17
        fn_state.u__SCR_EL3_FGTEn = s_0_17;
        // C s_0_19: const #13608u : u32
        let s_0_19: u32 = 13608;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_HFGITR_EL2_Type_DCZVA(s_0_20)
        let s_0_21: bool = u_get_HFGITR_EL2_Type_DCZVA(state, tracer, s_0_20);
        // D s_0_22: write-var __HFGITR_EL2_DCZVA <= s_0_21
        fn_state.u__HFGITR_EL2_DCZVA = s_0_21;
        // C s_0_23: const #20784u : u32
        let s_0_23: u32 = 20784;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_SCTLR_EL2_Type_DZE(s_0_24)
        let s_0_25: bool = u_get_SCTLR_EL2_Type_DZE(state, tracer, s_0_24);
        // D s_0_26: write-var __SCTLR_EL2_DZE <= s_0_25
        fn_state.u__SCTLR_EL2_DZE = s_0_25;
        // D s_0_27: read-var __PSTATE_EL:u8
        let s_0_27: u8 = fn_state.u__PSTATE_EL;
        // D s_0_28: cast zx s_0_27 -> bv
        let s_0_28: Bits = Bits::new(s_0_27 as u128, 2u16);
        // C s_0_29: const #448u : u32
        let s_0_29: u32 = 448;
        // D s_0_30: read-reg s_0_29:u8
        let s_0_30: u8 = {
            let value = state.read_register::<u8>(s_0_29 as isize);
            tracer.read_register(s_0_29 as isize, value);
            value
        };
        // D s_0_31: cast zx s_0_30 -> bv
        let s_0_31: Bits = Bits::new(s_0_30 as u128, 2u16);
        // D s_0_32: cmp-eq s_0_28 s_0_31
        let s_0_32: bool = ((s_0_28) == (s_0_31));
        // N s_0_33: branch s_0_32 b27 b1
        if s_0_32 {
            return block_27(state, tracer, fn_state);
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
        // D s_5_5: call AArch64_MemZero(s_5_3, s_5_4)
        let s_5_5: () = AArch64_MemZero(state, tracer, s_5_3, s_5_4);
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
        // D s_6_5: call AArch64_MemZero(s_6_3, s_6_4)
        let s_6_5: () = AArch64_MemZero(state, tracer, s_6_3, s_6_4);
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
        // N s_7_2: branch s_7_1 b26 b8
        if s_7_1 {
            return block_26(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#101762 <= s_8_0
        fn_state.gs_101762 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#101762:u8
        let s_9_0: bool = fn_state.gs_101762;
        // N s_9_1: branch s_9_0 b25 b10
        if s_9_0 {
            return block_25(state, tracer, fn_state);
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
        // N s_10_2: branch s_10_1 b24 b11
        if s_10_1 {
            return block_24(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#101763 <= s_11_0
        fn_state.gs_101763 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#101763:u8
        let s_12_0: bool = fn_state.gs_101763;
        // N s_12_1: branch s_12_0 b20 b13
        if s_12_0 {
            return block_20(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#101765 <= s_13_0
        fn_state.gs_101765 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#101765:u8
        let s_14_0: bool = fn_state.gs_101765;
        // N s_14_1: branch s_14_0 b19 b15
        if s_14_0 {
            return block_19(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#101766 <= s_15_0
        fn_state.gs_101766 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#101766:u8
        let s_16_0: bool = fn_state.gs_101766;
        // N s_16_1: branch s_16_0 b18 b17
        if s_16_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #64s : i64
        let s_17_0: i64 = 64;
        // D s_17_1: read-var t:i
        let s_17_1: i128 = fn_state.t;
        // D s_17_2: call X_read(s_17_1, s_17_0)
        let s_17_2: Bits = X_read(state, tracer, s_17_1, s_17_0);
        // D s_17_3: cast reint s_17_2 -> u64
        let s_17_3: u64 = (s_17_2.value() as u64);
        // C s_17_4: const #1u : u32
        let s_17_4: u32 = 1;
        // D s_17_5: call AArch64_MemZero(s_17_3, s_17_4)
        let s_17_5: () = AArch64_MemZero(state, tracer, s_17_3, s_17_4);
        // N s_17_6: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #24u : u8
        let s_18_0: u8 = 24;
        // C s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 8u16);
        // C s_18_2: cast zx s_18_1 -> i
        let s_18_2: i128 = (s_18_1.value() as i128);
        // C s_18_3: cast reint s_18_2 -> i64
        let s_18_3: i64 = (s_18_2 as i64);
        // C s_18_4: cast zx s_18_3 -> i
        let s_18_4: i128 = (i128::try_from(s_18_3).unwrap());
        // C s_18_5: const #432u : u32
        let s_18_5: u32 = 432;
        // D s_18_6: read-reg s_18_5:u8
        let s_18_6: u8 = {
            let value = state.read_register::<u8>(s_18_5 as isize);
            tracer.read_register(s_18_5 as isize, value);
            value
        };
        // D s_18_7: call AArch64_SystemAccessTrap(s_18_6, s_18_4)
        let s_18_7: () = AArch64_SystemAccessTrap(state, tracer, s_18_6, s_18_4);
        // N s_18_8: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var __HFGITR_EL2_DCZVA:u8
        let s_19_0: bool = fn_state.u__HFGITR_EL2_DCZVA;
        // D s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 1u16);
        // C s_19_2: const #1u : u8
        let s_19_2: bool = true;
        // C s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 1u16);
        // D s_19_4: cmp-eq s_19_1 s_19_3
        let s_19_4: bool = ((s_19_1) == (s_19_3));
        // D s_19_5: write-var gs#101766 <= s_19_4
        fn_state.gs_101766 = s_19_4;
        // N s_19_6: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #424u : u32
        let s_20_0: u32 = 424;
        // D s_20_1: read-reg s_20_0:u8
        let s_20_1: u8 = {
            let value = state.read_register::<u8>(s_20_0 as isize);
            tracer.read_register(s_20_0 as isize, value);
            value
        };
        // C s_20_2: const #2u : u8
        let s_20_2: u8 = 2;
        // D s_20_3: cmp-lt s_20_1 s_20_2
        let s_20_3: bool = ((s_20_1) < (s_20_2));
        // D s_20_4: not s_20_3
        let s_20_4: bool = !s_20_3;
        // N s_20_5: branch s_20_4 b23 b21
        if s_20_4 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var __SCR_EL3_FGTEn:u8
        let s_21_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 1u16);
        // C s_21_2: const #1u : u8
        let s_21_2: bool = true;
        // C s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 1u16);
        // D s_21_4: cmp-eq s_21_1 s_21_3
        let s_21_4: bool = ((s_21_1) == (s_21_3));
        // D s_21_5: write-var gs#101764 <= s_21_4
        fn_state.gs_101764 = s_21_4;
        // N s_21_6: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#101764:u8
        let s_22_0: bool = fn_state.gs_101764;
        // D s_22_1: write-var gs#101765 <= s_22_0
        fn_state.gs_101765 = s_22_0;
        // N s_22_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #1u : u8
        let s_23_0: bool = true;
        // D s_23_1: write-var gs#101764 <= s_23_0
        fn_state.gs_101764 = s_23_0;
        // N s_23_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #146u : u32
        let s_24_0: u32 = 146;
        // S s_24_1: call IsFeatureImplemented(s_24_0)
        let s_24_1: bool = IsFeatureImplemented(state, tracer, s_24_0);
        // D s_24_2: write-var gs#101763 <= s_24_1
        fn_state.gs_101763 = s_24_1;
        // N s_24_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #24u : u8
        let s_25_0: u8 = 24;
        // C s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 8u16);
        // C s_25_2: cast zx s_25_1 -> i
        let s_25_2: i128 = (s_25_1.value() as i128);
        // C s_25_3: cast reint s_25_2 -> i64
        let s_25_3: i64 = (s_25_2 as i64);
        // C s_25_4: cast zx s_25_3 -> i
        let s_25_4: i128 = (i128::try_from(s_25_3).unwrap());
        // C s_25_5: const #432u : u32
        let s_25_5: u32 = 432;
        // D s_25_6: read-reg s_25_5:u8
        let s_25_6: u8 = {
            let value = state.read_register::<u8>(s_25_5 as isize);
            tracer.read_register(s_25_5 as isize, value);
            value
        };
        // D s_25_7: call AArch64_SystemAccessTrap(s_25_6, s_25_4)
        let s_25_7: () = AArch64_SystemAccessTrap(state, tracer, s_25_6, s_25_4);
        // N s_25_8: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var __HCR_EL2_TDZ:u8
        let s_26_0: bool = fn_state.u__HCR_EL2_TDZ;
        // D s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 1u16);
        // C s_26_2: const #1u : u8
        let s_26_2: bool = true;
        // C s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 1u16);
        // D s_26_4: cmp-eq s_26_1 s_26_3
        let s_26_4: bool = ((s_26_1) == (s_26_3));
        // D s_26_5: write-var gs#101762 <= s_26_4
        fn_state.gs_101762 = s_26_4;
        // N s_26_6: jump b9
        return block_9(state, tracer, fn_state);
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
        // N s_27_2: branch s_27_1 b73 b28
        if s_27_1 {
            return block_73(state, tracer, fn_state);
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
        // D s_28_1: write-var gs#101768 <= s_28_0
        fn_state.gs_101768 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#101768:u8
        let s_29_0: bool = fn_state.gs_101768;
        // D s_29_1: not s_29_0
        let s_29_1: bool = !s_29_0;
        // N s_29_2: branch s_29_1 b72 b30
        if s_29_1 {
            return block_72(state, tracer, fn_state);
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
        // D s_30_1: write-var gs#101769 <= s_30_0
        fn_state.gs_101769 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#101769:u8
        let s_31_0: bool = fn_state.gs_101769;
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
        // C s_32_0: const #() : ()
        let s_32_0: () = ();
        // S s_32_1: call EL2Enabled(s_32_0)
        let s_32_1: bool = EL2Enabled(state, tracer, s_32_0);
        // N s_32_2: branch s_32_1 b65 b33
        if s_32_1 {
            return block_65(state, tracer, fn_state);
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
        // D s_33_1: write-var gs#101770 <= s_33_0
        fn_state.gs_101770 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#101770:u8
        let s_34_0: bool = fn_state.gs_101770;
        // N s_34_1: branch s_34_0 b64 b35
        if s_34_0 {
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
        // C s_35_0: const #0u : u8
        let s_35_0: bool = false;
        // D s_35_1: write-var gs#101771 <= s_35_0
        fn_state.gs_101771 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#101771:u8
        let s_36_0: bool = fn_state.gs_101771;
        // N s_36_1: branch s_36_0 b63 b37
        if s_36_0 {
            return block_63(state, tracer, fn_state);
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
        // N s_37_2: branch s_37_1 b62 b38
        if s_37_1 {
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
        // D s_38_1: write-var gs#101772 <= s_38_0
        fn_state.gs_101772 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#101772:u8
        let s_39_0: bool = fn_state.gs_101772;
        // N s_39_1: branch s_39_0 b61 b40
        if s_39_0 {
            return block_61(state, tracer, fn_state);
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
        // D s_40_1: write-var gs#101773 <= s_40_0
        fn_state.gs_101773 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#101773:u8
        let s_41_0: bool = fn_state.gs_101773;
        // N s_41_1: branch s_41_0 b57 b42
        if s_41_0 {
            return block_57(state, tracer, fn_state);
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
        // D s_42_1: write-var gs#101775 <= s_42_0
        fn_state.gs_101775 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#101775:u8
        let s_43_0: bool = fn_state.gs_101775;
        // N s_43_1: branch s_43_0 b56 b44
        if s_43_0 {
            return block_56(state, tracer, fn_state);
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
        // D s_44_1: write-var gs#101776 <= s_44_0
        fn_state.gs_101776 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#101776:u8
        let s_45_0: bool = fn_state.gs_101776;
        // N s_45_1: branch s_45_0 b55 b46
        if s_45_0 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #() : ()
        let s_46_0: () = ();
        // S s_46_1: call EL2Enabled(s_46_0)
        let s_46_1: bool = EL2Enabled(state, tracer, s_46_0);
        // N s_46_2: branch s_46_1 b54 b47
        if s_46_1 {
            return block_54(state, tracer, fn_state);
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
        // D s_47_1: write-var gs#101777 <= s_47_0
        fn_state.gs_101777 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#101777:u8
        let s_48_0: bool = fn_state.gs_101777;
        // N s_48_1: branch s_48_0 b53 b49
        if s_48_0 {
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
        // D s_49_1: write-var gs#101778 <= s_49_0
        fn_state.gs_101778 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#101778:u8
        let s_50_0: bool = fn_state.gs_101778;
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
        // C s_51_0: const #64s : i64
        let s_51_0: i64 = 64;
        // D s_51_1: read-var t:i
        let s_51_1: i128 = fn_state.t;
        // D s_51_2: call X_read(s_51_1, s_51_0)
        let s_51_2: Bits = X_read(state, tracer, s_51_1, s_51_0);
        // D s_51_3: cast reint s_51_2 -> u64
        let s_51_3: u64 = (s_51_2.value() as u64);
        // C s_51_4: const #1u : u32
        let s_51_4: u32 = 1;
        // D s_51_5: call AArch64_MemZero(s_51_3, s_51_4)
        let s_51_5: () = AArch64_MemZero(state, tracer, s_51_3, s_51_4);
        // N s_51_6: return
        return;
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #24u : u8
        let s_52_0: u8 = 24;
        // C s_52_1: cast zx s_52_0 -> bv
        let s_52_1: Bits = Bits::new(s_52_0 as u128, 8u16);
        // C s_52_2: cast zx s_52_1 -> i
        let s_52_2: i128 = (s_52_1.value() as i128);
        // C s_52_3: cast reint s_52_2 -> i64
        let s_52_3: i64 = (s_52_2 as i64);
        // C s_52_4: cast zx s_52_3 -> i
        let s_52_4: i128 = (i128::try_from(s_52_3).unwrap());
        // C s_52_5: const #432u : u32
        let s_52_5: u32 = 432;
        // D s_52_6: read-reg s_52_5:u8
        let s_52_6: u8 = {
            let value = state.read_register::<u8>(s_52_5 as isize);
            tracer.read_register(s_52_5 as isize, value);
            value
        };
        // D s_52_7: call AArch64_SystemAccessTrap(s_52_6, s_52_4)
        let s_52_7: () = AArch64_SystemAccessTrap(state, tracer, s_52_6, s_52_4);
        // N s_52_8: return
        return;
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var __SCTLR_EL2_DZE:u8
        let s_53_0: bool = fn_state.u__SCTLR_EL2_DZE;
        // D s_53_1: cast zx s_53_0 -> bv
        let s_53_1: Bits = Bits::new(s_53_0 as u128, 1u16);
        // C s_53_2: const #0u : u8
        let s_53_2: bool = false;
        // C s_53_3: cast zx s_53_2 -> bv
        let s_53_3: Bits = Bits::new(s_53_2 as u128, 1u16);
        // D s_53_4: cmp-eq s_53_1 s_53_3
        let s_53_4: bool = ((s_53_1) == (s_53_3));
        // D s_53_5: write-var gs#101778 <= s_53_4
        fn_state.gs_101778 = s_53_4;
        // N s_53_6: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #102552u : u32
        let s_54_0: u32 = 102552;
        // D s_54_1: read-reg s_54_0:struct
        let s_54_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_54_0 as isize);
            tracer.read_register(s_54_0 as isize, value);
            value
        };
        // D s_54_2: call _get_HCR_EL2_Type_E2H(s_54_1)
        let s_54_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_54_1);
        // C s_54_3: const #102552u : u32
        let s_54_3: u32 = 102552;
        // D s_54_4: read-reg s_54_3:struct
        let s_54_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_54_3 as isize);
            tracer.read_register(s_54_3 as isize, value);
            value
        };
        // D s_54_5: call _get_HCR_EL2_Type_TGE(s_54_4)
        let s_54_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_54_4);
        // D s_54_6: cast zx s_54_2 -> bv
        let s_54_6: Bits = Bits::new(s_54_2 as u128, 1u16);
        // D s_54_7: cast zx s_54_5 -> bv
        let s_54_7: Bits = Bits::new(s_54_5 as u128, 1u16);
        // D s_54_8: cast reint s_54_6 -> u128
        let s_54_8: u128 = (s_54_6.value() as u128);
        // D s_54_9: size-of s_54_6
        let s_54_9: u16 = s_54_6.length();
        // D s_54_10: cast reint s_54_7 -> u128
        let s_54_10: u128 = (s_54_7.value() as u128);
        // D s_54_11: size-of s_54_7
        let s_54_11: u16 = s_54_7.length();
        // D s_54_12: lsl s_54_8 s_54_11
        let s_54_12: u128 = s_54_8 << s_54_11;
        // D s_54_13: or s_54_12 s_54_10
        let s_54_13: u128 = ((s_54_12) | (s_54_10));
        // D s_54_14: add s_54_9 s_54_11
        let s_54_14: u16 = (s_54_9 + s_54_11);
        // D s_54_15: create-bits s_54_13 s_54_14
        let s_54_15: Bits = Bits::new(s_54_13, s_54_14);
        // D s_54_16: cast reint s_54_15 -> u8
        let s_54_16: u8 = (s_54_15.value() as u8);
        // D s_54_17: cast zx s_54_16 -> bv
        let s_54_17: Bits = Bits::new(s_54_16 as u128, 2u16);
        // C s_54_18: const #3u : u8
        let s_54_18: u8 = 3;
        // C s_54_19: cast zx s_54_18 -> bv
        let s_54_19: Bits = Bits::new(s_54_18 as u128, 2u16);
        // D s_54_20: cmp-eq s_54_17 s_54_19
        let s_54_20: bool = ((s_54_17) == (s_54_19));
        // D s_54_21: write-var gs#101777 <= s_54_20
        fn_state.gs_101777 = s_54_20;
        // N s_54_22: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #24u : u8
        let s_55_0: u8 = 24;
        // C s_55_1: cast zx s_55_0 -> bv
        let s_55_1: Bits = Bits::new(s_55_0 as u128, 8u16);
        // C s_55_2: cast zx s_55_1 -> i
        let s_55_2: i128 = (s_55_1.value() as i128);
        // C s_55_3: cast reint s_55_2 -> i64
        let s_55_3: i64 = (s_55_2 as i64);
        // C s_55_4: cast zx s_55_3 -> i
        let s_55_4: i128 = (i128::try_from(s_55_3).unwrap());
        // C s_55_5: const #432u : u32
        let s_55_5: u32 = 432;
        // D s_55_6: read-reg s_55_5:u8
        let s_55_6: u8 = {
            let value = state.read_register::<u8>(s_55_5 as isize);
            tracer.read_register(s_55_5 as isize, value);
            value
        };
        // D s_55_7: call AArch64_SystemAccessTrap(s_55_6, s_55_4)
        let s_55_7: () = AArch64_SystemAccessTrap(state, tracer, s_55_6, s_55_4);
        // N s_55_8: return
        return;
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var __HFGITR_EL2_DCZVA:u8
        let s_56_0: bool = fn_state.u__HFGITR_EL2_DCZVA;
        // D s_56_1: cast zx s_56_0 -> bv
        let s_56_1: Bits = Bits::new(s_56_0 as u128, 1u16);
        // C s_56_2: const #1u : u8
        let s_56_2: bool = true;
        // C s_56_3: cast zx s_56_2 -> bv
        let s_56_3: Bits = Bits::new(s_56_2 as u128, 1u16);
        // D s_56_4: cmp-eq s_56_1 s_56_3
        let s_56_4: bool = ((s_56_1) == (s_56_3));
        // D s_56_5: write-var gs#101776 <= s_56_4
        fn_state.gs_101776 = s_56_4;
        // N s_56_6: jump b45
        return block_45(state, tracer, fn_state);
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
        // C s_57_2: const #2u : u8
        let s_57_2: u8 = 2;
        // D s_57_3: cmp-lt s_57_1 s_57_2
        let s_57_3: bool = ((s_57_1) < (s_57_2));
        // D s_57_4: not s_57_3
        let s_57_4: bool = !s_57_3;
        // N s_57_5: branch s_57_4 b60 b58
        if s_57_4 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var __SCR_EL3_FGTEn:u8
        let s_58_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_58_1: cast zx s_58_0 -> bv
        let s_58_1: Bits = Bits::new(s_58_0 as u128, 1u16);
        // C s_58_2: const #1u : u8
        let s_58_2: bool = true;
        // C s_58_3: cast zx s_58_2 -> bv
        let s_58_3: Bits = Bits::new(s_58_2 as u128, 1u16);
        // D s_58_4: cmp-eq s_58_1 s_58_3
        let s_58_4: bool = ((s_58_1) == (s_58_3));
        // D s_58_5: write-var gs#101774 <= s_58_4
        fn_state.gs_101774 = s_58_4;
        // N s_58_6: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var gs#101774:u8
        let s_59_0: bool = fn_state.gs_101774;
        // D s_59_1: write-var gs#101775 <= s_59_0
        fn_state.gs_101775 = s_59_0;
        // N s_59_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #1u : u8
        let s_60_0: bool = true;
        // D s_60_1: write-var gs#101774 <= s_60_0
        fn_state.gs_101774 = s_60_0;
        // N s_60_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #146u : u32
        let s_61_0: u32 = 146;
        // S s_61_1: call IsFeatureImplemented(s_61_0)
        let s_61_1: bool = IsFeatureImplemented(state, tracer, s_61_0);
        // D s_61_2: write-var gs#101773 <= s_61_1
        fn_state.gs_101773 = s_61_1;
        // N s_61_3: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #102552u : u32
        let s_62_0: u32 = 102552;
        // D s_62_1: read-reg s_62_0:struct
        let s_62_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_62_0 as isize);
            tracer.read_register(s_62_0 as isize, value);
            value
        };
        // D s_62_2: call _get_HCR_EL2_Type_E2H(s_62_1)
        let s_62_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_62_1);
        // C s_62_3: const #102552u : u32
        let s_62_3: u32 = 102552;
        // D s_62_4: read-reg s_62_3:struct
        let s_62_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_62_3 as isize);
            tracer.read_register(s_62_3 as isize, value);
            value
        };
        // D s_62_5: call _get_HCR_EL2_Type_TGE(s_62_4)
        let s_62_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_62_4);
        // D s_62_6: cast zx s_62_2 -> bv
        let s_62_6: Bits = Bits::new(s_62_2 as u128, 1u16);
        // D s_62_7: cast zx s_62_5 -> bv
        let s_62_7: Bits = Bits::new(s_62_5 as u128, 1u16);
        // D s_62_8: cast reint s_62_6 -> u128
        let s_62_8: u128 = (s_62_6.value() as u128);
        // D s_62_9: size-of s_62_6
        let s_62_9: u16 = s_62_6.length();
        // D s_62_10: cast reint s_62_7 -> u128
        let s_62_10: u128 = (s_62_7.value() as u128);
        // D s_62_11: size-of s_62_7
        let s_62_11: u16 = s_62_7.length();
        // D s_62_12: lsl s_62_8 s_62_11
        let s_62_12: u128 = s_62_8 << s_62_11;
        // D s_62_13: or s_62_12 s_62_10
        let s_62_13: u128 = ((s_62_12) | (s_62_10));
        // D s_62_14: add s_62_9 s_62_11
        let s_62_14: u16 = (s_62_9 + s_62_11);
        // D s_62_15: create-bits s_62_13 s_62_14
        let s_62_15: Bits = Bits::new(s_62_13, s_62_14);
        // D s_62_16: cast reint s_62_15 -> u8
        let s_62_16: u8 = (s_62_15.value() as u8);
        // D s_62_17: cast zx s_62_16 -> bv
        let s_62_17: Bits = Bits::new(s_62_16 as u128, 2u16);
        // C s_62_18: const #3u : u8
        let s_62_18: u8 = 3;
        // C s_62_19: cast zx s_62_18 -> bv
        let s_62_19: Bits = Bits::new(s_62_18 as u128, 2u16);
        // D s_62_20: cmp-ne s_62_17 s_62_19
        let s_62_20: bool = ((s_62_17) != (s_62_19));
        // D s_62_21: write-var gs#101772 <= s_62_20
        fn_state.gs_101772 = s_62_20;
        // N s_62_22: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #24u : u8
        let s_63_0: u8 = 24;
        // C s_63_1: cast zx s_63_0 -> bv
        let s_63_1: Bits = Bits::new(s_63_0 as u128, 8u16);
        // C s_63_2: cast zx s_63_1 -> i
        let s_63_2: i128 = (s_63_1.value() as i128);
        // C s_63_3: cast reint s_63_2 -> i64
        let s_63_3: i64 = (s_63_2 as i64);
        // C s_63_4: cast zx s_63_3 -> i
        let s_63_4: i128 = (i128::try_from(s_63_3).unwrap());
        // C s_63_5: const #432u : u32
        let s_63_5: u32 = 432;
        // D s_63_6: read-reg s_63_5:u8
        let s_63_6: u8 = {
            let value = state.read_register::<u8>(s_63_5 as isize);
            tracer.read_register(s_63_5 as isize, value);
            value
        };
        // D s_63_7: call AArch64_SystemAccessTrap(s_63_6, s_63_4)
        let s_63_7: () = AArch64_SystemAccessTrap(state, tracer, s_63_6, s_63_4);
        // N s_63_8: return
        return;
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var __HCR_EL2_TDZ:u8
        let s_64_0: bool = fn_state.u__HCR_EL2_TDZ;
        // D s_64_1: cast zx s_64_0 -> bv
        let s_64_1: Bits = Bits::new(s_64_0 as u128, 1u16);
        // C s_64_2: const #1u : u8
        let s_64_2: bool = true;
        // C s_64_3: cast zx s_64_2 -> bv
        let s_64_3: Bits = Bits::new(s_64_2 as u128, 1u16);
        // D s_64_4: cmp-eq s_64_1 s_64_3
        let s_64_4: bool = ((s_64_1) == (s_64_3));
        // D s_64_5: write-var gs#101771 <= s_64_4
        fn_state.gs_101771 = s_64_4;
        // N s_64_6: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #102552u : u32
        let s_65_0: u32 = 102552;
        // D s_65_1: read-reg s_65_0:struct
        let s_65_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_65_0 as isize);
            tracer.read_register(s_65_0 as isize, value);
            value
        };
        // D s_65_2: call _get_HCR_EL2_Type_E2H(s_65_1)
        let s_65_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_65_1);
        // C s_65_3: const #102552u : u32
        let s_65_3: u32 = 102552;
        // D s_65_4: read-reg s_65_3:struct
        let s_65_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_65_3 as isize);
            tracer.read_register(s_65_3 as isize, value);
            value
        };
        // D s_65_5: call _get_HCR_EL2_Type_TGE(s_65_4)
        let s_65_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_65_4);
        // D s_65_6: cast zx s_65_2 -> bv
        let s_65_6: Bits = Bits::new(s_65_2 as u128, 1u16);
        // D s_65_7: cast zx s_65_5 -> bv
        let s_65_7: Bits = Bits::new(s_65_5 as u128, 1u16);
        // D s_65_8: cast reint s_65_6 -> u128
        let s_65_8: u128 = (s_65_6.value() as u128);
        // D s_65_9: size-of s_65_6
        let s_65_9: u16 = s_65_6.length();
        // D s_65_10: cast reint s_65_7 -> u128
        let s_65_10: u128 = (s_65_7.value() as u128);
        // D s_65_11: size-of s_65_7
        let s_65_11: u16 = s_65_7.length();
        // D s_65_12: lsl s_65_8 s_65_11
        let s_65_12: u128 = s_65_8 << s_65_11;
        // D s_65_13: or s_65_12 s_65_10
        let s_65_13: u128 = ((s_65_12) | (s_65_10));
        // D s_65_14: add s_65_9 s_65_11
        let s_65_14: u16 = (s_65_9 + s_65_11);
        // D s_65_15: create-bits s_65_13 s_65_14
        let s_65_15: Bits = Bits::new(s_65_13, s_65_14);
        // D s_65_16: cast reint s_65_15 -> u8
        let s_65_16: u8 = (s_65_15.value() as u8);
        // D s_65_17: cast zx s_65_16 -> bv
        let s_65_17: Bits = Bits::new(s_65_16 as u128, 2u16);
        // C s_65_18: const #3u : u8
        let s_65_18: u8 = 3;
        // C s_65_19: cast zx s_65_18 -> bv
        let s_65_19: Bits = Bits::new(s_65_18 as u128, 2u16);
        // D s_65_20: cmp-ne s_65_17 s_65_19
        let s_65_20: bool = ((s_65_17) != (s_65_19));
        // D s_65_21: write-var gs#101770 <= s_65_20
        fn_state.gs_101770 = s_65_20;
        // N s_65_22: jump b34
        return block_34(state, tracer, fn_state);
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
        // N s_66_2: branch s_66_1 b71 b67
        if s_66_1 {
            return block_71(state, tracer, fn_state);
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
        // D s_67_1: write-var gs#101780 <= s_67_0
        fn_state.gs_101780 = s_67_0;
        // N s_67_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var gs#101780:u8
        let s_68_0: bool = fn_state.gs_101780;
        // N s_68_1: branch s_68_0 b70 b69
        if s_68_0 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #24u : u8
        let s_69_0: u8 = 24;
        // C s_69_1: cast zx s_69_0 -> bv
        let s_69_1: Bits = Bits::new(s_69_0 as u128, 8u16);
        // C s_69_2: cast zx s_69_1 -> i
        let s_69_2: i128 = (s_69_1.value() as i128);
        // C s_69_3: cast reint s_69_2 -> i64
        let s_69_3: i64 = (s_69_2 as i64);
        // C s_69_4: cast zx s_69_3 -> i
        let s_69_4: i128 = (i128::try_from(s_69_3).unwrap());
        // C s_69_5: const #440u : u32
        let s_69_5: u32 = 440;
        // D s_69_6: read-reg s_69_5:u8
        let s_69_6: u8 = {
            let value = state.read_register::<u8>(s_69_5 as isize);
            tracer.read_register(s_69_5 as isize, value);
            value
        };
        // D s_69_7: call AArch64_SystemAccessTrap(s_69_6, s_69_4)
        let s_69_7: () = AArch64_SystemAccessTrap(state, tracer, s_69_6, s_69_4);
        // N s_69_8: return
        return;
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #24u : u8
        let s_70_0: u8 = 24;
        // C s_70_1: cast zx s_70_0 -> bv
        let s_70_1: Bits = Bits::new(s_70_0 as u128, 8u16);
        // C s_70_2: cast zx s_70_1 -> i
        let s_70_2: i128 = (s_70_1.value() as i128);
        // C s_70_3: cast reint s_70_2 -> i64
        let s_70_3: i64 = (s_70_2 as i64);
        // C s_70_4: cast zx s_70_3 -> i
        let s_70_4: i128 = (i128::try_from(s_70_3).unwrap());
        // C s_70_5: const #432u : u32
        let s_70_5: u32 = 432;
        // D s_70_6: read-reg s_70_5:u8
        let s_70_6: u8 = {
            let value = state.read_register::<u8>(s_70_5 as isize);
            tracer.read_register(s_70_5 as isize, value);
            value
        };
        // D s_70_7: call AArch64_SystemAccessTrap(s_70_6, s_70_4)
        let s_70_7: () = AArch64_SystemAccessTrap(state, tracer, s_70_6, s_70_4);
        // N s_70_8: return
        return;
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var __HCR_EL2_TGE:u8
        let s_71_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_71_1: cast zx s_71_0 -> bv
        let s_71_1: Bits = Bits::new(s_71_0 as u128, 1u16);
        // C s_71_2: const #1u : u8
        let s_71_2: bool = true;
        // C s_71_3: cast zx s_71_2 -> bv
        let s_71_3: Bits = Bits::new(s_71_2 as u128, 1u16);
        // D s_71_4: cmp-eq s_71_1 s_71_3
        let s_71_4: bool = ((s_71_1) == (s_71_3));
        // D s_71_5: write-var gs#101780 <= s_71_4
        fn_state.gs_101780 = s_71_4;
        // N s_71_6: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var __SCTLR_EL1_DZE:u8
        let s_72_0: bool = fn_state.u__SCTLR_EL1_DZE;
        // D s_72_1: cast zx s_72_0 -> bv
        let s_72_1: Bits = Bits::new(s_72_0 as u128, 1u16);
        // C s_72_2: const #0u : u8
        let s_72_2: bool = false;
        // C s_72_3: cast zx s_72_2 -> bv
        let s_72_3: Bits = Bits::new(s_72_2 as u128, 1u16);
        // D s_72_4: cmp-eq s_72_1 s_72_3
        let s_72_4: bool = ((s_72_1) == (s_72_3));
        // D s_72_5: write-var gs#101769 <= s_72_4
        fn_state.gs_101769 = s_72_4;
        // N s_72_6: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #102552u : u32
        let s_73_0: u32 = 102552;
        // D s_73_1: read-reg s_73_0:struct
        let s_73_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_73_0 as isize);
            tracer.read_register(s_73_0 as isize, value);
            value
        };
        // D s_73_2: call _get_HCR_EL2_Type_E2H(s_73_1)
        let s_73_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_73_1);
        // C s_73_3: const #102552u : u32
        let s_73_3: u32 = 102552;
        // D s_73_4: read-reg s_73_3:struct
        let s_73_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_73_3 as isize);
            tracer.read_register(s_73_3 as isize, value);
            value
        };
        // D s_73_5: call _get_HCR_EL2_Type_TGE(s_73_4)
        let s_73_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_73_4);
        // D s_73_6: cast zx s_73_2 -> bv
        let s_73_6: Bits = Bits::new(s_73_2 as u128, 1u16);
        // D s_73_7: cast zx s_73_5 -> bv
        let s_73_7: Bits = Bits::new(s_73_5 as u128, 1u16);
        // D s_73_8: cast reint s_73_6 -> u128
        let s_73_8: u128 = (s_73_6.value() as u128);
        // D s_73_9: size-of s_73_6
        let s_73_9: u16 = s_73_6.length();
        // D s_73_10: cast reint s_73_7 -> u128
        let s_73_10: u128 = (s_73_7.value() as u128);
        // D s_73_11: size-of s_73_7
        let s_73_11: u16 = s_73_7.length();
        // D s_73_12: lsl s_73_8 s_73_11
        let s_73_12: u128 = s_73_8 << s_73_11;
        // D s_73_13: or s_73_12 s_73_10
        let s_73_13: u128 = ((s_73_12) | (s_73_10));
        // D s_73_14: add s_73_9 s_73_11
        let s_73_14: u16 = (s_73_9 + s_73_11);
        // D s_73_15: create-bits s_73_13 s_73_14
        let s_73_15: Bits = Bits::new(s_73_13, s_73_14);
        // D s_73_16: cast reint s_73_15 -> u8
        let s_73_16: u8 = (s_73_15.value() as u8);
        // D s_73_17: cast zx s_73_16 -> bv
        let s_73_17: Bits = Bits::new(s_73_16 as u128, 2u16);
        // C s_73_18: const #3u : u8
        let s_73_18: u8 = 3;
        // C s_73_19: cast zx s_73_18 -> bv
        let s_73_19: Bits = Bits::new(s_73_18 as u128, 2u16);
        // D s_73_20: cmp-eq s_73_17 s_73_19
        let s_73_20: bool = ((s_73_17) == (s_73_19));
        // D s_73_21: write-var gs#101768 <= s_73_20
        fn_state.gs_101768 = s_73_20;
        // N s_73_22: jump b29
        return block_29(state, tracer, fn_state);
    }
}
