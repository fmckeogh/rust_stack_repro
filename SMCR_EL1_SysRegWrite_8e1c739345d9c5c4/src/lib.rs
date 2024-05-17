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
use u_get_CPTR_EL3_Type_ESM::*;
use X_read::*;
use AArch64_SystemAccessTrap::*;
use u__IMPDEF_boolean::*;
use NVMem_set::*;
use u_get_HCR_EL2_Type_NV1::*;
use u_get_CPTR_EL2_Type_SMEN::*;
use Mk_SMCR_EL1_Type::*;
use Mk_SMCR_EL2_Type::*;
use u_get_HCR_EL2_Type_NV::*;
use u_get_CPTR_EL2_Type_TSM::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_CPACR_EL1_Type_SMEN::*;
use EL2Enabled::*;
use EDSCR_read::*;
use u_get_HCR_EL2_Type_NV2::*;
use common::*;
pub fn SMCR_EL1_SysRegWrite_8e1c739345d9c5c4<T: Tracer>(
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
        u__EDSCR_SDD: bool,
        gs_91942: bool,
        gs_91937: bool,
        gs_91935: bool,
        u__CPTR_EL3_ESM: bool,
        gs_91948: bool,
        u__CPACR_EL1_SMEN: u8,
        gs_91946: bool,
        gs_91951: bool,
        gs_91949: bool,
        gs_91957: bool,
        gs_91956: bool,
        gs_91959: bool,
        gs_91947: bool,
        gs_91964: bool,
        gs_91933: bool,
        gs_91932: bool,
        u__CPTR_EL2_TSM: bool,
        gs_91969: bool,
        gs_91965: bool,
        gs_91966: bool,
        gs_91936: bool,
        u__PSTATE_EL: u8,
        u__CPTR_EL2_SMEN: u8,
        gs_91950: bool,
        gs_91958: bool,
        gs_91943: bool,
        gs_91934: bool,
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
        // S s_0_4: call EDSCR_read(s_0_3)
        let s_0_4: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_0_3);
        // S s_0_5: call _get_EDSCR_Type_SDD(s_0_4)
        let s_0_5: bool = u_get_EDSCR_Type_SDD(state, tracer, s_0_4);
        // D s_0_6: write-var __EDSCR_SDD <= s_0_5
        fn_state.u__EDSCR_SDD = s_0_5;
        // C s_0_7: const #16840u : u32
        let s_0_7: u32 = 16840;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_CPTR_EL3_Type_ESM(s_0_8)
        let s_0_9: bool = u_get_CPTR_EL3_Type_ESM(state, tracer, s_0_8);
        // D s_0_10: write-var __CPTR_EL3_ESM <= s_0_9
        fn_state.u__CPTR_EL3_ESM = s_0_9;
        // C s_0_11: const #12088u : u32
        let s_0_11: u32 = 12088;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_CPACR_EL1_Type_SMEN(s_0_12)
        let s_0_13: u8 = u_get_CPACR_EL1_Type_SMEN(state, tracer, s_0_12);
        // D s_0_14: write-var __CPACR_EL1_SMEN <= s_0_13
        fn_state.u__CPACR_EL1_SMEN = s_0_13;
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
        // C s_0_19: const #11088u : u32
        let s_0_19: u32 = 11088;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_CPTR_EL2_Type_TSM(s_0_20)
        let s_0_21: bool = u_get_CPTR_EL2_Type_TSM(state, tracer, s_0_20);
        // D s_0_22: write-var __CPTR_EL2_TSM <= s_0_21
        fn_state.u__CPTR_EL2_TSM = s_0_21;
        // C s_0_23: const #11088u : u32
        let s_0_23: u32 = 11088;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_CPTR_EL2_Type_SMEN(s_0_24)
        let s_0_25: u8 = u_get_CPTR_EL2_Type_SMEN(state, tracer, s_0_24);
        // D s_0_26: write-var __CPTR_EL2_SMEN <= s_0_25
        fn_state.u__CPTR_EL2_SMEN = s_0_25;
        // D s_0_27: read-var __PSTATE_EL:u8
        let s_0_27: u8 = fn_state.u__PSTATE_EL;
        // D s_0_28: cast zx s_0_27 -> bv
        let s_0_28: Bits = Bits::new(s_0_27 as u128, 2u16);
        // C s_0_29: const #448u : u32
        let s_0_29: u32 = 448;
        // D s_0_30: read-reg s_0_29:u8
        let s_0_30: u8 = {
            let value = state.read_register::<u8>(s_0_29 as isize);
            tracer.read_register(s_0_29 as isize, value);
            value
        };
        // D s_0_31: cast zx s_0_30 -> bv
        let s_0_31: Bits = Bits::new(s_0_30 as u128, 2u16);
        // D s_0_32: cmp-eq s_0_28 s_0_31
        let s_0_32: bool = ((s_0_28) == (s_0_31));
        // N s_0_33: branch s_0_32 b102 b1
        if s_0_32 {
            return block_102(state, tracer, fn_state);
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
        // D s_5_0: read-var __CPTR_EL3_ESM:u8
        let s_5_0: bool = fn_state.u__CPTR_EL3_ESM;
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
        // D s_6_4: call Mk_SMCR_EL1_Type(s_6_3)
        let s_6_4: ProductType5c790c8ef59cc8b2 = Mk_SMCR_EL1_Type(state, tracer, s_6_3);
        // C s_6_5: const #17304u : u32
        let s_6_5: u32 = 17304;
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
        // C s_7_0: const #29u : u8
        let s_7_0: u8 = 29;
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
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call Halted(s_8_0)
        let s_8_1: bool = Halted(state, tracer, s_8_0);
        // N s_8_2: branch s_8_1 b47 b9
        if s_8_1 {
            return block_47(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#91932 <= s_9_0
        fn_state.gs_91932 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#91932:u8
        let s_10_0: bool = fn_state.gs_91932;
        // N s_10_1: branch s_10_0 b46 b11
        if s_10_0 {
            return block_46(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#91933 <= s_11_0
        fn_state.gs_91933 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#91933:u8
        let s_12_0: bool = fn_state.gs_91933;
        // N s_12_1: branch s_12_0 b45 b13
        if s_12_0 {
            return block_45(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#91934 <= s_13_0
        fn_state.gs_91934 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#91934:u8
        let s_14_0: bool = fn_state.gs_91934;
        // N s_14_1: branch s_14_0 b44 b15
        if s_14_0 {
            return block_44(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#91935 <= s_15_0
        fn_state.gs_91935 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#91935:u8
        let s_16_0: bool = fn_state.gs_91935;
        // N s_16_1: branch s_16_0 b43 b17
        if s_16_0 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var __HCR_EL2_E2H:u8
        let s_17_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 1u16);
        // C s_17_2: const #0u : u8
        let s_17_2: bool = false;
        // C s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 1u16);
        // D s_17_4: cmp-eq s_17_1 s_17_3
        let s_17_4: bool = ((s_17_1) == (s_17_3));
        // N s_17_5: branch s_17_4 b42 b18
        if s_17_4 {
            return block_42(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#91936 <= s_18_0
        fn_state.gs_91936 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#91936:u8
        let s_19_0: bool = fn_state.gs_91936;
        // N s_19_1: branch s_19_0 b41 b20
        if s_19_0 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var __HCR_EL2_E2H:u8
        let s_20_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 1u16);
        // C s_20_2: const #1u : u8
        let s_20_2: bool = true;
        // C s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 1u16);
        // D s_20_4: cmp-eq s_20_1 s_20_3
        let s_20_4: bool = ((s_20_1) == (s_20_3));
        // N s_20_5: branch s_20_4 b37 b21
        if s_20_4 {
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
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#91942 <= s_21_0
        fn_state.gs_91942 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#91942:u8
        let s_22_0: bool = fn_state.gs_91942;
        // N s_22_1: branch s_22_0 b36 b23
        if s_22_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #424u : u32
        let s_23_0: u32 = 424;
        // D s_23_1: read-reg s_23_0:u8
        let s_23_1: u8 = {
            let value = state.read_register::<u8>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // C s_23_2: const #2u : u8
        let s_23_2: u8 = 2;
        // D s_23_3: cmp-lt s_23_1 s_23_2
        let s_23_3: bool = ((s_23_1) < (s_23_2));
        // N s_23_4: branch s_23_3 b35 b24
        if s_23_3 {
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
        // C s_24_0: const #0u : u8
        let s_24_0: bool = false;
        // D s_24_1: write-var gs#91943 <= s_24_0
        fn_state.gs_91943 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#91943:u8
        let s_25_0: bool = fn_state.gs_91943;
        // N s_25_1: branch s_25_0 b29 b26
        if s_25_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var __HCR_EL2_E2H:u8
        let s_26_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 1u16);
        // C s_26_2: const #1u : u8
        let s_26_2: bool = true;
        // C s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 1u16);
        // D s_26_4: cmp-eq s_26_1 s_26_3
        let s_26_4: bool = ((s_26_1) == (s_26_3));
        // N s_26_5: branch s_26_4 b28 b27
        if s_26_4 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
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
        // D s_27_4: call Mk_SMCR_EL1_Type(s_27_3)
        let s_27_4: ProductType5c790c8ef59cc8b2 = Mk_SMCR_EL1_Type(
            state,
            tracer,
            s_27_3,
        );
        // C s_27_5: const #17304u : u32
        let s_27_5: u32 = 17304;
        // N s_27_6: write-reg s_27_5 <= s_27_4
        let s_27_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_27_5 as isize, s_27_4);
            tracer.write_register(s_27_5 as isize, s_27_4);
        };
        // N s_27_7: return
        return;
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
        // D s_28_4: call Mk_SMCR_EL2_Type(s_28_3)
        let s_28_4: ProductType5c790c8ef59cc8b2 = Mk_SMCR_EL2_Type(
            state,
            tracer,
            s_28_3,
        );
        // C s_28_5: const #10376u : u32
        let s_28_5: u32 = 10376;
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
        // D s_30_1: write-var gs#91946 <= s_30_0
        fn_state.gs_91946 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#91946:u8
        let s_31_0: bool = fn_state.gs_91946;
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
        // C s_32_0: const #29u : u8
        let s_32_0: u8 = 29;
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
        // D s_34_0: read-var __EDSCR_SDD:u8
        let s_34_0: bool = fn_state.u__EDSCR_SDD;
        // D s_34_1: cast zx s_34_0 -> bv
        let s_34_1: Bits = Bits::new(s_34_0 as u128, 1u16);
        // C s_34_2: const #1u : u8
        let s_34_2: bool = true;
        // C s_34_3: cast zx s_34_2 -> bv
        let s_34_3: Bits = Bits::new(s_34_2 as u128, 1u16);
        // D s_34_4: cmp-eq s_34_1 s_34_3
        let s_34_4: bool = ((s_34_1) == (s_34_3));
        // D s_34_5: write-var gs#91946 <= s_34_4
        fn_state.gs_91946 = s_34_4;
        // N s_34_6: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var __CPTR_EL3_ESM:u8
        let s_35_0: bool = fn_state.u__CPTR_EL3_ESM;
        // D s_35_1: cast zx s_35_0 -> bv
        let s_35_1: Bits = Bits::new(s_35_0 as u128, 1u16);
        // C s_35_2: const #0u : u8
        let s_35_2: bool = false;
        // C s_35_3: cast zx s_35_2 -> bv
        let s_35_3: Bits = Bits::new(s_35_2 as u128, 1u16);
        // D s_35_4: cmp-eq s_35_1 s_35_3
        let s_35_4: bool = ((s_35_1) == (s_35_3));
        // D s_35_5: write-var gs#91943 <= s_35_4
        fn_state.gs_91943 = s_35_4;
        // N s_35_6: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #29u : u8
        let s_36_0: u8 = 29;
        // C s_36_1: cast zx s_36_0 -> bv
        let s_36_1: Bits = Bits::new(s_36_0 as u128, 8u16);
        // C s_36_2: cast zx s_36_1 -> i
        let s_36_2: i128 = (s_36_1.value() as i128);
        // C s_36_3: cast reint s_36_2 -> i64
        let s_36_3: i64 = (s_36_2 as i64);
        // C s_36_4: cast zx s_36_3 -> i
        let s_36_4: i128 = (i128::try_from(s_36_3).unwrap());
        // C s_36_5: const #432u : u32
        let s_36_5: u32 = 432;
        // D s_36_6: read-reg s_36_5:u8
        let s_36_6: u8 = {
            let value = state.read_register::<u8>(s_36_5 as isize);
            tracer.read_register(s_36_5 as isize, value);
            value
        };
        // D s_36_7: call AArch64_SystemAccessTrap(s_36_6, s_36_4)
        let s_36_7: () = AArch64_SystemAccessTrap(state, tracer, s_36_6, s_36_4);
        // N s_36_8: return
        return;
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var __CPTR_EL2_SMEN:u8
        let s_37_0: u8 = fn_state.u__CPTR_EL2_SMEN;
        // C s_37_1: const #0s : i
        let s_37_1: i128 = 0;
        // D s_37_2: cast zx s_37_0 -> bv
        let s_37_2: Bits = Bits::new(s_37_0 as u128, 2u16);
        // C s_37_3: const #1s : i64
        let s_37_3: i64 = 1;
        // C s_37_4: cast zx s_37_3 -> i
        let s_37_4: i128 = (i128::try_from(s_37_3).unwrap());
        // C s_37_5: const #0s : i
        let s_37_5: i128 = 0;
        // C s_37_6: add s_37_5 s_37_4
        let s_37_6: i128 = (s_37_5 + s_37_4);
        // D s_37_7: bit-extract s_37_2 s_37_1 s_37_6
        let s_37_7: Bits = (Bits::new(
            ((s_37_2) >> (s_37_1)).value(),
            u16::try_from(s_37_6).unwrap(),
        ));
        // D s_37_8: cast reint s_37_7 -> u8
        let s_37_8: bool = ((s_37_7.value()) != 0);
        // D s_37_9: cast zx s_37_8 -> bv
        let s_37_9: Bits = Bits::new(s_37_8 as u128, 1u16);
        // C s_37_10: const #0u : u8
        let s_37_10: bool = false;
        // C s_37_11: cast zx s_37_10 -> bv
        let s_37_11: Bits = Bits::new(s_37_10 as u128, 1u16);
        // D s_37_12: cmp-eq s_37_9 s_37_11
        let s_37_12: bool = ((s_37_9) == (s_37_11));
        // D s_37_13: not s_37_12
        let s_37_13: bool = !s_37_12;
        // N s_37_14: branch s_37_13 b40 b38
        if s_37_13 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #1u : u8
        let s_38_0: bool = true;
        // D s_38_1: write-var gs#91937 <= s_38_0
        fn_state.gs_91937 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#91937:u8
        let s_39_0: bool = fn_state.gs_91937;
        // D s_39_1: write-var gs#91942 <= s_39_0
        fn_state.gs_91942 = s_39_0;
        // N s_39_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #0u : u8
        let s_40_0: bool = false;
        // D s_40_1: write-var gs#91937 <= s_40_0
        fn_state.gs_91937 = s_40_0;
        // N s_40_2: jump b39
        return block_39(state, tracer, fn_state);
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
        // D s_42_0: read-var __CPTR_EL2_TSM:u8
        let s_42_0: bool = fn_state.u__CPTR_EL2_TSM;
        // D s_42_1: cast zx s_42_0 -> bv
        let s_42_1: Bits = Bits::new(s_42_0 as u128, 1u16);
        // C s_42_2: const #1u : u8
        let s_42_2: bool = true;
        // C s_42_3: cast zx s_42_2 -> bv
        let s_42_3: Bits = Bits::new(s_42_2 as u128, 1u16);
        // D s_42_4: cmp-eq s_42_1 s_42_3
        let s_42_4: bool = ((s_42_1) == (s_42_3));
        // D s_42_5: write-var gs#91936 <= s_42_4
        fn_state.gs_91936 = s_42_4;
        // N s_42_6: jump b19
        return block_19(state, tracer, fn_state);
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
        // D s_44_5: write-var gs#91935 <= s_44_4
        fn_state.gs_91935 = s_44_4;
        // N s_44_6: jump b16
        return block_16(state, tracer, fn_state);
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
        // D s_45_2: write-var gs#91934 <= s_45_1
        fn_state.gs_91934 = s_45_1;
        // N s_45_3: jump b14
        return block_14(state, tracer, fn_state);
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
        // D s_46_5: write-var gs#91933 <= s_46_4
        fn_state.gs_91933 = s_46_4;
        // N s_46_6: jump b12
        return block_12(state, tracer, fn_state);
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
        // D s_47_4: write-var gs#91932 <= s_47_3
        fn_state.gs_91932 = s_47_3;
        // N s_47_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #() : ()
        let s_48_0: () = ();
        // S s_48_1: call Halted(s_48_0)
        let s_48_1: bool = Halted(state, tracer, s_48_0);
        // N s_48_2: branch s_48_1 b101 b49
        if s_48_1 {
            return block_101(state, tracer, fn_state);
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
        // D s_49_1: write-var gs#91947 <= s_49_0
        fn_state.gs_91947 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#91947:u8
        let s_50_0: bool = fn_state.gs_91947;
        // N s_50_1: branch s_50_0 b100 b51
        if s_50_0 {
            return block_100(state, tracer, fn_state);
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
        // D s_51_1: write-var gs#91948 <= s_51_0
        fn_state.gs_91948 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#91948:u8
        let s_52_0: bool = fn_state.gs_91948;
        // N s_52_1: branch s_52_0 b99 b53
        if s_52_0 {
            return block_99(state, tracer, fn_state);
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
        // D s_53_1: write-var gs#91949 <= s_53_0
        fn_state.gs_91949 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#91949:u8
        let s_54_0: bool = fn_state.gs_91949;
        // N s_54_1: branch s_54_0 b98 b55
        if s_54_0 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #0u : u8
        let s_55_0: bool = false;
        // D s_55_1: write-var gs#91950 <= s_55_0
        fn_state.gs_91950 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#91950:u8
        let s_56_0: bool = fn_state.gs_91950;
        // N s_56_1: branch s_56_0 b97 b57
        if s_56_0 {
            return block_97(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var __CPACR_EL1_SMEN:u8
        let s_57_0: u8 = fn_state.u__CPACR_EL1_SMEN;
        // C s_57_1: const #0s : i
        let s_57_1: i128 = 0;
        // D s_57_2: cast zx s_57_0 -> bv
        let s_57_2: Bits = Bits::new(s_57_0 as u128, 2u16);
        // C s_57_3: const #1s : i64
        let s_57_3: i64 = 1;
        // C s_57_4: cast zx s_57_3 -> i
        let s_57_4: i128 = (i128::try_from(s_57_3).unwrap());
        // C s_57_5: const #0s : i
        let s_57_5: i128 = 0;
        // C s_57_6: add s_57_5 s_57_4
        let s_57_6: i128 = (s_57_5 + s_57_4);
        // D s_57_7: bit-extract s_57_2 s_57_1 s_57_6
        let s_57_7: Bits = (Bits::new(
            ((s_57_2) >> (s_57_1)).value(),
            u16::try_from(s_57_6).unwrap(),
        ));
        // D s_57_8: cast reint s_57_7 -> u8
        let s_57_8: bool = ((s_57_7.value()) != 0);
        // D s_57_9: cast zx s_57_8 -> bv
        let s_57_9: Bits = Bits::new(s_57_8 as u128, 1u16);
        // C s_57_10: const #0u : u8
        let s_57_10: bool = false;
        // C s_57_11: cast zx s_57_10 -> bv
        let s_57_11: Bits = Bits::new(s_57_10 as u128, 1u16);
        // D s_57_12: cmp-eq s_57_9 s_57_11
        let s_57_12: bool = ((s_57_9) == (s_57_11));
        // D s_57_13: not s_57_12
        let s_57_13: bool = !s_57_12;
        // N s_57_14: branch s_57_13 b96 b58
        if s_57_13 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #1u : u8
        let s_58_0: bool = true;
        // D s_58_1: write-var gs#91951 <= s_58_0
        fn_state.gs_91951 = s_58_0;
        // N s_58_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var gs#91951:u8
        let s_59_0: bool = fn_state.gs_91951;
        // N s_59_1: branch s_59_0 b95 b60
        if s_59_0 {
            return block_95(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #() : ()
        let s_60_0: () = ();
        // S s_60_1: call EL2Enabled(s_60_0)
        let s_60_1: bool = EL2Enabled(state, tracer, s_60_0);
        // N s_60_2: branch s_60_1 b94 b61
        if s_60_1 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #0u : u8
        let s_61_0: bool = false;
        // D s_61_1: write-var gs#91956 <= s_61_0
        fn_state.gs_91956 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#91956:u8
        let s_62_0: bool = fn_state.gs_91956;
        // N s_62_1: branch s_62_0 b93 b63
        if s_62_0 {
            return block_93(state, tracer, fn_state);
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
        // D s_63_1: write-var gs#91957 <= s_63_0
        fn_state.gs_91957 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#91957:u8
        let s_64_0: bool = fn_state.gs_91957;
        // N s_64_1: branch s_64_0 b92 b65
        if s_64_0 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #() : ()
        let s_65_0: () = ();
        // S s_65_1: call EL2Enabled(s_65_0)
        let s_65_1: bool = EL2Enabled(state, tracer, s_65_0);
        // N s_65_2: branch s_65_1 b91 b66
        if s_65_1 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_66(state, tracer, fn_state);
        };
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #0u : u8
        let s_66_0: bool = false;
        // D s_66_1: write-var gs#91958 <= s_66_0
        fn_state.gs_91958 = s_66_0;
        // N s_66_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#91958:u8
        let s_67_0: bool = fn_state.gs_91958;
        // N s_67_1: branch s_67_0 b87 b68
        if s_67_0 {
            return block_87(state, tracer, fn_state);
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
        // D s_68_1: write-var gs#91964 <= s_68_0
        fn_state.gs_91964 = s_68_0;
        // N s_68_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var gs#91964:u8
        let s_69_0: bool = fn_state.gs_91964;
        // N s_69_1: branch s_69_0 b86 b70
        if s_69_0 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_70(state, tracer, fn_state);
        };
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #424u : u32
        let s_70_0: u32 = 424;
        // D s_70_1: read-reg s_70_0:u8
        let s_70_1: u8 = {
            let value = state.read_register::<u8>(s_70_0 as isize);
            tracer.read_register(s_70_0 as isize, value);
            value
        };
        // C s_70_2: const #2u : u8
        let s_70_2: u8 = 2;
        // D s_70_3: cmp-lt s_70_1 s_70_2
        let s_70_3: bool = ((s_70_1) < (s_70_2));
        // N s_70_4: branch s_70_3 b85 b71
        if s_70_3 {
            return block_85(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #0u : u8
        let s_71_0: bool = false;
        // D s_71_1: write-var gs#91965 <= s_71_0
        fn_state.gs_91965 = s_71_0;
        // N s_71_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var gs#91965:u8
        let s_72_0: bool = fn_state.gs_91965;
        // N s_72_1: branch s_72_0 b79 b73
        if s_72_0 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #() : ()
        let s_73_0: () = ();
        // S s_73_1: call EL2Enabled(s_73_0)
        let s_73_1: bool = EL2Enabled(state, tracer, s_73_0);
        // N s_73_2: branch s_73_1 b78 b74
        if s_73_1 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_74(state, tracer, fn_state);
        };
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #0u : u8
        let s_74_0: bool = false;
        // D s_74_1: write-var gs#91966 <= s_74_0
        fn_state.gs_91966 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var gs#91966:u8
        let s_75_0: bool = fn_state.gs_91966;
        // N s_75_1: branch s_75_0 b77 b76
        if s_75_0 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_76(state, tracer, fn_state);
        };
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
        // D s_76_4: call Mk_SMCR_EL1_Type(s_76_3)
        let s_76_4: ProductType5c790c8ef59cc8b2 = Mk_SMCR_EL1_Type(
            state,
            tracer,
            s_76_3,
        );
        // C s_76_5: const #17304u : u32
        let s_76_5: u32 = 17304;
        // N s_76_6: write-reg s_76_5 <= s_76_4
        let s_76_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_76_5 as isize, s_76_4);
            tracer.write_register(s_76_5 as isize, s_76_4);
        };
        // N s_76_7: return
        return;
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #496u : u12
        let s_77_0: u16 = 496;
        // C s_77_1: cast zx s_77_0 -> bv
        let s_77_1: Bits = Bits::new(s_77_0 as u128, 12u16);
        // C s_77_2: cast zx s_77_1 -> i
        let s_77_2: i128 = (s_77_1.value() as i128);
        // C s_77_3: cast reint s_77_2 -> i64
        let s_77_3: i64 = (s_77_2 as i64);
        // C s_77_4: const #64s : i64
        let s_77_4: i64 = 64;
        // D s_77_5: read-var t:i
        let s_77_5: i128 = fn_state.t;
        // D s_77_6: call X_read(s_77_5, s_77_4)
        let s_77_6: Bits = X_read(state, tracer, s_77_5, s_77_4);
        // D s_77_7: cast reint s_77_6 -> u64
        let s_77_7: u64 = (s_77_6.value() as u64);
        // C s_77_8: cast zx s_77_3 -> i
        let s_77_8: i128 = (i128::try_from(s_77_3).unwrap());
        // D s_77_9: call NVMem_set(s_77_8, s_77_7)
        let s_77_9: () = NVMem_set(state, tracer, s_77_8, s_77_7);
        // N s_77_10: return
        return;
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
        // D s_78_2: call _get_HCR_EL2_Type_NV2(s_78_1)
        let s_78_2: bool = u_get_HCR_EL2_Type_NV2(state, tracer, s_78_1);
        // C s_78_3: const #102552u : u32
        let s_78_3: u32 = 102552;
        // D s_78_4: read-reg s_78_3:struct
        let s_78_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_78_3 as isize);
            tracer.read_register(s_78_3 as isize, value);
            value
        };
        // D s_78_5: call _get_HCR_EL2_Type_NV1(s_78_4)
        let s_78_5: bool = u_get_HCR_EL2_Type_NV1(state, tracer, s_78_4);
        // C s_78_6: const #102552u : u32
        let s_78_6: u32 = 102552;
        // D s_78_7: read-reg s_78_6:struct
        let s_78_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_78_6 as isize);
            tracer.read_register(s_78_6 as isize, value);
            value
        };
        // D s_78_8: call _get_HCR_EL2_Type_NV(s_78_7)
        let s_78_8: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_78_7);
        // D s_78_9: cast zx s_78_5 -> bv
        let s_78_9: Bits = Bits::new(s_78_5 as u128, 1u16);
        // D s_78_10: cast zx s_78_8 -> bv
        let s_78_10: Bits = Bits::new(s_78_8 as u128, 1u16);
        // D s_78_11: cast reint s_78_9 -> u128
        let s_78_11: u128 = (s_78_9.value() as u128);
        // D s_78_12: size-of s_78_9
        let s_78_12: u16 = s_78_9.length();
        // D s_78_13: cast reint s_78_10 -> u128
        let s_78_13: u128 = (s_78_10.value() as u128);
        // D s_78_14: size-of s_78_10
        let s_78_14: u16 = s_78_10.length();
        // D s_78_15: lsl s_78_11 s_78_14
        let s_78_15: u128 = s_78_11 << s_78_14;
        // D s_78_16: or s_78_15 s_78_13
        let s_78_16: u128 = ((s_78_15) | (s_78_13));
        // D s_78_17: add s_78_12 s_78_14
        let s_78_17: u16 = (s_78_12 + s_78_14);
        // D s_78_18: create-bits s_78_16 s_78_17
        let s_78_18: Bits = Bits::new(s_78_16, s_78_17);
        // D s_78_19: cast reint s_78_18 -> u8
        let s_78_19: u8 = (s_78_18.value() as u8);
        // D s_78_20: cast zx s_78_2 -> bv
        let s_78_20: Bits = Bits::new(s_78_2 as u128, 1u16);
        // D s_78_21: cast zx s_78_19 -> bv
        let s_78_21: Bits = Bits::new(s_78_19 as u128, 2u16);
        // D s_78_22: cast reint s_78_20 -> u128
        let s_78_22: u128 = (s_78_20.value() as u128);
        // D s_78_23: size-of s_78_20
        let s_78_23: u16 = s_78_20.length();
        // D s_78_24: cast reint s_78_21 -> u128
        let s_78_24: u128 = (s_78_21.value() as u128);
        // D s_78_25: size-of s_78_21
        let s_78_25: u16 = s_78_21.length();
        // D s_78_26: lsl s_78_22 s_78_25
        let s_78_26: u128 = s_78_22 << s_78_25;
        // D s_78_27: or s_78_26 s_78_24
        let s_78_27: u128 = ((s_78_26) | (s_78_24));
        // D s_78_28: add s_78_23 s_78_25
        let s_78_28: u16 = (s_78_23 + s_78_25);
        // D s_78_29: create-bits s_78_27 s_78_28
        let s_78_29: Bits = Bits::new(s_78_27, s_78_28);
        // D s_78_30: cast reint s_78_29 -> u8
        let s_78_30: u8 = (s_78_29.value() as u8);
        // D s_78_31: cast zx s_78_30 -> bv
        let s_78_31: Bits = Bits::new(s_78_30 as u128, 3u16);
        // C s_78_32: const #7u : u8
        let s_78_32: u8 = 7;
        // C s_78_33: cast zx s_78_32 -> bv
        let s_78_33: Bits = Bits::new(s_78_32 as u128, 3u16);
        // D s_78_34: cmp-eq s_78_31 s_78_33
        let s_78_34: bool = ((s_78_31) == (s_78_33));
        // D s_78_35: write-var gs#91966 <= s_78_34
        fn_state.gs_91966 = s_78_34;
        // N s_78_36: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #() : ()
        let s_79_0: () = ();
        // S s_79_1: call Halted(s_79_0)
        let s_79_1: bool = Halted(state, tracer, s_79_0);
        // N s_79_2: branch s_79_1 b84 b80
        if s_79_1 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #0u : u8
        let s_80_0: bool = false;
        // D s_80_1: write-var gs#91969 <= s_80_0
        fn_state.gs_91969 = s_80_0;
        // N s_80_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#91969:u8
        let s_81_0: bool = fn_state.gs_91969;
        // N s_81_1: branch s_81_0 b83 b82
        if s_81_0 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_82(state, tracer, fn_state);
        };
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #29u : u8
        let s_82_0: u8 = 29;
        // C s_82_1: cast zx s_82_0 -> bv
        let s_82_1: Bits = Bits::new(s_82_0 as u128, 8u16);
        // C s_82_2: cast zx s_82_1 -> i
        let s_82_2: i128 = (s_82_1.value() as i128);
        // C s_82_3: cast reint s_82_2 -> i64
        let s_82_3: i64 = (s_82_2 as i64);
        // C s_82_4: cast zx s_82_3 -> i
        let s_82_4: i128 = (i128::try_from(s_82_3).unwrap());
        // C s_82_5: const #424u : u32
        let s_82_5: u32 = 424;
        // D s_82_6: read-reg s_82_5:u8
        let s_82_6: u8 = {
            let value = state.read_register::<u8>(s_82_5 as isize);
            tracer.read_register(s_82_5 as isize, value);
            value
        };
        // D s_82_7: call AArch64_SystemAccessTrap(s_82_6, s_82_4)
        let s_82_7: () = AArch64_SystemAccessTrap(state, tracer, s_82_6, s_82_4);
        // N s_82_8: return
        return;
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_83_0: panic
        panic!("{:?}", ());
        // N s_83_1: return
        return;
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var __EDSCR_SDD:u8
        let s_84_0: bool = fn_state.u__EDSCR_SDD;
        // D s_84_1: cast zx s_84_0 -> bv
        let s_84_1: Bits = Bits::new(s_84_0 as u128, 1u16);
        // C s_84_2: const #1u : u8
        let s_84_2: bool = true;
        // C s_84_3: cast zx s_84_2 -> bv
        let s_84_3: Bits = Bits::new(s_84_2 as u128, 1u16);
        // D s_84_4: cmp-eq s_84_1 s_84_3
        let s_84_4: bool = ((s_84_1) == (s_84_3));
        // D s_84_5: write-var gs#91969 <= s_84_4
        fn_state.gs_91969 = s_84_4;
        // N s_84_6: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var __CPTR_EL3_ESM:u8
        let s_85_0: bool = fn_state.u__CPTR_EL3_ESM;
        // D s_85_1: cast zx s_85_0 -> bv
        let s_85_1: Bits = Bits::new(s_85_0 as u128, 1u16);
        // C s_85_2: const #0u : u8
        let s_85_2: bool = false;
        // C s_85_3: cast zx s_85_2 -> bv
        let s_85_3: Bits = Bits::new(s_85_2 as u128, 1u16);
        // D s_85_4: cmp-eq s_85_1 s_85_3
        let s_85_4: bool = ((s_85_1) == (s_85_3));
        // D s_85_5: write-var gs#91965 <= s_85_4
        fn_state.gs_91965 = s_85_4;
        // N s_85_6: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #29u : u8
        let s_86_0: u8 = 29;
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
        // D s_87_0: read-var __CPTR_EL2_SMEN:u8
        let s_87_0: u8 = fn_state.u__CPTR_EL2_SMEN;
        // C s_87_1: const #0s : i
        let s_87_1: i128 = 0;
        // D s_87_2: cast zx s_87_0 -> bv
        let s_87_2: Bits = Bits::new(s_87_0 as u128, 2u16);
        // C s_87_3: const #1s : i64
        let s_87_3: i64 = 1;
        // C s_87_4: cast zx s_87_3 -> i
        let s_87_4: i128 = (i128::try_from(s_87_3).unwrap());
        // C s_87_5: const #0s : i
        let s_87_5: i128 = 0;
        // C s_87_6: add s_87_5 s_87_4
        let s_87_6: i128 = (s_87_5 + s_87_4);
        // D s_87_7: bit-extract s_87_2 s_87_1 s_87_6
        let s_87_7: Bits = (Bits::new(
            ((s_87_2) >> (s_87_1)).value(),
            u16::try_from(s_87_6).unwrap(),
        ));
        // D s_87_8: cast reint s_87_7 -> u8
        let s_87_8: bool = ((s_87_7.value()) != 0);
        // D s_87_9: cast zx s_87_8 -> bv
        let s_87_9: Bits = Bits::new(s_87_8 as u128, 1u16);
        // C s_87_10: const #0u : u8
        let s_87_10: bool = false;
        // C s_87_11: cast zx s_87_10 -> bv
        let s_87_11: Bits = Bits::new(s_87_10 as u128, 1u16);
        // D s_87_12: cmp-eq s_87_9 s_87_11
        let s_87_12: bool = ((s_87_9) == (s_87_11));
        // D s_87_13: not s_87_12
        let s_87_13: bool = !s_87_12;
        // N s_87_14: branch s_87_13 b90 b88
        if s_87_13 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_88(state, tracer, fn_state);
        };
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #1u : u8
        let s_88_0: bool = true;
        // D s_88_1: write-var gs#91959 <= s_88_0
        fn_state.gs_91959 = s_88_0;
        // N s_88_2: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_89_0: read-var gs#91959:u8
        let s_89_0: bool = fn_state.gs_91959;
        // D s_89_1: write-var gs#91964 <= s_89_0
        fn_state.gs_91964 = s_89_0;
        // N s_89_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #0u : u8
        let s_90_0: bool = false;
        // D s_90_1: write-var gs#91959 <= s_90_0
        fn_state.gs_91959 = s_90_0;
        // N s_90_2: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_91_0: read-var __HCR_EL2_E2H:u8
        let s_91_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_91_1: cast zx s_91_0 -> bv
        let s_91_1: Bits = Bits::new(s_91_0 as u128, 1u16);
        // C s_91_2: const #1u : u8
        let s_91_2: bool = true;
        // C s_91_3: cast zx s_91_2 -> bv
        let s_91_3: Bits = Bits::new(s_91_2 as u128, 1u16);
        // D s_91_4: cmp-eq s_91_1 s_91_3
        let s_91_4: bool = ((s_91_1) == (s_91_3));
        // D s_91_5: write-var gs#91958 <= s_91_4
        fn_state.gs_91958 = s_91_4;
        // N s_91_6: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #29u : u8
        let s_92_0: u8 = 29;
        // C s_92_1: cast zx s_92_0 -> bv
        let s_92_1: Bits = Bits::new(s_92_0 as u128, 8u16);
        // C s_92_2: cast zx s_92_1 -> i
        let s_92_2: i128 = (s_92_1.value() as i128);
        // C s_92_3: cast reint s_92_2 -> i64
        let s_92_3: i64 = (s_92_2 as i64);
        // C s_92_4: cast zx s_92_3 -> i
        let s_92_4: i128 = (i128::try_from(s_92_3).unwrap());
        // C s_92_5: const #432u : u32
        let s_92_5: u32 = 432;
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
        // D s_93_0: read-var __CPTR_EL2_TSM:u8
        let s_93_0: bool = fn_state.u__CPTR_EL2_TSM;
        // D s_93_1: cast zx s_93_0 -> bv
        let s_93_1: Bits = Bits::new(s_93_0 as u128, 1u16);
        // C s_93_2: const #1u : u8
        let s_93_2: bool = true;
        // C s_93_3: cast zx s_93_2 -> bv
        let s_93_3: Bits = Bits::new(s_93_2 as u128, 1u16);
        // D s_93_4: cmp-eq s_93_1 s_93_3
        let s_93_4: bool = ((s_93_1) == (s_93_3));
        // D s_93_5: write-var gs#91957 <= s_93_4
        fn_state.gs_91957 = s_93_4;
        // N s_93_6: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var __HCR_EL2_E2H:u8
        let s_94_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_94_1: cast zx s_94_0 -> bv
        let s_94_1: Bits = Bits::new(s_94_0 as u128, 1u16);
        // C s_94_2: const #0u : u8
        let s_94_2: bool = false;
        // C s_94_3: cast zx s_94_2 -> bv
        let s_94_3: Bits = Bits::new(s_94_2 as u128, 1u16);
        // D s_94_4: cmp-eq s_94_1 s_94_3
        let s_94_4: bool = ((s_94_1) == (s_94_3));
        // D s_94_5: write-var gs#91956 <= s_94_4
        fn_state.gs_91956 = s_94_4;
        // N s_94_6: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #29u : u8
        let s_95_0: u8 = 29;
        // C s_95_1: cast zx s_95_0 -> bv
        let s_95_1: Bits = Bits::new(s_95_0 as u128, 8u16);
        // C s_95_2: cast zx s_95_1 -> i
        let s_95_2: i128 = (s_95_1.value() as i128);
        // C s_95_3: cast reint s_95_2 -> i64
        let s_95_3: i64 = (s_95_2 as i64);
        // C s_95_4: cast zx s_95_3 -> i
        let s_95_4: i128 = (i128::try_from(s_95_3).unwrap());
        // C s_95_5: const #440u : u32
        let s_95_5: u32 = 440;
        // D s_95_6: read-reg s_95_5:u8
        let s_95_6: u8 = {
            let value = state.read_register::<u8>(s_95_5 as isize);
            tracer.read_register(s_95_5 as isize, value);
            value
        };
        // D s_95_7: call AArch64_SystemAccessTrap(s_95_6, s_95_4)
        let s_95_7: () = AArch64_SystemAccessTrap(state, tracer, s_95_6, s_95_4);
        // N s_95_8: return
        return;
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #0u : u8
        let s_96_0: bool = false;
        // D s_96_1: write-var gs#91951 <= s_96_0
        fn_state.gs_91951 = s_96_0;
        // N s_96_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_97_0: panic
        panic!("{:?}", ());
        // N s_97_1: return
        return;
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var __CPTR_EL3_ESM:u8
        let s_98_0: bool = fn_state.u__CPTR_EL3_ESM;
        // D s_98_1: cast zx s_98_0 -> bv
        let s_98_1: Bits = Bits::new(s_98_0 as u128, 1u16);
        // C s_98_2: const #0u : u8
        let s_98_2: bool = false;
        // C s_98_3: cast zx s_98_2 -> bv
        let s_98_3: Bits = Bits::new(s_98_2 as u128, 1u16);
        // D s_98_4: cmp-eq s_98_1 s_98_3
        let s_98_4: bool = ((s_98_1) == (s_98_3));
        // D s_98_5: write-var gs#91950 <= s_98_4
        fn_state.gs_91950 = s_98_4;
        // N s_98_6: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_99_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_99_1: call __IMPDEF_boolean(s_99_0)
        let s_99_1: bool = u__IMPDEF_boolean(state, tracer, s_99_0);
        // D s_99_2: write-var gs#91949 <= s_99_1
        fn_state.gs_91949 = s_99_1;
        // N s_99_3: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_100_0: read-var __EDSCR_SDD:u8
        let s_100_0: bool = fn_state.u__EDSCR_SDD;
        // D s_100_1: cast zx s_100_0 -> bv
        let s_100_1: Bits = Bits::new(s_100_0 as u128, 1u16);
        // C s_100_2: const #1u : u8
        let s_100_2: bool = true;
        // C s_100_3: cast zx s_100_2 -> bv
        let s_100_3: Bits = Bits::new(s_100_2 as u128, 1u16);
        // D s_100_4: cmp-eq s_100_1 s_100_3
        let s_100_4: bool = ((s_100_1) == (s_100_3));
        // D s_100_5: write-var gs#91948 <= s_100_4
        fn_state.gs_91948 = s_100_4;
        // N s_100_6: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #424u : u32
        let s_101_0: u32 = 424;
        // D s_101_1: read-reg s_101_0:u8
        let s_101_1: u8 = {
            let value = state.read_register::<u8>(s_101_0 as isize);
            tracer.read_register(s_101_0 as isize, value);
            value
        };
        // C s_101_2: const #2u : u8
        let s_101_2: u8 = 2;
        // D s_101_3: cmp-lt s_101_1 s_101_2
        let s_101_3: bool = ((s_101_1) < (s_101_2));
        // D s_101_4: write-var gs#91947 <= s_101_3
        fn_state.gs_91947 = s_101_3;
        // N s_101_5: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_102_0: panic
        panic!("{:?}", ());
        // N s_102_1: return
        return;
    }
}
