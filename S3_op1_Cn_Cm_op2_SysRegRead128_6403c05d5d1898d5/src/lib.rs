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
use IsSCTLR2EL2Enabled::*;
use u_get_HCR_EL2_Type_E2H::*;
use Halted::*;
use u_get_SCTLR2_EL2_Type_EnIDCP128::*;
use u__IMPDEF_boolean::*;
use u_get_HCRX_EL2_Type_EnIDCP128::*;
use AArch64_SystemAccessTrap::*;
use IsSCTLR2EL1Enabled::*;
use u_get_SCTLR2_EL1_Type_EnIDCP128::*;
use u_get_SCTLR_EL1_Type_TIDCP::*;
use u_get_SCR_EL3_Type_EnIDCP128::*;
use IsHCRXEL2Enabled::*;
use u_get_HCR_EL2_Type_TIDCP::*;
use u_get_EDSCR_Type_SDD::*;
use AArch64_ImpDefSysRegRead128::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use EDSCR_read::*;
use common::*;
pub fn S3_op1_Cn_Cm_op2_SysRegRead128_6403c05d5d1898d5<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    op0: u8,
    op1: u8,
    CRn: u8,
    op2: u8,
    CRm: u8,
    t: i128,
    t2: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_137251: bool,
        gs_137228: bool,
        u__HCRX_EL2_EnIDCP128: bool,
        gs_137237: bool,
        gs_137222: bool,
        gs_137256: bool,
        u__SCR_EL3_EnIDCP128: bool,
        gs_137236: bool,
        gs_137239: bool,
        gs_137227: bool,
        gs_137240: bool,
        u__HCR_EL2_TIDCP: bool,
        gs_137233: bool,
        u__SCTLR_EL2_TIDCP: bool,
        u__SCTLR_EL1_TIDCP: bool,
        gs_137241: bool,
        gs_137221: bool,
        gs_137230: bool,
        gs_137257: bool,
        gs_137219: bool,
        gs_137242: bool,
        gs_137244: bool,
        gs_137243: bool,
        gs_137255: bool,
        u__PSTATE_EL: u8,
        gs_137248: bool,
        gs_137220: bool,
        gs_137235: bool,
        gs_137250: bool,
        u__HCR_EL2_TGE: bool,
        u__EDSCR_SDD: bool,
        gs_137225: bool,
        gs_137223: bool,
        u__SCTLR2_EL1_EnIDCP128: bool,
        gs_137253: bool,
        gs_137245: bool,
        gs_137229: bool,
        gs_137252: bool,
        gs_137226: bool,
        gs_137231: bool,
        gs_137238: bool,
        gs_137246: bool,
        gs_137232: bool,
        gs_137249: bool,
        u__SCTLR2_EL2_EnIDCP128: bool,
        gs_137247: bool,
        el: u8,
        op0: u8,
        op1: u8,
        CRn: u8,
        op2: u8,
        CRm: u8,
        t: i128,
        t2: i128,
    }
    let fn_state = FunctionState {
        el,
        op0,
        op1,
        CRn,
        op2,
        CRm,
        t,
        t2,
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
        // C s_0_3: const #() : ()
        let s_0_3: () = ();
        // S s_0_4: call EDSCR_read(s_0_3)
        let s_0_4: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_0_3);
        // S s_0_5: call _get_EDSCR_Type_SDD(s_0_4)
        let s_0_5: bool = u_get_EDSCR_Type_SDD(state, tracer, s_0_4);
        // D s_0_6: write-var __EDSCR_SDD <= s_0_5
        fn_state.u__EDSCR_SDD = s_0_5;
        // C s_0_7: const #90704u : u32
        let s_0_7: u32 = 90704;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_SCR_EL3_Type_EnIDCP128(s_0_8)
        let s_0_9: bool = u_get_SCR_EL3_Type_EnIDCP128(state, tracer, s_0_8);
        // D s_0_10: write-var __SCR_EL3_EnIDCP128 <= s_0_9
        fn_state.u__SCR_EL3_EnIDCP128 = s_0_9;
        // C s_0_11: const #90272u : u32
        let s_0_11: u32 = 90272;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_SCTLR_EL1_Type_TIDCP(s_0_12)
        let s_0_13: bool = u_get_SCTLR_EL1_Type_TIDCP(state, tracer, s_0_12);
        // D s_0_14: write-var __SCTLR_EL1_TIDCP <= s_0_13
        fn_state.u__SCTLR_EL1_TIDCP = s_0_13;
        // C s_0_15: const #102552u : u32
        let s_0_15: u32 = 102552;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_HCR_EL2_Type_TGE(s_0_16)
        let s_0_17: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_0_16);
        // D s_0_18: write-var __HCR_EL2_TGE <= s_0_17
        fn_state.u__HCR_EL2_TGE = s_0_17;
        // C s_0_19: const #20784u : u32
        let s_0_19: u32 = 20784;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_SCTLR_EL2_Type_TIDCP(s_0_20)
        let s_0_21: bool = u_get_SCTLR_EL2_Type_TIDCP(state, tracer, s_0_20);
        // D s_0_22: write-var __SCTLR_EL2_TIDCP <= s_0_21
        fn_state.u__SCTLR_EL2_TIDCP = s_0_21;
        // C s_0_23: const #11720u : u32
        let s_0_23: u32 = 11720;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_SCTLR2_EL1_Type_EnIDCP128(s_0_24)
        let s_0_25: bool = u_get_SCTLR2_EL1_Type_EnIDCP128(state, tracer, s_0_24);
        // D s_0_26: write-var __SCTLR2_EL1_EnIDCP128 <= s_0_25
        fn_state.u__SCTLR2_EL1_EnIDCP128 = s_0_25;
        // C s_0_27: const #102680u : u32
        let s_0_27: u32 = 102680;
        // D s_0_28: read-reg s_0_27:struct
        let s_0_28: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_27 as isize);
            tracer.read_register(s_0_27 as isize, value);
            value
        };
        // D s_0_29: call _get_SCTLR2_EL2_Type_EnIDCP128(s_0_28)
        let s_0_29: bool = u_get_SCTLR2_EL2_Type_EnIDCP128(state, tracer, s_0_28);
        // D s_0_30: write-var __SCTLR2_EL2_EnIDCP128 <= s_0_29
        fn_state.u__SCTLR2_EL2_EnIDCP128 = s_0_29;
        // C s_0_31: const #22528u : u32
        let s_0_31: u32 = 22528;
        // D s_0_32: read-reg s_0_31:struct
        let s_0_32: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_31 as isize);
            tracer.read_register(s_0_31 as isize, value);
            value
        };
        // D s_0_33: call _get_HCRX_EL2_Type_EnIDCP128(s_0_32)
        let s_0_33: bool = u_get_HCRX_EL2_Type_EnIDCP128(state, tracer, s_0_32);
        // D s_0_34: write-var __HCRX_EL2_EnIDCP128 <= s_0_33
        fn_state.u__HCRX_EL2_EnIDCP128 = s_0_33;
        // C s_0_35: const #102552u : u32
        let s_0_35: u32 = 102552;
        // D s_0_36: read-reg s_0_35:struct
        let s_0_36: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_35 as isize);
            tracer.read_register(s_0_35 as isize, value);
            value
        };
        // D s_0_37: call _get_HCR_EL2_Type_TIDCP(s_0_36)
        let s_0_37: bool = u_get_HCR_EL2_Type_TIDCP(state, tracer, s_0_36);
        // D s_0_38: write-var __HCR_EL2_TIDCP <= s_0_37
        fn_state.u__HCR_EL2_TIDCP = s_0_37;
        // D s_0_39: read-var __PSTATE_EL:u8
        let s_0_39: u8 = fn_state.u__PSTATE_EL;
        // D s_0_40: cast zx s_0_39 -> bv
        let s_0_40: Bits = Bits::new(s_0_39 as u128, 2u16);
        // C s_0_41: const #448u : u32
        let s_0_41: u32 = 448;
        // D s_0_42: read-reg s_0_41:u8
        let s_0_42: u8 = {
            let value = state.read_register::<u8>(s_0_41 as isize);
            tracer.read_register(s_0_41 as isize, value);
            value
        };
        // D s_0_43: cast zx s_0_42 -> bv
        let s_0_43: Bits = Bits::new(s_0_42 as u128, 2u16);
        // D s_0_44: cmp-eq s_0_40 s_0_43
        let s_0_44: bool = ((s_0_40) == (s_0_43));
        // N s_0_45: branch s_0_44 b69 b1
        if s_0_44 {
            return block_69(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b31 b2
        if s_1_5 {
            return block_31(state, tracer, fn_state);
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
        // C s_5_0: const #1s : i
        let s_5_0: i128 = 1;
        // D s_5_1: read-var t:i
        let s_5_1: i128 = fn_state.t;
        // D s_5_2: add s_5_1 s_5_0
        let s_5_2: i128 = (s_5_1 + s_5_0);
        // D s_5_3: read-var op0:u8
        let s_5_3: u8 = fn_state.op0;
        // D s_5_4: read-var op1:u8
        let s_5_4: u8 = fn_state.op1;
        // D s_5_5: read-var CRn:u8
        let s_5_5: u8 = fn_state.CRn;
        // D s_5_6: read-var CRm:u8
        let s_5_6: u8 = fn_state.CRm;
        // D s_5_7: read-var op2:u8
        let s_5_7: u8 = fn_state.op2;
        // D s_5_8: read-var t:i
        let s_5_8: i128 = fn_state.t;
        // D s_5_9: call AArch64_ImpDefSysRegRead128(s_5_3, s_5_4, s_5_5, s_5_6, s_5_7, s_5_8, s_5_2)
        let s_5_9: () = AArch64_ImpDefSysRegRead128(
            state,
            tracer,
            s_5_3,
            s_5_4,
            s_5_5,
            s_5_6,
            s_5_7,
            s_5_8,
            s_5_2,
        );
        // N s_5_10: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call Halted(s_6_0)
        let s_6_1: bool = Halted(state, tracer, s_6_0);
        // N s_6_2: branch s_6_1 b30 b7
        if s_6_1 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#137219 <= s_7_0
        fn_state.gs_137219 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#137219:u8
        let s_8_0: bool = fn_state.gs_137219;
        // N s_8_1: branch s_8_0 b29 b9
        if s_8_0 {
            return block_29(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#137220 <= s_9_0
        fn_state.gs_137220 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#137220:u8
        let s_10_0: bool = fn_state.gs_137220;
        // N s_10_1: branch s_10_0 b28 b11
        if s_10_0 {
            return block_28(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#137221 <= s_11_0
        fn_state.gs_137221 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#137221:u8
        let s_12_0: bool = fn_state.gs_137221;
        // N s_12_1: branch s_12_0 b27 b13
        if s_12_0 {
            return block_27(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#137222 <= s_13_0
        fn_state.gs_137222 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#137222:u8
        let s_14_0: bool = fn_state.gs_137222;
        // N s_14_1: branch s_14_0 b26 b15
        if s_14_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #424u : u32
        let s_15_0: u32 = 424;
        // D s_15_1: read-reg s_15_0:u8
        let s_15_1: u8 = {
            let value = state.read_register::<u8>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // C s_15_2: const #2u : u8
        let s_15_2: u8 = 2;
        // D s_15_3: cmp-lt s_15_1 s_15_2
        let s_15_3: bool = ((s_15_1) < (s_15_2));
        // N s_15_4: branch s_15_3 b25 b16
        if s_15_3 {
            return block_25(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#137223 <= s_16_0
        fn_state.gs_137223 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#137223:u8
        let s_17_0: bool = fn_state.gs_137223;
        // N s_17_1: branch s_17_0 b19 b18
        if s_17_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1s : i
        let s_18_0: i128 = 1;
        // D s_18_1: read-var t:i
        let s_18_1: i128 = fn_state.t;
        // D s_18_2: add s_18_1 s_18_0
        let s_18_2: i128 = (s_18_1 + s_18_0);
        // D s_18_3: read-var op0:u8
        let s_18_3: u8 = fn_state.op0;
        // D s_18_4: read-var op1:u8
        let s_18_4: u8 = fn_state.op1;
        // D s_18_5: read-var CRn:u8
        let s_18_5: u8 = fn_state.CRn;
        // D s_18_6: read-var CRm:u8
        let s_18_6: u8 = fn_state.CRm;
        // D s_18_7: read-var op2:u8
        let s_18_7: u8 = fn_state.op2;
        // D s_18_8: read-var t:i
        let s_18_8: i128 = fn_state.t;
        // D s_18_9: call AArch64_ImpDefSysRegRead128(s_18_3, s_18_4, s_18_5, s_18_6, s_18_7, s_18_8, s_18_2)
        let s_18_9: () = AArch64_ImpDefSysRegRead128(
            state,
            tracer,
            s_18_3,
            s_18_4,
            s_18_5,
            s_18_6,
            s_18_7,
            s_18_8,
            s_18_2,
        );
        // N s_18_10: return
        return;
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
        // N s_19_2: branch s_19_1 b24 b20
        if s_19_1 {
            return block_24(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#137225 <= s_20_0
        fn_state.gs_137225 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#137225:u8
        let s_21_0: bool = fn_state.gs_137225;
        // N s_21_1: branch s_21_0 b23 b22
        if s_21_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #20u : u8
        let s_22_0: u8 = 20;
        // C s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 8u16);
        // C s_22_2: cast zx s_22_1 -> i
        let s_22_2: i128 = (s_22_1.value() as i128);
        // C s_22_3: cast reint s_22_2 -> i64
        let s_22_3: i64 = (s_22_2 as i64);
        // C s_22_4: cast zx s_22_3 -> i
        let s_22_4: i128 = (i128::try_from(s_22_3).unwrap());
        // C s_22_5: const #424u : u32
        let s_22_5: u32 = 424;
        // D s_22_6: read-reg s_22_5:u8
        let s_22_6: u8 = {
            let value = state.read_register::<u8>(s_22_5 as isize);
            tracer.read_register(s_22_5 as isize, value);
            value
        };
        // D s_22_7: call AArch64_SystemAccessTrap(s_22_6, s_22_4)
        let s_22_7: () = AArch64_SystemAccessTrap(state, tracer, s_22_6, s_22_4);
        // N s_22_8: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: panic
        panic!("{:?}", ());
        // N s_23_1: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var __EDSCR_SDD:u8
        let s_24_0: bool = fn_state.u__EDSCR_SDD;
        // D s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 1u16);
        // C s_24_2: const #1u : u8
        let s_24_2: bool = true;
        // C s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 1u16);
        // D s_24_4: cmp-eq s_24_1 s_24_3
        let s_24_4: bool = ((s_24_1) == (s_24_3));
        // D s_24_5: write-var gs#137225 <= s_24_4
        fn_state.gs_137225 = s_24_4;
        // N s_24_6: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var __SCR_EL3_EnIDCP128:u8
        let s_25_0: bool = fn_state.u__SCR_EL3_EnIDCP128;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 1u16);
        // C s_25_2: const #0u : u8
        let s_25_2: bool = false;
        // C s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // D s_25_4: cmp-eq s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) == (s_25_3));
        // D s_25_5: write-var gs#137223 <= s_25_4
        fn_state.gs_137223 = s_25_4;
        // N s_25_6: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_26_0: panic
        panic!("{:?}", ());
        // N s_26_1: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var __SCR_EL3_EnIDCP128:u8
        let s_27_0: bool = fn_state.u__SCR_EL3_EnIDCP128;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 1u16);
        // C s_27_2: const #0u : u8
        let s_27_2: bool = false;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: write-var gs#137222 <= s_27_4
        fn_state.gs_137222 = s_27_4;
        // N s_27_6: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_28_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_28_1: call __IMPDEF_boolean(s_28_0)
        let s_28_1: bool = u__IMPDEF_boolean(state, tracer, s_28_0);
        // D s_28_2: write-var gs#137221 <= s_28_1
        fn_state.gs_137221 = s_28_1;
        // N s_28_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var __EDSCR_SDD:u8
        let s_29_0: bool = fn_state.u__EDSCR_SDD;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 1u16);
        // C s_29_2: const #1u : u8
        let s_29_2: bool = true;
        // C s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // D s_29_4: cmp-eq s_29_1 s_29_3
        let s_29_4: bool = ((s_29_1) == (s_29_3));
        // D s_29_5: write-var gs#137220 <= s_29_4
        fn_state.gs_137220 = s_29_4;
        // N s_29_6: jump b10
        return block_10(state, tracer, fn_state);
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
        // D s_30_4: write-var gs#137219 <= s_30_3
        fn_state.gs_137219 = s_30_3;
        // N s_30_5: jump b8
        return block_8(state, tracer, fn_state);
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
        // N s_31_2: branch s_31_1 b68 b32
        if s_31_1 {
            return block_68(state, tracer, fn_state);
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
        // D s_32_1: write-var gs#137226 <= s_32_0
        fn_state.gs_137226 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#137226:u8
        let s_33_0: bool = fn_state.gs_137226;
        // N s_33_1: branch s_33_0 b67 b34
        if s_33_0 {
            return block_67(state, tracer, fn_state);
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
        // D s_34_1: write-var gs#137227 <= s_34_0
        fn_state.gs_137227 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#137227:u8
        let s_35_0: bool = fn_state.gs_137227;
        // N s_35_1: branch s_35_0 b66 b36
        if s_35_0 {
            return block_66(state, tracer, fn_state);
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
        // D s_36_1: write-var gs#137228 <= s_36_0
        fn_state.gs_137228 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#137228:u8
        let s_37_0: bool = fn_state.gs_137228;
        // N s_37_1: branch s_37_0 b65 b38
        if s_37_0 {
            return block_65(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#137229 <= s_38_0
        fn_state.gs_137229 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#137229:u8
        let s_39_0: bool = fn_state.gs_137229;
        // N s_39_1: branch s_39_0 b64 b40
        if s_39_0 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #() : ()
        let s_40_0: () = ();
        // S s_40_1: call EL2Enabled(s_40_0)
        let s_40_1: bool = EL2Enabled(state, tracer, s_40_0);
        // N s_40_2: branch s_40_1 b63 b41
        if s_40_1 {
            return block_63(state, tracer, fn_state);
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
        // D s_41_1: write-var gs#137230 <= s_41_0
        fn_state.gs_137230 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#137230:u8
        let s_42_0: bool = fn_state.gs_137230;
        // N s_42_1: branch s_42_0 b62 b43
        if s_42_0 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #() : ()
        let s_43_0: () = ();
        // S s_43_1: call EL2Enabled(s_43_0)
        let s_43_1: bool = EL2Enabled(state, tracer, s_43_0);
        // N s_43_2: branch s_43_1 b58 b44
        if s_43_1 {
            return block_58(state, tracer, fn_state);
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
        // D s_44_1: write-var gs#137232 <= s_44_0
        fn_state.gs_137232 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#137232:u8
        let s_45_0: bool = fn_state.gs_137232;
        // N s_45_1: branch s_45_0 b57 b46
        if s_45_0 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #424u : u32
        let s_46_0: u32 = 424;
        // D s_46_1: read-reg s_46_0:u8
        let s_46_1: u8 = {
            let value = state.read_register::<u8>(s_46_0 as isize);
            tracer.read_register(s_46_0 as isize, value);
            value
        };
        // C s_46_2: const #2u : u8
        let s_46_2: u8 = 2;
        // D s_46_3: cmp-lt s_46_1 s_46_2
        let s_46_3: bool = ((s_46_1) < (s_46_2));
        // N s_46_4: branch s_46_3 b56 b47
        if s_46_3 {
            return block_56(state, tracer, fn_state);
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
        // D s_47_1: write-var gs#137233 <= s_47_0
        fn_state.gs_137233 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#137233:u8
        let s_48_0: bool = fn_state.gs_137233;
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
        // C s_49_0: const #1s : i
        let s_49_0: i128 = 1;
        // D s_49_1: read-var t:i
        let s_49_1: i128 = fn_state.t;
        // D s_49_2: add s_49_1 s_49_0
        let s_49_2: i128 = (s_49_1 + s_49_0);
        // D s_49_3: read-var op0:u8
        let s_49_3: u8 = fn_state.op0;
        // D s_49_4: read-var op1:u8
        let s_49_4: u8 = fn_state.op1;
        // D s_49_5: read-var CRn:u8
        let s_49_5: u8 = fn_state.CRn;
        // D s_49_6: read-var CRm:u8
        let s_49_6: u8 = fn_state.CRm;
        // D s_49_7: read-var op2:u8
        let s_49_7: u8 = fn_state.op2;
        // D s_49_8: read-var t:i
        let s_49_8: i128 = fn_state.t;
        // D s_49_9: call AArch64_ImpDefSysRegRead128(s_49_3, s_49_4, s_49_5, s_49_6, s_49_7, s_49_8, s_49_2)
        let s_49_9: () = AArch64_ImpDefSysRegRead128(
            state,
            tracer,
            s_49_3,
            s_49_4,
            s_49_5,
            s_49_6,
            s_49_7,
            s_49_8,
            s_49_2,
        );
        // N s_49_10: return
        return;
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
        // D s_51_1: write-var gs#137235 <= s_51_0
        fn_state.gs_137235 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#137235:u8
        let s_52_0: bool = fn_state.gs_137235;
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
        // C s_53_0: const #20u : u8
        let s_53_0: u8 = 20;
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
        // D s_53_7: call AArch64_SystemAccessTrap(s_53_6, s_53_4)
        let s_53_7: () = AArch64_SystemAccessTrap(state, tracer, s_53_6, s_53_4);
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
        // D s_55_0: read-var __EDSCR_SDD:u8
        let s_55_0: bool = fn_state.u__EDSCR_SDD;
        // D s_55_1: cast zx s_55_0 -> bv
        let s_55_1: Bits = Bits::new(s_55_0 as u128, 1u16);
        // C s_55_2: const #1u : u8
        let s_55_2: bool = true;
        // C s_55_3: cast zx s_55_2 -> bv
        let s_55_3: Bits = Bits::new(s_55_2 as u128, 1u16);
        // D s_55_4: cmp-eq s_55_1 s_55_3
        let s_55_4: bool = ((s_55_1) == (s_55_3));
        // D s_55_5: write-var gs#137235 <= s_55_4
        fn_state.gs_137235 = s_55_4;
        // N s_55_6: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var __SCR_EL3_EnIDCP128:u8
        let s_56_0: bool = fn_state.u__SCR_EL3_EnIDCP128;
        // D s_56_1: cast zx s_56_0 -> bv
        let s_56_1: Bits = Bits::new(s_56_0 as u128, 1u16);
        // C s_56_2: const #0u : u8
        let s_56_2: bool = false;
        // C s_56_3: cast zx s_56_2 -> bv
        let s_56_3: Bits = Bits::new(s_56_2 as u128, 1u16);
        // D s_56_4: cmp-eq s_56_1 s_56_3
        let s_56_4: bool = ((s_56_1) == (s_56_3));
        // D s_56_5: write-var gs#137233 <= s_56_4
        fn_state.gs_137233 = s_56_4;
        // N s_56_6: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #20u : u8
        let s_57_0: u8 = 20;
        // C s_57_1: cast zx s_57_0 -> bv
        let s_57_1: Bits = Bits::new(s_57_0 as u128, 8u16);
        // C s_57_2: cast zx s_57_1 -> i
        let s_57_2: i128 = (s_57_1.value() as i128);
        // C s_57_3: cast reint s_57_2 -> i64
        let s_57_3: i64 = (s_57_2 as i64);
        // C s_57_4: cast zx s_57_3 -> i
        let s_57_4: i128 = (i128::try_from(s_57_3).unwrap());
        // C s_57_5: const #432u : u32
        let s_57_5: u32 = 432;
        // D s_57_6: read-reg s_57_5:u8
        let s_57_6: u8 = {
            let value = state.read_register::<u8>(s_57_5 as isize);
            tracer.read_register(s_57_5 as isize, value);
            value
        };
        // D s_57_7: call AArch64_SystemAccessTrap(s_57_6, s_57_4)
        let s_57_7: () = AArch64_SystemAccessTrap(state, tracer, s_57_6, s_57_4);
        // N s_57_8: return
        return;
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #() : ()
        let s_58_0: () = ();
        // S s_58_1: call IsHCRXEL2Enabled(s_58_0)
        let s_58_1: bool = IsHCRXEL2Enabled(state, tracer, s_58_0);
        // S s_58_2: not s_58_1
        let s_58_2: bool = !s_58_1;
        // N s_58_3: branch s_58_2 b61 b59
        if s_58_2 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var __HCRX_EL2_EnIDCP128:u8
        let s_59_0: bool = fn_state.u__HCRX_EL2_EnIDCP128;
        // D s_59_1: cast zx s_59_0 -> bv
        let s_59_1: Bits = Bits::new(s_59_0 as u128, 1u16);
        // C s_59_2: const #0u : u8
        let s_59_2: bool = false;
        // C s_59_3: cast zx s_59_2 -> bv
        let s_59_3: Bits = Bits::new(s_59_2 as u128, 1u16);
        // D s_59_4: cmp-eq s_59_1 s_59_3
        let s_59_4: bool = ((s_59_1) == (s_59_3));
        // D s_59_5: write-var gs#137231 <= s_59_4
        fn_state.gs_137231 = s_59_4;
        // N s_59_6: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#137231:u8
        let s_60_0: bool = fn_state.gs_137231;
        // D s_60_1: write-var gs#137232 <= s_60_0
        fn_state.gs_137232 = s_60_0;
        // N s_60_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #1u : u8
        let s_61_0: bool = true;
        // D s_61_1: write-var gs#137231 <= s_61_0
        fn_state.gs_137231 = s_61_0;
        // N s_61_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #20u : u8
        let s_62_0: u8 = 20;
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
        // D s_63_0: read-var __HCR_EL2_TIDCP:u8
        let s_63_0: bool = fn_state.u__HCR_EL2_TIDCP;
        // D s_63_1: cast zx s_63_0 -> bv
        let s_63_1: Bits = Bits::new(s_63_0 as u128, 1u16);
        // C s_63_2: const #1u : u8
        let s_63_2: bool = true;
        // C s_63_3: cast zx s_63_2 -> bv
        let s_63_3: Bits = Bits::new(s_63_2 as u128, 1u16);
        // D s_63_4: cmp-eq s_63_1 s_63_3
        let s_63_4: bool = ((s_63_1) == (s_63_3));
        // D s_63_5: write-var gs#137230 <= s_63_4
        fn_state.gs_137230 = s_63_4;
        // N s_63_6: jump b42
        return block_42(state, tracer, fn_state);
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
        // D s_65_0: read-var __SCR_EL3_EnIDCP128:u8
        let s_65_0: bool = fn_state.u__SCR_EL3_EnIDCP128;
        // D s_65_1: cast zx s_65_0 -> bv
        let s_65_1: Bits = Bits::new(s_65_0 as u128, 1u16);
        // C s_65_2: const #0u : u8
        let s_65_2: bool = false;
        // C s_65_3: cast zx s_65_2 -> bv
        let s_65_3: Bits = Bits::new(s_65_2 as u128, 1u16);
        // D s_65_4: cmp-eq s_65_1 s_65_3
        let s_65_4: bool = ((s_65_1) == (s_65_3));
        // D s_65_5: write-var gs#137229 <= s_65_4
        fn_state.gs_137229 = s_65_4;
        // N s_65_6: jump b39
        return block_39(state, tracer, fn_state);
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
        // D s_66_2: write-var gs#137228 <= s_66_1
        fn_state.gs_137228 = s_66_1;
        // N s_66_3: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var __EDSCR_SDD:u8
        let s_67_0: bool = fn_state.u__EDSCR_SDD;
        // D s_67_1: cast zx s_67_0 -> bv
        let s_67_1: Bits = Bits::new(s_67_0 as u128, 1u16);
        // C s_67_2: const #1u : u8
        let s_67_2: bool = true;
        // C s_67_3: cast zx s_67_2 -> bv
        let s_67_3: Bits = Bits::new(s_67_2 as u128, 1u16);
        // D s_67_4: cmp-eq s_67_1 s_67_3
        let s_67_4: bool = ((s_67_1) == (s_67_3));
        // D s_67_5: write-var gs#137227 <= s_67_4
        fn_state.gs_137227 = s_67_4;
        // N s_67_6: jump b35
        return block_35(state, tracer, fn_state);
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
        // D s_68_4: write-var gs#137226 <= s_68_3
        fn_state.gs_137226 = s_68_3;
        // N s_68_5: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #() : ()
        let s_69_0: () = ();
        // S s_69_1: call Halted(s_69_0)
        let s_69_1: bool = Halted(state, tracer, s_69_0);
        // N s_69_2: branch s_69_1 b152 b70
        if s_69_1 {
            return block_152(state, tracer, fn_state);
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
        // D s_70_1: write-var gs#137236 <= s_70_0
        fn_state.gs_137236 = s_70_0;
        // N s_70_2: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var gs#137236:u8
        let s_71_0: bool = fn_state.gs_137236;
        // N s_71_1: branch s_71_0 b151 b72
        if s_71_0 {
            return block_151(state, tracer, fn_state);
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
        // D s_72_1: write-var gs#137237 <= s_72_0
        fn_state.gs_137237 = s_72_0;
        // N s_72_2: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var gs#137237:u8
        let s_73_0: bool = fn_state.gs_137237;
        // N s_73_1: branch s_73_0 b150 b74
        if s_73_0 {
            return block_150(state, tracer, fn_state);
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
        // D s_74_1: write-var gs#137238 <= s_74_0
        fn_state.gs_137238 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var gs#137238:u8
        let s_75_0: bool = fn_state.gs_137238;
        // N s_75_1: branch s_75_0 b149 b76
        if s_75_0 {
            return block_149(state, tracer, fn_state);
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
        // D s_76_1: write-var gs#137239 <= s_76_0
        fn_state.gs_137239 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var gs#137239:u8
        let s_77_0: bool = fn_state.gs_137239;
        // N s_77_1: branch s_77_0 b148 b78
        if s_77_0 {
            return block_148(state, tracer, fn_state);
        } else {
            return block_78(state, tracer, fn_state);
        };
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #() : ()
        let s_78_0: () = ();
        // S s_78_1: call EL2Enabled(s_78_0)
        let s_78_1: bool = EL2Enabled(state, tracer, s_78_0);
        // N s_78_2: branch s_78_1 b147 b79
        if s_78_1 {
            return block_147(state, tracer, fn_state);
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
        // D s_79_1: write-var gs#137240 <= s_79_0
        fn_state.gs_137240 = s_79_0;
        // N s_79_2: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var gs#137240:u8
        let s_80_0: bool = fn_state.gs_137240;
        // D s_80_1: not s_80_0
        let s_80_1: bool = !s_80_0;
        // N s_80_2: branch s_80_1 b146 b81
        if s_80_1 {
            return block_146(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #0u : u8
        let s_81_0: bool = false;
        // D s_81_1: write-var gs#137241 <= s_81_0
        fn_state.gs_137241 = s_81_0;
        // N s_81_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var gs#137241:u8
        let s_82_0: bool = fn_state.gs_137241;
        // N s_82_1: branch s_82_0 b140 b83
        if s_82_0 {
            return block_140(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #() : ()
        let s_83_0: () = ();
        // S s_83_1: call EL2Enabled(s_83_0)
        let s_83_1: bool = EL2Enabled(state, tracer, s_83_0);
        // N s_83_2: branch s_83_1 b139 b84
        if s_83_1 {
            return block_139(state, tracer, fn_state);
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
        // D s_84_1: write-var gs#137242 <= s_84_0
        fn_state.gs_137242 = s_84_0;
        // N s_84_2: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var gs#137242:u8
        let s_85_0: bool = fn_state.gs_137242;
        // N s_85_1: branch s_85_0 b138 b86
        if s_85_0 {
            return block_138(state, tracer, fn_state);
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
        // D s_86_1: write-var gs#137243 <= s_86_0
        fn_state.gs_137243 = s_86_0;
        // N s_86_2: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var gs#137243:u8
        let s_87_0: bool = fn_state.gs_137243;
        // N s_87_1: branch s_87_0 b137 b88
        if s_87_0 {
            return block_137(state, tracer, fn_state);
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
        // S s_88_1: call EL2Enabled(s_88_0)
        let s_88_1: bool = EL2Enabled(state, tracer, s_88_0);
        // N s_88_2: branch s_88_1 b136 b89
        if s_88_1 {
            return block_136(state, tracer, fn_state);
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
        // D s_89_1: write-var gs#137244 <= s_89_0
        fn_state.gs_137244 = s_89_0;
        // N s_89_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var gs#137244:u8
        let s_90_0: bool = fn_state.gs_137244;
        // D s_90_1: not s_90_0
        let s_90_1: bool = !s_90_0;
        // N s_90_2: branch s_90_1 b132 b91
        if s_90_1 {
            return block_132(state, tracer, fn_state);
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
        // D s_91_1: write-var gs#137246 <= s_91_0
        fn_state.gs_137246 = s_91_0;
        // N s_91_2: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var gs#137246:u8
        let s_92_0: bool = fn_state.gs_137246;
        // N s_92_1: branch s_92_0 b126 b93
        if s_92_0 {
            return block_126(state, tracer, fn_state);
        } else {
            return block_93(state, tracer, fn_state);
        };
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #() : ()
        let s_93_0: () = ();
        // S s_93_1: call EL2Enabled(s_93_0)
        let s_93_1: bool = EL2Enabled(state, tracer, s_93_0);
        // N s_93_2: branch s_93_1 b125 b94
        if s_93_1 {
            return block_125(state, tracer, fn_state);
        } else {
            return block_94(state, tracer, fn_state);
        };
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #0u : u8
        let s_94_0: bool = false;
        // D s_94_1: write-var gs#137247 <= s_94_0
        fn_state.gs_137247 = s_94_0;
        // N s_94_2: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_95_0: read-var gs#137247:u8
        let s_95_0: bool = fn_state.gs_137247;
        // N s_95_1: branch s_95_0 b121 b96
        if s_95_0 {
            return block_121(state, tracer, fn_state);
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
        // D s_96_1: write-var gs#137249 <= s_96_0
        fn_state.gs_137249 = s_96_0;
        // N s_96_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var gs#137249:u8
        let s_97_0: bool = fn_state.gs_137249;
        // N s_97_1: branch s_97_0 b120 b98
        if s_97_0 {
            return block_120(state, tracer, fn_state);
        } else {
            return block_98(state, tracer, fn_state);
        };
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #() : ()
        let s_98_0: () = ();
        // S s_98_1: call EL2Enabled(s_98_0)
        let s_98_1: bool = EL2Enabled(state, tracer, s_98_0);
        // N s_98_2: branch s_98_1 b119 b99
        if s_98_1 {
            return block_119(state, tracer, fn_state);
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
        // D s_99_1: write-var gs#137250 <= s_99_0
        fn_state.gs_137250 = s_99_0;
        // N s_99_2: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_100_0: read-var gs#137250:u8
        let s_100_0: bool = fn_state.gs_137250;
        // N s_100_1: branch s_100_0 b115 b101
        if s_100_0 {
            return block_115(state, tracer, fn_state);
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
        // D s_101_1: write-var gs#137252 <= s_101_0
        fn_state.gs_137252 = s_101_0;
        // N s_101_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var gs#137252:u8
        let s_102_0: bool = fn_state.gs_137252;
        // N s_102_1: branch s_102_0 b114 b103
        if s_102_0 {
            return block_114(state, tracer, fn_state);
        } else {
            return block_103(state, tracer, fn_state);
        };
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #424u : u32
        let s_103_0: u32 = 424;
        // D s_103_1: read-reg s_103_0:u8
        let s_103_1: u8 = {
            let value = state.read_register::<u8>(s_103_0 as isize);
            tracer.read_register(s_103_0 as isize, value);
            value
        };
        // C s_103_2: const #2u : u8
        let s_103_2: u8 = 2;
        // D s_103_3: cmp-lt s_103_1 s_103_2
        let s_103_3: bool = ((s_103_1) < (s_103_2));
        // N s_103_4: branch s_103_3 b113 b104
        if s_103_3 {
            return block_113(state, tracer, fn_state);
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
        // D s_104_1: write-var gs#137253 <= s_104_0
        fn_state.gs_137253 = s_104_0;
        // N s_104_2: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_105_0: read-var gs#137253:u8
        let s_105_0: bool = fn_state.gs_137253;
        // N s_105_1: branch s_105_0 b107 b106
        if s_105_0 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_106(state, tracer, fn_state);
        };
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #1s : i
        let s_106_0: i128 = 1;
        // D s_106_1: read-var t:i
        let s_106_1: i128 = fn_state.t;
        // D s_106_2: add s_106_1 s_106_0
        let s_106_2: i128 = (s_106_1 + s_106_0);
        // D s_106_3: read-var op0:u8
        let s_106_3: u8 = fn_state.op0;
        // D s_106_4: read-var op1:u8
        let s_106_4: u8 = fn_state.op1;
        // D s_106_5: read-var CRn:u8
        let s_106_5: u8 = fn_state.CRn;
        // D s_106_6: read-var CRm:u8
        let s_106_6: u8 = fn_state.CRm;
        // D s_106_7: read-var op2:u8
        let s_106_7: u8 = fn_state.op2;
        // D s_106_8: read-var t:i
        let s_106_8: i128 = fn_state.t;
        // D s_106_9: call AArch64_ImpDefSysRegRead128(s_106_3, s_106_4, s_106_5, s_106_6, s_106_7, s_106_8, s_106_2)
        let s_106_9: () = AArch64_ImpDefSysRegRead128(
            state,
            tracer,
            s_106_3,
            s_106_4,
            s_106_5,
            s_106_6,
            s_106_7,
            s_106_8,
            s_106_2,
        );
        // N s_106_10: return
        return;
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #() : ()
        let s_107_0: () = ();
        // S s_107_1: call Halted(s_107_0)
        let s_107_1: bool = Halted(state, tracer, s_107_0);
        // N s_107_2: branch s_107_1 b112 b108
        if s_107_1 {
            return block_112(state, tracer, fn_state);
        } else {
            return block_108(state, tracer, fn_state);
        };
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #0u : u8
        let s_108_0: bool = false;
        // D s_108_1: write-var gs#137255 <= s_108_0
        fn_state.gs_137255 = s_108_0;
        // N s_108_2: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_109_0: read-var gs#137255:u8
        let s_109_0: bool = fn_state.gs_137255;
        // N s_109_1: branch s_109_0 b111 b110
        if s_109_0 {
            return block_111(state, tracer, fn_state);
        } else {
            return block_110(state, tracer, fn_state);
        };
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_110_0: const #20u : u8
        let s_110_0: u8 = 20;
        // C s_110_1: cast zx s_110_0 -> bv
        let s_110_1: Bits = Bits::new(s_110_0 as u128, 8u16);
        // C s_110_2: cast zx s_110_1 -> i
        let s_110_2: i128 = (s_110_1.value() as i128);
        // C s_110_3: cast reint s_110_2 -> i64
        let s_110_3: i64 = (s_110_2 as i64);
        // C s_110_4: cast zx s_110_3 -> i
        let s_110_4: i128 = (i128::try_from(s_110_3).unwrap());
        // C s_110_5: const #424u : u32
        let s_110_5: u32 = 424;
        // D s_110_6: read-reg s_110_5:u8
        let s_110_6: u8 = {
            let value = state.read_register::<u8>(s_110_5 as isize);
            tracer.read_register(s_110_5 as isize, value);
            value
        };
        // D s_110_7: call AArch64_SystemAccessTrap(s_110_6, s_110_4)
        let s_110_7: () = AArch64_SystemAccessTrap(state, tracer, s_110_6, s_110_4);
        // N s_110_8: return
        return;
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_111_0: panic
        panic!("{:?}", ());
        // N s_111_1: return
        return;
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_112_0: read-var __EDSCR_SDD:u8
        let s_112_0: bool = fn_state.u__EDSCR_SDD;
        // D s_112_1: cast zx s_112_0 -> bv
        let s_112_1: Bits = Bits::new(s_112_0 as u128, 1u16);
        // C s_112_2: const #1u : u8
        let s_112_2: bool = true;
        // C s_112_3: cast zx s_112_2 -> bv
        let s_112_3: Bits = Bits::new(s_112_2 as u128, 1u16);
        // D s_112_4: cmp-eq s_112_1 s_112_3
        let s_112_4: bool = ((s_112_1) == (s_112_3));
        // D s_112_5: write-var gs#137255 <= s_112_4
        fn_state.gs_137255 = s_112_4;
        // N s_112_6: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var __SCR_EL3_EnIDCP128:u8
        let s_113_0: bool = fn_state.u__SCR_EL3_EnIDCP128;
        // D s_113_1: cast zx s_113_0 -> bv
        let s_113_1: Bits = Bits::new(s_113_0 as u128, 1u16);
        // C s_113_2: const #0u : u8
        let s_113_2: bool = false;
        // C s_113_3: cast zx s_113_2 -> bv
        let s_113_3: Bits = Bits::new(s_113_2 as u128, 1u16);
        // D s_113_4: cmp-eq s_113_1 s_113_3
        let s_113_4: bool = ((s_113_1) == (s_113_3));
        // D s_113_5: write-var gs#137253 <= s_113_4
        fn_state.gs_137253 = s_113_4;
        // N s_113_6: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #20u : u8
        let s_114_0: u8 = 20;
        // C s_114_1: cast zx s_114_0 -> bv
        let s_114_1: Bits = Bits::new(s_114_0 as u128, 8u16);
        // C s_114_2: cast zx s_114_1 -> i
        let s_114_2: i128 = (s_114_1.value() as i128);
        // C s_114_3: cast reint s_114_2 -> i64
        let s_114_3: i64 = (s_114_2 as i64);
        // C s_114_4: cast zx s_114_3 -> i
        let s_114_4: i128 = (i128::try_from(s_114_3).unwrap());
        // C s_114_5: const #432u : u32
        let s_114_5: u32 = 432;
        // D s_114_6: read-reg s_114_5:u8
        let s_114_6: u8 = {
            let value = state.read_register::<u8>(s_114_5 as isize);
            tracer.read_register(s_114_5 as isize, value);
            value
        };
        // D s_114_7: call AArch64_SystemAccessTrap(s_114_6, s_114_4)
        let s_114_7: () = AArch64_SystemAccessTrap(state, tracer, s_114_6, s_114_4);
        // N s_114_8: return
        return;
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #() : ()
        let s_115_0: () = ();
        // S s_115_1: call IsHCRXEL2Enabled(s_115_0)
        let s_115_1: bool = IsHCRXEL2Enabled(state, tracer, s_115_0);
        // S s_115_2: not s_115_1
        let s_115_2: bool = !s_115_1;
        // N s_115_3: branch s_115_2 b118 b116
        if s_115_2 {
            return block_118(state, tracer, fn_state);
        } else {
            return block_116(state, tracer, fn_state);
        };
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_116_0: read-var __HCRX_EL2_EnIDCP128:u8
        let s_116_0: bool = fn_state.u__HCRX_EL2_EnIDCP128;
        // D s_116_1: cast zx s_116_0 -> bv
        let s_116_1: Bits = Bits::new(s_116_0 as u128, 1u16);
        // C s_116_2: const #0u : u8
        let s_116_2: bool = false;
        // C s_116_3: cast zx s_116_2 -> bv
        let s_116_3: Bits = Bits::new(s_116_2 as u128, 1u16);
        // D s_116_4: cmp-eq s_116_1 s_116_3
        let s_116_4: bool = ((s_116_1) == (s_116_3));
        // D s_116_5: write-var gs#137251 <= s_116_4
        fn_state.gs_137251 = s_116_4;
        // N s_116_6: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_117_0: read-var gs#137251:u8
        let s_117_0: bool = fn_state.gs_137251;
        // D s_117_1: write-var gs#137252 <= s_117_0
        fn_state.gs_137252 = s_117_0;
        // N s_117_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #1u : u8
        let s_118_0: bool = true;
        // D s_118_1: write-var gs#137251 <= s_118_0
        fn_state.gs_137251 = s_118_0;
        // N s_118_2: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #102552u : u32
        let s_119_0: u32 = 102552;
        // D s_119_1: read-reg s_119_0:struct
        let s_119_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_119_0 as isize);
            tracer.read_register(s_119_0 as isize, value);
            value
        };
        // D s_119_2: call _get_HCR_EL2_Type_E2H(s_119_1)
        let s_119_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_119_1);
        // C s_119_3: const #102552u : u32
        let s_119_3: u32 = 102552;
        // D s_119_4: read-reg s_119_3:struct
        let s_119_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_119_3 as isize);
            tracer.read_register(s_119_3 as isize, value);
            value
        };
        // D s_119_5: call _get_HCR_EL2_Type_TGE(s_119_4)
        let s_119_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_119_4);
        // D s_119_6: cast zx s_119_2 -> bv
        let s_119_6: Bits = Bits::new(s_119_2 as u128, 1u16);
        // D s_119_7: cast zx s_119_5 -> bv
        let s_119_7: Bits = Bits::new(s_119_5 as u128, 1u16);
        // D s_119_8: cast reint s_119_6 -> u128
        let s_119_8: u128 = (s_119_6.value() as u128);
        // D s_119_9: size-of s_119_6
        let s_119_9: u16 = s_119_6.length();
        // D s_119_10: cast reint s_119_7 -> u128
        let s_119_10: u128 = (s_119_7.value() as u128);
        // D s_119_11: size-of s_119_7
        let s_119_11: u16 = s_119_7.length();
        // D s_119_12: lsl s_119_8 s_119_11
        let s_119_12: u128 = s_119_8 << s_119_11;
        // D s_119_13: or s_119_12 s_119_10
        let s_119_13: u128 = ((s_119_12) | (s_119_10));
        // D s_119_14: add s_119_9 s_119_11
        let s_119_14: u16 = (s_119_9 + s_119_11);
        // D s_119_15: create-bits s_119_13 s_119_14
        let s_119_15: Bits = Bits::new(s_119_13, s_119_14);
        // D s_119_16: cast reint s_119_15 -> u8
        let s_119_16: u8 = (s_119_15.value() as u8);
        // D s_119_17: cast zx s_119_16 -> bv
        let s_119_17: Bits = Bits::new(s_119_16 as u128, 2u16);
        // C s_119_18: const #3u : u8
        let s_119_18: u8 = 3;
        // C s_119_19: cast zx s_119_18 -> bv
        let s_119_19: Bits = Bits::new(s_119_18 as u128, 2u16);
        // D s_119_20: cmp-ne s_119_17 s_119_19
        let s_119_20: bool = ((s_119_17) != (s_119_19));
        // D s_119_21: write-var gs#137250 <= s_119_20
        fn_state.gs_137250 = s_119_20;
        // N s_119_22: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_120_0: const #20u : u8
        let s_120_0: u8 = 20;
        // C s_120_1: cast zx s_120_0 -> bv
        let s_120_1: Bits = Bits::new(s_120_0 as u128, 8u16);
        // C s_120_2: cast zx s_120_1 -> i
        let s_120_2: i128 = (s_120_1.value() as i128);
        // C s_120_3: cast reint s_120_2 -> i64
        let s_120_3: i64 = (s_120_2 as i64);
        // C s_120_4: cast zx s_120_3 -> i
        let s_120_4: i128 = (i128::try_from(s_120_3).unwrap());
        // C s_120_5: const #432u : u32
        let s_120_5: u32 = 432;
        // D s_120_6: read-reg s_120_5:u8
        let s_120_6: u8 = {
            let value = state.read_register::<u8>(s_120_5 as isize);
            tracer.read_register(s_120_5 as isize, value);
            value
        };
        // D s_120_7: call AArch64_SystemAccessTrap(s_120_6, s_120_4)
        let s_120_7: () = AArch64_SystemAccessTrap(state, tracer, s_120_6, s_120_4);
        // N s_120_8: return
        return;
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #() : ()
        let s_121_0: () = ();
        // S s_121_1: call IsSCTLR2EL2Enabled(s_121_0)
        let s_121_1: bool = IsSCTLR2EL2Enabled(state, tracer, s_121_0);
        // S s_121_2: not s_121_1
        let s_121_2: bool = !s_121_1;
        // N s_121_3: branch s_121_2 b124 b122
        if s_121_2 {
            return block_124(state, tracer, fn_state);
        } else {
            return block_122(state, tracer, fn_state);
        };
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_122_0: read-var __SCTLR2_EL2_EnIDCP128:u8
        let s_122_0: bool = fn_state.u__SCTLR2_EL2_EnIDCP128;
        // D s_122_1: cast zx s_122_0 -> bv
        let s_122_1: Bits = Bits::new(s_122_0 as u128, 1u16);
        // C s_122_2: const #0u : u8
        let s_122_2: bool = false;
        // C s_122_3: cast zx s_122_2 -> bv
        let s_122_3: Bits = Bits::new(s_122_2 as u128, 1u16);
        // D s_122_4: cmp-eq s_122_1 s_122_3
        let s_122_4: bool = ((s_122_1) == (s_122_3));
        // D s_122_5: write-var gs#137248 <= s_122_4
        fn_state.gs_137248 = s_122_4;
        // N s_122_6: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_123_0: read-var gs#137248:u8
        let s_123_0: bool = fn_state.gs_137248;
        // D s_123_1: write-var gs#137249 <= s_123_0
        fn_state.gs_137249 = s_123_0;
        // N s_123_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #1u : u8
        let s_124_0: bool = true;
        // D s_124_1: write-var gs#137248 <= s_124_0
        fn_state.gs_137248 = s_124_0;
        // N s_124_2: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_125_0: const #102552u : u32
        let s_125_0: u32 = 102552;
        // D s_125_1: read-reg s_125_0:struct
        let s_125_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_125_0 as isize);
            tracer.read_register(s_125_0 as isize, value);
            value
        };
        // D s_125_2: call _get_HCR_EL2_Type_E2H(s_125_1)
        let s_125_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_125_1);
        // C s_125_3: const #102552u : u32
        let s_125_3: u32 = 102552;
        // D s_125_4: read-reg s_125_3:struct
        let s_125_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_125_3 as isize);
            tracer.read_register(s_125_3 as isize, value);
            value
        };
        // D s_125_5: call _get_HCR_EL2_Type_TGE(s_125_4)
        let s_125_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_125_4);
        // D s_125_6: cast zx s_125_2 -> bv
        let s_125_6: Bits = Bits::new(s_125_2 as u128, 1u16);
        // D s_125_7: cast zx s_125_5 -> bv
        let s_125_7: Bits = Bits::new(s_125_5 as u128, 1u16);
        // D s_125_8: cast reint s_125_6 -> u128
        let s_125_8: u128 = (s_125_6.value() as u128);
        // D s_125_9: size-of s_125_6
        let s_125_9: u16 = s_125_6.length();
        // D s_125_10: cast reint s_125_7 -> u128
        let s_125_10: u128 = (s_125_7.value() as u128);
        // D s_125_11: size-of s_125_7
        let s_125_11: u16 = s_125_7.length();
        // D s_125_12: lsl s_125_8 s_125_11
        let s_125_12: u128 = s_125_8 << s_125_11;
        // D s_125_13: or s_125_12 s_125_10
        let s_125_13: u128 = ((s_125_12) | (s_125_10));
        // D s_125_14: add s_125_9 s_125_11
        let s_125_14: u16 = (s_125_9 + s_125_11);
        // D s_125_15: create-bits s_125_13 s_125_14
        let s_125_15: Bits = Bits::new(s_125_13, s_125_14);
        // D s_125_16: cast reint s_125_15 -> u8
        let s_125_16: u8 = (s_125_15.value() as u8);
        // D s_125_17: cast zx s_125_16 -> bv
        let s_125_17: Bits = Bits::new(s_125_16 as u128, 2u16);
        // C s_125_18: const #3u : u8
        let s_125_18: u8 = 3;
        // C s_125_19: cast zx s_125_18 -> bv
        let s_125_19: Bits = Bits::new(s_125_18 as u128, 2u16);
        // D s_125_20: cmp-eq s_125_17 s_125_19
        let s_125_20: bool = ((s_125_17) == (s_125_19));
        // D s_125_21: write-var gs#137247 <= s_125_20
        fn_state.gs_137247 = s_125_20;
        // N s_125_22: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_126_0: const #() : ()
        let s_126_0: () = ();
        // S s_126_1: call EL2Enabled(s_126_0)
        let s_126_1: bool = EL2Enabled(state, tracer, s_126_0);
        // N s_126_2: branch s_126_1 b131 b127
        if s_126_1 {
            return block_131(state, tracer, fn_state);
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
        // D s_127_1: write-var gs#137256 <= s_127_0
        fn_state.gs_137256 = s_127_0;
        // N s_127_2: jump b128
        return block_128(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_128_0: read-var gs#137256:u8
        let s_128_0: bool = fn_state.gs_137256;
        // N s_128_1: branch s_128_0 b130 b129
        if s_128_0 {
            return block_130(state, tracer, fn_state);
        } else {
            return block_129(state, tracer, fn_state);
        };
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_129_0: const #20u : u8
        let s_129_0: u8 = 20;
        // C s_129_1: cast zx s_129_0 -> bv
        let s_129_1: Bits = Bits::new(s_129_0 as u128, 8u16);
        // C s_129_2: cast zx s_129_1 -> i
        let s_129_2: i128 = (s_129_1.value() as i128);
        // C s_129_3: cast reint s_129_2 -> i64
        let s_129_3: i64 = (s_129_2 as i64);
        // C s_129_4: cast zx s_129_3 -> i
        let s_129_4: i128 = (i128::try_from(s_129_3).unwrap());
        // C s_129_5: const #440u : u32
        let s_129_5: u32 = 440;
        // D s_129_6: read-reg s_129_5:u8
        let s_129_6: u8 = {
            let value = state.read_register::<u8>(s_129_5 as isize);
            tracer.read_register(s_129_5 as isize, value);
            value
        };
        // D s_129_7: call AArch64_SystemAccessTrap(s_129_6, s_129_4)
        let s_129_7: () = AArch64_SystemAccessTrap(state, tracer, s_129_6, s_129_4);
        // N s_129_8: return
        return;
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_130_0: const #20u : u8
        let s_130_0: u8 = 20;
        // C s_130_1: cast zx s_130_0 -> bv
        let s_130_1: Bits = Bits::new(s_130_0 as u128, 8u16);
        // C s_130_2: cast zx s_130_1 -> i
        let s_130_2: i128 = (s_130_1.value() as i128);
        // C s_130_3: cast reint s_130_2 -> i64
        let s_130_3: i64 = (s_130_2 as i64);
        // C s_130_4: cast zx s_130_3 -> i
        let s_130_4: i128 = (i128::try_from(s_130_3).unwrap());
        // C s_130_5: const #432u : u32
        let s_130_5: u32 = 432;
        // D s_130_6: read-reg s_130_5:u8
        let s_130_6: u8 = {
            let value = state.read_register::<u8>(s_130_5 as isize);
            tracer.read_register(s_130_5 as isize, value);
            value
        };
        // D s_130_7: call AArch64_SystemAccessTrap(s_130_6, s_130_4)
        let s_130_7: () = AArch64_SystemAccessTrap(state, tracer, s_130_6, s_130_4);
        // N s_130_8: return
        return;
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_131_0: read-var __HCR_EL2_TGE:u8
        let s_131_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_131_1: cast zx s_131_0 -> bv
        let s_131_1: Bits = Bits::new(s_131_0 as u128, 1u16);
        // C s_131_2: const #1u : u8
        let s_131_2: bool = true;
        // C s_131_3: cast zx s_131_2 -> bv
        let s_131_3: Bits = Bits::new(s_131_2 as u128, 1u16);
        // D s_131_4: cmp-eq s_131_1 s_131_3
        let s_131_4: bool = ((s_131_1) == (s_131_3));
        // D s_131_5: write-var gs#137256 <= s_131_4
        fn_state.gs_137256 = s_131_4;
        // N s_131_6: jump b128
        return block_128(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_132_0: const #() : ()
        let s_132_0: () = ();
        // S s_132_1: call IsSCTLR2EL1Enabled(s_132_0)
        let s_132_1: bool = IsSCTLR2EL1Enabled(state, tracer, s_132_0);
        // S s_132_2: not s_132_1
        let s_132_2: bool = !s_132_1;
        // N s_132_3: branch s_132_2 b135 b133
        if s_132_2 {
            return block_135(state, tracer, fn_state);
        } else {
            return block_133(state, tracer, fn_state);
        };
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_133_0: read-var __SCTLR2_EL1_EnIDCP128:u8
        let s_133_0: bool = fn_state.u__SCTLR2_EL1_EnIDCP128;
        // D s_133_1: cast zx s_133_0 -> bv
        let s_133_1: Bits = Bits::new(s_133_0 as u128, 1u16);
        // C s_133_2: const #0u : u8
        let s_133_2: bool = false;
        // C s_133_3: cast zx s_133_2 -> bv
        let s_133_3: Bits = Bits::new(s_133_2 as u128, 1u16);
        // D s_133_4: cmp-eq s_133_1 s_133_3
        let s_133_4: bool = ((s_133_1) == (s_133_3));
        // D s_133_5: write-var gs#137245 <= s_133_4
        fn_state.gs_137245 = s_133_4;
        // N s_133_6: jump b134
        return block_134(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_134_0: read-var gs#137245:u8
        let s_134_0: bool = fn_state.gs_137245;
        // D s_134_1: write-var gs#137246 <= s_134_0
        fn_state.gs_137246 = s_134_0;
        // N s_134_2: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_135_0: const #1u : u8
        let s_135_0: bool = true;
        // D s_135_1: write-var gs#137245 <= s_135_0
        fn_state.gs_137245 = s_135_0;
        // N s_135_2: jump b134
        return block_134(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_136_0: const #102552u : u32
        let s_136_0: u32 = 102552;
        // D s_136_1: read-reg s_136_0:struct
        let s_136_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_136_0 as isize);
            tracer.read_register(s_136_0 as isize, value);
            value
        };
        // D s_136_2: call _get_HCR_EL2_Type_E2H(s_136_1)
        let s_136_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_136_1);
        // C s_136_3: const #102552u : u32
        let s_136_3: u32 = 102552;
        // D s_136_4: read-reg s_136_3:struct
        let s_136_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_136_3 as isize);
            tracer.read_register(s_136_3 as isize, value);
            value
        };
        // D s_136_5: call _get_HCR_EL2_Type_TGE(s_136_4)
        let s_136_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_136_4);
        // D s_136_6: cast zx s_136_2 -> bv
        let s_136_6: Bits = Bits::new(s_136_2 as u128, 1u16);
        // D s_136_7: cast zx s_136_5 -> bv
        let s_136_7: Bits = Bits::new(s_136_5 as u128, 1u16);
        // D s_136_8: cast reint s_136_6 -> u128
        let s_136_8: u128 = (s_136_6.value() as u128);
        // D s_136_9: size-of s_136_6
        let s_136_9: u16 = s_136_6.length();
        // D s_136_10: cast reint s_136_7 -> u128
        let s_136_10: u128 = (s_136_7.value() as u128);
        // D s_136_11: size-of s_136_7
        let s_136_11: u16 = s_136_7.length();
        // D s_136_12: lsl s_136_8 s_136_11
        let s_136_12: u128 = s_136_8 << s_136_11;
        // D s_136_13: or s_136_12 s_136_10
        let s_136_13: u128 = ((s_136_12) | (s_136_10));
        // D s_136_14: add s_136_9 s_136_11
        let s_136_14: u16 = (s_136_9 + s_136_11);
        // D s_136_15: create-bits s_136_13 s_136_14
        let s_136_15: Bits = Bits::new(s_136_13, s_136_14);
        // D s_136_16: cast reint s_136_15 -> u8
        let s_136_16: u8 = (s_136_15.value() as u8);
        // D s_136_17: cast zx s_136_16 -> bv
        let s_136_17: Bits = Bits::new(s_136_16 as u128, 2u16);
        // C s_136_18: const #3u : u8
        let s_136_18: u8 = 3;
        // C s_136_19: cast zx s_136_18 -> bv
        let s_136_19: Bits = Bits::new(s_136_18 as u128, 2u16);
        // D s_136_20: cmp-eq s_136_17 s_136_19
        let s_136_20: bool = ((s_136_17) == (s_136_19));
        // D s_136_21: write-var gs#137244 <= s_136_20
        fn_state.gs_137244 = s_136_20;
        // N s_136_22: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_137_0: const #20u : u8
        let s_137_0: u8 = 20;
        // C s_137_1: cast zx s_137_0 -> bv
        let s_137_1: Bits = Bits::new(s_137_0 as u128, 8u16);
        // C s_137_2: cast zx s_137_1 -> i
        let s_137_2: i128 = (s_137_1.value() as i128);
        // C s_137_3: cast reint s_137_2 -> i64
        let s_137_3: i64 = (s_137_2 as i64);
        // C s_137_4: cast zx s_137_3 -> i
        let s_137_4: i128 = (i128::try_from(s_137_3).unwrap());
        // C s_137_5: const #432u : u32
        let s_137_5: u32 = 432;
        // D s_137_6: read-reg s_137_5:u8
        let s_137_6: u8 = {
            let value = state.read_register::<u8>(s_137_5 as isize);
            tracer.read_register(s_137_5 as isize, value);
            value
        };
        // D s_137_7: call AArch64_SystemAccessTrap(s_137_6, s_137_4)
        let s_137_7: () = AArch64_SystemAccessTrap(state, tracer, s_137_6, s_137_4);
        // N s_137_8: return
        return;
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_138_0: read-var __SCTLR_EL2_TIDCP:u8
        let s_138_0: bool = fn_state.u__SCTLR_EL2_TIDCP;
        // D s_138_1: cast zx s_138_0 -> bv
        let s_138_1: Bits = Bits::new(s_138_0 as u128, 1u16);
        // C s_138_2: const #1u : u8
        let s_138_2: bool = true;
        // C s_138_3: cast zx s_138_2 -> bv
        let s_138_3: Bits = Bits::new(s_138_2 as u128, 1u16);
        // D s_138_4: cmp-eq s_138_1 s_138_3
        let s_138_4: bool = ((s_138_1) == (s_138_3));
        // D s_138_5: write-var gs#137243 <= s_138_4
        fn_state.gs_137243 = s_138_4;
        // N s_138_6: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_139_0: const #102552u : u32
        let s_139_0: u32 = 102552;
        // D s_139_1: read-reg s_139_0:struct
        let s_139_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_139_0 as isize);
            tracer.read_register(s_139_0 as isize, value);
            value
        };
        // D s_139_2: call _get_HCR_EL2_Type_E2H(s_139_1)
        let s_139_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_139_1);
        // C s_139_3: const #102552u : u32
        let s_139_3: u32 = 102552;
        // D s_139_4: read-reg s_139_3:struct
        let s_139_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_139_3 as isize);
            tracer.read_register(s_139_3 as isize, value);
            value
        };
        // D s_139_5: call _get_HCR_EL2_Type_TGE(s_139_4)
        let s_139_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_139_4);
        // D s_139_6: cast zx s_139_2 -> bv
        let s_139_6: Bits = Bits::new(s_139_2 as u128, 1u16);
        // D s_139_7: cast zx s_139_5 -> bv
        let s_139_7: Bits = Bits::new(s_139_5 as u128, 1u16);
        // D s_139_8: cast reint s_139_6 -> u128
        let s_139_8: u128 = (s_139_6.value() as u128);
        // D s_139_9: size-of s_139_6
        let s_139_9: u16 = s_139_6.length();
        // D s_139_10: cast reint s_139_7 -> u128
        let s_139_10: u128 = (s_139_7.value() as u128);
        // D s_139_11: size-of s_139_7
        let s_139_11: u16 = s_139_7.length();
        // D s_139_12: lsl s_139_8 s_139_11
        let s_139_12: u128 = s_139_8 << s_139_11;
        // D s_139_13: or s_139_12 s_139_10
        let s_139_13: u128 = ((s_139_12) | (s_139_10));
        // D s_139_14: add s_139_9 s_139_11
        let s_139_14: u16 = (s_139_9 + s_139_11);
        // D s_139_15: create-bits s_139_13 s_139_14
        let s_139_15: Bits = Bits::new(s_139_13, s_139_14);
        // D s_139_16: cast reint s_139_15 -> u8
        let s_139_16: u8 = (s_139_15.value() as u8);
        // D s_139_17: cast zx s_139_16 -> bv
        let s_139_17: Bits = Bits::new(s_139_16 as u128, 2u16);
        // C s_139_18: const #3u : u8
        let s_139_18: u8 = 3;
        // C s_139_19: cast zx s_139_18 -> bv
        let s_139_19: Bits = Bits::new(s_139_18 as u128, 2u16);
        // D s_139_20: cmp-eq s_139_17 s_139_19
        let s_139_20: bool = ((s_139_17) == (s_139_19));
        // D s_139_21: write-var gs#137242 <= s_139_20
        fn_state.gs_137242 = s_139_20;
        // N s_139_22: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_140_0: const #() : ()
        let s_140_0: () = ();
        // S s_140_1: call EL2Enabled(s_140_0)
        let s_140_1: bool = EL2Enabled(state, tracer, s_140_0);
        // N s_140_2: branch s_140_1 b145 b141
        if s_140_1 {
            return block_145(state, tracer, fn_state);
        } else {
            return block_141(state, tracer, fn_state);
        };
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_141_0: const #0u : u8
        let s_141_0: bool = false;
        // D s_141_1: write-var gs#137257 <= s_141_0
        fn_state.gs_137257 = s_141_0;
        // N s_141_2: jump b142
        return block_142(state, tracer, fn_state);
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_142_0: read-var gs#137257:u8
        let s_142_0: bool = fn_state.gs_137257;
        // N s_142_1: branch s_142_0 b144 b143
        if s_142_0 {
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
        // C s_143_0: const #20u : u8
        let s_143_0: u8 = 20;
        // C s_143_1: cast zx s_143_0 -> bv
        let s_143_1: Bits = Bits::new(s_143_0 as u128, 8u16);
        // C s_143_2: cast zx s_143_1 -> i
        let s_143_2: i128 = (s_143_1.value() as i128);
        // C s_143_3: cast reint s_143_2 -> i64
        let s_143_3: i64 = (s_143_2 as i64);
        // C s_143_4: cast zx s_143_3 -> i
        let s_143_4: i128 = (i128::try_from(s_143_3).unwrap());
        // C s_143_5: const #440u : u32
        let s_143_5: u32 = 440;
        // D s_143_6: read-reg s_143_5:u8
        let s_143_6: u8 = {
            let value = state.read_register::<u8>(s_143_5 as isize);
            tracer.read_register(s_143_5 as isize, value);
            value
        };
        // D s_143_7: call AArch64_SystemAccessTrap(s_143_6, s_143_4)
        let s_143_7: () = AArch64_SystemAccessTrap(state, tracer, s_143_6, s_143_4);
        // N s_143_8: return
        return;
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_144_0: const #20u : u8
        let s_144_0: u8 = 20;
        // C s_144_1: cast zx s_144_0 -> bv
        let s_144_1: Bits = Bits::new(s_144_0 as u128, 8u16);
        // C s_144_2: cast zx s_144_1 -> i
        let s_144_2: i128 = (s_144_1.value() as i128);
        // C s_144_3: cast reint s_144_2 -> i64
        let s_144_3: i64 = (s_144_2 as i64);
        // C s_144_4: cast zx s_144_3 -> i
        let s_144_4: i128 = (i128::try_from(s_144_3).unwrap());
        // C s_144_5: const #432u : u32
        let s_144_5: u32 = 432;
        // D s_144_6: read-reg s_144_5:u8
        let s_144_6: u8 = {
            let value = state.read_register::<u8>(s_144_5 as isize);
            tracer.read_register(s_144_5 as isize, value);
            value
        };
        // D s_144_7: call AArch64_SystemAccessTrap(s_144_6, s_144_4)
        let s_144_7: () = AArch64_SystemAccessTrap(state, tracer, s_144_6, s_144_4);
        // N s_144_8: return
        return;
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_145_0: read-var __HCR_EL2_TGE:u8
        let s_145_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_145_1: cast zx s_145_0 -> bv
        let s_145_1: Bits = Bits::new(s_145_0 as u128, 1u16);
        // C s_145_2: const #1u : u8
        let s_145_2: bool = true;
        // C s_145_3: cast zx s_145_2 -> bv
        let s_145_3: Bits = Bits::new(s_145_2 as u128, 1u16);
        // D s_145_4: cmp-eq s_145_1 s_145_3
        let s_145_4: bool = ((s_145_1) == (s_145_3));
        // D s_145_5: write-var gs#137257 <= s_145_4
        fn_state.gs_137257 = s_145_4;
        // N s_145_6: jump b142
        return block_142(state, tracer, fn_state);
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_146_0: read-var __SCTLR_EL1_TIDCP:u8
        let s_146_0: bool = fn_state.u__SCTLR_EL1_TIDCP;
        // D s_146_1: cast zx s_146_0 -> bv
        let s_146_1: Bits = Bits::new(s_146_0 as u128, 1u16);
        // C s_146_2: const #1u : u8
        let s_146_2: bool = true;
        // C s_146_3: cast zx s_146_2 -> bv
        let s_146_3: Bits = Bits::new(s_146_2 as u128, 1u16);
        // D s_146_4: cmp-eq s_146_1 s_146_3
        let s_146_4: bool = ((s_146_1) == (s_146_3));
        // D s_146_5: write-var gs#137241 <= s_146_4
        fn_state.gs_137241 = s_146_4;
        // N s_146_6: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_147_0: const #102552u : u32
        let s_147_0: u32 = 102552;
        // D s_147_1: read-reg s_147_0:struct
        let s_147_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_147_0 as isize);
            tracer.read_register(s_147_0 as isize, value);
            value
        };
        // D s_147_2: call _get_HCR_EL2_Type_E2H(s_147_1)
        let s_147_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_147_1);
        // C s_147_3: const #102552u : u32
        let s_147_3: u32 = 102552;
        // D s_147_4: read-reg s_147_3:struct
        let s_147_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_147_3 as isize);
            tracer.read_register(s_147_3 as isize, value);
            value
        };
        // D s_147_5: call _get_HCR_EL2_Type_TGE(s_147_4)
        let s_147_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_147_4);
        // D s_147_6: cast zx s_147_2 -> bv
        let s_147_6: Bits = Bits::new(s_147_2 as u128, 1u16);
        // D s_147_7: cast zx s_147_5 -> bv
        let s_147_7: Bits = Bits::new(s_147_5 as u128, 1u16);
        // D s_147_8: cast reint s_147_6 -> u128
        let s_147_8: u128 = (s_147_6.value() as u128);
        // D s_147_9: size-of s_147_6
        let s_147_9: u16 = s_147_6.length();
        // D s_147_10: cast reint s_147_7 -> u128
        let s_147_10: u128 = (s_147_7.value() as u128);
        // D s_147_11: size-of s_147_7
        let s_147_11: u16 = s_147_7.length();
        // D s_147_12: lsl s_147_8 s_147_11
        let s_147_12: u128 = s_147_8 << s_147_11;
        // D s_147_13: or s_147_12 s_147_10
        let s_147_13: u128 = ((s_147_12) | (s_147_10));
        // D s_147_14: add s_147_9 s_147_11
        let s_147_14: u16 = (s_147_9 + s_147_11);
        // D s_147_15: create-bits s_147_13 s_147_14
        let s_147_15: Bits = Bits::new(s_147_13, s_147_14);
        // D s_147_16: cast reint s_147_15 -> u8
        let s_147_16: u8 = (s_147_15.value() as u8);
        // D s_147_17: cast zx s_147_16 -> bv
        let s_147_17: Bits = Bits::new(s_147_16 as u128, 2u16);
        // C s_147_18: const #3u : u8
        let s_147_18: u8 = 3;
        // C s_147_19: cast zx s_147_18 -> bv
        let s_147_19: Bits = Bits::new(s_147_18 as u128, 2u16);
        // D s_147_20: cmp-eq s_147_17 s_147_19
        let s_147_20: bool = ((s_147_17) == (s_147_19));
        // D s_147_21: write-var gs#137240 <= s_147_20
        fn_state.gs_137240 = s_147_20;
        // N s_147_22: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_148_0: panic
        panic!("{:?}", ());
        // N s_148_1: return
        return;
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_149_0: read-var __SCR_EL3_EnIDCP128:u8
        let s_149_0: bool = fn_state.u__SCR_EL3_EnIDCP128;
        // D s_149_1: cast zx s_149_0 -> bv
        let s_149_1: Bits = Bits::new(s_149_0 as u128, 1u16);
        // C s_149_2: const #0u : u8
        let s_149_2: bool = false;
        // C s_149_3: cast zx s_149_2 -> bv
        let s_149_3: Bits = Bits::new(s_149_2 as u128, 1u16);
        // D s_149_4: cmp-eq s_149_1 s_149_3
        let s_149_4: bool = ((s_149_1) == (s_149_3));
        // D s_149_5: write-var gs#137239 <= s_149_4
        fn_state.gs_137239 = s_149_4;
        // N s_149_6: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_150_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_150_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_150_1: call __IMPDEF_boolean(s_150_0)
        let s_150_1: bool = u__IMPDEF_boolean(state, tracer, s_150_0);
        // D s_150_2: write-var gs#137238 <= s_150_1
        fn_state.gs_137238 = s_150_1;
        // N s_150_3: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_151_0: read-var __EDSCR_SDD:u8
        let s_151_0: bool = fn_state.u__EDSCR_SDD;
        // D s_151_1: cast zx s_151_0 -> bv
        let s_151_1: Bits = Bits::new(s_151_0 as u128, 1u16);
        // C s_151_2: const #1u : u8
        let s_151_2: bool = true;
        // C s_151_3: cast zx s_151_2 -> bv
        let s_151_3: Bits = Bits::new(s_151_2 as u128, 1u16);
        // D s_151_4: cmp-eq s_151_1 s_151_3
        let s_151_4: bool = ((s_151_1) == (s_151_3));
        // D s_151_5: write-var gs#137237 <= s_151_4
        fn_state.gs_137237 = s_151_4;
        // N s_151_6: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_152_0: const #424u : u32
        let s_152_0: u32 = 424;
        // D s_152_1: read-reg s_152_0:u8
        let s_152_1: u8 = {
            let value = state.read_register::<u8>(s_152_0 as isize);
            tracer.read_register(s_152_0 as isize, value);
            value
        };
        // C s_152_2: const #2u : u8
        let s_152_2: u8 = 2;
        // D s_152_3: cmp-lt s_152_1 s_152_2
        let s_152_3: bool = ((s_152_1) < (s_152_2));
        // D s_152_4: write-var gs#137236 <= s_152_3
        fn_state.gs_137236 = s_152_3;
        // N s_152_5: jump b71
        return block_71(state, tracer, fn_state);
    }
}
