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
use u_get_SCTLR_EL2_Type_TIDCP::*;
use AArch64_ImpDefSysRegRead::*;
use u_get_SCTLR_EL1_Type_TIDCP::*;
use u_get_HCR_EL2_Type_E2H::*;
use u_get_HCR_EL2_Type_TIDCP::*;
use AArch64_SystemAccessTrap::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use common::*;
pub fn S3_op1_Cn_Cm_op2_SysRegRead_9e206a297d1c5480<T: Tracer>(
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
        u__HCR_EL2_TGE: bool,
        gs_69134: bool,
        gs_69132: bool,
        gs_69136: bool,
        u__HCR_EL2_TIDCP: bool,
        u__PSTATE_EL: u8,
        gs_69133: bool,
        gs_69137: bool,
        u__SCTLR_EL2_TIDCP: bool,
        u__SCTLR_EL1_TIDCP: bool,
        gs_69135: bool,
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
        // D s_0_5: call _get_SCTLR_EL1_Type_TIDCP(s_0_4)
        let s_0_5: bool = u_get_SCTLR_EL1_Type_TIDCP(state, tracer, s_0_4);
        // D s_0_6: write-var __SCTLR_EL1_TIDCP <= s_0_5
        fn_state.u__SCTLR_EL1_TIDCP = s_0_5;
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
        // C s_0_11: const #20784u : u32
        let s_0_11: u32 = 20784;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_SCTLR_EL2_Type_TIDCP(s_0_12)
        let s_0_13: bool = u_get_SCTLR_EL2_Type_TIDCP(state, tracer, s_0_12);
        // D s_0_14: write-var __SCTLR_EL2_TIDCP <= s_0_13
        fn_state.u__SCTLR_EL2_TIDCP = s_0_13;
        // C s_0_15: const #102552u : u32
        let s_0_15: u32 = 102552;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_HCR_EL2_Type_TIDCP(s_0_16)
        let s_0_17: bool = u_get_HCR_EL2_Type_TIDCP(state, tracer, s_0_16);
        // D s_0_18: write-var __HCR_EL2_TIDCP <= s_0_17
        fn_state.u__HCR_EL2_TIDCP = s_0_17;
        // D s_0_19: read-var __PSTATE_EL:u8
        let s_0_19: u8 = fn_state.u__PSTATE_EL;
        // D s_0_20: cast zx s_0_19 -> bv
        let s_0_20: Bits = Bits::new(s_0_19 as u128, 2u16);
        // C s_0_21: const #448u : u32
        let s_0_21: u32 = 448;
        // D s_0_22: read-reg s_0_21:u8
        let s_0_22: u8 = {
            let value = state.read_register::<u8>(s_0_21 as isize);
            tracer.read_register(s_0_21 as isize, value);
            value
        };
        // D s_0_23: cast zx s_0_22 -> bv
        let s_0_23: Bits = Bits::new(s_0_22 as u128, 2u16);
        // D s_0_24: cmp-eq s_0_20 s_0_23
        let s_0_24: bool = ((s_0_20) == (s_0_23));
        // N s_0_25: branch s_0_24 b13 b1
        if s_0_24 {
            return block_13(state, tracer, fn_state);
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
        // D s_5_0: read-var op0:u8
        let s_5_0: u8 = fn_state.op0;
        // D s_5_1: read-var op1:u8
        let s_5_1: u8 = fn_state.op1;
        // D s_5_2: read-var CRn:u8
        let s_5_2: u8 = fn_state.CRn;
        // D s_5_3: read-var CRm:u8
        let s_5_3: u8 = fn_state.CRm;
        // D s_5_4: read-var op2:u8
        let s_5_4: u8 = fn_state.op2;
        // D s_5_5: read-var t:i
        let s_5_5: i128 = fn_state.t;
        // D s_5_6: call AArch64_ImpDefSysRegRead(s_5_0, s_5_1, s_5_2, s_5_3, s_5_4, s_5_5)
        let s_5_6: () = AArch64_ImpDefSysRegRead(
            state,
            tracer,
            s_5_0,
            s_5_1,
            s_5_2,
            s_5_3,
            s_5_4,
            s_5_5,
        );
        // N s_5_7: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var op0:u8
        let s_6_0: u8 = fn_state.op0;
        // D s_6_1: read-var op1:u8
        let s_6_1: u8 = fn_state.op1;
        // D s_6_2: read-var CRn:u8
        let s_6_2: u8 = fn_state.CRn;
        // D s_6_3: read-var CRm:u8
        let s_6_3: u8 = fn_state.CRm;
        // D s_6_4: read-var op2:u8
        let s_6_4: u8 = fn_state.op2;
        // D s_6_5: read-var t:i
        let s_6_5: i128 = fn_state.t;
        // D s_6_6: call AArch64_ImpDefSysRegRead(s_6_0, s_6_1, s_6_2, s_6_3, s_6_4, s_6_5)
        let s_6_6: () = AArch64_ImpDefSysRegRead(
            state,
            tracer,
            s_6_0,
            s_6_1,
            s_6_2,
            s_6_3,
            s_6_4,
            s_6_5,
        );
        // N s_6_7: return
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
        // N s_7_2: branch s_7_1 b12 b8
        if s_7_1 {
            return block_12(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#69132 <= s_8_0
        fn_state.gs_69132 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#69132:u8
        let s_9_0: bool = fn_state.gs_69132;
        // N s_9_1: branch s_9_0 b11 b10
        if s_9_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var op0:u8
        let s_10_0: u8 = fn_state.op0;
        // D s_10_1: read-var op1:u8
        let s_10_1: u8 = fn_state.op1;
        // D s_10_2: read-var CRn:u8
        let s_10_2: u8 = fn_state.CRn;
        // D s_10_3: read-var CRm:u8
        let s_10_3: u8 = fn_state.CRm;
        // D s_10_4: read-var op2:u8
        let s_10_4: u8 = fn_state.op2;
        // D s_10_5: read-var t:i
        let s_10_5: i128 = fn_state.t;
        // D s_10_6: call AArch64_ImpDefSysRegRead(s_10_0, s_10_1, s_10_2, s_10_3, s_10_4, s_10_5)
        let s_10_6: () = AArch64_ImpDefSysRegRead(
            state,
            tracer,
            s_10_0,
            s_10_1,
            s_10_2,
            s_10_3,
            s_10_4,
            s_10_5,
        );
        // N s_10_7: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #24u : u8
        let s_11_0: u8 = 24;
        // C s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 8u16);
        // C s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (s_11_1.value() as i128);
        // C s_11_3: cast reint s_11_2 -> i64
        let s_11_3: i64 = (s_11_2 as i64);
        // C s_11_4: cast zx s_11_3 -> i
        let s_11_4: i128 = (i128::try_from(s_11_3).unwrap());
        // C s_11_5: const #432u : u32
        let s_11_5: u32 = 432;
        // D s_11_6: read-reg s_11_5:u8
        let s_11_6: u8 = {
            let value = state.read_register::<u8>(s_11_5 as isize);
            tracer.read_register(s_11_5 as isize, value);
            value
        };
        // D s_11_7: call AArch64_SystemAccessTrap(s_11_6, s_11_4)
        let s_11_7: () = AArch64_SystemAccessTrap(state, tracer, s_11_6, s_11_4);
        // N s_11_8: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var __HCR_EL2_TIDCP:u8
        let s_12_0: bool = fn_state.u__HCR_EL2_TIDCP;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 1u16);
        // C s_12_2: const #1u : u8
        let s_12_2: bool = true;
        // C s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 1u16);
        // D s_12_4: cmp-eq s_12_1 s_12_3
        let s_12_4: bool = ((s_12_1) == (s_12_3));
        // D s_12_5: write-var gs#69132 <= s_12_4
        fn_state.gs_69132 = s_12_4;
        // N s_12_6: jump b9
        return block_9(state, tracer, fn_state);
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
        // N s_13_2: branch s_13_1 b34 b14
        if s_13_1 {
            return block_34(state, tracer, fn_state);
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
        // D s_14_1: write-var gs#69133 <= s_14_0
        fn_state.gs_69133 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#69133:u8
        let s_15_0: bool = fn_state.gs_69133;
        // D s_15_1: not s_15_0
        let s_15_1: bool = !s_15_0;
        // N s_15_2: branch s_15_1 b33 b16
        if s_15_1 {
            return block_33(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#69134 <= s_16_0
        fn_state.gs_69134 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#69134:u8
        let s_17_0: bool = fn_state.gs_69134;
        // N s_17_1: branch s_17_0 b27 b18
        if s_17_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call EL2Enabled(s_18_0)
        let s_18_1: bool = EL2Enabled(state, tracer, s_18_0);
        // N s_18_2: branch s_18_1 b26 b19
        if s_18_1 {
            return block_26(state, tracer, fn_state);
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
        // D s_19_1: write-var gs#69135 <= s_19_0
        fn_state.gs_69135 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#69135:u8
        let s_20_0: bool = fn_state.gs_69135;
        // N s_20_1: branch s_20_0 b25 b21
        if s_20_0 {
            return block_25(state, tracer, fn_state);
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
        // D s_21_1: write-var gs#69136 <= s_21_0
        fn_state.gs_69136 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#69136:u8
        let s_22_0: bool = fn_state.gs_69136;
        // N s_22_1: branch s_22_0 b24 b23
        if s_22_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var op0:u8
        let s_23_0: u8 = fn_state.op0;
        // D s_23_1: read-var op1:u8
        let s_23_1: u8 = fn_state.op1;
        // D s_23_2: read-var CRn:u8
        let s_23_2: u8 = fn_state.CRn;
        // D s_23_3: read-var CRm:u8
        let s_23_3: u8 = fn_state.CRm;
        // D s_23_4: read-var op2:u8
        let s_23_4: u8 = fn_state.op2;
        // D s_23_5: read-var t:i
        let s_23_5: i128 = fn_state.t;
        // D s_23_6: call AArch64_ImpDefSysRegRead(s_23_0, s_23_1, s_23_2, s_23_3, s_23_4, s_23_5)
        let s_23_6: () = AArch64_ImpDefSysRegRead(
            state,
            tracer,
            s_23_0,
            s_23_1,
            s_23_2,
            s_23_3,
            s_23_4,
            s_23_5,
        );
        // N s_23_7: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #24u : u8
        let s_24_0: u8 = 24;
        // C s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 8u16);
        // C s_24_2: cast zx s_24_1 -> i
        let s_24_2: i128 = (s_24_1.value() as i128);
        // C s_24_3: cast reint s_24_2 -> i64
        let s_24_3: i64 = (s_24_2 as i64);
        // C s_24_4: cast zx s_24_3 -> i
        let s_24_4: i128 = (i128::try_from(s_24_3).unwrap());
        // C s_24_5: const #432u : u32
        let s_24_5: u32 = 432;
        // D s_24_6: read-reg s_24_5:u8
        let s_24_6: u8 = {
            let value = state.read_register::<u8>(s_24_5 as isize);
            tracer.read_register(s_24_5 as isize, value);
            value
        };
        // D s_24_7: call AArch64_SystemAccessTrap(s_24_6, s_24_4)
        let s_24_7: () = AArch64_SystemAccessTrap(state, tracer, s_24_6, s_24_4);
        // N s_24_8: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var __SCTLR_EL2_TIDCP:u8
        let s_25_0: bool = fn_state.u__SCTLR_EL2_TIDCP;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 1u16);
        // C s_25_2: const #1u : u8
        let s_25_2: bool = true;
        // C s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // D s_25_4: cmp-eq s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) == (s_25_3));
        // D s_25_5: write-var gs#69136 <= s_25_4
        fn_state.gs_69136 = s_25_4;
        // N s_25_6: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #102552u : u32
        let s_26_0: u32 = 102552;
        // D s_26_1: read-reg s_26_0:struct
        let s_26_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_26_0 as isize);
            tracer.read_register(s_26_0 as isize, value);
            value
        };
        // D s_26_2: call _get_HCR_EL2_Type_E2H(s_26_1)
        let s_26_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_26_1);
        // C s_26_3: const #102552u : u32
        let s_26_3: u32 = 102552;
        // D s_26_4: read-reg s_26_3:struct
        let s_26_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_26_3 as isize);
            tracer.read_register(s_26_3 as isize, value);
            value
        };
        // D s_26_5: call _get_HCR_EL2_Type_TGE(s_26_4)
        let s_26_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_26_4);
        // D s_26_6: cast zx s_26_2 -> bv
        let s_26_6: Bits = Bits::new(s_26_2 as u128, 1u16);
        // D s_26_7: cast zx s_26_5 -> bv
        let s_26_7: Bits = Bits::new(s_26_5 as u128, 1u16);
        // D s_26_8: cast reint s_26_6 -> u128
        let s_26_8: u128 = (s_26_6.value() as u128);
        // D s_26_9: size-of s_26_6
        let s_26_9: u16 = s_26_6.length();
        // D s_26_10: cast reint s_26_7 -> u128
        let s_26_10: u128 = (s_26_7.value() as u128);
        // D s_26_11: size-of s_26_7
        let s_26_11: u16 = s_26_7.length();
        // D s_26_12: lsl s_26_8 s_26_11
        let s_26_12: u128 = s_26_8 << s_26_11;
        // D s_26_13: or s_26_12 s_26_10
        let s_26_13: u128 = ((s_26_12) | (s_26_10));
        // D s_26_14: add s_26_9 s_26_11
        let s_26_14: u16 = (s_26_9 + s_26_11);
        // D s_26_15: create-bits s_26_13 s_26_14
        let s_26_15: Bits = Bits::new(s_26_13, s_26_14);
        // D s_26_16: cast reint s_26_15 -> u8
        let s_26_16: u8 = (s_26_15.value() as u8);
        // D s_26_17: cast zx s_26_16 -> bv
        let s_26_17: Bits = Bits::new(s_26_16 as u128, 2u16);
        // C s_26_18: const #3u : u8
        let s_26_18: u8 = 3;
        // C s_26_19: cast zx s_26_18 -> bv
        let s_26_19: Bits = Bits::new(s_26_18 as u128, 2u16);
        // D s_26_20: cmp-eq s_26_17 s_26_19
        let s_26_20: bool = ((s_26_17) == (s_26_19));
        // D s_26_21: write-var gs#69135 <= s_26_20
        fn_state.gs_69135 = s_26_20;
        // N s_26_22: jump b20
        return block_20(state, tracer, fn_state);
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
        // N s_27_2: branch s_27_1 b32 b28
        if s_27_1 {
            return block_32(state, tracer, fn_state);
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
        // D s_28_1: write-var gs#69137 <= s_28_0
        fn_state.gs_69137 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#69137:u8
        let s_29_0: bool = fn_state.gs_69137;
        // N s_29_1: branch s_29_0 b31 b30
        if s_29_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
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
        // C s_30_5: const #440u : u32
        let s_30_5: u32 = 440;
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
        // C s_31_0: const #24u : u8
        let s_31_0: u8 = 24;
        // C s_31_1: cast zx s_31_0 -> bv
        let s_31_1: Bits = Bits::new(s_31_0 as u128, 8u16);
        // C s_31_2: cast zx s_31_1 -> i
        let s_31_2: i128 = (s_31_1.value() as i128);
        // C s_31_3: cast reint s_31_2 -> i64
        let s_31_3: i64 = (s_31_2 as i64);
        // C s_31_4: cast zx s_31_3 -> i
        let s_31_4: i128 = (i128::try_from(s_31_3).unwrap());
        // C s_31_5: const #432u : u32
        let s_31_5: u32 = 432;
        // D s_31_6: read-reg s_31_5:u8
        let s_31_6: u8 = {
            let value = state.read_register::<u8>(s_31_5 as isize);
            tracer.read_register(s_31_5 as isize, value);
            value
        };
        // D s_31_7: call AArch64_SystemAccessTrap(s_31_6, s_31_4)
        let s_31_7: () = AArch64_SystemAccessTrap(state, tracer, s_31_6, s_31_4);
        // N s_31_8: return
        return;
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var __HCR_EL2_TGE:u8
        let s_32_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 1u16);
        // C s_32_2: const #1u : u8
        let s_32_2: bool = true;
        // C s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 1u16);
        // D s_32_4: cmp-eq s_32_1 s_32_3
        let s_32_4: bool = ((s_32_1) == (s_32_3));
        // D s_32_5: write-var gs#69137 <= s_32_4
        fn_state.gs_69137 = s_32_4;
        // N s_32_6: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var __SCTLR_EL1_TIDCP:u8
        let s_33_0: bool = fn_state.u__SCTLR_EL1_TIDCP;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 1u16);
        // C s_33_2: const #1u : u8
        let s_33_2: bool = true;
        // C s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 1u16);
        // D s_33_4: cmp-eq s_33_1 s_33_3
        let s_33_4: bool = ((s_33_1) == (s_33_3));
        // D s_33_5: write-var gs#69134 <= s_33_4
        fn_state.gs_69134 = s_33_4;
        // N s_33_6: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #102552u : u32
        let s_34_0: u32 = 102552;
        // D s_34_1: read-reg s_34_0:struct
        let s_34_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_34_0 as isize);
            tracer.read_register(s_34_0 as isize, value);
            value
        };
        // D s_34_2: call _get_HCR_EL2_Type_E2H(s_34_1)
        let s_34_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_34_1);
        // C s_34_3: const #102552u : u32
        let s_34_3: u32 = 102552;
        // D s_34_4: read-reg s_34_3:struct
        let s_34_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_34_3 as isize);
            tracer.read_register(s_34_3 as isize, value);
            value
        };
        // D s_34_5: call _get_HCR_EL2_Type_TGE(s_34_4)
        let s_34_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_34_4);
        // D s_34_6: cast zx s_34_2 -> bv
        let s_34_6: Bits = Bits::new(s_34_2 as u128, 1u16);
        // D s_34_7: cast zx s_34_5 -> bv
        let s_34_7: Bits = Bits::new(s_34_5 as u128, 1u16);
        // D s_34_8: cast reint s_34_6 -> u128
        let s_34_8: u128 = (s_34_6.value() as u128);
        // D s_34_9: size-of s_34_6
        let s_34_9: u16 = s_34_6.length();
        // D s_34_10: cast reint s_34_7 -> u128
        let s_34_10: u128 = (s_34_7.value() as u128);
        // D s_34_11: size-of s_34_7
        let s_34_11: u16 = s_34_7.length();
        // D s_34_12: lsl s_34_8 s_34_11
        let s_34_12: u128 = s_34_8 << s_34_11;
        // D s_34_13: or s_34_12 s_34_10
        let s_34_13: u128 = ((s_34_12) | (s_34_10));
        // D s_34_14: add s_34_9 s_34_11
        let s_34_14: u16 = (s_34_9 + s_34_11);
        // D s_34_15: create-bits s_34_13 s_34_14
        let s_34_15: Bits = Bits::new(s_34_13, s_34_14);
        // D s_34_16: cast reint s_34_15 -> u8
        let s_34_16: u8 = (s_34_15.value() as u8);
        // D s_34_17: cast zx s_34_16 -> bv
        let s_34_17: Bits = Bits::new(s_34_16 as u128, 2u16);
        // C s_34_18: const #3u : u8
        let s_34_18: u8 = 3;
        // C s_34_19: cast zx s_34_18 -> bv
        let s_34_19: Bits = Bits::new(s_34_18 as u128, 2u16);
        // D s_34_20: cmp-eq s_34_17 s_34_19
        let s_34_20: bool = ((s_34_17) == (s_34_19));
        // D s_34_21: write-var gs#69133 <= s_34_20
        fn_state.gs_69133 = s_34_20;
        // N s_34_22: jump b15
        return block_15(state, tracer, fn_state);
    }
}
