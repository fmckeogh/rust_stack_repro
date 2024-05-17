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
use u_get_HFGITR_EL2_Type_nGCSEPP::*;
use EXLOCKException::*;
use Halted::*;
use GetCurrentEXLOCKEN::*;
use IsFeatureImplemented::*;
use AArch64_SystemAccessTrap::*;
use EL2Enabled::*;
use GCSPUSHX::*;
use common::*;
pub fn GCSPUSHX_SysOpsWrite_c023a281d843c45a<T: Tracer>(
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
        gs_101915: bool,
        gs_101924: bool,
        gs_101917: bool,
        gs_101918: bool,
        gs_101919: bool,
        gs_101920: bool,
        gs_101922: bool,
        gs_101926: bool,
        gs_101925: bool,
        u__SCR_EL3_FGTEn: bool,
        gs_101921: bool,
        gs_101927: bool,
        u__PSTATE_EXLOCK: bool,
        u__HFGITR_EL2_nGCSEPP: bool,
        u__PSTATE_EL: u8,
        gs_101916: bool,
        gs_101923: bool,
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
        // C s_0_3: const #16976u : u32
        let s_0_3: u32 = 16976;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: bool = {
            let value = state.read_register::<bool>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: write-var __PSTATE_EXLOCK <= s_0_4
        fn_state.u__PSTATE_EXLOCK = s_0_4;
        // C s_0_6: const #90704u : u32
        let s_0_6: u32 = 90704;
        // D s_0_7: read-reg s_0_6:struct
        let s_0_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_6 as isize);
            tracer.read_register(s_0_6 as isize, value);
            value
        };
        // D s_0_8: call _get_SCR_EL3_Type_FGTEn(s_0_7)
        let s_0_8: bool = u_get_SCR_EL3_Type_FGTEn(state, tracer, s_0_7);
        // D s_0_9: write-var __SCR_EL3_FGTEn <= s_0_8
        fn_state.u__SCR_EL3_FGTEn = s_0_8;
        // C s_0_10: const #13608u : u32
        let s_0_10: u32 = 13608;
        // D s_0_11: read-reg s_0_10:struct
        let s_0_11: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_10 as isize);
            tracer.read_register(s_0_10 as isize, value);
            value
        };
        // D s_0_12: call _get_HFGITR_EL2_Type_nGCSEPP(s_0_11)
        let s_0_12: bool = u_get_HFGITR_EL2_Type_nGCSEPP(state, tracer, s_0_11);
        // D s_0_13: write-var __HFGITR_EL2_nGCSEPP <= s_0_12
        fn_state.u__HFGITR_EL2_nGCSEPP = s_0_12;
        // D s_0_14: read-var __PSTATE_EL:u8
        let s_0_14: u8 = fn_state.u__PSTATE_EL;
        // D s_0_15: cast zx s_0_14 -> bv
        let s_0_15: Bits = Bits::new(s_0_14 as u128, 2u16);
        // C s_0_16: const #448u : u32
        let s_0_16: u32 = 448;
        // D s_0_17: read-reg s_0_16:u8
        let s_0_17: u8 = {
            let value = state.read_register::<u8>(s_0_16 as isize);
            tracer.read_register(s_0_16 as isize, value);
            value
        };
        // D s_0_18: cast zx s_0_17 -> bv
        let s_0_18: Bits = Bits::new(s_0_17 as u128, 2u16);
        // D s_0_19: cmp-eq s_0_15 s_0_18
        let s_0_19: bool = ((s_0_15) == (s_0_18));
        // N s_0_20: branch s_0_19 b55 b1
        if s_0_19 {
            return block_55(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b29 b2
        if s_1_5 {
            return block_29(state, tracer, fn_state);
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
        // N s_2_6: branch s_2_5 b17 b3
        if s_2_5 {
            return block_17(state, tracer, fn_state);
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
        // C s_5_0: const #245u : u32
        let s_5_0: u32 = 245;
        // S s_5_1: call IsFeatureImplemented(s_5_0)
        let s_5_1: bool = IsFeatureImplemented(state, tracer, s_5_0);
        // N s_5_2: branch s_5_1 b16 b6
        if s_5_1 {
            return block_16(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#101915 <= s_6_0
        fn_state.gs_101915 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#101915:u8
        let s_7_0: bool = fn_state.gs_101915;
        // N s_7_1: branch s_7_0 b15 b8
        if s_7_0 {
            return block_15(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#101916 <= s_8_0
        fn_state.gs_101916 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#101916:u8
        let s_9_0: bool = fn_state.gs_101916;
        // N s_9_1: branch s_9_0 b14 b10
        if s_9_0 {
            return block_14(state, tracer, fn_state);
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
        // D s_10_1: write-var gs#101917 <= s_10_0
        fn_state.gs_101917 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#101917:u8
        let s_11_0: bool = fn_state.gs_101917;
        // N s_11_1: branch s_11_0 b13 b12
        if s_11_0 {
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
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call GCSPUSHX(s_12_0)
        let s_12_1: () = GCSPUSHX(state, tracer, s_12_0);
        // N s_12_2: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call EXLOCKException(s_13_0)
        let s_13_1: () = EXLOCKException(state, tracer, s_13_0);
        // N s_13_2: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var __PSTATE_EXLOCK:u8
        let s_14_0: bool = fn_state.u__PSTATE_EXLOCK;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 1u16);
        // C s_14_2: const #0u : u8
        let s_14_2: bool = false;
        // C s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 1u16);
        // D s_14_4: cmp-eq s_14_1 s_14_3
        let s_14_4: bool = ((s_14_1) == (s_14_3));
        // D s_14_5: write-var gs#101917 <= s_14_4
        fn_state.gs_101917 = s_14_4;
        // N s_14_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call Halted(s_15_0)
        let s_15_1: bool = Halted(state, tracer, s_15_0);
        // S s_15_2: not s_15_1
        let s_15_2: bool = !s_15_1;
        // D s_15_3: write-var gs#101916 <= s_15_2
        fn_state.gs_101916 = s_15_2;
        // N s_15_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call GetCurrentEXLOCKEN(s_16_0)
        let s_16_1: bool = GetCurrentEXLOCKEN(state, tracer, s_16_0);
        // D s_16_2: write-var gs#101915 <= s_16_1
        fn_state.gs_101915 = s_16_1;
        // N s_16_3: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #245u : u32
        let s_17_0: u32 = 245;
        // S s_17_1: call IsFeatureImplemented(s_17_0)
        let s_17_1: bool = IsFeatureImplemented(state, tracer, s_17_0);
        // N s_17_2: branch s_17_1 b28 b18
        if s_17_1 {
            return block_28(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#101918 <= s_18_0
        fn_state.gs_101918 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#101918:u8
        let s_19_0: bool = fn_state.gs_101918;
        // N s_19_1: branch s_19_0 b27 b20
        if s_19_0 {
            return block_27(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#101919 <= s_20_0
        fn_state.gs_101919 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#101919:u8
        let s_21_0: bool = fn_state.gs_101919;
        // N s_21_1: branch s_21_0 b26 b22
        if s_21_0 {
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
        // D s_22_1: write-var gs#101920 <= s_22_0
        fn_state.gs_101920 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#101920:u8
        let s_23_0: bool = fn_state.gs_101920;
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
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call GCSPUSHX(s_24_0)
        let s_24_1: () = GCSPUSHX(state, tracer, s_24_0);
        // N s_24_2: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call EXLOCKException(s_25_0)
        let s_25_1: () = EXLOCKException(state, tracer, s_25_0);
        // N s_25_2: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var __PSTATE_EXLOCK:u8
        let s_26_0: bool = fn_state.u__PSTATE_EXLOCK;
        // D s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 1u16);
        // C s_26_2: const #0u : u8
        let s_26_2: bool = false;
        // C s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 1u16);
        // D s_26_4: cmp-eq s_26_1 s_26_3
        let s_26_4: bool = ((s_26_1) == (s_26_3));
        // D s_26_5: write-var gs#101920 <= s_26_4
        fn_state.gs_101920 = s_26_4;
        // N s_26_6: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #() : ()
        let s_27_0: () = ();
        // S s_27_1: call Halted(s_27_0)
        let s_27_1: bool = Halted(state, tracer, s_27_0);
        // S s_27_2: not s_27_1
        let s_27_2: bool = !s_27_1;
        // D s_27_3: write-var gs#101919 <= s_27_2
        fn_state.gs_101919 = s_27_2;
        // N s_27_4: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call GetCurrentEXLOCKEN(s_28_0)
        let s_28_1: bool = GetCurrentEXLOCKEN(state, tracer, s_28_0);
        // D s_28_2: write-var gs#101918 <= s_28_1
        fn_state.gs_101918 = s_28_1;
        // N s_28_3: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #245u : u32
        let s_29_0: u32 = 245;
        // S s_29_1: call IsFeatureImplemented(s_29_0)
        let s_29_1: bool = IsFeatureImplemented(state, tracer, s_29_0);
        // N s_29_2: branch s_29_1 b54 b30
        if s_29_1 {
            return block_54(state, tracer, fn_state);
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
        // D s_30_1: write-var gs#101921 <= s_30_0
        fn_state.gs_101921 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#101921:u8
        let s_31_0: bool = fn_state.gs_101921;
        // N s_31_1: branch s_31_0 b53 b32
        if s_31_0 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #0u : u8
        let s_32_0: bool = false;
        // D s_32_1: write-var gs#101922 <= s_32_0
        fn_state.gs_101922 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#101922:u8
        let s_33_0: bool = fn_state.gs_101922;
        // N s_33_1: branch s_33_0 b52 b34
        if s_33_0 {
            return block_52(state, tracer, fn_state);
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
        // D s_34_1: write-var gs#101923 <= s_34_0
        fn_state.gs_101923 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#101923:u8
        let s_35_0: bool = fn_state.gs_101923;
        // N s_35_1: branch s_35_0 b51 b36
        if s_35_0 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #() : ()
        let s_36_0: () = ();
        // S s_36_1: call EL2Enabled(s_36_0)
        let s_36_1: bool = EL2Enabled(state, tracer, s_36_0);
        // N s_36_2: branch s_36_1 b50 b37
        if s_36_1 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var gs#101924 <= s_37_0
        fn_state.gs_101924 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#101924:u8
        let s_38_0: bool = fn_state.gs_101924;
        // N s_38_1: branch s_38_0 b46 b39
        if s_38_0 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #0u : u8
        let s_39_0: bool = false;
        // D s_39_1: write-var gs#101926 <= s_39_0
        fn_state.gs_101926 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#101926:u8
        let s_40_0: bool = fn_state.gs_101926;
        // N s_40_1: branch s_40_0 b45 b41
        if s_40_0 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #0u : u8
        let s_41_0: bool = false;
        // D s_41_1: write-var gs#101927 <= s_41_0
        fn_state.gs_101927 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#101927:u8
        let s_42_0: bool = fn_state.gs_101927;
        // N s_42_1: branch s_42_0 b44 b43
        if s_42_0 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #() : ()
        let s_43_0: () = ();
        // S s_43_1: call GCSPUSHX(s_43_0)
        let s_43_1: () = GCSPUSHX(state, tracer, s_43_0);
        // N s_43_2: return
        return;
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #24u : u8
        let s_44_0: u8 = 24;
        // C s_44_1: cast zx s_44_0 -> bv
        let s_44_1: Bits = Bits::new(s_44_0 as u128, 8u16);
        // C s_44_2: cast zx s_44_1 -> i
        let s_44_2: i128 = (s_44_1.value() as i128);
        // C s_44_3: cast reint s_44_2 -> i64
        let s_44_3: i64 = (s_44_2 as i64);
        // C s_44_4: cast zx s_44_3 -> i
        let s_44_4: i128 = (i128::try_from(s_44_3).unwrap());
        // C s_44_5: const #432u : u32
        let s_44_5: u32 = 432;
        // D s_44_6: read-reg s_44_5:u8
        let s_44_6: u8 = {
            let value = state.read_register::<u8>(s_44_5 as isize);
            tracer.read_register(s_44_5 as isize, value);
            value
        };
        // D s_44_7: call AArch64_SystemAccessTrap(s_44_6, s_44_4)
        let s_44_7: () = AArch64_SystemAccessTrap(state, tracer, s_44_6, s_44_4);
        // N s_44_8: return
        return;
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var __HFGITR_EL2_nGCSEPP:u8
        let s_45_0: bool = fn_state.u__HFGITR_EL2_nGCSEPP;
        // D s_45_1: cast zx s_45_0 -> bv
        let s_45_1: Bits = Bits::new(s_45_0 as u128, 1u16);
        // C s_45_2: const #0u : u8
        let s_45_2: bool = false;
        // C s_45_3: cast zx s_45_2 -> bv
        let s_45_3: Bits = Bits::new(s_45_2 as u128, 1u16);
        // D s_45_4: cmp-eq s_45_1 s_45_3
        let s_45_4: bool = ((s_45_1) == (s_45_3));
        // D s_45_5: write-var gs#101927 <= s_45_4
        fn_state.gs_101927 = s_45_4;
        // N s_45_6: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #424u : u32
        let s_46_0: u32 = 424;
        // D s_46_1: read-reg s_46_0:u8
        let s_46_1: u8 = {
            let value = state.read_register::<u8>(s_46_0 as isize);
            tracer.read_register(s_46_0 as isize, value);
            value
        };
        // C s_46_2: const #2u : u8
        let s_46_2: u8 = 2;
        // D s_46_3: cmp-lt s_46_1 s_46_2
        let s_46_3: bool = ((s_46_1) < (s_46_2));
        // D s_46_4: not s_46_3
        let s_46_4: bool = !s_46_3;
        // N s_46_5: branch s_46_4 b49 b47
        if s_46_4 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var __SCR_EL3_FGTEn:u8
        let s_47_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_47_1: cast zx s_47_0 -> bv
        let s_47_1: Bits = Bits::new(s_47_0 as u128, 1u16);
        // C s_47_2: const #1u : u8
        let s_47_2: bool = true;
        // C s_47_3: cast zx s_47_2 -> bv
        let s_47_3: Bits = Bits::new(s_47_2 as u128, 1u16);
        // D s_47_4: cmp-eq s_47_1 s_47_3
        let s_47_4: bool = ((s_47_1) == (s_47_3));
        // D s_47_5: write-var gs#101925 <= s_47_4
        fn_state.gs_101925 = s_47_4;
        // N s_47_6: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#101925:u8
        let s_48_0: bool = fn_state.gs_101925;
        // D s_48_1: write-var gs#101926 <= s_48_0
        fn_state.gs_101926 = s_48_0;
        // N s_48_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #1u : u8
        let s_49_0: bool = true;
        // D s_49_1: write-var gs#101925 <= s_49_0
        fn_state.gs_101925 = s_49_0;
        // N s_49_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #146u : u32
        let s_50_0: u32 = 146;
        // S s_50_1: call IsFeatureImplemented(s_50_0)
        let s_50_1: bool = IsFeatureImplemented(state, tracer, s_50_0);
        // D s_50_2: write-var gs#101924 <= s_50_1
        fn_state.gs_101924 = s_50_1;
        // N s_50_3: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #() : ()
        let s_51_0: () = ();
        // S s_51_1: call EXLOCKException(s_51_0)
        let s_51_1: () = EXLOCKException(state, tracer, s_51_0);
        // N s_51_2: return
        return;
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var __PSTATE_EXLOCK:u8
        let s_52_0: bool = fn_state.u__PSTATE_EXLOCK;
        // D s_52_1: cast zx s_52_0 -> bv
        let s_52_1: Bits = Bits::new(s_52_0 as u128, 1u16);
        // C s_52_2: const #0u : u8
        let s_52_2: bool = false;
        // C s_52_3: cast zx s_52_2 -> bv
        let s_52_3: Bits = Bits::new(s_52_2 as u128, 1u16);
        // D s_52_4: cmp-eq s_52_1 s_52_3
        let s_52_4: bool = ((s_52_1) == (s_52_3));
        // D s_52_5: write-var gs#101923 <= s_52_4
        fn_state.gs_101923 = s_52_4;
        // N s_52_6: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #() : ()
        let s_53_0: () = ();
        // S s_53_1: call Halted(s_53_0)
        let s_53_1: bool = Halted(state, tracer, s_53_0);
        // S s_53_2: not s_53_1
        let s_53_2: bool = !s_53_1;
        // D s_53_3: write-var gs#101922 <= s_53_2
        fn_state.gs_101922 = s_53_2;
        // N s_53_4: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #() : ()
        let s_54_0: () = ();
        // S s_54_1: call GetCurrentEXLOCKEN(s_54_0)
        let s_54_1: bool = GetCurrentEXLOCKEN(state, tracer, s_54_0);
        // D s_54_2: write-var gs#101921 <= s_54_1
        fn_state.gs_101921 = s_54_1;
        // N s_54_3: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_55_0: panic
        panic!("{:?}", ());
        // N s_55_1: return
        return;
    }
}
