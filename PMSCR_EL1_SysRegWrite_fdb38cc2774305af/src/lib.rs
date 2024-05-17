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
use Mk_PMSCR_EL1_Type::*;
use Halted::*;
use u_get_MDCR_EL3_Type_NSPB::*;
use X_read::*;
use AArch64_SystemAccessTrap::*;
use IsFeatureImplemented::*;
use u_get_SCR_EL3_Type_NS::*;
use u__IMPDEF_boolean::*;
use NVMem_set::*;
use u_get_HCR_EL2_Type_NV1::*;
use u_get_HCR_EL2_Type_NV::*;
use ELUsingAArch32::*;
use u_get_SCR_EL3_Type_NSE::*;
use u_get_MDCR_EL3_Type_NSPBE::*;
use u_get_EDSCR_Type_SDD::*;
use EL2Enabled::*;
use EDSCR_read::*;
use u_get_HCR_EL2_Type_NV2::*;
use common::*;
pub fn PMSCR_EL1_SysRegWrite_fdb38cc2774305af<T: Tracer>(
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
        gs_90788: bool,
        gs_90801: bool,
        u__HCR_EL2_E2H: bool,
        gs_90791: bool,
        gs_90789: bool,
        gs_90803: bool,
        u__MDCR_EL3_NSPBE: bool,
        gs_90802: bool,
        gs_90779: bool,
        gs_90796: bool,
        gs_90799: bool,
        gs_90783: bool,
        gs_90781: bool,
        gs_90790: bool,
        u__SCR_EL3_NSE: bool,
        gs_90778: bool,
        u__PSTATE_EL: u8,
        gs_90798: bool,
        u__HCR_EL2_NV: bool,
        gs_90797: bool,
        gs_90782: bool,
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
        // C s_0_11: const #22712u : u32
        let s_0_11: u32 = 22712;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_MDCR_EL3_Type_NSPBE(s_0_12)
        let s_0_13: bool = u_get_MDCR_EL3_Type_NSPBE(state, tracer, s_0_12);
        // D s_0_14: write-var __MDCR_EL3_NSPBE <= s_0_13
        fn_state.u__MDCR_EL3_NSPBE = s_0_13;
        // C s_0_15: const #90704u : u32
        let s_0_15: u32 = 90704;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_SCR_EL3_Type_NSE(s_0_16)
        let s_0_17: bool = u_get_SCR_EL3_Type_NSE(state, tracer, s_0_16);
        // D s_0_18: write-var __SCR_EL3_NSE <= s_0_17
        fn_state.u__SCR_EL3_NSE = s_0_17;
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
        // N s_0_25: branch s_0_24 b70 b1
        if s_0_24 {
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
        // N s_1_6: branch s_1_5 b59 b2
        if s_1_5 {
            return block_59(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#90778 <= s_6_0
        fn_state.gs_90778 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#90778:u8
        let s_7_0: bool = fn_state.gs_90778;
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
        // D s_8_1: write-var gs#90779 <= s_8_0
        fn_state.gs_90779 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#90779:u8
        let s_9_0: bool = fn_state.gs_90779;
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
        // D s_11_4: call Mk_PMSCR_EL1_Type(s_11_3)
        let s_11_4: ProductType5c790c8ef59cc8b2 = Mk_PMSCR_EL1_Type(
            state,
            tracer,
            s_11_3,
        );
        // C s_11_5: const #21072u : u32
        let s_11_5: u32 = 21072;
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
        // D s_12_5: write-var gs#90779 <= s_12_4
        fn_state.gs_90779 = s_12_4;
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
        // D s_13_4: write-var gs#90778 <= s_13_3
        fn_state.gs_90778 = s_13_3;
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
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call Halted(s_16_0)
        let s_16_1: bool = Halted(state, tracer, s_16_0);
        // N s_16_2: branch s_16_1 b58 b17
        if s_16_1 {
            return block_58(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#90781 <= s_17_0
        fn_state.gs_90781 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#90781:u8
        let s_18_0: bool = fn_state.gs_90781;
        // N s_18_1: branch s_18_0 b57 b19
        if s_18_0 {
            return block_57(state, tracer, fn_state);
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
        // D s_19_1: write-var gs#90782 <= s_19_0
        fn_state.gs_90782 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#90782:u8
        let s_20_0: bool = fn_state.gs_90782;
        // N s_20_1: branch s_20_0 b56 b21
        if s_20_0 {
            return block_56(state, tracer, fn_state);
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
        // D s_21_1: write-var gs#90783 <= s_21_0
        fn_state.gs_90783 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#90783:u8
        let s_22_0: bool = fn_state.gs_90783;
        // N s_22_1: branch s_22_0 b46 b23
        if s_22_0 {
            return block_46(state, tracer, fn_state);
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
        // D s_23_1: write-var gs#90791 <= s_23_0
        fn_state.gs_90791 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#90791:u8
        let s_24_0: bool = fn_state.gs_90791;
        // N s_24_1: branch s_24_0 b45 b25
        if s_24_0 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #424u : u32
        let s_25_0: u32 = 424;
        // D s_25_1: read-reg s_25_0:u8
        let s_25_1: u8 = {
            let value = state.read_register::<u8>(s_25_0 as isize);
            tracer.read_register(s_25_0 as isize, value);
            value
        };
        // C s_25_2: const #2u : u8
        let s_25_2: u8 = 2;
        // D s_25_3: cmp-lt s_25_1 s_25_2
        let s_25_3: bool = ((s_25_1) < (s_25_2));
        // N s_25_4: branch s_25_3 b35 b26
        if s_25_3 {
            return block_35(state, tracer, fn_state);
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
        // D s_26_1: write-var gs#90799 <= s_26_0
        fn_state.gs_90799 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#90799:u8
        let s_27_0: bool = fn_state.gs_90799;
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
        // D s_28_1: read-var t:i
        let s_28_1: i128 = fn_state.t;
        // D s_28_2: call X_read(s_28_1, s_28_0)
        let s_28_2: Bits = X_read(state, tracer, s_28_1, s_28_0);
        // D s_28_3: cast reint s_28_2 -> u64
        let s_28_3: u64 = (s_28_2.value() as u64);
        // D s_28_4: call Mk_PMSCR_EL1_Type(s_28_3)
        let s_28_4: ProductType5c790c8ef59cc8b2 = Mk_PMSCR_EL1_Type(
            state,
            tracer,
            s_28_3,
        );
        // C s_28_5: const #21072u : u32
        let s_28_5: u32 = 21072;
        // N s_28_6: write-reg s_28_5 <= s_28_4
        let s_28_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_28_5 as isize, s_28_4);
            tracer.write_register(s_28_5 as isize, s_28_4);
        };
        // N s_28_7: return
        return;
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #() : ()
        let s_29_0: () = ();
        // S s_29_1: call Halted(s_29_0)
        let s_29_1: bool = Halted(state, tracer, s_29_0);
        // N s_29_2: branch s_29_1 b34 b30
        if s_29_1 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #0u : u8
        let s_30_0: bool = false;
        // D s_30_1: write-var gs#90801 <= s_30_0
        fn_state.gs_90801 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#90801:u8
        let s_31_0: bool = fn_state.gs_90801;
        // N s_31_1: branch s_31_0 b33 b32
        if s_31_0 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
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
        // C s_32_5: const #424u : u32
        let s_32_5: u32 = 424;
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
        // C s_34_0: const #() : ()
        let s_34_0: () = ();
        // S s_34_1: call EDSCR_read(s_34_0)
        let s_34_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_34_0);
        // S s_34_2: call _get_EDSCR_Type_SDD(s_34_1)
        let s_34_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_34_1);
        // S s_34_3: cast zx s_34_2 -> bv
        let s_34_3: Bits = Bits::new(s_34_2 as u128, 1u16);
        // C s_34_4: const #1u : u8
        let s_34_4: bool = true;
        // C s_34_5: cast zx s_34_4 -> bv
        let s_34_5: Bits = Bits::new(s_34_4 as u128, 1u16);
        // S s_34_6: cmp-eq s_34_3 s_34_5
        let s_34_6: bool = ((s_34_3) == (s_34_5));
        // D s_34_7: write-var gs#90801 <= s_34_6
        fn_state.gs_90801 = s_34_6;
        // N s_34_8: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #22712u : u32
        let s_35_0: u32 = 22712;
        // D s_35_1: read-reg s_35_0:struct
        let s_35_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_35_0 as isize);
            tracer.read_register(s_35_0 as isize, value);
            value
        };
        // D s_35_2: call _get_MDCR_EL3_Type_NSPB(s_35_1)
        let s_35_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_35_1);
        // C s_35_3: const #0s : i
        let s_35_3: i128 = 0;
        // D s_35_4: cast zx s_35_2 -> bv
        let s_35_4: Bits = Bits::new(s_35_2 as u128, 2u16);
        // C s_35_5: const #1u : u64
        let s_35_5: u64 = 1;
        // D s_35_6: bit-extract s_35_4 s_35_3 s_35_5
        let s_35_6: Bits = (Bits::new(
            ((s_35_4) >> (s_35_3)).value(),
            u16::try_from(s_35_5).unwrap(),
        ));
        // D s_35_7: cast reint s_35_6 -> u8
        let s_35_7: bool = ((s_35_6.value()) != 0);
        // C s_35_8: const #0s : i
        let s_35_8: i128 = 0;
        // C s_35_9: const #0u : u64
        let s_35_9: u64 = 0;
        // D s_35_10: cast zx s_35_7 -> u64
        let s_35_10: u64 = (s_35_7 as u64);
        // C s_35_11: const #1u : u64
        let s_35_11: u64 = 1;
        // D s_35_12: and s_35_10 s_35_11
        let s_35_12: u64 = ((s_35_10) & (s_35_11));
        // D s_35_13: cmp-eq s_35_12 s_35_11
        let s_35_13: bool = ((s_35_12) == (s_35_11));
        // D s_35_14: lsl s_35_10 s_35_8
        let s_35_14: u64 = s_35_10 << s_35_8;
        // D s_35_15: or s_35_9 s_35_14
        let s_35_15: u64 = ((s_35_9) | (s_35_14));
        // D s_35_16: cmpl s_35_14
        let s_35_16: u64 = !s_35_14;
        // D s_35_17: and s_35_9 s_35_16
        let s_35_17: u64 = ((s_35_9) & (s_35_16));
        // D s_35_18: select s_35_13 s_35_15 s_35_17
        let s_35_18: u64 = if s_35_13 { s_35_15 } else { s_35_17 };
        // D s_35_19: cast trunc s_35_18 -> u8
        let s_35_19: bool = ((s_35_18) != 0);
        // D s_35_20: cast zx s_35_19 -> bv
        let s_35_20: Bits = Bits::new(s_35_19 as u128, 1u16);
        // C s_35_21: const #0u : u8
        let s_35_21: bool = false;
        // C s_35_22: cast zx s_35_21 -> bv
        let s_35_22: Bits = Bits::new(s_35_21 as u128, 1u16);
        // D s_35_23: cmp-eq s_35_20 s_35_22
        let s_35_23: bool = ((s_35_20) == (s_35_22));
        // N s_35_24: branch s_35_23 b44 b36
        if s_35_23 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #22712u : u32
        let s_36_0: u32 = 22712;
        // D s_36_1: read-reg s_36_0:struct
        let s_36_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_36_0 as isize);
            tracer.read_register(s_36_0 as isize, value);
            value
        };
        // D s_36_2: call _get_MDCR_EL3_Type_NSPB(s_36_1)
        let s_36_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_36_1);
        // C s_36_3: const #1s : i
        let s_36_3: i128 = 1;
        // D s_36_4: cast zx s_36_2 -> bv
        let s_36_4: Bits = Bits::new(s_36_2 as u128, 2u16);
        // C s_36_5: const #1u : u64
        let s_36_5: u64 = 1;
        // D s_36_6: bit-extract s_36_4 s_36_3 s_36_5
        let s_36_6: Bits = (Bits::new(
            ((s_36_4) >> (s_36_3)).value(),
            u16::try_from(s_36_5).unwrap(),
        ));
        // D s_36_7: cast reint s_36_6 -> u8
        let s_36_7: bool = ((s_36_6.value()) != 0);
        // C s_36_8: const #0s : i
        let s_36_8: i128 = 0;
        // C s_36_9: const #0u : u64
        let s_36_9: u64 = 0;
        // D s_36_10: cast zx s_36_7 -> u64
        let s_36_10: u64 = (s_36_7 as u64);
        // C s_36_11: const #1u : u64
        let s_36_11: u64 = 1;
        // D s_36_12: and s_36_10 s_36_11
        let s_36_12: u64 = ((s_36_10) & (s_36_11));
        // D s_36_13: cmp-eq s_36_12 s_36_11
        let s_36_13: bool = ((s_36_12) == (s_36_11));
        // D s_36_14: lsl s_36_10 s_36_8
        let s_36_14: u64 = s_36_10 << s_36_8;
        // D s_36_15: or s_36_9 s_36_14
        let s_36_15: u64 = ((s_36_9) | (s_36_14));
        // D s_36_16: cmpl s_36_14
        let s_36_16: u64 = !s_36_14;
        // D s_36_17: and s_36_9 s_36_16
        let s_36_17: u64 = ((s_36_9) & (s_36_16));
        // D s_36_18: select s_36_13 s_36_15 s_36_17
        let s_36_18: u64 = if s_36_13 { s_36_15 } else { s_36_17 };
        // D s_36_19: cast trunc s_36_18 -> u8
        let s_36_19: bool = ((s_36_18) != 0);
        // C s_36_20: const #90704u : u32
        let s_36_20: u32 = 90704;
        // D s_36_21: read-reg s_36_20:struct
        let s_36_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_36_20 as isize);
            tracer.read_register(s_36_20 as isize, value);
            value
        };
        // D s_36_22: call _get_SCR_EL3_Type_NS(s_36_21)
        let s_36_22: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_36_21);
        // D s_36_23: cast zx s_36_19 -> bv
        let s_36_23: Bits = Bits::new(s_36_19 as u128, 1u16);
        // D s_36_24: cast zx s_36_22 -> bv
        let s_36_24: Bits = Bits::new(s_36_22 as u128, 1u16);
        // D s_36_25: cmp-ne s_36_23 s_36_24
        let s_36_25: bool = ((s_36_23) != (s_36_24));
        // D s_36_26: write-var gs#90796 <= s_36_25
        fn_state.gs_90796 = s_36_25;
        // N s_36_27: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#90796:u8
        let s_37_0: bool = fn_state.gs_90796;
        // N s_37_1: branch s_37_0 b43 b38
        if s_37_0 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #232u : u32
        let s_38_0: u32 = 232;
        // S s_38_1: call IsFeatureImplemented(s_38_0)
        let s_38_1: bool = IsFeatureImplemented(state, tracer, s_38_0);
        // N s_38_2: branch s_38_1 b42 b39
        if s_38_1 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #0u : u8
        let s_39_0: bool = false;
        // D s_39_1: write-var gs#90797 <= s_39_0
        fn_state.gs_90797 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#90797:u8
        let s_40_0: bool = fn_state.gs_90797;
        // D s_40_1: write-var gs#90798 <= s_40_0
        fn_state.gs_90798 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#90798:u8
        let s_41_0: bool = fn_state.gs_90798;
        // D s_41_1: write-var gs#90799 <= s_41_0
        fn_state.gs_90799 = s_41_0;
        // N s_41_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var __MDCR_EL3_NSPBE:u8
        let s_42_0: bool = fn_state.u__MDCR_EL3_NSPBE;
        // D s_42_1: cast zx s_42_0 -> bv
        let s_42_1: Bits = Bits::new(s_42_0 as u128, 1u16);
        // D s_42_2: read-var __SCR_EL3_NSE:u8
        let s_42_2: bool = fn_state.u__SCR_EL3_NSE;
        // D s_42_3: cast zx s_42_2 -> bv
        let s_42_3: Bits = Bits::new(s_42_2 as u128, 1u16);
        // D s_42_4: cmp-ne s_42_1 s_42_3
        let s_42_4: bool = ((s_42_1) != (s_42_3));
        // D s_42_5: write-var gs#90797 <= s_42_4
        fn_state.gs_90797 = s_42_4;
        // N s_42_6: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #1u : u8
        let s_43_0: bool = true;
        // D s_43_1: write-var gs#90798 <= s_43_0
        fn_state.gs_90798 = s_43_0;
        // N s_43_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #1u : u8
        let s_44_0: bool = true;
        // D s_44_1: write-var gs#90796 <= s_44_0
        fn_state.gs_90796 = s_44_0;
        // N s_44_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_45_0: panic
        panic!("{:?}", ());
        // N s_45_1: return
        return;
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #22712u : u32
        let s_46_0: u32 = 22712;
        // D s_46_1: read-reg s_46_0:struct
        let s_46_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_46_0 as isize);
            tracer.read_register(s_46_0 as isize, value);
            value
        };
        // D s_46_2: call _get_MDCR_EL3_Type_NSPB(s_46_1)
        let s_46_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_46_1);
        // C s_46_3: const #0s : i
        let s_46_3: i128 = 0;
        // D s_46_4: cast zx s_46_2 -> bv
        let s_46_4: Bits = Bits::new(s_46_2 as u128, 2u16);
        // C s_46_5: const #1u : u64
        let s_46_5: u64 = 1;
        // D s_46_6: bit-extract s_46_4 s_46_3 s_46_5
        let s_46_6: Bits = (Bits::new(
            ((s_46_4) >> (s_46_3)).value(),
            u16::try_from(s_46_5).unwrap(),
        ));
        // D s_46_7: cast reint s_46_6 -> u8
        let s_46_7: bool = ((s_46_6.value()) != 0);
        // C s_46_8: const #0s : i
        let s_46_8: i128 = 0;
        // C s_46_9: const #0u : u64
        let s_46_9: u64 = 0;
        // D s_46_10: cast zx s_46_7 -> u64
        let s_46_10: u64 = (s_46_7 as u64);
        // C s_46_11: const #1u : u64
        let s_46_11: u64 = 1;
        // D s_46_12: and s_46_10 s_46_11
        let s_46_12: u64 = ((s_46_10) & (s_46_11));
        // D s_46_13: cmp-eq s_46_12 s_46_11
        let s_46_13: bool = ((s_46_12) == (s_46_11));
        // D s_46_14: lsl s_46_10 s_46_8
        let s_46_14: u64 = s_46_10 << s_46_8;
        // D s_46_15: or s_46_9 s_46_14
        let s_46_15: u64 = ((s_46_9) | (s_46_14));
        // D s_46_16: cmpl s_46_14
        let s_46_16: u64 = !s_46_14;
        // D s_46_17: and s_46_9 s_46_16
        let s_46_17: u64 = ((s_46_9) & (s_46_16));
        // D s_46_18: select s_46_13 s_46_15 s_46_17
        let s_46_18: u64 = if s_46_13 { s_46_15 } else { s_46_17 };
        // D s_46_19: cast trunc s_46_18 -> u8
        let s_46_19: bool = ((s_46_18) != 0);
        // D s_46_20: cast zx s_46_19 -> bv
        let s_46_20: Bits = Bits::new(s_46_19 as u128, 1u16);
        // C s_46_21: const #0u : u8
        let s_46_21: bool = false;
        // C s_46_22: cast zx s_46_21 -> bv
        let s_46_22: Bits = Bits::new(s_46_21 as u128, 1u16);
        // D s_46_23: cmp-eq s_46_20 s_46_22
        let s_46_23: bool = ((s_46_20) == (s_46_22));
        // N s_46_24: branch s_46_23 b55 b47
        if s_46_23 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #22712u : u32
        let s_47_0: u32 = 22712;
        // D s_47_1: read-reg s_47_0:struct
        let s_47_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_47_0 as isize);
            tracer.read_register(s_47_0 as isize, value);
            value
        };
        // D s_47_2: call _get_MDCR_EL3_Type_NSPB(s_47_1)
        let s_47_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_47_1);
        // C s_47_3: const #1s : i
        let s_47_3: i128 = 1;
        // D s_47_4: cast zx s_47_2 -> bv
        let s_47_4: Bits = Bits::new(s_47_2 as u128, 2u16);
        // C s_47_5: const #1u : u64
        let s_47_5: u64 = 1;
        // D s_47_6: bit-extract s_47_4 s_47_3 s_47_5
        let s_47_6: Bits = (Bits::new(
            ((s_47_4) >> (s_47_3)).value(),
            u16::try_from(s_47_5).unwrap(),
        ));
        // D s_47_7: cast reint s_47_6 -> u8
        let s_47_7: bool = ((s_47_6.value()) != 0);
        // C s_47_8: const #0s : i
        let s_47_8: i128 = 0;
        // C s_47_9: const #0u : u64
        let s_47_9: u64 = 0;
        // D s_47_10: cast zx s_47_7 -> u64
        let s_47_10: u64 = (s_47_7 as u64);
        // C s_47_11: const #1u : u64
        let s_47_11: u64 = 1;
        // D s_47_12: and s_47_10 s_47_11
        let s_47_12: u64 = ((s_47_10) & (s_47_11));
        // D s_47_13: cmp-eq s_47_12 s_47_11
        let s_47_13: bool = ((s_47_12) == (s_47_11));
        // D s_47_14: lsl s_47_10 s_47_8
        let s_47_14: u64 = s_47_10 << s_47_8;
        // D s_47_15: or s_47_9 s_47_14
        let s_47_15: u64 = ((s_47_9) | (s_47_14));
        // D s_47_16: cmpl s_47_14
        let s_47_16: u64 = !s_47_14;
        // D s_47_17: and s_47_9 s_47_16
        let s_47_17: u64 = ((s_47_9) & (s_47_16));
        // D s_47_18: select s_47_13 s_47_15 s_47_17
        let s_47_18: u64 = if s_47_13 { s_47_15 } else { s_47_17 };
        // D s_47_19: cast trunc s_47_18 -> u8
        let s_47_19: bool = ((s_47_18) != 0);
        // C s_47_20: const #90704u : u32
        let s_47_20: u32 = 90704;
        // D s_47_21: read-reg s_47_20:struct
        let s_47_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_47_20 as isize);
            tracer.read_register(s_47_20 as isize, value);
            value
        };
        // D s_47_22: call _get_SCR_EL3_Type_NS(s_47_21)
        let s_47_22: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_47_21);
        // D s_47_23: cast zx s_47_19 -> bv
        let s_47_23: Bits = Bits::new(s_47_19 as u128, 1u16);
        // D s_47_24: cast zx s_47_22 -> bv
        let s_47_24: Bits = Bits::new(s_47_22 as u128, 1u16);
        // D s_47_25: cmp-ne s_47_23 s_47_24
        let s_47_25: bool = ((s_47_23) != (s_47_24));
        // D s_47_26: write-var gs#90788 <= s_47_25
        fn_state.gs_90788 = s_47_25;
        // N s_47_27: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#90788:u8
        let s_48_0: bool = fn_state.gs_90788;
        // N s_48_1: branch s_48_0 b54 b49
        if s_48_0 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #232u : u32
        let s_49_0: u32 = 232;
        // S s_49_1: call IsFeatureImplemented(s_49_0)
        let s_49_1: bool = IsFeatureImplemented(state, tracer, s_49_0);
        // N s_49_2: branch s_49_1 b53 b50
        if s_49_1 {
            return block_53(state, tracer, fn_state);
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
        // D s_50_1: write-var gs#90789 <= s_50_0
        fn_state.gs_90789 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#90789:u8
        let s_51_0: bool = fn_state.gs_90789;
        // D s_51_1: write-var gs#90790 <= s_51_0
        fn_state.gs_90790 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#90790:u8
        let s_52_0: bool = fn_state.gs_90790;
        // D s_52_1: write-var gs#90791 <= s_52_0
        fn_state.gs_90791 = s_52_0;
        // N s_52_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var __MDCR_EL3_NSPBE:u8
        let s_53_0: bool = fn_state.u__MDCR_EL3_NSPBE;
        // D s_53_1: cast zx s_53_0 -> bv
        let s_53_1: Bits = Bits::new(s_53_0 as u128, 1u16);
        // D s_53_2: read-var __SCR_EL3_NSE:u8
        let s_53_2: bool = fn_state.u__SCR_EL3_NSE;
        // D s_53_3: cast zx s_53_2 -> bv
        let s_53_3: Bits = Bits::new(s_53_2 as u128, 1u16);
        // D s_53_4: cmp-ne s_53_1 s_53_3
        let s_53_4: bool = ((s_53_1) != (s_53_3));
        // D s_53_5: write-var gs#90789 <= s_53_4
        fn_state.gs_90789 = s_53_4;
        // N s_53_6: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #1u : u8
        let s_54_0: bool = true;
        // D s_54_1: write-var gs#90790 <= s_54_0
        fn_state.gs_90790 = s_54_0;
        // N s_54_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #1u : u8
        let s_55_0: bool = true;
        // D s_55_1: write-var gs#90788 <= s_55_0
        fn_state.gs_90788 = s_55_0;
        // N s_55_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_56_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_56_1: call __IMPDEF_boolean(s_56_0)
        let s_56_1: bool = u__IMPDEF_boolean(state, tracer, s_56_0);
        // D s_56_2: write-var gs#90783 <= s_56_1
        fn_state.gs_90783 = s_56_1;
        // N s_56_3: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #() : ()
        let s_57_0: () = ();
        // S s_57_1: call EDSCR_read(s_57_0)
        let s_57_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_57_0);
        // S s_57_2: call _get_EDSCR_Type_SDD(s_57_1)
        let s_57_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_57_1);
        // S s_57_3: cast zx s_57_2 -> bv
        let s_57_3: Bits = Bits::new(s_57_2 as u128, 1u16);
        // C s_57_4: const #1u : u8
        let s_57_4: bool = true;
        // C s_57_5: cast zx s_57_4 -> bv
        let s_57_5: Bits = Bits::new(s_57_4 as u128, 1u16);
        // S s_57_6: cmp-eq s_57_3 s_57_5
        let s_57_6: bool = ((s_57_3) == (s_57_5));
        // D s_57_7: write-var gs#90782 <= s_57_6
        fn_state.gs_90782 = s_57_6;
        // N s_57_8: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #424u : u32
        let s_58_0: u32 = 424;
        // D s_58_1: read-reg s_58_0:u8
        let s_58_1: u8 = {
            let value = state.read_register::<u8>(s_58_0 as isize);
            tracer.read_register(s_58_0 as isize, value);
            value
        };
        // C s_58_2: const #2u : u8
        let s_58_2: u8 = 2;
        // D s_58_3: cmp-lt s_58_1 s_58_2
        let s_58_3: bool = ((s_58_1) < (s_58_2));
        // D s_58_4: write-var gs#90781 <= s_58_3
        fn_state.gs_90781 = s_58_3;
        // N s_58_5: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #() : ()
        let s_59_0: () = ();
        // S s_59_1: call EL2Enabled(s_59_0)
        let s_59_1: bool = EL2Enabled(state, tracer, s_59_0);
        // N s_59_2: branch s_59_1 b69 b60
        if s_59_1 {
            return block_69(state, tracer, fn_state);
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
        // D s_60_1: write-var gs#90802 <= s_60_0
        fn_state.gs_90802 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#90802:u8
        let s_61_0: bool = fn_state.gs_90802;
        // N s_61_1: branch s_61_0 b68 b62
        if s_61_0 {
            return block_68(state, tracer, fn_state);
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
        // N s_62_2: branch s_62_1 b67 b63
        if s_62_1 {
            return block_67(state, tracer, fn_state);
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
        // D s_63_1: write-var gs#90803 <= s_63_0
        fn_state.gs_90803 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#90803:u8
        let s_64_0: bool = fn_state.gs_90803;
        // N s_64_1: branch s_64_0 b66 b65
        if s_64_0 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_65_0: panic
        panic!("{:?}", ());
        // N s_65_1: return
        return;
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #24u : u8
        let s_66_0: u8 = 24;
        // C s_66_1: cast zx s_66_0 -> bv
        let s_66_1: Bits = Bits::new(s_66_0 as u128, 8u16);
        // C s_66_2: cast zx s_66_1 -> i
        let s_66_2: i128 = (s_66_1.value() as i128);
        // C s_66_3: cast reint s_66_2 -> i64
        let s_66_3: i64 = (s_66_2 as i64);
        // C s_66_4: cast zx s_66_3 -> i
        let s_66_4: i128 = (i128::try_from(s_66_3).unwrap());
        // C s_66_5: const #432u : u32
        let s_66_5: u32 = 432;
        // D s_66_6: read-reg s_66_5:u8
        let s_66_6: u8 = {
            let value = state.read_register::<u8>(s_66_5 as isize);
            tracer.read_register(s_66_5 as isize, value);
            value
        };
        // D s_66_7: call AArch64_SystemAccessTrap(s_66_6, s_66_4)
        let s_66_7: () = AArch64_SystemAccessTrap(state, tracer, s_66_6, s_66_4);
        // N s_66_8: return
        return;
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var __HCR_EL2_NV:u8
        let s_67_0: bool = fn_state.u__HCR_EL2_NV;
        // D s_67_1: cast zx s_67_0 -> bv
        let s_67_1: Bits = Bits::new(s_67_0 as u128, 1u16);
        // C s_67_2: const #1u : u8
        let s_67_2: bool = true;
        // C s_67_3: cast zx s_67_2 -> bv
        let s_67_3: Bits = Bits::new(s_67_2 as u128, 1u16);
        // D s_67_4: cmp-eq s_67_1 s_67_3
        let s_67_4: bool = ((s_67_1) == (s_67_3));
        // D s_67_5: write-var gs#90803 <= s_67_4
        fn_state.gs_90803 = s_67_4;
        // N s_67_6: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #2088u : u12
        let s_68_0: u16 = 2088;
        // C s_68_1: cast zx s_68_0 -> bv
        let s_68_1: Bits = Bits::new(s_68_0 as u128, 12u16);
        // C s_68_2: cast zx s_68_1 -> i
        let s_68_2: i128 = (s_68_1.value() as i128);
        // C s_68_3: cast reint s_68_2 -> i64
        let s_68_3: i64 = (s_68_2 as i64);
        // C s_68_4: const #64s : i64
        let s_68_4: i64 = 64;
        // D s_68_5: read-var t:i
        let s_68_5: i128 = fn_state.t;
        // D s_68_6: call X_read(s_68_5, s_68_4)
        let s_68_6: Bits = X_read(state, tracer, s_68_5, s_68_4);
        // D s_68_7: cast reint s_68_6 -> u64
        let s_68_7: u64 = (s_68_6.value() as u64);
        // C s_68_8: cast zx s_68_3 -> i
        let s_68_8: i128 = (i128::try_from(s_68_3).unwrap());
        // D s_68_9: call NVMem_set(s_68_8, s_68_7)
        let s_68_9: () = NVMem_set(state, tracer, s_68_8, s_68_7);
        // N s_68_10: return
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
        // D s_69_2: call _get_HCR_EL2_Type_NV2(s_69_1)
        let s_69_2: bool = u_get_HCR_EL2_Type_NV2(state, tracer, s_69_1);
        // C s_69_3: const #102552u : u32
        let s_69_3: u32 = 102552;
        // D s_69_4: read-reg s_69_3:struct
        let s_69_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_69_3 as isize);
            tracer.read_register(s_69_3 as isize, value);
            value
        };
        // D s_69_5: call _get_HCR_EL2_Type_NV1(s_69_4)
        let s_69_5: bool = u_get_HCR_EL2_Type_NV1(state, tracer, s_69_4);
        // C s_69_6: const #102552u : u32
        let s_69_6: u32 = 102552;
        // D s_69_7: read-reg s_69_6:struct
        let s_69_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_69_6 as isize);
            tracer.read_register(s_69_6 as isize, value);
            value
        };
        // D s_69_8: call _get_HCR_EL2_Type_NV(s_69_7)
        let s_69_8: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_69_7);
        // D s_69_9: cast zx s_69_5 -> bv
        let s_69_9: Bits = Bits::new(s_69_5 as u128, 1u16);
        // D s_69_10: cast zx s_69_8 -> bv
        let s_69_10: Bits = Bits::new(s_69_8 as u128, 1u16);
        // D s_69_11: cast reint s_69_9 -> u128
        let s_69_11: u128 = (s_69_9.value() as u128);
        // D s_69_12: size-of s_69_9
        let s_69_12: u16 = s_69_9.length();
        // D s_69_13: cast reint s_69_10 -> u128
        let s_69_13: u128 = (s_69_10.value() as u128);
        // D s_69_14: size-of s_69_10
        let s_69_14: u16 = s_69_10.length();
        // D s_69_15: lsl s_69_11 s_69_14
        let s_69_15: u128 = s_69_11 << s_69_14;
        // D s_69_16: or s_69_15 s_69_13
        let s_69_16: u128 = ((s_69_15) | (s_69_13));
        // D s_69_17: add s_69_12 s_69_14
        let s_69_17: u16 = (s_69_12 + s_69_14);
        // D s_69_18: create-bits s_69_16 s_69_17
        let s_69_18: Bits = Bits::new(s_69_16, s_69_17);
        // D s_69_19: cast reint s_69_18 -> u8
        let s_69_19: u8 = (s_69_18.value() as u8);
        // D s_69_20: cast zx s_69_2 -> bv
        let s_69_20: Bits = Bits::new(s_69_2 as u128, 1u16);
        // D s_69_21: cast zx s_69_19 -> bv
        let s_69_21: Bits = Bits::new(s_69_19 as u128, 2u16);
        // D s_69_22: cast reint s_69_20 -> u128
        let s_69_22: u128 = (s_69_20.value() as u128);
        // D s_69_23: size-of s_69_20
        let s_69_23: u16 = s_69_20.length();
        // D s_69_24: cast reint s_69_21 -> u128
        let s_69_24: u128 = (s_69_21.value() as u128);
        // D s_69_25: size-of s_69_21
        let s_69_25: u16 = s_69_21.length();
        // D s_69_26: lsl s_69_22 s_69_25
        let s_69_26: u128 = s_69_22 << s_69_25;
        // D s_69_27: or s_69_26 s_69_24
        let s_69_27: u128 = ((s_69_26) | (s_69_24));
        // D s_69_28: add s_69_23 s_69_25
        let s_69_28: u16 = (s_69_23 + s_69_25);
        // D s_69_29: create-bits s_69_27 s_69_28
        let s_69_29: Bits = Bits::new(s_69_27, s_69_28);
        // D s_69_30: cast reint s_69_29 -> u8
        let s_69_30: u8 = (s_69_29.value() as u8);
        // D s_69_31: cast zx s_69_30 -> bv
        let s_69_31: Bits = Bits::new(s_69_30 as u128, 3u16);
        // C s_69_32: const #5u : u8
        let s_69_32: u8 = 5;
        // C s_69_33: cast zx s_69_32 -> bv
        let s_69_33: Bits = Bits::new(s_69_32 as u128, 3u16);
        // D s_69_34: cmp-eq s_69_31 s_69_33
        let s_69_34: bool = ((s_69_31) == (s_69_33));
        // D s_69_35: write-var gs#90802 <= s_69_34
        fn_state.gs_90802 = s_69_34;
        // N s_69_36: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_70_0: panic
        panic!("{:?}", ());
        // N s_70_1: return
        return;
    }
}
