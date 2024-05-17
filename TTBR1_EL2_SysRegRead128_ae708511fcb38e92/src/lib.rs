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
use u_get_HFGRTR_EL2_Type_TTBR1_EL1::*;
use u_get_HCR_EL2_Type_NV2::*;
use u_get_SCR_EL3_Type_FGTEn::*;
use u_get_HCR_EL2_Type_E2H::*;
use u_get_SCR_EL3_Type_D128En::*;
use Halted::*;
use u_get_HCR_EL2_Type_TRVM::*;
use IsFeatureImplemented::*;
use AArch64_SystemAccessTrap::*;
use u__IMPDEF_boolean::*;
use u_get_HCR_EL2_Type_NV1::*;
use X_set::*;
use u_get_HCR_EL2_Type_NV::*;
use u_get_EDSCR_Type_SDD::*;
use TTBR1_EL1_read::*;
use EL2Enabled::*;
use NVMem_read__1::*;
use Split::*;
use EDSCR_read::*;
use common::*;
pub fn TTBR1_EL2_SysRegRead128_ae708511fcb38e92<T: Tracer>(
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
        gs_137428: bool,
        gs_671136: ProductTypebc91b195b0b2a883,
        u__HCR_EL2_E2H: bool,
        gs_137453: bool,
        gs_137448: bool,
        gs_137450: bool,
        gs_137429: bool,
        gs_137430: bool,
        gs_137451: bool,
        gs_137446: bool,
        gs_137447: bool,
        gs_137454: bool,
        gs_137456: bool,
        u__PSTATE_EL: u8,
        gs_137449: bool,
        gs_137457: bool,
        gs_137473: bool,
        ga_239511: ProductType782ac6922b48c20d,
        u__SCR_EL3_D128En: bool,
        gs_671144: ProductTypebc91b195b0b2a883,
        u__EDSCR_SDD: bool,
        u__SCR_EL3_FGTEn: bool,
        gs_671181: ProductTypebc91b195b0b2a883,
        gs_137452: bool,
        gs_137427: bool,
        gs_671120: ProductTypebc91b195b0b2a883,
        u__HCR_EL2_TRVM: bool,
        gs_137455: bool,
        gs_671194: ProductTypebc91b195b0b2a883,
        u__HFGRTR_EL2_TTBR1_EL1: bool,
        ga_239546: ProductType782ac6922b48c20d,
        gs_671188: Bits,
        ga_239538: ProductType782ac6922b48c20d,
        gs_137431: bool,
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
        // D s_0_13: call _get_HCR_EL2_Type_TRVM(s_0_12)
        let s_0_13: bool = u_get_HCR_EL2_Type_TRVM(state, tracer, s_0_12);
        // D s_0_14: write-var __HCR_EL2_TRVM <= s_0_13
        fn_state.u__HCR_EL2_TRVM = s_0_13;
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
        // C s_0_19: const #16592u : u32
        let s_0_19: u32 = 16592;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_HFGRTR_EL2_Type_TTBR1_EL1(s_0_20)
        let s_0_21: bool = u_get_HFGRTR_EL2_Type_TTBR1_EL1(state, tracer, s_0_20);
        // D s_0_22: write-var __HFGRTR_EL2_TTBR1_EL1 <= s_0_21
        fn_state.u__HFGRTR_EL2_TTBR1_EL1 = s_0_21;
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
        // N s_0_33: branch s_0_32 b83 b1
        if s_0_32 {
            return block_83(state, tracer, fn_state);
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
        // D s_5_2: write-var ga#239546 <= s_5_1
        fn_state.ga_239546 = s_5_1;
        // D s_5_3: read-var ga#239546.0:struct
        let s_5_3: u128 = fn_state.ga_239546._0;
        // C s_5_4: const #64s : i64
        let s_5_4: i64 = 64;
        // D s_5_5: cast zx s_5_3 -> bv
        let s_5_5: Bits = Bits::new(s_5_3 as u128, 128u16);
        // C s_5_6: cast zx s_5_4 -> i
        let s_5_6: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_7: call Split(s_5_5, s_5_6)
        let s_5_7: ProductTypebc91b195b0b2a883 = Split(state, tracer, s_5_5, s_5_6);
        // D s_5_8: write-var gs#671120 <= s_5_7
        fn_state.gs_671120 = s_5_7;
        // D s_5_9: read-var gs#671120.0:struct
        let s_5_9: Bits = fn_state.gs_671120._0;
        // D s_5_10: cast reint s_5_9 -> u64
        let s_5_10: u64 = (s_5_9.value() as u64);
        // D s_5_11: read-var gs#671120.1:struct
        let s_5_11: Bits = fn_state.gs_671120._1;
        // D s_5_12: cast reint s_5_11 -> u64
        let s_5_12: u64 = (s_5_11.value() as u64);
        // C s_5_13: const #1s : i
        let s_5_13: i128 = 1;
        // D s_5_14: read-var t:i
        let s_5_14: i128 = fn_state.t;
        // D s_5_15: add s_5_14 s_5_13
        let s_5_15: i128 = (s_5_14 + s_5_13);
        // C s_5_16: const #64s : i64
        let s_5_16: i64 = 64;
        // D s_5_17: cast zx s_5_10 -> bv
        let s_5_17: Bits = Bits::new(s_5_10 as u128, 64u16);
        // D s_5_18: call X_set(s_5_15, s_5_16, s_5_17)
        let s_5_18: () = X_set(state, tracer, s_5_15, s_5_16, s_5_17);
        // C s_5_19: const #64s : i64
        let s_5_19: i64 = 64;
        // D s_5_20: cast zx s_5_12 -> bv
        let s_5_20: Bits = Bits::new(s_5_12 as u128, 64u16);
        // D s_5_21: read-var t:i
        let s_5_21: i128 = fn_state.t;
        // D s_5_22: call X_set(s_5_21, s_5_19, s_5_20)
        let s_5_22: () = X_set(state, tracer, s_5_21, s_5_19, s_5_20);
        // N s_5_23: return
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
        // D s_7_1: write-var gs#137427 <= s_7_0
        fn_state.gs_137427 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#137427:u8
        let s_8_0: bool = fn_state.gs_137427;
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
        // D s_9_1: write-var gs#137428 <= s_9_0
        fn_state.gs_137428 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#137428:u8
        let s_10_0: bool = fn_state.gs_137428;
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
        // D s_11_1: write-var gs#137429 <= s_11_0
        fn_state.gs_137429 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#137429:u8
        let s_12_0: bool = fn_state.gs_137429;
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
        // D s_13_1: write-var gs#137430 <= s_13_0
        fn_state.gs_137430 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#137430:u8
        let s_14_0: bool = fn_state.gs_137430;
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
        // D s_16_1: write-var gs#137431 <= s_16_0
        fn_state.gs_137431 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#137431:u8
        let s_17_0: bool = fn_state.gs_137431;
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
        // D s_19_2: write-var ga#239538 <= s_19_1
        fn_state.ga_239538 = s_19_1;
        // D s_19_3: read-var ga#239538.0:struct
        let s_19_3: u128 = fn_state.ga_239538._0;
        // C s_19_4: const #64s : i64
        let s_19_4: i64 = 64;
        // D s_19_5: cast zx s_19_3 -> bv
        let s_19_5: Bits = Bits::new(s_19_3 as u128, 128u16);
        // C s_19_6: cast zx s_19_4 -> i
        let s_19_6: i128 = (i128::try_from(s_19_4).unwrap());
        // D s_19_7: call Split(s_19_5, s_19_6)
        let s_19_7: ProductTypebc91b195b0b2a883 = Split(state, tracer, s_19_5, s_19_6);
        // D s_19_8: write-var gs#671136 <= s_19_7
        fn_state.gs_671136 = s_19_7;
        // D s_19_9: read-var gs#671136.0:struct
        let s_19_9: Bits = fn_state.gs_671136._0;
        // D s_19_10: cast reint s_19_9 -> u64
        let s_19_10: u64 = (s_19_9.value() as u64);
        // D s_19_11: read-var gs#671136.1:struct
        let s_19_11: Bits = fn_state.gs_671136._1;
        // D s_19_12: cast reint s_19_11 -> u64
        let s_19_12: u64 = (s_19_11.value() as u64);
        // C s_19_13: const #1s : i
        let s_19_13: i128 = 1;
        // D s_19_14: read-var t:i
        let s_19_14: i128 = fn_state.t;
        // D s_19_15: add s_19_14 s_19_13
        let s_19_15: i128 = (s_19_14 + s_19_13);
        // C s_19_16: const #64s : i64
        let s_19_16: i64 = 64;
        // D s_19_17: cast zx s_19_10 -> bv
        let s_19_17: Bits = Bits::new(s_19_10 as u128, 64u16);
        // D s_19_18: call X_set(s_19_15, s_19_16, s_19_17)
        let s_19_18: () = X_set(state, tracer, s_19_15, s_19_16, s_19_17);
        // C s_19_19: const #64s : i64
        let s_19_19: i64 = 64;
        // D s_19_20: cast zx s_19_12 -> bv
        let s_19_20: Bits = Bits::new(s_19_12 as u128, 64u16);
        // D s_19_21: read-var t:i
        let s_19_21: i128 = fn_state.t;
        // D s_19_22: call X_set(s_19_21, s_19_19, s_19_20)
        let s_19_22: () = X_set(state, tracer, s_19_21, s_19_19, s_19_20);
        // N s_19_23: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #18432u : u32
        let s_20_0: u32 = 18432;
        // D s_20_1: read-reg s_20_0:u128
        let s_20_1: u128 = {
            let value = state.read_register::<u128>(s_20_0 as isize);
            tracer.read_register(s_20_0 as isize, value);
            value
        };
        // C s_20_2: const #64s : i64
        let s_20_2: i64 = 64;
        // D s_20_3: cast zx s_20_1 -> bv
        let s_20_3: Bits = Bits::new(s_20_1 as u128, 128u16);
        // C s_20_4: cast zx s_20_2 -> i
        let s_20_4: i128 = (i128::try_from(s_20_2).unwrap());
        // D s_20_5: call Split(s_20_3, s_20_4)
        let s_20_5: ProductTypebc91b195b0b2a883 = Split(state, tracer, s_20_3, s_20_4);
        // D s_20_6: write-var gs#671144 <= s_20_5
        fn_state.gs_671144 = s_20_5;
        // D s_20_7: read-var gs#671144.0:struct
        let s_20_7: Bits = fn_state.gs_671144._0;
        // D s_20_8: cast reint s_20_7 -> u64
        let s_20_8: u64 = (s_20_7.value() as u64);
        // D s_20_9: read-var gs#671144.1:struct
        let s_20_9: Bits = fn_state.gs_671144._1;
        // D s_20_10: cast reint s_20_9 -> u64
        let s_20_10: u64 = (s_20_9.value() as u64);
        // C s_20_11: const #1s : i
        let s_20_11: i128 = 1;
        // D s_20_12: read-var t:i
        let s_20_12: i128 = fn_state.t;
        // D s_20_13: add s_20_12 s_20_11
        let s_20_13: i128 = (s_20_12 + s_20_11);
        // C s_20_14: const #64s : i64
        let s_20_14: i64 = 64;
        // D s_20_15: cast zx s_20_8 -> bv
        let s_20_15: Bits = Bits::new(s_20_8 as u128, 64u16);
        // D s_20_16: call X_set(s_20_13, s_20_14, s_20_15)
        let s_20_16: () = X_set(state, tracer, s_20_13, s_20_14, s_20_15);
        // C s_20_17: const #64s : i64
        let s_20_17: i64 = 64;
        // D s_20_18: cast zx s_20_10 -> bv
        let s_20_18: Bits = Bits::new(s_20_10 as u128, 64u16);
        // D s_20_19: read-var t:i
        let s_20_19: i128 = fn_state.t;
        // D s_20_20: call X_set(s_20_19, s_20_17, s_20_18)
        let s_20_20: () = X_set(state, tracer, s_20_19, s_20_17, s_20_18);
        // N s_20_21: return
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
        // D s_22_1: write-var gs#137446 <= s_22_0
        fn_state.gs_137446 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#137446:u8
        let s_23_0: bool = fn_state.gs_137446;
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
        // D s_26_5: write-var gs#137446 <= s_26_4
        fn_state.gs_137446 = s_26_4;
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
        // D s_27_5: write-var gs#137431 <= s_27_4
        fn_state.gs_137431 = s_27_4;
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
        // D s_29_5: write-var gs#137430 <= s_29_4
        fn_state.gs_137430 = s_29_4;
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
        // D s_30_2: write-var gs#137429 <= s_30_1
        fn_state.gs_137429 = s_30_1;
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
        // D s_31_5: write-var gs#137428 <= s_31_4
        fn_state.gs_137428 = s_31_4;
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
        // D s_32_4: write-var gs#137427 <= s_32_3
        fn_state.gs_137427 = s_32_3;
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
        // N s_33_2: branch s_33_1 b82 b34
        if s_33_1 {
            return block_82(state, tracer, fn_state);
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
        // D s_34_1: write-var gs#137447 <= s_34_0
        fn_state.gs_137447 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#137447:u8
        let s_35_0: bool = fn_state.gs_137447;
        // N s_35_1: branch s_35_0 b81 b36
        if s_35_0 {
            return block_81(state, tracer, fn_state);
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
        // D s_36_1: write-var gs#137448 <= s_36_0
        fn_state.gs_137448 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#137448:u8
        let s_37_0: bool = fn_state.gs_137448;
        // N s_37_1: branch s_37_0 b80 b38
        if s_37_0 {
            return block_80(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#137449 <= s_38_0
        fn_state.gs_137449 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#137449:u8
        let s_39_0: bool = fn_state.gs_137449;
        // N s_39_1: branch s_39_0 b79 b40
        if s_39_0 {
            return block_79(state, tracer, fn_state);
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
        // D s_40_1: write-var gs#137450 <= s_40_0
        fn_state.gs_137450 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#137450:u8
        let s_41_0: bool = fn_state.gs_137450;
        // N s_41_1: branch s_41_0 b78 b42
        if s_41_0 {
            return block_78(state, tracer, fn_state);
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
        // N s_42_2: branch s_42_1 b77 b43
        if s_42_1 {
            return block_77(state, tracer, fn_state);
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
        // D s_43_1: write-var gs#137451 <= s_43_0
        fn_state.gs_137451 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#137451:u8
        let s_44_0: bool = fn_state.gs_137451;
        // N s_44_1: branch s_44_0 b76 b45
        if s_44_0 {
            return block_76(state, tracer, fn_state);
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
        // N s_45_2: branch s_45_1 b75 b46
        if s_45_1 {
            return block_75(state, tracer, fn_state);
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
        // D s_46_1: write-var gs#137452 <= s_46_0
        fn_state.gs_137452 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#137452:u8
        let s_47_0: bool = fn_state.gs_137452;
        // N s_47_1: branch s_47_0 b71 b48
        if s_47_0 {
            return block_71(state, tracer, fn_state);
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
        // D s_48_1: write-var gs#137454 <= s_48_0
        fn_state.gs_137454 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#137454:u8
        let s_49_0: bool = fn_state.gs_137454;
        // N s_49_1: branch s_49_0 b70 b50
        if s_49_0 {
            return block_70(state, tracer, fn_state);
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
        // D s_50_1: write-var gs#137455 <= s_50_0
        fn_state.gs_137455 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#137455:u8
        let s_51_0: bool = fn_state.gs_137455;
        // N s_51_1: branch s_51_0 b69 b52
        if s_51_0 {
            return block_69(state, tracer, fn_state);
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
        // N s_52_4: branch s_52_3 b68 b53
        if s_52_3 {
            return block_68(state, tracer, fn_state);
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
        // D s_53_1: write-var gs#137456 <= s_53_0
        fn_state.gs_137456 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#137456:u8
        let s_54_0: bool = fn_state.gs_137456;
        // N s_54_1: branch s_54_0 b62 b55
        if s_54_0 {
            return block_62(state, tracer, fn_state);
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
        // N s_55_2: branch s_55_1 b61 b56
        if s_55_1 {
            return block_61(state, tracer, fn_state);
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
        // D s_56_1: write-var gs#137457 <= s_56_0
        fn_state.gs_137457 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#137457:u8
        let s_57_0: bool = fn_state.gs_137457;
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
        // D s_58_2: write-var ga#239511 <= s_58_1
        fn_state.ga_239511 = s_58_1;
        // D s_58_3: read-var ga#239511.0:struct
        let s_58_3: u128 = fn_state.ga_239511._0;
        // C s_58_4: const #64s : i64
        let s_58_4: i64 = 64;
        // D s_58_5: cast zx s_58_3 -> bv
        let s_58_5: Bits = Bits::new(s_58_3 as u128, 128u16);
        // C s_58_6: cast zx s_58_4 -> i
        let s_58_6: i128 = (i128::try_from(s_58_4).unwrap());
        // D s_58_7: call Split(s_58_5, s_58_6)
        let s_58_7: ProductTypebc91b195b0b2a883 = Split(state, tracer, s_58_5, s_58_6);
        // D s_58_8: write-var gs#671181 <= s_58_7
        fn_state.gs_671181 = s_58_7;
        // D s_58_9: read-var gs#671181.0:struct
        let s_58_9: Bits = fn_state.gs_671181._0;
        // D s_58_10: cast reint s_58_9 -> u64
        let s_58_10: u64 = (s_58_9.value() as u64);
        // D s_58_11: read-var gs#671181.1:struct
        let s_58_11: Bits = fn_state.gs_671181._1;
        // D s_58_12: cast reint s_58_11 -> u64
        let s_58_12: u64 = (s_58_11.value() as u64);
        // C s_58_13: const #1s : i
        let s_58_13: i128 = 1;
        // D s_58_14: read-var t:i
        let s_58_14: i128 = fn_state.t;
        // D s_58_15: add s_58_14 s_58_13
        let s_58_15: i128 = (s_58_14 + s_58_13);
        // C s_58_16: const #64s : i64
        let s_58_16: i64 = 64;
        // D s_58_17: cast zx s_58_10 -> bv
        let s_58_17: Bits = Bits::new(s_58_10 as u128, 64u16);
        // D s_58_18: call X_set(s_58_15, s_58_16, s_58_17)
        let s_58_18: () = X_set(state, tracer, s_58_15, s_58_16, s_58_17);
        // C s_58_19: const #64s : i64
        let s_58_19: i64 = 64;
        // D s_58_20: cast zx s_58_12 -> bv
        let s_58_20: Bits = Bits::new(s_58_12 as u128, 64u16);
        // D s_58_21: read-var t:i
        let s_58_21: i128 = fn_state.t;
        // D s_58_22: call X_set(s_58_21, s_58_19, s_58_20)
        let s_58_22: () = X_set(state, tracer, s_58_21, s_58_19, s_58_20);
        // N s_58_23: return
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
        // C s_59_5: cast zx s_59_3 -> i
        let s_59_5: i128 = (i128::try_from(s_59_3).unwrap());
        // S s_59_6: call NVMem_read__1(s_59_5, s_59_4)
        let s_59_6: Bits = NVMem_read__1(state, tracer, s_59_5, s_59_4);
        // D s_59_7: write-var gs#671188 <= s_59_6
        fn_state.gs_671188 = s_59_6;
        // N s_59_8: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#671188:bv
        let s_60_0: Bits = fn_state.gs_671188;
        // D s_60_1: cast reint s_60_0 -> u128
        let s_60_1: u128 = (s_60_0.value() as u128);
        // C s_60_2: const #64s : i64
        let s_60_2: i64 = 64;
        // D s_60_3: cast zx s_60_1 -> bv
        let s_60_3: Bits = Bits::new(s_60_1 as u128, 128u16);
        // C s_60_4: cast zx s_60_2 -> i
        let s_60_4: i128 = (i128::try_from(s_60_2).unwrap());
        // D s_60_5: call Split(s_60_3, s_60_4)
        let s_60_5: ProductTypebc91b195b0b2a883 = Split(state, tracer, s_60_3, s_60_4);
        // D s_60_6: write-var gs#671194 <= s_60_5
        fn_state.gs_671194 = s_60_5;
        // D s_60_7: read-var gs#671194.0:struct
        let s_60_7: Bits = fn_state.gs_671194._0;
        // D s_60_8: cast reint s_60_7 -> u64
        let s_60_8: u64 = (s_60_7.value() as u64);
        // D s_60_9: read-var gs#671194.1:struct
        let s_60_9: Bits = fn_state.gs_671194._1;
        // D s_60_10: cast reint s_60_9 -> u64
        let s_60_10: u64 = (s_60_9.value() as u64);
        // C s_60_11: const #1s : i
        let s_60_11: i128 = 1;
        // D s_60_12: read-var t:i
        let s_60_12: i128 = fn_state.t;
        // D s_60_13: add s_60_12 s_60_11
        let s_60_13: i128 = (s_60_12 + s_60_11);
        // C s_60_14: const #64s : i64
        let s_60_14: i64 = 64;
        // D s_60_15: cast zx s_60_8 -> bv
        let s_60_15: Bits = Bits::new(s_60_8 as u128, 64u16);
        // D s_60_16: call X_set(s_60_13, s_60_14, s_60_15)
        let s_60_16: () = X_set(state, tracer, s_60_13, s_60_14, s_60_15);
        // C s_60_17: const #64s : i64
        let s_60_17: i64 = 64;
        // D s_60_18: cast zx s_60_10 -> bv
        let s_60_18: Bits = Bits::new(s_60_10 as u128, 64u16);
        // D s_60_19: read-var t:i
        let s_60_19: i128 = fn_state.t;
        // D s_60_20: call X_set(s_60_19, s_60_17, s_60_18)
        let s_60_20: () = X_set(state, tracer, s_60_19, s_60_17, s_60_18);
        // N s_60_21: return
        return;
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #102552u : u32
        let s_61_0: u32 = 102552;
        // D s_61_1: read-reg s_61_0:struct
        let s_61_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_61_0 as isize);
            tracer.read_register(s_61_0 as isize, value);
            value
        };
        // D s_61_2: call _get_HCR_EL2_Type_NV2(s_61_1)
        let s_61_2: bool = u_get_HCR_EL2_Type_NV2(state, tracer, s_61_1);
        // C s_61_3: const #102552u : u32
        let s_61_3: u32 = 102552;
        // D s_61_4: read-reg s_61_3:struct
        let s_61_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_61_3 as isize);
            tracer.read_register(s_61_3 as isize, value);
            value
        };
        // D s_61_5: call _get_HCR_EL2_Type_NV1(s_61_4)
        let s_61_5: bool = u_get_HCR_EL2_Type_NV1(state, tracer, s_61_4);
        // C s_61_6: const #102552u : u32
        let s_61_6: u32 = 102552;
        // D s_61_7: read-reg s_61_6:struct
        let s_61_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_61_6 as isize);
            tracer.read_register(s_61_6 as isize, value);
            value
        };
        // D s_61_8: call _get_HCR_EL2_Type_NV(s_61_7)
        let s_61_8: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_61_7);
        // D s_61_9: cast zx s_61_5 -> bv
        let s_61_9: Bits = Bits::new(s_61_5 as u128, 1u16);
        // D s_61_10: cast zx s_61_8 -> bv
        let s_61_10: Bits = Bits::new(s_61_8 as u128, 1u16);
        // D s_61_11: cast reint s_61_9 -> u128
        let s_61_11: u128 = (s_61_9.value() as u128);
        // D s_61_12: size-of s_61_9
        let s_61_12: u16 = s_61_9.length();
        // D s_61_13: cast reint s_61_10 -> u128
        let s_61_13: u128 = (s_61_10.value() as u128);
        // D s_61_14: size-of s_61_10
        let s_61_14: u16 = s_61_10.length();
        // D s_61_15: lsl s_61_11 s_61_14
        let s_61_15: u128 = s_61_11 << s_61_14;
        // D s_61_16: or s_61_15 s_61_13
        let s_61_16: u128 = ((s_61_15) | (s_61_13));
        // D s_61_17: add s_61_12 s_61_14
        let s_61_17: u16 = (s_61_12 + s_61_14);
        // D s_61_18: create-bits s_61_16 s_61_17
        let s_61_18: Bits = Bits::new(s_61_16, s_61_17);
        // D s_61_19: cast reint s_61_18 -> u8
        let s_61_19: u8 = (s_61_18.value() as u8);
        // D s_61_20: cast zx s_61_2 -> bv
        let s_61_20: Bits = Bits::new(s_61_2 as u128, 1u16);
        // D s_61_21: cast zx s_61_19 -> bv
        let s_61_21: Bits = Bits::new(s_61_19 as u128, 2u16);
        // D s_61_22: cast reint s_61_20 -> u128
        let s_61_22: u128 = (s_61_20.value() as u128);
        // D s_61_23: size-of s_61_20
        let s_61_23: u16 = s_61_20.length();
        // D s_61_24: cast reint s_61_21 -> u128
        let s_61_24: u128 = (s_61_21.value() as u128);
        // D s_61_25: size-of s_61_21
        let s_61_25: u16 = s_61_21.length();
        // D s_61_26: lsl s_61_22 s_61_25
        let s_61_26: u128 = s_61_22 << s_61_25;
        // D s_61_27: or s_61_26 s_61_24
        let s_61_27: u128 = ((s_61_26) | (s_61_24));
        // D s_61_28: add s_61_23 s_61_25
        let s_61_28: u16 = (s_61_23 + s_61_25);
        // D s_61_29: create-bits s_61_27 s_61_28
        let s_61_29: Bits = Bits::new(s_61_27, s_61_28);
        // D s_61_30: cast reint s_61_29 -> u8
        let s_61_30: u8 = (s_61_29.value() as u8);
        // D s_61_31: cast zx s_61_30 -> bv
        let s_61_31: Bits = Bits::new(s_61_30 as u128, 3u16);
        // C s_61_32: const #7u : u8
        let s_61_32: u8 = 7;
        // C s_61_33: cast zx s_61_32 -> bv
        let s_61_33: Bits = Bits::new(s_61_32 as u128, 3u16);
        // D s_61_34: cmp-eq s_61_31 s_61_33
        let s_61_34: bool = ((s_61_31) == (s_61_33));
        // D s_61_35: write-var gs#137457 <= s_61_34
        fn_state.gs_137457 = s_61_34;
        // N s_61_36: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #() : ()
        let s_62_0: () = ();
        // S s_62_1: call Halted(s_62_0)
        let s_62_1: bool = Halted(state, tracer, s_62_0);
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
        // D s_63_1: write-var gs#137473 <= s_63_0
        fn_state.gs_137473 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#137473:u8
        let s_64_0: bool = fn_state.gs_137473;
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
        // C s_65_0: const #20u : u8
        let s_65_0: u8 = 20;
        // C s_65_1: cast zx s_65_0 -> bv
        let s_65_1: Bits = Bits::new(s_65_0 as u128, 8u16);
        // C s_65_2: cast zx s_65_1 -> i
        let s_65_2: i128 = (s_65_1.value() as i128);
        // C s_65_3: cast reint s_65_2 -> i64
        let s_65_3: i64 = (s_65_2 as i64);
        // C s_65_4: cast zx s_65_3 -> i
        let s_65_4: i128 = (i128::try_from(s_65_3).unwrap());
        // C s_65_5: const #424u : u32
        let s_65_5: u32 = 424;
        // D s_65_6: read-reg s_65_5:u8
        let s_65_6: u8 = {
            let value = state.read_register::<u8>(s_65_5 as isize);
            tracer.read_register(s_65_5 as isize, value);
            value
        };
        // D s_65_7: call AArch64_SystemAccessTrap(s_65_6, s_65_4)
        let s_65_7: () = AArch64_SystemAccessTrap(state, tracer, s_65_6, s_65_4);
        // N s_65_8: return
        return;
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_66_0: panic
        panic!("{:?}", ());
        // N s_66_1: return
        return;
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var __EDSCR_SDD:u8
        let s_67_0: bool = fn_state.u__EDSCR_SDD;
        // D s_67_1: cast zx s_67_0 -> bv
        let s_67_1: Bits = Bits::new(s_67_0 as u128, 1u16);
        // C s_67_2: const #1u : u8
        let s_67_2: bool = true;
        // C s_67_3: cast zx s_67_2 -> bv
        let s_67_3: Bits = Bits::new(s_67_2 as u128, 1u16);
        // D s_67_4: cmp-eq s_67_1 s_67_3
        let s_67_4: bool = ((s_67_1) == (s_67_3));
        // D s_67_5: write-var gs#137473 <= s_67_4
        fn_state.gs_137473 = s_67_4;
        // N s_67_6: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var __SCR_EL3_D128En:u8
        let s_68_0: bool = fn_state.u__SCR_EL3_D128En;
        // D s_68_1: cast zx s_68_0 -> bv
        let s_68_1: Bits = Bits::new(s_68_0 as u128, 1u16);
        // C s_68_2: const #0u : u8
        let s_68_2: bool = false;
        // C s_68_3: cast zx s_68_2 -> bv
        let s_68_3: Bits = Bits::new(s_68_2 as u128, 1u16);
        // D s_68_4: cmp-eq s_68_1 s_68_3
        let s_68_4: bool = ((s_68_1) == (s_68_3));
        // D s_68_5: write-var gs#137456 <= s_68_4
        fn_state.gs_137456 = s_68_4;
        // N s_68_6: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #20u : u8
        let s_69_0: u8 = 20;
        // C s_69_1: cast zx s_69_0 -> bv
        let s_69_1: Bits = Bits::new(s_69_0 as u128, 8u16);
        // C s_69_2: cast zx s_69_1 -> i
        let s_69_2: i128 = (s_69_1.value() as i128);
        // C s_69_3: cast reint s_69_2 -> i64
        let s_69_3: i64 = (s_69_2 as i64);
        // C s_69_4: cast zx s_69_3 -> i
        let s_69_4: i128 = (i128::try_from(s_69_3).unwrap());
        // C s_69_5: const #432u : u32
        let s_69_5: u32 = 432;
        // D s_69_6: read-reg s_69_5:u8
        let s_69_6: u8 = {
            let value = state.read_register::<u8>(s_69_5 as isize);
            tracer.read_register(s_69_5 as isize, value);
            value
        };
        // D s_69_7: call AArch64_SystemAccessTrap(s_69_6, s_69_4)
        let s_69_7: () = AArch64_SystemAccessTrap(state, tracer, s_69_6, s_69_4);
        // N s_69_8: return
        return;
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var __HFGRTR_EL2_TTBR1_EL1:u8
        let s_70_0: bool = fn_state.u__HFGRTR_EL2_TTBR1_EL1;
        // D s_70_1: cast zx s_70_0 -> bv
        let s_70_1: Bits = Bits::new(s_70_0 as u128, 1u16);
        // C s_70_2: const #1u : u8
        let s_70_2: bool = true;
        // C s_70_3: cast zx s_70_2 -> bv
        let s_70_3: Bits = Bits::new(s_70_2 as u128, 1u16);
        // D s_70_4: cmp-eq s_70_1 s_70_3
        let s_70_4: bool = ((s_70_1) == (s_70_3));
        // D s_70_5: write-var gs#137455 <= s_70_4
        fn_state.gs_137455 = s_70_4;
        // N s_70_6: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #424u : u32
        let s_71_0: u32 = 424;
        // D s_71_1: read-reg s_71_0:u8
        let s_71_1: u8 = {
            let value = state.read_register::<u8>(s_71_0 as isize);
            tracer.read_register(s_71_0 as isize, value);
            value
        };
        // C s_71_2: const #2u : u8
        let s_71_2: u8 = 2;
        // D s_71_3: cmp-lt s_71_1 s_71_2
        let s_71_3: bool = ((s_71_1) < (s_71_2));
        // D s_71_4: not s_71_3
        let s_71_4: bool = !s_71_3;
        // N s_71_5: branch s_71_4 b74 b72
        if s_71_4 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_72(state, tracer, fn_state);
        };
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var __SCR_EL3_FGTEn:u8
        let s_72_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_72_1: cast zx s_72_0 -> bv
        let s_72_1: Bits = Bits::new(s_72_0 as u128, 1u16);
        // C s_72_2: const #1u : u8
        let s_72_2: bool = true;
        // C s_72_3: cast zx s_72_2 -> bv
        let s_72_3: Bits = Bits::new(s_72_2 as u128, 1u16);
        // D s_72_4: cmp-eq s_72_1 s_72_3
        let s_72_4: bool = ((s_72_1) == (s_72_3));
        // D s_72_5: write-var gs#137453 <= s_72_4
        fn_state.gs_137453 = s_72_4;
        // N s_72_6: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var gs#137453:u8
        let s_73_0: bool = fn_state.gs_137453;
        // D s_73_1: write-var gs#137454 <= s_73_0
        fn_state.gs_137454 = s_73_0;
        // N s_73_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #1u : u8
        let s_74_0: bool = true;
        // D s_74_1: write-var gs#137453 <= s_74_0
        fn_state.gs_137453 = s_74_0;
        // N s_74_2: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #146u : u32
        let s_75_0: u32 = 146;
        // S s_75_1: call IsFeatureImplemented(s_75_0)
        let s_75_1: bool = IsFeatureImplemented(state, tracer, s_75_0);
        // D s_75_2: write-var gs#137452 <= s_75_1
        fn_state.gs_137452 = s_75_1;
        // N s_75_3: jump b47
        return block_47(state, tracer, fn_state);
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
        // D s_77_0: read-var __HCR_EL2_TRVM:u8
        let s_77_0: bool = fn_state.u__HCR_EL2_TRVM;
        // D s_77_1: cast zx s_77_0 -> bv
        let s_77_1: Bits = Bits::new(s_77_0 as u128, 1u16);
        // C s_77_2: const #1u : u8
        let s_77_2: bool = true;
        // C s_77_3: cast zx s_77_2 -> bv
        let s_77_3: Bits = Bits::new(s_77_2 as u128, 1u16);
        // D s_77_4: cmp-eq s_77_1 s_77_3
        let s_77_4: bool = ((s_77_1) == (s_77_3));
        // D s_77_5: write-var gs#137451 <= s_77_4
        fn_state.gs_137451 = s_77_4;
        // N s_77_6: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_78_0: panic
        panic!("{:?}", ());
        // N s_78_1: return
        return;
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var __SCR_EL3_D128En:u8
        let s_79_0: bool = fn_state.u__SCR_EL3_D128En;
        // D s_79_1: cast zx s_79_0 -> bv
        let s_79_1: Bits = Bits::new(s_79_0 as u128, 1u16);
        // C s_79_2: const #0u : u8
        let s_79_2: bool = false;
        // C s_79_3: cast zx s_79_2 -> bv
        let s_79_3: Bits = Bits::new(s_79_2 as u128, 1u16);
        // D s_79_4: cmp-eq s_79_1 s_79_3
        let s_79_4: bool = ((s_79_1) == (s_79_3));
        // D s_79_5: write-var gs#137450 <= s_79_4
        fn_state.gs_137450 = s_79_4;
        // N s_79_6: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_80_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_80_1: call __IMPDEF_boolean(s_80_0)
        let s_80_1: bool = u__IMPDEF_boolean(state, tracer, s_80_0);
        // D s_80_2: write-var gs#137449 <= s_80_1
        fn_state.gs_137449 = s_80_1;
        // N s_80_3: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var __EDSCR_SDD:u8
        let s_81_0: bool = fn_state.u__EDSCR_SDD;
        // D s_81_1: cast zx s_81_0 -> bv
        let s_81_1: Bits = Bits::new(s_81_0 as u128, 1u16);
        // C s_81_2: const #1u : u8
        let s_81_2: bool = true;
        // C s_81_3: cast zx s_81_2 -> bv
        let s_81_3: Bits = Bits::new(s_81_2 as u128, 1u16);
        // D s_81_4: cmp-eq s_81_1 s_81_3
        let s_81_4: bool = ((s_81_1) == (s_81_3));
        // D s_81_5: write-var gs#137448 <= s_81_4
        fn_state.gs_137448 = s_81_4;
        // N s_81_6: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #424u : u32
        let s_82_0: u32 = 424;
        // D s_82_1: read-reg s_82_0:u8
        let s_82_1: u8 = {
            let value = state.read_register::<u8>(s_82_0 as isize);
            tracer.read_register(s_82_0 as isize, value);
            value
        };
        // C s_82_2: const #2u : u8
        let s_82_2: u8 = 2;
        // D s_82_3: cmp-lt s_82_1 s_82_2
        let s_82_3: bool = ((s_82_1) < (s_82_2));
        // D s_82_4: write-var gs#137447 <= s_82_3
        fn_state.gs_137447 = s_82_3;
        // N s_82_5: jump b35
        return block_35(state, tracer, fn_state);
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
}
