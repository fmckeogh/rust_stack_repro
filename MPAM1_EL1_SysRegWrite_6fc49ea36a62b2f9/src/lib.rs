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
use Mk_MPAM1_EL1_Type::*;
use NVMem_set::*;
use X_read::*;
use AArch64_SystemAccessTrap::*;
use u_get_HCR_EL2_Type_NV1::*;
use u__set_MPAM1_EL1::*;
use u_get_HCR_EL2_Type_NV::*;
use ELUsingAArch32::*;
use u_get_EDSCR_Type_SDD::*;
use EL2Enabled::*;
use EDSCR_read::*;
use u_get_MPAM3_EL3_Type_TRAPLOWER::*;
use common::*;
pub fn MPAM1_EL1_SysRegWrite_6fc49ea36a62b2f9<T: Tracer>(
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
        gs_86778: bool,
        u__EDSCR_SDD: bool,
        gs_86770: bool,
        gs_86771: bool,
        gs_86773: bool,
        u__PSTATE_EL: u8,
        gs_86776: bool,
        gs_86777: bool,
        u__HCR_EL2_NV: bool,
        u__MPAM3_EL3_TRAPLOWER: bool,
        gs_86775: bool,
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
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call MPAM3_EL3_read(s_0_7)
        let s_0_8: ProductType5c790c8ef59cc8b2 = MPAM3_EL3_read(state, tracer, s_0_7);
        // S s_0_9: call _get_MPAM3_EL3_Type_TRAPLOWER(s_0_8)
        let s_0_9: bool = u_get_MPAM3_EL3_Type_TRAPLOWER(state, tracer, s_0_8);
        // D s_0_10: write-var __MPAM3_EL3_TRAPLOWER <= s_0_9
        fn_state.u__MPAM3_EL3_TRAPLOWER = s_0_9;
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
        // C s_0_15: const #() : ()
        let s_0_15: () = ();
        // S s_0_16: call EDSCR_read(s_0_15)
        let s_0_16: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_0_15);
        // S s_0_17: call _get_EDSCR_Type_SDD(s_0_16)
        let s_0_17: bool = u_get_EDSCR_Type_SDD(state, tracer, s_0_16);
        // D s_0_18: write-var __EDSCR_SDD <= s_0_17
        fn_state.u__EDSCR_SDD = s_0_17;
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
        // N s_0_25: branch s_0_24 b43 b1
        if s_0_24 {
            return block_43(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b27 b2
        if s_1_5 {
            return block_27(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#86770 <= s_6_0
        fn_state.gs_86770 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#86770:u8
        let s_7_0: bool = fn_state.gs_86770;
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
        // D s_8_1: write-var gs#86771 <= s_8_0
        fn_state.gs_86771 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#86771:u8
        let s_9_0: bool = fn_state.gs_86771;
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
        // D s_11_4: call Mk_MPAM1_EL1_Type(s_11_3)
        let s_11_4: ProductType5c790c8ef59cc8b2 = Mk_MPAM1_EL1_Type(
            state,
            tracer,
            s_11_3,
        );
        // D s_11_5: call __set_MPAM1_EL1(s_11_4)
        let s_11_5: () = u__set_MPAM1_EL1(state, tracer, s_11_4);
        // N s_11_6: return
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
        // D s_12_5: write-var gs#86771 <= s_12_4
        fn_state.gs_86771 = s_12_4;
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
        // D s_13_4: write-var gs#86770 <= s_13_3
        fn_state.gs_86770 = s_13_3;
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
        // C s_16_0: const #424u : u32
        let s_16_0: u32 = 424;
        // D s_16_1: read-reg s_16_0:u8
        let s_16_1: u8 = {
            let value = state.read_register::<u8>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // C s_16_2: const #2u : u8
        let s_16_2: u8 = 2;
        // D s_16_3: cmp-lt s_16_1 s_16_2
        let s_16_3: bool = ((s_16_1) < (s_16_2));
        // N s_16_4: branch s_16_3 b26 b17
        if s_16_3 {
            return block_26(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#86773 <= s_17_0
        fn_state.gs_86773 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#86773:u8
        let s_18_0: bool = fn_state.gs_86773;
        // N s_18_1: branch s_18_0 b20 b19
        if s_18_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #64s : i64
        let s_19_0: i64 = 64;
        // D s_19_1: read-var t:i
        let s_19_1: i128 = fn_state.t;
        // D s_19_2: call X_read(s_19_1, s_19_0)
        let s_19_2: Bits = X_read(state, tracer, s_19_1, s_19_0);
        // D s_19_3: cast reint s_19_2 -> u64
        let s_19_3: u64 = (s_19_2.value() as u64);
        // D s_19_4: call Mk_MPAM1_EL1_Type(s_19_3)
        let s_19_4: ProductType5c790c8ef59cc8b2 = Mk_MPAM1_EL1_Type(
            state,
            tracer,
            s_19_3,
        );
        // D s_19_5: call __set_MPAM1_EL1(s_19_4)
        let s_19_5: () = u__set_MPAM1_EL1(state, tracer, s_19_4);
        // N s_19_6: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call Halted(s_20_0)
        let s_20_1: bool = Halted(state, tracer, s_20_0);
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
        // D s_21_1: write-var gs#86775 <= s_21_0
        fn_state.gs_86775 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#86775:u8
        let s_22_0: bool = fn_state.gs_86775;
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
        // C s_23_0: const #24u : u8
        let s_23_0: u8 = 24;
        // C s_23_1: cast zx s_23_0 -> bv
        let s_23_1: Bits = Bits::new(s_23_0 as u128, 8u16);
        // C s_23_2: cast zx s_23_1 -> i
        let s_23_2: i128 = (s_23_1.value() as i128);
        // C s_23_3: cast reint s_23_2 -> i64
        let s_23_3: i64 = (s_23_2 as i64);
        // C s_23_4: cast zx s_23_3 -> i
        let s_23_4: i128 = (i128::try_from(s_23_3).unwrap());
        // C s_23_5: const #424u : u32
        let s_23_5: u32 = 424;
        // D s_23_6: read-reg s_23_5:u8
        let s_23_6: u8 = {
            let value = state.read_register::<u8>(s_23_5 as isize);
            tracer.read_register(s_23_5 as isize, value);
            value
        };
        // D s_23_7: call AArch64_SystemAccessTrap(s_23_6, s_23_4)
        let s_23_7: () = AArch64_SystemAccessTrap(state, tracer, s_23_6, s_23_4);
        // N s_23_8: return
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
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var __EDSCR_SDD:u8
        let s_25_0: bool = fn_state.u__EDSCR_SDD;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 1u16);
        // C s_25_2: const #1u : u8
        let s_25_2: bool = true;
        // C s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // D s_25_4: cmp-eq s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) == (s_25_3));
        // D s_25_5: write-var gs#86775 <= s_25_4
        fn_state.gs_86775 = s_25_4;
        // N s_25_6: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var __MPAM3_EL3_TRAPLOWER:u8
        let s_26_0: bool = fn_state.u__MPAM3_EL3_TRAPLOWER;
        // D s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 1u16);
        // C s_26_2: const #1u : u8
        let s_26_2: bool = true;
        // C s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 1u16);
        // D s_26_4: cmp-eq s_26_1 s_26_3
        let s_26_4: bool = ((s_26_1) == (s_26_3));
        // D s_26_5: write-var gs#86773 <= s_26_4
        fn_state.gs_86773 = s_26_4;
        // N s_26_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #() : ()
        let s_27_0: () = ();
        // S s_27_1: call EL2Enabled(s_27_0)
        let s_27_1: bool = EL2Enabled(state, tracer, s_27_0);
        // N s_27_2: branch s_27_1 b42 b28
        if s_27_1 {
            return block_42(state, tracer, fn_state);
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
        // D s_28_1: write-var gs#86776 <= s_28_0
        fn_state.gs_86776 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#86776:u8
        let s_29_0: bool = fn_state.gs_86776;
        // N s_29_1: branch s_29_0 b41 b30
        if s_29_0 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #() : ()
        let s_30_0: () = ();
        // S s_30_1: call EL2Enabled(s_30_0)
        let s_30_1: bool = EL2Enabled(state, tracer, s_30_0);
        // N s_30_2: branch s_30_1 b40 b31
        if s_30_1 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #0u : u8
        let s_31_0: bool = false;
        // D s_31_1: write-var gs#86777 <= s_31_0
        fn_state.gs_86777 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#86777:u8
        let s_32_0: bool = fn_state.gs_86777;
        // N s_32_1: branch s_32_0 b34 b33
        if s_32_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_33_0: panic
        panic!("{:?}", ());
        // N s_33_1: return
        return;
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #424u : u32
        let s_34_0: u32 = 424;
        // D s_34_1: read-reg s_34_0:u8
        let s_34_1: u8 = {
            let value = state.read_register::<u8>(s_34_0 as isize);
            tracer.read_register(s_34_0 as isize, value);
            value
        };
        // C s_34_2: const #2u : u8
        let s_34_2: u8 = 2;
        // D s_34_3: cmp-lt s_34_1 s_34_2
        let s_34_3: bool = ((s_34_1) < (s_34_2));
        // N s_34_4: branch s_34_3 b39 b35
        if s_34_3 {
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
        // D s_35_1: write-var gs#86778 <= s_35_0
        fn_state.gs_86778 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#86778:u8
        let s_36_0: bool = fn_state.gs_86778;
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
        // C s_38_5: const #424u : u32
        let s_38_5: u32 = 424;
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
        // D s_39_0: read-var __MPAM3_EL3_TRAPLOWER:u8
        let s_39_0: bool = fn_state.u__MPAM3_EL3_TRAPLOWER;
        // D s_39_1: cast zx s_39_0 -> bv
        let s_39_1: Bits = Bits::new(s_39_0 as u128, 1u16);
        // C s_39_2: const #1u : u8
        let s_39_2: bool = true;
        // C s_39_3: cast zx s_39_2 -> bv
        let s_39_3: Bits = Bits::new(s_39_2 as u128, 1u16);
        // D s_39_4: cmp-eq s_39_1 s_39_3
        let s_39_4: bool = ((s_39_1) == (s_39_3));
        // D s_39_5: write-var gs#86778 <= s_39_4
        fn_state.gs_86778 = s_39_4;
        // N s_39_6: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var __HCR_EL2_NV:u8
        let s_40_0: bool = fn_state.u__HCR_EL2_NV;
        // D s_40_1: cast zx s_40_0 -> bv
        let s_40_1: Bits = Bits::new(s_40_0 as u128, 1u16);
        // C s_40_2: const #1u : u8
        let s_40_2: bool = true;
        // C s_40_3: cast zx s_40_2 -> bv
        let s_40_3: Bits = Bits::new(s_40_2 as u128, 1u16);
        // D s_40_4: cmp-eq s_40_1 s_40_3
        let s_40_4: bool = ((s_40_1) == (s_40_3));
        // D s_40_5: write-var gs#86777 <= s_40_4
        fn_state.gs_86777 = s_40_4;
        // N s_40_6: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #2304u : u12
        let s_41_0: u16 = 2304;
        // C s_41_1: cast zx s_41_0 -> bv
        let s_41_1: Bits = Bits::new(s_41_0 as u128, 12u16);
        // C s_41_2: cast zx s_41_1 -> i
        let s_41_2: i128 = (s_41_1.value() as i128);
        // C s_41_3: cast reint s_41_2 -> i64
        let s_41_3: i64 = (s_41_2 as i64);
        // C s_41_4: const #64s : i64
        let s_41_4: i64 = 64;
        // D s_41_5: read-var t:i
        let s_41_5: i128 = fn_state.t;
        // D s_41_6: call X_read(s_41_5, s_41_4)
        let s_41_6: Bits = X_read(state, tracer, s_41_5, s_41_4);
        // D s_41_7: cast reint s_41_6 -> u64
        let s_41_7: u64 = (s_41_6.value() as u64);
        // C s_41_8: cast zx s_41_3 -> i
        let s_41_8: i128 = (i128::try_from(s_41_3).unwrap());
        // D s_41_9: call NVMem_set(s_41_8, s_41_7)
        let s_41_9: () = NVMem_set(state, tracer, s_41_8, s_41_7);
        // N s_41_10: return
        return;
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #102552u : u32
        let s_42_0: u32 = 102552;
        // D s_42_1: read-reg s_42_0:struct
        let s_42_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_42_0 as isize);
            tracer.read_register(s_42_0 as isize, value);
            value
        };
        // D s_42_2: call _get_HCR_EL2_Type_NV2(s_42_1)
        let s_42_2: bool = u_get_HCR_EL2_Type_NV2(state, tracer, s_42_1);
        // C s_42_3: const #102552u : u32
        let s_42_3: u32 = 102552;
        // D s_42_4: read-reg s_42_3:struct
        let s_42_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_42_3 as isize);
            tracer.read_register(s_42_3 as isize, value);
            value
        };
        // D s_42_5: call _get_HCR_EL2_Type_NV1(s_42_4)
        let s_42_5: bool = u_get_HCR_EL2_Type_NV1(state, tracer, s_42_4);
        // C s_42_6: const #102552u : u32
        let s_42_6: u32 = 102552;
        // D s_42_7: read-reg s_42_6:struct
        let s_42_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_42_6 as isize);
            tracer.read_register(s_42_6 as isize, value);
            value
        };
        // D s_42_8: call _get_HCR_EL2_Type_NV(s_42_7)
        let s_42_8: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_42_7);
        // D s_42_9: cast zx s_42_5 -> bv
        let s_42_9: Bits = Bits::new(s_42_5 as u128, 1u16);
        // D s_42_10: cast zx s_42_8 -> bv
        let s_42_10: Bits = Bits::new(s_42_8 as u128, 1u16);
        // D s_42_11: cast reint s_42_9 -> u128
        let s_42_11: u128 = (s_42_9.value() as u128);
        // D s_42_12: size-of s_42_9
        let s_42_12: u16 = s_42_9.length();
        // D s_42_13: cast reint s_42_10 -> u128
        let s_42_13: u128 = (s_42_10.value() as u128);
        // D s_42_14: size-of s_42_10
        let s_42_14: u16 = s_42_10.length();
        // D s_42_15: lsl s_42_11 s_42_14
        let s_42_15: u128 = s_42_11 << s_42_14;
        // D s_42_16: or s_42_15 s_42_13
        let s_42_16: u128 = ((s_42_15) | (s_42_13));
        // D s_42_17: add s_42_12 s_42_14
        let s_42_17: u16 = (s_42_12 + s_42_14);
        // D s_42_18: create-bits s_42_16 s_42_17
        let s_42_18: Bits = Bits::new(s_42_16, s_42_17);
        // D s_42_19: cast reint s_42_18 -> u8
        let s_42_19: u8 = (s_42_18.value() as u8);
        // D s_42_20: cast zx s_42_2 -> bv
        let s_42_20: Bits = Bits::new(s_42_2 as u128, 1u16);
        // D s_42_21: cast zx s_42_19 -> bv
        let s_42_21: Bits = Bits::new(s_42_19 as u128, 2u16);
        // D s_42_22: cast reint s_42_20 -> u128
        let s_42_22: u128 = (s_42_20.value() as u128);
        // D s_42_23: size-of s_42_20
        let s_42_23: u16 = s_42_20.length();
        // D s_42_24: cast reint s_42_21 -> u128
        let s_42_24: u128 = (s_42_21.value() as u128);
        // D s_42_25: size-of s_42_21
        let s_42_25: u16 = s_42_21.length();
        // D s_42_26: lsl s_42_22 s_42_25
        let s_42_26: u128 = s_42_22 << s_42_25;
        // D s_42_27: or s_42_26 s_42_24
        let s_42_27: u128 = ((s_42_26) | (s_42_24));
        // D s_42_28: add s_42_23 s_42_25
        let s_42_28: u16 = (s_42_23 + s_42_25);
        // D s_42_29: create-bits s_42_27 s_42_28
        let s_42_29: Bits = Bits::new(s_42_27, s_42_28);
        // D s_42_30: cast reint s_42_29 -> u8
        let s_42_30: u8 = (s_42_29.value() as u8);
        // D s_42_31: cast zx s_42_30 -> bv
        let s_42_31: Bits = Bits::new(s_42_30 as u128, 3u16);
        // C s_42_32: const #5u : u8
        let s_42_32: u8 = 5;
        // C s_42_33: cast zx s_42_32 -> bv
        let s_42_33: Bits = Bits::new(s_42_32 as u128, 3u16);
        // D s_42_34: cmp-eq s_42_31 s_42_33
        let s_42_34: bool = ((s_42_31) == (s_42_33));
        // D s_42_35: write-var gs#86776 <= s_42_34
        fn_state.gs_86776 = s_42_34;
        // N s_42_36: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_43_0: panic
        panic!("{:?}", ());
        // N s_43_1: return
        return;
    }
}
