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
use EXLOCKException::*;
use u_get_HCR_EL2_Type_NV::*;
use Halted::*;
use GetCurrentEXLOCKEN::*;
use Mk_SPSR_EL1_Type::*;
use Mk_SPSR_EL2_Type::*;
use X_read::*;
use IsFeatureImplemented::*;
use EL2Enabled::*;
use AArch64_SystemAccessTrap::*;
use u_get_HCR_EL2_Type_NV2::*;
use common::*;
pub fn SPSR_EL2_SysRegWrite_5e3ca78864ff5189<T: Tracer>(
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
        gs_96498: bool,
        gs_96500: bool,
        gs_96505: bool,
        gs_96508: bool,
        gs_96504: bool,
        gs_96499: bool,
        gs_96502: bool,
        u__PSTATE_EL: u8,
        gs_96503: bool,
        u__HCR_EL2_NV: bool,
        gs_96506: bool,
        gs_96507: bool,
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
        // D s_0_7: read-var __PSTATE_EL:u8
        let s_0_7: u8 = fn_state.u__PSTATE_EL;
        // D s_0_8: cast zx s_0_7 -> bv
        let s_0_8: Bits = Bits::new(s_0_7 as u128, 2u16);
        // C s_0_9: const #448u : u32
        let s_0_9: u32 = 448;
        // D s_0_10: read-reg s_0_9:u8
        let s_0_10: u8 = {
            let value = state.read_register::<u8>(s_0_9 as isize);
            tracer.read_register(s_0_9 as isize, value);
            value
        };
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 2u16);
        // D s_0_12: cmp-eq s_0_8 s_0_11
        let s_0_12: bool = ((s_0_8) == (s_0_11));
        // N s_0_13: branch s_0_12 b46 b1
        if s_0_12 {
            return block_46(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b18 b2
        if s_1_5 {
            return block_18(state, tracer, fn_state);
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
        // D s_5_4: call Mk_SPSR_EL2_Type(s_5_3)
        let s_5_4: ProductType5c790c8ef59cc8b2 = Mk_SPSR_EL2_Type(state, tracer, s_5_3);
        // C s_5_5: const #15736u : u32
        let s_5_5: u32 = 15736;
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
        // C s_6_0: const #245u : u32
        let s_6_0: u32 = 245;
        // S s_6_1: call IsFeatureImplemented(s_6_0)
        let s_6_1: bool = IsFeatureImplemented(state, tracer, s_6_0);
        // N s_6_2: branch s_6_1 b17 b7
        if s_6_1 {
            return block_17(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#96498 <= s_7_0
        fn_state.gs_96498 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#96498:u8
        let s_8_0: bool = fn_state.gs_96498;
        // N s_8_1: branch s_8_0 b16 b9
        if s_8_0 {
            return block_16(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#96499 <= s_9_0
        fn_state.gs_96499 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#96499:u8
        let s_10_0: bool = fn_state.gs_96499;
        // N s_10_1: branch s_10_0 b15 b11
        if s_10_0 {
            return block_15(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#96500 <= s_11_0
        fn_state.gs_96500 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#96500:u8
        let s_12_0: bool = fn_state.gs_96500;
        // N s_12_1: branch s_12_0 b14 b13
        if s_12_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #64s : i64
        let s_13_0: i64 = 64;
        // D s_13_1: read-var t:i
        let s_13_1: i128 = fn_state.t;
        // D s_13_2: call X_read(s_13_1, s_13_0)
        let s_13_2: Bits = X_read(state, tracer, s_13_1, s_13_0);
        // D s_13_3: cast reint s_13_2 -> u64
        let s_13_3: u64 = (s_13_2.value() as u64);
        // D s_13_4: call Mk_SPSR_EL2_Type(s_13_3)
        let s_13_4: ProductType5c790c8ef59cc8b2 = Mk_SPSR_EL2_Type(
            state,
            tracer,
            s_13_3,
        );
        // C s_13_5: const #15736u : u32
        let s_13_5: u32 = 15736;
        // N s_13_6: write-reg s_13_5 <= s_13_4
        let s_13_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_13_5 as isize, s_13_4);
            tracer.write_register(s_13_5 as isize, s_13_4);
        };
        // N s_13_7: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call EXLOCKException(s_14_0)
        let s_14_1: () = EXLOCKException(state, tracer, s_14_0);
        // N s_14_2: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #16976u : u32
        let s_15_0: u32 = 16976;
        // D s_15_1: read-reg s_15_0:u8
        let s_15_1: bool = {
            let value = state.read_register::<bool>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // D s_15_2: cast zx s_15_1 -> bv
        let s_15_2: Bits = Bits::new(s_15_1 as u128, 1u16);
        // C s_15_3: const #1u : u8
        let s_15_3: bool = true;
        // C s_15_4: cast zx s_15_3 -> bv
        let s_15_4: Bits = Bits::new(s_15_3 as u128, 1u16);
        // D s_15_5: cmp-eq s_15_2 s_15_4
        let s_15_5: bool = ((s_15_2) == (s_15_4));
        // D s_15_6: write-var gs#96500 <= s_15_5
        fn_state.gs_96500 = s_15_5;
        // N s_15_7: jump b12
        return block_12(state, tracer, fn_state);
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
        // S s_16_2: not s_16_1
        let s_16_2: bool = !s_16_1;
        // D s_16_3: write-var gs#96499 <= s_16_2
        fn_state.gs_96499 = s_16_2;
        // N s_16_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call GetCurrentEXLOCKEN(s_17_0)
        let s_17_1: bool = GetCurrentEXLOCKEN(state, tracer, s_17_0);
        // D s_17_2: write-var gs#96498 <= s_17_1
        fn_state.gs_96498 = s_17_1;
        // N s_17_3: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #245u : u32
        let s_18_0: u32 = 245;
        // S s_18_1: call IsFeatureImplemented(s_18_0)
        let s_18_1: bool = IsFeatureImplemented(state, tracer, s_18_0);
        // N s_18_2: branch s_18_1 b45 b19
        if s_18_1 {
            return block_45(state, tracer, fn_state);
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
        // D s_19_1: write-var gs#96502 <= s_19_0
        fn_state.gs_96502 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#96502:u8
        let s_20_0: bool = fn_state.gs_96502;
        // N s_20_1: branch s_20_0 b44 b21
        if s_20_0 {
            return block_44(state, tracer, fn_state);
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
        // D s_21_1: write-var gs#96503 <= s_21_0
        fn_state.gs_96503 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#96503:u8
        let s_22_0: bool = fn_state.gs_96503;
        // N s_22_1: branch s_22_0 b43 b23
        if s_22_0 {
            return block_43(state, tracer, fn_state);
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
        // D s_23_1: write-var gs#96504 <= s_23_0
        fn_state.gs_96504 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#96504:u8
        let s_24_0: bool = fn_state.gs_96504;
        // N s_24_1: branch s_24_0 b42 b25
        if s_24_0 {
            return block_42(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#96505 <= s_25_0
        fn_state.gs_96505 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#96505:u8
        let s_26_0: bool = fn_state.gs_96505;
        // N s_26_1: branch s_26_0 b41 b27
        if s_26_0 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // D s_27_1: write-var gs#96506 <= s_27_0
        fn_state.gs_96506 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#96506:u8
        let s_28_0: bool = fn_state.gs_96506;
        // N s_28_1: branch s_28_0 b40 b29
        if s_28_0 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #() : ()
        let s_29_0: () = ();
        // S s_29_1: call EL2Enabled(s_29_0)
        let s_29_1: bool = EL2Enabled(state, tracer, s_29_0);
        // N s_29_2: branch s_29_1 b39 b30
        if s_29_1 {
            return block_39(state, tracer, fn_state);
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
        // D s_30_1: write-var gs#96507 <= s_30_0
        fn_state.gs_96507 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#96507:u8
        let s_31_0: bool = fn_state.gs_96507;
        // N s_31_1: branch s_31_0 b38 b32
        if s_31_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #() : ()
        let s_32_0: () = ();
        // S s_32_1: call EL2Enabled(s_32_0)
        let s_32_1: bool = EL2Enabled(state, tracer, s_32_0);
        // N s_32_2: branch s_32_1 b37 b33
        if s_32_1 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #0u : u8
        let s_33_0: bool = false;
        // D s_33_1: write-var gs#96508 <= s_33_0
        fn_state.gs_96508 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#96508:u8
        let s_34_0: bool = fn_state.gs_96508;
        // N s_34_1: branch s_34_0 b36 b35
        if s_34_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_35_0: panic
        panic!("{:?}", ());
        // N s_35_1: return
        return;
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #24u : u8
        let s_36_0: u8 = 24;
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
        // D s_37_0: read-var __HCR_EL2_NV:u8
        let s_37_0: bool = fn_state.u__HCR_EL2_NV;
        // D s_37_1: cast zx s_37_0 -> bv
        let s_37_1: Bits = Bits::new(s_37_0 as u128, 1u16);
        // C s_37_2: const #1u : u8
        let s_37_2: bool = true;
        // C s_37_3: cast zx s_37_2 -> bv
        let s_37_3: Bits = Bits::new(s_37_2 as u128, 1u16);
        // D s_37_4: cmp-eq s_37_1 s_37_3
        let s_37_4: bool = ((s_37_1) == (s_37_3));
        // D s_37_5: write-var gs#96508 <= s_37_4
        fn_state.gs_96508 = s_37_4;
        // N s_37_6: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #64s : i64
        let s_38_0: i64 = 64;
        // D s_38_1: read-var t:i
        let s_38_1: i128 = fn_state.t;
        // D s_38_2: call X_read(s_38_1, s_38_0)
        let s_38_2: Bits = X_read(state, tracer, s_38_1, s_38_0);
        // D s_38_3: cast reint s_38_2 -> u64
        let s_38_3: u64 = (s_38_2.value() as u64);
        // D s_38_4: call Mk_SPSR_EL1_Type(s_38_3)
        let s_38_4: ProductType5c790c8ef59cc8b2 = Mk_SPSR_EL1_Type(
            state,
            tracer,
            s_38_3,
        );
        // C s_38_5: const #90648u : u32
        let s_38_5: u32 = 90648;
        // N s_38_6: write-reg s_38_5 <= s_38_4
        let s_38_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_38_5 as isize, s_38_4);
            tracer.write_register(s_38_5 as isize, s_38_4);
        };
        // N s_38_7: return
        return;
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #102552u : u32
        let s_39_0: u32 = 102552;
        // D s_39_1: read-reg s_39_0:struct
        let s_39_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_39_0 as isize);
            tracer.read_register(s_39_0 as isize, value);
            value
        };
        // D s_39_2: call _get_HCR_EL2_Type_NV2(s_39_1)
        let s_39_2: bool = u_get_HCR_EL2_Type_NV2(state, tracer, s_39_1);
        // C s_39_3: const #102552u : u32
        let s_39_3: u32 = 102552;
        // D s_39_4: read-reg s_39_3:struct
        let s_39_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_39_3 as isize);
            tracer.read_register(s_39_3 as isize, value);
            value
        };
        // D s_39_5: call _get_HCR_EL2_Type_NV(s_39_4)
        let s_39_5: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_39_4);
        // D s_39_6: cast zx s_39_2 -> bv
        let s_39_6: Bits = Bits::new(s_39_2 as u128, 1u16);
        // D s_39_7: cast zx s_39_5 -> bv
        let s_39_7: Bits = Bits::new(s_39_5 as u128, 1u16);
        // D s_39_8: cast reint s_39_6 -> u128
        let s_39_8: u128 = (s_39_6.value() as u128);
        // D s_39_9: size-of s_39_6
        let s_39_9: u16 = s_39_6.length();
        // D s_39_10: cast reint s_39_7 -> u128
        let s_39_10: u128 = (s_39_7.value() as u128);
        // D s_39_11: size-of s_39_7
        let s_39_11: u16 = s_39_7.length();
        // D s_39_12: lsl s_39_8 s_39_11
        let s_39_12: u128 = s_39_8 << s_39_11;
        // D s_39_13: or s_39_12 s_39_10
        let s_39_13: u128 = ((s_39_12) | (s_39_10));
        // D s_39_14: add s_39_9 s_39_11
        let s_39_14: u16 = (s_39_9 + s_39_11);
        // D s_39_15: create-bits s_39_13 s_39_14
        let s_39_15: Bits = Bits::new(s_39_13, s_39_14);
        // D s_39_16: cast reint s_39_15 -> u8
        let s_39_16: u8 = (s_39_15.value() as u8);
        // D s_39_17: cast zx s_39_16 -> bv
        let s_39_17: Bits = Bits::new(s_39_16 as u128, 2u16);
        // C s_39_18: const #3u : u8
        let s_39_18: u8 = 3;
        // C s_39_19: cast zx s_39_18 -> bv
        let s_39_19: Bits = Bits::new(s_39_18 as u128, 2u16);
        // D s_39_20: cmp-eq s_39_17 s_39_19
        let s_39_20: bool = ((s_39_17) == (s_39_19));
        // D s_39_21: write-var gs#96507 <= s_39_20
        fn_state.gs_96507 = s_39_20;
        // N s_39_22: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #() : ()
        let s_40_0: () = ();
        // S s_40_1: call EXLOCKException(s_40_0)
        let s_40_1: () = EXLOCKException(state, tracer, s_40_0);
        // N s_40_2: return
        return;
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var __HCR_EL2_NV:u8
        let s_41_0: bool = fn_state.u__HCR_EL2_NV;
        // D s_41_1: cast zx s_41_0 -> bv
        let s_41_1: Bits = Bits::new(s_41_0 as u128, 1u16);
        // C s_41_2: const #1u : u8
        let s_41_2: bool = true;
        // C s_41_3: cast zx s_41_2 -> bv
        let s_41_3: Bits = Bits::new(s_41_2 as u128, 1u16);
        // D s_41_4: cmp-eq s_41_1 s_41_3
        let s_41_4: bool = ((s_41_1) == (s_41_3));
        // D s_41_5: write-var gs#96506 <= s_41_4
        fn_state.gs_96506 = s_41_4;
        // N s_41_6: jump b28
        return block_28(state, tracer, fn_state);
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
        // D s_42_2: write-var gs#96505 <= s_42_1
        fn_state.gs_96505 = s_42_1;
        // N s_42_3: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #16976u : u32
        let s_43_0: u32 = 16976;
        // D s_43_1: read-reg s_43_0:u8
        let s_43_1: bool = {
            let value = state.read_register::<bool>(s_43_0 as isize);
            tracer.read_register(s_43_0 as isize, value);
            value
        };
        // D s_43_2: cast zx s_43_1 -> bv
        let s_43_2: Bits = Bits::new(s_43_1 as u128, 1u16);
        // C s_43_3: const #1u : u8
        let s_43_3: bool = true;
        // C s_43_4: cast zx s_43_3 -> bv
        let s_43_4: Bits = Bits::new(s_43_3 as u128, 1u16);
        // D s_43_5: cmp-eq s_43_2 s_43_4
        let s_43_5: bool = ((s_43_2) == (s_43_4));
        // D s_43_6: write-var gs#96504 <= s_43_5
        fn_state.gs_96504 = s_43_5;
        // N s_43_7: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #() : ()
        let s_44_0: () = ();
        // S s_44_1: call Halted(s_44_0)
        let s_44_1: bool = Halted(state, tracer, s_44_0);
        // S s_44_2: not s_44_1
        let s_44_2: bool = !s_44_1;
        // D s_44_3: write-var gs#96503 <= s_44_2
        fn_state.gs_96503 = s_44_2;
        // N s_44_4: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #() : ()
        let s_45_0: () = ();
        // S s_45_1: call GetCurrentEXLOCKEN(s_45_0)
        let s_45_1: bool = GetCurrentEXLOCKEN(state, tracer, s_45_0);
        // D s_45_2: write-var gs#96502 <= s_45_1
        fn_state.gs_96502 = s_45_1;
        // N s_45_3: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_46_0: panic
        panic!("{:?}", ());
        // N s_46_1: return
        return;
    }
}
