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
use X_set::*;
use u_get_MDCR_EL3_Type_TDA::*;
use Halted::*;
use u__get_DBGDTRRX_EL0::*;
use u_get_MDCR_EL2_Type_TDCC::*;
use u_get_MDSCR_EL1_Type_TDCC::*;
use u_get_MDCR_EL3_Type_TDCC::*;
use AArch64_SystemAccessTrap::*;
use EL2Enabled::*;
use u_get_MDCR_EL2_Type_TDE::*;
use u_get_MDCR_EL2_Type_TDA::*;
use u_get_HCR_EL2_Type_TGE::*;
use common::*;
pub fn DBGDTRRX_EL0_SysRegRead_54d73d4619fa5dbf<T: Tracer>(
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
        gs_59779: bool,
        gs_59775: bool,
        gs_59772: bool,
        gs_59771: bool,
        gs_59768: bool,
        gs_59770: bool,
        gs_59781: bool,
        u__MDCR_EL3_TDA: bool,
        gs_59778: bool,
        u__MDCR_EL3_TDCC: bool,
        gs_59777: bool,
        gs_59767: bool,
        u__MDSCR_EL1_TDCC: bool,
        gs_59773: bool,
        u__PSTATE_EL: u8,
        u__MDCR_EL2_TDCC: bool,
        gs_59776: bool,
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
        // C s_0_3: const #104648u : u32
        let s_0_3: u32 = 104648;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_MDSCR_EL1_Type_TDCC(s_0_4)
        let s_0_5: bool = u_get_MDSCR_EL1_Type_TDCC(state, tracer, s_0_4);
        // D s_0_6: write-var __MDSCR_EL1_TDCC <= s_0_5
        fn_state.u__MDSCR_EL1_TDCC = s_0_5;
        // C s_0_7: const #104880u : u32
        let s_0_7: u32 = 104880;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_MDCR_EL2_Type_TDCC(s_0_8)
        let s_0_9: bool = u_get_MDCR_EL2_Type_TDCC(state, tracer, s_0_8);
        // D s_0_10: write-var __MDCR_EL2_TDCC <= s_0_9
        fn_state.u__MDCR_EL2_TDCC = s_0_9;
        // C s_0_11: const #22712u : u32
        let s_0_11: u32 = 22712;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_MDCR_EL3_Type_TDCC(s_0_12)
        let s_0_13: bool = u_get_MDCR_EL3_Type_TDCC(state, tracer, s_0_12);
        // D s_0_14: write-var __MDCR_EL3_TDCC <= s_0_13
        fn_state.u__MDCR_EL3_TDCC = s_0_13;
        // C s_0_15: const #22712u : u32
        let s_0_15: u32 = 22712;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_MDCR_EL3_Type_TDA(s_0_16)
        let s_0_17: bool = u_get_MDCR_EL3_Type_TDA(state, tracer, s_0_16);
        // D s_0_18: write-var __MDCR_EL3_TDA <= s_0_17
        fn_state.u__MDCR_EL3_TDA = s_0_17;
        // C s_0_19: const #() : ()
        let s_0_19: () = ();
        // S s_0_20: call Halted(s_0_19)
        let s_0_20: bool = Halted(state, tracer, s_0_19);
        // N s_0_21: branch s_0_20 b70 b1
        if s_0_20 {
            return block_70(state, tracer, fn_state);
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
        // C s_1_2: const #448u : u32
        let s_1_2: u32 = 448;
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
        // N s_1_6: branch s_1_5 b39 b2
        if s_1_5 {
            return block_39(state, tracer, fn_state);
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
        // C s_2_2: const #440u : u32
        let s_2_2: u32 = 440;
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
        // N s_2_6: branch s_2_5 b18 b3
        if s_2_5 {
            return block_18(state, tracer, fn_state);
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
        // C s_3_2: const #432u : u32
        let s_3_2: u32 = 432;
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
        // N s_3_6: branch s_3_5 b7 b4
        if s_3_5 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var __PSTATE_EL:u8
        let s_4_0: u8 = fn_state.u__PSTATE_EL;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 2u16);
        // C s_4_2: const #424u : u32
        let s_4_2: u32 = 424;
        // D s_4_3: read-reg s_4_2:u8
        let s_4_3: u8 = {
            let value = state.read_register::<u8>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 2u16);
        // D s_4_5: cmp-eq s_4_1 s_4_4
        let s_4_5: bool = ((s_4_1) == (s_4_4));
        // N s_4_6: branch s_4_5 b6 b5
        if s_4_5 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #64s : i64
        let s_6_0: i64 = 64;
        // C s_6_1: const #103184u : u32
        let s_6_1: u32 = 103184;
        // D s_6_2: read-reg s_6_1:u64
        let s_6_2: u64 = {
            let value = state.read_register::<u64>(s_6_1 as isize);
            tracer.read_register(s_6_1 as isize, value);
            value
        };
        // D s_6_3: call __get_DBGDTRRX_EL0(s_6_2)
        let s_6_3: u64 = u__get_DBGDTRRX_EL0(state, tracer, s_6_2);
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 64u16);
        // D s_6_5: read-var t:i
        let s_6_5: i128 = fn_state.t;
        // D s_6_6: call X_set(s_6_5, s_6_0, s_6_4)
        let s_6_6: () = X_set(state, tracer, s_6_5, s_6_0, s_6_4);
        // N s_6_7: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #424u : u32
        let s_7_0: u32 = 424;
        // D s_7_1: read-reg s_7_0:u8
        let s_7_1: u8 = {
            let value = state.read_register::<u8>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // C s_7_2: const #2u : u8
        let s_7_2: u8 = 2;
        // D s_7_3: cmp-lt s_7_1 s_7_2
        let s_7_3: bool = ((s_7_1) < (s_7_2));
        // N s_7_4: branch s_7_3 b17 b8
        if s_7_3 {
            return block_17(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#59767 <= s_8_0
        fn_state.gs_59767 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#59767:u8
        let s_9_0: bool = fn_state.gs_59767;
        // N s_9_1: branch s_9_0 b16 b10
        if s_9_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #424u : u32
        let s_10_0: u32 = 424;
        // D s_10_1: read-reg s_10_0:u8
        let s_10_1: u8 = {
            let value = state.read_register::<u8>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // C s_10_2: const #2u : u8
        let s_10_2: u8 = 2;
        // D s_10_3: cmp-lt s_10_1 s_10_2
        let s_10_3: bool = ((s_10_1) < (s_10_2));
        // N s_10_4: branch s_10_3 b15 b11
        if s_10_3 {
            return block_15(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#59768 <= s_11_0
        fn_state.gs_59768 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#59768:u8
        let s_12_0: bool = fn_state.gs_59768;
        // N s_12_1: branch s_12_0 b14 b13
        if s_12_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #64s : i64
        let s_13_0: i64 = 64;
        // C s_13_1: const #103184u : u32
        let s_13_1: u32 = 103184;
        // D s_13_2: read-reg s_13_1:u64
        let s_13_2: u64 = {
            let value = state.read_register::<u64>(s_13_1 as isize);
            tracer.read_register(s_13_1 as isize, value);
            value
        };
        // D s_13_3: call __get_DBGDTRRX_EL0(s_13_2)
        let s_13_3: u64 = u__get_DBGDTRRX_EL0(state, tracer, s_13_2);
        // D s_13_4: cast zx s_13_3 -> bv
        let s_13_4: Bits = Bits::new(s_13_3 as u128, 64u16);
        // D s_13_5: read-var t:i
        let s_13_5: i128 = fn_state.t;
        // D s_13_6: call X_set(s_13_5, s_13_0, s_13_4)
        let s_13_6: () = X_set(state, tracer, s_13_5, s_13_0, s_13_4);
        // N s_13_7: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #24u : u8
        let s_14_0: u8 = 24;
        // C s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 8u16);
        // C s_14_2: cast zx s_14_1 -> i
        let s_14_2: i128 = (s_14_1.value() as i128);
        // C s_14_3: cast reint s_14_2 -> i64
        let s_14_3: i64 = (s_14_2 as i64);
        // C s_14_4: cast zx s_14_3 -> i
        let s_14_4: i128 = (i128::try_from(s_14_3).unwrap());
        // C s_14_5: const #424u : u32
        let s_14_5: u32 = 424;
        // D s_14_6: read-reg s_14_5:u8
        let s_14_6: u8 = {
            let value = state.read_register::<u8>(s_14_5 as isize);
            tracer.read_register(s_14_5 as isize, value);
            value
        };
        // D s_14_7: call AArch64_SystemAccessTrap(s_14_6, s_14_4)
        let s_14_7: () = AArch64_SystemAccessTrap(state, tracer, s_14_6, s_14_4);
        // N s_14_8: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var __MDCR_EL3_TDA:u8
        let s_15_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 1u16);
        // C s_15_2: const #1u : u8
        let s_15_2: bool = true;
        // C s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 1u16);
        // D s_15_4: cmp-eq s_15_1 s_15_3
        let s_15_4: bool = ((s_15_1) == (s_15_3));
        // D s_15_5: write-var gs#59768 <= s_15_4
        fn_state.gs_59768 = s_15_4;
        // N s_15_6: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #24u : u8
        let s_16_0: u8 = 24;
        // C s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 8u16);
        // C s_16_2: cast zx s_16_1 -> i
        let s_16_2: i128 = (s_16_1.value() as i128);
        // C s_16_3: cast reint s_16_2 -> i64
        let s_16_3: i64 = (s_16_2 as i64);
        // C s_16_4: cast zx s_16_3 -> i
        let s_16_4: i128 = (i128::try_from(s_16_3).unwrap());
        // C s_16_5: const #424u : u32
        let s_16_5: u32 = 424;
        // D s_16_6: read-reg s_16_5:u8
        let s_16_6: u8 = {
            let value = state.read_register::<u8>(s_16_5 as isize);
            tracer.read_register(s_16_5 as isize, value);
            value
        };
        // D s_16_7: call AArch64_SystemAccessTrap(s_16_6, s_16_4)
        let s_16_7: () = AArch64_SystemAccessTrap(state, tracer, s_16_6, s_16_4);
        // N s_16_8: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var __MDCR_EL3_TDCC:u8
        let s_17_0: bool = fn_state.u__MDCR_EL3_TDCC;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 1u16);
        // C s_17_2: const #1u : u8
        let s_17_2: bool = true;
        // C s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 1u16);
        // D s_17_4: cmp-eq s_17_1 s_17_3
        let s_17_4: bool = ((s_17_1) == (s_17_3));
        // D s_17_5: write-var gs#59767 <= s_17_4
        fn_state.gs_59767 = s_17_4;
        // N s_17_6: jump b9
        return block_9(state, tracer, fn_state);
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
        // N s_18_2: branch s_18_1 b38 b19
        if s_18_1 {
            return block_38(state, tracer, fn_state);
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
        // D s_19_1: write-var gs#59770 <= s_19_0
        fn_state.gs_59770 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#59770:u8
        let s_20_0: bool = fn_state.gs_59770;
        // N s_20_1: branch s_20_0 b37 b21
        if s_20_0 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call EL2Enabled(s_21_0)
        let s_21_1: bool = EL2Enabled(state, tracer, s_21_0);
        // N s_21_2: branch s_21_1 b36 b22
        if s_21_1 {
            return block_36(state, tracer, fn_state);
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
        // D s_22_1: write-var gs#59771 <= s_22_0
        fn_state.gs_59771 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#59771:u8
        let s_23_0: bool = fn_state.gs_59771;
        // N s_23_1: branch s_23_0 b35 b24
        if s_23_0 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #424u : u32
        let s_24_0: u32 = 424;
        // D s_24_1: read-reg s_24_0:u8
        let s_24_1: u8 = {
            let value = state.read_register::<u8>(s_24_0 as isize);
            tracer.read_register(s_24_0 as isize, value);
            value
        };
        // C s_24_2: const #2u : u8
        let s_24_2: u8 = 2;
        // D s_24_3: cmp-lt s_24_1 s_24_2
        let s_24_3: bool = ((s_24_1) < (s_24_2));
        // N s_24_4: branch s_24_3 b34 b25
        if s_24_3 {
            return block_34(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#59772 <= s_25_0
        fn_state.gs_59772 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#59772:u8
        let s_26_0: bool = fn_state.gs_59772;
        // N s_26_1: branch s_26_0 b33 b27
        if s_26_0 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #424u : u32
        let s_27_0: u32 = 424;
        // D s_27_1: read-reg s_27_0:u8
        let s_27_1: u8 = {
            let value = state.read_register::<u8>(s_27_0 as isize);
            tracer.read_register(s_27_0 as isize, value);
            value
        };
        // C s_27_2: const #2u : u8
        let s_27_2: u8 = 2;
        // D s_27_3: cmp-lt s_27_1 s_27_2
        let s_27_3: bool = ((s_27_1) < (s_27_2));
        // N s_27_4: branch s_27_3 b32 b28
        if s_27_3 {
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
        // D s_28_1: write-var gs#59773 <= s_28_0
        fn_state.gs_59773 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#59773:u8
        let s_29_0: bool = fn_state.gs_59773;
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
        // C s_30_0: const #64s : i64
        let s_30_0: i64 = 64;
        // C s_30_1: const #103184u : u32
        let s_30_1: u32 = 103184;
        // D s_30_2: read-reg s_30_1:u64
        let s_30_2: u64 = {
            let value = state.read_register::<u64>(s_30_1 as isize);
            tracer.read_register(s_30_1 as isize, value);
            value
        };
        // D s_30_3: call __get_DBGDTRRX_EL0(s_30_2)
        let s_30_3: u64 = u__get_DBGDTRRX_EL0(state, tracer, s_30_2);
        // D s_30_4: cast zx s_30_3 -> bv
        let s_30_4: Bits = Bits::new(s_30_3 as u128, 64u16);
        // D s_30_5: read-var t:i
        let s_30_5: i128 = fn_state.t;
        // D s_30_6: call X_set(s_30_5, s_30_0, s_30_4)
        let s_30_6: () = X_set(state, tracer, s_30_5, s_30_0, s_30_4);
        // N s_30_7: return
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
        // C s_31_5: const #424u : u32
        let s_31_5: u32 = 424;
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
        // D s_32_0: read-var __MDCR_EL3_TDA:u8
        let s_32_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 1u16);
        // C s_32_2: const #1u : u8
        let s_32_2: bool = true;
        // C s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 1u16);
        // D s_32_4: cmp-eq s_32_1 s_32_3
        let s_32_4: bool = ((s_32_1) == (s_32_3));
        // D s_32_5: write-var gs#59773 <= s_32_4
        fn_state.gs_59773 = s_32_4;
        // N s_32_6: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #24u : u8
        let s_33_0: u8 = 24;
        // C s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 8u16);
        // C s_33_2: cast zx s_33_1 -> i
        let s_33_2: i128 = (s_33_1.value() as i128);
        // C s_33_3: cast reint s_33_2 -> i64
        let s_33_3: i64 = (s_33_2 as i64);
        // C s_33_4: cast zx s_33_3 -> i
        let s_33_4: i128 = (i128::try_from(s_33_3).unwrap());
        // C s_33_5: const #424u : u32
        let s_33_5: u32 = 424;
        // D s_33_6: read-reg s_33_5:u8
        let s_33_6: u8 = {
            let value = state.read_register::<u8>(s_33_5 as isize);
            tracer.read_register(s_33_5 as isize, value);
            value
        };
        // D s_33_7: call AArch64_SystemAccessTrap(s_33_6, s_33_4)
        let s_33_7: () = AArch64_SystemAccessTrap(state, tracer, s_33_6, s_33_4);
        // N s_33_8: return
        return;
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var __MDCR_EL3_TDCC:u8
        let s_34_0: bool = fn_state.u__MDCR_EL3_TDCC;
        // D s_34_1: cast zx s_34_0 -> bv
        let s_34_1: Bits = Bits::new(s_34_0 as u128, 1u16);
        // C s_34_2: const #1u : u8
        let s_34_2: bool = true;
        // C s_34_3: cast zx s_34_2 -> bv
        let s_34_3: Bits = Bits::new(s_34_2 as u128, 1u16);
        // D s_34_4: cmp-eq s_34_1 s_34_3
        let s_34_4: bool = ((s_34_1) == (s_34_3));
        // D s_34_5: write-var gs#59772 <= s_34_4
        fn_state.gs_59772 = s_34_4;
        // N s_34_6: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #24u : u8
        let s_35_0: u8 = 24;
        // C s_35_1: cast zx s_35_0 -> bv
        let s_35_1: Bits = Bits::new(s_35_0 as u128, 8u16);
        // C s_35_2: cast zx s_35_1 -> i
        let s_35_2: i128 = (s_35_1.value() as i128);
        // C s_35_3: cast reint s_35_2 -> i64
        let s_35_3: i64 = (s_35_2 as i64);
        // C s_35_4: cast zx s_35_3 -> i
        let s_35_4: i128 = (i128::try_from(s_35_3).unwrap());
        // C s_35_5: const #432u : u32
        let s_35_5: u32 = 432;
        // D s_35_6: read-reg s_35_5:u8
        let s_35_6: u8 = {
            let value = state.read_register::<u8>(s_35_5 as isize);
            tracer.read_register(s_35_5 as isize, value);
            value
        };
        // D s_35_7: call AArch64_SystemAccessTrap(s_35_6, s_35_4)
        let s_35_7: () = AArch64_SystemAccessTrap(state, tracer, s_35_6, s_35_4);
        // N s_35_8: return
        return;
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #104880u : u32
        let s_36_0: u32 = 104880;
        // D s_36_1: read-reg s_36_0:struct
        let s_36_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_36_0 as isize);
            tracer.read_register(s_36_0 as isize, value);
            value
        };
        // D s_36_2: call _get_MDCR_EL2_Type_TDE(s_36_1)
        let s_36_2: bool = u_get_MDCR_EL2_Type_TDE(state, tracer, s_36_1);
        // C s_36_3: const #104880u : u32
        let s_36_3: u32 = 104880;
        // D s_36_4: read-reg s_36_3:struct
        let s_36_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_36_3 as isize);
            tracer.read_register(s_36_3 as isize, value);
            value
        };
        // D s_36_5: call _get_MDCR_EL2_Type_TDA(s_36_4)
        let s_36_5: bool = u_get_MDCR_EL2_Type_TDA(state, tracer, s_36_4);
        // D s_36_6: cast zx s_36_2 -> bv
        let s_36_6: Bits = Bits::new(s_36_2 as u128, 1u16);
        // D s_36_7: cast zx s_36_5 -> bv
        let s_36_7: Bits = Bits::new(s_36_5 as u128, 1u16);
        // D s_36_8: cast reint s_36_6 -> u128
        let s_36_8: u128 = (s_36_6.value() as u128);
        // D s_36_9: size-of s_36_6
        let s_36_9: u16 = s_36_6.length();
        // D s_36_10: cast reint s_36_7 -> u128
        let s_36_10: u128 = (s_36_7.value() as u128);
        // D s_36_11: size-of s_36_7
        let s_36_11: u16 = s_36_7.length();
        // D s_36_12: lsl s_36_8 s_36_11
        let s_36_12: u128 = s_36_8 << s_36_11;
        // D s_36_13: or s_36_12 s_36_10
        let s_36_13: u128 = ((s_36_12) | (s_36_10));
        // D s_36_14: add s_36_9 s_36_11
        let s_36_14: u16 = (s_36_9 + s_36_11);
        // D s_36_15: create-bits s_36_13 s_36_14
        let s_36_15: Bits = Bits::new(s_36_13, s_36_14);
        // D s_36_16: cast reint s_36_15 -> u8
        let s_36_16: u8 = (s_36_15.value() as u8);
        // D s_36_17: cast zx s_36_16 -> bv
        let s_36_17: Bits = Bits::new(s_36_16 as u128, 2u16);
        // C s_36_18: const #0u : u8
        let s_36_18: u8 = 0;
        // C s_36_19: cast zx s_36_18 -> bv
        let s_36_19: Bits = Bits::new(s_36_18 as u128, 2u16);
        // D s_36_20: cmp-ne s_36_17 s_36_19
        let s_36_20: bool = ((s_36_17) != (s_36_19));
        // D s_36_21: write-var gs#59771 <= s_36_20
        fn_state.gs_59771 = s_36_20;
        // N s_36_22: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #24u : u8
        let s_37_0: u8 = 24;
        // C s_37_1: cast zx s_37_0 -> bv
        let s_37_1: Bits = Bits::new(s_37_0 as u128, 8u16);
        // C s_37_2: cast zx s_37_1 -> i
        let s_37_2: i128 = (s_37_1.value() as i128);
        // C s_37_3: cast reint s_37_2 -> i64
        let s_37_3: i64 = (s_37_2 as i64);
        // C s_37_4: cast zx s_37_3 -> i
        let s_37_4: i128 = (i128::try_from(s_37_3).unwrap());
        // C s_37_5: const #432u : u32
        let s_37_5: u32 = 432;
        // D s_37_6: read-reg s_37_5:u8
        let s_37_6: u8 = {
            let value = state.read_register::<u8>(s_37_5 as isize);
            tracer.read_register(s_37_5 as isize, value);
            value
        };
        // D s_37_7: call AArch64_SystemAccessTrap(s_37_6, s_37_4)
        let s_37_7: () = AArch64_SystemAccessTrap(state, tracer, s_37_6, s_37_4);
        // N s_37_8: return
        return;
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var __MDCR_EL2_TDCC:u8
        let s_38_0: bool = fn_state.u__MDCR_EL2_TDCC;
        // D s_38_1: cast zx s_38_0 -> bv
        let s_38_1: Bits = Bits::new(s_38_0 as u128, 1u16);
        // C s_38_2: const #1u : u8
        let s_38_2: bool = true;
        // C s_38_3: cast zx s_38_2 -> bv
        let s_38_3: Bits = Bits::new(s_38_2 as u128, 1u16);
        // D s_38_4: cmp-eq s_38_1 s_38_3
        let s_38_4: bool = ((s_38_1) == (s_38_3));
        // D s_38_5: write-var gs#59770 <= s_38_4
        fn_state.gs_59770 = s_38_4;
        // N s_38_6: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var __MDSCR_EL1_TDCC:u8
        let s_39_0: bool = fn_state.u__MDSCR_EL1_TDCC;
        // D s_39_1: cast zx s_39_0 -> bv
        let s_39_1: Bits = Bits::new(s_39_0 as u128, 1u16);
        // C s_39_2: const #1u : u8
        let s_39_2: bool = true;
        // C s_39_3: cast zx s_39_2 -> bv
        let s_39_3: Bits = Bits::new(s_39_2 as u128, 1u16);
        // D s_39_4: cmp-eq s_39_1 s_39_3
        let s_39_4: bool = ((s_39_1) == (s_39_3));
        // N s_39_5: branch s_39_4 b64 b40
        if s_39_4 {
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
        // D s_41_1: write-var gs#59775 <= s_41_0
        fn_state.gs_59775 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#59775:u8
        let s_42_0: bool = fn_state.gs_59775;
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
        // D s_44_1: write-var gs#59777 <= s_44_0
        fn_state.gs_59777 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#59777:u8
        let s_45_0: bool = fn_state.gs_59777;
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
        // D s_47_1: write-var gs#59778 <= s_47_0
        fn_state.gs_59778 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#59778:u8
        let s_48_0: bool = fn_state.gs_59778;
        // N s_48_1: branch s_48_0 b55 b49
        if s_48_0 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #424u : u32
        let s_49_0: u32 = 424;
        // D s_49_1: read-reg s_49_0:u8
        let s_49_1: u8 = {
            let value = state.read_register::<u8>(s_49_0 as isize);
            tracer.read_register(s_49_0 as isize, value);
            value
        };
        // C s_49_2: const #2u : u8
        let s_49_2: u8 = 2;
        // D s_49_3: cmp-lt s_49_1 s_49_2
        let s_49_3: bool = ((s_49_1) < (s_49_2));
        // N s_49_4: branch s_49_3 b54 b50
        if s_49_3 {
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
        // D s_50_1: write-var gs#59779 <= s_50_0
        fn_state.gs_59779 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#59779:u8
        let s_51_0: bool = fn_state.gs_59779;
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
        // C s_52_0: const #64s : i64
        let s_52_0: i64 = 64;
        // C s_52_1: const #103184u : u32
        let s_52_1: u32 = 103184;
        // D s_52_2: read-reg s_52_1:u64
        let s_52_2: u64 = {
            let value = state.read_register::<u64>(s_52_1 as isize);
            tracer.read_register(s_52_1 as isize, value);
            value
        };
        // D s_52_3: call __get_DBGDTRRX_EL0(s_52_2)
        let s_52_3: u64 = u__get_DBGDTRRX_EL0(state, tracer, s_52_2);
        // D s_52_4: cast zx s_52_3 -> bv
        let s_52_4: Bits = Bits::new(s_52_3 as u128, 64u16);
        // D s_52_5: read-var t:i
        let s_52_5: i128 = fn_state.t;
        // D s_52_6: call X_set(s_52_5, s_52_0, s_52_4)
        let s_52_6: () = X_set(state, tracer, s_52_5, s_52_0, s_52_4);
        // N s_52_7: return
        return;
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #24u : u8
        let s_53_0: u8 = 24;
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
        // D s_54_0: read-var __MDCR_EL3_TDA:u8
        let s_54_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_54_1: cast zx s_54_0 -> bv
        let s_54_1: Bits = Bits::new(s_54_0 as u128, 1u16);
        // C s_54_2: const #1u : u8
        let s_54_2: bool = true;
        // C s_54_3: cast zx s_54_2 -> bv
        let s_54_3: Bits = Bits::new(s_54_2 as u128, 1u16);
        // D s_54_4: cmp-eq s_54_1 s_54_3
        let s_54_4: bool = ((s_54_1) == (s_54_3));
        // D s_54_5: write-var gs#59779 <= s_54_4
        fn_state.gs_59779 = s_54_4;
        // N s_54_6: jump b51
        return block_51(state, tracer, fn_state);
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
        // C s_55_5: const #424u : u32
        let s_55_5: u32 = 424;
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
        // D s_56_0: read-var __MDCR_EL3_TDCC:u8
        let s_56_0: bool = fn_state.u__MDCR_EL3_TDCC;
        // D s_56_1: cast zx s_56_0 -> bv
        let s_56_1: Bits = Bits::new(s_56_0 as u128, 1u16);
        // C s_56_2: const #1u : u8
        let s_56_2: bool = true;
        // C s_56_3: cast zx s_56_2 -> bv
        let s_56_3: Bits = Bits::new(s_56_2 as u128, 1u16);
        // D s_56_4: cmp-eq s_56_1 s_56_3
        let s_56_4: bool = ((s_56_1) == (s_56_3));
        // D s_56_5: write-var gs#59778 <= s_56_4
        fn_state.gs_59778 = s_56_4;
        // N s_56_6: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #24u : u8
        let s_57_0: u8 = 24;
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
        // C s_58_0: const #102552u : u32
        let s_58_0: u32 = 102552;
        // D s_58_1: read-reg s_58_0:struct
        let s_58_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_58_0 as isize);
            tracer.read_register(s_58_0 as isize, value);
            value
        };
        // D s_58_2: call _get_HCR_EL2_Type_TGE(s_58_1)
        let s_58_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_58_1);
        // D s_58_3: cast zx s_58_2 -> bv
        let s_58_3: Bits = Bits::new(s_58_2 as u128, 1u16);
        // C s_58_4: const #1u : u8
        let s_58_4: bool = true;
        // C s_58_5: cast zx s_58_4 -> bv
        let s_58_5: Bits = Bits::new(s_58_4 as u128, 1u16);
        // D s_58_6: cmp-eq s_58_3 s_58_5
        let s_58_6: bool = ((s_58_3) == (s_58_5));
        // N s_58_7: branch s_58_6 b61 b59
        if s_58_6 {
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
        // C s_59_0: const #104880u : u32
        let s_59_0: u32 = 104880;
        // D s_59_1: read-reg s_59_0:struct
        let s_59_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_59_0 as isize);
            tracer.read_register(s_59_0 as isize, value);
            value
        };
        // D s_59_2: call _get_MDCR_EL2_Type_TDE(s_59_1)
        let s_59_2: bool = u_get_MDCR_EL2_Type_TDE(state, tracer, s_59_1);
        // C s_59_3: const #104880u : u32
        let s_59_3: u32 = 104880;
        // D s_59_4: read-reg s_59_3:struct
        let s_59_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_59_3 as isize);
            tracer.read_register(s_59_3 as isize, value);
            value
        };
        // D s_59_5: call _get_MDCR_EL2_Type_TDA(s_59_4)
        let s_59_5: bool = u_get_MDCR_EL2_Type_TDA(state, tracer, s_59_4);
        // D s_59_6: cast zx s_59_2 -> bv
        let s_59_6: Bits = Bits::new(s_59_2 as u128, 1u16);
        // D s_59_7: cast zx s_59_5 -> bv
        let s_59_7: Bits = Bits::new(s_59_5 as u128, 1u16);
        // D s_59_8: cast reint s_59_6 -> u128
        let s_59_8: u128 = (s_59_6.value() as u128);
        // D s_59_9: size-of s_59_6
        let s_59_9: u16 = s_59_6.length();
        // D s_59_10: cast reint s_59_7 -> u128
        let s_59_10: u128 = (s_59_7.value() as u128);
        // D s_59_11: size-of s_59_7
        let s_59_11: u16 = s_59_7.length();
        // D s_59_12: lsl s_59_8 s_59_11
        let s_59_12: u128 = s_59_8 << s_59_11;
        // D s_59_13: or s_59_12 s_59_10
        let s_59_13: u128 = ((s_59_12) | (s_59_10));
        // D s_59_14: add s_59_9 s_59_11
        let s_59_14: u16 = (s_59_9 + s_59_11);
        // D s_59_15: create-bits s_59_13 s_59_14
        let s_59_15: Bits = Bits::new(s_59_13, s_59_14);
        // D s_59_16: cast reint s_59_15 -> u8
        let s_59_16: u8 = (s_59_15.value() as u8);
        // D s_59_17: cast zx s_59_16 -> bv
        let s_59_17: Bits = Bits::new(s_59_16 as u128, 2u16);
        // C s_59_18: const #0u : u8
        let s_59_18: u8 = 0;
        // C s_59_19: cast zx s_59_18 -> bv
        let s_59_19: Bits = Bits::new(s_59_18 as u128, 2u16);
        // D s_59_20: cmp-ne s_59_17 s_59_19
        let s_59_20: bool = ((s_59_17) != (s_59_19));
        // D s_59_21: write-var gs#59776 <= s_59_20
        fn_state.gs_59776 = s_59_20;
        // N s_59_22: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#59776:u8
        let s_60_0: bool = fn_state.gs_59776;
        // D s_60_1: write-var gs#59777 <= s_60_0
        fn_state.gs_59777 = s_60_0;
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
        // D s_61_1: write-var gs#59776 <= s_61_0
        fn_state.gs_59776 = s_61_0;
        // N s_61_2: jump b60
        return block_60(state, tracer, fn_state);
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
        // D s_63_0: read-var __MDCR_EL2_TDCC:u8
        let s_63_0: bool = fn_state.u__MDCR_EL2_TDCC;
        // D s_63_1: cast zx s_63_0 -> bv
        let s_63_1: Bits = Bits::new(s_63_0 as u128, 1u16);
        // C s_63_2: const #1u : u8
        let s_63_2: bool = true;
        // C s_63_3: cast zx s_63_2 -> bv
        let s_63_3: Bits = Bits::new(s_63_2 as u128, 1u16);
        // D s_63_4: cmp-eq s_63_1 s_63_3
        let s_63_4: bool = ((s_63_1) == (s_63_3));
        // D s_63_5: write-var gs#59775 <= s_63_4
        fn_state.gs_59775 = s_63_4;
        // N s_63_6: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #() : ()
        let s_64_0: () = ();
        // S s_64_1: call EL2Enabled(s_64_0)
        let s_64_1: bool = EL2Enabled(state, tracer, s_64_0);
        // N s_64_2: branch s_64_1 b69 b65
        if s_64_1 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #0u : u8
        let s_65_0: bool = false;
        // D s_65_1: write-var gs#59781 <= s_65_0
        fn_state.gs_59781 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#59781:u8
        let s_66_0: bool = fn_state.gs_59781;
        // N s_66_1: branch s_66_0 b68 b67
        if s_66_0 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #24u : u8
        let s_67_0: u8 = 24;
        // C s_67_1: cast zx s_67_0 -> bv
        let s_67_1: Bits = Bits::new(s_67_0 as u128, 8u16);
        // C s_67_2: cast zx s_67_1 -> i
        let s_67_2: i128 = (s_67_1.value() as i128);
        // C s_67_3: cast reint s_67_2 -> i64
        let s_67_3: i64 = (s_67_2 as i64);
        // C s_67_4: cast zx s_67_3 -> i
        let s_67_4: i128 = (i128::try_from(s_67_3).unwrap());
        // C s_67_5: const #440u : u32
        let s_67_5: u32 = 440;
        // D s_67_6: read-reg s_67_5:u8
        let s_67_6: u8 = {
            let value = state.read_register::<u8>(s_67_5 as isize);
            tracer.read_register(s_67_5 as isize, value);
            value
        };
        // D s_67_7: call AArch64_SystemAccessTrap(s_67_6, s_67_4)
        let s_67_7: () = AArch64_SystemAccessTrap(state, tracer, s_67_6, s_67_4);
        // N s_67_8: return
        return;
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #24u : u8
        let s_68_0: u8 = 24;
        // C s_68_1: cast zx s_68_0 -> bv
        let s_68_1: Bits = Bits::new(s_68_0 as u128, 8u16);
        // C s_68_2: cast zx s_68_1 -> i
        let s_68_2: i128 = (s_68_1.value() as i128);
        // C s_68_3: cast reint s_68_2 -> i64
        let s_68_3: i64 = (s_68_2 as i64);
        // C s_68_4: cast zx s_68_3 -> i
        let s_68_4: i128 = (i128::try_from(s_68_3).unwrap());
        // C s_68_5: const #432u : u32
        let s_68_5: u32 = 432;
        // D s_68_6: read-reg s_68_5:u8
        let s_68_6: u8 = {
            let value = state.read_register::<u8>(s_68_5 as isize);
            tracer.read_register(s_68_5 as isize, value);
            value
        };
        // D s_68_7: call AArch64_SystemAccessTrap(s_68_6, s_68_4)
        let s_68_7: () = AArch64_SystemAccessTrap(state, tracer, s_68_6, s_68_4);
        // N s_68_8: return
        return;
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #102552u : u32
        let s_69_0: u32 = 102552;
        // D s_69_1: read-reg s_69_0:struct
        let s_69_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_69_0 as isize);
            tracer.read_register(s_69_0 as isize, value);
            value
        };
        // D s_69_2: call _get_HCR_EL2_Type_TGE(s_69_1)
        let s_69_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_69_1);
        // D s_69_3: cast zx s_69_2 -> bv
        let s_69_3: Bits = Bits::new(s_69_2 as u128, 1u16);
        // C s_69_4: const #1u : u8
        let s_69_4: bool = true;
        // C s_69_5: cast zx s_69_4 -> bv
        let s_69_5: Bits = Bits::new(s_69_4 as u128, 1u16);
        // D s_69_6: cmp-eq s_69_3 s_69_5
        let s_69_6: bool = ((s_69_3) == (s_69_5));
        // D s_69_7: write-var gs#59781 <= s_69_6
        fn_state.gs_59781 = s_69_6;
        // N s_69_8: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #64s : i64
        let s_70_0: i64 = 64;
        // C s_70_1: const #103184u : u32
        let s_70_1: u32 = 103184;
        // D s_70_2: read-reg s_70_1:u64
        let s_70_2: u64 = {
            let value = state.read_register::<u64>(s_70_1 as isize);
            tracer.read_register(s_70_1 as isize, value);
            value
        };
        // D s_70_3: call __get_DBGDTRRX_EL0(s_70_2)
        let s_70_3: u64 = u__get_DBGDTRRX_EL0(state, tracer, s_70_2);
        // D s_70_4: cast zx s_70_3 -> bv
        let s_70_4: Bits = Bits::new(s_70_3 as u128, 64u16);
        // D s_70_5: read-var t:i
        let s_70_5: i128 = fn_state.t;
        // D s_70_6: call X_set(s_70_5, s_70_0, s_70_4)
        let s_70_6: () = X_set(state, tracer, s_70_5, s_70_0, s_70_4);
        // N s_70_7: return
        return;
    }
}
