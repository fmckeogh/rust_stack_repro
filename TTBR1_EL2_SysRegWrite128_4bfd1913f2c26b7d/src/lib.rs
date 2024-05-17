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
use NVMem_set__1::*;
use u_get_SCR_EL3_Type_FGTEn::*;
use u_get_HCR_EL2_Type_E2H::*;
use u_get_SCR_EL3_Type_D128En::*;
use u_get_HFGWTR_EL2_Type_TTBR1_EL1::*;
use Halted::*;
use IsFeatureImplemented::*;
use u__IMPDEF_boolean::*;
use X_read::*;
use AArch64_SystemAccessTrap::*;
use Mk_TTBR1_EL1_Type::*;
use u_get_HCR_EL2_Type_NV1::*;
use u_get_HCR_EL2_Type_TVM::*;
use u_get_HCR_EL2_Type_NV::*;
use u_get_EDSCR_Type_SDD::*;
use TTBR1_EL1_write::*;
use TTBR1_EL1_read::*;
use EL2Enabled::*;
use EDSCR_read::*;
use u_get_HCR_EL2_Type_NV2::*;
use common::*;
pub fn TTBR1_EL2_SysRegWrite128_4bfd1913f2c26b7d<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    op0: u8,
    op1: u8,
    CRn: u8,
    op2: u8,
    CRm: u8,
    t: i128,
    t2: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_137951: bool,
        u__HCR_EL2_E2H: bool,
        gs_137958: bool,
        gs_137950: bool,
        ga_240395: ProductType782ac6922b48c20d,
        gs_137949: bool,
        gs_137953: bool,
        u__HFGWTR_EL2_TTBR1_EL1: bool,
        gs_137935: bool,
        gs_137954: bool,
        u__PSTATE_EL: u8,
        gs_137932: bool,
        gs_137948: bool,
        gs_137957: bool,
        gs_137934: bool,
        gs_137956: bool,
        gs_137933: bool,
        u__SCR_EL3_D128En: bool,
        u__EDSCR_SDD: bool,
        u__SCR_EL3_FGTEn: bool,
        gs_137931: bool,
        gs_137955: bool,
        u__HCR_EL2_TVM: bool,
        gs_137968: bool,
        gs_137952: bool,
        gs_137947: bool,
        ga_240384: ProductType782ac6922b48c20d,
        ga_240352: ProductType782ac6922b48c20d,
        el: u8,
        op0: u8,
        op1: u8,
        CRn: u8,
        op2: u8,
        CRm: u8,
        t: i128,
        t2: i128,
    }
    let fn_state = FunctionState {
        el,
        op0,
        op1,
        CRn,
        op2,
        CRm,
        t,
        t2,
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
        // C s_0_7: const #90704u : u32
        let s_0_7: u32 = 90704;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_SCR_EL3_Type_D128En(s_0_8)
        let s_0_9: bool = u_get_SCR_EL3_Type_D128En(state, tracer, s_0_8);
        // D s_0_10: write-var __SCR_EL3_D128En <= s_0_9
        fn_state.u__SCR_EL3_D128En = s_0_9;
        // C s_0_11: const #102552u : u32
        let s_0_11: u32 = 102552;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_HCR_EL2_Type_TVM(s_0_12)
        let s_0_13: bool = u_get_HCR_EL2_Type_TVM(state, tracer, s_0_12);
        // D s_0_14: write-var __HCR_EL2_TVM <= s_0_13
        fn_state.u__HCR_EL2_TVM = s_0_13;
        // C s_0_15: const #90704u : u32
        let s_0_15: u32 = 90704;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_SCR_EL3_Type_FGTEn(s_0_16)
        let s_0_17: bool = u_get_SCR_EL3_Type_FGTEn(state, tracer, s_0_16);
        // D s_0_18: write-var __SCR_EL3_FGTEn <= s_0_17
        fn_state.u__SCR_EL3_FGTEn = s_0_17;
        // C s_0_19: const #100992u : u32
        let s_0_19: u32 = 100992;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_HFGWTR_EL2_Type_TTBR1_EL1(s_0_20)
        let s_0_21: bool = u_get_HFGWTR_EL2_Type_TTBR1_EL1(state, tracer, s_0_20);
        // D s_0_22: write-var __HFGWTR_EL2_TTBR1_EL1 <= s_0_21
        fn_state.u__HFGWTR_EL2_TTBR1_EL1 = s_0_21;
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
        // N s_0_33: branch s_0_32 b82 b1
        if s_0_32 {
            return block_82(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b33 b2
        if s_1_5 {
            return block_33(state, tracer, fn_state);
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
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call TTBR1_EL1_read(s_5_0)
        let s_5_1: ProductType782ac6922b48c20d = TTBR1_EL1_read(state, tracer, s_5_0);
        // D s_5_2: write-var ga#240395 <= s_5_1
        fn_state.ga_240395 = s_5_1;
        // D s_5_3: read-var ga#240395.0:struct
        let s_5_3: u128 = fn_state.ga_240395._0;
        // C s_5_4: const #1s : i
        let s_5_4: i128 = 1;
        // D s_5_5: read-var t:i
        let s_5_5: i128 = fn_state.t;
        // D s_5_6: add s_5_5 s_5_4
        let s_5_6: i128 = (s_5_5 + s_5_4);
        // C s_5_7: const #64s : i64
        let s_5_7: i64 = 64;
        // D s_5_8: call X_read(s_5_6, s_5_7)
        let s_5_8: Bits = X_read(state, tracer, s_5_6, s_5_7);
        // D s_5_9: cast reint s_5_8 -> u64
        let s_5_9: u64 = (s_5_8.value() as u64);
        // C s_5_10: const #64s : i64
        let s_5_10: i64 = 64;
        // D s_5_11: read-var t:i
        let s_5_11: i128 = fn_state.t;
        // D s_5_12: call X_read(s_5_11, s_5_10)
        let s_5_12: Bits = X_read(state, tracer, s_5_11, s_5_10);
        // D s_5_13: cast reint s_5_12 -> u64
        let s_5_13: u64 = (s_5_12.value() as u64);
        // D s_5_14: cast zx s_5_9 -> bv
        let s_5_14: Bits = Bits::new(s_5_9 as u128, 64u16);
        // D s_5_15: cast zx s_5_13 -> bv
        let s_5_15: Bits = Bits::new(s_5_13 as u128, 64u16);
        // D s_5_16: cast reint s_5_14 -> u128
        let s_5_16: u128 = (s_5_14.value() as u128);
        // D s_5_17: size-of s_5_14
        let s_5_17: u16 = s_5_14.length();
        // D s_5_18: cast reint s_5_15 -> u128
        let s_5_18: u128 = (s_5_15.value() as u128);
        // D s_5_19: size-of s_5_15
        let s_5_19: u16 = s_5_15.length();
        // D s_5_20: lsl s_5_16 s_5_19
        let s_5_20: u128 = s_5_16 << s_5_19;
        // D s_5_21: or s_5_20 s_5_18
        let s_5_21: u128 = ((s_5_20) | (s_5_18));
        // D s_5_22: add s_5_17 s_5_19
        let s_5_22: u16 = (s_5_17 + s_5_19);
        // D s_5_23: create-bits s_5_21 s_5_22
        let s_5_23: Bits = Bits::new(s_5_21, s_5_22);
        // D s_5_24: cast reint s_5_23 -> u128
        let s_5_24: u128 = (s_5_23.value() as u128);
        // C s_5_25: const #0s : i
        let s_5_25: i128 = 0;
        // D s_5_26: cast zx s_5_3 -> bv
        let s_5_26: Bits = Bits::new(s_5_3 as u128, 128u16);
        // D s_5_27: cast zx s_5_24 -> bv
        let s_5_27: Bits = Bits::new(s_5_24 as u128, 128u16);
        // C s_5_28: const #127s : i
        let s_5_28: i128 = 127;
        // C s_5_29: const #1u : u64
        let s_5_29: u64 = 1;
        // C s_5_30: cast zx s_5_29 -> bv
        let s_5_30: Bits = Bits::new(s_5_29 as u128, 64u16);
        // C s_5_31: lsl s_5_30 s_5_28
        let s_5_31: Bits = s_5_30 << s_5_28;
        // C s_5_32: sub s_5_31 s_5_30
        let s_5_32: Bits = ((s_5_31) - (s_5_30));
        // D s_5_33: and s_5_27 s_5_32
        let s_5_33: Bits = ((s_5_27) & (s_5_32));
        // D s_5_34: lsl s_5_33 s_5_25
        let s_5_34: Bits = s_5_33 << s_5_25;
        // C s_5_35: lsl s_5_32 s_5_25
        let s_5_35: Bits = s_5_32 << s_5_25;
        // C s_5_36: cmpl s_5_35
        let s_5_36: Bits = !s_5_35;
        // D s_5_37: and s_5_26 s_5_36
        let s_5_37: Bits = ((s_5_26) & (s_5_36));
        // D s_5_38: or s_5_37 s_5_34
        let s_5_38: Bits = ((s_5_37) | (s_5_34));
        // D s_5_39: cast reint s_5_38 -> u128
        let s_5_39: u128 = (s_5_38.value() as u128);
        // D s_5_40: call Mk_TTBR1_EL1_Type(s_5_39)
        let s_5_40: ProductType782ac6922b48c20d = Mk_TTBR1_EL1_Type(
            state,
            tracer,
            s_5_39,
        );
        // D s_5_41: call TTBR1_EL1_write(s_5_40)
        let s_5_41: () = TTBR1_EL1_write(state, tracer, s_5_40);
        // N s_5_42: return
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
        // N s_6_2: branch s_6_1 b32 b7
        if s_6_1 {
            return block_32(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#137931 <= s_7_0
        fn_state.gs_137931 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#137931:u8
        let s_8_0: bool = fn_state.gs_137931;
        // N s_8_1: branch s_8_0 b31 b9
        if s_8_0 {
            return block_31(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#137932 <= s_9_0
        fn_state.gs_137932 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#137932:u8
        let s_10_0: bool = fn_state.gs_137932;
        // N s_10_1: branch s_10_0 b30 b11
        if s_10_0 {
            return block_30(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#137933 <= s_11_0
        fn_state.gs_137933 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#137933:u8
        let s_12_0: bool = fn_state.gs_137933;
        // N s_12_1: branch s_12_0 b29 b13
        if s_12_0 {
            return block_29(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#137934 <= s_13_0
        fn_state.gs_137934 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#137934:u8
        let s_14_0: bool = fn_state.gs_137934;
        // N s_14_1: branch s_14_0 b28 b15
        if s_14_0 {
            return block_28(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#137935 <= s_16_0
        fn_state.gs_137935 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#137935:u8
        let s_17_0: bool = fn_state.gs_137935;
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
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call TTBR1_EL1_read(s_19_0)
        let s_19_1: ProductType782ac6922b48c20d = TTBR1_EL1_read(state, tracer, s_19_0);
        // D s_19_2: write-var ga#240384 <= s_19_1
        fn_state.ga_240384 = s_19_1;
        // D s_19_3: read-var ga#240384.0:struct
        let s_19_3: u128 = fn_state.ga_240384._0;
        // C s_19_4: const #1s : i
        let s_19_4: i128 = 1;
        // D s_19_5: read-var t:i
        let s_19_5: i128 = fn_state.t;
        // D s_19_6: add s_19_5 s_19_4
        let s_19_6: i128 = (s_19_5 + s_19_4);
        // C s_19_7: const #64s : i64
        let s_19_7: i64 = 64;
        // D s_19_8: call X_read(s_19_6, s_19_7)
        let s_19_8: Bits = X_read(state, tracer, s_19_6, s_19_7);
        // D s_19_9: cast reint s_19_8 -> u64
        let s_19_9: u64 = (s_19_8.value() as u64);
        // C s_19_10: const #64s : i64
        let s_19_10: i64 = 64;
        // D s_19_11: read-var t:i
        let s_19_11: i128 = fn_state.t;
        // D s_19_12: call X_read(s_19_11, s_19_10)
        let s_19_12: Bits = X_read(state, tracer, s_19_11, s_19_10);
        // D s_19_13: cast reint s_19_12 -> u64
        let s_19_13: u64 = (s_19_12.value() as u64);
        // D s_19_14: cast zx s_19_9 -> bv
        let s_19_14: Bits = Bits::new(s_19_9 as u128, 64u16);
        // D s_19_15: cast zx s_19_13 -> bv
        let s_19_15: Bits = Bits::new(s_19_13 as u128, 64u16);
        // D s_19_16: cast reint s_19_14 -> u128
        let s_19_16: u128 = (s_19_14.value() as u128);
        // D s_19_17: size-of s_19_14
        let s_19_17: u16 = s_19_14.length();
        // D s_19_18: cast reint s_19_15 -> u128
        let s_19_18: u128 = (s_19_15.value() as u128);
        // D s_19_19: size-of s_19_15
        let s_19_19: u16 = s_19_15.length();
        // D s_19_20: lsl s_19_16 s_19_19
        let s_19_20: u128 = s_19_16 << s_19_19;
        // D s_19_21: or s_19_20 s_19_18
        let s_19_21: u128 = ((s_19_20) | (s_19_18));
        // D s_19_22: add s_19_17 s_19_19
        let s_19_22: u16 = (s_19_17 + s_19_19);
        // D s_19_23: create-bits s_19_21 s_19_22
        let s_19_23: Bits = Bits::new(s_19_21, s_19_22);
        // D s_19_24: cast reint s_19_23 -> u128
        let s_19_24: u128 = (s_19_23.value() as u128);
        // C s_19_25: const #0s : i
        let s_19_25: i128 = 0;
        // D s_19_26: cast zx s_19_3 -> bv
        let s_19_26: Bits = Bits::new(s_19_3 as u128, 128u16);
        // D s_19_27: cast zx s_19_24 -> bv
        let s_19_27: Bits = Bits::new(s_19_24 as u128, 128u16);
        // C s_19_28: const #127s : i
        let s_19_28: i128 = 127;
        // C s_19_29: const #1u : u64
        let s_19_29: u64 = 1;
        // C s_19_30: cast zx s_19_29 -> bv
        let s_19_30: Bits = Bits::new(s_19_29 as u128, 64u16);
        // C s_19_31: lsl s_19_30 s_19_28
        let s_19_31: Bits = s_19_30 << s_19_28;
        // C s_19_32: sub s_19_31 s_19_30
        let s_19_32: Bits = ((s_19_31) - (s_19_30));
        // D s_19_33: and s_19_27 s_19_32
        let s_19_33: Bits = ((s_19_27) & (s_19_32));
        // D s_19_34: lsl s_19_33 s_19_25
        let s_19_34: Bits = s_19_33 << s_19_25;
        // C s_19_35: lsl s_19_32 s_19_25
        let s_19_35: Bits = s_19_32 << s_19_25;
        // C s_19_36: cmpl s_19_35
        let s_19_36: Bits = !s_19_35;
        // D s_19_37: and s_19_26 s_19_36
        let s_19_37: Bits = ((s_19_26) & (s_19_36));
        // D s_19_38: or s_19_37 s_19_34
        let s_19_38: Bits = ((s_19_37) | (s_19_34));
        // D s_19_39: cast reint s_19_38 -> u128
        let s_19_39: u128 = (s_19_38.value() as u128);
        // D s_19_40: call Mk_TTBR1_EL1_Type(s_19_39)
        let s_19_40: ProductType782ac6922b48c20d = Mk_TTBR1_EL1_Type(
            state,
            tracer,
            s_19_39,
        );
        // D s_19_41: call TTBR1_EL1_write(s_19_40)
        let s_19_41: () = TTBR1_EL1_write(state, tracer, s_19_40);
        // N s_19_42: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1s : i
        let s_20_0: i128 = 1;
        // D s_20_1: read-var t:i
        let s_20_1: i128 = fn_state.t;
        // D s_20_2: add s_20_1 s_20_0
        let s_20_2: i128 = (s_20_1 + s_20_0);
        // C s_20_3: const #64s : i64
        let s_20_3: i64 = 64;
        // D s_20_4: call X_read(s_20_2, s_20_3)
        let s_20_4: Bits = X_read(state, tracer, s_20_2, s_20_3);
        // C s_20_5: const #64s : i64
        let s_20_5: i64 = 64;
        // D s_20_6: read-var t:i
        let s_20_6: i128 = fn_state.t;
        // D s_20_7: call X_read(s_20_6, s_20_5)
        let s_20_7: Bits = X_read(state, tracer, s_20_6, s_20_5);
        // C s_20_8: const #18432u : u32
        let s_20_8: u32 = 18432;
        // D s_20_9: read-reg s_20_8:struct
        let s_20_9: ProductType782ac6922b48c20d = {
            let value = state
                .read_register::<ProductType782ac6922b48c20d>(s_20_8 as isize);
            tracer.read_register(s_20_8 as isize, value);
            value
        };
        // C s_20_10: const #18432u : u32
        let s_20_10: u32 = 18432;
        // N s_20_11: write-reg s_20_10 <= s_20_9
        let s_20_11: () = {
            state
                .write_register::<ProductType782ac6922b48c20d>(s_20_10 as isize, s_20_9);
            tracer.write_register(s_20_10 as isize, s_20_9);
        };
        // N s_20_12: return
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
        // D s_22_1: write-var gs#137947 <= s_22_0
        fn_state.gs_137947 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#137947:u8
        let s_23_0: bool = fn_state.gs_137947;
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
        // C s_24_0: const #20u : u8
        let s_24_0: u8 = 20;
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
        // D s_26_0: read-var __EDSCR_SDD:u8
        let s_26_0: bool = fn_state.u__EDSCR_SDD;
        // D s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 1u16);
        // C s_26_2: const #1u : u8
        let s_26_2: bool = true;
        // C s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 1u16);
        // D s_26_4: cmp-eq s_26_1 s_26_3
        let s_26_4: bool = ((s_26_1) == (s_26_3));
        // D s_26_5: write-var gs#137947 <= s_26_4
        fn_state.gs_137947 = s_26_4;
        // N s_26_6: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var __SCR_EL3_D128En:u8
        let s_27_0: bool = fn_state.u__SCR_EL3_D128En;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 1u16);
        // C s_27_2: const #0u : u8
        let s_27_2: bool = false;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: write-var gs#137935 <= s_27_4
        fn_state.gs_137935 = s_27_4;
        // N s_27_6: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_28_0: panic
        panic!("{:?}", ());
        // N s_28_1: return
        return;
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var __SCR_EL3_D128En:u8
        let s_29_0: bool = fn_state.u__SCR_EL3_D128En;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 1u16);
        // C s_29_2: const #0u : u8
        let s_29_2: bool = false;
        // C s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // D s_29_4: cmp-eq s_29_1 s_29_3
        let s_29_4: bool = ((s_29_1) == (s_29_3));
        // D s_29_5: write-var gs#137934 <= s_29_4
        fn_state.gs_137934 = s_29_4;
        // N s_29_6: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_30_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_30_1: call __IMPDEF_boolean(s_30_0)
        let s_30_1: bool = u__IMPDEF_boolean(state, tracer, s_30_0);
        // D s_30_2: write-var gs#137933 <= s_30_1
        fn_state.gs_137933 = s_30_1;
        // N s_30_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var __EDSCR_SDD:u8
        let s_31_0: bool = fn_state.u__EDSCR_SDD;
        // D s_31_1: cast zx s_31_0 -> bv
        let s_31_1: Bits = Bits::new(s_31_0 as u128, 1u16);
        // C s_31_2: const #1u : u8
        let s_31_2: bool = true;
        // C s_31_3: cast zx s_31_2 -> bv
        let s_31_3: Bits = Bits::new(s_31_2 as u128, 1u16);
        // D s_31_4: cmp-eq s_31_1 s_31_3
        let s_31_4: bool = ((s_31_1) == (s_31_3));
        // D s_31_5: write-var gs#137932 <= s_31_4
        fn_state.gs_137932 = s_31_4;
        // N s_31_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #424u : u32
        let s_32_0: u32 = 424;
        // D s_32_1: read-reg s_32_0:u8
        let s_32_1: u8 = {
            let value = state.read_register::<u8>(s_32_0 as isize);
            tracer.read_register(s_32_0 as isize, value);
            value
        };
        // C s_32_2: const #2u : u8
        let s_32_2: u8 = 2;
        // D s_32_3: cmp-lt s_32_1 s_32_2
        let s_32_3: bool = ((s_32_1) < (s_32_2));
        // D s_32_4: write-var gs#137931 <= s_32_3
        fn_state.gs_137931 = s_32_3;
        // N s_32_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #() : ()
        let s_33_0: () = ();
        // S s_33_1: call Halted(s_33_0)
        let s_33_1: bool = Halted(state, tracer, s_33_0);
        // N s_33_2: branch s_33_1 b81 b34
        if s_33_1 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #0u : u8
        let s_34_0: bool = false;
        // D s_34_1: write-var gs#137948 <= s_34_0
        fn_state.gs_137948 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#137948:u8
        let s_35_0: bool = fn_state.gs_137948;
        // N s_35_1: branch s_35_0 b80 b36
        if s_35_0 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #0u : u8
        let s_36_0: bool = false;
        // D s_36_1: write-var gs#137949 <= s_36_0
        fn_state.gs_137949 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#137949:u8
        let s_37_0: bool = fn_state.gs_137949;
        // N s_37_1: branch s_37_0 b79 b38
        if s_37_0 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #0u : u8
        let s_38_0: bool = false;
        // D s_38_1: write-var gs#137950 <= s_38_0
        fn_state.gs_137950 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#137950:u8
        let s_39_0: bool = fn_state.gs_137950;
        // N s_39_1: branch s_39_0 b78 b40
        if s_39_0 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #0u : u8
        let s_40_0: bool = false;
        // D s_40_1: write-var gs#137951 <= s_40_0
        fn_state.gs_137951 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#137951:u8
        let s_41_0: bool = fn_state.gs_137951;
        // N s_41_1: branch s_41_0 b77 b42
        if s_41_0 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #() : ()
        let s_42_0: () = ();
        // S s_42_1: call EL2Enabled(s_42_0)
        let s_42_1: bool = EL2Enabled(state, tracer, s_42_0);
        // N s_42_2: branch s_42_1 b76 b43
        if s_42_1 {
            return block_76(state, tracer, fn_state);
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
        // D s_43_1: write-var gs#137952 <= s_43_0
        fn_state.gs_137952 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#137952:u8
        let s_44_0: bool = fn_state.gs_137952;
        // N s_44_1: branch s_44_0 b75 b45
        if s_44_0 {
            return block_75(state, tracer, fn_state);
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
        // N s_45_2: branch s_45_1 b74 b46
        if s_45_1 {
            return block_74(state, tracer, fn_state);
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
        // D s_46_1: write-var gs#137953 <= s_46_0
        fn_state.gs_137953 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#137953:u8
        let s_47_0: bool = fn_state.gs_137953;
        // N s_47_1: branch s_47_0 b70 b48
        if s_47_0 {
            return block_70(state, tracer, fn_state);
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
        // D s_48_1: write-var gs#137955 <= s_48_0
        fn_state.gs_137955 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#137955:u8
        let s_49_0: bool = fn_state.gs_137955;
        // N s_49_1: branch s_49_0 b69 b50
        if s_49_0 {
            return block_69(state, tracer, fn_state);
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
        // D s_50_1: write-var gs#137956 <= s_50_0
        fn_state.gs_137956 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#137956:u8
        let s_51_0: bool = fn_state.gs_137956;
        // N s_51_1: branch s_51_0 b68 b52
        if s_51_0 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #424u : u32
        let s_52_0: u32 = 424;
        // D s_52_1: read-reg s_52_0:u8
        let s_52_1: u8 = {
            let value = state.read_register::<u8>(s_52_0 as isize);
            tracer.read_register(s_52_0 as isize, value);
            value
        };
        // C s_52_2: const #2u : u8
        let s_52_2: u8 = 2;
        // D s_52_3: cmp-lt s_52_1 s_52_2
        let s_52_3: bool = ((s_52_1) < (s_52_2));
        // N s_52_4: branch s_52_3 b67 b53
        if s_52_3 {
            return block_67(state, tracer, fn_state);
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
        // D s_53_1: write-var gs#137957 <= s_53_0
        fn_state.gs_137957 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#137957:u8
        let s_54_0: bool = fn_state.gs_137957;
        // N s_54_1: branch s_54_0 b61 b55
        if s_54_0 {
            return block_61(state, tracer, fn_state);
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
        // N s_55_2: branch s_55_1 b60 b56
        if s_55_1 {
            return block_60(state, tracer, fn_state);
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
        // D s_56_1: write-var gs#137958 <= s_56_0
        fn_state.gs_137958 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#137958:u8
        let s_57_0: bool = fn_state.gs_137958;
        // N s_57_1: branch s_57_0 b59 b58
        if s_57_0 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #() : ()
        let s_58_0: () = ();
        // S s_58_1: call TTBR1_EL1_read(s_58_0)
        let s_58_1: ProductType782ac6922b48c20d = TTBR1_EL1_read(state, tracer, s_58_0);
        // D s_58_2: write-var ga#240352 <= s_58_1
        fn_state.ga_240352 = s_58_1;
        // D s_58_3: read-var ga#240352.0:struct
        let s_58_3: u128 = fn_state.ga_240352._0;
        // C s_58_4: const #1s : i
        let s_58_4: i128 = 1;
        // D s_58_5: read-var t:i
        let s_58_5: i128 = fn_state.t;
        // D s_58_6: add s_58_5 s_58_4
        let s_58_6: i128 = (s_58_5 + s_58_4);
        // C s_58_7: const #64s : i64
        let s_58_7: i64 = 64;
        // D s_58_8: call X_read(s_58_6, s_58_7)
        let s_58_8: Bits = X_read(state, tracer, s_58_6, s_58_7);
        // D s_58_9: cast reint s_58_8 -> u64
        let s_58_9: u64 = (s_58_8.value() as u64);
        // C s_58_10: const #64s : i64
        let s_58_10: i64 = 64;
        // D s_58_11: read-var t:i
        let s_58_11: i128 = fn_state.t;
        // D s_58_12: call X_read(s_58_11, s_58_10)
        let s_58_12: Bits = X_read(state, tracer, s_58_11, s_58_10);
        // D s_58_13: cast reint s_58_12 -> u64
        let s_58_13: u64 = (s_58_12.value() as u64);
        // D s_58_14: cast zx s_58_9 -> bv
        let s_58_14: Bits = Bits::new(s_58_9 as u128, 64u16);
        // D s_58_15: cast zx s_58_13 -> bv
        let s_58_15: Bits = Bits::new(s_58_13 as u128, 64u16);
        // D s_58_16: cast reint s_58_14 -> u128
        let s_58_16: u128 = (s_58_14.value() as u128);
        // D s_58_17: size-of s_58_14
        let s_58_17: u16 = s_58_14.length();
        // D s_58_18: cast reint s_58_15 -> u128
        let s_58_18: u128 = (s_58_15.value() as u128);
        // D s_58_19: size-of s_58_15
        let s_58_19: u16 = s_58_15.length();
        // D s_58_20: lsl s_58_16 s_58_19
        let s_58_20: u128 = s_58_16 << s_58_19;
        // D s_58_21: or s_58_20 s_58_18
        let s_58_21: u128 = ((s_58_20) | (s_58_18));
        // D s_58_22: add s_58_17 s_58_19
        let s_58_22: u16 = (s_58_17 + s_58_19);
        // D s_58_23: create-bits s_58_21 s_58_22
        let s_58_23: Bits = Bits::new(s_58_21, s_58_22);
        // D s_58_24: cast reint s_58_23 -> u128
        let s_58_24: u128 = (s_58_23.value() as u128);
        // C s_58_25: const #0s : i
        let s_58_25: i128 = 0;
        // D s_58_26: cast zx s_58_3 -> bv
        let s_58_26: Bits = Bits::new(s_58_3 as u128, 128u16);
        // D s_58_27: cast zx s_58_24 -> bv
        let s_58_27: Bits = Bits::new(s_58_24 as u128, 128u16);
        // C s_58_28: const #127s : i
        let s_58_28: i128 = 127;
        // C s_58_29: const #1u : u64
        let s_58_29: u64 = 1;
        // C s_58_30: cast zx s_58_29 -> bv
        let s_58_30: Bits = Bits::new(s_58_29 as u128, 64u16);
        // C s_58_31: lsl s_58_30 s_58_28
        let s_58_31: Bits = s_58_30 << s_58_28;
        // C s_58_32: sub s_58_31 s_58_30
        let s_58_32: Bits = ((s_58_31) - (s_58_30));
        // D s_58_33: and s_58_27 s_58_32
        let s_58_33: Bits = ((s_58_27) & (s_58_32));
        // D s_58_34: lsl s_58_33 s_58_25
        let s_58_34: Bits = s_58_33 << s_58_25;
        // C s_58_35: lsl s_58_32 s_58_25
        let s_58_35: Bits = s_58_32 << s_58_25;
        // C s_58_36: cmpl s_58_35
        let s_58_36: Bits = !s_58_35;
        // D s_58_37: and s_58_26 s_58_36
        let s_58_37: Bits = ((s_58_26) & (s_58_36));
        // D s_58_38: or s_58_37 s_58_34
        let s_58_38: Bits = ((s_58_37) | (s_58_34));
        // D s_58_39: cast reint s_58_38 -> u128
        let s_58_39: u128 = (s_58_38.value() as u128);
        // D s_58_40: call Mk_TTBR1_EL1_Type(s_58_39)
        let s_58_40: ProductType782ac6922b48c20d = Mk_TTBR1_EL1_Type(
            state,
            tracer,
            s_58_39,
        );
        // D s_58_41: call TTBR1_EL1_write(s_58_40)
        let s_58_41: () = TTBR1_EL1_write(state, tracer, s_58_40);
        // N s_58_42: return
        return;
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #528u : u12
        let s_59_0: u16 = 528;
        // C s_59_1: cast zx s_59_0 -> bv
        let s_59_1: Bits = Bits::new(s_59_0 as u128, 12u16);
        // C s_59_2: cast zx s_59_1 -> i
        let s_59_2: i128 = (s_59_1.value() as i128);
        // C s_59_3: cast reint s_59_2 -> i64
        let s_59_3: i64 = (s_59_2 as i64);
        // C s_59_4: const #128s : i64
        let s_59_4: i64 = 128;
        // C s_59_5: const #1s : i
        let s_59_5: i128 = 1;
        // D s_59_6: read-var t:i
        let s_59_6: i128 = fn_state.t;
        // D s_59_7: add s_59_6 s_59_5
        let s_59_7: i128 = (s_59_6 + s_59_5);
        // C s_59_8: const #64s : i64
        let s_59_8: i64 = 64;
        // D s_59_9: call X_read(s_59_7, s_59_8)
        let s_59_9: Bits = X_read(state, tracer, s_59_7, s_59_8);
        // D s_59_10: cast reint s_59_9 -> u64
        let s_59_10: u64 = (s_59_9.value() as u64);
        // C s_59_11: const #64s : i64
        let s_59_11: i64 = 64;
        // D s_59_12: read-var t:i
        let s_59_12: i128 = fn_state.t;
        // D s_59_13: call X_read(s_59_12, s_59_11)
        let s_59_13: Bits = X_read(state, tracer, s_59_12, s_59_11);
        // D s_59_14: cast reint s_59_13 -> u64
        let s_59_14: u64 = (s_59_13.value() as u64);
        // D s_59_15: cast zx s_59_10 -> bv
        let s_59_15: Bits = Bits::new(s_59_10 as u128, 64u16);
        // D s_59_16: cast zx s_59_14 -> bv
        let s_59_16: Bits = Bits::new(s_59_14 as u128, 64u16);
        // D s_59_17: cast reint s_59_15 -> u128
        let s_59_17: u128 = (s_59_15.value() as u128);
        // D s_59_18: size-of s_59_15
        let s_59_18: u16 = s_59_15.length();
        // D s_59_19: cast reint s_59_16 -> u128
        let s_59_19: u128 = (s_59_16.value() as u128);
        // D s_59_20: size-of s_59_16
        let s_59_20: u16 = s_59_16.length();
        // D s_59_21: lsl s_59_17 s_59_20
        let s_59_21: u128 = s_59_17 << s_59_20;
        // D s_59_22: or s_59_21 s_59_19
        let s_59_22: u128 = ((s_59_21) | (s_59_19));
        // D s_59_23: add s_59_18 s_59_20
        let s_59_23: u16 = (s_59_18 + s_59_20);
        // D s_59_24: create-bits s_59_22 s_59_23
        let s_59_24: Bits = Bits::new(s_59_22, s_59_23);
        // D s_59_25: cast reint s_59_24 -> u128
        let s_59_25: u128 = (s_59_24.value() as u128);
        // C s_59_26: cast zx s_59_3 -> i
        let s_59_26: i128 = (i128::try_from(s_59_3).unwrap());
        // D s_59_27: cast zx s_59_25 -> bv
        let s_59_27: Bits = Bits::new(s_59_25 as u128, 128u16);
        // D s_59_28: call NVMem_set__1(s_59_26, s_59_4, s_59_27)
        let s_59_28: () = NVMem_set__1(state, tracer, s_59_26, s_59_4, s_59_27);
        // N s_59_29: return
        return;
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #102552u : u32
        let s_60_0: u32 = 102552;
        // D s_60_1: read-reg s_60_0:struct
        let s_60_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_60_0 as isize);
            tracer.read_register(s_60_0 as isize, value);
            value
        };
        // D s_60_2: call _get_HCR_EL2_Type_NV2(s_60_1)
        let s_60_2: bool = u_get_HCR_EL2_Type_NV2(state, tracer, s_60_1);
        // C s_60_3: const #102552u : u32
        let s_60_3: u32 = 102552;
        // D s_60_4: read-reg s_60_3:struct
        let s_60_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_60_3 as isize);
            tracer.read_register(s_60_3 as isize, value);
            value
        };
        // D s_60_5: call _get_HCR_EL2_Type_NV1(s_60_4)
        let s_60_5: bool = u_get_HCR_EL2_Type_NV1(state, tracer, s_60_4);
        // C s_60_6: const #102552u : u32
        let s_60_6: u32 = 102552;
        // D s_60_7: read-reg s_60_6:struct
        let s_60_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_60_6 as isize);
            tracer.read_register(s_60_6 as isize, value);
            value
        };
        // D s_60_8: call _get_HCR_EL2_Type_NV(s_60_7)
        let s_60_8: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_60_7);
        // D s_60_9: cast zx s_60_5 -> bv
        let s_60_9: Bits = Bits::new(s_60_5 as u128, 1u16);
        // D s_60_10: cast zx s_60_8 -> bv
        let s_60_10: Bits = Bits::new(s_60_8 as u128, 1u16);
        // D s_60_11: cast reint s_60_9 -> u128
        let s_60_11: u128 = (s_60_9.value() as u128);
        // D s_60_12: size-of s_60_9
        let s_60_12: u16 = s_60_9.length();
        // D s_60_13: cast reint s_60_10 -> u128
        let s_60_13: u128 = (s_60_10.value() as u128);
        // D s_60_14: size-of s_60_10
        let s_60_14: u16 = s_60_10.length();
        // D s_60_15: lsl s_60_11 s_60_14
        let s_60_15: u128 = s_60_11 << s_60_14;
        // D s_60_16: or s_60_15 s_60_13
        let s_60_16: u128 = ((s_60_15) | (s_60_13));
        // D s_60_17: add s_60_12 s_60_14
        let s_60_17: u16 = (s_60_12 + s_60_14);
        // D s_60_18: create-bits s_60_16 s_60_17
        let s_60_18: Bits = Bits::new(s_60_16, s_60_17);
        // D s_60_19: cast reint s_60_18 -> u8
        let s_60_19: u8 = (s_60_18.value() as u8);
        // D s_60_20: cast zx s_60_2 -> bv
        let s_60_20: Bits = Bits::new(s_60_2 as u128, 1u16);
        // D s_60_21: cast zx s_60_19 -> bv
        let s_60_21: Bits = Bits::new(s_60_19 as u128, 2u16);
        // D s_60_22: cast reint s_60_20 -> u128
        let s_60_22: u128 = (s_60_20.value() as u128);
        // D s_60_23: size-of s_60_20
        let s_60_23: u16 = s_60_20.length();
        // D s_60_24: cast reint s_60_21 -> u128
        let s_60_24: u128 = (s_60_21.value() as u128);
        // D s_60_25: size-of s_60_21
        let s_60_25: u16 = s_60_21.length();
        // D s_60_26: lsl s_60_22 s_60_25
        let s_60_26: u128 = s_60_22 << s_60_25;
        // D s_60_27: or s_60_26 s_60_24
        let s_60_27: u128 = ((s_60_26) | (s_60_24));
        // D s_60_28: add s_60_23 s_60_25
        let s_60_28: u16 = (s_60_23 + s_60_25);
        // D s_60_29: create-bits s_60_27 s_60_28
        let s_60_29: Bits = Bits::new(s_60_27, s_60_28);
        // D s_60_30: cast reint s_60_29 -> u8
        let s_60_30: u8 = (s_60_29.value() as u8);
        // D s_60_31: cast zx s_60_30 -> bv
        let s_60_31: Bits = Bits::new(s_60_30 as u128, 3u16);
        // C s_60_32: const #7u : u8
        let s_60_32: u8 = 7;
        // C s_60_33: cast zx s_60_32 -> bv
        let s_60_33: Bits = Bits::new(s_60_32 as u128, 3u16);
        // D s_60_34: cmp-eq s_60_31 s_60_33
        let s_60_34: bool = ((s_60_31) == (s_60_33));
        // D s_60_35: write-var gs#137958 <= s_60_34
        fn_state.gs_137958 = s_60_34;
        // N s_60_36: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #() : ()
        let s_61_0: () = ();
        // S s_61_1: call Halted(s_61_0)
        let s_61_1: bool = Halted(state, tracer, s_61_0);
        // N s_61_2: branch s_61_1 b66 b62
        if s_61_1 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #0u : u8
        let s_62_0: bool = false;
        // D s_62_1: write-var gs#137968 <= s_62_0
        fn_state.gs_137968 = s_62_0;
        // N s_62_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var gs#137968:u8
        let s_63_0: bool = fn_state.gs_137968;
        // N s_63_1: branch s_63_0 b65 b64
        if s_63_0 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #20u : u8
        let s_64_0: u8 = 20;
        // C s_64_1: cast zx s_64_0 -> bv
        let s_64_1: Bits = Bits::new(s_64_0 as u128, 8u16);
        // C s_64_2: cast zx s_64_1 -> i
        let s_64_2: i128 = (s_64_1.value() as i128);
        // C s_64_3: cast reint s_64_2 -> i64
        let s_64_3: i64 = (s_64_2 as i64);
        // C s_64_4: cast zx s_64_3 -> i
        let s_64_4: i128 = (i128::try_from(s_64_3).unwrap());
        // C s_64_5: const #424u : u32
        let s_64_5: u32 = 424;
        // D s_64_6: read-reg s_64_5:u8
        let s_64_6: u8 = {
            let value = state.read_register::<u8>(s_64_5 as isize);
            tracer.read_register(s_64_5 as isize, value);
            value
        };
        // D s_64_7: call AArch64_SystemAccessTrap(s_64_6, s_64_4)
        let s_64_7: () = AArch64_SystemAccessTrap(state, tracer, s_64_6, s_64_4);
        // N s_64_8: return
        return;
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
        // D s_66_0: read-var __EDSCR_SDD:u8
        let s_66_0: bool = fn_state.u__EDSCR_SDD;
        // D s_66_1: cast zx s_66_0 -> bv
        let s_66_1: Bits = Bits::new(s_66_0 as u128, 1u16);
        // C s_66_2: const #1u : u8
        let s_66_2: bool = true;
        // C s_66_3: cast zx s_66_2 -> bv
        let s_66_3: Bits = Bits::new(s_66_2 as u128, 1u16);
        // D s_66_4: cmp-eq s_66_1 s_66_3
        let s_66_4: bool = ((s_66_1) == (s_66_3));
        // D s_66_5: write-var gs#137968 <= s_66_4
        fn_state.gs_137968 = s_66_4;
        // N s_66_6: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var __SCR_EL3_D128En:u8
        let s_67_0: bool = fn_state.u__SCR_EL3_D128En;
        // D s_67_1: cast zx s_67_0 -> bv
        let s_67_1: Bits = Bits::new(s_67_0 as u128, 1u16);
        // C s_67_2: const #0u : u8
        let s_67_2: bool = false;
        // C s_67_3: cast zx s_67_2 -> bv
        let s_67_3: Bits = Bits::new(s_67_2 as u128, 1u16);
        // D s_67_4: cmp-eq s_67_1 s_67_3
        let s_67_4: bool = ((s_67_1) == (s_67_3));
        // D s_67_5: write-var gs#137957 <= s_67_4
        fn_state.gs_137957 = s_67_4;
        // N s_67_6: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #20u : u8
        let s_68_0: u8 = 20;
        // C s_68_1: cast zx s_68_0 -> bv
        let s_68_1: Bits = Bits::new(s_68_0 as u128, 8u16);
        // C s_68_2: cast zx s_68_1 -> i
        let s_68_2: i128 = (s_68_1.value() as i128);
        // C s_68_3: cast reint s_68_2 -> i64
        let s_68_3: i64 = (s_68_2 as i64);
        // C s_68_4: cast zx s_68_3 -> i
        let s_68_4: i128 = (i128::try_from(s_68_3).unwrap());
        // C s_68_5: const #432u : u32
        let s_68_5: u32 = 432;
        // D s_68_6: read-reg s_68_5:u8
        let s_68_6: u8 = {
            let value = state.read_register::<u8>(s_68_5 as isize);
            tracer.read_register(s_68_5 as isize, value);
            value
        };
        // D s_68_7: call AArch64_SystemAccessTrap(s_68_6, s_68_4)
        let s_68_7: () = AArch64_SystemAccessTrap(state, tracer, s_68_6, s_68_4);
        // N s_68_8: return
        return;
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var __HFGWTR_EL2_TTBR1_EL1:u8
        let s_69_0: bool = fn_state.u__HFGWTR_EL2_TTBR1_EL1;
        // D s_69_1: cast zx s_69_0 -> bv
        let s_69_1: Bits = Bits::new(s_69_0 as u128, 1u16);
        // C s_69_2: const #1u : u8
        let s_69_2: bool = true;
        // C s_69_3: cast zx s_69_2 -> bv
        let s_69_3: Bits = Bits::new(s_69_2 as u128, 1u16);
        // D s_69_4: cmp-eq s_69_1 s_69_3
        let s_69_4: bool = ((s_69_1) == (s_69_3));
        // D s_69_5: write-var gs#137956 <= s_69_4
        fn_state.gs_137956 = s_69_4;
        // N s_69_6: jump b51
        return block_51(state, tracer, fn_state);
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
        // D s_70_4: not s_70_3
        let s_70_4: bool = !s_70_3;
        // N s_70_5: branch s_70_4 b73 b71
        if s_70_4 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var __SCR_EL3_FGTEn:u8
        let s_71_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_71_1: cast zx s_71_0 -> bv
        let s_71_1: Bits = Bits::new(s_71_0 as u128, 1u16);
        // C s_71_2: const #1u : u8
        let s_71_2: bool = true;
        // C s_71_3: cast zx s_71_2 -> bv
        let s_71_3: Bits = Bits::new(s_71_2 as u128, 1u16);
        // D s_71_4: cmp-eq s_71_1 s_71_3
        let s_71_4: bool = ((s_71_1) == (s_71_3));
        // D s_71_5: write-var gs#137954 <= s_71_4
        fn_state.gs_137954 = s_71_4;
        // N s_71_6: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var gs#137954:u8
        let s_72_0: bool = fn_state.gs_137954;
        // D s_72_1: write-var gs#137955 <= s_72_0
        fn_state.gs_137955 = s_72_0;
        // N s_72_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #1u : u8
        let s_73_0: bool = true;
        // D s_73_1: write-var gs#137954 <= s_73_0
        fn_state.gs_137954 = s_73_0;
        // N s_73_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #146u : u32
        let s_74_0: u32 = 146;
        // S s_74_1: call IsFeatureImplemented(s_74_0)
        let s_74_1: bool = IsFeatureImplemented(state, tracer, s_74_0);
        // D s_74_2: write-var gs#137953 <= s_74_1
        fn_state.gs_137953 = s_74_1;
        // N s_74_3: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #20u : u8
        let s_75_0: u8 = 20;
        // C s_75_1: cast zx s_75_0 -> bv
        let s_75_1: Bits = Bits::new(s_75_0 as u128, 8u16);
        // C s_75_2: cast zx s_75_1 -> i
        let s_75_2: i128 = (s_75_1.value() as i128);
        // C s_75_3: cast reint s_75_2 -> i64
        let s_75_3: i64 = (s_75_2 as i64);
        // C s_75_4: cast zx s_75_3 -> i
        let s_75_4: i128 = (i128::try_from(s_75_3).unwrap());
        // C s_75_5: const #432u : u32
        let s_75_5: u32 = 432;
        // D s_75_6: read-reg s_75_5:u8
        let s_75_6: u8 = {
            let value = state.read_register::<u8>(s_75_5 as isize);
            tracer.read_register(s_75_5 as isize, value);
            value
        };
        // D s_75_7: call AArch64_SystemAccessTrap(s_75_6, s_75_4)
        let s_75_7: () = AArch64_SystemAccessTrap(state, tracer, s_75_6, s_75_4);
        // N s_75_8: return
        return;
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var __HCR_EL2_TVM:u8
        let s_76_0: bool = fn_state.u__HCR_EL2_TVM;
        // D s_76_1: cast zx s_76_0 -> bv
        let s_76_1: Bits = Bits::new(s_76_0 as u128, 1u16);
        // C s_76_2: const #1u : u8
        let s_76_2: bool = true;
        // C s_76_3: cast zx s_76_2 -> bv
        let s_76_3: Bits = Bits::new(s_76_2 as u128, 1u16);
        // D s_76_4: cmp-eq s_76_1 s_76_3
        let s_76_4: bool = ((s_76_1) == (s_76_3));
        // D s_76_5: write-var gs#137952 <= s_76_4
        fn_state.gs_137952 = s_76_4;
        // N s_76_6: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_77_0: panic
        panic!("{:?}", ());
        // N s_77_1: return
        return;
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var __SCR_EL3_D128En:u8
        let s_78_0: bool = fn_state.u__SCR_EL3_D128En;
        // D s_78_1: cast zx s_78_0 -> bv
        let s_78_1: Bits = Bits::new(s_78_0 as u128, 1u16);
        // C s_78_2: const #0u : u8
        let s_78_2: bool = false;
        // C s_78_3: cast zx s_78_2 -> bv
        let s_78_3: Bits = Bits::new(s_78_2 as u128, 1u16);
        // D s_78_4: cmp-eq s_78_1 s_78_3
        let s_78_4: bool = ((s_78_1) == (s_78_3));
        // D s_78_5: write-var gs#137951 <= s_78_4
        fn_state.gs_137951 = s_78_4;
        // N s_78_6: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_79_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_79_1: call __IMPDEF_boolean(s_79_0)
        let s_79_1: bool = u__IMPDEF_boolean(state, tracer, s_79_0);
        // D s_79_2: write-var gs#137950 <= s_79_1
        fn_state.gs_137950 = s_79_1;
        // N s_79_3: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var __EDSCR_SDD:u8
        let s_80_0: bool = fn_state.u__EDSCR_SDD;
        // D s_80_1: cast zx s_80_0 -> bv
        let s_80_1: Bits = Bits::new(s_80_0 as u128, 1u16);
        // C s_80_2: const #1u : u8
        let s_80_2: bool = true;
        // C s_80_3: cast zx s_80_2 -> bv
        let s_80_3: Bits = Bits::new(s_80_2 as u128, 1u16);
        // D s_80_4: cmp-eq s_80_1 s_80_3
        let s_80_4: bool = ((s_80_1) == (s_80_3));
        // D s_80_5: write-var gs#137949 <= s_80_4
        fn_state.gs_137949 = s_80_4;
        // N s_80_6: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #424u : u32
        let s_81_0: u32 = 424;
        // D s_81_1: read-reg s_81_0:u8
        let s_81_1: u8 = {
            let value = state.read_register::<u8>(s_81_0 as isize);
            tracer.read_register(s_81_0 as isize, value);
            value
        };
        // C s_81_2: const #2u : u8
        let s_81_2: u8 = 2;
        // D s_81_3: cmp-lt s_81_1 s_81_2
        let s_81_3: bool = ((s_81_1) < (s_81_2));
        // D s_81_4: write-var gs#137948 <= s_81_3
        fn_state.gs_137948 = s_81_3;
        // N s_81_5: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_82_0: panic
        panic!("{:?}", ());
        // N s_82_1: return
        return;
    }
}
