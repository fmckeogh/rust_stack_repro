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
use u_get_ICC_SRE_EL2_Type_Enable::*;
use Halted::*;
use u_get_ICC_SRE_EL3_Type_Enable::*;
use u__IMPDEF_boolean::*;
use u__get_ICC_SRE_EL1::*;
use AArch64_SystemAccessTrap::*;
use u_get_SCR_EL3_Type_NS::*;
use ICC_SRE_EL1_read::*;
use X_set::*;
use u__get_ICC_SRE_EL1_S::*;
use u__get_ICC_SRE_EL1_NS::*;
use u_get_EDSCR_Type_SDD::*;
use EL2Enabled::*;
use EDSCR_read::*;
use common::*;
pub fn ICC_SRE_EL1_SysRegRead_66e16dcbec736265<T: Tracer>(
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
        u__SCR_EL3_NS: bool,
        u__EDSCR_SDD: bool,
        gs_62334: bool,
        ga_67851: ProductType5c790c8ef59cc8b2,
        gs_62331: bool,
        ga_67888: ProductType5c790c8ef59cc8b2,
        ga_67848: ProductType5c790c8ef59cc8b2,
        gs_62333: bool,
        gs_62342: bool,
        gs_62332: bool,
        gs_62339: bool,
        ga_67876: ProductType5c790c8ef59cc8b2,
        ga_67855: ProductType5c790c8ef59cc8b2,
        gs_62338: bool,
        gs_62330: bool,
        ga_67885: ProductType5c790c8ef59cc8b2,
        gs_62340: bool,
        u__ICC_SRE_EL2_Enable: bool,
        gs_62341: bool,
        u__PSTATE_EL: u8,
        u__ICC_SRE_EL3_Enable: bool,
        gs_62348: bool,
        ga_67880: ProductType5c790c8ef59cc8b2,
        ga_67873: ProductType5c790c8ef59cc8b2,
        gs_62344: bool,
        gs_62343: bool,
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
        // C s_0_7: const #10200u : u32
        let s_0_7: u32 = 10200;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_ICC_SRE_EL3_Type_Enable(s_0_8)
        let s_0_9: bool = u_get_ICC_SRE_EL3_Type_Enable(state, tracer, s_0_8);
        // D s_0_10: write-var __ICC_SRE_EL3_Enable <= s_0_9
        fn_state.u__ICC_SRE_EL3_Enable = s_0_9;
        // C s_0_11: const #16368u : u32
        let s_0_11: u32 = 16368;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_ICC_SRE_EL2_Type_Enable(s_0_12)
        let s_0_13: bool = u_get_ICC_SRE_EL2_Type_Enable(state, tracer, s_0_12);
        // D s_0_14: write-var __ICC_SRE_EL2_Enable <= s_0_13
        fn_state.u__ICC_SRE_EL2_Enable = s_0_13;
        // C s_0_15: const #90704u : u32
        let s_0_15: u32 = 90704;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_SCR_EL3_Type_NS(s_0_16)
        let s_0_17: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_0_16);
        // D s_0_18: write-var __SCR_EL3_NS <= s_0_17
        fn_state.u__SCR_EL3_NS = s_0_17;
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
        // N s_0_25: branch s_0_24 b71 b1
        if s_0_24 {
            return block_71(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b37 b2
        if s_1_5 {
            return block_37(state, tracer, fn_state);
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
        // D s_5_0: read-var __SCR_EL3_NS:u8
        let s_5_0: bool = fn_state.u__SCR_EL3_NS;
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
        // C s_6_1: const #22960u : u32
        let s_6_1: u32 = 22960;
        // D s_6_2: read-reg s_6_1:struct
        let s_6_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_6_1 as isize);
            tracer.read_register(s_6_1 as isize, value);
            value
        };
        // D s_6_3: call __get_ICC_SRE_EL1_NS(s_6_2)
        let s_6_3: ProductType5c790c8ef59cc8b2 = u__get_ICC_SRE_EL1_NS(
            state,
            tracer,
            s_6_2,
        );
        // D s_6_4: write-var ga#67888 <= s_6_3
        fn_state.ga_67888 = s_6_3;
        // D s_6_5: read-var ga#67888.0:struct
        let s_6_5: u64 = fn_state.ga_67888._0;
        // D s_6_6: cast zx s_6_5 -> bv
        let s_6_6: Bits = Bits::new(s_6_5 as u128, 64u16);
        // D s_6_7: read-var t:i
        let s_6_7: i128 = fn_state.t;
        // D s_6_8: call X_set(s_6_7, s_6_0, s_6_6)
        let s_6_8: () = X_set(state, tracer, s_6_7, s_6_0, s_6_6);
        // N s_6_9: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #64s : i64
        let s_7_0: i64 = 64;
        // C s_7_1: const #11904u : u32
        let s_7_1: u32 = 11904;
        // D s_7_2: read-reg s_7_1:struct
        let s_7_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_7_1 as isize);
            tracer.read_register(s_7_1 as isize, value);
            value
        };
        // D s_7_3: call __get_ICC_SRE_EL1_S(s_7_2)
        let s_7_3: ProductType5c790c8ef59cc8b2 = u__get_ICC_SRE_EL1_S(
            state,
            tracer,
            s_7_2,
        );
        // D s_7_4: write-var ga#67885 <= s_7_3
        fn_state.ga_67885 = s_7_3;
        // D s_7_5: read-var ga#67885.0:struct
        let s_7_5: u64 = fn_state.ga_67885._0;
        // D s_7_6: cast zx s_7_5 -> bv
        let s_7_6: Bits = Bits::new(s_7_5 as u128, 64u16);
        // D s_7_7: read-var t:i
        let s_7_7: i128 = fn_state.t;
        // D s_7_8: call X_set(s_7_7, s_7_0, s_7_6)
        let s_7_8: () = X_set(state, tracer, s_7_7, s_7_0, s_7_6);
        // N s_7_9: return
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
        // N s_8_2: branch s_8_1 b36 b9
        if s_8_1 {
            return block_36(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#62330 <= s_9_0
        fn_state.gs_62330 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#62330:u8
        let s_10_0: bool = fn_state.gs_62330;
        // N s_10_1: branch s_10_0 b35 b11
        if s_10_0 {
            return block_35(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#62331 <= s_11_0
        fn_state.gs_62331 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#62331:u8
        let s_12_0: bool = fn_state.gs_62331;
        // N s_12_1: branch s_12_0 b34 b13
        if s_12_0 {
            return block_34(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#62332 <= s_13_0
        fn_state.gs_62332 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#62332:u8
        let s_14_0: bool = fn_state.gs_62332;
        // N s_14_1: branch s_14_0 b33 b15
        if s_14_0 {
            return block_33(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#62333 <= s_15_0
        fn_state.gs_62333 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#62333:u8
        let s_16_0: bool = fn_state.gs_62333;
        // N s_16_1: branch s_16_0 b32 b17
        if s_16_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #424u : u32
        let s_17_0: u32 = 424;
        // D s_17_1: read-reg s_17_0:u8
        let s_17_1: u8 = {
            let value = state.read_register::<u8>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // C s_17_2: const #2u : u8
        let s_17_2: u8 = 2;
        // D s_17_3: cmp-lt s_17_1 s_17_2
        let s_17_3: bool = ((s_17_1) < (s_17_2));
        // N s_17_4: branch s_17_3 b31 b18
        if s_17_3 {
            return block_31(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#62334 <= s_18_0
        fn_state.gs_62334 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#62334:u8
        let s_19_0: bool = fn_state.gs_62334;
        // N s_19_1: branch s_19_0 b25 b20
        if s_19_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #424u : u32
        let s_20_0: u32 = 424;
        // D s_20_1: read-reg s_20_0:u8
        let s_20_1: u8 = {
            let value = state.read_register::<u8>(s_20_0 as isize);
            tracer.read_register(s_20_0 as isize, value);
            value
        };
        // C s_20_2: const #2u : u8
        let s_20_2: u8 = 2;
        // D s_20_3: cmp-lt s_20_1 s_20_2
        let s_20_3: bool = ((s_20_1) < (s_20_2));
        // N s_20_4: branch s_20_3 b22 b21
        if s_20_3 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #64s : i64
        let s_21_0: i64 = 64;
        // C s_21_1: const #() : ()
        let s_21_1: () = ();
        // S s_21_2: call ICC_SRE_EL1_read(s_21_1)
        let s_21_2: ProductType5c790c8ef59cc8b2 = ICC_SRE_EL1_read(
            state,
            tracer,
            s_21_1,
        );
        // S s_21_3: call __get_ICC_SRE_EL1(s_21_2)
        let s_21_3: ProductType5c790c8ef59cc8b2 = u__get_ICC_SRE_EL1(
            state,
            tracer,
            s_21_2,
        );
        // D s_21_4: write-var ga#67880 <= s_21_3
        fn_state.ga_67880 = s_21_3;
        // D s_21_5: read-var ga#67880.0:struct
        let s_21_5: u64 = fn_state.ga_67880._0;
        // D s_21_6: cast zx s_21_5 -> bv
        let s_21_6: Bits = Bits::new(s_21_5 as u128, 64u16);
        // D s_21_7: read-var t:i
        let s_21_7: i128 = fn_state.t;
        // D s_21_8: call X_set(s_21_7, s_21_0, s_21_6)
        let s_21_8: () = X_set(state, tracer, s_21_7, s_21_0, s_21_6);
        // N s_21_9: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var __SCR_EL3_NS:u8
        let s_22_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 1u16);
        // C s_22_2: const #0u : u8
        let s_22_2: bool = false;
        // C s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 1u16);
        // D s_22_4: cmp-eq s_22_1 s_22_3
        let s_22_4: bool = ((s_22_1) == (s_22_3));
        // N s_22_5: branch s_22_4 b24 b23
        if s_22_4 {
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
        // C s_23_0: const #64s : i64
        let s_23_0: i64 = 64;
        // C s_23_1: const #22960u : u32
        let s_23_1: u32 = 22960;
        // D s_23_2: read-reg s_23_1:struct
        let s_23_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_23_1 as isize);
            tracer.read_register(s_23_1 as isize, value);
            value
        };
        // D s_23_3: call __get_ICC_SRE_EL1_NS(s_23_2)
        let s_23_3: ProductType5c790c8ef59cc8b2 = u__get_ICC_SRE_EL1_NS(
            state,
            tracer,
            s_23_2,
        );
        // D s_23_4: write-var ga#67876 <= s_23_3
        fn_state.ga_67876 = s_23_3;
        // D s_23_5: read-var ga#67876.0:struct
        let s_23_5: u64 = fn_state.ga_67876._0;
        // D s_23_6: cast zx s_23_5 -> bv
        let s_23_6: Bits = Bits::new(s_23_5 as u128, 64u16);
        // D s_23_7: read-var t:i
        let s_23_7: i128 = fn_state.t;
        // D s_23_8: call X_set(s_23_7, s_23_0, s_23_6)
        let s_23_8: () = X_set(state, tracer, s_23_7, s_23_0, s_23_6);
        // N s_23_9: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #64s : i64
        let s_24_0: i64 = 64;
        // C s_24_1: const #11904u : u32
        let s_24_1: u32 = 11904;
        // D s_24_2: read-reg s_24_1:struct
        let s_24_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_24_1 as isize);
            tracer.read_register(s_24_1 as isize, value);
            value
        };
        // D s_24_3: call __get_ICC_SRE_EL1_S(s_24_2)
        let s_24_3: ProductType5c790c8ef59cc8b2 = u__get_ICC_SRE_EL1_S(
            state,
            tracer,
            s_24_2,
        );
        // D s_24_4: write-var ga#67873 <= s_24_3
        fn_state.ga_67873 = s_24_3;
        // D s_24_5: read-var ga#67873.0:struct
        let s_24_5: u64 = fn_state.ga_67873._0;
        // D s_24_6: cast zx s_24_5 -> bv
        let s_24_6: Bits = Bits::new(s_24_5 as u128, 64u16);
        // D s_24_7: read-var t:i
        let s_24_7: i128 = fn_state.t;
        // D s_24_8: call X_set(s_24_7, s_24_0, s_24_6)
        let s_24_8: () = X_set(state, tracer, s_24_7, s_24_0, s_24_6);
        // N s_24_9: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call Halted(s_25_0)
        let s_25_1: bool = Halted(state, tracer, s_25_0);
        // N s_25_2: branch s_25_1 b30 b26
        if s_25_1 {
            return block_30(state, tracer, fn_state);
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
        // D s_26_1: write-var gs#62338 <= s_26_0
        fn_state.gs_62338 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#62338:u8
        let s_27_0: bool = fn_state.gs_62338;
        // N s_27_1: branch s_27_0 b29 b28
        if s_27_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #24u : u8
        let s_28_0: u8 = 24;
        // C s_28_1: cast zx s_28_0 -> bv
        let s_28_1: Bits = Bits::new(s_28_0 as u128, 8u16);
        // C s_28_2: cast zx s_28_1 -> i
        let s_28_2: i128 = (s_28_1.value() as i128);
        // C s_28_3: cast reint s_28_2 -> i64
        let s_28_3: i64 = (s_28_2 as i64);
        // C s_28_4: cast zx s_28_3 -> i
        let s_28_4: i128 = (i128::try_from(s_28_3).unwrap());
        // C s_28_5: const #424u : u32
        let s_28_5: u32 = 424;
        // D s_28_6: read-reg s_28_5:u8
        let s_28_6: u8 = {
            let value = state.read_register::<u8>(s_28_5 as isize);
            tracer.read_register(s_28_5 as isize, value);
            value
        };
        // D s_28_7: call AArch64_SystemAccessTrap(s_28_6, s_28_4)
        let s_28_7: () = AArch64_SystemAccessTrap(state, tracer, s_28_6, s_28_4);
        // N s_28_8: return
        return;
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_29_0: panic
        panic!("{:?}", ());
        // N s_29_1: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var __EDSCR_SDD:u8
        let s_30_0: bool = fn_state.u__EDSCR_SDD;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 1u16);
        // C s_30_2: const #1u : u8
        let s_30_2: bool = true;
        // C s_30_3: cast zx s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 1u16);
        // D s_30_4: cmp-eq s_30_1 s_30_3
        let s_30_4: bool = ((s_30_1) == (s_30_3));
        // D s_30_5: write-var gs#62338 <= s_30_4
        fn_state.gs_62338 = s_30_4;
        // N s_30_6: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var __ICC_SRE_EL3_Enable:u8
        let s_31_0: bool = fn_state.u__ICC_SRE_EL3_Enable;
        // D s_31_1: cast zx s_31_0 -> bv
        let s_31_1: Bits = Bits::new(s_31_0 as u128, 1u16);
        // C s_31_2: const #0u : u8
        let s_31_2: bool = false;
        // C s_31_3: cast zx s_31_2 -> bv
        let s_31_3: Bits = Bits::new(s_31_2 as u128, 1u16);
        // D s_31_4: cmp-eq s_31_1 s_31_3
        let s_31_4: bool = ((s_31_1) == (s_31_3));
        // D s_31_5: write-var gs#62334 <= s_31_4
        fn_state.gs_62334 = s_31_4;
        // N s_31_6: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_32_0: panic
        panic!("{:?}", ());
        // N s_32_1: return
        return;
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var __ICC_SRE_EL3_Enable:u8
        let s_33_0: bool = fn_state.u__ICC_SRE_EL3_Enable;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 1u16);
        // C s_33_2: const #0u : u8
        let s_33_2: bool = false;
        // C s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 1u16);
        // D s_33_4: cmp-eq s_33_1 s_33_3
        let s_33_4: bool = ((s_33_1) == (s_33_3));
        // D s_33_5: write-var gs#62333 <= s_33_4
        fn_state.gs_62333 = s_33_4;
        // N s_33_6: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_34_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_34_1: call __IMPDEF_boolean(s_34_0)
        let s_34_1: bool = u__IMPDEF_boolean(state, tracer, s_34_0);
        // D s_34_2: write-var gs#62332 <= s_34_1
        fn_state.gs_62332 = s_34_1;
        // N s_34_3: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var __EDSCR_SDD:u8
        let s_35_0: bool = fn_state.u__EDSCR_SDD;
        // D s_35_1: cast zx s_35_0 -> bv
        let s_35_1: Bits = Bits::new(s_35_0 as u128, 1u16);
        // C s_35_2: const #1u : u8
        let s_35_2: bool = true;
        // C s_35_3: cast zx s_35_2 -> bv
        let s_35_3: Bits = Bits::new(s_35_2 as u128, 1u16);
        // D s_35_4: cmp-eq s_35_1 s_35_3
        let s_35_4: bool = ((s_35_1) == (s_35_3));
        // D s_35_5: write-var gs#62331 <= s_35_4
        fn_state.gs_62331 = s_35_4;
        // N s_35_6: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #424u : u32
        let s_36_0: u32 = 424;
        // D s_36_1: read-reg s_36_0:u8
        let s_36_1: u8 = {
            let value = state.read_register::<u8>(s_36_0 as isize);
            tracer.read_register(s_36_0 as isize, value);
            value
        };
        // C s_36_2: const #2u : u8
        let s_36_2: u8 = 2;
        // D s_36_3: cmp-lt s_36_1 s_36_2
        let s_36_3: bool = ((s_36_1) < (s_36_2));
        // D s_36_4: write-var gs#62330 <= s_36_3
        fn_state.gs_62330 = s_36_3;
        // N s_36_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #() : ()
        let s_37_0: () = ();
        // S s_37_1: call Halted(s_37_0)
        let s_37_1: bool = Halted(state, tracer, s_37_0);
        // N s_37_2: branch s_37_1 b70 b38
        if s_37_1 {
            return block_70(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#62339 <= s_38_0
        fn_state.gs_62339 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#62339:u8
        let s_39_0: bool = fn_state.gs_62339;
        // N s_39_1: branch s_39_0 b69 b40
        if s_39_0 {
            return block_69(state, tracer, fn_state);
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
        // D s_40_1: write-var gs#62340 <= s_40_0
        fn_state.gs_62340 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#62340:u8
        let s_41_0: bool = fn_state.gs_62340;
        // N s_41_1: branch s_41_0 b68 b42
        if s_41_0 {
            return block_68(state, tracer, fn_state);
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
        // D s_42_1: write-var gs#62341 <= s_42_0
        fn_state.gs_62341 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#62341:u8
        let s_43_0: bool = fn_state.gs_62341;
        // N s_43_1: branch s_43_0 b67 b44
        if s_43_0 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #0u : u8
        let s_44_0: bool = false;
        // D s_44_1: write-var gs#62342 <= s_44_0
        fn_state.gs_62342 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#62342:u8
        let s_45_0: bool = fn_state.gs_62342;
        // N s_45_1: branch s_45_0 b66 b46
        if s_45_0 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #() : ()
        let s_46_0: () = ();
        // S s_46_1: call EL2Enabled(s_46_0)
        let s_46_1: bool = EL2Enabled(state, tracer, s_46_0);
        // N s_46_2: branch s_46_1 b65 b47
        if s_46_1 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #0u : u8
        let s_47_0: bool = false;
        // D s_47_1: write-var gs#62343 <= s_47_0
        fn_state.gs_62343 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#62343:u8
        let s_48_0: bool = fn_state.gs_62343;
        // N s_48_1: branch s_48_0 b64 b49
        if s_48_0 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #424u : u32
        let s_49_0: u32 = 424;
        // D s_49_1: read-reg s_49_0:u8
        let s_49_1: u8 = {
            let value = state.read_register::<u8>(s_49_0 as isize);
            tracer.read_register(s_49_0 as isize, value);
            value
        };
        // C s_49_2: const #2u : u8
        let s_49_2: u8 = 2;
        // D s_49_3: cmp-lt s_49_1 s_49_2
        let s_49_3: bool = ((s_49_1) < (s_49_2));
        // N s_49_4: branch s_49_3 b63 b50
        if s_49_3 {
            return block_63(state, tracer, fn_state);
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
        // D s_50_1: write-var gs#62344 <= s_50_0
        fn_state.gs_62344 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#62344:u8
        let s_51_0: bool = fn_state.gs_62344;
        // N s_51_1: branch s_51_0 b57 b52
        if s_51_0 {
            return block_57(state, tracer, fn_state);
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
        // N s_52_4: branch s_52_3 b54 b53
        if s_52_3 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #64s : i64
        let s_53_0: i64 = 64;
        // C s_53_1: const #() : ()
        let s_53_1: () = ();
        // S s_53_2: call ICC_SRE_EL1_read(s_53_1)
        let s_53_2: ProductType5c790c8ef59cc8b2 = ICC_SRE_EL1_read(
            state,
            tracer,
            s_53_1,
        );
        // S s_53_3: call __get_ICC_SRE_EL1(s_53_2)
        let s_53_3: ProductType5c790c8ef59cc8b2 = u__get_ICC_SRE_EL1(
            state,
            tracer,
            s_53_2,
        );
        // D s_53_4: write-var ga#67855 <= s_53_3
        fn_state.ga_67855 = s_53_3;
        // D s_53_5: read-var ga#67855.0:struct
        let s_53_5: u64 = fn_state.ga_67855._0;
        // D s_53_6: cast zx s_53_5 -> bv
        let s_53_6: Bits = Bits::new(s_53_5 as u128, 64u16);
        // D s_53_7: read-var t:i
        let s_53_7: i128 = fn_state.t;
        // D s_53_8: call X_set(s_53_7, s_53_0, s_53_6)
        let s_53_8: () = X_set(state, tracer, s_53_7, s_53_0, s_53_6);
        // N s_53_9: return
        return;
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var __SCR_EL3_NS:u8
        let s_54_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_54_1: cast zx s_54_0 -> bv
        let s_54_1: Bits = Bits::new(s_54_0 as u128, 1u16);
        // C s_54_2: const #0u : u8
        let s_54_2: bool = false;
        // C s_54_3: cast zx s_54_2 -> bv
        let s_54_3: Bits = Bits::new(s_54_2 as u128, 1u16);
        // D s_54_4: cmp-eq s_54_1 s_54_3
        let s_54_4: bool = ((s_54_1) == (s_54_3));
        // N s_54_5: branch s_54_4 b56 b55
        if s_54_4 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #64s : i64
        let s_55_0: i64 = 64;
        // C s_55_1: const #22960u : u32
        let s_55_1: u32 = 22960;
        // D s_55_2: read-reg s_55_1:struct
        let s_55_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_55_1 as isize);
            tracer.read_register(s_55_1 as isize, value);
            value
        };
        // D s_55_3: call __get_ICC_SRE_EL1_NS(s_55_2)
        let s_55_3: ProductType5c790c8ef59cc8b2 = u__get_ICC_SRE_EL1_NS(
            state,
            tracer,
            s_55_2,
        );
        // D s_55_4: write-var ga#67851 <= s_55_3
        fn_state.ga_67851 = s_55_3;
        // D s_55_5: read-var ga#67851.0:struct
        let s_55_5: u64 = fn_state.ga_67851._0;
        // D s_55_6: cast zx s_55_5 -> bv
        let s_55_6: Bits = Bits::new(s_55_5 as u128, 64u16);
        // D s_55_7: read-var t:i
        let s_55_7: i128 = fn_state.t;
        // D s_55_8: call X_set(s_55_7, s_55_0, s_55_6)
        let s_55_8: () = X_set(state, tracer, s_55_7, s_55_0, s_55_6);
        // N s_55_9: return
        return;
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #64s : i64
        let s_56_0: i64 = 64;
        // C s_56_1: const #11904u : u32
        let s_56_1: u32 = 11904;
        // D s_56_2: read-reg s_56_1:struct
        let s_56_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_56_1 as isize);
            tracer.read_register(s_56_1 as isize, value);
            value
        };
        // D s_56_3: call __get_ICC_SRE_EL1_S(s_56_2)
        let s_56_3: ProductType5c790c8ef59cc8b2 = u__get_ICC_SRE_EL1_S(
            state,
            tracer,
            s_56_2,
        );
        // D s_56_4: write-var ga#67848 <= s_56_3
        fn_state.ga_67848 = s_56_3;
        // D s_56_5: read-var ga#67848.0:struct
        let s_56_5: u64 = fn_state.ga_67848._0;
        // D s_56_6: cast zx s_56_5 -> bv
        let s_56_6: Bits = Bits::new(s_56_5 as u128, 64u16);
        // D s_56_7: read-var t:i
        let s_56_7: i128 = fn_state.t;
        // D s_56_8: call X_set(s_56_7, s_56_0, s_56_6)
        let s_56_8: () = X_set(state, tracer, s_56_7, s_56_0, s_56_6);
        // N s_56_9: return
        return;
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #() : ()
        let s_57_0: () = ();
        // S s_57_1: call Halted(s_57_0)
        let s_57_1: bool = Halted(state, tracer, s_57_0);
        // N s_57_2: branch s_57_1 b62 b58
        if s_57_1 {
            return block_62(state, tracer, fn_state);
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
        // D s_58_1: write-var gs#62348 <= s_58_0
        fn_state.gs_62348 = s_58_0;
        // N s_58_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var gs#62348:u8
        let s_59_0: bool = fn_state.gs_62348;
        // N s_59_1: branch s_59_0 b61 b60
        if s_59_0 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #24u : u8
        let s_60_0: u8 = 24;
        // C s_60_1: cast zx s_60_0 -> bv
        let s_60_1: Bits = Bits::new(s_60_0 as u128, 8u16);
        // C s_60_2: cast zx s_60_1 -> i
        let s_60_2: i128 = (s_60_1.value() as i128);
        // C s_60_3: cast reint s_60_2 -> i64
        let s_60_3: i64 = (s_60_2 as i64);
        // C s_60_4: cast zx s_60_3 -> i
        let s_60_4: i128 = (i128::try_from(s_60_3).unwrap());
        // C s_60_5: const #424u : u32
        let s_60_5: u32 = 424;
        // D s_60_6: read-reg s_60_5:u8
        let s_60_6: u8 = {
            let value = state.read_register::<u8>(s_60_5 as isize);
            tracer.read_register(s_60_5 as isize, value);
            value
        };
        // D s_60_7: call AArch64_SystemAccessTrap(s_60_6, s_60_4)
        let s_60_7: () = AArch64_SystemAccessTrap(state, tracer, s_60_6, s_60_4);
        // N s_60_8: return
        return;
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_61_0: panic
        panic!("{:?}", ());
        // N s_61_1: return
        return;
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var __EDSCR_SDD:u8
        let s_62_0: bool = fn_state.u__EDSCR_SDD;
        // D s_62_1: cast zx s_62_0 -> bv
        let s_62_1: Bits = Bits::new(s_62_0 as u128, 1u16);
        // C s_62_2: const #1u : u8
        let s_62_2: bool = true;
        // C s_62_3: cast zx s_62_2 -> bv
        let s_62_3: Bits = Bits::new(s_62_2 as u128, 1u16);
        // D s_62_4: cmp-eq s_62_1 s_62_3
        let s_62_4: bool = ((s_62_1) == (s_62_3));
        // D s_62_5: write-var gs#62348 <= s_62_4
        fn_state.gs_62348 = s_62_4;
        // N s_62_6: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var __ICC_SRE_EL3_Enable:u8
        let s_63_0: bool = fn_state.u__ICC_SRE_EL3_Enable;
        // D s_63_1: cast zx s_63_0 -> bv
        let s_63_1: Bits = Bits::new(s_63_0 as u128, 1u16);
        // C s_63_2: const #0u : u8
        let s_63_2: bool = false;
        // C s_63_3: cast zx s_63_2 -> bv
        let s_63_3: Bits = Bits::new(s_63_2 as u128, 1u16);
        // D s_63_4: cmp-eq s_63_1 s_63_3
        let s_63_4: bool = ((s_63_1) == (s_63_3));
        // D s_63_5: write-var gs#62344 <= s_63_4
        fn_state.gs_62344 = s_63_4;
        // N s_63_6: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #24u : u8
        let s_64_0: u8 = 24;
        // C s_64_1: cast zx s_64_0 -> bv
        let s_64_1: Bits = Bits::new(s_64_0 as u128, 8u16);
        // C s_64_2: cast zx s_64_1 -> i
        let s_64_2: i128 = (s_64_1.value() as i128);
        // C s_64_3: cast reint s_64_2 -> i64
        let s_64_3: i64 = (s_64_2 as i64);
        // C s_64_4: cast zx s_64_3 -> i
        let s_64_4: i128 = (i128::try_from(s_64_3).unwrap());
        // C s_64_5: const #432u : u32
        let s_64_5: u32 = 432;
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
        // D s_65_0: read-var __ICC_SRE_EL2_Enable:u8
        let s_65_0: bool = fn_state.u__ICC_SRE_EL2_Enable;
        // D s_65_1: cast zx s_65_0 -> bv
        let s_65_1: Bits = Bits::new(s_65_0 as u128, 1u16);
        // C s_65_2: const #0u : u8
        let s_65_2: bool = false;
        // C s_65_3: cast zx s_65_2 -> bv
        let s_65_3: Bits = Bits::new(s_65_2 as u128, 1u16);
        // D s_65_4: cmp-eq s_65_1 s_65_3
        let s_65_4: bool = ((s_65_1) == (s_65_3));
        // D s_65_5: write-var gs#62343 <= s_65_4
        fn_state.gs_62343 = s_65_4;
        // N s_65_6: jump b48
        return block_48(state, tracer, fn_state);
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
        // D s_67_0: read-var __ICC_SRE_EL3_Enable:u8
        let s_67_0: bool = fn_state.u__ICC_SRE_EL3_Enable;
        // D s_67_1: cast zx s_67_0 -> bv
        let s_67_1: Bits = Bits::new(s_67_0 as u128, 1u16);
        // C s_67_2: const #0u : u8
        let s_67_2: bool = false;
        // C s_67_3: cast zx s_67_2 -> bv
        let s_67_3: Bits = Bits::new(s_67_2 as u128, 1u16);
        // D s_67_4: cmp-eq s_67_1 s_67_3
        let s_67_4: bool = ((s_67_1) == (s_67_3));
        // D s_67_5: write-var gs#62342 <= s_67_4
        fn_state.gs_62342 = s_67_4;
        // N s_67_6: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_68_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_68_1: call __IMPDEF_boolean(s_68_0)
        let s_68_1: bool = u__IMPDEF_boolean(state, tracer, s_68_0);
        // D s_68_2: write-var gs#62341 <= s_68_1
        fn_state.gs_62341 = s_68_1;
        // N s_68_3: jump b43
        return block_43(state, tracer, fn_state);
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
        // D s_69_5: write-var gs#62340 <= s_69_4
        fn_state.gs_62340 = s_69_4;
        // N s_69_6: jump b41
        return block_41(state, tracer, fn_state);
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
        // D s_70_4: write-var gs#62339 <= s_70_3
        fn_state.gs_62339 = s_70_3;
        // N s_70_5: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_71_0: panic
        panic!("{:?}", ());
        // N s_71_1: return
        return;
    }
}
