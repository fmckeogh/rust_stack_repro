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
use AArch64_TLBI_RVA::*;
use u_get_HCR_EL2_Type_E2H::*;
use u_get_HCR_EL2_Type_NV::*;
use SecurityStateAtEL::*;
use X_read::*;
use VMID_read::*;
use EL2Enabled::*;
use AArch64_SystemAccessTrap::*;
use common::*;
pub fn TLBI_RVAE2_SysOpsWrite_b0982ca76d6c7651<T: Tracer>(
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
        u__PSTATE_EL: u8,
        u__HCR_EL2_NV: bool,
        gs_102456: bool,
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
        // C s_0_3: const #102552u : u32
        let s_0_3: u32 = 102552;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_HCR_EL2_Type_NV(s_0_4)
        let s_0_5: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_0_4);
        // D s_0_6: write-var __HCR_EL2_NV <= s_0_5
        fn_state.u__HCR_EL2_NV = s_0_5;
        // C s_0_7: const #102552u : u32
        let s_0_7: u32 = 102552;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_HCR_EL2_Type_E2H(s_0_8)
        let s_0_9: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_0_8);
        // D s_0_10: write-var __HCR_EL2_E2H <= s_0_9
        fn_state.u__HCR_EL2_E2H = s_0_9;
        // D s_0_11: read-var __PSTATE_EL:u8
        let s_0_11: u8 = fn_state.u__PSTATE_EL;
        // D s_0_12: cast zx s_0_11 -> bv
        let s_0_12: Bits = Bits::new(s_0_11 as u128, 2u16);
        // C s_0_13: const #448u : u32
        let s_0_13: u32 = 448;
        // D s_0_14: read-reg s_0_13:u8
        let s_0_14: u8 = {
            let value = state.read_register::<u8>(s_0_13 as isize);
            tracer.read_register(s_0_13 as isize, value);
            value
        };
        // D s_0_15: cast zx s_0_14 -> bv
        let s_0_15: Bits = Bits::new(s_0_14 as u128, 2u16);
        // D s_0_16: cmp-eq s_0_12 s_0_15
        let s_0_16: bool = ((s_0_12) == (s_0_15));
        // N s_0_17: branch s_0_16 b19 b1
        if s_0_16 {
            return block_19(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b13 b2
        if s_1_5 {
            return block_13(state, tracer, fn_state);
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
        // N s_2_6: branch s_2_5 b10 b3
        if s_2_5 {
            return block_10(state, tracer, fn_state);
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
        // S s_5_2: not s_5_1
        let s_5_2: bool = !s_5_1;
        // N s_5_3: branch s_5_2 b9 b6
        if s_5_2 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var __HCR_EL2_E2H:u8
        let s_6_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 1u16);
        // C s_6_2: const #1u : u8
        let s_6_2: bool = true;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // N s_6_5: branch s_6_4 b8 b7
        if s_6_4 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #432u : u32
        let s_7_0: u32 = 432;
        // D s_7_1: read-reg s_7_0:u8
        let s_7_1: u8 = {
            let value = state.read_register::<u8>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: call SecurityStateAtEL(s_7_1)
        let s_7_2: u32 = SecurityStateAtEL(state, tracer, s_7_1);
        // C s_7_3: const #() : ()
        let s_7_3: () = ();
        // S s_7_4: call VMID_read(s_7_3)
        let s_7_4: u16 = VMID_read(state, tracer, s_7_3);
        // C s_7_5: const #64s : i64
        let s_7_5: i64 = 64;
        // D s_7_6: read-var t:i
        let s_7_6: i128 = fn_state.t;
        // D s_7_7: call X_read(s_7_6, s_7_5)
        let s_7_7: Bits = X_read(state, tracer, s_7_6, s_7_5);
        // D s_7_8: cast reint s_7_7 -> u64
        let s_7_8: u64 = (s_7_7.value() as u64);
        // C s_7_9: const #2u : u32
        let s_7_9: u32 = 2;
        // C s_7_10: const #0u : u32
        let s_7_10: u32 = 0;
        // C s_7_11: const #0u : u32
        let s_7_11: u32 = 0;
        // C s_7_12: const #0u : u32
        let s_7_12: u32 = 0;
        // D s_7_13: call AArch64_TLBI_RVA(s_7_2, s_7_9, s_7_4, s_7_10, s_7_11, s_7_12, s_7_8)
        let s_7_13: () = AArch64_TLBI_RVA(
            state,
            tracer,
            s_7_2,
            s_7_9,
            s_7_4,
            s_7_10,
            s_7_11,
            s_7_12,
            s_7_8,
        );
        // N s_7_14: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #432u : u32
        let s_8_0: u32 = 432;
        // D s_8_1: read-reg s_8_0:u8
        let s_8_1: u8 = {
            let value = state.read_register::<u8>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: call SecurityStateAtEL(s_8_1)
        let s_8_2: u32 = SecurityStateAtEL(state, tracer, s_8_1);
        // C s_8_3: const #64s : i64
        let s_8_3: i64 = 64;
        // D s_8_4: read-var t:i
        let s_8_4: i128 = fn_state.t;
        // D s_8_5: call X_read(s_8_4, s_8_3)
        let s_8_5: Bits = X_read(state, tracer, s_8_4, s_8_3);
        // D s_8_6: cast reint s_8_5 -> u64
        let s_8_6: u64 = (s_8_5.value() as u64);
        // C s_8_7: const #3u : u32
        let s_8_7: u32 = 3;
        // C s_8_8: const #1080u : u32
        let s_8_8: u32 = 1080;
        // D s_8_9: read-reg s_8_8:u16
        let s_8_9: u16 = {
            let value = state.read_register::<u16>(s_8_8 as isize);
            tracer.read_register(s_8_8 as isize, value);
            value
        };
        // C s_8_10: const #0u : u32
        let s_8_10: u32 = 0;
        // C s_8_11: const #0u : u32
        let s_8_11: u32 = 0;
        // C s_8_12: const #0u : u32
        let s_8_12: u32 = 0;
        // D s_8_13: call AArch64_TLBI_RVA(s_8_2, s_8_7, s_8_9, s_8_10, s_8_11, s_8_12, s_8_6)
        let s_8_13: () = AArch64_TLBI_RVA(
            state,
            tracer,
            s_8_2,
            s_8_7,
            s_8_9,
            s_8_10,
            s_8_11,
            s_8_12,
            s_8_6,
        );
        // N s_8_14: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: panic
        panic!("{:?}", ());
        // N s_9_1: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var __HCR_EL2_E2H:u8
        let s_10_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 1u16);
        // C s_10_2: const #1u : u8
        let s_10_2: bool = true;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // N s_10_5: branch s_10_4 b12 b11
        if s_10_4 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #432u : u32
        let s_11_0: u32 = 432;
        // D s_11_1: read-reg s_11_0:u8
        let s_11_1: u8 = {
            let value = state.read_register::<u8>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // D s_11_2: call SecurityStateAtEL(s_11_1)
        let s_11_2: u32 = SecurityStateAtEL(state, tracer, s_11_1);
        // C s_11_3: const #() : ()
        let s_11_3: () = ();
        // S s_11_4: call VMID_read(s_11_3)
        let s_11_4: u16 = VMID_read(state, tracer, s_11_3);
        // C s_11_5: const #64s : i64
        let s_11_5: i64 = 64;
        // D s_11_6: read-var t:i
        let s_11_6: i128 = fn_state.t;
        // D s_11_7: call X_read(s_11_6, s_11_5)
        let s_11_7: Bits = X_read(state, tracer, s_11_6, s_11_5);
        // D s_11_8: cast reint s_11_7 -> u64
        let s_11_8: u64 = (s_11_7.value() as u64);
        // C s_11_9: const #2u : u32
        let s_11_9: u32 = 2;
        // C s_11_10: const #0u : u32
        let s_11_10: u32 = 0;
        // C s_11_11: const #0u : u32
        let s_11_11: u32 = 0;
        // C s_11_12: const #0u : u32
        let s_11_12: u32 = 0;
        // D s_11_13: call AArch64_TLBI_RVA(s_11_2, s_11_9, s_11_4, s_11_10, s_11_11, s_11_12, s_11_8)
        let s_11_13: () = AArch64_TLBI_RVA(
            state,
            tracer,
            s_11_2,
            s_11_9,
            s_11_4,
            s_11_10,
            s_11_11,
            s_11_12,
            s_11_8,
        );
        // N s_11_14: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #432u : u32
        let s_12_0: u32 = 432;
        // D s_12_1: read-reg s_12_0:u8
        let s_12_1: u8 = {
            let value = state.read_register::<u8>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: call SecurityStateAtEL(s_12_1)
        let s_12_2: u32 = SecurityStateAtEL(state, tracer, s_12_1);
        // C s_12_3: const #64s : i64
        let s_12_3: i64 = 64;
        // D s_12_4: read-var t:i
        let s_12_4: i128 = fn_state.t;
        // D s_12_5: call X_read(s_12_4, s_12_3)
        let s_12_5: Bits = X_read(state, tracer, s_12_4, s_12_3);
        // D s_12_6: cast reint s_12_5 -> u64
        let s_12_6: u64 = (s_12_5.value() as u64);
        // C s_12_7: const #3u : u32
        let s_12_7: u32 = 3;
        // C s_12_8: const #1080u : u32
        let s_12_8: u32 = 1080;
        // D s_12_9: read-reg s_12_8:u16
        let s_12_9: u16 = {
            let value = state.read_register::<u16>(s_12_8 as isize);
            tracer.read_register(s_12_8 as isize, value);
            value
        };
        // C s_12_10: const #0u : u32
        let s_12_10: u32 = 0;
        // C s_12_11: const #0u : u32
        let s_12_11: u32 = 0;
        // C s_12_12: const #0u : u32
        let s_12_12: u32 = 0;
        // D s_12_13: call AArch64_TLBI_RVA(s_12_2, s_12_7, s_12_9, s_12_10, s_12_11, s_12_12, s_12_6)
        let s_12_13: () = AArch64_TLBI_RVA(
            state,
            tracer,
            s_12_2,
            s_12_7,
            s_12_9,
            s_12_10,
            s_12_11,
            s_12_12,
            s_12_6,
        );
        // N s_12_14: return
        return;
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
        // N s_13_2: branch s_13_1 b18 b14
        if s_13_1 {
            return block_18(state, tracer, fn_state);
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
        // D s_14_1: write-var gs#102456 <= s_14_0
        fn_state.gs_102456 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#102456:u8
        let s_15_0: bool = fn_state.gs_102456;
        // N s_15_1: branch s_15_0 b17 b16
        if s_15_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: panic
        panic!("{:?}", ());
        // N s_16_1: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #24u : u8
        let s_17_0: u8 = 24;
        // C s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 8u16);
        // C s_17_2: cast zx s_17_1 -> i
        let s_17_2: i128 = (s_17_1.value() as i128);
        // C s_17_3: cast reint s_17_2 -> i64
        let s_17_3: i64 = (s_17_2 as i64);
        // C s_17_4: cast zx s_17_3 -> i
        let s_17_4: i128 = (i128::try_from(s_17_3).unwrap());
        // C s_17_5: const #432u : u32
        let s_17_5: u32 = 432;
        // D s_17_6: read-reg s_17_5:u8
        let s_17_6: u8 = {
            let value = state.read_register::<u8>(s_17_5 as isize);
            tracer.read_register(s_17_5 as isize, value);
            value
        };
        // D s_17_7: call AArch64_SystemAccessTrap(s_17_6, s_17_4)
        let s_17_7: () = AArch64_SystemAccessTrap(state, tracer, s_17_6, s_17_4);
        // N s_17_8: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var __HCR_EL2_NV:u8
        let s_18_0: bool = fn_state.u__HCR_EL2_NV;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 1u16);
        // C s_18_2: const #1u : u8
        let s_18_2: bool = true;
        // C s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 1u16);
        // D s_18_4: cmp-eq s_18_1 s_18_3
        let s_18_4: bool = ((s_18_1) == (s_18_3));
        // D s_18_5: write-var gs#102456 <= s_18_4
        fn_state.gs_102456 = s_18_4;
        // N s_18_6: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: panic
        panic!("{:?}", ());
        // N s_19_1: return
        return;
    }
}
