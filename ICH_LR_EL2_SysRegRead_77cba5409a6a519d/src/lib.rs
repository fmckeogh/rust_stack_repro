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
use u_get_HCR_EL2_Type_NV::*;
use u_get_ICC_SRE_EL2_Type_SRE::*;
use u__get_ICH_LR_EL2::*;
use NVMem_read::*;
use AArch64_SystemAccessTrap::*;
use EL2Enabled::*;
use u_get_ICC_SRE_EL3_Type_SRE::*;
use u_get_HCR_EL2_Type_NV2::*;
use common::*;
pub fn ICH_LR_EL2_SysRegRead_77cba5409a6a519d<T: Tracer>(
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
        u__ICC_SRE_EL2_SRE: bool,
        ga_68465: u64,
        u__ICC_SRE_EL3_SRE: bool,
        ga_68481: ProductType5c790c8ef59cc8b2,
        u__PSTATE_EL: u8,
        gs_62495: bool,
        ga_68474: ProductType5c790c8ef59cc8b2,
        u__HCR_EL2_NV: bool,
        gs_62496: bool,
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
        // C s_0_7: const #16368u : u32
        let s_0_7: u32 = 16368;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_ICC_SRE_EL2_Type_SRE(s_0_8)
        let s_0_9: bool = u_get_ICC_SRE_EL2_Type_SRE(state, tracer, s_0_8);
        // D s_0_10: write-var __ICC_SRE_EL2_SRE <= s_0_9
        fn_state.u__ICC_SRE_EL2_SRE = s_0_9;
        // C s_0_11: const #10200u : u32
        let s_0_11: u32 = 10200;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_ICC_SRE_EL3_Type_SRE(s_0_12)
        let s_0_13: bool = u_get_ICC_SRE_EL3_Type_SRE(state, tracer, s_0_12);
        // D s_0_14: write-var __ICC_SRE_EL3_SRE <= s_0_13
        fn_state.u__ICC_SRE_EL3_SRE = s_0_13;
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
        // N s_0_21: branch s_0_20 b23 b1
        if s_0_20 {
            return block_23(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b11 b2
        if s_1_5 {
            return block_11(state, tracer, fn_state);
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
        // N s_2_6: branch s_2_5 b8 b3
        if s_2_5 {
            return block_8(state, tracer, fn_state);
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
        // D s_5_0: read-var __ICC_SRE_EL3_SRE:u8
        let s_5_0: bool = fn_state.u__ICC_SRE_EL3_SRE;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 1u16);
        // C s_5_2: const #0u : u8
        let s_5_2: bool = false;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // N s_5_5: branch s_5_4 b7 b6
        if s_5_4 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #64s : i64
        let s_6_0: i64 = 64;
        // C s_6_1: const #4s : i
        let s_6_1: i128 = 4;
        // C s_6_2: const #10240u : u32
        let s_6_2: u32 = 10240;
        // D s_6_3: read-reg s_6_2:[struct; 16]
        let s_6_3: [ProductType5c790c8ef59cc8b2; 16usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 16usize]>(s_6_2 as isize);
            tracer.read_register(s_6_2 as isize, value);
            value
        };
        // D s_6_4: read-element s_6_3[s_6_1]
        let s_6_4: ProductType5c790c8ef59cc8b2 = s_6_3[(s_6_1) as usize];
        // D s_6_5: call __get_ICH_LR_EL2(s_6_4)
        let s_6_5: ProductType5c790c8ef59cc8b2 = u__get_ICH_LR_EL2(state, tracer, s_6_4);
        // D s_6_6: write-var ga#68481 <= s_6_5
        fn_state.ga_68481 = s_6_5;
        // D s_6_7: read-var ga#68481.0:struct
        let s_6_7: u64 = fn_state.ga_68481._0;
        // D s_6_8: cast zx s_6_7 -> bv
        let s_6_8: Bits = Bits::new(s_6_7 as u128, 64u16);
        // D s_6_9: read-var t:i
        let s_6_9: i128 = fn_state.t;
        // D s_6_10: call X_set(s_6_9, s_6_0, s_6_8)
        let s_6_10: () = X_set(state, tracer, s_6_9, s_6_0, s_6_8);
        // N s_6_11: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #24u : u8
        let s_7_0: u8 = 24;
        // C s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 8u16);
        // C s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (s_7_1.value() as i128);
        // C s_7_3: cast reint s_7_2 -> i64
        let s_7_3: i64 = (s_7_2 as i64);
        // C s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // C s_7_5: const #424u : u32
        let s_7_5: u32 = 424;
        // D s_7_6: read-reg s_7_5:u8
        let s_7_6: u8 = {
            let value = state.read_register::<u8>(s_7_5 as isize);
            tracer.read_register(s_7_5 as isize, value);
            value
        };
        // D s_7_7: call AArch64_SystemAccessTrap(s_7_6, s_7_4)
        let s_7_7: () = AArch64_SystemAccessTrap(state, tracer, s_7_6, s_7_4);
        // N s_7_8: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var __ICC_SRE_EL2_SRE:u8
        let s_8_0: bool = fn_state.u__ICC_SRE_EL2_SRE;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 1u16);
        // C s_8_2: const #0u : u8
        let s_8_2: bool = false;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 1u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // N s_8_5: branch s_8_4 b10 b9
        if s_8_4 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #64s : i64
        let s_9_0: i64 = 64;
        // C s_9_1: const #4s : i
        let s_9_1: i128 = 4;
        // C s_9_2: const #10240u : u32
        let s_9_2: u32 = 10240;
        // D s_9_3: read-reg s_9_2:[struct; 16]
        let s_9_3: [ProductType5c790c8ef59cc8b2; 16usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 16usize]>(s_9_2 as isize);
            tracer.read_register(s_9_2 as isize, value);
            value
        };
        // D s_9_4: read-element s_9_3[s_9_1]
        let s_9_4: ProductType5c790c8ef59cc8b2 = s_9_3[(s_9_1) as usize];
        // D s_9_5: call __get_ICH_LR_EL2(s_9_4)
        let s_9_5: ProductType5c790c8ef59cc8b2 = u__get_ICH_LR_EL2(state, tracer, s_9_4);
        // D s_9_6: write-var ga#68474 <= s_9_5
        fn_state.ga_68474 = s_9_5;
        // D s_9_7: read-var ga#68474.0:struct
        let s_9_7: u64 = fn_state.ga_68474._0;
        // D s_9_8: cast zx s_9_7 -> bv
        let s_9_8: Bits = Bits::new(s_9_7 as u128, 64u16);
        // D s_9_9: read-var t:i
        let s_9_9: i128 = fn_state.t;
        // D s_9_10: call X_set(s_9_9, s_9_0, s_9_8)
        let s_9_10: () = X_set(state, tracer, s_9_9, s_9_0, s_9_8);
        // N s_9_11: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #24u : u8
        let s_10_0: u8 = 24;
        // C s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 8u16);
        // C s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (s_10_1.value() as i128);
        // C s_10_3: cast reint s_10_2 -> i64
        let s_10_3: i64 = (s_10_2 as i64);
        // C s_10_4: cast zx s_10_3 -> i
        let s_10_4: i128 = (i128::try_from(s_10_3).unwrap());
        // C s_10_5: const #432u : u32
        let s_10_5: u32 = 432;
        // D s_10_6: read-reg s_10_5:u8
        let s_10_6: u8 = {
            let value = state.read_register::<u8>(s_10_5 as isize);
            tracer.read_register(s_10_5 as isize, value);
            value
        };
        // D s_10_7: call AArch64_SystemAccessTrap(s_10_6, s_10_4)
        let s_10_7: () = AArch64_SystemAccessTrap(state, tracer, s_10_6, s_10_4);
        // N s_10_8: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call EL2Enabled(s_11_0)
        let s_11_1: bool = EL2Enabled(state, tracer, s_11_0);
        // N s_11_2: branch s_11_1 b22 b12
        if s_11_1 {
            return block_22(state, tracer, fn_state);
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
        // D s_12_1: write-var gs#62495 <= s_12_0
        fn_state.gs_62495 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#62495:u8
        let s_13_0: bool = fn_state.gs_62495;
        // N s_13_1: branch s_13_0 b20 b14
        if s_13_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call EL2Enabled(s_14_0)
        let s_14_1: bool = EL2Enabled(state, tracer, s_14_0);
        // N s_14_2: branch s_14_1 b19 b15
        if s_14_1 {
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
        // D s_15_1: write-var gs#62496 <= s_15_0
        fn_state.gs_62496 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#62496:u8
        let s_16_0: bool = fn_state.gs_62496;
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
        // N s_17_0: panic
        panic!("{:?}", ());
        // N s_17_1: return
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
        // D s_19_0: read-var __HCR_EL2_NV:u8
        let s_19_0: bool = fn_state.u__HCR_EL2_NV;
        // D s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 1u16);
        // C s_19_2: const #1u : u8
        let s_19_2: bool = true;
        // C s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 1u16);
        // D s_19_4: cmp-eq s_19_1 s_19_3
        let s_19_4: bool = ((s_19_1) == (s_19_3));
        // D s_19_5: write-var gs#62496 <= s_19_4
        fn_state.gs_62496 = s_19_4;
        // N s_19_6: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1024u : u12
        let s_20_0: u16 = 1024;
        // C s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 12u16);
        // C s_20_2: cast zx s_20_1 -> i
        let s_20_2: i128 = (s_20_1.value() as i128);
        // C s_20_3: cast reint s_20_2 -> i64
        let s_20_3: i64 = (s_20_2 as i64);
        // C s_20_4: const #32s : i64
        let s_20_4: i64 = 32;
        // C s_20_5: cast zx s_20_3 -> i
        let s_20_5: i128 = (i128::try_from(s_20_3).unwrap());
        // C s_20_6: cast zx s_20_4 -> i
        let s_20_6: i128 = (i128::try_from(s_20_4).unwrap());
        // C s_20_7: add s_20_5 s_20_6
        let s_20_7: i128 = (s_20_5 + s_20_6);
        // C s_20_8: cast reint s_20_7 -> i64
        let s_20_8: i64 = (s_20_7 as i64);
        // C s_20_9: cast zx s_20_8 -> i
        let s_20_9: i128 = (i128::try_from(s_20_8).unwrap());
        // S s_20_10: call NVMem_read(s_20_9)
        let s_20_10: u64 = NVMem_read(state, tracer, s_20_9);
        // D s_20_11: write-var ga#68465 <= s_20_10
        fn_state.ga_68465 = s_20_10;
        // N s_20_12: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var ga#68465:u64
        let s_21_0: u64 = fn_state.ga_68465;
        // D s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 64u16);
        // D s_21_2: read-var t:i
        let s_21_2: i128 = fn_state.t;
        // C s_21_3: const #64s : i64
        let s_21_3: i64 = 64;
        // D s_21_4: call X_set(s_21_2, s_21_3, s_21_1)
        let s_21_4: () = X_set(state, tracer, s_21_2, s_21_3, s_21_1);
        // N s_21_5: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #102552u : u32
        let s_22_0: u32 = 102552;
        // D s_22_1: read-reg s_22_0:struct
        let s_22_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_22_0 as isize);
            tracer.read_register(s_22_0 as isize, value);
            value
        };
        // D s_22_2: call _get_HCR_EL2_Type_NV2(s_22_1)
        let s_22_2: bool = u_get_HCR_EL2_Type_NV2(state, tracer, s_22_1);
        // C s_22_3: const #102552u : u32
        let s_22_3: u32 = 102552;
        // D s_22_4: read-reg s_22_3:struct
        let s_22_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_22_3 as isize);
            tracer.read_register(s_22_3 as isize, value);
            value
        };
        // D s_22_5: call _get_HCR_EL2_Type_NV(s_22_4)
        let s_22_5: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_22_4);
        // D s_22_6: cast zx s_22_2 -> bv
        let s_22_6: Bits = Bits::new(s_22_2 as u128, 1u16);
        // D s_22_7: cast zx s_22_5 -> bv
        let s_22_7: Bits = Bits::new(s_22_5 as u128, 1u16);
        // D s_22_8: cast reint s_22_6 -> u128
        let s_22_8: u128 = (s_22_6.value() as u128);
        // D s_22_9: size-of s_22_6
        let s_22_9: u16 = s_22_6.length();
        // D s_22_10: cast reint s_22_7 -> u128
        let s_22_10: u128 = (s_22_7.value() as u128);
        // D s_22_11: size-of s_22_7
        let s_22_11: u16 = s_22_7.length();
        // D s_22_12: lsl s_22_8 s_22_11
        let s_22_12: u128 = s_22_8 << s_22_11;
        // D s_22_13: or s_22_12 s_22_10
        let s_22_13: u128 = ((s_22_12) | (s_22_10));
        // D s_22_14: add s_22_9 s_22_11
        let s_22_14: u16 = (s_22_9 + s_22_11);
        // D s_22_15: create-bits s_22_13 s_22_14
        let s_22_15: Bits = Bits::new(s_22_13, s_22_14);
        // D s_22_16: cast reint s_22_15 -> u8
        let s_22_16: u8 = (s_22_15.value() as u8);
        // D s_22_17: cast zx s_22_16 -> bv
        let s_22_17: Bits = Bits::new(s_22_16 as u128, 2u16);
        // C s_22_18: const #3u : u8
        let s_22_18: u8 = 3;
        // C s_22_19: cast zx s_22_18 -> bv
        let s_22_19: Bits = Bits::new(s_22_18 as u128, 2u16);
        // D s_22_20: cmp-eq s_22_17 s_22_19
        let s_22_20: bool = ((s_22_17) == (s_22_19));
        // D s_22_21: write-var gs#62495 <= s_22_20
        fn_state.gs_62495 = s_22_20;
        // N s_22_22: jump b13
        return block_13(state, tracer, fn_state);
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
}
