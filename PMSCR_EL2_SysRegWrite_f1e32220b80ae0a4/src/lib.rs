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
use u_get_SCR_EL3_Type_FGTEn::*;
use u_get_HDFGWTR_EL2_Type_PMSCR_EL1::*;
use u_get_HCR_EL2_Type_E2H::*;
use Mk_PMSCR_EL1_Type::*;
use Halted::*;
use u_get_MDCR_EL3_Type_NSPB::*;
use X_read::*;
use AArch64_SystemAccessTrap::*;
use IsFeatureImplemented::*;
use u_get_SCR_EL3_Type_NS::*;
use u__IMPDEF_boolean::*;
use Mk_PMSCR_EL2_Type::*;
use NVMem_set::*;
use u_get_HCR_EL2_Type_NV1::*;
use u_get_HCR_EL2_Type_NV::*;
use u_get_SCR_EL3_Type_NSE::*;
use u_get_MDCR_EL3_Type_NSPBE::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_MDCR_EL2_Type_TPMS::*;
use EL2Enabled::*;
use EDSCR_read::*;
use u_get_HCR_EL2_Type_NV2::*;
use common::*;
pub fn PMSCR_EL2_SysRegWrite_f1e32220b80ae0a4<T: Tracer>(
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
        gs_90875: bool,
        gs_90866: bool,
        u__MDCR_EL3_NSPBE: bool,
        gs_90845: bool,
        gs_90878: bool,
        gs_90872: bool,
        gs_90844: bool,
        gs_90830: bool,
        gs_90853: bool,
        gs_90864: bool,
        u__PSTATE_EL: u8,
        gs_90831: bool,
        gs_90847: bool,
        u__HDFGWTR_EL2_PMSCR_EL1: bool,
        gs_90859: bool,
        gs_90858: bool,
        gs_90863: bool,
        gs_90852: bool,
        gs_90862: bool,
        gs_90839: bool,
        gs_90873: bool,
        u__SCR_EL3_FGTEn: bool,
        gs_90851: bool,
        gs_90865: bool,
        gs_90846: bool,
        u__MDCR_EL2_TPMS: bool,
        gs_90829: bool,
        gs_90874: bool,
        gs_90861: bool,
        gs_90860: bool,
        gs_90838: bool,
        u__SCR_EL3_NSE: bool,
        gs_90837: bool,
        gs_90871: bool,
        gs_90836: bool,
        gs_90850: bool,
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
        // C s_0_3: const #22712u : u32
        let s_0_3: u32 = 22712;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_MDCR_EL3_Type_NSPBE(s_0_4)
        let s_0_5: bool = u_get_MDCR_EL3_Type_NSPBE(state, tracer, s_0_4);
        // D s_0_6: write-var __MDCR_EL3_NSPBE <= s_0_5
        fn_state.u__MDCR_EL3_NSPBE = s_0_5;
        // C s_0_7: const #90704u : u32
        let s_0_7: u32 = 90704;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_SCR_EL3_Type_NSE(s_0_8)
        let s_0_9: bool = u_get_SCR_EL3_Type_NSE(state, tracer, s_0_8);
        // D s_0_10: write-var __SCR_EL3_NSE <= s_0_9
        fn_state.u__SCR_EL3_NSE = s_0_9;
        // C s_0_11: const #90704u : u32
        let s_0_11: u32 = 90704;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_SCR_EL3_Type_FGTEn(s_0_12)
        let s_0_13: bool = u_get_SCR_EL3_Type_FGTEn(state, tracer, s_0_12);
        // D s_0_14: write-var __SCR_EL3_FGTEn <= s_0_13
        fn_state.u__SCR_EL3_FGTEn = s_0_13;
        // C s_0_15: const #17360u : u32
        let s_0_15: u32 = 17360;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_HDFGWTR_EL2_Type_PMSCR_EL1(s_0_16)
        let s_0_17: bool = u_get_HDFGWTR_EL2_Type_PMSCR_EL1(state, tracer, s_0_16);
        // D s_0_18: write-var __HDFGWTR_EL2_PMSCR_EL1 <= s_0_17
        fn_state.u__HDFGWTR_EL2_PMSCR_EL1 = s_0_17;
        // C s_0_19: const #104880u : u32
        let s_0_19: u32 = 104880;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_MDCR_EL2_Type_TPMS(s_0_20)
        let s_0_21: bool = u_get_MDCR_EL2_Type_TPMS(state, tracer, s_0_20);
        // D s_0_22: write-var __MDCR_EL2_TPMS <= s_0_21
        fn_state.u__MDCR_EL2_TPMS = s_0_21;
        // C s_0_23: const #102552u : u32
        let s_0_23: u32 = 102552;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_HCR_EL2_Type_E2H(s_0_24)
        let s_0_25: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_0_24);
        // D s_0_26: write-var __HCR_EL2_E2H <= s_0_25
        fn_state.u__HCR_EL2_E2H = s_0_25;
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
        // N s_0_33: branch s_0_32 b118 b1
        if s_0_32 {
            return block_118(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b51 b2
        if s_1_5 {
            return block_51(state, tracer, fn_state);
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
        // D s_5_1: read-var t:i
        let s_5_1: i128 = fn_state.t;
        // D s_5_2: call X_read(s_5_1, s_5_0)
        let s_5_2: Bits = X_read(state, tracer, s_5_1, s_5_0);
        // D s_5_3: cast reint s_5_2 -> u64
        let s_5_3: u64 = (s_5_2.value() as u64);
        // D s_5_4: call Mk_PMSCR_EL1_Type(s_5_3)
        let s_5_4: ProductType5c790c8ef59cc8b2 = Mk_PMSCR_EL1_Type(state, tracer, s_5_3);
        // C s_5_5: const #21072u : u32
        let s_5_5: u32 = 21072;
        // N s_5_6: write-reg s_5_5 <= s_5_4
        let s_5_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_5_5 as isize, s_5_4);
            tracer.write_register(s_5_5 as isize, s_5_4);
        };
        // N s_5_7: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call Halted(s_6_0)
        let s_6_1: bool = Halted(state, tracer, s_6_0);
        // N s_6_2: branch s_6_1 b50 b7
        if s_6_1 {
            return block_50(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#90829 <= s_7_0
        fn_state.gs_90829 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#90829:u8
        let s_8_0: bool = fn_state.gs_90829;
        // N s_8_1: branch s_8_0 b49 b9
        if s_8_0 {
            return block_49(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#90830 <= s_9_0
        fn_state.gs_90830 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#90830:u8
        let s_10_0: bool = fn_state.gs_90830;
        // N s_10_1: branch s_10_0 b48 b11
        if s_10_0 {
            return block_48(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#90831 <= s_11_0
        fn_state.gs_90831 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#90831:u8
        let s_12_0: bool = fn_state.gs_90831;
        // N s_12_1: branch s_12_0 b38 b13
        if s_12_0 {
            return block_38(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#90839 <= s_13_0
        fn_state.gs_90839 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#90839:u8
        let s_14_0: bool = fn_state.gs_90839;
        // N s_14_1: branch s_14_0 b37 b15
        if s_14_0 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #424u : u32
        let s_15_0: u32 = 424;
        // D s_15_1: read-reg s_15_0:u8
        let s_15_1: u8 = {
            let value = state.read_register::<u8>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // C s_15_2: const #2u : u8
        let s_15_2: u8 = 2;
        // D s_15_3: cmp-lt s_15_1 s_15_2
        let s_15_3: bool = ((s_15_1) < (s_15_2));
        // N s_15_4: branch s_15_3 b27 b16
        if s_15_3 {
            return block_27(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#90847 <= s_16_0
        fn_state.gs_90847 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#90847:u8
        let s_17_0: bool = fn_state.gs_90847;
        // N s_17_1: branch s_17_0 b21 b18
        if s_17_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var __HCR_EL2_E2H:u8
        let s_18_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 1u16);
        // C s_18_2: const #1u : u8
        let s_18_2: bool = true;
        // C s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 1u16);
        // D s_18_4: cmp-eq s_18_1 s_18_3
        let s_18_4: bool = ((s_18_1) == (s_18_3));
        // N s_18_5: branch s_18_4 b20 b19
        if s_18_4 {
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
        // D s_19_4: call Mk_PMSCR_EL1_Type(s_19_3)
        let s_19_4: ProductType5c790c8ef59cc8b2 = Mk_PMSCR_EL1_Type(
            state,
            tracer,
            s_19_3,
        );
        // C s_19_5: const #21072u : u32
        let s_19_5: u32 = 21072;
        // N s_19_6: write-reg s_19_5 <= s_19_4
        let s_19_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_19_5 as isize, s_19_4);
            tracer.write_register(s_19_5 as isize, s_19_4);
        };
        // N s_19_7: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #64s : i64
        let s_20_0: i64 = 64;
        // D s_20_1: read-var t:i
        let s_20_1: i128 = fn_state.t;
        // D s_20_2: call X_read(s_20_1, s_20_0)
        let s_20_2: Bits = X_read(state, tracer, s_20_1, s_20_0);
        // D s_20_3: cast reint s_20_2 -> u64
        let s_20_3: u64 = (s_20_2.value() as u64);
        // D s_20_4: call Mk_PMSCR_EL2_Type(s_20_3)
        let s_20_4: ProductType5c790c8ef59cc8b2 = Mk_PMSCR_EL2_Type(
            state,
            tracer,
            s_20_3,
        );
        // C s_20_5: const #104928u : u32
        let s_20_5: u32 = 104928;
        // N s_20_6: write-reg s_20_5 <= s_20_4
        let s_20_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_20_5 as isize, s_20_4);
            tracer.write_register(s_20_5 as isize, s_20_4);
        };
        // N s_20_7: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call Halted(s_21_0)
        let s_21_1: bool = Halted(state, tracer, s_21_0);
        // N s_21_2: branch s_21_1 b26 b22
        if s_21_1 {
            return block_26(state, tracer, fn_state);
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
        // D s_22_1: write-var gs#90850 <= s_22_0
        fn_state.gs_90850 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#90850:u8
        let s_23_0: bool = fn_state.gs_90850;
        // N s_23_1: branch s_23_0 b25 b24
        if s_23_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #24u : u8
        let s_24_0: u8 = 24;
        // C s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 8u16);
        // C s_24_2: cast zx s_24_1 -> i
        let s_24_2: i128 = (s_24_1.value() as i128);
        // C s_24_3: cast reint s_24_2 -> i64
        let s_24_3: i64 = (s_24_2 as i64);
        // C s_24_4: cast zx s_24_3 -> i
        let s_24_4: i128 = (i128::try_from(s_24_3).unwrap());
        // C s_24_5: const #424u : u32
        let s_24_5: u32 = 424;
        // D s_24_6: read-reg s_24_5:u8
        let s_24_6: u8 = {
            let value = state.read_register::<u8>(s_24_5 as isize);
            tracer.read_register(s_24_5 as isize, value);
            value
        };
        // D s_24_7: call AArch64_SystemAccessTrap(s_24_6, s_24_4)
        let s_24_7: () = AArch64_SystemAccessTrap(state, tracer, s_24_6, s_24_4);
        // N s_24_8: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_25_0: panic
        panic!("{:?}", ());
        // N s_25_1: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #() : ()
        let s_26_0: () = ();
        // S s_26_1: call EDSCR_read(s_26_0)
        let s_26_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_26_0);
        // S s_26_2: call _get_EDSCR_Type_SDD(s_26_1)
        let s_26_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_26_1);
        // S s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 1u16);
        // C s_26_4: const #1u : u8
        let s_26_4: bool = true;
        // C s_26_5: cast zx s_26_4 -> bv
        let s_26_5: Bits = Bits::new(s_26_4 as u128, 1u16);
        // S s_26_6: cmp-eq s_26_3 s_26_5
        let s_26_6: bool = ((s_26_3) == (s_26_5));
        // D s_26_7: write-var gs#90850 <= s_26_6
        fn_state.gs_90850 = s_26_6;
        // N s_26_8: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #22712u : u32
        let s_27_0: u32 = 22712;
        // D s_27_1: read-reg s_27_0:struct
        let s_27_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_27_0 as isize);
            tracer.read_register(s_27_0 as isize, value);
            value
        };
        // D s_27_2: call _get_MDCR_EL3_Type_NSPB(s_27_1)
        let s_27_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_27_1);
        // C s_27_3: const #0s : i
        let s_27_3: i128 = 0;
        // D s_27_4: cast zx s_27_2 -> bv
        let s_27_4: Bits = Bits::new(s_27_2 as u128, 2u16);
        // C s_27_5: const #1u : u64
        let s_27_5: u64 = 1;
        // D s_27_6: bit-extract s_27_4 s_27_3 s_27_5
        let s_27_6: Bits = (Bits::new(
            ((s_27_4) >> (s_27_3)).value(),
            u16::try_from(s_27_5).unwrap(),
        ));
        // D s_27_7: cast reint s_27_6 -> u8
        let s_27_7: bool = ((s_27_6.value()) != 0);
        // C s_27_8: const #0s : i
        let s_27_8: i128 = 0;
        // C s_27_9: const #0u : u64
        let s_27_9: u64 = 0;
        // D s_27_10: cast zx s_27_7 -> u64
        let s_27_10: u64 = (s_27_7 as u64);
        // C s_27_11: const #1u : u64
        let s_27_11: u64 = 1;
        // D s_27_12: and s_27_10 s_27_11
        let s_27_12: u64 = ((s_27_10) & (s_27_11));
        // D s_27_13: cmp-eq s_27_12 s_27_11
        let s_27_13: bool = ((s_27_12) == (s_27_11));
        // D s_27_14: lsl s_27_10 s_27_8
        let s_27_14: u64 = s_27_10 << s_27_8;
        // D s_27_15: or s_27_9 s_27_14
        let s_27_15: u64 = ((s_27_9) | (s_27_14));
        // D s_27_16: cmpl s_27_14
        let s_27_16: u64 = !s_27_14;
        // D s_27_17: and s_27_9 s_27_16
        let s_27_17: u64 = ((s_27_9) & (s_27_16));
        // D s_27_18: select s_27_13 s_27_15 s_27_17
        let s_27_18: u64 = if s_27_13 { s_27_15 } else { s_27_17 };
        // D s_27_19: cast trunc s_27_18 -> u8
        let s_27_19: bool = ((s_27_18) != 0);
        // D s_27_20: cast zx s_27_19 -> bv
        let s_27_20: Bits = Bits::new(s_27_19 as u128, 1u16);
        // C s_27_21: const #0u : u8
        let s_27_21: bool = false;
        // C s_27_22: cast zx s_27_21 -> bv
        let s_27_22: Bits = Bits::new(s_27_21 as u128, 1u16);
        // D s_27_23: cmp-eq s_27_20 s_27_22
        let s_27_23: bool = ((s_27_20) == (s_27_22));
        // N s_27_24: branch s_27_23 b36 b28
        if s_27_23 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #22712u : u32
        let s_28_0: u32 = 22712;
        // D s_28_1: read-reg s_28_0:struct
        let s_28_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_28_0 as isize);
            tracer.read_register(s_28_0 as isize, value);
            value
        };
        // D s_28_2: call _get_MDCR_EL3_Type_NSPB(s_28_1)
        let s_28_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_28_1);
        // C s_28_3: const #1s : i
        let s_28_3: i128 = 1;
        // D s_28_4: cast zx s_28_2 -> bv
        let s_28_4: Bits = Bits::new(s_28_2 as u128, 2u16);
        // C s_28_5: const #1u : u64
        let s_28_5: u64 = 1;
        // D s_28_6: bit-extract s_28_4 s_28_3 s_28_5
        let s_28_6: Bits = (Bits::new(
            ((s_28_4) >> (s_28_3)).value(),
            u16::try_from(s_28_5).unwrap(),
        ));
        // D s_28_7: cast reint s_28_6 -> u8
        let s_28_7: bool = ((s_28_6.value()) != 0);
        // C s_28_8: const #0s : i
        let s_28_8: i128 = 0;
        // C s_28_9: const #0u : u64
        let s_28_9: u64 = 0;
        // D s_28_10: cast zx s_28_7 -> u64
        let s_28_10: u64 = (s_28_7 as u64);
        // C s_28_11: const #1u : u64
        let s_28_11: u64 = 1;
        // D s_28_12: and s_28_10 s_28_11
        let s_28_12: u64 = ((s_28_10) & (s_28_11));
        // D s_28_13: cmp-eq s_28_12 s_28_11
        let s_28_13: bool = ((s_28_12) == (s_28_11));
        // D s_28_14: lsl s_28_10 s_28_8
        let s_28_14: u64 = s_28_10 << s_28_8;
        // D s_28_15: or s_28_9 s_28_14
        let s_28_15: u64 = ((s_28_9) | (s_28_14));
        // D s_28_16: cmpl s_28_14
        let s_28_16: u64 = !s_28_14;
        // D s_28_17: and s_28_9 s_28_16
        let s_28_17: u64 = ((s_28_9) & (s_28_16));
        // D s_28_18: select s_28_13 s_28_15 s_28_17
        let s_28_18: u64 = if s_28_13 { s_28_15 } else { s_28_17 };
        // D s_28_19: cast trunc s_28_18 -> u8
        let s_28_19: bool = ((s_28_18) != 0);
        // C s_28_20: const #90704u : u32
        let s_28_20: u32 = 90704;
        // D s_28_21: read-reg s_28_20:struct
        let s_28_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_28_20 as isize);
            tracer.read_register(s_28_20 as isize, value);
            value
        };
        // D s_28_22: call _get_SCR_EL3_Type_NS(s_28_21)
        let s_28_22: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_28_21);
        // D s_28_23: cast zx s_28_19 -> bv
        let s_28_23: Bits = Bits::new(s_28_19 as u128, 1u16);
        // D s_28_24: cast zx s_28_22 -> bv
        let s_28_24: Bits = Bits::new(s_28_22 as u128, 1u16);
        // D s_28_25: cmp-ne s_28_23 s_28_24
        let s_28_25: bool = ((s_28_23) != (s_28_24));
        // D s_28_26: write-var gs#90844 <= s_28_25
        fn_state.gs_90844 = s_28_25;
        // N s_28_27: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#90844:u8
        let s_29_0: bool = fn_state.gs_90844;
        // N s_29_1: branch s_29_0 b35 b30
        if s_29_0 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #232u : u32
        let s_30_0: u32 = 232;
        // S s_30_1: call IsFeatureImplemented(s_30_0)
        let s_30_1: bool = IsFeatureImplemented(state, tracer, s_30_0);
        // N s_30_2: branch s_30_1 b34 b31
        if s_30_1 {
            return block_34(state, tracer, fn_state);
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
        // D s_31_1: write-var gs#90845 <= s_31_0
        fn_state.gs_90845 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#90845:u8
        let s_32_0: bool = fn_state.gs_90845;
        // D s_32_1: write-var gs#90846 <= s_32_0
        fn_state.gs_90846 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#90846:u8
        let s_33_0: bool = fn_state.gs_90846;
        // D s_33_1: write-var gs#90847 <= s_33_0
        fn_state.gs_90847 = s_33_0;
        // N s_33_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var __MDCR_EL3_NSPBE:u8
        let s_34_0: bool = fn_state.u__MDCR_EL3_NSPBE;
        // D s_34_1: cast zx s_34_0 -> bv
        let s_34_1: Bits = Bits::new(s_34_0 as u128, 1u16);
        // D s_34_2: read-var __SCR_EL3_NSE:u8
        let s_34_2: bool = fn_state.u__SCR_EL3_NSE;
        // D s_34_3: cast zx s_34_2 -> bv
        let s_34_3: Bits = Bits::new(s_34_2 as u128, 1u16);
        // D s_34_4: cmp-ne s_34_1 s_34_3
        let s_34_4: bool = ((s_34_1) != (s_34_3));
        // D s_34_5: write-var gs#90845 <= s_34_4
        fn_state.gs_90845 = s_34_4;
        // N s_34_6: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #1u : u8
        let s_35_0: bool = true;
        // D s_35_1: write-var gs#90846 <= s_35_0
        fn_state.gs_90846 = s_35_0;
        // N s_35_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #1u : u8
        let s_36_0: bool = true;
        // D s_36_1: write-var gs#90844 <= s_36_0
        fn_state.gs_90844 = s_36_0;
        // N s_36_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_37_0: panic
        panic!("{:?}", ());
        // N s_37_1: return
        return;
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #22712u : u32
        let s_38_0: u32 = 22712;
        // D s_38_1: read-reg s_38_0:struct
        let s_38_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_38_0 as isize);
            tracer.read_register(s_38_0 as isize, value);
            value
        };
        // D s_38_2: call _get_MDCR_EL3_Type_NSPB(s_38_1)
        let s_38_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_38_1);
        // C s_38_3: const #0s : i
        let s_38_3: i128 = 0;
        // D s_38_4: cast zx s_38_2 -> bv
        let s_38_4: Bits = Bits::new(s_38_2 as u128, 2u16);
        // C s_38_5: const #1u : u64
        let s_38_5: u64 = 1;
        // D s_38_6: bit-extract s_38_4 s_38_3 s_38_5
        let s_38_6: Bits = (Bits::new(
            ((s_38_4) >> (s_38_3)).value(),
            u16::try_from(s_38_5).unwrap(),
        ));
        // D s_38_7: cast reint s_38_6 -> u8
        let s_38_7: bool = ((s_38_6.value()) != 0);
        // C s_38_8: const #0s : i
        let s_38_8: i128 = 0;
        // C s_38_9: const #0u : u64
        let s_38_9: u64 = 0;
        // D s_38_10: cast zx s_38_7 -> u64
        let s_38_10: u64 = (s_38_7 as u64);
        // C s_38_11: const #1u : u64
        let s_38_11: u64 = 1;
        // D s_38_12: and s_38_10 s_38_11
        let s_38_12: u64 = ((s_38_10) & (s_38_11));
        // D s_38_13: cmp-eq s_38_12 s_38_11
        let s_38_13: bool = ((s_38_12) == (s_38_11));
        // D s_38_14: lsl s_38_10 s_38_8
        let s_38_14: u64 = s_38_10 << s_38_8;
        // D s_38_15: or s_38_9 s_38_14
        let s_38_15: u64 = ((s_38_9) | (s_38_14));
        // D s_38_16: cmpl s_38_14
        let s_38_16: u64 = !s_38_14;
        // D s_38_17: and s_38_9 s_38_16
        let s_38_17: u64 = ((s_38_9) & (s_38_16));
        // D s_38_18: select s_38_13 s_38_15 s_38_17
        let s_38_18: u64 = if s_38_13 { s_38_15 } else { s_38_17 };
        // D s_38_19: cast trunc s_38_18 -> u8
        let s_38_19: bool = ((s_38_18) != 0);
        // D s_38_20: cast zx s_38_19 -> bv
        let s_38_20: Bits = Bits::new(s_38_19 as u128, 1u16);
        // C s_38_21: const #0u : u8
        let s_38_21: bool = false;
        // C s_38_22: cast zx s_38_21 -> bv
        let s_38_22: Bits = Bits::new(s_38_21 as u128, 1u16);
        // D s_38_23: cmp-eq s_38_20 s_38_22
        let s_38_23: bool = ((s_38_20) == (s_38_22));
        // N s_38_24: branch s_38_23 b47 b39
        if s_38_23 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #22712u : u32
        let s_39_0: u32 = 22712;
        // D s_39_1: read-reg s_39_0:struct
        let s_39_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_39_0 as isize);
            tracer.read_register(s_39_0 as isize, value);
            value
        };
        // D s_39_2: call _get_MDCR_EL3_Type_NSPB(s_39_1)
        let s_39_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_39_1);
        // C s_39_3: const #1s : i
        let s_39_3: i128 = 1;
        // D s_39_4: cast zx s_39_2 -> bv
        let s_39_4: Bits = Bits::new(s_39_2 as u128, 2u16);
        // C s_39_5: const #1u : u64
        let s_39_5: u64 = 1;
        // D s_39_6: bit-extract s_39_4 s_39_3 s_39_5
        let s_39_6: Bits = (Bits::new(
            ((s_39_4) >> (s_39_3)).value(),
            u16::try_from(s_39_5).unwrap(),
        ));
        // D s_39_7: cast reint s_39_6 -> u8
        let s_39_7: bool = ((s_39_6.value()) != 0);
        // C s_39_8: const #0s : i
        let s_39_8: i128 = 0;
        // C s_39_9: const #0u : u64
        let s_39_9: u64 = 0;
        // D s_39_10: cast zx s_39_7 -> u64
        let s_39_10: u64 = (s_39_7 as u64);
        // C s_39_11: const #1u : u64
        let s_39_11: u64 = 1;
        // D s_39_12: and s_39_10 s_39_11
        let s_39_12: u64 = ((s_39_10) & (s_39_11));
        // D s_39_13: cmp-eq s_39_12 s_39_11
        let s_39_13: bool = ((s_39_12) == (s_39_11));
        // D s_39_14: lsl s_39_10 s_39_8
        let s_39_14: u64 = s_39_10 << s_39_8;
        // D s_39_15: or s_39_9 s_39_14
        let s_39_15: u64 = ((s_39_9) | (s_39_14));
        // D s_39_16: cmpl s_39_14
        let s_39_16: u64 = !s_39_14;
        // D s_39_17: and s_39_9 s_39_16
        let s_39_17: u64 = ((s_39_9) & (s_39_16));
        // D s_39_18: select s_39_13 s_39_15 s_39_17
        let s_39_18: u64 = if s_39_13 { s_39_15 } else { s_39_17 };
        // D s_39_19: cast trunc s_39_18 -> u8
        let s_39_19: bool = ((s_39_18) != 0);
        // C s_39_20: const #90704u : u32
        let s_39_20: u32 = 90704;
        // D s_39_21: read-reg s_39_20:struct
        let s_39_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_39_20 as isize);
            tracer.read_register(s_39_20 as isize, value);
            value
        };
        // D s_39_22: call _get_SCR_EL3_Type_NS(s_39_21)
        let s_39_22: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_39_21);
        // D s_39_23: cast zx s_39_19 -> bv
        let s_39_23: Bits = Bits::new(s_39_19 as u128, 1u16);
        // D s_39_24: cast zx s_39_22 -> bv
        let s_39_24: Bits = Bits::new(s_39_22 as u128, 1u16);
        // D s_39_25: cmp-ne s_39_23 s_39_24
        let s_39_25: bool = ((s_39_23) != (s_39_24));
        // D s_39_26: write-var gs#90836 <= s_39_25
        fn_state.gs_90836 = s_39_25;
        // N s_39_27: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#90836:u8
        let s_40_0: bool = fn_state.gs_90836;
        // N s_40_1: branch s_40_0 b46 b41
        if s_40_0 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #232u : u32
        let s_41_0: u32 = 232;
        // S s_41_1: call IsFeatureImplemented(s_41_0)
        let s_41_1: bool = IsFeatureImplemented(state, tracer, s_41_0);
        // N s_41_2: branch s_41_1 b45 b42
        if s_41_1 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #0u : u8
        let s_42_0: bool = false;
        // D s_42_1: write-var gs#90837 <= s_42_0
        fn_state.gs_90837 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#90837:u8
        let s_43_0: bool = fn_state.gs_90837;
        // D s_43_1: write-var gs#90838 <= s_43_0
        fn_state.gs_90838 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#90838:u8
        let s_44_0: bool = fn_state.gs_90838;
        // D s_44_1: write-var gs#90839 <= s_44_0
        fn_state.gs_90839 = s_44_0;
        // N s_44_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var __MDCR_EL3_NSPBE:u8
        let s_45_0: bool = fn_state.u__MDCR_EL3_NSPBE;
        // D s_45_1: cast zx s_45_0 -> bv
        let s_45_1: Bits = Bits::new(s_45_0 as u128, 1u16);
        // D s_45_2: read-var __SCR_EL3_NSE:u8
        let s_45_2: bool = fn_state.u__SCR_EL3_NSE;
        // D s_45_3: cast zx s_45_2 -> bv
        let s_45_3: Bits = Bits::new(s_45_2 as u128, 1u16);
        // D s_45_4: cmp-ne s_45_1 s_45_3
        let s_45_4: bool = ((s_45_1) != (s_45_3));
        // D s_45_5: write-var gs#90837 <= s_45_4
        fn_state.gs_90837 = s_45_4;
        // N s_45_6: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #1u : u8
        let s_46_0: bool = true;
        // D s_46_1: write-var gs#90838 <= s_46_0
        fn_state.gs_90838 = s_46_0;
        // N s_46_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #1u : u8
        let s_47_0: bool = true;
        // D s_47_1: write-var gs#90836 <= s_47_0
        fn_state.gs_90836 = s_47_0;
        // N s_47_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_48_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_48_1: call __IMPDEF_boolean(s_48_0)
        let s_48_1: bool = u__IMPDEF_boolean(state, tracer, s_48_0);
        // D s_48_2: write-var gs#90831 <= s_48_1
        fn_state.gs_90831 = s_48_1;
        // N s_48_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #() : ()
        let s_49_0: () = ();
        // S s_49_1: call EDSCR_read(s_49_0)
        let s_49_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_49_0);
        // S s_49_2: call _get_EDSCR_Type_SDD(s_49_1)
        let s_49_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_49_1);
        // S s_49_3: cast zx s_49_2 -> bv
        let s_49_3: Bits = Bits::new(s_49_2 as u128, 1u16);
        // C s_49_4: const #1u : u8
        let s_49_4: bool = true;
        // C s_49_5: cast zx s_49_4 -> bv
        let s_49_5: Bits = Bits::new(s_49_4 as u128, 1u16);
        // S s_49_6: cmp-eq s_49_3 s_49_5
        let s_49_6: bool = ((s_49_3) == (s_49_5));
        // D s_49_7: write-var gs#90830 <= s_49_6
        fn_state.gs_90830 = s_49_6;
        // N s_49_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #424u : u32
        let s_50_0: u32 = 424;
        // D s_50_1: read-reg s_50_0:u8
        let s_50_1: u8 = {
            let value = state.read_register::<u8>(s_50_0 as isize);
            tracer.read_register(s_50_0 as isize, value);
            value
        };
        // C s_50_2: const #2u : u8
        let s_50_2: u8 = 2;
        // D s_50_3: cmp-lt s_50_1 s_50_2
        let s_50_3: bool = ((s_50_1) < (s_50_2));
        // D s_50_4: write-var gs#90829 <= s_50_3
        fn_state.gs_90829 = s_50_3;
        // N s_50_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #() : ()
        let s_51_0: () = ();
        // S s_51_1: call Halted(s_51_0)
        let s_51_1: bool = Halted(state, tracer, s_51_0);
        // N s_51_2: branch s_51_1 b117 b52
        if s_51_1 {
            return block_117(state, tracer, fn_state);
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
        // D s_52_1: write-var gs#90851 <= s_52_0
        fn_state.gs_90851 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#90851:u8
        let s_53_0: bool = fn_state.gs_90851;
        // N s_53_1: branch s_53_0 b116 b54
        if s_53_0 {
            return block_116(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #0u : u8
        let s_54_0: bool = false;
        // D s_54_1: write-var gs#90852 <= s_54_0
        fn_state.gs_90852 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#90852:u8
        let s_55_0: bool = fn_state.gs_90852;
        // N s_55_1: branch s_55_0 b115 b56
        if s_55_0 {
            return block_115(state, tracer, fn_state);
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
        // D s_56_1: write-var gs#90853 <= s_56_0
        fn_state.gs_90853 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#90853:u8
        let s_57_0: bool = fn_state.gs_90853;
        // N s_57_1: branch s_57_0 b105 b58
        if s_57_0 {
            return block_105(state, tracer, fn_state);
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
        // D s_58_1: write-var gs#90861 <= s_58_0
        fn_state.gs_90861 = s_58_0;
        // N s_58_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var gs#90861:u8
        let s_59_0: bool = fn_state.gs_90861;
        // N s_59_1: branch s_59_0 b104 b60
        if s_59_0 {
            return block_104(state, tracer, fn_state);
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
        // N s_60_2: branch s_60_1 b103 b61
        if s_60_1 {
            return block_103(state, tracer, fn_state);
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
        // D s_61_1: write-var gs#90862 <= s_61_0
        fn_state.gs_90862 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#90862:u8
        let s_62_0: bool = fn_state.gs_90862;
        // N s_62_1: branch s_62_0 b99 b63
        if s_62_0 {
            return block_99(state, tracer, fn_state);
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
        // D s_63_1: write-var gs#90864 <= s_63_0
        fn_state.gs_90864 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#90864:u8
        let s_64_0: bool = fn_state.gs_90864;
        // N s_64_1: branch s_64_0 b98 b65
        if s_64_0 {
            return block_98(state, tracer, fn_state);
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
        // D s_65_1: write-var gs#90865 <= s_65_0
        fn_state.gs_90865 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#90865:u8
        let s_66_0: bool = fn_state.gs_90865;
        // N s_66_1: branch s_66_0 b97 b67
        if s_66_0 {
            return block_97(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #() : ()
        let s_67_0: () = ();
        // S s_67_1: call EL2Enabled(s_67_0)
        let s_67_1: bool = EL2Enabled(state, tracer, s_67_0);
        // N s_67_2: branch s_67_1 b96 b68
        if s_67_1 {
            return block_96(state, tracer, fn_state);
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
        // D s_68_1: write-var gs#90866 <= s_68_0
        fn_state.gs_90866 = s_68_0;
        // N s_68_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var gs#90866:u8
        let s_69_0: bool = fn_state.gs_90866;
        // N s_69_1: branch s_69_0 b95 b70
        if s_69_0 {
            return block_95(state, tracer, fn_state);
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
        // D s_71_1: write-var gs#90874 <= s_71_0
        fn_state.gs_90874 = s_71_0;
        // N s_71_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var gs#90874:u8
        let s_72_0: bool = fn_state.gs_90874;
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
        // D s_74_1: write-var gs#90875 <= s_74_0
        fn_state.gs_90875 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var gs#90875:u8
        let s_75_0: bool = fn_state.gs_90875;
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
        // D s_76_4: call Mk_PMSCR_EL1_Type(s_76_3)
        let s_76_4: ProductType5c790c8ef59cc8b2 = Mk_PMSCR_EL1_Type(
            state,
            tracer,
            s_76_3,
        );
        // C s_76_5: const #21072u : u32
        let s_76_5: u32 = 21072;
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
        // C s_77_0: const #2088u : u12
        let s_77_0: u16 = 2088;
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
        // D s_78_35: write-var gs#90875 <= s_78_34
        fn_state.gs_90875 = s_78_34;
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
        // D s_80_1: write-var gs#90878 <= s_80_0
        fn_state.gs_90878 = s_80_0;
        // N s_80_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#90878:u8
        let s_81_0: bool = fn_state.gs_90878;
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
        // C s_82_0: const #24u : u8
        let s_82_0: u8 = 24;
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
        // C s_84_0: const #() : ()
        let s_84_0: () = ();
        // S s_84_1: call EDSCR_read(s_84_0)
        let s_84_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_84_0);
        // S s_84_2: call _get_EDSCR_Type_SDD(s_84_1)
        let s_84_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_84_1);
        // S s_84_3: cast zx s_84_2 -> bv
        let s_84_3: Bits = Bits::new(s_84_2 as u128, 1u16);
        // C s_84_4: const #1u : u8
        let s_84_4: bool = true;
        // C s_84_5: cast zx s_84_4 -> bv
        let s_84_5: Bits = Bits::new(s_84_4 as u128, 1u16);
        // S s_84_6: cmp-eq s_84_3 s_84_5
        let s_84_6: bool = ((s_84_3) == (s_84_5));
        // D s_84_7: write-var gs#90878 <= s_84_6
        fn_state.gs_90878 = s_84_6;
        // N s_84_8: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #22712u : u32
        let s_85_0: u32 = 22712;
        // D s_85_1: read-reg s_85_0:struct
        let s_85_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_85_0 as isize);
            tracer.read_register(s_85_0 as isize, value);
            value
        };
        // D s_85_2: call _get_MDCR_EL3_Type_NSPB(s_85_1)
        let s_85_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_85_1);
        // C s_85_3: const #0s : i
        let s_85_3: i128 = 0;
        // D s_85_4: cast zx s_85_2 -> bv
        let s_85_4: Bits = Bits::new(s_85_2 as u128, 2u16);
        // C s_85_5: const #1u : u64
        let s_85_5: u64 = 1;
        // D s_85_6: bit-extract s_85_4 s_85_3 s_85_5
        let s_85_6: Bits = (Bits::new(
            ((s_85_4) >> (s_85_3)).value(),
            u16::try_from(s_85_5).unwrap(),
        ));
        // D s_85_7: cast reint s_85_6 -> u8
        let s_85_7: bool = ((s_85_6.value()) != 0);
        // C s_85_8: const #0s : i
        let s_85_8: i128 = 0;
        // C s_85_9: const #0u : u64
        let s_85_9: u64 = 0;
        // D s_85_10: cast zx s_85_7 -> u64
        let s_85_10: u64 = (s_85_7 as u64);
        // C s_85_11: const #1u : u64
        let s_85_11: u64 = 1;
        // D s_85_12: and s_85_10 s_85_11
        let s_85_12: u64 = ((s_85_10) & (s_85_11));
        // D s_85_13: cmp-eq s_85_12 s_85_11
        let s_85_13: bool = ((s_85_12) == (s_85_11));
        // D s_85_14: lsl s_85_10 s_85_8
        let s_85_14: u64 = s_85_10 << s_85_8;
        // D s_85_15: or s_85_9 s_85_14
        let s_85_15: u64 = ((s_85_9) | (s_85_14));
        // D s_85_16: cmpl s_85_14
        let s_85_16: u64 = !s_85_14;
        // D s_85_17: and s_85_9 s_85_16
        let s_85_17: u64 = ((s_85_9) & (s_85_16));
        // D s_85_18: select s_85_13 s_85_15 s_85_17
        let s_85_18: u64 = if s_85_13 { s_85_15 } else { s_85_17 };
        // D s_85_19: cast trunc s_85_18 -> u8
        let s_85_19: bool = ((s_85_18) != 0);
        // D s_85_20: cast zx s_85_19 -> bv
        let s_85_20: Bits = Bits::new(s_85_19 as u128, 1u16);
        // C s_85_21: const #0u : u8
        let s_85_21: bool = false;
        // C s_85_22: cast zx s_85_21 -> bv
        let s_85_22: Bits = Bits::new(s_85_21 as u128, 1u16);
        // D s_85_23: cmp-eq s_85_20 s_85_22
        let s_85_23: bool = ((s_85_20) == (s_85_22));
        // N s_85_24: branch s_85_23 b94 b86
        if s_85_23 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_86(state, tracer, fn_state);
        };
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #22712u : u32
        let s_86_0: u32 = 22712;
        // D s_86_1: read-reg s_86_0:struct
        let s_86_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_86_0 as isize);
            tracer.read_register(s_86_0 as isize, value);
            value
        };
        // D s_86_2: call _get_MDCR_EL3_Type_NSPB(s_86_1)
        let s_86_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_86_1);
        // C s_86_3: const #1s : i
        let s_86_3: i128 = 1;
        // D s_86_4: cast zx s_86_2 -> bv
        let s_86_4: Bits = Bits::new(s_86_2 as u128, 2u16);
        // C s_86_5: const #1u : u64
        let s_86_5: u64 = 1;
        // D s_86_6: bit-extract s_86_4 s_86_3 s_86_5
        let s_86_6: Bits = (Bits::new(
            ((s_86_4) >> (s_86_3)).value(),
            u16::try_from(s_86_5).unwrap(),
        ));
        // D s_86_7: cast reint s_86_6 -> u8
        let s_86_7: bool = ((s_86_6.value()) != 0);
        // C s_86_8: const #0s : i
        let s_86_8: i128 = 0;
        // C s_86_9: const #0u : u64
        let s_86_9: u64 = 0;
        // D s_86_10: cast zx s_86_7 -> u64
        let s_86_10: u64 = (s_86_7 as u64);
        // C s_86_11: const #1u : u64
        let s_86_11: u64 = 1;
        // D s_86_12: and s_86_10 s_86_11
        let s_86_12: u64 = ((s_86_10) & (s_86_11));
        // D s_86_13: cmp-eq s_86_12 s_86_11
        let s_86_13: bool = ((s_86_12) == (s_86_11));
        // D s_86_14: lsl s_86_10 s_86_8
        let s_86_14: u64 = s_86_10 << s_86_8;
        // D s_86_15: or s_86_9 s_86_14
        let s_86_15: u64 = ((s_86_9) | (s_86_14));
        // D s_86_16: cmpl s_86_14
        let s_86_16: u64 = !s_86_14;
        // D s_86_17: and s_86_9 s_86_16
        let s_86_17: u64 = ((s_86_9) & (s_86_16));
        // D s_86_18: select s_86_13 s_86_15 s_86_17
        let s_86_18: u64 = if s_86_13 { s_86_15 } else { s_86_17 };
        // D s_86_19: cast trunc s_86_18 -> u8
        let s_86_19: bool = ((s_86_18) != 0);
        // C s_86_20: const #90704u : u32
        let s_86_20: u32 = 90704;
        // D s_86_21: read-reg s_86_20:struct
        let s_86_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_86_20 as isize);
            tracer.read_register(s_86_20 as isize, value);
            value
        };
        // D s_86_22: call _get_SCR_EL3_Type_NS(s_86_21)
        let s_86_22: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_86_21);
        // D s_86_23: cast zx s_86_19 -> bv
        let s_86_23: Bits = Bits::new(s_86_19 as u128, 1u16);
        // D s_86_24: cast zx s_86_22 -> bv
        let s_86_24: Bits = Bits::new(s_86_22 as u128, 1u16);
        // D s_86_25: cmp-ne s_86_23 s_86_24
        let s_86_25: bool = ((s_86_23) != (s_86_24));
        // D s_86_26: write-var gs#90871 <= s_86_25
        fn_state.gs_90871 = s_86_25;
        // N s_86_27: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var gs#90871:u8
        let s_87_0: bool = fn_state.gs_90871;
        // N s_87_1: branch s_87_0 b93 b88
        if s_87_0 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_88(state, tracer, fn_state);
        };
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #232u : u32
        let s_88_0: u32 = 232;
        // S s_88_1: call IsFeatureImplemented(s_88_0)
        let s_88_1: bool = IsFeatureImplemented(state, tracer, s_88_0);
        // N s_88_2: branch s_88_1 b92 b89
        if s_88_1 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_89(state, tracer, fn_state);
        };
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #0u : u8
        let s_89_0: bool = false;
        // D s_89_1: write-var gs#90872 <= s_89_0
        fn_state.gs_90872 = s_89_0;
        // N s_89_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var gs#90872:u8
        let s_90_0: bool = fn_state.gs_90872;
        // D s_90_1: write-var gs#90873 <= s_90_0
        fn_state.gs_90873 = s_90_0;
        // N s_90_2: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_91_0: read-var gs#90873:u8
        let s_91_0: bool = fn_state.gs_90873;
        // D s_91_1: write-var gs#90874 <= s_91_0
        fn_state.gs_90874 = s_91_0;
        // N s_91_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var __MDCR_EL3_NSPBE:u8
        let s_92_0: bool = fn_state.u__MDCR_EL3_NSPBE;
        // D s_92_1: cast zx s_92_0 -> bv
        let s_92_1: Bits = Bits::new(s_92_0 as u128, 1u16);
        // D s_92_2: read-var __SCR_EL3_NSE:u8
        let s_92_2: bool = fn_state.u__SCR_EL3_NSE;
        // D s_92_3: cast zx s_92_2 -> bv
        let s_92_3: Bits = Bits::new(s_92_2 as u128, 1u16);
        // D s_92_4: cmp-ne s_92_1 s_92_3
        let s_92_4: bool = ((s_92_1) != (s_92_3));
        // D s_92_5: write-var gs#90872 <= s_92_4
        fn_state.gs_90872 = s_92_4;
        // N s_92_6: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #1u : u8
        let s_93_0: bool = true;
        // D s_93_1: write-var gs#90873 <= s_93_0
        fn_state.gs_90873 = s_93_0;
        // N s_93_2: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #1u : u8
        let s_94_0: bool = true;
        // D s_94_1: write-var gs#90871 <= s_94_0
        fn_state.gs_90871 = s_94_0;
        // N s_94_2: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #24u : u8
        let s_95_0: u8 = 24;
        // C s_95_1: cast zx s_95_0 -> bv
        let s_95_1: Bits = Bits::new(s_95_0 as u128, 8u16);
        // C s_95_2: cast zx s_95_1 -> i
        let s_95_2: i128 = (s_95_1.value() as i128);
        // C s_95_3: cast reint s_95_2 -> i64
        let s_95_3: i64 = (s_95_2 as i64);
        // C s_95_4: cast zx s_95_3 -> i
        let s_95_4: i128 = (i128::try_from(s_95_3).unwrap());
        // C s_95_5: const #432u : u32
        let s_95_5: u32 = 432;
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
        // D s_96_0: read-var __MDCR_EL2_TPMS:u8
        let s_96_0: bool = fn_state.u__MDCR_EL2_TPMS;
        // D s_96_1: cast zx s_96_0 -> bv
        let s_96_1: Bits = Bits::new(s_96_0 as u128, 1u16);
        // C s_96_2: const #1u : u8
        let s_96_2: bool = true;
        // C s_96_3: cast zx s_96_2 -> bv
        let s_96_3: Bits = Bits::new(s_96_2 as u128, 1u16);
        // D s_96_4: cmp-eq s_96_1 s_96_3
        let s_96_4: bool = ((s_96_1) == (s_96_3));
        // D s_96_5: write-var gs#90866 <= s_96_4
        fn_state.gs_90866 = s_96_4;
        // N s_96_6: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #24u : u8
        let s_97_0: u8 = 24;
        // C s_97_1: cast zx s_97_0 -> bv
        let s_97_1: Bits = Bits::new(s_97_0 as u128, 8u16);
        // C s_97_2: cast zx s_97_1 -> i
        let s_97_2: i128 = (s_97_1.value() as i128);
        // C s_97_3: cast reint s_97_2 -> i64
        let s_97_3: i64 = (s_97_2 as i64);
        // C s_97_4: cast zx s_97_3 -> i
        let s_97_4: i128 = (i128::try_from(s_97_3).unwrap());
        // C s_97_5: const #432u : u32
        let s_97_5: u32 = 432;
        // D s_97_6: read-reg s_97_5:u8
        let s_97_6: u8 = {
            let value = state.read_register::<u8>(s_97_5 as isize);
            tracer.read_register(s_97_5 as isize, value);
            value
        };
        // D s_97_7: call AArch64_SystemAccessTrap(s_97_6, s_97_4)
        let s_97_7: () = AArch64_SystemAccessTrap(state, tracer, s_97_6, s_97_4);
        // N s_97_8: return
        return;
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var __HDFGWTR_EL2_PMSCR_EL1:u8
        let s_98_0: bool = fn_state.u__HDFGWTR_EL2_PMSCR_EL1;
        // D s_98_1: cast zx s_98_0 -> bv
        let s_98_1: Bits = Bits::new(s_98_0 as u128, 1u16);
        // C s_98_2: const #1u : u8
        let s_98_2: bool = true;
        // C s_98_3: cast zx s_98_2 -> bv
        let s_98_3: Bits = Bits::new(s_98_2 as u128, 1u16);
        // D s_98_4: cmp-eq s_98_1 s_98_3
        let s_98_4: bool = ((s_98_1) == (s_98_3));
        // D s_98_5: write-var gs#90865 <= s_98_4
        fn_state.gs_90865 = s_98_4;
        // N s_98_6: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #424u : u32
        let s_99_0: u32 = 424;
        // D s_99_1: read-reg s_99_0:u8
        let s_99_1: u8 = {
            let value = state.read_register::<u8>(s_99_0 as isize);
            tracer.read_register(s_99_0 as isize, value);
            value
        };
        // C s_99_2: const #2u : u8
        let s_99_2: u8 = 2;
        // D s_99_3: cmp-lt s_99_1 s_99_2
        let s_99_3: bool = ((s_99_1) < (s_99_2));
        // D s_99_4: not s_99_3
        let s_99_4: bool = !s_99_3;
        // N s_99_5: branch s_99_4 b102 b100
        if s_99_4 {
            return block_102(state, tracer, fn_state);
        } else {
            return block_100(state, tracer, fn_state);
        };
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_100_0: read-var __SCR_EL3_FGTEn:u8
        let s_100_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_100_1: cast zx s_100_0 -> bv
        let s_100_1: Bits = Bits::new(s_100_0 as u128, 1u16);
        // C s_100_2: const #1u : u8
        let s_100_2: bool = true;
        // C s_100_3: cast zx s_100_2 -> bv
        let s_100_3: Bits = Bits::new(s_100_2 as u128, 1u16);
        // D s_100_4: cmp-eq s_100_1 s_100_3
        let s_100_4: bool = ((s_100_1) == (s_100_3));
        // D s_100_5: write-var gs#90863 <= s_100_4
        fn_state.gs_90863 = s_100_4;
        // N s_100_6: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_101_0: read-var gs#90863:u8
        let s_101_0: bool = fn_state.gs_90863;
        // D s_101_1: write-var gs#90864 <= s_101_0
        fn_state.gs_90864 = s_101_0;
        // N s_101_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #1u : u8
        let s_102_0: bool = true;
        // D s_102_1: write-var gs#90863 <= s_102_0
        fn_state.gs_90863 = s_102_0;
        // N s_102_2: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #146u : u32
        let s_103_0: u32 = 146;
        // S s_103_1: call IsFeatureImplemented(s_103_0)
        let s_103_1: bool = IsFeatureImplemented(state, tracer, s_103_0);
        // D s_103_2: write-var gs#90862 <= s_103_1
        fn_state.gs_90862 = s_103_1;
        // N s_103_3: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_104_0: panic
        panic!("{:?}", ());
        // N s_104_1: return
        return;
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #22712u : u32
        let s_105_0: u32 = 22712;
        // D s_105_1: read-reg s_105_0:struct
        let s_105_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_105_0 as isize);
            tracer.read_register(s_105_0 as isize, value);
            value
        };
        // D s_105_2: call _get_MDCR_EL3_Type_NSPB(s_105_1)
        let s_105_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_105_1);
        // C s_105_3: const #0s : i
        let s_105_3: i128 = 0;
        // D s_105_4: cast zx s_105_2 -> bv
        let s_105_4: Bits = Bits::new(s_105_2 as u128, 2u16);
        // C s_105_5: const #1u : u64
        let s_105_5: u64 = 1;
        // D s_105_6: bit-extract s_105_4 s_105_3 s_105_5
        let s_105_6: Bits = (Bits::new(
            ((s_105_4) >> (s_105_3)).value(),
            u16::try_from(s_105_5).unwrap(),
        ));
        // D s_105_7: cast reint s_105_6 -> u8
        let s_105_7: bool = ((s_105_6.value()) != 0);
        // C s_105_8: const #0s : i
        let s_105_8: i128 = 0;
        // C s_105_9: const #0u : u64
        let s_105_9: u64 = 0;
        // D s_105_10: cast zx s_105_7 -> u64
        let s_105_10: u64 = (s_105_7 as u64);
        // C s_105_11: const #1u : u64
        let s_105_11: u64 = 1;
        // D s_105_12: and s_105_10 s_105_11
        let s_105_12: u64 = ((s_105_10) & (s_105_11));
        // D s_105_13: cmp-eq s_105_12 s_105_11
        let s_105_13: bool = ((s_105_12) == (s_105_11));
        // D s_105_14: lsl s_105_10 s_105_8
        let s_105_14: u64 = s_105_10 << s_105_8;
        // D s_105_15: or s_105_9 s_105_14
        let s_105_15: u64 = ((s_105_9) | (s_105_14));
        // D s_105_16: cmpl s_105_14
        let s_105_16: u64 = !s_105_14;
        // D s_105_17: and s_105_9 s_105_16
        let s_105_17: u64 = ((s_105_9) & (s_105_16));
        // D s_105_18: select s_105_13 s_105_15 s_105_17
        let s_105_18: u64 = if s_105_13 { s_105_15 } else { s_105_17 };
        // D s_105_19: cast trunc s_105_18 -> u8
        let s_105_19: bool = ((s_105_18) != 0);
        // D s_105_20: cast zx s_105_19 -> bv
        let s_105_20: Bits = Bits::new(s_105_19 as u128, 1u16);
        // C s_105_21: const #0u : u8
        let s_105_21: bool = false;
        // C s_105_22: cast zx s_105_21 -> bv
        let s_105_22: Bits = Bits::new(s_105_21 as u128, 1u16);
        // D s_105_23: cmp-eq s_105_20 s_105_22
        let s_105_23: bool = ((s_105_20) == (s_105_22));
        // N s_105_24: branch s_105_23 b114 b106
        if s_105_23 {
            return block_114(state, tracer, fn_state);
        } else {
            return block_106(state, tracer, fn_state);
        };
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #22712u : u32
        let s_106_0: u32 = 22712;
        // D s_106_1: read-reg s_106_0:struct
        let s_106_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_106_0 as isize);
            tracer.read_register(s_106_0 as isize, value);
            value
        };
        // D s_106_2: call _get_MDCR_EL3_Type_NSPB(s_106_1)
        let s_106_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_106_1);
        // C s_106_3: const #1s : i
        let s_106_3: i128 = 1;
        // D s_106_4: cast zx s_106_2 -> bv
        let s_106_4: Bits = Bits::new(s_106_2 as u128, 2u16);
        // C s_106_5: const #1u : u64
        let s_106_5: u64 = 1;
        // D s_106_6: bit-extract s_106_4 s_106_3 s_106_5
        let s_106_6: Bits = (Bits::new(
            ((s_106_4) >> (s_106_3)).value(),
            u16::try_from(s_106_5).unwrap(),
        ));
        // D s_106_7: cast reint s_106_6 -> u8
        let s_106_7: bool = ((s_106_6.value()) != 0);
        // C s_106_8: const #0s : i
        let s_106_8: i128 = 0;
        // C s_106_9: const #0u : u64
        let s_106_9: u64 = 0;
        // D s_106_10: cast zx s_106_7 -> u64
        let s_106_10: u64 = (s_106_7 as u64);
        // C s_106_11: const #1u : u64
        let s_106_11: u64 = 1;
        // D s_106_12: and s_106_10 s_106_11
        let s_106_12: u64 = ((s_106_10) & (s_106_11));
        // D s_106_13: cmp-eq s_106_12 s_106_11
        let s_106_13: bool = ((s_106_12) == (s_106_11));
        // D s_106_14: lsl s_106_10 s_106_8
        let s_106_14: u64 = s_106_10 << s_106_8;
        // D s_106_15: or s_106_9 s_106_14
        let s_106_15: u64 = ((s_106_9) | (s_106_14));
        // D s_106_16: cmpl s_106_14
        let s_106_16: u64 = !s_106_14;
        // D s_106_17: and s_106_9 s_106_16
        let s_106_17: u64 = ((s_106_9) & (s_106_16));
        // D s_106_18: select s_106_13 s_106_15 s_106_17
        let s_106_18: u64 = if s_106_13 { s_106_15 } else { s_106_17 };
        // D s_106_19: cast trunc s_106_18 -> u8
        let s_106_19: bool = ((s_106_18) != 0);
        // C s_106_20: const #90704u : u32
        let s_106_20: u32 = 90704;
        // D s_106_21: read-reg s_106_20:struct
        let s_106_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_106_20 as isize);
            tracer.read_register(s_106_20 as isize, value);
            value
        };
        // D s_106_22: call _get_SCR_EL3_Type_NS(s_106_21)
        let s_106_22: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_106_21);
        // D s_106_23: cast zx s_106_19 -> bv
        let s_106_23: Bits = Bits::new(s_106_19 as u128, 1u16);
        // D s_106_24: cast zx s_106_22 -> bv
        let s_106_24: Bits = Bits::new(s_106_22 as u128, 1u16);
        // D s_106_25: cmp-ne s_106_23 s_106_24
        let s_106_25: bool = ((s_106_23) != (s_106_24));
        // D s_106_26: write-var gs#90858 <= s_106_25
        fn_state.gs_90858 = s_106_25;
        // N s_106_27: jump b107
        return block_107(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_107_0: read-var gs#90858:u8
        let s_107_0: bool = fn_state.gs_90858;
        // N s_107_1: branch s_107_0 b113 b108
        if s_107_0 {
            return block_113(state, tracer, fn_state);
        } else {
            return block_108(state, tracer, fn_state);
        };
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #232u : u32
        let s_108_0: u32 = 232;
        // S s_108_1: call IsFeatureImplemented(s_108_0)
        let s_108_1: bool = IsFeatureImplemented(state, tracer, s_108_0);
        // N s_108_2: branch s_108_1 b112 b109
        if s_108_1 {
            return block_112(state, tracer, fn_state);
        } else {
            return block_109(state, tracer, fn_state);
        };
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #0u : u8
        let s_109_0: bool = false;
        // D s_109_1: write-var gs#90859 <= s_109_0
        fn_state.gs_90859 = s_109_0;
        // N s_109_2: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_110_0: read-var gs#90859:u8
        let s_110_0: bool = fn_state.gs_90859;
        // D s_110_1: write-var gs#90860 <= s_110_0
        fn_state.gs_90860 = s_110_0;
        // N s_110_2: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var gs#90860:u8
        let s_111_0: bool = fn_state.gs_90860;
        // D s_111_1: write-var gs#90861 <= s_111_0
        fn_state.gs_90861 = s_111_0;
        // N s_111_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_112_0: read-var __MDCR_EL3_NSPBE:u8
        let s_112_0: bool = fn_state.u__MDCR_EL3_NSPBE;
        // D s_112_1: cast zx s_112_0 -> bv
        let s_112_1: Bits = Bits::new(s_112_0 as u128, 1u16);
        // D s_112_2: read-var __SCR_EL3_NSE:u8
        let s_112_2: bool = fn_state.u__SCR_EL3_NSE;
        // D s_112_3: cast zx s_112_2 -> bv
        let s_112_3: Bits = Bits::new(s_112_2 as u128, 1u16);
        // D s_112_4: cmp-ne s_112_1 s_112_3
        let s_112_4: bool = ((s_112_1) != (s_112_3));
        // D s_112_5: write-var gs#90859 <= s_112_4
        fn_state.gs_90859 = s_112_4;
        // N s_112_6: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_113_0: const #1u : u8
        let s_113_0: bool = true;
        // D s_113_1: write-var gs#90860 <= s_113_0
        fn_state.gs_90860 = s_113_0;
        // N s_113_2: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #1u : u8
        let s_114_0: bool = true;
        // D s_114_1: write-var gs#90858 <= s_114_0
        fn_state.gs_90858 = s_114_0;
        // N s_114_2: jump b107
        return block_107(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_115_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_115_1: call __IMPDEF_boolean(s_115_0)
        let s_115_1: bool = u__IMPDEF_boolean(state, tracer, s_115_0);
        // D s_115_2: write-var gs#90853 <= s_115_1
        fn_state.gs_90853 = s_115_1;
        // N s_115_3: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #() : ()
        let s_116_0: () = ();
        // S s_116_1: call EDSCR_read(s_116_0)
        let s_116_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_116_0);
        // S s_116_2: call _get_EDSCR_Type_SDD(s_116_1)
        let s_116_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_116_1);
        // S s_116_3: cast zx s_116_2 -> bv
        let s_116_3: Bits = Bits::new(s_116_2 as u128, 1u16);
        // C s_116_4: const #1u : u8
        let s_116_4: bool = true;
        // C s_116_5: cast zx s_116_4 -> bv
        let s_116_5: Bits = Bits::new(s_116_4 as u128, 1u16);
        // S s_116_6: cmp-eq s_116_3 s_116_5
        let s_116_6: bool = ((s_116_3) == (s_116_5));
        // D s_116_7: write-var gs#90852 <= s_116_6
        fn_state.gs_90852 = s_116_6;
        // N s_116_8: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #424u : u32
        let s_117_0: u32 = 424;
        // D s_117_1: read-reg s_117_0:u8
        let s_117_1: u8 = {
            let value = state.read_register::<u8>(s_117_0 as isize);
            tracer.read_register(s_117_0 as isize, value);
            value
        };
        // C s_117_2: const #2u : u8
        let s_117_2: u8 = 2;
        // D s_117_3: cmp-lt s_117_1 s_117_2
        let s_117_3: bool = ((s_117_1) < (s_117_2));
        // D s_117_4: write-var gs#90851 <= s_117_3
        fn_state.gs_90851 = s_117_3;
        // N s_117_5: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_118_0: panic
        panic!("{:?}", ());
        // N s_118_1: return
        return;
    }
}
