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
use u_get_HCR_EL2_Type_NV2::*;
use u_get_HCR_EL2_Type_E2H::*;
use MPAM3_EL3_read::*;
use Halted::*;
use NVMem_read::*;
use AArch64_SystemAccessTrap::*;
use MPAM1_EL1_read::*;
use u__get_MPAM1_EL1::*;
use u_get_HCR_EL2_Type_NV1::*;
use X_set::*;
use u_get_HCR_EL2_Type_NV::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_MPAM2_EL2_Type_TRAPMPAM1EL1::*;
use u__get_MPAM2_EL2::*;
use EL2Enabled::*;
use EDSCR_read::*;
use u_get_MPAM3_EL3_Type_TRAPLOWER::*;
use common::*;
pub fn MPAM2_EL2_SysRegRead_8458d45c0c29ff85<T: Tracer>(
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
        ga_71131: ProductType5c790c8ef59cc8b2,
        u__HCR_EL2_E2H: bool,
        gs_63320: bool,
        u__EDSCR_SDD: bool,
        gs_63319: bool,
        ga_71116: ProductType5c790c8ef59cc8b2,
        gs_63322: bool,
        u__MPAM2_EL2_TRAPMPAM1EL1: bool,
        ga_71127: ProductType5c790c8ef59cc8b2,
        ga_71114: u64,
        gs_63325: bool,
        gs_63316: bool,
        ga_71136: ProductType5c790c8ef59cc8b2,
        gs_63321: bool,
        u__PSTATE_EL: u8,
        u__MPAM3_EL3_TRAPLOWER: bool,
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
        // C s_0_3: const #() : ()
        let s_0_3: () = ();
        // S s_0_4: call MPAM3_EL3_read(s_0_3)
        let s_0_4: ProductType5c790c8ef59cc8b2 = MPAM3_EL3_read(state, tracer, s_0_3);
        // S s_0_5: call _get_MPAM3_EL3_Type_TRAPLOWER(s_0_4)
        let s_0_5: bool = u_get_MPAM3_EL3_Type_TRAPLOWER(state, tracer, s_0_4);
        // D s_0_6: write-var __MPAM3_EL3_TRAPLOWER <= s_0_5
        fn_state.u__MPAM3_EL3_TRAPLOWER = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call EDSCR_read(s_0_7)
        let s_0_8: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_0_7);
        // S s_0_9: call _get_EDSCR_Type_SDD(s_0_8)
        let s_0_9: bool = u_get_EDSCR_Type_SDD(state, tracer, s_0_8);
        // D s_0_10: write-var __EDSCR_SDD <= s_0_9
        fn_state.u__EDSCR_SDD = s_0_9;
        // C s_0_11: const #90504u : u32
        let s_0_11: u32 = 90504;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_MPAM2_EL2_Type_TRAPMPAM1EL1(s_0_12)
        let s_0_13: bool = u_get_MPAM2_EL2_Type_TRAPMPAM1EL1(state, tracer, s_0_12);
        // D s_0_14: write-var __MPAM2_EL2_TRAPMPAM1EL1 <= s_0_13
        fn_state.u__MPAM2_EL2_TRAPMPAM1EL1 = s_0_13;
        // C s_0_15: const #102552u : u32
        let s_0_15: u32 = 102552;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_HCR_EL2_Type_E2H(s_0_16)
        let s_0_17: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_0_16);
        // D s_0_18: write-var __HCR_EL2_E2H <= s_0_17
        fn_state.u__HCR_EL2_E2H = s_0_17;
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
        // N s_0_25: branch s_0_24 b41 b1
        if s_0_24 {
            return block_41(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b19 b2
        if s_1_5 {
            return block_19(state, tracer, fn_state);
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
        // C s_5_1: const #() : ()
        let s_5_1: () = ();
        // S s_5_2: call MPAM1_EL1_read(s_5_1)
        let s_5_2: ProductType5c790c8ef59cc8b2 = MPAM1_EL1_read(state, tracer, s_5_1);
        // S s_5_3: call __get_MPAM1_EL1(s_5_2)
        let s_5_3: ProductType5c790c8ef59cc8b2 = u__get_MPAM1_EL1(state, tracer, s_5_2);
        // D s_5_4: write-var ga#71136 <= s_5_3
        fn_state.ga_71136 = s_5_3;
        // D s_5_5: read-var ga#71136.0:struct
        let s_5_5: u64 = fn_state.ga_71136._0;
        // D s_5_6: cast zx s_5_5 -> bv
        let s_5_6: Bits = Bits::new(s_5_5 as u128, 64u16);
        // D s_5_7: read-var t:i
        let s_5_7: i128 = fn_state.t;
        // D s_5_8: call X_set(s_5_7, s_5_0, s_5_6)
        let s_5_8: () = X_set(state, tracer, s_5_7, s_5_0, s_5_6);
        // N s_5_9: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #424u : u32
        let s_6_0: u32 = 424;
        // D s_6_1: read-reg s_6_0:u8
        let s_6_1: u8 = {
            let value = state.read_register::<u8>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // C s_6_2: const #2u : u8
        let s_6_2: u8 = 2;
        // D s_6_3: cmp-lt s_6_1 s_6_2
        let s_6_3: bool = ((s_6_1) < (s_6_2));
        // N s_6_4: branch s_6_3 b18 b7
        if s_6_3 {
            return block_18(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#63316 <= s_7_0
        fn_state.gs_63316 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#63316:u8
        let s_8_0: bool = fn_state.gs_63316;
        // N s_8_1: branch s_8_0 b12 b9
        if s_8_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var __HCR_EL2_E2H:u8
        let s_9_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 1u16);
        // C s_9_2: const #1u : u8
        let s_9_2: bool = true;
        // C s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 1u16);
        // D s_9_4: cmp-eq s_9_1 s_9_3
        let s_9_4: bool = ((s_9_1) == (s_9_3));
        // N s_9_5: branch s_9_4 b11 b10
        if s_9_4 {
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
        // C s_10_0: const #64s : i64
        let s_10_0: i64 = 64;
        // C s_10_1: const #() : ()
        let s_10_1: () = ();
        // S s_10_2: call MPAM1_EL1_read(s_10_1)
        let s_10_2: ProductType5c790c8ef59cc8b2 = MPAM1_EL1_read(state, tracer, s_10_1);
        // S s_10_3: call __get_MPAM1_EL1(s_10_2)
        let s_10_3: ProductType5c790c8ef59cc8b2 = u__get_MPAM1_EL1(
            state,
            tracer,
            s_10_2,
        );
        // D s_10_4: write-var ga#71131 <= s_10_3
        fn_state.ga_71131 = s_10_3;
        // D s_10_5: read-var ga#71131.0:struct
        let s_10_5: u64 = fn_state.ga_71131._0;
        // D s_10_6: cast zx s_10_5 -> bv
        let s_10_6: Bits = Bits::new(s_10_5 as u128, 64u16);
        // D s_10_7: read-var t:i
        let s_10_7: i128 = fn_state.t;
        // D s_10_8: call X_set(s_10_7, s_10_0, s_10_6)
        let s_10_8: () = X_set(state, tracer, s_10_7, s_10_0, s_10_6);
        // N s_10_9: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #64s : i64
        let s_11_0: i64 = 64;
        // C s_11_1: const #90504u : u32
        let s_11_1: u32 = 90504;
        // D s_11_2: read-reg s_11_1:struct
        let s_11_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_11_1 as isize);
            tracer.read_register(s_11_1 as isize, value);
            value
        };
        // D s_11_3: call __get_MPAM2_EL2(s_11_2)
        let s_11_3: ProductType5c790c8ef59cc8b2 = u__get_MPAM2_EL2(
            state,
            tracer,
            s_11_2,
        );
        // D s_11_4: write-var ga#71127 <= s_11_3
        fn_state.ga_71127 = s_11_3;
        // D s_11_5: read-var ga#71127.0:struct
        let s_11_5: u64 = fn_state.ga_71127._0;
        // D s_11_6: cast zx s_11_5 -> bv
        let s_11_6: Bits = Bits::new(s_11_5 as u128, 64u16);
        // D s_11_7: read-var t:i
        let s_11_7: i128 = fn_state.t;
        // D s_11_8: call X_set(s_11_7, s_11_0, s_11_6)
        let s_11_8: () = X_set(state, tracer, s_11_7, s_11_0, s_11_6);
        // N s_11_9: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call Halted(s_12_0)
        let s_12_1: bool = Halted(state, tracer, s_12_0);
        // N s_12_2: branch s_12_1 b17 b13
        if s_12_1 {
            return block_17(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#63319 <= s_13_0
        fn_state.gs_63319 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#63319:u8
        let s_14_0: bool = fn_state.gs_63319;
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
        // C s_15_0: const #24u : u8
        let s_15_0: u8 = 24;
        // C s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 8u16);
        // C s_15_2: cast zx s_15_1 -> i
        let s_15_2: i128 = (s_15_1.value() as i128);
        // C s_15_3: cast reint s_15_2 -> i64
        let s_15_3: i64 = (s_15_2 as i64);
        // C s_15_4: cast zx s_15_3 -> i
        let s_15_4: i128 = (i128::try_from(s_15_3).unwrap());
        // C s_15_5: const #424u : u32
        let s_15_5: u32 = 424;
        // D s_15_6: read-reg s_15_5:u8
        let s_15_6: u8 = {
            let value = state.read_register::<u8>(s_15_5 as isize);
            tracer.read_register(s_15_5 as isize, value);
            value
        };
        // D s_15_7: call AArch64_SystemAccessTrap(s_15_6, s_15_4)
        let s_15_7: () = AArch64_SystemAccessTrap(state, tracer, s_15_6, s_15_4);
        // N s_15_8: return
        return;
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
        // D s_17_0: read-var __EDSCR_SDD:u8
        let s_17_0: bool = fn_state.u__EDSCR_SDD;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 1u16);
        // C s_17_2: const #1u : u8
        let s_17_2: bool = true;
        // C s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 1u16);
        // D s_17_4: cmp-eq s_17_1 s_17_3
        let s_17_4: bool = ((s_17_1) == (s_17_3));
        // D s_17_5: write-var gs#63319 <= s_17_4
        fn_state.gs_63319 = s_17_4;
        // N s_17_6: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var __MPAM3_EL3_TRAPLOWER:u8
        let s_18_0: bool = fn_state.u__MPAM3_EL3_TRAPLOWER;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 1u16);
        // C s_18_2: const #1u : u8
        let s_18_2: bool = true;
        // C s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 1u16);
        // D s_18_4: cmp-eq s_18_1 s_18_3
        let s_18_4: bool = ((s_18_1) == (s_18_3));
        // D s_18_5: write-var gs#63316 <= s_18_4
        fn_state.gs_63316 = s_18_4;
        // N s_18_6: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #424u : u32
        let s_19_0: u32 = 424;
        // D s_19_1: read-reg s_19_0:u8
        let s_19_1: u8 = {
            let value = state.read_register::<u8>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // C s_19_2: const #2u : u8
        let s_19_2: u8 = 2;
        // D s_19_3: cmp-lt s_19_1 s_19_2
        let s_19_3: bool = ((s_19_1) < (s_19_2));
        // N s_19_4: branch s_19_3 b40 b20
        if s_19_3 {
            return block_40(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#63320 <= s_20_0
        fn_state.gs_63320 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#63320:u8
        let s_21_0: bool = fn_state.gs_63320;
        // N s_21_1: branch s_21_0 b34 b22
        if s_21_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call EL2Enabled(s_22_0)
        let s_22_1: bool = EL2Enabled(state, tracer, s_22_0);
        // N s_22_2: branch s_22_1 b33 b23
        if s_22_1 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#63321 <= s_23_0
        fn_state.gs_63321 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#63321:u8
        let s_24_0: bool = fn_state.gs_63321;
        // N s_24_1: branch s_24_0 b32 b25
        if s_24_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call EL2Enabled(s_25_0)
        let s_25_1: bool = EL2Enabled(state, tracer, s_25_0);
        // N s_25_2: branch s_25_1 b31 b26
        if s_25_1 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // D s_26_1: write-var gs#63322 <= s_26_0
        fn_state.gs_63322 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#63322:u8
        let s_27_0: bool = fn_state.gs_63322;
        // N s_27_1: branch s_27_0 b29 b28
        if s_27_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #64s : i64
        let s_28_0: i64 = 64;
        // C s_28_1: const #() : ()
        let s_28_1: () = ();
        // S s_28_2: call MPAM1_EL1_read(s_28_1)
        let s_28_2: ProductType5c790c8ef59cc8b2 = MPAM1_EL1_read(state, tracer, s_28_1);
        // S s_28_3: call __get_MPAM1_EL1(s_28_2)
        let s_28_3: ProductType5c790c8ef59cc8b2 = u__get_MPAM1_EL1(
            state,
            tracer,
            s_28_2,
        );
        // D s_28_4: write-var ga#71116 <= s_28_3
        fn_state.ga_71116 = s_28_3;
        // D s_28_5: read-var ga#71116.0:struct
        let s_28_5: u64 = fn_state.ga_71116._0;
        // D s_28_6: cast zx s_28_5 -> bv
        let s_28_6: Bits = Bits::new(s_28_5 as u128, 64u16);
        // D s_28_7: read-var t:i
        let s_28_7: i128 = fn_state.t;
        // D s_28_8: call X_set(s_28_7, s_28_0, s_28_6)
        let s_28_8: () = X_set(state, tracer, s_28_7, s_28_0, s_28_6);
        // N s_28_9: return
        return;
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #2304u : u12
        let s_29_0: u16 = 2304;
        // C s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 12u16);
        // C s_29_2: cast zx s_29_1 -> i
        let s_29_2: i128 = (s_29_1.value() as i128);
        // C s_29_3: cast reint s_29_2 -> i64
        let s_29_3: i64 = (s_29_2 as i64);
        // C s_29_4: cast zx s_29_3 -> i
        let s_29_4: i128 = (i128::try_from(s_29_3).unwrap());
        // S s_29_5: call NVMem_read(s_29_4)
        let s_29_5: u64 = NVMem_read(state, tracer, s_29_4);
        // D s_29_6: write-var ga#71114 <= s_29_5
        fn_state.ga_71114 = s_29_5;
        // N s_29_7: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var ga#71114:u64
        let s_30_0: u64 = fn_state.ga_71114;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 64u16);
        // D s_30_2: read-var t:i
        let s_30_2: i128 = fn_state.t;
        // C s_30_3: const #64s : i64
        let s_30_3: i64 = 64;
        // D s_30_4: call X_set(s_30_2, s_30_3, s_30_1)
        let s_30_4: () = X_set(state, tracer, s_30_2, s_30_3, s_30_1);
        // N s_30_5: return
        return;
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #102552u : u32
        let s_31_0: u32 = 102552;
        // D s_31_1: read-reg s_31_0:struct
        let s_31_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_31_0 as isize);
            tracer.read_register(s_31_0 as isize, value);
            value
        };
        // D s_31_2: call _get_HCR_EL2_Type_NV2(s_31_1)
        let s_31_2: bool = u_get_HCR_EL2_Type_NV2(state, tracer, s_31_1);
        // C s_31_3: const #102552u : u32
        let s_31_3: u32 = 102552;
        // D s_31_4: read-reg s_31_3:struct
        let s_31_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_31_3 as isize);
            tracer.read_register(s_31_3 as isize, value);
            value
        };
        // D s_31_5: call _get_HCR_EL2_Type_NV1(s_31_4)
        let s_31_5: bool = u_get_HCR_EL2_Type_NV1(state, tracer, s_31_4);
        // C s_31_6: const #102552u : u32
        let s_31_6: u32 = 102552;
        // D s_31_7: read-reg s_31_6:struct
        let s_31_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_31_6 as isize);
            tracer.read_register(s_31_6 as isize, value);
            value
        };
        // D s_31_8: call _get_HCR_EL2_Type_NV(s_31_7)
        let s_31_8: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_31_7);
        // D s_31_9: cast zx s_31_5 -> bv
        let s_31_9: Bits = Bits::new(s_31_5 as u128, 1u16);
        // D s_31_10: cast zx s_31_8 -> bv
        let s_31_10: Bits = Bits::new(s_31_8 as u128, 1u16);
        // D s_31_11: cast reint s_31_9 -> u128
        let s_31_11: u128 = (s_31_9.value() as u128);
        // D s_31_12: size-of s_31_9
        let s_31_12: u16 = s_31_9.length();
        // D s_31_13: cast reint s_31_10 -> u128
        let s_31_13: u128 = (s_31_10.value() as u128);
        // D s_31_14: size-of s_31_10
        let s_31_14: u16 = s_31_10.length();
        // D s_31_15: lsl s_31_11 s_31_14
        let s_31_15: u128 = s_31_11 << s_31_14;
        // D s_31_16: or s_31_15 s_31_13
        let s_31_16: u128 = ((s_31_15) | (s_31_13));
        // D s_31_17: add s_31_12 s_31_14
        let s_31_17: u16 = (s_31_12 + s_31_14);
        // D s_31_18: create-bits s_31_16 s_31_17
        let s_31_18: Bits = Bits::new(s_31_16, s_31_17);
        // D s_31_19: cast reint s_31_18 -> u8
        let s_31_19: u8 = (s_31_18.value() as u8);
        // D s_31_20: cast zx s_31_2 -> bv
        let s_31_20: Bits = Bits::new(s_31_2 as u128, 1u16);
        // D s_31_21: cast zx s_31_19 -> bv
        let s_31_21: Bits = Bits::new(s_31_19 as u128, 2u16);
        // D s_31_22: cast reint s_31_20 -> u128
        let s_31_22: u128 = (s_31_20.value() as u128);
        // D s_31_23: size-of s_31_20
        let s_31_23: u16 = s_31_20.length();
        // D s_31_24: cast reint s_31_21 -> u128
        let s_31_24: u128 = (s_31_21.value() as u128);
        // D s_31_25: size-of s_31_21
        let s_31_25: u16 = s_31_21.length();
        // D s_31_26: lsl s_31_22 s_31_25
        let s_31_26: u128 = s_31_22 << s_31_25;
        // D s_31_27: or s_31_26 s_31_24
        let s_31_27: u128 = ((s_31_26) | (s_31_24));
        // D s_31_28: add s_31_23 s_31_25
        let s_31_28: u16 = (s_31_23 + s_31_25);
        // D s_31_29: create-bits s_31_27 s_31_28
        let s_31_29: Bits = Bits::new(s_31_27, s_31_28);
        // D s_31_30: cast reint s_31_29 -> u8
        let s_31_30: u8 = (s_31_29.value() as u8);
        // D s_31_31: cast zx s_31_30 -> bv
        let s_31_31: Bits = Bits::new(s_31_30 as u128, 3u16);
        // C s_31_32: const #7u : u8
        let s_31_32: u8 = 7;
        // C s_31_33: cast zx s_31_32 -> bv
        let s_31_33: Bits = Bits::new(s_31_32 as u128, 3u16);
        // D s_31_34: cmp-eq s_31_31 s_31_33
        let s_31_34: bool = ((s_31_31) == (s_31_33));
        // D s_31_35: write-var gs#63322 <= s_31_34
        fn_state.gs_63322 = s_31_34;
        // N s_31_36: jump b27
        return block_27(state, tracer, fn_state);
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
        // D s_33_0: read-var __MPAM2_EL2_TRAPMPAM1EL1:u8
        let s_33_0: bool = fn_state.u__MPAM2_EL2_TRAPMPAM1EL1;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 1u16);
        // C s_33_2: const #1u : u8
        let s_33_2: bool = true;
        // C s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 1u16);
        // D s_33_4: cmp-eq s_33_1 s_33_3
        let s_33_4: bool = ((s_33_1) == (s_33_3));
        // D s_33_5: write-var gs#63321 <= s_33_4
        fn_state.gs_63321 = s_33_4;
        // N s_33_6: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #() : ()
        let s_34_0: () = ();
        // S s_34_1: call Halted(s_34_0)
        let s_34_1: bool = Halted(state, tracer, s_34_0);
        // N s_34_2: branch s_34_1 b39 b35
        if s_34_1 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #0u : u8
        let s_35_0: bool = false;
        // D s_35_1: write-var gs#63325 <= s_35_0
        fn_state.gs_63325 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#63325:u8
        let s_36_0: bool = fn_state.gs_63325;
        // N s_36_1: branch s_36_0 b38 b37
        if s_36_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
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
        // C s_37_5: const #424u : u32
        let s_37_5: u32 = 424;
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
        // N s_38_0: panic
        panic!("{:?}", ());
        // N s_38_1: return
        return;
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var __EDSCR_SDD:u8
        let s_39_0: bool = fn_state.u__EDSCR_SDD;
        // D s_39_1: cast zx s_39_0 -> bv
        let s_39_1: Bits = Bits::new(s_39_0 as u128, 1u16);
        // C s_39_2: const #1u : u8
        let s_39_2: bool = true;
        // C s_39_3: cast zx s_39_2 -> bv
        let s_39_3: Bits = Bits::new(s_39_2 as u128, 1u16);
        // D s_39_4: cmp-eq s_39_1 s_39_3
        let s_39_4: bool = ((s_39_1) == (s_39_3));
        // D s_39_5: write-var gs#63325 <= s_39_4
        fn_state.gs_63325 = s_39_4;
        // N s_39_6: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var __MPAM3_EL3_TRAPLOWER:u8
        let s_40_0: bool = fn_state.u__MPAM3_EL3_TRAPLOWER;
        // D s_40_1: cast zx s_40_0 -> bv
        let s_40_1: Bits = Bits::new(s_40_0 as u128, 1u16);
        // C s_40_2: const #1u : u8
        let s_40_2: bool = true;
        // C s_40_3: cast zx s_40_2 -> bv
        let s_40_3: Bits = Bits::new(s_40_2 as u128, 1u16);
        // D s_40_4: cmp-eq s_40_1 s_40_3
        let s_40_4: bool = ((s_40_1) == (s_40_3));
        // D s_40_5: write-var gs#63320 <= s_40_4
        fn_state.gs_63320 = s_40_4;
        // N s_40_6: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_41_0: panic
        panic!("{:?}", ());
        // N s_41_1: return
        return;
    }
}
