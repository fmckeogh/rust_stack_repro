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
use u_get_HFGWTR_EL2_Type_TTBR0_EL1::*;
use u_get_SCR_EL3_Type_D128En::*;
use u_get_HCR_EL2_Type_E2H::*;
use Halted::*;
use IsFeatureImplemented::*;
use X_read::*;
use AArch64_SystemAccessTrap::*;
use u__IMPDEF_boolean::*;
use TTBR0_EL1_read::*;
use u_get_HCR_EL2_Type_NV1::*;
use u_get_HCRX_EL2_Type_D128En::*;
use Mk_TTBR0_EL1_Type::*;
use TTBR0_EL2_write::*;
use IsHCRXEL2Enabled::*;
use u_get_HCR_EL2_Type_TVM::*;
use TTBR0_EL2_read::*;
use u_get_HCR_EL2_Type_NV::*;
use Mk_TTBR0_EL2_Type::*;
use u_get_EDSCR_Type_SDD::*;
use TTBR0_EL1_write::*;
use EL2Enabled::*;
use EDSCR_read::*;
use u_get_HCR_EL2_Type_NV2::*;
use common::*;
pub fn TTBR0_EL1_SysRegWrite128_66ee61de597f133e<T: Tracer>(
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
        gs_137846: bool,
        u__HCR_EL2_E2H: bool,
        u__HCRX_EL2_D128En: bool,
        ga_240131: ProductType782ac6922b48c20d,
        gs_137838: bool,
        gs_137841: bool,
        gs_137824: bool,
        ga_240120: ProductType782ac6922b48c20d,
        gs_137837: bool,
        gs_137823: bool,
        gs_137843: bool,
        gs_137848: bool,
        u__PSTATE_EL: u8,
        u__HFGWTR_EL2_TTBR0_EL1: bool,
        gs_137844: bool,
        gs_137849: bool,
        gs_137826: bool,
        u__SCR_EL3_D128En: bool,
        u__EDSCR_SDD: bool,
        gs_137842: bool,
        u__SCR_EL3_FGTEn: bool,
        gs_137822: bool,
        gs_137840: bool,
        gs_137850: bool,
        u__HCR_EL2_TVM: bool,
        gs_137839: bool,
        ga_240110: ProductType782ac6922b48c20d,
        gs_137847: bool,
        gs_137825: bool,
        gs_137845: bool,
        gs_137860: bool,
        ga_240086: ProductType782ac6922b48c20d,
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
        // D s_0_21: call _get_HFGWTR_EL2_Type_TTBR0_EL1(s_0_20)
        let s_0_21: bool = u_get_HFGWTR_EL2_Type_TTBR0_EL1(state, tracer, s_0_20);
        // D s_0_22: write-var __HFGWTR_EL2_TTBR0_EL1 <= s_0_21
        fn_state.u__HFGWTR_EL2_TTBR0_EL1 = s_0_21;
        // C s_0_23: const #22528u : u32
        let s_0_23: u32 = 22528;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_HCRX_EL2_Type_D128En(s_0_24)
        let s_0_25: bool = u_get_HCRX_EL2_Type_D128En(state, tracer, s_0_24);
        // D s_0_26: write-var __HCRX_EL2_D128En <= s_0_25
        fn_state.u__HCRX_EL2_D128En = s_0_25;
        // C s_0_27: const #102552u : u32
        let s_0_27: u32 = 102552;
        // D s_0_28: read-reg s_0_27:struct
        let s_0_28: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_27 as isize);
            tracer.read_register(s_0_27 as isize, value);
            value
        };
        // D s_0_29: call _get_HCR_EL2_Type_E2H(s_0_28)
        let s_0_29: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_0_28);
        // D s_0_30: write-var __HCR_EL2_E2H <= s_0_29
        fn_state.u__HCR_EL2_E2H = s_0_29;
        // D s_0_31: read-var __PSTATE_EL:u8
        let s_0_31: u8 = fn_state.u__PSTATE_EL;
        // D s_0_32: cast zx s_0_31 -> bv
        let s_0_32: Bits = Bits::new(s_0_31 as u128, 2u16);
        // C s_0_33: const #448u : u32
        let s_0_33: u32 = 448;
        // D s_0_34: read-reg s_0_33:u8
        let s_0_34: u8 = {
            let value = state.read_register::<u8>(s_0_33 as isize);
            tracer.read_register(s_0_33 as isize, value);
            value
        };
        // D s_0_35: cast zx s_0_34 -> bv
        let s_0_35: Bits = Bits::new(s_0_34 as u128, 2u16);
        // D s_0_36: cmp-eq s_0_32 s_0_35
        let s_0_36: bool = ((s_0_32) == (s_0_35));
        // N s_0_37: branch s_0_36 b90 b1
        if s_0_36 {
            return block_90(state, tracer, fn_state);
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
        // S s_5_1: call TTBR0_EL1_read(s_5_0)
        let s_5_1: ProductType782ac6922b48c20d = TTBR0_EL1_read(state, tracer, s_5_0);
        // D s_5_2: write-var ga#240131 <= s_5_1
        fn_state.ga_240131 = s_5_1;
        // D s_5_3: read-var ga#240131.0:struct
        let s_5_3: u128 = fn_state.ga_240131._0;
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
        // D s_5_40: call Mk_TTBR0_EL1_Type(s_5_39)
        let s_5_40: ProductType782ac6922b48c20d = Mk_TTBR0_EL1_Type(
            state,
            tracer,
            s_5_39,
        );
        // D s_5_41: call TTBR0_EL1_write(s_5_40)
        let s_5_41: () = TTBR0_EL1_write(state, tracer, s_5_40);
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
        // D s_7_1: write-var gs#137822 <= s_7_0
        fn_state.gs_137822 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#137822:u8
        let s_8_0: bool = fn_state.gs_137822;
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
        // D s_9_1: write-var gs#137823 <= s_9_0
        fn_state.gs_137823 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#137823:u8
        let s_10_0: bool = fn_state.gs_137823;
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
        // D s_11_1: write-var gs#137824 <= s_11_0
        fn_state.gs_137824 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#137824:u8
        let s_12_0: bool = fn_state.gs_137824;
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
        // D s_13_1: write-var gs#137825 <= s_13_0
        fn_state.gs_137825 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#137825:u8
        let s_14_0: bool = fn_state.gs_137825;
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
        // D s_16_1: write-var gs#137826 <= s_16_0
        fn_state.gs_137826 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#137826:u8
        let s_17_0: bool = fn_state.gs_137826;
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
        // S s_19_1: call TTBR0_EL1_read(s_19_0)
        let s_19_1: ProductType782ac6922b48c20d = TTBR0_EL1_read(state, tracer, s_19_0);
        // D s_19_2: write-var ga#240120 <= s_19_1
        fn_state.ga_240120 = s_19_1;
        // D s_19_3: read-var ga#240120.0:struct
        let s_19_3: u128 = fn_state.ga_240120._0;
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
        // D s_19_40: call Mk_TTBR0_EL1_Type(s_19_39)
        let s_19_40: ProductType782ac6922b48c20d = Mk_TTBR0_EL1_Type(
            state,
            tracer,
            s_19_39,
        );
        // D s_19_41: call TTBR0_EL1_write(s_19_40)
        let s_19_41: () = TTBR0_EL1_write(state, tracer, s_19_40);
        // N s_19_42: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call TTBR0_EL2_read(s_20_0)
        let s_20_1: ProductType782ac6922b48c20d = TTBR0_EL2_read(state, tracer, s_20_0);
        // D s_20_2: write-var ga#240110 <= s_20_1
        fn_state.ga_240110 = s_20_1;
        // D s_20_3: read-var ga#240110.0:struct
        let s_20_3: u128 = fn_state.ga_240110._0;
        // C s_20_4: const #1s : i
        let s_20_4: i128 = 1;
        // D s_20_5: read-var t:i
        let s_20_5: i128 = fn_state.t;
        // D s_20_6: add s_20_5 s_20_4
        let s_20_6: i128 = (s_20_5 + s_20_4);
        // C s_20_7: const #64s : i64
        let s_20_7: i64 = 64;
        // D s_20_8: call X_read(s_20_6, s_20_7)
        let s_20_8: Bits = X_read(state, tracer, s_20_6, s_20_7);
        // D s_20_9: cast reint s_20_8 -> u64
        let s_20_9: u64 = (s_20_8.value() as u64);
        // C s_20_10: const #64s : i64
        let s_20_10: i64 = 64;
        // D s_20_11: read-var t:i
        let s_20_11: i128 = fn_state.t;
        // D s_20_12: call X_read(s_20_11, s_20_10)
        let s_20_12: Bits = X_read(state, tracer, s_20_11, s_20_10);
        // D s_20_13: cast reint s_20_12 -> u64
        let s_20_13: u64 = (s_20_12.value() as u64);
        // D s_20_14: cast zx s_20_9 -> bv
        let s_20_14: Bits = Bits::new(s_20_9 as u128, 64u16);
        // D s_20_15: cast zx s_20_13 -> bv
        let s_20_15: Bits = Bits::new(s_20_13 as u128, 64u16);
        // D s_20_16: cast reint s_20_14 -> u128
        let s_20_16: u128 = (s_20_14.value() as u128);
        // D s_20_17: size-of s_20_14
        let s_20_17: u16 = s_20_14.length();
        // D s_20_18: cast reint s_20_15 -> u128
        let s_20_18: u128 = (s_20_15.value() as u128);
        // D s_20_19: size-of s_20_15
        let s_20_19: u16 = s_20_15.length();
        // D s_20_20: lsl s_20_16 s_20_19
        let s_20_20: u128 = s_20_16 << s_20_19;
        // D s_20_21: or s_20_20 s_20_18
        let s_20_21: u128 = ((s_20_20) | (s_20_18));
        // D s_20_22: add s_20_17 s_20_19
        let s_20_22: u16 = (s_20_17 + s_20_19);
        // D s_20_23: create-bits s_20_21 s_20_22
        let s_20_23: Bits = Bits::new(s_20_21, s_20_22);
        // D s_20_24: cast reint s_20_23 -> u128
        let s_20_24: u128 = (s_20_23.value() as u128);
        // C s_20_25: const #0s : i
        let s_20_25: i128 = 0;
        // D s_20_26: cast zx s_20_3 -> bv
        let s_20_26: Bits = Bits::new(s_20_3 as u128, 128u16);
        // D s_20_27: cast zx s_20_24 -> bv
        let s_20_27: Bits = Bits::new(s_20_24 as u128, 128u16);
        // C s_20_28: const #127s : i
        let s_20_28: i128 = 127;
        // C s_20_29: const #1u : u64
        let s_20_29: u64 = 1;
        // C s_20_30: cast zx s_20_29 -> bv
        let s_20_30: Bits = Bits::new(s_20_29 as u128, 64u16);
        // C s_20_31: lsl s_20_30 s_20_28
        let s_20_31: Bits = s_20_30 << s_20_28;
        // C s_20_32: sub s_20_31 s_20_30
        let s_20_32: Bits = ((s_20_31) - (s_20_30));
        // D s_20_33: and s_20_27 s_20_32
        let s_20_33: Bits = ((s_20_27) & (s_20_32));
        // D s_20_34: lsl s_20_33 s_20_25
        let s_20_34: Bits = s_20_33 << s_20_25;
        // C s_20_35: lsl s_20_32 s_20_25
        let s_20_35: Bits = s_20_32 << s_20_25;
        // C s_20_36: cmpl s_20_35
        let s_20_36: Bits = !s_20_35;
        // D s_20_37: and s_20_26 s_20_36
        let s_20_37: Bits = ((s_20_26) & (s_20_36));
        // D s_20_38: or s_20_37 s_20_34
        let s_20_38: Bits = ((s_20_37) | (s_20_34));
        // D s_20_39: cast reint s_20_38 -> u128
        let s_20_39: u128 = (s_20_38.value() as u128);
        // D s_20_40: call Mk_TTBR0_EL2_Type(s_20_39)
        let s_20_40: ProductType782ac6922b48c20d = Mk_TTBR0_EL2_Type(
            state,
            tracer,
            s_20_39,
        );
        // D s_20_41: call TTBR0_EL2_write(s_20_40)
        let s_20_41: () = TTBR0_EL2_write(state, tracer, s_20_40);
        // N s_20_42: return
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
        // D s_22_1: write-var gs#137837 <= s_22_0
        fn_state.gs_137837 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#137837:u8
        let s_23_0: bool = fn_state.gs_137837;
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
        // D s_26_5: write-var gs#137837 <= s_26_4
        fn_state.gs_137837 = s_26_4;
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
        // D s_27_5: write-var gs#137826 <= s_27_4
        fn_state.gs_137826 = s_27_4;
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
        // D s_29_5: write-var gs#137825 <= s_29_4
        fn_state.gs_137825 = s_29_4;
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
        // D s_30_2: write-var gs#137824 <= s_30_1
        fn_state.gs_137824 = s_30_1;
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
        // D s_31_5: write-var gs#137823 <= s_31_4
        fn_state.gs_137823 = s_31_4;
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
        // D s_32_4: write-var gs#137822 <= s_32_3
        fn_state.gs_137822 = s_32_3;
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
        // N s_33_2: branch s_33_1 b89 b34
        if s_33_1 {
            return block_89(state, tracer, fn_state);
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
        // D s_34_1: write-var gs#137838 <= s_34_0
        fn_state.gs_137838 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#137838:u8
        let s_35_0: bool = fn_state.gs_137838;
        // N s_35_1: branch s_35_0 b88 b36
        if s_35_0 {
            return block_88(state, tracer, fn_state);
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
        // D s_36_1: write-var gs#137839 <= s_36_0
        fn_state.gs_137839 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#137839:u8
        let s_37_0: bool = fn_state.gs_137839;
        // N s_37_1: branch s_37_0 b87 b38
        if s_37_0 {
            return block_87(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#137840 <= s_38_0
        fn_state.gs_137840 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#137840:u8
        let s_39_0: bool = fn_state.gs_137840;
        // N s_39_1: branch s_39_0 b86 b40
        if s_39_0 {
            return block_86(state, tracer, fn_state);
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
        // D s_40_1: write-var gs#137841 <= s_40_0
        fn_state.gs_137841 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#137841:u8
        let s_41_0: bool = fn_state.gs_137841;
        // N s_41_1: branch s_41_0 b85 b42
        if s_41_0 {
            return block_85(state, tracer, fn_state);
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
        // N s_42_2: branch s_42_1 b84 b43
        if s_42_1 {
            return block_84(state, tracer, fn_state);
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
        // D s_43_1: write-var gs#137842 <= s_43_0
        fn_state.gs_137842 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#137842:u8
        let s_44_0: bool = fn_state.gs_137842;
        // N s_44_1: branch s_44_0 b83 b45
        if s_44_0 {
            return block_83(state, tracer, fn_state);
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
        // N s_45_2: branch s_45_1 b82 b46
        if s_45_1 {
            return block_82(state, tracer, fn_state);
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
        // D s_46_1: write-var gs#137843 <= s_46_0
        fn_state.gs_137843 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#137843:u8
        let s_47_0: bool = fn_state.gs_137843;
        // N s_47_1: branch s_47_0 b78 b48
        if s_47_0 {
            return block_78(state, tracer, fn_state);
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
        // D s_48_1: write-var gs#137845 <= s_48_0
        fn_state.gs_137845 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#137845:u8
        let s_49_0: bool = fn_state.gs_137845;
        // N s_49_1: branch s_49_0 b77 b50
        if s_49_0 {
            return block_77(state, tracer, fn_state);
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
        // D s_50_1: write-var gs#137846 <= s_50_0
        fn_state.gs_137846 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#137846:u8
        let s_51_0: bool = fn_state.gs_137846;
        // N s_51_1: branch s_51_0 b76 b52
        if s_51_0 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #() : ()
        let s_52_0: () = ();
        // S s_52_1: call EL2Enabled(s_52_0)
        let s_52_1: bool = EL2Enabled(state, tracer, s_52_0);
        // N s_52_2: branch s_52_1 b72 b53
        if s_52_1 {
            return block_72(state, tracer, fn_state);
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
        // D s_53_1: write-var gs#137848 <= s_53_0
        fn_state.gs_137848 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#137848:u8
        let s_54_0: bool = fn_state.gs_137848;
        // N s_54_1: branch s_54_0 b71 b55
        if s_54_0 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #424u : u32
        let s_55_0: u32 = 424;
        // D s_55_1: read-reg s_55_0:u8
        let s_55_1: u8 = {
            let value = state.read_register::<u8>(s_55_0 as isize);
            tracer.read_register(s_55_0 as isize, value);
            value
        };
        // C s_55_2: const #2u : u8
        let s_55_2: u8 = 2;
        // D s_55_3: cmp-lt s_55_1 s_55_2
        let s_55_3: bool = ((s_55_1) < (s_55_2));
        // N s_55_4: branch s_55_3 b70 b56
        if s_55_3 {
            return block_70(state, tracer, fn_state);
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
        // D s_56_1: write-var gs#137849 <= s_56_0
        fn_state.gs_137849 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#137849:u8
        let s_57_0: bool = fn_state.gs_137849;
        // N s_57_1: branch s_57_0 b64 b58
        if s_57_0 {
            return block_64(state, tracer, fn_state);
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
        // S s_58_1: call EL2Enabled(s_58_0)
        let s_58_1: bool = EL2Enabled(state, tracer, s_58_0);
        // N s_58_2: branch s_58_1 b63 b59
        if s_58_1 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #0u : u8
        let s_59_0: bool = false;
        // D s_59_1: write-var gs#137850 <= s_59_0
        fn_state.gs_137850 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#137850:u8
        let s_60_0: bool = fn_state.gs_137850;
        // N s_60_1: branch s_60_0 b62 b61
        if s_60_0 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #() : ()
        let s_61_0: () = ();
        // S s_61_1: call TTBR0_EL1_read(s_61_0)
        let s_61_1: ProductType782ac6922b48c20d = TTBR0_EL1_read(state, tracer, s_61_0);
        // D s_61_2: write-var ga#240086 <= s_61_1
        fn_state.ga_240086 = s_61_1;
        // D s_61_3: read-var ga#240086.0:struct
        let s_61_3: u128 = fn_state.ga_240086._0;
        // C s_61_4: const #1s : i
        let s_61_4: i128 = 1;
        // D s_61_5: read-var t:i
        let s_61_5: i128 = fn_state.t;
        // D s_61_6: add s_61_5 s_61_4
        let s_61_6: i128 = (s_61_5 + s_61_4);
        // C s_61_7: const #64s : i64
        let s_61_7: i64 = 64;
        // D s_61_8: call X_read(s_61_6, s_61_7)
        let s_61_8: Bits = X_read(state, tracer, s_61_6, s_61_7);
        // D s_61_9: cast reint s_61_8 -> u64
        let s_61_9: u64 = (s_61_8.value() as u64);
        // C s_61_10: const #64s : i64
        let s_61_10: i64 = 64;
        // D s_61_11: read-var t:i
        let s_61_11: i128 = fn_state.t;
        // D s_61_12: call X_read(s_61_11, s_61_10)
        let s_61_12: Bits = X_read(state, tracer, s_61_11, s_61_10);
        // D s_61_13: cast reint s_61_12 -> u64
        let s_61_13: u64 = (s_61_12.value() as u64);
        // D s_61_14: cast zx s_61_9 -> bv
        let s_61_14: Bits = Bits::new(s_61_9 as u128, 64u16);
        // D s_61_15: cast zx s_61_13 -> bv
        let s_61_15: Bits = Bits::new(s_61_13 as u128, 64u16);
        // D s_61_16: cast reint s_61_14 -> u128
        let s_61_16: u128 = (s_61_14.value() as u128);
        // D s_61_17: size-of s_61_14
        let s_61_17: u16 = s_61_14.length();
        // D s_61_18: cast reint s_61_15 -> u128
        let s_61_18: u128 = (s_61_15.value() as u128);
        // D s_61_19: size-of s_61_15
        let s_61_19: u16 = s_61_15.length();
        // D s_61_20: lsl s_61_16 s_61_19
        let s_61_20: u128 = s_61_16 << s_61_19;
        // D s_61_21: or s_61_20 s_61_18
        let s_61_21: u128 = ((s_61_20) | (s_61_18));
        // D s_61_22: add s_61_17 s_61_19
        let s_61_22: u16 = (s_61_17 + s_61_19);
        // D s_61_23: create-bits s_61_21 s_61_22
        let s_61_23: Bits = Bits::new(s_61_21, s_61_22);
        // D s_61_24: cast reint s_61_23 -> u128
        let s_61_24: u128 = (s_61_23.value() as u128);
        // C s_61_25: const #0s : i
        let s_61_25: i128 = 0;
        // D s_61_26: cast zx s_61_3 -> bv
        let s_61_26: Bits = Bits::new(s_61_3 as u128, 128u16);
        // D s_61_27: cast zx s_61_24 -> bv
        let s_61_27: Bits = Bits::new(s_61_24 as u128, 128u16);
        // C s_61_28: const #127s : i
        let s_61_28: i128 = 127;
        // C s_61_29: const #1u : u64
        let s_61_29: u64 = 1;
        // C s_61_30: cast zx s_61_29 -> bv
        let s_61_30: Bits = Bits::new(s_61_29 as u128, 64u16);
        // C s_61_31: lsl s_61_30 s_61_28
        let s_61_31: Bits = s_61_30 << s_61_28;
        // C s_61_32: sub s_61_31 s_61_30
        let s_61_32: Bits = ((s_61_31) - (s_61_30));
        // D s_61_33: and s_61_27 s_61_32
        let s_61_33: Bits = ((s_61_27) & (s_61_32));
        // D s_61_34: lsl s_61_33 s_61_25
        let s_61_34: Bits = s_61_33 << s_61_25;
        // C s_61_35: lsl s_61_32 s_61_25
        let s_61_35: Bits = s_61_32 << s_61_25;
        // C s_61_36: cmpl s_61_35
        let s_61_36: Bits = !s_61_35;
        // D s_61_37: and s_61_26 s_61_36
        let s_61_37: Bits = ((s_61_26) & (s_61_36));
        // D s_61_38: or s_61_37 s_61_34
        let s_61_38: Bits = ((s_61_37) | (s_61_34));
        // D s_61_39: cast reint s_61_38 -> u128
        let s_61_39: u128 = (s_61_38.value() as u128);
        // D s_61_40: call Mk_TTBR0_EL1_Type(s_61_39)
        let s_61_40: ProductType782ac6922b48c20d = Mk_TTBR0_EL1_Type(
            state,
            tracer,
            s_61_39,
        );
        // D s_61_41: call TTBR0_EL1_write(s_61_40)
        let s_61_41: () = TTBR0_EL1_write(state, tracer, s_61_40);
        // N s_61_42: return
        return;
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #512u : u12
        let s_62_0: u16 = 512;
        // C s_62_1: cast zx s_62_0 -> bv
        let s_62_1: Bits = Bits::new(s_62_0 as u128, 12u16);
        // C s_62_2: cast zx s_62_1 -> i
        let s_62_2: i128 = (s_62_1.value() as i128);
        // C s_62_3: cast reint s_62_2 -> i64
        let s_62_3: i64 = (s_62_2 as i64);
        // C s_62_4: const #128s : i64
        let s_62_4: i64 = 128;
        // C s_62_5: const #1s : i
        let s_62_5: i128 = 1;
        // D s_62_6: read-var t:i
        let s_62_6: i128 = fn_state.t;
        // D s_62_7: add s_62_6 s_62_5
        let s_62_7: i128 = (s_62_6 + s_62_5);
        // C s_62_8: const #64s : i64
        let s_62_8: i64 = 64;
        // D s_62_9: call X_read(s_62_7, s_62_8)
        let s_62_9: Bits = X_read(state, tracer, s_62_7, s_62_8);
        // D s_62_10: cast reint s_62_9 -> u64
        let s_62_10: u64 = (s_62_9.value() as u64);
        // C s_62_11: const #64s : i64
        let s_62_11: i64 = 64;
        // D s_62_12: read-var t:i
        let s_62_12: i128 = fn_state.t;
        // D s_62_13: call X_read(s_62_12, s_62_11)
        let s_62_13: Bits = X_read(state, tracer, s_62_12, s_62_11);
        // D s_62_14: cast reint s_62_13 -> u64
        let s_62_14: u64 = (s_62_13.value() as u64);
        // D s_62_15: cast zx s_62_10 -> bv
        let s_62_15: Bits = Bits::new(s_62_10 as u128, 64u16);
        // D s_62_16: cast zx s_62_14 -> bv
        let s_62_16: Bits = Bits::new(s_62_14 as u128, 64u16);
        // D s_62_17: cast reint s_62_15 -> u128
        let s_62_17: u128 = (s_62_15.value() as u128);
        // D s_62_18: size-of s_62_15
        let s_62_18: u16 = s_62_15.length();
        // D s_62_19: cast reint s_62_16 -> u128
        let s_62_19: u128 = (s_62_16.value() as u128);
        // D s_62_20: size-of s_62_16
        let s_62_20: u16 = s_62_16.length();
        // D s_62_21: lsl s_62_17 s_62_20
        let s_62_21: u128 = s_62_17 << s_62_20;
        // D s_62_22: or s_62_21 s_62_19
        let s_62_22: u128 = ((s_62_21) | (s_62_19));
        // D s_62_23: add s_62_18 s_62_20
        let s_62_23: u16 = (s_62_18 + s_62_20);
        // D s_62_24: create-bits s_62_22 s_62_23
        let s_62_24: Bits = Bits::new(s_62_22, s_62_23);
        // D s_62_25: cast reint s_62_24 -> u128
        let s_62_25: u128 = (s_62_24.value() as u128);
        // C s_62_26: cast zx s_62_3 -> i
        let s_62_26: i128 = (i128::try_from(s_62_3).unwrap());
        // D s_62_27: cast zx s_62_25 -> bv
        let s_62_27: Bits = Bits::new(s_62_25 as u128, 128u16);
        // D s_62_28: call NVMem_set__1(s_62_26, s_62_4, s_62_27)
        let s_62_28: () = NVMem_set__1(state, tracer, s_62_26, s_62_4, s_62_27);
        // N s_62_29: return
        return;
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #102552u : u32
        let s_63_0: u32 = 102552;
        // D s_63_1: read-reg s_63_0:struct
        let s_63_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_63_0 as isize);
            tracer.read_register(s_63_0 as isize, value);
            value
        };
        // D s_63_2: call _get_HCR_EL2_Type_NV2(s_63_1)
        let s_63_2: bool = u_get_HCR_EL2_Type_NV2(state, tracer, s_63_1);
        // C s_63_3: const #102552u : u32
        let s_63_3: u32 = 102552;
        // D s_63_4: read-reg s_63_3:struct
        let s_63_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_63_3 as isize);
            tracer.read_register(s_63_3 as isize, value);
            value
        };
        // D s_63_5: call _get_HCR_EL2_Type_NV1(s_63_4)
        let s_63_5: bool = u_get_HCR_EL2_Type_NV1(state, tracer, s_63_4);
        // C s_63_6: const #102552u : u32
        let s_63_6: u32 = 102552;
        // D s_63_7: read-reg s_63_6:struct
        let s_63_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_63_6 as isize);
            tracer.read_register(s_63_6 as isize, value);
            value
        };
        // D s_63_8: call _get_HCR_EL2_Type_NV(s_63_7)
        let s_63_8: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_63_7);
        // D s_63_9: cast zx s_63_5 -> bv
        let s_63_9: Bits = Bits::new(s_63_5 as u128, 1u16);
        // D s_63_10: cast zx s_63_8 -> bv
        let s_63_10: Bits = Bits::new(s_63_8 as u128, 1u16);
        // D s_63_11: cast reint s_63_9 -> u128
        let s_63_11: u128 = (s_63_9.value() as u128);
        // D s_63_12: size-of s_63_9
        let s_63_12: u16 = s_63_9.length();
        // D s_63_13: cast reint s_63_10 -> u128
        let s_63_13: u128 = (s_63_10.value() as u128);
        // D s_63_14: size-of s_63_10
        let s_63_14: u16 = s_63_10.length();
        // D s_63_15: lsl s_63_11 s_63_14
        let s_63_15: u128 = s_63_11 << s_63_14;
        // D s_63_16: or s_63_15 s_63_13
        let s_63_16: u128 = ((s_63_15) | (s_63_13));
        // D s_63_17: add s_63_12 s_63_14
        let s_63_17: u16 = (s_63_12 + s_63_14);
        // D s_63_18: create-bits s_63_16 s_63_17
        let s_63_18: Bits = Bits::new(s_63_16, s_63_17);
        // D s_63_19: cast reint s_63_18 -> u8
        let s_63_19: u8 = (s_63_18.value() as u8);
        // D s_63_20: cast zx s_63_2 -> bv
        let s_63_20: Bits = Bits::new(s_63_2 as u128, 1u16);
        // D s_63_21: cast zx s_63_19 -> bv
        let s_63_21: Bits = Bits::new(s_63_19 as u128, 2u16);
        // D s_63_22: cast reint s_63_20 -> u128
        let s_63_22: u128 = (s_63_20.value() as u128);
        // D s_63_23: size-of s_63_20
        let s_63_23: u16 = s_63_20.length();
        // D s_63_24: cast reint s_63_21 -> u128
        let s_63_24: u128 = (s_63_21.value() as u128);
        // D s_63_25: size-of s_63_21
        let s_63_25: u16 = s_63_21.length();
        // D s_63_26: lsl s_63_22 s_63_25
        let s_63_26: u128 = s_63_22 << s_63_25;
        // D s_63_27: or s_63_26 s_63_24
        let s_63_27: u128 = ((s_63_26) | (s_63_24));
        // D s_63_28: add s_63_23 s_63_25
        let s_63_28: u16 = (s_63_23 + s_63_25);
        // D s_63_29: create-bits s_63_27 s_63_28
        let s_63_29: Bits = Bits::new(s_63_27, s_63_28);
        // D s_63_30: cast reint s_63_29 -> u8
        let s_63_30: u8 = (s_63_29.value() as u8);
        // D s_63_31: cast zx s_63_30 -> bv
        let s_63_31: Bits = Bits::new(s_63_30 as u128, 3u16);
        // C s_63_32: const #7u : u8
        let s_63_32: u8 = 7;
        // C s_63_33: cast zx s_63_32 -> bv
        let s_63_33: Bits = Bits::new(s_63_32 as u128, 3u16);
        // D s_63_34: cmp-eq s_63_31 s_63_33
        let s_63_34: bool = ((s_63_31) == (s_63_33));
        // D s_63_35: write-var gs#137850 <= s_63_34
        fn_state.gs_137850 = s_63_34;
        // N s_63_36: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #() : ()
        let s_64_0: () = ();
        // S s_64_1: call Halted(s_64_0)
        let s_64_1: bool = Halted(state, tracer, s_64_0);
        // N s_64_2: branch s_64_1 b69 b65
        if s_64_1 {
            return block_69(state, tracer, fn_state);
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
        // D s_65_1: write-var gs#137860 <= s_65_0
        fn_state.gs_137860 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#137860:u8
        let s_66_0: bool = fn_state.gs_137860;
        // N s_66_1: branch s_66_0 b68 b67
        if s_66_0 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #20u : u8
        let s_67_0: u8 = 20;
        // C s_67_1: cast zx s_67_0 -> bv
        let s_67_1: Bits = Bits::new(s_67_0 as u128, 8u16);
        // C s_67_2: cast zx s_67_1 -> i
        let s_67_2: i128 = (s_67_1.value() as i128);
        // C s_67_3: cast reint s_67_2 -> i64
        let s_67_3: i64 = (s_67_2 as i64);
        // C s_67_4: cast zx s_67_3 -> i
        let s_67_4: i128 = (i128::try_from(s_67_3).unwrap());
        // C s_67_5: const #424u : u32
        let s_67_5: u32 = 424;
        // D s_67_6: read-reg s_67_5:u8
        let s_67_6: u8 = {
            let value = state.read_register::<u8>(s_67_5 as isize);
            tracer.read_register(s_67_5 as isize, value);
            value
        };
        // D s_67_7: call AArch64_SystemAccessTrap(s_67_6, s_67_4)
        let s_67_7: () = AArch64_SystemAccessTrap(state, tracer, s_67_6, s_67_4);
        // N s_67_8: return
        return;
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_68_0: panic
        panic!("{:?}", ());
        // N s_68_1: return
        return;
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var __EDSCR_SDD:u8
        let s_69_0: bool = fn_state.u__EDSCR_SDD;
        // D s_69_1: cast zx s_69_0 -> bv
        let s_69_1: Bits = Bits::new(s_69_0 as u128, 1u16);
        // C s_69_2: const #1u : u8
        let s_69_2: bool = true;
        // C s_69_3: cast zx s_69_2 -> bv
        let s_69_3: Bits = Bits::new(s_69_2 as u128, 1u16);
        // D s_69_4: cmp-eq s_69_1 s_69_3
        let s_69_4: bool = ((s_69_1) == (s_69_3));
        // D s_69_5: write-var gs#137860 <= s_69_4
        fn_state.gs_137860 = s_69_4;
        // N s_69_6: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var __SCR_EL3_D128En:u8
        let s_70_0: bool = fn_state.u__SCR_EL3_D128En;
        // D s_70_1: cast zx s_70_0 -> bv
        let s_70_1: Bits = Bits::new(s_70_0 as u128, 1u16);
        // C s_70_2: const #0u : u8
        let s_70_2: bool = false;
        // C s_70_3: cast zx s_70_2 -> bv
        let s_70_3: Bits = Bits::new(s_70_2 as u128, 1u16);
        // D s_70_4: cmp-eq s_70_1 s_70_3
        let s_70_4: bool = ((s_70_1) == (s_70_3));
        // D s_70_5: write-var gs#137849 <= s_70_4
        fn_state.gs_137849 = s_70_4;
        // N s_70_6: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #20u : u8
        let s_71_0: u8 = 20;
        // C s_71_1: cast zx s_71_0 -> bv
        let s_71_1: Bits = Bits::new(s_71_0 as u128, 8u16);
        // C s_71_2: cast zx s_71_1 -> i
        let s_71_2: i128 = (s_71_1.value() as i128);
        // C s_71_3: cast reint s_71_2 -> i64
        let s_71_3: i64 = (s_71_2 as i64);
        // C s_71_4: cast zx s_71_3 -> i
        let s_71_4: i128 = (i128::try_from(s_71_3).unwrap());
        // C s_71_5: const #432u : u32
        let s_71_5: u32 = 432;
        // D s_71_6: read-reg s_71_5:u8
        let s_71_6: u8 = {
            let value = state.read_register::<u8>(s_71_5 as isize);
            tracer.read_register(s_71_5 as isize, value);
            value
        };
        // D s_71_7: call AArch64_SystemAccessTrap(s_71_6, s_71_4)
        let s_71_7: () = AArch64_SystemAccessTrap(state, tracer, s_71_6, s_71_4);
        // N s_71_8: return
        return;
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #() : ()
        let s_72_0: () = ();
        // S s_72_1: call IsHCRXEL2Enabled(s_72_0)
        let s_72_1: bool = IsHCRXEL2Enabled(state, tracer, s_72_0);
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
        // D s_73_0: read-var __HCRX_EL2_D128En:u8
        let s_73_0: bool = fn_state.u__HCRX_EL2_D128En;
        // D s_73_1: cast zx s_73_0 -> bv
        let s_73_1: Bits = Bits::new(s_73_0 as u128, 1u16);
        // C s_73_2: const #0u : u8
        let s_73_2: bool = false;
        // C s_73_3: cast zx s_73_2 -> bv
        let s_73_3: Bits = Bits::new(s_73_2 as u128, 1u16);
        // D s_73_4: cmp-eq s_73_1 s_73_3
        let s_73_4: bool = ((s_73_1) == (s_73_3));
        // D s_73_5: write-var gs#137847 <= s_73_4
        fn_state.gs_137847 = s_73_4;
        // N s_73_6: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var gs#137847:u8
        let s_74_0: bool = fn_state.gs_137847;
        // D s_74_1: write-var gs#137848 <= s_74_0
        fn_state.gs_137848 = s_74_0;
        // N s_74_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #1u : u8
        let s_75_0: bool = true;
        // D s_75_1: write-var gs#137847 <= s_75_0
        fn_state.gs_137847 = s_75_0;
        // N s_75_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #20u : u8
        let s_76_0: u8 = 20;
        // C s_76_1: cast zx s_76_0 -> bv
        let s_76_1: Bits = Bits::new(s_76_0 as u128, 8u16);
        // C s_76_2: cast zx s_76_1 -> i
        let s_76_2: i128 = (s_76_1.value() as i128);
        // C s_76_3: cast reint s_76_2 -> i64
        let s_76_3: i64 = (s_76_2 as i64);
        // C s_76_4: cast zx s_76_3 -> i
        let s_76_4: i128 = (i128::try_from(s_76_3).unwrap());
        // C s_76_5: const #432u : u32
        let s_76_5: u32 = 432;
        // D s_76_6: read-reg s_76_5:u8
        let s_76_6: u8 = {
            let value = state.read_register::<u8>(s_76_5 as isize);
            tracer.read_register(s_76_5 as isize, value);
            value
        };
        // D s_76_7: call AArch64_SystemAccessTrap(s_76_6, s_76_4)
        let s_76_7: () = AArch64_SystemAccessTrap(state, tracer, s_76_6, s_76_4);
        // N s_76_8: return
        return;
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var __HFGWTR_EL2_TTBR0_EL1:u8
        let s_77_0: bool = fn_state.u__HFGWTR_EL2_TTBR0_EL1;
        // D s_77_1: cast zx s_77_0 -> bv
        let s_77_1: Bits = Bits::new(s_77_0 as u128, 1u16);
        // C s_77_2: const #1u : u8
        let s_77_2: bool = true;
        // C s_77_3: cast zx s_77_2 -> bv
        let s_77_3: Bits = Bits::new(s_77_2 as u128, 1u16);
        // D s_77_4: cmp-eq s_77_1 s_77_3
        let s_77_4: bool = ((s_77_1) == (s_77_3));
        // D s_77_5: write-var gs#137846 <= s_77_4
        fn_state.gs_137846 = s_77_4;
        // N s_77_6: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #424u : u32
        let s_78_0: u32 = 424;
        // D s_78_1: read-reg s_78_0:u8
        let s_78_1: u8 = {
            let value = state.read_register::<u8>(s_78_0 as isize);
            tracer.read_register(s_78_0 as isize, value);
            value
        };
        // C s_78_2: const #2u : u8
        let s_78_2: u8 = 2;
        // D s_78_3: cmp-lt s_78_1 s_78_2
        let s_78_3: bool = ((s_78_1) < (s_78_2));
        // D s_78_4: not s_78_3
        let s_78_4: bool = !s_78_3;
        // N s_78_5: branch s_78_4 b81 b79
        if s_78_4 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var __SCR_EL3_FGTEn:u8
        let s_79_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_79_1: cast zx s_79_0 -> bv
        let s_79_1: Bits = Bits::new(s_79_0 as u128, 1u16);
        // C s_79_2: const #1u : u8
        let s_79_2: bool = true;
        // C s_79_3: cast zx s_79_2 -> bv
        let s_79_3: Bits = Bits::new(s_79_2 as u128, 1u16);
        // D s_79_4: cmp-eq s_79_1 s_79_3
        let s_79_4: bool = ((s_79_1) == (s_79_3));
        // D s_79_5: write-var gs#137844 <= s_79_4
        fn_state.gs_137844 = s_79_4;
        // N s_79_6: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var gs#137844:u8
        let s_80_0: bool = fn_state.gs_137844;
        // D s_80_1: write-var gs#137845 <= s_80_0
        fn_state.gs_137845 = s_80_0;
        // N s_80_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #1u : u8
        let s_81_0: bool = true;
        // D s_81_1: write-var gs#137844 <= s_81_0
        fn_state.gs_137844 = s_81_0;
        // N s_81_2: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #146u : u32
        let s_82_0: u32 = 146;
        // S s_82_1: call IsFeatureImplemented(s_82_0)
        let s_82_1: bool = IsFeatureImplemented(state, tracer, s_82_0);
        // D s_82_2: write-var gs#137843 <= s_82_1
        fn_state.gs_137843 = s_82_1;
        // N s_82_3: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #20u : u8
        let s_83_0: u8 = 20;
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
        // D s_84_0: read-var __HCR_EL2_TVM:u8
        let s_84_0: bool = fn_state.u__HCR_EL2_TVM;
        // D s_84_1: cast zx s_84_0 -> bv
        let s_84_1: Bits = Bits::new(s_84_0 as u128, 1u16);
        // C s_84_2: const #1u : u8
        let s_84_2: bool = true;
        // C s_84_3: cast zx s_84_2 -> bv
        let s_84_3: Bits = Bits::new(s_84_2 as u128, 1u16);
        // D s_84_4: cmp-eq s_84_1 s_84_3
        let s_84_4: bool = ((s_84_1) == (s_84_3));
        // D s_84_5: write-var gs#137842 <= s_84_4
        fn_state.gs_137842 = s_84_4;
        // N s_84_6: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_85_0: panic
        panic!("{:?}", ());
        // N s_85_1: return
        return;
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var __SCR_EL3_D128En:u8
        let s_86_0: bool = fn_state.u__SCR_EL3_D128En;
        // D s_86_1: cast zx s_86_0 -> bv
        let s_86_1: Bits = Bits::new(s_86_0 as u128, 1u16);
        // C s_86_2: const #0u : u8
        let s_86_2: bool = false;
        // C s_86_3: cast zx s_86_2 -> bv
        let s_86_3: Bits = Bits::new(s_86_2 as u128, 1u16);
        // D s_86_4: cmp-eq s_86_1 s_86_3
        let s_86_4: bool = ((s_86_1) == (s_86_3));
        // D s_86_5: write-var gs#137841 <= s_86_4
        fn_state.gs_137841 = s_86_4;
        // N s_86_6: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_87_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_87_1: call __IMPDEF_boolean(s_87_0)
        let s_87_1: bool = u__IMPDEF_boolean(state, tracer, s_87_0);
        // D s_87_2: write-var gs#137840 <= s_87_1
        fn_state.gs_137840 = s_87_1;
        // N s_87_3: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var __EDSCR_SDD:u8
        let s_88_0: bool = fn_state.u__EDSCR_SDD;
        // D s_88_1: cast zx s_88_0 -> bv
        let s_88_1: Bits = Bits::new(s_88_0 as u128, 1u16);
        // C s_88_2: const #1u : u8
        let s_88_2: bool = true;
        // C s_88_3: cast zx s_88_2 -> bv
        let s_88_3: Bits = Bits::new(s_88_2 as u128, 1u16);
        // D s_88_4: cmp-eq s_88_1 s_88_3
        let s_88_4: bool = ((s_88_1) == (s_88_3));
        // D s_88_5: write-var gs#137839 <= s_88_4
        fn_state.gs_137839 = s_88_4;
        // N s_88_6: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #424u : u32
        let s_89_0: u32 = 424;
        // D s_89_1: read-reg s_89_0:u8
        let s_89_1: u8 = {
            let value = state.read_register::<u8>(s_89_0 as isize);
            tracer.read_register(s_89_0 as isize, value);
            value
        };
        // C s_89_2: const #2u : u8
        let s_89_2: u8 = 2;
        // D s_89_3: cmp-lt s_89_1 s_89_2
        let s_89_3: bool = ((s_89_1) < (s_89_2));
        // D s_89_4: write-var gs#137838 <= s_89_3
        fn_state.gs_137838 = s_89_3;
        // N s_89_5: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_90_0: panic
        panic!("{:?}", ());
        // N s_90_1: return
        return;
    }
}
