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
use u_get_HCR_EL2_Type_NV1::*;
use u_get_HCR_EL2_Type_E2H::*;
use u_get_HCR_EL2_Type_NV::*;
use Mk_CNTV_CVAL_EL0_Type::*;
use ELUsingAArch32::*;
use u_get_CNTHCTL_EL2_Type_EL1NVVCT::*;
use X_read::*;
use AArch64_SystemAccessTrap::*;
use EL2Enabled::*;
use NVMem_set::*;
use u_get_HCR_EL2_Type_TGE::*;
use u_get_HCR_EL2_Type_NV2::*;
use common::*;
pub fn CNTV_CVAL_EL0_SysRegWrite_ad2fa653919bf7cc<T: Tracer>(
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
        u__HCR_EL2_E2H: bool,
        gs_81952: bool,
        gs_81959: bool,
        gs_81953: bool,
        gs_81957: bool,
        u__CNTHCTL_EL2_EL1NVVCT: bool,
        gs_81956: bool,
        u__PSTATE_EL: u8,
        u__HCR_EL2_NV: bool,
        gs_81958: bool,
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
        // C s_0_3: const #12808u : u32
        let s_0_3: u32 = 12808;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_CNTHCTL_EL2_Type_EL1NVVCT(s_0_4)
        let s_0_5: bool = u_get_CNTHCTL_EL2_Type_EL1NVVCT(state, tracer, s_0_4);
        // D s_0_6: write-var __CNTHCTL_EL2_EL1NVVCT <= s_0_5
        fn_state.u__CNTHCTL_EL2_EL1NVVCT = s_0_5;
        // C s_0_7: const #102552u : u32
        let s_0_7: u32 = 102552;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_HCR_EL2_Type_NV(s_0_8)
        let s_0_9: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_0_8);
        // D s_0_10: write-var __HCR_EL2_NV <= s_0_9
        fn_state.u__HCR_EL2_NV = s_0_9;
        // C s_0_11: const #102552u : u32
        let s_0_11: u32 = 102552;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_HCR_EL2_Type_E2H(s_0_12)
        let s_0_13: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_0_12);
        // D s_0_14: write-var __HCR_EL2_E2H <= s_0_13
        fn_state.u__HCR_EL2_E2H = s_0_13;
        // D s_0_15: read-var __PSTATE_EL:u8
        let s_0_15: u8 = fn_state.u__PSTATE_EL;
        // D s_0_16: cast zx s_0_15 -> bv
        let s_0_16: Bits = Bits::new(s_0_15 as u128, 2u16);
        // C s_0_17: const #448u : u32
        let s_0_17: u32 = 448;
        // D s_0_18: read-reg s_0_17:u8
        let s_0_18: u8 = {
            let value = state.read_register::<u8>(s_0_17 as isize);
            tracer.read_register(s_0_17 as isize, value);
            value
        };
        // D s_0_19: cast zx s_0_18 -> bv
        let s_0_19: Bits = Bits::new(s_0_18 as u128, 2u16);
        // D s_0_20: cmp-eq s_0_16 s_0_19
        let s_0_20: bool = ((s_0_16) == (s_0_19));
        // N s_0_21: branch s_0_20 b36 b1
        if s_0_20 {
            return block_36(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b17 b2
        if s_1_5 {
            return block_17(state, tracer, fn_state);
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
        // N s_2_6: branch s_2_5 b14 b3
        if s_2_5 {
            return block_14(state, tracer, fn_state);
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
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call EL2Enabled(s_5_0)
        let s_5_1: bool = EL2Enabled(state, tracer, s_5_0);
        // N s_5_2: branch s_5_1 b13 b6
        if s_5_1 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#81952 <= s_6_0
        fn_state.gs_81952 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#81952:u8
        let s_7_0: bool = fn_state.gs_81952;
        // N s_7_1: branch s_7_0 b12 b8
        if s_7_0 {
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
        // D s_8_1: write-var gs#81953 <= s_8_0
        fn_state.gs_81953 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#81953:u8
        let s_9_0: bool = fn_state.gs_81953;
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
        // N s_10_0: panic
        panic!("{:?}", ());
        // N s_10_1: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #64s : i64
        let s_11_0: i64 = 64;
        // D s_11_1: read-var t:i
        let s_11_1: i128 = fn_state.t;
        // D s_11_2: call X_read(s_11_1, s_11_0)
        let s_11_2: Bits = X_read(state, tracer, s_11_1, s_11_0);
        // D s_11_3: cast reint s_11_2 -> u64
        let s_11_3: u64 = (s_11_2.value() as u64);
        // D s_11_4: call Mk_CNTV_CVAL_EL0_Type(s_11_3)
        let s_11_4: ProductType5c790c8ef59cc8b2 = Mk_CNTV_CVAL_EL0_Type(
            state,
            tracer,
            s_11_3,
        );
        // C s_11_5: const #23632u : u32
        let s_11_5: u32 = 23632;
        // N s_11_6: write-reg s_11_5 <= s_11_4
        let s_11_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_11_5 as isize, s_11_4);
            tracer.write_register(s_11_5 as isize, s_11_4);
        };
        // N s_11_7: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var __HCR_EL2_E2H:u8
        let s_12_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 1u16);
        // C s_12_2: const #1u : u8
        let s_12_2: bool = true;
        // C s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 1u16);
        // D s_12_4: cmp-eq s_12_1 s_12_3
        let s_12_4: bool = ((s_12_1) == (s_12_3));
        // D s_12_5: write-var gs#81953 <= s_12_4
        fn_state.gs_81953 = s_12_4;
        // N s_12_6: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #432u : u32
        let s_13_0: u32 = 432;
        // D s_13_1: read-reg s_13_0:u8
        let s_13_1: u8 = {
            let value = state.read_register::<u8>(s_13_0 as isize);
            tracer.read_register(s_13_0 as isize, value);
            value
        };
        // D s_13_2: call ELUsingAArch32(s_13_1)
        let s_13_2: bool = ELUsingAArch32(state, tracer, s_13_1);
        // D s_13_3: not s_13_2
        let s_13_3: bool = !s_13_2;
        // D s_13_4: write-var gs#81952 <= s_13_3
        fn_state.gs_81952 = s_13_3;
        // N s_13_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var __HCR_EL2_E2H:u8
        let s_14_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 1u16);
        // C s_14_2: const #1u : u8
        let s_14_2: bool = true;
        // C s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 1u16);
        // D s_14_4: cmp-eq s_14_1 s_14_3
        let s_14_4: bool = ((s_14_1) == (s_14_3));
        // N s_14_5: branch s_14_4 b16 b15
        if s_14_4 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: panic
        panic!("{:?}", ());
        // N s_15_1: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #64s : i64
        let s_16_0: i64 = 64;
        // D s_16_1: read-var t:i
        let s_16_1: i128 = fn_state.t;
        // D s_16_2: call X_read(s_16_1, s_16_0)
        let s_16_2: Bits = X_read(state, tracer, s_16_1, s_16_0);
        // D s_16_3: cast reint s_16_2 -> u64
        let s_16_3: u64 = (s_16_2.value() as u64);
        // D s_16_4: call Mk_CNTV_CVAL_EL0_Type(s_16_3)
        let s_16_4: ProductType5c790c8ef59cc8b2 = Mk_CNTV_CVAL_EL0_Type(
            state,
            tracer,
            s_16_3,
        );
        // C s_16_5: const #23632u : u32
        let s_16_5: u32 = 23632;
        // N s_16_6: write-reg s_16_5 <= s_16_4
        let s_16_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_16_5 as isize, s_16_4);
            tracer.write_register(s_16_5 as isize, s_16_4);
        };
        // N s_16_7: return
        return;
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
        // N s_17_2: branch s_17_1 b35 b18
        if s_17_1 {
            return block_35(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#81956 <= s_18_0
        fn_state.gs_81956 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#81956:u8
        let s_19_0: bool = fn_state.gs_81956;
        // N s_19_1: branch s_19_0 b26 b20
        if s_19_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
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
        // N s_20_2: branch s_20_1 b25 b21
        if s_20_1 {
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
        // D s_21_1: write-var gs#81957 <= s_21_0
        fn_state.gs_81957 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#81957:u8
        let s_22_0: bool = fn_state.gs_81957;
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
        // D s_25_0: read-var __HCR_EL2_NV:u8
        let s_25_0: bool = fn_state.u__HCR_EL2_NV;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 1u16);
        // C s_25_2: const #1u : u8
        let s_25_2: bool = true;
        // C s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // D s_25_4: cmp-eq s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) == (s_25_3));
        // D s_25_5: write-var gs#81957 <= s_25_4
        fn_state.gs_81957 = s_25_4;
        // N s_25_6: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #() : ()
        let s_26_0: () = ();
        // S s_26_1: call EL2Enabled(s_26_0)
        let s_26_1: bool = EL2Enabled(state, tracer, s_26_0);
        // N s_26_2: branch s_26_1 b34 b27
        if s_26_1 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // D s_27_1: write-var gs#81958 <= s_27_0
        fn_state.gs_81958 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#81958:u8
        let s_28_0: bool = fn_state.gs_81958;
        // N s_28_1: branch s_28_0 b33 b29
        if s_28_0 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #0u : u8
        let s_29_0: bool = false;
        // D s_29_1: write-var gs#81959 <= s_29_0
        fn_state.gs_81959 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#81959:u8
        let s_30_0: bool = fn_state.gs_81959;
        // N s_30_1: branch s_30_0 b32 b31
        if s_30_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #360u : u12
        let s_31_0: u16 = 360;
        // C s_31_1: cast zx s_31_0 -> bv
        let s_31_1: Bits = Bits::new(s_31_0 as u128, 12u16);
        // C s_31_2: cast zx s_31_1 -> i
        let s_31_2: i128 = (s_31_1.value() as i128);
        // C s_31_3: cast reint s_31_2 -> i64
        let s_31_3: i64 = (s_31_2 as i64);
        // C s_31_4: const #64s : i64
        let s_31_4: i64 = 64;
        // D s_31_5: read-var t:i
        let s_31_5: i128 = fn_state.t;
        // D s_31_6: call X_read(s_31_5, s_31_4)
        let s_31_6: Bits = X_read(state, tracer, s_31_5, s_31_4);
        // D s_31_7: cast reint s_31_6 -> u64
        let s_31_7: u64 = (s_31_6.value() as u64);
        // C s_31_8: cast zx s_31_3 -> i
        let s_31_8: i128 = (i128::try_from(s_31_3).unwrap());
        // D s_31_9: call NVMem_set(s_31_8, s_31_7)
        let s_31_9: () = NVMem_set(state, tracer, s_31_8, s_31_7);
        // N s_31_10: return
        return;
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #24u : u8
        let s_32_0: u8 = 24;
        // C s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 8u16);
        // C s_32_2: cast zx s_32_1 -> i
        let s_32_2: i128 = (s_32_1.value() as i128);
        // C s_32_3: cast reint s_32_2 -> i64
        let s_32_3: i64 = (s_32_2 as i64);
        // C s_32_4: cast zx s_32_3 -> i
        let s_32_4: i128 = (i128::try_from(s_32_3).unwrap());
        // C s_32_5: const #432u : u32
        let s_32_5: u32 = 432;
        // D s_32_6: read-reg s_32_5:u8
        let s_32_6: u8 = {
            let value = state.read_register::<u8>(s_32_5 as isize);
            tracer.read_register(s_32_5 as isize, value);
            value
        };
        // D s_32_7: call AArch64_SystemAccessTrap(s_32_6, s_32_4)
        let s_32_7: () = AArch64_SystemAccessTrap(state, tracer, s_32_6, s_32_4);
        // N s_32_8: return
        return;
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var __CNTHCTL_EL2_EL1NVVCT:u8
        let s_33_0: bool = fn_state.u__CNTHCTL_EL2_EL1NVVCT;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 1u16);
        // C s_33_2: const #1u : u8
        let s_33_2: bool = true;
        // C s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 1u16);
        // D s_33_4: cmp-eq s_33_1 s_33_3
        let s_33_4: bool = ((s_33_1) == (s_33_3));
        // D s_33_5: write-var gs#81959 <= s_33_4
        fn_state.gs_81959 = s_33_4;
        // N s_33_6: jump b30
        return block_30(state, tracer, fn_state);
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
        // D s_34_20: cmp-ne s_34_17 s_34_19
        let s_34_20: bool = ((s_34_17) != (s_34_19));
        // D s_34_21: write-var gs#81958 <= s_34_20
        fn_state.gs_81958 = s_34_20;
        // N s_34_22: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #102552u : u32
        let s_35_0: u32 = 102552;
        // D s_35_1: read-reg s_35_0:struct
        let s_35_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_35_0 as isize);
            tracer.read_register(s_35_0 as isize, value);
            value
        };
        // D s_35_2: call _get_HCR_EL2_Type_NV2(s_35_1)
        let s_35_2: bool = u_get_HCR_EL2_Type_NV2(state, tracer, s_35_1);
        // C s_35_3: const #102552u : u32
        let s_35_3: u32 = 102552;
        // D s_35_4: read-reg s_35_3:struct
        let s_35_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_35_3 as isize);
            tracer.read_register(s_35_3 as isize, value);
            value
        };
        // D s_35_5: call _get_HCR_EL2_Type_NV1(s_35_4)
        let s_35_5: bool = u_get_HCR_EL2_Type_NV1(state, tracer, s_35_4);
        // C s_35_6: const #102552u : u32
        let s_35_6: u32 = 102552;
        // D s_35_7: read-reg s_35_6:struct
        let s_35_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_35_6 as isize);
            tracer.read_register(s_35_6 as isize, value);
            value
        };
        // D s_35_8: call _get_HCR_EL2_Type_NV(s_35_7)
        let s_35_8: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_35_7);
        // D s_35_9: cast zx s_35_5 -> bv
        let s_35_9: Bits = Bits::new(s_35_5 as u128, 1u16);
        // D s_35_10: cast zx s_35_8 -> bv
        let s_35_10: Bits = Bits::new(s_35_8 as u128, 1u16);
        // D s_35_11: cast reint s_35_9 -> u128
        let s_35_11: u128 = (s_35_9.value() as u128);
        // D s_35_12: size-of s_35_9
        let s_35_12: u16 = s_35_9.length();
        // D s_35_13: cast reint s_35_10 -> u128
        let s_35_13: u128 = (s_35_10.value() as u128);
        // D s_35_14: size-of s_35_10
        let s_35_14: u16 = s_35_10.length();
        // D s_35_15: lsl s_35_11 s_35_14
        let s_35_15: u128 = s_35_11 << s_35_14;
        // D s_35_16: or s_35_15 s_35_13
        let s_35_16: u128 = ((s_35_15) | (s_35_13));
        // D s_35_17: add s_35_12 s_35_14
        let s_35_17: u16 = (s_35_12 + s_35_14);
        // D s_35_18: create-bits s_35_16 s_35_17
        let s_35_18: Bits = Bits::new(s_35_16, s_35_17);
        // D s_35_19: cast reint s_35_18 -> u8
        let s_35_19: u8 = (s_35_18.value() as u8);
        // D s_35_20: cast zx s_35_2 -> bv
        let s_35_20: Bits = Bits::new(s_35_2 as u128, 1u16);
        // D s_35_21: cast zx s_35_19 -> bv
        let s_35_21: Bits = Bits::new(s_35_19 as u128, 2u16);
        // D s_35_22: cast reint s_35_20 -> u128
        let s_35_22: u128 = (s_35_20.value() as u128);
        // D s_35_23: size-of s_35_20
        let s_35_23: u16 = s_35_20.length();
        // D s_35_24: cast reint s_35_21 -> u128
        let s_35_24: u128 = (s_35_21.value() as u128);
        // D s_35_25: size-of s_35_21
        let s_35_25: u16 = s_35_21.length();
        // D s_35_26: lsl s_35_22 s_35_25
        let s_35_26: u128 = s_35_22 << s_35_25;
        // D s_35_27: or s_35_26 s_35_24
        let s_35_27: u128 = ((s_35_26) | (s_35_24));
        // D s_35_28: add s_35_23 s_35_25
        let s_35_28: u16 = (s_35_23 + s_35_25);
        // D s_35_29: create-bits s_35_27 s_35_28
        let s_35_29: Bits = Bits::new(s_35_27, s_35_28);
        // D s_35_30: cast reint s_35_29 -> u8
        let s_35_30: u8 = (s_35_29.value() as u8);
        // D s_35_31: cast zx s_35_30 -> bv
        let s_35_31: Bits = Bits::new(s_35_30 as u128, 3u16);
        // C s_35_32: const #5u : u8
        let s_35_32: u8 = 5;
        // C s_35_33: cast zx s_35_32 -> bv
        let s_35_33: Bits = Bits::new(s_35_32 as u128, 3u16);
        // D s_35_34: cmp-eq s_35_31 s_35_33
        let s_35_34: bool = ((s_35_31) == (s_35_33));
        // D s_35_35: write-var gs#81956 <= s_35_34
        fn_state.gs_81956 = s_35_34;
        // N s_35_36: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_36_0: panic
        panic!("{:?}", ());
        // N s_36_1: return
        return;
    }
}
