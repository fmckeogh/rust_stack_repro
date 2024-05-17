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
use u_get_HCR_EL2_Type_E2H::*;
use u_get_HCR_EL2_Type_NV::*;
use NVMem_read::*;
use AArch64_SystemAccessTrap::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_NV1::*;
use u_get_HCR_EL2_Type_NV2::*;
use common::*;
pub fn ELR_EL2_SysRegRead_cdeb265f6e920d74<T: Tracer>(
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
        gs_61279: bool,
        ga_64616: u64,
        gs_61280: bool,
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
        // D s_0_5: call _get_HCR_EL2_Type_E2H(s_0_4)
        let s_0_5: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_0_4);
        // D s_0_6: write-var __HCR_EL2_E2H <= s_0_5
        fn_state.u__HCR_EL2_E2H = s_0_5;
        // D s_0_7: read-var __PSTATE_EL:u8
        let s_0_7: u8 = fn_state.u__PSTATE_EL;
        // D s_0_8: cast zx s_0_7 -> bv
        let s_0_8: Bits = Bits::new(s_0_7 as u128, 2u16);
        // C s_0_9: const #448u : u32
        let s_0_9: u32 = 448;
        // D s_0_10: read-reg s_0_9:u8
        let s_0_10: u8 = {
            let value = state.read_register::<u8>(s_0_9 as isize);
            tracer.read_register(s_0_9 as isize, value);
            value
        };
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 2u16);
        // D s_0_12: cmp-eq s_0_8 s_0_11
        let s_0_12: bool = ((s_0_8) == (s_0_11));
        // N s_0_13: branch s_0_12 b21 b1
        if s_0_12 {
            return block_21(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b9 b2
        if s_1_5 {
            return block_9(state, tracer, fn_state);
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
        // C s_5_1: const #18312u : u32
        let s_5_1: u32 = 18312;
        // D s_5_2: read-reg s_5_1:u64
        let s_5_2: u64 = {
            let value = state.read_register::<u64>(s_5_1 as isize);
            tracer.read_register(s_5_1 as isize, value);
            value
        };
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 64u16);
        // D s_5_4: read-var t:i
        let s_5_4: i128 = fn_state.t;
        // D s_5_5: call X_set(s_5_4, s_5_0, s_5_3)
        let s_5_5: () = X_set(state, tracer, s_5_4, s_5_0, s_5_3);
        // N s_5_6: return
        return;
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
        // C s_7_0: const #64s : i64
        let s_7_0: i64 = 64;
        // C s_7_1: const #18312u : u32
        let s_7_1: u32 = 18312;
        // D s_7_2: read-reg s_7_1:u64
        let s_7_2: u64 = {
            let value = state.read_register::<u64>(s_7_1 as isize);
            tracer.read_register(s_7_1 as isize, value);
            value
        };
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 64u16);
        // D s_7_4: read-var t:i
        let s_7_4: i128 = fn_state.t;
        // D s_7_5: call X_set(s_7_4, s_7_0, s_7_3)
        let s_7_5: () = X_set(state, tracer, s_7_4, s_7_0, s_7_3);
        // N s_7_6: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #64s : i64
        let s_8_0: i64 = 64;
        // C s_8_1: const #17224u : u32
        let s_8_1: u32 = 17224;
        // D s_8_2: read-reg s_8_1:u64
        let s_8_2: u64 = {
            let value = state.read_register::<u64>(s_8_1 as isize);
            tracer.read_register(s_8_1 as isize, value);
            value
        };
        // D s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 64u16);
        // D s_8_4: read-var t:i
        let s_8_4: i128 = fn_state.t;
        // D s_8_5: call X_set(s_8_4, s_8_0, s_8_3)
        let s_8_5: () = X_set(state, tracer, s_8_4, s_8_0, s_8_3);
        // N s_8_6: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call EL2Enabled(s_9_0)
        let s_9_1: bool = EL2Enabled(state, tracer, s_9_0);
        // N s_9_2: branch s_9_1 b20 b10
        if s_9_1 {
            return block_20(state, tracer, fn_state);
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
        // D s_10_1: write-var gs#61279 <= s_10_0
        fn_state.gs_61279 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#61279:u8
        let s_11_0: bool = fn_state.gs_61279;
        // N s_11_1: branch s_11_0 b19 b12
        if s_11_0 {
            return block_19(state, tracer, fn_state);
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
        // N s_12_2: branch s_12_1 b18 b13
        if s_12_1 {
            return block_18(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#61280 <= s_13_0
        fn_state.gs_61280 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#61280:u8
        let s_14_0: bool = fn_state.gs_61280;
        // N s_14_1: branch s_14_0 b16 b15
        if s_14_0 {
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
        // C s_15_0: const #64s : i64
        let s_15_0: i64 = 64;
        // C s_15_1: const #18312u : u32
        let s_15_1: u32 = 18312;
        // D s_15_2: read-reg s_15_1:u64
        let s_15_2: u64 = {
            let value = state.read_register::<u64>(s_15_1 as isize);
            tracer.read_register(s_15_1 as isize, value);
            value
        };
        // D s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 64u16);
        // D s_15_4: read-var t:i
        let s_15_4: i128 = fn_state.t;
        // D s_15_5: call X_set(s_15_4, s_15_0, s_15_3)
        let s_15_5: () = X_set(state, tracer, s_15_4, s_15_0, s_15_3);
        // N s_15_6: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #560u : u12
        let s_16_0: u16 = 560;
        // C s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 12u16);
        // C s_16_2: cast zx s_16_1 -> i
        let s_16_2: i128 = (s_16_1.value() as i128);
        // C s_16_3: cast reint s_16_2 -> i64
        let s_16_3: i64 = (s_16_2 as i64);
        // C s_16_4: cast zx s_16_3 -> i
        let s_16_4: i128 = (i128::try_from(s_16_3).unwrap());
        // S s_16_5: call NVMem_read(s_16_4)
        let s_16_5: u64 = NVMem_read(state, tracer, s_16_4);
        // D s_16_6: write-var ga#64616 <= s_16_5
        fn_state.ga_64616 = s_16_5;
        // N s_16_7: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var ga#64616:u64
        let s_17_0: u64 = fn_state.ga_64616;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 64u16);
        // D s_17_2: read-var t:i
        let s_17_2: i128 = fn_state.t;
        // C s_17_3: const #64s : i64
        let s_17_3: i64 = 64;
        // D s_17_4: call X_set(s_17_2, s_17_3, s_17_1)
        let s_17_4: () = X_set(state, tracer, s_17_2, s_17_3, s_17_1);
        // N s_17_5: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #102552u : u32
        let s_18_0: u32 = 102552;
        // D s_18_1: read-reg s_18_0:struct
        let s_18_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // D s_18_2: call _get_HCR_EL2_Type_NV2(s_18_1)
        let s_18_2: bool = u_get_HCR_EL2_Type_NV2(state, tracer, s_18_1);
        // C s_18_3: const #102552u : u32
        let s_18_3: u32 = 102552;
        // D s_18_4: read-reg s_18_3:struct
        let s_18_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_18_3 as isize);
            tracer.read_register(s_18_3 as isize, value);
            value
        };
        // D s_18_5: call _get_HCR_EL2_Type_NV1(s_18_4)
        let s_18_5: bool = u_get_HCR_EL2_Type_NV1(state, tracer, s_18_4);
        // C s_18_6: const #102552u : u32
        let s_18_6: u32 = 102552;
        // D s_18_7: read-reg s_18_6:struct
        let s_18_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_18_6 as isize);
            tracer.read_register(s_18_6 as isize, value);
            value
        };
        // D s_18_8: call _get_HCR_EL2_Type_NV(s_18_7)
        let s_18_8: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_18_7);
        // D s_18_9: cast zx s_18_5 -> bv
        let s_18_9: Bits = Bits::new(s_18_5 as u128, 1u16);
        // D s_18_10: cast zx s_18_8 -> bv
        let s_18_10: Bits = Bits::new(s_18_8 as u128, 1u16);
        // D s_18_11: cast reint s_18_9 -> u128
        let s_18_11: u128 = (s_18_9.value() as u128);
        // D s_18_12: size-of s_18_9
        let s_18_12: u16 = s_18_9.length();
        // D s_18_13: cast reint s_18_10 -> u128
        let s_18_13: u128 = (s_18_10.value() as u128);
        // D s_18_14: size-of s_18_10
        let s_18_14: u16 = s_18_10.length();
        // D s_18_15: lsl s_18_11 s_18_14
        let s_18_15: u128 = s_18_11 << s_18_14;
        // D s_18_16: or s_18_15 s_18_13
        let s_18_16: u128 = ((s_18_15) | (s_18_13));
        // D s_18_17: add s_18_12 s_18_14
        let s_18_17: u16 = (s_18_12 + s_18_14);
        // D s_18_18: create-bits s_18_16 s_18_17
        let s_18_18: Bits = Bits::new(s_18_16, s_18_17);
        // D s_18_19: cast reint s_18_18 -> u8
        let s_18_19: u8 = (s_18_18.value() as u8);
        // D s_18_20: cast zx s_18_2 -> bv
        let s_18_20: Bits = Bits::new(s_18_2 as u128, 1u16);
        // D s_18_21: cast zx s_18_19 -> bv
        let s_18_21: Bits = Bits::new(s_18_19 as u128, 2u16);
        // D s_18_22: cast reint s_18_20 -> u128
        let s_18_22: u128 = (s_18_20.value() as u128);
        // D s_18_23: size-of s_18_20
        let s_18_23: u16 = s_18_20.length();
        // D s_18_24: cast reint s_18_21 -> u128
        let s_18_24: u128 = (s_18_21.value() as u128);
        // D s_18_25: size-of s_18_21
        let s_18_25: u16 = s_18_21.length();
        // D s_18_26: lsl s_18_22 s_18_25
        let s_18_26: u128 = s_18_22 << s_18_25;
        // D s_18_27: or s_18_26 s_18_24
        let s_18_27: u128 = ((s_18_26) | (s_18_24));
        // D s_18_28: add s_18_23 s_18_25
        let s_18_28: u16 = (s_18_23 + s_18_25);
        // D s_18_29: create-bits s_18_27 s_18_28
        let s_18_29: Bits = Bits::new(s_18_27, s_18_28);
        // D s_18_30: cast reint s_18_29 -> u8
        let s_18_30: u8 = (s_18_29.value() as u8);
        // D s_18_31: cast zx s_18_30 -> bv
        let s_18_31: Bits = Bits::new(s_18_30 as u128, 3u16);
        // C s_18_32: const #7u : u8
        let s_18_32: u8 = 7;
        // C s_18_33: cast zx s_18_32 -> bv
        let s_18_33: Bits = Bits::new(s_18_32 as u128, 3u16);
        // D s_18_34: cmp-eq s_18_31 s_18_33
        let s_18_34: bool = ((s_18_31) == (s_18_33));
        // D s_18_35: write-var gs#61280 <= s_18_34
        fn_state.gs_61280 = s_18_34;
        // N s_18_36: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #24u : u8
        let s_19_0: u8 = 24;
        // C s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 8u16);
        // C s_19_2: cast zx s_19_1 -> i
        let s_19_2: i128 = (s_19_1.value() as i128);
        // C s_19_3: cast reint s_19_2 -> i64
        let s_19_3: i64 = (s_19_2 as i64);
        // C s_19_4: cast zx s_19_3 -> i
        let s_19_4: i128 = (i128::try_from(s_19_3).unwrap());
        // C s_19_5: const #432u : u32
        let s_19_5: u32 = 432;
        // D s_19_6: read-reg s_19_5:u8
        let s_19_6: u8 = {
            let value = state.read_register::<u8>(s_19_5 as isize);
            tracer.read_register(s_19_5 as isize, value);
            value
        };
        // D s_19_7: call AArch64_SystemAccessTrap(s_19_6, s_19_4)
        let s_19_7: () = AArch64_SystemAccessTrap(state, tracer, s_19_6, s_19_4);
        // N s_19_8: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #102552u : u32
        let s_20_0: u32 = 102552;
        // D s_20_1: read-reg s_20_0:struct
        let s_20_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_20_0 as isize);
            tracer.read_register(s_20_0 as isize, value);
            value
        };
        // D s_20_2: call _get_HCR_EL2_Type_NV2(s_20_1)
        let s_20_2: bool = u_get_HCR_EL2_Type_NV2(state, tracer, s_20_1);
        // C s_20_3: const #102552u : u32
        let s_20_3: u32 = 102552;
        // D s_20_4: read-reg s_20_3:struct
        let s_20_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_20_3 as isize);
            tracer.read_register(s_20_3 as isize, value);
            value
        };
        // D s_20_5: call _get_HCR_EL2_Type_NV1(s_20_4)
        let s_20_5: bool = u_get_HCR_EL2_Type_NV1(state, tracer, s_20_4);
        // C s_20_6: const #102552u : u32
        let s_20_6: u32 = 102552;
        // D s_20_7: read-reg s_20_6:struct
        let s_20_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_20_6 as isize);
            tracer.read_register(s_20_6 as isize, value);
            value
        };
        // D s_20_8: call _get_HCR_EL2_Type_NV(s_20_7)
        let s_20_8: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_20_7);
        // D s_20_9: cast zx s_20_5 -> bv
        let s_20_9: Bits = Bits::new(s_20_5 as u128, 1u16);
        // D s_20_10: cast zx s_20_8 -> bv
        let s_20_10: Bits = Bits::new(s_20_8 as u128, 1u16);
        // D s_20_11: cast reint s_20_9 -> u128
        let s_20_11: u128 = (s_20_9.value() as u128);
        // D s_20_12: size-of s_20_9
        let s_20_12: u16 = s_20_9.length();
        // D s_20_13: cast reint s_20_10 -> u128
        let s_20_13: u128 = (s_20_10.value() as u128);
        // D s_20_14: size-of s_20_10
        let s_20_14: u16 = s_20_10.length();
        // D s_20_15: lsl s_20_11 s_20_14
        let s_20_15: u128 = s_20_11 << s_20_14;
        // D s_20_16: or s_20_15 s_20_13
        let s_20_16: u128 = ((s_20_15) | (s_20_13));
        // D s_20_17: add s_20_12 s_20_14
        let s_20_17: u16 = (s_20_12 + s_20_14);
        // D s_20_18: create-bits s_20_16 s_20_17
        let s_20_18: Bits = Bits::new(s_20_16, s_20_17);
        // D s_20_19: cast reint s_20_18 -> u8
        let s_20_19: u8 = (s_20_18.value() as u8);
        // D s_20_20: cast zx s_20_2 -> bv
        let s_20_20: Bits = Bits::new(s_20_2 as u128, 1u16);
        // D s_20_21: cast zx s_20_19 -> bv
        let s_20_21: Bits = Bits::new(s_20_19 as u128, 2u16);
        // D s_20_22: cast reint s_20_20 -> u128
        let s_20_22: u128 = (s_20_20.value() as u128);
        // D s_20_23: size-of s_20_20
        let s_20_23: u16 = s_20_20.length();
        // D s_20_24: cast reint s_20_21 -> u128
        let s_20_24: u128 = (s_20_21.value() as u128);
        // D s_20_25: size-of s_20_21
        let s_20_25: u16 = s_20_21.length();
        // D s_20_26: lsl s_20_22 s_20_25
        let s_20_26: u128 = s_20_22 << s_20_25;
        // D s_20_27: or s_20_26 s_20_24
        let s_20_27: u128 = ((s_20_26) | (s_20_24));
        // D s_20_28: add s_20_23 s_20_25
        let s_20_28: u16 = (s_20_23 + s_20_25);
        // D s_20_29: create-bits s_20_27 s_20_28
        let s_20_29: Bits = Bits::new(s_20_27, s_20_28);
        // D s_20_30: cast reint s_20_29 -> u8
        let s_20_30: u8 = (s_20_29.value() as u8);
        // D s_20_31: cast zx s_20_30 -> bv
        let s_20_31: Bits = Bits::new(s_20_30 as u128, 3u16);
        // C s_20_32: const #3u : u8
        let s_20_32: u8 = 3;
        // C s_20_33: cast zx s_20_32 -> bv
        let s_20_33: Bits = Bits::new(s_20_32 as u128, 3u16);
        // D s_20_34: cmp-eq s_20_31 s_20_33
        let s_20_34: bool = ((s_20_31) == (s_20_33));
        // D s_20_35: write-var gs#61279 <= s_20_34
        fn_state.gs_61279 = s_20_34;
        // N s_20_36: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_21_0: panic
        panic!("{:?}", ());
        // N s_21_1: return
        return;
    }
}
