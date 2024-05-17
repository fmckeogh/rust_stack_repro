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
use u_get_HCR_EL2_Type_E2H::*;
use Mk_CNTV_CVAL_EL0_Type::*;
use IsFeatureImplemented::*;
use NVMem_set::*;
use X_read::*;
use AArch64_SystemAccessTrap::*;
use u_get_SCR_EL3_Type_NS::*;
use u_get_HCR_EL2_Type_NV1::*;
use u_get_CNTHCTL_EL2_Type_EL1TVT::*;
use u_get_HCR_EL2_Type_NV::*;
use Mk_CNTHVS_CVAL_EL2_Type::*;
use Mk_CNTHV_CVAL_EL2_Type::*;
use u_get_CNTHCTL_EL2_Type_EL0VTEN::*;
use EL2Enabled::*;
use u_get_CNTKCTL_EL1_Type_EL0VTEN::*;
use u_get_HCR_EL2_Type_TGE::*;
use u_get_HCR_EL2_Type_NV2::*;
use common::*;
pub fn CNTHV_CVAL_EL2_SysRegWrite_16bd78456d53d9ab<T: Tracer>(
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
        gs_81701: bool,
        u__HCR_EL2_TGE: bool,
        gs_81694: bool,
        u__CNTHCTL_EL2_EL0VTEN: bool,
        u__CNTHCTL_EL2_EL1TVT: bool,
        gs_81708: bool,
        gs_81712: bool,
        gs_81704: bool,
        gs_81705: bool,
        gs_81713: bool,
        gs_81696: bool,
        gs_81695: bool,
        gs_81714: bool,
        gs_81700: bool,
        gs_81718: bool,
        u__CNTKCTL_EL1_EL0VTEN: bool,
        u__PSTATE_EL: u8,
        gs_81706: bool,
        gs_81707: bool,
        gs_81709: bool,
        gs_81710: bool,
        gs_81711: bool,
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
        // C s_0_3: const #22056u : u32
        let s_0_3: u32 = 22056;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_CNTKCTL_EL1_Type_EL0VTEN(s_0_4)
        let s_0_5: bool = u_get_CNTKCTL_EL1_Type_EL0VTEN(state, tracer, s_0_4);
        // D s_0_6: write-var __CNTKCTL_EL1_EL0VTEN <= s_0_5
        fn_state.u__CNTKCTL_EL1_EL0VTEN = s_0_5;
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
        // C s_0_11: const #12808u : u32
        let s_0_11: u32 = 12808;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_CNTHCTL_EL2_Type_EL0VTEN(s_0_12)
        let s_0_13: bool = u_get_CNTHCTL_EL2_Type_EL0VTEN(state, tracer, s_0_12);
        // D s_0_14: write-var __CNTHCTL_EL2_EL0VTEN <= s_0_13
        fn_state.u__CNTHCTL_EL2_EL0VTEN = s_0_13;
        // C s_0_15: const #12808u : u32
        let s_0_15: u32 = 12808;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_CNTHCTL_EL2_Type_EL1TVT(s_0_16)
        let s_0_17: bool = u_get_CNTHCTL_EL2_Type_EL1TVT(state, tracer, s_0_16);
        // D s_0_18: write-var __CNTHCTL_EL2_EL1TVT <= s_0_17
        fn_state.u__CNTHCTL_EL2_EL1TVT = s_0_17;
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
        // N s_0_25: branch s_0_24 b31 b1
        if s_0_24 {
            return block_31(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b20 b2
        if s_1_5 {
            return block_20(state, tracer, fn_state);
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
        // D s_5_4: call Mk_CNTV_CVAL_EL0_Type(s_5_3)
        let s_5_4: ProductType5c790c8ef59cc8b2 = Mk_CNTV_CVAL_EL0_Type(
            state,
            tracer,
            s_5_3,
        );
        // C s_5_5: const #23632u : u32
        let s_5_5: u32 = 23632;
        // N s_5_6: write-reg s_5_5 <= s_5_4
        let s_5_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_5_5 as isize, s_5_4);
            tracer.write_register(s_5_5 as isize, s_5_4);
        };
        // N s_5_7: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #102552u : u32
        let s_6_0: u32 = 102552;
        // D s_6_1: read-reg s_6_0:struct
        let s_6_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: call _get_HCR_EL2_Type_E2H(s_6_1)
        let s_6_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_6_1);
        // D s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // C s_6_4: const #1u : u8
        let s_6_4: bool = true;
        // C s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 1u16);
        // D s_6_6: cmp-eq s_6_3 s_6_5
        let s_6_6: bool = ((s_6_3) == (s_6_5));
        // N s_6_7: branch s_6_6 b19 b7
        if s_6_6 {
            return block_19(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#81694 <= s_7_0
        fn_state.gs_81694 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#81694:u8
        let s_8_0: bool = fn_state.gs_81694;
        // N s_8_1: branch s_8_0 b18 b9
        if s_8_0 {
            return block_18(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#81695 <= s_9_0
        fn_state.gs_81695 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#81695:u8
        let s_10_0: bool = fn_state.gs_81695;
        // N s_10_1: branch s_10_0 b17 b11
        if s_10_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #102552u : u32
        let s_11_0: u32 = 102552;
        // D s_11_1: read-reg s_11_0:struct
        let s_11_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // D s_11_2: call _get_HCR_EL2_Type_E2H(s_11_1)
        let s_11_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_11_1);
        // D s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 1u16);
        // C s_11_4: const #1u : u8
        let s_11_4: bool = true;
        // C s_11_5: cast zx s_11_4 -> bv
        let s_11_5: Bits = Bits::new(s_11_4 as u128, 1u16);
        // D s_11_6: cmp-eq s_11_3 s_11_5
        let s_11_6: bool = ((s_11_3) == (s_11_5));
        // N s_11_7: branch s_11_6 b16 b12
        if s_11_6 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#81696 <= s_12_0
        fn_state.gs_81696 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#81696:u8
        let s_13_0: bool = fn_state.gs_81696;
        // N s_13_1: branch s_13_0 b15 b14
        if s_13_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #64s : i64
        let s_14_0: i64 = 64;
        // D s_14_1: read-var t:i
        let s_14_1: i128 = fn_state.t;
        // D s_14_2: call X_read(s_14_1, s_14_0)
        let s_14_2: Bits = X_read(state, tracer, s_14_1, s_14_0);
        // D s_14_3: cast reint s_14_2 -> u64
        let s_14_3: u64 = (s_14_2.value() as u64);
        // D s_14_4: call Mk_CNTV_CVAL_EL0_Type(s_14_3)
        let s_14_4: ProductType5c790c8ef59cc8b2 = Mk_CNTV_CVAL_EL0_Type(
            state,
            tracer,
            s_14_3,
        );
        // C s_14_5: const #23632u : u32
        let s_14_5: u32 = 23632;
        // N s_14_6: write-reg s_14_5 <= s_14_4
        let s_14_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_14_5 as isize, s_14_4);
            tracer.write_register(s_14_5 as isize, s_14_4);
        };
        // N s_14_7: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #64s : i64
        let s_15_0: i64 = 64;
        // D s_15_1: read-var t:i
        let s_15_1: i128 = fn_state.t;
        // D s_15_2: call X_read(s_15_1, s_15_0)
        let s_15_2: Bits = X_read(state, tracer, s_15_1, s_15_0);
        // D s_15_3: cast reint s_15_2 -> u64
        let s_15_3: u64 = (s_15_2.value() as u64);
        // D s_15_4: call Mk_CNTHV_CVAL_EL2_Type(s_15_3)
        let s_15_4: ProductType5c790c8ef59cc8b2 = Mk_CNTHV_CVAL_EL2_Type(
            state,
            tracer,
            s_15_3,
        );
        // C s_15_5: const #103152u : u32
        let s_15_5: u32 = 103152;
        // N s_15_6: write-reg s_15_5 <= s_15_4
        let s_15_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_15_5 as isize, s_15_4);
            tracer.write_register(s_15_5 as isize, s_15_4);
        };
        // N s_15_7: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #90704u : u32
        let s_16_0: u32 = 90704;
        // D s_16_1: read-reg s_16_0:struct
        let s_16_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // D s_16_2: call _get_SCR_EL3_Type_NS(s_16_1)
        let s_16_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_16_1);
        // D s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 1u16);
        // C s_16_4: const #1u : u8
        let s_16_4: bool = true;
        // C s_16_5: cast zx s_16_4 -> bv
        let s_16_5: Bits = Bits::new(s_16_4 as u128, 1u16);
        // D s_16_6: cmp-eq s_16_3 s_16_5
        let s_16_6: bool = ((s_16_3) == (s_16_5));
        // D s_16_7: write-var gs#81696 <= s_16_6
        fn_state.gs_81696 = s_16_6;
        // N s_16_8: jump b13
        return block_13(state, tracer, fn_state);
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
        // D s_17_4: call Mk_CNTHVS_CVAL_EL2_Type(s_17_3)
        let s_17_4: ProductType5c790c8ef59cc8b2 = Mk_CNTHVS_CVAL_EL2_Type(
            state,
            tracer,
            s_17_3,
        );
        // C s_17_5: const #10064u : u32
        let s_17_5: u32 = 10064;
        // N s_17_6: write-reg s_17_5 <= s_17_4
        let s_17_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_17_5 as isize, s_17_4);
            tracer.write_register(s_17_5 as isize, s_17_4);
        };
        // N s_17_7: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #117u : u32
        let s_18_0: u32 = 117;
        // S s_18_1: call IsFeatureImplemented(s_18_0)
        let s_18_1: bool = IsFeatureImplemented(state, tracer, s_18_0);
        // D s_18_2: write-var gs#81695 <= s_18_1
        fn_state.gs_81695 = s_18_1;
        // N s_18_3: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #90704u : u32
        let s_19_0: u32 = 90704;
        // D s_19_1: read-reg s_19_0:struct
        let s_19_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: call _get_SCR_EL3_Type_NS(s_19_1)
        let s_19_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_19_1);
        // D s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 1u16);
        // C s_19_4: const #0u : u8
        let s_19_4: bool = false;
        // C s_19_5: cast zx s_19_4 -> bv
        let s_19_5: Bits = Bits::new(s_19_4 as u128, 1u16);
        // D s_19_6: cmp-eq s_19_3 s_19_5
        let s_19_6: bool = ((s_19_3) == (s_19_5));
        // D s_19_7: write-var gs#81694 <= s_19_6
        fn_state.gs_81694 = s_19_6;
        // N s_19_8: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call EL2Enabled(s_20_0)
        let s_20_1: bool = EL2Enabled(state, tracer, s_20_0);
        // N s_20_2: branch s_20_1 b30 b21
        if s_20_1 {
            return block_30(state, tracer, fn_state);
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
        // D s_21_1: write-var gs#81700 <= s_21_0
        fn_state.gs_81700 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#81700:u8
        let s_22_0: bool = fn_state.gs_81700;
        // N s_22_1: branch s_22_0 b29 b23
        if s_22_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #() : ()
        let s_23_0: () = ();
        // S s_23_1: call EL2Enabled(s_23_0)
        let s_23_1: bool = EL2Enabled(state, tracer, s_23_0);
        // N s_23_2: branch s_23_1 b28 b24
        if s_23_1 {
            return block_28(state, tracer, fn_state);
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
        // D s_24_1: write-var gs#81701 <= s_24_0
        fn_state.gs_81701 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#81701:u8
        let s_25_0: bool = fn_state.gs_81701;
        // N s_25_1: branch s_25_0 b27 b26
        if s_25_0 {
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
        // C s_26_0: const #64s : i64
        let s_26_0: i64 = 64;
        // D s_26_1: read-var t:i
        let s_26_1: i128 = fn_state.t;
        // D s_26_2: call X_read(s_26_1, s_26_0)
        let s_26_2: Bits = X_read(state, tracer, s_26_1, s_26_0);
        // D s_26_3: cast reint s_26_2 -> u64
        let s_26_3: u64 = (s_26_2.value() as u64);
        // D s_26_4: call Mk_CNTV_CVAL_EL0_Type(s_26_3)
        let s_26_4: ProductType5c790c8ef59cc8b2 = Mk_CNTV_CVAL_EL0_Type(
            state,
            tracer,
            s_26_3,
        );
        // C s_26_5: const #23632u : u32
        let s_26_5: u32 = 23632;
        // N s_26_6: write-reg s_26_5 <= s_26_4
        let s_26_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_26_5 as isize, s_26_4);
            tracer.write_register(s_26_5 as isize, s_26_4);
        };
        // N s_26_7: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #360u : u12
        let s_27_0: u16 = 360;
        // C s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 12u16);
        // C s_27_2: cast zx s_27_1 -> i
        let s_27_2: i128 = (s_27_1.value() as i128);
        // C s_27_3: cast reint s_27_2 -> i64
        let s_27_3: i64 = (s_27_2 as i64);
        // C s_27_4: const #64s : i64
        let s_27_4: i64 = 64;
        // D s_27_5: read-var t:i
        let s_27_5: i128 = fn_state.t;
        // D s_27_6: call X_read(s_27_5, s_27_4)
        let s_27_6: Bits = X_read(state, tracer, s_27_5, s_27_4);
        // D s_27_7: cast reint s_27_6 -> u64
        let s_27_7: u64 = (s_27_6.value() as u64);
        // C s_27_8: cast zx s_27_3 -> i
        let s_27_8: i128 = (i128::try_from(s_27_3).unwrap());
        // D s_27_9: call NVMem_set(s_27_8, s_27_7)
        let s_27_9: () = NVMem_set(state, tracer, s_27_8, s_27_7);
        // N s_27_10: return
        return;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #102552u : u32
        let s_28_0: u32 = 102552;
        // D s_28_1: read-reg s_28_0:struct
        let s_28_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_28_0 as isize);
            tracer.read_register(s_28_0 as isize, value);
            value
        };
        // D s_28_2: call _get_HCR_EL2_Type_NV2(s_28_1)
        let s_28_2: bool = u_get_HCR_EL2_Type_NV2(state, tracer, s_28_1);
        // C s_28_3: const #102552u : u32
        let s_28_3: u32 = 102552;
        // D s_28_4: read-reg s_28_3:struct
        let s_28_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_28_3 as isize);
            tracer.read_register(s_28_3 as isize, value);
            value
        };
        // D s_28_5: call _get_HCR_EL2_Type_NV1(s_28_4)
        let s_28_5: bool = u_get_HCR_EL2_Type_NV1(state, tracer, s_28_4);
        // C s_28_6: const #102552u : u32
        let s_28_6: u32 = 102552;
        // D s_28_7: read-reg s_28_6:struct
        let s_28_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_28_6 as isize);
            tracer.read_register(s_28_6 as isize, value);
            value
        };
        // D s_28_8: call _get_HCR_EL2_Type_NV(s_28_7)
        let s_28_8: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_28_7);
        // D s_28_9: cast zx s_28_5 -> bv
        let s_28_9: Bits = Bits::new(s_28_5 as u128, 1u16);
        // D s_28_10: cast zx s_28_8 -> bv
        let s_28_10: Bits = Bits::new(s_28_8 as u128, 1u16);
        // D s_28_11: cast reint s_28_9 -> u128
        let s_28_11: u128 = (s_28_9.value() as u128);
        // D s_28_12: size-of s_28_9
        let s_28_12: u16 = s_28_9.length();
        // D s_28_13: cast reint s_28_10 -> u128
        let s_28_13: u128 = (s_28_10.value() as u128);
        // D s_28_14: size-of s_28_10
        let s_28_14: u16 = s_28_10.length();
        // D s_28_15: lsl s_28_11 s_28_14
        let s_28_15: u128 = s_28_11 << s_28_14;
        // D s_28_16: or s_28_15 s_28_13
        let s_28_16: u128 = ((s_28_15) | (s_28_13));
        // D s_28_17: add s_28_12 s_28_14
        let s_28_17: u16 = (s_28_12 + s_28_14);
        // D s_28_18: create-bits s_28_16 s_28_17
        let s_28_18: Bits = Bits::new(s_28_16, s_28_17);
        // D s_28_19: cast reint s_28_18 -> u8
        let s_28_19: u8 = (s_28_18.value() as u8);
        // D s_28_20: cast zx s_28_2 -> bv
        let s_28_20: Bits = Bits::new(s_28_2 as u128, 1u16);
        // D s_28_21: cast zx s_28_19 -> bv
        let s_28_21: Bits = Bits::new(s_28_19 as u128, 2u16);
        // D s_28_22: cast reint s_28_20 -> u128
        let s_28_22: u128 = (s_28_20.value() as u128);
        // D s_28_23: size-of s_28_20
        let s_28_23: u16 = s_28_20.length();
        // D s_28_24: cast reint s_28_21 -> u128
        let s_28_24: u128 = (s_28_21.value() as u128);
        // D s_28_25: size-of s_28_21
        let s_28_25: u16 = s_28_21.length();
        // D s_28_26: lsl s_28_22 s_28_25
        let s_28_26: u128 = s_28_22 << s_28_25;
        // D s_28_27: or s_28_26 s_28_24
        let s_28_27: u128 = ((s_28_26) | (s_28_24));
        // D s_28_28: add s_28_23 s_28_25
        let s_28_28: u16 = (s_28_23 + s_28_25);
        // D s_28_29: create-bits s_28_27 s_28_28
        let s_28_29: Bits = Bits::new(s_28_27, s_28_28);
        // D s_28_30: cast reint s_28_29 -> u8
        let s_28_30: u8 = (s_28_29.value() as u8);
        // D s_28_31: cast zx s_28_30 -> bv
        let s_28_31: Bits = Bits::new(s_28_30 as u128, 3u16);
        // C s_28_32: const #7u : u8
        let s_28_32: u8 = 7;
        // C s_28_33: cast zx s_28_32 -> bv
        let s_28_33: Bits = Bits::new(s_28_32 as u128, 3u16);
        // D s_28_34: cmp-eq s_28_31 s_28_33
        let s_28_34: bool = ((s_28_31) == (s_28_33));
        // D s_28_35: write-var gs#81701 <= s_28_34
        fn_state.gs_81701 = s_28_34;
        // N s_28_36: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #24u : u8
        let s_29_0: u8 = 24;
        // C s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 8u16);
        // C s_29_2: cast zx s_29_1 -> i
        let s_29_2: i128 = (s_29_1.value() as i128);
        // C s_29_3: cast reint s_29_2 -> i64
        let s_29_3: i64 = (s_29_2 as i64);
        // C s_29_4: cast zx s_29_3 -> i
        let s_29_4: i128 = (i128::try_from(s_29_3).unwrap());
        // C s_29_5: const #432u : u32
        let s_29_5: u32 = 432;
        // D s_29_6: read-reg s_29_5:u8
        let s_29_6: u8 = {
            let value = state.read_register::<u8>(s_29_5 as isize);
            tracer.read_register(s_29_5 as isize, value);
            value
        };
        // D s_29_7: call AArch64_SystemAccessTrap(s_29_6, s_29_4)
        let s_29_7: () = AArch64_SystemAccessTrap(state, tracer, s_29_6, s_29_4);
        // N s_29_8: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var __CNTHCTL_EL2_EL1TVT:u8
        let s_30_0: bool = fn_state.u__CNTHCTL_EL2_EL1TVT;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 1u16);
        // C s_30_2: const #1u : u8
        let s_30_2: bool = true;
        // C s_30_3: cast zx s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 1u16);
        // D s_30_4: cmp-eq s_30_1 s_30_3
        let s_30_4: bool = ((s_30_1) == (s_30_3));
        // D s_30_5: write-var gs#81700 <= s_30_4
        fn_state.gs_81700 = s_30_4;
        // N s_30_6: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #() : ()
        let s_31_0: () = ();
        // S s_31_1: call EL2Enabled(s_31_0)
        let s_31_1: bool = EL2Enabled(state, tracer, s_31_0);
        // N s_31_2: branch s_31_1 b79 b32
        if s_31_1 {
            return block_79(state, tracer, fn_state);
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
        // D s_32_1: write-var gs#81704 <= s_32_0
        fn_state.gs_81704 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#81704:u8
        let s_33_0: bool = fn_state.gs_81704;
        // D s_33_1: not s_33_0
        let s_33_1: bool = !s_33_0;
        // N s_33_2: branch s_33_1 b78 b34
        if s_33_1 {
            return block_78(state, tracer, fn_state);
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
        // D s_34_1: write-var gs#81705 <= s_34_0
        fn_state.gs_81705 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#81705:u8
        let s_35_0: bool = fn_state.gs_81705;
        // N s_35_1: branch s_35_0 b72 b36
        if s_35_0 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #() : ()
        let s_36_0: () = ();
        // S s_36_1: call EL2Enabled(s_36_0)
        let s_36_1: bool = EL2Enabled(state, tracer, s_36_0);
        // N s_36_2: branch s_36_1 b71 b37
        if s_36_1 {
            return block_71(state, tracer, fn_state);
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
        // D s_37_1: write-var gs#81706 <= s_37_0
        fn_state.gs_81706 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#81706:u8
        let s_38_0: bool = fn_state.gs_81706;
        // N s_38_1: branch s_38_0 b70 b39
        if s_38_0 {
            return block_70(state, tracer, fn_state);
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
        // D s_39_1: write-var gs#81707 <= s_39_0
        fn_state.gs_81707 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#81707:u8
        let s_40_0: bool = fn_state.gs_81707;
        // N s_40_1: branch s_40_0 b69 b41
        if s_40_0 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #() : ()
        let s_41_0: () = ();
        // S s_41_1: call EL2Enabled(s_41_0)
        let s_41_1: bool = EL2Enabled(state, tracer, s_41_0);
        // N s_41_2: branch s_41_1 b68 b42
        if s_41_1 {
            return block_68(state, tracer, fn_state);
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
        // D s_42_1: write-var gs#81708 <= s_42_0
        fn_state.gs_81708 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#81708:u8
        let s_43_0: bool = fn_state.gs_81708;
        // N s_43_1: branch s_43_0 b67 b44
        if s_43_0 {
            return block_67(state, tracer, fn_state);
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
        // D s_44_1: write-var gs#81709 <= s_44_0
        fn_state.gs_81709 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#81709:u8
        let s_45_0: bool = fn_state.gs_81709;
        // N s_45_1: branch s_45_0 b66 b46
        if s_45_0 {
            return block_66(state, tracer, fn_state);
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
        // N s_46_2: branch s_46_1 b65 b47
        if s_46_1 {
            return block_65(state, tracer, fn_state);
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
        // D s_47_1: write-var gs#81710 <= s_47_0
        fn_state.gs_81710 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#81710:u8
        let s_48_0: bool = fn_state.gs_81710;
        // N s_48_1: branch s_48_0 b64 b49
        if s_48_0 {
            return block_64(state, tracer, fn_state);
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
        // D s_49_1: write-var gs#81711 <= s_49_0
        fn_state.gs_81711 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#81711:u8
        let s_50_0: bool = fn_state.gs_81711;
        // N s_50_1: branch s_50_0 b63 b51
        if s_50_0 {
            return block_63(state, tracer, fn_state);
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
        // D s_51_1: write-var gs#81712 <= s_51_0
        fn_state.gs_81712 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#81712:u8
        let s_52_0: bool = fn_state.gs_81712;
        // N s_52_1: branch s_52_0 b62 b53
        if s_52_0 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #() : ()
        let s_53_0: () = ();
        // S s_53_1: call EL2Enabled(s_53_0)
        let s_53_1: bool = EL2Enabled(state, tracer, s_53_0);
        // N s_53_2: branch s_53_1 b61 b54
        if s_53_1 {
            return block_61(state, tracer, fn_state);
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
        // D s_54_1: write-var gs#81713 <= s_54_0
        fn_state.gs_81713 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#81713:u8
        let s_55_0: bool = fn_state.gs_81713;
        // N s_55_1: branch s_55_0 b60 b56
        if s_55_0 {
            return block_60(state, tracer, fn_state);
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
        // D s_56_1: write-var gs#81714 <= s_56_0
        fn_state.gs_81714 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#81714:u8
        let s_57_0: bool = fn_state.gs_81714;
        // N s_57_1: branch s_57_0 b59 b58
        if s_57_0 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #64s : i64
        let s_58_0: i64 = 64;
        // D s_58_1: read-var t:i
        let s_58_1: i128 = fn_state.t;
        // D s_58_2: call X_read(s_58_1, s_58_0)
        let s_58_2: Bits = X_read(state, tracer, s_58_1, s_58_0);
        // D s_58_3: cast reint s_58_2 -> u64
        let s_58_3: u64 = (s_58_2.value() as u64);
        // D s_58_4: call Mk_CNTV_CVAL_EL0_Type(s_58_3)
        let s_58_4: ProductType5c790c8ef59cc8b2 = Mk_CNTV_CVAL_EL0_Type(
            state,
            tracer,
            s_58_3,
        );
        // C s_58_5: const #23632u : u32
        let s_58_5: u32 = 23632;
        // N s_58_6: write-reg s_58_5 <= s_58_4
        let s_58_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_58_5 as isize, s_58_4);
            tracer.write_register(s_58_5 as isize, s_58_4);
        };
        // N s_58_7: return
        return;
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #64s : i64
        let s_59_0: i64 = 64;
        // D s_59_1: read-var t:i
        let s_59_1: i128 = fn_state.t;
        // D s_59_2: call X_read(s_59_1, s_59_0)
        let s_59_2: Bits = X_read(state, tracer, s_59_1, s_59_0);
        // D s_59_3: cast reint s_59_2 -> u64
        let s_59_3: u64 = (s_59_2.value() as u64);
        // D s_59_4: call Mk_CNTHV_CVAL_EL2_Type(s_59_3)
        let s_59_4: ProductType5c790c8ef59cc8b2 = Mk_CNTHV_CVAL_EL2_Type(
            state,
            tracer,
            s_59_3,
        );
        // C s_59_5: const #103152u : u32
        let s_59_5: u32 = 103152;
        // N s_59_6: write-reg s_59_5 <= s_59_4
        let s_59_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_59_5 as isize, s_59_4);
            tracer.write_register(s_59_5 as isize, s_59_4);
        };
        // N s_59_7: return
        return;
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #90704u : u32
        let s_60_0: u32 = 90704;
        // D s_60_1: read-reg s_60_0:struct
        let s_60_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_60_0 as isize);
            tracer.read_register(s_60_0 as isize, value);
            value
        };
        // D s_60_2: call _get_SCR_EL3_Type_NS(s_60_1)
        let s_60_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_60_1);
        // D s_60_3: cast zx s_60_2 -> bv
        let s_60_3: Bits = Bits::new(s_60_2 as u128, 1u16);
        // C s_60_4: const #1u : u8
        let s_60_4: bool = true;
        // C s_60_5: cast zx s_60_4 -> bv
        let s_60_5: Bits = Bits::new(s_60_4 as u128, 1u16);
        // D s_60_6: cmp-eq s_60_3 s_60_5
        let s_60_6: bool = ((s_60_3) == (s_60_5));
        // D s_60_7: write-var gs#81714 <= s_60_6
        fn_state.gs_81714 = s_60_6;
        // N s_60_8: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #102552u : u32
        let s_61_0: u32 = 102552;
        // D s_61_1: read-reg s_61_0:struct
        let s_61_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_61_0 as isize);
            tracer.read_register(s_61_0 as isize, value);
            value
        };
        // D s_61_2: call _get_HCR_EL2_Type_E2H(s_61_1)
        let s_61_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_61_1);
        // C s_61_3: const #102552u : u32
        let s_61_3: u32 = 102552;
        // D s_61_4: read-reg s_61_3:struct
        let s_61_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_61_3 as isize);
            tracer.read_register(s_61_3 as isize, value);
            value
        };
        // D s_61_5: call _get_HCR_EL2_Type_TGE(s_61_4)
        let s_61_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_61_4);
        // D s_61_6: cast zx s_61_2 -> bv
        let s_61_6: Bits = Bits::new(s_61_2 as u128, 1u16);
        // D s_61_7: cast zx s_61_5 -> bv
        let s_61_7: Bits = Bits::new(s_61_5 as u128, 1u16);
        // D s_61_8: cast reint s_61_6 -> u128
        let s_61_8: u128 = (s_61_6.value() as u128);
        // D s_61_9: size-of s_61_6
        let s_61_9: u16 = s_61_6.length();
        // D s_61_10: cast reint s_61_7 -> u128
        let s_61_10: u128 = (s_61_7.value() as u128);
        // D s_61_11: size-of s_61_7
        let s_61_11: u16 = s_61_7.length();
        // D s_61_12: lsl s_61_8 s_61_11
        let s_61_12: u128 = s_61_8 << s_61_11;
        // D s_61_13: or s_61_12 s_61_10
        let s_61_13: u128 = ((s_61_12) | (s_61_10));
        // D s_61_14: add s_61_9 s_61_11
        let s_61_14: u16 = (s_61_9 + s_61_11);
        // D s_61_15: create-bits s_61_13 s_61_14
        let s_61_15: Bits = Bits::new(s_61_13, s_61_14);
        // D s_61_16: cast reint s_61_15 -> u8
        let s_61_16: u8 = (s_61_15.value() as u8);
        // D s_61_17: cast zx s_61_16 -> bv
        let s_61_17: Bits = Bits::new(s_61_16 as u128, 2u16);
        // C s_61_18: const #3u : u8
        let s_61_18: u8 = 3;
        // C s_61_19: cast zx s_61_18 -> bv
        let s_61_19: Bits = Bits::new(s_61_18 as u128, 2u16);
        // D s_61_20: cmp-eq s_61_17 s_61_19
        let s_61_20: bool = ((s_61_17) == (s_61_19));
        // D s_61_21: write-var gs#81713 <= s_61_20
        fn_state.gs_81713 = s_61_20;
        // N s_61_22: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #64s : i64
        let s_62_0: i64 = 64;
        // D s_62_1: read-var t:i
        let s_62_1: i128 = fn_state.t;
        // D s_62_2: call X_read(s_62_1, s_62_0)
        let s_62_2: Bits = X_read(state, tracer, s_62_1, s_62_0);
        // D s_62_3: cast reint s_62_2 -> u64
        let s_62_3: u64 = (s_62_2.value() as u64);
        // D s_62_4: call Mk_CNTHVS_CVAL_EL2_Type(s_62_3)
        let s_62_4: ProductType5c790c8ef59cc8b2 = Mk_CNTHVS_CVAL_EL2_Type(
            state,
            tracer,
            s_62_3,
        );
        // C s_62_5: const #10064u : u32
        let s_62_5: u32 = 10064;
        // N s_62_6: write-reg s_62_5 <= s_62_4
        let s_62_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_62_5 as isize, s_62_4);
            tracer.write_register(s_62_5 as isize, s_62_4);
        };
        // N s_62_7: return
        return;
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #117u : u32
        let s_63_0: u32 = 117;
        // S s_63_1: call IsFeatureImplemented(s_63_0)
        let s_63_1: bool = IsFeatureImplemented(state, tracer, s_63_0);
        // D s_63_2: write-var gs#81712 <= s_63_1
        fn_state.gs_81712 = s_63_1;
        // N s_63_3: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #90704u : u32
        let s_64_0: u32 = 90704;
        // D s_64_1: read-reg s_64_0:struct
        let s_64_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_64_0 as isize);
            tracer.read_register(s_64_0 as isize, value);
            value
        };
        // D s_64_2: call _get_SCR_EL3_Type_NS(s_64_1)
        let s_64_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_64_1);
        // D s_64_3: cast zx s_64_2 -> bv
        let s_64_3: Bits = Bits::new(s_64_2 as u128, 1u16);
        // C s_64_4: const #0u : u8
        let s_64_4: bool = false;
        // C s_64_5: cast zx s_64_4 -> bv
        let s_64_5: Bits = Bits::new(s_64_4 as u128, 1u16);
        // D s_64_6: cmp-eq s_64_3 s_64_5
        let s_64_6: bool = ((s_64_3) == (s_64_5));
        // D s_64_7: write-var gs#81711 <= s_64_6
        fn_state.gs_81711 = s_64_6;
        // N s_64_8: jump b50
        return block_50(state, tracer, fn_state);
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
        // D s_65_20: cmp-eq s_65_17 s_65_19
        let s_65_20: bool = ((s_65_17) == (s_65_19));
        // D s_65_21: write-var gs#81710 <= s_65_20
        fn_state.gs_81710 = s_65_20;
        // N s_65_22: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #24u : u8
        let s_66_0: u8 = 24;
        // C s_66_1: cast zx s_66_0 -> bv
        let s_66_1: Bits = Bits::new(s_66_0 as u128, 8u16);
        // C s_66_2: cast zx s_66_1 -> i
        let s_66_2: i128 = (s_66_1.value() as i128);
        // C s_66_3: cast reint s_66_2 -> i64
        let s_66_3: i64 = (s_66_2 as i64);
        // C s_66_4: cast zx s_66_3 -> i
        let s_66_4: i128 = (i128::try_from(s_66_3).unwrap());
        // C s_66_5: const #432u : u32
        let s_66_5: u32 = 432;
        // D s_66_6: read-reg s_66_5:u8
        let s_66_6: u8 = {
            let value = state.read_register::<u8>(s_66_5 as isize);
            tracer.read_register(s_66_5 as isize, value);
            value
        };
        // D s_66_7: call AArch64_SystemAccessTrap(s_66_6, s_66_4)
        let s_66_7: () = AArch64_SystemAccessTrap(state, tracer, s_66_6, s_66_4);
        // N s_66_8: return
        return;
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var __CNTHCTL_EL2_EL1TVT:u8
        let s_67_0: bool = fn_state.u__CNTHCTL_EL2_EL1TVT;
        // D s_67_1: cast zx s_67_0 -> bv
        let s_67_1: Bits = Bits::new(s_67_0 as u128, 1u16);
        // C s_67_2: const #1u : u8
        let s_67_2: bool = true;
        // C s_67_3: cast zx s_67_2 -> bv
        let s_67_3: Bits = Bits::new(s_67_2 as u128, 1u16);
        // D s_67_4: cmp-eq s_67_1 s_67_3
        let s_67_4: bool = ((s_67_1) == (s_67_3));
        // D s_67_5: write-var gs#81709 <= s_67_4
        fn_state.gs_81709 = s_67_4;
        // N s_67_6: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #102552u : u32
        let s_68_0: u32 = 102552;
        // D s_68_1: read-reg s_68_0:struct
        let s_68_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_68_0 as isize);
            tracer.read_register(s_68_0 as isize, value);
            value
        };
        // D s_68_2: call _get_HCR_EL2_Type_E2H(s_68_1)
        let s_68_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_68_1);
        // C s_68_3: const #102552u : u32
        let s_68_3: u32 = 102552;
        // D s_68_4: read-reg s_68_3:struct
        let s_68_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_68_3 as isize);
            tracer.read_register(s_68_3 as isize, value);
            value
        };
        // D s_68_5: call _get_HCR_EL2_Type_TGE(s_68_4)
        let s_68_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_68_4);
        // D s_68_6: cast zx s_68_2 -> bv
        let s_68_6: Bits = Bits::new(s_68_2 as u128, 1u16);
        // D s_68_7: cast zx s_68_5 -> bv
        let s_68_7: Bits = Bits::new(s_68_5 as u128, 1u16);
        // D s_68_8: cast reint s_68_6 -> u128
        let s_68_8: u128 = (s_68_6.value() as u128);
        // D s_68_9: size-of s_68_6
        let s_68_9: u16 = s_68_6.length();
        // D s_68_10: cast reint s_68_7 -> u128
        let s_68_10: u128 = (s_68_7.value() as u128);
        // D s_68_11: size-of s_68_7
        let s_68_11: u16 = s_68_7.length();
        // D s_68_12: lsl s_68_8 s_68_11
        let s_68_12: u128 = s_68_8 << s_68_11;
        // D s_68_13: or s_68_12 s_68_10
        let s_68_13: u128 = ((s_68_12) | (s_68_10));
        // D s_68_14: add s_68_9 s_68_11
        let s_68_14: u16 = (s_68_9 + s_68_11);
        // D s_68_15: create-bits s_68_13 s_68_14
        let s_68_15: Bits = Bits::new(s_68_13, s_68_14);
        // D s_68_16: cast reint s_68_15 -> u8
        let s_68_16: u8 = (s_68_15.value() as u8);
        // D s_68_17: cast zx s_68_16 -> bv
        let s_68_17: Bits = Bits::new(s_68_16 as u128, 2u16);
        // C s_68_18: const #3u : u8
        let s_68_18: u8 = 3;
        // C s_68_19: cast zx s_68_18 -> bv
        let s_68_19: Bits = Bits::new(s_68_18 as u128, 2u16);
        // D s_68_20: cmp-ne s_68_17 s_68_19
        let s_68_20: bool = ((s_68_17) != (s_68_19));
        // D s_68_21: write-var gs#81708 <= s_68_20
        fn_state.gs_81708 = s_68_20;
        // N s_68_22: jump b43
        return block_43(state, tracer, fn_state);
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
        // C s_69_5: const #432u : u32
        let s_69_5: u32 = 432;
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
        // D s_70_0: read-var __CNTHCTL_EL2_EL0VTEN:u8
        let s_70_0: bool = fn_state.u__CNTHCTL_EL2_EL0VTEN;
        // D s_70_1: cast zx s_70_0 -> bv
        let s_70_1: Bits = Bits::new(s_70_0 as u128, 1u16);
        // C s_70_2: const #0u : u8
        let s_70_2: bool = false;
        // C s_70_3: cast zx s_70_2 -> bv
        let s_70_3: Bits = Bits::new(s_70_2 as u128, 1u16);
        // D s_70_4: cmp-eq s_70_1 s_70_3
        let s_70_4: bool = ((s_70_1) == (s_70_3));
        // D s_70_5: write-var gs#81707 <= s_70_4
        fn_state.gs_81707 = s_70_4;
        // N s_70_6: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #102552u : u32
        let s_71_0: u32 = 102552;
        // D s_71_1: read-reg s_71_0:struct
        let s_71_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_71_0 as isize);
            tracer.read_register(s_71_0 as isize, value);
            value
        };
        // D s_71_2: call _get_HCR_EL2_Type_E2H(s_71_1)
        let s_71_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_71_1);
        // C s_71_3: const #102552u : u32
        let s_71_3: u32 = 102552;
        // D s_71_4: read-reg s_71_3:struct
        let s_71_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_71_3 as isize);
            tracer.read_register(s_71_3 as isize, value);
            value
        };
        // D s_71_5: call _get_HCR_EL2_Type_TGE(s_71_4)
        let s_71_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_71_4);
        // D s_71_6: cast zx s_71_2 -> bv
        let s_71_6: Bits = Bits::new(s_71_2 as u128, 1u16);
        // D s_71_7: cast zx s_71_5 -> bv
        let s_71_7: Bits = Bits::new(s_71_5 as u128, 1u16);
        // D s_71_8: cast reint s_71_6 -> u128
        let s_71_8: u128 = (s_71_6.value() as u128);
        // D s_71_9: size-of s_71_6
        let s_71_9: u16 = s_71_6.length();
        // D s_71_10: cast reint s_71_7 -> u128
        let s_71_10: u128 = (s_71_7.value() as u128);
        // D s_71_11: size-of s_71_7
        let s_71_11: u16 = s_71_7.length();
        // D s_71_12: lsl s_71_8 s_71_11
        let s_71_12: u128 = s_71_8 << s_71_11;
        // D s_71_13: or s_71_12 s_71_10
        let s_71_13: u128 = ((s_71_12) | (s_71_10));
        // D s_71_14: add s_71_9 s_71_11
        let s_71_14: u16 = (s_71_9 + s_71_11);
        // D s_71_15: create-bits s_71_13 s_71_14
        let s_71_15: Bits = Bits::new(s_71_13, s_71_14);
        // D s_71_16: cast reint s_71_15 -> u8
        let s_71_16: u8 = (s_71_15.value() as u8);
        // D s_71_17: cast zx s_71_16 -> bv
        let s_71_17: Bits = Bits::new(s_71_16 as u128, 2u16);
        // C s_71_18: const #3u : u8
        let s_71_18: u8 = 3;
        // C s_71_19: cast zx s_71_18 -> bv
        let s_71_19: Bits = Bits::new(s_71_18 as u128, 2u16);
        // D s_71_20: cmp-eq s_71_17 s_71_19
        let s_71_20: bool = ((s_71_17) == (s_71_19));
        // D s_71_21: write-var gs#81706 <= s_71_20
        fn_state.gs_81706 = s_71_20;
        // N s_71_22: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #() : ()
        let s_72_0: () = ();
        // S s_72_1: call EL2Enabled(s_72_0)
        let s_72_1: bool = EL2Enabled(state, tracer, s_72_0);
        // N s_72_2: branch s_72_1 b77 b73
        if s_72_1 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #0u : u8
        let s_73_0: bool = false;
        // D s_73_1: write-var gs#81718 <= s_73_0
        fn_state.gs_81718 = s_73_0;
        // N s_73_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var gs#81718:u8
        let s_74_0: bool = fn_state.gs_81718;
        // N s_74_1: branch s_74_0 b76 b75
        if s_74_0 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #24u : u8
        let s_75_0: u8 = 24;
        // C s_75_1: cast zx s_75_0 -> bv
        let s_75_1: Bits = Bits::new(s_75_0 as u128, 8u16);
        // C s_75_2: cast zx s_75_1 -> i
        let s_75_2: i128 = (s_75_1.value() as i128);
        // C s_75_3: cast reint s_75_2 -> i64
        let s_75_3: i64 = (s_75_2 as i64);
        // C s_75_4: cast zx s_75_3 -> i
        let s_75_4: i128 = (i128::try_from(s_75_3).unwrap());
        // C s_75_5: const #440u : u32
        let s_75_5: u32 = 440;
        // D s_75_6: read-reg s_75_5:u8
        let s_75_6: u8 = {
            let value = state.read_register::<u8>(s_75_5 as isize);
            tracer.read_register(s_75_5 as isize, value);
            value
        };
        // D s_75_7: call AArch64_SystemAccessTrap(s_75_6, s_75_4)
        let s_75_7: () = AArch64_SystemAccessTrap(state, tracer, s_75_6, s_75_4);
        // N s_75_8: return
        return;
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
        // D s_77_0: read-var __HCR_EL2_TGE:u8
        let s_77_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_77_1: cast zx s_77_0 -> bv
        let s_77_1: Bits = Bits::new(s_77_0 as u128, 1u16);
        // C s_77_2: const #1u : u8
        let s_77_2: bool = true;
        // C s_77_3: cast zx s_77_2 -> bv
        let s_77_3: Bits = Bits::new(s_77_2 as u128, 1u16);
        // D s_77_4: cmp-eq s_77_1 s_77_3
        let s_77_4: bool = ((s_77_1) == (s_77_3));
        // D s_77_5: write-var gs#81718 <= s_77_4
        fn_state.gs_81718 = s_77_4;
        // N s_77_6: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var __CNTKCTL_EL1_EL0VTEN:u8
        let s_78_0: bool = fn_state.u__CNTKCTL_EL1_EL0VTEN;
        // D s_78_1: cast zx s_78_0 -> bv
        let s_78_1: Bits = Bits::new(s_78_0 as u128, 1u16);
        // C s_78_2: const #0u : u8
        let s_78_2: bool = false;
        // C s_78_3: cast zx s_78_2 -> bv
        let s_78_3: Bits = Bits::new(s_78_2 as u128, 1u16);
        // D s_78_4: cmp-eq s_78_1 s_78_3
        let s_78_4: bool = ((s_78_1) == (s_78_3));
        // D s_78_5: write-var gs#81705 <= s_78_4
        fn_state.gs_81705 = s_78_4;
        // N s_78_6: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #102552u : u32
        let s_79_0: u32 = 102552;
        // D s_79_1: read-reg s_79_0:struct
        let s_79_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_79_0 as isize);
            tracer.read_register(s_79_0 as isize, value);
            value
        };
        // D s_79_2: call _get_HCR_EL2_Type_E2H(s_79_1)
        let s_79_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_79_1);
        // C s_79_3: const #102552u : u32
        let s_79_3: u32 = 102552;
        // D s_79_4: read-reg s_79_3:struct
        let s_79_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_79_3 as isize);
            tracer.read_register(s_79_3 as isize, value);
            value
        };
        // D s_79_5: call _get_HCR_EL2_Type_TGE(s_79_4)
        let s_79_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_79_4);
        // D s_79_6: cast zx s_79_2 -> bv
        let s_79_6: Bits = Bits::new(s_79_2 as u128, 1u16);
        // D s_79_7: cast zx s_79_5 -> bv
        let s_79_7: Bits = Bits::new(s_79_5 as u128, 1u16);
        // D s_79_8: cast reint s_79_6 -> u128
        let s_79_8: u128 = (s_79_6.value() as u128);
        // D s_79_9: size-of s_79_6
        let s_79_9: u16 = s_79_6.length();
        // D s_79_10: cast reint s_79_7 -> u128
        let s_79_10: u128 = (s_79_7.value() as u128);
        // D s_79_11: size-of s_79_7
        let s_79_11: u16 = s_79_7.length();
        // D s_79_12: lsl s_79_8 s_79_11
        let s_79_12: u128 = s_79_8 << s_79_11;
        // D s_79_13: or s_79_12 s_79_10
        let s_79_13: u128 = ((s_79_12) | (s_79_10));
        // D s_79_14: add s_79_9 s_79_11
        let s_79_14: u16 = (s_79_9 + s_79_11);
        // D s_79_15: create-bits s_79_13 s_79_14
        let s_79_15: Bits = Bits::new(s_79_13, s_79_14);
        // D s_79_16: cast reint s_79_15 -> u8
        let s_79_16: u8 = (s_79_15.value() as u8);
        // D s_79_17: cast zx s_79_16 -> bv
        let s_79_17: Bits = Bits::new(s_79_16 as u128, 2u16);
        // C s_79_18: const #3u : u8
        let s_79_18: u8 = 3;
        // C s_79_19: cast zx s_79_18 -> bv
        let s_79_19: Bits = Bits::new(s_79_18 as u128, 2u16);
        // D s_79_20: cmp-eq s_79_17 s_79_19
        let s_79_20: bool = ((s_79_17) == (s_79_19));
        // D s_79_21: write-var gs#81704 <= s_79_20
        fn_state.gs_81704 = s_79_20;
        // N s_79_22: jump b33
        return block_33(state, tracer, fn_state);
    }
}
