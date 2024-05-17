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
use EL2Enabled::*;
use u_get_HCR_EL2_Type_E2H::*;
use Mk_CNTV_CVAL_EL0_Type::*;
use IsFeatureImplemented::*;
use X_read::*;
use AArch64_SystemAccessTrap::*;
use u_get_SCR_EL3_Type_NS::*;
use u_get_CNTHCTL_EL2_Type_EL1TVT::*;
use ELUsingAArch32::*;
use Mk_CNTHVS_CVAL_EL2_Type::*;
use Mk_CNTHV_CVAL_EL2_Type::*;
use CNTVOFF_read::*;
use u_get_CNTHCTL_EL2_Type_EL0VTEN::*;
use u_get_CNTKCTL_EL1_Type_EL0VTEN::*;
use u_get_HCR_EL2_Type_TGE::*;
use PhysicalCountInt::*;
use common::*;
pub fn CNTHV_TVAL_EL2_SysRegWrite_b6bfac5d17f766fc<T: Tracer>(
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
        gs_81775: bool,
        gs_81782: bool,
        gs_81764: bool,
        gs_81781: bool,
        u__CNTHCTL_EL2_EL1TVT: bool,
        gs_81783: bool,
        gs_81773: bool,
        gs_81745: bool,
        gs_81746: bool,
        gs_81779: bool,
        gs_81732: bool,
        gs_81747: bool,
        u__CNTKCTL_EL1_EL0VTEN: bool,
        u__PSTATE_EL: u8,
        gs_81731: bool,
        gs_81778: bool,
        u__HCR_EL2_TGE: bool,
        gs_81784: bool,
        u__CNTHCTL_EL2_EL0VTEN: bool,
        gs_81776: bool,
        gs_81777: bool,
        gs_81785: bool,
        gs_81774: bool,
        gs_81802: bool,
        gs_81780: bool,
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
        // N s_0_25: branch s_0_24 b40 b1
        if s_0_24 {
            return block_40(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b32 b2
        if s_1_5 {
            return block_32(state, tracer, fn_state);
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
        // N s_2_6: branch s_2_5 b16 b3
        if s_2_5 {
            return block_16(state, tracer, fn_state);
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
        // C s_5_0: const #432u : u32
        let s_5_0: u32 = 432;
        // D s_5_1: read-reg s_5_0:u8
        let s_5_1: u8 = {
            let value = state.read_register::<u8>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // C s_5_2: const #2u : u8
        let s_5_2: u8 = 2;
        // D s_5_3: cmp-lt s_5_1 s_5_2
        let s_5_3: bool = ((s_5_1) < (s_5_2));
        // N s_5_4: branch s_5_3 b15 b6
        if s_5_3 {
            return block_15(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#81731 <= s_6_0
        fn_state.gs_81731 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#81731:u8
        let s_7_0: bool = fn_state.gs_81731;
        // N s_7_1: branch s_7_0 b14 b8
        if s_7_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
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
        // C s_8_2: const #2u : u8
        let s_8_2: u8 = 2;
        // D s_8_3: cmp-lt s_8_1 s_8_2
        let s_8_3: bool = ((s_8_1) < (s_8_2));
        // N s_8_4: branch s_8_3 b13 b9
        if s_8_3 {
            return block_13(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#81732 <= s_9_0
        fn_state.gs_81732 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#81732:u8
        let s_10_0: bool = fn_state.gs_81732;
        // N s_10_1: branch s_10_0 b12 b11
        if s_10_0 {
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
        // D s_11_22: call Mk_CNTV_CVAL_EL0_Type(s_11_21)
        let s_11_22: ProductType5c790c8ef59cc8b2 = Mk_CNTV_CVAL_EL0_Type(
            state,
            tracer,
            s_11_21,
        );
        // C s_11_23: const #23632u : u32
        let s_11_23: u32 = 23632;
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
        // C s_12_0: const #64s : i64
        let s_12_0: i64 = 64;
        // D s_12_1: read-var t:i
        let s_12_1: i128 = fn_state.t;
        // D s_12_2: call X_read(s_12_1, s_12_0)
        let s_12_2: Bits = X_read(state, tracer, s_12_1, s_12_0);
        // D s_12_3: cast reint s_12_2 -> u64
        let s_12_3: u64 = (s_12_2.value() as u64);
        // C s_12_4: const #0s : i
        let s_12_4: i128 = 0;
        // D s_12_5: cast zx s_12_3 -> bv
        let s_12_5: Bits = Bits::new(s_12_3 as u128, 64u16);
        // C s_12_6: const #1s : i64
        let s_12_6: i64 = 1;
        // C s_12_7: cast zx s_12_6 -> i
        let s_12_7: i128 = (i128::try_from(s_12_6).unwrap());
        // C s_12_8: const #31s : i
        let s_12_8: i128 = 31;
        // C s_12_9: add s_12_8 s_12_7
        let s_12_9: i128 = (s_12_8 + s_12_7);
        // D s_12_10: bit-extract s_12_5 s_12_4 s_12_9
        let s_12_10: Bits = (Bits::new(
            ((s_12_5) >> (s_12_4)).value(),
            u16::try_from(s_12_9).unwrap(),
        ));
        // D s_12_11: cast reint s_12_10 -> u32
        let s_12_11: u32 = (s_12_10.value() as u32);
        // C s_12_12: const #64s : i
        let s_12_12: i128 = 64;
        // D s_12_13: cast zx s_12_11 -> bv
        let s_12_13: Bits = Bits::new(s_12_11 as u128, 32u16);
        // D s_12_14: bits-cast sx s_12_13 -> bv length s_12_12
        let s_12_14: Bits = s_12_13.sign_extend(s_12_12);
        // D s_12_15: cast reint s_12_14 -> u64
        let s_12_15: u64 = (s_12_14.value() as u64);
        // C s_12_16: const #() : ()
        let s_12_16: () = ();
        // S s_12_17: call PhysicalCountInt(s_12_16)
        let s_12_17: u64 = PhysicalCountInt(state, tracer, s_12_16);
        // D s_12_18: cast zx s_12_15 -> bv
        let s_12_18: Bits = Bits::new(s_12_15 as u128, 64u16);
        // S s_12_19: cast zx s_12_17 -> bv
        let s_12_19: Bits = Bits::new(s_12_17 as u128, 64u16);
        // D s_12_20: add s_12_18 s_12_19
        let s_12_20: Bits = (s_12_18 + s_12_19);
        // D s_12_21: cast reint s_12_20 -> u64
        let s_12_21: u64 = (s_12_20.value() as u64);
        // C s_12_22: const #() : ()
        let s_12_22: () = ();
        // S s_12_23: call CNTVOFF_read(s_12_22)
        let s_12_23: u64 = CNTVOFF_read(state, tracer, s_12_22);
        // D s_12_24: cast zx s_12_21 -> bv
        let s_12_24: Bits = Bits::new(s_12_21 as u128, 64u16);
        // S s_12_25: cast zx s_12_23 -> bv
        let s_12_25: Bits = Bits::new(s_12_23 as u128, 64u16);
        // D s_12_26: sub s_12_24 s_12_25
        let s_12_26: Bits = ((s_12_24) - (s_12_25));
        // D s_12_27: cast reint s_12_26 -> u64
        let s_12_27: u64 = (s_12_26.value() as u64);
        // D s_12_28: call Mk_CNTV_CVAL_EL0_Type(s_12_27)
        let s_12_28: ProductType5c790c8ef59cc8b2 = Mk_CNTV_CVAL_EL0_Type(
            state,
            tracer,
            s_12_27,
        );
        // C s_12_29: const #23632u : u32
        let s_12_29: u32 = 23632;
        // N s_12_30: write-reg s_12_29 <= s_12_28
        let s_12_30: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_29 as isize, s_12_28);
            tracer.write_register(s_12_29 as isize, s_12_28);
        };
        // N s_12_31: return
        return;
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
        // D s_13_3: write-var gs#81732 <= s_13_2
        fn_state.gs_81732 = s_13_2;
        // N s_13_4: jump b10
        return block_10(state, tracer, fn_state);
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
        // C s_14_4: const #0s : i
        let s_14_4: i128 = 0;
        // D s_14_5: cast zx s_14_3 -> bv
        let s_14_5: Bits = Bits::new(s_14_3 as u128, 64u16);
        // C s_14_6: const #1s : i64
        let s_14_6: i64 = 1;
        // C s_14_7: cast zx s_14_6 -> i
        let s_14_7: i128 = (i128::try_from(s_14_6).unwrap());
        // C s_14_8: const #31s : i
        let s_14_8: i128 = 31;
        // C s_14_9: add s_14_8 s_14_7
        let s_14_9: i128 = (s_14_8 + s_14_7);
        // D s_14_10: bit-extract s_14_5 s_14_4 s_14_9
        let s_14_10: Bits = (Bits::new(
            ((s_14_5) >> (s_14_4)).value(),
            u16::try_from(s_14_9).unwrap(),
        ));
        // D s_14_11: cast reint s_14_10 -> u32
        let s_14_11: u32 = (s_14_10.value() as u32);
        // C s_14_12: const #64s : i
        let s_14_12: i128 = 64;
        // D s_14_13: cast zx s_14_11 -> bv
        let s_14_13: Bits = Bits::new(s_14_11 as u128, 32u16);
        // D s_14_14: bits-cast sx s_14_13 -> bv length s_14_12
        let s_14_14: Bits = s_14_13.sign_extend(s_14_12);
        // D s_14_15: cast reint s_14_14 -> u64
        let s_14_15: u64 = (s_14_14.value() as u64);
        // C s_14_16: const #() : ()
        let s_14_16: () = ();
        // S s_14_17: call PhysicalCountInt(s_14_16)
        let s_14_17: u64 = PhysicalCountInt(state, tracer, s_14_16);
        // D s_14_18: cast zx s_14_15 -> bv
        let s_14_18: Bits = Bits::new(s_14_15 as u128, 64u16);
        // S s_14_19: cast zx s_14_17 -> bv
        let s_14_19: Bits = Bits::new(s_14_17 as u128, 64u16);
        // D s_14_20: add s_14_18 s_14_19
        let s_14_20: Bits = (s_14_18 + s_14_19);
        // D s_14_21: cast reint s_14_20 -> u64
        let s_14_21: u64 = (s_14_20.value() as u64);
        // D s_14_22: cast zx s_14_21 -> bv
        let s_14_22: Bits = Bits::new(s_14_21 as u128, 64u16);
        // C s_14_23: const #22400u : u32
        let s_14_23: u32 = 22400;
        // D s_14_24: read-reg s_14_23:u64
        let s_14_24: u64 = {
            let value = state.read_register::<u64>(s_14_23 as isize);
            tracer.read_register(s_14_23 as isize, value);
            value
        };
        // D s_14_25: cast zx s_14_24 -> bv
        let s_14_25: Bits = Bits::new(s_14_24 as u128, 64u16);
        // D s_14_26: sub s_14_22 s_14_25
        let s_14_26: Bits = ((s_14_22) - (s_14_25));
        // D s_14_27: cast reint s_14_26 -> u64
        let s_14_27: u64 = (s_14_26.value() as u64);
        // D s_14_28: call Mk_CNTV_CVAL_EL0_Type(s_14_27)
        let s_14_28: ProductType5c790c8ef59cc8b2 = Mk_CNTV_CVAL_EL0_Type(
            state,
            tracer,
            s_14_27,
        );
        // C s_14_29: const #23632u : u32
        let s_14_29: u32 = 23632;
        // N s_14_30: write-reg s_14_29 <= s_14_28
        let s_14_30: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_14_29 as isize, s_14_28);
            tracer.write_register(s_14_29 as isize, s_14_28);
        };
        // N s_14_31: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #432u : u32
        let s_15_0: u32 = 432;
        // D s_15_1: read-reg s_15_0:u8
        let s_15_1: u8 = {
            let value = state.read_register::<u8>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // D s_15_2: call ELUsingAArch32(s_15_1)
        let s_15_2: bool = ELUsingAArch32(state, tracer, s_15_1);
        // D s_15_3: not s_15_2
        let s_15_3: bool = !s_15_2;
        // D s_15_4: write-var gs#81731 <= s_15_3
        fn_state.gs_81731 = s_15_3;
        // N s_15_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #102552u : u32
        let s_16_0: u32 = 102552;
        // D s_16_1: read-reg s_16_0:struct
        let s_16_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // D s_16_2: call _get_HCR_EL2_Type_E2H(s_16_1)
        let s_16_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_16_1);
        // D s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 1u16);
        // C s_16_4: const #1u : u8
        let s_16_4: bool = true;
        // C s_16_5: cast zx s_16_4 -> bv
        let s_16_5: Bits = Bits::new(s_16_4 as u128, 1u16);
        // D s_16_6: cmp-eq s_16_3 s_16_5
        let s_16_6: bool = ((s_16_3) == (s_16_5));
        // N s_16_7: branch s_16_6 b31 b17
        if s_16_6 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#81745 <= s_17_0
        fn_state.gs_81745 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#81745:u8
        let s_18_0: bool = fn_state.gs_81745;
        // N s_18_1: branch s_18_0 b30 b19
        if s_18_0 {
            return block_30(state, tracer, fn_state);
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
        // D s_19_1: write-var gs#81746 <= s_19_0
        fn_state.gs_81746 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#81746:u8
        let s_20_0: bool = fn_state.gs_81746;
        // N s_20_1: branch s_20_0 b29 b21
        if s_20_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #102552u : u32
        let s_21_0: u32 = 102552;
        // D s_21_1: read-reg s_21_0:struct
        let s_21_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // D s_21_2: call _get_HCR_EL2_Type_E2H(s_21_1)
        let s_21_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_21_1);
        // D s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 1u16);
        // C s_21_4: const #1u : u8
        let s_21_4: bool = true;
        // C s_21_5: cast zx s_21_4 -> bv
        let s_21_5: Bits = Bits::new(s_21_4 as u128, 1u16);
        // D s_21_6: cmp-eq s_21_3 s_21_5
        let s_21_6: bool = ((s_21_3) == (s_21_5));
        // N s_21_7: branch s_21_6 b28 b22
        if s_21_6 {
            return block_28(state, tracer, fn_state);
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
        // D s_22_1: write-var gs#81747 <= s_22_0
        fn_state.gs_81747 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#81747:u8
        let s_23_0: bool = fn_state.gs_81747;
        // N s_23_1: branch s_23_0 b27 b24
        if s_23_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #102552u : u32
        let s_24_0: u32 = 102552;
        // D s_24_1: read-reg s_24_0:struct
        let s_24_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_24_0 as isize);
            tracer.read_register(s_24_0 as isize, value);
            value
        };
        // D s_24_2: call _get_HCR_EL2_Type_E2H(s_24_1)
        let s_24_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_24_1);
        // D s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 1u16);
        // C s_24_4: const #0u : u8
        let s_24_4: bool = false;
        // C s_24_5: cast zx s_24_4 -> bv
        let s_24_5: Bits = Bits::new(s_24_4 as u128, 1u16);
        // D s_24_6: cmp-eq s_24_3 s_24_5
        let s_24_6: bool = ((s_24_3) == (s_24_5));
        // N s_24_7: branch s_24_6 b26 b25
        if s_24_6 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #64s : i64
        let s_25_0: i64 = 64;
        // D s_25_1: read-var t:i
        let s_25_1: i128 = fn_state.t;
        // D s_25_2: call X_read(s_25_1, s_25_0)
        let s_25_2: Bits = X_read(state, tracer, s_25_1, s_25_0);
        // D s_25_3: cast reint s_25_2 -> u64
        let s_25_3: u64 = (s_25_2.value() as u64);
        // C s_25_4: const #0s : i
        let s_25_4: i128 = 0;
        // D s_25_5: cast zx s_25_3 -> bv
        let s_25_5: Bits = Bits::new(s_25_3 as u128, 64u16);
        // C s_25_6: const #1s : i64
        let s_25_6: i64 = 1;
        // C s_25_7: cast zx s_25_6 -> i
        let s_25_7: i128 = (i128::try_from(s_25_6).unwrap());
        // C s_25_8: const #31s : i
        let s_25_8: i128 = 31;
        // C s_25_9: add s_25_8 s_25_7
        let s_25_9: i128 = (s_25_8 + s_25_7);
        // D s_25_10: bit-extract s_25_5 s_25_4 s_25_9
        let s_25_10: Bits = (Bits::new(
            ((s_25_5) >> (s_25_4)).value(),
            u16::try_from(s_25_9).unwrap(),
        ));
        // D s_25_11: cast reint s_25_10 -> u32
        let s_25_11: u32 = (s_25_10.value() as u32);
        // C s_25_12: const #64s : i
        let s_25_12: i128 = 64;
        // D s_25_13: cast zx s_25_11 -> bv
        let s_25_13: Bits = Bits::new(s_25_11 as u128, 32u16);
        // D s_25_14: bits-cast sx s_25_13 -> bv length s_25_12
        let s_25_14: Bits = s_25_13.sign_extend(s_25_12);
        // D s_25_15: cast reint s_25_14 -> u64
        let s_25_15: u64 = (s_25_14.value() as u64);
        // C s_25_16: const #() : ()
        let s_25_16: () = ();
        // S s_25_17: call PhysicalCountInt(s_25_16)
        let s_25_17: u64 = PhysicalCountInt(state, tracer, s_25_16);
        // D s_25_18: cast zx s_25_15 -> bv
        let s_25_18: Bits = Bits::new(s_25_15 as u128, 64u16);
        // S s_25_19: cast zx s_25_17 -> bv
        let s_25_19: Bits = Bits::new(s_25_17 as u128, 64u16);
        // D s_25_20: add s_25_18 s_25_19
        let s_25_20: Bits = (s_25_18 + s_25_19);
        // D s_25_21: cast reint s_25_20 -> u64
        let s_25_21: u64 = (s_25_20.value() as u64);
        // D s_25_22: call Mk_CNTV_CVAL_EL0_Type(s_25_21)
        let s_25_22: ProductType5c790c8ef59cc8b2 = Mk_CNTV_CVAL_EL0_Type(
            state,
            tracer,
            s_25_21,
        );
        // C s_25_23: const #23632u : u32
        let s_25_23: u32 = 23632;
        // N s_25_24: write-reg s_25_23 <= s_25_22
        let s_25_24: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_25_23 as isize, s_25_22);
            tracer.write_register(s_25_23 as isize, s_25_22);
        };
        // N s_25_25: return
        return;
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
        // C s_26_4: const #0s : i
        let s_26_4: i128 = 0;
        // D s_26_5: cast zx s_26_3 -> bv
        let s_26_5: Bits = Bits::new(s_26_3 as u128, 64u16);
        // C s_26_6: const #1s : i64
        let s_26_6: i64 = 1;
        // C s_26_7: cast zx s_26_6 -> i
        let s_26_7: i128 = (i128::try_from(s_26_6).unwrap());
        // C s_26_8: const #31s : i
        let s_26_8: i128 = 31;
        // C s_26_9: add s_26_8 s_26_7
        let s_26_9: i128 = (s_26_8 + s_26_7);
        // D s_26_10: bit-extract s_26_5 s_26_4 s_26_9
        let s_26_10: Bits = (Bits::new(
            ((s_26_5) >> (s_26_4)).value(),
            u16::try_from(s_26_9).unwrap(),
        ));
        // D s_26_11: cast reint s_26_10 -> u32
        let s_26_11: u32 = (s_26_10.value() as u32);
        // C s_26_12: const #64s : i
        let s_26_12: i128 = 64;
        // D s_26_13: cast zx s_26_11 -> bv
        let s_26_13: Bits = Bits::new(s_26_11 as u128, 32u16);
        // D s_26_14: bits-cast sx s_26_13 -> bv length s_26_12
        let s_26_14: Bits = s_26_13.sign_extend(s_26_12);
        // D s_26_15: cast reint s_26_14 -> u64
        let s_26_15: u64 = (s_26_14.value() as u64);
        // C s_26_16: const #() : ()
        let s_26_16: () = ();
        // S s_26_17: call PhysicalCountInt(s_26_16)
        let s_26_17: u64 = PhysicalCountInt(state, tracer, s_26_16);
        // D s_26_18: cast zx s_26_15 -> bv
        let s_26_18: Bits = Bits::new(s_26_15 as u128, 64u16);
        // S s_26_19: cast zx s_26_17 -> bv
        let s_26_19: Bits = Bits::new(s_26_17 as u128, 64u16);
        // D s_26_20: add s_26_18 s_26_19
        let s_26_20: Bits = (s_26_18 + s_26_19);
        // D s_26_21: cast reint s_26_20 -> u64
        let s_26_21: u64 = (s_26_20.value() as u64);
        // D s_26_22: cast zx s_26_21 -> bv
        let s_26_22: Bits = Bits::new(s_26_21 as u128, 64u16);
        // C s_26_23: const #22400u : u32
        let s_26_23: u32 = 22400;
        // D s_26_24: read-reg s_26_23:u64
        let s_26_24: u64 = {
            let value = state.read_register::<u64>(s_26_23 as isize);
            tracer.read_register(s_26_23 as isize, value);
            value
        };
        // D s_26_25: cast zx s_26_24 -> bv
        let s_26_25: Bits = Bits::new(s_26_24 as u128, 64u16);
        // D s_26_26: sub s_26_22 s_26_25
        let s_26_26: Bits = ((s_26_22) - (s_26_25));
        // D s_26_27: cast reint s_26_26 -> u64
        let s_26_27: u64 = (s_26_26.value() as u64);
        // D s_26_28: call Mk_CNTV_CVAL_EL0_Type(s_26_27)
        let s_26_28: ProductType5c790c8ef59cc8b2 = Mk_CNTV_CVAL_EL0_Type(
            state,
            tracer,
            s_26_27,
        );
        // C s_26_29: const #23632u : u32
        let s_26_29: u32 = 23632;
        // N s_26_30: write-reg s_26_29 <= s_26_28
        let s_26_30: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_26_29 as isize, s_26_28);
            tracer.write_register(s_26_29 as isize, s_26_28);
        };
        // N s_26_31: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #64s : i64
        let s_27_0: i64 = 64;
        // D s_27_1: read-var t:i
        let s_27_1: i128 = fn_state.t;
        // D s_27_2: call X_read(s_27_1, s_27_0)
        let s_27_2: Bits = X_read(state, tracer, s_27_1, s_27_0);
        // D s_27_3: cast reint s_27_2 -> u64
        let s_27_3: u64 = (s_27_2.value() as u64);
        // C s_27_4: const #0s : i
        let s_27_4: i128 = 0;
        // D s_27_5: cast zx s_27_3 -> bv
        let s_27_5: Bits = Bits::new(s_27_3 as u128, 64u16);
        // C s_27_6: const #1s : i64
        let s_27_6: i64 = 1;
        // C s_27_7: cast zx s_27_6 -> i
        let s_27_7: i128 = (i128::try_from(s_27_6).unwrap());
        // C s_27_8: const #31s : i
        let s_27_8: i128 = 31;
        // C s_27_9: add s_27_8 s_27_7
        let s_27_9: i128 = (s_27_8 + s_27_7);
        // D s_27_10: bit-extract s_27_5 s_27_4 s_27_9
        let s_27_10: Bits = (Bits::new(
            ((s_27_5) >> (s_27_4)).value(),
            u16::try_from(s_27_9).unwrap(),
        ));
        // D s_27_11: cast reint s_27_10 -> u32
        let s_27_11: u32 = (s_27_10.value() as u32);
        // C s_27_12: const #64s : i
        let s_27_12: i128 = 64;
        // D s_27_13: cast zx s_27_11 -> bv
        let s_27_13: Bits = Bits::new(s_27_11 as u128, 32u16);
        // D s_27_14: bits-cast sx s_27_13 -> bv length s_27_12
        let s_27_14: Bits = s_27_13.sign_extend(s_27_12);
        // D s_27_15: cast reint s_27_14 -> u64
        let s_27_15: u64 = (s_27_14.value() as u64);
        // C s_27_16: const #() : ()
        let s_27_16: () = ();
        // S s_27_17: call PhysicalCountInt(s_27_16)
        let s_27_17: u64 = PhysicalCountInt(state, tracer, s_27_16);
        // D s_27_18: cast zx s_27_15 -> bv
        let s_27_18: Bits = Bits::new(s_27_15 as u128, 64u16);
        // S s_27_19: cast zx s_27_17 -> bv
        let s_27_19: Bits = Bits::new(s_27_17 as u128, 64u16);
        // D s_27_20: add s_27_18 s_27_19
        let s_27_20: Bits = (s_27_18 + s_27_19);
        // D s_27_21: cast reint s_27_20 -> u64
        let s_27_21: u64 = (s_27_20.value() as u64);
        // D s_27_22: call Mk_CNTHV_CVAL_EL2_Type(s_27_21)
        let s_27_22: ProductType5c790c8ef59cc8b2 = Mk_CNTHV_CVAL_EL2_Type(
            state,
            tracer,
            s_27_21,
        );
        // C s_27_23: const #103152u : u32
        let s_27_23: u32 = 103152;
        // N s_27_24: write-reg s_27_23 <= s_27_22
        let s_27_24: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_27_23 as isize, s_27_22);
            tracer.write_register(s_27_23 as isize, s_27_22);
        };
        // N s_27_25: return
        return;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #90704u : u32
        let s_28_0: u32 = 90704;
        // D s_28_1: read-reg s_28_0:struct
        let s_28_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_28_0 as isize);
            tracer.read_register(s_28_0 as isize, value);
            value
        };
        // D s_28_2: call _get_SCR_EL3_Type_NS(s_28_1)
        let s_28_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_28_1);
        // D s_28_3: cast zx s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 1u16);
        // C s_28_4: const #1u : u8
        let s_28_4: bool = true;
        // C s_28_5: cast zx s_28_4 -> bv
        let s_28_5: Bits = Bits::new(s_28_4 as u128, 1u16);
        // D s_28_6: cmp-eq s_28_3 s_28_5
        let s_28_6: bool = ((s_28_3) == (s_28_5));
        // D s_28_7: write-var gs#81747 <= s_28_6
        fn_state.gs_81747 = s_28_6;
        // N s_28_8: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #64s : i64
        let s_29_0: i64 = 64;
        // D s_29_1: read-var t:i
        let s_29_1: i128 = fn_state.t;
        // D s_29_2: call X_read(s_29_1, s_29_0)
        let s_29_2: Bits = X_read(state, tracer, s_29_1, s_29_0);
        // D s_29_3: cast reint s_29_2 -> u64
        let s_29_3: u64 = (s_29_2.value() as u64);
        // C s_29_4: const #0s : i
        let s_29_4: i128 = 0;
        // D s_29_5: cast zx s_29_3 -> bv
        let s_29_5: Bits = Bits::new(s_29_3 as u128, 64u16);
        // C s_29_6: const #1s : i64
        let s_29_6: i64 = 1;
        // C s_29_7: cast zx s_29_6 -> i
        let s_29_7: i128 = (i128::try_from(s_29_6).unwrap());
        // C s_29_8: const #31s : i
        let s_29_8: i128 = 31;
        // C s_29_9: add s_29_8 s_29_7
        let s_29_9: i128 = (s_29_8 + s_29_7);
        // D s_29_10: bit-extract s_29_5 s_29_4 s_29_9
        let s_29_10: Bits = (Bits::new(
            ((s_29_5) >> (s_29_4)).value(),
            u16::try_from(s_29_9).unwrap(),
        ));
        // D s_29_11: cast reint s_29_10 -> u32
        let s_29_11: u32 = (s_29_10.value() as u32);
        // C s_29_12: const #64s : i
        let s_29_12: i128 = 64;
        // D s_29_13: cast zx s_29_11 -> bv
        let s_29_13: Bits = Bits::new(s_29_11 as u128, 32u16);
        // D s_29_14: bits-cast sx s_29_13 -> bv length s_29_12
        let s_29_14: Bits = s_29_13.sign_extend(s_29_12);
        // D s_29_15: cast reint s_29_14 -> u64
        let s_29_15: u64 = (s_29_14.value() as u64);
        // C s_29_16: const #() : ()
        let s_29_16: () = ();
        // S s_29_17: call PhysicalCountInt(s_29_16)
        let s_29_17: u64 = PhysicalCountInt(state, tracer, s_29_16);
        // D s_29_18: cast zx s_29_15 -> bv
        let s_29_18: Bits = Bits::new(s_29_15 as u128, 64u16);
        // S s_29_19: cast zx s_29_17 -> bv
        let s_29_19: Bits = Bits::new(s_29_17 as u128, 64u16);
        // D s_29_20: add s_29_18 s_29_19
        let s_29_20: Bits = (s_29_18 + s_29_19);
        // D s_29_21: cast reint s_29_20 -> u64
        let s_29_21: u64 = (s_29_20.value() as u64);
        // D s_29_22: call Mk_CNTHVS_CVAL_EL2_Type(s_29_21)
        let s_29_22: ProductType5c790c8ef59cc8b2 = Mk_CNTHVS_CVAL_EL2_Type(
            state,
            tracer,
            s_29_21,
        );
        // C s_29_23: const #10064u : u32
        let s_29_23: u32 = 10064;
        // N s_29_24: write-reg s_29_23 <= s_29_22
        let s_29_24: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_29_23 as isize, s_29_22);
            tracer.write_register(s_29_23 as isize, s_29_22);
        };
        // N s_29_25: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #117u : u32
        let s_30_0: u32 = 117;
        // S s_30_1: call IsFeatureImplemented(s_30_0)
        let s_30_1: bool = IsFeatureImplemented(state, tracer, s_30_0);
        // D s_30_2: write-var gs#81746 <= s_30_1
        fn_state.gs_81746 = s_30_1;
        // N s_30_3: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #90704u : u32
        let s_31_0: u32 = 90704;
        // D s_31_1: read-reg s_31_0:struct
        let s_31_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_31_0 as isize);
            tracer.read_register(s_31_0 as isize, value);
            value
        };
        // D s_31_2: call _get_SCR_EL3_Type_NS(s_31_1)
        let s_31_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_31_1);
        // D s_31_3: cast zx s_31_2 -> bv
        let s_31_3: Bits = Bits::new(s_31_2 as u128, 1u16);
        // C s_31_4: const #0u : u8
        let s_31_4: bool = false;
        // C s_31_5: cast zx s_31_4 -> bv
        let s_31_5: Bits = Bits::new(s_31_4 as u128, 1u16);
        // D s_31_6: cmp-eq s_31_3 s_31_5
        let s_31_6: bool = ((s_31_3) == (s_31_5));
        // D s_31_7: write-var gs#81745 <= s_31_6
        fn_state.gs_81745 = s_31_6;
        // N s_31_8: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #() : ()
        let s_32_0: () = ();
        // S s_32_1: call EL2Enabled(s_32_0)
        let s_32_1: bool = EL2Enabled(state, tracer, s_32_0);
        // N s_32_2: branch s_32_1 b39 b33
        if s_32_1 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #0u : u8
        let s_33_0: bool = false;
        // D s_33_1: write-var gs#81764 <= s_33_0
        fn_state.gs_81764 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#81764:u8
        let s_34_0: bool = fn_state.gs_81764;
        // N s_34_1: branch s_34_0 b38 b35
        if s_34_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #432u : u32
        let s_35_0: u32 = 432;
        // D s_35_1: read-reg s_35_0:u8
        let s_35_1: u8 = {
            let value = state.read_register::<u8>(s_35_0 as isize);
            tracer.read_register(s_35_0 as isize, value);
            value
        };
        // C s_35_2: const #2u : u8
        let s_35_2: u8 = 2;
        // D s_35_3: cmp-lt s_35_1 s_35_2
        let s_35_3: bool = ((s_35_1) < (s_35_2));
        // N s_35_4: branch s_35_3 b37 b36
        if s_35_3 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #64s : i64
        let s_36_0: i64 = 64;
        // D s_36_1: read-var t:i
        let s_36_1: i128 = fn_state.t;
        // D s_36_2: call X_read(s_36_1, s_36_0)
        let s_36_2: Bits = X_read(state, tracer, s_36_1, s_36_0);
        // D s_36_3: cast reint s_36_2 -> u64
        let s_36_3: u64 = (s_36_2.value() as u64);
        // C s_36_4: const #0s : i
        let s_36_4: i128 = 0;
        // D s_36_5: cast zx s_36_3 -> bv
        let s_36_5: Bits = Bits::new(s_36_3 as u128, 64u16);
        // C s_36_6: const #1s : i64
        let s_36_6: i64 = 1;
        // C s_36_7: cast zx s_36_6 -> i
        let s_36_7: i128 = (i128::try_from(s_36_6).unwrap());
        // C s_36_8: const #31s : i
        let s_36_8: i128 = 31;
        // C s_36_9: add s_36_8 s_36_7
        let s_36_9: i128 = (s_36_8 + s_36_7);
        // D s_36_10: bit-extract s_36_5 s_36_4 s_36_9
        let s_36_10: Bits = (Bits::new(
            ((s_36_5) >> (s_36_4)).value(),
            u16::try_from(s_36_9).unwrap(),
        ));
        // D s_36_11: cast reint s_36_10 -> u32
        let s_36_11: u32 = (s_36_10.value() as u32);
        // C s_36_12: const #64s : i
        let s_36_12: i128 = 64;
        // D s_36_13: cast zx s_36_11 -> bv
        let s_36_13: Bits = Bits::new(s_36_11 as u128, 32u16);
        // D s_36_14: bits-cast sx s_36_13 -> bv length s_36_12
        let s_36_14: Bits = s_36_13.sign_extend(s_36_12);
        // D s_36_15: cast reint s_36_14 -> u64
        let s_36_15: u64 = (s_36_14.value() as u64);
        // C s_36_16: const #() : ()
        let s_36_16: () = ();
        // S s_36_17: call PhysicalCountInt(s_36_16)
        let s_36_17: u64 = PhysicalCountInt(state, tracer, s_36_16);
        // D s_36_18: cast zx s_36_15 -> bv
        let s_36_18: Bits = Bits::new(s_36_15 as u128, 64u16);
        // S s_36_19: cast zx s_36_17 -> bv
        let s_36_19: Bits = Bits::new(s_36_17 as u128, 64u16);
        // D s_36_20: add s_36_18 s_36_19
        let s_36_20: Bits = (s_36_18 + s_36_19);
        // D s_36_21: cast reint s_36_20 -> u64
        let s_36_21: u64 = (s_36_20.value() as u64);
        // D s_36_22: call Mk_CNTV_CVAL_EL0_Type(s_36_21)
        let s_36_22: ProductType5c790c8ef59cc8b2 = Mk_CNTV_CVAL_EL0_Type(
            state,
            tracer,
            s_36_21,
        );
        // C s_36_23: const #23632u : u32
        let s_36_23: u32 = 23632;
        // N s_36_24: write-reg s_36_23 <= s_36_22
        let s_36_24: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_36_23 as isize, s_36_22);
            tracer.write_register(s_36_23 as isize, s_36_22);
        };
        // N s_36_25: return
        return;
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #64s : i64
        let s_37_0: i64 = 64;
        // D s_37_1: read-var t:i
        let s_37_1: i128 = fn_state.t;
        // D s_37_2: call X_read(s_37_1, s_37_0)
        let s_37_2: Bits = X_read(state, tracer, s_37_1, s_37_0);
        // D s_37_3: cast reint s_37_2 -> u64
        let s_37_3: u64 = (s_37_2.value() as u64);
        // C s_37_4: const #0s : i
        let s_37_4: i128 = 0;
        // D s_37_5: cast zx s_37_3 -> bv
        let s_37_5: Bits = Bits::new(s_37_3 as u128, 64u16);
        // C s_37_6: const #1s : i64
        let s_37_6: i64 = 1;
        // C s_37_7: cast zx s_37_6 -> i
        let s_37_7: i128 = (i128::try_from(s_37_6).unwrap());
        // C s_37_8: const #31s : i
        let s_37_8: i128 = 31;
        // C s_37_9: add s_37_8 s_37_7
        let s_37_9: i128 = (s_37_8 + s_37_7);
        // D s_37_10: bit-extract s_37_5 s_37_4 s_37_9
        let s_37_10: Bits = (Bits::new(
            ((s_37_5) >> (s_37_4)).value(),
            u16::try_from(s_37_9).unwrap(),
        ));
        // D s_37_11: cast reint s_37_10 -> u32
        let s_37_11: u32 = (s_37_10.value() as u32);
        // C s_37_12: const #64s : i
        let s_37_12: i128 = 64;
        // D s_37_13: cast zx s_37_11 -> bv
        let s_37_13: Bits = Bits::new(s_37_11 as u128, 32u16);
        // D s_37_14: bits-cast sx s_37_13 -> bv length s_37_12
        let s_37_14: Bits = s_37_13.sign_extend(s_37_12);
        // D s_37_15: cast reint s_37_14 -> u64
        let s_37_15: u64 = (s_37_14.value() as u64);
        // C s_37_16: const #() : ()
        let s_37_16: () = ();
        // S s_37_17: call PhysicalCountInt(s_37_16)
        let s_37_17: u64 = PhysicalCountInt(state, tracer, s_37_16);
        // D s_37_18: cast zx s_37_15 -> bv
        let s_37_18: Bits = Bits::new(s_37_15 as u128, 64u16);
        // S s_37_19: cast zx s_37_17 -> bv
        let s_37_19: Bits = Bits::new(s_37_17 as u128, 64u16);
        // D s_37_20: add s_37_18 s_37_19
        let s_37_20: Bits = (s_37_18 + s_37_19);
        // D s_37_21: cast reint s_37_20 -> u64
        let s_37_21: u64 = (s_37_20.value() as u64);
        // D s_37_22: cast zx s_37_21 -> bv
        let s_37_22: Bits = Bits::new(s_37_21 as u128, 64u16);
        // C s_37_23: const #22400u : u32
        let s_37_23: u32 = 22400;
        // D s_37_24: read-reg s_37_23:u64
        let s_37_24: u64 = {
            let value = state.read_register::<u64>(s_37_23 as isize);
            tracer.read_register(s_37_23 as isize, value);
            value
        };
        // D s_37_25: cast zx s_37_24 -> bv
        let s_37_25: Bits = Bits::new(s_37_24 as u128, 64u16);
        // D s_37_26: sub s_37_22 s_37_25
        let s_37_26: Bits = ((s_37_22) - (s_37_25));
        // D s_37_27: cast reint s_37_26 -> u64
        let s_37_27: u64 = (s_37_26.value() as u64);
        // D s_37_28: call Mk_CNTV_CVAL_EL0_Type(s_37_27)
        let s_37_28: ProductType5c790c8ef59cc8b2 = Mk_CNTV_CVAL_EL0_Type(
            state,
            tracer,
            s_37_27,
        );
        // C s_37_29: const #23632u : u32
        let s_37_29: u32 = 23632;
        // N s_37_30: write-reg s_37_29 <= s_37_28
        let s_37_30: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_37_29 as isize, s_37_28);
            tracer.write_register(s_37_29 as isize, s_37_28);
        };
        // N s_37_31: return
        return;
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #24u : u8
        let s_38_0: u8 = 24;
        // C s_38_1: cast zx s_38_0 -> bv
        let s_38_1: Bits = Bits::new(s_38_0 as u128, 8u16);
        // C s_38_2: cast zx s_38_1 -> i
        let s_38_2: i128 = (s_38_1.value() as i128);
        // C s_38_3: cast reint s_38_2 -> i64
        let s_38_3: i64 = (s_38_2 as i64);
        // C s_38_4: cast zx s_38_3 -> i
        let s_38_4: i128 = (i128::try_from(s_38_3).unwrap());
        // C s_38_5: const #432u : u32
        let s_38_5: u32 = 432;
        // D s_38_6: read-reg s_38_5:u8
        let s_38_6: u8 = {
            let value = state.read_register::<u8>(s_38_5 as isize);
            tracer.read_register(s_38_5 as isize, value);
            value
        };
        // D s_38_7: call AArch64_SystemAccessTrap(s_38_6, s_38_4)
        let s_38_7: () = AArch64_SystemAccessTrap(state, tracer, s_38_6, s_38_4);
        // N s_38_8: return
        return;
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var __CNTHCTL_EL2_EL1TVT:u8
        let s_39_0: bool = fn_state.u__CNTHCTL_EL2_EL1TVT;
        // D s_39_1: cast zx s_39_0 -> bv
        let s_39_1: Bits = Bits::new(s_39_0 as u128, 1u16);
        // C s_39_2: const #1u : u8
        let s_39_2: bool = true;
        // C s_39_3: cast zx s_39_2 -> bv
        let s_39_3: Bits = Bits::new(s_39_2 as u128, 1u16);
        // D s_39_4: cmp-eq s_39_1 s_39_3
        let s_39_4: bool = ((s_39_1) == (s_39_3));
        // D s_39_5: write-var gs#81764 <= s_39_4
        fn_state.gs_81764 = s_39_4;
        // N s_39_6: jump b34
        return block_34(state, tracer, fn_state);
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
        // N s_40_2: branch s_40_1 b96 b41
        if s_40_1 {
            return block_96(state, tracer, fn_state);
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
        // D s_41_1: write-var gs#81773 <= s_41_0
        fn_state.gs_81773 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#81773:u8
        let s_42_0: bool = fn_state.gs_81773;
        // D s_42_1: not s_42_0
        let s_42_1: bool = !s_42_0;
        // N s_42_2: branch s_42_1 b95 b43
        if s_42_1 {
            return block_95(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #0u : u8
        let s_43_0: bool = false;
        // D s_43_1: write-var gs#81774 <= s_43_0
        fn_state.gs_81774 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#81774:u8
        let s_44_0: bool = fn_state.gs_81774;
        // N s_44_1: branch s_44_0 b89 b45
        if s_44_0 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #() : ()
        let s_45_0: () = ();
        // S s_45_1: call EL2Enabled(s_45_0)
        let s_45_1: bool = EL2Enabled(state, tracer, s_45_0);
        // N s_45_2: branch s_45_1 b88 b46
        if s_45_1 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #0u : u8
        let s_46_0: bool = false;
        // D s_46_1: write-var gs#81775 <= s_46_0
        fn_state.gs_81775 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#81775:u8
        let s_47_0: bool = fn_state.gs_81775;
        // N s_47_1: branch s_47_0 b87 b48
        if s_47_0 {
            return block_87(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #0u : u8
        let s_48_0: bool = false;
        // D s_48_1: write-var gs#81776 <= s_48_0
        fn_state.gs_81776 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#81776:u8
        let s_49_0: bool = fn_state.gs_81776;
        // N s_49_1: branch s_49_0 b86 b50
        if s_49_0 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #() : ()
        let s_50_0: () = ();
        // S s_50_1: call EL2Enabled(s_50_0)
        let s_50_1: bool = EL2Enabled(state, tracer, s_50_0);
        // N s_50_2: branch s_50_1 b85 b51
        if s_50_1 {
            return block_85(state, tracer, fn_state);
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
        // D s_51_1: write-var gs#81777 <= s_51_0
        fn_state.gs_81777 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#81777:u8
        let s_52_0: bool = fn_state.gs_81777;
        // N s_52_1: branch s_52_0 b84 b53
        if s_52_0 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #0u : u8
        let s_53_0: bool = false;
        // D s_53_1: write-var gs#81778 <= s_53_0
        fn_state.gs_81778 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#81778:u8
        let s_54_0: bool = fn_state.gs_81778;
        // N s_54_1: branch s_54_0 b83 b55
        if s_54_0 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #() : ()
        let s_55_0: () = ();
        // S s_55_1: call EL2Enabled(s_55_0)
        let s_55_1: bool = EL2Enabled(state, tracer, s_55_0);
        // N s_55_2: branch s_55_1 b82 b56
        if s_55_1 {
            return block_82(state, tracer, fn_state);
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
        // D s_56_1: write-var gs#81779 <= s_56_0
        fn_state.gs_81779 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#81779:u8
        let s_57_0: bool = fn_state.gs_81779;
        // N s_57_1: branch s_57_0 b81 b58
        if s_57_0 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #0u : u8
        let s_58_0: bool = false;
        // D s_58_1: write-var gs#81780 <= s_58_0
        fn_state.gs_81780 = s_58_0;
        // N s_58_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var gs#81780:u8
        let s_59_0: bool = fn_state.gs_81780;
        // N s_59_1: branch s_59_0 b80 b60
        if s_59_0 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #0u : u8
        let s_60_0: bool = false;
        // D s_60_1: write-var gs#81781 <= s_60_0
        fn_state.gs_81781 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#81781:u8
        let s_61_0: bool = fn_state.gs_81781;
        // N s_61_1: branch s_61_0 b79 b62
        if s_61_0 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #() : ()
        let s_62_0: () = ();
        // S s_62_1: call EL2Enabled(s_62_0)
        let s_62_1: bool = EL2Enabled(state, tracer, s_62_0);
        // N s_62_2: branch s_62_1 b78 b63
        if s_62_1 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #0u : u8
        let s_63_0: bool = false;
        // D s_63_1: write-var gs#81782 <= s_63_0
        fn_state.gs_81782 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#81782:u8
        let s_64_0: bool = fn_state.gs_81782;
        // N s_64_1: branch s_64_0 b77 b65
        if s_64_0 {
            return block_77(state, tracer, fn_state);
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
        // D s_65_1: write-var gs#81783 <= s_65_0
        fn_state.gs_81783 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#81783:u8
        let s_66_0: bool = fn_state.gs_81783;
        // N s_66_1: branch s_66_0 b76 b67
        if s_66_0 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #432u : u32
        let s_67_0: u32 = 432;
        // D s_67_1: read-reg s_67_0:u8
        let s_67_1: u8 = {
            let value = state.read_register::<u8>(s_67_0 as isize);
            tracer.read_register(s_67_0 as isize, value);
            value
        };
        // C s_67_2: const #2u : u8
        let s_67_2: u8 = 2;
        // D s_67_3: cmp-lt s_67_1 s_67_2
        let s_67_3: bool = ((s_67_1) < (s_67_2));
        // N s_67_4: branch s_67_3 b72 b68
        if s_67_3 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #0u : u8
        let s_68_0: bool = false;
        // D s_68_1: write-var gs#81785 <= s_68_0
        fn_state.gs_81785 = s_68_0;
        // N s_68_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var gs#81785:u8
        let s_69_0: bool = fn_state.gs_81785;
        // N s_69_1: branch s_69_0 b71 b70
        if s_69_0 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_70(state, tracer, fn_state);
        };
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #64s : i64
        let s_70_0: i64 = 64;
        // D s_70_1: read-var t:i
        let s_70_1: i128 = fn_state.t;
        // D s_70_2: call X_read(s_70_1, s_70_0)
        let s_70_2: Bits = X_read(state, tracer, s_70_1, s_70_0);
        // D s_70_3: cast reint s_70_2 -> u64
        let s_70_3: u64 = (s_70_2.value() as u64);
        // C s_70_4: const #0s : i
        let s_70_4: i128 = 0;
        // D s_70_5: cast zx s_70_3 -> bv
        let s_70_5: Bits = Bits::new(s_70_3 as u128, 64u16);
        // C s_70_6: const #1s : i64
        let s_70_6: i64 = 1;
        // C s_70_7: cast zx s_70_6 -> i
        let s_70_7: i128 = (i128::try_from(s_70_6).unwrap());
        // C s_70_8: const #31s : i
        let s_70_8: i128 = 31;
        // C s_70_9: add s_70_8 s_70_7
        let s_70_9: i128 = (s_70_8 + s_70_7);
        // D s_70_10: bit-extract s_70_5 s_70_4 s_70_9
        let s_70_10: Bits = (Bits::new(
            ((s_70_5) >> (s_70_4)).value(),
            u16::try_from(s_70_9).unwrap(),
        ));
        // D s_70_11: cast reint s_70_10 -> u32
        let s_70_11: u32 = (s_70_10.value() as u32);
        // C s_70_12: const #64s : i
        let s_70_12: i128 = 64;
        // D s_70_13: cast zx s_70_11 -> bv
        let s_70_13: Bits = Bits::new(s_70_11 as u128, 32u16);
        // D s_70_14: bits-cast sx s_70_13 -> bv length s_70_12
        let s_70_14: Bits = s_70_13.sign_extend(s_70_12);
        // D s_70_15: cast reint s_70_14 -> u64
        let s_70_15: u64 = (s_70_14.value() as u64);
        // C s_70_16: const #() : ()
        let s_70_16: () = ();
        // S s_70_17: call PhysicalCountInt(s_70_16)
        let s_70_17: u64 = PhysicalCountInt(state, tracer, s_70_16);
        // D s_70_18: cast zx s_70_15 -> bv
        let s_70_18: Bits = Bits::new(s_70_15 as u128, 64u16);
        // S s_70_19: cast zx s_70_17 -> bv
        let s_70_19: Bits = Bits::new(s_70_17 as u128, 64u16);
        // D s_70_20: add s_70_18 s_70_19
        let s_70_20: Bits = (s_70_18 + s_70_19);
        // D s_70_21: cast reint s_70_20 -> u64
        let s_70_21: u64 = (s_70_20.value() as u64);
        // D s_70_22: call Mk_CNTV_CVAL_EL0_Type(s_70_21)
        let s_70_22: ProductType5c790c8ef59cc8b2 = Mk_CNTV_CVAL_EL0_Type(
            state,
            tracer,
            s_70_21,
        );
        // C s_70_23: const #23632u : u32
        let s_70_23: u32 = 23632;
        // N s_70_24: write-reg s_70_23 <= s_70_22
        let s_70_24: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_70_23 as isize, s_70_22);
            tracer.write_register(s_70_23 as isize, s_70_22);
        };
        // N s_70_25: return
        return;
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #64s : i64
        let s_71_0: i64 = 64;
        // D s_71_1: read-var t:i
        let s_71_1: i128 = fn_state.t;
        // D s_71_2: call X_read(s_71_1, s_71_0)
        let s_71_2: Bits = X_read(state, tracer, s_71_1, s_71_0);
        // D s_71_3: cast reint s_71_2 -> u64
        let s_71_3: u64 = (s_71_2.value() as u64);
        // C s_71_4: const #0s : i
        let s_71_4: i128 = 0;
        // D s_71_5: cast zx s_71_3 -> bv
        let s_71_5: Bits = Bits::new(s_71_3 as u128, 64u16);
        // C s_71_6: const #1s : i64
        let s_71_6: i64 = 1;
        // C s_71_7: cast zx s_71_6 -> i
        let s_71_7: i128 = (i128::try_from(s_71_6).unwrap());
        // C s_71_8: const #31s : i
        let s_71_8: i128 = 31;
        // C s_71_9: add s_71_8 s_71_7
        let s_71_9: i128 = (s_71_8 + s_71_7);
        // D s_71_10: bit-extract s_71_5 s_71_4 s_71_9
        let s_71_10: Bits = (Bits::new(
            ((s_71_5) >> (s_71_4)).value(),
            u16::try_from(s_71_9).unwrap(),
        ));
        // D s_71_11: cast reint s_71_10 -> u32
        let s_71_11: u32 = (s_71_10.value() as u32);
        // C s_71_12: const #64s : i
        let s_71_12: i128 = 64;
        // D s_71_13: cast zx s_71_11 -> bv
        let s_71_13: Bits = Bits::new(s_71_11 as u128, 32u16);
        // D s_71_14: bits-cast sx s_71_13 -> bv length s_71_12
        let s_71_14: Bits = s_71_13.sign_extend(s_71_12);
        // D s_71_15: cast reint s_71_14 -> u64
        let s_71_15: u64 = (s_71_14.value() as u64);
        // C s_71_16: const #() : ()
        let s_71_16: () = ();
        // S s_71_17: call PhysicalCountInt(s_71_16)
        let s_71_17: u64 = PhysicalCountInt(state, tracer, s_71_16);
        // D s_71_18: cast zx s_71_15 -> bv
        let s_71_18: Bits = Bits::new(s_71_15 as u128, 64u16);
        // S s_71_19: cast zx s_71_17 -> bv
        let s_71_19: Bits = Bits::new(s_71_17 as u128, 64u16);
        // D s_71_20: add s_71_18 s_71_19
        let s_71_20: Bits = (s_71_18 + s_71_19);
        // D s_71_21: cast reint s_71_20 -> u64
        let s_71_21: u64 = (s_71_20.value() as u64);
        // D s_71_22: cast zx s_71_21 -> bv
        let s_71_22: Bits = Bits::new(s_71_21 as u128, 64u16);
        // C s_71_23: const #22400u : u32
        let s_71_23: u32 = 22400;
        // D s_71_24: read-reg s_71_23:u64
        let s_71_24: u64 = {
            let value = state.read_register::<u64>(s_71_23 as isize);
            tracer.read_register(s_71_23 as isize, value);
            value
        };
        // D s_71_25: cast zx s_71_24 -> bv
        let s_71_25: Bits = Bits::new(s_71_24 as u128, 64u16);
        // D s_71_26: sub s_71_22 s_71_25
        let s_71_26: Bits = ((s_71_22) - (s_71_25));
        // D s_71_27: cast reint s_71_26 -> u64
        let s_71_27: u64 = (s_71_26.value() as u64);
        // D s_71_28: call Mk_CNTV_CVAL_EL0_Type(s_71_27)
        let s_71_28: ProductType5c790c8ef59cc8b2 = Mk_CNTV_CVAL_EL0_Type(
            state,
            tracer,
            s_71_27,
        );
        // C s_71_29: const #23632u : u32
        let s_71_29: u32 = 23632;
        // N s_71_30: write-reg s_71_29 <= s_71_28
        let s_71_30: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_71_29 as isize, s_71_28);
            tracer.write_register(s_71_29 as isize, s_71_28);
        };
        // N s_71_31: return
        return;
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
        // S s_72_2: not s_72_1
        let s_72_2: bool = !s_72_1;
        // N s_72_3: branch s_72_2 b75 b73
        if s_72_2 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #102552u : u32
        let s_73_0: u32 = 102552;
        // D s_73_1: read-reg s_73_0:struct
        let s_73_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_73_0 as isize);
            tracer.read_register(s_73_0 as isize, value);
            value
        };
        // D s_73_2: call _get_HCR_EL2_Type_E2H(s_73_1)
        let s_73_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_73_1);
        // C s_73_3: const #102552u : u32
        let s_73_3: u32 = 102552;
        // D s_73_4: read-reg s_73_3:struct
        let s_73_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_73_3 as isize);
            tracer.read_register(s_73_3 as isize, value);
            value
        };
        // D s_73_5: call _get_HCR_EL2_Type_TGE(s_73_4)
        let s_73_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_73_4);
        // D s_73_6: cast zx s_73_2 -> bv
        let s_73_6: Bits = Bits::new(s_73_2 as u128, 1u16);
        // D s_73_7: cast zx s_73_5 -> bv
        let s_73_7: Bits = Bits::new(s_73_5 as u128, 1u16);
        // D s_73_8: cast reint s_73_6 -> u128
        let s_73_8: u128 = (s_73_6.value() as u128);
        // D s_73_9: size-of s_73_6
        let s_73_9: u16 = s_73_6.length();
        // D s_73_10: cast reint s_73_7 -> u128
        let s_73_10: u128 = (s_73_7.value() as u128);
        // D s_73_11: size-of s_73_7
        let s_73_11: u16 = s_73_7.length();
        // D s_73_12: lsl s_73_8 s_73_11
        let s_73_12: u128 = s_73_8 << s_73_11;
        // D s_73_13: or s_73_12 s_73_10
        let s_73_13: u128 = ((s_73_12) | (s_73_10));
        // D s_73_14: add s_73_9 s_73_11
        let s_73_14: u16 = (s_73_9 + s_73_11);
        // D s_73_15: create-bits s_73_13 s_73_14
        let s_73_15: Bits = Bits::new(s_73_13, s_73_14);
        // D s_73_16: cast reint s_73_15 -> u8
        let s_73_16: u8 = (s_73_15.value() as u8);
        // D s_73_17: cast zx s_73_16 -> bv
        let s_73_17: Bits = Bits::new(s_73_16 as u128, 2u16);
        // C s_73_18: const #3u : u8
        let s_73_18: u8 = 3;
        // C s_73_19: cast zx s_73_18 -> bv
        let s_73_19: Bits = Bits::new(s_73_18 as u128, 2u16);
        // D s_73_20: cmp-ne s_73_17 s_73_19
        let s_73_20: bool = ((s_73_17) != (s_73_19));
        // D s_73_21: write-var gs#81784 <= s_73_20
        fn_state.gs_81784 = s_73_20;
        // N s_73_22: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var gs#81784:u8
        let s_74_0: bool = fn_state.gs_81784;
        // D s_74_1: write-var gs#81785 <= s_74_0
        fn_state.gs_81785 = s_74_0;
        // N s_74_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #1u : u8
        let s_75_0: bool = true;
        // D s_75_1: write-var gs#81784 <= s_75_0
        fn_state.gs_81784 = s_75_0;
        // N s_75_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #64s : i64
        let s_76_0: i64 = 64;
        // D s_76_1: read-var t:i
        let s_76_1: i128 = fn_state.t;
        // D s_76_2: call X_read(s_76_1, s_76_0)
        let s_76_2: Bits = X_read(state, tracer, s_76_1, s_76_0);
        // D s_76_3: cast reint s_76_2 -> u64
        let s_76_3: u64 = (s_76_2.value() as u64);
        // C s_76_4: const #0s : i
        let s_76_4: i128 = 0;
        // D s_76_5: cast zx s_76_3 -> bv
        let s_76_5: Bits = Bits::new(s_76_3 as u128, 64u16);
        // C s_76_6: const #1s : i64
        let s_76_6: i64 = 1;
        // C s_76_7: cast zx s_76_6 -> i
        let s_76_7: i128 = (i128::try_from(s_76_6).unwrap());
        // C s_76_8: const #31s : i
        let s_76_8: i128 = 31;
        // C s_76_9: add s_76_8 s_76_7
        let s_76_9: i128 = (s_76_8 + s_76_7);
        // D s_76_10: bit-extract s_76_5 s_76_4 s_76_9
        let s_76_10: Bits = (Bits::new(
            ((s_76_5) >> (s_76_4)).value(),
            u16::try_from(s_76_9).unwrap(),
        ));
        // D s_76_11: cast reint s_76_10 -> u32
        let s_76_11: u32 = (s_76_10.value() as u32);
        // C s_76_12: const #64s : i
        let s_76_12: i128 = 64;
        // D s_76_13: cast zx s_76_11 -> bv
        let s_76_13: Bits = Bits::new(s_76_11 as u128, 32u16);
        // D s_76_14: bits-cast sx s_76_13 -> bv length s_76_12
        let s_76_14: Bits = s_76_13.sign_extend(s_76_12);
        // D s_76_15: cast reint s_76_14 -> u64
        let s_76_15: u64 = (s_76_14.value() as u64);
        // C s_76_16: const #() : ()
        let s_76_16: () = ();
        // S s_76_17: call PhysicalCountInt(s_76_16)
        let s_76_17: u64 = PhysicalCountInt(state, tracer, s_76_16);
        // D s_76_18: cast zx s_76_15 -> bv
        let s_76_18: Bits = Bits::new(s_76_15 as u128, 64u16);
        // S s_76_19: cast zx s_76_17 -> bv
        let s_76_19: Bits = Bits::new(s_76_17 as u128, 64u16);
        // D s_76_20: add s_76_18 s_76_19
        let s_76_20: Bits = (s_76_18 + s_76_19);
        // D s_76_21: cast reint s_76_20 -> u64
        let s_76_21: u64 = (s_76_20.value() as u64);
        // D s_76_22: call Mk_CNTHV_CVAL_EL2_Type(s_76_21)
        let s_76_22: ProductType5c790c8ef59cc8b2 = Mk_CNTHV_CVAL_EL2_Type(
            state,
            tracer,
            s_76_21,
        );
        // C s_76_23: const #103152u : u32
        let s_76_23: u32 = 103152;
        // N s_76_24: write-reg s_76_23 <= s_76_22
        let s_76_24: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_76_23 as isize, s_76_22);
            tracer.write_register(s_76_23 as isize, s_76_22);
        };
        // N s_76_25: return
        return;
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #90704u : u32
        let s_77_0: u32 = 90704;
        // D s_77_1: read-reg s_77_0:struct
        let s_77_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_77_0 as isize);
            tracer.read_register(s_77_0 as isize, value);
            value
        };
        // D s_77_2: call _get_SCR_EL3_Type_NS(s_77_1)
        let s_77_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_77_1);
        // D s_77_3: cast zx s_77_2 -> bv
        let s_77_3: Bits = Bits::new(s_77_2 as u128, 1u16);
        // C s_77_4: const #1u : u8
        let s_77_4: bool = true;
        // C s_77_5: cast zx s_77_4 -> bv
        let s_77_5: Bits = Bits::new(s_77_4 as u128, 1u16);
        // D s_77_6: cmp-eq s_77_3 s_77_5
        let s_77_6: bool = ((s_77_3) == (s_77_5));
        // D s_77_7: write-var gs#81783 <= s_77_6
        fn_state.gs_81783 = s_77_6;
        // N s_77_8: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #102552u : u32
        let s_78_0: u32 = 102552;
        // D s_78_1: read-reg s_78_0:struct
        let s_78_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_78_0 as isize);
            tracer.read_register(s_78_0 as isize, value);
            value
        };
        // D s_78_2: call _get_HCR_EL2_Type_E2H(s_78_1)
        let s_78_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_78_1);
        // C s_78_3: const #102552u : u32
        let s_78_3: u32 = 102552;
        // D s_78_4: read-reg s_78_3:struct
        let s_78_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_78_3 as isize);
            tracer.read_register(s_78_3 as isize, value);
            value
        };
        // D s_78_5: call _get_HCR_EL2_Type_TGE(s_78_4)
        let s_78_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_78_4);
        // D s_78_6: cast zx s_78_2 -> bv
        let s_78_6: Bits = Bits::new(s_78_2 as u128, 1u16);
        // D s_78_7: cast zx s_78_5 -> bv
        let s_78_7: Bits = Bits::new(s_78_5 as u128, 1u16);
        // D s_78_8: cast reint s_78_6 -> u128
        let s_78_8: u128 = (s_78_6.value() as u128);
        // D s_78_9: size-of s_78_6
        let s_78_9: u16 = s_78_6.length();
        // D s_78_10: cast reint s_78_7 -> u128
        let s_78_10: u128 = (s_78_7.value() as u128);
        // D s_78_11: size-of s_78_7
        let s_78_11: u16 = s_78_7.length();
        // D s_78_12: lsl s_78_8 s_78_11
        let s_78_12: u128 = s_78_8 << s_78_11;
        // D s_78_13: or s_78_12 s_78_10
        let s_78_13: u128 = ((s_78_12) | (s_78_10));
        // D s_78_14: add s_78_9 s_78_11
        let s_78_14: u16 = (s_78_9 + s_78_11);
        // D s_78_15: create-bits s_78_13 s_78_14
        let s_78_15: Bits = Bits::new(s_78_13, s_78_14);
        // D s_78_16: cast reint s_78_15 -> u8
        let s_78_16: u8 = (s_78_15.value() as u8);
        // D s_78_17: cast zx s_78_16 -> bv
        let s_78_17: Bits = Bits::new(s_78_16 as u128, 2u16);
        // C s_78_18: const #3u : u8
        let s_78_18: u8 = 3;
        // C s_78_19: cast zx s_78_18 -> bv
        let s_78_19: Bits = Bits::new(s_78_18 as u128, 2u16);
        // D s_78_20: cmp-eq s_78_17 s_78_19
        let s_78_20: bool = ((s_78_17) == (s_78_19));
        // D s_78_21: write-var gs#81782 <= s_78_20
        fn_state.gs_81782 = s_78_20;
        // N s_78_22: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #64s : i64
        let s_79_0: i64 = 64;
        // D s_79_1: read-var t:i
        let s_79_1: i128 = fn_state.t;
        // D s_79_2: call X_read(s_79_1, s_79_0)
        let s_79_2: Bits = X_read(state, tracer, s_79_1, s_79_0);
        // D s_79_3: cast reint s_79_2 -> u64
        let s_79_3: u64 = (s_79_2.value() as u64);
        // C s_79_4: const #0s : i
        let s_79_4: i128 = 0;
        // D s_79_5: cast zx s_79_3 -> bv
        let s_79_5: Bits = Bits::new(s_79_3 as u128, 64u16);
        // C s_79_6: const #1s : i64
        let s_79_6: i64 = 1;
        // C s_79_7: cast zx s_79_6 -> i
        let s_79_7: i128 = (i128::try_from(s_79_6).unwrap());
        // C s_79_8: const #31s : i
        let s_79_8: i128 = 31;
        // C s_79_9: add s_79_8 s_79_7
        let s_79_9: i128 = (s_79_8 + s_79_7);
        // D s_79_10: bit-extract s_79_5 s_79_4 s_79_9
        let s_79_10: Bits = (Bits::new(
            ((s_79_5) >> (s_79_4)).value(),
            u16::try_from(s_79_9).unwrap(),
        ));
        // D s_79_11: cast reint s_79_10 -> u32
        let s_79_11: u32 = (s_79_10.value() as u32);
        // C s_79_12: const #64s : i
        let s_79_12: i128 = 64;
        // D s_79_13: cast zx s_79_11 -> bv
        let s_79_13: Bits = Bits::new(s_79_11 as u128, 32u16);
        // D s_79_14: bits-cast sx s_79_13 -> bv length s_79_12
        let s_79_14: Bits = s_79_13.sign_extend(s_79_12);
        // D s_79_15: cast reint s_79_14 -> u64
        let s_79_15: u64 = (s_79_14.value() as u64);
        // C s_79_16: const #() : ()
        let s_79_16: () = ();
        // S s_79_17: call PhysicalCountInt(s_79_16)
        let s_79_17: u64 = PhysicalCountInt(state, tracer, s_79_16);
        // D s_79_18: cast zx s_79_15 -> bv
        let s_79_18: Bits = Bits::new(s_79_15 as u128, 64u16);
        // S s_79_19: cast zx s_79_17 -> bv
        let s_79_19: Bits = Bits::new(s_79_17 as u128, 64u16);
        // D s_79_20: add s_79_18 s_79_19
        let s_79_20: Bits = (s_79_18 + s_79_19);
        // D s_79_21: cast reint s_79_20 -> u64
        let s_79_21: u64 = (s_79_20.value() as u64);
        // D s_79_22: call Mk_CNTHVS_CVAL_EL2_Type(s_79_21)
        let s_79_22: ProductType5c790c8ef59cc8b2 = Mk_CNTHVS_CVAL_EL2_Type(
            state,
            tracer,
            s_79_21,
        );
        // C s_79_23: const #10064u : u32
        let s_79_23: u32 = 10064;
        // N s_79_24: write-reg s_79_23 <= s_79_22
        let s_79_24: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_79_23 as isize, s_79_22);
            tracer.write_register(s_79_23 as isize, s_79_22);
        };
        // N s_79_25: return
        return;
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #117u : u32
        let s_80_0: u32 = 117;
        // S s_80_1: call IsFeatureImplemented(s_80_0)
        let s_80_1: bool = IsFeatureImplemented(state, tracer, s_80_0);
        // D s_80_2: write-var gs#81781 <= s_80_1
        fn_state.gs_81781 = s_80_1;
        // N s_80_3: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #90704u : u32
        let s_81_0: u32 = 90704;
        // D s_81_1: read-reg s_81_0:struct
        let s_81_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_81_0 as isize);
            tracer.read_register(s_81_0 as isize, value);
            value
        };
        // D s_81_2: call _get_SCR_EL3_Type_NS(s_81_1)
        let s_81_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_81_1);
        // D s_81_3: cast zx s_81_2 -> bv
        let s_81_3: Bits = Bits::new(s_81_2 as u128, 1u16);
        // C s_81_4: const #0u : u8
        let s_81_4: bool = false;
        // C s_81_5: cast zx s_81_4 -> bv
        let s_81_5: Bits = Bits::new(s_81_4 as u128, 1u16);
        // D s_81_6: cmp-eq s_81_3 s_81_5
        let s_81_6: bool = ((s_81_3) == (s_81_5));
        // D s_81_7: write-var gs#81780 <= s_81_6
        fn_state.gs_81780 = s_81_6;
        // N s_81_8: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #102552u : u32
        let s_82_0: u32 = 102552;
        // D s_82_1: read-reg s_82_0:struct
        let s_82_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_0 as isize);
            tracer.read_register(s_82_0 as isize, value);
            value
        };
        // D s_82_2: call _get_HCR_EL2_Type_E2H(s_82_1)
        let s_82_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_82_1);
        // C s_82_3: const #102552u : u32
        let s_82_3: u32 = 102552;
        // D s_82_4: read-reg s_82_3:struct
        let s_82_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_3 as isize);
            tracer.read_register(s_82_3 as isize, value);
            value
        };
        // D s_82_5: call _get_HCR_EL2_Type_TGE(s_82_4)
        let s_82_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_82_4);
        // D s_82_6: cast zx s_82_2 -> bv
        let s_82_6: Bits = Bits::new(s_82_2 as u128, 1u16);
        // D s_82_7: cast zx s_82_5 -> bv
        let s_82_7: Bits = Bits::new(s_82_5 as u128, 1u16);
        // D s_82_8: cast reint s_82_6 -> u128
        let s_82_8: u128 = (s_82_6.value() as u128);
        // D s_82_9: size-of s_82_6
        let s_82_9: u16 = s_82_6.length();
        // D s_82_10: cast reint s_82_7 -> u128
        let s_82_10: u128 = (s_82_7.value() as u128);
        // D s_82_11: size-of s_82_7
        let s_82_11: u16 = s_82_7.length();
        // D s_82_12: lsl s_82_8 s_82_11
        let s_82_12: u128 = s_82_8 << s_82_11;
        // D s_82_13: or s_82_12 s_82_10
        let s_82_13: u128 = ((s_82_12) | (s_82_10));
        // D s_82_14: add s_82_9 s_82_11
        let s_82_14: u16 = (s_82_9 + s_82_11);
        // D s_82_15: create-bits s_82_13 s_82_14
        let s_82_15: Bits = Bits::new(s_82_13, s_82_14);
        // D s_82_16: cast reint s_82_15 -> u8
        let s_82_16: u8 = (s_82_15.value() as u8);
        // D s_82_17: cast zx s_82_16 -> bv
        let s_82_17: Bits = Bits::new(s_82_16 as u128, 2u16);
        // C s_82_18: const #3u : u8
        let s_82_18: u8 = 3;
        // C s_82_19: cast zx s_82_18 -> bv
        let s_82_19: Bits = Bits::new(s_82_18 as u128, 2u16);
        // D s_82_20: cmp-eq s_82_17 s_82_19
        let s_82_20: bool = ((s_82_17) == (s_82_19));
        // D s_82_21: write-var gs#81779 <= s_82_20
        fn_state.gs_81779 = s_82_20;
        // N s_82_22: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #24u : u8
        let s_83_0: u8 = 24;
        // C s_83_1: cast zx s_83_0 -> bv
        let s_83_1: Bits = Bits::new(s_83_0 as u128, 8u16);
        // C s_83_2: cast zx s_83_1 -> i
        let s_83_2: i128 = (s_83_1.value() as i128);
        // C s_83_3: cast reint s_83_2 -> i64
        let s_83_3: i64 = (s_83_2 as i64);
        // C s_83_4: cast zx s_83_3 -> i
        let s_83_4: i128 = (i128::try_from(s_83_3).unwrap());
        // C s_83_5: const #432u : u32
        let s_83_5: u32 = 432;
        // D s_83_6: read-reg s_83_5:u8
        let s_83_6: u8 = {
            let value = state.read_register::<u8>(s_83_5 as isize);
            tracer.read_register(s_83_5 as isize, value);
            value
        };
        // D s_83_7: call AArch64_SystemAccessTrap(s_83_6, s_83_4)
        let s_83_7: () = AArch64_SystemAccessTrap(state, tracer, s_83_6, s_83_4);
        // N s_83_8: return
        return;
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var __CNTHCTL_EL2_EL1TVT:u8
        let s_84_0: bool = fn_state.u__CNTHCTL_EL2_EL1TVT;
        // D s_84_1: cast zx s_84_0 -> bv
        let s_84_1: Bits = Bits::new(s_84_0 as u128, 1u16);
        // C s_84_2: const #1u : u8
        let s_84_2: bool = true;
        // C s_84_3: cast zx s_84_2 -> bv
        let s_84_3: Bits = Bits::new(s_84_2 as u128, 1u16);
        // D s_84_4: cmp-eq s_84_1 s_84_3
        let s_84_4: bool = ((s_84_1) == (s_84_3));
        // D s_84_5: write-var gs#81778 <= s_84_4
        fn_state.gs_81778 = s_84_4;
        // N s_84_6: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #102552u : u32
        let s_85_0: u32 = 102552;
        // D s_85_1: read-reg s_85_0:struct
        let s_85_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_85_0 as isize);
            tracer.read_register(s_85_0 as isize, value);
            value
        };
        // D s_85_2: call _get_HCR_EL2_Type_E2H(s_85_1)
        let s_85_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_85_1);
        // C s_85_3: const #102552u : u32
        let s_85_3: u32 = 102552;
        // D s_85_4: read-reg s_85_3:struct
        let s_85_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_85_3 as isize);
            tracer.read_register(s_85_3 as isize, value);
            value
        };
        // D s_85_5: call _get_HCR_EL2_Type_TGE(s_85_4)
        let s_85_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_85_4);
        // D s_85_6: cast zx s_85_2 -> bv
        let s_85_6: Bits = Bits::new(s_85_2 as u128, 1u16);
        // D s_85_7: cast zx s_85_5 -> bv
        let s_85_7: Bits = Bits::new(s_85_5 as u128, 1u16);
        // D s_85_8: cast reint s_85_6 -> u128
        let s_85_8: u128 = (s_85_6.value() as u128);
        // D s_85_9: size-of s_85_6
        let s_85_9: u16 = s_85_6.length();
        // D s_85_10: cast reint s_85_7 -> u128
        let s_85_10: u128 = (s_85_7.value() as u128);
        // D s_85_11: size-of s_85_7
        let s_85_11: u16 = s_85_7.length();
        // D s_85_12: lsl s_85_8 s_85_11
        let s_85_12: u128 = s_85_8 << s_85_11;
        // D s_85_13: or s_85_12 s_85_10
        let s_85_13: u128 = ((s_85_12) | (s_85_10));
        // D s_85_14: add s_85_9 s_85_11
        let s_85_14: u16 = (s_85_9 + s_85_11);
        // D s_85_15: create-bits s_85_13 s_85_14
        let s_85_15: Bits = Bits::new(s_85_13, s_85_14);
        // D s_85_16: cast reint s_85_15 -> u8
        let s_85_16: u8 = (s_85_15.value() as u8);
        // D s_85_17: cast zx s_85_16 -> bv
        let s_85_17: Bits = Bits::new(s_85_16 as u128, 2u16);
        // C s_85_18: const #3u : u8
        let s_85_18: u8 = 3;
        // C s_85_19: cast zx s_85_18 -> bv
        let s_85_19: Bits = Bits::new(s_85_18 as u128, 2u16);
        // D s_85_20: cmp-ne s_85_17 s_85_19
        let s_85_20: bool = ((s_85_17) != (s_85_19));
        // D s_85_21: write-var gs#81777 <= s_85_20
        fn_state.gs_81777 = s_85_20;
        // N s_85_22: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #24u : u8
        let s_86_0: u8 = 24;
        // C s_86_1: cast zx s_86_0 -> bv
        let s_86_1: Bits = Bits::new(s_86_0 as u128, 8u16);
        // C s_86_2: cast zx s_86_1 -> i
        let s_86_2: i128 = (s_86_1.value() as i128);
        // C s_86_3: cast reint s_86_2 -> i64
        let s_86_3: i64 = (s_86_2 as i64);
        // C s_86_4: cast zx s_86_3 -> i
        let s_86_4: i128 = (i128::try_from(s_86_3).unwrap());
        // C s_86_5: const #432u : u32
        let s_86_5: u32 = 432;
        // D s_86_6: read-reg s_86_5:u8
        let s_86_6: u8 = {
            let value = state.read_register::<u8>(s_86_5 as isize);
            tracer.read_register(s_86_5 as isize, value);
            value
        };
        // D s_86_7: call AArch64_SystemAccessTrap(s_86_6, s_86_4)
        let s_86_7: () = AArch64_SystemAccessTrap(state, tracer, s_86_6, s_86_4);
        // N s_86_8: return
        return;
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var __CNTHCTL_EL2_EL0VTEN:u8
        let s_87_0: bool = fn_state.u__CNTHCTL_EL2_EL0VTEN;
        // D s_87_1: cast zx s_87_0 -> bv
        let s_87_1: Bits = Bits::new(s_87_0 as u128, 1u16);
        // C s_87_2: const #0u : u8
        let s_87_2: bool = false;
        // C s_87_3: cast zx s_87_2 -> bv
        let s_87_3: Bits = Bits::new(s_87_2 as u128, 1u16);
        // D s_87_4: cmp-eq s_87_1 s_87_3
        let s_87_4: bool = ((s_87_1) == (s_87_3));
        // D s_87_5: write-var gs#81776 <= s_87_4
        fn_state.gs_81776 = s_87_4;
        // N s_87_6: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #102552u : u32
        let s_88_0: u32 = 102552;
        // D s_88_1: read-reg s_88_0:struct
        let s_88_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_88_0 as isize);
            tracer.read_register(s_88_0 as isize, value);
            value
        };
        // D s_88_2: call _get_HCR_EL2_Type_E2H(s_88_1)
        let s_88_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_88_1);
        // C s_88_3: const #102552u : u32
        let s_88_3: u32 = 102552;
        // D s_88_4: read-reg s_88_3:struct
        let s_88_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_88_3 as isize);
            tracer.read_register(s_88_3 as isize, value);
            value
        };
        // D s_88_5: call _get_HCR_EL2_Type_TGE(s_88_4)
        let s_88_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_88_4);
        // D s_88_6: cast zx s_88_2 -> bv
        let s_88_6: Bits = Bits::new(s_88_2 as u128, 1u16);
        // D s_88_7: cast zx s_88_5 -> bv
        let s_88_7: Bits = Bits::new(s_88_5 as u128, 1u16);
        // D s_88_8: cast reint s_88_6 -> u128
        let s_88_8: u128 = (s_88_6.value() as u128);
        // D s_88_9: size-of s_88_6
        let s_88_9: u16 = s_88_6.length();
        // D s_88_10: cast reint s_88_7 -> u128
        let s_88_10: u128 = (s_88_7.value() as u128);
        // D s_88_11: size-of s_88_7
        let s_88_11: u16 = s_88_7.length();
        // D s_88_12: lsl s_88_8 s_88_11
        let s_88_12: u128 = s_88_8 << s_88_11;
        // D s_88_13: or s_88_12 s_88_10
        let s_88_13: u128 = ((s_88_12) | (s_88_10));
        // D s_88_14: add s_88_9 s_88_11
        let s_88_14: u16 = (s_88_9 + s_88_11);
        // D s_88_15: create-bits s_88_13 s_88_14
        let s_88_15: Bits = Bits::new(s_88_13, s_88_14);
        // D s_88_16: cast reint s_88_15 -> u8
        let s_88_16: u8 = (s_88_15.value() as u8);
        // D s_88_17: cast zx s_88_16 -> bv
        let s_88_17: Bits = Bits::new(s_88_16 as u128, 2u16);
        // C s_88_18: const #3u : u8
        let s_88_18: u8 = 3;
        // C s_88_19: cast zx s_88_18 -> bv
        let s_88_19: Bits = Bits::new(s_88_18 as u128, 2u16);
        // D s_88_20: cmp-eq s_88_17 s_88_19
        let s_88_20: bool = ((s_88_17) == (s_88_19));
        // D s_88_21: write-var gs#81775 <= s_88_20
        fn_state.gs_81775 = s_88_20;
        // N s_88_22: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #() : ()
        let s_89_0: () = ();
        // S s_89_1: call EL2Enabled(s_89_0)
        let s_89_1: bool = EL2Enabled(state, tracer, s_89_0);
        // N s_89_2: branch s_89_1 b94 b90
        if s_89_1 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_90(state, tracer, fn_state);
        };
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #0u : u8
        let s_90_0: bool = false;
        // D s_90_1: write-var gs#81802 <= s_90_0
        fn_state.gs_81802 = s_90_0;
        // N s_90_2: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_91_0: read-var gs#81802:u8
        let s_91_0: bool = fn_state.gs_81802;
        // N s_91_1: branch s_91_0 b93 b92
        if s_91_0 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_92(state, tracer, fn_state);
        };
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #24u : u8
        let s_92_0: u8 = 24;
        // C s_92_1: cast zx s_92_0 -> bv
        let s_92_1: Bits = Bits::new(s_92_0 as u128, 8u16);
        // C s_92_2: cast zx s_92_1 -> i
        let s_92_2: i128 = (s_92_1.value() as i128);
        // C s_92_3: cast reint s_92_2 -> i64
        let s_92_3: i64 = (s_92_2 as i64);
        // C s_92_4: cast zx s_92_3 -> i
        let s_92_4: i128 = (i128::try_from(s_92_3).unwrap());
        // C s_92_5: const #440u : u32
        let s_92_5: u32 = 440;
        // D s_92_6: read-reg s_92_5:u8
        let s_92_6: u8 = {
            let value = state.read_register::<u8>(s_92_5 as isize);
            tracer.read_register(s_92_5 as isize, value);
            value
        };
        // D s_92_7: call AArch64_SystemAccessTrap(s_92_6, s_92_4)
        let s_92_7: () = AArch64_SystemAccessTrap(state, tracer, s_92_6, s_92_4);
        // N s_92_8: return
        return;
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #24u : u8
        let s_93_0: u8 = 24;
        // C s_93_1: cast zx s_93_0 -> bv
        let s_93_1: Bits = Bits::new(s_93_0 as u128, 8u16);
        // C s_93_2: cast zx s_93_1 -> i
        let s_93_2: i128 = (s_93_1.value() as i128);
        // C s_93_3: cast reint s_93_2 -> i64
        let s_93_3: i64 = (s_93_2 as i64);
        // C s_93_4: cast zx s_93_3 -> i
        let s_93_4: i128 = (i128::try_from(s_93_3).unwrap());
        // C s_93_5: const #432u : u32
        let s_93_5: u32 = 432;
        // D s_93_6: read-reg s_93_5:u8
        let s_93_6: u8 = {
            let value = state.read_register::<u8>(s_93_5 as isize);
            tracer.read_register(s_93_5 as isize, value);
            value
        };
        // D s_93_7: call AArch64_SystemAccessTrap(s_93_6, s_93_4)
        let s_93_7: () = AArch64_SystemAccessTrap(state, tracer, s_93_6, s_93_4);
        // N s_93_8: return
        return;
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var __HCR_EL2_TGE:u8
        let s_94_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_94_1: cast zx s_94_0 -> bv
        let s_94_1: Bits = Bits::new(s_94_0 as u128, 1u16);
        // C s_94_2: const #1u : u8
        let s_94_2: bool = true;
        // C s_94_3: cast zx s_94_2 -> bv
        let s_94_3: Bits = Bits::new(s_94_2 as u128, 1u16);
        // D s_94_4: cmp-eq s_94_1 s_94_3
        let s_94_4: bool = ((s_94_1) == (s_94_3));
        // D s_94_5: write-var gs#81802 <= s_94_4
        fn_state.gs_81802 = s_94_4;
        // N s_94_6: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_95_0: read-var __CNTKCTL_EL1_EL0VTEN:u8
        let s_95_0: bool = fn_state.u__CNTKCTL_EL1_EL0VTEN;
        // D s_95_1: cast zx s_95_0 -> bv
        let s_95_1: Bits = Bits::new(s_95_0 as u128, 1u16);
        // C s_95_2: const #0u : u8
        let s_95_2: bool = false;
        // C s_95_3: cast zx s_95_2 -> bv
        let s_95_3: Bits = Bits::new(s_95_2 as u128, 1u16);
        // D s_95_4: cmp-eq s_95_1 s_95_3
        let s_95_4: bool = ((s_95_1) == (s_95_3));
        // D s_95_5: write-var gs#81774 <= s_95_4
        fn_state.gs_81774 = s_95_4;
        // N s_95_6: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #102552u : u32
        let s_96_0: u32 = 102552;
        // D s_96_1: read-reg s_96_0:struct
        let s_96_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_96_0 as isize);
            tracer.read_register(s_96_0 as isize, value);
            value
        };
        // D s_96_2: call _get_HCR_EL2_Type_E2H(s_96_1)
        let s_96_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_96_1);
        // C s_96_3: const #102552u : u32
        let s_96_3: u32 = 102552;
        // D s_96_4: read-reg s_96_3:struct
        let s_96_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_96_3 as isize);
            tracer.read_register(s_96_3 as isize, value);
            value
        };
        // D s_96_5: call _get_HCR_EL2_Type_TGE(s_96_4)
        let s_96_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_96_4);
        // D s_96_6: cast zx s_96_2 -> bv
        let s_96_6: Bits = Bits::new(s_96_2 as u128, 1u16);
        // D s_96_7: cast zx s_96_5 -> bv
        let s_96_7: Bits = Bits::new(s_96_5 as u128, 1u16);
        // D s_96_8: cast reint s_96_6 -> u128
        let s_96_8: u128 = (s_96_6.value() as u128);
        // D s_96_9: size-of s_96_6
        let s_96_9: u16 = s_96_6.length();
        // D s_96_10: cast reint s_96_7 -> u128
        let s_96_10: u128 = (s_96_7.value() as u128);
        // D s_96_11: size-of s_96_7
        let s_96_11: u16 = s_96_7.length();
        // D s_96_12: lsl s_96_8 s_96_11
        let s_96_12: u128 = s_96_8 << s_96_11;
        // D s_96_13: or s_96_12 s_96_10
        let s_96_13: u128 = ((s_96_12) | (s_96_10));
        // D s_96_14: add s_96_9 s_96_11
        let s_96_14: u16 = (s_96_9 + s_96_11);
        // D s_96_15: create-bits s_96_13 s_96_14
        let s_96_15: Bits = Bits::new(s_96_13, s_96_14);
        // D s_96_16: cast reint s_96_15 -> u8
        let s_96_16: u8 = (s_96_15.value() as u8);
        // D s_96_17: cast zx s_96_16 -> bv
        let s_96_17: Bits = Bits::new(s_96_16 as u128, 2u16);
        // C s_96_18: const #3u : u8
        let s_96_18: u8 = 3;
        // C s_96_19: cast zx s_96_18 -> bv
        let s_96_19: Bits = Bits::new(s_96_18 as u128, 2u16);
        // D s_96_20: cmp-eq s_96_17 s_96_19
        let s_96_20: bool = ((s_96_17) == (s_96_19));
        // D s_96_21: write-var gs#81773 <= s_96_20
        fn_state.gs_81773 = s_96_20;
        // N s_96_22: jump b42
        return block_42(state, tracer, fn_state);
    }
}
