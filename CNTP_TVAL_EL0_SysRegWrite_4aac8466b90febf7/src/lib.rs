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
use ELUsingAArch32::*;
use u_get_HCR_EL2_Type_NV::*;
use Mk_CNTP_CVAL_EL0_Type::*;
use X_read::*;
use AArch64_SystemAccessTrap::*;
use EL2Enabled::*;
use PhysicalCountInt::*;
use common::*;
pub fn CNTP_TVAL_EL0_SysRegWrite_4aac8466b90febf7<T: Tracer>(
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
        gs_81860: bool,
        u__HCR_EL2_E2H: bool,
        gs_81869: bool,
        u__PSTATE_EL: u8,
        u__HCR_EL2_NV: bool,
        gs_81859: bool,
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
        // N s_0_17: branch s_0_16 b23 b1
        if s_0_16 {
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
        // D s_6_1: write-var gs#81859 <= s_6_0
        fn_state.gs_81859 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#81859:u8
        let s_7_0: bool = fn_state.gs_81859;
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
        // D s_8_1: write-var gs#81860 <= s_8_0
        fn_state.gs_81860 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#81860:u8
        let s_9_0: bool = fn_state.gs_81860;
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
        // C s_11_4: const #0s : i
        let s_11_4: i128 = 0;
        // D s_11_5: cast zx s_11_3 -> bv
        let s_11_5: Bits = Bits::new(s_11_3 as u128, 64u16);
        // C s_11_6: const #1s : i64
        let s_11_6: i64 = 1;
        // C s_11_7: cast zx s_11_6 -> i
        let s_11_7: i128 = (i128::try_from(s_11_6).unwrap());
        // C s_11_8: const #31s : i
        let s_11_8: i128 = 31;
        // C s_11_9: add s_11_8 s_11_7
        let s_11_9: i128 = (s_11_8 + s_11_7);
        // D s_11_10: bit-extract s_11_5 s_11_4 s_11_9
        let s_11_10: Bits = (Bits::new(
            ((s_11_5) >> (s_11_4)).value(),
            u16::try_from(s_11_9).unwrap(),
        ));
        // D s_11_11: cast reint s_11_10 -> u32
        let s_11_11: u32 = (s_11_10.value() as u32);
        // C s_11_12: const #64s : i
        let s_11_12: i128 = 64;
        // D s_11_13: cast zx s_11_11 -> bv
        let s_11_13: Bits = Bits::new(s_11_11 as u128, 32u16);
        // D s_11_14: bits-cast sx s_11_13 -> bv length s_11_12
        let s_11_14: Bits = s_11_13.sign_extend(s_11_12);
        // D s_11_15: cast reint s_11_14 -> u64
        let s_11_15: u64 = (s_11_14.value() as u64);
        // C s_11_16: const #() : ()
        let s_11_16: () = ();
        // S s_11_17: call PhysicalCountInt(s_11_16)
        let s_11_17: u64 = PhysicalCountInt(state, tracer, s_11_16);
        // D s_11_18: cast zx s_11_15 -> bv
        let s_11_18: Bits = Bits::new(s_11_15 as u128, 64u16);
        // S s_11_19: cast zx s_11_17 -> bv
        let s_11_19: Bits = Bits::new(s_11_17 as u128, 64u16);
        // D s_11_20: add s_11_18 s_11_19
        let s_11_20: Bits = (s_11_18 + s_11_19);
        // D s_11_21: cast reint s_11_20 -> u64
        let s_11_21: u64 = (s_11_20.value() as u64);
        // D s_11_22: call Mk_CNTP_CVAL_EL0_Type(s_11_21)
        let s_11_22: ProductType5c790c8ef59cc8b2 = Mk_CNTP_CVAL_EL0_Type(
            state,
            tracer,
            s_11_21,
        );
        // C s_11_23: const #20800u : u32
        let s_11_23: u32 = 20800;
        // N s_11_24: write-reg s_11_23 <= s_11_22
        let s_11_24: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_11_23 as isize, s_11_22);
            tracer.write_register(s_11_23 as isize, s_11_22);
        };
        // N s_11_25: return
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
        // D s_12_5: write-var gs#81860 <= s_12_4
        fn_state.gs_81860 = s_12_4;
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
        // D s_13_4: write-var gs#81859 <= s_13_3
        fn_state.gs_81859 = s_13_3;
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
        // C s_16_4: const #0s : i
        let s_16_4: i128 = 0;
        // D s_16_5: cast zx s_16_3 -> bv
        let s_16_5: Bits = Bits::new(s_16_3 as u128, 64u16);
        // C s_16_6: const #1s : i64
        let s_16_6: i64 = 1;
        // C s_16_7: cast zx s_16_6 -> i
        let s_16_7: i128 = (i128::try_from(s_16_6).unwrap());
        // C s_16_8: const #31s : i
        let s_16_8: i128 = 31;
        // C s_16_9: add s_16_8 s_16_7
        let s_16_9: i128 = (s_16_8 + s_16_7);
        // D s_16_10: bit-extract s_16_5 s_16_4 s_16_9
        let s_16_10: Bits = (Bits::new(
            ((s_16_5) >> (s_16_4)).value(),
            u16::try_from(s_16_9).unwrap(),
        ));
        // D s_16_11: cast reint s_16_10 -> u32
        let s_16_11: u32 = (s_16_10.value() as u32);
        // C s_16_12: const #64s : i
        let s_16_12: i128 = 64;
        // D s_16_13: cast zx s_16_11 -> bv
        let s_16_13: Bits = Bits::new(s_16_11 as u128, 32u16);
        // D s_16_14: bits-cast sx s_16_13 -> bv length s_16_12
        let s_16_14: Bits = s_16_13.sign_extend(s_16_12);
        // D s_16_15: cast reint s_16_14 -> u64
        let s_16_15: u64 = (s_16_14.value() as u64);
        // C s_16_16: const #() : ()
        let s_16_16: () = ();
        // S s_16_17: call PhysicalCountInt(s_16_16)
        let s_16_17: u64 = PhysicalCountInt(state, tracer, s_16_16);
        // D s_16_18: cast zx s_16_15 -> bv
        let s_16_18: Bits = Bits::new(s_16_15 as u128, 64u16);
        // S s_16_19: cast zx s_16_17 -> bv
        let s_16_19: Bits = Bits::new(s_16_17 as u128, 64u16);
        // D s_16_20: add s_16_18 s_16_19
        let s_16_20: Bits = (s_16_18 + s_16_19);
        // D s_16_21: cast reint s_16_20 -> u64
        let s_16_21: u64 = (s_16_20.value() as u64);
        // D s_16_22: call Mk_CNTP_CVAL_EL0_Type(s_16_21)
        let s_16_22: ProductType5c790c8ef59cc8b2 = Mk_CNTP_CVAL_EL0_Type(
            state,
            tracer,
            s_16_21,
        );
        // C s_16_23: const #20800u : u32
        let s_16_23: u32 = 20800;
        // N s_16_24: write-reg s_16_23 <= s_16_22
        let s_16_24: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_16_23 as isize, s_16_22);
            tracer.write_register(s_16_23 as isize, s_16_22);
        };
        // N s_16_25: return
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
        // N s_17_2: branch s_17_1 b22 b18
        if s_17_1 {
            return block_22(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#81869 <= s_18_0
        fn_state.gs_81869 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#81869:u8
        let s_19_0: bool = fn_state.gs_81869;
        // N s_19_1: branch s_19_0 b21 b20
        if s_19_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: panic
        panic!("{:?}", ());
        // N s_20_1: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #24u : u8
        let s_21_0: u8 = 24;
        // C s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 8u16);
        // C s_21_2: cast zx s_21_1 -> i
        let s_21_2: i128 = (s_21_1.value() as i128);
        // C s_21_3: cast reint s_21_2 -> i64
        let s_21_3: i64 = (s_21_2 as i64);
        // C s_21_4: cast zx s_21_3 -> i
        let s_21_4: i128 = (i128::try_from(s_21_3).unwrap());
        // C s_21_5: const #432u : u32
        let s_21_5: u32 = 432;
        // D s_21_6: read-reg s_21_5:u8
        let s_21_6: u8 = {
            let value = state.read_register::<u8>(s_21_5 as isize);
            tracer.read_register(s_21_5 as isize, value);
            value
        };
        // D s_21_7: call AArch64_SystemAccessTrap(s_21_6, s_21_4)
        let s_21_7: () = AArch64_SystemAccessTrap(state, tracer, s_21_6, s_21_4);
        // N s_21_8: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var __HCR_EL2_NV:u8
        let s_22_0: bool = fn_state.u__HCR_EL2_NV;
        // D s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 1u16);
        // C s_22_2: const #1u : u8
        let s_22_2: bool = true;
        // C s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 1u16);
        // D s_22_4: cmp-eq s_22_1 s_22_3
        let s_22_4: bool = ((s_22_1) == (s_22_3));
        // D s_22_5: write-var gs#81869 <= s_22_4
        fn_state.gs_81869 = s_22_4;
        // N s_22_6: jump b19
        return block_19(state, tracer, fn_state);
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
