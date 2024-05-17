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
use u_get_HCR_EL2_Type_NV1::*;
use u_get_HCR_EL2_Type_E2H::*;
use u_get_HCR_EL2_Type_NV::*;
use ELUsingAArch32::*;
use TTBR1_EL1_write::*;
use X_read::*;
use TTBR1_EL1_read::*;
use EL2Enabled::*;
use AArch64_SystemAccessTrap::*;
use Mk_TTBR1_EL1_Type::*;
use NVMem_set::*;
use common::*;
pub fn TTBR1_EL1_SysRegWrite_4d156e352c93f29e<T: Tracer>(
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
        gs_96931: bool,
        gs_96939: bool,
        gs_96932: bool,
        ga_147674: ProductType782ac6922b48c20d,
        ga_147662: ProductType782ac6922b48c20d,
        u__PSTATE_EL: u8,
        gs_96940: bool,
        u__HCR_EL2_NV: bool,
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
        // D s_0_11: read-var __PSTATE_EL:u8
        let s_0_11: u8 = fn_state.u__PSTATE_EL;
        // D s_0_12: cast zx s_0_11 -> bv
        let s_0_12: Bits = Bits::new(s_0_11 as u128, 2u16);
        // C s_0_13: const #448u : u32
        let s_0_13: u32 = 448;
        // D s_0_14: read-reg s_0_13:u8
        let s_0_14: u8 = {
            let value = state.read_register::<u8>(s_0_13 as isize);
            tracer.read_register(s_0_13 as isize, value);
            value
        };
        // D s_0_15: cast zx s_0_14 -> bv
        let s_0_15: Bits = Bits::new(s_0_14 as u128, 2u16);
        // D s_0_16: cmp-eq s_0_12 s_0_15
        let s_0_16: bool = ((s_0_12) == (s_0_15));
        // N s_0_17: branch s_0_16 b28 b1
        if s_0_16 {
            return block_28(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b17 b2
        if s_1_5 {
            return block_17(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#96931 <= s_6_0
        fn_state.gs_96931 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#96931:u8
        let s_7_0: bool = fn_state.gs_96931;
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
        // D s_8_1: write-var gs#96932 <= s_8_0
        fn_state.gs_96932 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#96932:u8
        let s_9_0: bool = fn_state.gs_96932;
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
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call TTBR1_EL1_read(s_11_0)
        let s_11_1: ProductType782ac6922b48c20d = TTBR1_EL1_read(state, tracer, s_11_0);
        // D s_11_2: write-var ga#147674 <= s_11_1
        fn_state.ga_147674 = s_11_1;
        // D s_11_3: read-var ga#147674.0:struct
        let s_11_3: u128 = fn_state.ga_147674._0;
        // C s_11_4: const #64s : i64
        let s_11_4: i64 = 64;
        // D s_11_5: read-var t:i
        let s_11_5: i128 = fn_state.t;
        // D s_11_6: call X_read(s_11_5, s_11_4)
        let s_11_6: Bits = X_read(state, tracer, s_11_5, s_11_4);
        // D s_11_7: cast reint s_11_6 -> u64
        let s_11_7: u64 = (s_11_6.value() as u64);
        // C s_11_8: const #0s : i
        let s_11_8: i128 = 0;
        // D s_11_9: cast zx s_11_3 -> bv
        let s_11_9: Bits = Bits::new(s_11_3 as u128, 128u16);
        // D s_11_10: cast zx s_11_7 -> bv
        let s_11_10: Bits = Bits::new(s_11_7 as u128, 64u16);
        // C s_11_11: const #63s : i
        let s_11_11: i128 = 63;
        // C s_11_12: const #1u : u64
        let s_11_12: u64 = 1;
        // C s_11_13: cast zx s_11_12 -> bv
        let s_11_13: Bits = Bits::new(s_11_12 as u128, 64u16);
        // C s_11_14: lsl s_11_13 s_11_11
        let s_11_14: Bits = s_11_13 << s_11_11;
        // C s_11_15: sub s_11_14 s_11_13
        let s_11_15: Bits = ((s_11_14) - (s_11_13));
        // D s_11_16: and s_11_10 s_11_15
        let s_11_16: Bits = ((s_11_10) & (s_11_15));
        // D s_11_17: lsl s_11_16 s_11_8
        let s_11_17: Bits = s_11_16 << s_11_8;
        // C s_11_18: lsl s_11_15 s_11_8
        let s_11_18: Bits = s_11_15 << s_11_8;
        // C s_11_19: cmpl s_11_18
        let s_11_19: Bits = !s_11_18;
        // D s_11_20: and s_11_9 s_11_19
        let s_11_20: Bits = ((s_11_9) & (s_11_19));
        // D s_11_21: or s_11_20 s_11_17
        let s_11_21: Bits = ((s_11_20) | (s_11_17));
        // D s_11_22: cast reint s_11_21 -> u128
        let s_11_22: u128 = (s_11_21.value() as u128);
        // D s_11_23: call Mk_TTBR1_EL1_Type(s_11_22)
        let s_11_23: ProductType782ac6922b48c20d = Mk_TTBR1_EL1_Type(
            state,
            tracer,
            s_11_22,
        );
        // D s_11_24: call TTBR1_EL1_write(s_11_23)
        let s_11_24: () = TTBR1_EL1_write(state, tracer, s_11_23);
        // N s_11_25: return
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
        // D s_12_5: write-var gs#96932 <= s_12_4
        fn_state.gs_96932 = s_12_4;
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
        // D s_13_4: write-var gs#96931 <= s_13_3
        fn_state.gs_96931 = s_13_3;
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
        // S s_16_1: call TTBR1_EL1_read(s_16_0)
        let s_16_1: ProductType782ac6922b48c20d = TTBR1_EL1_read(state, tracer, s_16_0);
        // D s_16_2: write-var ga#147662 <= s_16_1
        fn_state.ga_147662 = s_16_1;
        // D s_16_3: read-var ga#147662.0:struct
        let s_16_3: u128 = fn_state.ga_147662._0;
        // C s_16_4: const #64s : i64
        let s_16_4: i64 = 64;
        // D s_16_5: read-var t:i
        let s_16_5: i128 = fn_state.t;
        // D s_16_6: call X_read(s_16_5, s_16_4)
        let s_16_6: Bits = X_read(state, tracer, s_16_5, s_16_4);
        // D s_16_7: cast reint s_16_6 -> u64
        let s_16_7: u64 = (s_16_6.value() as u64);
        // C s_16_8: const #0s : i
        let s_16_8: i128 = 0;
        // D s_16_9: cast zx s_16_3 -> bv
        let s_16_9: Bits = Bits::new(s_16_3 as u128, 128u16);
        // D s_16_10: cast zx s_16_7 -> bv
        let s_16_10: Bits = Bits::new(s_16_7 as u128, 64u16);
        // C s_16_11: const #63s : i
        let s_16_11: i128 = 63;
        // C s_16_12: const #1u : u64
        let s_16_12: u64 = 1;
        // C s_16_13: cast zx s_16_12 -> bv
        let s_16_13: Bits = Bits::new(s_16_12 as u128, 64u16);
        // C s_16_14: lsl s_16_13 s_16_11
        let s_16_14: Bits = s_16_13 << s_16_11;
        // C s_16_15: sub s_16_14 s_16_13
        let s_16_15: Bits = ((s_16_14) - (s_16_13));
        // D s_16_16: and s_16_10 s_16_15
        let s_16_16: Bits = ((s_16_10) & (s_16_15));
        // D s_16_17: lsl s_16_16 s_16_8
        let s_16_17: Bits = s_16_16 << s_16_8;
        // C s_16_18: lsl s_16_15 s_16_8
        let s_16_18: Bits = s_16_15 << s_16_8;
        // C s_16_19: cmpl s_16_18
        let s_16_19: Bits = !s_16_18;
        // D s_16_20: and s_16_9 s_16_19
        let s_16_20: Bits = ((s_16_9) & (s_16_19));
        // D s_16_21: or s_16_20 s_16_17
        let s_16_21: Bits = ((s_16_20) | (s_16_17));
        // D s_16_22: cast reint s_16_21 -> u128
        let s_16_22: u128 = (s_16_21.value() as u128);
        // D s_16_23: call Mk_TTBR1_EL1_Type(s_16_22)
        let s_16_23: ProductType782ac6922b48c20d = Mk_TTBR1_EL1_Type(
            state,
            tracer,
            s_16_22,
        );
        // D s_16_24: call TTBR1_EL1_write(s_16_23)
        let s_16_24: () = TTBR1_EL1_write(state, tracer, s_16_23);
        // N s_16_25: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call EL2Enabled(s_17_0)
        let s_17_1: bool = EL2Enabled(state, tracer, s_17_0);
        // N s_17_2: branch s_17_1 b27 b18
        if s_17_1 {
            return block_27(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#96939 <= s_18_0
        fn_state.gs_96939 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#96939:u8
        let s_19_0: bool = fn_state.gs_96939;
        // N s_19_1: branch s_19_0 b26 b20
        if s_19_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call EL2Enabled(s_20_0)
        let s_20_1: bool = EL2Enabled(state, tracer, s_20_0);
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
        // D s_21_1: write-var gs#96940 <= s_21_0
        fn_state.gs_96940 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#96940:u8
        let s_22_0: bool = fn_state.gs_96940;
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
        // N s_23_0: panic
        panic!("{:?}", ());
        // N s_23_1: return
        return;
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
        // C s_24_5: const #432u : u32
        let s_24_5: u32 = 432;
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
        // D s_25_0: read-var __HCR_EL2_NV:u8
        let s_25_0: bool = fn_state.u__HCR_EL2_NV;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 1u16);
        // C s_25_2: const #1u : u8
        let s_25_2: bool = true;
        // C s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // D s_25_4: cmp-eq s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) == (s_25_3));
        // D s_25_5: write-var gs#96940 <= s_25_4
        fn_state.gs_96940 = s_25_4;
        // N s_25_6: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #528u : u12
        let s_26_0: u16 = 528;
        // C s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 12u16);
        // C s_26_2: cast zx s_26_1 -> i
        let s_26_2: i128 = (s_26_1.value() as i128);
        // C s_26_3: cast reint s_26_2 -> i64
        let s_26_3: i64 = (s_26_2 as i64);
        // C s_26_4: const #64s : i64
        let s_26_4: i64 = 64;
        // D s_26_5: read-var t:i
        let s_26_5: i128 = fn_state.t;
        // D s_26_6: call X_read(s_26_5, s_26_4)
        let s_26_6: Bits = X_read(state, tracer, s_26_5, s_26_4);
        // D s_26_7: cast reint s_26_6 -> u64
        let s_26_7: u64 = (s_26_6.value() as u64);
        // C s_26_8: cast zx s_26_3 -> i
        let s_26_8: i128 = (i128::try_from(s_26_3).unwrap());
        // D s_26_9: call NVMem_set(s_26_8, s_26_7)
        let s_26_9: () = NVMem_set(state, tracer, s_26_8, s_26_7);
        // N s_26_10: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #102552u : u32
        let s_27_0: u32 = 102552;
        // D s_27_1: read-reg s_27_0:struct
        let s_27_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_27_0 as isize);
            tracer.read_register(s_27_0 as isize, value);
            value
        };
        // D s_27_2: call _get_HCR_EL2_Type_NV2(s_27_1)
        let s_27_2: bool = u_get_HCR_EL2_Type_NV2(state, tracer, s_27_1);
        // C s_27_3: const #102552u : u32
        let s_27_3: u32 = 102552;
        // D s_27_4: read-reg s_27_3:struct
        let s_27_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_27_3 as isize);
            tracer.read_register(s_27_3 as isize, value);
            value
        };
        // D s_27_5: call _get_HCR_EL2_Type_NV1(s_27_4)
        let s_27_5: bool = u_get_HCR_EL2_Type_NV1(state, tracer, s_27_4);
        // C s_27_6: const #102552u : u32
        let s_27_6: u32 = 102552;
        // D s_27_7: read-reg s_27_6:struct
        let s_27_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_27_6 as isize);
            tracer.read_register(s_27_6 as isize, value);
            value
        };
        // D s_27_8: call _get_HCR_EL2_Type_NV(s_27_7)
        let s_27_8: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_27_7);
        // D s_27_9: cast zx s_27_5 -> bv
        let s_27_9: Bits = Bits::new(s_27_5 as u128, 1u16);
        // D s_27_10: cast zx s_27_8 -> bv
        let s_27_10: Bits = Bits::new(s_27_8 as u128, 1u16);
        // D s_27_11: cast reint s_27_9 -> u128
        let s_27_11: u128 = (s_27_9.value() as u128);
        // D s_27_12: size-of s_27_9
        let s_27_12: u16 = s_27_9.length();
        // D s_27_13: cast reint s_27_10 -> u128
        let s_27_13: u128 = (s_27_10.value() as u128);
        // D s_27_14: size-of s_27_10
        let s_27_14: u16 = s_27_10.length();
        // D s_27_15: lsl s_27_11 s_27_14
        let s_27_15: u128 = s_27_11 << s_27_14;
        // D s_27_16: or s_27_15 s_27_13
        let s_27_16: u128 = ((s_27_15) | (s_27_13));
        // D s_27_17: add s_27_12 s_27_14
        let s_27_17: u16 = (s_27_12 + s_27_14);
        // D s_27_18: create-bits s_27_16 s_27_17
        let s_27_18: Bits = Bits::new(s_27_16, s_27_17);
        // D s_27_19: cast reint s_27_18 -> u8
        let s_27_19: u8 = (s_27_18.value() as u8);
        // D s_27_20: cast zx s_27_2 -> bv
        let s_27_20: Bits = Bits::new(s_27_2 as u128, 1u16);
        // D s_27_21: cast zx s_27_19 -> bv
        let s_27_21: Bits = Bits::new(s_27_19 as u128, 2u16);
        // D s_27_22: cast reint s_27_20 -> u128
        let s_27_22: u128 = (s_27_20.value() as u128);
        // D s_27_23: size-of s_27_20
        let s_27_23: u16 = s_27_20.length();
        // D s_27_24: cast reint s_27_21 -> u128
        let s_27_24: u128 = (s_27_21.value() as u128);
        // D s_27_25: size-of s_27_21
        let s_27_25: u16 = s_27_21.length();
        // D s_27_26: lsl s_27_22 s_27_25
        let s_27_26: u128 = s_27_22 << s_27_25;
        // D s_27_27: or s_27_26 s_27_24
        let s_27_27: u128 = ((s_27_26) | (s_27_24));
        // D s_27_28: add s_27_23 s_27_25
        let s_27_28: u16 = (s_27_23 + s_27_25);
        // D s_27_29: create-bits s_27_27 s_27_28
        let s_27_29: Bits = Bits::new(s_27_27, s_27_28);
        // D s_27_30: cast reint s_27_29 -> u8
        let s_27_30: u8 = (s_27_29.value() as u8);
        // D s_27_31: cast zx s_27_30 -> bv
        let s_27_31: Bits = Bits::new(s_27_30 as u128, 3u16);
        // C s_27_32: const #5u : u8
        let s_27_32: u8 = 5;
        // C s_27_33: cast zx s_27_32 -> bv
        let s_27_33: Bits = Bits::new(s_27_32 as u128, 3u16);
        // D s_27_34: cmp-eq s_27_31 s_27_33
        let s_27_34: bool = ((s_27_31) == (s_27_33));
        // D s_27_35: write-var gs#96939 <= s_27_34
        fn_state.gs_96939 = s_27_34;
        // N s_27_36: jump b19
        return block_19(state, tracer, fn_state);
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
}
