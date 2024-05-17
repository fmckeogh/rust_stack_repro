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
use X_set::*;
use u_get_MDCR_EL2_Type_TDA::*;
use u__get_OSDTRRX_EL1::*;
use u_get_MDCR_EL3_Type_TDA::*;
use Halted::*;
use ConstrainUnpredictableBool::*;
use u_get_MDCR_EL2_Type_TDCC::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_MDCR_EL3_Type_TDCC::*;
use AArch64_SystemAccessTrap::*;
use u__IMPDEF_boolean::*;
use EL2Enabled::*;
use EDSCR_read::*;
use u_get_MDCR_EL2_Type_TDE::*;
use common::*;
pub fn OSDTRRX_EL1_SysRegRead_ed7388c985c3d430<T: Tracer>(
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
        gs_63505: bool,
        gs_63526: bool,
        gs_63520: bool,
        gs_63516: bool,
        gs_63507: bool,
        gs_63525: bool,
        gs_63514: bool,
        gs_63504: bool,
        u__EDSCR_SDD: bool,
        gs_63511: bool,
        gs_63509: bool,
        gs_63503: bool,
        gs_63512: bool,
        gs_63510: bool,
        u__MDCR_EL3_TDA: bool,
        gs_63524: bool,
        gs_63529: bool,
        gs_63519: bool,
        gs_63506: bool,
        gs_63522: bool,
        u__MDCR_EL3_TDCC: bool,
        gs_63517: bool,
        gs_63501: bool,
        gs_63515: bool,
        gs_63527: bool,
        gs_63530: bool,
        gs_63508: bool,
        gs_63518: bool,
        u__PSTATE_EL: u8,
        u__MDCR_EL2_TDCC: bool,
        gs_63523: bool,
        gs_63521: bool,
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
        // C s_0_7: const #22712u : u32
        let s_0_7: u32 = 22712;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_MDCR_EL3_Type_TDCC(s_0_8)
        let s_0_9: bool = u_get_MDCR_EL3_Type_TDCC(state, tracer, s_0_8);
        // D s_0_10: write-var __MDCR_EL3_TDCC <= s_0_9
        fn_state.u__MDCR_EL3_TDCC = s_0_9;
        // C s_0_11: const #22712u : u32
        let s_0_11: u32 = 22712;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_MDCR_EL3_Type_TDA(s_0_12)
        let s_0_13: bool = u_get_MDCR_EL3_Type_TDA(state, tracer, s_0_12);
        // D s_0_14: write-var __MDCR_EL3_TDA <= s_0_13
        fn_state.u__MDCR_EL3_TDA = s_0_13;
        // C s_0_15: const #104880u : u32
        let s_0_15: u32 = 104880;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_MDCR_EL2_Type_TDCC(s_0_16)
        let s_0_17: bool = u_get_MDCR_EL2_Type_TDCC(state, tracer, s_0_16);
        // D s_0_18: write-var __MDCR_EL2_TDCC <= s_0_17
        fn_state.u__MDCR_EL2_TDCC = s_0_17;
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
        // N s_0_25: branch s_0_24 b119 b1
        if s_0_24 {
            return block_119(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call Halted(s_1_0)
        let s_1_1: bool = Halted(state, tracer, s_1_0);
        // N s_1_2: branch s_1_1 b118 b2
        if s_1_1 {
            return block_118(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#63501 <= s_2_0
        fn_state.gs_63501 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#63501:u8
        let s_3_0: bool = fn_state.gs_63501;
        // N s_3_1: branch s_3_0 b117 b4
        if s_3_0 {
            return block_117(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var __PSTATE_EL:u8
        let s_4_0: u8 = fn_state.u__PSTATE_EL;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 2u16);
        // C s_4_2: const #440u : u32
        let s_4_2: u32 = 440;
        // D s_4_3: read-reg s_4_2:u8
        let s_4_3: u8 = {
            let value = state.read_register::<u8>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 2u16);
        // D s_4_5: cmp-eq s_4_1 s_4_4
        let s_4_5: bool = ((s_4_1) == (s_4_4));
        // N s_4_6: branch s_4_5 b58 b5
        if s_4_5 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var __PSTATE_EL:u8
        let s_5_0: u8 = fn_state.u__PSTATE_EL;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // C s_5_2: const #432u : u32
        let s_5_2: u32 = 432;
        // D s_5_3: read-reg s_5_2:u8
        let s_5_3: u8 = {
            let value = state.read_register::<u8>(s_5_2 as isize);
            tracer.read_register(s_5_2 as isize, value);
            value
        };
        // D s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 2u16);
        // D s_5_5: cmp-eq s_5_1 s_5_4
        let s_5_5: bool = ((s_5_1) == (s_5_4));
        // N s_5_6: branch s_5_5 b9 b6
        if s_5_5 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var __PSTATE_EL:u8
        let s_6_0: u8 = fn_state.u__PSTATE_EL;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 2u16);
        // C s_6_2: const #424u : u32
        let s_6_2: u32 = 424;
        // D s_6_3: read-reg s_6_2:u8
        let s_6_3: u8 = {
            let value = state.read_register::<u8>(s_6_2 as isize);
            tracer.read_register(s_6_2 as isize, value);
            value
        };
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 2u16);
        // D s_6_5: cmp-eq s_6_1 s_6_4
        let s_6_5: bool = ((s_6_1) == (s_6_4));
        // N s_6_6: branch s_6_5 b8 b7
        if s_6_5 {
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
        // N s_7_0: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #64s : i64
        let s_8_0: i64 = 64;
        // C s_8_1: const #15872u : u32
        let s_8_1: u32 = 15872;
        // D s_8_2: read-reg s_8_1:u64
        let s_8_2: u64 = {
            let value = state.read_register::<u64>(s_8_1 as isize);
            tracer.read_register(s_8_1 as isize, value);
            value
        };
        // D s_8_3: call __get_OSDTRRX_EL1(s_8_2)
        let s_8_3: u64 = u__get_OSDTRRX_EL1(state, tracer, s_8_2);
        // D s_8_4: cast zx s_8_3 -> bv
        let s_8_4: Bits = Bits::new(s_8_3 as u128, 64u16);
        // D s_8_5: read-var t:i
        let s_8_5: i128 = fn_state.t;
        // D s_8_6: call X_set(s_8_5, s_8_0, s_8_4)
        let s_8_6: () = X_set(state, tracer, s_8_5, s_8_0, s_8_4);
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
        // S s_9_1: call Halted(s_9_0)
        let s_9_1: bool = Halted(state, tracer, s_9_0);
        // N s_9_2: branch s_9_1 b57 b10
        if s_9_1 {
            return block_57(state, tracer, fn_state);
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
        // D s_10_1: write-var gs#63503 <= s_10_0
        fn_state.gs_63503 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#63503:u8
        let s_11_0: bool = fn_state.gs_63503;
        // N s_11_1: branch s_11_0 b56 b12
        if s_11_0 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#63504 <= s_12_0
        fn_state.gs_63504 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#63504:u8
        let s_13_0: bool = fn_state.gs_63504;
        // N s_13_1: branch s_13_0 b55 b14
        if s_13_0 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#63505 <= s_14_0
        fn_state.gs_63505 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#63505:u8
        let s_15_0: bool = fn_state.gs_63505;
        // N s_15_1: branch s_15_0 b54 b16
        if s_15_0 {
            return block_54(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#63506 <= s_16_0
        fn_state.gs_63506 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#63506:u8
        let s_17_0: bool = fn_state.gs_63506;
        // N s_17_1: branch s_17_0 b53 b18
        if s_17_0 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call Halted(s_18_0)
        let s_18_1: bool = Halted(state, tracer, s_18_0);
        // N s_18_2: branch s_18_1 b52 b19
        if s_18_1 {
            return block_52(state, tracer, fn_state);
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
        // D s_19_1: write-var gs#63507 <= s_19_0
        fn_state.gs_63507 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#63507:u8
        let s_20_0: bool = fn_state.gs_63507;
        // N s_20_1: branch s_20_0 b51 b21
        if s_20_0 {
            return block_51(state, tracer, fn_state);
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
        // D s_21_1: write-var gs#63508 <= s_21_0
        fn_state.gs_63508 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#63508:u8
        let s_22_0: bool = fn_state.gs_63508;
        // N s_22_1: branch s_22_0 b50 b23
        if s_22_0 {
            return block_50(state, tracer, fn_state);
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
        // D s_23_1: write-var gs#63509 <= s_23_0
        fn_state.gs_63509 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#63509:u8
        let s_24_0: bool = fn_state.gs_63509;
        // N s_24_1: branch s_24_0 b49 b25
        if s_24_0 {
            return block_49(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#63510 <= s_25_0
        fn_state.gs_63510 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#63510:u8
        let s_26_0: bool = fn_state.gs_63510;
        // N s_26_1: branch s_26_0 b48 b27
        if s_26_0 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
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
        // N s_27_4: branch s_27_3 b47 b28
        if s_27_3 {
            return block_47(state, tracer, fn_state);
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
        // D s_28_1: write-var gs#63511 <= s_28_0
        fn_state.gs_63511 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#63511:u8
        let s_29_0: bool = fn_state.gs_63511;
        // N s_29_1: branch s_29_0 b41 b30
        if s_29_0 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #424u : u32
        let s_30_0: u32 = 424;
        // D s_30_1: read-reg s_30_0:u8
        let s_30_1: u8 = {
            let value = state.read_register::<u8>(s_30_0 as isize);
            tracer.read_register(s_30_0 as isize, value);
            value
        };
        // C s_30_2: const #2u : u8
        let s_30_2: u8 = 2;
        // D s_30_3: cmp-lt s_30_1 s_30_2
        let s_30_3: bool = ((s_30_1) < (s_30_2));
        // N s_30_4: branch s_30_3 b40 b31
        if s_30_3 {
            return block_40(state, tracer, fn_state);
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
        // D s_31_1: write-var gs#63512 <= s_31_0
        fn_state.gs_63512 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#63512:u8
        let s_32_0: bool = fn_state.gs_63512;
        // N s_32_1: branch s_32_0 b34 b33
        if s_32_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #64s : i64
        let s_33_0: i64 = 64;
        // C s_33_1: const #15872u : u32
        let s_33_1: u32 = 15872;
        // D s_33_2: read-reg s_33_1:u64
        let s_33_2: u64 = {
            let value = state.read_register::<u64>(s_33_1 as isize);
            tracer.read_register(s_33_1 as isize, value);
            value
        };
        // D s_33_3: call __get_OSDTRRX_EL1(s_33_2)
        let s_33_3: u64 = u__get_OSDTRRX_EL1(state, tracer, s_33_2);
        // D s_33_4: cast zx s_33_3 -> bv
        let s_33_4: Bits = Bits::new(s_33_3 as u128, 64u16);
        // D s_33_5: read-var t:i
        let s_33_5: i128 = fn_state.t;
        // D s_33_6: call X_set(s_33_5, s_33_0, s_33_4)
        let s_33_6: () = X_set(state, tracer, s_33_5, s_33_0, s_33_4);
        // N s_33_7: return
        return;
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #() : ()
        let s_34_0: () = ();
        // S s_34_1: call Halted(s_34_0)
        let s_34_1: bool = Halted(state, tracer, s_34_0);
        // N s_34_2: branch s_34_1 b39 b35
        if s_34_1 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #0u : u8
        let s_35_0: bool = false;
        // D s_35_1: write-var gs#63514 <= s_35_0
        fn_state.gs_63514 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#63514:u8
        let s_36_0: bool = fn_state.gs_63514;
        // N s_36_1: branch s_36_0 b38 b37
        if s_36_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #24u : u8
        let s_37_0: u8 = 24;
        // C s_37_1: cast zx s_37_0 -> bv
        let s_37_1: Bits = Bits::new(s_37_0 as u128, 8u16);
        // C s_37_2: cast zx s_37_1 -> i
        let s_37_2: i128 = (s_37_1.value() as i128);
        // C s_37_3: cast reint s_37_2 -> i64
        let s_37_3: i64 = (s_37_2 as i64);
        // C s_37_4: cast zx s_37_3 -> i
        let s_37_4: i128 = (i128::try_from(s_37_3).unwrap());
        // C s_37_5: const #424u : u32
        let s_37_5: u32 = 424;
        // D s_37_6: read-reg s_37_5:u8
        let s_37_6: u8 = {
            let value = state.read_register::<u8>(s_37_5 as isize);
            tracer.read_register(s_37_5 as isize, value);
            value
        };
        // D s_37_7: call AArch64_SystemAccessTrap(s_37_6, s_37_4)
        let s_37_7: () = AArch64_SystemAccessTrap(state, tracer, s_37_6, s_37_4);
        // N s_37_8: return
        return;
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_38_0: panic
        panic!("{:?}", ());
        // N s_38_1: return
        return;
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var __EDSCR_SDD:u8
        let s_39_0: bool = fn_state.u__EDSCR_SDD;
        // D s_39_1: cast zx s_39_0 -> bv
        let s_39_1: Bits = Bits::new(s_39_0 as u128, 1u16);
        // C s_39_2: const #1u : u8
        let s_39_2: bool = true;
        // C s_39_3: cast zx s_39_2 -> bv
        let s_39_3: Bits = Bits::new(s_39_2 as u128, 1u16);
        // D s_39_4: cmp-eq s_39_1 s_39_3
        let s_39_4: bool = ((s_39_1) == (s_39_3));
        // D s_39_5: write-var gs#63514 <= s_39_4
        fn_state.gs_63514 = s_39_4;
        // N s_39_6: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var __MDCR_EL3_TDA:u8
        let s_40_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_40_1: cast zx s_40_0 -> bv
        let s_40_1: Bits = Bits::new(s_40_0 as u128, 1u16);
        // C s_40_2: const #1u : u8
        let s_40_2: bool = true;
        // C s_40_3: cast zx s_40_2 -> bv
        let s_40_3: Bits = Bits::new(s_40_2 as u128, 1u16);
        // D s_40_4: cmp-eq s_40_1 s_40_3
        let s_40_4: bool = ((s_40_1) == (s_40_3));
        // D s_40_5: write-var gs#63512 <= s_40_4
        fn_state.gs_63512 = s_40_4;
        // N s_40_6: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #() : ()
        let s_41_0: () = ();
        // S s_41_1: call Halted(s_41_0)
        let s_41_1: bool = Halted(state, tracer, s_41_0);
        // N s_41_2: branch s_41_1 b46 b42
        if s_41_1 {
            return block_46(state, tracer, fn_state);
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
        // D s_42_1: write-var gs#63515 <= s_42_0
        fn_state.gs_63515 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#63515:u8
        let s_43_0: bool = fn_state.gs_63515;
        // N s_43_1: branch s_43_0 b45 b44
        if s_43_0 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
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
        // C s_44_5: const #424u : u32
        let s_44_5: u32 = 424;
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
        // N s_45_0: panic
        panic!("{:?}", ());
        // N s_45_1: return
        return;
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var __EDSCR_SDD:u8
        let s_46_0: bool = fn_state.u__EDSCR_SDD;
        // D s_46_1: cast zx s_46_0 -> bv
        let s_46_1: Bits = Bits::new(s_46_0 as u128, 1u16);
        // C s_46_2: const #1u : u8
        let s_46_2: bool = true;
        // C s_46_3: cast zx s_46_2 -> bv
        let s_46_3: Bits = Bits::new(s_46_2 as u128, 1u16);
        // D s_46_4: cmp-eq s_46_1 s_46_3
        let s_46_4: bool = ((s_46_1) == (s_46_3));
        // D s_46_5: write-var gs#63515 <= s_46_4
        fn_state.gs_63515 = s_46_4;
        // N s_46_6: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var __MDCR_EL3_TDCC:u8
        let s_47_0: bool = fn_state.u__MDCR_EL3_TDCC;
        // D s_47_1: cast zx s_47_0 -> bv
        let s_47_1: Bits = Bits::new(s_47_0 as u128, 1u16);
        // C s_47_2: const #1u : u8
        let s_47_2: bool = true;
        // C s_47_3: cast zx s_47_2 -> bv
        let s_47_3: Bits = Bits::new(s_47_2 as u128, 1u16);
        // D s_47_4: cmp-eq s_47_1 s_47_3
        let s_47_4: bool = ((s_47_1) == (s_47_3));
        // D s_47_5: write-var gs#63511 <= s_47_4
        fn_state.gs_63511 = s_47_4;
        // N s_47_6: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_48_0: panic
        panic!("{:?}", ());
        // N s_48_1: return
        return;
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var __MDCR_EL3_TDA:u8
        let s_49_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_49_1: cast zx s_49_0 -> bv
        let s_49_1: Bits = Bits::new(s_49_0 as u128, 1u16);
        // C s_49_2: const #1u : u8
        let s_49_2: bool = true;
        // C s_49_3: cast zx s_49_2 -> bv
        let s_49_3: Bits = Bits::new(s_49_2 as u128, 1u16);
        // D s_49_4: cmp-eq s_49_1 s_49_3
        let s_49_4: bool = ((s_49_1) == (s_49_3));
        // D s_49_5: write-var gs#63510 <= s_49_4
        fn_state.gs_63510 = s_49_4;
        // N s_49_6: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_50_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_50_1: call __IMPDEF_boolean(s_50_0)
        let s_50_1: bool = u__IMPDEF_boolean(state, tracer, s_50_0);
        // D s_50_2: write-var gs#63509 <= s_50_1
        fn_state.gs_63509 = s_50_1;
        // N s_50_3: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var __EDSCR_SDD:u8
        let s_51_0: bool = fn_state.u__EDSCR_SDD;
        // D s_51_1: cast zx s_51_0 -> bv
        let s_51_1: Bits = Bits::new(s_51_0 as u128, 1u16);
        // C s_51_2: const #1u : u8
        let s_51_2: bool = true;
        // C s_51_3: cast zx s_51_2 -> bv
        let s_51_3: Bits = Bits::new(s_51_2 as u128, 1u16);
        // D s_51_4: cmp-eq s_51_1 s_51_3
        let s_51_4: bool = ((s_51_1) == (s_51_3));
        // D s_51_5: write-var gs#63508 <= s_51_4
        fn_state.gs_63508 = s_51_4;
        // N s_51_6: jump b22
        return block_22(state, tracer, fn_state);
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
        // D s_52_4: write-var gs#63507 <= s_52_3
        fn_state.gs_63507 = s_52_3;
        // N s_52_5: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_53_0: panic
        panic!("{:?}", ());
        // N s_53_1: return
        return;
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var __MDCR_EL3_TDCC:u8
        let s_54_0: bool = fn_state.u__MDCR_EL3_TDCC;
        // D s_54_1: cast zx s_54_0 -> bv
        let s_54_1: Bits = Bits::new(s_54_0 as u128, 1u16);
        // C s_54_2: const #1u : u8
        let s_54_2: bool = true;
        // C s_54_3: cast zx s_54_2 -> bv
        let s_54_3: Bits = Bits::new(s_54_2 as u128, 1u16);
        // D s_54_4: cmp-eq s_54_1 s_54_3
        let s_54_4: bool = ((s_54_1) == (s_54_3));
        // D s_54_5: write-var gs#63506 <= s_54_4
        fn_state.gs_63506 = s_54_4;
        // N s_54_6: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_55_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_55_1: call __IMPDEF_boolean(s_55_0)
        let s_55_1: bool = u__IMPDEF_boolean(state, tracer, s_55_0);
        // D s_55_2: write-var gs#63505 <= s_55_1
        fn_state.gs_63505 = s_55_1;
        // N s_55_3: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var __EDSCR_SDD:u8
        let s_56_0: bool = fn_state.u__EDSCR_SDD;
        // D s_56_1: cast zx s_56_0 -> bv
        let s_56_1: Bits = Bits::new(s_56_0 as u128, 1u16);
        // C s_56_2: const #1u : u8
        let s_56_2: bool = true;
        // C s_56_3: cast zx s_56_2 -> bv
        let s_56_3: Bits = Bits::new(s_56_2 as u128, 1u16);
        // D s_56_4: cmp-eq s_56_1 s_56_3
        let s_56_4: bool = ((s_56_1) == (s_56_3));
        // D s_56_5: write-var gs#63504 <= s_56_4
        fn_state.gs_63504 = s_56_4;
        // N s_56_6: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #424u : u32
        let s_57_0: u32 = 424;
        // D s_57_1: read-reg s_57_0:u8
        let s_57_1: u8 = {
            let value = state.read_register::<u8>(s_57_0 as isize);
            tracer.read_register(s_57_0 as isize, value);
            value
        };
        // C s_57_2: const #2u : u8
        let s_57_2: u8 = 2;
        // D s_57_3: cmp-lt s_57_1 s_57_2
        let s_57_3: bool = ((s_57_1) < (s_57_2));
        // D s_57_4: write-var gs#63503 <= s_57_3
        fn_state.gs_63503 = s_57_3;
        // N s_57_5: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #() : ()
        let s_58_0: () = ();
        // S s_58_1: call Halted(s_58_0)
        let s_58_1: bool = Halted(state, tracer, s_58_0);
        // N s_58_2: branch s_58_1 b116 b59
        if s_58_1 {
            return block_116(state, tracer, fn_state);
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
        // D s_59_1: write-var gs#63516 <= s_59_0
        fn_state.gs_63516 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#63516:u8
        let s_60_0: bool = fn_state.gs_63516;
        // N s_60_1: branch s_60_0 b115 b61
        if s_60_0 {
            return block_115(state, tracer, fn_state);
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
        // D s_61_1: write-var gs#63517 <= s_61_0
        fn_state.gs_63517 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#63517:u8
        let s_62_0: bool = fn_state.gs_63517;
        // N s_62_1: branch s_62_0 b114 b63
        if s_62_0 {
            return block_114(state, tracer, fn_state);
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
        // D s_63_1: write-var gs#63518 <= s_63_0
        fn_state.gs_63518 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#63518:u8
        let s_64_0: bool = fn_state.gs_63518;
        // N s_64_1: branch s_64_0 b113 b65
        if s_64_0 {
            return block_113(state, tracer, fn_state);
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
        // D s_65_1: write-var gs#63519 <= s_65_0
        fn_state.gs_63519 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#63519:u8
        let s_66_0: bool = fn_state.gs_63519;
        // N s_66_1: branch s_66_0 b112 b67
        if s_66_0 {
            return block_112(state, tracer, fn_state);
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
        // S s_67_1: call Halted(s_67_0)
        let s_67_1: bool = Halted(state, tracer, s_67_0);
        // N s_67_2: branch s_67_1 b111 b68
        if s_67_1 {
            return block_111(state, tracer, fn_state);
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
        // D s_68_1: write-var gs#63520 <= s_68_0
        fn_state.gs_63520 = s_68_0;
        // N s_68_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var gs#63520:u8
        let s_69_0: bool = fn_state.gs_63520;
        // N s_69_1: branch s_69_0 b110 b70
        if s_69_0 {
            return block_110(state, tracer, fn_state);
        } else {
            return block_70(state, tracer, fn_state);
        };
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #0u : u8
        let s_70_0: bool = false;
        // D s_70_1: write-var gs#63521 <= s_70_0
        fn_state.gs_63521 = s_70_0;
        // N s_70_2: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var gs#63521:u8
        let s_71_0: bool = fn_state.gs_63521;
        // N s_71_1: branch s_71_0 b109 b72
        if s_71_0 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_72(state, tracer, fn_state);
        };
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #0u : u8
        let s_72_0: bool = false;
        // D s_72_1: write-var gs#63522 <= s_72_0
        fn_state.gs_63522 = s_72_0;
        // N s_72_2: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var gs#63522:u8
        let s_73_0: bool = fn_state.gs_63522;
        // N s_73_1: branch s_73_0 b108 b74
        if s_73_0 {
            return block_108(state, tracer, fn_state);
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
        // D s_74_1: write-var gs#63523 <= s_74_0
        fn_state.gs_63523 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var gs#63523:u8
        let s_75_0: bool = fn_state.gs_63523;
        // N s_75_1: branch s_75_0 b107 b76
        if s_75_0 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_76(state, tracer, fn_state);
        };
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #() : ()
        let s_76_0: () = ();
        // S s_76_1: call EL2Enabled(s_76_0)
        let s_76_1: bool = EL2Enabled(state, tracer, s_76_0);
        // N s_76_2: branch s_76_1 b106 b77
        if s_76_1 {
            return block_106(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #0u : u8
        let s_77_0: bool = false;
        // D s_77_1: write-var gs#63524 <= s_77_0
        fn_state.gs_63524 = s_77_0;
        // N s_77_2: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var gs#63524:u8
        let s_78_0: bool = fn_state.gs_63524;
        // N s_78_1: branch s_78_0 b105 b79
        if s_78_0 {
            return block_105(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #() : ()
        let s_79_0: () = ();
        // S s_79_1: call EL2Enabled(s_79_0)
        let s_79_1: bool = EL2Enabled(state, tracer, s_79_0);
        // N s_79_2: branch s_79_1 b104 b80
        if s_79_1 {
            return block_104(state, tracer, fn_state);
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
        // D s_80_1: write-var gs#63525 <= s_80_0
        fn_state.gs_63525 = s_80_0;
        // N s_80_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#63525:u8
        let s_81_0: bool = fn_state.gs_63525;
        // N s_81_1: branch s_81_0 b103 b82
        if s_81_0 {
            return block_103(state, tracer, fn_state);
        } else {
            return block_82(state, tracer, fn_state);
        };
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
        // N s_82_4: branch s_82_3 b102 b83
        if s_82_3 {
            return block_102(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #0u : u8
        let s_83_0: bool = false;
        // D s_83_1: write-var gs#63526 <= s_83_0
        fn_state.gs_63526 = s_83_0;
        // N s_83_2: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var gs#63526:u8
        let s_84_0: bool = fn_state.gs_63526;
        // N s_84_1: branch s_84_0 b96 b85
        if s_84_0 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #424u : u32
        let s_85_0: u32 = 424;
        // D s_85_1: read-reg s_85_0:u8
        let s_85_1: u8 = {
            let value = state.read_register::<u8>(s_85_0 as isize);
            tracer.read_register(s_85_0 as isize, value);
            value
        };
        // C s_85_2: const #2u : u8
        let s_85_2: u8 = 2;
        // D s_85_3: cmp-lt s_85_1 s_85_2
        let s_85_3: bool = ((s_85_1) < (s_85_2));
        // N s_85_4: branch s_85_3 b95 b86
        if s_85_3 {
            return block_95(state, tracer, fn_state);
        } else {
            return block_86(state, tracer, fn_state);
        };
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #0u : u8
        let s_86_0: bool = false;
        // D s_86_1: write-var gs#63527 <= s_86_0
        fn_state.gs_63527 = s_86_0;
        // N s_86_2: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var gs#63527:u8
        let s_87_0: bool = fn_state.gs_63527;
        // N s_87_1: branch s_87_0 b89 b88
        if s_87_0 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_88(state, tracer, fn_state);
        };
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #64s : i64
        let s_88_0: i64 = 64;
        // C s_88_1: const #15872u : u32
        let s_88_1: u32 = 15872;
        // D s_88_2: read-reg s_88_1:u64
        let s_88_2: u64 = {
            let value = state.read_register::<u64>(s_88_1 as isize);
            tracer.read_register(s_88_1 as isize, value);
            value
        };
        // D s_88_3: call __get_OSDTRRX_EL1(s_88_2)
        let s_88_3: u64 = u__get_OSDTRRX_EL1(state, tracer, s_88_2);
        // D s_88_4: cast zx s_88_3 -> bv
        let s_88_4: Bits = Bits::new(s_88_3 as u128, 64u16);
        // D s_88_5: read-var t:i
        let s_88_5: i128 = fn_state.t;
        // D s_88_6: call X_set(s_88_5, s_88_0, s_88_4)
        let s_88_6: () = X_set(state, tracer, s_88_5, s_88_0, s_88_4);
        // N s_88_7: return
        return;
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #() : ()
        let s_89_0: () = ();
        // S s_89_1: call Halted(s_89_0)
        let s_89_1: bool = Halted(state, tracer, s_89_0);
        // N s_89_2: branch s_89_1 b94 b90
        if s_89_1 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_90(state, tracer, fn_state);
        };
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #0u : u8
        let s_90_0: bool = false;
        // D s_90_1: write-var gs#63529 <= s_90_0
        fn_state.gs_63529 = s_90_0;
        // N s_90_2: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_91_0: read-var gs#63529:u8
        let s_91_0: bool = fn_state.gs_63529;
        // N s_91_1: branch s_91_0 b93 b92
        if s_91_0 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_92(state, tracer, fn_state);
        };
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #24u : u8
        let s_92_0: u8 = 24;
        // C s_92_1: cast zx s_92_0 -> bv
        let s_92_1: Bits = Bits::new(s_92_0 as u128, 8u16);
        // C s_92_2: cast zx s_92_1 -> i
        let s_92_2: i128 = (s_92_1.value() as i128);
        // C s_92_3: cast reint s_92_2 -> i64
        let s_92_3: i64 = (s_92_2 as i64);
        // C s_92_4: cast zx s_92_3 -> i
        let s_92_4: i128 = (i128::try_from(s_92_3).unwrap());
        // C s_92_5: const #424u : u32
        let s_92_5: u32 = 424;
        // D s_92_6: read-reg s_92_5:u8
        let s_92_6: u8 = {
            let value = state.read_register::<u8>(s_92_5 as isize);
            tracer.read_register(s_92_5 as isize, value);
            value
        };
        // D s_92_7: call AArch64_SystemAccessTrap(s_92_6, s_92_4)
        let s_92_7: () = AArch64_SystemAccessTrap(state, tracer, s_92_6, s_92_4);
        // N s_92_8: return
        return;
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_93_0: panic
        panic!("{:?}", ());
        // N s_93_1: return
        return;
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var __EDSCR_SDD:u8
        let s_94_0: bool = fn_state.u__EDSCR_SDD;
        // D s_94_1: cast zx s_94_0 -> bv
        let s_94_1: Bits = Bits::new(s_94_0 as u128, 1u16);
        // C s_94_2: const #1u : u8
        let s_94_2: bool = true;
        // C s_94_3: cast zx s_94_2 -> bv
        let s_94_3: Bits = Bits::new(s_94_2 as u128, 1u16);
        // D s_94_4: cmp-eq s_94_1 s_94_3
        let s_94_4: bool = ((s_94_1) == (s_94_3));
        // D s_94_5: write-var gs#63529 <= s_94_4
        fn_state.gs_63529 = s_94_4;
        // N s_94_6: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_95_0: read-var __MDCR_EL3_TDA:u8
        let s_95_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_95_1: cast zx s_95_0 -> bv
        let s_95_1: Bits = Bits::new(s_95_0 as u128, 1u16);
        // C s_95_2: const #1u : u8
        let s_95_2: bool = true;
        // C s_95_3: cast zx s_95_2 -> bv
        let s_95_3: Bits = Bits::new(s_95_2 as u128, 1u16);
        // D s_95_4: cmp-eq s_95_1 s_95_3
        let s_95_4: bool = ((s_95_1) == (s_95_3));
        // D s_95_5: write-var gs#63527 <= s_95_4
        fn_state.gs_63527 = s_95_4;
        // N s_95_6: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #() : ()
        let s_96_0: () = ();
        // S s_96_1: call Halted(s_96_0)
        let s_96_1: bool = Halted(state, tracer, s_96_0);
        // N s_96_2: branch s_96_1 b101 b97
        if s_96_1 {
            return block_101(state, tracer, fn_state);
        } else {
            return block_97(state, tracer, fn_state);
        };
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #0u : u8
        let s_97_0: bool = false;
        // D s_97_1: write-var gs#63530 <= s_97_0
        fn_state.gs_63530 = s_97_0;
        // N s_97_2: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var gs#63530:u8
        let s_98_0: bool = fn_state.gs_63530;
        // N s_98_1: branch s_98_0 b100 b99
        if s_98_0 {
            return block_100(state, tracer, fn_state);
        } else {
            return block_99(state, tracer, fn_state);
        };
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #24u : u8
        let s_99_0: u8 = 24;
        // C s_99_1: cast zx s_99_0 -> bv
        let s_99_1: Bits = Bits::new(s_99_0 as u128, 8u16);
        // C s_99_2: cast zx s_99_1 -> i
        let s_99_2: i128 = (s_99_1.value() as i128);
        // C s_99_3: cast reint s_99_2 -> i64
        let s_99_3: i64 = (s_99_2 as i64);
        // C s_99_4: cast zx s_99_3 -> i
        let s_99_4: i128 = (i128::try_from(s_99_3).unwrap());
        // C s_99_5: const #424u : u32
        let s_99_5: u32 = 424;
        // D s_99_6: read-reg s_99_5:u8
        let s_99_6: u8 = {
            let value = state.read_register::<u8>(s_99_5 as isize);
            tracer.read_register(s_99_5 as isize, value);
            value
        };
        // D s_99_7: call AArch64_SystemAccessTrap(s_99_6, s_99_4)
        let s_99_7: () = AArch64_SystemAccessTrap(state, tracer, s_99_6, s_99_4);
        // N s_99_8: return
        return;
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_100_0: panic
        panic!("{:?}", ());
        // N s_100_1: return
        return;
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_101_0: read-var __EDSCR_SDD:u8
        let s_101_0: bool = fn_state.u__EDSCR_SDD;
        // D s_101_1: cast zx s_101_0 -> bv
        let s_101_1: Bits = Bits::new(s_101_0 as u128, 1u16);
        // C s_101_2: const #1u : u8
        let s_101_2: bool = true;
        // C s_101_3: cast zx s_101_2 -> bv
        let s_101_3: Bits = Bits::new(s_101_2 as u128, 1u16);
        // D s_101_4: cmp-eq s_101_1 s_101_3
        let s_101_4: bool = ((s_101_1) == (s_101_3));
        // D s_101_5: write-var gs#63530 <= s_101_4
        fn_state.gs_63530 = s_101_4;
        // N s_101_6: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var __MDCR_EL3_TDCC:u8
        let s_102_0: bool = fn_state.u__MDCR_EL3_TDCC;
        // D s_102_1: cast zx s_102_0 -> bv
        let s_102_1: Bits = Bits::new(s_102_0 as u128, 1u16);
        // C s_102_2: const #1u : u8
        let s_102_2: bool = true;
        // C s_102_3: cast zx s_102_2 -> bv
        let s_102_3: Bits = Bits::new(s_102_2 as u128, 1u16);
        // D s_102_4: cmp-eq s_102_1 s_102_3
        let s_102_4: bool = ((s_102_1) == (s_102_3));
        // D s_102_5: write-var gs#63526 <= s_102_4
        fn_state.gs_63526 = s_102_4;
        // N s_102_6: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #24u : u8
        let s_103_0: u8 = 24;
        // C s_103_1: cast zx s_103_0 -> bv
        let s_103_1: Bits = Bits::new(s_103_0 as u128, 8u16);
        // C s_103_2: cast zx s_103_1 -> i
        let s_103_2: i128 = (s_103_1.value() as i128);
        // C s_103_3: cast reint s_103_2 -> i64
        let s_103_3: i64 = (s_103_2 as i64);
        // C s_103_4: cast zx s_103_3 -> i
        let s_103_4: i128 = (i128::try_from(s_103_3).unwrap());
        // C s_103_5: const #432u : u32
        let s_103_5: u32 = 432;
        // D s_103_6: read-reg s_103_5:u8
        let s_103_6: u8 = {
            let value = state.read_register::<u8>(s_103_5 as isize);
            tracer.read_register(s_103_5 as isize, value);
            value
        };
        // D s_103_7: call AArch64_SystemAccessTrap(s_103_6, s_103_4)
        let s_103_7: () = AArch64_SystemAccessTrap(state, tracer, s_103_6, s_103_4);
        // N s_103_8: return
        return;
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #104880u : u32
        let s_104_0: u32 = 104880;
        // D s_104_1: read-reg s_104_0:struct
        let s_104_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_104_0 as isize);
            tracer.read_register(s_104_0 as isize, value);
            value
        };
        // D s_104_2: call _get_MDCR_EL2_Type_TDE(s_104_1)
        let s_104_2: bool = u_get_MDCR_EL2_Type_TDE(state, tracer, s_104_1);
        // C s_104_3: const #104880u : u32
        let s_104_3: u32 = 104880;
        // D s_104_4: read-reg s_104_3:struct
        let s_104_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_104_3 as isize);
            tracer.read_register(s_104_3 as isize, value);
            value
        };
        // D s_104_5: call _get_MDCR_EL2_Type_TDA(s_104_4)
        let s_104_5: bool = u_get_MDCR_EL2_Type_TDA(state, tracer, s_104_4);
        // D s_104_6: cast zx s_104_2 -> bv
        let s_104_6: Bits = Bits::new(s_104_2 as u128, 1u16);
        // D s_104_7: cast zx s_104_5 -> bv
        let s_104_7: Bits = Bits::new(s_104_5 as u128, 1u16);
        // D s_104_8: cast reint s_104_6 -> u128
        let s_104_8: u128 = (s_104_6.value() as u128);
        // D s_104_9: size-of s_104_6
        let s_104_9: u16 = s_104_6.length();
        // D s_104_10: cast reint s_104_7 -> u128
        let s_104_10: u128 = (s_104_7.value() as u128);
        // D s_104_11: size-of s_104_7
        let s_104_11: u16 = s_104_7.length();
        // D s_104_12: lsl s_104_8 s_104_11
        let s_104_12: u128 = s_104_8 << s_104_11;
        // D s_104_13: or s_104_12 s_104_10
        let s_104_13: u128 = ((s_104_12) | (s_104_10));
        // D s_104_14: add s_104_9 s_104_11
        let s_104_14: u16 = (s_104_9 + s_104_11);
        // D s_104_15: create-bits s_104_13 s_104_14
        let s_104_15: Bits = Bits::new(s_104_13, s_104_14);
        // D s_104_16: cast reint s_104_15 -> u8
        let s_104_16: u8 = (s_104_15.value() as u8);
        // D s_104_17: cast zx s_104_16 -> bv
        let s_104_17: Bits = Bits::new(s_104_16 as u128, 2u16);
        // C s_104_18: const #0u : u8
        let s_104_18: u8 = 0;
        // C s_104_19: cast zx s_104_18 -> bv
        let s_104_19: Bits = Bits::new(s_104_18 as u128, 2u16);
        // D s_104_20: cmp-ne s_104_17 s_104_19
        let s_104_20: bool = ((s_104_17) != (s_104_19));
        // D s_104_21: write-var gs#63525 <= s_104_20
        fn_state.gs_63525 = s_104_20;
        // N s_104_22: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #24u : u8
        let s_105_0: u8 = 24;
        // C s_105_1: cast zx s_105_0 -> bv
        let s_105_1: Bits = Bits::new(s_105_0 as u128, 8u16);
        // C s_105_2: cast zx s_105_1 -> i
        let s_105_2: i128 = (s_105_1.value() as i128);
        // C s_105_3: cast reint s_105_2 -> i64
        let s_105_3: i64 = (s_105_2 as i64);
        // C s_105_4: cast zx s_105_3 -> i
        let s_105_4: i128 = (i128::try_from(s_105_3).unwrap());
        // C s_105_5: const #432u : u32
        let s_105_5: u32 = 432;
        // D s_105_6: read-reg s_105_5:u8
        let s_105_6: u8 = {
            let value = state.read_register::<u8>(s_105_5 as isize);
            tracer.read_register(s_105_5 as isize, value);
            value
        };
        // D s_105_7: call AArch64_SystemAccessTrap(s_105_6, s_105_4)
        let s_105_7: () = AArch64_SystemAccessTrap(state, tracer, s_105_6, s_105_4);
        // N s_105_8: return
        return;
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var __MDCR_EL2_TDCC:u8
        let s_106_0: bool = fn_state.u__MDCR_EL2_TDCC;
        // D s_106_1: cast zx s_106_0 -> bv
        let s_106_1: Bits = Bits::new(s_106_0 as u128, 1u16);
        // C s_106_2: const #1u : u8
        let s_106_2: bool = true;
        // C s_106_3: cast zx s_106_2 -> bv
        let s_106_3: Bits = Bits::new(s_106_2 as u128, 1u16);
        // D s_106_4: cmp-eq s_106_1 s_106_3
        let s_106_4: bool = ((s_106_1) == (s_106_3));
        // D s_106_5: write-var gs#63524 <= s_106_4
        fn_state.gs_63524 = s_106_4;
        // N s_106_6: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_107_0: panic
        panic!("{:?}", ());
        // N s_107_1: return
        return;
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var __MDCR_EL3_TDA:u8
        let s_108_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_108_1: cast zx s_108_0 -> bv
        let s_108_1: Bits = Bits::new(s_108_0 as u128, 1u16);
        // C s_108_2: const #1u : u8
        let s_108_2: bool = true;
        // C s_108_3: cast zx s_108_2 -> bv
        let s_108_3: Bits = Bits::new(s_108_2 as u128, 1u16);
        // D s_108_4: cmp-eq s_108_1 s_108_3
        let s_108_4: bool = ((s_108_1) == (s_108_3));
        // D s_108_5: write-var gs#63523 <= s_108_4
        fn_state.gs_63523 = s_108_4;
        // N s_108_6: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_109_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_109_1: call __IMPDEF_boolean(s_109_0)
        let s_109_1: bool = u__IMPDEF_boolean(state, tracer, s_109_0);
        // D s_109_2: write-var gs#63522 <= s_109_1
        fn_state.gs_63522 = s_109_1;
        // N s_109_3: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_110_0: read-var __EDSCR_SDD:u8
        let s_110_0: bool = fn_state.u__EDSCR_SDD;
        // D s_110_1: cast zx s_110_0 -> bv
        let s_110_1: Bits = Bits::new(s_110_0 as u128, 1u16);
        // C s_110_2: const #1u : u8
        let s_110_2: bool = true;
        // C s_110_3: cast zx s_110_2 -> bv
        let s_110_3: Bits = Bits::new(s_110_2 as u128, 1u16);
        // D s_110_4: cmp-eq s_110_1 s_110_3
        let s_110_4: bool = ((s_110_1) == (s_110_3));
        // D s_110_5: write-var gs#63521 <= s_110_4
        fn_state.gs_63521 = s_110_4;
        // N s_110_6: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_111_0: const #424u : u32
        let s_111_0: u32 = 424;
        // D s_111_1: read-reg s_111_0:u8
        let s_111_1: u8 = {
            let value = state.read_register::<u8>(s_111_0 as isize);
            tracer.read_register(s_111_0 as isize, value);
            value
        };
        // C s_111_2: const #2u : u8
        let s_111_2: u8 = 2;
        // D s_111_3: cmp-lt s_111_1 s_111_2
        let s_111_3: bool = ((s_111_1) < (s_111_2));
        // D s_111_4: write-var gs#63520 <= s_111_3
        fn_state.gs_63520 = s_111_3;
        // N s_111_5: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_112_0: panic
        panic!("{:?}", ());
        // N s_112_1: return
        return;
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var __MDCR_EL3_TDCC:u8
        let s_113_0: bool = fn_state.u__MDCR_EL3_TDCC;
        // D s_113_1: cast zx s_113_0 -> bv
        let s_113_1: Bits = Bits::new(s_113_0 as u128, 1u16);
        // C s_113_2: const #1u : u8
        let s_113_2: bool = true;
        // C s_113_3: cast zx s_113_2 -> bv
        let s_113_3: Bits = Bits::new(s_113_2 as u128, 1u16);
        // D s_113_4: cmp-eq s_113_1 s_113_3
        let s_113_4: bool = ((s_113_1) == (s_113_3));
        // D s_113_5: write-var gs#63519 <= s_113_4
        fn_state.gs_63519 = s_113_4;
        // N s_113_6: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_114_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_114_1: call __IMPDEF_boolean(s_114_0)
        let s_114_1: bool = u__IMPDEF_boolean(state, tracer, s_114_0);
        // D s_114_2: write-var gs#63518 <= s_114_1
        fn_state.gs_63518 = s_114_1;
        // N s_114_3: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_115_0: read-var __EDSCR_SDD:u8
        let s_115_0: bool = fn_state.u__EDSCR_SDD;
        // D s_115_1: cast zx s_115_0 -> bv
        let s_115_1: Bits = Bits::new(s_115_0 as u128, 1u16);
        // C s_115_2: const #1u : u8
        let s_115_2: bool = true;
        // C s_115_3: cast zx s_115_2 -> bv
        let s_115_3: Bits = Bits::new(s_115_2 as u128, 1u16);
        // D s_115_4: cmp-eq s_115_1 s_115_3
        let s_115_4: bool = ((s_115_1) == (s_115_3));
        // D s_115_5: write-var gs#63517 <= s_115_4
        fn_state.gs_63517 = s_115_4;
        // N s_115_6: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #424u : u32
        let s_116_0: u32 = 424;
        // D s_116_1: read-reg s_116_0:u8
        let s_116_1: u8 = {
            let value = state.read_register::<u8>(s_116_0 as isize);
            tracer.read_register(s_116_0 as isize, value);
            value
        };
        // C s_116_2: const #2u : u8
        let s_116_2: u8 = 2;
        // D s_116_3: cmp-lt s_116_1 s_116_2
        let s_116_3: bool = ((s_116_1) < (s_116_2));
        // D s_116_4: write-var gs#63516 <= s_116_3
        fn_state.gs_63516 = s_116_3;
        // N s_116_5: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #64s : i64
        let s_117_0: i64 = 64;
        // C s_117_1: const #15872u : u32
        let s_117_1: u32 = 15872;
        // D s_117_2: read-reg s_117_1:u64
        let s_117_2: u64 = {
            let value = state.read_register::<u64>(s_117_1 as isize);
            tracer.read_register(s_117_1 as isize, value);
            value
        };
        // D s_117_3: call __get_OSDTRRX_EL1(s_117_2)
        let s_117_3: u64 = u__get_OSDTRRX_EL1(state, tracer, s_117_2);
        // D s_117_4: cast zx s_117_3 -> bv
        let s_117_4: Bits = Bits::new(s_117_3 as u128, 64u16);
        // D s_117_5: read-var t:i
        let s_117_5: i128 = fn_state.t;
        // D s_117_6: call X_set(s_117_5, s_117_0, s_117_4)
        let s_117_6: () = X_set(state, tracer, s_117_5, s_117_0, s_117_4);
        // N s_117_7: return
        return;
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #70u : u32
        let s_118_0: u32 = 70;
        // S s_118_1: call ConstrainUnpredictableBool(s_118_0)
        let s_118_1: bool = ConstrainUnpredictableBool(state, tracer, s_118_0);
        // D s_118_2: write-var gs#63501 <= s_118_1
        fn_state.gs_63501 = s_118_1;
        // N s_118_3: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_119_0: panic
        panic!("{:?}", ());
        // N s_119_1: return
        return;
    }
}
