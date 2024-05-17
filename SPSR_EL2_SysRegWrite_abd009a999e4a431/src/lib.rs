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
use Mk_SPSR_EL1_Type::*;
use GetCurrentEXLOCKEN::*;
use IsFeatureImplemented::*;
use AArch64_SystemAccessTrap::*;
use X_read::*;
use NVMem_set::*;
use u_get_HCR_EL2_Type_NV1::*;
use EXLOCKException::*;
use u_get_HCR_EL2_Type_NV::*;
use Mk_SPSR_EL2_Type::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_NV2::*;
use common::*;
pub fn SPSR_EL2_SysRegWrite_abd009a999e4a431<T: Tracer>(
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
        gs_96522: bool,
        u__HCR_EL2_E2H: bool,
        gs_96517: bool,
        gs_96518: bool,
        gs_96520: bool,
        gs_96512: bool,
        gs_96524: bool,
        gs_96511: bool,
        gs_96519: bool,
        gs_96513: bool,
        gs_96521: bool,
        u__PSTATE_EL: u8,
        gs_96514: bool,
        gs_96523: bool,
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
        // D s_0_5: call _get_HCR_EL2_Type_E2H(s_0_4)
        let s_0_5: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_0_4);
        // D s_0_6: write-var __HCR_EL2_E2H <= s_0_5
        fn_state.u__HCR_EL2_E2H = s_0_5;
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
        // N s_0_13: branch s_0_12 b54 b1
        if s_0_12 {
            return block_54(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b23 b2
        if s_1_5 {
            return block_23(state, tracer, fn_state);
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
        // D s_5_4: call Mk_SPSR_EL1_Type(s_5_3)
        let s_5_4: ProductType5c790c8ef59cc8b2 = Mk_SPSR_EL1_Type(state, tracer, s_5_3);
        // C s_5_5: const #90648u : u32
        let s_5_5: u32 = 90648;
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
        // N s_6_2: branch s_6_1 b22 b7
        if s_6_1 {
            return block_22(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#96511 <= s_7_0
        fn_state.gs_96511 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#96511:u8
        let s_8_0: bool = fn_state.gs_96511;
        // N s_8_1: branch s_8_0 b21 b9
        if s_8_0 {
            return block_21(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#96512 <= s_9_0
        fn_state.gs_96512 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#96512:u8
        let s_10_0: bool = fn_state.gs_96512;
        // N s_10_1: branch s_10_0 b20 b11
        if s_10_0 {
            return block_20(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#96513 <= s_11_0
        fn_state.gs_96513 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#96513:u8
        let s_12_0: bool = fn_state.gs_96513;
        // N s_12_1: branch s_12_0 b19 b13
        if s_12_0 {
            return block_19(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#96514 <= s_13_0
        fn_state.gs_96514 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#96514:u8
        let s_14_0: bool = fn_state.gs_96514;
        // N s_14_1: branch s_14_0 b18 b15
        if s_14_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var __HCR_EL2_E2H:u8
        let s_15_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 1u16);
        // C s_15_2: const #1u : u8
        let s_15_2: bool = true;
        // C s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 1u16);
        // D s_15_4: cmp-eq s_15_1 s_15_3
        let s_15_4: bool = ((s_15_1) == (s_15_3));
        // N s_15_5: branch s_15_4 b17 b16
        if s_15_4 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #64s : i64
        let s_16_0: i64 = 64;
        // D s_16_1: read-var t:i
        let s_16_1: i128 = fn_state.t;
        // D s_16_2: call X_read(s_16_1, s_16_0)
        let s_16_2: Bits = X_read(state, tracer, s_16_1, s_16_0);
        // D s_16_3: cast reint s_16_2 -> u64
        let s_16_3: u64 = (s_16_2.value() as u64);
        // D s_16_4: call Mk_SPSR_EL1_Type(s_16_3)
        let s_16_4: ProductType5c790c8ef59cc8b2 = Mk_SPSR_EL1_Type(
            state,
            tracer,
            s_16_3,
        );
        // C s_16_5: const #90648u : u32
        let s_16_5: u32 = 90648;
        // N s_16_6: write-reg s_16_5 <= s_16_4
        let s_16_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_16_5 as isize, s_16_4);
            tracer.write_register(s_16_5 as isize, s_16_4);
        };
        // N s_16_7: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #64s : i64
        let s_17_0: i64 = 64;
        // D s_17_1: read-var t:i
        let s_17_1: i128 = fn_state.t;
        // D s_17_2: call X_read(s_17_1, s_17_0)
        let s_17_2: Bits = X_read(state, tracer, s_17_1, s_17_0);
        // D s_17_3: cast reint s_17_2 -> u64
        let s_17_3: u64 = (s_17_2.value() as u64);
        // D s_17_4: call Mk_SPSR_EL2_Type(s_17_3)
        let s_17_4: ProductType5c790c8ef59cc8b2 = Mk_SPSR_EL2_Type(
            state,
            tracer,
            s_17_3,
        );
        // C s_17_5: const #15736u : u32
        let s_17_5: u32 = 15736;
        // N s_17_6: write-reg s_17_5 <= s_17_4
        let s_17_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_17_5 as isize, s_17_4);
            tracer.write_register(s_17_5 as isize, s_17_4);
        };
        // N s_17_7: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call EXLOCKException(s_18_0)
        let s_18_1: () = EXLOCKException(state, tracer, s_18_0);
        // N s_18_2: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var __HCR_EL2_E2H:u8
        let s_19_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 1u16);
        // C s_19_2: const #1u : u8
        let s_19_2: bool = true;
        // C s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 1u16);
        // D s_19_4: cmp-eq s_19_1 s_19_3
        let s_19_4: bool = ((s_19_1) == (s_19_3));
        // D s_19_5: write-var gs#96514 <= s_19_4
        fn_state.gs_96514 = s_19_4;
        // N s_19_6: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #16976u : u32
        let s_20_0: u32 = 16976;
        // D s_20_1: read-reg s_20_0:u8
        let s_20_1: bool = {
            let value = state.read_register::<bool>(s_20_0 as isize);
            tracer.read_register(s_20_0 as isize, value);
            value
        };
        // D s_20_2: cast zx s_20_1 -> bv
        let s_20_2: Bits = Bits::new(s_20_1 as u128, 1u16);
        // C s_20_3: const #1u : u8
        let s_20_3: bool = true;
        // C s_20_4: cast zx s_20_3 -> bv
        let s_20_4: Bits = Bits::new(s_20_3 as u128, 1u16);
        // D s_20_5: cmp-eq s_20_2 s_20_4
        let s_20_5: bool = ((s_20_2) == (s_20_4));
        // D s_20_6: write-var gs#96513 <= s_20_5
        fn_state.gs_96513 = s_20_5;
        // N s_20_7: jump b12
        return block_12(state, tracer, fn_state);
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
        // S s_21_2: not s_21_1
        let s_21_2: bool = !s_21_1;
        // D s_21_3: write-var gs#96512 <= s_21_2
        fn_state.gs_96512 = s_21_2;
        // N s_21_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call GetCurrentEXLOCKEN(s_22_0)
        let s_22_1: bool = GetCurrentEXLOCKEN(state, tracer, s_22_0);
        // D s_22_2: write-var gs#96511 <= s_22_1
        fn_state.gs_96511 = s_22_1;
        // N s_22_3: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #245u : u32
        let s_23_0: u32 = 245;
        // S s_23_1: call IsFeatureImplemented(s_23_0)
        let s_23_1: bool = IsFeatureImplemented(state, tracer, s_23_0);
        // N s_23_2: branch s_23_1 b53 b24
        if s_23_1 {
            return block_53(state, tracer, fn_state);
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
        // D s_24_1: write-var gs#96517 <= s_24_0
        fn_state.gs_96517 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#96517:u8
        let s_25_0: bool = fn_state.gs_96517;
        // N s_25_1: branch s_25_0 b52 b26
        if s_25_0 {
            return block_52(state, tracer, fn_state);
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
        // D s_26_1: write-var gs#96518 <= s_26_0
        fn_state.gs_96518 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#96518:u8
        let s_27_0: bool = fn_state.gs_96518;
        // N s_27_1: branch s_27_0 b51 b28
        if s_27_0 {
            return block_51(state, tracer, fn_state);
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
        // D s_28_1: write-var gs#96519 <= s_28_0
        fn_state.gs_96519 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#96519:u8
        let s_29_0: bool = fn_state.gs_96519;
        // N s_29_1: branch s_29_0 b44 b30
        if s_29_0 {
            return block_44(state, tracer, fn_state);
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
        // D s_30_1: write-var gs#96522 <= s_30_0
        fn_state.gs_96522 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#96522:u8
        let s_31_0: bool = fn_state.gs_96522;
        // N s_31_1: branch s_31_0 b43 b32
        if s_31_0 {
            return block_43(state, tracer, fn_state);
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
        // N s_32_2: branch s_32_1 b42 b33
        if s_32_1 {
            return block_42(state, tracer, fn_state);
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
        // D s_33_1: write-var gs#96523 <= s_33_0
        fn_state.gs_96523 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#96523:u8
        let s_34_0: bool = fn_state.gs_96523;
        // N s_34_1: branch s_34_0 b41 b35
        if s_34_0 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #() : ()
        let s_35_0: () = ();
        // S s_35_1: call EL2Enabled(s_35_0)
        let s_35_1: bool = EL2Enabled(state, tracer, s_35_0);
        // N s_35_2: branch s_35_1 b40 b36
        if s_35_1 {
            return block_40(state, tracer, fn_state);
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
        // D s_36_1: write-var gs#96524 <= s_36_0
        fn_state.gs_96524 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#96524:u8
        let s_37_0: bool = fn_state.gs_96524;
        // N s_37_1: branch s_37_0 b39 b38
        if s_37_0 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
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
        // C s_39_0: const #352u : u12
        let s_39_0: u16 = 352;
        // C s_39_1: cast zx s_39_0 -> bv
        let s_39_1: Bits = Bits::new(s_39_0 as u128, 12u16);
        // C s_39_2: cast zx s_39_1 -> i
        let s_39_2: i128 = (s_39_1.value() as i128);
        // C s_39_3: cast reint s_39_2 -> i64
        let s_39_3: i64 = (s_39_2 as i64);
        // C s_39_4: const #64s : i64
        let s_39_4: i64 = 64;
        // D s_39_5: read-var t:i
        let s_39_5: i128 = fn_state.t;
        // D s_39_6: call X_read(s_39_5, s_39_4)
        let s_39_6: Bits = X_read(state, tracer, s_39_5, s_39_4);
        // D s_39_7: cast reint s_39_6 -> u64
        let s_39_7: u64 = (s_39_6.value() as u64);
        // C s_39_8: cast zx s_39_3 -> i
        let s_39_8: i128 = (i128::try_from(s_39_3).unwrap());
        // D s_39_9: call NVMem_set(s_39_8, s_39_7)
        let s_39_9: () = NVMem_set(state, tracer, s_39_8, s_39_7);
        // N s_39_10: return
        return;
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #102552u : u32
        let s_40_0: u32 = 102552;
        // D s_40_1: read-reg s_40_0:struct
        let s_40_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_40_0 as isize);
            tracer.read_register(s_40_0 as isize, value);
            value
        };
        // D s_40_2: call _get_HCR_EL2_Type_NV2(s_40_1)
        let s_40_2: bool = u_get_HCR_EL2_Type_NV2(state, tracer, s_40_1);
        // C s_40_3: const #102552u : u32
        let s_40_3: u32 = 102552;
        // D s_40_4: read-reg s_40_3:struct
        let s_40_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_40_3 as isize);
            tracer.read_register(s_40_3 as isize, value);
            value
        };
        // D s_40_5: call _get_HCR_EL2_Type_NV1(s_40_4)
        let s_40_5: bool = u_get_HCR_EL2_Type_NV1(state, tracer, s_40_4);
        // C s_40_6: const #102552u : u32
        let s_40_6: u32 = 102552;
        // D s_40_7: read-reg s_40_6:struct
        let s_40_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_40_6 as isize);
            tracer.read_register(s_40_6 as isize, value);
            value
        };
        // D s_40_8: call _get_HCR_EL2_Type_NV(s_40_7)
        let s_40_8: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_40_7);
        // D s_40_9: cast zx s_40_5 -> bv
        let s_40_9: Bits = Bits::new(s_40_5 as u128, 1u16);
        // D s_40_10: cast zx s_40_8 -> bv
        let s_40_10: Bits = Bits::new(s_40_8 as u128, 1u16);
        // D s_40_11: cast reint s_40_9 -> u128
        let s_40_11: u128 = (s_40_9.value() as u128);
        // D s_40_12: size-of s_40_9
        let s_40_12: u16 = s_40_9.length();
        // D s_40_13: cast reint s_40_10 -> u128
        let s_40_13: u128 = (s_40_10.value() as u128);
        // D s_40_14: size-of s_40_10
        let s_40_14: u16 = s_40_10.length();
        // D s_40_15: lsl s_40_11 s_40_14
        let s_40_15: u128 = s_40_11 << s_40_14;
        // D s_40_16: or s_40_15 s_40_13
        let s_40_16: u128 = ((s_40_15) | (s_40_13));
        // D s_40_17: add s_40_12 s_40_14
        let s_40_17: u16 = (s_40_12 + s_40_14);
        // D s_40_18: create-bits s_40_16 s_40_17
        let s_40_18: Bits = Bits::new(s_40_16, s_40_17);
        // D s_40_19: cast reint s_40_18 -> u8
        let s_40_19: u8 = (s_40_18.value() as u8);
        // D s_40_20: cast zx s_40_2 -> bv
        let s_40_20: Bits = Bits::new(s_40_2 as u128, 1u16);
        // D s_40_21: cast zx s_40_19 -> bv
        let s_40_21: Bits = Bits::new(s_40_19 as u128, 2u16);
        // D s_40_22: cast reint s_40_20 -> u128
        let s_40_22: u128 = (s_40_20.value() as u128);
        // D s_40_23: size-of s_40_20
        let s_40_23: u16 = s_40_20.length();
        // D s_40_24: cast reint s_40_21 -> u128
        let s_40_24: u128 = (s_40_21.value() as u128);
        // D s_40_25: size-of s_40_21
        let s_40_25: u16 = s_40_21.length();
        // D s_40_26: lsl s_40_22 s_40_25
        let s_40_26: u128 = s_40_22 << s_40_25;
        // D s_40_27: or s_40_26 s_40_24
        let s_40_27: u128 = ((s_40_26) | (s_40_24));
        // D s_40_28: add s_40_23 s_40_25
        let s_40_28: u16 = (s_40_23 + s_40_25);
        // D s_40_29: create-bits s_40_27 s_40_28
        let s_40_29: Bits = Bits::new(s_40_27, s_40_28);
        // D s_40_30: cast reint s_40_29 -> u8
        let s_40_30: u8 = (s_40_29.value() as u8);
        // D s_40_31: cast zx s_40_30 -> bv
        let s_40_31: Bits = Bits::new(s_40_30 as u128, 3u16);
        // C s_40_32: const #7u : u8
        let s_40_32: u8 = 7;
        // C s_40_33: cast zx s_40_32 -> bv
        let s_40_33: Bits = Bits::new(s_40_32 as u128, 3u16);
        // D s_40_34: cmp-eq s_40_31 s_40_33
        let s_40_34: bool = ((s_40_31) == (s_40_33));
        // D s_40_35: write-var gs#96524 <= s_40_34
        fn_state.gs_96524 = s_40_34;
        // N s_40_36: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #24u : u8
        let s_41_0: u8 = 24;
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
        // C s_42_32: const #3u : u8
        let s_42_32: u8 = 3;
        // C s_42_33: cast zx s_42_32 -> bv
        let s_42_33: Bits = Bits::new(s_42_32 as u128, 3u16);
        // D s_42_34: cmp-eq s_42_31 s_42_33
        let s_42_34: bool = ((s_42_31) == (s_42_33));
        // D s_42_35: write-var gs#96523 <= s_42_34
        fn_state.gs_96523 = s_42_34;
        // N s_42_36: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #() : ()
        let s_43_0: () = ();
        // S s_43_1: call EXLOCKException(s_43_0)
        let s_43_1: () = EXLOCKException(state, tracer, s_43_0);
        // N s_43_2: return
        return;
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #102552u : u32
        let s_44_0: u32 = 102552;
        // D s_44_1: read-reg s_44_0:struct
        let s_44_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_44_0 as isize);
            tracer.read_register(s_44_0 as isize, value);
            value
        };
        // D s_44_2: call _get_HCR_EL2_Type_NV(s_44_1)
        let s_44_2: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_44_1);
        // D s_44_3: cast zx s_44_2 -> bv
        let s_44_3: Bits = Bits::new(s_44_2 as u128, 1u16);
        // C s_44_4: const #0u : u8
        let s_44_4: bool = false;
        // C s_44_5: cast zx s_44_4 -> bv
        let s_44_5: Bits = Bits::new(s_44_4 as u128, 1u16);
        // D s_44_6: cmp-eq s_44_3 s_44_5
        let s_44_6: bool = ((s_44_3) == (s_44_5));
        // N s_44_7: branch s_44_6 b50 b45
        if s_44_6 {
            return block_50(state, tracer, fn_state);
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
        // N s_45_2: branch s_45_1 b49 b46
        if s_45_1 {
            return block_49(state, tracer, fn_state);
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
        // D s_46_1: write-var gs#96520 <= s_46_0
        fn_state.gs_96520 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#96520:u8
        let s_47_0: bool = fn_state.gs_96520;
        // D s_47_1: write-var gs#96521 <= s_47_0
        fn_state.gs_96521 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#96521:u8
        let s_48_0: bool = fn_state.gs_96521;
        // D s_48_1: write-var gs#96522 <= s_48_0
        fn_state.gs_96522 = s_48_0;
        // N s_48_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #102552u : u32
        let s_49_0: u32 = 102552;
        // D s_49_1: read-reg s_49_0:struct
        let s_49_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_49_0 as isize);
            tracer.read_register(s_49_0 as isize, value);
            value
        };
        // D s_49_2: call _get_HCR_EL2_Type_NV1(s_49_1)
        let s_49_2: bool = u_get_HCR_EL2_Type_NV1(state, tracer, s_49_1);
        // C s_49_3: const #102552u : u32
        let s_49_3: u32 = 102552;
        // D s_49_4: read-reg s_49_3:struct
        let s_49_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_49_3 as isize);
            tracer.read_register(s_49_3 as isize, value);
            value
        };
        // D s_49_5: call _get_HCR_EL2_Type_NV(s_49_4)
        let s_49_5: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_49_4);
        // D s_49_6: cast zx s_49_2 -> bv
        let s_49_6: Bits = Bits::new(s_49_2 as u128, 1u16);
        // D s_49_7: cast zx s_49_5 -> bv
        let s_49_7: Bits = Bits::new(s_49_5 as u128, 1u16);
        // D s_49_8: cast reint s_49_6 -> u128
        let s_49_8: u128 = (s_49_6.value() as u128);
        // D s_49_9: size-of s_49_6
        let s_49_9: u16 = s_49_6.length();
        // D s_49_10: cast reint s_49_7 -> u128
        let s_49_10: u128 = (s_49_7.value() as u128);
        // D s_49_11: size-of s_49_7
        let s_49_11: u16 = s_49_7.length();
        // D s_49_12: lsl s_49_8 s_49_11
        let s_49_12: u128 = s_49_8 << s_49_11;
        // D s_49_13: or s_49_12 s_49_10
        let s_49_13: u128 = ((s_49_12) | (s_49_10));
        // D s_49_14: add s_49_9 s_49_11
        let s_49_14: u16 = (s_49_9 + s_49_11);
        // D s_49_15: create-bits s_49_13 s_49_14
        let s_49_15: Bits = Bits::new(s_49_13, s_49_14);
        // D s_49_16: cast reint s_49_15 -> u8
        let s_49_16: u8 = (s_49_15.value() as u8);
        // D s_49_17: cast zx s_49_16 -> bv
        let s_49_17: Bits = Bits::new(s_49_16 as u128, 2u16);
        // C s_49_18: const #1u : u8
        let s_49_18: u8 = 1;
        // C s_49_19: cast zx s_49_18 -> bv
        let s_49_19: Bits = Bits::new(s_49_18 as u128, 2u16);
        // D s_49_20: cmp-eq s_49_17 s_49_19
        let s_49_20: bool = ((s_49_17) == (s_49_19));
        // D s_49_21: write-var gs#96520 <= s_49_20
        fn_state.gs_96520 = s_49_20;
        // N s_49_22: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #1u : u8
        let s_50_0: bool = true;
        // D s_50_1: write-var gs#96521 <= s_50_0
        fn_state.gs_96521 = s_50_0;
        // N s_50_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #16976u : u32
        let s_51_0: u32 = 16976;
        // D s_51_1: read-reg s_51_0:u8
        let s_51_1: bool = {
            let value = state.read_register::<bool>(s_51_0 as isize);
            tracer.read_register(s_51_0 as isize, value);
            value
        };
        // D s_51_2: cast zx s_51_1 -> bv
        let s_51_2: Bits = Bits::new(s_51_1 as u128, 1u16);
        // C s_51_3: const #1u : u8
        let s_51_3: bool = true;
        // C s_51_4: cast zx s_51_3 -> bv
        let s_51_4: Bits = Bits::new(s_51_3 as u128, 1u16);
        // D s_51_5: cmp-eq s_51_2 s_51_4
        let s_51_5: bool = ((s_51_2) == (s_51_4));
        // D s_51_6: write-var gs#96519 <= s_51_5
        fn_state.gs_96519 = s_51_5;
        // N s_51_7: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #() : ()
        let s_52_0: () = ();
        // S s_52_1: call Halted(s_52_0)
        let s_52_1: bool = Halted(state, tracer, s_52_0);
        // S s_52_2: not s_52_1
        let s_52_2: bool = !s_52_1;
        // D s_52_3: write-var gs#96518 <= s_52_2
        fn_state.gs_96518 = s_52_2;
        // N s_52_4: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #() : ()
        let s_53_0: () = ();
        // S s_53_1: call GetCurrentEXLOCKEN(s_53_0)
        let s_53_1: bool = GetCurrentEXLOCKEN(state, tracer, s_53_0);
        // D s_53_2: write-var gs#96517 <= s_53_1
        fn_state.gs_96517 = s_53_1;
        // N s_53_3: jump b25
        return block_25(state, tracer, fn_state);
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
}
