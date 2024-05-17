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
use u_get_HCR_EL2_Type_E2H::*;
use u_get_HFGWTR_EL2_Type_TTBR1_EL1::*;
use IsFeatureImplemented::*;
use AArch64_SystemAccessTrap::*;
use X_read::*;
use NVMem_set::*;
use u_get_HCR_EL2_Type_NV1::*;
use Mk_TTBR1_EL1_Type::*;
use u_get_HCR_EL2_Type_TVM::*;
use u_get_HCR_EL2_Type_NV::*;
use TTBR1_EL1_write::*;
use TTBR1_EL1_read::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_NV2::*;
use common::*;
pub fn TTBR1_EL1_SysRegWrite_949dc27ace2a7dbe<T: Tracer>(
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
        gs_96956: bool,
        u__HCR_EL2_E2H: bool,
        gs_96954: bool,
        gs_96955: bool,
        ga_147716: ProductType782ac6922b48c20d,
        u__SCR_EL3_FGTEn: bool,
        ga_147723: ProductType782ac6922b48c20d,
        gs_96957: bool,
        u__HCR_EL2_TVM: bool,
        u__HFGWTR_EL2_TTBR1_EL1: bool,
        gs_96952: bool,
        gs_96953: bool,
        ga_147704: ProductType782ac6922b48c20d,
        u__PSTATE_EL: u8,
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
        // D s_0_5: call _get_HCR_EL2_Type_TVM(s_0_4)
        let s_0_5: bool = u_get_HCR_EL2_Type_TVM(state, tracer, s_0_4);
        // D s_0_6: write-var __HCR_EL2_TVM <= s_0_5
        fn_state.u__HCR_EL2_TVM = s_0_5;
        // C s_0_7: const #90704u : u32
        let s_0_7: u32 = 90704;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_SCR_EL3_Type_FGTEn(s_0_8)
        let s_0_9: bool = u_get_SCR_EL3_Type_FGTEn(state, tracer, s_0_8);
        // D s_0_10: write-var __SCR_EL3_FGTEn <= s_0_9
        fn_state.u__SCR_EL3_FGTEn = s_0_9;
        // C s_0_11: const #100992u : u32
        let s_0_11: u32 = 100992;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_HFGWTR_EL2_Type_TTBR1_EL1(s_0_12)
        let s_0_13: bool = u_get_HFGWTR_EL2_Type_TTBR1_EL1(state, tracer, s_0_12);
        // D s_0_14: write-var __HFGWTR_EL2_TTBR1_EL1 <= s_0_13
        fn_state.u__HFGWTR_EL2_TTBR1_EL1 = s_0_13;
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
        // N s_0_25: branch s_0_24 b34 b1
        if s_0_24 {
            return block_34(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b9 b2
        if s_1_5 {
            return block_9(state, tracer, fn_state);
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
        // D s_5_2: write-var ga#147723 <= s_5_1
        fn_state.ga_147723 = s_5_1;
        // D s_5_3: read-var ga#147723.0:struct
        let s_5_3: u128 = fn_state.ga_147723._0;
        // C s_5_4: const #64s : i64
        let s_5_4: i64 = 64;
        // D s_5_5: read-var t:i
        let s_5_5: i128 = fn_state.t;
        // D s_5_6: call X_read(s_5_5, s_5_4)
        let s_5_6: Bits = X_read(state, tracer, s_5_5, s_5_4);
        // D s_5_7: cast reint s_5_6 -> u64
        let s_5_7: u64 = (s_5_6.value() as u64);
        // C s_5_8: const #0s : i
        let s_5_8: i128 = 0;
        // D s_5_9: cast zx s_5_3 -> bv
        let s_5_9: Bits = Bits::new(s_5_3 as u128, 128u16);
        // D s_5_10: cast zx s_5_7 -> bv
        let s_5_10: Bits = Bits::new(s_5_7 as u128, 64u16);
        // C s_5_11: const #63s : i
        let s_5_11: i128 = 63;
        // C s_5_12: const #1u : u64
        let s_5_12: u64 = 1;
        // C s_5_13: cast zx s_5_12 -> bv
        let s_5_13: Bits = Bits::new(s_5_12 as u128, 64u16);
        // C s_5_14: lsl s_5_13 s_5_11
        let s_5_14: Bits = s_5_13 << s_5_11;
        // C s_5_15: sub s_5_14 s_5_13
        let s_5_15: Bits = ((s_5_14) - (s_5_13));
        // D s_5_16: and s_5_10 s_5_15
        let s_5_16: Bits = ((s_5_10) & (s_5_15));
        // D s_5_17: lsl s_5_16 s_5_8
        let s_5_17: Bits = s_5_16 << s_5_8;
        // C s_5_18: lsl s_5_15 s_5_8
        let s_5_18: Bits = s_5_15 << s_5_8;
        // C s_5_19: cmpl s_5_18
        let s_5_19: Bits = !s_5_18;
        // D s_5_20: and s_5_9 s_5_19
        let s_5_20: Bits = ((s_5_9) & (s_5_19));
        // D s_5_21: or s_5_20 s_5_17
        let s_5_21: Bits = ((s_5_20) | (s_5_17));
        // D s_5_22: cast reint s_5_21 -> u128
        let s_5_22: u128 = (s_5_21.value() as u128);
        // D s_5_23: call Mk_TTBR1_EL1_Type(s_5_22)
        let s_5_23: ProductType782ac6922b48c20d = Mk_TTBR1_EL1_Type(
            state,
            tracer,
            s_5_22,
        );
        // D s_5_24: call TTBR1_EL1_write(s_5_23)
        let s_5_24: () = TTBR1_EL1_write(state, tracer, s_5_23);
        // N s_5_25: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var __HCR_EL2_E2H:u8
        let s_6_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 1u16);
        // C s_6_2: const #1u : u8
        let s_6_2: bool = true;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // N s_6_5: branch s_6_4 b8 b7
        if s_6_4 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call TTBR1_EL1_read(s_7_0)
        let s_7_1: ProductType782ac6922b48c20d = TTBR1_EL1_read(state, tracer, s_7_0);
        // D s_7_2: write-var ga#147716 <= s_7_1
        fn_state.ga_147716 = s_7_1;
        // D s_7_3: read-var ga#147716.0:struct
        let s_7_3: u128 = fn_state.ga_147716._0;
        // C s_7_4: const #64s : i64
        let s_7_4: i64 = 64;
        // D s_7_5: read-var t:i
        let s_7_5: i128 = fn_state.t;
        // D s_7_6: call X_read(s_7_5, s_7_4)
        let s_7_6: Bits = X_read(state, tracer, s_7_5, s_7_4);
        // D s_7_7: cast reint s_7_6 -> u64
        let s_7_7: u64 = (s_7_6.value() as u64);
        // C s_7_8: const #0s : i
        let s_7_8: i128 = 0;
        // D s_7_9: cast zx s_7_3 -> bv
        let s_7_9: Bits = Bits::new(s_7_3 as u128, 128u16);
        // D s_7_10: cast zx s_7_7 -> bv
        let s_7_10: Bits = Bits::new(s_7_7 as u128, 64u16);
        // C s_7_11: const #63s : i
        let s_7_11: i128 = 63;
        // C s_7_12: const #1u : u64
        let s_7_12: u64 = 1;
        // C s_7_13: cast zx s_7_12 -> bv
        let s_7_13: Bits = Bits::new(s_7_12 as u128, 64u16);
        // C s_7_14: lsl s_7_13 s_7_11
        let s_7_14: Bits = s_7_13 << s_7_11;
        // C s_7_15: sub s_7_14 s_7_13
        let s_7_15: Bits = ((s_7_14) - (s_7_13));
        // D s_7_16: and s_7_10 s_7_15
        let s_7_16: Bits = ((s_7_10) & (s_7_15));
        // D s_7_17: lsl s_7_16 s_7_8
        let s_7_17: Bits = s_7_16 << s_7_8;
        // C s_7_18: lsl s_7_15 s_7_8
        let s_7_18: Bits = s_7_15 << s_7_8;
        // C s_7_19: cmpl s_7_18
        let s_7_19: Bits = !s_7_18;
        // D s_7_20: and s_7_9 s_7_19
        let s_7_20: Bits = ((s_7_9) & (s_7_19));
        // D s_7_21: or s_7_20 s_7_17
        let s_7_21: Bits = ((s_7_20) | (s_7_17));
        // D s_7_22: cast reint s_7_21 -> u128
        let s_7_22: u128 = (s_7_21.value() as u128);
        // D s_7_23: call Mk_TTBR1_EL1_Type(s_7_22)
        let s_7_23: ProductType782ac6922b48c20d = Mk_TTBR1_EL1_Type(
            state,
            tracer,
            s_7_22,
        );
        // D s_7_24: call TTBR1_EL1_write(s_7_23)
        let s_7_24: () = TTBR1_EL1_write(state, tracer, s_7_23);
        // N s_7_25: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #64s : i64
        let s_8_0: i64 = 64;
        // D s_8_1: read-var t:i
        let s_8_1: i128 = fn_state.t;
        // D s_8_2: call X_read(s_8_1, s_8_0)
        let s_8_2: Bits = X_read(state, tracer, s_8_1, s_8_0);
        // C s_8_3: const #18432u : u32
        let s_8_3: u32 = 18432;
        // D s_8_4: read-reg s_8_3:struct
        let s_8_4: ProductType782ac6922b48c20d = {
            let value = state
                .read_register::<ProductType782ac6922b48c20d>(s_8_3 as isize);
            tracer.read_register(s_8_3 as isize, value);
            value
        };
        // C s_8_5: const #18432u : u32
        let s_8_5: u32 = 18432;
        // N s_8_6: write-reg s_8_5 <= s_8_4
        let s_8_6: () = {
            state.write_register::<ProductType782ac6922b48c20d>(s_8_5 as isize, s_8_4);
            tracer.write_register(s_8_5 as isize, s_8_4);
        };
        // N s_8_7: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call EL2Enabled(s_9_0)
        let s_9_1: bool = EL2Enabled(state, tracer, s_9_0);
        // N s_9_2: branch s_9_1 b33 b10
        if s_9_1 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#96952 <= s_10_0
        fn_state.gs_96952 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#96952:u8
        let s_11_0: bool = fn_state.gs_96952;
        // N s_11_1: branch s_11_0 b32 b12
        if s_11_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call EL2Enabled(s_12_0)
        let s_12_1: bool = EL2Enabled(state, tracer, s_12_0);
        // N s_12_2: branch s_12_1 b31 b13
        if s_12_1 {
            return block_31(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#96953 <= s_13_0
        fn_state.gs_96953 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#96953:u8
        let s_14_0: bool = fn_state.gs_96953;
        // N s_14_1: branch s_14_0 b27 b15
        if s_14_0 {
            return block_27(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#96955 <= s_15_0
        fn_state.gs_96955 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#96955:u8
        let s_16_0: bool = fn_state.gs_96955;
        // N s_16_1: branch s_16_0 b26 b17
        if s_16_0 {
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
        // D s_17_1: write-var gs#96956 <= s_17_0
        fn_state.gs_96956 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#96956:u8
        let s_18_0: bool = fn_state.gs_96956;
        // N s_18_1: branch s_18_0 b25 b19
        if s_18_0 {
            return block_25(state, tracer, fn_state);
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
        // S s_19_1: call EL2Enabled(s_19_0)
        let s_19_1: bool = EL2Enabled(state, tracer, s_19_0);
        // N s_19_2: branch s_19_1 b24 b20
        if s_19_1 {
            return block_24(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#96957 <= s_20_0
        fn_state.gs_96957 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#96957:u8
        let s_21_0: bool = fn_state.gs_96957;
        // N s_21_1: branch s_21_0 b23 b22
        if s_21_0 {
            return block_23(state, tracer, fn_state);
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
        // S s_22_1: call TTBR1_EL1_read(s_22_0)
        let s_22_1: ProductType782ac6922b48c20d = TTBR1_EL1_read(state, tracer, s_22_0);
        // D s_22_2: write-var ga#147704 <= s_22_1
        fn_state.ga_147704 = s_22_1;
        // D s_22_3: read-var ga#147704.0:struct
        let s_22_3: u128 = fn_state.ga_147704._0;
        // C s_22_4: const #64s : i64
        let s_22_4: i64 = 64;
        // D s_22_5: read-var t:i
        let s_22_5: i128 = fn_state.t;
        // D s_22_6: call X_read(s_22_5, s_22_4)
        let s_22_6: Bits = X_read(state, tracer, s_22_5, s_22_4);
        // D s_22_7: cast reint s_22_6 -> u64
        let s_22_7: u64 = (s_22_6.value() as u64);
        // C s_22_8: const #0s : i
        let s_22_8: i128 = 0;
        // D s_22_9: cast zx s_22_3 -> bv
        let s_22_9: Bits = Bits::new(s_22_3 as u128, 128u16);
        // D s_22_10: cast zx s_22_7 -> bv
        let s_22_10: Bits = Bits::new(s_22_7 as u128, 64u16);
        // C s_22_11: const #63s : i
        let s_22_11: i128 = 63;
        // C s_22_12: const #1u : u64
        let s_22_12: u64 = 1;
        // C s_22_13: cast zx s_22_12 -> bv
        let s_22_13: Bits = Bits::new(s_22_12 as u128, 64u16);
        // C s_22_14: lsl s_22_13 s_22_11
        let s_22_14: Bits = s_22_13 << s_22_11;
        // C s_22_15: sub s_22_14 s_22_13
        let s_22_15: Bits = ((s_22_14) - (s_22_13));
        // D s_22_16: and s_22_10 s_22_15
        let s_22_16: Bits = ((s_22_10) & (s_22_15));
        // D s_22_17: lsl s_22_16 s_22_8
        let s_22_17: Bits = s_22_16 << s_22_8;
        // C s_22_18: lsl s_22_15 s_22_8
        let s_22_18: Bits = s_22_15 << s_22_8;
        // C s_22_19: cmpl s_22_18
        let s_22_19: Bits = !s_22_18;
        // D s_22_20: and s_22_9 s_22_19
        let s_22_20: Bits = ((s_22_9) & (s_22_19));
        // D s_22_21: or s_22_20 s_22_17
        let s_22_21: Bits = ((s_22_20) | (s_22_17));
        // D s_22_22: cast reint s_22_21 -> u128
        let s_22_22: u128 = (s_22_21.value() as u128);
        // D s_22_23: call Mk_TTBR1_EL1_Type(s_22_22)
        let s_22_23: ProductType782ac6922b48c20d = Mk_TTBR1_EL1_Type(
            state,
            tracer,
            s_22_22,
        );
        // D s_22_24: call TTBR1_EL1_write(s_22_23)
        let s_22_24: () = TTBR1_EL1_write(state, tracer, s_22_23);
        // N s_22_25: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #528u : u12
        let s_23_0: u16 = 528;
        // C s_23_1: cast zx s_23_0 -> bv
        let s_23_1: Bits = Bits::new(s_23_0 as u128, 12u16);
        // C s_23_2: cast zx s_23_1 -> i
        let s_23_2: i128 = (s_23_1.value() as i128);
        // C s_23_3: cast reint s_23_2 -> i64
        let s_23_3: i64 = (s_23_2 as i64);
        // C s_23_4: const #64s : i64
        let s_23_4: i64 = 64;
        // D s_23_5: read-var t:i
        let s_23_5: i128 = fn_state.t;
        // D s_23_6: call X_read(s_23_5, s_23_4)
        let s_23_6: Bits = X_read(state, tracer, s_23_5, s_23_4);
        // D s_23_7: cast reint s_23_6 -> u64
        let s_23_7: u64 = (s_23_6.value() as u64);
        // C s_23_8: cast zx s_23_3 -> i
        let s_23_8: i128 = (i128::try_from(s_23_3).unwrap());
        // D s_23_9: call NVMem_set(s_23_8, s_23_7)
        let s_23_9: () = NVMem_set(state, tracer, s_23_8, s_23_7);
        // N s_23_10: return
        return;
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
        // D s_24_2: call _get_HCR_EL2_Type_NV2(s_24_1)
        let s_24_2: bool = u_get_HCR_EL2_Type_NV2(state, tracer, s_24_1);
        // C s_24_3: const #102552u : u32
        let s_24_3: u32 = 102552;
        // D s_24_4: read-reg s_24_3:struct
        let s_24_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_24_3 as isize);
            tracer.read_register(s_24_3 as isize, value);
            value
        };
        // D s_24_5: call _get_HCR_EL2_Type_NV1(s_24_4)
        let s_24_5: bool = u_get_HCR_EL2_Type_NV1(state, tracer, s_24_4);
        // C s_24_6: const #102552u : u32
        let s_24_6: u32 = 102552;
        // D s_24_7: read-reg s_24_6:struct
        let s_24_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_24_6 as isize);
            tracer.read_register(s_24_6 as isize, value);
            value
        };
        // D s_24_8: call _get_HCR_EL2_Type_NV(s_24_7)
        let s_24_8: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_24_7);
        // D s_24_9: cast zx s_24_5 -> bv
        let s_24_9: Bits = Bits::new(s_24_5 as u128, 1u16);
        // D s_24_10: cast zx s_24_8 -> bv
        let s_24_10: Bits = Bits::new(s_24_8 as u128, 1u16);
        // D s_24_11: cast reint s_24_9 -> u128
        let s_24_11: u128 = (s_24_9.value() as u128);
        // D s_24_12: size-of s_24_9
        let s_24_12: u16 = s_24_9.length();
        // D s_24_13: cast reint s_24_10 -> u128
        let s_24_13: u128 = (s_24_10.value() as u128);
        // D s_24_14: size-of s_24_10
        let s_24_14: u16 = s_24_10.length();
        // D s_24_15: lsl s_24_11 s_24_14
        let s_24_15: u128 = s_24_11 << s_24_14;
        // D s_24_16: or s_24_15 s_24_13
        let s_24_16: u128 = ((s_24_15) | (s_24_13));
        // D s_24_17: add s_24_12 s_24_14
        let s_24_17: u16 = (s_24_12 + s_24_14);
        // D s_24_18: create-bits s_24_16 s_24_17
        let s_24_18: Bits = Bits::new(s_24_16, s_24_17);
        // D s_24_19: cast reint s_24_18 -> u8
        let s_24_19: u8 = (s_24_18.value() as u8);
        // D s_24_20: cast zx s_24_2 -> bv
        let s_24_20: Bits = Bits::new(s_24_2 as u128, 1u16);
        // D s_24_21: cast zx s_24_19 -> bv
        let s_24_21: Bits = Bits::new(s_24_19 as u128, 2u16);
        // D s_24_22: cast reint s_24_20 -> u128
        let s_24_22: u128 = (s_24_20.value() as u128);
        // D s_24_23: size-of s_24_20
        let s_24_23: u16 = s_24_20.length();
        // D s_24_24: cast reint s_24_21 -> u128
        let s_24_24: u128 = (s_24_21.value() as u128);
        // D s_24_25: size-of s_24_21
        let s_24_25: u16 = s_24_21.length();
        // D s_24_26: lsl s_24_22 s_24_25
        let s_24_26: u128 = s_24_22 << s_24_25;
        // D s_24_27: or s_24_26 s_24_24
        let s_24_27: u128 = ((s_24_26) | (s_24_24));
        // D s_24_28: add s_24_23 s_24_25
        let s_24_28: u16 = (s_24_23 + s_24_25);
        // D s_24_29: create-bits s_24_27 s_24_28
        let s_24_29: Bits = Bits::new(s_24_27, s_24_28);
        // D s_24_30: cast reint s_24_29 -> u8
        let s_24_30: u8 = (s_24_29.value() as u8);
        // D s_24_31: cast zx s_24_30 -> bv
        let s_24_31: Bits = Bits::new(s_24_30 as u128, 3u16);
        // C s_24_32: const #7u : u8
        let s_24_32: u8 = 7;
        // C s_24_33: cast zx s_24_32 -> bv
        let s_24_33: Bits = Bits::new(s_24_32 as u128, 3u16);
        // D s_24_34: cmp-eq s_24_31 s_24_33
        let s_24_34: bool = ((s_24_31) == (s_24_33));
        // D s_24_35: write-var gs#96957 <= s_24_34
        fn_state.gs_96957 = s_24_34;
        // N s_24_36: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #24u : u8
        let s_25_0: u8 = 24;
        // C s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 8u16);
        // C s_25_2: cast zx s_25_1 -> i
        let s_25_2: i128 = (s_25_1.value() as i128);
        // C s_25_3: cast reint s_25_2 -> i64
        let s_25_3: i64 = (s_25_2 as i64);
        // C s_25_4: cast zx s_25_3 -> i
        let s_25_4: i128 = (i128::try_from(s_25_3).unwrap());
        // C s_25_5: const #432u : u32
        let s_25_5: u32 = 432;
        // D s_25_6: read-reg s_25_5:u8
        let s_25_6: u8 = {
            let value = state.read_register::<u8>(s_25_5 as isize);
            tracer.read_register(s_25_5 as isize, value);
            value
        };
        // D s_25_7: call AArch64_SystemAccessTrap(s_25_6, s_25_4)
        let s_25_7: () = AArch64_SystemAccessTrap(state, tracer, s_25_6, s_25_4);
        // N s_25_8: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var __HFGWTR_EL2_TTBR1_EL1:u8
        let s_26_0: bool = fn_state.u__HFGWTR_EL2_TTBR1_EL1;
        // D s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 1u16);
        // C s_26_2: const #1u : u8
        let s_26_2: bool = true;
        // C s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 1u16);
        // D s_26_4: cmp-eq s_26_1 s_26_3
        let s_26_4: bool = ((s_26_1) == (s_26_3));
        // D s_26_5: write-var gs#96956 <= s_26_4
        fn_state.gs_96956 = s_26_4;
        // N s_26_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #424u : u32
        let s_27_0: u32 = 424;
        // D s_27_1: read-reg s_27_0:u8
        let s_27_1: u8 = {
            let value = state.read_register::<u8>(s_27_0 as isize);
            tracer.read_register(s_27_0 as isize, value);
            value
        };
        // C s_27_2: const #2u : u8
        let s_27_2: u8 = 2;
        // D s_27_3: cmp-lt s_27_1 s_27_2
        let s_27_3: bool = ((s_27_1) < (s_27_2));
        // D s_27_4: not s_27_3
        let s_27_4: bool = !s_27_3;
        // N s_27_5: branch s_27_4 b30 b28
        if s_27_4 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var __SCR_EL3_FGTEn:u8
        let s_28_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_28_1: cast zx s_28_0 -> bv
        let s_28_1: Bits = Bits::new(s_28_0 as u128, 1u16);
        // C s_28_2: const #1u : u8
        let s_28_2: bool = true;
        // C s_28_3: cast zx s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 1u16);
        // D s_28_4: cmp-eq s_28_1 s_28_3
        let s_28_4: bool = ((s_28_1) == (s_28_3));
        // D s_28_5: write-var gs#96954 <= s_28_4
        fn_state.gs_96954 = s_28_4;
        // N s_28_6: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#96954:u8
        let s_29_0: bool = fn_state.gs_96954;
        // D s_29_1: write-var gs#96955 <= s_29_0
        fn_state.gs_96955 = s_29_0;
        // N s_29_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #1u : u8
        let s_30_0: bool = true;
        // D s_30_1: write-var gs#96954 <= s_30_0
        fn_state.gs_96954 = s_30_0;
        // N s_30_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #146u : u32
        let s_31_0: u32 = 146;
        // S s_31_1: call IsFeatureImplemented(s_31_0)
        let s_31_1: bool = IsFeatureImplemented(state, tracer, s_31_0);
        // D s_31_2: write-var gs#96953 <= s_31_1
        fn_state.gs_96953 = s_31_1;
        // N s_31_3: jump b14
        return block_14(state, tracer, fn_state);
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
        // D s_33_0: read-var __HCR_EL2_TVM:u8
        let s_33_0: bool = fn_state.u__HCR_EL2_TVM;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 1u16);
        // C s_33_2: const #1u : u8
        let s_33_2: bool = true;
        // C s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 1u16);
        // D s_33_4: cmp-eq s_33_1 s_33_3
        let s_33_4: bool = ((s_33_1) == (s_33_3));
        // D s_33_5: write-var gs#96952 <= s_33_4
        fn_state.gs_96952 = s_33_4;
        // N s_33_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_34_0: panic
        panic!("{:?}", ());
        // N s_34_1: return
        return;
    }
}
