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
use NVMem_set::*;
use u_get_HCR_EL2_Type_NV2::*;
use u_get_HCR_EL2_Type_NV::*;
use AArch64_SystemAccessTrap::*;
use X_read::*;
use u_get_SCR_EL3_Type_EEL2::*;
use IsCurrentSecurityState::*;
use Mk_VSTCR_EL2_Type::*;
use EL2Enabled::*;
use common::*;
pub fn VSTCR_EL2_SysRegWrite_9398e11d9c39d020<T: Tracer>(
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
        gs_97054: bool,
        u__PSTATE_EL: u8,
        gs_97055: bool,
        u__HCR_EL2_NV: bool,
        u__SCR_EL3_EEL2: bool,
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
        // C s_0_7: const #90704u : u32
        let s_0_7: u32 = 90704;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_SCR_EL3_Type_EEL2(s_0_8)
        let s_0_9: bool = u_get_SCR_EL3_Type_EEL2(state, tracer, s_0_8);
        // D s_0_10: write-var __SCR_EL3_EEL2 <= s_0_9
        fn_state.u__SCR_EL3_EEL2 = s_0_9;
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
        // N s_0_17: branch s_0_16 b24 b1
        if s_0_16 {
            return block_24(state, tracer, fn_state);
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
        // D s_5_0: read-var __SCR_EL3_EEL2:u8
        let s_5_0: bool = fn_state.u__SCR_EL3_EEL2;
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
        // D s_6_1: read-var t:i
        let s_6_1: i128 = fn_state.t;
        // D s_6_2: call X_read(s_6_1, s_6_0)
        let s_6_2: Bits = X_read(state, tracer, s_6_1, s_6_0);
        // D s_6_3: cast reint s_6_2 -> u64
        let s_6_3: u64 = (s_6_2.value() as u64);
        // D s_6_4: call Mk_VSTCR_EL2_Type(s_6_3)
        let s_6_4: ProductType5c790c8ef59cc8b2 = Mk_VSTCR_EL2_Type(state, tracer, s_6_3);
        // C s_6_5: const #11168u : u32
        let s_6_5: u32 = 11168;
        // N s_6_6: write-reg s_6_5 <= s_6_4
        let s_6_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_6_5 as isize, s_6_4);
            tracer.write_register(s_6_5 as isize, s_6_4);
        };
        // N s_6_7: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: panic
        panic!("{:?}", ());
        // N s_7_1: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #3u : u32
        let s_8_0: u32 = 3;
        // S s_8_1: call IsCurrentSecurityState(s_8_0)
        let s_8_1: bool = IsCurrentSecurityState(state, tracer, s_8_0);
        // S s_8_2: not s_8_1
        let s_8_2: bool = !s_8_1;
        // N s_8_3: branch s_8_2 b10 b9
        if s_8_2 {
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
        // D s_9_1: read-var t:i
        let s_9_1: i128 = fn_state.t;
        // D s_9_2: call X_read(s_9_1, s_9_0)
        let s_9_2: Bits = X_read(state, tracer, s_9_1, s_9_0);
        // D s_9_3: cast reint s_9_2 -> u64
        let s_9_3: u64 = (s_9_2.value() as u64);
        // D s_9_4: call Mk_VSTCR_EL2_Type(s_9_3)
        let s_9_4: ProductType5c790c8ef59cc8b2 = Mk_VSTCR_EL2_Type(state, tracer, s_9_3);
        // C s_9_5: const #11168u : u32
        let s_9_5: u32 = 11168;
        // N s_9_6: write-reg s_9_5 <= s_9_4
        let s_9_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_9_5 as isize, s_9_4);
            tracer.write_register(s_9_5 as isize, s_9_4);
        };
        // N s_9_7: return
        return;
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
        // C s_11_0: const #3u : u32
        let s_11_0: u32 = 3;
        // S s_11_1: call IsCurrentSecurityState(s_11_0)
        let s_11_1: bool = IsCurrentSecurityState(state, tracer, s_11_0);
        // S s_11_2: not s_11_1
        let s_11_2: bool = !s_11_1;
        // N s_11_3: branch s_11_2 b23 b12
        if s_11_2 {
            return block_23(state, tracer, fn_state);
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
        // N s_12_2: branch s_12_1 b22 b13
        if s_12_1 {
            return block_22(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#97054 <= s_13_0
        fn_state.gs_97054 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#97054:u8
        let s_14_0: bool = fn_state.gs_97054;
        // N s_14_1: branch s_14_0 b21 b15
        if s_14_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call EL2Enabled(s_15_0)
        let s_15_1: bool = EL2Enabled(state, tracer, s_15_0);
        // N s_15_2: branch s_15_1 b20 b16
        if s_15_1 {
            return block_20(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#97055 <= s_16_0
        fn_state.gs_97055 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#97055:u8
        let s_17_0: bool = fn_state.gs_97055;
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
        // N s_18_0: panic
        panic!("{:?}", ());
        // N s_18_1: return
        return;
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
        // D s_20_0: read-var __HCR_EL2_NV:u8
        let s_20_0: bool = fn_state.u__HCR_EL2_NV;
        // D s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 1u16);
        // C s_20_2: const #1u : u8
        let s_20_2: bool = true;
        // C s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 1u16);
        // D s_20_4: cmp-eq s_20_1 s_20_3
        let s_20_4: bool = ((s_20_1) == (s_20_3));
        // D s_20_5: write-var gs#97055 <= s_20_4
        fn_state.gs_97055 = s_20_4;
        // N s_20_6: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #72u : u12
        let s_21_0: u16 = 72;
        // C s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 12u16);
        // C s_21_2: cast zx s_21_1 -> i
        let s_21_2: i128 = (s_21_1.value() as i128);
        // C s_21_3: cast reint s_21_2 -> i64
        let s_21_3: i64 = (s_21_2 as i64);
        // C s_21_4: const #64s : i64
        let s_21_4: i64 = 64;
        // D s_21_5: read-var t:i
        let s_21_5: i128 = fn_state.t;
        // D s_21_6: call X_read(s_21_5, s_21_4)
        let s_21_6: Bits = X_read(state, tracer, s_21_5, s_21_4);
        // D s_21_7: cast reint s_21_6 -> u64
        let s_21_7: u64 = (s_21_6.value() as u64);
        // C s_21_8: cast zx s_21_3 -> i
        let s_21_8: i128 = (i128::try_from(s_21_3).unwrap());
        // D s_21_9: call NVMem_set(s_21_8, s_21_7)
        let s_21_9: () = NVMem_set(state, tracer, s_21_8, s_21_7);
        // N s_21_10: return
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
        // D s_22_21: write-var gs#97054 <= s_22_20
        fn_state.gs_97054 = s_22_20;
        // N s_22_22: jump b14
        return block_14(state, tracer, fn_state);
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
        // N s_24_0: panic
        panic!("{:?}", ());
        // N s_24_1: return
        return;
    }
}
