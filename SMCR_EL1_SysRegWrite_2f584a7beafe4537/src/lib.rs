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
use Halted::*;
use NVMem_set::*;
use u_get_CPTR_EL3_Type_ESM::*;
use X_read::*;
use AArch64_SystemAccessTrap::*;
use u__IMPDEF_boolean::*;
use u_get_HCR_EL2_Type_NV1::*;
use u_get_CPTR_EL2_Type_SMEN::*;
use Mk_SMCR_EL1_Type::*;
use u_get_HCR_EL2_Type_NV::*;
use ELUsingAArch32::*;
use u_get_EDSCR_Type_SDD::*;
use EL2Enabled::*;
use EDSCR_read::*;
use u_get_HCR_EL2_Type_NV2::*;
use common::*;
pub fn SMCR_EL1_SysRegWrite_2f584a7beafe4537<T: Tracer>(
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
        gs_91925: bool,
        u__HCR_EL2_E2H: bool,
        gs_91916: bool,
        gs_91920: bool,
        u__EDSCR_SDD: bool,
        u__CPTR_EL3_ESM: bool,
        gs_91914: bool,
        gs_91913: bool,
        gs_91928: bool,
        gs_91929: bool,
        gs_91927: bool,
        gs_91919: bool,
        u__PSTATE_EL: u8,
        u__CPTR_EL2_SMEN: u8,
        u__HCR_EL2_NV: bool,
        gs_91918: bool,
        gs_91917: bool,
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
        // C s_0_11: const #() : ()
        let s_0_11: () = ();
        // S s_0_12: call EDSCR_read(s_0_11)
        let s_0_12: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_0_11);
        // S s_0_13: call _get_EDSCR_Type_SDD(s_0_12)
        let s_0_13: bool = u_get_EDSCR_Type_SDD(state, tracer, s_0_12);
        // D s_0_14: write-var __EDSCR_SDD <= s_0_13
        fn_state.u__EDSCR_SDD = s_0_13;
        // C s_0_15: const #16840u : u32
        let s_0_15: u32 = 16840;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_CPTR_EL3_Type_ESM(s_0_16)
        let s_0_17: bool = u_get_CPTR_EL3_Type_ESM(state, tracer, s_0_16);
        // D s_0_18: write-var __CPTR_EL3_ESM <= s_0_17
        fn_state.u__CPTR_EL3_ESM = s_0_17;
        // C s_0_19: const #11088u : u32
        let s_0_19: u32 = 11088;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_CPTR_EL2_Type_SMEN(s_0_20)
        let s_0_21: u8 = u_get_CPTR_EL2_Type_SMEN(state, tracer, s_0_20);
        // D s_0_22: write-var __CPTR_EL2_SMEN <= s_0_21
        fn_state.u__CPTR_EL2_SMEN = s_0_21;
        // D s_0_23: read-var __PSTATE_EL:u8
        let s_0_23: u8 = fn_state.u__PSTATE_EL;
        // D s_0_24: cast zx s_0_23 -> bv
        let s_0_24: Bits = Bits::new(s_0_23 as u128, 2u16);
        // C s_0_25: const #448u : u32
        let s_0_25: u32 = 448;
        // D s_0_26: read-reg s_0_25:u8
        let s_0_26: u8 = {
            let value = state.read_register::<u8>(s_0_25 as isize);
            tracer.read_register(s_0_25 as isize, value);
            value
        };
        // D s_0_27: cast zx s_0_26 -> bv
        let s_0_27: Bits = Bits::new(s_0_26 as u128, 2u16);
        // D s_0_28: cmp-eq s_0_24 s_0_27
        let s_0_28: bool = ((s_0_24) == (s_0_27));
        // N s_0_29: branch s_0_28 b59 b1
        if s_0_28 {
            return block_59(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b48 b2
        if s_1_5 {
            return block_48(state, tracer, fn_state);
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
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call EL2Enabled(s_5_0)
        let s_5_1: bool = EL2Enabled(state, tracer, s_5_0);
        // N s_5_2: branch s_5_1 b15 b6
        if s_5_1 {
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
        // D s_6_1: write-var gs#91913 <= s_6_0
        fn_state.gs_91913 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#91913:u8
        let s_7_0: bool = fn_state.gs_91913;
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
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#91914 <= s_8_0
        fn_state.gs_91914 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#91914:u8
        let s_9_0: bool = fn_state.gs_91914;
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
        // D s_11_0: read-var __CPTR_EL3_ESM:u8
        let s_11_0: bool = fn_state.u__CPTR_EL3_ESM;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 1u16);
        // C s_11_2: const #0u : u8
        let s_11_2: bool = false;
        // C s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 1u16);
        // D s_11_4: cmp-eq s_11_1 s_11_3
        let s_11_4: bool = ((s_11_1) == (s_11_3));
        // N s_11_5: branch s_11_4 b13 b12
        if s_11_4 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
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
        // D s_12_4: call Mk_SMCR_EL1_Type(s_12_3)
        let s_12_4: ProductType5c790c8ef59cc8b2 = Mk_SMCR_EL1_Type(
            state,
            tracer,
            s_12_3,
        );
        // C s_12_5: const #17304u : u32
        let s_12_5: u32 = 17304;
        // N s_12_6: write-reg s_12_5 <= s_12_4
        let s_12_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_12_5 as isize, s_12_4);
            tracer.write_register(s_12_5 as isize, s_12_4);
        };
        // N s_12_7: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #29u : u8
        let s_13_0: u8 = 29;
        // C s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 8u16);
        // C s_13_2: cast zx s_13_1 -> i
        let s_13_2: i128 = (s_13_1.value() as i128);
        // C s_13_3: cast reint s_13_2 -> i64
        let s_13_3: i64 = (s_13_2 as i64);
        // C s_13_4: cast zx s_13_3 -> i
        let s_13_4: i128 = (i128::try_from(s_13_3).unwrap());
        // C s_13_5: const #424u : u32
        let s_13_5: u32 = 424;
        // D s_13_6: read-reg s_13_5:u8
        let s_13_6: u8 = {
            let value = state.read_register::<u8>(s_13_5 as isize);
            tracer.read_register(s_13_5 as isize, value);
            value
        };
        // D s_13_7: call AArch64_SystemAccessTrap(s_13_6, s_13_4)
        let s_13_7: () = AArch64_SystemAccessTrap(state, tracer, s_13_6, s_13_4);
        // N s_13_8: return
        return;
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
        // D s_14_5: write-var gs#91914 <= s_14_4
        fn_state.gs_91914 = s_14_4;
        // N s_14_6: jump b9
        return block_9(state, tracer, fn_state);
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
        // D s_15_4: write-var gs#91913 <= s_15_3
        fn_state.gs_91913 = s_15_3;
        // N s_15_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var __HCR_EL2_E2H:u8
        let s_16_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 1u16);
        // C s_16_2: const #1u : u8
        let s_16_2: bool = true;
        // C s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 1u16);
        // D s_16_4: cmp-eq s_16_1 s_16_3
        let s_16_4: bool = ((s_16_1) == (s_16_3));
        // N s_16_5: branch s_16_4 b18 b17
        if s_16_4 {
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
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call Halted(s_18_0)
        let s_18_1: bool = Halted(state, tracer, s_18_0);
        // N s_18_2: branch s_18_1 b47 b19
        if s_18_1 {
            return block_47(state, tracer, fn_state);
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
        // D s_19_1: write-var gs#91916 <= s_19_0
        fn_state.gs_91916 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#91916:u8
        let s_20_0: bool = fn_state.gs_91916;
        // N s_20_1: branch s_20_0 b46 b21
        if s_20_0 {
            return block_46(state, tracer, fn_state);
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
        // D s_21_1: write-var gs#91917 <= s_21_0
        fn_state.gs_91917 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#91917:u8
        let s_22_0: bool = fn_state.gs_91917;
        // N s_22_1: branch s_22_0 b45 b23
        if s_22_0 {
            return block_45(state, tracer, fn_state);
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
        // D s_23_1: write-var gs#91918 <= s_23_0
        fn_state.gs_91918 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#91918:u8
        let s_24_0: bool = fn_state.gs_91918;
        // N s_24_1: branch s_24_0 b44 b25
        if s_24_0 {
            return block_44(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#91919 <= s_25_0
        fn_state.gs_91919 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#91919:u8
        let s_26_0: bool = fn_state.gs_91919;
        // N s_26_1: branch s_26_0 b43 b27
        if s_26_0 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var __CPTR_EL2_SMEN:u8
        let s_27_0: u8 = fn_state.u__CPTR_EL2_SMEN;
        // C s_27_1: const #0s : i
        let s_27_1: i128 = 0;
        // D s_27_2: cast zx s_27_0 -> bv
        let s_27_2: Bits = Bits::new(s_27_0 as u128, 2u16);
        // C s_27_3: const #1s : i64
        let s_27_3: i64 = 1;
        // C s_27_4: cast zx s_27_3 -> i
        let s_27_4: i128 = (i128::try_from(s_27_3).unwrap());
        // C s_27_5: const #0s : i
        let s_27_5: i128 = 0;
        // C s_27_6: add s_27_5 s_27_4
        let s_27_6: i128 = (s_27_5 + s_27_4);
        // D s_27_7: bit-extract s_27_2 s_27_1 s_27_6
        let s_27_7: Bits = (Bits::new(
            ((s_27_2) >> (s_27_1)).value(),
            u16::try_from(s_27_6).unwrap(),
        ));
        // D s_27_8: cast reint s_27_7 -> u8
        let s_27_8: bool = ((s_27_7.value()) != 0);
        // D s_27_9: cast zx s_27_8 -> bv
        let s_27_9: Bits = Bits::new(s_27_8 as u128, 1u16);
        // C s_27_10: const #0u : u8
        let s_27_10: bool = false;
        // C s_27_11: cast zx s_27_10 -> bv
        let s_27_11: Bits = Bits::new(s_27_10 as u128, 1u16);
        // D s_27_12: cmp-eq s_27_9 s_27_11
        let s_27_12: bool = ((s_27_9) == (s_27_11));
        // D s_27_13: not s_27_12
        let s_27_13: bool = !s_27_12;
        // N s_27_14: branch s_27_13 b42 b28
        if s_27_13 {
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
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var gs#91920 <= s_28_0
        fn_state.gs_91920 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#91920:u8
        let s_29_0: bool = fn_state.gs_91920;
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
        // C s_30_0: const #424u : u32
        let s_30_0: u32 = 424;
        // D s_30_1: read-reg s_30_0:u8
        let s_30_1: u8 = {
            let value = state.read_register::<u8>(s_30_0 as isize);
            tracer.read_register(s_30_0 as isize, value);
            value
        };
        // C s_30_2: const #2u : u8
        let s_30_2: u8 = 2;
        // D s_30_3: cmp-lt s_30_1 s_30_2
        let s_30_3: bool = ((s_30_1) < (s_30_2));
        // N s_30_4: branch s_30_3 b40 b31
        if s_30_3 {
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
        // D s_31_1: write-var gs#91925 <= s_31_0
        fn_state.gs_91925 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#91925:u8
        let s_32_0: bool = fn_state.gs_91925;
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
        // C s_33_0: const #64s : i64
        let s_33_0: i64 = 64;
        // D s_33_1: read-var t:i
        let s_33_1: i128 = fn_state.t;
        // D s_33_2: call X_read(s_33_1, s_33_0)
        let s_33_2: Bits = X_read(state, tracer, s_33_1, s_33_0);
        // D s_33_3: cast reint s_33_2 -> u64
        let s_33_3: u64 = (s_33_2.value() as u64);
        // D s_33_4: call Mk_SMCR_EL1_Type(s_33_3)
        let s_33_4: ProductType5c790c8ef59cc8b2 = Mk_SMCR_EL1_Type(
            state,
            tracer,
            s_33_3,
        );
        // C s_33_5: const #17304u : u32
        let s_33_5: u32 = 17304;
        // N s_33_6: write-reg s_33_5 <= s_33_4
        let s_33_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_33_5 as isize, s_33_4);
            tracer.write_register(s_33_5 as isize, s_33_4);
        };
        // N s_33_7: return
        return;
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
        // D s_35_1: write-var gs#91927 <= s_35_0
        fn_state.gs_91927 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#91927:u8
        let s_36_0: bool = fn_state.gs_91927;
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
        // C s_37_0: const #29u : u8
        let s_37_0: u8 = 29;
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
        // D s_39_5: write-var gs#91927 <= s_39_4
        fn_state.gs_91927 = s_39_4;
        // N s_39_6: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var __CPTR_EL3_ESM:u8
        let s_40_0: bool = fn_state.u__CPTR_EL3_ESM;
        // D s_40_1: cast zx s_40_0 -> bv
        let s_40_1: Bits = Bits::new(s_40_0 as u128, 1u16);
        // C s_40_2: const #0u : u8
        let s_40_2: bool = false;
        // C s_40_3: cast zx s_40_2 -> bv
        let s_40_3: Bits = Bits::new(s_40_2 as u128, 1u16);
        // D s_40_4: cmp-eq s_40_1 s_40_3
        let s_40_4: bool = ((s_40_1) == (s_40_3));
        // D s_40_5: write-var gs#91925 <= s_40_4
        fn_state.gs_91925 = s_40_4;
        // N s_40_6: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #29u : u8
        let s_41_0: u8 = 29;
        // C s_41_1: cast zx s_41_0 -> bv
        let s_41_1: Bits = Bits::new(s_41_0 as u128, 8u16);
        // C s_41_2: cast zx s_41_1 -> i
        let s_41_2: i128 = (s_41_1.value() as i128);
        // C s_41_3: cast reint s_41_2 -> i64
        let s_41_3: i64 = (s_41_2 as i64);
        // C s_41_4: cast zx s_41_3 -> i
        let s_41_4: i128 = (i128::try_from(s_41_3).unwrap());
        // C s_41_5: const #432u : u32
        let s_41_5: u32 = 432;
        // D s_41_6: read-reg s_41_5:u8
        let s_41_6: u8 = {
            let value = state.read_register::<u8>(s_41_5 as isize);
            tracer.read_register(s_41_5 as isize, value);
            value
        };
        // D s_41_7: call AArch64_SystemAccessTrap(s_41_6, s_41_4)
        let s_41_7: () = AArch64_SystemAccessTrap(state, tracer, s_41_6, s_41_4);
        // N s_41_8: return
        return;
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #0u : u8
        let s_42_0: bool = false;
        // D s_42_1: write-var gs#91920 <= s_42_0
        fn_state.gs_91920 = s_42_0;
        // N s_42_2: jump b29
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
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var __CPTR_EL3_ESM:u8
        let s_44_0: bool = fn_state.u__CPTR_EL3_ESM;
        // D s_44_1: cast zx s_44_0 -> bv
        let s_44_1: Bits = Bits::new(s_44_0 as u128, 1u16);
        // C s_44_2: const #0u : u8
        let s_44_2: bool = false;
        // C s_44_3: cast zx s_44_2 -> bv
        let s_44_3: Bits = Bits::new(s_44_2 as u128, 1u16);
        // D s_44_4: cmp-eq s_44_1 s_44_3
        let s_44_4: bool = ((s_44_1) == (s_44_3));
        // D s_44_5: write-var gs#91919 <= s_44_4
        fn_state.gs_91919 = s_44_4;
        // N s_44_6: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_45_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_45_1: call __IMPDEF_boolean(s_45_0)
        let s_45_1: bool = u__IMPDEF_boolean(state, tracer, s_45_0);
        // D s_45_2: write-var gs#91918 <= s_45_1
        fn_state.gs_91918 = s_45_1;
        // N s_45_3: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var __EDSCR_SDD:u8
        let s_46_0: bool = fn_state.u__EDSCR_SDD;
        // D s_46_1: cast zx s_46_0 -> bv
        let s_46_1: Bits = Bits::new(s_46_0 as u128, 1u16);
        // C s_46_2: const #1u : u8
        let s_46_2: bool = true;
        // C s_46_3: cast zx s_46_2 -> bv
        let s_46_3: Bits = Bits::new(s_46_2 as u128, 1u16);
        // D s_46_4: cmp-eq s_46_1 s_46_3
        let s_46_4: bool = ((s_46_1) == (s_46_3));
        // D s_46_5: write-var gs#91917 <= s_46_4
        fn_state.gs_91917 = s_46_4;
        // N s_46_6: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #424u : u32
        let s_47_0: u32 = 424;
        // D s_47_1: read-reg s_47_0:u8
        let s_47_1: u8 = {
            let value = state.read_register::<u8>(s_47_0 as isize);
            tracer.read_register(s_47_0 as isize, value);
            value
        };
        // C s_47_2: const #2u : u8
        let s_47_2: u8 = 2;
        // D s_47_3: cmp-lt s_47_1 s_47_2
        let s_47_3: bool = ((s_47_1) < (s_47_2));
        // D s_47_4: write-var gs#91916 <= s_47_3
        fn_state.gs_91916 = s_47_3;
        // N s_47_5: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #() : ()
        let s_48_0: () = ();
        // S s_48_1: call EL2Enabled(s_48_0)
        let s_48_1: bool = EL2Enabled(state, tracer, s_48_0);
        // N s_48_2: branch s_48_1 b58 b49
        if s_48_1 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #0u : u8
        let s_49_0: bool = false;
        // D s_49_1: write-var gs#91928 <= s_49_0
        fn_state.gs_91928 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#91928:u8
        let s_50_0: bool = fn_state.gs_91928;
        // N s_50_1: branch s_50_0 b57 b51
        if s_50_0 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #() : ()
        let s_51_0: () = ();
        // S s_51_1: call EL2Enabled(s_51_0)
        let s_51_1: bool = EL2Enabled(state, tracer, s_51_0);
        // N s_51_2: branch s_51_1 b56 b52
        if s_51_1 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #0u : u8
        let s_52_0: bool = false;
        // D s_52_1: write-var gs#91929 <= s_52_0
        fn_state.gs_91929 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#91929:u8
        let s_53_0: bool = fn_state.gs_91929;
        // N s_53_1: branch s_53_0 b55 b54
        if s_53_0 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_54_0: panic
        panic!("{:?}", ());
        // N s_54_1: return
        return;
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
        // C s_55_5: const #432u : u32
        let s_55_5: u32 = 432;
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
        // D s_56_0: read-var __HCR_EL2_NV:u8
        let s_56_0: bool = fn_state.u__HCR_EL2_NV;
        // D s_56_1: cast zx s_56_0 -> bv
        let s_56_1: Bits = Bits::new(s_56_0 as u128, 1u16);
        // C s_56_2: const #1u : u8
        let s_56_2: bool = true;
        // C s_56_3: cast zx s_56_2 -> bv
        let s_56_3: Bits = Bits::new(s_56_2 as u128, 1u16);
        // D s_56_4: cmp-eq s_56_1 s_56_3
        let s_56_4: bool = ((s_56_1) == (s_56_3));
        // D s_56_5: write-var gs#91929 <= s_56_4
        fn_state.gs_91929 = s_56_4;
        // N s_56_6: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #496u : u12
        let s_57_0: u16 = 496;
        // C s_57_1: cast zx s_57_0 -> bv
        let s_57_1: Bits = Bits::new(s_57_0 as u128, 12u16);
        // C s_57_2: cast zx s_57_1 -> i
        let s_57_2: i128 = (s_57_1.value() as i128);
        // C s_57_3: cast reint s_57_2 -> i64
        let s_57_3: i64 = (s_57_2 as i64);
        // C s_57_4: const #64s : i64
        let s_57_4: i64 = 64;
        // D s_57_5: read-var t:i
        let s_57_5: i128 = fn_state.t;
        // D s_57_6: call X_read(s_57_5, s_57_4)
        let s_57_6: Bits = X_read(state, tracer, s_57_5, s_57_4);
        // D s_57_7: cast reint s_57_6 -> u64
        let s_57_7: u64 = (s_57_6.value() as u64);
        // C s_57_8: cast zx s_57_3 -> i
        let s_57_8: i128 = (i128::try_from(s_57_3).unwrap());
        // D s_57_9: call NVMem_set(s_57_8, s_57_7)
        let s_57_9: () = NVMem_set(state, tracer, s_57_8, s_57_7);
        // N s_57_10: return
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
        // D s_58_2: call _get_HCR_EL2_Type_NV2(s_58_1)
        let s_58_2: bool = u_get_HCR_EL2_Type_NV2(state, tracer, s_58_1);
        // C s_58_3: const #102552u : u32
        let s_58_3: u32 = 102552;
        // D s_58_4: read-reg s_58_3:struct
        let s_58_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_58_3 as isize);
            tracer.read_register(s_58_3 as isize, value);
            value
        };
        // D s_58_5: call _get_HCR_EL2_Type_NV1(s_58_4)
        let s_58_5: bool = u_get_HCR_EL2_Type_NV1(state, tracer, s_58_4);
        // C s_58_6: const #102552u : u32
        let s_58_6: u32 = 102552;
        // D s_58_7: read-reg s_58_6:struct
        let s_58_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_58_6 as isize);
            tracer.read_register(s_58_6 as isize, value);
            value
        };
        // D s_58_8: call _get_HCR_EL2_Type_NV(s_58_7)
        let s_58_8: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_58_7);
        // D s_58_9: cast zx s_58_5 -> bv
        let s_58_9: Bits = Bits::new(s_58_5 as u128, 1u16);
        // D s_58_10: cast zx s_58_8 -> bv
        let s_58_10: Bits = Bits::new(s_58_8 as u128, 1u16);
        // D s_58_11: cast reint s_58_9 -> u128
        let s_58_11: u128 = (s_58_9.value() as u128);
        // D s_58_12: size-of s_58_9
        let s_58_12: u16 = s_58_9.length();
        // D s_58_13: cast reint s_58_10 -> u128
        let s_58_13: u128 = (s_58_10.value() as u128);
        // D s_58_14: size-of s_58_10
        let s_58_14: u16 = s_58_10.length();
        // D s_58_15: lsl s_58_11 s_58_14
        let s_58_15: u128 = s_58_11 << s_58_14;
        // D s_58_16: or s_58_15 s_58_13
        let s_58_16: u128 = ((s_58_15) | (s_58_13));
        // D s_58_17: add s_58_12 s_58_14
        let s_58_17: u16 = (s_58_12 + s_58_14);
        // D s_58_18: create-bits s_58_16 s_58_17
        let s_58_18: Bits = Bits::new(s_58_16, s_58_17);
        // D s_58_19: cast reint s_58_18 -> u8
        let s_58_19: u8 = (s_58_18.value() as u8);
        // D s_58_20: cast zx s_58_2 -> bv
        let s_58_20: Bits = Bits::new(s_58_2 as u128, 1u16);
        // D s_58_21: cast zx s_58_19 -> bv
        let s_58_21: Bits = Bits::new(s_58_19 as u128, 2u16);
        // D s_58_22: cast reint s_58_20 -> u128
        let s_58_22: u128 = (s_58_20.value() as u128);
        // D s_58_23: size-of s_58_20
        let s_58_23: u16 = s_58_20.length();
        // D s_58_24: cast reint s_58_21 -> u128
        let s_58_24: u128 = (s_58_21.value() as u128);
        // D s_58_25: size-of s_58_21
        let s_58_25: u16 = s_58_21.length();
        // D s_58_26: lsl s_58_22 s_58_25
        let s_58_26: u128 = s_58_22 << s_58_25;
        // D s_58_27: or s_58_26 s_58_24
        let s_58_27: u128 = ((s_58_26) | (s_58_24));
        // D s_58_28: add s_58_23 s_58_25
        let s_58_28: u16 = (s_58_23 + s_58_25);
        // D s_58_29: create-bits s_58_27 s_58_28
        let s_58_29: Bits = Bits::new(s_58_27, s_58_28);
        // D s_58_30: cast reint s_58_29 -> u8
        let s_58_30: u8 = (s_58_29.value() as u8);
        // D s_58_31: cast zx s_58_30 -> bv
        let s_58_31: Bits = Bits::new(s_58_30 as u128, 3u16);
        // C s_58_32: const #5u : u8
        let s_58_32: u8 = 5;
        // C s_58_33: cast zx s_58_32 -> bv
        let s_58_33: Bits = Bits::new(s_58_32 as u128, 3u16);
        // D s_58_34: cmp-eq s_58_31 s_58_33
        let s_58_34: bool = ((s_58_31) == (s_58_33));
        // D s_58_35: write-var gs#91928 <= s_58_34
        fn_state.gs_91928 = s_58_34;
        // N s_58_36: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_59_0: panic
        panic!("{:?}", ());
        // N s_59_1: return
        return;
    }
}
